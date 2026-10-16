//! The report options' tools (decisions.md, entry 59): the reports added with them as
//! data (`report`), the report list (`report_list`), and the settings kept in the
//! schedule: the production info, the day out of days code letters and a sub-board's
//! prep start and wrap end.

use method_schedule_core::dood::{dood_with, sort_rows, summary, Options, SortKey};
use method_schedule_core::model::{ElementId, SheetId};
use method_schedule_core::report_data::{self, element_text, Slot};
use method_schedule_core::setup::{DoodCodes, Production};
use method_schedule_core::{Date, Schedule};
use serde_json::{json, Map, Value};

use super::super::files::{edit, load, Edited};
use super::super::schema::{array, boolean, name, nullable, object, one_of};
use super::{
    board_prop, date_json, date_prop, id_prop, obj, path_prop, pick, quoted, revision_prop,
    sub_board_prop, Args, Ctx, Tool, ToolError, ToolOut, ToolResult,
};

const KINDS: &[&str] = &[
    "dood_grid",
    "breakdown",
    "breakdown_day",
    "element_detail",
    "vertical",
    "multi_unit",
    "calendar",
];

pub fn tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "report",
            title: "A report as data",
            description: "One of the reports beyond the one-line and the day out of days \
                          summary, as data, for a sub-board: dood_grid (the day out of days \
                          grid in the schedule's code letters; sort and no_holds apply), \
                          breakdown (each scene in board order with its day and its \
                          elements by category), breakdown_day (each shoot day's elements \
                          merged by category, with how many of the day's scenes list each), \
                          element_detail (each element's scenes with day and date, and its \
                          work dates), vertical (the cast ID grid: a row per cast member, a \
                          scene per column), multi_unit (every sub-board of the board by \
                          date; elements filters the days) or calendar (each date from and \
                          to: what it is, the shoot day, scenes, pages and minutes, and \
                          events).",
            mutating: false,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        board_prop(),
                        sub_board_prop(),
                        ("kind", one_of("Which report.", KINDS)),
                        (
                            "sort",
                            array(
                                "dood_grid: sort keys in order (board_id, name, start or \
                                 first_work, finish or last_work, work or worked, hold \
                                 or held), each optionally with :desc.",
                                name("A key."),
                                4,
                            ),
                        ),
                        (
                            "no_holds",
                            boolean("dood_grid: as if no element had holds. Default false."),
                        ),
                        (
                            "elements",
                            array(
                                "multi_unit: only days with a scene listing one of these \
                                 element ids.",
                                id_prop("An element id."),
                                1000,
                            ),
                        ),
                        (
                            "from",
                            date_prop("calendar: the first date. Default: the first shoot day."),
                        ),
                        (
                            "to",
                            date_prop("calendar: the last date. Default: the last shoot day."),
                        ),
                    ],
                    &["path", "kind"],
                )
            },
            run: report,
        },
        Tool {
            name: "report_list",
            title: "Report list and settings",
            description: "The schedule's report list in order (each report's name, kind \
                          and options: date range, orientation, toggles, category and \
                          element filters, header layout, text size, columns, page \
                          break, shoot day lines, day breaks, total count, category \
                          columns and strip layout), the header layouts, the production \
                          info and the day out of days code letters.",
            mutating: false,
            destructive: false,
            schema: || object(vec![path_prop()], &["path"]),
            run: report_list,
        },
        Tool {
            name: "set_production",
            title: "Set the production info",
            description: "Sets the production info that report headers show: company, \
                          director, producer, unit_production_manager, assistant_director, \
                          art_director, set_dresser, script_date, production_number, \
                          preparer, timing, episode and custom_1 to custom_4 (each given \
                          field; an empty string clears it; text, 200 characters at most). \
                          The title is the schedule's title.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("company", name("The production company.")),
                        ("director", name("The director.")),
                        ("producer", name("The producer.")),
                        (
                            "unit_production_manager",
                            name("The unit production manager."),
                        ),
                        ("assistant_director", name("The assistant director.")),
                        ("art_director", name("The art director.")),
                        ("set_dresser", name("The set dresser.")),
                        ("script_date", name("The script's date, as text.")),
                        ("production_number", name("The production number.")),
                        ("preparer", name("Who prepared the schedule.")),
                        ("timing", name("The script timing.")),
                        ("episode", name("The episode.")),
                        ("custom_1", name("A free field.")),
                        ("custom_2", name("A free field.")),
                        ("custom_3", name("A free field.")),
                        ("custom_4", name("A free field.")),
                    ],
                    &["path", "revision"],
                )
            },
            run: set_production,
        },
        Tool {
            name: "set_dood_codes",
            title: "Set the day out of days code letters",
            description: "Changes the day out of days code letters (start, work, finish, \
                          hold, drop, pickup, travel, fitting, rehearsal, holiday, \
                          company_move): each 1 to 3 letters, digits or /, no two the same. \
                          default: true puts the usual letters back first.",
            mutating: true,
            destructive: false,
            schema: || {
                let mut props = vec![
                    path_prop(),
                    revision_prop(),
                    ("default", boolean("Start from the usual letters.")),
                ];
                for k in DoodCodes::KEYS {
                    props.push((k, name("The letters.")));
                }
                object(props, &["path", "revision"])
            },
            run: set_dood_codes,
        },
        Tool {
            name: "set_prep_wrap",
            title: "Set prep start and wrap end",
            description: "Sets a sub-board's prep start and wrap end dates (null clears), \
                          which the budget exchange file carries when set. Wrap can't end \
                          before prep starts.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        board_prop(),
                        sub_board_prop(),
                        (
                            "prep_start",
                            nullable(date_prop("The first day of prep, or null.")),
                        ),
                        (
                            "wrap_end",
                            nullable(date_prop("The last day of wrap, or null.")),
                        ),
                    ],
                    &["path", "revision"],
                )
            },
            run: set_prep_wrap,
        },
    ]
}

fn slot_json(slot: &Slot) -> Value {
    json!({"day": slot.day, "date": date_json(slot.date)})
}

fn scene_json(s: &Schedule, h: SheetId) -> Value {
    s.sheet(h).map_or(Value::Null, |x| {
        json!({
            "sheet": h.0,
            "scenes": x.scenes,
            "int_ext": x.int_ext,
            "set": x.set,
            "time_of_day": x.time_of_day,
            "eighths": x.pages.0,
            "pages": x.pages.to_string(),
        })
    })
}

fn category_name(s: &Schedule, c: method_schedule_core::model::CategoryId) -> String {
    s.category(c).map_or(String::new(), |x| x.name.clone())
}

fn report(ctx: &Ctx, a: &Args) -> ToolResult {
    let l = load(ctx, a)?;
    let s = &l.schedule;
    let (b, sb) = pick(s, a)?;
    let kind = a.req("kind")?;
    let data: Value = match kind {
        "dood_grid" => {
            let opts = Options {
                no_holds: a.opt_bool("no_holds").unwrap_or(false),
            };
            let mut rows = dood_with(s, b, sb, opts)?;
            let mut keys = Vec::new();
            for k in a.list("sort") {
                let t = k.as_str().unwrap_or_default();
                let (n, desc) = match t.split_once(':') {
                    Some((n, "desc")) => (n, true),
                    Some((n, _)) => (n, false),
                    None => (t, false),
                };
                keys.push((
                    SortKey::from_name(n)
                        .ok_or_else(|| ToolError::invalid(format!("sort: unknown {}", quoted(n))))?,
                    desc,
                ));
            }
            sort_rows(s, &mut rows, &keys);
            let sums = summary(&rows);
            json!(rows
                .iter()
                .zip(sums)
                .map(|(r, sum)| {
                    let e = s.element(r.element);
                    json!({
                        "element": r.element.0,
                        "board_id": e.and_then(|x| x.board_id.clone()),
                        "name": e.map_or(String::new(), |x| x.name.clone()),
                        "category": e.map_or(String::new(), |x| category_name(s, x.category)),
                        "cells": r.cells.iter().map(|(d, c)| json!({"date": d.to_string(), "code": c.code_with(&s.dood_codes)})).collect::<Vec<_>>(),
                        "work": sum.work,
                        "hold": sum.hold,
                        "start": date_json(sum.start),
                        "finish": date_json(sum.finish),
                        "total": sum.total,
                    })
                })
                .collect::<Vec<_>>())
        }
        "breakdown" => json!(report_data::breakdown_by_strip(s, b, sb)?
            .iter()
            .map(|r| json!({
                "scene": scene_json(s, r.sheet),
                "slot": slot_json(&r.slot),
                "elements": r.elements.iter().map(|(c, els)| json!({
                    "category": category_name(s, *c),
                    "elements": els.iter().map(|e| element_text(s, *e)).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>()),
        "breakdown_day" => json!(report_data::breakdown_by_day(s, b, sb)?
            .iter()
            .map(|d| json!({
                "day": d.number,
                "date": date_json(d.date),
                "scenes": d.sheets.iter().map(|h| scene_json(s, *h)).collect::<Vec<_>>(),
                "eighths": d.pages.0,
                "minutes": d.time.0,
                "elements": d.elements.iter().map(|(c, els)| json!({
                    "category": category_name(s, *c),
                    "elements": els.iter().map(|(e, n)| json!({"element": e.0, "text": element_text(s, *e), "scenes": n, "needed": d.needed.iter().find(|(x, _)| x == e).map_or(1, |(_, q)| *q), "partial": d.partial.contains(e)})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>()),
        "element_detail" => json!(report_data::element_detail(s, b, sb, |_, _| true)?
            .iter()
            .map(|d| json!({
                "element": d.element.0,
                "text": element_text(s, d.element),
                "category": category_name(s, d.category),
                "scenes": d.scenes.iter().map(|(h, slot)| json!({"scene": scene_json(s, *h), "slot": slot_json(slot)})).collect::<Vec<_>>(),
                "work_dates": d.work_dates.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>()),
        "vertical" => {
            let v = report_data::vertical(s, b, sb, s.cast())?;
            json!({
                "cast": v.cast.iter().map(|e| json!({"element": e.0, "text": element_text(s, *e)})).collect::<Vec<_>>(),
                "days": v.days.iter().map(|d| json!({
                    "day": d.number,
                    "date": date_json(d.date),
                    "eighths": d.pages.0,
                    "minutes": d.time.0,
                    "scenes": d.scenes.iter().map(|(h, rows)| json!({"scene": scene_json(s, *h), "rows": rows})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            })
        }
        "multi_unit" => {
            let only: Vec<ElementId> = a.u32s("elements").into_iter().map(ElementId).collect();
            let (dates, undated) = report_data::multi_unit(s, b, s.cast(), &only)?;
            let sub_name = |id| {
                s.board(b)
                    .and_then(|x| x.sub_boards.iter().find(|y| y.id == id))
                    .map_or(String::new(), |y| y.name.clone())
            };
            json!({
                "dates": dates.iter().map(|(d, units)| json!({
                    "date": d.to_string(),
                    "units": units.iter().map(|u| json!({
                        "sub_board": u.sub_board.0,
                        "name": sub_name(u.sub_board),
                        "day": u.number,
                        "eighths": u.pages.0,
                        "minutes": u.time.0,
                        "scenes": u.lines.iter().filter_map(|l| match l {
                            method_schedule_core::reports::OneLine::Scene { sheet, .. } => Some(scene_json(s, *sheet)),
                            _ => None,
                        }).collect::<Vec<_>>(),
                    })).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
                "undated": undated.iter().map(|x| x.0).collect::<Vec<_>>(),
            })
        }
        _ => {
            let span = report_data::shoot_span(s, b, sb)?;
            let from: Option<Date> = a.date("from")?.or(span.map(|x| x.0));
            let to: Option<Date> = a.date("to")?.or(span.map(|x| x.1));
            let (Some(from), Some(to)) = (from, to) else {
                return Err(ToolError::invalid(
                    "the sub-board has no dated shoot days: give from and to",
                ));
            };
            if to < from || to.0 - from.0 > 3660 {
                return Err(ToolError::invalid(
                    "to must be on or after from, at most ten years later",
                ));
            }
            json!(report_data::calendar_dates(s, b, sb, from, to)?
                .iter()
                .map(|d| json!({
                    "date": d.date.to_string(),
                    "kind": super::read::day_kind_name(d.kind),
                    "shoot_day": d.shoot_day,
                    "scenes": d.sheets.iter().map(|h| scene_json(s, *h)).collect::<Vec<_>>(),
                    "eighths": d.pages.0,
                    "minutes": d.time.0,
                    "events": d.events,
                }))
                .collect::<Vec<_>>())
        }
    };
    Ok(ToolOut {
        summary: format!("The {kind} report of {}.", l.shown),
        data: json!({"path": l.shown, "kind": kind, "report": data}),
    })
}

fn report_list(ctx: &Ctx, a: &Args) -> ToolResult {
    let l = load(ctx, a)?;
    let s = &l.schedule;
    let list: Vec<Value> = s
        .report_list()
        .iter()
        .map(|r| {
            let o = &r.options;
            json!({
                "name": r.name,
                "kind": r.kind.name(),
                "from": date_json(o.from),
                "to": date_json(o.to),
                "landscape": r.landscape(),
                "synopsis": o.synopsis,
                "cast_names": o.cast_names,
                "page_per_day": o.page_break == method_schedule_core::setup::PageBreak::Day,
                "page_break": o.page_break.name(),
                "shoot_days": o.shoot_days,
                "day_breaks": o.day_breaks,
                "total_count": o.total_count,
                "element_columns": o.element_columns.iter().map(|c| c.0).collect::<Vec<_>>(),
                "layout": o.layout.and_then(|id| s.layouts.get(id)).map(|l| json!({"id": l.id.0, "name": l.name})),
                "banners": o.banners,
                "categories": o.categories.iter().map(|c| c.0).collect::<Vec<_>>(),
                "elements": o.elements.iter().map(|e| e.0).collect::<Vec<_>>(),
                "colors": o.colors,
                "icons": o.icons,
                "header": s.header_of(o.header).name,
                "text": o.text.name(),
                "columns": o.columns.iter().map(report_column).collect::<Vec<_>>(),
                "no_holds": o.no_holds,
                "weekdays_only": o.weekdays_only,
            })
        })
        .collect();
    let mut codes = Map::new();
    for k in DoodCodes::KEYS {
        codes.insert(k.to_string(), json!(s.dood_codes.get(k)));
    }
    Ok(ToolOut {
        summary: format!("{} reports in {}.", list.len(), l.shown),
        data: json!({
            "path": l.shown,
            "revision": l.revision,
            "reports": list,
            "header_layouts": s.header_list().iter().map(header_json).collect::<Vec<_>>(),
            "production": production_json(s),
            "dood_codes": codes,
        }),
    })
}

fn done<T>(e: &Edited<T>, what: &str, mut extra: Map<String, Value>) -> ToolOut {
    extra.insert("path".into(), json!(e.shown));
    extra.insert("changed".into(), json!(e.changed));
    extra.insert("revision".into(), json!(e.revision));
    let summary = if e.changed {
        format!("{what} in {}. Revision {}.", e.shown, e.revision)
    } else {
        format!("{what}: nothing changed in {}; nothing written.", e.shown)
    };
    ToolOut {
        summary,
        data: Value::Object(extra),
    }
}

/// A header layout: id, name, looks and rows of item names (`{"text": ...}` for custom
/// text).
fn header_json(h: &method_schedule_core::headers::HeaderDesign) -> Value {
    use method_schedule_core::headers::HeaderItem;
    json!({
        "id": h.id.0,
        "name": h.name,
        "include": h.include,
        "first_page_only": h.first_page_only,
        "bold": h.bold,
        "italic": h.italic,
        "align": h.align.name(),
        "size": h.size.name(),
        "rows": h.rows.iter().map(|r| r.iter().map(|i| match i {
            HeaderItem::Text(t) => json!({"text": t}),
            other => json!(other.name()),
        }).collect::<Vec<_>>()).collect::<Vec<_>>(),
    })
}

/// The title and every production field, by name.
fn production_json(s: &method_schedule_core::model::Schedule) -> Value {
    let mut m = Map::new();
    m.insert("title".into(), json!(s.title));
    for f in Production::FIELDS {
        m.insert(
            f.to_string(),
            json!(s.production.get(f).unwrap_or_default()),
        );
    }
    Value::Object(m)
}

fn set_production(ctx: &Ctx, a: &Args) -> ToolResult {
    let e = edit(ctx, a, |s| {
        for f in Production::FIELDS {
            if let Some(v) = a.str(f) {
                s.production
                    .set(f, v)
                    .map_err(|e| ToolError::invalid(e.to_string()))?;
            }
        }
        Ok(s.production.clone())
    })?;
    let mut out = Map::new();
    for f in Production::FIELDS {
        out.insert(f.to_string(), json!(e.out.get(f).unwrap_or_default()));
    }
    Ok(done(&e, "Production info set", out))
}

fn set_dood_codes(ctx: &Ctx, a: &Args) -> ToolResult {
    let e = edit(ctx, a, |s| {
        let mut c = if a.opt_bool("default").unwrap_or(false) {
            DoodCodes::default()
        } else {
            s.dood_codes.clone()
        };
        for k in DoodCodes::KEYS {
            if let Some(v) = a.str(k) {
                *c.get_mut(k).expect("known") = v.trim().to_string();
            }
        }
        if let Some(why) = c.problem() {
            return Err(ToolError::refused(format!("{why}; nothing written")));
        }
        s.dood_codes = c.clone();
        Ok(c)
    })?;
    Ok(done(
        &e,
        "Day out of days codes set",
        obj(vec![("codes", json!(e.out.text()))]),
    ))
}

fn set_prep_wrap(ctx: &Ctx, a: &Args) -> ToolResult {
    let prep = a.date("prep_start")?;
    let wrap = a.date("wrap_end")?;
    let e = edit(ctx, a, |s| {
        let (b, sb) = pick(s, a)?;
        let cal = s.calendar_mut(b, sb)?;
        if a.has("prep_start") {
            cal.prep_start = prep;
        }
        if a.has("wrap_end") {
            cal.wrap_end = wrap;
        }
        if let (Some(p), Some(w)) = (cal.prep_start, cal.wrap_end) {
            if w < p {
                return Err(ToolError::refused(format!(
                    "wrap ({w}) can't end before prep starts ({p}); nothing written"
                )));
            }
        }
        Ok((cal.prep_start, cal.wrap_end))
    })?;
    Ok(done(
        &e,
        "Prep and wrap set",
        obj(vec![
            ("prep_start", date_json(e.out.0)),
            ("wrap_end", date_json(e.out.1)),
        ]),
    ))
}

/// A report column as the file has it: its field's name when plain, else an object with
/// `field`, `line` (1 to 4), `span`, `align`, `bold`, `italic` and `size` (null: the
/// report's text size).
fn report_column(c: &method_schedule_core::setup::ReportColumn) -> Value {
    if c.is_plain() {
        return json!(c.field.name());
    }
    json!({
        "field": c.field.name(),
        "line": c.line + 1,
        "span": c.span,
        "align": c.align.name(),
        "bold": c.bold,
        "italic": c.italic,
        "size": c.size.map(|t| t.name()),
    })
}
