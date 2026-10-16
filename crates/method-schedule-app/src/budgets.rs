//! Performance budgets at the kickoff's 1,500-strip target (decisions.md, entries 24 and
//! 176): the edit and the board view, the day out of days grid, opening and checking a
//! file, and each report's data, each timed as the median of several runs and failed when
//! over its budget. Release builds only; CI runs them in their own job with
//! `cargo test --release --workspace --locked -- --ignored budget --nocapture`.
//!
//! The 5 ms for an edit plus the board view is the target; the other budgets are a
//! generous multiple of what was measured on the owner's Mac (entry 176), so a shared
//! runner passes them and a slip of several times fails.
//!
//! Every budget is for the owner's Mac. On a slower machine (a shared CI runner) each is
//! multiplied by how much slower a fixed piece of work runs there, at least 1x and at most
//! 4x, and the run prints that factor (entry 191).

use std::time::{Duration, Instant};

use method_schedule_core::model::Place;
use method_schedule_core::sample::sample;
use method_schedule_core::setup::{ReportKind, ReportSetup};

use crate::session::Session;
use crate::view::{self, view};

/// Runs each timing this many times and keeps the median.
const RUNS: usize = 15;

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}

/// The median of `RUNS` timings of `f`, after one run to warm up.
fn median(mut f: impl FnMut()) -> Duration {
    f();
    let mut times: Vec<Duration> = (0..RUNS)
        .map(|_| {
            let t = Instant::now();
            f();
            t.elapsed()
        })
        .collect();
    times.sort();
    times[RUNS / 2]
}

/// What the budgets are multiplied by on this machine, in percent: worked out once, from
/// how long a fixed piece of work takes here against the owner's Mac (entry 191).
fn scale() -> u32 {
    static SCALE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *SCALE.get_or_init(method_schedule_core::sample::budget_scale_percent)
}

/// Prints `took` against `budget`, scaled for this machine, and says whether it is within.
fn within(label: &str, took: Duration, budget: Duration) -> bool {
    let scale = scale();
    let allowed = budget * scale / 100;
    let ok = took <= allowed;
    println!(
        "budget {label:<36} {took:>10.2?} of {allowed:>8.2?} ({budget:?} x{}.{:02}){}",
        scale / 100,
        scale % 100,
        if ok { "" } else { "  OVER" }
    );
    ok
}

/// Budgets hold for release builds only: a debug build is many times slower.
fn release() -> bool {
    if cfg!(debug_assertions) {
        println!("budgets skipped: run them with cargo test --release");
    }
    !cfg!(debug_assertions)
}

/// Each report's budget: about four to five times its time on the owner's Mac, rounded
/// up (decisions.md, entry 176). The element detail lists every scene of every element.
fn report_budget(kind: ReportKind) -> Duration {
    ms(match kind {
        ReportKind::ElementDetail => 100,
        ReportKind::Dood => 40,
        ReportKind::OneLine
        | ReportKind::Shooting
        | ReportKind::Breakdown
        | ReportKind::BreakdownDay
        | ReportKind::MultiUnit => 25,
        ReportKind::Elements => 15,
        ReportKind::Vertical => 8,
        ReportKind::Totals | ReportKind::Calendar => 5,
        ReportKind::RedFlags => 3,
        ReportKind::Conflicts => 2,
    })
}

/// A session holding the 1,500-scene sample.
fn big() -> Session {
    let mut s = Session::new();
    s.edit(|sch| {
        *sch = sample(1500);
        Ok(())
    })
    .unwrap();
    s
}

/// An edit (moving one strip, through the session's copy, check and undo step) plus the
/// board view the page redraws from: under 5 ms. With the view serialized as the page
/// receives it, under 15 ms. METHOD_SCHEDULE_VIEW_DUMP=path also writes that view, which
/// page-tests/board-budget.mjs times the page's redraw from.
#[test]
#[ignore]
fn budget_edit_and_board_view_at_1500_strips() {
    if !release() {
        return;
    }
    let mut s = big();
    let (b, sb) = (
        s.schedule().boards[0].id,
        s.schedule().boards[0].sub_boards[0].id,
    );
    let strips: Vec<_> = s.schedule().boards[0].sub_boards[0]
        .strips
        .iter()
        .map(|x| x.id)
        .collect();
    let mut i = 0;
    let mut step = |s: &mut Session, json: bool| {
        i += 1;
        let pick = strips[(i * 37) % strips.len()];
        s.edit(|sch| {
            sch.move_strips(
                b,
                &[pick],
                Place {
                    sub_board: Some(sb),
                    index: (i * 101) % strips.len(),
                },
            )
            .map_err(|e| e.to_string())
        })
        .unwrap();
        let v = view(s, None).unwrap();
        if json {
            assert!(serde_json::to_string(&v).unwrap().len() > 100_000);
        }
    };
    let plain = median(|| step(&mut s, false));
    let with_json = median(|| step(&mut s, true));
    let edit_view = within("edit + board view", plain, ms(5));
    let edit_json = within("edit + board view + JSON", with_json, ms(15));
    if let Ok(path) = std::env::var("METHOD_SCHEDULE_VIEW_DUMP") {
        std::fs::write(
            path,
            serde_json::to_string(&view(&s, None).unwrap()).unwrap(),
        )
        .unwrap();
    }
    assert!(edit_view && edit_json, "over budget at 1,500 strips");
}

/// The Day out of days tab's grid for the cast, opening and checking the file (what
/// `method-schedule check FILE` does after reading it), and the data of each report in
/// the default list, at 1,500 strips.
#[test]
#[ignore]
fn budget_dood_check_and_reports_at_1500_strips() {
    if !release() {
        return;
    }
    let s = big();
    let sch = s.schedule();
    let sub = view::first_sub(sch);
    let mut ok = true;
    let grid = median(|| {
        assert!(!view::dood_grid(sch, sub, None).unwrap().rows.is_empty());
    });
    ok &= within("day out of days grid", grid, ms(20));
    let text = method_schedule_format::to_string(sch);
    let check = median(|| {
        method_schedule_format::from_str(&text).unwrap();
    });
    ok &= within("read and check the file", check, ms(30));
    for kind in ReportKind::ALL {
        let setup = ReportSetup::of(kind);
        let took = median(|| {
            crate::reports::build(sch, sub, 0, &setup, "sun").unwrap();
        });
        ok &= within(
            &format!("report {}", kind.name()),
            took,
            report_budget(kind),
        );
    }
    assert!(ok, "over budget at 1,500 strips");
}
