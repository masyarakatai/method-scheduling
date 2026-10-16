//! The layout library in the app (decisions.md, entry 78): named layouts of scene strips,
//! day breaks and shoot day strips, picked per board and by the one-line report, saved
//! and loaded as layout files. The edits are plain functions on a [`Schedule`] (one undo
//! step each through `Session::edit`); the two file commands show their dialogs from
//! Rust, so the page never names a path.

use std::path::{Path, PathBuf};

use serde::Deserialize;
use tauri::{Runtime, WebviewWindow};
use tauri_plugin_dialog::DialogExt;

use method_schedule_core::layouts::{LayoutBody, LayoutId, LayoutKind};
use method_schedule_core::model::{DayLook, Error};
use method_schedule_core::Schedule;

use crate::app::{finish, AppState, CmdResult};
use crate::board::{DayLookInput, EditResult, LayoutColumnInput};
use crate::docs::Doc;
use crate::export::ExportOutcome;

/// A layout's body as the page sends it: a strip layout's columns, or a day break or
/// shoot day layout's switches and text.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutInput {
    #[serde(default)]
    pub columns: Option<Vec<LayoutColumnInput>>,
    /// A strip layout's text size step (decision 87).
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub look: Option<DayLookInput>,
    /// A strip layout's mark switches (decision 136).
    #[serde(default)]
    pub marks: Option<crate::board::MarksInput>,
}

fn say(e: Error) -> String {
    match e {
        Error::EmptyName => "Type a name for the layout.".into(),
        Error::DuplicateName(n) => format!("There is already a layout named {n:?}."),
        Error::NoSuchBoard(_) => "That board is no longer in the schedule.".into(),
        Error::Broken(why) => {
            let mut s = why;
            if let Some(c) = s.get(..1) {
                s = c.to_uppercase() + &s[1..];
            }
            if !s.ends_with('.') {
                s.push('.');
            }
            s
        }
        other => crate::elements::say(other),
    }
}

fn kind(name: &str) -> EditResult<LayoutKind> {
    LayoutKind::from_name(name).ok_or_else(|| format!("{name:?} is not a layout type."))
}

fn layout_id(s: &Schedule, id: u32) -> EditResult<LayoutId> {
    s.layouts
        .get(LayoutId(id))
        .map(|l| l.id)
        .ok_or_else(|| "That layout is no longer in the schedule.".to_string())
}

/// A named layout's day look: a board's, without the shoot day strip switch (a board's
/// own setting; picking a shoot day layout turns it on).
fn look(input: &DayLookInput) -> EditResult<DayLook> {
    Ok(DayLook {
        shoot_day_strip: false,
        ..crate::board::day_look(input)?
    })
}

/// A body of type `kind` from what the page sent.
fn body(kind: LayoutKind, input: &LayoutInput) -> EditResult<LayoutBody> {
    match kind {
        LayoutKind::Strips => {
            let cols = input
                .columns
                .as_deref()
                .ok_or("Choose the fields the strips show.")?;
            Ok(LayoutBody::Strips(crate::board::strip_layout_with(
                cols,
                input.size.as_deref().unwrap_or(""),
                input.marks.unwrap_or_default(),
            )?))
        }
        LayoutKind::DayBreaks => Ok(LayoutBody::DayBreaks(look(
            input.look.as_ref().ok_or("Choose what day breaks show.")?,
        )?)),
        LayoutKind::ShootDay => Ok(LayoutBody::ShootDay(look(
            input
                .look
                .as_ref()
                .ok_or("Choose what the shoot day strip shows.")?,
        )?)),
    }
}

/// Adds a named layout of type `kind`; with `board`, that board picks it. Returns its id.
pub fn add_named(
    s: &mut Schedule,
    name: &str,
    kind_name: &str,
    input: &LayoutInput,
    board: Option<u32>,
) -> EditResult<u32> {
    let k = kind(kind_name)?;
    let b = body(k, input)?;
    let id = s.add_layout(name, b).map_err(say)?;
    if let Some(board) = board {
        let board = crate::board::board_id(s, board)?;
        s.use_layout(board, k, Some(id)).map_err(say)?;
    }
    Ok(id.0)
}

/// Changes what a named layout lays out (every board that picked it follows).
pub fn save_named(s: &mut Schedule, layout: u32, input: &LayoutInput) -> EditResult {
    let id = layout_id(s, layout)?;
    let k = s.layouts.get(id).expect("layout_id checked it").body.kind();
    let b = body(k, input)?;
    s.set_layout_body(id, b).map_err(say)
}

pub fn rename_named(s: &mut Schedule, layout: u32, name: &str) -> EditResult {
    let id = layout_id(s, layout)?;
    s.rename_layout(id, name).map_err(say)
}

pub fn move_named(s: &mut Schedule, layout: u32, to: usize) -> EditResult {
    let id = layout_id(s, layout)?;
    s.move_layout(id, to).map_err(say)
}

/// Deletes a named layout; boards that picked it go back to their own.
pub fn delete_named(s: &mut Schedule, layout: u32) -> EditResult {
    let id = layout_id(s, layout)?;
    s.delete_layout(id).map_err(say)
}

/// Board `board` picks layout `layout` of type `kind` (none: its own).
pub fn pick_layout(
    s: &mut Schedule,
    board: u32,
    kind_name: &str,
    layout: Option<u32>,
) -> EditResult {
    let board = crate::board::board_id(s, board)?;
    let k = kind(kind_name)?;
    let id = layout.map(|l| layout_id(s, l)).transpose()?;
    s.use_layout(board, k, id).map_err(say)
}

/// The one-line report picks strip layout `layout` (none: the board's).
pub fn pick_one_line(s: &mut Schedule, layout: Option<u32>) -> EditResult {
    let id = layout.map(|l| layout_id(s, l)).transpose()?;
    s.use_one_line_layout(id).map_err(say)
}

/// The text of a layout file holding `layouts` (all when empty), and how many.
pub fn layouts_file(s: &Schedule, layouts: &[u32]) -> EditResult<(String, usize)> {
    let picked: Vec<_> = if layouts.is_empty() {
        s.layouts.layouts.clone()
    } else {
        let mut out = Vec::new();
        for &id in layouts {
            out.push(
                s.layouts
                    .get(LayoutId(id))
                    .cloned()
                    .ok_or("That layout is no longer in the schedule.")?,
            );
        }
        out
    };
    if picked.is_empty() {
        return Err(
            "There are no layouts to save. Save a strip layout as a named layout first."
                .to_string(),
        );
    }
    let n = picked.len();
    Ok((method_schedule_format::layouts_to_string(s, &picked), n))
}

/// Adds the layouts of a layout file's text. Returns how many.
pub fn load_layouts(s: &mut Schedule, text: &str) -> EditResult<usize> {
    let read = method_schedule_format::layouts_from_str(s, text)
        .map_err(|e| format!("That isn't a layout file Method Schedule can read: {e}."))?;
    if read.is_empty() {
        return Err("That layout file holds no layouts.".to_string());
    }
    s.import_layouts(read).map(|ids| ids.len()).map_err(say)
}

// ---------------------------------------------------------------------------
// Commands

fn edit_with<R: Runtime>(
    window: &WebviewWindow<R>,
    state: &AppState,
    f: impl FnOnce(&mut Schedule) -> EditResult,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(f)?;
    finish(window, &s, None)
}

/// Adds a named layout (from the Strip Layout or Day Breaks dialog), and with `board` picks
/// it there, as one undo step.
#[tauri::command(rename_all = "snake_case")]
pub fn add_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    name: String,
    kind: String,
    layout: LayoutInput,
    board: Option<u32>,
) -> CmdResult {
    edit_with(&window, &state, |s| {
        add_named(s, &name, &kind, &layout, board).map(|_| ())
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    id: u32,
    layout: LayoutInput,
) -> CmdResult {
    edit_with(&window, &state, |s| save_named(s, id, &layout))
}

#[tauri::command(rename_all = "snake_case")]
pub fn rename_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    id: u32,
    name: String,
) -> CmdResult {
    edit_with(&window, &state, |s| rename_named(s, id, &name))
}

#[tauri::command(rename_all = "snake_case")]
pub fn move_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    id: u32,
    to: usize,
) -> CmdResult {
    edit_with(&window, &state, |s| move_named(s, id, to))
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_layout<R: Runtime>(window: WebviewWindow<R>, state: Doc, id: u32) -> CmdResult {
    edit_with(&window, &state, |s| delete_named(s, id))
}

#[tauri::command(rename_all = "snake_case")]
pub fn use_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    board: u32,
    kind: String,
    id: Option<u32>,
) -> CmdResult {
    edit_with(&window, &state, |s| pick_layout(s, board, &kind, id))
}

#[tauri::command(rename_all = "snake_case")]
pub fn use_one_line_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    id: Option<u32>,
) -> CmdResult {
    edit_with(&window, &state, |s| pick_one_line(s, id))
}

const EXT: &str = "mtlayouts";

/// Save Layouts: the named layouts `ids` (all when empty) into a layout file where a Save
/// dialog says. Never replaces the open schedule's file.
#[tauri::command(rename_all = "snake_case")]
pub async fn export_layouts<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    ids: Vec<u32>,
) -> CmdResult<ExportOutcome> {
    let (text, dir, stem, open) = {
        let s = state.lock();
        let (text, _) = layouts_file(s.schedule(), &ids)?;
        let path = s.path().map(PathBuf::from);
        let dir = path.as_deref().and_then(Path::parent).map(PathBuf::from);
        let stem = crate::app::safe_file_stem(&s.schedule().title);
        (text, dir, stem, path)
    };
    let mut dialog = window
        .dialog()
        .file()
        .set_title("Save layouts")
        .add_filter("Method Schedule layouts", &[EXT, "json"])
        .set_file_name(format!("{stem} - Layouts.{EXT}"))
        .set_parent(&window);
    if let Some(d) = dir {
        dialog = dialog.set_directory(d);
    }
    let Some(picked) = dialog.blocking_save_file() else {
        return Ok(ExportOutcome::default());
    };
    let path = crate::export::with_extension(picked.into_path().map_err(|e| e.to_string())?, EXT);
    let file = crate::export::write_export(open.as_deref(), &path, text.as_bytes())?;
    Ok(ExportOutcome {
        done: true,
        file: Some(file),
    })
}

/// Load Layouts: adds the layouts of a layout file an Open dialog picks, as one undo
/// step. `None` when cancelled.
#[tauri::command(rename_all = "snake_case")]
pub async fn import_layouts<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
) -> CmdResult<Option<crate::view::View>> {
    let dir = state
        .lock()
        .path()
        .and_then(Path::parent)
        .map(PathBuf::from);
    let mut dialog = window
        .dialog()
        .file()
        .set_title("Load layouts")
        .add_filter("Method Schedule layouts", &[EXT, "json"])
        .set_parent(&window);
    if let Some(d) = dir {
        dialog = dialog.set_directory(d);
    }
    let Some(picked) = dialog.blocking_pick_file() else {
        return Ok(None);
    };
    let path = picked.into_path().map_err(|e| e.to_string())?;
    let bytes =
        crate::safety::read_limited(&path).map_err(|e| format!("Couldn't read that file: {e}."))?;
    let text = String::from_utf8(bytes)
        .map_err(|_| "That isn't a layout file Method Schedule can read.".to_string())?;
    let mut s = state.lock();
    let n = s.edit(|x| load_layouts(x, &text))?;
    let what = if n == 1 {
        "1 layout".to_string()
    } else {
        format!("{n} layouts")
    };
    finish(&window, &s, Some(format!("Loaded {what}."))).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::Session;

    fn col(field: &str, width: &str, second_line: bool) -> LayoutColumnInput {
        LayoutColumnInput {
            field: field.into(),
            width: width.into(),
            line: if second_line { 2 } else { 1 },
            span: 1,
            align: String::new(),
            bold: false,
            italic: false,
            text: String::new(),
            category: None,
            board_ids: false,
            rule_after: false,
            rule_under: false,
        }
    }

    #[test]
    fn named_layouts_are_added_picked_saved_and_loaded() {
        let mut s = Session::new();
        let b = s.schedule().boards[0].id.0;
        let two = LayoutInput {
            columns: Some(vec![
                col("scene", "narrow", false),
                col("set", "wide", false),
                col("synopsis", "widest", true),
            ]),
            size: Some("small".into()),
            look: None,
            marks: None,
        };
        let id = s
            .edit(|x| add_named(x, "Two lines", "strips", &two, Some(b)))
            .unwrap();
        let v = crate::view::view(&s, None).unwrap();
        assert_eq!(v.board.uses.strips, Some(id));
        assert_eq!(v.board.layout.len(), 3);
        assert_eq!(v.board.layout[2].line, 2);
        // Its text size goes with it (decisions.md, entry 87).
        assert_eq!(v.board.layout_size, "small");
        assert_eq!(v.board.own_layout_size, "normal");
        assert_eq!(v.layouts[0].size, "small");
        assert_eq!(v.board.own_layout.len(), 6, "its own stays");
        // Refused: a name taken, no field on line 1, a body of the wrong type.
        assert!(s
            .edit(|x| add_named(x, "two lines", "strips", &two, None))
            .is_err());
        let all_second = LayoutInput {
            columns: Some(vec![col("scene", "narrow", true)]),
            size: None,
            look: None,
            marks: None,
        };
        let e = s
            .edit(|x| add_named(x, "Odd", "strips", &all_second, None))
            .unwrap_err();
        assert!(e.ends_with('.'), "{e}");
        assert!(s
            .edit(|x| add_named(x, "Odd", "panels", &two, None))
            .is_err());
        // A day break layout picked by the board; the shoot day strip from a layout.
        let night = LayoutInput {
            columns: None,
            size: None,
            look: Some(DayLookInput {
                sunset: true,
                text: "Night".into(),
                ..DayLookInput::default()
            }),
            marks: None,
        };
        let n = s
            .edit(|x| add_named(x, "Night", "day_breaks", &night, Some(b)))
            .unwrap();
        let sd = s
            .edit(|x| add_named(x, "Shoot day", "shoot_day", &night, None))
            .unwrap();
        s.edit(|x| pick_layout(x, b, "shoot_day", Some(sd)))
            .unwrap();
        assert!(s.edit(|x| pick_layout(x, b, "strips", Some(n))).is_err());
        let v = crate::view::view(&s, None).unwrap();
        assert!(v.board.day_look.sunset && v.board.day_look.shoot_day_strip);
        assert_eq!(v.board.shoot_text, "Night");
        // The one-line report picks its own.
        s.edit(|x| pick_one_line(x, None)).unwrap();
        // Save and load as a file: names that clash get a number.
        let (text, count) = layouts_file(s.schedule(), &[]).unwrap();
        assert_eq!(count, 3);
        let loaded = s.edit(|x| load_layouts(x, &text)).unwrap();
        assert_eq!(loaded, 3);
        assert!(s
            .schedule()
            .layouts
            .layouts
            .iter()
            .any(|l| l.name == "Night 2"));
        assert!(s
            .edit(|x| load_layouts(x, "{\"format\": \"method-schedule\"}"))
            .is_err());
        // Deleting a picked layout sends the board back to its own.
        s.edit(|x| delete_named(x, id)).unwrap();
        let v = crate::view::view(&s, None).unwrap();
        assert_eq!(v.board.uses.strips, None);
        assert_eq!(v.board.layout.len(), 6);
        s.schedule().check().unwrap();
    }
}
