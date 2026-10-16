//! "Explain this count": why one element's day out of days totals on one sub-board are
//! what they are, date by date (roadmap phase 3; decisions.md, entry 25). After Method
//! Budget's "explain this number".
//!
//! The explanation is worked out from the schedule with the engine's own pieces
//! ([`crate::dood::dood`] for the row and [`crate::dood::gaps`] for the drop), not copied
//! from a report, and a test holds every code and count to the day out of days. The rules
//! are decisions 10 to 12:
//! - Work comes from the shoot days whose sheets list the element.
//! - With holds on, every shoot day strictly between the first and last work day is held.
//! - With holds and drops on, the one drop is the longest gap of at least the minimum
//!   (calendar days strictly between two work or event days), the earlier on a tie.
//! - Travel, fitting and rehearsal events count as worked wherever they fall; on a day
//!   that would be held they still count as worked. Other events count nowhere.
//! - A holiday or company travel day counts as worked only with holds on, inside the work
//!   span and outside the drop.

use std::collections::BTreeMap;

use crate::diff::{days_text, event_text, on_off, quoted, sheet_name};
use crate::dood::{dood, gaps, Cell, Gap};
use crate::model::{BoardId, DoodRules, ElementId, Error, Schedule, SheetId, SubBoardId};
use crate::{Date, Weekday};

/// Where one date's day goes in the totals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Counted {
    Work,
    Hold,
    Nowhere,
}

impl Counted {
    pub fn key(self) -> &'static str {
        match self {
            Counted::Work => "work",
            Counted::Hold => "hold",
            Counted::Nowhere => "nowhere",
        }
    }
}

/// Why a date counts the way it does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    /// The element works: its sheets are on this shoot day.
    Work,
    /// A travel, fitting or rehearsal event: counts as worked wherever it falls.
    Event,
    /// A counted event on a day that would otherwise be held: worked, not held.
    EventOnHold,
    /// A counted event on a work day: counted once, as work, and a conflict.
    EventOnWork,
    /// A shoot day between the first and last work day, holds on, outside the drop.
    Hold,
    /// A holiday inside the work span, holds on, outside the drop: counts as worked.
    Holiday,
    /// A company travel day inside the work span, holds on, outside the drop.
    Move,
    /// Inside the work span, but holds are off.
    HoldsOff,
    /// Inside the drop.
    InDrop,
    /// A weekly day off or a single day off.
    DayOff,
    /// Outside the work span (between an event and the span).
    OutsideSpan,
}

impl Reason {
    pub fn key(self) -> &'static str {
        match self {
            Reason::Work => "work",
            Reason::Event => "event",
            Reason::EventOnHold => "event_on_hold",
            Reason::EventOnWork => "event_on_work",
            Reason::Hold => "hold",
            Reason::Holiday => "holiday",
            Reason::Move => "move",
            Reason::HoldsOff => "holds_off",
            Reason::InDrop => "in_drop",
            Reason::DayOff => "day_off",
            Reason::OutsideSpan => "outside_span",
        }
    }
}

/// One date of the element's row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DayNote {
    pub date: Date,
    /// The day out of days code (`SW`, `H`, `FTH`, `FTPW`, `/`, `M`, or empty).
    pub code: String,
    pub counted: Counted,
    pub reason: Reason,
    /// The shoot day number, when the date is a shoot day.
    pub shoot_day: Option<u32>,
    /// The element's sheets on this shoot day.
    pub sheets: Vec<SheetId>,
    /// Every event on this date that lists the element, counted or not, in words.
    pub events: Vec<String>,
    /// The date in one plain sentence.
    pub text: String,
}

/// What became of one gap.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapOutcome {
    /// The one drop.
    Dropped,
    /// Held because it is shorter than the minimum.
    TooShort,
    /// Long enough, but another gap took the one drop.
    Longer,
    /// Held: drops are off.
    DropsOff,
    /// Counts nowhere: holds are off.
    HoldsOff,
}

impl GapOutcome {
    pub fn key(self) -> &'static str {
        match self {
            GapOutcome::Dropped => "dropped",
            GapOutcome::TooShort => "too_short",
            GapOutcome::Longer => "another_gap_dropped",
            GapOutcome::DropsOff => "drops_off",
            GapOutcome::HoldsOff => "holds_off",
        }
    }
}

/// One gap between work or event days inside the span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GapNote {
    pub gap: Gap,
    pub outcome: GapOutcome,
    pub text: String,
}

/// Why an element's counts on a sub-board are what they are.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Explanation {
    pub element: ElementId,
    pub board: BoardId,
    pub sub_board: SubBoardId,
    pub rules: DoodRules,
    pub start: Option<Date>,
    pub finish: Option<Date>,
    /// The summary's Work: every day that counts as worked.
    pub work: u32,
    pub hold: u32,
    /// Work plus hold.
    pub total: u32,
    /// What Work is made of.
    pub work_days: u32,
    pub event_days: u32,
    pub holidays: u32,
    pub moves: u32,
    /// Every date of the row, first to last.
    pub days: Vec<DayNote>,
    pub gaps: Vec<GapNote>,
    pub drop: Option<Gap>,
    /// The totals, the rules and the drop in plain sentences.
    pub summary: Vec<String>,
}

impl Explanation {
    /// The summary, then each gap, then each date, as text for a terminal.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for s in &self.summary {
            out.push_str(s);
            out.push('\n');
        }
        if !self.gaps.is_empty() {
            out.push_str("\nGaps\n");
            for g in &self.gaps {
                out.push_str(&format!("  {}\n", g.text));
            }
        }
        if !self.days.is_empty() {
            out.push_str("\nDay by day\n");
            for d in &self.days {
                let code = if d.code.is_empty() { "." } else { &d.code };
                out.push_str(&format!("  {code:<4} {}\n", d.text));
            }
        }
        out
    }
}

fn weekday(d: Date) -> &'static str {
    match d.weekday() {
        Weekday::Mon => "Mon",
        Weekday::Tue => "Tue",
        Weekday::Wed => "Wed",
        Weekday::Thu => "Thu",
        Weekday::Fri => "Fri",
        Weekday::Sat => "Sat",
        Weekday::Sun => "Sun",
    }
}

fn gap_words(g: &Gap) -> String {
    format!("{} between {} and {}", days_text(g.days), g.after, g.before)
}

/// Explains `element`'s day out of days on `board`'s sub-board `sub`. The sub-board
/// needs a start date, as the day out of days does.
pub fn explain(
    s: &Schedule,
    board: BoardId,
    sub: SubBoardId,
    element: ElementId,
) -> Result<Explanation, Error> {
    let el = s.element(element).ok_or(Error::NoSuchElement(element))?;
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let sb = b
        .sub_boards
        .iter()
        .find(|x| x.id == sub)
        .ok_or(Error::NoSuchSubBoard(sub))?;
    let cal = &sb.calendar;
    let rules = el.rules;
    let rows = dood(s, board, sub)?;
    let row = rows.iter().find(|r| r.element == element);
    let start_date = sb
        .start
        .ok_or_else(|| Error::Broken(format!("sub-board {:?} has no start date", sb.name)))?;

    // Shoot day numbers by date, and the element's sheets on each.
    let mut shoot_days: BTreeMap<Date, (u32, Vec<SheetId>)> = BTreeMap::new();
    let index = s.sheet_index();
    for day in crate::days::days_with(s, &index, board, sub)?.days {
        let Some(date) = cal.date_of_day(start_date, day.number) else {
            break;
        };
        // An omitted scene is no work (decision 91), as in the day out of days.
        let mine = day
            .counted_sheets()
            .filter(|sh| index.get(sh).is_some_and(|x| x.elements.contains(&element)))
            .collect();
        shoot_days.insert(date, (day.number, mine));
    }
    let work: Vec<Date> = shoot_days
        .iter()
        .filter(|(_, (_, sheets))| !sheets.is_empty())
        .map(|(d, _)| *d)
        .collect();
    // Events that list the element, by date.
    let name_of = |id: ElementId| s.element(id).map_or(id.0.to_string(), |e| e.name.clone());
    let mut events: BTreeMap<Date, Vec<(bool, String)>> = BTreeMap::new();
    let mut counted_event_days = Vec::new();
    for ev in cal.events.iter().filter(|e| e.elements.contains(&element)) {
        let counts = ev.kind.code().is_some();
        let words = event_text(ev, name_of);
        let mut d = ev.from;
        while d <= ev.to {
            events.entry(d).or_default().push((counts, words.clone()));
            if counts {
                counted_event_days.push(d);
            }
            d = d.add_days(1);
        }
    }
    let (all_gaps, chosen) = gaps(&work, &counted_event_days, rules);
    let drop = chosen.map(|i| all_gaps[i]);
    let (start, finish) = (work.first().copied(), work.last().copied());
    let in_span = |d: Date| start.is_some_and(|a| d >= a) && finish.is_some_and(|z| d <= z);
    let in_drop = |d: Date| drop.is_some_and(|g| d > g.after && d < g.before);

    let mut out = Explanation {
        element,
        board,
        sub_board: sub,
        rules,
        start,
        finish,
        work: 0,
        hold: 0,
        total: 0,
        work_days: 0,
        event_days: 0,
        holidays: 0,
        moves: 0,
        days: Vec::new(),
        gaps: Vec::new(),
        drop,
        summary: Vec::new(),
    };
    let drop_words = drop.map_or(String::new(), |g| {
        format!("the drop between {} and {}", g.after, g.before)
    });
    for &(date, cell) in row.map_or(&[][..], |r| &r.cells[..]) {
        let (shoot_day, sheets) = shoot_days
            .get(&date)
            .cloned()
            .map_or((None, Vec::new()), |(n, sh)| (Some(n), sh));
        let evs = events.get(&date).cloned().unwrap_or_default();
        let counted_ev: Vec<&String> = evs.iter().filter(|e| e.0).map(|e| &e.1).collect();
        let other_ev: Vec<&String> = evs.iter().filter(|e| !e.0).map(|e| &e.1).collect();
        let when = match shoot_day {
            Some(n) => format!("{} {date}, shoot day {n}", weekday(date)),
            None => format!("{} {date}", weekday(date)),
        };
        let what_day = if cal.is_move(date) {
            "a company travel day"
        } else if cal.is_holiday(date) {
            "a holiday"
        } else if cal.is_off(date) {
            "a day off"
        } else {
            "a shoot day"
        };
        let (counted, reason, mut text) = match cell {
            Cell::Work {
                start: s0,
                finish: f0,
                drop: d0,
                pickup: p0,
            }
            | Cell::EventOnWork {
                start: s0,
                finish: f0,
                drop: d0,
                pickup: p0,
                ..
            } => {
                out.work_days += 1;
                let scenes: Vec<String> = sheets
                    .iter()
                    .filter_map(|id| s.sheet(*id).map(sheet_name))
                    .collect();
                let mut marks = Vec::new();
                if s0 {
                    marks.push("the first work day (S)".to_string());
                }
                if p0 {
                    marks.push("the pick-up after the drop (P)".to_string());
                }
                if d0 {
                    marks.push("the last work day before the drop (D)".to_string());
                }
                if f0 {
                    marks.push("the last work day (F)".to_string());
                }
                let mut t = format!("{when}: works ({})", scenes.join(", "));
                if !marks.is_empty() {
                    t.push_str(&format!("; {}", marks.join(", ")));
                }
                if counted_ev.is_empty() {
                    (Counted::Work, Reason::Work, t)
                } else {
                    t.push_str(&format!(
                        "; the {} that day shows its code first and counts once, as this work \
                         day; an event on a work day is a conflict",
                        counted_ev
                            .iter()
                            .map(|x| x.as_str())
                            .collect::<Vec<_>>()
                            .join(" and ")
                    ));
                    (Counted::Work, Reason::EventOnWork, t)
                }
            }
            Cell::Event {
                held, drop, pickup, ..
            } => {
                out.event_days += 1;
                let ev = counted_ev
                    .first()
                    .map_or("an event".to_string(), |x| x.to_string());
                if drop || pickup {
                    let edge = if drop {
                        "the last day before the drop (D)"
                    } else {
                        "the pick-up after the drop (P)"
                    };
                    (
                        Counted::Work,
                        Reason::Event,
                        format!(
                            "{when}: {ev} counts as worked and bounds the drop as a work day would: {edge}"
                        ),
                    )
                } else if held {
                    (
                        Counted::Work,
                        Reason::EventOnHold,
                        format!(
                            "{when}: {ev} counts as worked; the day would be held, so it shows with H but counts as worked, not held"
                        ),
                    )
                } else if in_span(date) {
                    (
                        Counted::Work,
                        Reason::Event,
                        format!("{when}: {ev} counts as worked"),
                    )
                } else {
                    let side = if start.is_some_and(|a| date < a) {
                        "before the first work day"
                    } else if finish.is_some_and(|z| date > z) {
                        "after the last work day"
                    } else {
                        "with no work days"
                    };
                    (
                        Counted::Work,
                        Reason::Event,
                        format!(
                            "{when}: {ev} counts as worked; {side}, as events count wherever they fall (they don't move the start or finish)"
                        ),
                    )
                }
            }
            Cell::Hold => (
                Counted::Hold,
                Reason::Hold,
                format!("{when}: held, a shoot day between work days with holds on"),
            ),
            Cell::Holiday => {
                out.holidays += 1;
                (
                    Counted::Work,
                    Reason::Holiday,
                    format!(
                        "{when}: a holiday inside the work span, holds on and outside any drop, so it counts as worked"
                    ),
                )
            }
            Cell::Move => {
                out.moves += 1;
                (
                    Counted::Work,
                    Reason::Move,
                    format!(
                        "{when}: a company travel day inside the work span, holds on and outside any drop, so it counts as worked"
                    ),
                )
            }
            Cell::Idle => {
                if !in_span(date) {
                    (
                        Counted::Nowhere,
                        Reason::OutsideSpan,
                        format!("{when}: outside the work span, counts nowhere"),
                    )
                } else if in_drop(date) {
                    (
                        Counted::Nowhere,
                        Reason::InDrop,
                        format!("{when}: {what_day} inside {drop_words}, counts nowhere"),
                    )
                } else if !rules.holds {
                    let verb = if what_day == "a shoot day" {
                        "not held"
                    } else {
                        "not counted"
                    };
                    (
                        Counted::Nowhere,
                        Reason::HoldsOff,
                        format!("{when}: {what_day} inside the work span, {verb}: holds are off"),
                    )
                } else {
                    (
                        Counted::Nowhere,
                        Reason::DayOff,
                        format!("{when}: a day off, counts nowhere"),
                    )
                }
            }
        };
        if !other_ev.is_empty() {
            text.push_str(&format!(
                "; {} counts nowhere (only travel, fittings and rehearsals count)",
                other_ev
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .join(" and ")
            ));
        }
        match counted {
            Counted::Work => out.work += 1,
            Counted::Hold => out.hold += 1,
            Counted::Nowhere => {}
        }
        out.days.push(DayNote {
            date,
            code: cell.code(),
            counted,
            reason,
            shoot_day,
            sheets,
            events: evs.into_iter().map(|e| e.1).collect(),
            text,
        });
    }
    out.total = out.work + out.hold;

    // The gaps.
    let min = rules.min_drop_gap.max(1);
    for (i, g) in all_gaps.iter().enumerate() {
        let (outcome, why) = if !rules.holds {
            (
                GapOutcome::HoldsOff,
                "counts nowhere: holds are off".to_string(),
            )
        } else if !rules.drops {
            (GapOutcome::DropsOff, "held: drops are off".to_string())
        } else if Some(i) == chosen {
            let ties = all_gaps.iter().filter(|x| x.days == g.days).count();
            let tie = if ties > 1 {
                format!(", the earliest of {ties} gaps that long")
            } else {
                String::new()
            };
            (
                GapOutcome::Dropped,
                format!(
                    "dropped: the longest gap of at least {}{tie}",
                    days_text(min)
                ),
            )
        } else if g.days < min {
            (
                GapOutcome::TooShort,
                format!(
                    "held: shorter than the minimum drop gap of {}",
                    days_text(min)
                ),
            )
        } else {
            let d = drop.expect("a long enough gap means a drop");
            let why = if d.days == g.days {
                format!(
                    "held: an earlier gap as long ({}) took the one drop",
                    gap_words(&d)
                )
            } else {
                format!("held: a longer gap ({}) took the one drop", gap_words(&d))
            };
            (GapOutcome::Longer, why)
        };
        out.gaps.push(GapNote {
            gap: *g,
            outcome,
            text: format!("{}: {why}", upper(&gap_words(g))),
        });
    }

    // The summary.
    let cat = s
        .category(el.category)
        .map_or(String::new(), |c| c.name.clone());
    let mut sm = Vec::new();
    sm.push(format!(
        "{} ({cat}) on {} (board {}): work {} + hold {} = total {}{}.",
        quoted(&el.name),
        quoted(&sb.name),
        quoted(&b.name),
        out.work,
        out.hold,
        out.total,
        match (start, finish) {
            (Some(a), Some(z)) => format!(", start {a}, finish {z}"),
            _ => ", no work days".to_string(),
        }
    ));
    if row.is_none() {
        sm.push(format!(
            "{} has no work days and no events on this sub-board, so every count is 0.",
            quoted(&el.name)
        ));
    } else {
        let mut parts = vec![format!(
            "{} worked",
            plural(out.work_days, "shoot day", "shoot days")
        )];
        if out.event_days > 0 {
            parts.push(format!(
                "{} counted as worked",
                plural(out.event_days, "event day", "event days")
            ));
        }
        if out.holidays > 0 {
            parts.push(format!(
                "{} counted as worked",
                plural(out.holidays, "holiday", "holidays")
            ));
        }
        if out.moves > 0 {
            parts.push(format!(
                "{} counted as worked",
                plural(out.moves, "company travel day", "company travel days")
            ));
        }
        sm.push(format!("Work {}: {}.", out.work, parts.join(", ")));
        sm.push(format!(
            "Hold {}: {}.",
            out.hold,
            if rules.holds {
                "shoot days between the first and last work day that are not work, event or drop days"
            } else {
                "holds are off, so nothing is held"
            }
        ));
    }
    sm.push(format!(
        "Rules: holds {}, drops {}, minimum drop gap {}.",
        on_off(rules.holds),
        on_off(rules.drops),
        days_text(rules.min_drop_gap)
    ));
    sm.push(match (rules.holds, rules.drops, drop) {
        (false, _, _) => "No holds and no drop: holds are off.".to_string(),
        (true, false, _) => "No drop: drops are off, so every gap is held.".to_string(),
        (true, true, Some(g)) => format!(
            "The drop: {}, the longest gap of at least {}; the days inside count nowhere.",
            gap_words(&g),
            days_text(min)
        ),
        (true, true, None) => format!(
            "No drop: no gap is {} or longer, so every gap is held.",
            days_text(min)
        ),
    });
    if let Some(r) = row {
        debug_assert_eq!((out.work, out.hold), (r.worked, r.held));
    }
    out.summary = sm;
    Ok(out)
}

fn plural(n: u32, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

fn upper(s: &str) -> String {
    crate::diff::upper_first(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::{auto_day_breaks, Limit};
    use crate::model::{Event, EventKind, Place, Sheet};
    use crate::testgen::Rng;
    use crate::Eighths;

    /// 25 one-eighth sheets, one a day from Mon 2027-03-01 (the dood tests' ZTEST run 1).
    fn run(days_worked: &[u32], rules: DoodRules, holidays: &[&str]) -> (Schedule, ElementId) {
        let mut s = Schedule::new("ZTEST Lantern Creek");
        let cat = s.category_named("Cast").unwrap();
        let ada = s.add_element(cat, "ADA").unwrap();
        s.element_mut(ada).unwrap().rules = rules;
        for n in 1..=25u32 {
            s.add_sheet(Sheet {
                scenes: n.to_string(),
                pages: Eighths(1),
                elements: if days_worked.contains(&n) {
                    vec![ada]
                } else {
                    vec![]
                },
                ..Sheet::default()
            });
        }
        let (b, sb) = (s.boards[0].id, s.boards[0].sub_boards[0].id);
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
        (s, ada)
    }

    fn ex(s: &Schedule, e: ElementId) -> Explanation {
        explain(s, s.boards[0].id, s.boards[0].sub_boards[0].id, e).unwrap()
    }

    const DROPS7: DoodRules = DoodRules {
        holds: true,
        drops: true,
        min_drop_gap: 7,
    };

    #[test]
    fn explains_work_holds_and_the_one_drop() {
        // Days 1, 5, 11 and 21 with a holiday on Wed 2027-03-03: gaps of 6, 7 and 13
        // calendar days. The 13 drops; the 7 is long enough but held.
        let (s, ada) = run(&[1, 5, 11, 21], DROPS7, &["2027-03-03"]);
        let x = ex(&s, ada);
        assert_eq!(x.summary[0], "\"ADA\" (Cast) on \"Main\" (board \"Board 1\"): work 5 + hold 8 = total 13, start 2027-03-01, finish 2027-03-30.");
        assert_eq!(
            x.summary[1],
            "Work 5: 4 shoot days worked, 1 holiday counted as worked."
        );
        assert_eq!(
            x.summary[4],
            "The drop: 13 days between 2027-03-16 and 2027-03-30, the longest gap of at least 7 days; the days inside count nowhere."
        );
        let g: Vec<&str> = x.gaps.iter().map(|g| g.text.as_str()).collect();
        assert_eq!(
            g,
            [
                "6 days between 2027-03-01 and 2027-03-08: held: shorter than the minimum drop gap of 7 days",
                "7 days between 2027-03-08 and 2027-03-16: held: a longer gap (13 days between 2027-03-16 and 2027-03-30) took the one drop",
                "13 days between 2027-03-16 and 2027-03-30: dropped: the longest gap of at least 7 days",
            ]
        );
        let day = |d: &str| x.days.iter().find(|n| n.date.to_string() == d).unwrap();
        assert_eq!(
            day("2027-03-01").text,
            "Mon 2027-03-01, shoot day 1: works (scene 1); the first work day (S)"
        );
        assert_eq!(day("2027-03-03").reason, Reason::Holiday);
        assert_eq!(day("2027-03-02").counted, Counted::Hold);
        assert_eq!(
            day("2027-03-06").text,
            "Sat 2027-03-06: a day off, counts nowhere"
        );
        assert_eq!(day("2027-03-17").reason, Reason::InDrop);
        assert_eq!(
            day("2027-03-20").text,
            "Sat 2027-03-20: a day off inside the drop between 2027-03-16 and 2027-03-30, counts nowhere"
        );
        assert_eq!(day("2027-03-30").code, "PWF");
        assert!(!x.to_text().contains('\u{2014}'));
    }

    #[test]
    fn explains_events_and_rules_that_are_off() {
        let (mut s, ada) = run(&[4, 7], DoodRules::default(), &[]);
        let d = |x: &str| x.parse::<Date>().unwrap();
        let cal = &mut s.boards[0].sub_boards[0].calendar;
        cal.events.push(Event {
            name: "Wardrobe".into(),
            kind: EventKind::Fitting,
            from: d("2027-03-01"),
            to: d("2027-03-01"),
            elements: vec![ada],
            color: None,
        });
        cal.events.push(Event {
            name: String::new(),
            kind: EventKind::Other("Camera test".into()),
            from: d("2027-03-05"),
            to: d("2027-03-05"),
            elements: vec![ada],
            color: None,
        });
        cal.moves.push(d("2027-03-08"));
        let x = ex(&s, ada);
        assert_eq!((x.work, x.hold, x.event_days), (3, 0, 1));
        let day = |dd: &str| x.days.iter().find(|n| n.date.to_string() == dd).unwrap();
        assert!(day("2027-03-01").text.contains("fitting \"Wardrobe\" on 2027-03-01 (ADA) counts as worked; before the first work day"), "{}", day("2027-03-01").text);
        assert_eq!(day("2027-03-05").reason, Reason::HoldsOff);
        assert!(
            day("2027-03-05")
                .text
                .contains("other event \"Camera test\" on 2027-03-05 (ADA) counts nowhere"),
            "{}",
            day("2027-03-05").text
        );
        assert!(day("2027-03-08")
            .text
            .contains("a company travel day inside the work span, not counted: holds are off"));
        assert_eq!(x.summary[4], "No holds and no drop: holds are off.");
        assert!(x.gaps.iter().all(|g| g.outcome == GapOutcome::HoldsOff));
    }

    /// Every date's code and every count agree with the day out of days, on random
    /// schedules with random rules.
    #[test]
    fn property_agrees_with_the_day_out_of_days() {
        let mut rng = Rng::new(1010);
        for case in 0..200 {
            let mut s = crate::merge::tests::random_schedule(&mut rng);
            for _ in 0..rng.below(6) {
                crate::merge::tests::random_edit(&mut rng, &mut s);
            }
            for e in &mut s.elements {
                e.rules = DoodRules {
                    holds: rng.below(3) != 0,
                    drops: rng.below(2) == 0,
                    min_drop_gap: 1 + rng.below(6) as u32,
                };
            }
            let (b, sb) = (s.boards[0].id, s.boards[0].sub_boards[0].id);
            let Ok(rows) = dood(&s, b, sb) else { continue };
            for e in &s.elements {
                let x = explain(&s, b, sb, e.id).unwrap();
                match rows.iter().find(|r| r.element == e.id) {
                    Some(r) => {
                        assert_eq!((x.work, x.hold), (r.worked, r.held), "case {case}");
                        let codes: Vec<String> = r.cells.iter().map(|c| c.1.code()).collect();
                        let mine: Vec<String> = x.days.iter().map(|d| d.code.clone()).collect();
                        assert_eq!(codes, mine, "case {case}");
                        assert_eq!(x.work, x.work_days + x.event_days + x.holidays + x.moves);
                        assert_eq!((x.start, x.finish), (r.start, r.finish));
                    }
                    None => assert_eq!(x.total, 0),
                }
                for d in &x.days {
                    assert_eq!(
                        d.counted == Counted::Hold,
                        d.code == "H",
                        "case {case}: {}",
                        d.text
                    );
                }
            }
        }
    }
}
