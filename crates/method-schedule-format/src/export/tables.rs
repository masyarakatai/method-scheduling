//! The reports as tables: one sheet each, with the columns `method-schedule ... --csv` prints.
//!
//! Page lengths take two cells: `eighths`, a whole number of eighths, and [`PAGES`],
//! the same length as text (`1 3/8`). CSV leaves the text column out
//! (`Sheet::without`), since it repeats the eighths.

use method_schedule_core::dood::{dood, summary};
use method_schedule_core::model::{
    BoardId, CategoryId, ElementId, Error, Sheet as Header, SheetId, SubBoardId,
};
use method_schedule_core::report_data::elements_by_category;
use method_schedule_core::reports::{self, OneLine};
use method_schedule_core::setup::{ReportKind, ReportOptions};
use method_schedule_core::{Eighths, Schedule};

use super::{Cell, ExportError, Formula, Sheet, Workbook, OMITTED};

/// The column holding a page length as text, beside the `eighths` column.
pub const PAGES: &str = "pages";

fn columns_of(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

fn int(n: u32) -> Cell {
    Cell::Int(u64::from(n))
}

fn pages_text(p: Eighths) -> Cell {
    Cell::Text(p.to_string())
}

/// The day out of days summary of a sub-board: one row per element it schedules.
pub fn dood_summary(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
) -> Result<Workbook, ExportError> {
    let mut rows = dood(s, board, sub)?;
    // A category's rows together, in category order (DOD-01).
    method_schedule_core::dood::group_by_category(s, &mut rows);
    let mut out = Vec::new();
    for (i, r) in summary(&rows).into_iter().enumerate() {
        let e = s
            .element(r.element)
            .ok_or(Error::NoSuchElement(r.element))?;
        let cat = s.category(e.category).map_or("", |c| c.name.as_str());
        out.push(vec![
            Cell::text(cat),
            Cell::text(e.board_id.as_deref().unwrap_or("")),
            Cell::text(e.name.as_str()),
            int(r.work),
            int(r.hold),
            Cell::date(r.start),
            Cell::date(r.finish),
            // Work plus hold, as a working formula.
            Formula::add(&[3, 4], i, u64::from(r.total)),
        ]);
    }
    Ok(Workbook {
        title: s.title.clone(),
        sheets: vec![Sheet {
            name: "Day Out of Days",
            columns: columns_of(&[
                "category", "board_id", "element", "work", "hold", "start", "finish", "total",
            ]),
            rows: out,
        }],
    })
}

/// The one-line sheet's name (the day totals' formulas name it).
const ONE_LINE: &str = "One-Line";

/// The options of the schedule's first one-line report in its report list (the defaults
/// when it has none set).
pub fn one_line_options(s: &Schedule) -> ReportOptions {
    s.report_list()
        .into_iter()
        .find(|r| r.kind == ReportKind::OneLine)
        .map(|r| r.options)
        .unwrap_or_default()
}

/// Which columns the one-line sheet has (decision 103).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OneLineColumns {
    /// As the reference app writes it: a first column for day break and banner text,
    /// then the report's strip layout fields in order, a day break row closing each day.
    #[default]
    Layout,
    /// Our stable columns, `day` to `text` (decision 93), a row per scene or banner.
    Fixed,
}

impl OneLineColumns {
    pub fn name(self) -> &'static str {
        match self {
            OneLineColumns::Layout => "layout",
            OneLineColumns::Fixed => "fixed",
        }
    }

    pub fn from_name(name: &str) -> Option<OneLineColumns> {
        match name {
            "layout" => Some(OneLineColumns::Layout),
            "fixed" => Some(OneLineColumns::Fixed),
            _ => None,
        }
    }
}

/// The one-line schedule of a sub-board, in board order, with `columns`.
/// `cast` names the category whose board IDs fill the cast cells (usually Cast). The
/// layout and element columns follow the schedule's one-line report options
/// ([`one_line_options`]).
pub fn one_line(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
    cast: Option<CategoryId>,
    columns: OneLineColumns,
) -> Result<Workbook, ExportError> {
    one_line_with(s, board, sub, cast, &one_line_options(s), columns)
}

/// How the one-line's element columns list an element on a sheet: its name with `names`,
/// otherwise its board ID (its name when it has none); then ` xN` when the sheet needs
/// more than one.
fn element_label(s: &Schedule, h: &Header, e: ElementId, names: bool) -> String {
    let Some(x) = s.element(e) else {
        return String::new();
    };
    let mut out = match (&x.board_id, names) {
        (Some(id), false) => id.clone(),
        _ => x.name.clone(),
    };
    let (n, _) = h.quantity(e);
    if n != 1 {
        out.push_str(&format!(" x{n}"));
    }
    out
}

/// The header of the layout sheet's first column, which holds day break and banner text.
pub const BREAKS: &str = "Day breaks and banners";

/// [`one_line`] with the report options `o`.
///
/// With [`OneLineColumns::Layout`] the sheet follows the report's strip layout
/// (`Schedule::report_fields`): a first column ([`BREAKS`]) for banner and day break
/// text, then each field as the board shows it (`reports::strip_cell`), each a column
/// titled with the field's label. Each day closes with a day break row (when the
/// options show day breaks): `End of Day N`, the date, the page total and the time, a
/// cell each.
///
/// With [`OneLineColumns::Fixed`] it has our stable columns, `day` to `text`, a row per
/// scene or banner, so tools that read earlier exports keep working.
///
/// Either way, one column per category the options' filter shows follows, in category
/// order, each listing the scene's elements of that category (`; ` between them) by
/// board ID or, with `cast_names`, by name, with the sheet's quantity as ` xN` when it
/// is more than one. The layout sheet ends with two working columns, `kind` and
/// `eighths`, that the totals read.
///
/// A second sheet, `Day Totals`, has a row per shoot day (its scenes and eighths, as
/// formulas over the day's rows of the first sheet) and a `total` row adding them up. A
/// CSV is the first sheet alone.
pub fn one_line_with(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
    cast: Option<CategoryId>,
    o: &ReportOptions,
    columns: OneLineColumns,
) -> Result<Workbook, ExportError> {
    let days = reports::one_line(s, board, sub, cast)?;
    let cats: Vec<_> = s
        .categories
        .iter()
        .filter(|c| o.shows_category(c.id))
        .collect();
    let category_cells = |sheet: SheetId| -> Result<Vec<Cell>, ExportError> {
        let h = s.sheet(sheet).ok_or(Error::NoSuchSheet(sheet))?;
        let groups = elements_by_category(s, &h.elements);
        Ok(cats
            .iter()
            .map(|c| {
                Cell::text(
                    groups
                        .iter()
                        .find(|(g, _)| *g == c.id)
                        .map(|(_, els)| {
                            els.iter()
                                .map(|e| element_label(s, h, *e, o.cast_names))
                                .collect::<Vec<_>>()
                                .join("; ")
                        })
                        .unwrap_or_default(),
                )
            })
            .collect())
    };
    let mut out: Vec<Vec<Cell>> = Vec::new();
    // Each day's rows of the one-line sheet, for the day totals.
    let mut day_rows = Vec::new();
    let (cols, kind_col, eighths_col) = match columns {
        OneLineColumns::Fixed => {
            for day in &days {
                let first = out.len();
                for line in &day.lines {
                    out.push(match line {
                        OneLine::Scene {
                            sheet,
                            scenes,
                            int_ext,
                            set,
                            time_of_day,
                            pages,
                            synopsis,
                            cast,
                        } => {
                            // An omitted scene is listed, but its length counts nowhere
                            // (decision 91): no eighths for the totals to add.
                            let omitted = s.plan.is_omitted(*sheet);
                            let mut row = vec![
                                int(day.number),
                                Cell::date(day.date),
                                Cell::text(if omitted { OMITTED } else { "scene" }),
                                Cell::text(scenes.as_str()),
                                Cell::text(int_ext.as_str()),
                                Cell::text(set.as_str()),
                                Cell::text(time_of_day.as_str()),
                                if omitted { Cell::Empty } else { int(pages.0) },
                                pages_text(*pages),
                                Cell::Empty,
                                Cell::text(cast.join(" ")),
                                Cell::text(synopsis.as_str()),
                            ];
                            row.extend(category_cells(*sheet)?);
                            row
                        }
                        OneLine::Banner { text, duration, .. } => {
                            let mut row = vec![
                                int(day.number),
                                Cell::date(day.date),
                                Cell::text("banner"),
                                Cell::Empty,
                                Cell::Empty,
                                Cell::Empty,
                                Cell::Empty,
                                Cell::Empty,
                                Cell::Empty,
                                int(duration.0),
                                Cell::Empty,
                                Cell::text(text.as_str()),
                            ];
                            row.extend(std::iter::repeat_n(Cell::Empty, cats.len()));
                            row
                        }
                    });
                }
                day_rows.push(first..out.len());
            }
            let mut cols = columns_of(&[
                "day",
                "date",
                "kind",
                "scenes",
                "int_ext",
                "set",
                "time_of_day",
                "eighths",
                PAGES,
                "minutes",
                "cast",
                "text",
            ]);
            cols.extend(cats.iter().map(|c| c.name.clone()));
            (cols, 2, 7)
        }
        OneLineColumns::Layout => {
            // Columns, so a category's elements lists as on the board (decision 194).
            let fields = s.report_columns(board, ReportKind::OneLine, o);
            // The break column, the fields, the categories, then `kind` and `eighths`.
            let width = 1 + fields.len() + cats.len() + 2;
            let (kind_col, eighths_col) = (width - 2, width - 1);
            let blank = |kind: &str| {
                let mut row = vec![Cell::Empty; width];
                row[kind_col] = Cell::text(kind);
                row
            };
            for day in &days {
                let first = out.len();
                for line in &day.lines {
                    match line {
                        OneLine::Scene { sheet, pages, .. } => {
                            let h = s.sheet(*sheet).ok_or(Error::NoSuchSheet(*sheet))?;
                            let mut row = vec![Cell::Empty];
                            row.extend(fields.iter().map(|f| {
                                let t = reports::report_cell(s, h, f, cast);
                                if t.is_empty() {
                                    Cell::Empty
                                } else {
                                    Cell::Text(t)
                                }
                            }));
                            row.extend(category_cells(*sheet)?);
                            if s.plan.is_omitted(*sheet) {
                                row.push(Cell::text(OMITTED));
                                row.push(Cell::Empty);
                            } else {
                                row.push(Cell::text("scene"));
                                row.push(int(pages.0));
                            }
                            out.push(row);
                        }
                        OneLine::Banner { text, .. } => {
                            let mut row = blank("banner");
                            row[0] = Cell::text(text.as_str());
                            out.push(row);
                        }
                    }
                }
                if o.day_breaks {
                    // The day's end, its date, page total and time, a cell each, in the
                    // cells before the working columns (the last ones joined when the
                    // layout has fewer fields).
                    let parts = [
                        format!("End of Day {}", day.number),
                        day.date.map_or(String::new(), |d| d.to_string()),
                        format!("{} pages", day.pages),
                        format!("{}:{:02}", day.time.0 / 60, day.time.0 % 60),
                    ];
                    let room = kind_col.min(parts.len());
                    let mut row = blank("day_break");
                    for (i, p) in parts[..room].iter().enumerate() {
                        let text = if i + 1 == room {
                            parts[i..]
                                .iter()
                                .filter(|x| !x.is_empty())
                                .cloned()
                                .collect::<Vec<_>>()
                                .join("  ")
                        } else {
                            p.clone()
                        };
                        if !text.is_empty() {
                            row[i] = Cell::Text(text);
                        }
                    }
                    out.push(row);
                }
                day_rows.push(first..out.len());
            }
            let mut cols = vec![BREAKS.to_string()];
            cols.extend(
                fields
                    .iter()
                    .map(|f| match f.category.and_then(|c| s.category(c)) {
                        Some(cat)
                            if f.field
                                == method_schedule_core::model::StripField::CategoryElements =>
                        {
                            cat.name.clone()
                        }
                        _ => f.field.label().to_string(),
                    }),
            );
            cols.extend(cats.iter().map(|c| c.name.clone()));
            cols.push("kind".to_string());
            cols.push("eighths".to_string());
            (cols, kind_col, eighths_col)
        }
    };
    // The day totals sheet: a row per day over its rows of the one-line sheet, then the
    // whole schedule, each total a working formula.
    let mut totals = Vec::new();
    for (day, rows) in days.iter().zip(day_rows) {
        let scene_rows: Vec<&Vec<Cell>> = out[rows.clone()]
            .iter()
            .filter(|r| r[kind_col] == Cell::text("scene"))
            .collect();
        let eighths: u64 = scene_rows
            .iter()
            .map(|r| match r[eighths_col] {
                Cell::Int(n) => n,
                _ => 0,
            })
            .sum();
        totals.push(vec![
            int(day.number),
            Cell::date(day.date),
            Formula::count_if_in(
                Some(ONE_LINE),
                kind_col,
                rows.clone(),
                "scene",
                scene_rows.len() as u64,
            ),
            Formula::sum_in(Some(ONE_LINE), eighths_col, rows, eighths),
        ]);
    }
    let value = |col: usize| -> u64 {
        totals
            .iter()
            .map(|r: &Vec<Cell>| match &r[col] {
                Cell::Int(n) => *n,
                Cell::Formula(f) => f.value(),
                _ => 0,
            })
            .sum()
    };
    let n = totals.len();
    let (scenes, eighths) = (value(2), value(3));
    totals.push(vec![
        Cell::text("total"),
        Cell::Empty,
        Formula::sum(2, 0..n, scenes),
        Formula::sum(3, 0..n, eighths),
    ]);
    Ok(Workbook {
        title: s.title.clone(),
        sheets: vec![
            Sheet {
                name: ONE_LINE,
                columns: cols,
                rows: out,
            },
            Sheet {
                name: "Day Totals",
                columns: columns_of(&["day", "date", "scenes", "eighths"]),
                rows: totals,
            },
        ],
    })
}

/// Every element with its sheets, page length, shoot days and dates on a sub-board.
pub fn element_list(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
) -> Result<Workbook, ExportError> {
    let list = reports::element_list(s, board, sub)?;
    let rows = list
        .iter()
        .map(|l| {
            vec![
                Cell::text(s.category(l.category).map_or("", |c| c.name.as_str())),
                Cell::text(l.board_id.as_deref().unwrap_or("")),
                Cell::text(l.name.as_str()),
                int(l.sheets),
                int(l.pages.0),
                pages_text(l.pages),
                int(l.days),
                Cell::date(l.start),
                Cell::date(l.finish),
            ]
        })
        .collect();
    Ok(Workbook {
        title: s.title.clone(),
        sheets: vec![Sheet {
            name: "Elements",
            columns: columns_of(&[
                "category", "board_id", "element", "sheets", "eighths", PAGES, "days", "start",
                "finish",
            ]),
            rows,
        }],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::export::{csv, xlsx};
    use method_schedule_core::days::{auto_day_breaks, Limit};
    use method_schedule_core::fountain;
    use method_schedule_core::model::{Banner, Minutes, Place, StripField, StripKind};
    use method_schedule_core::setup::ReportColumn;
    use method_schedule_core::Date;

    /// The Harbor Light script on the first sub-board, as `method-schedule` builds it in the CLI's
    /// end-to-end test, with a banner, board IDs and a start date.
    fn harbor() -> (Schedule, BoardId, SubBoardId, CategoryId) {
        let mut s = Schedule::new("ZTEST Harbor & Light");
        let script = fountain::parse(include_str!(
            "../../../../fixtures/scripts/harbor-light.fountain"
        ));
        let cast = s.category_named("Cast").unwrap();
        s.import_script(&script, cast).unwrap();
        let (b, sb) = (s.boards[0].id, s.boards[0].sub_boards[0].id);
        let ids: Vec<_> = s.boards[0].boneyard.iter().map(|x| x.id).collect();
        s.move_strips(
            b,
            &ids,
            Place {
                sub_board: Some(sb),
                index: 0,
            },
        )
        .unwrap();
        s.insert_strip(
            b,
            StripKind::Banner(Banner {
                text: "=Move to the harbor".into(),
                duration: Minutes(30),
                ..Default::default()
            }),
            Place {
                sub_board: Some(sb),
                index: 2,
            },
        )
        .unwrap();
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(3))).unwrap();
        let tove = s.element_named(cast, "TOVE").unwrap();
        let kit = s.element_named(cast, "KIT").unwrap();
        s.set_board_id(tove, Some("1")).unwrap();
        s.set_board_id(kit, Some("2")).unwrap();
        s.boards[0].sub_boards[0].start = "2027-03-01".parse().ok();
        (s, b, sb, cast)
    }

    fn d(s: &str) -> Cell {
        Cell::Date(s.parse::<Date>().unwrap())
    }

    /// Writes `wb`, reads it back, checks every cell matches, and returns the data rows.
    fn round_trip(wb: &Workbook) -> Vec<Vec<Cell>> {
        let bytes = xlsx::write(wb).unwrap();
        assert_eq!(xlsx::write(wb).unwrap(), bytes, "deterministic");
        let back = xlsx::read_cells(&bytes).unwrap();
        assert_eq!(back.len(), wb.sheets.len());
        for (sheet, read) in wb.sheets.iter().zip(&back) {
            assert_eq!(read.name, sheet.name);
            let header: Vec<Cell> = sheet.columns.iter().map(Cell::text).collect();
            assert_eq!(read.rows[0], header);
            assert_eq!(&read.rows[1..], &sheet.rows[..]);
        }
        back[0].rows[1..].to_vec()
    }

    fn find<'a>(rows: &'a [Vec<Cell>], name: &str) -> &'a [Cell] {
        rows.iter()
            .find(|r| r[2] == Cell::text(name))
            .unwrap_or_else(|| panic!("no row for {name}"))
    }

    #[test]
    fn dood_summary_round_trips() {
        let (s, b, sb, _) = harbor();
        let wb = dood_summary(&s, b, sb).unwrap();
        assert_eq!(wb.title, "ZTEST Harbor & Light");
        let rows = round_trip(&wb);
        let kit = find(&rows, "KIT");
        assert_eq!(kit[0], Cell::text("Cast"));
        assert_eq!(kit[1], Cell::text("2"));
        assert_eq!(kit[3], Cell::Int(3));
        assert_eq!(kit[5], d("2027-03-01"));
        assert_eq!(kit[6], d("2027-03-03"));
    }

    #[test]
    fn one_line_round_trips() {
        let (s, b, sb, cast) = harbor();
        let wb = one_line(&s, b, sb, Some(cast), OneLineColumns::Fixed).unwrap();
        let rows = round_trip(&wb);
        // 2/8 + 1/8 | banner, 2/8 + 1/8 | 1/8
        assert_eq!(rows[0][0], Cell::Int(1));
        assert_eq!(rows[0][1], d("2027-03-01"));
        assert_eq!(rows[0][2], Cell::text("scene"));
        assert_eq!(rows[0][5], Cell::text("NET LOFT"));
        assert_eq!(rows[0][7], Cell::Int(2));
        assert_eq!(rows[0][8], Cell::text("2/8"));
        assert_eq!(rows[0][10], Cell::text("1 2"));
        let banner = rows.iter().find(|r| r[2] == Cell::text("banner")).unwrap();
        assert_eq!(banner[0], Cell::Int(2));
        assert_eq!(banner[9], Cell::Int(30));
        // Formula-looking text stays text in a workbook.
        assert_eq!(banner[11], Cell::text("=Move to the harbor"));
    }

    /// Review: an omitted scene's eighths went into the Day Totals sheet, so a day's
    /// total there disagreed with the day's pages. It is listed with its own kind now
    /// and adds nothing, in both column sets.
    #[test]
    fn an_omitted_scene_is_listed_and_adds_nothing() {
        let (mut s, b, sb, cast) = harbor();
        let first = s.sheets[0].id;
        s.plan.omitted.push(first);
        for columns in [OneLineColumns::Fixed, OneLineColumns::Layout] {
            let wb = one_line(&s, b, sb, Some(cast), columns).unwrap();
            let rows = round_trip(&wb);
            let kind = wb.sheets[0]
                .columns
                .iter()
                .position(|c| c == "kind")
                .unwrap();
            let gone: Vec<&Vec<Cell>> = rows
                .iter()
                .filter(|r| r[kind] == Cell::text(OMITTED))
                .collect();
            assert_eq!(gone.len(), 1, "{columns:?}");
            // Day 1 was 2/8 + 1/8: only the 1/8 counts now, one scene.
            let days = &wb.sheets[1];
            assert_eq!(total(&days.rows[0][2]).0, 1, "{columns:?}");
            assert_eq!(total(&days.rows[0][3]).0, 1, "{columns:?}");
        }
    }

    /// The value a total shows, and its formula.
    fn total(c: &Cell) -> (u64, &str) {
        match c {
            Cell::Formula(f) => (f.value(), f.expr()),
            other => panic!("{other:?} is not a formula"),
        }
    }

    #[test]
    fn totals_are_working_formulas() {
        let (s, b, sb, cast) = harbor();
        let wb = one_line(&s, b, sb, Some(cast), OneLineColumns::Fixed).unwrap();
        round_trip(&wb);
        let days = &wb.sheets[1];
        assert_eq!(days.name, "Day Totals");
        assert_eq!(days.columns, ["day", "date", "scenes", "eighths"]);
        // Day 1: rows 2 and 3 of the one-line sheet, two scenes, 2/8 + 1/8.
        assert_eq!(days.rows[0][0], Cell::Int(1));
        assert_eq!(
            total(&days.rows[0][2]),
            (2, "COUNTIF('One-Line'!C2:C3,\"scene\")")
        );
        assert_eq!(total(&days.rows[0][3]), (3, "SUM('One-Line'!H2:H3)"));
        // Day 2 starts with the banner: three rows, two of them scenes.
        assert_eq!(total(&days.rows[1][3]), (3, "SUM('One-Line'!H4:H6)"));
        // The grand total adds the days up, and matches the one-line sheet.
        let last = days.rows.last().unwrap();
        assert_eq!(last[0], Cell::text("total"));
        let n = days.rows.len();
        assert_eq!(total(&last[3]).1, format!("SUM(D2:D{n})"));
        let eighths: u64 = wb.sheets[0]
            .rows
            .iter()
            .filter_map(|r| match r[7] {
                Cell::Int(n) => Some(n),
                _ => None,
            })
            .sum();
        assert_eq!(total(&last[3]).0, eighths);
        assert_eq!(total(&last[2]).0, 5);

        // The day out of days total is work plus hold.
        let wb = dood_summary(&s, b, sb).unwrap();
        let rows = round_trip(&wb);
        let kit = find(&rows, "KIT");
        let at = rows.iter().position(|r| r[2] == Cell::text("KIT")).unwrap();
        assert_eq!(
            total(&kit[7]),
            (3, format!("D{}+E{}", at + 2, at + 2).as_str())
        );
        // A CSV has the values.
        let text =
            String::from_utf8(csv::write(&wb.sheets[0], csv::Delimiter::Comma, false)).unwrap();
        assert!(
            text.contains(",KIT,3,0,2027-03-01,2027-03-03,3\r\n"),
            "{text}"
        );
    }

    #[test]
    fn one_line_has_a_column_per_shown_category_with_quantities() {
        let (mut s, b, sb, cast) = harbor();
        let props = s.add_category("Props");
        let needle = s.add_element(props, "Netting needle").unwrap();
        let first = s.sheets[0].id;
        s.tag(first, needle).unwrap();
        s.set_quantity(first, needle, 3, true).unwrap();
        let wb = one_line(&s, b, sb, Some(cast), OneLineColumns::Fixed).unwrap();
        let cols = &wb.sheets[0].columns;
        // The fixed columns stay where they were; the categories follow, in order.
        assert_eq!(cols[11], "text");
        assert_eq!(
            cols[12..],
            s.categories
                .iter()
                .map(|c| c.name.clone())
                .collect::<Vec<_>>()[..]
        );
        let at = |name: &str| cols.iter().position(|c| c == name).unwrap();
        let rows = round_trip(&wb);
        assert_eq!(rows[0][at("Cast")], Cell::text("1; 2"));
        assert_eq!(rows[0][at("Props")], Cell::text("Netting needle x3"));
        let banner = rows.iter().find(|r| r[2] == Cell::text("banner")).unwrap();
        assert_eq!(banner.len(), cols.len());
        assert!(banner[12..].iter().all(|c| *c == Cell::Empty));

        // By name, and only the categories the one-line report's filter shows.
        let mut o = one_line_options(&s);
        o.cast_names = true;
        o.categories = vec![cast];
        let wb = one_line_with(&s, b, sb, Some(cast), &o, OneLineColumns::Fixed).unwrap();
        assert_eq!(wb.sheets[0].columns[12..], ["Cast".to_string()]);
        assert_eq!(wb.sheets[0].rows[0][12], Cell::text("TOVE; KIT"));
        // The saved report setup is what the plain export follows.
        let mut list = s.report_list();
        list[0].options = o;
        s.set_report_list(list).unwrap();
        assert_eq!(
            one_line(&s, b, sb, Some(cast), OneLineColumns::Fixed).unwrap(),
            wb
        );
    }

    #[test]
    fn element_list_round_trips() {
        let (s, b, sb, _) = harbor();
        let wb = element_list(&s, b, sb).unwrap();
        let rows = round_trip(&wb);
        assert_eq!(rows[0][2], Cell::text("TOVE"));
        let kit = find(&rows, "KIT");
        assert_eq!(
            kit[3..7],
            [Cell::Int(4), Cell::Int(6), Cell::text("6/8"), Cell::Int(3)]
        );
        assert_eq!(kit[7], d("2027-03-01"));
        assert_eq!(kit[8], d("2027-03-03"));
    }

    #[test]
    fn csv_leaves_out_the_pages_text_and_neutralizes_formulas() {
        let (s, b, sb, cast) = harbor();
        let wb = one_line(&s, b, sb, Some(cast), OneLineColumns::Fixed).unwrap();
        let text = String::from_utf8(csv::write(
            &wb.sheets[0].without(PAGES),
            csv::Delimiter::Comma,
            false,
        ))
        .unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(
            lines[0],
            "day,date,kind,scenes,int_ext,set,time_of_day,eighths,minutes,cast,text,Cast"
        );
        assert!(lines[1].starts_with("1,2027-03-01,scene,1,INT,NET LOFT,NIGHT,2,,1 2,"));
        assert!(
            lines.contains(&"2,2027-03-02,banner,,,,,,30,,'=Move to the harbor,"),
            "{text}"
        );
    }

    #[test]
    fn one_line_follows_the_strip_layout_by_default() {
        let (mut s, b, sb, cast) = harbor();
        let wb = one_line(&s, b, sb, Some(cast), OneLineColumns::default()).unwrap();
        let sheet = &wb.sheets[0];
        assert_eq!(
            sheet.columns,
            [
                BREAKS,
                "Scene",
                "I/E",
                "Set",
                "Time of day",
                "Pages",
                "Cast IDs",
                "Cast",
                "kind",
                "eighths"
            ]
        );
        let rows = round_trip(&wb);
        // The first scene: the strip's cells as the board shows them, as text.
        assert_eq!(rows[0][0], Cell::Empty);
        assert_eq!(rows[0][3], Cell::text("NET LOFT"));
        assert_eq!(rows[0][5], Cell::text("2/8"));
        assert_eq!(rows[0][6], Cell::text("1, 2"));
        assert_eq!(rows[0][7], Cell::text("1; 2"));
        assert_eq!(rows[0][8..], [Cell::text("scene"), Cell::Int(2)]);
        // Day 1 closes with its day break: end, date, pages, time, a cell each.
        assert_eq!(rows[2][0], Cell::text("End of Day 1"));
        assert_eq!(rows[2][1], Cell::text("2027-03-01"));
        assert_eq!(rows[2][2], Cell::text("3/8 pages"));
        assert_eq!(rows[2][8], Cell::text("day_break"));
        assert_eq!(rows[2][9], Cell::Empty);
        // The banner's text sits in the first column.
        assert_eq!(rows[3][0], Cell::text("=Move to the harbor"));
        assert_eq!(rows[3][8], Cell::text("banner"));
        // The day totals read the working columns, day breaks included in the range.
        let days = &wb.sheets[1];
        assert_eq!(
            total(&days.rows[0][2]),
            (2, "COUNTIF('One-Line'!I2:I4,\"scene\")")
        );
        assert_eq!(total(&days.rows[0][3]), (3, "SUM('One-Line'!J2:J4)"));
        assert_eq!(total(&days.rows.last().unwrap()[2]).0, 5);
        // The fixed columns, the totals the same.
        let fixed = one_line(&s, b, sb, Some(cast), OneLineColumns::Fixed).unwrap();
        assert_eq!(fixed.sheets[0].columns[0], "day");
        let values = |w: &Workbook| -> Vec<u64> {
            w.sheets[1]
                .rows
                .iter()
                .flat_map(|r| r[2..].iter().map(|c| total(c).0))
                .collect()
        };
        assert_eq!(values(&wb), values(&fixed));

        // The report's own columns, and no day breaks when it shows none.
        let mut o = one_line_options(&s);
        o.columns = vec![ReportColumn::of(StripField::Set)];
        o.day_breaks = false;
        let mut list = s.report_list();
        list[0].options = o;
        s.set_report_list(list).unwrap();
        let wb = one_line(&s, b, sb, Some(cast), OneLineColumns::Layout).unwrap();
        assert_eq!(
            wb.sheets[0].columns,
            [BREAKS, "Set", "Cast", "kind", "eighths"]
        );
        assert!(wb.sheets[0]
            .rows
            .iter()
            .all(|r| r[3] != Cell::text("day_break")));
        // A CSV keeps the layout's page text.
        let text = String::from_utf8(csv::write(
            &wb.sheets[0].without(PAGES),
            csv::Delimiter::Comma,
            false,
        ))
        .unwrap();
        assert!(
            text.starts_with("Day breaks and banners,Set,Cast,kind,eighths\r\n"),
            "{text}"
        );
    }

    #[test]
    fn a_narrow_layout_joins_the_last_day_break_cells() {
        let (s, b, sb, cast) = harbor();
        let mut o = one_line_options(&s);
        o.columns = vec![ReportColumn::of(StripField::Set)];
        // The break column, Set and Cast before `kind`: room for three cells.
        let wb = one_line_with(&s, b, sb, Some(cast), &o, OneLineColumns::Layout).unwrap();
        let brk = &wb.sheets[0].rows[2];
        assert_eq!(brk[0], Cell::text("End of Day 1"));
        assert_eq!(brk[1], Cell::text("2027-03-01"));
        let Cell::Text(t) = &brk[2] else {
            panic!("{brk:?}")
        };
        assert!(t.starts_with("3/8 pages  "), "{t}");
    }

    #[test]
    fn a_missing_board_is_an_error() {
        let (s, _, sb, _) = harbor();
        assert!(matches!(
            dood_summary(&s, BoardId(9999), sb),
            Err(ExportError::Schedule(_))
        ));
    }
}
