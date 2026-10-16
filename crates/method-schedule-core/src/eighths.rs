//! Page lengths in eighths of a page.
//!
//! A scene's length is counted in eighths: `1 3/8` is one page and three eighths, stored as
//! 11. Eight eighths always roll into a page, so there is one way to write every length.
//! Integers only, no floats (decisions.md, entry 3).

use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

/// A page length as a whole number of eighths.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Eighths(pub u32);

impl Eighths {
    pub const ZERO: Eighths = Eighths(0);

    /// `pages` whole pages plus `eighths` eighths. An `eighths` of 8 or more rolls over.
    /// `None` on overflow.
    pub fn new(pages: u32, eighths: u32) -> Option<Eighths> {
        pages.checked_mul(8)?.checked_add(eighths).map(Eighths)
    }

    /// Whole pages.
    pub fn pages(self) -> u32 {
        self.0 / 8
    }

    /// Eighths left over after the whole pages, 0 to 7.
    pub fn remainder(self) -> u32 {
        self.0 % 8
    }

    pub fn checked_add(self, other: Eighths) -> Option<Eighths> {
        self.0.checked_add(other.0).map(Eighths)
    }
}

impl Add for Eighths {
    type Output = Eighths;
    /// Saturates: a total past `u32::MAX` eighths stays there rather than panicking.
    fn add(self, other: Eighths) -> Eighths {
        Eighths(self.0.saturating_add(other.0))
    }
}

impl AddAssign for Eighths {
    fn add_assign(&mut self, other: Eighths) {
        self.0 = self.0.saturating_add(other.0);
    }
}

impl Sum for Eighths {
    fn sum<I: Iterator<Item = Eighths>>(iter: I) -> Eighths {
        iter.fold(Eighths::ZERO, Add::add)
    }
}

/// `0`, `3/8`, `2`, `1 3/8`. How the reference product shows a zero length is an open
/// engine check; `0` until we know.
impl fmt::Display for Eighths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.pages(), self.remainder()) {
            (p, 0) => write!(f, "{p}"),
            (0, r) => write!(f, "{r}/8"),
            (p, r) => write!(f, "{p} {r}/8"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseEighthsError {
    Empty,
    Invalid,
    Overflow,
}

impl fmt::Display for ParseEighthsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ParseEighthsError::Empty => "page length is empty",
            ParseEighthsError::Invalid => {
                "page length must look like 2, 3/8 or 1 3/8 (eighths only)"
            }
            ParseEighthsError::Overflow => "page length is too large",
        })
    }
}

impl std::error::Error for ParseEighthsError {}

/// Reads `2`, `3/8`, `1 3/8` and `11/8` (which rolls into `1 3/8`). Surrounding and
/// repeated inner spaces are fine. Only eighths are accepted as a fraction.
impl FromStr for Eighths {
    type Err = ParseEighthsError;

    fn from_str(s: &str) -> Result<Eighths, ParseEighthsError> {
        let mut parts = s.split_whitespace();
        let first = parts.next().ok_or(ParseEighthsError::Empty)?;
        let second = parts.next();
        if parts.next().is_some() {
            return Err(ParseEighthsError::Invalid);
        }
        match second {
            None => match first.split_once('/') {
                Some((n, d)) => fraction(n, d).map(Eighths),
                None => Eighths::new(number(first)?, 0).ok_or(ParseEighthsError::Overflow),
            },
            Some(frac) => {
                let (n, d) = frac.split_once('/').ok_or(ParseEighthsError::Invalid)?;
                Eighths::new(number(first)?, fraction(n, d)?).ok_or(ParseEighthsError::Overflow)
            }
        }
    }
}

impl Eighths {
    /// Reads a page length the way the reference app reads one from a script file: like
    /// `FromStr`, except a bare number counts eighths, so `1` is 1/8 (behavior note 16).
    pub fn from_script(s: &str) -> Result<Eighths, ParseEighthsError> {
        let t = s.trim();
        if !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()) {
            return Ok(Eighths(number(t)?));
        }
        t.parse()
    }
}

fn number(s: &str) -> Result<u32, ParseEighthsError> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ParseEighthsError::Invalid);
    }
    s.parse().map_err(|_| ParseEighthsError::Overflow)
}

fn fraction(n: &str, d: &str) -> Result<u32, ParseEighthsError> {
    if number(d)? != 8 {
        return Err(ParseEighthsError::Invalid);
    }
    number(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testgen::Rng;

    fn e(s: &str) -> Eighths {
        s.parse().unwrap()
    }

    #[test]
    fn displays_pages_and_eighths() {
        assert_eq!(Eighths(0).to_string(), "0");
        assert_eq!(Eighths(3).to_string(), "3/8");
        assert_eq!(Eighths(8).to_string(), "1");
        assert_eq!(Eighths(11).to_string(), "1 3/8");
        assert_eq!(Eighths(16).to_string(), "2");
    }

    #[test]
    fn parses_every_written_form() {
        assert_eq!(e("2"), Eighths(16));
        assert_eq!(e("3/8"), Eighths(3));
        assert_eq!(e("1 3/8"), Eighths(11));
        assert_eq!(e("  1   3/8 "), Eighths(11));
        assert_eq!(e("0"), Eighths(0));
        assert_eq!(e("0/8"), Eighths(0));
    }

    #[test]
    fn more_than_eight_eighths_rolls_into_pages() {
        assert_eq!(e("11/8"), Eighths(11));
        assert_eq!(e("11/8").to_string(), "1 3/8");
        assert_eq!(e("1 9/8").to_string(), "2 1/8");
        assert_eq!((e("5/8") + e("6/8")).to_string(), "1 3/8");
    }

    #[test]
    fn rejects_bad_input() {
        use ParseEighthsError::*;
        for (s, err) in [
            ("", Empty),
            ("   ", Empty),
            ("1/4", Invalid),
            ("1 1/4", Invalid),
            ("-1", Invalid),
            ("+1", Invalid),
            ("1.5", Invalid),
            ("1 3", Invalid),
            ("1 2 3/8", Invalid),
            ("3/", Invalid),
            ("/8", Invalid),
            ("99999999999", Overflow),
            ("536870912", Overflow),
        ] {
            assert_eq!(s.parse::<Eighths>(), Err(err), "{s:?}");
        }
    }

    #[test]
    fn script_lengths_read_a_bare_number_as_eighths() {
        // ZTEST run 3: each form as the real app read it.
        for (form, want) in [
            ("8/8", 8),
            ("1 0/8", 8),
            ("1", 1),
            ("4/8", 4),
            ("10/8", 10),
            ("2 1/8", 17),
        ] {
            assert_eq!(Eighths::from_script(form), Ok(Eighths(want)), "{form}");
        }
        assert!(Eighths::from_script("").is_err());
    }

    #[test]
    fn sums() {
        let total: Eighths = ["1 3/8", "5/8", "2"].iter().map(|s| e(s)).sum();
        assert_eq!(total.to_string(), "4");
    }

    #[test]
    fn property_display_parse_round_trip() {
        let mut rng = Rng::new(0x5eed_e1e1);
        for _ in 0..10_000 {
            let x = Eighths(rng.below(1_000_000) as u32);
            assert_eq!(x.to_string().parse::<Eighths>(), Ok(x));
            assert_eq!(Eighths::new(x.pages(), x.remainder()), Some(x));
            assert!(x.remainder() < 8);
        }
    }
}
