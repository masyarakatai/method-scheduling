# Behavior note 10: MMS 5 manual (2009) and EP Scheduling manual (about 2006)

Source: MMS 5 user manual as OCR text on archive.org
(`archive.org/stream/manualzilla-id-5897802`), and the EP Scheduling user manual PDF
(about 2006, 139 pages) hosted at assistantdirectors.com. Both read October 2026; raw copies
in the gitignored `imports/research-cache/`. Old editions: anything in notes 01 to 07 wins.
The OCR is noisy; table rows below were reconstructed and may be incomplete.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

## DOOD (both manuals agree)

- Per element switches: allow hold days, allow drop and pick-up days, minimum days between
  drop and pick-up. Same three switches as MMS 10.
- Category defaults for the same three, each with its own "apply to existing elements" box
  (MMS 5).
- Element manager columns: occurrences, start date, finish date, total days, company
  travel, work, hold, holiday, travel, rehearsal, fitting. The same split we plan to keep.
- DOOD totals (MMS 5): holiday is counted between the element's start and finish; the
  grand total is work plus hold plus company travel. Holiday is a separate column and not
  in the total there. MMS 10 says qualifying holidays and travel days add to worked
  (note 01). Conflict; MMS 10 wins until E-D7 is run.
- DOOD preferences (MMS 5): every code letter could be changed (start, work, hold, drop,
  pick-up, finish, holiday, company travel, travel, rehearsal, fitting), one letter each;
  skip names after the first page; whether DOOD-type flags print; header and footer
  templates. MMS 10 has no code customization in the manual. Our default is MMS's letters;
  custom letters could be an opt-in.

## Day breaks

- Day break text showed day number, date, pages and time estimate, dates from the
  production start date of the board's calendar.
- Auto day breaks: one prompt, the maximum pages per day; breaks fill the whole board.
  The 2006 manual says "pages or strips" for the max, a hint that a strip-count mode
  existed. MMS 10 has eighths, shoot time and screen time.
- **Allow multiple day breaks** (a per-stripboard switch in both manuals): lets several day
  breaks follow one set of strips so a scene that takes more than one day shows on more
  than one day. Not mentioned in the MMS 10 manual. Engine check E-B5.

## Calendar

- Calendars are separate named objects: days of the week off, special days (company travel,
  holiday, day off, none), production dates (prep start, production start, end of
  production, end of post) in order. Production start drives all dates.
- A stripboard picks one calendar.

## Preferences worth knowing

- Autosave reminder interval (or inactive) instead of real autosave.
- Where new strips go: after the active strip, end of scheduled area, or end of Boneyard.
- Unknown element name typed: create with or without a warning.
- Sheet numbering automatic or manual. Time shown with colon or decimal.
- Hide board IDs on the breakdown sheet. Add new elements to the sheet automatically. Going
  past the last sheet creates a new one.
- Linked elements: insert automatically (or ask), remove after confirmation.
- Schedule password (removed in MMS 10).

## Shortcuts (MMS 5 table, Mac shown, Windows uses Ctrl)

Recorded so our default key map can match long-time users. Several numbers in the OCR were
garbled; these are the readable ones.

- Open, new from template, save, close: Cmd O, N, S, Shift W.
- Breakdown sheet window Cmd Y, element quick entry Cmd E, stripboard Cmd U, strip layout
  manager Cmd M, find Cmd F, find sheet Cmd G, find scene Cmd J, print Cmd P, preferences
  Cmd semicolon, DOOD preferences Cmd Shift semicolon.
- Sheets: first, next, previous, last with Cmd plus number or arrow keys.
- Element linking Cmd L, red flag entry Cmd K, new element Cmd B, edit element Cmd K (in the
  element manager).
- Board: insert day break Cmd comma, insert banner Cmd B, cut and paste strips, Boneyard
  toggle Cmd 5 (per video), previous and next strip design Cmd 7 and 8, previous and next
  board Cmd 9 and 0, previous and next calendar Cmd 2 and 3.
- MMS 10 says it keeps MMS 6 shortcuts and lists them in Help; current documented ones:
  Cmd N, O, S, Shift S, W, Z, Shift Z, B (banner), X, C, V, Shift O (open shared). Engine
  check E-K1: dump the in-app shortcut list once.

## What this means for Method Schedule

1. Keep work, hold, holiday, company travel, travel, rehearsal, fitting as separate counts;
   compute totals two ways only if the owner wants an MMS 5 style total. v1.
2. Code letters configurable later; default to MMS letters.
3. A "multiple day breaks" behavior if E-B5 shows MMS 10 still supports it.
4. Default key map from the MMS 10 Help list, cross-checked with this table. v1.
