# Behavior note 22: screen block 3d, calendar, events, split views, boards (real app)

Source: our own checks in the real reference app, 2026-10-12, an unattended full-screen
block on `ZTEST Lantern Creek`, plus our app in its real window. No edits were kept.
Clean-room: behavior in our own words, no copied text, no screenshots kept.
This note outranks notes 01 to 21 wherever they disagree.

## Breakdown (A2, A4, E-S17)

- Shoot time and screen time are two small boxes each (hours and minutes, minutes and
  seconds). Location, unit, sequence and script day are pick lists. A box beside the
  sheet shows its planned weekday, date and shoot day.
- The New Sheet button has a menu: Duplicate Sheet, Merge Sheets, Delete Sheet.
- **Duplicate Sheet** puts the copy right after the original, in the same day, with the
  same scene number, selected. As ours.

## Boards (A8, E-S12, E-S13, BRD-17)

- The board picker lists the boards with an edit button each, then Add Board and
  Reorder. Editing a board renames it and offers Duplicate Board and Delete Board; delete
  is off while it is the only board. As ours.
- The board's View menu turns total strips, banners and shoot day strips on and off.

## Split views (SPL-01 to SPL-07)

- A menu at the top right: single screen, advanced find and replace, board beside
  breakdown, board beside board, calendar beside board, board beside images, and
  **breakdown beside images** (ours lacks this one).
- In board beside breakdown each half has its own title bar; the breakdown reflows its
  fields and shows the category boxes in fewer columns.

## Calendar (CAL-04 to CAL-07, A13, A14)

- One long view of weeks that scrolls through the schedule, not one month at a time.
  Holidays carry a star, company travel days a plane.
- Above it: board and sub-board, category and element filters, the start date, a go to
  date field, reset, export and print.
- **An Actions menu for the selected date**: working day, off day, company travel,
  holiday, add an event, add a red flag, set as the start date.
- **A View menu of switches** (all or none): shoot day and off day labels, page numbers,
  estimated time, scenes per day, events, board strips, banners, red flags, and more.

## Events (EVT-01 to EVT-03)

- The add event dialog: an optional name, from and to dates, a type, **a color** from ten,
  and **any number of elements** (optional).
- Types: travel, fitting and rehearsal (marked as counting in the day out of days),
  unavailable, camera test, and **a list of post-production kinds** (post-production in
  general, visual effects, dubbing, sync sound, music recording, picture editing, sound
  editing, sound mixing, titles and graphics, trailer, and a few more).

## Day out of days and elements (DOD-14, ELM-21)

- Editing a row opens the element's editor on its day out of days rules (hold, drop and
  pickup, minimum gap). Ours edits holds and drops on the row itself, one step shorter.
- The element editor's days and dates tab lists the shoot day numbers and their dates.

## Strip colors (BRD-13)

- A table of every I/E and time of day pair, each with **its own strip color and its own
  text color**. Ours gives each pair a strip color but sets text color per strip type.

## Our app in its real window (live)

- Keys: Cmd period (Breakdown), Cmd U (Board), Shift Cmd D (Calendar), Cmd 3, 4, 6.
- Typed dates: `3/1/27` refused in one sentence; `2/15/2027` accepted and shown back as
  `2027-02-15` once the field loses focus.
- Two windows: a banner added in the new window leaves the first untouched; Cmd W asks
  Save, Don't Save or Cancel; Quit asks per window; Don't Save on one and Cancel on the
  other keeps both open, and the first one's recovery copy still follows later edits.
- Elements: header-field rows, category settings, the events, conflicts and red flag
  columns, and the element editor with leave off the board, notes, links and images.
- Day out of days: hold and drop boxes per row, Sort, Without holds, Vertical, Codes.
- Reports: the report list and the option panel. Plan menu: the day timeline opens on the
  selected day.

## Found in our app

- An inline error line on the Calendar (a refused date) pushes the controls below it
  down, so the next click lands on the wrong control (as the status line did before
  decision 48).
- A new window opens exactly on top of the first; it should be offset.
- The report date range fields hint `M/D/YYYY` while the app shows dates as `2027-03-01`.
- The day timeline says "No scene location is on the map", which is unclear (it means no
  location has coordinates).
