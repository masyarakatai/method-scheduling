//! The data of the reports beyond the first five (crate::reports): the breakdown report
//! by strip and by shoot day, the element detail, the vertical one-line with its cast ID
//! grid, the multi-unit one-line merged by date, and the calendar's dates. Plain values;
//! layouts are the app's, and our own (decisions.md, entries 23 and 59).
//!
//! What each report carries follows behavior note 05 (in our words): the breakdown by day
//! merges each category's elements over the day's scenes, the basis of a call sheet; the
//! element detail lists every scene an element is in with its day and date, and its work
//! dates; the vertical one-line keeps one row per cast member; the multi-unit one-line
//! puts every sub-board's day on a date together, each under its unit's name.

use std::collections::BTreeMap;

use crate::days::days;
use crate::dood::dood;
use crate::model::{
    BoardId, CategoryId, ElementId, Error, Minutes, Schedule, SheetId, StripKind, SubBoard,
    SubBoardId,
};
use crate::reports::{id_order, one_line, OneLine};
use crate::{Date, Eighths};

fn sub(s: &Schedule, board: BoardId, id: SubBoardId) -> Result<&SubBoard, Error> {
    s.board(board)
        .ok_or(Error::NoSuchBoard(board))?
        .sub_boards
        .iter()
        .find(|x| x.id == id)
        .ok_or(Error::NoSuchSubBoard(id))
}

/// An element as reports list it: `ID NAME`, or its name alone.
pub fn element_text(s: &Schedule, id: ElementId) -> String {
    s.element(id)
        .map_or_else(String::new, |e| match &e.board_id {
            Some(b) => format!("{b} {}", e.name),
            None => e.name.clone(),
        })
}

/// A sheet's elements by category, in category order, each category's in board ID then
/// name order. Empty categories are left out.
pub fn elements_by_category(
    s: &Schedule,
    elements: &[ElementId],
) -> Vec<(CategoryId, Vec<ElementId>)> {
    let mut out = Vec::new();
    for cat in &s.categories {
        let mut els: Vec<_> = elements
            .iter()
            .filter_map(|e| s.element(*e))
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
        out.push((cat.id, els.into_iter().map(|e| e.id).collect()));
    }
    out
}

/// Where a sheet's strip is on a sub-board: its shoot day and date, or none when it is
/// after the last day break.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub day: Option<u32>,
    pub date: Option<Date>,
}

/// Every sheet strip of a sub-board in board order, with its day.
fn sheet_slots(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
) -> Result<Vec<(SheetId, Slot)>, Error> {
    let sb = sub(s, board, id)?;
    let ds = days(s, board, id)?;
    let mut day_of = vec![None; sb.strips.len()];
    for d in &ds.days {
        for e in &d.entries {
            day_of[e.at] = Some(d.number);
        }
    }
    Ok(sb
        .strips
        .iter()
        .enumerate()
        .filter_map(|(at, st)| match st.kind {
            StripKind::Sheet(h) => {
                let day = day_of[at];
                let date = day.and_then(|n| sb.start.and_then(|x| sb.calendar.date_of_day(x, n)));
                Some((h, Slot { day, date }))
            }
            _ => None,
        })
        .collect())
}

/// One block of the breakdown report by strip: a sheet with its elements by category.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakdownSheet {
    pub sheet: SheetId,
    pub slot: Slot,
    pub elements: Vec<(CategoryId, Vec<ElementId>)>,
}

/// The breakdown report by strip: every sheet on the sub-board in board order, the
/// scheduled ones with their day, then those after the last day break.
pub fn breakdown_by_strip(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
) -> Result<Vec<BreakdownSheet>, Error> {
    sheet_slots(s, board, id)?
        .into_iter()
        .map(|(h, slot)| {
            let sheet = s.sheet(h).ok_or(Error::NoSuchSheet(h))?;
            Ok(BreakdownSheet {
                sheet: h,
                slot,
                elements: elements_by_category(s, &sheet.elements),
            })
        })
        .collect()
}

/// One shoot day of the breakdown report by day.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreakdownDay {
    pub number: u32,
    pub date: Option<Date>,
    /// The day's sheets in board order.
    pub sheets: Vec<SheetId>,
    pub pages: Eighths,
    pub time: Minutes,
    /// Each category's elements over the whole day, each once, with how many of the
    /// day's scenes list it.
    pub elements: Vec<(CategoryId, Vec<(ElementId, u32)>)>,
    /// The day's quantity of each element of a category that uses daily quantities, by
    /// decision 63's one rule ([`crate::reports::day_quantities`]): the sum over the
    /// day's sheets, one for a sheet that gives none. Elements of other categories count
    /// once a day and are not listed.
    pub needed: Vec<(ElementId, u32)>,
    /// Those of `needed` whose total is partial: a sheet marked its quantity not to count
    /// daily (shown with an asterisk, as the daily totals are).
    pub partial: Vec<ElementId>,
}

/// The breakdown report by shoot day: one merged element block per category per day.
pub fn breakdown_by_day(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
) -> Result<Vec<BreakdownDay>, Error> {
    let sb = sub(s, board, id)?;
    let mut out = Vec::new();
    let (index, element_index) = (s.sheet_index(), s.element_index());
    for d in crate::days::days_with(s, &index, board, id)?.days {
        let sheets: Vec<SheetId> = d.entries.iter().filter_map(|e| e.sheet).collect();
        let mut count: BTreeMap<ElementId, u32> = BTreeMap::new();
        let mut order: Vec<ElementId> = Vec::new();
        let mut on_day: Vec<&crate::model::Sheet> = Vec::new();
        // An omitted scene is listed but counts nowhere (decision 91).
        for h in d.counted_sheets() {
            let sheet = *index.get(&h).ok_or(Error::NoSuchSheet(h))?;
            on_day.push(sheet);
            for e in &sheet.elements {
                let n = count.entry(*e).or_insert(0);
                if *n == 0 {
                    order.push(*e);
                }
                *n += 1;
            }
        }
        let quantities = crate::reports::day_quantities(s, &element_index, &on_day);
        let elements = elements_by_category(s, &order)
            .into_iter()
            .map(|(c, els)| (c, els.into_iter().map(|e| (e, count[&e])).collect()))
            .collect();
        out.push(BreakdownDay {
            number: d.number,
            date: sb.start.and_then(|x| sb.calendar.date_of_day(x, d.number)),
            sheets,
            pages: d.pages,
            time: d.time,
            elements,
            needed: quantities.iter().map(|q| (q.element, q.count)).collect(),
            partial: quantities
                .iter()
                .filter(|q| q.partial)
                .map(|q| q.element)
                .collect(),
        });
    }
    Ok(out)
}

/// One element of the element detail report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElementDetail {
    pub element: ElementId,
    pub category: CategoryId,
    /// Every sheet listing it on the sub-board, in board order, with its day.
    pub scenes: Vec<(SheetId, Slot)>,
    /// Its work dates on the sub-board (the day out of days' work days), when the
    /// sub-board has a start date.
    pub work_dates: Vec<Date>,
}

/// The element detail of every element on a sub-board's sheets, by category in category
/// order, then board ID and name. `keep` picks which elements (all when it lets every
/// one through).
pub fn element_detail(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
    keep: impl Fn(ElementId, CategoryId) -> bool,
) -> Result<Vec<ElementDetail>, Error> {
    let slots = sheet_slots(s, board, id)?;
    let rows = dood(s, board, id).ok();
    let index = s.sheet_index();
    let mut all: Vec<ElementId> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (h, _) in &slots {
        for e in &index.get(h).ok_or(Error::NoSuchSheet(*h))?.elements {
            if seen.insert(*e) {
                all.push(*e);
            }
        }
    }
    let mut out = Vec::new();
    for (cat, els) in elements_by_category(s, &all) {
        for e in els {
            if !keep(e, cat) {
                continue;
            }
            let scenes = slots
                .iter()
                .filter(|(h, _)| index.get(h).is_some_and(|x| x.elements.contains(&e)))
                .copied()
                .collect();
            let work_dates = rows
                .as_ref()
                .and_then(|r| r.iter().find(|x| x.element == e))
                .map(|r| {
                    r.cells
                        .iter()
                        .filter(|(_, c)| c.is_work())
                        .map(|(d, _)| *d)
                        .collect()
                })
                .unwrap_or_default();
            out.push(ElementDetail {
                element: e,
                category: cat,
                scenes,
                work_dates,
            });
        }
    }
    Ok(out)
}

/// The vertical one-line: a column per scene in board order (shoot days in turn), and a
/// fixed row per cast member, so each cast member keeps one line down the whole schedule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vertical {
    /// The cast, in board ID then name order: the rows, and the cover list.
    pub cast: Vec<ElementId>,
    pub days: Vec<VerticalDay>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerticalDay {
    pub number: u32,
    pub date: Option<Date>,
    pub pages: Eighths,
    /// The day's shoot time with banners, for its day break column (behavior note 23).
    pub time: Minutes,
    /// Each scene with the rows (indexes into `cast`) it lists.
    pub scenes: Vec<(SheetId, Vec<usize>)>,
}

pub fn vertical(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
    cast: Option<CategoryId>,
) -> Result<Vertical, Error> {
    let sb = sub(s, board, id)?;
    let (index, elements) = (s.sheet_index(), s.element_index());
    let ds = crate::days::days_with(s, &index, board, id)?;
    let mut rows: Vec<ElementId> = Vec::new();
    let mut out_days = Vec::new();
    for d in &ds.days {
        let mut scenes = Vec::new();
        // The cast's work: an omitted scene is none (decision 91).
        for h in d.counted_sheets() {
            let sheet = *index.get(&h).ok_or(Error::NoSuchSheet(h))?;
            let on: Vec<ElementId> = sheet
                .elements
                .iter()
                .copied()
                // A cast member left off the board keeps its row, in board ID order with
                // the others (RPT-20, behavior note 27, decision 142).
                .filter(|e| elements.get(e).is_some_and(|x| Some(x.category) == cast))
                .collect();
            for e in &on {
                if !rows.contains(e) {
                    rows.push(*e);
                }
            }
            scenes.push((h, on));
        }
        out_days.push((d, scenes));
    }
    rows.sort_by(|a, b| {
        let (x, y) = (s.element(*a).unwrap(), s.element(*b).unwrap());
        id_order(x.board_id.as_deref(), y.board_id.as_deref()).then_with(|| {
            x.name
                .to_ascii_uppercase()
                .cmp(&y.name.to_ascii_uppercase())
        })
    });
    let days = out_days
        .into_iter()
        .map(|(d, scenes)| VerticalDay {
            number: d.number,
            date: sb.start.and_then(|x| sb.calendar.date_of_day(x, d.number)),
            pages: d.pages,
            time: d.time,
            scenes: scenes
                .into_iter()
                .map(|(h, on)| {
                    let mut at: Vec<usize> = on
                        .iter()
                        .filter_map(|e| rows.iter().position(|r| r == e))
                        .collect();
                    at.sort_unstable();
                    (h, at)
                })
                .collect(),
        })
        .collect();
    Ok(Vertical { cast: rows, days })
}

/// The multi-unit one-line: each date with its units' days, and the sub-boards left out
/// for having no start date.
pub type MultiUnit = (Vec<(Date, Vec<UnitDay>)>, Vec<SubBoardId>);

/// One unit's shoot day on a date of the multi-unit one-line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitDay {
    pub sub_board: SubBoardId,
    pub number: u32,
    pub lines: Vec<OneLine>,
    pub pages: Eighths,
    pub time: Minutes,
}

/// The multi-unit one-line: every dated shoot day of the board's sub-boards, by date,
/// each date's units in sub-board order. With `only` not empty, a unit's day is left out
/// unless one of its scenes lists one of those elements (and a date with none left goes).
/// Sub-boards without a start date are left out (their days have no dates); they are
/// returned apart.
pub fn multi_unit(
    s: &Schedule,
    board: BoardId,
    cast: Option<CategoryId>,
    only: &[ElementId],
) -> Result<MultiUnit, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let mut by_date: BTreeMap<Date, Vec<UnitDay>> = BTreeMap::new();
    let mut undated = Vec::new();
    for sb in &b.sub_boards {
        if sb.start.is_none() {
            undated.push(sb.id);
            continue;
        }
        for day in one_line(s, board, sb.id, cast)? {
            let Some(date) = day.date else { continue };
            if !only.is_empty() {
                let hit = day.lines.iter().any(|l| match l {
                    OneLine::Scene { sheet, .. } => s
                        .sheet(*sheet)
                        .is_some_and(|h| h.elements.iter().any(|e| only.contains(e))),
                    OneLine::Banner { .. } => false,
                });
                if !hit {
                    continue;
                }
            }
            by_date.entry(date).or_default().push(UnitDay {
                sub_board: sb.id,
                number: day.number,
                lines: day.lines,
                pages: day.pages,
                time: day.time,
            });
        }
    }
    Ok((by_date.into_iter().collect(), undated))
}

/// One date of a sub-board's calendar, as the calendar report and the Calendar tab's
/// month show it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarDate {
    pub date: Date,
    pub kind: crate::model::DayKind,
    pub shoot_day: Option<u32>,
    /// The day's sheets in board order (a shoot day's).
    pub sheets: Vec<SheetId>,
    pub pages: Eighths,
    /// The estimated length of the day: its sheets' shoot times plus banners.
    pub time: Minutes,
    /// Indexes into the calendar's events of those covering the date.
    pub events: Vec<usize>,
}

/// Every date from `from` to `to` (inclusive) of a sub-board's calendar.
pub fn calendar_dates(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
    from: Date,
    to: Date,
) -> Result<Vec<CalendarDate>, Error> {
    let sb = sub(s, board, id)?;
    let ds = days(s, board, id)?;
    let mut shoot: BTreeMap<Date, &crate::days::Day> = BTreeMap::new();
    if let Some(start) = sb.start {
        for d in &ds.days {
            if let Some(date) = sb.calendar.date_of_day(start, d.number) {
                shoot.insert(date, d);
            }
        }
    }
    let mut out = Vec::new();
    let mut d = from;
    while d <= to {
        let day = shoot.get(&d);
        out.push(CalendarDate {
            date: d,
            kind: sb.calendar.day_kind(d),
            shoot_day: day.map(|x| x.number),
            sheets: day.map_or_else(Vec::new, |x| {
                x.entries.iter().filter_map(|e| e.sheet).collect()
            }),
            pages: day.map_or(Eighths::ZERO, |x| x.pages),
            time: day.map_or(Minutes(0), |x| x.time),
            events: sb
                .calendar
                .events
                .iter()
                .enumerate()
                .filter(|(_, ev)| ev.from <= d && d <= ev.to)
                .map(|(i, _)| i)
                .collect(),
        });
        d = d.add_days(1);
    }
    Ok(out)
}

/// The first and last shoot day's dates of a sub-board, when it has a start date and a
/// day break.
pub fn shoot_span(
    s: &Schedule,
    board: BoardId,
    id: SubBoardId,
) -> Result<Option<(Date, Date)>, Error> {
    let sb = sub(s, board, id)?;
    let n = days(s, board, id)?.days.len() as u32;
    let Some(start) = sb.start else {
        return Ok(None);
    };
    if n == 0 {
        return Ok(None);
    }
    Ok(sb
        .calendar
        .date_of_day(start, 1)
        .zip(sb.calendar.date_of_day(start, n)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::{auto_day_breaks, Limit};
    use crate::fountain;
    use crate::model::{Event, EventKind, Place};

    /// Harbor Light, three pages a day from Mon 2027-03-01 (crate::reports' fixture),
    /// with board IDs for TOVE (1) and KIT (2) and a prop on the first sheet.
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
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(3))).unwrap();
        let tove = s.element_named(cast, "TOVE").unwrap();
        let kit = s.element_named(cast, "KIT").unwrap();
        s.set_board_id(tove, Some("1")).unwrap();
        s.set_board_id(kit, Some("2")).unwrap();
        let props = s.add_category("Props");
        let needle = s.add_element(props, "Netting needle").unwrap();
        let first = s.sheets[0].id;
        s.tag(first, needle).unwrap();
        s.boards[0].sub_boards[0].start = "2027-03-01".parse().ok();
        (s, b, sb, cast)
    }

    #[test]
    fn breakdown_by_strip_lists_every_sheet_with_its_day() {
        let (s, b, sb, cast) = harbor();
        let rows = breakdown_by_strip(&s, b, sb).unwrap();
        assert_eq!(rows.len(), s.sheets.len());
        assert_eq!(rows[0].slot.day, Some(1));
        assert_eq!(rows[0].slot.date, "2027-03-01".parse().ok());
        assert_eq!(rows[0].elements[0].0, cast);
        assert_eq!(rows[0].elements.len(), 2);
        assert!(rows.iter().all(|r| r.slot.day.is_some()));
    }

    #[test]
    fn breakdown_by_day_merges_elements_and_counts_scenes() {
        let (s, b, sb, cast) = harbor();
        let days = breakdown_by_day(&s, b, sb).unwrap();
        assert_eq!(days.len(), 3);
        let kit = s.element_named(cast, "KIT").unwrap();
        // KIT is on both sheets of day 1: listed once, in two scenes.
        let (c, els) = &days[0].elements[0];
        assert_eq!(*c, cast);
        assert!(els.contains(&(kit, 2)));
        assert_eq!(days[0].sheets.len(), 2);
        assert_eq!(days[0].date, "2027-03-01".parse().ok());
        assert!(days[0].needed.is_empty());
        // A sheet that needs three of KIT: by decision 63's sum, the day calls for four
        // (three, and one on the other sheet), once the cast uses daily quantities; the
        // day break's text says the same.
        let mut s = s;
        let first = days[0].sheets[0];
        s.set_quantity(first, kit, 3, true).unwrap();
        let days = breakdown_by_day(&s, b, sb).unwrap();
        assert!(days[0].needed.is_empty(), "the cast counts once a day");
        s.category_mut(cast).unwrap().daily_quantity = true;
        let days = breakdown_by_day(&s, b, sb).unwrap();
        assert!(days[0].needed.contains(&(kit, 4)), "{:?}", days[0].needed);
        let on_day: Vec<&crate::model::Sheet> = days[0]
            .sheets
            .iter()
            .map(|h| s.sheet(*h).unwrap())
            .collect();
        let text = crate::reports::day_quantity_text(&s, &on_day);
        let total: u32 = days[0].needed.iter().map(|(_, n)| n).sum();
        assert_eq!(text, format!("Cast {total}"));
        // A quantity not counted daily: partial, as the daily totals mark it.
        s.set_quantity(first, kit, 3, false).unwrap();
        let days = breakdown_by_day(&s, b, sb).unwrap();
        assert!(days[0].needed.contains(&(kit, 1)));
        assert_eq!(days[0].partial, [kit]);
    }

    #[test]
    fn element_detail_lists_scenes_and_work_dates() {
        let (s, b, sb, cast) = harbor();
        let all = element_detail(&s, b, sb, |_, _| true).unwrap();
        let kit = s.element_named(cast, "KIT").unwrap();
        let d = all.iter().find(|x| x.element == kit).unwrap();
        assert_eq!(d.scenes.len(), 4);
        assert_eq!(d.work_dates.len(), 3);
        assert_eq!(all[0].category, cast);
        // Only the cast.
        let only = element_detail(&s, b, sb, |_, c| c == cast).unwrap();
        assert!(only.iter().all(|x| x.category == cast));
        assert!(only.len() < all.len());
    }

    #[test]
    fn vertical_keeps_a_row_per_cast_member() {
        let (s, b, sb, cast) = harbor();
        let v = vertical(&s, b, sb, Some(cast)).unwrap();
        let names: Vec<_> = v
            .cast
            .iter()
            .map(|e| s.element(*e).unwrap().name.as_str())
            .collect();
        assert_eq!(&names[..2], ["TOVE", "KIT"]);
        assert_eq!(v.days.len(), 3);
        // Scene 1 has TOVE and KIT: rows 0 and 1.
        assert_eq!(v.days[0].scenes[0].1, [0, 1]);
        // A cast member left off the board keeps its row, as in the reference app
        // (behavior note 27, decision 142).
        let mut off = s.clone();
        off.element_mut(v.cast[0]).unwrap().exclude_from_board = true;
        let w = vertical(&off, b, sb, Some(cast)).unwrap();
        assert_eq!(w.cast, v.cast);
        assert_eq!(w.days[0].scenes[0].1, [0, 1]);
        // Each day carries its totals for its day break column (behavior note 23).
        let ds = crate::days::days(&s, b, sb).unwrap();
        for (v, d) in v.days.iter().zip(&ds.days) {
            assert_eq!((v.pages, v.time), (d.pages, d.time));
        }
    }

    #[test]
    fn multi_unit_merges_sub_boards_by_date() {
        let (mut s, b, sb, cast) = harbor();
        let unit = s
            .add_sub_board(b, "Second unit", "2027-03-02".parse().ok())
            .unwrap();
        // Move the last day's scene to the second unit, on its own day, starting Mar 3.
        let last = s.boards[0].sub_boards[0]
            .strips
            .iter()
            .rev()
            .find(|x| matches!(x.kind, StripKind::Sheet(_)))
            .unwrap()
            .id;
        s.move_strips(
            b,
            &[last],
            Place {
                sub_board: Some(unit),
                index: 0,
            },
        )
        .unwrap();
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(unit),
                index: 1,
            },
        )
        .unwrap();
        let (dates, undated) = multi_unit(&s, b, Some(cast), &[]).unwrap();
        assert!(undated.is_empty());
        let mar2 = dates
            .iter()
            .find(|(d, _)| *d == "2027-03-02".parse().unwrap())
            .unwrap();
        assert_eq!(mar2.1.len(), 2);
        assert_eq!(mar2.1[0].sub_board, sb);
        assert_eq!(mar2.1[1].sub_board, unit);
        // Only days with TOVE.
        let tove = s.element_named(cast, "TOVE").unwrap();
        let (only, _) = multi_unit(&s, b, Some(cast), &[tove]).unwrap();
        assert!(only.len() < dates.len());
        assert!(only.iter().all(|(_, units)| !units.is_empty()));
    }

    #[test]
    fn calendar_dates_carry_days_scenes_and_events() {
        let (mut s, b, sb, cast) = harbor();
        let kit = s.element_named(cast, "KIT").unwrap();
        s.boards[0].sub_boards[0].calendar.events.push(Event {
            name: "Fitting".into(),
            kind: EventKind::Fitting,
            from: "2027-02-27".parse().unwrap(),
            to: "2027-03-01".parse().unwrap(),
            elements: vec![kit],
            color: None,
        });
        let (first, last) = shoot_span(&s, b, sb).unwrap().unwrap();
        assert_eq!(first, "2027-03-01".parse().unwrap());
        assert_eq!(last, "2027-03-03".parse().unwrap());
        let all = calendar_dates(&s, b, sb, "2027-02-28".parse().unwrap(), last).unwrap();
        assert_eq!(all.len(), 4);
        assert_eq!(all[0].kind, crate::model::DayKind::Off);
        assert_eq!(all[0].events, [0]);
        assert_eq!(all[1].shoot_day, Some(1));
        assert_eq!(all[1].sheets.len(), 2);
        assert_eq!(all[1].pages, Eighths(3));
    }
}
