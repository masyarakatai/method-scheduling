# Behavior note 27: screen block 4 (real app)

Source: our own checks in the real reference app, 2026-10-13, on `ZTEST Lantern Creek`,
plus our app in its real window. Every change was undone. Clean-room: behavior in our own
words, no copied text, no screenshots kept. This note outranks notes 01 to 26.

## Reference app

- **Move To the Boneyard** (E-S2, BRD-07): the strip goes to the top of the Boneyard (a
  second one moved lands above the first); the selection moves to the next strip where
  it was. Same as a sub-board (note 24) and as ours (decision 129).
- **New Banner with several strips selected** (E-B12, BRD-10): the banner goes above the
  bottommost selected strip, whichever was clicked last.
- **Paste with nothing to paste on** (E-S19): in an empty Boneyard, Command V pastes the
  copied strip there, selected.
- **A new board** (BRD-01): starts with an empty Default sub-board dated today; every
  sheet goes to its Boneyard, in sheet order.
- **Paste on another board** (E-S21): switching boards empties the strip clipboard; Paste
  is greyed on the other board. Copying again on the new board turns Paste back on there.
  The selected sheet stays selected across boards.
- **Red flag dates** (E-E17): a click picks one date alone; Command-click adds a date, or
  removes it when picked; **Shift-click adds** every date from the last date picked to
  this one, keeping the rest of the pick (Command is not needed). A Command-click that
  removes a date does not move the starting point of the next Shift-click. The small
  calendar opened on the month of the last date picked, not the shoot's.
- **Element Quantity on a strip** (E-S34, BRD-18): the field's picker lists **categories
  only** (its search finds category names, not elements); one category per field, a new
  pick replaces the old. The strip shows the category's name and its count on the strip,
  as `Cast Members (4)`. The red flag and conflict switches sit at the top of the
  designer beside the layout's name: they belong to the layout.
- **Spreadsheets and report options** (RPT-03, D59): with the one-line report's range
  starting on day 3, its Excel export still held every day from day 1. The workbook
  ignores the report's date range, as ours.
- **The DOOD report and the weekends** (DOD-19, D121): with Saturdays and Sundays hidden
  in the DOOD tab's View menu, the DOOD report leaves them out too (Friday is followed by
  Monday); shown, they print as OFF columns. The report pages by whole weeks from the
  week's first day: a range from Monday March 1 started at Sunday February 28.
- **The vertical one-line and an element off the board** (RPT-20, D128): with ADA set to
  exclude from board, the vertical one-line's cast list still had ADA's row.
- **Event kinds** (E-E3, EVT-03): the Add Event dialog's type list is None, Travel,
  Fitting, Unavailable, Rehearsal, Camera Test, then post kinds: Post-Production, VFX,
  Dubbing, Sync Sound, Music Recording, Picture Editing, Sound Editing, Sound Mixing,
  Titles Credit and Graphics, Trailer, Color Correction. Only Travel, Fitting and
  Rehearsal are marked as DOOD kinds. The dialog's dates start on the shoot's first day,
  not the day picked on the calendar.
- **An event at a drop's edge** (E-D22, DOD-04): ADA, drops allowed (minimum 7), worked
  day 1 (SWD) and came back on March 16 (PW). A travel event on March 15 (the day before
  the return) showed **TRHP** and March 16 became W: the event day takes the pickup, with
  H. A travel event on March 2 (the day after day 1) showed **TRHD** and day 1 became SW:
  the event day takes the drop. Each event day counted as worked (4 to 5 to 6). The
  event acts as a work day for the drop, and its code keeps the H it has inside the
  element's span (`TRH`, `FTH`), with the D or P after it.
- **A company travel day on a weekend** (E-D23, DOD-06): company travel on Saturday March
  6 showed **M** for each element held across it (BRUNO, CLEO, DARIO), and each one's
  worked total went up by one; elements not held then had nothing.
- **DOOD sort** (E-D25, DOD-13): sorting by held days, 9 to 1, ties kept board order (EVE
  before FINN, both 0). **Leaving the DOOD tab and coming back cleared the sort**, and the
  Sort dialog opened with empty rules.
- **The calendar's filter** (E-C11, CAL-07): the category list holds only the categories
  in use and the element list has a check box per element under its category, with All;
  several can be checked. With one element checked, strips without it are hidden (the
  day headers and every event stay).
- **Held days in conflicts** (E-E5, EVT-05): a copy of scene 1 on a second sub-board
  starting March 2. The conflicts list named EVE (working on both that day) and the
  header values DAY and INT, but not CLEO or DARIO, who were only held on March 2 in the
  first sub-board: **held days do not conflict**, as ours.
- **Moving into a category that numbers automatically** (E-L7, ELM-19): with Props set
  to auto-assign board IDs, FINN (no ID) moved from the cast to Props **kept no ID**. The
  category setting is a default for new elements only (note 24).
- **Which categories can go** (E-L5): Props' settings have Delete Category; the cast
  category's have none.
- **The default strip layout** (E-S22): two lines. First: scene number, I/E, set, day or
  night, script day, the cast IDs (`ID: Cast Members`), page eighths, unit. Second:
  synopsis, location, shoot time.
- **The Elements table** (ELM-16): board ID, name, occurrences, pages, days, start,
  finish, conflicts, red flag, each column sortable from its header; no events column.
- **Open Recent** (FIL-08) lists the last five schedules opened.
- **Find and replace** (SPL-05): the Edit menu has only Advanced Find and Replace; there is
  no separate simple one.
- **Reports** (RPT-06, EXP-10): the New Report menu offers breakdown, calendar,
  conflicts, daily category total, DOOD, DOOD summary, element detail, element list,
  one-line, one-line vertical, red flags and shooting schedule. The breakdown report's
  designer has Group By, strip or day; it exports as PDF or Excel. No report merges
  several units by date (RPT-12 not found).
- **A new board's start date** (BRD-01): today, as a new sub-board's (note 24); its first
  sub-board is named Default.

## Our app in its real window

Checked on `ZTEST-Lantern.mtsc` (restored from a copy afterwards) with the build from
before tonight's fixes. Passed: banners (insert, edit, color, alignment, two lines),
strip colors with automatic text color, the strip layout (two lines, element count),
named layouts and layout files, Show on Board, day break extras, Continue Scene on Next
Day, printing and the board PDF (title property set), Boards and Sub-boards (duplicate,
rename, reorder, delete, add), Delete on strips and day breaks, Insert Day Break Above,
parking a day break, Remove Breaks, stepping by board order and by scene number, sheet
number, decimal time, Quick add's `Category;Name`, adding in a category box,
quantities, New, Duplicate and Delete Sheet, categories (add, rename, refused duplicate,
automatic numbering, defaults, custom properties, Apply to Existing asking first),
board ID lock and Renumber, linked elements with "every sheet", merge, move, days and
dates, the DOOD grid (weekends, holds and drops, sort), calendar extras, events (kinds,
travel counted as worked, an event on a work day listed), red flags and their clashes,
every report's preview, report copies, production info, header layouts, the report
designer, exports (formulas in the DOOD summary), the split views, the shortcut sheet,
Open Recent, templates, script import (Final Draft and Fountain), script revision
preview, Suggest Strip Order, and simple and advanced find.

Found and fixed the same night (decisions 139 to 166): the calendar's drag did nothing
in the Tauri window; the breakdown by day printed no scene rows; the shoot day strip sat
at the end of the day; the conflict mark vanished on light strip colors; a large
tooltip covered the board; Escape did not close some dialogs; the first click after
renaming a board was lost; undoing New Sheet showed another sheet; a new board had no
start date; banners had ten colors; header values kept the typed spelling with no list;
Quick add made a duplicate for a name in another category and never asked for a
category; the revision preview named sheets by internal id; Color correction was
missing; the Find panel clipped its labels; text fields offered WebKit's saved values.
These fixes were built but the display locked before they could be seen in the window;
they are queued for the next block (ROADMAP M1).

## Screen block 5: tonight's fixes seen in the real window (2026-10-14)

Built from main at 1980c02 (decisions 139 to 166), on `ZTEST-Lantern.mtsc`, restored
from a copy afterwards. Every item passed:

- The shoot day strip heads each day ("Shoot day 1 of 25, Monday, March 1, 2027").
- The conflict mark sits on a red badge and reads on a yellow INT DAY strip.
- No tooltip over the board.
- New Banner with scenes 2 and 4 selected went above scene 4; the 49-swatch palette.
- Boards and Sub-boards: the first Up click after a rename acts; Escape closes it.
- Calendar: scene 4 dragged from day 4 to day 5 landed at the end of day 5, day 4 left
  empty; the elements filter with BRUNO and FINN checked showed scenes 4, 5 and 7; the
  calendar strip fields (pages added, elements removed); Save as PDF, two pages.
- Breakdown by day prints each day's scenes.
- DOOD: Saturdays and Sundays switch apart, the DOOD report follows each; a sort by name
  descending cleared after leaving the tab.
- Red flag dates: click 1, Shift-click 5, Command-click 3, Shift-click 10 picked nine.
- Header values: typing `room 2` listed ROOM 2 and the like, and Tab adopted `ROOM 2`.
- Quick add: an unknown name asks for the category (Escape keeps the text); `ada` adds
  the existing ADA.
- Undo of New Sheet returns to the sheet shown before.
- Element count with Cast picked shows `Cast (4)`, `Cast (1)`.
- The shortcut sheet shows no autofill list and closes on Escape; the Find panel reads
  "All rows" and "contains" in full.
- New from Template with Autosave off: the template is picked first; cancelling it asks
  nothing and keeps the edits.
- Element images: TIFF and GIF thumbnails draw; the Add dialog takes every image type
  but PDF, one file at a time.
- Call sheet data export (day 4, BRUNO in scene 4); Schedule Locks with Drift; Turnarounds
  (12 hours, days without a call time named); Locations with degrees and offset, and the
  Day Timeline's sunrise 06:18 and sunset 17:51 for March 4.

Noted, small:

- A day break's sunrise and sunset come from the times typed on its sheets only; a
  location's worked-out times show in the Day Timeline and the call sheet, not on the
  board's day break.
- The calendar's element filter list stays open after Escape (a click outside closes
  it).
- Find and Replace's replace box cuts its placeholder text.
- The Locations dialog's Close button took a second try; Escape closed it.
- The call sheet export's save dialog opened in the folder last used for images.
