//! `method-schedule merge-driver` as git runs it (docs/MERGE.md, "With git"): a real `git merge` of
//! two branches that changed different fields of one schedule, then two that changed the
//! same one. Git runs only inside a temporary folder this test makes, with the user's and
//! the system's git settings left out.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use method_schedule_core::model::Place;
use method_schedule_core::{fountain, Schedule};

/// A fresh folder, removed when the test ends.
struct Dir(PathBuf);

impl Dir {
    fn new(name: &str) -> Dir {
        let p = std::env::temp_dir().join(format!(
            "method-schedule-merge-git-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        Dir(p)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn git(dir: &Path, args: &[&str]) -> Output {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args([
            "-c",
            "user.name=ZTEST",
            "-c",
            "user.email=ztest@example.invalid",
        ])
        .args([
            "-c",
            "commit.gpgsign=false",
            "-c",
            "core.hooksPath=/dev/null",
        ])
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("HOME", dir)
        .output()
        .expect("git runs")
}

fn ok(o: Output) -> Output {
    assert!(
        o.status.success(),
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    o
}

/// The Harbor Light script on Main, as a schedule.
fn harbor() -> Schedule {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/scripts/harbor-light.fountain"
    ))
    .unwrap();
    let mut s = Schedule::new("ZTEST Harbor Light");
    let cast = s.add_category("Cast");
    s.import_script(&fountain::parse(&text), cast).unwrap();
    let (b, sb) = (s.boards[0].id, s.boards[0].sub_boards[0].id);
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
    s
}

fn read(dir: &Path) -> Schedule {
    method_schedule_format::from_str(&std::fs::read_to_string(dir.join("film.mtsc")).unwrap())
        .expect("the merged file is a valid schedule")
}

fn write(dir: &Path, s: &Schedule) {
    std::fs::write(dir.join("film.mtsc"), method_schedule_format::to_string(s)).unwrap();
}

#[test]
fn merge_driver_under_git() {
    if Command::new("git").arg("--version").output().is_err() {
        eprintln!("git is not installed; skipping");
        return;
    }
    let d = Dir::new("driver");
    let dir = d.0.as_path();
    ok(git(dir, &["init", "--quiet", "--initial-branch=main"]));
    let driver = format!(
        "\"{}\" merge-driver %O %A %B %P",
        env!("CARGO_BIN_EXE_method-schedule")
    );
    ok(git(
        dir,
        &["config", "merge.method-schedule.name", "Schedule merge"],
    ));
    ok(git(
        dir,
        &["config", "merge.method-schedule.driver", &driver],
    ));
    std::fs::write(dir.join(".gitattributes"), "*.mtsc merge=method-schedule\n").unwrap();
    let base = harbor();
    write(dir, &base);
    ok(git(dir, &["add", "-A"]));
    ok(git(dir, &["commit", "--quiet", "-m", "base"]));

    // Different fields on each branch: a clean merge with both.
    ok(git(dir, &["checkout", "--quiet", "-b", "theirs"]));
    let mut t = base.clone();
    t.sheets[1].synopsis = "A storm comes in.".into();
    t.boards[0].sub_boards[0].start = "2027-03-01".parse().ok();
    write(dir, &t);
    ok(git(dir, &["commit", "--quiet", "-am", "theirs"]));
    ok(git(dir, &["checkout", "--quiet", "main"]));
    let mut o = base.clone();
    o.sheets[0].set = "BOATHOUSE".into();
    write(dir, &o);
    ok(git(dir, &["commit", "--quiet", "-am", "ours"]));
    ok(git(dir, &["merge", "--quiet", "--no-edit", "theirs"]));
    let merged = read(dir);
    assert_eq!(merged.sheets[0].set, "BOATHOUSE");
    assert_eq!(merged.sheets[1].synopsis, "A storm comes in.");
    assert!(merged.boards[0].sub_boards[0].start.is_some());

    // The same field both ways: git stops with the file conflicted, and the file is
    // still a valid schedule, settled for ours.
    ok(git(dir, &["checkout", "--quiet", "-b", "theirs2"]));
    let mut t = merged.clone();
    t.sheets[0].set = "BARN".into();
    write(dir, &t);
    ok(git(dir, &["commit", "--quiet", "-am", "theirs2"]));
    ok(git(dir, &["checkout", "--quiet", "main"]));
    let mut o = merged.clone();
    o.sheets[0].set = "NET SHED".into();
    write(dir, &o);
    ok(git(dir, &["commit", "--quiet", "-am", "ours2"]));
    let m = git(dir, &["merge", "--no-edit", "theirs2"]);
    assert!(!m.status.success(), "the merge stops on the conflict");
    let err = String::from_utf8_lossy(&m.stderr).into_owned() + &String::from_utf8_lossy(&m.stdout);
    assert!(err.contains("film.mtsc"), "{err}");
    let status =
        String::from_utf8_lossy(&ok(git(dir, &["status", "--porcelain"])).stdout).into_owned();
    assert!(status.contains("UU film.mtsc"), "{status}");
    let settled = read(dir);
    assert_eq!(settled.sheets[0].set, "NET SHED");
    settled.check().unwrap();
}
