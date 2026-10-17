//! The `plan` key: planning data for the features beyond the reference app
//! (method_schedule_core::plan; docs/FORMAT.md). Left out while empty.

use serde::{Deserialize, Serialize};

use method_schedule_core::model::{BoardId, CategoryId, ElementId, SheetId, StripId, SubBoardId};
use method_schedule_core::plan::{
    CallTime, GeoLocation, Lock, LockedScene, Plan, Turnaround, DEFAULT_LONG_DAY,
};

use method_schedule_core::solar::DaylightSaving;

use crate::{date, default_given, FormatError};

fn is_zero_i(n: &i32) -> bool {
    *n == 0
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FilePlan {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    locations: Vec<FileLocation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    call_times: Vec<FileCallTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    long_day_minutes: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    turnarounds: Vec<FileTurnaround>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    locks: Vec<FileLock>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pinned: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    omitted: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileLocation {
    name: String,
    /// Millionths of a degree, north positive.
    latitude_microdeg: i32,
    /// Millionths of a degree, east positive.
    longitude_microdeg: i32,
    #[serde(default, skip_serializing_if = "is_zero_i")]
    utc_offset_minutes: i32,
    /// A daylight saving rule's name (decision 193); left out for none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    daylight_saving: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileCallTime {
    /// The day break strip that closes the day.
    day: u32,
    /// Minutes after midnight.
    minutes: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileTurnaround {
    category: u32,
    minutes: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileLock {
    name: String,
    board: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    made: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    scenes: Vec<FileLockedScene>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileLockedScene {
    sheet: u32,
    sub_board: u32,
    day: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    cast: Vec<u32>,
}

pub(crate) fn out_plan(p: &Plan) -> Option<FilePlan> {
    if p.is_empty() {
        return None;
    }
    Some(FilePlan {
        locations: p
            .locations
            .iter()
            .map(|l| FileLocation {
                name: l.name.clone(),
                latitude_microdeg: l.latitude,
                longitude_microdeg: l.longitude,
                utc_offset_minutes: l.utc_offset,
                daylight_saving: l.daylight_saving.map(|r| r.name().to_string()),
            })
            .collect(),
        call_times: p
            .call_times
            .iter()
            .map(|c| FileCallTime {
                day: c.day.0,
                minutes: c.minutes,
            })
            .collect(),
        long_day_minutes: p.long_day,
        turnarounds: p
            .turnarounds
            .iter()
            .map(|t| FileTurnaround {
                category: t.category.0,
                minutes: t.minutes,
            })
            .collect(),
        locks: p
            .locks
            .iter()
            .map(|l| FileLock {
                name: l.name.clone(),
                board: l.board.0,
                made: l.made.map(|d| d.to_string()),
                scenes: l
                    .scenes
                    .iter()
                    .map(|sc| FileLockedScene {
                        sheet: sc.sheet.0,
                        sub_board: sc.sub_board.0,
                        day: sc.day,
                        date: sc.date.map(|d| d.to_string()),
                        cast: sc.cast.iter().map(|e| e.0).collect(),
                    })
                    .collect(),
            })
            .collect(),
        pinned: p.pinned.iter().map(|x| x.0).collect(),
        omitted: p.omitted.iter().map(|x| x.0).collect(),
    })
}

fn bad(why: String) -> FormatError {
    FormatError::BadValue(format!("plan: {why}"))
}

pub(crate) fn in_plan(f: Option<FilePlan>) -> Result<Plan, FormatError> {
    let Some(f) = f else {
        return Ok(Plan::default());
    };
    let mut p = Plan::default();
    for l in f.locations {
        let name = l.name.clone();
        if name.trim() != name {
            return Err(bad(format!(
                "location {name:?} has spaces around it; the name is written trimmed"
            )));
        }
        if p.location(&name).is_some() {
            return Err(bad(format!("location {name:?} is listed twice")));
        }
        // Only the names the writer writes: none is left out, not written.
        let daylight_saving = match l.daylight_saving.as_deref() {
            None => None,
            Some("none") => return Err(default_given("daylight_saving", "none")),
            Some(t) => Some(
                DaylightSaving::ALL
                    .into_iter()
                    .find(|r| r.name() == t)
                    .ok_or_else(|| {
                        bad(format!(
                            "location {name:?}: daylight saving {t:?} is not us_canada, eu, australia or new_zealand"
                        ))
                    })?,
            ),
        };
        p.set_location(GeoLocation {
            name: l.name,
            latitude: l.latitude_microdeg,
            longitude: l.longitude_microdeg,
            utc_offset: l.utc_offset_minutes,
            daylight_saving,
        })
        .map_err(|e| bad(format!("location {name:?}: {e}")))?;
    }
    for c in f.call_times {
        if c.minutes >= 24 * 60 {
            return Err(bad(format!("call time {} is past 23:59", c.minutes)));
        }
        if p.call_times.iter().any(|x| x.day.0 == c.day) {
            return Err(bad(format!("day {} has two call times", c.day)));
        }
        p.call_times.push(CallTime {
            day: StripId(c.day),
            minutes: c.minutes,
        });
    }
    if let Some(m) = f.long_day_minutes {
        if m == 0 || m > 24 * 60 {
            return Err(bad(format!("long day limit {m} is out of range")));
        }
        if m == DEFAULT_LONG_DAY {
            return Err(default_given("long_day_minutes", &m.to_string()));
        }
    }
    p.long_day = f.long_day_minutes;
    for t in f.turnarounds {
        if t.minutes == 0 || t.minutes > 48 * 60 {
            return Err(bad(format!("turnaround {} is out of range", t.minutes)));
        }
        if p.turnarounds.iter().any(|x| x.category.0 == t.category) {
            return Err(bad(format!("category {} has two turnarounds", t.category)));
        }
        p.turnarounds.push(Turnaround {
            category: CategoryId(t.category),
            minutes: t.minutes,
        });
    }
    for l in f.locks {
        if l.name.trim().is_empty() || p.lock(&l.name).is_some() {
            return Err(bad(format!(
                "lock name {:?} is empty or used twice",
                l.name
            )));
        }
        let scenes = l
            .scenes
            .into_iter()
            .map(|sc| {
                Ok(LockedScene {
                    sheet: SheetId(sc.sheet),
                    sub_board: SubBoardId(sc.sub_board),
                    day: sc.day,
                    date: sc.date.as_deref().map(date).transpose()?,
                    cast: sc.cast.into_iter().map(ElementId).collect(),
                })
            })
            .collect::<Result<_, FormatError>>()?;
        p.locks.push(Lock {
            name: l.name,
            board: BoardId(l.board),
            made: l.made.as_deref().map(date).transpose()?,
            scenes,
        });
    }
    p.pinned = f.pinned.into_iter().map(StripId).collect();
    p.omitted = f.omitted.into_iter().map(SheetId).collect();
    if p.is_empty() {
        return Err(default_given("plan", "{}"));
    }
    Ok(p)
}
