//! Report data: what each report shows, as plain values. Layout and output formats live
//! elsewhere (CLI tables, CSV, and later xlsx and PDF from the app).
//!
//! Report contents follow common industry practice (inventory section 11); we never copy
//! another product's report layout.

use crate::days::days_with;
use crate::dood::dood;
use crate::model::{
    BannerStyle, BoardId, CategoryId, ElementId, Error, Minutes, Schedule, Sheet, SheetId,
    StripField, StripKind, SubBoardId,
};
use crate::{Date, Eighths};

/// One line of a one-line schedule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OneLine {
    Scene {
        /// The scene's breakdown sheet (for a strip layout's cells, `strip_cell`).
        sheet: SheetId,
        scenes: String,
        int_ext: String,
        set: String,
        time_of_day: String,
        pages: Eighths,
        synopsis: String,
        /// Board IDs (or names, when an element has no ID) of the cast on the sheet.
        cast: Vec<String>,
    },
    Banner {
        text: String,
        duration: Minutes,
        /// Its look on the board (bold, color, size, alignment), for reports drawn as the board is.
        style: BannerStyle,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OneLineDay {
    pub number: u32,
    /// `None` when the sub-board has no start date.
    pub date: Option<Date>,
    pub lines: Vec<OneLine>,
    pub pages: Eighths,
    pub time: Minutes,
}

/// The one-line schedule of a sub-board: each shoot day with its strips in board order.
/// `cast` names the category whose elements appear on each line (usually Cast).
pub fn one_line(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
    cast: Option<CategoryId>,
) -> Result<Vec<OneLineDay>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let mut out = Vec::new();
    let index = s.sheet_index();
    for day in days_with(s, &index, board, sub)?.days {
        let date = sb
            .start
            .and_then(|st| sb.calendar.date_of_day(st, day.number));
        let mut lines = Vec::new();
        for e in &day.entries {
            let strip = &sb.strips[e.at];
            match &strip.kind {
                StripKind::Sheet(sheet) => {
                    let h = *index.get(sheet).ok_or(Error::NoSuchSheet(*sheet))?;
                    lines.push(OneLine::Scene {
                        sheet: h.id,
                        scenes: h.scenes.clone(),
                        int_ext: h.int_ext.clone(),
                        set: h.set.clone(),
                        time_of_day: h.time_of_day.clone(),
                        pages: h.pages,
                        synopsis: h.synopsis.clone(),
                        cast: cast_ids(s, h, cast),
                    });
                }
                StripKind::Banner(bn) => lines.push(OneLine::Banner {
                    text: bn.text.clone(),
                    duration: bn.duration,
                    style: bn.style,
                }),
                StripKind::DayBreak => {}
            }
        }
        out.push(OneLineDay {
            number: day.number,
            date,
            lines,
            pages: day.pages,
            time: day.time,
        });
    }
    Ok(out)
}

/// What a scene strip shows for `field` (the strip layout, decision 39): the sheet's
/// text as typed, pages as eighths text (`1 3/8`), the shoot time as `h:mm`, and for
/// `Cast` the board IDs (or names, where an element has none) of `cast`'s elements on the
/// sheet in board ID order, joined by `", "`.
pub fn strip_cell(
    s: &Schedule,
    sheet: &Sheet,
    field: StripField,
    cast: Option<CategoryId>,
) -> String {
    let ids = match field {
        StripField::Cast | StripField::CastCount => cast_ids(s, sheet, cast),
        _ => Vec::new(),
    };
    strip_cell_with(s, sheet, field, &ids)
}

/// What a strip layout's column shows on a scene strip: [`strip_cell_with`], but an
/// element count of one category shows the category's name and how many of its elements
/// the sheet has, as `Cast Members (4)`, and `Props (0)` with none (BRD-18, behavior
/// note 27, decision 147). Elements left off the board are not counted, as in the other
/// counts (our reading, E-S34).
pub fn column_cell(
    s: &Schedule,
    sheet: &Sheet,
    column: &crate::model::StripColumn,
    ids: &[String],
) -> String {
    if column.field == StripField::ElementCount {
        if let Some(cat) = column.category.and_then(|c| s.category(c)) {
            let n = sheet
                .elements
                .iter()
                .filter_map(|e| s.element(*e))
                .filter(|e| e.category == cat.id && !s.element_excluded(e))
                .count();
            return format!("{} ({n})", cat.name);
        }
    }
    if column.field == StripField::CategoryElements {
        return category_elements(s, sheet, column);
    }
    strip_cell_with(s, sheet, column.field, ids)
}

/// What a column of a category's elements shows (decision 194): the sheet's elements of
/// the column's category in board ID order, then name, by name or, with the column's
/// switch, by board ID (the name where an element has none), joined by `", "`. Elements
/// left off the board are left out, as everywhere on strips. Empty with no category.
pub fn category_elements(
    s: &Schedule,
    sheet: &Sheet,
    column: &crate::model::StripColumn,
) -> String {
    let Some(cat) = column.category.filter(|c| s.category(*c).is_some()) else {
        return String::new();
    };
    let mut els: Vec<&crate::model::Element> = sheet
        .elements
        .iter()
        .filter_map(|e| s.element(*e))
        .filter(|e| e.category == cat && !s.element_excluded(e))
        .collect();
    els.sort_by(|a, b| {
        id_order(a.board_id.as_deref(), b.board_id.as_deref()).then_with(|| a.name.cmp(&b.name))
    });
    els.iter()
        .map(|e| match &e.board_id {
            Some(id) if column.board_ids => id.as_str(),
            _ => e.name.as_str(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// What a report shows in a column of the board's layout: [`strip_cell`], but a
/// category's elements as the board shows them (decision 194). A report's element count
/// counts every element (decision 147), so its category is not used.
pub fn report_cell(
    s: &Schedule,
    sheet: &Sheet,
    column: &crate::model::StripColumn,
    cast: Option<CategoryId>,
) -> String {
    if column.field == StripField::CategoryElements {
        return category_elements(s, sheet, column);
    }
    strip_cell(s, sheet, column.field, cast)
}

/// [`strip_cell`] with the sheet's [`cast_ids`] worked out once by the caller, for a
/// strip of many cells.
pub fn strip_cell_with(s: &Schedule, sheet: &Sheet, field: StripField, ids: &[String]) -> String {
    match field {
        StripField::Scene => sheet.scenes.clone(),
        StripField::IntExt => sheet.int_ext.clone(),
        StripField::Set => sheet.set.clone(),
        StripField::TimeOfDay => sheet.time_of_day.clone(),
        StripField::Pages => sheet.pages.to_string(),
        StripField::Cast => ids.join(", "),
        StripField::Location => sheet.location.clone(),
        StripField::ScriptDay => sheet.script_day.clone(),
        StripField::Unit => sheet.unit.clone(),
        StripField::Sequence => sheet.sequence.clone(),
        StripField::Episode => sheet.episode.clone(),
        StripField::ShootTime => sheet
            .shoot_time
            .map(|m| format!("{}:{:02}", m.0 / 60, m.0 % 60))
            .unwrap_or_default(),
        StripField::Synopsis => sheet.synopsis.clone(),
        // Resource counts (decision 66): empty rather than 0, so a strip stays quiet.
        StripField::CastCount => match ids.len() {
            0 => String::new(),
            n => n.to_string(),
        },
        StripField::ElementCount => {
            let n = sheet
                .elements
                .iter()
                .filter_map(|e| s.element(*e))
                .filter(|e| s.category(e.category).is_some() && !s.element_excluded(e))
                .count();
            if n == 0 {
                String::new()
            } else {
                n.to_string()
            }
        }
        StripField::Elements => {
            let cast = s.cast();
            let mut els: Vec<&crate::model::Element> = sheet
                .elements
                .iter()
                .filter_map(|e| s.element(*e))
                .filter(|e| Some(e.category) != cast)
                .filter(|e| s.category(e.category).is_some() && !s.element_excluded(e))
                .collect();
            let at = |c: CategoryId| s.categories.iter().position(|x| x.id == c);
            els.sort_by(|a, b| {
                at(a.category)
                    .cmp(&at(b.category))
                    .then_with(|| id_order(a.board_id.as_deref(), b.board_id.as_deref()))
                    .then_with(|| a.name.cmp(&b.name))
            });
            els.iter()
                .map(|e| e.board_id.clone().unwrap_or_else(|| e.name.clone()))
                .collect::<Vec<_>>()
                .join(", ")
        }
        // Nothing of the sheet: a spacer is empty and a custom text is the column's own,
        // the same on every strip, so the layout carries it rather than each strip. A
        // category's elements needs the column's category: [`column_cell`].
        StripField::Spacer | StripField::Text | StripField::CategoryElements => String::new(),
    }
}

/// The board IDs (or names) of `cast`'s elements on a sheet, in board ID order, leaving
/// out the elements left off the board (decisions 62, 105 and 120).
pub fn cast_ids(s: &Schedule, sheet: &Sheet, cast: Option<CategoryId>) -> Vec<String> {
    cast_ids_by(sheet, cast, |e| s.element(e))
}

/// [`cast_ids`] for a whole board: the cast's elements on the board put in board ID order
/// once, so each strip only sorts its own by their place in that order (decision 190).
pub struct CastOrder<'a> {
    /// Each cast element shown on the board: its place in board ID order (elements that
    /// sort the same share one) and what the strip shows for it.
    place: crate::model::IdMap<ElementId, (u32, &'a str)>,
}

impl<'a> CastOrder<'a> {
    pub fn new(s: &'a Schedule, cast: Option<CategoryId>) -> CastOrder<'a> {
        let mut els: Vec<&crate::model::Element> = s
            .elements
            .iter()
            .filter(|e| Some(e.category) == cast && !e.exclude_from_board)
            .collect();
        els.sort_by(|a, b| cast_order(a, b));
        let mut place = crate::model::IdMap::default();
        let mut at = 0;
        for (i, e) in els.iter().enumerate() {
            if i > 0 && cast_order(els[i - 1], e).is_ne() {
                at = i as u32;
            }
            place.insert(e.id, (at, e.board_id.as_deref().unwrap_or(&e.name)));
        }
        CastOrder { place }
    }

    /// [`cast_ids`] of `sheet`.
    pub fn ids(&self, sheet: &Sheet) -> Vec<String> {
        let mut on: Vec<(u32, &str)> = Vec::with_capacity(sheet.elements.len());
        on.extend(sheet.elements.iter().filter_map(|e| self.place.get(e)));
        // Stable, so elements in the same place keep the sheet's order, as in cast_ids.
        on.sort_by_key(|x| x.0);
        on.into_iter().map(|(_, text)| text.to_string()).collect()
    }
}

/// The order [`cast_ids`] puts elements in.
fn cast_order(a: &crate::model::Element, b: &crate::model::Element) -> std::cmp::Ordering {
    id_order(a.board_id.as_deref(), b.board_id.as_deref()).then(a.name.cmp(&b.name))
}

fn cast_ids_by<'a>(
    sheet: &Sheet,
    cast: Option<CategoryId>,
    element: impl Fn(ElementId) -> Option<&'a crate::model::Element>,
) -> Vec<String> {
    let mut els: Vec<&crate::model::Element> = sheet
        .elements
        .iter()
        .filter_map(|e| element(*e))
        // An element's own setting decides (decision 120).
        .filter(|e| Some(e.category) == cast && !e.exclude_from_board)
        .collect();
    els.sort_by(|a, b| cast_order(a, b));
    els.into_iter()
        .map(|e| e.board_id.as_ref().unwrap_or(&e.name).clone())
        .collect()
}

/// Board IDs in natural order (behavior note 17, A1 and A2): the leading run of digits
/// by its value, IDs with no leading digits after every ID with them, then the rest of
/// the text, then the ID itself so `7` comes before `07`; no ID sorts last. So 1, 1a, 2,
/// 7, 07, 9, 10, X. Callers break a tie (the same ID) by name.
pub fn id_order(a: Option<&str>, b: Option<&str>) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    fn split(id: &str) -> (&str, &str) {
        let n = id.bytes().take_while(u8::is_ascii_digit).count();
        id.split_at(n)
    }
    /// Two runs of digits by value, however long, without parsing.
    fn by_value(x: &str, y: &str) -> Ordering {
        let (x, y) = (x.trim_start_matches('0'), y.trim_start_matches('0'));
        x.len().cmp(&y.len()).then_with(|| x.cmp(y))
    }
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(x), Some(y)) => {
            let ((xd, xr), (yd, yr)) = (split(x), split(y));
            let number = match (xd.is_empty(), yd.is_empty()) {
                (false, false) => by_value(xd, yd),
                (false, true) => Ordering::Less,
                (true, false) => Ordering::Greater,
                (true, true) => Ordering::Equal,
            };
            number
                .then_with(|| xr.cmp(yr))
                .then_with(|| xd.len().cmp(&yd.len()))
                .then_with(|| x.cmp(y))
        }
    }
}

/// One line of the element list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementLine {
    pub element: ElementId,
    pub category: CategoryId,
    pub board_id: Option<String>,
    pub name: String,
    /// Sheets the element is on, anywhere in the schedule, omitted scenes left out.
    pub sheets: u32,
    /// Their total page length.
    pub pages: Eighths,
    /// Shoot days on the sub-board (work days only).
    pub days: u32,
    pub start: Option<Date>,
    pub finish: Option<Date>,
}

/// Every element, grouped by category in category order, then by board ID and name.
/// Day counts and dates come from the given sub-board; elements it doesn't schedule show
/// none.
pub fn element_list(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
) -> Result<Vec<ElementLine>, Error> {
    let rows = dood(s, board, sub).ok();
    let mut out = Vec::new();
    for cat in &s.categories {
        let mut lines: Vec<ElementLine> = s
            .elements
            .iter()
            .filter(|e| e.category == cat.id)
            .map(|e| {
                // An omitted scene counts nowhere (decision 91).
                let on: Vec<_> = s
                    .sheets
                    .iter()
                    .filter(|h| h.elements.contains(&e.id) && !s.plan.is_omitted(h.id))
                    .collect();
                let row = rows
                    .as_ref()
                    .and_then(|r| r.iter().find(|x| x.element == e.id));
                let work_days = row.map_or(0, |r| {
                    r.cells.iter().filter(|(_, c)| c.is_work()).count() as u32
                });
                ElementLine {
                    element: e.id,
                    category: cat.id,
                    board_id: e.board_id.clone(),
                    name: e.name.clone(),
                    sheets: on.len() as u32,
                    pages: on.iter().map(|h| h.pages).sum(),
                    days: work_days,
                    start: row.and_then(|r| r.start),
                    finish: row.and_then(|r| r.finish),
                }
            })
            .collect();
        lines.sort_by(|a, b| {
            id_order(a.board_id.as_deref(), b.board_id.as_deref()).then_with(|| {
                a.name
                    .to_ascii_uppercase()
                    .cmp(&b.name.to_ascii_uppercase())
            })
        });
        out.extend(lines);
    }
    Ok(out)
}

/// One scene on a shooting schedule: the sheet's header with its elements by category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShootingScene {
    pub sheet: crate::model::SheetId,
    pub scenes: String,
    pub int_ext: String,
    pub set: String,
    pub time_of_day: String,
    pub pages: Eighths,
    pub synopsis: String,
    pub location: String,
    /// Categories in schedule order, each with its elements by board ID then name. Empty
    /// categories are left out. An element shows as `ID NAME`, or its name alone.
    pub elements: Vec<(CategoryId, Vec<String>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShootingLine {
    Scene(ShootingScene),
    Banner {
        text: String,
        duration: Minutes,
        /// Its look on the board, as for the one-line schedule.
        style: BannerStyle,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShootingDay {
    pub number: u32,
    pub date: Option<Date>,
    pub lines: Vec<ShootingLine>,
    pub pages: Eighths,
    pub time: Minutes,
}

/// The shooting schedule of a sub-board: each day's scenes with all their elements.
pub fn shooting_schedule(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
) -> Result<Vec<ShootingDay>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let mut out = Vec::new();
    let (index, element_index) = (s.sheet_index(), s.element_index());
    for day in days_with(s, &index, board, sub)?.days {
        let date = sb
            .start
            .and_then(|st| sb.calendar.date_of_day(st, day.number));
        let mut lines = Vec::new();
        for e in &day.entries {
            let strip = &sb.strips[e.at];
            match &strip.kind {
                StripKind::Sheet(sheet) => {
                    let h = *index.get(sheet).ok_or(Error::NoSuchSheet(*sheet))?;
                    let mut elements = Vec::new();
                    for cat in &s.categories {
                        let mut els: Vec<_> = h
                            .elements
                            .iter()
                            .filter_map(|e| element_index.get(e).copied())
                            .filter(|e| e.category == cat.id)
                            .collect();
                        if els.is_empty() {
                            continue;
                        }
                        els.sort_by(|a, b| {
                            id_order(a.board_id.as_deref(), b.board_id.as_deref()).then_with(|| {
                                a.name
                                    .to_ascii_uppercase()
                                    .cmp(&b.name.to_ascii_uppercase())
                            })
                        });
                        let names = els
                            .iter()
                            .map(|e| match &e.board_id {
                                Some(id) => format!("{id} {}", e.name),
                                None => e.name.clone(),
                            })
                            .collect();
                        elements.push((cat.id, names));
                    }
                    lines.push(ShootingLine::Scene(ShootingScene {
                        sheet: h.id,
                        scenes: h.scenes.clone(),
                        int_ext: h.int_ext.clone(),
                        set: h.set.clone(),
                        time_of_day: h.time_of_day.clone(),
                        pages: h.pages,
                        synopsis: h.synopsis.clone(),
                        location: h.location.clone(),
                        elements,
                    }));
                }
                StripKind::Banner(bn) => lines.push(ShootingLine::Banner {
                    text: bn.text.clone(),
                    duration: bn.duration,
                    style: bn.style,
                }),
                StripKind::DayBreak => {}
            }
        }
        out.push(ShootingDay {
            number: day.number,
            date,
            lines,
            pages: day.pages,
            time: day.time,
        });
    }
    Ok(out)
}

/// How many of each category a shoot day uses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CategoryTotals {
    pub number: u32,
    pub date: Option<Date>,
    /// One count per category, in schedule order, zeros included.
    pub counts: Vec<(CategoryId, u32)>,
    /// The categories whose count leaves out a quantity a sheet marked not to count
    /// daily: a partial total, shown with an asterisk (inventory section 3).
    pub partial: Vec<CategoryId>,
}

/// A day's quantity of one element, decision 63's one rule: the sum over the day's
/// sheets of the element's quantity on each (one when none is given), leaving out those
/// a sheet marks not to count daily; `partial` when one was left out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayQuantity {
    pub element: ElementId,
    pub category: CategoryId,
    pub count: u32,
    pub partial: bool,
}

/// [`DayQuantity`] for every element of a category that uses daily quantities on any of
/// `sheets`, in first-seen order. Elements of any other category count once a day and
/// are left out. The one place the rule is worked out: the day break, the daily totals
/// and the breakdown report by day all read it.
pub fn day_quantities(
    s: &Schedule,
    elements: &crate::model::ElementIndex<'_>,
    sheets: &[&Sheet],
) -> Vec<DayQuantity> {
    let daily: std::collections::HashSet<CategoryId> = s
        .categories
        .iter()
        .filter(|c| c.daily_quantity)
        .map(|c| c.id)
        .collect();
    let mut out: Vec<DayQuantity> = Vec::new();
    if daily.is_empty() {
        return out;
    }
    let mut at: std::collections::HashMap<ElementId, usize> = std::collections::HashMap::new();
    for h in sheets {
        for e in &h.elements {
            let Some(x) = elements.get(e).filter(|x| daily.contains(&x.category)) else {
                continue;
            };
            let i = *at.entry(*e).or_insert_with(|| {
                out.push(DayQuantity {
                    element: *e,
                    category: x.category,
                    count: 0,
                    partial: false,
                });
                out.len() - 1
            });
            match h.quantity(*e) {
                (q, true) => out[i].count = out[i].count.saturating_add(q),
                (_, false) => out[i].partial = true,
            }
        }
    }
    out
}

/// Each daily-quantity category's total over `quantities`, in schedule order: (category,
/// total, partial), those with none on the day left out.
fn category_quantities(s: &Schedule, quantities: &[DayQuantity]) -> Vec<(CategoryId, u32, bool)> {
    s.categories
        .iter()
        .filter(|c| c.daily_quantity)
        .filter_map(|c| {
            let mut mine = quantities.iter().filter(|q| q.category == c.id).peekable();
            mine.peek()?;
            let (n, partial) = mine.fold((0u32, false), |(n, p), q| {
                (n.saturating_add(q.count), p || q.partial)
            });
            Some((c.id, n, partial))
        })
        .collect()
}

/// What a day break shows for the day's element quantities (decision 87): each category
/// that uses daily quantities with any on the day, in schedule order, as `Background 45`
/// (an asterisk when partial), joined by `" · "`. Empty when there are none.
pub fn day_quantity_text(s: &Schedule, sheets: &[&Sheet]) -> String {
    day_quantity_text_with(s, &s.element_index(), sheets)
}

/// [`day_quantity_text`] with the elements already indexed (once for a whole board).
pub fn day_quantity_text_with(
    s: &Schedule,
    elements: &crate::model::ElementIndex<'_>,
    sheets: &[&Sheet],
) -> String {
    // An omitted scene counts nowhere (decision 91).
    let sheets: Vec<&Sheet> = sheets
        .iter()
        .copied()
        .filter(|h| !s.plan.is_omitted(h.id))
        .collect();
    let q = day_quantities(s, elements, &sheets);
    category_quantities(s, &q)
        .into_iter()
        .filter(|(_, n, partial)| *n > 0 || *partial)
        .filter_map(|(c, n, partial)| {
            let star = if partial { "*" } else { "" };
            s.category(c).map(|c| format!("{} {n}{star}", c.name))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// Daily category totals for a sub-board (inventory section 3). For a category that
/// uses daily quantities, the day's total is the sum, over the day's sheets, of each of
/// its elements' quantity on the sheet (one when none is given), leaving out quantities
/// a sheet marks not to count daily; the total is then partial. For any other category,
/// each element counts once a day, however many sheets it is on.
pub fn daily_category_totals(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
) -> Result<Vec<CategoryTotals>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let mut out = Vec::new();
    let (index, elements) = (s.sheet_index(), s.element_index());
    for day in crate::days::days_with(s, &index, board, sub)?.days {
        let mut used = std::collections::BTreeSet::new();
        let mut sheets: Vec<&Sheet> = Vec::new();
        // An omitted scene counts nowhere (decision 91).
        for sheet in day.counted_sheets() {
            if let Some(h) = index.get(&sheet) {
                used.extend(h.elements.iter().copied());
                sheets.push(h);
            }
        }
        let daily = category_quantities(s, &day_quantities(s, &elements, &sheets));
        let mut partial = Vec::new();
        let counts = s
            .categories
            .iter()
            .map(|c| {
                if !c.daily_quantity {
                    let mine =
                        |e: &&ElementId| elements.get(*e).is_some_and(|x| x.category == c.id);
                    return (c.id, used.iter().filter(mine).count() as u32);
                }
                let (n, left_out) = daily
                    .iter()
                    .find(|(x, ..)| *x == c.id)
                    .map_or((0, false), |(_, n, p)| (*n, *p));
                if left_out {
                    partial.push(c.id);
                }
                (c.id, n)
            })
            .collect();
        out.push(CategoryTotals {
            partial,
            number: day.number,
            date: sb
                .start
                .and_then(|st| sb.calendar.date_of_day(st, day.number)),
            counts,
        });
    }
    Ok(out)
}

/// A breakdown header field that works as a category of its own (inventory section 4;
/// behavior note 04): each value it takes on the sheets is a row with sheets, pages,
/// work days and dates, in the Elements tab and the day out of days. Derived from the
/// sheets, never stored, never edited as an element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderField {
    Set,
    Location,
    Unit,
    Episode,
    IntExt,
    TimeOfDay,
    ScriptDay,
    Sequence,
}

impl HeaderField {
    /// Every header field with one value per sheet (behavior note 24: the reference app's
    /// list shows each of them), the first four as before (decision 118).
    pub const ALL: [HeaderField; 8] = [
        HeaderField::Set,
        HeaderField::Location,
        HeaderField::Unit,
        HeaderField::Episode,
        HeaderField::IntExt,
        HeaderField::TimeOfDay,
        HeaderField::ScriptDay,
        HeaderField::Sequence,
    ];

    /// `set`, `location`, `unit`, `episode`, `int_ext`, `time_of_day`, `script_day`,
    /// `sequence`: the sheet's own field names.
    pub fn key(self) -> &'static str {
        match self {
            HeaderField::Set => "set",
            HeaderField::Location => "location",
            HeaderField::Unit => "unit",
            HeaderField::Episode => "episode",
            HeaderField::IntExt => "int_ext",
            HeaderField::TimeOfDay => "time_of_day",
            HeaderField::ScriptDay => "script_day",
            HeaderField::Sequence => "sequence",
        }
    }

    pub fn from_key(key: &str) -> Option<HeaderField> {
        HeaderField::ALL.into_iter().find(|f| f.key() == key)
    }

    /// `Set`, `Location`, `Unit`, `Episode`, `I/E`, `Time of day`, `Script day`,
    /// `Sequence`.
    pub fn label(self) -> &'static str {
        match self {
            HeaderField::Set => "Set",
            HeaderField::Location => "Location",
            HeaderField::Unit => "Unit",
            HeaderField::Episode => "Episode",
            HeaderField::IntExt => "I/E",
            HeaderField::TimeOfDay => "Time of day",
            HeaderField::ScriptDay => "Script day",
            HeaderField::Sequence => "Sequence",
        }
    }

    /// The sheet's value of the field, trimmed.
    pub fn value(self, h: &Sheet) -> &str {
        match self {
            HeaderField::Set => h.set.trim(),
            HeaderField::Location => h.location.trim(),
            HeaderField::Unit => h.unit.trim(),
            HeaderField::Episode => h.episode.trim(),
            HeaderField::IntExt => h.int_ext.trim(),
            HeaderField::TimeOfDay => h.time_of_day.trim(),
            HeaderField::ScriptDay => h.script_day.trim(),
            HeaderField::Sequence => h.sequence.trim(),
        }
    }

    /// The sheet's own text for the field, to set it.
    pub fn slot(self, h: &mut Sheet) -> &mut String {
        match self {
            HeaderField::Set => &mut h.set,
            HeaderField::Location => &mut h.location,
            HeaderField::Unit => &mut h.unit,
            HeaderField::Episode => &mut h.episode,
            HeaderField::IntExt => &mut h.int_ext,
            HeaderField::TimeOfDay => &mut h.time_of_day,
            HeaderField::ScriptDay => &mut h.script_day,
            HeaderField::Sequence => &mut h.sequence,
        }
    }
}

/// The spelling a value typed in a header field of `sheet` takes: the value another
/// sheet already has in that field when it matches ignoring case (the first met in sheet
/// order, as [`header_category`] spells it), else the value as typed, trimmed. Typing
/// `room 2` where a sheet has `ROOM 2` gives `ROOM 2` (behavior note 25, E-L12;
/// decisions.md, entry 155). The sheet's own value does not count, so a value only it
/// has can be spelled again.
pub fn header_spelling(s: &Schedule, field: HeaderField, sheet: SheetId, typed: &str) -> String {
    let t = typed.trim();
    if t.is_empty() {
        return String::new();
    }
    s.sheets
        .iter()
        .filter(|h| h.id != sheet)
        .map(|h| field.value(h))
        .find(|v| crate::model::same_name(v, t))
        .unwrap_or(t)
        .to_string()
}

/// Sets header field `field` of `sheet` to `typed` in the spelling [`header_spelling`]
/// gives. False when there is no such sheet.
pub fn set_header_value(s: &mut Schedule, sheet: SheetId, field: HeaderField, typed: &str) -> bool {
    let v = header_spelling(s, field, sheet, typed);
    match s.sheet_mut(sheet) {
        Some(h) => {
            *field.slot(h) = v;
            true
        }
        None => false,
    }
}

/// A header field as a category: a copy of the schedule where every value the field
/// takes is an element of one more category, on the sheets that have that value
/// (values that differ only in case are one, spelled as first met). Run
/// [`element_list`] or [`crate::dood::dood`] on `schedule` and keep the rows of
/// `category` for the field's counts and day out of days; the element names are the
/// values. Rules are the defaults (no holds), events and red flags none.
pub struct HeaderCategory {
    pub schedule: Schedule,
    pub category: CategoryId,
}

pub fn header_category(s: &Schedule, field: HeaderField) -> HeaderCategory {
    let mut x = s.clone();
    for b in &mut x.boards {
        for sb in &mut b.sub_boards {
            sb.calendar.events.clear();
        }
    }
    // Pushed directly, so a user category of the same name stays apart.
    let category = CategoryId(x.fresh());
    x.categories.push(crate::model::Category {
        id: category,
        name: field.label().to_string(),
        auto_board_id: false,
        exclude_from_board: false,
        defaults: crate::model::DoodRules::default(),
        cast: false,
        daily_quantity: false,
        properties: Vec::new(),
        multi_line: Vec::new(),
    });
    for i in 0..x.sheets.len() {
        let value = field.value(&x.sheets[i]).to_string();
        if value.is_empty() {
            continue;
        }
        let id = match x.element_named(category, &value) {
            Some(id) => id,
            None => x.add_element(category, &value).expect("a new name"),
        };
        x.sheets[i].elements.push(id);
    }
    HeaderCategory {
        schedule: x,
        category,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::{auto_day_breaks, Limit};
    use crate::fountain;
    use crate::model::{Banner, Place};

    /// The board-wide cast order gives every strip the same list as `cast_ids`: board IDs
    /// in natural order, names where an element has none, left-off elements left out
    /// (decision 190).
    #[test]
    fn cast_order_matches_cast_ids() {
        let mut s = crate::sample::sample(300);
        let cast = s.cast();
        let ids: Vec<ElementId> = s
            .elements
            .iter()
            .filter(|e| Some(e.category) == cast)
            .map(|e| e.id)
            .collect();
        for (i, id) in ids.iter().enumerate() {
            let e = s.element_mut(*id).unwrap();
            match i % 7 {
                0 => e.board_id = None,
                1 => e.exclude_from_board = true,
                2 => e.board_id = Some(format!("0{i}")),
                3 => e.board_id = Some(format!("{}a", i / 2)),
                4 => e.board_id = Some("X".to_string()),
                _ => {}
            }
        }
        let order = CastOrder::new(&s, cast);
        let mut seen = 0;
        for h in &s.sheets {
            let want = cast_ids(&s, h, cast);
            seen += want.len();
            assert_eq!(order.ids(h), want, "sheet {}", h.id.0);
        }
        assert!(seen > 300);
        assert!(CastOrder::new(&s, None).ids(&s.sheets[0]).is_empty());
    }

    fn harbor() -> (Schedule, BoardId, SubBoardId, CategoryId) {
        let mut s = Schedule::new("ZTEST Harbor Light");
        let script = fountain::parse(include_str!(
            "../../../fixtures/scripts/harbor-light.fountain"
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
                text: "Move to the harbor".into(),
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

    #[test]
    fn strip_cells_show_each_field_as_the_board_does() {
        let (mut s, _, _, cast) = harbor();
        let id = s.sheets[0].id;
        {
            let h = s.sheet_mut(id).unwrap();
            h.location = "Boatyard loft".into();
            h.script_day = "D1".into();
            h.unit = "Main".into();
            h.sequence = "Opening".into();
            h.episode = "101".into();
            h.shoot_time = Some(Minutes(95));
            h.synopsis = "Tove mends the net.".into();
        }
        let h = s.sheet(id).unwrap().clone();
        let cells: Vec<String> = StripField::ALL
            .iter()
            .map(|f| strip_cell(&s, &h, *f, Some(cast)))
            .collect();
        assert_eq!(
            cells,
            [
                "1",
                "INT",
                "NET LOFT",
                "NIGHT",
                "2/8",
                "1, 2",
                "Boatyard loft",
                "D1",
                "Main",
                "Opening",
                "101",
                "1:35",
                "Tove mends the net.",
                "2",
                "2",
                "",
                "",
                "",
                ""
            ]
        );
        // No shoot time is empty; no cast category, no IDs.
        let mut bare = h.clone();
        bare.shoot_time = None;
        assert_eq!(strip_cell(&s, &bare, StripField::ShootTime, Some(cast)), "");
        assert_eq!(strip_cell(&s, &bare, StripField::Cast, None), "");
        // Resource counts (decision 66): none is empty, not 0.
        bare.elements.clear();
        assert_eq!(strip_cell(&s, &bare, StripField::CastCount, Some(cast)), "");
        assert_eq!(
            strip_cell(&s, &bare, StripField::ElementCount, Some(cast)),
            ""
        );
    }

    #[test]
    fn one_line_lists_days_scenes_banners_and_cast_ids() {
        let (s, b, sb, cast) = harbor();
        let days = one_line(&s, b, sb, Some(cast)).unwrap();
        // 2/8 + 1/8 | banner, 2/8 + 1/8 | 1/8
        assert_eq!(days.len(), 3);
        assert_eq!(days[0].date, "2027-03-01".parse().ok());
        assert_eq!(days[0].pages, Eighths(3));
        match &days[0].lines[0] {
            OneLine::Scene {
                scenes, set, cast, ..
            } => {
                assert_eq!((scenes.as_str(), set.as_str()), ("1", "NET LOFT"));
                // TOVE (1) before KIT (2), by board ID.
                assert_eq!(cast, &["1", "2"]);
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            &days[1].lines[0],
            OneLine::Banner {
                duration: Minutes(30),
                ..
            }
        ));
        // HARBORMASTER LUND has no board ID, so shows by name after the numbered cast.
        let OneLine::Scene { cast, .. } = &days[1].lines[1] else {
            panic!()
        };
        assert_eq!(cast, &["2", "HARBORMASTER LUND"]);
    }

    #[test]
    fn element_list_counts_sheets_pages_and_days() {
        let (s, b, sb, _) = harbor();
        let list = element_list(&s, b, sb).unwrap();
        let line = |n: &str| list.iter().find(|l| l.name == n).unwrap();
        let kit = line("KIT");
        assert_eq!((kit.sheets, kit.pages, kit.days), (4, Eighths(6), 3));
        assert_eq!(kit.start, "2027-03-01".parse().ok());
        assert_eq!(kit.finish, "2027-03-03".parse().ok());
        assert_eq!(list[0].name, "TOVE");
        assert_eq!(list[1].name, "KIT");
    }

    #[test]
    fn shooting_schedule_groups_elements_by_category() {
        let (mut s, b, sb, cast) = harbor();
        let props = s.add_category("Props");
        let needle = s.add_element(props, "Netting needle").unwrap();
        let first_sheet = s.sheets[0].id;
        s.tag(first_sheet, needle).unwrap();
        let days = shooting_schedule(&s, b, sb).unwrap();
        let ShootingLine::Scene(sc) = &days[0].lines[0] else {
            panic!()
        };
        assert_eq!(sc.set, "NET LOFT");
        assert_eq!(
            sc.elements,
            [
                (cast, vec!["1 TOVE".to_string(), "2 KIT".to_string()]),
                (props, vec!["Netting needle".to_string()])
            ]
        );
        assert!(matches!(days[1].lines[0], ShootingLine::Banner { .. }));
    }

    #[test]
    fn daily_quantities_add_up_and_a_left_out_one_marks_the_total_partial() {
        let (mut s, b, sb, cast) = harbor();
        let crowd = s.add_category("Background");
        s.category_mut(crowd).unwrap().daily_quantity = true;
        let dock = s.add_element(crowd, "Dock workers").unwrap();
        let gulls = s.add_element(crowd, "Gull keepers").unwrap();
        // Day 1 holds sheets 0 and 1.
        let (h0, h1) = (s.sheets[0].id, s.sheets[1].id);
        for h in [h0, h1] {
            s.tag(h, dock).unwrap();
        }
        s.tag(h1, gulls).unwrap();
        s.set_quantity(h0, dock, 20, true).unwrap();
        s.set_quantity(h1, dock, 20, false).unwrap();
        // What a day break shows (decision 87): the categories using daily quantities,
        // an asterisk on a partial total; nothing for a day without them.
        let both = [s.sheet(h0).unwrap(), s.sheet(h1).unwrap()];
        assert_eq!(day_quantity_text(&s, &both), "Background 21*");
        assert_eq!(day_quantity_text(&s, &both[..1]), "Background 20");
        assert_eq!(day_quantity_text(&s, &[]), "");
        let totals = daily_category_totals(&s, b, sb).unwrap();
        let day1 = &totals[0];
        let n = |c| day1.counts.iter().find(|(x, _)| *x == c).unwrap().1;
        // 20 dock workers counted once (the second sheet's 20 left out), 1 gull keeper.
        assert_eq!(n(crowd), 21);
        assert_eq!(day1.partial, [crowd]);
        // Cast doesn't use daily quantities: each element once.
        s.set_quantity(h0, s.element_named(cast, "KIT").unwrap(), 3, true)
            .unwrap();
        let totals = daily_category_totals(&s, b, sb).unwrap();
        assert_eq!(
            totals[0].counts.iter().find(|(c, _)| *c == cast).unwrap().1,
            2
        );
        // Counted on both sheets, it adds twice.
        s.set_quantity(h1, dock, 20, true).unwrap();
        let totals = daily_category_totals(&s, b, sb).unwrap();
        assert_eq!(
            totals[0]
                .counts
                .iter()
                .find(|(c, _)| *c == crowd)
                .unwrap()
                .1,
            41
        );
        assert!(totals[0].partial.is_empty());
    }

    #[test]
    fn a_category_left_off_the_board_shows_no_ids_on_strips() {
        let (mut s, b, sb, cast) = harbor();
        let h = s.sheets[0].clone();
        assert_eq!(strip_cell(&s, &h, StripField::Cast, Some(cast)), "1, 2");
        s.category_mut(cast).unwrap().exclude_from_board = true;
        assert_eq!(strip_cell(&s, &h, StripField::Cast, Some(cast)), "1, 2");
        // The category's setting reaches its elements when applied (decision 120).
        s.apply_defaults(cast).unwrap();
        assert_eq!(strip_cell(&s, &h, StripField::Cast, Some(cast)), "");
        let OneLine::Scene { cast: ids, .. } =
            &one_line(&s, b, sb, Some(cast)).unwrap()[0].lines[0]
        else {
            panic!()
        };
        assert!(ids.is_empty());
    }

    /// One element left off the board on its own (decision 105): its ID leaves the
    /// strips and the one-line, the rest of its category stays.
    #[test]
    fn an_element_left_off_the_board_shows_no_id() {
        let (mut s, b, sb, cast) = harbor();
        let h = s.sheets[0].clone();
        let one = s.element_named(cast, "TOVE").unwrap();
        let all = strip_cell(&s, &h, StripField::ElementCount, Some(cast));
        s.element_mut(one).unwrap().exclude_from_board = true;
        assert!(s.excluded(one));
        assert_eq!(strip_cell(&s, &h, StripField::Cast, Some(cast)), "2");
        assert_eq!(strip_cell(&s, &h, StripField::CastCount, Some(cast)), "1");
        let OneLine::Scene { cast: ids, .. } =
            &one_line(&s, b, sb, Some(cast)).unwrap()[0].lines[0]
        else {
            panic!()
        };
        assert_eq!(ids, &["2".to_string()]);
        // It leaves the element count too.
        let fewer = strip_cell(&s, &h, StripField::ElementCount, Some(cast));
        assert_eq!(
            fewer.parse::<usize>().unwrap_or(0) + 1,
            all.parse::<usize>().unwrap()
        );
    }

    /// BRD-18 (behavior note 27, decision 147): an element count of one category shows
    /// its name and how many of its elements the sheet has, `(0)` with none, leaving out
    /// the ones left off the board; deleting the category drops the pick, and an unknown
    /// category is refused.
    #[test]
    fn an_element_count_counts_its_category() {
        use crate::model::{StripColumn, StripLayout};
        let (mut s, b, _, cast) = harbor();
        let h = s.sheets[0].clone();
        let col = |c: Option<CategoryId>| StripColumn {
            category: c,
            ..StripColumn::of(StripField::ElementCount)
        };
        let all = strip_cell(&s, &h, StripField::ElementCount, Some(cast));
        assert_eq!(
            column_cell(&s, &h, &col(None), &[]),
            all,
            "no category: all"
        );
        let n = strip_cell(&s, &h, StripField::CastCount, Some(cast));
        assert_eq!(
            column_cell(&s, &h, &col(Some(cast)), &[]),
            format!("Cast ({n})")
        );
        let props = s.add_category("Props");
        assert_eq!(column_cell(&s, &h, &col(Some(props)), &[]), "Props (0)");
        // Left off the board: not counted, as the cast count leaves it out.
        let tove = s.element_named(cast, "TOVE").unwrap();
        assert!(h.elements.contains(&tove));
        s.element_mut(tove).unwrap().exclude_from_board = true;
        let fewer = strip_cell(&s, &h, StripField::CastCount, Some(cast));
        assert_eq!(
            fewer.parse::<usize>().unwrap() + 1,
            n.parse::<usize>().unwrap()
        );
        assert_eq!(
            column_cell(&s, &h, &col(Some(cast)), &[]),
            format!("Cast ({fewer})")
        );
        s.element_mut(tove).unwrap().exclude_from_board = false;
        // On a board's layout and a named one; a missing category is refused, and so is
        // a category on another field.
        let mut l = StripLayout::default();
        l.columns.push(col(Some(props)));
        s.set_strip_layout(b, l.clone()).unwrap();
        s.check().unwrap();
        assert!(s.strip_layout_of(b).text(&s).contains("counting Props"));
        let mut bad = l.clone();
        bad.columns.last_mut().unwrap().category = Some(CategoryId(999_999));
        assert!(s.set_strip_layout(b, bad.clone()).is_err());
        let mut broken = s.clone();
        broken.boards[0].strip_layout = bad;
        assert!(broken.check().is_err());
        let mut bad = l.clone();
        bad.columns[0].category = Some(cast);
        assert!(bad.problem().is_some());
        let named = s
            .add_layout("Counts", crate::layouts::LayoutBody::Strips(l.clone()))
            .unwrap();
        // Deleting the category (an empty one; a full one can't go) drops the pick: the
        // count counts every element again.
        s.delete_category(props).unwrap();
        s.check().unwrap();
        let last = |x: &StripLayout| x.columns.last().unwrap().category;
        assert_eq!(last(&s.board(b).unwrap().strip_layout), None);
        let crate::layouts::LayoutBody::Strips(x) = &s.layouts.get(named).unwrap().body else {
            panic!()
        };
        assert_eq!(last(x), None);
        // A layout loaded with a category this schedule lacks counts every element.
        let ids = s
            .import_layouts(vec![(
                "Loaded".into(),
                crate::layouts::LayoutBody::Strips(l),
            )])
            .unwrap();
        let crate::layouts::LayoutBody::Strips(y) = &s.layouts.get(ids[0]).unwrap().body else {
            panic!()
        };
        assert!(y.counted().next().is_none());
        s.check().unwrap();
    }

    /// Decision 194: a column of one category's elements lists them by name, or by board
    /// ID with its switch, in board ID order; empty with no category, left off the board
    /// left out; refused on a report's own columns and the switch on another field.
    #[test]
    fn a_column_lists_one_categorys_elements() {
        use crate::model::{StripColumn, StripLayout};
        let (mut s, b, _, _) = harbor();
        let props = s.add_category("Props");
        let rope = s.add_element(props, "ROPE").unwrap();
        let lamp = s.add_element(props, "LAMP").unwrap();
        let net = s.add_element(props, "NET").unwrap();
        let h = s.sheets[0].id;
        for e in [rope, lamp, net] {
            s.tag(h, e).unwrap();
        }
        s.set_board_id(rope, Some("P1")).unwrap();
        s.set_board_id(lamp, Some("P2")).unwrap();
        let col = |c: Option<CategoryId>, ids: bool| StripColumn {
            category: c,
            board_ids: ids,
            ..StripColumn::of(StripField::CategoryElements)
        };
        let sheet = s.sheet(h).unwrap().clone();
        assert_eq!(column_cell(&s, &sheet, &col(None, false), &[]), "");
        assert_eq!(
            column_cell(&s, &sheet, &col(Some(props), false), &[]),
            "ROPE, LAMP, NET"
        );
        assert_eq!(
            column_cell(&s, &sheet, &col(Some(props), true), &[]),
            "P1, P2, NET"
        );
        assert_eq!(
            report_cell(&s, &sheet, &col(Some(props), true), s.cast()),
            "P1, P2, NET"
        );
        assert_eq!(
            strip_cell(&s, &sheet, StripField::CategoryElements, s.cast()),
            ""
        );
        s.element_mut(lamp).unwrap().exclude_from_board = true;
        assert_eq!(
            column_cell(&s, &sheet, &col(Some(props), false), &[]),
            "ROPE, NET"
        );
        // On the board's layout, said in words; the switch on another field is refused.
        let mut l = StripLayout::default();
        l.columns.push(col(Some(props), true));
        s.set_strip_layout(b, l.clone()).unwrap();
        s.check().unwrap();
        assert!(s
            .strip_layout_of(b)
            .text(&s)
            .contains("Category elements (listing Props, by board ID)"));
        let mut bad = l.clone();
        bad.columns[0].board_ids = true;
        assert!(bad.problem().is_some());
        assert!(!StripField::CategoryElements.of_sheet());
        assert!(StripField::CategoryElements.picks_category());
    }

    #[test]
    fn header_fields_work_as_categories() {
        let (mut s, b, sb, _) = harbor();
        let before = s.clone();
        let hc = header_category(&s, HeaderField::Set);
        assert_eq!(s, before, "the schedule itself is untouched");
        let list: Vec<ElementLine> = element_list(&hc.schedule, b, sb)
            .unwrap()
            .into_iter()
            .filter(|l| l.category == hc.category)
            .collect();
        assert!(!list.is_empty());
        let loft = list.iter().find(|l| l.name == "NET LOFT").unwrap();
        assert!(loft.sheets >= 1 && loft.days >= 1);
        assert_eq!(loft.start, "2027-03-01".parse().ok());
        // Values differing in case are one; an empty value is no row.
        s.sheets[1].set = "net loft".into();
        s.sheets[2].set = "  ".into();
        let hc = header_category(&s, HeaderField::Set);
        let rows: Vec<_> = crate::dood::dood(&hc.schedule, b, sb)
            .unwrap()
            .into_iter()
            .filter(|r| hc.schedule.element(r.element).unwrap().category == hc.category)
            .collect();
        let names: Vec<&str> = rows
            .iter()
            .map(|r| hc.schedule.element(r.element).unwrap().name.as_str())
            .collect();
        assert_eq!(
            names
                .iter()
                .filter(|n| n.eq_ignore_ascii_case("net loft"))
                .count(),
            1
        );
        assert_eq!(HeaderField::from_key("episode"), Some(HeaderField::Episode));
        assert_eq!(HeaderField::from_key("cast"), None);
    }

    #[test]
    fn a_typed_header_value_takes_the_existing_spelling() {
        // Behavior note 25 (E-L12): `room 2` typed in a sheet's set used `ROOM 2`.
        let (mut s, _, _, _) = harbor();
        let (a, b, c) = (s.sheets[0].id, s.sheets[1].id, s.sheets[2].id);
        s.sheets[0].set = "ROOM 2".into();
        s.sheets[1].set = "Room 2".into();
        assert!(set_header_value(&mut s, c, HeaderField::Set, "  room 2 "));
        assert_eq!(s.sheet(c).unwrap().set, "ROOM 2", "the first met");
        // A value no other sheet has is kept as typed, trimmed; the sheet's own does not count.
        assert!(set_header_value(
            &mut s,
            c,
            HeaderField::Location,
            " Pier 9 "
        ));
        assert_eq!(s.sheet(c).unwrap().location, "Pier 9");
        assert!(set_header_value(&mut s, c, HeaderField::Location, "PIER 9"));
        assert_eq!(s.sheet(c).unwrap().location, "PIER 9");
        // Every one-value field, I/E and time of day included.
        s.sheets[0].int_ext = "INT".into();
        assert!(set_header_value(&mut s, b, HeaderField::IntExt, "int"));
        assert_eq!(s.sheet(b).unwrap().int_ext, "INT");
        assert!(set_header_value(&mut s, a, HeaderField::Set, ""));
        assert_eq!(s.sheet(a).unwrap().set, "");
        assert!(!set_header_value(
            &mut s,
            SheetId(9999),
            HeaderField::Set,
            "x"
        ));
        s.check().unwrap();
    }

    #[test]
    fn every_header_field_works_as_a_category() {
        // Behavior note 24: I/E, time of day, script day and sequence too.
        let (mut s, b, sb, _) = harbor();
        s.sheets[0].script_day = "D1".into();
        s.sheets[0].sequence = "Opening".into();
        for f in HeaderField::ALL {
            assert_eq!(HeaderField::from_key(f.key()), Some(f));
            let hc = header_category(&s, f);
            let names: Vec<String> = element_list(&hc.schedule, b, sb)
                .unwrap()
                .into_iter()
                .filter(|l| l.category == hc.category)
                .map(|l| l.name)
                .collect();
            let want = match f {
                HeaderField::IntExt => Some("INT"),
                HeaderField::TimeOfDay => Some("NIGHT"),
                HeaderField::ScriptDay => Some("D1"),
                HeaderField::Sequence => Some("Opening"),
                _ => None,
            };
            if let Some(w) = want {
                assert!(names.iter().any(|n| n == w), "{f:?}: {names:?}");
            }
        }
    }

    #[test]
    fn daily_totals_count_each_element_once_a_day() {
        let (s, b, sb, cast) = harbor();
        let totals = daily_category_totals(&s, b, sb).unwrap();
        // Day 1: TOVE and KIT (KIT is on both sheets). Day 2: LUND and KIT. Day 3: KIT.
        let cast_counts: Vec<u32> = totals
            .iter()
            .map(|d| d.counts.iter().find(|(c, _)| *c == cast).unwrap().1)
            .collect();
        assert_eq!(cast_counts, [2, 2, 1]);
        assert_eq!(totals[2].date, "2027-03-03".parse().ok());
    }

    #[test]
    fn board_ids_sort_naturally_then_none() {
        // Behavior note 17, A2: the real app showed 1, 1a, 2, 7, 07, 9.
        let mut v = vec![
            Some("X"),
            Some("10"),
            None,
            Some("07"),
            Some("9"),
            Some("2"),
            Some("1a"),
            Some("7"),
            Some("1"),
        ];
        v.sort_by(|a, b| id_order(*a, *b));
        assert_eq!(
            v,
            [
                Some("1"),
                Some("1a"),
                Some("2"),
                Some("7"),
                Some("07"),
                Some("9"),
                Some("10"),
                Some("X"),
                None
            ]
        );
        // Text after the number, then text with no number; long numbers by value.
        let mut w = vec![
            Some("B"),
            Some("1B"),
            Some("A"),
            Some("1A"),
            Some("99999999999999999999999"),
            Some("100"),
        ];
        w.sort_by(|a, b| id_order(*a, *b));
        assert_eq!(
            w,
            [
                Some("1A"),
                Some("1B"),
                Some("100"),
                Some("99999999999999999999999"),
                Some("A"),
                Some("B")
            ]
        );
        assert_eq!(id_order(Some("7"), Some("7")), std::cmp::Ordering::Equal);
    }
}
