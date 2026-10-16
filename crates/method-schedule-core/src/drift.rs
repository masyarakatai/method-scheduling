//! Schedule locks and drift per actor (decisions.md, BYD-05).
//!
//! A lock is a named record, kept in the schedule, of where every scheduled scene of a
//! board was (sub-board, shoot day and date) and which cast it had. Drift compares the
//! board now with a lock, actor by actor: work days added, dropped, and scenes that moved
//! to another day. It answers "what did we promise each actor, and what changed since"
//! without keeping a second copy of the schedule; What Changed ([`crate::diff`]) still
//! compares whole files, and the lock's sheet, sub-board and element ids are the same
//! stable ids the diff matches by.
//!
//! A work day is a date when the sub-board has a start date, else the sub-board and day
//! number. A scene counts for an actor when the actor is on the sheet (now, or in the
//! lock). Strips below the last day break and in the Boneyard are not scheduled.

use std::collections::{BTreeMap, BTreeSet};

use crate::days::days_with;
use crate::model::{same_name, BoardId, ElementId, Error, Schedule, SheetId, SubBoardId};
use crate::plan::{Lock, LockedScene};
use crate::Date;

/// Where a scene is shot: a date, or a sub-board's day number when it has no dates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkDay {
    Date(Date),
    Day(SubBoardId, u32),
}

impl WorkDay {
    pub fn text(&self, s: &Schedule) -> String {
        match *self {
            WorkDay::Date(d) => d.to_string(),
            WorkDay::Day(sb, n) => {
                let name = s
                    .boards
                    .iter()
                    .flat_map(|b| &b.sub_boards)
                    .find(|x| x.id == sb)
                    .map_or("a deleted sub-board", |x| x.name.as_str());
                format!("{name} day {n}")
            }
        }
    }
}

fn work_day(sc: &LockedScene) -> WorkDay {
    match sc.date {
        Some(d) => WorkDay::Date(d),
        None => WorkDay::Day(sc.sub_board, sc.day),
    }
}

/// Where every scheduled scene of `board` is now, with its cast.
pub fn snapshot(s: &Schedule, board: BoardId) -> Result<Vec<LockedScene>, Error> {
    let b = s.board(board).ok_or(Error::NoSuchBoard(board))?;
    let cast = s.cast();
    let (sheets, elements) = (s.sheet_index(), s.element_index());
    let mut out = Vec::new();
    for sb in &b.sub_boards {
        for d in days_with(s, &sheets, board, sb.id)?.days {
            let date = sb
                .start
                .and_then(|st| sb.calendar.date_of_day(st, d.number));
            // Each part of a scene spanning days is a scene on its day (decision 76); an
            // omitted scene is none (decision 91).
            for sheet in d.counted_sheets() {
                let Some(h) = sheets.get(&sheet) else {
                    continue;
                };
                out.push(LockedScene {
                    sheet,
                    sub_board: sb.id,
                    day: d.number,
                    date,
                    cast: h
                        .elements
                        .iter()
                        .copied()
                        .filter(|e| elements.get(e).is_some_and(|x| Some(x.category) == cast))
                        .collect(),
                });
            }
        }
    }
    Ok(out)
}

/// Locks `board` as it is now under `name` (unique without regard to case), made on
/// `made` if given.
pub fn lock(s: &mut Schedule, board: BoardId, name: &str, made: Option<Date>) -> Result<(), Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::EmptyName);
    }
    if name.chars().count() > crate::plan::MAX_LOCK_NAME || name.chars().any(char::is_control) {
        return Err(Error::Broken(format!(
            "a lock name is one line of at most {} characters",
            crate::plan::MAX_LOCK_NAME
        )));
    }
    if s.plan.lock(name).is_some() {
        return Err(Error::Broken(format!(
            "there is already a lock called {name:?}"
        )));
    }
    let scenes = snapshot(s, board)?;
    s.plan.locks.push(Lock {
        name: name.to_string(),
        board,
        made,
        scenes,
    });
    Ok(())
}

/// Deletes a lock by name.
pub fn unlock(s: &mut Schedule, name: &str) -> Result<(), Error> {
    let before = s.plan.locks.len();
    s.plan.locks.retain(|l| !same_name(&l.name, name));
    if s.plan.locks.len() == before {
        return Err(Error::Broken(format!("there is no lock called {name:?}")));
    }
    Ok(())
}

/// A scene that moved to another day for an actor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    pub sheet: SheetId,
    pub from: WorkDay,
    pub to: WorkDay,
}

/// One actor's drift against a lock.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorDrift {
    pub element: ElementId,
    pub name: String,
    pub board_id: Option<String>,
    /// Work days in the lock and now.
    pub before: usize,
    pub after: usize,
    pub added: Vec<WorkDay>,
    pub dropped: Vec<WorkDay>,
    pub moved: Vec<Move>,
}

impl ActorDrift {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.dropped.is_empty() && self.moved.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Drift {
    pub lock: String,
    pub made: Option<Date>,
    /// Every actor in the lock or now, in board ID order; unchanged ones too.
    pub actors: Vec<ActorDrift>,
}

impl Drift {
    pub fn changed(&self) -> impl Iterator<Item = &ActorDrift> {
        self.actors.iter().filter(|a| !a.is_empty())
    }
}

/// Each actor's drift on the lock's board since the lock called `name`.
pub fn drift(s: &Schedule, name: &str) -> Result<Drift, Error> {
    let lk = s
        .plan
        .lock(name)
        .ok_or_else(|| Error::Broken(format!("there is no lock called {name:?}")))?;
    let now = snapshot(s, lk.board)?;
    // Actor to (scene to its work days: more than one for a scene spanning days), then
    // and now.
    type Plan = BTreeMap<ElementId, BTreeMap<SheetId, BTreeSet<WorkDay>>>;
    let collect = |scenes: &[LockedScene]| {
        let mut m: Plan = BTreeMap::new();
        for sc in scenes {
            for e in &sc.cast {
                m.entry(*e)
                    .or_default()
                    .entry(sc.sheet)
                    .or_default()
                    .insert(work_day(sc));
            }
        }
        m
    };
    let (then, today) = (collect(&lk.scenes), collect(&now));
    let mut ids: BTreeSet<ElementId> = then.keys().copied().collect();
    ids.extend(today.keys().copied());
    let empty = BTreeMap::new();
    let mut actors = Vec::new();
    for el in ids {
        let a = then.get(&el).unwrap_or(&empty);
        let b = today.get(&el).unwrap_or(&empty);
        let days_a: BTreeSet<WorkDay> = a.values().flatten().copied().collect();
        let days_b: BTreeSet<WorkDay> = b.values().flatten().copied().collect();
        let mut moved = Vec::new();
        for (sheet, now) in b {
            // A scene the actor was on in the lock, now on other days: each day it left
            // paired with a day it went to, in date order.
            let Some(was) = a.get(sheet) else { continue };
            for (from, to) in was.difference(now).zip(now.difference(was)) {
                moved.push(Move {
                    sheet: *sheet,
                    from: *from,
                    to: *to,
                });
            }
        }
        let e = s.element(el);
        actors.push(ActorDrift {
            element: el,
            name: e.map_or_else(|| format!("Element {}", el.0), |e| e.name.clone()),
            board_id: e.and_then(|e| e.board_id.clone()),
            before: days_a.len(),
            after: days_b.len(),
            added: days_b.difference(&days_a).copied().collect(),
            dropped: days_a.difference(&days_b).copied().collect(),
            moved,
        });
    }
    actors.sort_by(|x, y| {
        crate::reports::id_order(x.board_id.as_deref(), y.board_id.as_deref())
            .then_with(|| x.name.cmp(&y.name))
    });
    Ok(Drift {
        lock: lk.name.clone(),
        made: lk.made,
        actors,
    })
}

/// The drift as lines of text: one block per actor that drifted.
pub fn drift_text(s: &Schedule, d: &Drift) -> String {
    let mut out = String::new();
    let days = |v: &[WorkDay]| v.iter().map(|w| w.text(s)).collect::<Vec<_>>().join(", ");
    for a in d.changed() {
        let id = a
            .board_id
            .as_deref()
            .map(|x| format!("{x} "))
            .unwrap_or_default();
        let n = |k: usize| {
            if k == 1 {
                "1 day".to_string()
            } else {
                format!("{k} days")
            }
        };
        out.push_str(&format!(
            "{id}{}: {}, was {}\n",
            a.name,
            n(a.after),
            n(a.before)
        ));
        if !a.added.is_empty() {
            out.push_str(&format!("  added: {}\n", days(&a.added)));
        }
        if !a.dropped.is_empty() {
            out.push_str(&format!("  dropped: {}\n", days(&a.dropped)));
        }
        for m in &a.moved {
            let scene = s.sheet(m.sheet).map_or_else(
                || "A scene no longer in the schedule".to_string(),
                crate::timeline::scene_name,
            );
            out.push_str(&format!(
                "  moved: {scene} {} -> {}\n",
                m.from.text(s),
                m.to.text(s)
            ));
        }
    }
    if out.is_empty() {
        out.push_str("No actor's days changed since the lock.\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Place;
    use crate::timeline::tests::la;

    #[test]
    fn lock_then_drift() {
        let (mut s, b, sb) = la();
        lock(&mut s, b, "White", "2024-06-01".parse().ok()).unwrap();
        assert!(lock(&mut s, b, "white", None).is_err());
        let d = drift(&s, "WHITE").unwrap();
        assert!(d.changed().next().is_none());
        assert_eq!(
            drift_text(&s, &d),
            "No actor's days changed since the lock.\n"
        );
        // Move scene 4 (ANN and BO, day 2) to the top of day 1: both lose Friday; ANN
        // keeps Thursday, BO gains it.
        let s4 = s.boards[0].sub_boards[0].strips[5].id;
        s.move_strips(
            b,
            &[s4],
            Place {
                sub_board: Some(sb),
                index: 0,
            },
        )
        .unwrap();
        let d = drift(&s, "White").unwrap();
        let ann = d.actors.iter().find(|a| a.name == "ANN").unwrap();
        assert_eq!(ann.before, 2);
        assert_eq!(ann.after, 1);
        assert_eq!(
            ann.dropped,
            vec![WorkDay::Date("2024-06-21".parse().unwrap())]
        );
        assert_eq!(ann.moved.len(), 1);
        let bo = d.actors.iter().find(|a| a.name == "BO").unwrap();
        assert_eq!(bo.added, vec![WorkDay::Date("2024-06-20".parse().unwrap())]);
        assert_eq!(
            bo.dropped,
            vec![WorkDay::Date("2024-06-21".parse().unwrap())]
        );
        let text = drift_text(&s, &d);
        assert!(text.contains("ANN: 1 day, was 2 days\n  dropped: 2024-06-21\n  moved: Sc 4 2024-06-21 -> 2024-06-20\n"), "{text}");
        unlock(&mut s, "white").unwrap();
        assert!(drift(&s, "White").is_err());
    }

    /// Review: drift once saw one work day per scene, so a scene spanning days lost one.
    #[test]
    fn every_part_day_counts() {
        let (mut s, board, sub, sheet) = crate::timeline::tests::split_two_days();
        let cast = s.cast().unwrap();
        let ann = s.add_element(cast, "ANN").unwrap();
        s.tag(sheet, ann).unwrap();
        lock(&mut s, board, "L", None).unwrap();
        let d = drift(&s, "L").unwrap();
        let a = d.actors.iter().find(|x| x.element == ann).unwrap();
        assert_eq!(a.before, 2, "ANN works two days (the scene spans them)");
        assert!(a.is_empty());
        // Moving the second part a day later is one move, 2 March to 3 March.
        s.insert_strip(
            board,
            crate::model::StripKind::DayBreak,
            Place {
                sub_board: Some(sub),
                index: 2,
            },
        )
        .unwrap();
        let d = drift(&s, "L").unwrap();
        let a = d.actors.iter().find(|x| x.element == ann).unwrap();
        assert_eq!((a.before, a.after), (2, 2));
        assert_eq!(a.moved.len(), 1);
        assert_eq!(
            a.moved[0].from,
            WorkDay::Date("2027-03-02".parse().unwrap())
        );
        assert_eq!(a.moved[0].to, WorkDay::Date("2027-03-03".parse().unwrap()));
    }
}
