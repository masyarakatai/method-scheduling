//! Day out of days: per element, what it does on each date of a sub-board's schedule.
//!
//! Rules checked in the real app (decisions.md, entry 10; behavior note 14):
//! - Work comes from strips above a day break. S marks the first work day, F the last.
//! - With holds on, every shoot day strictly between first and last work is held. Days
//!   off count nowhere.
//! - With holds and drops on, the longest gap of at least the minimum (calendar days
//!   strictly between two work days) becomes the one drop; ties go to the earlier gap.
//!   D joins the last work day before it, P the first after. Days inside count nowhere.
//! - A holiday inside the work span counts as worked when holds are on and it is not
//!   inside the drop.
//!
//! - Travel, fitting and rehearsal events count as worked wherever they fall, also before
//!   the start or after the finish (the row grows to show them). They never move S or F.
//!   On a day that would be held they show with H (`FTH`) but count as worked, not held.
//!   Inside the work span they split gaps, so one can stop a drop, and they act as work
//!   days for the drop's bounds: an event day at a drop's edge takes the D or P after its
//!   H (`TRHD`, `TRHP`), and the work day beside it loses it (E-D22, behavior note 27).
//! - A company travel day takes no shoot day number and counts like a holiday (`M`).
//!
//! - A counted event on a day the element also works (E-D6, behavior notes 17 and 18)
//!   shows the event's code followed by the whole work code (a fitting on a pickup day is
//!   `FTPW`, on a plain work day `FTW`; travel on the first day `TRSW`, on the last
//!   `TRWF`), counts once, as a worked day, and is flagged: it is a conflict
//!   ([`crate::conflicts`]).
//!
//! Checked in ZTEST runs 1, 2 and screen blocks 1 and 2 (behavior notes 14, 15, 17 and
//! 18).

use std::collections::BTreeSet;

use crate::days::days_with;
use crate::model::{BoardId, DoodRules, ElementId, Error, Schedule, SubBoardId};
use crate::Date;

/// What one element does on one date.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Work {
        start: bool,
        finish: bool,
        drop: bool,
        pickup: bool,
    },
    Hold,
    /// A counted event. `held` is set when the day would otherwise be a hold; `drop` and
    /// `pickup` when the event bounds the drop (E-D22, decision 139).
    Event {
        code: &'static str,
        held: bool,
        drop: bool,
        pickup: bool,
    },
    /// A counted event on a day the element works on this sub-board: counted once, as a
    /// work day, and flagged as a conflict. Its code is the event's code followed by the
    /// whole work code (decision 47).
    EventOnWork {
        code: &'static str,
        start: bool,
        finish: bool,
        drop: bool,
        pickup: bool,
    },
    /// A holiday that counts as worked.
    Holiday,
    /// A company travel day that counts as worked.
    Move,
    /// Inside the span but counting nowhere: a day off, a day inside the drop, an
    /// uncounted holiday, or an unheld shoot day.
    Idle,
}

impl Cell {
    /// [`Cell::code`] in the schedule's own letters (crate::setup::DoodCodes). With the
    /// default letters the two are the same.
    pub fn code_with(self, c: &crate::setup::DoodCodes) -> String {
        let work = |start: bool, finish: bool, drop: bool, pickup: bool| {
            let mut s = String::new();
            if start {
                s.push_str(&c.start);
            }
            if pickup {
                s.push_str(&c.pickup);
            }
            s.push_str(&c.work);
            if drop {
                s.push_str(&c.drop);
            }
            if finish {
                s.push_str(&c.finish);
            }
            s
        };
        match self {
            Cell::Work {
                start,
                finish,
                drop,
                pickup,
            } => work(start, finish, drop, pickup),
            Cell::Hold => c.hold.clone(),
            Cell::Event {
                code,
                held,
                drop,
                pickup,
            } => {
                format!(
                    "{}{}{}{}",
                    c.event(code),
                    if held { c.hold.as_str() } else { "" },
                    if drop { c.drop.as_str() } else { "" },
                    if pickup { c.pickup.as_str() } else { "" }
                )
            }
            Cell::EventOnWork {
                code,
                start,
                finish,
                drop,
                pickup,
            } => format!("{}{}", c.event(code), work(start, finish, drop, pickup)),
            Cell::Holiday => c.holiday.clone(),
            Cell::Move => c.company_move.clone(),
            Cell::Idle => String::new(),
        }
    }

    /// The code a day out of days shows: `SW`, `W`, `WF`, `SWF`, `SWD`, `WD`, `PW`, `PWF`,
    /// `H`, `/`, an event's code (`FT`, `FTH`, at a drop's edge `TRHD` or `TRHP`, or on a work day followed by the work code:
    /// `FTW`, `FTPW`, `TRSW`, `FTWF`), or empty.
    pub fn code(self) -> String {
        match self {
            Cell::Work {
                start,
                finish,
                drop,
                pickup,
            } => {
                let mut s = String::new();
                if start {
                    s.push('S');
                }
                if pickup {
                    s.push('P');
                }
                s.push('W');
                if drop {
                    s.push('D');
                }
                if finish {
                    s.push('F');
                }
                s
            }
            Cell::Hold => "H".to_string(),
            // The event's code, H, then D or P (behavior note 27): `FTH`, `TRHD`, `TRHP`.
            Cell::Event {
                code,
                held,
                drop,
                pickup,
            } => format!(
                "{code}{}{}{}",
                if held { "H" } else { "" },
                if drop { "D" } else { "" },
                if pickup { "P" } else { "" }
            ),
            Cell::EventOnWork {
                code,
                start,
                finish,
                drop,
                pickup,
            } => {
                // The event's code, then the whole work code (behavior note 18): `TRSW`,
                // `FTPW`, `FTW`, `FTWF`.
                let work = Cell::Work {
                    start,
                    finish,
                    drop,
                    pickup,
                };
                format!("{code}{}", work.code())
            }
            Cell::Holiday => "/".to_string(),
            Cell::Move => "M".to_string(),
            Cell::Idle => String::new(),
        }
    }

    /// A day the element works on the sub-board, with or without an event on it.
    pub fn is_work(self) -> bool {
        matches!(self, Cell::Work { .. } | Cell::EventOnWork { .. })
    }

    /// A day the DOOD flags with a warning: an event on a work day.
    pub fn is_flagged(self) -> bool {
        matches!(self, Cell::EventOnWork { .. })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub element: ElementId,
    /// First and last work day from the strips. `None` for an element with events only.
    pub start: Option<Date>,
    pub finish: Option<Date>,
    /// Every date from the first work or event day to the last, in order.
    pub cells: Vec<(Date, Cell)>,
    /// Work days, counted events, and holidays and travel days that count.
    pub worked: u32,
    pub held: u32,
}

/// One line of the day out of days summary. There is no holiday count: a holiday that
/// counts is in `work`, and the reference app's summary leaves its Holiday column empty
/// (E-D20, behavior note 25), so ours has none (decision 117). The budget exchange keeps
/// its own `holiday` count apart (EXCHANGE.md).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SummaryRow {
    pub element: ElementId,
    pub work: u32,
    pub hold: u32,
    pub start: Option<Date>,
    pub finish: Option<Date>,
    /// Work plus hold, as the reference product's summary shows it.
    pub total: u32,
}

/// The summary of a day out of days.
pub fn summary(rows: &[Row]) -> Vec<SummaryRow> {
    rows.iter()
        .map(|r| SummaryRow {
            element: r.element,
            work: r.worked,
            hold: r.held,
            start: r.start,
            finish: r.finish,
            total: r.worked + r.held,
        })
        .collect()
}

/// What a day out of days row can be sorted by (behavior note 01: board ID, name, first
/// and last work day, worked and held days).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    BoardId,
    Name,
    Start,
    Finish,
    Work,
    Hold,
}

impl SortKey {
    pub const ALL: [SortKey; 6] = [
        SortKey::BoardId,
        SortKey::Name,
        SortKey::Start,
        SortKey::Finish,
        SortKey::Work,
        SortKey::Hold,
    ];

    pub fn name(self) -> &'static str {
        match self {
            SortKey::BoardId => "board_id",
            SortKey::Name => "name",
            SortKey::Start => "start",
            SortKey::Finish => "finish",
            SortKey::Work => "work",
            SortKey::Hold => "hold",
        }
    }

    /// The key's name, or the reference app's word for it (behavior note 19):
    /// `first_work`, `last_work`, `worked`, `held`.
    pub fn from_name(name: &str) -> Option<SortKey> {
        match name {
            "first_work" => Some(SortKey::Start),
            "last_work" => Some(SortKey::Finish),
            "worked" => Some(SortKey::Work),
            "held" => Some(SortKey::Hold),
            _ => Self::ALL.into_iter().find(|k| k.name() == name),
        }
    }

    /// As the Sort dialog says it, in the reference app's terms (behavior note 19).
    pub fn label(self) -> &'static str {
        match self {
            SortKey::BoardId => "Board ID",
            SortKey::Name => "Element name",
            SortKey::Start => "First working day",
            SortKey::Finish => "Last working day",
            SortKey::Work => "Worked days",
            SortKey::Hold => "Held days",
        }
    }
}

/// Sorts rows by up to four keys, each ascending or descending (`true`: descending).
/// The sort is stable (ties keep the order they had) and a row missing the value (no
/// board ID, no start for an element with events only) goes last whichever the
/// direction. Behavior notes 01 and 27; the reference app keeps such a sort only while
/// its tab and sub-board are shown, and so does the app (it is never saved).
pub fn sort_rows(s: &Schedule, rows: &mut [Row], keys: &[(SortKey, bool)]) {
    use std::cmp::Ordering;
    let name = |r: &Row| {
        s.element(r.element)
            .map(|e| e.name.to_ascii_uppercase())
            .unwrap_or_default()
    };
    let id = |r: &Row| s.element(r.element).and_then(|e| e.board_id.clone());
    rows.sort_by(|a, b| {
        for &(key, desc) in keys.iter().take(4) {
            // Blanks last, whichever the direction.
            let blank = |x: bool, y: bool| match (x, y) {
                (true, false) => Some(Ordering::Greater),
                (false, true) => Some(Ordering::Less),
                (true, true) => Some(Ordering::Equal),
                _ => None,
            };
            // The direction applies to values; blanks stay last either way.
            let dir = |o: Ordering| if desc { o.reverse() } else { o };
            let o = match key {
                SortKey::BoardId => {
                    let (x, y) = (id(a), id(b));
                    blank(x.is_none(), y.is_none()).unwrap_or_else(|| {
                        dir(crate::reports::id_order(x.as_deref(), y.as_deref()))
                    })
                }
                SortKey::Name => dir(name(a).cmp(&name(b))),
                SortKey::Start | SortKey::Finish => {
                    let pick = |r: &Row| {
                        if key == SortKey::Start {
                            r.start
                        } else {
                            r.finish
                        }
                    };
                    let (x, y) = (pick(a), pick(b));
                    blank(x.is_none(), y.is_none()).unwrap_or_else(|| dir(x.cmp(&y)))
                }
                SortKey::Work | SortKey::Hold => {
                    let pick = |r: &Row| {
                        if key == SortKey::Work {
                            r.worked
                        } else {
                            r.held
                        }
                    };
                    dir(pick(a).cmp(&pick(b)))
                }
            };
            if o != Ordering::Equal {
                return o;
            }
        }
        Ordering::Equal
    });
}

/// Puts rows in category order (the schedule's), keeping their order within each
/// category: a day out of days is read a category at a time (DOD-01), so a summary of
/// every category lists each one's rows together rather than mixed by board ID.
pub fn group_by_category(s: &Schedule, rows: &mut [Row]) {
    let at = |r: &Row| {
        s.element(r.element)
            .and_then(|e| s.categories.iter().position(|c| c.id == e.category))
            .unwrap_or(usize::MAX)
    };
    rows.sort_by_key(at);
}

/// The work dates of every element on a sub-board: the dates of the shoot days whose
/// strips list it.
fn work_dates(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
) -> Result<Vec<(ElementId, BTreeSet<Date>)>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let start = sb
        .start
        .ok_or_else(|| Error::Broken(format!("sub-board {:?} has no start date", sb.name)))?;
    let index = s.sheet_index();
    let mut out: Vec<(ElementId, BTreeSet<Date>)> = Vec::new();
    let mut at: std::collections::HashMap<ElementId, usize> = std::collections::HashMap::new();
    let mut dates = sb.calendar.day_dates(start);
    for day in days_with(s, &index, board, sub)?.days {
        let Some(date) = dates.date(day.number) else {
            return Err(Error::Broken(
                "the working week has no shoot days".to_string(),
            ));
        };
        // An omitted scene is no work (decision 91).
        for sheet in day.counted_sheets() {
            let Some(h) = index.get(&sheet) else { continue };
            for el in &h.elements {
                let i = *at.entry(*el).or_insert_with(|| {
                    out.push((*el, BTreeSet::new()));
                    out.len() - 1
                });
                out[i].1.insert(date);
            }
        }
    }
    Ok(out)
}

/// The counted event dates of every element on a sub-board, with their codes.
fn event_dates(cal: &crate::model::Calendar) -> Vec<(ElementId, Date, &'static str)> {
    let mut out = Vec::new();
    for ev in &cal.events {
        let Some(code) = ev.kind.code() else { continue };
        let mut d = ev.from;
        while d <= ev.to {
            for el in &ev.elements {
                out.push((*el, d, code));
            }
            d = d.add_days(1);
        }
    }
    out
}

/// Calendar days strictly between two consecutive work or counted event days inside the
/// work span, weekends and holidays included (decision 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gap {
    /// The work or event day before the gap.
    pub after: Date,
    /// The work or event day after it.
    pub before: Date,
    pub days: u32,
}

/// The gaps of an element with work on `work` (sorted) and counted events on
/// `event_days`, in date order, and which one is the drop: with holds and drops on, the
/// longest gap of at least the minimum (never under 1 day), the earlier on a tie. Event
/// days outside the work span don't split gaps; consecutive days leave no gap.
pub fn gaps(work: &[Date], event_days: &[Date], rules: DoodRules) -> (Vec<Gap>, Option<usize>) {
    let (Some(&first), Some(&last)) = (work.first(), work.last()) else {
        return (Vec::new(), None);
    };
    let mut marks: BTreeSet<Date> = work.iter().copied().collect();
    marks.extend(
        event_days
            .iter()
            .copied()
            .filter(|d| *d >= first && *d <= last),
    );
    let marks: Vec<Date> = marks.into_iter().collect();
    let out: Vec<Gap> = marks
        .windows(2)
        .map(|w| Gap {
            after: w[0],
            before: w[1],
            days: (w[1].0 - w[0].0 - 1) as u32,
        })
        .filter(|g| g.days > 0)
        .collect();
    let mut chosen = None;
    if rules.holds && rules.drops {
        let mut best = 0;
        for (i, g) in out.iter().enumerate() {
            if g.days >= rules.min_drop_gap.max(1) && g.days > best {
                best = g.days;
                chosen = Some(i);
            }
        }
    }
    (out, chosen)
}

/// The day out of days row of every element that works or has an event on the
/// sub-board. Rows sort by board ID in natural order ([`crate::reports::id_order`]: 1,
/// 1a, 2, 7, 07, 9, 10, then IDs with no number, then none), then name (E-D14, behavior
/// notes 15 and 17).
pub fn dood(s: &Schedule, board: BoardId, sub: SubBoardId) -> Result<Vec<Row>, Error> {
    dood_with(s, board, sub, Options::default())
}

/// How [`dood_with`] works the grid out, beyond each element's own switches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Options {
    /// As if no element had holds (and so no drops): the "without holds" view other
    /// tools offer (inventory, competitor features). Off by default.
    pub no_holds: bool,
}

/// [`dood`] with options.
pub fn dood_with(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
    opts: Options,
) -> Result<Vec<Row>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let cal = &b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?
        .calendar;
    let mut all = work_dates(s, board, sub)?;
    let events = event_dates(cal);
    for (el, _, _) in &events {
        if !all.iter().any(|(e, _)| e == el) {
            all.push((*el, BTreeSet::new()));
        }
    }
    let mut rows = Vec::new();
    for (el, dates) in all {
        let mut rules = s.element(el).ok_or(Error::NoSuchElement(el))?.rules;
        if opts.no_holds {
            rules.holds = false;
            rules.drops = false;
        }
        let work: Vec<Date> = dates.into_iter().collect();
        let mine: Vec<(Date, &'static str)> = events
            .iter()
            .filter(|(e, _, _)| *e == el)
            .map(|(_, d, c)| (*d, *c))
            .collect();
        let event_at = |d: Date| mine.iter().find(|(x, _)| *x == d).map(|(_, c)| *c);
        let (start, finish) = (work.first().copied(), work.last().copied());
        let in_span = |d: Date| start.is_some_and(|a| d >= a) && finish.is_some_and(|z| d <= z);

        // The drop: the longest gap of at least the minimum between work or event days
        // inside the span, the earlier on a tie.
        let event_days: Vec<Date> = mine.iter().map(|(d, _)| *d).collect();
        let (all_gaps, chosen) = gaps(&work, &event_days, rules);
        let drop: Option<(Date, Date)> = chosen.map(|i| (all_gaps[i].after, all_gaps[i].before));
        let in_drop = |d: Date| drop.is_some_and(|(a, z)| d > a && d < z);
        // An event day bounds the drop as a work day would: it takes the D or P, after
        // its H (E-D22, behavior note 27, decision 139).
        let held_day = |d: Date| in_span(d) && rules.holds && !in_drop(d) && cal.is_shoot_day(d);

        let first = work
            .iter()
            .chain(mine.iter().map(|(d, _)| d))
            .min()
            .copied();
        let last = work
            .iter()
            .chain(mine.iter().map(|(d, _)| d))
            .max()
            .copied();
        let mut row = Row {
            element: el,
            start,
            finish,
            cells: Vec::new(),
            worked: 0,
            held: 0,
        };
        let (Some(mut d), Some(last)) = (first, last) else {
            continue;
        };
        while d <= last {
            let cell = if work.binary_search(&d).is_ok() {
                row.worked += 1;
                let (start, finish) = (Some(d) == start, Some(d) == finish);
                let drop_day = drop.is_some_and(|(a, _)| a == d);
                let pickup = drop.is_some_and(|(_, z)| z == d);
                match event_at(d) {
                    Some(code) => Cell::EventOnWork {
                        code,
                        start,
                        finish,
                        drop: drop_day,
                        pickup,
                    },
                    None => Cell::Work {
                        start,
                        finish,
                        drop: drop_day,
                        pickup,
                    },
                }
            } else if let Some(code) = event_at(d) {
                row.worked += 1;
                Cell::Event {
                    code,
                    held: held_day(d),
                    drop: drop.is_some_and(|(a, _)| a == d),
                    pickup: drop.is_some_and(|(_, z)| z == d),
                }
            } else if !in_span(d) || in_drop(d) || !rules.holds {
                Cell::Idle
            } else if cal.is_holiday(d) {
                row.worked += 1;
                Cell::Holiday
            } else if cal.is_move(d) {
                row.worked += 1;
                Cell::Move
            } else if cal.is_off(d) {
                Cell::Idle
            } else {
                row.held += 1;
                Cell::Hold
            };
            row.cells.push((d, cell));
            d = d.add_days(1);
        }
        rows.push(row);
    }
    rows.sort_by(|a, b| {
        let (x, y) = (s.element(a.element).unwrap(), s.element(b.element).unwrap());
        crate::reports::id_order(x.board_id.as_deref(), y.board_id.as_deref()).then_with(|| {
            x.name
                .to_ascii_uppercase()
                .cmp(&y.name.to_ascii_uppercase())
        })
    });
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::{auto_day_breaks, Limit};
    use crate::model::{DoodRules, Event, EventKind, Place, Sheet};
    use crate::Eighths;

    /// ZTEST run 1 (behavior note 14): 25 one-eighth sheets, one a day from Mon
    /// 2027-03-01, each listed cast member on the given days.
    struct Run {
        s: Schedule,
        b: BoardId,
        sb: SubBoardId,
    }

    fn run(cast: &[(&str, &[u32], DoodRules)], holidays: &[&str]) -> Run {
        let mut s = Schedule::new("ZTEST Lantern Creek");
        let cat = s.category_named("Cast").unwrap();
        let mut ids = Vec::new();
        for (name, _, rules) in cast {
            let id = s.add_element(cat, name).unwrap();
            s.element_mut(id).unwrap().rules = *rules;
            ids.push(id);
        }
        for n in 1..=25u32 {
            let elements = cast
                .iter()
                .zip(&ids)
                .filter(|((_, days, _), _)| days.contains(&n))
                .map(|(_, id)| *id)
                .collect();
            s.add_sheet(Sheet {
                scenes: n.to_string(),
                pages: Eighths(1),
                elements,
                ..Sheet::default()
            });
        }
        let b = s.boards[0].id;
        let sb = s.boards[0].sub_boards[0].id;
        let strips: Vec<_> = s.boards[0].boneyard.iter().map(|x| x.id).collect();
        s.move_strips(
            b,
            &strips,
            Place {
                sub_board: Some(sb),
                index: 0,
            },
        )
        .unwrap();
        auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(1))).unwrap();
        let sub = &mut s.boards[0].sub_boards[0];
        sub.start = "2027-03-01".parse().ok();
        sub.calendar.holidays = holidays.iter().map(|h| h.parse().unwrap()).collect();
        Run { s, b, sb }
    }

    /// `name worked held: code code ...` with `.` for an empty cell, from start to finish.
    fn line(r: &Run, name: &str) -> String {
        let rows = dood(&r.s, r.b, r.sb).unwrap();
        let cat = r.s.category_named("Cast").unwrap();
        let id = r.s.element_named(cat, name).unwrap();
        let row = rows.iter().find(|x| x.element == id).unwrap();
        let codes: Vec<String> = row
            .cells
            .iter()
            .map(|(_, c)| match c.code() {
                c if c.is_empty() => ".".to_string(),
                c => c,
            })
            .collect();
        format!("{name} {} {}: {}", row.worked, row.held, codes.join(" "))
    }

    fn el(r: &Run, name: &str) -> ElementId {
        let cat = r.s.category_named("Cast").unwrap();
        r.s.element_named(cat, name).unwrap()
    }

    fn cal(r: &mut Run) -> &mut crate::model::Calendar {
        &mut r.s.boards[0].sub_boards[0].calendar
    }

    fn event(kind: EventKind, on: &str, who: ElementId) -> Event {
        let d: Date = on.parse().unwrap();
        Event {
            name: String::new(),
            kind,
            from: d,
            to: d,
            elements: vec![who],
            color: None,
        }
    }

    const OFF: DoodRules = DoodRules {
        holds: false,
        drops: false,
        min_drop_gap: 1,
    };
    const HOLDS: DoodRules = DoodRules {
        holds: true,
        drops: false,
        min_drop_gap: 1,
    };
    fn drops(min: u32) -> DoodRules {
        DoodRules {
            holds: true,
            drops: true,
            min_drop_gap: min,
        }
    }

    #[test]
    fn holds_off_counts_only_work() {
        let r = run(&[("DARIO", &[1, 3], OFF)], &[]);
        assert_eq!(line(&r, "DARIO"), "DARIO 2 0: SW . WF");
    }

    #[test]
    fn single_day_and_consecutive_days() {
        let r = run(&[("EVE", &[1, 2, 3], HOLDS), ("FINN", &[5], OFF)], &[]);
        assert_eq!(line(&r, "EVE"), "EVE 3 0: SW W WF");
        assert_eq!(line(&r, "FINN"), "FINN 1 0: SWF");
    }

    #[test]
    fn holds_skip_weekends() {
        // E-D5: Thu Mar 4 and Tue Mar 9.
        let r = run(&[("BRUNO", &[4, 7], HOLDS)], &[]);
        assert_eq!(line(&r, "BRUNO"), "BRUNO 2 2: SW H . . H WF");
    }

    #[test]
    fn drop_needs_a_gap_of_at_least_the_minimum() {
        // E-D1: Mar 1 and Mar 15 have 13 days between.
        let r = run(&[("CLEO", &[1, 11], drops(13))], &[]);
        assert!(line(&r, "CLEO").starts_with("CLEO 2 0: SWD . . . . . . . . . . . . . PWF"));
        let r = run(&[("CLEO", &[1, 11], drops(14))], &[]);
        assert_eq!(
            line(&r, "CLEO"),
            "CLEO 2 9: SW H H H H . . H H H H H . . WF"
        );
    }

    #[test]
    fn one_drop_in_the_longest_gap() {
        // E-D2: two equal gaps of 13; the earlier drops, the later is held.
        let r = run(&[("ADA", &[1, 11, 21], drops(7))], &[]);
        assert_eq!(
            line(&r, "ADA"),
            "ADA 3 9: SWD . . . . . . . . . . . . . PW H H H H . . H H H H H . . WF"
        );
        // Gaps of 3, 9 and 13: the 13 drops, the 9 is held.
        let r = run(&[("ADA", &[1, 5, 11, 21], drops(7))], &[]);
        let l = line(&r, "ADA");
        assert!(
            l.starts_with("ADA 4 8: SW H H H W . . H H H H H . . WD"),
            "{l}"
        );
        assert!(l.ends_with("PWF"), "{l}");
        // Gaps of 13 then 8: the first, longer one drops.
        let r = run(&[("ADA", &[1, 11, 18], drops(7))], &[]);
        assert_eq!(
            line(&r, "ADA"),
            "ADA 3 6: SWD . . . . . . . . . . . . . PW H H H H . . H H WF"
        );
    }

    #[test]
    fn holidays() {
        // Wed Mar 3 a holiday: days reflow, Day 3 is Thu Mar 4.
        let r = run(
            &[
                ("ADA", &[1, 11, 18], drops(7)),
                ("CLEO", &[1, 11], drops(14)),
                ("DARIO", &[1, 11], HOLDS),
                ("EVE", &[1, 2, 3], HOLDS),
            ],
            &["2027-03-03"],
        );
        // Inside the span with holds on: counts as worked.
        assert_eq!(line(&r, "EVE"), "EVE 4 0: SW W / WF");
        // The summary counts it in Work, with no holiday count of its own (E-D20).
        let sum = summary(&dood(&r.s, r.b, r.sb).unwrap());
        let eve = sum.iter().find(|x| x.element == el(&r, "EVE")).unwrap();
        assert_eq!((eve.work, eve.hold, eve.total), (4, 0, 4));
        assert!(line(&r, "DARIO").starts_with("DARIO 3 9: SW H / H H"));
        // The holiday stretched CLEO's gap to 14, so it now drops (E-D3).
        assert!(line(&r, "CLEO").starts_with("CLEO 2 0: SWD"));
        // Inside the drop: counts nowhere.
        assert!(line(&r, "ADA").starts_with("ADA 3 6: SWD . ."));
        // Holds off: not counted (E-D13).
        let r = run(&[("EVE", &[1, 2, 3], OFF)], &["2027-03-03"]);
        assert_eq!(line(&r, "EVE"), "EVE 3 0: SW W . WF");
    }

    /// ZTEST run 2 (behavior note 15): the final state of the run against the exported
    /// summary, then each event rule alone.
    #[test]
    fn run_two_matches_the_exported_summary() {
        let mut r = run(
            &[
                ("ADA", &[1, 11, 18], drops(7)),
                ("BRUNO", &[4, 7], HOLDS),
                ("CLEO", &[1, 11], drops(14)),
                ("DARIO", &[1, 11], HOLDS),
                ("EVE", &[1, 2, 3], OFF),
                ("FINN", &[5], OFF),
            ],
            &["2027-03-03"],
        );
        let (bruno, cleo, finn) = (el(&r, "BRUNO"), el(&r, "CLEO"), el(&r, "FINN"));
        let c = cal(&mut r);
        c.events
            .push(event(EventKind::Fitting, "2027-03-08", bruno));
        c.events.push(event(EventKind::Travel, "2027-03-08", cleo));
        c.events.push(event(EventKind::Fitting, "2027-02-26", finn));
        c.moves.push("2027-03-17".parse().unwrap());

        let rows = dood(&r.s, r.b, r.sb).unwrap();
        let got: Vec<String> = summary(&rows)
            .iter()
            .map(|x| {
                let name = &r.s.element(x.element).unwrap().name;
                format!(
                    "{name} {} {} {} {} {}",
                    x.work,
                    x.hold,
                    x.start.unwrap(),
                    x.finish.unwrap(),
                    x.total
                )
            })
            .collect();
        assert_eq!(
            got,
            [
                "ADA 4 6 2027-03-01 2027-03-26 10",
                "BRUNO 3 1 2027-03-05 2027-03-10 4",
                "CLEO 4 8 2027-03-01 2027-03-16 12",
                "DARIO 3 9 2027-03-01 2027-03-16 12",
                "EVE 3 0 2027-03-01 2027-03-04 3",
                "FINN 2 0 2027-03-08 2027-03-08 2",
            ]
        );
        // Fitting on a hold day: shown with H, counted as worked.
        assert_eq!(line(&r, "BRUNO"), "BRUNO 3 1: SW . . FTH H WF");
        // Travel inside the gap stopped the drop; the holiday counts again.
        assert!(
            line(&r, "CLEO").starts_with("CLEO 4 8: SW H / H H . . TRH H"),
            "{}",
            line(&r, "CLEO")
        );
        // A fitting before the start: shown, counted, start unchanged.
        assert_eq!(line(&r, "FINN"), "FINN 2 0: FT . . . . . . . . . SWF");
        // Company travel day inside the held span counts as worked.
        assert!(line(&r, "ADA").contains("PW M H"), "{}", line(&r, "ADA"));
    }

    #[test]
    fn an_event_on_a_work_day_shows_first_counts_once_and_is_flagged() {
        // Behavior notes 17 (A4 and A5) and 18 (E-D6).
        let mut r = run(
            &[("ADA", &[1, 11, 12], drops(7)), ("EVE", &[1, 2, 3], HOLDS)],
            &[],
        );
        let before = (line(&r, "ADA"), line(&r, "EVE"));
        assert!(before.0.starts_with("ADA 3 0: SWD"), "{}", before.0);
        let (ada, eve) = (el(&r, "ADA"), el(&r, "EVE"));
        let c = cal(&mut r);
        // A fitting on the pickup day: the fitting code, then the whole work code.
        c.events.push(event(EventKind::Fitting, "2027-03-15", ada));
        // Travel on the first and the last day, a rehearsal in between.
        c.events.push(event(EventKind::Travel, "2027-03-01", eve));
        c.events
            .push(event(EventKind::Rehearsal, "2027-03-02", eve));
        c.events.push(event(EventKind::Travel, "2027-03-03", eve));
        let ada_line = line(&r, "ADA");
        assert!(ada_line.ends_with(". FTPW WF"), "{ada_line}");
        assert_eq!(line(&r, "EVE"), "EVE 3 0: TRSW RW TRWF");
        // Totals don't change: each day counts once, as worked.
        assert_eq!(ada_line.replace("FTPW", "PW"), before.0);
        let rows = dood(&r.s, r.b, r.sb).unwrap();
        let flagged: Vec<(ElementId, String)> = rows
            .iter()
            .flat_map(|row| {
                row.cells
                    .iter()
                    .filter(|(_, c)| c.is_flagged())
                    .map(|(d, _)| (row.element, d.to_string()))
            })
            .collect();
        assert_eq!(flagged.len(), 4);
        assert!(flagged.contains(&(ada, "2027-03-15".to_string())));
        assert!(rows
            .iter()
            .flat_map(|r| &r.cells)
            .filter(|(_, c)| c.is_flagged())
            .all(|(_, c)| c.is_work()));
        // The event's code, then the whole work code: S, P, W, D, F.
        let all = Cell::EventOnWork {
            code: "TR",
            start: true,
            finish: true,
            drop: true,
            pickup: true,
        };
        assert_eq!(all.code(), "TRSPWDF");
        let plain = Cell::EventOnWork {
            code: "FT",
            start: false,
            finish: false,
            drop: false,
            pickup: false,
        };
        assert_eq!(plain.code(), "FTW");
    }

    #[test]
    fn a_weekend_day_marked_working_is_a_shoot_day() {
        // Saturday 2027-03-06 back to working: day 6 moves onto it, and holds fill it.
        let mut r = run(&[("BRUNO", &[4, 7], HOLDS)], &[]);
        cal(&mut r).set_day_kind("2027-03-06".parse().unwrap(), crate::model::DayKind::Work);
        assert_eq!(line(&r, "BRUNO"), "BRUNO 2 2: SW H H . WF");
    }

    #[test]
    fn the_calendars_working_week_is_the_dood_week() {
        // DOD-15 (note 26): one working week. A six-day week (Sunday off) makes
        // Saturday a shoot day: day 6 lands on it and holds fill it.
        let mut r = run(&[("BRUNO", &[4, 7], HOLDS)], &[]);
        cal(&mut r).weekly_off = vec![crate::date::Weekday::Sun];
        // Thu Mar 4 to Mon Mar 8 (day 7 in a six-day week): Fri and Sat held, Sun off.
        assert_eq!(line(&r, "BRUNO"), "BRUNO 2 2: SW H H . WF");
        // A day off of its own (not a holiday): the days move on a day and it counts
        // nowhere.
        cal(&mut r).off_days = vec!["2027-03-05".parse().unwrap()];
        assert_eq!(line(&r, "BRUNO"), "BRUNO 2 2: SW . H . H WF");
    }

    #[test]
    fn spanning_scenes_two_strips_a_day_and_strips_off_the_days() {
        // ADA on sheets 1 and 3, FINN on sheet 1 only.
        let mut r = run(&[("ADA", &[1, 3], HOLDS), ("FINN", &[1], OFF)], &[]);
        let first = r.s.sheets[0].id;
        // Sheet 1 continues first thing on day 3 (decision 76): its elements work there
        // too, and ADA, on both of day 3's strips, works it once (E-D16).
        r.s.add_part(
            r.b,
            first,
            Place {
                sub_board: Some(r.sb),
                index: 4,
            },
        )
        .unwrap();
        assert_eq!(line(&r, "ADA"), "ADA 2 1: SW H WF");
        assert_eq!(line(&r, "FINN"), "FINN 2 0: SW . WF");
        // A strip after the last day break and one in the Boneyard are not scheduled
        // (E-D15): their elements get no row.
        let cat = r.s.category_named("Cast").unwrap();
        let (gus, hal) = (
            r.s.add_element(cat, "GUS").unwrap(),
            r.s.add_element(cat, "HAL").unwrap(),
        );
        for who in [gus, hal] {
            let h = r.s.add_sheet(Sheet {
                scenes: "X".into(),
                pages: Eighths(1),
                elements: vec![who],
                ..Sheet::default()
            });
            assert!(r.s.boards[0]
                .boneyard
                .iter()
                .any(|x| x.kind == crate::model::StripKind::Sheet(h)));
        }
        let parked = r.s.boards[0].boneyard[0].id;
        let end = r.s.boards[0].sub_boards[0].strips.len();
        r.s.move_strips(
            r.b,
            &[parked],
            Place {
                sub_board: Some(r.sb),
                index: end,
            },
        )
        .unwrap();
        let rows = dood(&r.s, r.b, r.sb).unwrap();
        assert!(rows.iter().all(|x| x.element != gus && x.element != hal));
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn element_with_events_only() {
        let mut r = run(&[("ADA", &[1], OFF), ("GUS", &[], OFF)], &[]);
        let gus = el(&r, "GUS");
        cal(&mut r)
            .events
            .push(event(EventKind::Fitting, "2027-03-02", gus));
        cal(&mut r).events.push(event(
            EventKind::Other("Camera test".into()),
            "2027-03-03",
            gus,
        ));
        let rows = dood(&r.s, r.b, r.sb).unwrap();
        let row = rows.iter().find(|x| x.element == gus).unwrap();
        assert_eq!((row.start, row.worked, row.cells.len()), (None, 1, 1));
    }

    #[test]
    fn post_production_events_never_count_in_the_day_out_of_days() {
        // Decision 109: every post-production kind shows on the calendar only.
        let mut r = run(&[("ADA", &[1, 3], HOLDS)], &[]);
        let ada = el(&r, "ADA");
        let plain = dood(&r.s, r.b, r.sb).unwrap();
        for (i, k) in crate::model::PostKind::ALL.into_iter().enumerate() {
            let day = ["2027-03-02", "2027-03-09", "2027-03-01"][i % 3];
            cal(&mut r).events.push(event(EventKind::Post(k), day, ada));
            assert_eq!(EventKind::Post(k).code(), None);
            assert_eq!(EventKind::from_key(k.key(), ""), Some(EventKind::Post(k)));
        }
        assert_eq!(dood(&r.s, r.b, r.sb).unwrap(), plain);
        assert_eq!(EventKind::keys().len(), 17);
        // Color correction, the reference app's last post kind (EVT-03, decision 163).
        let cc = crate::model::PostKind::ColorCorrection;
        assert_eq!(crate::model::PostKind::ALL.last(), Some(&cc));
        assert_eq!(
            EventKind::from_key("color_correction", ""),
            Some(EventKind::Post(cc))
        );
        assert_eq!(cc.label(), "Color correction");
        assert_eq!(EventKind::from_key("dance", ""), None);
    }

    #[test]
    fn unavailable_and_camera_test_neither_show_nor_count() {
        // Behavior note 17, A8: the cell stays what it was (here a hold).
        let mut r = run(&[("ADA", &[1, 3], HOLDS)], &[]);
        let ada = el(&r, "ADA");
        let plain = dood(&r.s, r.b, r.sb).unwrap();
        cal(&mut r)
            .events
            .push(event(EventKind::Unavailable, "2027-03-02", ada));
        cal(&mut r)
            .events
            .push(event(EventKind::CameraTest, "2027-03-02", ada));
        cal(&mut r)
            .events
            .push(event(EventKind::CameraTest, "2027-03-09", ada));
        assert_eq!(dood(&r.s, r.b, r.sb).unwrap(), plain);
        assert_eq!(EventKind::Unavailable.code(), None);
        assert_eq!(EventKind::CameraTest.key(), "camera_test");
        assert_eq!(
            EventKind::from_key("unavailable", ""),
            Some(EventKind::Unavailable)
        );
    }

    #[test]
    fn an_event_at_a_drops_edge_takes_the_drop_or_pickup_after_its_hold() {
        // E-D22 (behavior note 27, decision 139): a counted event acts as a work day for
        // the drop's bounds. Its code keeps the H, then D or P; the work day beside it
        // loses the letter. Days between it and the work too few for a drop are held.
        let mut r = run(&[("ADA", &[1, 11], drops(5))], &[]);
        let ada = el(&r, "ADA");
        cal(&mut r)
            .events
            .push(event(EventKind::Fitting, "2027-03-03", ada));
        assert_eq!(
            line(&r, "ADA"),
            "ADA 3 1: SW H FTHD . . . . . . . . . . . PWF"
        );
        // At the other end: travel two days before the return, the weekend between.
        let mut r = run(&[("ADA", &[1, 11], drops(5))], &[]);
        cal(&mut r)
            .events
            .push(event(EventKind::Travel, "2027-03-12", ada));
        assert_eq!(
            line(&r, "ADA"),
            "ADA 3 0: SWD . . . . . . . . . . TRHP . . WF"
        );
    }

    #[test]
    fn travel_beside_a_drop_moves_its_drop_or_pickup_as_the_reference_app() {
        // Behavior note 27: ADA, drops at 7, works day 1 (March 1) and returns March 16.
        let days: &[u32] = &[1, 12, 13];
        assert_eq!(
            line(&run(&[("ADA", days, drops(7))], &[]), "ADA"),
            "ADA 3 0: SWD . . . . . . . . . . . . . . PW WF"
        );
        // Travel the day after the last work day: the drop starts after the event.
        let mut r = run(&[("ADA", days, drops(7))], &[]);
        let ada = el(&r, "ADA");
        cal(&mut r)
            .events
            .push(event(EventKind::Travel, "2027-03-02", ada));
        assert_eq!(
            line(&r, "ADA"),
            "ADA 4 0: SW TRHD . . . . . . . . . . . . . PW WF"
        );
        // Travel the day before the return: the event day is the pickup.
        let mut r = run(&[("ADA", days, drops(7))], &[]);
        cal(&mut r)
            .events
            .push(event(EventKind::Travel, "2027-03-15", ada));
        assert_eq!(
            line(&r, "ADA"),
            "ADA 4 0: SWD . . . . . . . . . . . . . TRHP W WF"
        );
        // Both: each event day counts as worked.
        cal(&mut r)
            .events
            .push(event(EventKind::Travel, "2027-03-02", ada));
        assert_eq!(
            line(&r, "ADA"),
            "ADA 5 0: SW TRHD . . . . . . . . . . . . TRHP W WF"
        );
    }

    #[test]
    fn rows_group_by_category_keeping_their_order() {
        let mut r = run(&[("ADA", &[1], OFF), ("BEN", &[2], OFF)], &[]);
        let cast = r.s.category_named("Cast").unwrap();
        let props = r.s.add_category("Props");
        let lamp = r.s.add_element(props, "LAMP").unwrap();
        let first = r.s.sheets[0].id;
        r.s.tag(first, lamp).unwrap();
        for (e, id) in [(el(&r, "ADA"), "1"), (lamp, "1"), (el(&r, "BEN"), "2")] {
            r.s.set_board_id(e, Some(id)).unwrap();
        }
        let mut rows = dood(&r.s, r.b, r.sb).unwrap();
        let names = |rows: &[Row]| -> Vec<String> {
            rows.iter()
                .map(|x| r.s.element(x.element).unwrap().name.clone())
                .collect()
        };
        // The engine's order mixes categories by board ID.
        assert_eq!(names(&rows), ["ADA", "LAMP", "BEN"]);
        group_by_category(&r.s, &mut rows);
        assert_eq!(names(&rows), ["ADA", "BEN", "LAMP"]);
        assert_eq!(r.s.element(rows[0].element).unwrap().category, cast);
    }

    #[test]
    fn needs_a_start_date() {
        let s = Schedule::new("ZTEST");
        let (b, sb) = (s.boards[0].id, s.boards[0].sub_boards[0].id);
        assert!(dood(&s, b, sb).is_err());
    }

    #[test]
    fn without_holds_counts_work_and_events_only() {
        let mut r = run(
            &[("BRUNO", &[4, 7], HOLDS), ("CLEO", &[1, 11], drops(1))],
            &[],
        );
        let bruno = el(&r, "BRUNO");
        cal(&mut r)
            .events
            .push(event(EventKind::Fitting, "2027-03-05", bruno));
        let rows = dood_with(&r.s, r.b, r.sb, Options { no_holds: true }).unwrap();
        let b = rows.iter().find(|x| x.element == bruno).unwrap();
        let codes: Vec<String> = b.cells.iter().map(|(_, c)| c.code()).collect();
        assert_eq!(codes, ["SW", "FT", "", "", "", "WF"]);
        assert_eq!((b.worked, b.held), (3, 0));
        let cleo = rows.iter().find(|x| x.element == el(&r, "CLEO")).unwrap();
        assert_eq!(cleo.held, 0);
        assert!(cleo.cells.iter().all(|(_, c)| !c.code().contains('D')));
        // The default is the element's own switches.
        let with = dood(&r.s, r.b, r.sb).unwrap();
        assert_eq!(with.iter().find(|x| x.element == bruno).unwrap().held, 1);
    }

    #[test]
    fn custom_letters_replace_every_code() {
        let mut r = run(
            &[("CLEO", &[1, 11], drops(1)), ("BRUNO", &[4, 7], HOLDS)],
            &["2027-03-05"],
        );
        let bruno = el(&r, "BRUNO");
        cal(&mut r)
            .events
            .push(event(EventKind::Travel, "2027-03-04", bruno));
        let mut c = crate::setup::DoodCodes::default();
        let rows = dood(&r.s, r.b, r.sb).unwrap();
        // The default letters give the same codes as code().
        for row in &rows {
            for (_, cell) in &row.cells {
                assert_eq!(cell.code_with(&c), cell.code());
            }
        }
        c.start = "B".into();
        c.work = "X".into();
        c.finish = "E".into();
        c.hold = "O".into();
        c.travel = "T".into();
        c.holiday = "HL".into();
        let b = rows.iter().find(|x| x.element == bruno).unwrap();
        let codes: Vec<String> = b.cells.iter().map(|(_, x)| x.code_with(&c)).collect();
        assert_eq!(codes, ["TBX", "HL", "", "", "O", "O", "XE"]);
    }

    #[test]
    fn sort_keys_take_the_reference_apps_words() {
        assert_eq!(SortKey::from_name("first_work"), Some(SortKey::Start));
        assert_eq!(SortKey::from_name("last_work"), Some(SortKey::Finish));
        assert_eq!(SortKey::from_name("worked"), Some(SortKey::Work));
        assert_eq!(SortKey::from_name("held"), Some(SortKey::Hold));
        assert_eq!(SortKey::from_name("start"), Some(SortKey::Start));
        assert_eq!(SortKey::from_name("scene"), None);
        assert_eq!(SortKey::Start.label(), "First working day");
    }

    #[test]
    fn rows_sort_by_several_keys_stably_with_blanks_last() {
        let mut r = run(
            &[
                ("ADA", &[3], OFF),
                ("BEN", &[1, 2], OFF),
                ("CAL", &[1], OFF),
                ("DOT", &[2, 3], OFF),
            ],
            &[],
        );
        let cat = r.s.category_named("Cast").unwrap();
        for (name, id) in [("BEN", "2"), ("CAL", "1")] {
            let e = r.s.element_named(cat, name).unwrap();
            r.s.set_board_id(e, Some(id)).unwrap();
        }
        let names = |rows: &[Row]| {
            rows.iter()
                .map(|x| r.s.element(x.element).unwrap().name.clone())
                .collect::<Vec<_>>()
        };
        let mut rows = dood(&r.s, r.b, r.sb).unwrap();
        assert_eq!(names(&rows), ["CAL", "BEN", "ADA", "DOT"]);
        // Board ID descending: the IDs reversed, no ID still last, in their order.
        sort_rows(&r.s, &mut rows, &[(SortKey::BoardId, true)]);
        assert_eq!(names(&rows), ["BEN", "CAL", "ADA", "DOT"]);
        // Work days descending, then name.
        sort_rows(
            &r.s,
            &mut rows,
            &[(SortKey::Work, true), (SortKey::Name, false)],
        );
        assert_eq!(names(&rows), ["BEN", "DOT", "ADA", "CAL"]);
        // Start ascending; ties keep the order they had.
        sort_rows(&r.s, &mut rows, &[(SortKey::Start, false)]);
        assert_eq!(names(&rows), ["BEN", "CAL", "DOT", "ADA"]);
        sort_rows(
            &r.s,
            &mut rows,
            &[(SortKey::Finish, true), (SortKey::Name, true)],
        );
        assert_eq!(names(&rows), ["DOT", "ADA", "BEN", "CAL"]);
        assert_eq!(SortKey::from_name("hold"), Some(SortKey::Hold));
        assert_eq!(SortKey::from_name("scene"), None);
    }
}
