# Screen block 3: parity tour (runbook)

Purpose: fill the parity tracker's `mms_checked` and `live_checked` columns
(`docs/parity/features.csv`). Part A watches the reference app do each v1 feature not yet
checked there; Part B checks our app in its real window for everything shipped since
block 2. One long unattended block, owner approved, screen lock taken
(`~/.claude/coordination/screen-lock.json`).

Rules: throwaway `ZTEST` schedules only (`ZTEST Lantern Creek`, `ZTEST Block1`, or a new
`ZTEST Block3` made from `scripts/make-ztest-fdx.py`). Never open a schedule that isn't
ZTEST, never use a non-ZTEST Open Recent entry, never inspect native files. Leave the
reference app on its start screen at the end. Results go in behavior note 19 (Part A,
our own words) and docs/APP.md "Live checks" (Part B), then the CSV.

Pace: full-screen batches; wait a second between clicks on the board (two quick clicks
open the sheet); right-click menus for strip actions; Escape before switching tabs.

## Order for a short block (after note 19)

Note 19 (a background block, 2026-10-12) settled the menus, preferences, production info,
the report and header designers, the breakdown and calendar designers and the DOOD sort.
With limited time, run in this order and stop when time is up:

1. A25: report Export per report (Reports list needs a real click), open the saved one-line
   and DOOD summary Excel files (formulas or values), PDF properties in Preview, image
   types in the Add dialog, the board toolbar's download and print icons.
2. A21 and E-R14: the date range per report or per sub-board; the day break and total
   strip choices and the element category picker in a report's design.
3. A9, A12, A23: strip Delete, Move To, day break insert and Remove All, a parked day
   break, strip colors, print the board.
4. B1, B2, B6b: our app beside the reference app at the same size (density), the right-click
   menu, two windows.
5. A18: key equivalents, read from each menu with the menu open (not readable in the
   background).
6. Everything else in Part A, then Part B row by row.

## Part A: the reference app

| # | Rows | Do | Record |
|---|---|---|---|
| A1 | FIL-08, FIL-09, SAV-01, SAV-02 | File menu: recent list; make an edit, Undo and Redo several steps; New Schedule then close it unsaved | Undo depth; what close asks; autosave toggle place |
| A2 | BRK-01, BRK-02 | Breakdown: type into every header field; pick an I/E and a time of day value, add a new one, check it on another sheet | Which fields check their format; pick lists shared |
| A3 | BRK-04, BRK-09 | The planned date beside the slugline: click it; Previous and Next order | Does it jump to the board; order is board order |
| A4 | BRK-05, BRK-06, BRK-08 | New sheet with a strip selected on the board, and with the Boneyard showing; duplicate a sheet (if offered); delete a sheet | Where new and duplicated strips land; scene number of a duplicate; the confirmation |
| A5 | BRK-11 to BRK-14 | Category boxes: hide empty; add by clicking in a box; quick add an unknown name; select two elements and cut or copy them to another sheet | Each behavior |
| A6 | ELM-02, ELM-04, ELM-07, ELM-13, ELM-12 | New schedule's categories (count and kinds only, do not copy the list); a category's settings; custom properties on a category and an element; element notes | What exists; where it is edited |
| A7 | ELM-09, ELM-15, ELM-16, ELM-21 | Elements tab: the header-field "categories" (sets, locations); the table's columns; sort by a column; an element's days and dates view | Columns and counts shown |
| A8 | BRD-01 to BRD-05 | Board menu: new, duplicate, rename, reorder, delete boards and sub-boards; a new strip's place; drag between sub-boards and to the Boneyard | Defaults and refusals |
| A9 | BRD-07, BRD-08, KEY-04 | Move To from the context menu; Delete Strip(s) on a scene; the Delete key on a selected scene | What each asks and deletes |
| A10 | BRD-09, BRD-12, BRD-13 | Sort with three keys; board find; strip colors (where set, per I/E and time of day) | Options shown |
| A11 | BRD-14, BRD-19, BRD-20 | Strip designer: add and remove a field, two lines; double-click a strip; print the board | Print options offered |
| A12 | DBK-04 to DBK-06, DBK-09 | Move and delete a day break; drag one into the Boneyard; insert above and below; remove all breaks (look in menus) | Each behavior |
| A13 | CAL-03, CAL-04, CAL-09 | Preferences for week start and date format; month view; print the calendar | What exists |
| A14 | EVT-02, EVT-05 | Create an event from an element and from a sheet; drag an event on the calendar; two sub-boards dated the same week sharing an actor | Where events start; the conflict display |
| A15 | DOD-01, DOD-09, DOD-12, DOD-14, DOD-15 | DOOD by board, sub-board and category; sort rows; edit hold and drop from a row; working week change follows | Each behavior |
| A16 | RPT-01 to RPT-05, RPT-07, RPT-13, EXP-01 | Reports list: one-line, shooting schedule, DOOD and summary, element list, breakdown by strip, calendar; report options (date range, header, toggles); save one as PDF to `~/ZTEST` | Options per report |
| A17 | SPL-01, SPL-05 | Split screen with the breakdown beside the board; find and replace on a text field | Layout and options |
| A18 | KEY-01, KEY-02, KEY-05 | Every menu's key equivalents (read the menus; list the command and key in our words); a shortcut sheet in Help. Gaps to settle (decision 79): the keys for Insert Day Break (older manuals: Cmd comma; ours Cmd D, as Cmd comma is Settings), Preferences or Settings (older: Cmd semicolon), find sheet (older Cmd G; ours is Find Next), quick entry (older Cmd E; ours is Export), strip layouts (older Cmd M; the Mac's Minimize), the breakdown window (older Cmd Y), board and calendar steps (older Cmd 2, 3, 5, 7 to 0; ours are the tabs), first and last sheet, the split screen's keys, and whether Close is Cmd W | Full key table for E-K1 |
| A20 | E-L2, E-L10 to E-L13 | Linked elements: link B to A and C to B, add A to a sheet (does C come); the retroactive choice; a link across categories. A category left off the board: is it in the DOOD. Daily quantity: an element with no quantity, two sheets on one day, where a partial total shows. Header values differing in case (`Kitchen`, `KITCHEN`): one row or two. Which categories can't be renamed or deleted | Each result |
| A21 | E-R7 to E-R9, E-D24, E-D25, E-C11 | Report date range: per report or per sub-board. Breakdown by day: what the number in brackets counts. Vertical one-line: cover page, cast row order. Custom DOOD letters in the summary export. DOOD sort keys and when a sort clears. Calendar: where a strip dropped on a day lands; does the filter hide or dim; are week start and date format per app or per schedule | Each result |
| A22 | E-I5, E-I6 | Import a revised version of the ZTEST script into a schedule that has tags and board places: what changes, what is kept. (E-I5, how a tagged `.fdx` links a word to its category, needs a tagged `.fdx` from an invented script; ask the owner whether Final Draft is available) | Each result |
| A23 | E-S23 to E-S28, E-K2, E-B14 to E-B17, E-B5, E-B10, E-S5, E-C6, E-S22 | Delete on a scene strip (asks? every board?). Insert Daybreak key and where it lands. Auto breaks by screen time; does Remove All ask. A day break parked in the Boneyard: how it looks. Breakdown header date: does it jump to the strip. Strip colors: per value or per I/E and time of day pair; per schedule or per board. Print the board: options and look. Show and hide on the board: saved per board. Banner colors and heights. Resource counts offered on strips. Day break extras and the shoot day strip's place and wording. A scene spanning days: how its length and days count. The default two-line layout's fields; boards pick layouts by name; the one-line's own layout | Each result |
| A24 | E-R3, E-S29 to E-S33, E-E14, E-I7 | Report designer: merge two cells, set one field's size (one row or stacked; steps or points). Sheet number: free text or a number; step by sheet and by scene number (where empty ones go). Templates: what one keeps and clears. Header field order: per schedule or per user; can a field hide. Advanced find: operators per field type, AND and OR, no text. Split screen: which panes, drag between them, per user or per schedule. Red flag types: built-in ones, colors, reorder, delete one in use. A scene the revised script cut (decision 91): does it still count pages, time and work | Each result |
| A25 | EXP-09 to EXP-12, IMP-09, EXP-01 to EXP-03, IMP-01 | Import and export list (docs/research/interop-mms-open-formats.md section 5): File > Import (or Import Script), the file types its dialog allows (Fountain, PDF, text, other writing apps?), and any element, cast or CSV import in File or the Elements tab; File > Export, every entry; on the Reports tab, each report's export choices (one-line, vertical one-line, shooting schedule, DOOD, DOOD summary, element list, element detail, breakdown by strip and by day, daily category total, calendar, conflict, red flag); save the one-line and by-day breakdown as Excel to `~/ZTEST` and open them (columns, quantities, formulas or values); a saved PDF's properties in Preview's Inspector; the Board tab's export and print icons; the Calendar tab's export and print icons; the red flag view's export; copy strips and paste into TextEdit; the image types Element > Images > Add allows; any email or share command | Full list of formats per menu; one-line Excel columns; formulas or values; PDF properties; clipboard text or none; image types |
| A19 | Open checks | E-D20 summary holiday column; E-L8 renumber skipping a locked number; banner with nothing selected; paste on another board; Shift-click order for merge | Each result |

## Part B: our app in its real window

Build the app and wrap it in the bundle as in block 2. Open `~/ZTEST/ZTEST-Lantern.mtsc`
and `~/ZTEST/big.mtsc`.

| # | Do | Pass when |
|---|---|---|
| B1 | Breakdown tab at the reference app's window size: count fields and category boxes in view | At least as many as the reference app |
| B2 | Board at the same size: strip lines in view; right-click a strip, every item; Shift+F10 | At least as many lines; every item works and undoes |
| B3 | Typed dates (`3/1/2027`, `2027-03-01`, a bad one) in the start date, an event and a red flag | Accepted or refused in one sentence |
| B4 | Status messages during copy, paste and refusals | Nothing below moves |
| B5 | Every feature landed tonight (decisions 54 to 91), tab by tab in the table below, on `ZTEST-Lantern.mtsc` | Each works in the real window as in the harness, and undoes in one step |
| B6 | Print and PDF of the board, the calendar and each new report | Pages and headers right |
| B6a | File > New Window and Open in New Window with a second schedule; edit both; Window menu (or macOS's); Close Window (Cmd W) on the edited one; open the first file again from Open Recent (decision 86). Split screen with the breakdown, board, calendar and images panes; drag the bar (decision 80) | Each window keeps its own undo and autosave; close asks; the open file comes to the front; the panes follow the selection |
| B6b | Two windows live (decision 88): with two schedules open, choose menu commands (Undo, Save, a tab, Find) with each window in front in turn; edit both, then Quit and answer Don't Save on the first and Cancel on the second, then keep editing the first (wait 10 s); Quit again and save both; with an import running (Import Script, a large script) click the window's close button twice | Each menu command acts on the window in front only; after Cancel the first window's changes are still autosaved or kept as a recovery copy and nothing was thrown away; one question per window, never two; no hang or spinning cursor anywhere |
| B7 | Quit | No lock file in Recovery |

### B5 by tab

One pass, left to right through the tabs, then the menus. Rows are the CSV rows to mark
`live_checked`.

| Where | Check | Rows |
|---|---|---|
| Board, strips | Delete and Backspace on a scene (asks, sheet gone from every board); Move To; Insert Day Break Below (Cmd comma, B) and Above (Shift Cmd comma); park a day break in the Boneyard; Continue Scene on Next Day (parts 2/3, pages on the first) | BRD-08, KEY-04, DBK-05, DBK-06, DBK-11 |
| Board, sub-board header | Break by pages, shoot time and screen time; Remove Breaks and Board > Remove All Day Breaks | DBK-08, DBK-09 |
| Board menu | Banner dialog (bold, nine colors, two lines, text size, alignment); Strip Colors; Show on Board; Day Breaks (extras, the day's quantities, shoot day strip); strip designer (four lines, merged cells, cast and element counts); Layouts (library, Save to File, Load from File); Print Board and Save Board as PDF | BRD-11, BRD-13, BRD-17, DBK-02, DBK-03, BRD-15, BRD-18, BRD-16, FIL-05, BRD-20 |
| Breakdown | Planned date link jumps to the strip; Header Field Order; step by board order, sheet number, scene number; sheet number field and Go to Sheet Number; quantities with the daily switch; scene images; decimal time; Category;Name quick add; conflict and red flag marks | BRK-04, BRK-03, BRK-10, BRK-17, BRK-15, BRK-16, BRK-18, EVT-06, RFL-02 |
| Elements | Default list of 18 after Cast; exclude from board; daily quantity switch; properties per category and values per element; notes; images; Events, Conflicts and Red flags columns and column sort; linked elements (retroactive); days and dates panel; header field rows; New Event from an element | ELM-02, ELM-04, ELM-06, ELM-07, ELM-13, ELM-12, ELM-14, ELM-16, ELM-20, ELM-21, ELM-09, EVT-02 |
| Day out of days | Heading sort and Sort dialog (four keys); H and D boxes; Codes dialog; vertical view; without holds | DOD-12, DOD-13, DOD-14, DOD-16, DOD-10, BYD-12 |
| Calendar | Week start and date format from Settings (date titles follow); strips, pages, Go To, Today, First Shoot Day; drag a strip and an event; filter; Strip Fields; Print and PDF; prep and wrap; unavailable and camera test events | CAL-03, CAL-05, CAL-06, EVT-02, CAL-07, CAL-08, CAL-09, CAL-10, EVT-03 |
| Reports | Breakdown by strip and by day (sum, partial marked); calendar; element detail; vertical one-line; conflict and red flag reports; multi-unit; options (date range, header, category filter, colors, text size); report designer (merged cells, size, bold, italic); Up, Down, named copies; xlsx of detail, breakdown, board | RPT-05, RPT-06, RPT-07, RPT-08, RPT-10, RPT-11, RPT-12, RPT-13, RPT-14, RPT-15, RPT-16, EXP-03 |
| Red flags | A color per type on every mark; table filter and PDF | RFL-01, RFL-03 |
| Plan menu | Import Script with Tags; revise a script (a cut scene stays struck and adds no pages, time or work to its day); Export Call Sheet Data; Day Timeline; Locations (sun); Turnarounds and Rest; Schedule Locks (drift); Suggest Order (preview, one undo) | IMP-04, BYD-01, BYD-02, BYD-04, BYD-08, BYD-09, BYD-05, BYD-06 |
| File, View, Help | Save as Template and New from Template; split panes (breakdown, board, calendar, images; Single View, Shift Cmd 0); advanced find rows; keys (Cmd J, Cmd L, Cmd K); Help > Keyboard Shortcuts (Cmd /); windows are B6a and B6b | FIL-04, SPL-01 to SPL-04, SPL-07, SPL-06, KEY-02, KEY-05, FIL-03 |
