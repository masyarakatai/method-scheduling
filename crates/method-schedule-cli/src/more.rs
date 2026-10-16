//! The reports and settings added with the report options (decisions.md, entry 59): the
//! day out of days grid, the breakdown by strip and by day, the element detail, the whole
//! board, the calendar, the vertical and multi-unit one-lines, and the production info,
//! code letters, prep and wrap dates and report list kept in the schedule.

use method_schedule_core::dood::{dood_with, sort_rows, summary, Options, SortKey};
use method_schedule_core::model::{DayKind, ElementId};
use method_schedule_core::report_data::{self, element_text};
use method_schedule_core::setup::{key_label, DoodCodes, Production};
use method_schedule_core::{Date, Schedule};
use method_schedule_format::export as tables;

use super::{chosen_board, chosen_board_mut, export, first, flag, load, positional, save};

fn date(d: Option<Date>) -> String {
    d.map_or(String::new(), |d| d.to_string())
}

fn scene_text(s: &Schedule, sheet: method_schedule_core::model::SheetId) -> String {
    s.sheet(sheet).map_or_else(String::new, |h| {
        format!(
            "{:<6} {:<8} {:<20} {:<8} {:>7}",
            h.scenes,
            h.int_ext,
            h.set,
            h.time_of_day,
            h.pages.to_string()
        )
    })
}

/// `dood FILE --grid`: the grid, a column per date, in the schedule's code letters.
/// `--sort KEY[:desc],...` sorts the rows (board_id, name, start, finish, work, hold),
/// `--no-holds` works it out as if no element had holds.
pub fn dood_grid_cmd(args: &[String]) -> Result<String, String> {
    let path = args.first().ok_or(
        "usage: method-schedule dood FILE --grid [--no-holds] [--sort KEY[:desc],...] [--csv | --xlsx OUT]",
    )?;
    let s = load(path)?;
    let (b, sb) = first(&s)?;
    let opts = Options {
        no_holds: args.iter().any(|a| a == "--no-holds"),
    };
    let mut rows = dood_with(&s, b, sb, opts).map_err(|e| e.to_string())?;
    if let Some(keys) = flag(args, "--sort") {
        sort_rows(&s, &mut rows, &sort_keys(keys)?);
    }
    // The spreadsheet takes the same options as the text.
    if let Some(out) = export(args, || tables::dood_grid_of(&s, &rows, None))? {
        return Ok(out);
    }
    let first = rows
        .iter()
        .filter_map(|r| r.cells.first())
        .map(|c| c.0)
        .min();
    let last = rows
        .iter()
        .filter_map(|r| r.cells.last())
        .map(|c| c.0)
        .max();
    let (Some(first), Some(last)) = (first, last) else {
        return Ok("no element works on a shoot day or has an event\n".to_string());
    };
    let width = (last.0 - first.0 + 1) as usize;
    let mut out = format!("{:<6} {:<20} {first} to {last}\n", "ID", "ELEMENT");
    for (r, sum) in rows.iter().zip(summary(&rows)) {
        let e = s.element(r.element).ok_or("missing element")?;
        let mut cells = vec![".".to_string(); width];
        for (d, c) in &r.cells {
            let code = c.code_with(&s.dood_codes);
            if !code.is_empty() {
                cells[(d.0 - first.0) as usize] = code;
            }
        }
        out.push_str(&format!(
            "{:<6} {:<20} {}  work {} hold {} total {}\n",
            e.board_id.as_deref().unwrap_or(""),
            e.name,
            cells.join(" "),
            sum.work,
            sum.hold,
            sum.total
        ));
    }
    Ok(out)
}

fn sort_keys(text: &str) -> Result<Vec<(SortKey, bool)>, String> {
    let keys: Vec<(SortKey, bool)> = text
        .split(',')
        .map(|k| {
            let (name, desc) = match k.split_once(':') {
                Some((n, "desc")) => (n, true),
                Some((n, "asc")) => (n, false),
                Some(_) => return Err(format!("{k:?}: a direction is asc or desc")),
                None => (k, false),
            };
            SortKey::from_name(name.trim())
                .map(|key| (key, desc))
                .ok_or_else(|| {
                    format!(
                        "{name:?} is not a sort key (board_id, name, start, finish, work, hold)"
                    )
                })
        })
        .collect::<Result<_, _>>()?;
    if keys.len() > 4 {
        return Err("at most four sort keys".to_string());
    }
    Ok(keys)
}

/// `breakdown FILE [--by-day] [--csv | --xlsx OUT]`.
pub fn breakdown_cmd(args: &[String]) -> Result<String, String> {
    let path = args
        .first()
        .ok_or("usage: method-schedule breakdown FILE [--by-day] [--csv | --xlsx OUT]")?;
    let s = load(path)?;
    let (b, sb) = first(&s)?;
    let by_day = args.iter().any(|a| a == "--by-day");
    let table = if by_day {
        export(args, || tables::breakdown_day(&s, b, sb))?
    } else {
        export(args, || tables::breakdown(&s, b, sb))?
    };
    if let Some(out) = table {
        return Ok(out);
    }
    let cat = |c| s.category(c).map_or("", |x| x.name.as_str());
    let mut out = String::new();
    if by_day {
        for d in report_data::breakdown_by_day(&s, b, sb).map_err(|e| e.to_string())? {
            out.push_str(&format!(
                "Day {}  {}  {}, {} pages\n",
                d.number,
                date(d.date),
                super::count(d.sheets.len(), "scene"),
                d.pages
            ));
            for (c, els) in &d.elements {
                let list: Vec<String> = els
                    .iter()
                    .map(|(e, n)| match d.needed.iter().find(|(x, _)| x == e) {
                        Some((_, q)) => {
                            let star = if d.partial.contains(e) { "*" } else { "" };
                            format!("{} x{q}{star} ({n})", element_text(&s, *e))
                        }
                        None => format!("{} ({n})", element_text(&s, *e)),
                    })
                    .collect();
                out.push_str(&format!("  {}: {}\n", cat(*c), list.join(", ")));
            }
        }
        return Ok(out);
    }
    for r in report_data::breakdown_by_strip(&s, b, sb).map_err(|e| e.to_string())? {
        let day = r.slot.day.map_or("not scheduled".to_string(), |n| {
            format!("day {n} {}", date(r.slot.date))
        });
        out.push_str(&format!("{}  {day}\n", scene_text(&s, r.sheet)));
        for (c, els) in &r.elements {
            let list: Vec<String> = els.iter().map(|e| element_text(&s, *e)).collect();
            out.push_str(&format!("  {}: {}\n", cat(*c), list.join(", ")));
        }
    }
    Ok(out)
}

/// `detail FILE [--csv | --xlsx OUT]`: each element's scenes with day and date.
pub fn detail_cmd(args: &[String]) -> Result<String, String> {
    let path = args
        .first()
        .ok_or("usage: method-schedule detail FILE [--csv | --xlsx OUT]")?;
    let s = load(path)?;
    let (b, sb) = first(&s)?;
    if let Some(out) = export(args, || tables::element_detail(&s, b, sb))? {
        return Ok(out);
    }
    let mut out = String::new();
    for d in report_data::element_detail(&s, b, sb, |_, _| true).map_err(|e| e.to_string())? {
        out.push_str(&format!(
            "{} ({}), {} work days\n",
            element_text(&s, d.element),
            s.category(d.category).map_or("", |c| c.name.as_str()),
            d.work_dates.len()
        ));
        for (h, slot) in &d.scenes {
            let day = slot.day.map_or("not scheduled".to_string(), |n| {
                format!("day {n} {}", date(slot.date))
            });
            out.push_str(&format!("  {}  {day}\n", scene_text(&s, *h)));
        }
    }
    Ok(out)
}

/// `board FILE --csv | --xlsx OUT`: every strip of the board.
pub fn board_cmd(args: &[String]) -> Result<String, String> {
    let usage = "usage: method-schedule board FILE (--csv | --xlsx OUT)";
    let path = args.first().ok_or(usage)?;
    let s = load(path)?;
    let (b, _) = first(&s)?;
    export(args, || tables::board(&s, b))?.ok_or_else(|| usage.to_string())
}

/// `calendar FILE [--from DATE] [--to DATE]`: each date with what it is, its shoot day,
/// scenes, pages, estimated length and events. Without dates, the first to the last
/// shoot day.
pub fn calendar_cmd(args: &[String]) -> Result<String, String> {
    let path = args
        .first()
        .ok_or("usage: method-schedule calendar FILE [--from DATE] [--to DATE]")?;
    let s = load(path)?;
    let (b, sb) = first(&s)?;
    let parse = |f: &str| -> Result<Option<Date>, String> {
        flag(args, f)
            .map(|v| {
                v.parse::<Date>()
                    .map_err(|_| format!("{v:?} is not a YYYY-MM-DD date"))
            })
            .transpose()
    };
    let span = report_data::shoot_span(&s, b, sb).map_err(|e| e.to_string())?;
    let (Some(from), Some(to)) = (
        parse("--from")?.or(span.map(|x| x.0)),
        parse("--to")?.or(span.map(|x| x.1)),
    ) else {
        return Err("no shoot days to show: give --from and --to".to_string());
    };
    if to < from || (to.0 - from.0) > 3660 {
        return Err("--to must be on or after --from, at most ten years later".to_string());
    }
    let sub = &chosen_board(&s)?.sub_boards[0];
    let mut out = String::new();
    for d in report_data::calendar_dates(&s, b, sb, from, to).map_err(|e| e.to_string())? {
        let what = match (d.shoot_day, d.kind) {
            (Some(n), _) => format!("Day {n}"),
            (None, DayKind::Off) => "off".to_string(),
            (None, DayKind::Holiday) => "holiday".to_string(),
            (None, DayKind::Move) => "company travel".to_string(),
            (None, DayKind::Work) => String::new(),
        };
        let events: Vec<String> = d
            .events
            .iter()
            .filter_map(|i| sub.calendar.events.get(*i))
            .map(|e| e.title())
            .collect();
        let work = if d.shoot_day.is_some() {
            format!(
                "{} scenes, {} pages, {}:{:02}",
                d.sheets.len(),
                d.pages,
                d.time.0 / 60,
                d.time.0 % 60
            )
        } else {
            String::new()
        };
        out.push_str(&format!(
            "{} {:?}  {what:<15} {work}{}\n",
            d.date,
            d.date.weekday(),
            if events.is_empty() {
                String::new()
            } else {
                format!("  [{}]", events.join(", "))
            }
        ));
    }
    Ok(out)
}

/// `vertical FILE`: the cast ID grid, a row per cast member, a column per scene.
pub fn vertical_cmd(args: &[String]) -> Result<String, String> {
    let path = args.first().ok_or("usage: method-schedule vertical FILE")?;
    let s = load(path)?;
    let (b, sb) = first(&s)?;
    let v = report_data::vertical(&s, b, sb, s.cast()).map_err(|e| e.to_string())?;
    let mut out = String::new();
    let mut heads = Vec::new();
    for d in &v.days {
        for (h, _) in &d.scenes {
            heads.push(s.sheet(*h).map_or(String::new(), |x| x.scenes.clone()));
        }
        heads.push("|".to_string());
    }
    out.push_str(&format!("{:<24} {}\n", "", heads.join(" ")));
    for (row, e) in v.cast.iter().enumerate() {
        let mut cells = Vec::new();
        for d in &v.days {
            for (h, on) in &d.scenes {
                let w = s.sheet(*h).map_or(1, |x| x.scenes.chars().count().max(1));
                let mark = if on.contains(&row) { "X" } else { "." };
                cells.push(format!("{mark:<w$}"));
            }
            cells.push("|".to_string());
        }
        out.push_str(&format!(
            "{:<24} {}\n",
            element_text(&s, *e),
            cells.join(" ")
        ));
    }
    Ok(out)
}

/// `multiunit FILE [--element NAME]`: every sub-board's days by date.
pub fn multi_unit_cmd(args: &[String]) -> Result<String, String> {
    let path = args
        .first()
        .ok_or("usage: method-schedule multiunit FILE [--element NAME]")?;
    let s = load(path)?;
    let (b, _) = first(&s)?;
    let only: Vec<ElementId> = match flag(args, "--element") {
        None => Vec::new(),
        Some(name) => {
            let found: Vec<ElementId> = s
                .elements
                .iter()
                .filter(|e| e.name.eq_ignore_ascii_case(name.trim()))
                .map(|e| e.id)
                .collect();
            if found.is_empty() {
                return Err(format!("no element named {name:?}"));
            }
            found
        }
    };
    let board = chosen_board(&s)?;
    let (dates, undated) =
        report_data::multi_unit(&s, b, s.cast(), &only).map_err(|e| e.to_string())?;
    let name = |id| {
        board
            .sub_boards
            .iter()
            .find(|x| x.id == id)
            .map_or(String::new(), |x| x.name.clone())
    };
    let mut out = String::new();
    for (d, units) in dates {
        out.push_str(&format!("{d} {:?}\n", d.weekday()));
        for u in units {
            out.push_str(&format!(
                "  {}, day {}: {} pages\n",
                name(u.sub_board),
                u.number,
                u.pages
            ));
            for l in &u.lines {
                match l {
                    method_schedule_core::reports::OneLine::Scene { sheet, .. } => {
                        out.push_str(&format!("    {}\n", scene_text(&s, *sheet)));
                    }
                    method_schedule_core::reports::OneLine::Banner { text, .. } => {
                        out.push_str(&format!("    -- {text}\n"));
                    }
                }
            }
        }
    }
    for id in undated {
        out.push_str(&format!("{} has no start date and is left out\n", name(id)));
    }
    Ok(out)
}

/// `production FILE [--company T] [--director T] ...`: shows the production info, or sets
/// the fields given (each field's name with dashes, `--unit-production-manager`) and saves.
pub fn production_cmd(args: &[String]) -> Result<String, String> {
    let path = args.first().ok_or(
        "usage: method-schedule production FILE [--company TEXT] [--director TEXT] [--producer TEXT] \
         [--unit-production-manager TEXT] [--assistant-director TEXT] [--art-director TEXT] \
         [--set-dresser TEXT] [--script-date TEXT] [--production-number TEXT] [--preparer TEXT] \
         [--timing TEXT] [--episode TEXT] [--custom-1 TEXT] ... [--custom-4 TEXT]",
    )?;
    let mut s = load(path)?;
    let mut changed = false;
    for field in Production::FIELDS {
        if let Some(v) = flag(args, &format!("--{}", field.replace('_', "-"))) {
            s.production.set(field, v).map_err(|e| e.to_string())?;
            changed = true;
        }
    }
    if changed {
        save(path, &s)?;
    }
    let mut out = format!("{:<24} {}\n", "title", s.title);
    for field in Production::FIELDS {
        out.push_str(&format!(
            "{field:<24} {}\n",
            s.production.get(field).unwrap_or_default()
        ));
    }
    Ok(out)
}

/// `codes FILE [KEY=LETTERS...]`: shows the day out of days code letters, or sets them
/// (`hold=HD`, `default` for all the usual ones) and saves.
pub fn codes_cmd(args: &[String]) -> Result<String, String> {
    let path = args
        .first()
        .ok_or("usage: method-schedule codes FILE [KEY=LETTERS ...] [default]")?;
    let mut s = load(path)?;
    let sets = positional(&args[1..], &[]);
    if !sets.is_empty() {
        let mut codes = s.dood_codes.clone();
        for set in sets {
            if set == "default" {
                codes = DoodCodes::default();
                continue;
            }
            let (key, value) = set
                .split_once('=')
                .ok_or_else(|| format!("{set:?}: give KEY=LETTERS, such as hold=HD"))?;
            *codes.get_mut(key).ok_or_else(|| {
                format!("{key:?} is not a code ({})", DoodCodes::KEYS.join(", "))
            })? = value.trim().to_string();
        }
        if let Some(why) = codes.problem() {
            return Err(why);
        }
        s.dood_codes = codes;
        save(path, &s)?;
    }
    let mut out = String::new();
    for key in DoodCodes::KEYS {
        out.push_str(&format!(
            "{:<15} {}\n",
            key_label(key),
            s.dood_codes.get(key).unwrap_or_default()
        ));
    }
    Ok(out)
}

/// `prep FILE [--start DATE|none] [--end DATE|none]`: the first sub-board's prep start
/// and wrap end, for the budget exchange.
pub fn prep_cmd(args: &[String]) -> Result<String, String> {
    let path = args
        .first()
        .ok_or("usage: method-schedule prep FILE [--start DATE|none] [--end DATE|none]")?;
    let mut s = load(path)?;
    let parse = |v: &str| -> Result<Option<Date>, String> {
        if v == "none" {
            return Ok(None);
        }
        v.parse::<Date>()
            .map(Some)
            .map_err(|_| format!("{v:?} is not a YYYY-MM-DD date"))
    };
    let cal = &mut chosen_board_mut(&mut s)?.sub_boards[0].calendar;
    let mut changed = false;
    if let Some(v) = flag(args, "--start") {
        cal.prep_start = parse(v)?;
        changed = true;
    }
    if let Some(v) = flag(args, "--end") {
        cal.wrap_end = parse(v)?;
        changed = true;
    }
    let (p, w) = (cal.prep_start, cal.wrap_end);
    if changed {
        s.check().map_err(|e| e.to_string())?;
        save(path, &s)?;
    }
    Ok(format!(
        "prep start {}\nwrap end   {}\n",
        p.map_or("none".to_string(), |d| d.to_string()),
        w.map_or("none".to_string(), |d| d.to_string())
    ))
}

/// `reports FILE`: the report list, in order, with the options that differ from the
/// defaults.
/// `headers FILE`: the report header layouts, each with its id, looks and rows.
pub fn headers_cmd(args: &[String]) -> Result<String, String> {
    let path = args.first().ok_or("usage: method-schedule headers FILE")?;
    let s = load(path)?;
    let mut out = String::new();
    for h in s.header_list() {
        out.push_str(&format!("{:>2}  {}\n", h.id.0, h.text()));
    }
    Ok(out)
}

pub fn reports_cmd(args: &[String]) -> Result<String, String> {
    let path = args.first().ok_or("usage: method-schedule reports FILE")?;
    let s = load(path)?;
    let mut out = String::new();
    for (i, r) in s.report_list().iter().enumerate() {
        let o = &r.options;
        let mut notes = Vec::new();
        if o.from.is_some() || o.to.is_some() {
            notes.push(format!("{} to {}", date(o.from), date(o.to)));
        }
        if !o.categories.is_empty() {
            notes.push(format!("{} categories", o.categories.len()));
        }
        if !o.elements.is_empty() {
            notes.push(format!("{} elements", o.elements.len()));
        }
        if !o.columns.is_empty() {
            let cols: Vec<String> = o.columns.iter().map(|c| c.text()).collect();
            notes.push(format!("columns {}", cols.join(", ")));
        }
        if let Some(l) = o.layout.and_then(|id| s.layouts.get(id)) {
            notes.push(format!("layout {:?}", l.name));
        }
        if !o.element_columns.is_empty() {
            let names: Vec<String> = o
                .element_columns
                .iter()
                .filter_map(|c| s.category(*c).map(|c| c.name.clone()))
                .collect();
            notes.push(format!("category columns {}", names.join(", ")));
        }
        if o.page_break != method_schedule_core::setup::PageBreak::None {
            notes.push(format!("a page per {}", o.page_break.name()));
        }
        if !o.shoot_days {
            notes.push("no shoot day lines".to_string());
        }
        if !o.day_breaks {
            notes.push("no day breaks".to_string());
        }
        if o.total_count {
            notes.push("total count".to_string());
        }
        notes.push(format!("header {:?}", s.header_of(o.header).name));
        notes.push(
            if r.landscape() {
                "landscape"
            } else {
                "portrait"
            }
            .to_string(),
        );
        out.push_str(&format!(
            "{:>2}. {:<28} {:<15} {}\n",
            i + 1,
            r.name,
            r.kind.name(),
            notes.join(", ")
        ));
    }
    Ok(out)
}
