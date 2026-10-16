//! Conflicts: one element scheduled on the same date in two or more sub-boards of the
//! same board (inventory section 8, behavior note 03, engine check E-E1). Other boards
//! are other arrangements of the same sheets, so they are never compared.
//!
//! **An event on a work day is a conflict too** (behavior note 17, A4 and A5; E-D6): a
//! travel day, fitting or rehearsal for an element on a date it works on the same
//! sub-board. The real app raises its conflict count for it and flags the DOOD cell
//! ([`crate::dood::Cell::EventOnWork`]). Events that count nowhere (unavailable, camera
//! test, other kinds) never do (A8).
//!
//! Dates come from each sub-board's own start date, calendar and day breaks, as the day
//! out of days has them: strips below the last day break aren't scheduled (E-D15), and a
//! sub-board with no start date has no dates, so it can't conflict (it is listed as
//! undated). Only work days count, not holds. Elements of a category excluded from the
//! board are not "active" (E-E3's proposed rule) and are left out.
//!
//! **Event clashes across sub-boards are a separate, informational list.** The current
//! manual says overlapping events don't cause conflicts (E-E2). We report, apart and
//! clearly labeled, the dates where an element has two or more events on the board, or an
//! event in one sub-board and strip work in another. They are not conflicts.
//!
//! Warnings only: nothing here changes or blocks a schedule.

use std::collections::{BTreeMap, HashMap};

use crate::days::days_with;
use crate::model::{BoardId, ElementId, Error, Schedule, SheetId, StripKind, SubBoardId};
use crate::Date;

/// An element's strip work on one date in one sub-board.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Booking {
    pub sub_board: SubBoardId,
    /// The shoot day, from 1, counted in the sub-board.
    pub day: u32,
    /// The sheets on that day that list the element, in board order.
    pub sheets: Vec<SheetId>,
}

/// One element on one date in two or more sub-boards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conflict {
    pub element: ElementId,
    pub date: Date,
    /// One per sub-board, in the board's sub-board order. At least two.
    pub bookings: Vec<Booking>,
}

/// An event on a date.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventRef {
    pub sub_board: SubBoardId,
    /// The event's index in the sub-board's calendar.
    pub index: usize,
    pub name: String,
}

/// Informational, not a conflict: an element with two or more events on a date, or an
/// event in one sub-board and strip work in another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventClash {
    pub element: ElementId,
    pub date: Date,
    /// The element's events covering the date, in sub-board then calendar order.
    pub events: Vec<EventRef>,
    /// The element's strip work on the date, any sub-board.
    pub bookings: Vec<Booking>,
}

/// A counted event (travel, fitting, rehearsal) on a date the element works on the same
/// sub-board: a conflict.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventOnWork {
    pub element: ElementId,
    pub date: Date,
    pub sub_board: SubBoardId,
    /// The event's index in the sub-board's calendar.
    pub event: usize,
    pub name: String,
    /// The shoot day, from 1, counted in the sub-board.
    pub day: u32,
}

/// What [`conflicts`] found on one board.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Conflicts {
    /// Sorted by date, then by the element's place in the schedule.
    pub conflicts: Vec<Conflict>,
    /// Events on work days, conflicts too. Sorted like `conflicts`, then by sub-board and
    /// event.
    pub event_on_work: Vec<EventOnWork>,
    /// Informational only (see the module notes). Sorted like `conflicts`.
    pub event_clashes: Vec<EventClash>,
    /// Sub-boards whose strips have no dates (no start date, or a week with no working
    /// day), so they took no part.
    pub undated: Vec<SubBoardId>,
}

impl Conflicts {
    /// How many conflicts: elements booked twice on a date, and events on work days.
    /// Event clashes are not counted.
    pub fn count(&self) -> usize {
        self.conflicts.len() + self.event_on_work.len()
    }
}

/// The dated strip work of one sub-board: element and date to (day, sheets).
pub(crate) type Work = BTreeMap<(ElementId, Date), (u32, Vec<SheetId>)>;

/// The dated strip work of one sub-board, for elements `active` keeps. `None` when its
/// strips have no dates (no start date, or a week with no working day).
pub(crate) fn sub_board_work(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
    active: &dyn Fn(ElementId) -> bool,
) -> Result<Option<Work>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let Some(start) = sb.start else {
        return Ok(None);
    };
    let cal = &sb.calendar;
    let index = s.sheet_index();
    let list = days_with(s, &index, board, sub)?.days;
    if !list.is_empty() && cal.date_of_day(start, 1).is_none() {
        return Ok(None);
    }
    let mut out = Work::new();
    // Shoot day n is the nth shoot date on or after the start (Calendar::date_of_day),
    // walked once rather than counted again for every day.
    let mut date = start;
    for day in &list {
        while !cal.is_shoot_day(date) {
            date = date.add_days(1);
        }
        // An omitted scene is no work (decision 91).
        for sheet in day.counted_sheets() {
            let Some(h) = index.get(&sheet) else { continue };
            for el in h.elements.iter().copied().filter(|e| active(*e)) {
                let slot = out.entry((el, date)).or_insert((day.number, Vec::new()));
                slot.1.push(sheet);
            }
        }
        date = date.add_days(1);
    }
    Ok(Some(out))
}

/// The conflicts on `board`, and the event clashes beside them.
pub fn conflicts(s: &Schedule, board: BoardId) -> Result<Conflicts, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    // Nothing can clash with fewer than two dated sub-boards and no events: a booking
    // needs a second sub-board, an event on work or an event clash needs an event. The
    // board view asks on every redraw, so this skips working out every day's work.
    if b.sub_boards.iter().filter(|sb| sb.start.is_some()).count() < 2
        && b.sub_boards.iter().all(|sb| sb.calendar.events.is_empty())
    {
        let undated = b
            .sub_boards
            .iter()
            .filter(|sb| match sb.start {
                None => true,
                // As sub_board_work has it: days but no date for day 1.
                Some(start) => {
                    sb.strips.iter().any(|st| st.kind == StripKind::DayBreak)
                        && sb.calendar.date_of_day(start, 1).is_none()
                }
            })
            .map(|sb| sb.id)
            .collect();
        return Ok(Conflicts {
            undated,
            ..Conflicts::default()
        });
    }
    all_conflicts(s, board)
}

/// [`conflicts`] worked out in full, without the early answer.
fn all_conflicts(s: &Schedule, board: BoardId) -> Result<Conflicts, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    // An element's own setting decides (decision 120).
    let active = |e: ElementId| s.element(e).is_some_and(|el| !s.element_excluded(el));
    let order: HashMap<ElementId, usize> = s
        .elements
        .iter()
        .enumerate()
        .map(|(i, e)| (e.id, i))
        .collect();

    let mut out = Conflicts::default();
    // (element, date) -> bookings, in sub-board order.
    let mut booked: BTreeMap<(ElementId, Date), Vec<Booking>> = BTreeMap::new();
    for sb in &b.sub_boards {
        match sub_board_work(s, board, sb.id, &active)? {
            None => out.undated.push(sb.id),
            Some(work) => {
                for ((el, date), (day, sheets)) in work {
                    booked.entry((el, date)).or_default().push(Booking {
                        sub_board: sb.id,
                        day,
                        sheets,
                    });
                }
            }
        }
    }
    let mut events: BTreeMap<(ElementId, Date), Vec<EventRef>> = BTreeMap::new();
    for sb in &b.sub_boards {
        for (index, ev) in sb.calendar.events.iter().enumerate() {
            let counted = ev.kind.code().is_some();
            let mut d = ev.from;
            while d <= ev.to {
                for el in ev.elements.iter().copied().filter(|e| active(*e)) {
                    let here = booked
                        .get(&(el, d))
                        .and_then(|bks| bks.iter().find(|bk| bk.sub_board == sb.id));
                    if let (true, Some(bk)) = (counted, here) {
                        out.event_on_work.push(EventOnWork {
                            element: el,
                            date: d,
                            sub_board: sb.id,
                            event: index,
                            name: ev.name.clone(),
                            day: bk.day,
                        });
                    }
                    events.entry((el, d)).or_default().push(EventRef {
                        sub_board: sb.id,
                        index,
                        name: ev.name.clone(),
                    });
                }
                d = d.add_days(1);
            }
        }
    }

    for ((element, date), bookings) in &booked {
        if bookings.len() >= 2 {
            out.conflicts.push(Conflict {
                element: *element,
                date: *date,
                bookings: bookings.clone(),
            });
        }
    }
    for ((element, date), evs) in events {
        let bookings = booked.get(&(element, date)).cloned().unwrap_or_default();
        let elsewhere = bookings
            .iter()
            .any(|bk| evs.iter().any(|e| e.sub_board != bk.sub_board));
        if evs.len() >= 2 || elsewhere {
            out.event_clashes.push(EventClash {
                element,
                date,
                events: evs,
                bookings,
            });
        }
    }
    let rank = |e: &ElementId| order.get(e).copied().unwrap_or(usize::MAX);
    out.conflicts.sort_by_key(|a| (a.date, rank(&a.element)));
    let sub_rank = |id: SubBoardId| b.sub_boards.iter().position(|x| x.id == id);
    out.event_on_work
        .sort_by_key(|a| (a.date, rank(&a.element), sub_rank(a.sub_board), a.event));
    out.event_clashes
        .sort_by_key(|a| (a.date, rank(&a.element)));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Event, EventKind, Place, Sheet, StripId, StripKind};

    fn d(s: &str) -> Date {
        s.parse().unwrap()
    }

    /// Main from Mon 2027-03-01 with ADA on day 1 (sheet 1) and day 2 (sheet 2, with BEN);
    /// Unit B from Tue 2027-03-02 with ADA and BEN on its day 1 (sheet 3) and ADA on its
    /// day 2 (sheet 4). One day break after each sheet.
    fn two_units() -> (
        Schedule,
        BoardId,
        SubBoardId,
        SubBoardId,
        ElementId,
        ElementId,
    ) {
        let mut s = Schedule::new("ZTEST Conflicts");
        let cast = s.category_named("Cast").unwrap();
        let ada = s.add_element(cast, "ADA").unwrap();
        let ben = s.add_element(cast, "BEN").unwrap();
        let sheets: Vec<SheetId> = [vec![ada], vec![ada, ben], vec![ada, ben], vec![ada]]
            .into_iter()
            .enumerate()
            .map(|(i, elements)| {
                s.add_sheet(Sheet {
                    scenes: (i + 1).to_string(),
                    elements,
                    ..Sheet::default()
                })
            })
            .collect();
        let b = s.boards[0].id;
        let main = s.boards[0].sub_boards[0].id;
        let unit = s.add_sub_board(b, "Unit B", Some(d("2027-03-02"))).unwrap();
        s.boards[0].sub_boards[0].start = Some(d("2027-03-01"));
        for (i, sheet) in sheets.iter().enumerate() {
            let sub = if i < 2 { main } else { unit };
            let strip: StripId = s.boards[0]
                .boneyard
                .iter()
                .find(|x| x.kind == StripKind::Sheet(*sheet))
                .unwrap()
                .id;
            let end = s.boards[0]
                .sub_boards
                .iter()
                .find(|x| x.id == sub)
                .unwrap()
                .strips
                .len();
            let at = Place {
                sub_board: Some(sub),
                index: end,
            };
            s.move_strips(b, &[strip], at).unwrap();
            s.insert_strip(
                b,
                StripKind::DayBreak,
                Place {
                    sub_board: Some(sub),
                    index: end + 1,
                },
            )
            .unwrap();
        }
        s.check().unwrap();
        (s, b, main, unit, ada, ben)
    }

    #[test]
    fn finds_an_element_on_one_date_in_two_sub_boards() {
        let (s, b, main, unit, ada, ben) = two_units();
        let c = conflicts(&s, b).unwrap();
        // Tue Mar 2: Main day 2 (ADA, BEN) and Unit B day 1 (ADA, BEN). Wed Mar 3: only
        // Unit B day 2.
        assert_eq!(
            c.conflicts,
            [
                Conflict {
                    element: ada,
                    date: d("2027-03-02"),
                    bookings: vec![
                        Booking {
                            sub_board: main,
                            day: 2,
                            sheets: vec![s.sheets[1].id]
                        },
                        Booking {
                            sub_board: unit,
                            day: 1,
                            sheets: vec![s.sheets[2].id]
                        },
                    ],
                },
                Conflict {
                    element: ben,
                    date: d("2027-03-02"),
                    bookings: vec![
                        Booking {
                            sub_board: main,
                            day: 2,
                            sheets: vec![s.sheets[1].id]
                        },
                        Booking {
                            sub_board: unit,
                            day: 1,
                            sheets: vec![s.sheets[2].id]
                        },
                    ],
                },
            ]
        );
        assert!(c.event_clashes.is_empty());
        assert!(c.event_on_work.is_empty());
        assert_eq!(c.count(), 2);
        assert!(c.undated.is_empty());
    }

    #[test]
    fn other_boards_dates_and_exclusions() {
        let (mut s, b, _, unit, ada, _) = two_units();
        // A second board starts with every sheet in its Boneyard: nothing to compare, and
        // the first board's conflicts don't show there.
        let other = s.add_board("Board 2", None);
        assert!(conflicts(&s, other).unwrap().conflicts.is_empty());
        // Unit B a day later: Mar 3 and 4, against Main's Mar 1 and 2. No conflicts.
        s.boards[0].sub_boards[1].start = Some(d("2027-03-03"));
        assert!(conflicts(&s, b).unwrap().conflicts.is_empty());
        // Unit B with no start date: undated, no conflicts.
        s.boards[0].sub_boards[1].start = None;
        let c = conflicts(&s, b).unwrap();
        assert!(c.conflicts.is_empty());
        assert_eq!(c.undated, [unit]);
        // Unit B back on Mar 2, but Mar 2 a holiday there: its day 1 moves to Mar 3.
        s.boards[0].sub_boards[1].start = Some(d("2027-03-02"));
        s.boards[0].sub_boards[1]
            .calendar
            .set_day_kind(d("2027-03-02"), crate::model::DayKind::Holiday);
        assert!(conflicts(&s, b).unwrap().conflicts.is_empty());
        s.boards[0].sub_boards[1]
            .calendar
            .set_day_kind(d("2027-03-02"), crate::model::DayKind::Work);
        assert_eq!(conflicts(&s, b).unwrap().conflicts.len(), 2);
        // Nor is one element left off the board on its own (decision 105).
        s.element_mut(ada).unwrap().exclude_from_board = true;
        assert!(s.excluded(ada));
        let c = conflicts(&s, b).unwrap();
        assert!(!c.conflicts.is_empty() && c.conflicts.iter().all(|x| x.element != ada));
        s.element_mut(ada).unwrap().exclude_from_board = false;
        assert_eq!(conflicts(&s, b).unwrap().conflicts.len(), 2);
        // A category excluded from the board: a default for new elements only, until
        // applied to the existing ones (decision 120).
        let cast = s.element(ada).unwrap().category;
        s.category_mut(cast).unwrap().exclude_from_board = true;
        assert_eq!(conflicts(&s, b).unwrap().conflicts.len(), 2);
        s.apply_defaults(cast).unwrap();
        assert!(conflicts(&s, b).unwrap().conflicts.is_empty());
        assert!(conflicts(&s, BoardId(9999)).is_err());
    }

    #[test]
    fn strips_below_the_last_break_are_not_scheduled() {
        let (mut s, b, _, unit, _, _) = two_units();
        // With Unit B's day breaks gone, both its sheets sit below the last break (there
        // is none), so nothing there is scheduled.
        let breaks: Vec<StripId> = s.boards[0].sub_boards[1]
            .strips
            .iter()
            .filter(|x| x.kind == StripKind::DayBreak)
            .map(|x| x.id)
            .collect();
        s.delete_strip(b, breaks[1]).unwrap();
        s.delete_strip(b, breaks[0]).unwrap();
        let c = conflicts(&s, b).unwrap();
        assert!(c.conflicts.is_empty(), "{c:?}");
        assert!(c.undated.is_empty());
        assert!(s.boards[0].sub_boards.iter().any(|x| x.id == unit));
    }

    #[test]
    fn event_clashes_are_reported_apart() {
        let (mut s, b, main, unit, ada, ben) = two_units();
        let ev = |name: &str, from: &str, to: &str, els: Vec<ElementId>| Event {
            name: name.into(),
            kind: EventKind::Travel,
            from: d(from),
            to: d(to),
            elements: els,
            color: None,
        };
        // BEN travels on Unit B on Mar 1, while Main has... nothing for BEN on Mar 1 (ADA
        // only). Then a fitting for ADA on Main on Mar 3, while Unit B shoots ADA that day.
        s.set_event(
            b,
            unit,
            None,
            ev("Fly in", "2027-03-01", "2027-03-01", vec![ben]),
        )
        .unwrap();
        s.set_event(
            b,
            main,
            None,
            ev("Fitting", "2027-03-03", "2027-03-03", vec![ada]),
        )
        .unwrap();
        // And two overlapping events for BEN on Mar 8 and 9.
        s.set_event(
            b,
            main,
            None,
            ev("Rehearse", "2027-03-08", "2027-03-09", vec![ben]),
        )
        .unwrap();
        s.set_event(
            b,
            unit,
            None,
            ev("Press", "2027-03-09", "2027-03-09", vec![ben]),
        )
        .unwrap();
        let c = conflicts(&s, b).unwrap();
        assert_eq!(c.conflicts.len(), 2, "events never make conflicts");
        let got: Vec<(ElementId, String, Vec<String>)> = c
            .event_clashes
            .iter()
            .map(|x| {
                (
                    x.element,
                    x.date.to_string(),
                    x.events.iter().map(|e| e.name.clone()).collect(),
                )
            })
            .collect();
        assert_eq!(
            got,
            [
                (ada, "2027-03-03".into(), vec!["Fitting".into()]),
                (
                    ben,
                    "2027-03-09".into(),
                    vec!["Rehearse".into(), "Press".into()]
                ),
            ]
        );
        assert_eq!(c.event_clashes[0].bookings[0].sub_board, unit);
        assert!(
            c.event_on_work.is_empty(),
            "none on its own sub-board's work"
        );
        // An event and work in the same sub-board is not a clash but a conflict
        // (behavior note 17, A4 and A5; E-D6).
        s.boards[0].sub_boards[0].calendar.events.clear();
        s.boards[0].sub_boards[1].calendar.events.clear();
        s.set_event(
            b,
            unit,
            None,
            ev("Fit", "2027-03-03", "2027-03-03", vec![ada]),
        )
        .unwrap();
        let c = conflicts(&s, b).unwrap();
        assert!(c.event_clashes.is_empty());
        assert_eq!(
            c.event_on_work,
            [EventOnWork {
                element: ada,
                date: d("2027-03-03"),
                sub_board: unit,
                event: 0,
                name: "Fit".into(),
                day: 2,
            }]
        );
        assert_eq!(c.count(), 3);
        // An event that counts nowhere is no conflict (A8).
        s.boards[0].sub_boards[1].calendar.events[0].kind = EventKind::Other("Camera test".into());
        assert!(conflicts(&s, b).unwrap().event_on_work.is_empty());
        // Nor is post-production work (decision 109).
        s.boards[0].sub_boards[1].calendar.events[0].kind =
            EventKind::Post(crate::model::PostKind::SoundMixing);
        assert!(conflicts(&s, b).unwrap().event_on_work.is_empty());
        // Nor is one for an element left off the board, on its own or by its category.
        s.boards[0].sub_boards[1].calendar.events[0].kind = EventKind::Travel;
        let before = conflicts(&s, b).unwrap().count();
        s.element_mut(ada).unwrap().exclude_from_board = true;
        assert!(conflicts(&s, b).unwrap().count() < before);
        s.element_mut(ada).unwrap().exclude_from_board = false;
        assert_eq!(conflicts(&s, b).unwrap().count(), before);
        let cast = s.element(ada).unwrap().category;
        s.category_mut(cast).unwrap().exclude_from_board = true;
        s.apply_defaults(cast).unwrap();
        assert_eq!(conflicts(&s, b).unwrap().count(), 0);
    }

    /// Conflicts follow the calendar and the plan: a holiday moves a sub-board's days onto
    /// other dates, an omitted scene is no work (decision 91), and only travel, fittings
    /// and rehearsals on a work day are conflicts (EVT-07; unavailable and camera tests
    /// never are, behavior note 17, A8).
    #[test]
    fn holidays_omitted_scenes_and_uncounted_kinds() {
        let (mut s, b, main, unit, ada, _ben) = two_units();
        // A holiday on Tue Mar 2 on Main: its day 2 (ADA, BEN) moves to Wed Mar 3, where
        // Unit B's day 2 has ADA only.
        s.calendar_mut(b, main)
            .unwrap()
            .set_day_kind(d("2027-03-02"), crate::model::DayKind::Holiday);
        let c = conflicts(&s, b).unwrap();
        let got: Vec<(ElementId, Date)> = c.conflicts.iter().map(|x| (x.element, x.date)).collect();
        assert_eq!(got, [(ada, d("2027-03-03"))]);
        // Omitting Main's day 2 scene takes the work away, and the conflict with it.
        let sheet2 = s.sheets[1].id;
        s.plan.omitted.push(sheet2);
        assert_eq!(conflicts(&s, b).unwrap().count(), 0);
        s.plan.omitted.clear();
        // Events for ADA on Unit B's day 2 (Mar 3): only the counted kinds conflict.
        for (kind, counts) in [
            (EventKind::Unavailable, false),
            (EventKind::CameraTest, false),
            (EventKind::Other(String::new()), false),
            (EventKind::Rehearsal, true),
            (EventKind::Fitting, true),
            (EventKind::Travel, true),
        ] {
            let mut x = s.clone();
            x.set_event(
                b,
                unit,
                None,
                Event {
                    name: String::new(),
                    kind: kind.clone(),
                    from: d("2027-03-03"),
                    to: d("2027-03-03"),
                    elements: vec![ada],
                    color: None,
                },
            )
            .unwrap();
            let c = conflicts(&x, b).unwrap();
            assert_eq!(c.event_on_work.len(), usize::from(counts), "{kind:?}");
            assert_eq!(c.conflicts.len(), 1, "{kind:?} never adds a booking");
        }
    }

    /// The early answer for one dated sub-board and no events is the full answer.
    #[test]
    fn the_early_answer_matches_the_full_one() {
        let mut rng = crate::testgen::Rng::new(9090);
        let mut early = 0;
        for _ in 0..200 {
            let s = crate::merge::tests::random_schedule(&mut rng);
            for b in &s.boards {
                let quick = b.sub_boards.iter().filter(|x| x.start.is_some()).count() < 2
                    && b.sub_boards.iter().all(|x| x.calendar.events.is_empty());
                early += usize::from(quick);
                assert_eq!(conflicts(&s, b.id), all_conflicts(&s, b.id));
            }
        }
        assert!(
            early > 0,
            "the generator never made a board the early answer covers"
        );
    }
}
