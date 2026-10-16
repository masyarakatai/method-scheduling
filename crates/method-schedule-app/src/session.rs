//! One open schedule: the authoritative [`Schedule`], where it lives on disk, whether it has
//! unsaved changes, and undo/redo. No Tauri types here, so the whole edit -> view path is
//! testable without a window.

use std::collections::VecDeque;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use method_schedule_core::model::BoardId;
use method_schedule_core::{fdx, fountain, Schedule};

use crate::clipboard::Clip;
use crate::safety::{self, DiskCheck, DiskStamp};

/// Undo steps kept. Each step is a whole-schedule snapshot; the oldest goes first.
pub const UNDO_LIMIT: usize = 200;

/// The title a new schedule gets.
pub const UNTITLED: &str = "Untitled";

/// The file extension this slice suggests in Save dialogs. A working name, like the
/// format's: it changes when the product is named (decisions.md, entry 4).
pub const EXTENSION: &str = "mtsc";

/// Why a save didn't happen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveError {
    /// The file changed on disk since this schedule was read from or written to it.
    /// Nothing was written.
    ChangedOnDisk(PathBuf),
    /// Writing failed. The old file, if any, is untouched.
    Io(String),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::ChangedOnDisk(p) => write!(
                f,
                "{} changed on disk after it was opened here, so it was not overwritten. \
                 Use Save As to keep this version in another file.",
                file_name(p)
            ),
            SaveError::Io(why) => f.write_str(why),
        }
    }
}

fn file_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| p.display().to_string())
}

struct Step {
    schedule: Schedule,
    rev: u64,
}

pub struct Session {
    schedule: Schedule,
    path: Option<PathBuf>,
    /// What the file held when this session last read or wrote it.
    disk: Option<DiskStamp>,
    /// That version, read: the base when merging a change made outside the app
    /// (decisions.md, entry 179). Kept with `disk`.
    last_read: Option<Arc<Schedule>>,
    undo: VecDeque<Step>,
    redo: Vec<Step>,
    /// Which version of the schedule this is: every edit gets a new number, undo and redo
    /// bring an old one back. Unsaved changes means `rev != saved_rev`.
    rev: u64,
    next_rev: u64,
    saved_rev: u64,
    /// Autosave and recovery bookkeeping (src/safety.rs, src/safety_app.rs).
    safety: SafetyState,
    /// The board the window shows. Not part of the schedule and not undone; when it is
    /// gone (deleted, or undone away) the first board shows.
    board: Option<BoardId>,
    /// Cut, copy and paste (src/clipboard.rs). Not part of the schedule and not undone; it
    /// goes with the session, so a new or opened schedule starts with it empty (its ids
    /// mean nothing in another schedule).
    clipboard: Option<Clip>,
}

/// What autosave needs to know about the open schedule.
#[derive(Debug)]
struct SafetyState {
    /// Names this schedule's recovery copy (unique per open schedule).
    recovery_key: String,
    /// The latest change (edit, undo or redo).
    last_change: Option<Instant>,
    /// The first change not yet in the file or a recovery copy.
    pending_since: Option<Instant>,
    /// The rev in the recovery copy, if one was written and not removed.
    recovery_rev: Option<u64>,
    /// Recovered after a crash: autosave leaves the original file alone, and the first
    /// Save into it asks first.
    confirm_overwrite: bool,
    /// The file whose previous version this session has backed up.
    backed_up: Option<PathBuf>,
    /// The file changed outside the app: autosave into it is paused.
    conflict: bool,
    /// The hash of the outside version the user last put off deciding about (Later), so
    /// the check on focus doesn't ask again until the file changes again.
    put_off: Option<u64>,
}

impl SafetyState {
    fn new(path: Option<&Path>) -> SafetyState {
        SafetyState {
            recovery_key: safety::new_recovery_key(path),
            last_change: None,
            pending_since: None,
            recovery_rev: None,
            confirm_overwrite: false,
            backed_up: None,
            conflict: false,
            put_off: None,
        }
    }
}

impl Default for Session {
    fn default() -> Session {
        Session::new()
    }
}

impl Session {
    /// A new, empty, untitled schedule.
    pub fn new() -> Session {
        Session::with_schedule(Schedule::with_default_categories(UNTITLED), None)
    }

    pub(crate) fn with_schedule(schedule: Schedule, path: Option<PathBuf>) -> Session {
        let safety = SafetyState::new(path.as_deref());
        Session {
            schedule,
            path,
            disk: None,
            last_read: None,
            undo: VecDeque::new(),
            redo: Vec::new(),
            rev: 0,
            next_rev: 1,
            saved_rev: 0,
            safety,
            board: None,
            clipboard: None,
        }
    }

    /// A schedule recovered after a crash: unsaved, pointing at the file it came from
    /// (`None`: never saved), keeping the recovery copy it came from under `key`.
    /// Autosave leaves the original file alone until a Save into it, which asks first.
    pub fn recovered(text: &str, path: Option<PathBuf>, key: String) -> Result<Session, String> {
        let schedule = method_schedule_format::from_str(text)
            .map_err(|e| format!("The recovered schedule can't be read: {e}"))?;
        let mut s = Session::with_schedule(schedule, path);
        s.rev = 1;
        s.next_rev = 2;
        s.safety.recovery_key = key;
        s.safety.recovery_rev = Some(1);
        s.safety.confirm_overwrite = s.path.is_some();
        Ok(s)
    }

    /// Reads a schedule file and starts a fresh session on it. A template (decisions.md,
    /// entry 85) starts a new, untitled schedule instead, so the template stays as it is.
    pub fn open(path: &Path) -> Result<Session, String> {
        let (schedule, bytes) = read_schedule(path)?;
        if std::str::from_utf8(&bytes).is_ok_and(method_schedule_format::is_template) {
            return Ok(crate::templates::session_from(&schedule));
        }
        let mut s = Session::with_schedule(schedule, Some(path.to_path_buf()));
        s.disk = Some(DiskStamp::at(&bytes, path));
        s.last_read = Some(Arc::new(s.schedule.clone()));
        Ok(s)
    }

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    /// The board the window shows: the one chosen while it exists, else the first.
    pub fn shown_board(&self) -> Option<BoardId> {
        self.board
            .filter(|b| self.schedule.board(*b).is_some())
            .or_else(|| self.schedule.boards.first().map(|b| b.id))
    }

    /// Shows another board. Refused when there is no such board.
    pub fn show_board(&mut self, board: BoardId) -> Result<(), String> {
        if self.schedule.board(board).is_none() {
            return Err("That board is no longer in the schedule.".to_string());
        }
        self.board = Some(board);
        Ok(())
    }

    /// What Cut or Copy put on the app's clipboard.
    pub fn clipboard(&self) -> Option<&Clip> {
        self.clipboard.as_ref()
    }

    /// Puts something on the app's clipboard, or empties it with `None`.
    pub fn set_clipboard(&mut self, clip: Option<Clip>) {
        self.clipboard = clip;
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn is_dirty(&self) -> bool {
        self.rev != self.saved_rev
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Never edited, never saved, no file: a script import may name it.
    pub fn is_untouched(&self) -> bool {
        self.path.is_none() && self.rev == 0 && self.undo.is_empty() && self.redo.is_empty()
    }

    /// The file's name, or "Untitled".
    pub fn display_name(&self) -> String {
        match &self.path {
            Some(p) => file_name(p),
            None => UNTITLED.to_string(),
        }
    }

    /// Applies `f` to a copy of the schedule. On success the copy becomes current, the old
    /// one goes on the undo stack and the redo stack is cleared. On error, or when the
    /// schedule would no longer hang together (`Schedule::check`), nothing changes. An edit
    /// that changes nothing leaves history and the dirty flag alone.
    ///
    /// Every change to the schedule goes through here.
    pub fn edit<T>(
        &mut self,
        f: impl FnOnce(&mut Schedule) -> Result<T, String>,
    ) -> Result<T, String> {
        let mut next = self.schedule.clone();
        let out = f(&mut next)?;
        if next == self.schedule {
            return Ok(out);
        }
        next.check()
            .map_err(|e| format!("That change was refused: {e}"))?;
        let old = std::mem::replace(&mut self.schedule, next);
        self.undo.push_back(Step {
            schedule: old,
            rev: self.rev,
        });
        while self.undo.len() > UNDO_LIMIT {
            self.undo.pop_front();
        }
        self.redo.clear();
        self.rev = self.next_rev;
        self.next_rev += 1;
        self.note_change(Instant::now());
        Ok(out)
    }

    /// Steps back once. False when there is nothing to undo.
    pub fn undo(&mut self) -> bool {
        let Some(step) = self.undo.pop_back() else {
            return false;
        };
        let now = std::mem::replace(&mut self.schedule, step.schedule);
        self.redo.push(Step {
            schedule: now,
            rev: self.rev,
        });
        self.rev = step.rev;
        self.note_change(Instant::now());
        true
    }

    /// Steps forward once. False when there is nothing to redo.
    pub fn redo(&mut self) -> bool {
        let Some(step) = self.redo.pop() else {
            return false;
        };
        let now = std::mem::replace(&mut self.schedule, step.schedule);
        self.undo.push_back(Step {
            schedule: now,
            rev: self.rev,
        });
        self.rev = step.rev;
        self.note_change(Instant::now());
        true
    }

    /// Writes the schedule to `path` atomically, then remembers the path and marks the
    /// schedule saved.
    ///
    /// The outside-change guard: before writing over the file this session read or last
    /// wrote, the file is read and hashed, and the write is refused when it holds anything
    /// else (another program changed it). A file that is gone is written again. Any other
    /// path is new to this session; the Save dialog has already asked about replacing it.
    pub fn save_to(&mut self, path: &Path) -> Result<(), SaveError> {
        if let (Some(current), Some(stamp)) = (self.path.as_deref(), self.disk) {
            if safety::same_file(current, path) && stamp.check(path) == DiskCheck::Changed {
                return Err(SaveError::ChangedOnDisk(path.to_path_buf()));
            }
        }
        let text = method_schedule_format::to_string(&self.schedule);
        safety::write_atomically(path, text.as_bytes(), safety::Access::Document).map_err(|e| {
            SaveError::Io(format!(
                "Couldn't save {}: {}",
                file_name(path),
                safety::write_problem(&e)
            ))
        })?;
        self.path = Some(path.to_path_buf());
        self.disk = Some(DiskStamp::at(text.as_bytes(), path));
        self.last_read = Some(Arc::new(self.schedule.clone()));
        self.saved_rev = self.rev;
        self.safety.pending_since = None;
        self.safety.confirm_overwrite = false;
        self.safety.conflict = false;
        self.safety.put_off = None;
        Ok(())
    }

    /// What the schedule's file held when this session last read or wrote it.
    pub fn disk_stamp(&self) -> Option<DiskStamp> {
        self.disk
    }

    /// That version, read (the base for merging an outside change). `None` for a
    /// schedule never read from or written to its file here (new, recovered).
    pub fn last_read(&self) -> Option<Arc<Schedule>> {
        self.last_read.clone()
    }

    /// The check when the window gets focus ([`DiskStamp::quick_check`]): `Same` without
    /// a file or a stamp. A file only touched (a new time, the same content) gets its new
    /// time stamped, so the next check needs no read.
    pub fn disk_quick_check(&mut self) -> DiskCheck {
        let (Some(path), Some(stamp)) = (self.path.as_deref(), self.disk) else {
            return DiskCheck::Same;
        };
        let out = stamp.quick_check(path);
        if out == DiskCheck::Same {
            let now = std::fs::metadata(path).and_then(|m| m.modified()).ok();
            if now != stamp.modified {
                self.disk = Some(DiskStamp {
                    modified: now,
                    ..stamp
                });
            }
        }
        out
    }

    /// Takes the version in the file stamped `stamp` (read as `theirs`, when it could be)
    /// as the one this session last read: the outside change is settled (Keep Mine just
    /// before it saves over it, Merge once theirs is merged in). Unsaved changes stay
    /// unsaved.
    pub fn took_disk(&mut self, stamp: DiskStamp, theirs: Option<Arc<Schedule>>) {
        self.disk = Some(stamp);
        if theirs.is_some() {
            self.last_read = theirs;
        }
        self.safety.conflict = false;
        self.safety.put_off = None;
    }

    /// Load Theirs: the version in the file stamped `stamp` (read as `theirs`) becomes
    /// the schedule, as one undo step (Undo brings yours back, unsaved), and it is saved:
    /// it is what the file holds.
    pub fn load_disk(&mut self, stamp: DiskStamp, theirs: Schedule) -> Result<(), String> {
        let shared = Arc::new(theirs.clone());
        self.edit(|x| {
            *x = theirs;
            Ok(())
        })?;
        self.took_disk(stamp, Some(shared));
        self.saved_rev = self.rev;
        self.safety.pending_since = None;
        self.safety.confirm_overwrite = false;
        Ok(())
    }

    /// The user put off deciding about the outside version with this hash.
    pub fn put_off(&mut self, hash: u64) {
        self.safety.put_off = Some(hash);
    }

    /// The hash of the outside version put off, if any.
    pub fn put_off_hash(&self) -> Option<u64> {
        self.safety.put_off
    }

    /// Whether the schedule's file holds something other than what this session last read
    /// or wrote (another program changed it). Gone, or never read here, is not a change.
    pub fn changed_on_disk(&self) -> bool {
        match (self.path.as_deref(), self.disk) {
            (Some(path), Some(stamp)) => stamp.check(path) == DiskCheck::Changed,
            _ => false,
        }
    }

    // ---- autosave and recovery (src/safety.rs, src/safety_app.rs)

    /// Names this schedule's recovery copy.
    pub fn recovery_key(&self) -> &str {
        &self.safety.recovery_key
    }

    /// Notes a change at `at`. Every edit, undo and redo does, at now.
    pub fn note_change(&mut self, at: Instant) {
        self.safety.last_change = Some(at);
        self.safety.pending_since.get_or_insert(at);
    }

    /// Whether autosave should run at `now`: there are unsaved changes not yet in a
    /// recovery copy, and the schedule has been quiet long enough (or changing too long).
    pub fn autosave_due(&self, now: Instant) -> bool {
        self.is_dirty()
            && self.safety.recovery_rev != Some(self.rev)
            && self
                .safety
                .last_change
                .is_some_and(|t| safety::autosave_due(now, t, self.safety.pending_since))
    }

    /// Whether autosave may write into the schedule's own file: it has one, it isn't a
    /// recovered schedule waiting for its first Save, and its file didn't change outside.
    pub fn can_autosave_to_file(&self) -> bool {
        self.path.is_some() && !self.safety.confirm_overwrite && !self.safety.conflict
    }

    /// A recovered schedule whose first Save into its original file should ask.
    pub fn needs_overwrite_confirm(&self) -> bool {
        self.path.is_some() && self.safety.confirm_overwrite
    }

    /// The file changed outside the app: autosave into it is paused.
    pub fn in_conflict(&self) -> bool {
        self.safety.conflict
    }

    pub fn set_conflict(&mut self, on: bool) {
        self.safety.conflict = on;
    }

    /// The schedule as file text, for a recovery copy.
    pub fn file_text(&self) -> String {
        method_schedule_format::to_string(&self.schedule)
    }

    /// The current state is now in the recovery copy.
    pub fn recovery_written(&mut self) {
        self.safety.recovery_rev = Some(self.rev);
        self.safety.pending_since = None;
    }

    /// Whether the recovery copy holds exactly the current state.
    pub fn recovery_current(&self) -> bool {
        self.safety.recovery_rev == Some(self.rev)
    }

    /// There is a recovery copy, but nothing unsaved any more (saved, or undone back to
    /// the saved state): the copy should go.
    pub fn recovery_stale(&self) -> bool {
        self.safety.recovery_rev.is_some() && !self.is_dirty()
    }

    /// Whether a recovery copy was written and not removed.
    pub fn has_recovery(&self) -> bool {
        self.safety.recovery_rev.is_some()
    }

    /// The recovery copy was deleted.
    pub fn recovery_removed(&mut self) {
        self.safety.recovery_rev = None;
    }

    /// Whether this session has backed up the previous version of `path` (the first write
    /// into a file in a session always does).
    pub fn has_backed_up(&self, path: &Path) -> bool {
        self.safety.backed_up.as_deref() == Some(path)
    }

    pub fn backed_up(&mut self, path: &Path) {
        self.safety.backed_up = Some(path.to_path_buf());
    }

    /// Adds a script's scenes to the schedule as one edit: one sheet per scene at the end of
    /// the Boneyard, speaking characters as Cast elements. An untouched untitled schedule
    /// takes the script's title. Returns how many scenes were added.
    pub fn import_script(&mut self, script: &fountain::ImportedScript) -> Result<usize, String> {
        let rename = self.is_untouched() && self.schedule.title == UNTITLED;
        self.edit(|s| {
            if rename {
                if let Some(t) = script.title.as_deref().map(str::trim) {
                    if !t.is_empty() {
                        s.title = t.to_string();
                    }
                }
            }
            let cast = s.cast_or_add();
            s.import_script(script, cast)
                .map(|added| added.len())
                .map_err(|e| e.to_string())
        })
    }
}

/// What Import Script says after adding `added` scenes (feature IMP-05: the count shown).
pub fn import_notice(added: usize) -> String {
    match added {
        0 => "The script has no scene headings, so nothing was added.".to_string(),
        1 => "Added 1 scene to the Boneyard.".to_string(),
        n => format!("Added {n} scenes to the Boneyard."),
    }
}

/// Reads a schedule file strictly (size-limited, UTF-8, the format's reader and
/// `Schedule::check`), with the bytes it held. Errors start with the file's name.
pub fn read_schedule(path: &Path) -> Result<(Schedule, Vec<u8>), String> {
    let bytes = safety::read_limited(path).map_err(|e| format!("{}: {e}", file_name(path)))?;
    let schedule = parse_schedule(&bytes, path)?;
    Ok((schedule, bytes))
}

/// Reads `bytes`, just read from `path`, as a schedule (UTF-8, the format's reader and
/// `Schedule::check`). Errors start with the file's name.
pub fn parse_schedule(bytes: &[u8], path: &Path) -> Result<Schedule, String> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| format!("{}: not a text file", file_name(path)))?;
    method_schedule_format::from_str(text).map_err(|e| format!("{}: {e}", file_name(path)))
}

/// Reads a screenplay: Final Draft (`.fdx`) by its extension, Fountain otherwise, at the
/// default lines per page.
pub fn read_script(path: &Path) -> Result<fountain::ImportedScript, String> {
    read_script_with(path, fountain::LINES_PER_PAGE)
}

/// Reads a screenplay as [`read_script`] does, a Fountain one at `lines_per_page` lines a
/// page (the user's setting, decision 195). A Final Draft file keeps its own lengths.
pub fn read_script_with(
    path: &Path,
    lines_per_page: u32,
) -> Result<fountain::ImportedScript, String> {
    let bytes = safety::read_limited(path).map_err(|e| format!("{}: {e}", file_name(path)))?;
    let text = String::from_utf8(bytes)
        .map_err(|_| format!("{}: not a UTF-8 text file", file_name(path)))?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let is_fdx = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("fdx"));
    if is_fdx {
        fdx::parse(text).map_err(|e| format!("{}: {e}", file_name(path)))
    } else {
        Ok(fountain::parse_with(text, lines_per_page))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::safety::tests::temp_dir;
    use std::fs;

    fn harbor_light() -> fountain::ImportedScript {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/scripts/harbor-light.fountain");
        read_script(&path).unwrap()
    }

    #[test]
    fn the_board_shown_survives_edits_and_falls_back_when_gone() {
        let mut s = Session::new();
        let first = s.schedule().boards[0].id;
        assert_eq!(s.shown_board(), Some(first));
        let rain = s
            .edit(|x| crate::board::add_board(x, "Rain plan", None))
            .unwrap();
        let rain = BoardId(rain);
        s.show_board(rain).unwrap();
        assert!(s.show_board(BoardId(9999)).is_err());
        // Showing a board is not an edit: nothing to undo or save.
        assert_eq!(s.shown_board(), Some(rain));
        s.edit(|x| crate::board::rename_board(x, rain.0, "Wet"))
            .unwrap();
        assert_eq!(s.shown_board(), Some(rain));
        s.undo();
        s.undo();
        // The board is undone away: the first one shows.
        assert_eq!(s.shown_board(), Some(first));
        s.redo();
        assert_eq!(
            s.shown_board(),
            Some(rain),
            "redo brings it back, still chosen"
        );
    }

    #[test]
    fn edits_mark_dirty_and_undo_redo_walk_back_and_forth() {
        let mut s = Session::new();
        assert!(!s.is_dirty() && !s.can_undo() && s.is_untouched());
        s.edit(|x| {
            x.title = "One".into();
            Ok(())
        })
        .unwrap();
        s.edit(|x| {
            x.title = "Two".into();
            Ok(())
        })
        .unwrap();
        assert!(s.is_dirty() && s.can_undo() && !s.can_redo());
        assert!(s.undo());
        assert_eq!(s.schedule().title, "One");
        assert!(s.undo());
        assert_eq!(s.schedule().title, UNTITLED);
        assert!(!s.is_dirty(), "back where it started: nothing unsaved");
        assert!(!s.undo(), "nothing more to undo");
        assert!(s.redo());
        assert_eq!(s.schedule().title, "One");
        assert!(s.is_dirty());
        // A new edit drops the redo steps.
        s.edit(|x| {
            x.title = "Three".into();
            Ok(())
        })
        .unwrap();
        assert!(!s.can_redo());
    }

    #[test]
    fn refused_and_empty_edits_change_nothing() {
        let mut s = Session::new();
        let err = s.edit(|x| -> Result<(), String> {
            x.title = "Half done".into();
            Err("no".into())
        });
        assert_eq!(err, Err("no".to_string()));
        assert_eq!(s.schedule().title, UNTITLED);
        s.edit(|_| Ok(())).unwrap();
        assert!(!s.is_dirty() && !s.can_undo(), "a no-op edit isn't a step");
        // An edit that would break the schedule is refused whole.
        let broken = s.edit(|x| {
            let twin = x.categories[0].clone();
            x.categories.push(twin);
            Ok(())
        });
        assert!(broken.unwrap_err().starts_with("That change was refused: "));
        assert_eq!(
            s.schedule().categories.len(),
            1 + method_schedule_core::model::DEFAULT_CATEGORIES.len()
        );
        assert!(!s.can_undo());
    }

    #[test]
    fn autosave_timing_follows_edits_undo_and_recovery_copies() {
        use std::time::Duration;
        let mut s = Session::new();
        let t0 = Instant::now();
        assert!(
            !s.autosave_due(t0 + Duration::from_secs(600)),
            "nothing unsaved"
        );
        s.edit(|x| {
            x.title = "One".into();
            Ok(())
        })
        .unwrap();
        let soon = Instant::now();
        assert!(!s.autosave_due(soon), "still typing");
        assert!(s.autosave_due(soon + safety::AUTOSAVE_QUIET));
        s.recovery_written();
        assert!(s.recovery_current() && s.has_recovery());
        assert!(
            !s.autosave_due(soon + safety::AUTOSAVE_MAX_WAIT),
            "already in the recovery copy"
        );
        // Undo is a change like any other; back at the saved state the copy is stale.
        s.undo();
        assert!(!s.is_dirty() && s.recovery_stale());
        s.recovery_removed();
        assert!(!s.has_recovery());
        // No file: autosave can only write a recovery copy.
        assert!(!s.can_autosave_to_file());
    }

    #[test]
    fn a_recovered_schedule_is_unsaved_and_waits_to_replace_its_file() {
        let text = method_schedule_format::to_string(&Schedule::new("Recovered"));
        let s =
            Session::recovered(&text, Some(PathBuf::from("/x/film.mtsc")), "k-1".into()).unwrap();
        assert!(s.is_dirty() && s.recovery_current());
        assert_eq!(s.recovery_key(), "k-1");
        assert!(s.needs_overwrite_confirm() && !s.can_autosave_to_file());
        let untitled = Session::recovered(&text, None, "k-2".into()).unwrap();
        assert!(!untitled.needs_overwrite_confirm());
        assert!(Session::recovered("not a schedule", None, "k-3".into()).is_err());
    }

    #[test]
    fn undo_history_is_capped() {
        let mut s = Session::new();
        for i in 0..(UNDO_LIMIT + 25) {
            s.edit(|x| {
                x.title = format!("t{i}");
                Ok(())
            })
            .unwrap();
        }
        let mut n = 0;
        while s.undo() {
            n += 1;
        }
        assert_eq!(n, UNDO_LIMIT);
        assert_eq!(s.schedule().title, "t24");
        assert!(
            s.is_dirty(),
            "the oldest steps are gone, so this isn't the saved state"
        );
    }

    #[test]
    fn save_open_round_trip_and_dirty_flag() {
        let dir = temp_dir("roundtrip");
        let path = dir.join("harbor.mtsc");
        let mut s = Session::new();
        let added = s.import_script(&harbor_light()).unwrap();
        assert!(added > 3);
        assert_eq!(
            s.schedule().title,
            "Harbor Light",
            "untouched: takes the title"
        );
        assert!(s.is_dirty());
        s.save_to(&path).unwrap();
        assert!(!s.is_dirty());
        assert_eq!(s.display_name(), "harbor.mtsc");
        // Undo after saving is an unsaved change; redo returns to the saved state.
        s.undo();
        assert!(s.is_dirty());
        s.redo();
        assert!(!s.is_dirty());

        let opened = Session::open(&path).unwrap();
        assert_eq!(opened.schedule(), s.schedule());
        assert!(!opened.is_dirty() && !opened.can_undo());
        // A second import adds to the schedule and keeps its title.
        let mut again = opened;
        again.import_script(&harbor_light()).unwrap();
        assert_eq!(again.schedule().title, "Harbor Light");
        assert_eq!(again.schedule().sheets.len(), 2 * added);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_file_changed_on_disk_is_not_overwritten() {
        let dir = temp_dir("guard");
        let path = dir.join("guard.mtsc");
        let mut s = Session::new();
        s.save_to(&path).unwrap();
        s.edit(|x| {
            x.title = "Mine".into();
            Ok(())
        })
        .unwrap();
        // Saving over our own last write is fine.
        s.save_to(&path).unwrap();

        // Another program writes the file.
        let mut theirs = Session::open(&path).unwrap();
        theirs
            .edit(|x| {
                x.title = "Theirs".into();
                Ok(())
            })
            .unwrap();
        let their_text = method_schedule_format::to_string(theirs.schedule());
        fs::write(&path, &their_text).unwrap();

        s.edit(|x| {
            x.title = "Mine again".into();
            Ok(())
        })
        .unwrap();
        let err = s.save_to(&path).unwrap_err();
        assert!(matches!(err, SaveError::ChangedOnDisk(_)));
        assert!(err.to_string().contains("Save As"), "{err}");
        assert_eq!(fs::read_to_string(&path).unwrap(), their_text, "untouched");
        assert!(s.is_dirty(), "still unsaved");

        // Save As to another file works, and that file is guarded from then on.
        let other = dir.join("mine.mtsc");
        s.save_to(&other).unwrap();
        assert!(!s.is_dirty());
        assert_eq!(
            Session::open(&other).unwrap().schedule().title,
            "Mine again"
        );

        // A file that was deleted is simply written again.
        fs::remove_file(&other).unwrap();
        s.save_to(&other).unwrap();
        assert!(other.exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn opening_refuses_what_isnt_a_schedule() {
        let dir = temp_dir("refuse");
        let path = dir.join("not.mtsc");
        fs::write(&path, "{\"hello\": 1}").unwrap();
        let err = Session::open(&path).err().unwrap();
        assert!(err.starts_with("not.mtsc: "), "{err}");
        assert!(Session::open(&dir.join("missing.mtsc")).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn scripts_are_read_by_extension() {
        let dir = temp_dir("scripts");
        let fdx = dir.join("tiny.FDX");
        fs::write(
            &fdx,
            "<?xml version=\"1.0\"?><FinalDraft><Content>\
             <Paragraph Type=\"Scene Heading\"><Text>INT. LOFT - NIGHT</Text></Paragraph>\
             </Content></FinalDraft>",
        )
        .unwrap();
        let script = read_script(&fdx).unwrap();
        assert_eq!(script.scenes.len(), 1);
        assert_eq!(script.scenes[0].set, "LOFT");
        // Not XML: refused as a Final Draft file, with its name.
        fs::write(&fdx, "INT. LOFT - NIGHT").unwrap();
        assert!(read_script(&fdx).unwrap_err().starts_with("tiny.FDX: "));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn an_import_adds_to_the_end_of_every_boneyard_as_one_step() {
        use method_schedule_core::fountain;
        use method_schedule_core::model::{Place, StripKind};
        let script = fountain::parse(
            "Title: Tide Tables\n\nINT. LOFT - NIGHT\n\nANA\nHi.\n\nEXT. PIER - DAY\n\nBO\nNo.\n",
        );
        let mut s = Session::new();
        // A new schedule has no sheets, so nothing blank stays behind (IMP-08), and an
        // untouched one takes the script's title.
        assert!(s.schedule().sheets.is_empty());
        assert_eq!(s.import_script(&script), Ok(2));
        assert_eq!(s.schedule().title, "Tide Tables");
        // Schedule one, add a second board, rename: a second import keeps the title and
        // lands after what is there, in every Boneyard.
        s.edit(|x| {
            let (b, sb) = (x.boards[0].id, x.boards[0].sub_boards[0].id);
            let first = x.boards[0].boneyard[0].id;
            x.move_strips(
                b,
                &[first],
                Place {
                    sub_board: Some(sb),
                    index: 0,
                },
            )
            .map_err(|e| e.to_string())?;
            x.add_board("Rain", None);
            x.title = "Mine".to_string();
            Ok(())
        })
        .unwrap();
        let before = s.schedule().clone();
        assert_eq!(s.import_script(&script), Ok(2));
        let sheet = |k: &StripKind| match k {
            StripKind::Sheet(h) => Some(*h),
            _ => None,
        };
        let new: Vec<_> = s.schedule().sheets[2..].iter().map(|h| h.id).collect();
        for b in &s.schedule().boards {
            let tail: Vec<_> = b.boneyard[b.boneyard.len() - 2..]
                .iter()
                .filter_map(|st| sheet(&st.kind))
                .collect();
            assert_eq!(tail, new, "{}", b.name);
        }
        assert_eq!(s.schedule().title, "Mine");
        // The same names are the same cast, not new elements.
        assert_eq!(s.schedule().elements.len(), 2);
        s.undo();
        assert_eq!(s.schedule(), &before);
        // What the app says.
        assert_eq!(
            import_notice(0),
            "The script has no scene headings, so nothing was added."
        );
        assert_eq!(import_notice(1), "Added 1 scene to the Boneyard.");
        assert_eq!(import_notice(5), "Added 5 scenes to the Boneyard.");
    }
}
