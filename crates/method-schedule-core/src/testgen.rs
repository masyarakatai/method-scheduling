//! A tiny seeded random generator for property tests and the sample schedule, so the
//! engine needs no test dependencies (decisions.md, entry 3). Same seed, same sequence,
//! on every machine.
//!
//! With the `testgen` feature (or in this crate's own tests) it also builds random
//! schedules through the engine and mutates files, for the property and fuzz tests here
//! and in the format crate (decision 172). Not part of the engine's API.

#[cfg(any(test, feature = "testgen"))]
mod schedules;
#[cfg(any(test, feature = "testgen"))]
pub use schedules::{random_edit, random_schedule};

/// xorshift64*: fast, deterministic, good enough for generating test cases.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// A value in `0..n`. `n` must be above 0.
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

/// The environment variable that multiplies a fuzz test's rounds (the weekly job sets it).
#[cfg(any(test, feature = "testgen"))]
pub const ROUNDS_VAR: &str = "METHOD_SCHEDULE_FUZZ_ROUNDS";
/// The environment variable mixed into a fuzz test's seed, so a long run can cover new
/// cases; the same value gives the same cases again.
#[cfg(any(test, feature = "testgen"))]
pub const SEED_VAR: &str = "METHOD_SCHEDULE_FUZZ_SEED";

/// `default` rounds, times the whole number in [`ROUNDS_VAR`] when it is set.
#[cfg(any(test, feature = "testgen"))]
pub fn rounds(default: u64) -> u64 {
    let times = std::env::var(ROUNDS_VAR)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(1)
        .max(1);
    default.saturating_mul(times)
}

/// A fuzz test's generator: `seed`, mixed with the number in [`SEED_VAR`] when it is set.
/// A failing case is found again with the same variables and the case number.
#[cfg(any(test, feature = "testgen"))]
pub fn fuzz_rng(seed: u64) -> Rng {
    let extra = std::env::var(SEED_VAR)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .unwrap_or(0);
    Rng::new(seed ^ extra.wrapping_mul(0x9e37_79b9_7f4a_7c15))
}

/// One to four random edits to a file's bytes: a byte changed, a run cut out, one of
/// `snippets` put in, the end cut off, a run of up to 200 bytes doubled, or a few random
/// bytes put in. Each edit grows the file by at most a few hundred bytes.
#[cfg(any(test, feature = "testgen"))]
pub fn mutate_bytes(rng: &mut Rng, bytes: &mut Vec<u8>, snippets: &[&[u8]]) {
    for _ in 0..=rng.below(4) {
        let len = bytes.len() as u64;
        let at = rng.below(len + 1) as usize;
        match rng.below(6) {
            0 if at < bytes.len() => bytes[at] = rng.next_u64() as u8,
            1 => {
                let end = (at + rng.below(40) as usize).min(bytes.len());
                bytes.drain(at..end);
            }
            2 if !snippets.is_empty() => {
                let s = snippets[rng.below(snippets.len() as u64) as usize];
                bytes.splice(at..at, s.iter().copied());
            }
            3 => bytes.truncate(at),
            4 => {
                let end = (at + rng.below(200) as usize).min(bytes.len());
                let copy = bytes[at..end].to_vec();
                bytes.splice(at..at, copy);
            }
            _ => {
                for _ in 0..rng.below(4) {
                    bytes.insert(at.min(bytes.len()), rng.next_u64() as u8);
                }
            }
        }
    }
}
