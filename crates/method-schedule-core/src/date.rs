//! Calendar dates as a day count, written as ISO `YYYY-MM-DD`. Proleptic Gregorian, no
//! time zones: a schedule's dates are plain days.

use std::fmt;
use std::str::FromStr;

/// Days since 1970-01-01.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Weekday {
    pub const ALL: [Weekday; 7] = [
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
}

impl Date {
    /// `None` for a month or day that doesn't exist.
    pub fn from_ymd(y: i32, m: u32, d: u32) -> Option<Date> {
        if !(1..=12).contains(&m) || d == 0 || d > days_in_month(y, m) {
            return None;
        }
        // Count from March so the leap day falls at the end of the year.
        let y = if m <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y.rem_euclid(400);
        let mp = (m + 9) % 12;
        let doy = (153 * mp + 2) / 5 + d - 1;
        let doe = yoe as u32 * 365 + yoe as u32 / 4 - yoe as u32 / 100 + doy;
        Some(Date(era * 146_097 + doe as i32 - 719_468))
    }

    pub fn ymd(self) -> (i32, u32, u32) {
        let z = self.0 + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z.rem_euclid(146_097) as u32;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = yoe as i32 + era * 400 + i32::from(m <= 2);
        (y, m, d)
    }

    pub fn weekday(self) -> Weekday {
        // 1970-01-01 was a Thursday.
        Weekday::ALL[(self.0 + 3).rem_euclid(7) as usize]
    }

    pub fn add_days(self, n: i32) -> Date {
        Date(self.0 + n)
    }
}

fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (y, m, d) = self.ymd();
        write!(f, "{y:04}-{m:02}-{d:02}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseDateError;

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("date must be YYYY-MM-DD")
    }
}

impl std::error::Error for ParseDateError {}

impl FromStr for Date {
    type Err = ParseDateError;
    fn from_str(s: &str) -> Result<Date, ParseDateError> {
        let b = s.as_bytes();
        if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
            return Err(ParseDateError);
        }
        let num = |r: std::ops::Range<usize>| -> Result<u32, ParseDateError> {
            let part = &s[r];
            if !part.bytes().all(|c| c.is_ascii_digit()) {
                return Err(ParseDateError);
            }
            part.parse().map_err(|_| ParseDateError)
        };
        Date::from_ymd(num(0..4)? as i32, num(5..7)?, num(8..10)?).ok_or(ParseDateError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_dates() {
        assert_eq!(Date::from_ymd(1970, 1, 1), Some(Date(0)));
        assert_eq!(
            Date::from_ymd(2000, 3, 1).unwrap().to_string(),
            "2000-03-01"
        );
        let d: Date = "2027-03-01".parse().unwrap();
        assert_eq!(d.weekday(), Weekday::Mon);
        assert_eq!(d.add_days(-1).to_string(), "2027-02-28");
        assert_eq!(
            "2028-02-29"
                .parse::<Date>()
                .unwrap()
                .add_days(1)
                .to_string(),
            "2028-03-01"
        );
    }

    #[test]
    fn rejects_bad_dates() {
        for s in [
            "2027-02-29",
            "2027-13-01",
            "2027-00-10",
            "2027-1-01",
            "27-01-01",
            "2027/01/01",
            "2100-02-29",
        ] {
            assert!(s.parse::<Date>().is_err(), "{s}");
        }
        assert!("2000-02-29".parse::<Date>().is_ok());
    }

    #[test]
    fn property_round_trip_and_weekdays_advance() {
        let mut rng = crate::testgen::Rng::new(7);
        for _ in 0..5_000 {
            let d = Date(rng.below(200_000) as i32 - 100_000);
            let (y, m, dd) = d.ymd();
            assert_eq!(Date::from_ymd(y, m, dd), Some(d));
            if (0..=9999).contains(&y) {
                assert_eq!(d.to_string().parse::<Date>(), Ok(d));
            }
            let i = Weekday::ALL.iter().position(|w| *w == d.weekday()).unwrap();
            assert_eq!(d.add_days(1).weekday(), Weekday::ALL[(i + 1) % 7]);
        }
    }
}
