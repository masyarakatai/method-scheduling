#!/usr/bin/env python3
"""Parity tracker against the reference product (Movie Magic Scheduling).

Reads docs/parity/features.csv, checks it, and writes docs/parity/PARITY.md.

    python3 scripts/parity.py              # validate and write PARITY.md
    python3 scripts/parity.py --snapshot   # also record today's numbers in history.csv
    python3 scripts/parity.py --check      # exit 1 when PARITY.md is out of date

Scoring: have counts 1, partial 0.5, missing 0. Rows marked skip (status n/a) are left
out of every percentage. The MMS-checked percentage counts only rows whose feature the
reference app has (mms_checked n/a means it has none). v1 parity is over v1 rows; overall parity over v1 and later rows.
Standard library only.
"""

import argparse
import csv
import datetime
import io
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PARITY_DIR = ROOT / "docs" / "parity"
FEATURES = PARITY_DIR / "features.csv"
HISTORY = PARITY_DIR / "history.csv"
OUTPUT = PARITY_DIR / "PARITY.md"

COLUMNS = ["id", "area", "feature", "priority", "status", "mms_checked", "live_checked", "refs", "note"]
HISTORY_COLUMNS = [
    "date", "sha", "v1_pct", "overall_pct", "have", "partial", "missing", "mms_checked_pct", "live_checked_pct",
]

# Areas in inventory order (docs/research/mms-feature-inventory.md), with their headings.
AREAS = [
    ("files", "1. Files"),
    ("import", "2. Script import"),
    ("breakdown", "3. Breakdown sheets"),
    ("elements", "4. Categories and elements"),
    ("boards", "5. Boards and strips"),
    ("day-breaks", "6. Day breaks"),
    ("calendar", "7. Calendar"),
    ("events", "8. Events and conflicts"),
    ("red-flags", "9. Red flags"),
    ("dood", "10. Day out of days"),
    ("reports", "11. Reports"),
    ("exports", "12. Exports"),
    ("split-screen", "13. Split screen"),
    ("shortcuts", "14. Shortcuts"),
    ("saving", "15. Saving"),
    ("beyond", "Beyond: competitor features MMS lacks"),
]
AREA_TITLES = dict(AREAS)
PRIORITIES = ("v1", "later", "skip")
STATUSES = ("have", "partial", "missing", "n/a")
SCORE = {"have": 1.0, "partial": 0.5, "missing": 0.0}
ID_RE = re.compile(r"^[A-Z]{3}-\d{2}$")
FORBIDDEN = ("—", "!")  # house style: no em dashes, no exclamation marks
SPARK = "▁▂▃▄▅▆▇█"


class Invalid(Exception):
    pass


def read_features(path=FEATURES):
    text = path.read_text(encoding="utf-8")
    reader = csv.DictReader(io.StringIO(text))
    if reader.fieldnames != COLUMNS:
        raise Invalid(f"{path.name}: columns must be {','.join(COLUMNS)}, found {','.join(reader.fieldnames or [])}")
    rows, errors, seen = [], [], set()
    for n, row in enumerate(reader, start=2):
        where = f"{path.name} line {n}"
        if None in row or any(v is None for v in row.values()):
            errors.append(f"{where}: wrong number of fields")
            continue
        row = {k: v.strip() for k, v in row.items()}
        rid = row["id"]
        if not ID_RE.match(rid):
            errors.append(f"{where}: id {rid!r} is not like ABC-01")
        elif rid in seen:
            errors.append(f"{where}: id {rid} is used twice")
        seen.add(rid)
        if row["area"] not in AREA_TITLES:
            errors.append(f"{where}: unknown area {row['area']!r}")
        if not row["feature"]:
            errors.append(f"{where}: feature is empty")
        if row["priority"] not in PRIORITIES:
            errors.append(f"{where}: priority must be one of {', '.join(PRIORITIES)}")
        if row["status"] not in STATUSES:
            errors.append(f"{where}: status must be one of {', '.join(STATUSES)}")
        if (row["priority"] == "skip") != (row["status"] == "n/a"):
            errors.append(f"{where}: skip rows, and only they, have status n/a")
        if row["mms_checked"] not in ("yes", "no", "n/a"):
            errors.append(f"{where}: mms_checked must be yes, no or n/a")
        if row["live_checked"] not in ("yes", "no"):
            errors.append(f"{where}: live_checked must be yes or no")
        for col in COLUMNS:
            if any(ch in row[col] for ch in FORBIDDEN):
                errors.append(f"{where}: {col} has an em dash or an exclamation mark")
            if "|" in row[col]:
                errors.append(f"{where}: {col} has a | (it would break the tables)")
        rows.append(row)
    if errors:
        raise Invalid("\n".join(errors))
    return rows


def read_history(path=HISTORY):
    if not path.exists():
        return []
    with path.open(encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f)
        if reader.fieldnames != HISTORY_COLUMNS:
            raise Invalid(f"{path.name}: columns must be {','.join(HISTORY_COLUMNS)}")
        return list(reader)


def pct(num, den):
    return 100.0 * num / den if den else 0.0


def fmt(x):
    return f"{x:.1f}%"


def score(rows):
    return sum(SCORE[r["status"]] for r in rows)


def stats(rows):
    tracked = [r for r in rows if r["priority"] != "skip"]
    v1 = [r for r in tracked if r["priority"] == "v1"]
    # Only features the reference app has can be checked there (mms_checked n/a: it has none).
    in_mms = [r for r in tracked if r["mms_checked"] != "n/a"]
    counts = {s: sum(1 for r in rows if r["status"] == s) for s in STATUSES}
    return {
        "total": len(rows),
        "tracked": len(tracked),
        "v1_count": len(v1),
        "later_count": len(tracked) - len(v1),
        "skip_count": len(rows) - len(tracked),
        "v1_pct": pct(score(v1), len(v1)),
        "overall_pct": pct(score(tracked), len(tracked)),
        "counts": counts,
        "v1_counts": {s: sum(1 for r in v1 if r["status"] == s) for s in ("have", "partial", "missing")},
        "mms_count": len(in_mms),
        "mms_pct": pct(sum(r["mms_checked"] == "yes" for r in in_mms), len(in_mms)),
        "live_pct": pct(sum(r["live_checked"] == "yes" for r in tracked), len(tracked)),
    }


def sparkline(values):
    if not values:
        return ""
    return "".join(SPARK[min(len(SPARK) - 1, max(0, int(v / 100.0 * len(SPARK))))] for v in values)


def cell(text):
    return text if text else " "


def render(rows, history):
    s = stats(rows)
    c = s["counts"]
    out = []
    w = out.append
    w("# Parity with Movie Magic Scheduling")
    w("")
    w("Generated by `scripts/parity.py` from `features.csv`; edit the CSV, not this file.")
    w("How to update: [README.md](README.md). Features come from")
    w("`docs/research/mms-feature-inventory.md`, sections 1 to 15 plus the competitor list.")
    w("")
    w("## Headline")
    w("")
    w("| Measure | Value |")
    w("|---|---|")
    w(f"| v1 parity | **{fmt(s['v1_pct'])}** of {s['v1_count']} v1 features |")
    w(f"| Overall parity (v1 and later) | **{fmt(s['overall_pct'])}** of {s['tracked']} features |")
    w(f"| Have, partial, missing | {c['have']}, {c['partial']}, {c['missing']} |")
    vc = s["v1_counts"]
    w(f"| v1 have, partial, missing | {vc['have']}, {vc['partial']}, {vc['missing']} |")
    w(f"| Skipped (n/a, out of scope) | {s['skip_count']} |")
    w(f"| Checked in the real MMS | {fmt(s['mms_pct'])} of {s['mms_count']} tracked features the real MMS has |")
    w(f"| Checked live in our app | {fmt(s['live_pct'])} of tracked features |")
    w("")
    w("Have counts 1, partial 0.5, missing 0. Skip rows are left out of every percentage.")
    w("")

    w("## History")
    w("")
    if history:
        w("```")
        w(f"v1       {sparkline([float(h['v1_pct']) for h in history])}")
        w(f"overall  {sparkline([float(h['overall_pct']) for h in history])}")
        w("```")
        w("")
        w("| Date | Commit | v1 | Overall | Have | Partial | Missing | MMS checked | Live checked |")
        w("|---|---|---|---|---|---|---|---|---|")
        for h in history:
            w(
                f"| {h['date']} | `{h['sha']}` | {h['v1_pct']}% | {h['overall_pct']}% | {h['have']} | "
                f"{h['partial']} | {h['missing']} | {h['mms_checked_pct']}% | {h['live_checked_pct']}% |"
            )
    else:
        w("No snapshots yet: run `python3 scripts/parity.py --snapshot`.")
    w("")

    w("## By area")
    w("")
    w("| Area | Features | v1 | Overall | Have | Partial | Missing | Skip |")
    w("|---|---|---|---|---|---|---|---|")
    by_area = {a: [r for r in rows if r["area"] == a] for a, _ in AREAS}
    for area, title in AREAS:
        ar = by_area[area]
        if not ar:
            continue
        a = stats(ar)
        v1 = fmt(a["v1_pct"]) if a["v1_count"] else "-"
        ov = fmt(a["overall_pct"]) if a["tracked"] else "-"
        ac = a["counts"]
        w(
            f"| [{title}](#{anchor(title)}) | {len(ar)} | {v1} | {ov} | {ac['have']} | {ac['partial']} | "
            f"{ac['missing']} | {ac['n/a']} |"
        )
    w("")

    w("## What's left for v1")
    w("")
    left = [r for r in rows if r["priority"] == "v1" and r["status"] in ("missing", "partial")]
    if not left:
        w("Nothing: every v1 feature is in.")
    for area, title in AREAS:
        ar = [r for r in left if r["area"] == area]
        if not ar:
            continue
        w(f"**{title}**")
        w("")
        for r in sorted(ar, key=lambda r: (r["status"] != "missing", r["id"])):
            note = f" ({r['note']})" if r["note"] else ""
            w(f"- `{r['id']}` {r['status']}: {r['feature']}{note}")
        w("")

    w("## Features")
    w("")
    for area, title in AREAS:
        ar = by_area[area]
        if not ar:
            continue
        a = stats(ar)
        parts = []
        if a["v1_count"]:
            parts.append(f"v1 {fmt(a['v1_pct'])}")
        if a["tracked"]:
            parts.append(f"overall {fmt(a['overall_pct'])}")
        w(f"### {title}")
        w("")
        if parts:
            w(", ".join(parts).capitalize() + ".")
            w("")
        w("| ID | Feature | Priority | Status | MMS | Live | Refs | Note |")
        w("|---|---|---|---|---|---|---|---|")
        for r in ar:
            w(
                f"| {r['id']} | {r['feature']} | {r['priority']} | {r['status']} | {r['mms_checked']} | "
                f"{r['live_checked']} | {cell(r['refs'])} | {cell(r['note'])} |"
            )
        w("")
    return "\n".join(out).rstrip("\n") + "\n"


def anchor(title):
    a = title.lower()
    a = re.sub(r"[^a-z0-9 -]", "", a)
    return a.replace(" ", "-")


def git_sha():
    try:
        r = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, capture_output=True, text=True, check=True
        )
        return r.stdout.strip()
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def snapshot(rows, history, date):
    s = stats(rows)
    line = {
        "date": date,
        "sha": git_sha(),
        "v1_pct": f"{s['v1_pct']:.1f}",
        "overall_pct": f"{s['overall_pct']:.1f}",
        "have": str(s["counts"]["have"]),
        "partial": str(s["counts"]["partial"]),
        "missing": str(s["counts"]["missing"]),
        "mms_checked_pct": f"{s['mms_pct']:.1f}",
        "live_checked_pct": f"{s['live_pct']:.1f}",
    }
    # One line per date: a second snapshot the same day replaces the first.
    history = [h for h in history if h["date"] != date] + [line]
    history.sort(key=lambda h: h["date"])
    buf = io.StringIO()
    wr = csv.DictWriter(buf, fieldnames=HISTORY_COLUMNS, lineterminator="\n")
    wr.writeheader()
    wr.writerows(history)
    HISTORY.write_text(buf.getvalue(), encoding="utf-8")
    return history


def main(argv=None):
    p = argparse.ArgumentParser(description="Parity tracker against MMS: writes docs/parity/PARITY.md.")
    g = p.add_mutually_exclusive_group()
    g.add_argument("--snapshot", action="store_true", help="also append today's numbers to history.csv")
    g.add_argument("--check", action="store_true", help="exit 1 when PARITY.md is out of date with the CSV")
    p.add_argument("--date", help="snapshot date as YYYY-MM-DD (default: today)")
    args = p.parse_args(argv)

    try:
        rows = read_features()
        history = read_history()
    except Invalid as e:
        print(e, file=sys.stderr)
        return 2

    if args.check:
        want = render(rows, history)
        have = OUTPUT.read_text(encoding="utf-8") if OUTPUT.exists() else ""
        if want != have:
            print("docs/parity/PARITY.md is out of date: run python3 scripts/parity.py", file=sys.stderr)
            return 1
        print("docs/parity/PARITY.md is up to date.")
        return 0

    if args.snapshot:
        date = args.date or datetime.date.today().isoformat()
        try:
            datetime.date.fromisoformat(date)
        except ValueError:
            print(f"--date {date!r} is not YYYY-MM-DD", file=sys.stderr)
            return 2
        history = snapshot(rows, history, date)

    OUTPUT.write_text(render(rows, history), encoding="utf-8")
    s = stats(rows)
    print(
        f"v1 {fmt(s['v1_pct'])}, overall {fmt(s['overall_pct'])}; "
        f"have {s['counts']['have']}, partial {s['counts']['partial']}, missing {s['counts']['missing']}; "
        f"MMS checked {fmt(s['mms_pct'])}, live {fmt(s['live_pct'])}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
