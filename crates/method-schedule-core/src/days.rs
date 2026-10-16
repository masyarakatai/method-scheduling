//! Shoot days: the strips between day breaks, their totals, and automatic day breaks.
//!
//! Rules are the engine defaults from `docs/research/engine-checks.md` (E-B1 to E-B6,
//! E-D15) until the real app confirms them.

use crate::model::{
    Board, Error, Minutes, Schedule, Seconds, SheetId, Strip, StripId, StripKind, SubBoardId,
};
use crate::Eighths;

/// One strip of a day, with what callers ask of it, so none has to find it again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DayStrip {
    pub id: StripId,
    /// Its place in the sub-board's strips.
    pub at: usize,
    /// The sheet of a scene strip; none for a banner.
    pub sheet: Option<SheetId>,
    /// A scene strip that is its sheet's first on the board: a later part of a scene
    /// spanning days counts no pages or time (decision 76). False for a banner.
    pub first: bool,
    /// A scene a script revision left out (decision 69): it stays on the board, struck,
    /// but counts nowhere: no pages or time, no work for its elements (decision 91).
    pub omitted: bool,
}

impl DayStrip {
    /// The sheet of a scene that counts: on the day and not omitted.
    pub fn counted_sheet(&self) -> Option<SheetId> {
        self.sheet.filter(|_| !self.omitted)
    }
}

impl Day {
    /// The sheets of the day's scenes that count (not omitted), in board order; a sheet
    /// with two parts on the day is listed twice.
    pub fn counted_sheets(&self) -> impl Iterator<Item = SheetId> + '_ {
        self.entries.iter().filter_map(|e| e.counted_sheet())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Day {
    /// From 1, counted within the sub-board.
    pub number: u32,
    /// Sheet and banner strips, in board order.
    pub strips: Vec<StripId>,
    /// The same strips with their places and sheets.
    pub entries: Vec<DayStrip>,
    pub pages: Eighths,
    /// Shoot time of the sheets plus banner durations.
    pub time: Minutes,
    pub screen_time: Seconds,
    /// The day break that closes this day.
    pub break_id: StripId,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Days {
    pub days: Vec<Day>,
    /// Strips after the last day break. They are not scheduled (E-D15).
    pub unscheduled: Vec<StripId>,
}

/// What an automatic day break run limits each day by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Limit {
    Pages(Eighths),
    Time(Minutes),
    ScreenTime(Seconds),
}

fn sub_board(board: &Board, sub: SubBoardId) -> Result<&crate::model::SubBoard, Error> {
    board
        .sub_boards
        .iter()
        .find(|s| s.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))
}

/// The first strip of each sheet on a board, in board order: a scene spanning days counts
/// its pages and times there only (decision 76).
pub fn first_parts(b: &Board) -> crate::model::IdSet<StripId> {
    b.sheet_parts()
        .filter(|(_, _, first)| *first)
        .map(|(st, _, _)| st.id)
        .collect()
}

/// What one strip adds to its day: pages, minutes, screen seconds. A later part of a
/// scene spanning days adds nothing (`first` lists the first parts), nor does an
/// omitted scene.
fn weight_of(
    s: &Schedule,
    sheets: &crate::model::SheetIndex<'_>,
    first: &crate::model::IdSet<StripId>,
    st: &Strip,
) -> (Eighths, Minutes, Seconds) {
    match &st.kind {
        StripKind::Sheet(id) if !first.contains(&st.id) || s.plan.is_omitted(*id) => {
            (Eighths::ZERO, Minutes(0), Seconds(0))
        }
        StripKind::Sheet(id) => {
            sheets
                .get(id)
                .map_or((Eighths::ZERO, Minutes(0), Seconds(0)), |sh| {
                    (
                        sh.pages,
                        sh.shoot_time.unwrap_or_default(),
                        sh.screen_time.unwrap_or_default(),
                    )
                })
        }
        StripKind::Banner(b) => (Eighths::ZERO, b.duration, Seconds(0)),
        StripKind::DayBreak => (Eighths::ZERO, Minutes(0), Seconds(0)),
    }
}

/// The shoot days of one sub-board. Every day break closes a day, so two breaks in a row
/// make an empty day (E-B5).
pub fn days(s: &Schedule, board: crate::model::BoardId, sub: SubBoardId) -> Result<Days, Error> {
    days_with(s, &s.sheet_index(), board, sub)
}

/// [`days`] with the sheets already indexed (one index for a whole view or report).
pub fn days_with(
    s: &Schedule,
    sheets: &crate::model::SheetIndex<'_>,
    board: crate::model::BoardId,
    sub: SubBoardId,
) -> Result<Days, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = sub_board(b, sub)?;
    let first = first_parts(b);
    let mut out = Days::default();
    let empty = |number| Day {
        number,
        strips: Vec::new(),
        entries: Vec::new(),
        pages: Eighths::ZERO,
        time: Minutes(0),
        screen_time: Seconds(0),
        break_id: StripId(0),
    };
    let mut cur = empty(1);
    for (at, st) in sb.strips.iter().enumerate() {
        if st.kind == StripKind::DayBreak {
            let number = cur.number.saturating_add(1);
            cur.break_id = st.id;
            out.days.push(std::mem::replace(&mut cur, empty(number)));
            continue;
        }
        let (p, t, sc) = weight_of(s, sheets, &first, st);
        let sheet = match st.kind {
            StripKind::Sheet(id) => Some(id),
            _ => None,
        };
        cur.strips.push(st.id);
        cur.entries.push(DayStrip {
            id: st.id,
            at,
            sheet,
            first: sheet.is_some() && first.contains(&st.id),
            omitted: sheet.is_some_and(|h| s.plan.is_omitted(h)),
        });
        cur.pages += p;
        cur.time.0 = cur.time.0.saturating_add(t.0);
        cur.screen_time.0 = cur.screen_time.0.saturating_add(sc.0);
    }
    out.unscheduled = cur.strips;
    Ok(out)
}

/// Replaces every day break in the sub-board with new ones. Strips are added to a day in
/// board order, and the day closes **as soon as its total reaches or passes the limit**,
/// so a day can run over (checked in the real app, behavior note 16). A strip at or over
/// the limit on its own therefore gets a day to itself. Banners stay where they are and
/// count their duration toward a time limit. A break closes the last day (E-B6).
///
/// The new breaks take the old ones' ids in order, so day 1 keeps day 1's call time and
/// so on; only breaks past the old count are new, and old ones past the new count go
/// with their call times.
pub fn auto_day_breaks(
    s: &mut Schedule,
    board: crate::model::BoardId,
    sub: SubBoardId,
    limit: Limit,
) -> Result<(), Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let first = first_parts(b);
    let (old_breaks, strips): (Vec<_>, Vec<_>) = sub_board(b, sub)?
        .strips
        .iter()
        .cloned()
        .partition(|st| st.kind == StripKind::DayBreak);
    let mut old_breaks = old_breaks.into_iter().map(|st| st.id);

    let sheets = s.sheet_index();
    let amount = |st: &crate::model::Strip| -> u32 {
        let (p, t, sc) = weight_of(s, &sheets, &first, st);
        match limit {
            Limit::Pages(_) => p.0,
            Limit::Time(_) => t.0,
            Limit::ScreenTime(_) => sc.0,
        }
    };
    let max = match limit {
        Limit::Pages(e) => e.0,
        Limit::Time(m) => m.0,
        Limit::ScreenTime(sc) => sc.0,
    };

    // Positions (in the new list without breaks) before which a break goes.
    let mut breaks_after = Vec::new();
    let mut total: u32 = 0;
    let mut has_sheet = false;
    // The sheets in the open day: a second part of one starts a new day, so a scene
    // spanning days never has two parts in one day (DBK-11, decision 134).
    let mut in_day = std::collections::HashSet::new();
    for (i, st) in strips.iter().enumerate() {
        if let StripKind::Sheet(id) = st.kind {
            if !in_day.insert(id) {
                breaks_after.push(i);
                total = 0;
                has_sheet = false;
                in_day.clear();
                in_day.insert(id);
            }
        }
        total = total.saturating_add(amount(st));
        has_sheet |= matches!(st.kind, StripKind::Sheet(_));
        if has_sheet && total >= max {
            breaks_after.push(i + 1);
            total = 0;
            has_sheet = false;
            in_day.clear();
        }
    }
    if has_sheet || total > 0 {
        breaks_after.push(strips.len());
    }

    let mut fresh = Vec::with_capacity(strips.len() + breaks_after.len());
    let mut next = breaks_after.iter().peekable();
    let mut day_break = |s: &mut Schedule| {
        let id = old_breaks.next().unwrap_or_else(|| StripId(s.fresh()));
        Strip {
            id,
            kind: StripKind::DayBreak,
        }
    };
    for (i, st) in strips.into_iter().enumerate() {
        while next.peek() == Some(&&i) {
            next.next();
            fresh.push(day_break(s));
        }
        fresh.push(st);
    }
    for _ in next {
        fresh.push(day_break(s));
    }
    let b = s.boards.iter_mut().find(|b| b.id == board).unwrap();
    b.sub_boards
        .iter_mut()
        .find(|x| x.id == sub)
        .unwrap()
        .strips = fresh;
    crate::plan::tidy(s);
    Ok(())
}

/// Removes every day break from a sub-board ("remove all", inventory section 6, engine
/// check E-B4): its strips stay in order, all of them unscheduled. Returns how many went.
pub fn remove_day_breaks(
    s: &mut Schedule,
    board: crate::model::BoardId,
    sub: SubBoardId,
) -> Result<usize, Error> {
    let b = s
        .boards
        .iter_mut()
        .find(|b| b.id == board)
        .ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter_mut()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let before = sb.strips.len();
    sb.strips.retain(|st| st.kind != StripKind::DayBreak);
    let n = before - sb.strips.len();
    // Their call times go with them.
    crate::plan::tidy(s);
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Banner, Place, Sheet};

    /// A schedule with sheets of the given lengths (in eighths) scheduled on Main.
    fn board(lengths: &[u32]) -> (Schedule, crate::model::BoardId, SubBoardId) {
        let mut s = Schedule::new("ZTEST");
        for (i, l) in lengths.iter().enumerate() {
            s.add_sheet(Sheet {
                scenes: (i + 1).to_string(),
                pages: Eighths(*l),
                shoot_time: Some(Minutes(60 * l)),
                ..Sheet::default()
            });
        }
        let b = s.boards[0].id;
        let sb = s.boards[0].sub_boards[0].id;
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
        (s, b, sb)
    }

    fn shape(s: &Schedule, b: crate::model::BoardId, sb: SubBoardId) -> Vec<String> {
        days(s, b, sb)
            .unwrap()
            .days
            .iter()
            .map(|d| d.pages.to_string())
            .collect()
    }

    #[test]
    fn totals_per_day_and_unscheduled_tail() {
        let (mut s, b, sb) = board(&[3, 5, 4]);
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 2,
            },
        )
        .unwrap();
        let d = days(&s, b, sb).unwrap();
        assert_eq!(d.days.len(), 1);
        let brk = s.boards[0].sub_boards[0].strips[2].id;
        assert_eq!(d.days[0].break_id, brk);
        assert_eq!(d.days[0].pages, Eighths(8));
        assert_eq!(d.days[0].time, Minutes(480));
        assert_eq!(d.unscheduled.len(), 1);
    }

    #[test]
    fn two_breaks_in_a_row_make_an_empty_day() {
        let (mut s, b, sb) = board(&[8]);
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 1,
            },
        )
        .unwrap();
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 2,
            },
        )
        .unwrap();
        let d = days(&s, b, sb).unwrap();
        assert_eq!(d.days.len(), 2);
        assert!(d.days[1].strips.is_empty());
        assert_eq!(d.days[1].number, 2);
    }

    #[test]
    fn auto_closes_a_day_once_it_reaches_the_limit() {
        // ZTEST run 3 (behavior note 16): 1 | 1 | 1/8 + 4/8 + 1 2/8 | 2/8 + 3/8 + 5/8 |
        // 4/8 + 4/8 | 1 2/8 | 2 1/8 | the empty sheet.
        let (mut s, b, sb) = board(&[8, 8, 1, 4, 10, 2, 3, 5, 4, 4, 10, 17, 0]);
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        assert_eq!(
            shape(&s, b, sb),
            ["1", "1", "1 7/8", "1 2/8", "1", "1 2/8", "2 1/8", "0"]
        );
        assert!(days(&s, b, sb).unwrap().unscheduled.is_empty());
    }

    #[test]
    fn auto_with_exact_fits() {
        // 3/8 + 5/8 | 4/8 + 4/8 | 1
        let (mut s, b, sb) = board(&[3, 5, 4, 4, 8]);
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        assert_eq!(shape(&s, b, sb), ["1", "1", "1"]);
        assert!(days(&s, b, sb).unwrap().unscheduled.is_empty());
        s.check().unwrap();
    }

    #[test]
    fn a_strip_over_the_limit_joins_the_open_day() {
        // The day was under the limit, so the long strip joins it and closes it.
        let (mut s, b, sb) = board(&[2, 10, 2]);
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        assert_eq!(shape(&s, b, sb), ["1 4/8", "2/8"]);
        // Starting a day, it fills the day alone.
        let (mut s, b, sb) = board(&[10, 2]);
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        assert_eq!(shape(&s, b, sb), ["1 2/8", "2/8"]);
    }

    #[test]
    fn auto_replaces_existing_breaks() {
        // E-B4
        let (mut s, b, sb) = board(&[4, 4, 4, 4]);
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 1,
            },
        )
        .unwrap();
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 3,
            },
        )
        .unwrap();
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        assert_eq!(shape(&s, b, sb), ["1", "1"]);
        s.check().unwrap();
    }

    #[test]
    fn banners_count_toward_a_time_limit() {
        // 3:00, 3:00, banner 1:30, 2:00 with a 7:30 limit: the banner brings the day to
        // the limit and closes it (E-B3 still to check in the real app).
        let (mut s, b, sb) = board(&[3, 3, 2]);
        let banner = Banner {
            text: "Lunch move".to_string(),
            duration: Minutes(90),
            ..Default::default()
        };
        s.insert_strip(
            b,
            StripKind::Banner(banner),
            Place {
                sub_board: Some(sb),
                index: 2,
            },
        )
        .unwrap();
        auto_day_breaks(&mut s, b, sb, Limit::Time(Minutes(450))).unwrap();
        let d = days(&s, b, sb).unwrap();
        assert_eq!(
            d.days.iter().map(|d| d.time.0).collect::<Vec<_>>(),
            [450, 120]
        );
    }

    #[test]
    fn remove_all_leaves_the_strips_in_order_and_unscheduled() {
        let (mut s, b, sb) = board(&[4, 4, 4, 4]);
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        let before: Vec<_> = s.boards[0].sub_boards[0]
            .strips
            .iter()
            .filter(|x| x.kind != StripKind::DayBreak)
            .map(|x| x.id)
            .collect();
        assert_eq!(remove_day_breaks(&mut s, b, sb).unwrap(), 2);
        let d = days(&s, b, sb).unwrap();
        assert!(d.days.is_empty());
        assert_eq!(d.unscheduled, before);
        assert_eq!(remove_day_breaks(&mut s, b, sb).unwrap(), 0);
        assert!(remove_day_breaks(&mut s, b, SubBoardId(999_999)).is_err());
        s.check().unwrap();
    }

    #[test]
    fn auto_by_screen_time_closes_a_day_at_the_limit() {
        let (mut s, b, sb) = board(&[1, 1, 1]);
        for (i, secs) in [90, 40, 120].into_iter().enumerate() {
            let id = s.sheets[i].id;
            s.sheet_mut(id).unwrap().screen_time = Some(Seconds(secs));
        }
        auto_day_breaks(&mut s, b, sb, Limit::ScreenTime(Seconds(120))).unwrap();
        let d = days(&s, b, sb).unwrap();
        assert_eq!(
            d.days.iter().map(|d| d.screen_time.0).collect::<Vec<_>>(),
            [130, 120]
        );
    }

    #[test]
    fn a_scene_spanning_days_counts_its_length_once_and_works_every_day() {
        // Decision 76: scene 1 (one page) split across days 1 and 2.
        let (mut s, b, sb) = board(&[8, 4]);
        let cast = s.category_named("Cast").unwrap();
        let ada = s.add_element(cast, "ADA").unwrap();
        let one = s.sheets[0].id;
        s.tag(one, ada).unwrap();
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 1,
            },
        )
        .unwrap();
        // 1 | 2, then a part of 1 first in day 2, closed by a break.
        let part = s
            .add_part(
                b,
                one,
                Place {
                    sub_board: Some(sb),
                    index: 2,
                },
            )
            .unwrap();
        s.insert_strip(
            b,
            StripKind::DayBreak,
            Place {
                sub_board: Some(sb),
                index: 4,
            },
        )
        .unwrap();
        s.check().unwrap();
        assert_eq!(shape(&s, b, sb), ["1", "4/8"], "the part adds no pages");
        let d = days(&s, b, sb).unwrap();
        assert_eq!(d.days[1].strips[0], part);
        assert_eq!(d.days[1].time, Minutes(240));
        assert_eq!(s.sheet_strips(b, one).len(), 2);
        assert_eq!(s.sheets_in_board_order(b).len(), 2, "listed once");
        s.boards[0].sub_boards[0].start = "2027-03-01".parse().ok();
        let rows = crate::dood::dood(&s, b, sb).unwrap();
        assert_eq!(rows[0].worked, 2, "ADA works both days");
        // Never in the Boneyard while split; a part can be deleted on its own.
        assert!(s
            .move_strips(
                b,
                &[part],
                Place {
                    sub_board: None,
                    index: 0
                }
            )
            .is_err());
        assert!(s
            .add_part(
                b,
                one,
                Place {
                    sub_board: None,
                    index: 0
                }
            )
            .is_err());
        s.delete_strip(b, part).unwrap();
        assert_eq!(s.sheet_strips(b, one).len(), 1);
        let last = s.sheet_strips(b, one)[0];
        assert!(s.delete_strip(b, last).is_err(), "its only strip stays");
        s.check().unwrap();
    }

    #[test]
    fn sorting_keeps_a_scenes_parts_together_and_breaking_keeps_them_apart() {
        // DBK-11: scene 1 in two parts, scene 2 between them; sorted by scene then broken
        // at a limit no part reaches alone.
        let (mut s, b, sb) = board(&[2, 2, 2]);
        let one = s.sheets[0].id;
        let part = s
            .add_part(
                b,
                one,
                Place {
                    sub_board: Some(sb),
                    index: 2,
                },
            )
            .unwrap();
        let first = s.sheet_strips(b, one)[0];
        let scene = |s: &Schedule, st: StripId| -> String {
            let b = s.board(b).unwrap();
            let x = b.sub_boards[0].strips.iter().find(|x| x.id == st).unwrap();
            match x.kind {
                StripKind::Sheet(h) => s.sheet(h).unwrap().scenes.clone(),
                _ => "|".into(),
            }
        };
        let order = |s: &Schedule| -> Vec<String> {
            let b = s.board(b).unwrap();
            b.sub_boards[0]
                .strips
                .iter()
                .map(|x| scene(s, x.id))
                .collect()
        };
        assert_eq!(order(&s), ["1", "2", "1", "3"]);
        let key = crate::model::SortKey {
            field: crate::model::SortField::Scene,
            descending: true,
        };
        s.sort_strips(b, Some(sb), &[key]).unwrap();
        assert_eq!(
            order(&s),
            ["3", "2", "1", "1"],
            "the parts move as one block"
        );
        let strips = &s.board(b).unwrap().sub_boards[0].strips;
        assert_eq!(strips[2].id, first, "in part order");
        assert_eq!(strips[3].id, part);
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(16))).unwrap();
        assert_eq!(order(&s), ["3", "2", "1", "|", "1", "|"]);
        let d = days(&s, b, sb).unwrap();
        assert_eq!(d.days.len(), 2);
        assert_eq!(d.days[1].strips, [part]);
        // A limit of a page a day: one part a day still.
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(4))).unwrap();
        assert_eq!(order(&s), ["3", "2", "|", "1", "|", "1", "|"]);
        s.check().unwrap();
    }

    #[test]
    fn empty_sub_board_gets_no_breaks() {
        let mut s = Schedule::new("ZTEST");
        let b = s.boards[0].id;
        let sb = s.boards[0].sub_boards[0].id;
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
        assert!(days(&s, b, sb).unwrap().days.is_empty());
    }
}
