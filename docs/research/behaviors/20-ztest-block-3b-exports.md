# Behavior note 20: screen block 3b, report exports, Excel and PDF (real app)

Source: our own checks in the real reference app, 2026-10-12, a 20-minute full-screen
block on `ZTEST Lantern Creek`. The two exported files were saved as
`~/ZTEST/ZTEST Lantern Creek_oneline.xlsx` and `.pdf` and read as ordinary Excel and PDF
files (open formats, the app's output, not its native files).
Clean-room: behavior in our own words, no copied text, no screenshots kept, the files stay
outside the repo. This note outranks notes 01 to 19 wherever they disagree.

## Export per report (A25)

| Report | Export offers |
|---|---|
| One-line | PDF, Excel |
| Breakdown | PDF, Excel |
| Shooting schedule | PDF only |
| Calendar | PDF only |
| Conflicts | nothing while there are no conflicts (the menu is disabled) |

The board's toolbar has its own Export with PDF and Excel, and an icon to save the board
layout to a file (and one beside it, presumably to load one).

## The date range belongs to each report (E-R7)

- The one-line's range was Mar 1 to Apr 7; switching to the Calendar report changed the
  range to Feb 26 to Apr 7, so the calendar's range reaches back to an event before the
  first shoot day. The range is the report's own, worked out from what it shows, as ours.

## The one-line Excel file (E-R10, E-R12)

- One worksheet. A header row, then one row per strip in board order.
- **The columns follow the strip layout's fields**, in the layout's order, after a first
  column for day break and banner text: scene number, I/E, set, time of day, script day,
  cast IDs, page eighths, unit, synopsis, location, shoot time.
- Day breaks are rows with their text spread over cells: the day's end, its date, the
  page total and the estimated time, each as text.
- **Every cell is text**, page lengths included (`1/8`, `00:00`). **No formulas** anywhere.

## The PDF (E-R13)

- Two pages for 25 strips, real selectable text.
- Document properties: a fixed generic title, and the system's PDF library as creator and
  producer. No author, subject or keywords.

## What this changes

- E-R7 settled: per report (ours already).
- E-R12: the reference app's one-line Excel has no formulas; our working totals (decision
  95) go beyond it and stay.
- E-R13: ours sets more than the reference app (decision 96); nothing to change.
- E-R10: our one-line Excel keeps fixed columns plus category columns (decision 93); the
  reference app follows the strip layout. To change: the one-line export follows the
  report's strip layout by default.
