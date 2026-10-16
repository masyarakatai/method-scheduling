# Behavior note 05: reports, files, exports, sharing and split screen (MMS 10 manual)

Source: public MMS 10 manual, Reports section (all twelve report pages), Schedule Files,
Templates, Cloud Backup, Exporting Schedule Data, Shared File, Split Screen, "Coming from
MMS 6" pages, plus archived beta pages on saving, closing, undo and multiple open
schedules. Read October 2026.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.
We describe what each report contains, never how it is laid out on the page.

## Reports (what each carries)

- All reports work on the selected board and sub-board, with an optional from-to date
  range that applies to every report in that sub-board.
- **One-line schedule**: one line per sheet, banner and day break, in board order. Uses the
  same layout library as the board. Options: one or two lines per strip, columns, category
  filter, page break per day, show only day breaks that have strips, strip colors,
  banners, shoot day strips, red flag icons. Multi-unit export (10.13): several sub-boards
  merged into one document by calendar date, each unit's strips headed by its name, each
  unit ending its own day; an element filter drops days with no matching scenes. Excel
  export since 10.7.53, including element quantities.
- **One-line vertical** (10.15a): the old vertical strip, with a cast ID grid where each
  cast member keeps a fixed row, keyed to a cover page listing cast against IDs.
- **Shooting schedule**: one-line strips plus, under each strip, the elements by category.
  Category filter for department versions.
- **Element list**: per category, each element's board ID, name and work days (and
  occurrences and pages per the overview video). Excel.
- **DOOD report**: the DOOD grid for one category, pages added for more days or elements.
  Can be limited to chosen elements.
- **DOOD summary**: per element totals (worked, held, travel, fitting and so on) without
  the grid. Excel.
- **Element detail**: per element, each scene it is in with date, scene, set and header
  info; a field listing all its work dates; optional image. Excel.
- **Breakdown report**: printable sheets, with categories under each. Group by strip, or by
  shoot day (one merged element block per category per day, quantities in brackets, the
  basis for call sheets). Order follows the sub-board. Excel.
- **Daily category total**: per shoot day, totals per category (headcounts for department
  heads), several categories in one run, cumulative view. Needs day breaks.
- **Calendar report**: the calendar with shoot days, off days, holidays, events, red flags,
  estimated day length. Needs day breaks.
- **Conflict report** and **red flag report**: see note 03.
- Report designer per report: name, duplicate, delete, save and load as a file, header
  layouts drawn from production info (title, company, key people), font size, lines, page
  breaks, portrait or landscape, show report name, show board and sub-board.
- Output: PDF and print for all; Excel for some (one-line, element list, DOOD summary,
  element detail, breakdown). The board itself exports to Excel or PDF, all or selected
  strips, with or without day breaks and banners. 10.11 says breakdown data exports to
  Excel "with formulas".
- MMS 6 printing (note 08): print view showed the board as on screen, with ranges by date
  or scene, and an option to include the Boneyard, banners and day breaks.

## Files

- Native schedule file (proprietary, closed to us). New schedules sit in a temporary state
  until first saved.
- Open MMS 6 files directly; they open unsaved, all strips in the Boneyard; saving writes a
  new file. With autosave on, opening one forces a save-as first.
- Templates: a separate file carrying categories, field setup, custom fields, report and
  strip designs, calendar strip design, red flag types, and the I/E and time of day values
  with their colors. A template clears scenes, strips, boards, sets, locations, episodes and
  red flags. New from template; double-click a template to start a schedule.
- Board layouts and report layouts save and load as their own files. A layout that refers
  to a missing category still loads, with a warning and that field empty.
- Cloud backup is on by default; versions can be restored from any machine. Shared files
  live in the vendor cloud with live co-editing and a change history tab (10.12).
- Several schedules can be open at once; unsaved ones show an asterisk; closing prompts
  save, discard or cancel. Recent files in the File menu (10.8.33).
- Autosave toggle (10.7.53). MMS 5 had an autosave reminder prompt at an interval instead.
- Undo and redo, back to when the file was opened (beta manual). Undo doesn't navigate to
  the change.
- No schedule password in MMS 10 (MMS 6 had one).

## Exports to other products

- To MMS 6 (native, active board and sub-board; warning about lost features).
- To the budgeting app: a library file per active sub-board with every category that has an
  element, and each element's work days. Proprietary. Our open path is the exchange file in
  the kickoff section 7.

## Split screen

- Breakdown with board (selecting a strip shows its sheet; edits show live).
- Board with board (two sub-boards or boards; drag between sub-boards of the same board).
- Calendar with board (drag strips onto calendar days).
- Advanced find and replace beside the board or breakdown: field-based finds (category,
  synopsis, set, location, D/N, episode, unit, and scene length and page ranges), equal or
  not equal, starts with and contains on set and location, AND/OR rows, matches highlighted,
  case option, preview then bulk replace, inline edits from the results. Replacing a pick
  list value with blank unlinks it instead of leaving an empty value.
- Board find: field plus text, highlight matches, step with arrows.

## What this means for Method Schedule

1. v1 reports: one-line, shooting schedule, DOOD, DOOD summary, element list, breakdown,
   calendar; PDF from the app; CSV and xlsx for one-line, DOOD, DOOD summary and element
   list. Our own designs. [missing]
2. Templates as plain files of our own format (a schedule with no sheets). [missing] later.
3. Merge-by-date multi-unit one-line. later.
4. No cloud; sharing is a file plus diff and merge (kickoff). Cloud backup: skip.
5. Split screen: breakdown with board in v1 (it is how ADs work); the rest later.
