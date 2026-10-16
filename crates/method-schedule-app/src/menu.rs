//! The native menu bar, from one table.
//!
//! Every command item carries an id and keys from the page's shortcut list,
//! `ui/js/shortcuts.js` (a test below fails if the two drift apart). Choosing one sends its
//! id to the page as a [`EVENT`] event, and `main.js` runs the same handler its key press
//! runs, so a menu click and a key always do the same thing. [`QUIT`] is answered in Rust
//! instead (it asks about unsaved changes first, `app.rs`).
//!
//! Who answers a key: on macOS the page sees a key first and the menu's key equivalent
//! fires only if the page didn't handle it, so `main.js` leaves app keys (Save, Undo) to
//! the menu there. On Windows and Linux `main.js` handles them itself and drops a menu
//! event that repeats a key it just handled. Method Budget's `src/menu.rs`, scaled down.

use tauri::menu::{AboutMetadata, CheckMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Emitter, Manager, Runtime};

/// The event the page listens for; the payload is the item's id.
pub const EVENT: &str = "menu";
/// Quit, after asking about unsaved changes (`app.rs`).
pub const QUIT: &str = "app.quit";
/// File > Open Recent, a submenu filled from the recent list (src/safety_app.rs).
pub const OPEN_RECENT: &str = "file.open_recent";
/// File > Open Recent > one file: `file.recent.<n>`, answered in the page (which runs
/// `open_recent` with `n`).
pub const RECENT_PREFIX: &str = "file.recent.";
/// File > Open Recent > Clear Menu, answered in Rust.
pub const CLEAR_RECENT: &str = "file.recent.clear";
/// File > Autosave, a check item answered in Rust.
pub const AUTOSAVE: &str = "file.autosave";
/// File > Close Window (mod+w, the reference app's Close): answered in Rust, which closes
/// the focused window through the unsaved-changes question (decisions.md, entry 79).
pub const CLOSE: &str = "file.close";

/// One menu entry. Keys use the shortcut list's syntax (`mod+shift+s`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entry {
    Item {
        id: &'static str,
        label: &'static str,
        mac: Option<&'static str>,
        other: Option<&'static str>,
    },
    /// A check item answered in Rust (src/safety_app.rs).
    Check {
        id: &'static str,
        label: &'static str,
    },
    /// File > Open Recent: a submenu whose items src/safety_app.rs fills in.
    OpenRecent,
    Separator,
    /// Stock items: they act on text fields natively.
    Cut,
    Copy,
    Paste,
    SelectAll,
}

use Entry::Separator as SEP;

const fn item(id: &'static str, label: &'static str, keys: &'static str) -> Entry {
    Entry::Item {
        id,
        label,
        mac: Some(keys),
        other: Some(keys),
    }
}

const fn per_platform(
    id: &'static str,
    label: &'static str,
    mac: &'static str,
    other: &'static str,
) -> Entry {
    Entry::Item {
        id,
        label,
        mac: Some(mac),
        other: Some(other),
    }
}

const fn plain(id: &'static str, label: &'static str) -> Entry {
    Entry::Item {
        id,
        label,
        mac: None,
        other: None,
    }
}

pub const FILE: &[Entry] = &[
    item("file.new", "New", "mod+n"),
    item("file.new_window", "New Window", "mod+shift+n"),
    item("file.open", "Open\u{2026}", "mod+o"),
    plain("file.open_window", "Open in New Window\u{2026}"),
    Entry::OpenRecent,
    plain("file.new_from_template", "New from Template\u{2026}"),
    SEP,
    item("file.save", "Save", "mod+s"),
    item("file.save_as", "Save As\u{2026}", "mod+shift+s"),
    Entry::Check {
        id: AUTOSAVE,
        label: "Autosave",
    },
    plain("file.save_template", "Save as Template\u{2026}"),
    plain("file.revert_backup", "Revert to Backup\u{2026}"),
    SEP,
    plain("file.import_script", "Import Script\u{2026}"),
    SEP,
    plain("file.what_changed", "What Changed\u{2026}"),
    plain("file.what_changed_saved", "What Changed Since Saved"),
    plain("file.merge", "Merge Changes From\u{2026}"),
    SEP,
    item("file.export", "Export\u{2026}", "mod+e"),
    plain("file.export_budget", "Export for Budget\u{2026}"),
    SEP,
    item("file.print", "Print\u{2026}", "mod+p"),
    plain("file.save_pdf", "Save as PDF\u{2026}"),
    SEP,
    item(CLOSE, "Close Window", "mod+w"),
];

/// Windows and Linux end the File menu with Quit; macOS has it in the app menu.
pub const FILE_QUIT: &[Entry] = &[SEP, item(QUIT, "Quit", "mod+q")];

/// Settings (mod+, as on every Mac app): in the app menu on macOS, at the end of the Edit
/// menu elsewhere (decisions.md, entry 79). The page answers it.
pub const SETTINGS: &[Entry] = &[item("app.settings", "Settings\u{2026}", "mod+,")];

pub const EDIT: &[Entry] = &[
    item("edit.undo", "Undo", "mod+z"),
    per_platform("edit.redo", "Redo", "mod+shift+z", "mod+y"),
    SEP,
    Entry::Cut,
    Entry::Copy,
    Entry::Paste,
    Entry::SelectAll,
    SEP,
    // Provisional keys (decisions.md, 2, 26 and 34).
    item("edit.find", "Find\u{2026}", "mod+f"),
    item("edit.replace", "Replace\u{2026}", "mod+shift+f"),
    item("edit.find_next", "Find Next", "mod+g"),
    item("edit.find_previous", "Find Previous", "mod+shift+g"),
    SEP,
    item("edit.explain", "Explain This Count", "mod+i"),
    SEP,
    // The older reference app's element linking and red flag entry keys (note 10).
    item("edit.links", "Linked Elements\u{2026}", "mod+l"),
    item("edit.add_red_flag", "Add Red Flag\u{2026}", "mod+k"),
    plain("edit.red_flag_types", "Red Flag Types\u{2026}"),
];

/// The tabs. Keys provisional until checked against the reference app (decisions.md, 2).
pub const VIEW: &[Entry] = &[
    // The reference app's keys for Board, Breakdown and Calendar (behavior note 21); its
    // Reports key is Print here (decisions.md, entry 106).
    item("view.board", "Board", "mod+u"),
    item("view.breakdown", "Breakdown", "mod+."),
    item("view.elements", "Elements", "mod+3"),
    item("view.dood", "Day out of days", "mod+4"),
    item("view.calendar", "Calendar", "mod+shift+d"),
    item("view.reports", "Reports", "mod+6"),
    item("view.conflicts", "Conflicts", "mod+7"),
    item("view.red_flags", "Red Flags", "mod+8"),
    SEP,
    // Split screen (decisions.md, entry 80): a pane beside the board.
    item(
        "view.split_breakdown",
        "Breakdown Beside Board",
        "mod+shift+2",
    ),
    item("view.split_board", "Board Beside Board", "mod+shift+1"),
    item(
        "view.split_calendar",
        "Calendar Beside Board",
        "mod+shift+5",
    ),
    plain("view.split_images", "Images Beside Board"),
    // The sheet's images beside the Breakdown tab (behavior note 22).
    plain("view.split_breakdown_images", "Images Beside Breakdown"),
    item("view.single", "Single View", "mod+shift+0"),
];

/// The Board menu: banners, duplicating and merging strips, sorting, boards and
/// sub-boards (decisions.md, 29 to 32, 37 and 44). The reference app's manual names Cmd B
/// for a new banner, and its New Banner goes above the selected strip (behavior note 17),
/// so Cmd B inserts above; the other keys are provisional.
pub const BOARD: &[Entry] = &[
    item("board.banner_above", "Insert Banner Above\u{2026}", "mod+b"),
    item(
        "board.banner_below",
        "Insert Banner Below\u{2026}",
        "mod+shift+b",
    ),
    plain("board.edit_banner", "Edit Banner\u{2026}"),
    SEP,
    // Cmd comma inserted a day break in the reference app's older versions (inventory,
    // section 14), but it is the Mac's Settings key: ours is mod+d (decisions.md,
    // entry 79). Provisional.
    item("board.break_below", "Insert Day Break Below", "mod+d"),
    item("board.break_above", "Insert Day Break Above", "mod+alt+d"),
    plain("board.remove_breaks", "Remove All Day Breaks"),
    plain("board.continue_scene", "Continue Scene on Next Day"),
    SEP,
    plain("board.duplicate", "Duplicate Strip"),
    plain("board.merge", "Merge Strips\u{2026}"),
    SEP,
    plain("board.sort", "Sort Strips\u{2026}"),
    plain("board.layout", "Strip Layout\u{2026}"),
    plain("board.layouts", "Layouts\u{2026}"),
    plain("board.colors", "Strip Colors\u{2026}"),
    plain("board.day_look", "Day Breaks\u{2026}"),
    plain("board.show", "Show on Board\u{2026}"),
    SEP,
    plain("board.print", "Print Board\u{2026}"),
    plain("board.save_pdf", "Save Board as PDF\u{2026}"),
    SEP,
    plain("boards.new_board", "New Board\u{2026}"),
    plain("boards.duplicate", "Duplicate Board"),
    plain("boards.new_sub", "New Sub-board\u{2026}"),
    plain("boards.manage", "Boards and Sub-boards\u{2026}"),
];

/// The Breakdown menu: the sheet shown on the Breakdown tab (decisions.md, 37). Its
/// commands show the Breakdown tab first.
pub const BREAKDOWN: &[Entry] = &[
    plain("breakdown.new_sheet", "New Sheet"),
    plain("breakdown.duplicate_sheet", "Duplicate Sheet"),
    SEP,
    plain("breakdown.delete_sheet", "Delete Sheet\u{2026}"),
    SEP,
    // The older reference app's find scene key (note 10); its find sheet key (mod+g) is
    // Find Next here.
    item("breakdown.go_scene", "Go to Scene\u{2026}", "mod+j"),
    plain("breakdown.go_sheet", "Go to Sheet Number\u{2026}"),
    SEP,
    plain("breakdown.field_order", "Breakdown Designer\u{2026}"),
];

/// The Help menu: the shortcut sheet, made from ui/js/shortcuts.js (decisions.md,
/// entry 79), the user guide and the sample schedule (entries 198 and 199, src/help.rs),
/// and the suite's items (entry 212): What's New, Report a Bug, Licenses and Notices and
/// the suite's website. On Windows and Linux About follows them.
pub const HELP: &[Entry] = &[
    plain("help.guide", "User Guide"),
    item("help.shortcuts", "Keyboard Shortcuts", "mod+/"),
    plain("help.whats_new", "What\u{2019}s New"),
    SEP,
    plain("help.open_sample", "Open Sample"),
    SEP,
    plain("help.report_bug", "Report a Bug\u{2026}"),
    plain("help.licenses", "Licenses and Notices"),
    plain("help.suite", "methodproduction.com"),
    SEP,
    plain(crate::updater::CHECK_ITEM, "Check for Updates\u{2026}"),
    Entry::Check {
        id: crate::updater::AUTO_ITEM,
        label: "Check for Updates Automatically",
    },
];

/// A shortcut-list key spec as a Tauri accelerator: `mod+shift+s` is `CmdOrCtrl+Shift+S`.
pub fn accelerator(spec: &str) -> String {
    spec.split('+')
        .map(|part| match part {
            "mod" => "CmdOrCtrl".to_string(),
            "ctrl" => "Ctrl".to_string(),
            "shift" => "Shift".to_string(),
            "alt" => "Alt".to_string(),
            key => key.to_uppercase(),
        })
        .collect::<Vec<_>>()
        .join("+")
}

fn append<R: Runtime>(
    app: &AppHandle<R>,
    menu: &Submenu<R>,
    entries: &[Entry],
) -> tauri::Result<()> {
    for entry in entries {
        match *entry {
            Entry::Item {
                id,
                label,
                mac,
                other,
            } => {
                let keys = if cfg!(target_os = "macos") {
                    mac
                } else {
                    other
                };
                let accel = keys.map(accelerator);
                menu.append(&MenuItem::with_id(app, id, label, true, accel.as_deref())?)?;
            }
            Entry::Check { id, label } => {
                // On until the settings say otherwise; src/safety_app.rs (Autosave) and
                // src/updater.rs (automatic update checks) keep each in step.
                let on = if id == crate::updater::AUTO_ITEM {
                    app.try_state::<crate::updater::Updates>()
                        .is_none_or(|u| u.automatic())
                } else {
                    app.try_state::<crate::safety_app::Safety>()
                        .is_none_or(|s| s.autosave_on())
                };
                menu.append(&CheckMenuItem::with_id(
                    app,
                    id,
                    label,
                    true,
                    on,
                    None::<&str>,
                )?)?;
            }
            Entry::OpenRecent => {
                let recent = Submenu::with_id(app, OPEN_RECENT, "Open Recent", true)?;
                recent.append(&MenuItem::with_id(
                    app,
                    CLEAR_RECENT,
                    "Clear Menu",
                    false,
                    None::<&str>,
                )?)?;
                menu.append(&recent)?;
            }
            Entry::Separator => menu.append(&PredefinedMenuItem::separator(app)?)?,
            Entry::Cut => menu.append(&PredefinedMenuItem::cut(app, None)?)?,
            Entry::Copy => menu.append(&PredefinedMenuItem::copy(app, None)?)?,
            Entry::Paste => menu.append(&PredefinedMenuItem::paste(app, None)?)?,
            Entry::SelectAll => menu.append(&PredefinedMenuItem::select_all(app, None)?)?,
        }
    }
    Ok(())
}

fn submenu<R: Runtime>(
    app: &AppHandle<R>,
    title: &str,
    entries: &[Entry],
) -> tauri::Result<Submenu<R>> {
    let menu = Submenu::new(app, title, true)?;
    append(app, &menu, entries)?;
    Ok(menu)
}

/// Builds the menu bar. `&` marks the Alt-key letter on Windows and Linux (macOS ignores
/// it).
/// The Plan menu: the features beyond the reference app (src/plan.rs; decisions.md,
/// IMP-04 and BYD-01 to BYD-09). No keys.
pub const PLAN: &[Entry] = &[
    plain("plan.import_tags", "Import Script with Tags\u{2026}"),
    plain("plan.revise", "Import Script Revision\u{2026}"),
    SEP,
    plain("plan.timeline", "Day Timeline\u{2026}"),
    plain("plan.locations", "Locations\u{2026}"),
    plain("plan.rest", "Turnarounds and Rest\u{2026}"),
    plain("plan.print_call_sheet", "Print Call Sheet\u{2026}"),
    plain("plan.call_sheet", "Export Call Sheet Data\u{2026}"),
    SEP,
    plain("plan.locks", "Schedule Locks\u{2026}"),
    SEP,
    plain("plan.pin", "Pin Strips"),
    plain("plan.unpin", "Unpin Strips"),
    plain("plan.suggest", "Suggest Strip Order\u{2026}"),
];

/// About Method Schedule: the version, the website and the licence (decision 212).
fn about<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<PredefinedMenuItem<R>> {
    let name = app.package_info().name.clone();
    PredefinedMenuItem::about(
        app,
        Some(&format!("About {name}")),
        Some(AboutMetadata {
            name: Some(name.clone()),
            version: Some(app.package_info().version.to_string()),
            // A link on Windows and Linux; macOS shows the credits line.
            website: Some(crate::help::WEBSITE.into()),
            website_label: Some("methodschedule.com".into()),
            credits: Some("methodschedule.com".into()),
            copyright: Some(
                "Copyright (c) 2026 Method Production Suite Team and the Method Schedule contributors"
                    .into(),
            ),
            license: Some("MIT OR Apache-2.0".into()),
            ..Default::default()
        }),
    )
}

pub fn build<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let menu = Menu::new(app)?;

    #[cfg(target_os = "macos")]
    {
        // The app menu. Our own Quit item goes through the unsaved-changes guard; the
        // stock one would quit without asking.
        let name = app.package_info().name.clone();
        let app_menu = Submenu::new(app, &name, true)?;
        app_menu.append(&about(app)?)?;
        app_menu.append(&PredefinedMenuItem::separator(app)?)?;
        append(app, &app_menu, SETTINGS)?;
        app_menu.append(&PredefinedMenuItem::separator(app)?)?;
        app_menu.append(&PredefinedMenuItem::hide(
            app,
            Some(&format!("Hide {name}")),
        )?)?;
        app_menu.append(&PredefinedMenuItem::hide_others(app, None)?)?;
        app_menu.append(&PredefinedMenuItem::separator(app)?)?;
        app_menu.append(&MenuItem::with_id(
            app,
            QUIT,
            format!("Quit {name}"),
            true,
            Some(accelerator("mod+q")),
        )?)?;
        menu.append(&app_menu)?;
    }

    let file = submenu(app, "&File", FILE)?;
    if !cfg!(target_os = "macos") {
        append(app, &file, FILE_QUIT)?;
    }
    menu.append(&file)?;
    let edit = submenu(app, "&Edit", EDIT)?;
    if !cfg!(target_os = "macos") {
        append(app, &edit, &[SEP])?;
        append(app, &edit, SETTINGS)?;
    }
    menu.append(&edit)?;
    menu.append(&submenu(app, "&View", VIEW)?)?;
    menu.append(&submenu(app, "&Board", BOARD)?)?;
    menu.append(&submenu(app, "B&reakdown", BREAKDOWN)?)?;
    menu.append(&submenu(app, "&Plan", PLAN)?)?;

    #[cfg(target_os = "macos")]
    {
        let window = Submenu::with_id(app, tauri::menu::WINDOW_SUBMENU_ID, "Window", true)?;
        window.append(&PredefinedMenuItem::minimize(app, None)?)?;
        window.append(&PredefinedMenuItem::fullscreen(app, None)?)?;
        menu.append(&window)?;
    }
    // Every open window, to bring one to the front (src/windows.rs fills it in). macOS
    // lists them in its own Window menu.
    if !cfg!(target_os = "macos") {
        menu.append(&Submenu::with_id(
            app,
            crate::windows::WINDOW_LIST,
            "&Window",
            true,
        )?)?;
    }
    let help = Submenu::with_id(app, tauri::menu::HELP_SUBMENU_ID, "&Help", true)?;
    append(app, &help, HELP)?;
    if !cfg!(target_os = "macos") {
        help.append(&PredefinedMenuItem::separator(app)?)?;
        help.append(&about(app)?)?;
    }
    menu.append(&help)?;
    Ok(menu)
}

/// Sends a menu item's id to the page of the window in front (decisions.md, entry 86).
pub fn handle<R: Runtime>(app: &AppHandle<R>, id: &str) {
    let label = crate::windows::focused(app)
        .map(|w| w.label().to_string())
        .unwrap_or_else(|| crate::app::MAIN.to_string());
    let _ = app.emit_to(label.as_str(), EVENT, id);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHORTCUTS_JS: &str = include_str!("../ui/js/shortcuts.js");

    /// Every command item in the menus, check items as items without keys.
    fn items() -> Vec<Entry> {
        [
            FILE, FILE_QUIT, SETTINGS, EDIT, VIEW, BOARD, BREAKDOWN, PLAN, HELP,
        ]
        .iter()
        .flat_map(|m| m.iter().copied())
        .filter_map(|e| match e {
            Entry::Item { .. } => Some(e),
            Entry::Check { id, label } => Some(plain(id, label)),
            _ => None,
        })
        .collect()
    }

    /// The text after `key: '` (or `key: ['`) up to the next quote.
    fn quoted_after<'a>(line: &'a str, key: &str) -> Option<&'a str> {
        let at = line.find(&format!("{key}: "))? + key.len() + 2;
        let rest = line[at..].trim_start_matches('[');
        let rest = rest.strip_prefix('\'')?;
        Some(&rest[..rest.find('\'')?])
    }

    /// (mac keys, other keys, scope) for an id in shortcuts.js; the first alternative when
    /// there are several.
    fn js_entry(id: &str) -> Option<(Option<&str>, Option<&str>, &str)> {
        let needle = format!("{{ id: '{id}',");
        let line = SHORTCUTS_JS.lines().find(|l| l.contains(&needle))?;
        let scope = quoted_after(line, "scope").expect("every entry has a scope");
        if let Some(k) = quoted_after(line, "keys") {
            return Some((Some(k), Some(k), scope));
        }
        Some((
            quoted_after(line, "mac"),
            quoted_after(line, "other"),
            scope,
        ))
    }

    #[test]
    fn menu_keys_match_the_shortcut_list() {
        for e in items() {
            let Entry::Item { id, mac, other, .. } = e else {
                unreachable!()
            };
            let (js_mac, js_other, scope) =
                js_entry(id).unwrap_or_else(|| panic!("{id} is missing from shortcuts.js"));
            assert_eq!(mac, js_mac, "macOS keys for {id}");
            assert_eq!(other, js_other, "Windows/Linux keys for {id}");
            assert!(matches!(scope, "app" | "menu"), "{id} is {scope}");
        }
    }

    #[test]
    fn the_banner_key_inserts_above_first_in_the_menu() {
        // Behavior note 17: the reference app's New Banner (Cmd B) goes above the
        // selected strip.
        let first = BOARD.iter().find_map(|e| match e {
            Entry::Item { id, mac, .. } => Some((*id, *mac)),
            _ => None,
        });
        assert_eq!(first, Some(("board.banner_above", Some("mod+b"))));
    }

    #[test]
    fn every_app_shortcut_has_a_menu_item() {
        // App keys only work through the menu on macOS, so each one needs an item.
        let ids: Vec<&str> = items()
            .into_iter()
            .map(|e| match e {
                Entry::Item { id, .. } => id,
                _ => unreachable!(),
            })
            .collect();
        let mut checked = 0;
        for line in SHORTCUTS_JS.lines() {
            let Some(id) = quoted_after(line, "{ id") else {
                continue;
            };
            let scope = quoted_after(line, "scope").unwrap();
            if matches!(scope, "app" | "menu") {
                assert!(ids.contains(&id), "{id} ({scope}) has no menu item");
                checked += 1;
            }
        }
        assert_eq!(checked, ids.len(), "shortcuts.js lists every menu item");
    }

    #[test]
    fn no_two_items_share_keys_and_labels_follow_the_voice() {
        for pick in [0, 1] {
            let mut seen = std::collections::BTreeMap::new();
            for e in items() {
                let Entry::Item {
                    id,
                    label,
                    mac,
                    other,
                } = e
                else {
                    unreachable!()
                };
                assert!(
                    !label.contains('\u{2014}') && !label.contains('!'),
                    "{label}"
                );
                if let Some(k) = [mac, other][pick] {
                    if let Some(prev) = seen.insert(k, id) {
                        panic!("{prev} and {id} both use {k}");
                    }
                }
            }
        }
    }

    #[test]
    fn recent_ids_stay_clear_of_the_table() {
        // Recent files are numbered under their own prefix, which no table id uses.
        for e in items() {
            let Entry::Item { id, .. } = e else {
                unreachable!()
            };
            assert!(!id.starts_with(RECENT_PREFIX), "{id}");
        }
        assert!(CLEAR_RECENT.starts_with(RECENT_PREFIX));
        assert!(FILE.contains(&Entry::OpenRecent));
    }

    #[test]
    fn accelerators() {
        assert_eq!(accelerator("mod+shift+s"), "CmdOrCtrl+Shift+S");
        assert_eq!(accelerator("mod+z"), "CmdOrCtrl+Z");
        assert_eq!(accelerator("mod+."), "CmdOrCtrl+.");
        assert_eq!(accelerator("mod+alt+d"), "CmdOrCtrl+Alt+D");
    }

    #[test]
    fn every_accelerator_names_a_key_the_menu_takes() {
        // The keys the native menu's accelerator parser reads (letters, digits and these
        // marks); anything else would fail when the menu is built.
        for e in items() {
            let Entry::Item { id, mac, other, .. } = e else {
                unreachable!()
            };
            for spec in [mac, other].into_iter().flatten() {
                let key = spec.rsplit('+').next().unwrap();
                let ok = (key.len() == 1
                    && key
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b",./[]".contains(&b)))
                    || matches!(key, "up" | "down" | "enter" | "backspace" | "delete");
                assert!(ok, "{id}: {spec}");
            }
        }
    }

    #[test]
    fn the_reference_keys_are_here() {
        // The keys the reference app documents, where its command exists here (inventory
        // section 14, behavior note 21): New, Open, Save, Save As, Close, Undo, Redo, the
        // banner and the tabs it has keys for.
        let keys: std::collections::BTreeMap<&str, &str> = items()
            .into_iter()
            .filter_map(|e| match e {
                Entry::Item { id, mac, .. } => Some((id, mac?)),
                _ => None,
            })
            .collect();
        for (id, k) in [
            ("file.new", "mod+n"),
            ("file.open", "mod+o"),
            ("file.save", "mod+s"),
            ("file.save_as", "mod+shift+s"),
            (CLOSE, "mod+w"),
            ("edit.undo", "mod+z"),
            ("edit.redo", "mod+shift+z"),
            ("board.banner_above", "mod+b"),
            ("board.break_below", "mod+d"),
            ("app.settings", "mod+,"),
            ("breakdown.go_scene", "mod+j"),
            ("edit.links", "mod+l"),
            ("edit.add_red_flag", "mod+k"),
            ("file.print", "mod+p"),
            ("edit.find", "mod+f"),
            // Behavior note 21 (decisions.md, entry 106): the Schedule menu's keys where
            // they don't clash with a platform convention. Its Reports key, mod+p, is
            // Print (above), so Reports keeps ours.
            ("view.board", "mod+u"),
            ("view.breakdown", "mod+."),
            ("view.calendar", "mod+shift+d"),
            ("view.reports", "mod+6"),
            ("board.break_above", "mod+alt+d"),
        ] {
            assert_eq!(keys.get(id), Some(&k), "{id}");
        }
    }
}
