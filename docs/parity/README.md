# Parity tracker

How to update: when a feature lands (or a check in the real app or a live run in our app
settles one), edit its row in `features.csv`, then run
`python3 scripts/parity.py --snapshot` and commit `features.csv`, `history.csv` and
`PARITY.md` together. Without `--snapshot` the script only rewrites `PARITY.md`;
`--check` exits non-zero when `PARITY.md` is out of date with the CSV (for CI later). A
second snapshot on the same date replaces that date's line. The commit in a history line
is the one the snapshot was taken on, so the change it records lands in the next commit.

## Files

- `features.csv`: one row per feature of the reference product, from
  `docs/research/mms-feature-inventory.md` sections 1 to 15 (a bullet holding clearly
  separate features is split) plus its "Competitor features MMS lacks" list (area
  `beyond`). Our own words throughout.
- `PARITY.md`: generated; never edit it by hand.
- `history.csv`: one line per snapshot (date, commit, v1 %, overall %, have, partial,
  missing, % checked in MMS, % checked live).

## Columns

| Column | Values |
|---|---|
| `id` | Area prefix and number (`BRK-03`). Stable: never renumber, never reuse a retired id |
| `area` | `files`, `import`, `breakdown`, `elements`, `boards`, `day-breaks`, `calendar`, `events`, `red-flags`, `dood`, `reports`, `exports`, `split-screen`, `shortcuts`, `saving`, `beyond` |
| `feature` | Short, in our words |
| `priority` | `v1`, `later` or `skip`, as the inventory tags it |
| `status` | `have`, `partial`, `missing`; `n/a` for every skip row and only them |
| `mms_checked` | `yes` when a behavior note from 14 on, or a result in `engine-checks.md`, says it was checked in the real app; `n/a` when the reference app has no such feature at all (our extras, the `beyond` rows), the note saying so and naming the check where there was one |
| `live_checked` | `yes` when a live run checked it in our real app window (docs/APP.md "Live checks", decisions written from the real window) |
| `refs` | Decision numbers (`D12`), behavior notes, engine check ids, other docs; separated by `; ` |
| `note` | What is missing, or why the status is what it is |

## Scoring rules

- Have counts 1, partial 0.5, missing 0. v1 parity is over v1 rows; overall parity is
  over v1 and later rows. Skip rows are out of every percentage.
- The MMS-checked percentage counts only the tracked rows whose feature the reference app
  has: rows with `mms_checked` `n/a` are left out of it, as there is nothing to check
  there. The live-checked percentage still counts every tracked row.
- **have**: in the app (or, for engine-only features such as DOOD rules, in the engine the
  app uses), confirmed in the code. **partial**: part of it, or only in the engine, the
  command line or MCP and not in the app, or deliberately different from the real app in
  a way still to change. **missing**: not started, including features we refuse on
  purpose (the note says so).
- A bullet the inventory repeats in two sections is one row: recent files (`FIL-08`),
  merging strips and sheets (`BRK-07`), custom DOOD codes (`DOD-16`), Fountain import
  (`IMP-07`), PDFs of the board and calendar (`BRD-20`, `CAL-09`).
- Be conservative: when in doubt, partial rather than have, and no rather than yes.
