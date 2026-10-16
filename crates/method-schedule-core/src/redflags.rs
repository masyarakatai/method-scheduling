//! Red flag clashes: an element working, or at a counted event, on a date one of its red
//! flags rules out (inventory section 9; behavior notes 03 and 09; decision 33; engine
//! check E-E4).
//!
//! One sub-board at a time, as the calendar and the day out of days see it. Work dates
//! come from the sub-board's start date, calendar and day breaks, exactly as for conflicts
//! (`crate::conflicts`): strips below the last day break aren't scheduled, and a sub-board
//! with no start date has no work dates (it is reported as undated). Counted events
//! (travel, fitting, rehearsal: the kinds the day out of days counts) have their own dates,
//! so they are checked with or without a start date. Holds, days off, holidays and other
//! events never clash (E-E8, not yet checked).
//!
//! Red flags belong to elements and are global across boards; the clashes depend on the
//! sub-board. Every element counts, also one in a category excluded from the board: a flag
//! is set on purpose. Hard and soft flags clash alike; each clash says which it is.
//!
//! Warnings only: nothing here changes or blocks a schedule.

use std::collections::{BTreeMap, HashMap};

use crate::conflicts::sub_board_work;
use crate::model::{
    BoardId, ElementId, Error, RedFlagDates, RedFlagId, RedFlagTypeId, Schedule, SheetId,
    SubBoardId,
};
use crate::Date;

/// One red flag broken on one date.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clash {
    pub date: Date,
    pub element: ElementId,
    pub flag: RedFlagId,
    pub kind: RedFlagTypeId,
    pub hard: bool,
    /// The shoot day, from 1, when the element works that date.
    pub day: Option<u32>,
    /// The sheets that day listing the element, in board order. Empty when it doesn't
    /// work that date.
    pub sheets: Vec<SheetId>,
    /// The counted events covering the date that list the element: their indexes in the
    /// sub-board's calendar.
    pub events: Vec<usize>,
}

/// What an element does on a date: the shoot day and its sheets, and counted events.
type Busy = (Option<u32>, Vec<SheetId>, Vec<usize>);

/// What [`red_flags`] found on one sub-board.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RedFlags {
    /// Sorted by date, then by the element's place in the schedule, then by the flag's
    /// place on the element.
    pub clashes: Vec<Clash>,
    /// The sub-board's strips have no dates (no start date, or a week with no working
    /// day), so only events were checked.
    pub undated: bool,
}

/// The red flag clashes on one sub-board of `board`.
pub fn red_flags(s: &Schedule, board: BoardId, sub: SubBoardId) -> Result<RedFlags, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let flagged = |e: ElementId| s.element(e).is_some_and(|el| !el.red_flags.is_empty());
    let work = sub_board_work(s, board, sub, &flagged)?;
    let mut out = RedFlags {
        undated: work.is_none(),
        ..RedFlags::default()
    };
    let work = work.unwrap_or_default();

    // (element, date) -> counted events covering it, in calendar order.
    let mut events: BTreeMap<(ElementId, Date), Vec<usize>> = BTreeMap::new();
    for (index, ev) in sb.calendar.events.iter().enumerate() {
        if ev.kind.code().is_none() {
            continue;
        }
        let mut d = ev.from;
        while d <= ev.to {
            for el in ev.elements.iter().copied().filter(|e| flagged(*e)) {
                let list = events.entry((el, d)).or_default();
                if !list.contains(&index) {
                    list.push(index);
                }
            }
            d = d.add_days(1);
        }
    }

    // Every dated thing the flagged elements do, with what it is.
    let mut busy: BTreeMap<(ElementId, Date), Busy> = BTreeMap::new();
    for ((el, date), (day, sheets)) in work {
        busy.insert((el, date), (Some(day), sheets, Vec::new()));
    }
    for (key, evs) in events {
        busy.entry(key).or_insert((None, Vec::new(), Vec::new())).2 = evs;
    }

    let rank: HashMap<ElementId, usize> = s
        .elements
        .iter()
        .enumerate()
        .map(|(i, e)| (e.id, i))
        .collect();
    let mut keyed: Vec<((Date, usize, usize), Clash)> = Vec::new();
    for ((element, date), (day, sheets, evs)) in busy {
        let Some(el) = s.element(element) else {
            continue;
        };
        for (i, f) in el.red_flags.iter().enumerate() {
            if f.dates.covers(date) {
                keyed.push((
                    (date, rank[&element], i),
                    Clash {
                        date,
                        element,
                        flag: f.id,
                        kind: f.kind,
                        hard: f.hard,
                        day,
                        sheets: sheets.clone(),
                        events: evs.clone(),
                    },
                ));
            }
        }
    }
    keyed.sort_by_key(|(k, _)| *k);
    out.clashes = keyed.into_iter().map(|(_, c)| c).collect();
    Ok(out)
}

/// Dates picked one by one (several on a small calendar, behavior note 23) as the fewest
/// ranges: each run of following dates one range, in date order, each date once.
pub fn runs(dates: &[Date]) -> Vec<RedFlagDates> {
    let mut d: Vec<Date> = dates.to_vec();
    d.sort_unstable();
    d.dedup();
    let mut out: Vec<RedFlagDates> = Vec::new();
    for x in d {
        if let Some(RedFlagDates::Range { to, .. }) = out.last_mut() {
            if to.add_days(1) == x {
                *to = x;
                continue;
            }
        }
        out.push(RedFlagDates::Range { from: x, to: x });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        DayKind, Event, EventKind, Place, RedFlag, RedFlagDates, Sheet, StripId, StripKind,
    };

    fn d(s: &str) -> Date {
        s.parse().unwrap()
    }

    /// Main from Mon 2027-03-01, one sheet a day: day 1 ADA, day 2 ADA and BEN, day 3 BEN,
    /// day 4 ADA (Mon to Thu, Mar 1 to 4). A fifth sheet with ADA sits below the last day
    /// break. Types "Unavailable" and "Second job".
    fn fixture() -> (Schedule, BoardId, SubBoardId, ElementId, ElementId) {
        let mut s = Schedule::new("ZTEST Red Flags");
        let cast = s.category_named("Cast").unwrap();
        let ada = s.add_element(cast, "ADA").unwrap();
        let ben = s.add_element(cast, "BEN").unwrap();
        let b = s.boards[0].id;
        let main = s.boards[0].sub_boards[0].id;
        let lists = [vec![ada], vec![ada, ben], vec![ben], vec![ada], vec![ada]];
        for (i, elements) in lists.into_iter().enumerate() {
            let sheet = s.add_sheet(Sheet {
                scenes: (i + 1).to_string(),
                elements,
                ..Sheet::default()
            });
            let strip: StripId = s.boards[0]
                .boneyard
                .iter()
                .find(|x| x.kind == StripKind::Sheet(sheet))
                .unwrap()
                .id;
            let end = s.boards[0].sub_boards[0].strips.len();
            let at = Place {
                sub_board: Some(main),
                index: end,
            };
            s.move_strips(b, &[strip], at).unwrap();
            if i < 4 {
                s.insert_strip(
                    b,
                    StripKind::DayBreak,
                    Place {
                        sub_board: Some(main),
                        index: end + 1,
                    },
                )
                .unwrap();
            }
        }
        s.boards[0].sub_boards[0].start = Some(d("2027-03-01"));
        s.add_red_flag_type("Unavailable").unwrap();
        s.add_red_flag_type("Second job").unwrap();
        s.check().unwrap();
        (s, b, main, ada, ben)
    }

    fn flag(
        s: &mut Schedule,
        el: ElementId,
        kind: &str,
        dates: RedFlagDates,
        hard: bool,
    ) -> RedFlagId {
        let kind = s.red_flag_type_named(kind).unwrap();
        s.set_red_flag(
            el,
            RedFlag {
                id: RedFlagId(0),
                kind,
                dates,
                note: String::new(),
                hard,
            },
        )
        .unwrap()
    }

    fn summary(r: &RedFlags) -> Vec<(String, ElementId, RedFlagId, Option<u32>, usize)> {
        r.clashes
            .iter()
            .map(|c| (c.date.to_string(), c.element, c.flag, c.day, c.events.len()))
            .collect()
    }

    #[test]
    fn work_on_a_flagged_date_clashes() {
        let (mut s, b, main, ada, ben) = fixture();
        assert_eq!(red_flags(&s, b, main).unwrap(), RedFlags::default());
        // ADA away Mar 2 to 3 (hard); BEN not before Mar 3 (soft).
        let away = flag(
            &mut s,
            ada,
            "Unavailable",
            RedFlagDates::Range {
                from: d("2027-03-02"),
                to: d("2027-03-03"),
            },
            true,
        );
        let late = flag(
            &mut s,
            ben,
            "Second job",
            RedFlagDates::NotBefore(d("2027-03-03")),
            false,
        );
        let r = red_flags(&s, b, main).unwrap();
        assert!(!r.undated);
        // Mar 2: ADA and BEN both work day 2 (ADA first in the schedule). Mar 3: ADA is
        // off that day (BEN's day), BEN's flag allows Mar 3 on.
        assert_eq!(
            summary(&r),
            [
                ("2027-03-02".into(), ada, away, Some(2), 0),
                ("2027-03-02".into(), ben, late, Some(2), 0),
            ]
        );
        assert!(r.clashes[0].hard);
        assert!(!r.clashes[1].hard);
        assert_eq!(r.clashes[0].sheets, [s.sheets[1].id]);
        assert_eq!(
            r.clashes[0].kind,
            s.red_flag_type_named("unavailable").unwrap()
        );
    }

    #[test]
    fn open_ended_flags_unscheduled_strips_and_several_flags_a_day() {
        let (mut s, b, main, ada, _) = fixture();
        // ADA gone after Mar 3: day 4 (Mar 4) clashes; the fifth sheet below the last
        // break is not scheduled, so it never clashes.
        let gone = flag(
            &mut s,
            ada,
            "Second job",
            RedFlagDates::NotAfter(d("2027-03-03")),
            true,
        );
        // And a one-day flag on Mar 4 too: two clashes that day, in flag order.
        let day = flag(
            &mut s,
            ada,
            "Unavailable",
            RedFlagDates::Range {
                from: d("2027-03-04"),
                to: d("2027-03-04"),
            },
            false,
        );
        let r = red_flags(&s, b, main).unwrap();
        assert_eq!(
            summary(&r),
            [
                ("2027-03-04".into(), ada, gone, Some(4), 0),
                ("2027-03-04".into(), ada, day, Some(4), 0),
            ]
        );
        // A holiday on Mar 3 pushes day 3 to Mar 4 and day 4 to Mar 5: still after Mar 3,
        // and the one-day flag now meets BEN's day, not ADA's.
        s.boards[0].sub_boards[0]
            .calendar
            .set_day_kind(d("2027-03-03"), DayKind::Holiday);
        let r = red_flags(&s, b, main).unwrap();
        assert_eq!(summary(&r), [("2027-03-05".into(), ada, gone, Some(4), 0)]);
    }

    #[test]
    fn counted_events_clash_and_other_events_do_not() {
        let (mut s, b, main, ada, ben) = fixture();
        let away = flag(
            &mut s,
            ben,
            "Unavailable",
            RedFlagDates::Range {
                from: d("2027-02-25"),
                to: d("2027-02-26"),
            },
            true,
        );
        flag(
            &mut s,
            ada,
            "Unavailable",
            RedFlagDates::Range {
                from: d("2027-02-26"),
                to: d("2027-02-26"),
            },
            true,
        );
        let ev = |kind: EventKind, on: &str, who: Vec<ElementId>| Event {
            name: "Fit".into(),
            kind,
            from: d(on),
            to: d(on),
            elements: who,
            color: None,
        };
        // A fitting for BEN on Feb 26 (before the start: counted events have their own
        // dates), and a camera test for ADA that day (counted nowhere, so no clash).
        s.set_event(
            b,
            main,
            None,
            ev(EventKind::Fitting, "2027-02-26", vec![ben]),
        )
        .unwrap();
        s.set_event(
            b,
            main,
            None,
            ev(
                EventKind::Other("Camera test".into()),
                "2027-02-26",
                vec![ada],
            ),
        )
        .unwrap();
        let r = red_flags(&s, b, main).unwrap();
        assert_eq!(summary(&r), [("2027-02-26".into(), ben, away, None, 1)]);
        assert_eq!(r.clashes[0].events, [0]);
        assert!(r.clashes[0].sheets.is_empty());
        // Without a start date the strips have no dates, but events still clash.
        s.boards[0].sub_boards[0].start = None;
        let r = red_flags(&s, b, main).unwrap();
        assert!(r.undated);
        assert_eq!(r.clashes.len(), 1);
    }

    #[test]
    fn other_sub_boards_and_missing_ids() {
        let (mut s, b, main, ada, _) = fixture();
        flag(
            &mut s,
            ada,
            "Unavailable",
            RedFlagDates::NotBefore(d("2027-04-01")),
            false,
        );
        // Every scheduled day of ADA on Main is before April.
        assert_eq!(red_flags(&s, b, main).unwrap().clashes.len(), 3);
        // A second sub-board with nothing on it has no clashes; flags are global, the
        // clashes are the sub-board's.
        let unit = s.add_sub_board(b, "Unit B", Some(d("2027-03-01"))).unwrap();
        assert!(red_flags(&s, b, unit).unwrap().clashes.is_empty());
        assert!(red_flags(&s, BoardId(9999), main).is_err());
        assert!(red_flags(&s, b, SubBoardId(9999)).is_err());
    }

    #[test]
    fn edits_are_checked() {
        let (mut s, _, _, ada, ben) = fixture();
        let kind = s.red_flag_type_named("Unavailable").unwrap();
        let bad = RedFlag {
            id: RedFlagId(0),
            kind,
            dates: RedFlagDates::Range {
                from: d("2027-03-05"),
                to: d("2027-03-04"),
            },
            note: String::new(),
            hard: false,
        };
        assert!(s.set_red_flag(ada, bad.clone()).is_err());
        let ok = RedFlag {
            dates: RedFlagDates::NotAfter(d("2027-03-04")),
            ..bad.clone()
        };
        assert!(s
            .set_red_flag(
                ada,
                RedFlag {
                    kind: RedFlagTypeId(9999),
                    ..ok.clone()
                }
            )
            .is_err());
        assert!(s.set_red_flag(ElementId(9999), ok.clone()).is_err());
        let id = s.set_red_flag(ada, ok.clone()).unwrap();
        // Replace in place; another element can't take it; a missing id is refused.
        let changed = RedFlag {
            id,
            note: "Wedding".into(),
            ..ok.clone()
        };
        assert_eq!(s.set_red_flag(ada, changed.clone()), Ok(id));
        assert_eq!(s.red_flag(id).unwrap().1.note, "Wedding");
        assert!(s.set_red_flag(ben, changed).is_err());
        assert!(s
            .set_red_flag(
                ada,
                RedFlag {
                    id: RedFlagId(9999),
                    ..ok.clone()
                }
            )
            .is_err());
        // A type in use can't be deleted; names are unique ignoring case.
        assert!(matches!(s.delete_red_flag_type(kind), Err(Error::InUse(_))));
        assert_eq!(s.add_red_flag_type(" unavailable ").unwrap(), kind);
        assert!(s.add_red_flag_type("  ").is_err());
        let job = s.red_flag_type_named("Second job").unwrap();
        assert!(s.rename_red_flag_type(job, "UNAVAILABLE").is_err());
        s.move_red_flag_type(job, 0).unwrap();
        assert_eq!(s.red_flag_types[0].id, job);
        s.check().unwrap();
        // Merging elements carries the flags over.
        s.merge_elements(ben, &[ada]).unwrap();
        assert_eq!(s.red_flag(id).unwrap().0, ben);
        s.delete_red_flag(id).unwrap();
        assert!(s.delete_red_flag(id).is_err());
        s.delete_red_flag_type(kind).unwrap();
        s.check().unwrap();
        // check catches a flag with a missing type or reversed dates.
        let mut broken = s.clone();
        broken.elements[0].red_flags.push(RedFlag {
            id: RedFlagId(broken.next_id),
            ..ok
        });
        broken.next_id += 1;
        assert!(broken.check().is_err());
    }
}
