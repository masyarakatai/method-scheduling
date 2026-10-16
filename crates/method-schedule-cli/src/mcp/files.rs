//! Reading and writing schedule files: revisions, size limits, the checked edit and atomic
//! writes.
//!
//! The server keeps nothing between calls. A read returns the file's revision, a
//! fingerprint of its bytes; a change must give that revision back, and is refused, with
//! nothing written, when the file on disk no longer has it. A change is made on a copy
//! through `method-schedule-core`, then the copy must pass `Schedule::check` and come back unchanged
//! through the file format before it is written. Writes go to a temporary file in the same
//! folder, flushed, then renamed over the schedule, keeping its permissions; just before
//! the rename the file is read once more, so a save that lands while the change was being
//! made is not lost. A new file is linked into place, which never replaces one that
//! appeared meanwhile. Nothing here deletes a file other than its own temporary one.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use method_schedule_core::Schedule;

use super::limits;
use super::paths::{NEW_SCHEDULE_EXTS, READ_SCHEDULE_EXTS, SCHEDULE_EXTS};
use super::tools::{Args, Ctx, ToolError};

/// A file's revision: FNV-1a 64 of its bytes as 16 hex digits, the same fingerprint the
/// budget exchange file's `source_hash` uses. It notices any change; it is not a
/// security hash.
pub fn revision(bytes: &[u8]) -> String {
    method_schedule_format::exchange::fnv1a64(bytes)
}

/// Reads a file whole, refusing one over [`limits::MAX_FILE_BYTES`] (also one that grows
/// past it while being read).
pub fn read_file(path: &Path, shown: &str) -> Result<Vec<u8>, ToolError> {
    let too_big = || {
        ToolError::refused(format!(
            "{shown}: larger than this server reads ({} MB)",
            limits::MAX_FILE_BYTES / (1024 * 1024)
        ))
    };
    let f = std::fs::File::open(path).map_err(|e| ToolError::io(format!("{shown}: {e}")))?;
    let len = f
        .metadata()
        .map_err(|e| ToolError::io(format!("{shown}: {e}")))?
        .len();
    if len > limits::MAX_FILE_BYTES {
        return Err(too_big());
    }
    let mut bytes = Vec::with_capacity(len as usize);
    f.take(limits::MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| ToolError::io(format!("{shown}: {e}")))?;
    if bytes.len() as u64 > limits::MAX_FILE_BYTES {
        return Err(too_big());
    }
    Ok(bytes)
}

/// A file's bytes as UTF-8 text.
pub fn text(bytes: &[u8], shown: &str) -> Result<String, ToolError> {
    String::from_utf8(bytes.to_vec())
        .map_err(|_| ToolError::invalid(format!("{shown}: not a text file (not UTF-8)")))
}

/// Parses a schedule file's bytes (strict, and `Schedule::check` must pass).
pub fn parse(bytes: &[u8], shown: &str) -> Result<Schedule, ToolError> {
    method_schedule_format::from_str(&text(bytes, shown)?)
        .map_err(|e| ToolError::invalid(format!("{shown}: {e}")))
}

/// Whether `bytes`, read as `s`, are byte for byte what this build writes for it (as a
/// template when they are one).
pub fn is_canonical(s: &Schedule, bytes: &[u8]) -> bool {
    let template = std::str::from_utf8(bytes).is_ok_and(method_schedule_format::is_template);
    let written = if template {
        method_schedule_format::to_template_string_as_is(s).unwrap_or_default()
    } else {
        method_schedule_format::to_string(s)
    };
    written.as_bytes() == bytes
}

/// A schedule as read for a read tool.
pub struct Loaded {
    pub schedule: Schedule,
    /// The path as shown back to the client.
    pub shown: String,
    pub revision: String,
    /// Whether the file is byte for byte what this build would write.
    pub canonical: bool,
    /// Whether the file is a template (read only here).
    pub template: bool,
}

/// Reads the schedule named by the `path` argument.
pub fn load(ctx: &Ctx, a: &Args) -> Result<Loaded, ToolError> {
    load_from(ctx, a, "path")
}

/// Reads the schedule named by the argument `key`.
pub fn load_from(ctx: &Ctx, a: &Args, key: &str) -> Result<Loaded, ToolError> {
    let typed = a.req(key)?;
    let real = ctx.roots.existing(typed, READ_SCHEDULE_EXTS)?;
    let shown = ctx.roots.show(&real);
    let bytes = read_file(&real, &shown)?;
    let schedule = parse(&bytes, &shown)?;
    let template = method_schedule_format::is_template(&text(&bytes, &shown)?);
    Ok(Loaded {
        revision: revision(&bytes),
        canonical: is_canonical(&schedule, &bytes),
        schedule,
        shown,
        template,
    })
}

/// What a change did.
pub struct Edited<T> {
    pub shown: String,
    /// Whether anything changed (nothing is written when not).
    pub changed: bool,
    /// The file's revision now.
    pub revision: String,
    /// What the change function returned.
    pub out: T,
    /// The schedule after the change.
    pub schedule: Schedule,
}

/// Changes the schedule named by the `path` argument with `f` and writes it, refused with
/// nothing written when:
/// - the file's revision is not the `revision` argument (it changed since that read);
/// - `f` fails (the engine refused the change);
/// - the result fails `Schedule::check`, or doesn't come back the same through the file
///   format (a value the file can't hold);
/// - the file changes on disk while the change is being made.
pub fn edit<T>(
    ctx: &Ctx,
    a: &Args,
    f: impl FnOnce(&mut Schedule) -> Result<T, ToolError>,
) -> Result<Edited<T>, ToolError> {
    let typed = a.req("path")?;
    let want = a.req("revision")?;
    let is_mtst = Path::new(typed)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("mtst"));
    if is_mtst {
        return Err(template_refused(typed));
    }
    let real = ctx.roots.writable(typed, SCHEDULE_EXTS)?;
    let shown = ctx.roots.show(&real);
    let bytes = read_file(&real, &shown)?;
    if method_schedule_format::is_template(&text(&bytes, &shown)?) {
        return Err(template_refused(&shown));
    }
    if revision(&bytes) != want {
        return Err(conflict(&shown));
    }
    let before = parse(&bytes, &shown)?;
    let mut after = before.clone();
    let out = f(&mut after)?;
    if after == before {
        return Ok(Edited {
            shown,
            changed: false,
            revision: revision(&bytes),
            out,
            schedule: after,
        });
    }
    let new_bytes = checked_bytes(&after, &shown)?;
    replace(&real, &shown, &bytes, &new_bytes)?;
    Ok(Edited {
        shown,
        changed: true,
        revision: revision(&new_bytes),
        out,
        schedule: after,
    })
}

/// Writes a new schedule to the file named by the `path` argument, which must not exist.
/// Returns the path as shown and the file's revision.
pub fn create(ctx: &Ctx, a: &Args, s: &Schedule) -> Result<(String, String), ToolError> {
    let typed = a.req("path")?;
    let real = ctx.roots.target(typed, NEW_SCHEDULE_EXTS)?;
    let shown = ctx.roots.show(&real);
    if std::fs::symlink_metadata(&real).is_ok() {
        return Err(ToolError::refused(format!(
            "{shown} already exists; this server never replaces a file with a new one"
        )));
    }
    let bytes = checked_bytes(s, &shown)?;
    let tmp = write_temp(&real, &shown, &bytes, None)?;
    // A hard link fails when the name is taken, so a file that appeared since the check
    // above is never replaced.
    let linked = std::fs::hard_link(&tmp, &real);
    let _ = std::fs::remove_file(&tmp);
    linked.map_err(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            ToolError::refused(format!("{shown} already exists; nothing was written"))
        } else {
            ToolError::io(format!("{shown}: {e}"))
        }
    })?;
    Ok((shown, revision(&bytes)))
}

/// The bytes to write for `s`, after the checks every write goes through.
fn checked_bytes(s: &Schedule, shown: &str) -> Result<Vec<u8>, ToolError> {
    s.check().map_err(|e| {
        ToolError::refused(format!(
            "after this change the schedule wouldn't hang together ({e}). Nothing was \
             written to {shown}."
        ))
    })?;
    let text = method_schedule_format::to_string(s);
    match method_schedule_format::from_str(&text) {
        Ok(back) if back == *s => {}
        Ok(_) => {
            return Err(ToolError::refused(format!(
                "after this change the schedule wouldn't read back the same. Nothing was \
                 written to {shown}."
            )))
        }
        Err(e) => {
            return Err(ToolError::refused(format!(
                "after this change the file wouldn't read back ({e}). Nothing was written \
                 to {shown}."
            )))
        }
    }
    if text.len() as u64 > limits::MAX_FILE_BYTES {
        return Err(ToolError::refused(format!(
            "{shown}: the result would be larger than {} MB; nothing was written",
            limits::MAX_FILE_BYTES / (1024 * 1024)
        )));
    }
    Ok(text.into_bytes())
}

/// Replaces the file at `real`, whose bytes were `old`, with `new`, atomically.
fn replace(real: &Path, shown: &str, old: &[u8], new: &[u8]) -> Result<(), ToolError> {
    let perms = std::fs::metadata(real).ok().map(|m| m.permissions());
    let tmp = write_temp(real, shown, new, perms)?;
    let now = match read_file(real, shown) {
        Ok(b) => b,
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }
    };
    if now != old {
        let _ = std::fs::remove_file(&tmp);
        return Err(conflict(shown));
    }
    std::fs::rename(&tmp, real).map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        ToolError::io(format!("{shown}: {e}"))
    })
}

/// Writes `bytes` to a new temporary file beside `real`, flushed to disk.
fn write_temp(
    real: &Path,
    shown: &str,
    bytes: &[u8],
    perms: Option<std::fs::Permissions>,
) -> Result<PathBuf, ToolError> {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let name = real
        .file_name()
        .ok_or_else(|| ToolError::path(format!("{shown}: needs a file name")))?;
    let tmp = real.with_file_name(format!(
        ".{}.mcp-tmp-{}-{}",
        name.to_string_lossy(),
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let write = || -> std::io::Result<()> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp)?;
        f.write_all(bytes)?;
        if let Some(p) = perms {
            f.set_permissions(p)?;
        }
        f.sync_all()
    };
    write().map_err(|e| {
        let _ = std::fs::remove_file(&tmp);
        ToolError::io(format!("{shown}: {e}"))
    })?;
    Ok(tmp)
}

/// Templates are read only here: nothing written to one could keep it a template and
/// still be the change asked for (decision 85).
fn template_refused(shown: &str) -> ToolError {
    ToolError::refused(format!(
        "{shown} is a template, which this server reads but never changes. Change it in \
         the app, or start a schedule from it with `method-schedule new FILE --from \
         TEMPLATE` and change that. Nothing was written."
    ))
}

fn conflict(shown: &str) -> ToolError {
    ToolError::conflict(format!(
        "{shown} changed on disk since the read this change is based on (another program, \
         person or call saved it). Nothing was written. Read it again (read_schedule), look \
         at what changed, then make the change again with the new revision."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revisions_follow_the_bytes() {
        assert_eq!(revision(b""), "cbf29ce484222325");
        assert_ne!(revision(b"a"), revision(b"b"));
        assert_eq!(revision(b"same"), revision(b"same"));
    }

    #[test]
    fn replace_refuses_when_the_file_changed_underneath() {
        let dir =
            std::env::temp_dir().join(format!("method-schedule-mcp-files-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("a.mtsc");
        std::fs::write(&f, "one").unwrap();
        let e = replace(&f, "a.mtsc", b"other", b"two").unwrap_err();
        assert_eq!(e.kind, "conflict");
        assert_eq!(std::fs::read(&f).unwrap(), b"one");
        replace(&f, "a.mtsc", b"one", b"two").unwrap();
        assert_eq!(std::fs::read(&f).unwrap(), b"two");
        // No temporary file is left behind either way.
        let left: Vec<_> = std::fs::read_dir(&dir).unwrap().collect();
        assert_eq!(left.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
