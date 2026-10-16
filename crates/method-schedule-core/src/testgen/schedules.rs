//! Random schedules built through the engine, for property tests here and in the crates
//! that read and write schedules (the `testgen` feature, for tests only). Moved from the
//! merge tests, where they started.

use crate::days::{auto_day_breaks, Limit};
use crate::model::{
    Banner, CategoryId, DayKind, ElementId, Event, EventKind, Minutes, Place, RedFlag, RedFlagId,
    RedFlagTypeId, Sheet, SheetId, StripColumn, StripField, StripId, StripKind, StripLayout,
    StripWidth, SubBoardId,
};
use crate::{Date, Eighths, Schedule, Weekday};

use super::Rng;

const NAMES: &[&str] = &["ADA", "BEN", "CARL", "DEE", "EVE", "Lamp", "Car", "Dog"];
const CATS: &[&str] = &["Props", "Vehicles", "Animals", "Wardrobe"];

fn pick<'a, T>(rng: &mut Rng, v: &'a [T]) -> Option<&'a T> {
    if v.is_empty() {
        None
    } else {
        Some(&v[rng.below(v.len() as u64) as usize])
    }
}

fn date(rng: &mut Rng) -> Date {
    "2027-03-01"
        .parse::<Date>()
        .unwrap()
        .add_days(rng.below(40) as i32)
}

/// A random red flag of type `kind`: a range, or open-ended either way.
fn flag(rng: &mut Rng, kind: RedFlagTypeId) -> RedFlag {
    let d = date(rng);
    RedFlag {
        id: RedFlagId(0),
        kind,
        dates: match rng.below(4) {
            0 => crate::model::RedFlagDates::NotBefore(d),
            1 => crate::model::RedFlagDates::NotAfter(d),
            _ => crate::model::RedFlagDates::Range {
                from: d,
                to: d.add_days(rng.below(3) as i32),
            },
        },
        note: format!("Note {}", rng.below(3)),
        hard: rng.below(2) == 0,
    }
}

/// A small random schedule: a few categories, elements and sheets, most strips on
/// Main with day breaks, a start date, some calendar marks and events.
pub fn random_schedule(rng: &mut Rng) -> Schedule {
    let mut s = Schedule::new("ZTEST Merge");
    for _ in 0..rng.below(3) {
        let n = *pick(rng, CATS).unwrap();
        s.add_category(n);
    }
    let cats: Vec<CategoryId> = s.categories.iter().map(|c| c.id).collect();
    for _ in 0..3 + rng.below(5) {
        let c = *pick(rng, &cats).unwrap();
        let n = *pick(rng, NAMES).unwrap();
        let _ = s.add_element(c, n);
    }
    let els: Vec<ElementId> = s.elements.iter().map(|e| e.id).collect();
    for i in 0..3 + rng.below(6) {
        let mut elements = Vec::new();
        for e in &els {
            if rng.below(3) == 0 {
                elements.push(*e);
            }
        }
        s.add_sheet(Sheet {
            scenes: (i + 1).to_string(),
            pages: Eighths(1 + rng.below(8) as u32),
            elements,
            ..Sheet::default()
        });
    }
    let b = s.boards[0].id;
    let sb = s.boards[0].sub_boards[0].id;
    let ids: Vec<StripId> = s.boards[0]
        .boneyard
        .iter()
        .map(|x| x.id)
        .filter(|_| rng.below(4) != 0)
        .collect();
    s.move_strips(
        b,
        &ids,
        Place {
            sub_board: Some(sb),
            index: 0,
        },
    )
    .unwrap();
    auto_day_breaks(&mut s, b, sb, Limit::Pages(Eighths(8))).unwrap();
    s.boards[0].sub_boards[0].start = Some(date(rng));
    for _ in 0..rng.below(3) {
        let d = date(rng);
        s.boards[0].sub_boards[0]
            .calendar
            .set_day_kind(d, DayKind::Holiday);
    }
    if rng.below(2) == 0 {
        let e = *pick(rng, &els).unwrap();
        let d = date(rng);
        s.set_event(
            b,
            sb,
            None,
            Event {
                name: "Fitting".into(),
                kind: EventKind::Fitting,
                from: d,
                to: d,
                elements: vec![e],
                color: None,
            },
        )
        .unwrap();
    }
    if rng.below(2) == 0 {
        let t = s.add_red_flag_type("Unavailable").unwrap();
        for _ in 0..rng.below(3) {
            let e = *pick(rng, &els).unwrap();
            let f = flag(rng, t);
            s.set_red_flag(e, f).unwrap();
        }
    }
    s.check().unwrap();
    s
}

/// One random edit through the engine (refusals are fine: nothing changes).
pub fn random_edit(rng: &mut Rng, s: &mut Schedule) {
    let els: Vec<ElementId> = s.elements.iter().map(|e| e.id).collect();
    let cats: Vec<CategoryId> = s.categories.iter().map(|c| c.id).collect();
    let sheets: Vec<SheetId> = s.sheets.iter().map(|e| e.id).collect();
    let b = s.boards[rng.below(s.boards.len() as u64) as usize].clone();
    let subs: Vec<SubBoardId> = b.sub_boards.iter().map(|x| x.id).collect();
    let strips: Vec<StripId> = b
        .sub_boards
        .iter()
        .flat_map(|x| &x.strips)
        .chain(&b.boneyard)
        .map(|x| x.id)
        .collect();
    let place = |rng: &mut Rng| {
        let sub = if rng.below(4) == 0 {
            None
        } else {
            pick(rng, &subs).copied()
        };
        Place {
            sub_board: sub,
            index: rng.below(12) as usize,
        }
    };
    let types: Vec<RedFlagTypeId> = s.red_flag_types.iter().map(|t| t.id).collect();
    let flags: Vec<(ElementId, RedFlagId)> = s
        .elements
        .iter()
        .flat_map(|e| e.red_flags.iter().map(move |f| (e.id, f.id)))
        .collect();
    match rng.below(29) {
        0 => s.title = format!("ZTEST {}", rng.below(5)),
        1 => {
            let n = *pick(rng, CATS).unwrap();
            s.add_category(n);
        }
        2 => {
            if let Some(c) = pick(rng, &cats) {
                let n = *pick(rng, CATS).unwrap();
                let _ = s.rename_category(*c, &format!("{n} {}", rng.below(3)));
            }
        }
        3 => {
            if let Some(c) = pick(rng, &cats) {
                let _ = s.move_category(*c, rng.below(4) as usize);
            }
        }
        4 => {
            if let Some(c) = pick(rng, &cats) {
                let _ = s.delete_category(*c);
            }
        }
        5 => {
            if let Some(c) = pick(rng, &cats) {
                let n = *pick(rng, NAMES).unwrap();
                let _ = s.add_element(*c, n);
            }
        }
        6 => {
            if let Some(e) = pick(rng, &els) {
                let n = *pick(rng, NAMES).unwrap();
                let _ = s.rename_element(*e, n);
            }
        }
        7 => {
            if let Some(e) = pick(rng, &els) {
                let id = rng.below(5).to_string();
                let _ = s.set_board_id(*e, if id == "0" { None } else { Some(&id) });
            }
        }
        8 => {
            if let Some(e) = pick(rng, &els) {
                let r = &mut s.element_mut(*e).unwrap().rules;
                match rng.below(3) {
                    0 => r.holds = !r.holds,
                    1 => r.drops = !r.drops,
                    _ => r.min_drop_gap = 1 + rng.below(14) as u32,
                }
            }
        }
        9 => {
            if let Some(e) = pick(rng, &els) {
                let _ = s.delete_element(*e, rng.below(2) == 0);
            }
        }
        10 => {
            if let (Some(a), Some(c)) = (pick(rng, &els).copied(), pick(rng, &els).copied()) {
                let _ = s.merge_elements(a, &[c]);
            }
        }
        11 => {
            if let (Some(e), Some(c)) = (pick(rng, &els), pick(rng, &cats)) {
                let _ = s.move_element(*e, *c);
            }
        }
        12 => {
            if let (Some(sh), Some(e)) = (pick(rng, &sheets), pick(rng, &els)) {
                if rng.below(2) == 0 {
                    let _ = s.tag(*sh, *e);
                } else {
                    let _ = s.untag(*sh, *e);
                }
            }
        }
        13 => {
            let at = place(rng);
            let _ = s.add_sheet_at(
                Sheet {
                    scenes: format!("{}A", rng.below(9)),
                    pages: Eighths(1 + rng.below(8) as u32),
                    ..Sheet::default()
                },
                b.id,
                at,
            );
        }
        14 => {
            if let Some(sh) = pick(rng, &sheets) {
                let _ = s.delete_sheet(*sh);
            }
        }
        15 => {
            if let Some(sh) = pick(rng, &sheets) {
                let x = s.sheet_mut(*sh).unwrap();
                match rng.below(3) {
                    0 => x.set = format!("SET {}", rng.below(4)),
                    1 => x.pages = Eighths(1 + rng.below(16) as u32),
                    _ => x.synopsis = format!("Synopsis {}", rng.below(4)),
                }
            }
        }
        16 | 17 => {
            if !strips.is_empty() {
                let n = 1 + rng.below(3);
                let mut chosen = Vec::new();
                for _ in 0..n {
                    chosen.push(*pick(rng, &strips).unwrap());
                }
                let at = place(rng);
                let _ = s.move_strips(b.id, &chosen, at);
            }
        }
        18 => {
            let at = place(rng);
            let kind = if rng.below(2) == 0 {
                StripKind::DayBreak
            } else {
                StripKind::Banner(Banner {
                    text: format!("Banner {}", rng.below(3)),
                    duration: Minutes(rng.below(3) as u32 * 15),
                    ..Default::default()
                })
            };
            let _ = s.insert_strip(b.id, kind, at);
        }
        19 => {
            if let Some(st) = pick(rng, &strips) {
                let _ = s.delete_strip(b.id, *st);
            }
        }
        20 => {
            if let Some(sb) = pick(rng, &subs) {
                let kind = *pick(
                    rng,
                    &[DayKind::Work, DayKind::Off, DayKind::Holiday, DayKind::Move],
                )
                .unwrap();
                let d = date(rng);
                s.calendar_mut(b.id, *sb).unwrap().set_day_kind(d, kind);
            }
        }
        21 => {
            if let Some(sb) = pick(rng, &subs) {
                let off: Vec<Weekday> = Weekday::ALL
                    .iter()
                    .copied()
                    .filter(|_| rng.below(3) == 0)
                    .collect();
                let _ = s.calendar_mut(b.id, *sb).unwrap().set_weekly_off(&off);
            }
        }
        22 => {
            if let Some(sb) = pick(rng, &subs) {
                let n = s.calendar_mut(b.id, *sb).unwrap().events.len();
                if n > 0 && rng.below(2) == 0 {
                    let _ = s.delete_event(b.id, *sb, rng.below(n as u64) as usize);
                } else {
                    let from = date(rng);
                    let ev = Event {
                        name: format!("Event {}", rng.below(3)),
                        kind: if rng.below(2) == 0 {
                            EventKind::Travel
                        } else {
                            EventKind::Other("Camera test".into())
                        },
                        from,
                        to: from.add_days(rng.below(3) as i32),
                        elements: els.iter().copied().filter(|_| rng.below(3) == 0).collect(),
                        color: None,
                    };
                    let at = if n > 0 && rng.below(2) == 0 {
                        Some(rng.below(n as u64) as usize)
                    } else {
                        None
                    };
                    let _ = s.set_event(b.id, *sb, at, ev);
                }
            }
        }
        23 => {
            if let Some(sb) = pick(rng, &subs) {
                let start = if rng.below(4) == 0 {
                    None
                } else {
                    Some(date(rng))
                };
                let x = s.boards.iter_mut().find(|x| x.id == b.id).unwrap();
                x.sub_boards.iter_mut().find(|x| x.id == *sb).unwrap().start = start;
            }
        }
        24 => {
            if rng.below(2) == 0 {
                let _ = s.add_sub_board(b.id, &format!("Unit {}", rng.below(3)), Some(date(rng)));
            } else if let Some(sb) = pick(rng, &subs) {
                let _ = s.delete_sub_board(b.id, *sb);
            }
        }
        25 => {
            const TYPES: &[&str] = &["Unavailable", "Second job", "Child hours"];
            match (rng.below(4), pick(rng, &types).copied()) {
                (0, _) | (_, None) => {
                    let _ = s.add_red_flag_type(pick(rng, TYPES).unwrap());
                }
                (1, Some(t)) => {
                    let n = format!("{} {}", pick(rng, TYPES).unwrap(), rng.below(3));
                    let _ = s.rename_red_flag_type(t, &n);
                }
                (2, Some(t)) => {
                    let _ = s.move_red_flag_type(t, rng.below(3) as usize);
                }
                (_, Some(t)) => {
                    let _ = s.delete_red_flag_type(t);
                }
            }
        }
        26 => {
            if let (Some(e), Some(t)) = (pick(rng, &els).copied(), pick(rng, &types).copied()) {
                let mut f = flag(rng, t);
                let mut on = e;
                if let Some((fe, id)) = pick(rng, &flags).copied() {
                    if rng.below(2) == 0 {
                        f.id = id;
                        on = fe;
                    }
                }
                let _ = s.set_red_flag(on, f);
            }
        }
        27 => {
            if let Some((_, id)) = pick(rng, &flags) {
                let _ = s.delete_red_flag(*id);
            }
        }
        _ => match rng.below(6) {
            0 => {
                let _ = s.duplicate_board(b.id, &format!("Board {}", rng.below(3)));
            }
            5 => {
                // A strip layout: a few fields from the start of a shuffled list.
                let mut fields = StripField::ALL.to_vec();
                for i in (1..fields.len()).rev() {
                    fields.swap(i, rng.below(i as u64 + 1) as usize);
                }
                let n = 1 + rng.below(4) as usize;
                let columns = fields[..n]
                    .iter()
                    .map(|f| StripColumn {
                        width: StripWidth::ALL[rng.below(4) as usize],
                        bold: rng.below(4) == 0,
                        ..StripColumn::of(*f)
                    })
                    .collect();
                s.set_strip_layout(b.id, StripLayout::of(columns)).unwrap();
            }
            3 => {
                if let (Some(a), Some(c)) =
                    (pick(rng, &sheets).copied(), pick(rng, &sheets).copied())
                {
                    if a != c {
                        s.merge_sheets(a, &[c]).unwrap();
                    }
                }
            }
            4 => {
                if let Some(sh) = pick(rng, &sheets) {
                    s.duplicate_sheet(*sh).unwrap();
                }
            }
            1 => {
                if let (Some(a), Some(c)) = (pick(rng, &els).copied(), pick(rng, &els).copied()) {
                    if a != c && !s.element(a).unwrap().links.contains(&c) {
                        s.element_mut(a).unwrap().links.push(c);
                    }
                }
            }
            _ => {
                if let Some(e) = pick(rng, &els) {
                    s.element_mut(*e).unwrap().notes = format!("Note {}", rng.below(3));
                }
            }
        },
    }
    s.check().unwrap();
}
