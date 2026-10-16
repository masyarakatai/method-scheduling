//! The Plan menu: the features beyond the reference app (decisions.md, IMP-04 and BYD-01
//! to BYD-09). The engine does the work (`method_schedule_core::{plan, revise, timeline,
//! solar, rest, drift, optimize, callsheet}`); this reads what the page typed, turns the
//! results into what ui/js/plan.js draws, and makes every change one undo step through
//! `Session::edit`.
//!
//! Security: script and save paths come only from native dialogs shown here. A picked
//! revision is kept here by token until it is applied or replaced; the page never names
//! a file. Typed values (times, degrees, offsets, hours) are read strictly in Rust.

use std::sync::Mutex;

use serde::Serialize;
use tauri::{Manager, Runtime, WebviewWindow};
use tauri_plugin_dialog::DialogExt;

use method_schedule_core::callsheet::call_sheet;
use method_schedule_core::drift::{drift, lock, unlock, WorkDay};
use method_schedule_core::fountain::ImportedScript;
use method_schedule_core::model::{BoardId, CategoryId, StripId, SubBoardId};
use method_schedule_core::optimize::{apply as apply_order_to, suggest, Score};
use method_schedule_core::plan::{
    self as p, clock_text, degrees_text, hours_text, offset_text, parse_clock,
    parse_daylight_saving, parse_degrees, parse_hours, parse_offset, GeoLocation,
};
use method_schedule_core::rest::rest_report;
use method_schedule_core::revise::{import_tagged, revise, revision_text};
use method_schedule_core::solar::Sun;
use method_schedule_core::timeline::{
    past_wrap_in, push_past_wrap as push_strips, scene_name, timelines, EntryKind,
};
use method_schedule_core::{fdx, Schedule};

use crate::app::{finish, CmdResult};
use crate::docs::Doc;
use crate::export::ExportOutcome;
use crate::session;
use crate::view::{sub_ref, View};

fn ids(s: &Schedule, sub: u32) -> Result<(BoardId, SubBoardId), String> {
    let (b, sb) = sub_ref(s, sub)?;
    Ok((b.id, sb.id))
}

fn upper(e: impl ToString) -> String {
    let t = e.to_string();
    let mut c = t.chars();
    match c.next() {
        Some(f) => {
            let mut out: String = f.to_uppercase().chain(c).collect();
            if !out.ends_with('.') {
                out.push('.');
            }
            out
        }
        None => String::new(),
    }
}

// ---------------------------------------------------------------------------
// Scripts: tags and revisions

/// A revision picked and previewed, waiting for Apply.
struct Pending {
    token: u64,
    script: ImportedScript,
    tags: bool,
}

static PENDING: Mutex<Option<Pending>> = Mutex::new(None);
static NEXT_TOKEN: Mutex<u64> = Mutex::new(1);

/// A script, with tags (`.fdx` only) or without; a Fountain one at `lines` a page.
fn read_script(path: &std::path::Path, tags: bool, lines: u32) -> Result<ImportedScript, String> {
    let fdx_file = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("fdx"));
    if tags && !fdx_file {
        return Err("Production tags come only from Final Draft (.fdx) files.".to_string());
    }
    if !tags {
        return session::read_script_with(path, lines);
    }
    let bytes = crate::safety::read_limited(path).map_err(|e| e.to_string())?;
    let text = String::from_utf8(bytes).map_err(|_| "The script is not UTF-8 text.".to_string())?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    fdx::parse_with_tags(text).map_err(upper)
}

/// The user's Fountain lines per page (Settings, decision 195).
fn fountain_lines<R: Runtime>(window: &WebviewWindow<R>) -> u32 {
    window
        .app_handle()
        .state::<crate::safety_app::Safety>()
        .prefs()
        .fountain_lines
}

fn pick_script<R: Runtime>(
    window: &WebviewWindow<R>,
    tags: bool,
    title: &str,
) -> Option<std::path::PathBuf> {
    let exts: &[&str] = if tags { &["fdx"] } else { &["fountain", "fdx"] };
    window
        .dialog()
        .file()
        .set_title(title)
        .add_filter(
            if tags {
                "Final Draft script"
            } else {
                "Screenplay"
            },
            exts,
        )
        .set_parent(window)
        .blocking_pick_file()
        .and_then(|f| f.into_path().ok())
}

/// Import Script with Tags: an `.fdx` file's scenes, speaking characters and production
/// tags, into the Boneyard as one undo step. `None` when cancelled.
#[tauri::command]
pub async fn import_script_tags<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
) -> CmdResult<Option<View>> {
    let Some(path) = pick_script(&window, true, "Import script with tags") else {
        return Ok(None);
    };
    let script = read_script(&path, true, fountain_lines(&window))?;
    let tags: usize = script.scenes.iter().map(|s| s.tags.len()).sum();
    let mut s = state.lock();
    let added = s.edit(|x| {
        let cast = x.cast_or_add();
        import_tagged(x, &script, cast)
            .map(|v| v.len())
            .map_err(upper)
    })?;
    let notice = format!(
        "Added {added} {} to the Boneyard, with {tags} production {}.",
        if added == 1 { "scene" } else { "scenes" },
        if tags == 1 { "tag" } else { "tags" }
    );
    finish(&window, &s, Some(notice)).map(Some)
}

#[derive(Clone, Debug, Serialize)]
pub struct RevisionDto {
    pub token: u64,
    pub file: String,
    pub matched: usize,
    pub changed: usize,
    pub added: usize,
    pub omitted: usize,
    /// The report, one line each.
    pub lines: Vec<String>,
}

/// What a revision would do to `s`, worked out on a copy.
pub fn revision_preview(
    s: &Schedule,
    script: &ImportedScript,
    tags: bool,
) -> Result<(usize, usize, usize, usize, Vec<String>), String> {
    let mut copy = s.clone();
    let cast = copy.cast_or_add();
    let r = revise(&mut copy, script, cast, tags).map_err(upper)?;
    let lines = revision_text(&copy, &r)
        .lines()
        .map(str::to_string)
        .collect();
    Ok((
        r.matched.len(),
        r.changed().count(),
        r.added.len(),
        r.omitted.len(),
        lines,
    ))
}

/// Import Script Revision: the Open dialog, then the preview. `None` when cancelled.
#[tauri::command]
pub async fn revise_pick<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    tags: bool,
) -> CmdResult<Option<RevisionDto>> {
    let Some(path) = pick_script(&window, tags, "Import script revision") else {
        return Ok(None);
    };
    let script = read_script(&path, tags, fountain_lines(&window))?;
    let (matched, changed, added, omitted, lines) =
        revision_preview(state.lock().schedule(), &script, tags)?;
    let token = {
        let mut n = NEXT_TOKEN.lock().unwrap_or_else(|e| e.into_inner());
        *n += 1;
        *n
    };
    *PENDING.lock().unwrap_or_else(|e| e.into_inner()) = Some(Pending {
        token,
        script,
        tags,
    });
    Ok(Some(RevisionDto {
        token,
        file: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
        matched,
        changed,
        added,
        omitted,
        lines,
    }))
}

/// Applies the previewed revision as one undo step.
#[tauri::command]
pub fn revise_apply<R: Runtime>(window: WebviewWindow<R>, state: Doc, token: u64) -> CmdResult {
    let pending = PENDING
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .take()
        .filter(|x| x.token == token)
        .ok_or("That revision is no longer waiting; pick the script again.")?;
    let mut s = state.lock();
    let first = s.edit(|x| {
        let cast = x.cast_or_add();
        let r = revise(x, &pending.script, cast, pending.tags).map_err(upper)?;
        Ok(revision_text(x, &r)
            .lines()
            .next()
            .unwrap_or_default()
            .to_string())
    })?;
    finish(
        &window,
        &s,
        Some(format!("Revision merged: {first} Undo takes it back.")),
    )
}

// ---------------------------------------------------------------------------
// The timeline and call times

#[derive(Clone, Debug, Serialize)]
pub struct EntryDto {
    pub strip: u32,
    pub kind: &'static str,
    pub text: String,
    pub start: String,
    pub end: String,
    pub untimed: bool,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DayDto {
    pub number: u32,
    pub date: Option<String>,
    /// The day break that closes the day: call times are kept by it.
    pub day: u32,
    pub call: String,
    pub wrap: String,
    pub length: String,
    pub sun: String,
    pub entries: Vec<EntryDto>,
    pub warnings: Vec<String>,
    /// What runs past the wrap, in words (decision 192); empty when nothing does.
    pub past_wrap: String,
    /// The strips that would move to the next day, as `push_past_wrap` wants them back;
    /// empty when nothing runs past or this is the last day.
    pub push: Vec<u32>,
    /// The day they would move to.
    pub next: Option<u32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TimelineDto {
    pub sub_board: u32,
    pub sub_board_name: String,
    pub long_day: String,
    pub days: Vec<DayDto>,
    /// The day holding the strip asked about (its index in `days`), else 0.
    pub focus: usize,
}

fn sun_text(sun: Option<&method_schedule_core::timeline::DaySun>) -> String {
    match sun {
        None => String::new(),
        Some(x) => match x.sun {
            Sun::Times { rise, set } => format!(
                "Sunrise {}, sunset {} at {}",
                clock_text(rise),
                clock_text(set),
                x.location
            ),
            Sun::AlwaysUp => format!("The sun stays up at {}", x.location),
            Sun::AlwaysDown => format!("The sun does not rise at {}", x.location),
        },
    }
}

pub fn timeline_view(s: &Schedule, sub: u32, strip: Option<u32>) -> Result<TimelineDto, String> {
    let (b, sb) = ids(s, sub)?;
    let name = sub_ref(s, sub)?.1.name.clone();
    let mut focus = 0;
    let mut days = Vec::new();
    let all = timelines(s, b, sb).map_err(upper)?;
    for (i, t) in all.iter().enumerate() {
        if strip.is_some_and(|x| t.day.0 == x || t.entries.iter().any(|e| e.strip.0 == x)) {
            focus = i;
        }
        let past = past_wrap_in(s, b, sb, &all, t.number).map_err(upper)?;
        let push = if past.next.is_some() {
            past.strips.iter().map(|x| x.0).collect()
        } else {
            Vec::new()
        };
        days.push(DayDto {
            number: t.number,
            date: t.date.map(|d| d.to_string()),
            day: t.day.0,
            call: t.call.map_or(String::new(), |c| clock_text(c as i32)),
            wrap: if t.call.is_some() {
                t.time_text(t.wrap)
            } else {
                String::new()
            },
            length: hours_text(t.length),
            sun: sun_text(t.sun.as_ref()),
            entries: t
                .entries
                .iter()
                .map(|e| {
                    let (kind, text) = match &e.kind {
                        EntryKind::Scene(id) => (
                            "scene",
                            s.sheet(*id).map_or(String::new(), |h| {
                                format!(
                                    "{} {} {} {}",
                                    scene_name(h),
                                    h.int_ext,
                                    h.set,
                                    h.time_of_day
                                )
                            }),
                        ),
                        EntryKind::Banner(text) => ("banner", text.clone()),
                    };
                    EntryDto {
                        strip: e.strip.0,
                        kind,
                        text,
                        start: t.time_text(e.start),
                        end: t.time_text(e.end),
                        untimed: e.untimed,
                        warnings: e.warnings.clone(),
                    }
                })
                .collect(),
            warnings: t.warnings.clone(),
            past_wrap: if past.strips.is_empty() {
                String::new()
            } else {
                past.text()
            },
            push,
            next: past.next,
        });
    }
    Ok(TimelineDto {
        sub_board: sub,
        sub_board_name: name,
        long_day: hours_text(s.plan.long_day()),
        days,
        focus,
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn timeline_data(state: Doc, sub_board: u32, strip: Option<u32>) -> CmdResult<TimelineDto> {
    timeline_view(state.lock().schedule(), sub_board, strip)
}

/// Moves what runs past shoot day `day`'s wrap to the start of the next day (decision
/// 192), refused when it is no longer the `strips` previewed. Returns how many moved.
pub fn push_past(s: &mut Schedule, sub: u32, day: u32, strips: &[u32]) -> Result<usize, String> {
    let (b, sb) = ids(s, sub)?;
    let seen: Vec<StripId> = strips.iter().copied().map(StripId).collect();
    push_strips(s, b, sb, day, &seen).map_err(upper)
}

#[tauri::command(rename_all = "snake_case")]
pub fn push_past_wrap<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    sub_board: u32,
    day: u32,
    strips: Vec<u32>,
) -> CmdResult {
    if strips.len() > 100_000 {
        return Err("Too many strips at once.".to_string());
    }
    let mut s = state.lock();
    let n = s.edit(|x| push_past(x, sub_board, day, &strips))?;
    let what = if n == 1 {
        "1 strip".to_string()
    } else {
        format!("{n} strips")
    };
    finish(
        &window,
        &s,
        Some(format!(
            "Moved {what} to the start of day {}. Undo puts them back.",
            day + 1
        )),
    )
}

/// Sets the call time of the day closed by `day` (a day break), or clears it when empty.
pub fn set_call(s: &mut Schedule, day: u32, call: &str) -> Result<(), String> {
    let call = call.trim();
    let minutes = if call.is_empty() {
        None
    } else {
        Some(parse_clock(call).map_err(upper)?)
    };
    p::set_call_time(s, StripId(day), minutes).map_err(upper)
}

#[tauri::command]
pub fn set_call_time<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    day: u32,
    call: String,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| set_call(x, day, &call))?;
    finish(&window, &s, None)
}

/// The long day limit as typed (`12`, `10:30`); empty for the default.
pub fn set_long(s: &mut Schedule, value: &str) -> Result<(), String> {
    let v = value.trim();
    let m = if v.is_empty() {
        None
    } else {
        Some(parse_hours(v).map_err(upper)?)
    };
    p::set_long_day(s, m).map_err(upper)
}

#[tauri::command]
pub fn set_long_day<R: Runtime>(window: WebviewWindow<R>, state: Doc, value: String) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| set_long(x, &value))?;
    finish(&window, &s, None)
}

// ---------------------------------------------------------------------------
// Locations

#[derive(Clone, Debug, Serialize)]
pub struct LocationDto {
    pub name: String,
    pub sheets: usize,
    pub latitude: String,
    pub longitude: String,
    pub utc_offset: String,
    /// The daylight saving rule's name (`us_canada`, `eu`), empty for none (decision 193).
    pub daylight_saving: String,
}

pub fn locations_view(s: &Schedule) -> Vec<LocationDto> {
    let mut names: Vec<String> = Vec::new();
    for h in &s.sheets {
        let n = h.location.trim();
        if !n.is_empty() && !names.iter().any(|x| x.eq_ignore_ascii_case(n)) {
            names.push(n.to_string());
        }
    }
    for g in &s.plan.locations {
        if !names.iter().any(|x| x.eq_ignore_ascii_case(&g.name)) {
            names.push(g.name.clone());
        }
    }
    names
        .into_iter()
        .map(|n| {
            let g = s.plan.location(&n);
            LocationDto {
                sheets: s
                    .sheets
                    .iter()
                    .filter(|h| h.location.trim().eq_ignore_ascii_case(&n))
                    .count(),
                latitude: g.map_or(String::new(), |g| degrees_text(g.latitude)),
                longitude: g.map_or(String::new(), |g| degrees_text(g.longitude)),
                utc_offset: g.map_or(String::new(), |g| offset_text(g.utc_offset)),
                daylight_saving: g
                    .and_then(|g| g.daylight_saving)
                    .map_or(String::new(), |r| r.name().to_string()),
                name: n,
            }
        })
        .collect()
}

#[tauri::command]
pub fn locations_data(state: Doc) -> CmdResult<Vec<LocationDto>> {
    Ok(locations_view(state.lock().schedule()))
}

/// Places `name` on the map from typed degrees and offset and a daylight saving rule
/// (`us_canada`, `eu`, empty for none); both degrees empty takes it off. An empty offset
/// is the longitude's hour.
pub fn place_location(
    s: &mut Schedule,
    name: &str,
    latitude: &str,
    longitude: &str,
    utc_offset: &str,
    daylight_saving: &str,
) -> Result<(), String> {
    if latitude.trim().is_empty() && longitude.trim().is_empty() {
        s.plan.remove_location(name);
        return Ok(());
    }
    if latitude.trim().is_empty() || longitude.trim().is_empty() {
        return Err("Type both a latitude and a longitude, or neither.".to_string());
    }
    let lat = parse_degrees(latitude, 90).map_err(upper)?;
    let lon = parse_degrees(longitude, 180).map_err(upper)?;
    let offset = if utc_offset.trim().is_empty() {
        ((i64::from(lon) * 60 / 15 / 1_000_000) as i32 / 60) * 60
    } else {
        parse_offset(utc_offset).map_err(upper)?
    };
    let daylight_saving = parse_daylight_saving(daylight_saving).map_err(upper)?;
    s.plan
        .set_location(GeoLocation {
            name: name.to_string(),
            latitude: lat,
            longitude: lon,
            utc_offset: offset,
            daylight_saving,
        })
        .map_err(upper)
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_location<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    name: String,
    latitude: String,
    longitude: String,
    utc_offset: String,
    daylight_saving: String,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| {
        place_location(
            x,
            &name,
            &latitude,
            &longitude,
            &utc_offset,
            &daylight_saving,
        )
    })?;
    finish(&window, &s, None)
}

// ---------------------------------------------------------------------------
// Turnarounds and rest

#[derive(Clone, Debug, Serialize)]
pub struct TurnaroundDto {
    pub category: u32,
    pub name: String,
    pub minimum: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RestDto {
    pub turnarounds: Vec<TurnaroundDto>,
    pub short: Vec<String>,
    pub unchecked: usize,
}

pub fn rest_view(s: &Schedule, board: u32) -> Result<RestDto, String> {
    let b = crate::view::board_ref(s, board)?.id;
    let r = rest_report(s, b).map_err(upper)?;
    Ok(RestDto {
        turnarounds: s
            .categories
            .iter()
            .map(|c| TurnaroundDto {
                category: c.id.0,
                name: c.name.clone(),
                minimum: s.plan.turnaround(c.id).map_or(String::new(), hours_text),
            })
            .collect(),
        short: r.short.into_iter().map(|x| x.text).collect(),
        unchecked: r.unchecked,
    })
}

#[tauri::command]
pub fn rest_data(state: Doc, board: u32) -> CmdResult<RestDto> {
    rest_view(state.lock().schedule(), board)
}

pub fn set_minimum(s: &mut Schedule, category: u32, value: &str) -> Result<(), String> {
    let v = value.trim();
    let m = if v.is_empty() {
        None
    } else {
        Some(parse_hours(v).map_err(upper)?)
    };
    p::set_turnaround(s, CategoryId(category), m).map_err(upper)
}

#[tauri::command]
pub fn set_turnaround<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    category: u32,
    value: String,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| set_minimum(x, category, &value))?;
    finish(&window, &s, None)
}

// ---------------------------------------------------------------------------
// Locks and drift

#[derive(Clone, Debug, Serialize)]
pub struct LockDto {
    pub name: String,
    pub made: String,
    pub scenes: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct ActorDto {
    pub name: String,
    pub board_id: String,
    pub before: usize,
    pub after: usize,
    pub added: String,
    pub dropped: String,
    pub moved: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DriftDto {
    pub lock: String,
    pub actors: Vec<ActorDto>,
    /// Actors with no change, not listed.
    pub unchanged: usize,
}

#[tauri::command]
pub fn locks_data(state: Doc, board: u32) -> CmdResult<Vec<LockDto>> {
    let s = state.lock();
    Ok(s.schedule()
        .plan
        .locks
        .iter()
        .filter(|l| l.board.0 == board)
        .map(|l| LockDto {
            name: l.name.clone(),
            made: l.made.map_or(String::new(), |d| d.to_string()),
            scenes: l.scenes.len(),
        })
        .collect())
}

#[tauri::command]
pub fn lock_schedule<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    board: u32,
    name: String,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| {
        if name.chars().count() > 200 {
            return Err("A lock's name is 200 characters at most.".to_string());
        }
        lock(
            x,
            BoardId(board),
            &name,
            method_schedule_format::exchange::local_today(),
        )
        .map_err(upper)
    })?;
    finish(
        &window,
        &s,
        Some(format!("Locked the schedule as {:?}.", name.trim())),
    )
}

#[tauri::command]
pub fn delete_lock<R: Runtime>(window: WebviewWindow<R>, state: Doc, name: String) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| unlock(x, &name).map_err(upper))?;
    finish(&window, &s, None)
}

pub fn drift_view(s: &Schedule, name: &str) -> Result<DriftDto, String> {
    let d = drift(s, name).map_err(upper)?;
    let days = |v: &[WorkDay]| v.iter().map(|w| w.text(s)).collect::<Vec<_>>().join(", ");
    let unchanged = d.actors.iter().filter(|a| a.is_empty()).count();
    Ok(DriftDto {
        lock: d.lock.clone(),
        actors: d
            .changed()
            .map(|a| ActorDto {
                name: a.name.clone(),
                board_id: a.board_id.clone().unwrap_or_default(),
                before: a.before,
                after: a.after,
                added: days(&a.added),
                dropped: days(&a.dropped),
                moved: a
                    .moved
                    .iter()
                    .map(|m| {
                        let scene = s.sheet(m.sheet).map_or_else(
                            || "A scene no longer in the schedule".to_string(),
                            scene_name,
                        );
                        format!("{scene}: {} to {}", m.from.text(s), m.to.text(s))
                    })
                    .collect(),
            })
            .collect(),
        unchanged,
    })
}

#[tauri::command]
pub fn drift_data(state: Doc, name: String) -> CmdResult<DriftDto> {
    drift_view(state.lock().schedule(), &name)
}

// ---------------------------------------------------------------------------
// Pins and the order suggestion

#[tauri::command]
pub fn pin_strips<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    strips: Vec<u32>,
    pinned: bool,
) -> CmdResult {
    if strips.len() > 100_000 {
        return Err("Too many strips at once.".to_string());
    }
    let ids: Vec<StripId> = strips.into_iter().map(StripId).collect();
    let mut s = state.lock();
    let n = s.edit(|x| {
        p::set_pinned(x, &ids, pinned).map_err(|_| "Only scene strips can be pinned.".to_string())
    })?;
    let what = if n == 1 {
        "1 strip".to_string()
    } else {
        format!("{n} strips")
    };
    let verb = if pinned { "Pinned" } else { "Unpinned" };
    finish(&window, &s, Some(format!("{verb} {what}.")))
}

#[derive(Clone, Debug, Serialize)]
pub struct ScoreRow {
    pub label: &'static str,
    pub before: u32,
    pub after: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct SuggestionDto {
    pub sub_board: u32,
    pub sub_board_name: String,
    /// Every strip of the sub-board in the suggested order: Apply sends it back.
    pub order: Vec<u32>,
    pub moved: usize,
    pub pinned: usize,
    pub groups: usize,
    pub rows: Vec<ScoreRow>,
}

fn rows(a: &Score, b: &Score) -> Vec<ScoreRow> {
    vec![
        ScoreRow {
            label: "Shoot days",
            before: a.shoot_days,
            after: b.shoot_days,
        },
        ScoreRow {
            label: "Cast work days",
            before: a.cast_days,
            after: b.cast_days,
        },
        ScoreRow {
            label: "Location changes in a day",
            before: a.location_changes,
            after: b.location_changes,
        },
        ScoreRow {
            label: "Red flag clashes",
            before: a.red_flag_clashes,
            after: b.red_flag_clashes,
        },
    ]
}

pub fn suggestion_view(s: &Schedule, sub: u32) -> Result<SuggestionDto, String> {
    let (b, sb) = ids(s, sub)?;
    let sug = suggest(s, b, sb).map_err(upper)?;
    Ok(SuggestionDto {
        sub_board: sub,
        sub_board_name: sub_ref(s, sub)?.1.name.clone(),
        order: sug.order.iter().map(|x| x.0).collect(),
        moved: sug.moved,
        pinned: sug.pinned,
        groups: sug.groups,
        rows: rows(&sug.before, &sug.after),
    })
}

#[tauri::command(rename_all = "snake_case")]
pub fn suggest_order(state: Doc, sub_board: u32) -> CmdResult<SuggestionDto> {
    suggestion_view(state.lock().schedule(), sub_board)
}

/// Puts the previewed order on the board, refused when it is no longer the suggestion
/// (the board changed since the preview).
pub fn apply_order(s: &mut Schedule, sub: u32, order: &[u32]) -> Result<usize, String> {
    let (b, sb) = ids(s, sub)?;
    let sug = suggest(s, b, sb).map_err(upper)?;
    if sug.order.iter().map(|x| x.0).collect::<Vec<_>>() != order {
        return Err("The board changed since the preview; suggest the order again.".to_string());
    }
    apply_order_to(s, &sug).map_err(upper)?;
    Ok(sug.moved)
}

#[tauri::command(rename_all = "snake_case")]
pub fn apply_suggested_order<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    sub_board: u32,
    order: Vec<u32>,
) -> CmdResult {
    let mut s = state.lock();
    let n = s.edit(|x| apply_order(x, sub_board, &order))?;
    let what = if n == 1 {
        "1 scene".to_string()
    } else {
        format!("{n} scenes")
    };
    finish(
        &window,
        &s,
        Some(format!(
            "Moved {what} into the suggested order. Undo puts them back."
        )),
    )
}

// ---------------------------------------------------------------------------
// Call sheet data

/// The call sheet file's bytes for shoot day `day` of a sub-board: JSON or CSV.
pub fn call_sheet_bytes(s: &Schedule, sub: u32, day: u32, csv: bool) -> Result<Vec<u8>, String> {
    let (b, sb) = ids(s, sub)?;
    let c = call_sheet(s, b, sb, day).map_err(upper)?;
    Ok(if csv {
        method_schedule_format::callsheet::to_csv(&c)
    } else {
        method_schedule_format::callsheet::to_json(&c).into_bytes()
    })
}

/// Export Call Sheet Data: the Save dialog, then the file. `format`: `json` or `csv`.
#[tauri::command(rename_all = "snake_case")]
pub async fn export_call_sheet<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    sub_board: u32,
    day: u32,
    format: String,
) -> CmdResult<ExportOutcome> {
    let csv = match format.as_str() {
        "csv" => true,
        "json" => false,
        _ => return Err("That call sheet format isn't one the app knows.".to_string()),
    };
    // The Save dialog opens in the schedule's folder, as every export's (decision 171).
    let (bytes, dir, stem, open) = {
        let s = state.lock();
        let bytes = call_sheet_bytes(s.schedule(), sub_board, day, csv)?;
        let (dir, stem, open) = crate::export::place(&s);
        (bytes, dir, stem, open)
    };
    let ext = if csv { "csv" } else { "json" };
    crate::export::save_dialog(
        &window,
        "Export call sheet data",
        if csv { "CSV file" } else { "JSON file" },
        ext,
        format!("{stem} - Call Sheet Day {day}.{ext}"),
        dir,
        open.as_deref(),
        &bytes,
    )
}

/// Print Call Sheet: the same data as the JSON export (docs/CALLSHEET.md), for the page to
/// lay out as a printed sheet (decision 207).
#[tauri::command(rename_all = "snake_case")]
pub fn call_sheet_data(state: Doc, sub_board: u32, day: u32) -> CmdResult<serde_json::Value> {
    let s = state.lock();
    let bytes = call_sheet_bytes(s.schedule(), sub_board, day, false)?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

/// How many shoot days a sub-board has, for the export's day picker.
#[tauri::command(rename_all = "snake_case")]
pub fn shoot_day_count(state: Doc, sub_board: u32) -> CmdResult<u32> {
    let s = state.lock();
    let (b, sb) = ids(s.schedule(), sub_board)?;
    Ok(method_schedule_core::days::days(s.schedule(), b, sb)
        .map_err(upper)?
        .days
        .len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;
    use method_schedule_core::model::{Place, StripKind};

    fn harbor() -> Session {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/scripts/harbor-light.fountain");
        let script = session::read_script(&path).unwrap();
        let mut s = Session::new();
        s.import_script(&script).unwrap();
        s.edit(|x| {
            let b = x.boards[0].id;
            let sb = x.boards[0].sub_boards[0].id;
            let ids: Vec<_> = x.boards[0].boneyard.iter().map(|st| st.id).collect();
            let at = Place {
                sub_board: Some(sb),
                index: 0,
            };
            x.move_strips(b, &ids, at).map_err(|e| e.to_string())?;
            x.set_start(b, sb, "2027-03-01".parse().ok())
                .map_err(|e| e.to_string())?;
            for i in [3, 7] {
                x.insert_strip(
                    b,
                    StripKind::DayBreak,
                    Place {
                        sub_board: Some(sb),
                        index: i,
                    },
                )
                .map_err(|e| e.to_string())?;
            }
            Ok(())
        })
        .unwrap();
        s
    }

    #[test]
    fn call_times_locations_and_the_timeline() {
        let mut s = harbor();
        let sub = s.schedule().boards[0].sub_boards[0].id.0;
        let t = timeline_view(s.schedule(), sub, None).unwrap();
        assert_eq!(t.days.len(), 2);
        assert_eq!(t.long_day, "12:00");
        let day = t.days[0].day;
        s.edit(|x| set_call(x, day, "6:30")).unwrap();
        assert!(s
            .edit(|x| set_call(x, day, "25:00"))
            .unwrap_err()
            .starts_with("\"25:00\" is not a time"));
        let t = timeline_view(s.schedule(), sub, Some(day)).unwrap();
        assert_eq!(t.days[0].call, "06:30");
        assert_eq!(t.focus, 0);
        s.edit(|x| set_call(x, day, "")).unwrap();
        assert!(s.schedule().plan.call_times.is_empty());
        s.edit(|x| set_long(x, "10:30")).unwrap();
        assert_eq!(
            timeline_view(s.schedule(), sub, None).unwrap().long_day,
            "10:30"
        );
        // Locations: placed, read back, refused half typed, taken off.
        s.edit(|x| place_location(x, "Harbor", "41.5", "-70.6", "", ""))
            .unwrap();
        let l = locations_view(s.schedule());
        let h = l.iter().find(|x| x.name == "Harbor").unwrap();
        assert_eq!(
            (h.latitude.as_str(), h.utc_offset.as_str()),
            ("41.5", "-04:00")
        );
        assert_eq!(h.daylight_saving, "");
        // A daylight saving rule (decision 193): the standard offset, an hour more in summer.
        s.edit(|x| place_location(x, "Harbor", "41.5", "-70.6", "-5", "us_canada"))
            .unwrap();
        let l = locations_view(s.schedule());
        let h = l.iter().find(|x| x.name == "Harbor").unwrap();
        assert_eq!(
            (h.utc_offset.as_str(), h.daylight_saving.as_str()),
            ("-05:00", "us_canada")
        );
        let g = s.schedule().plan.location("harbor").unwrap();
        assert_eq!(g.offset_on("2027-07-01".parse().unwrap()), -240);
        assert_eq!(g.offset_on("2027-01-01".parse().unwrap()), -300);
        assert!(s
            .edit(|x| place_location(x, "Harbor", "41.5", "-70.6", "-5", "mars"))
            .is_err());
        assert!(s
            .edit(|x| place_location(x, "Harbor", "41.5", "", "", ""))
            .is_err());
        s.edit(|x| place_location(x, "Harbor", "", "", "", ""))
            .unwrap();
        assert!(s.schedule().plan.locations.is_empty());
    }

    /// Decision 192: the timeline lists what runs past wrap, and the push is one undo step.
    #[test]
    fn what_runs_past_wrap_moves_as_one_undo_step() {
        let mut s = harbor();
        let sub = s.schedule().boards[0].sub_boards[0].id.0;
        s.edit(|x| {
            for h in &mut x.sheets {
                h.shoot_time = Some(method_schedule_core::model::Minutes(300));
            }
            Ok(())
        })
        .unwrap();
        // Day 1 has three five-hour scenes: the third ends 15:00 after the call.
        let t = timeline_view(s.schedule(), sub, None).unwrap();
        let d = &t.days[0];
        assert_eq!((d.next, d.push.len()), (Some(2), 1));
        assert!(
            d.past_wrap
                .ends_with("They would move to the start of day 2."),
            "{}",
            d.past_wrap
        );
        // The last day has nowhere to go: said, but nothing to push.
        let last = t.days.last().unwrap();
        assert_eq!(last.next, None);
        assert!(last.push.is_empty());
        let before = s.schedule().clone();
        let err = s.edit(|x| push_past(x, sub, 1, &[])).unwrap_err();
        assert!(
            err.starts_with("The day changed since the preview"),
            "{err}"
        );
        assert_eq!(s.edit(|x| push_past(x, sub, 1, &d.push)).unwrap(), 1);
        let t = timeline_view(s.schedule(), sub, None).unwrap();
        assert!(t.days[0].past_wrap.is_empty());
        assert_eq!(t.days[1].entries[0].strip, d.push[0]);
        assert!(s.undo());
        assert_eq!(*s.schedule(), before);
    }

    #[test]
    fn rest_locks_order_and_call_sheet() {
        let mut s = harbor();
        let board = s.schedule().boards[0].id.0;
        let sub = s.schedule().boards[0].sub_boards[0].id.0;
        let cast = s.schedule().cast().unwrap().0;
        s.edit(|x| set_minimum(x, cast, "12")).unwrap();
        let r = rest_view(s.schedule(), board).unwrap();
        assert_eq!(r.turnarounds[0].minimum, "12:00");
        assert!(r.unchecked > 0);
        // Lock, move a strip, drift.
        s.edit(|x| lock(x, BoardId(board), "White", None).map_err(upper))
            .unwrap();
        let d = drift_view(s.schedule(), "white").unwrap();
        assert!(d.actors.is_empty());
        let sug = suggestion_view(s.schedule(), sub).unwrap();
        assert_eq!(sug.rows[0].before, sug.rows[0].after);
        let order = sug.order.clone();
        s.edit(|x| apply_order(x, sub, &order)).unwrap();
        assert!(s.edit(|x| apply_order(x, sub, &[1, 2])).is_err());
        let bytes = call_sheet_bytes(s.schedule(), sub, 1, false).unwrap();
        assert!(String::from_utf8(bytes)
            .unwrap()
            .contains("method-schedule-callsheet"));
        assert!(call_sheet_bytes(s.schedule(), sub, 9, true).is_err());
        let v: serde_json::Value =
            serde_json::from_slice(&call_sheet_bytes(s.schedule(), sub, 1, false).unwrap())
                .unwrap();
        assert_eq!(v["format"], "method-schedule-callsheet");
        assert!(v["scenes"].is_array() && v["cast"].is_array() && v["notes"].is_array());
    }

    /// The harness's fake (harness/fake-plan.js) refuses with Rust's words: each message,
    /// less the value typed at its start and its full stop, is in the fake's source.
    #[test]
    fn the_harness_fake_refuses_in_the_same_words() {
        let fake = include_str!("../harness/fake-plan.js");
        let mut s = harbor();
        let sub = s.schedule().boards[0].sub_boards[0].id.0;
        let cast = s.schedule().cast().unwrap().0;
        let mut said = Vec::new();
        for (lat, lon, off) in [
            ("41.5", "", ""),
            ("4x", "1", ""),
            ("91", "1", ""),
            ("1.1234567", "1", ""),
            ("1", "1", "+15"),
        ] {
            said.push(
                s.edit(|x| place_location(x, "Harbor", lat, lon, off, ""))
                    .unwrap_err(),
            );
        }
        said.push(
            s.edit(|x| place_location(x, "Harbor", "1", "1", "", "mars"))
                .unwrap_err(),
        );
        for v in ["25", "x"] {
            said.push(s.edit(|x| set_long(x, v)).unwrap_err());
        }
        said.push(s.edit(|x| set_minimum(x, cast, "49")).unwrap_err());
        said.push(s.edit(|x| apply_order(x, sub, &[1, 2])).unwrap_err());
        said.push(s.edit(|x| push_past(x, sub, 1, &[1, 2])).unwrap_err());
        for m in &said {
            let rest = match m.strip_prefix('"') {
                Some(r) => &r[r.find('"').unwrap() + 1..],
                None => m.as_str(),
            };
            let rest = rest.trim_end_matches('.');
            assert!(fake.contains(rest), "the fake never says {m:?}");
        }
        assert!(fake.contains("That revision is no longer waiting; pick the script again."));
        assert!(fake.contains("Only scene strips can be pinned."));
        assert!(fake.contains("into the suggested order. Undo puts them back."));
        assert!(fake.contains("to the start of day ${day + 1}. Undo puts them back."));
        assert!(fake.contains("They would move to the start of day"));
        assert!(fake.contains("the last shoot day: add a day break after it first."));
    }

    #[test]
    fn a_revision_preview_changes_nothing() {
        let s = harbor();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/scripts/harbor-light.fountain");
        let script = session::read_script(&path).unwrap();
        let before = s.schedule().clone();
        let (matched, changed, added, omitted, lines) =
            revision_preview(s.schedule(), &script, false).unwrap();
        assert_eq!(*s.schedule(), before);
        assert_eq!((added, omitted), (0, 0));
        assert_eq!(matched, script.scenes.len());
        assert_eq!(changed, 0);
        assert!(lines[0].contains("scenes matched"));
        assert!(read_script(&path, true, 56).is_err());
    }
}
