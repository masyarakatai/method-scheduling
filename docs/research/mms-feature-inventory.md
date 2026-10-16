# Movie Magic Scheduling: feature inventory (clean-room research)

Status: research notes, October 2026. Covers Movie Magic Scheduling 10 (MMS 10, current
release 10.15a, Sept 2026) and, where useful, the retired MMS 6 and older MMS 4 and 5.
Nothing has been checked in the real app yet; every rule here waits on the checks in
`engine-checks.md`.

Clean-room note: everything below is a functional description in our own words from public
sources (the vendor's public manual and release notes, public support pages, old public
manuals, public tutorial videos read through their caption tracks, third-party guides,
competitor help pages, reviews and forum snippets). No help text, labels beyond a few-word
industry term, report layouts, screenshots, icons, sample data, templates or bundled lists
(including the default category list) were copied. Nothing here describes the inside of any
native vendor file (`.mmsx`, `.msd`, `.sex`, `.mbl`, template and layout files), and that
topic was deliberately not researched. Raw fetched pages and captions sit in the gitignored
`imports/research-cache/`.

Version rule: when the current manual (MMS 10) and older material disagree, the current
manual wins, and the real app beats both. Each section says where they disagree.

Priority key used in every section:

- **v1**: needed for a credible first release, a scheduler can switch over.
- **later**: valuable, can follow v1.
- **skip**: out of scope (cloud, vendor ecosystem) or legally off limits.

Behavior notes referenced as "note NN" live in `behaviors/`.

---

## Summary

MMS is a desktop app (Mac and Windows) for breaking down a script and building a shooting
schedule. Its core:

1. **Breakdown sheets**: one per scene or scene part, with a header (scene numbers, I/E,
   set, time of day, pages in whole pages and eighths, shoot time, screen time, script day,
   location, unit, sequence, episode, synopsis, comments, sunrise and sunset) and elements
   listed by category.
2. **Categories and elements**: cast, background, props and so on. Elements carry a board
   ID (cast numbers), lockable and renumberable, DOOD switches (allow holds, allow drops,
   minimum drop gap), links to other elements, notes, custom properties, images.
3. **Boards**: each board is one arrangement of every strip (scenarios). A board has
   sub-boards (units, episode blocks, phases), each with its own start date and calendar,
   plus a Boneyard for unscheduled strips. Banners carry notes and durations.
4. **Day breaks** end shoot days and show day number, date, page and time totals; they can
   be placed by hand or automatically by max eighths, shoot time or screen time.
5. **Calendar per sub-board**: working week, off days, holidays, company travel days,
   events (travel, fitting, rehearsal, unavailable and others) tied to elements.
6. **Day out of days**: per sub-board and category, a code per element per day (SW, W, WF,
   SWF, H, WD, PW, TR, FT, R, holiday, move), with exact counting rules and separate totals.
7. **Conflicts and red flags**: same element on the same date in two sub-boards;
   availability flags and clashes.
8. **Reports**: one-line (and a vertical variant), shooting schedule, element list, DOOD,
   DOOD summary, element detail, breakdown (by strip or by day), daily category totals,
   calendar, conflict, red flag. PDF for all, Excel for several.
9. **Files**: a closed native format, MMS 6 import and export, templates and layouts as
   files, cloud backup and live shared files tied to vendor accounts.
10. **Split screen** views and an advanced find and replace.

The main pains users raise: subscription only with mandatory sign-in, MMS 10 seen as a step
back from MMS 6 for some, no call sheets, shallow script import, no revision merge,
collaboration tied to paid seats, no running clock on strips, no auto scheduling.

---

## 1. Files

- Native schedule file, closed format. New schedules are temporary until first saved. **skip**
  (we have our own open format).
- Open MMS 6 files: opens unsaved, every strip in the Boneyard; categories, element
  properties, I/E and time of day values, red flags (one per day) come across; quantities
  embedded in names stay in the name. **skip** (native file; off limits). Owner can export
  open formats from MMS instead.
- Several schedules open at once, a switcher, asterisk for unsaved. **later** (one window per
  file is enough for v1).
- Templates: a file with categories, fields and custom fields, report and strip designs,
  calendar strip design, red flag types, I/E and time of day values with colors; clears
  scenes, strips, boards, sets, locations, episodes and red flags. New from template;
  double-click opens a new schedule. **later** (v1 ships one built-in default of our own;
  user templates are a schedule file with no sheets).
- Board and report layouts saved and loaded as files; loading tolerates missing categories.
  **later**.
- Cloud backup (on by default), shared files with live co-editing and change history.
  **skip** (local stand-ins: rolling backups, diff and merge).
- No schedule password in MMS 10 (MMS 6 had one). **skip**.
- Recent files. **v1**.
- Undo and redo for the session (back to file open). **v1**.

Disagreement: MMS 6 stored layouts inside the schedule and had one per schedule (MMS 10
beta also); MMS 10 now has many plus layout files. Current wins.

Sources: manual Schedule Files, Templates, Cloud Backup, Shared File, Coming from MMS 6
pages; archived beta pages on saving and closing; release notes 10.8.33, 10.12, 10.14.14;
notes 05, 06.

## 2. Script import

- File import of Final Draft `.fdx`, the vendor screenwriter's files, and a vendor tagged
  export (`.sex`). **v1** for `.fdx`; **skip** the vendor formats.
- From a plain script: scene headings and speaking characters only. **v1** (our default).
- From a tagged export: tagged elements too, matched to existing elements by name, unknown
  ones created, unknown categories created. **later** as an opt-in reader of FDX production
  tags (structure still unknown, see `interop-formats.md` section 2).
- New sheets go to the end of the Boneyard in every board; a count is shown. **v1**.
- Not documented: heading split into I/E, set and time of day; scene numbers; eighths from
  the file; re-import behavior. Engine checks E-I1 to E-I4.
- Fountain is not supported by MMS. Our import of it is an extra, on by default since it is a
  format, not a behavior change. **v1**.
- MMS 6 tutorials show the import leaving a blank first sheet behind. **v1** (don't).

Sources: manual Importing Breakdown Sheets; overview video; MMS 6 tutorials; notes 04, 07,
08, 11.

## 3. Breakdown sheets

- Header fields: scenes (text, lists, ranges, lettered parts), I/E (pick list with user
  values), set, time of day (pick list with user values), sunrise and sunset (24-hour),
  pages (whole plus eighths; more than 8 eighths rolls over), shoot time (meant as h:mm,
  not enforced), synopsis, location, script pages, script day, screen time (meant as m:ss,
  not enforced), unit, sequence, episode, comments. All optional. **v1** (times as integer
  minutes and seconds; raw text kept only as an import fallback).
- Pick list fields share values across sheets; one value per sheet. **v1**.
- Header field order configurable. **later**.
- Planned date shown beside the slugline, jump to strip. **v1**.
- New sheet after the current one in the active sub-board, elsewhere at the end of the
  Boneyard; duplicate (to split a scene); merge; delete. **v1**.
- Step through sheets in active sub-board order (MMS 5 also by sheet number and scene
  number). **v1** for board order, **later** for the others.
- Elements by category on the sheet, add by clicking under a category or by typing; hide
  empty categories. **v1**.
- Quick add field: type, pick from matches, Enter; new name asks for a category; focus stays.
  **v1**.
- Copy, cut, paste elements between sheets; multi-select. **v1**.
- Element quantity per sheet, "use in daily quantity" per sheet, per-category switch to hide
  quantities (cast off by default), asterisk on partial totals. **later**.
- Images on scenes. **later** (sidecar folder, not inside the file).
- MMS 6: sheet number field (an index), colon or decimal time display, semicolon suffix
  creates an element without a prompt. **later** for decimal display and the semicolon.

Sources: manual Breakdown pages and Scene Header Information; release notes 10.6, 10.7,
10.13, 10.15a; notes 04, 08, 09, 11.

## 4. Categories and elements

- Built-in default categories (ours will be our own list, informed by the open category id
  proposal in `interop-formats.md` section 4, not the vendor's). Built-ins can't be
  reordered; user categories can. Names unique. **v1**.
- Category settings: auto board ID for new elements, exclude from board, default holds,
  default drops and minimum gap, use in daily quantity, custom element properties. **v1**
  for all but daily quantity (**later**).
- Category defaults apply to new elements only (MMS 10). MMS 5 had "apply to existing" per
  setting. We default to MMS 10 and add an explicit apply-to-existing action. **v1**.
  Engine check E-D19.
- Header fields (set, location, unit, episode) appear as built-in categories in the elements
  tab and the DOOD, with page and day counts. **v1**.
- Element: name (unique in category), board ID (alphanumeric allowed, unique in category),
  lock, category, DOOD switches, linked elements, notes, custom properties, images. **v1**
  except images (**later**).
- Elements table: ID, name, occurrences, pages, work days, start, finish, events, conflicts;
  sort by column or custom drag order. **v1**.
- Renumber unlocked elements from a chosen start in custom order, skipping locked numbers;
  alphanumeric IDs untouched. **v1**. Engine check E-L3.
- Merge two or more elements, choose the survivor. **v1**.
- Move to another category; custom properties are lost (manual) where the overview video
  said they were kept. Manual wins. **v1**.
- Linked elements: adding the main element adds the linked ones; series scope by episode;
  retroactive option; deleting a link stops future linking only; set and location elements
  can link to anything. **later**. Engine check E-L2.
- Days and dates tab (an element's shoot days and dates on the active sub-board). **v1**
  (cheap once the DOOD exists).

Sources: manual Elements, Categories, Linking Elements, Images pages; release notes 10.8.33,
10.13a, 10.15a; MMS 5 manual; notes 04, 07, 08, 09, 10.

## 5. Boards and strips

- Boards: many per schedule, each a full arrangement; duplicate, rename, reorder, delete
  (sheets survive). **v1**.
- Sub-boards: many per board, each with a start date and off days; no duplicate; delete
  (strips go to the Boneyard, per the beta manual). **v1**.
- Boneyard per board; unscheduled; start date ignored; new boards start with everything
  there. **v1**.
- New strips land after the selection in the active sub-board, at the end of the Boneyard
  elsewhere. **v1**.
- Move strips: drag, cut and paste (below the selection), copy and paste (all strip types),
  move to another sub-board. **v1**. Engine check E-S2.
- Merge strips into one sheet. **later**. Engine check E-S3.
- Delete strip deletes the sheet everywhere, with confirmation. **v1**.
- Sort by up to three header fields: banners to the top, day breaks removed, ascending.
  Episode sorts numerically. **v1**; scene number sort rule is engine check E-S4.
- Banners: text, duration (adds to the day's time), font, size, alignment, color, thin or
  standard height; above or below the selection; copy and paste. **v1** for text and
  duration, **later** for styling.
- Board find (field plus text, highlight, step). **v1**.
- Strip colors by I/E and time of day, font color per strip type. **v1**.
- Strip designer: field picker (scene, I/E, set, elements or ID list of a category, custom
  text, script day, pages, script page, eighths, location, unit, shoot time, screen time,
  spacer, episode, sequence, date, day, D/N, quantity, sunrise, sunset, comments), up to four
  rows, merged cells, scale to content, alignment, line styles. **v1** for a fixed layout
  with a field picker; **later** for the full designer and multiple layouts.
- Shared layout library between board and one-line report. **later**.
- Show or hide total strips, banners, shoot day strips. **later**.
- Resource counts on strips (10.12). **later**.
- Double-click a strip to open its sheet. **v1**.
- Print or PDF the board, all or selected strips. **v1** (PDF from the app).

Disagreements: MMS 5 had the Boneyard as a separate window and MMS 4 called it the
unscheduled area; MMS 6 had one calendar object per board; current model wins.

Sources: manual Board section; release notes 10.8.20, 10.12, 10.13, 10.14.14, 10.15.8;
notes 02, 07, 08, 09.

## 6. Day breaks

- Day break ends a shoot day: day number (count in the sub-board), date (from the
  calendar), total pages and estimated time of strips above it; optional sunrise (earliest
  of the day), sunset (latest), screen time, custom text, spacer, element quantity. **v1**
  for number, date, pages, time; **later** for the rest.
- Shoot day strip above each break, linked, showing the count and full date (10.13). **later**.
- Day breaks can move, be deleted, and be parked in the Boneyard (then no number). **v1**.
- Insert above or below the selection. **v1**.
- Automatic: by max eighths, max shoot time or max screen time, or remove all. Applying
  removes all existing breaks first and covers the whole sub-board. A strip larger than the
  max gets its own day (beta manual). **v1**. Engine checks E-B1 to E-B6.
- Banner durations count in the day's time. **v1**. Engine check E-B3 for auto by time.
- MMS 5 and the 2006 manual: "allow multiple day breaks" per board, for scenes spanning
  days. Not in MMS 10's manual. **later** pending E-B5.
- MMS 6 also auto-placed by time per day or pages per day; same idea, current wins.

Sources: manual Day Breaks and Strip Display; archived beta Day Breaks page; release notes
10.13, 10.14.14; notes 02, 07, 08, 10, 11.

## 7. Calendar

- One calendar per sub-board: start date, working week (per sub-board), per-date marks:
  off day, holiday, company travel, back to working. **v1**.
- Strips can't land on off days, holidays or company travel days; day breaks reflow to the
  next working date. **v1**. Engine checks E-C1, E-C2, E-C5.
- App-wide week start day and date format. **v1** (date format as a preference; files store
  ISO dates).
- Month view with strips, scenes and pages per day, estimated day length; go to date,
  today, first and last scheduled. **v1** (a simple month view), **later** for the extras.
- Drag strips (and in MMS 6 whole days) between dates. **later**.
- Filter by category and elements (session only). **later**.
- Calendar strip designer. **later**.
- Print and PDF, weekdays only or full week. **v1** (PDF).
- MMS 5 and 6: named calendars as separate objects, with prep start, production start,
  end of production and end of post dates; only production start mattered. Current per
  sub-board model wins. Prep and wrap dates could be an opt-in for the budget exchange.
  **later**.

Sources: manual Calendars section; release notes 10.7.99, 10.10.7, 10.12, 10.15.8; notes 03,
07, 08, 10.

## 8. Events and conflicts

- Events: name (optional), date range, type (none, travel, fitting, unavailable, rehearsal,
  camera test), color, elements; tied to the sub-board; created from the calendar, an
  element, or a sheet; draggable; default to the start date without day breaks. **v1**.
- Travel, fitting, rehearsal, unavailable show on the DOOD; none and camera test don't. **v1**.
  Engine check E-D9 for unavailable.
- Conflicts: the same active element in strips on the same date in two or more sub-boards
  of one board. Warnings only. Shown on a global list, the calendar day, the strip, the DOOD
  cell, the elements table. Only sub-boards with day breaks. **v1** (detection and a list);
  **later** for every icon placement. Engine checks E-E1 to E-E3.
- Events never cause conflicts (current manual). The 2026 overview video (older footage)
  said overlapping events did. Manual wins. Engine check E-E2.

Sources: manual Events, Conflict Handling; notes 03, 07.

## 9. Red flags

- Availability flags per element over dates, typed (user list, reorderable; built-in DOOD
  types can't be deleted), with notes and colors; global across boards; several per day.
  **later** (data shape in v1 format).
- Clash marker when a flagged element is scheduled on a flagged date: board strip,
  breakdown, calendar (scheduled-only view and list view), DOOD, elements, reports. **later**.
- Red flag manager table, filter by element, PDF. **later**.
- History: MMS 5 introduced red flags; its built-in DOOD flag types were how travel,
  fitting and rehearsal reached the DOOD (printing them was off by default). MMS 10 first
  turned these into events, then brought red flags back in May 2025 and split them from
  conflicts in July 2026. Current model wins.

Sources: manual Red Flag section and What Changed; release notes 10.9.14, 10.14.14,
10.15.8, 10.15a; red flag tips videos; notes 03, 09.

## 10. Day out of days

The core of the engine. Full rules in note 01; industry practice in note 12; checks E-D1 to
E-D19.

- Per board, sub-board and category (including set, location, unit, episode). **v1**.
- Codes: SW, W, WF, SWF, H, D (as WD, SWD), P (as PW, PWF), TR, FT, R, holiday, move; blank;
  grey off day. **v1**.
- Holds only when allowed: every non-working day strictly between first and last work,
  except company off days. **v1**.
- Drops only when holds and drops are allowed, with a minimum gap in calendar days; placed
  automatically in the first gap that fits; drop days uncounted. **v1**. Engine checks E-D1
  to E-D4 (gap measure, one drop or many, placement).
- Travel, fitting, rehearsal: shown and counted as worked regardless of switches, even
  outside the work span; they act as work when looking for a drop gap. **v1**.
- Holiday and move: shown and counted as worked only when holds are allowed, the element
  has work, the day is inside the work span and not inside a drop. **v1**.
- Start and finish come from strips only; events don't move them. **v1**.
- Totals kept apart: worked, held, travel, fitting, rehearsal, holiday, move; summary
  report shows them. **v1**.
- Weeks Sunday to Saturday across; one row per element; vertical and horizontal views.
  **v1** for one view, **later** for the other.
- Default order "scene order"; session-only multi-key sort (board ID, name, first, last,
  worked, held), stable, blanks last, vertical only. **v1** for basic sort, **later** for
  multi-key. Engine check E-D14.
- Edit hold and drop switches from the DOOD row. **v1**.
- Working week shared with the calendar. **v1**.
- MMS 5: DOOD code letters were configurable; a grand total of work plus hold plus company
  travel with holiday separate; events (flags) before day 1 widened the range. **later**
  for custom letters; totals follow MMS 10 (engine check E-D7); range widening pending
  E-D10.
- Extra: "explain this count" for any cell or total. **later** (phase 3).

Sources: manual Day-Out-of-Days and DOOD Rules; DOOD Report and Summary pages; release notes
beta 1.4.17, 10.5.40, 10.13, 10.13a; MMS 5 manual; hold days and red flag videos; notes 01,
07, 09, 10, 12.

## 11. Reports

What each report carries is in note 05. All of ours will be our own designs.

- One-line schedule. **v1**.
- Shooting schedule (strips with elements by category, category filter). **v1**.
- DOOD report and DOOD summary. **v1**.
- Element list. **v1**.
- Breakdown report (by strip; by day as a call sheet base). **v1** by strip, **later** by
  day.
- Calendar report. **v1** (PDF of the calendar view).
- Element detail. **later**.
- Daily category total. **later**.
- One-line vertical with a cast ID grid. **later**.
- Conflict and red flag reports. **later**.
- Multi-unit one-line merged by date, element filter. **later**.
- Report options: date range, category filter, page break per day, show or hide banners,
  day breaks, shoot day strips, colors, icons; portrait or landscape; production info header;
  multiple header layouts. **v1** for date range, production info header and the basic
  toggles; **later** for the rest.
- Report designer (drag fields, merge cells, fonts). **later**.
- Report order in the list, custom named copies. **later**.

Sources: manual Reports section; release notes 10.4.22 to 10.15a; overview video; notes 05,
06, 07.

## 12. Exports

- PDF for every report, the board and the calendar. **v1** (from the app).
- Excel for one-line, element list, DOOD summary, element detail, breakdown, and the board.
  **v1** for one-line, DOOD, DOOD summary and element list as xlsx and CSV (our own small
  writer), **later** for the rest.
- Export to MMS 6 native. **skip**.
- Budget library for the vendor's budgeting app (categories with work days of the active
  sub-board). **skip** (closed); our open `.days.json` exchange instead (kickoff section 7).
  **later**.
- Open interop formats MMS lacks: our open file, Fountain in, USS in and out. See
  `interop-formats.md`. **v1** for our file and Fountain; **later** for USS.

Sources: manual Exporting Schedule Data, Reports, Board Print and Download; notes 05,
`interop-formats.md`.

## 13. Split screen

- Breakdown with board (select a strip, see its sheet; live edits). **v1**.
- Board with board (two sub-boards or boards side by side; drag between sub-boards of one
  board). **later**.
- Calendar with board (drag strips to dates). **later**.
- Images panes. **later**.
- Advanced find and replace: field-based finds with equal, not equal, starts with,
  contains; AND and OR rows; scene length and page ranges; highlight; preview and bulk
  replace; inline edits; blank replace on pick list fields unlinks the value. **later**
  (simple find and replace on text fields in v1).
- One-click collapse back to single view. **later**.

Sources: manual Split Screen section; release notes 10.7, 10.11, 10.12, 10.13; note 05.

## 14. Shortcuts

- MMS 10 keeps MMS 6 shortcuts and lists them in Help. Documented in the current manual:
  new, open, save, save as, close, undo, redo, banner (Cmd B), cut, copy, paste, open shared.
  **v1** (default key map), engine check E-K1 for the full list.
- MMS 5 table (note 10): sheet window, quick entry, stripboard, layouts, find, find sheet,
  find scene, print, preferences, first, next, previous and last sheet, element linking,
  red flag entry, new and edit element, insert day break (Cmd comma), insert banner,
  Boneyard toggle, previous and next layout, board and calendar. **v1** where the action
  exists in our app.
- Right-click menu on strips (cut, copy, paste, delete, move to, merge, insert day break or
  banner above or below). **v1**.
- Delete or Backspace deletes selected strips or banners with confirmation. **v1**.
- A shortcut sheet in Help. **v1**.

Sources: manual pages naming keys; release notes 10.6, 10.7.99; MMS 5 manual; navigation
shortcuts video; notes 09, 10.

## 15. Saving

- Explicit save and save as; temporary until first saved; close prompts save, discard or
  cancel. **v1**.
- Autosave toggle (10.7.53; a rapid-edit miss fixed in 10.14.14). MMS 5 only had a timed
  reminder prompt. **v1** (autosave into the file by default with a toggle, kickoff
  section 6).
- Opening an MMS 6 file with autosave on forces save as. **skip** (no native import).
- Cloud backup and versions. **skip**; local rolling backups and crash recovery instead.
  **v1**.
- Recent files. **v1**.

Sources: manual Schedule Files; archived saving and closing pages; MMS 5 manual; notes 05,
10.

---

## Gaps and pain points

Gathered from forums (search snippets only; Reddit blocks fetchers), reviews, vendor support
pages and competitors' comparison pages. Details and sources in note 13.

1. **Subscription only** since MMS 10, priced in US dollars, with academic and multi-seat
   plans; MMS 6 is end of life (updates stopped July 2025, support Dec 2025). Hard in weaker
   currencies.
2. **Mandatory sign-in and multi-factor authentication**, cloud backup by default; shared
   files need a paid seat per collaborator; no read-only viewer.
3. **MMS 10 felt like a step back** to some MMS 6 users. Much of the 2026 release work is
   labeled MMS 6 parity (templates, layouts, daily quantities, vertical one-line, breakdown
   by day), which agrees.
4. **No call sheets.** ADs copy data into spreadsheet templates or other tools.
5. **Shallow script import** without pre-tagging; **no revision merge** for a new draft.
6. **No running clock** on strips (call time plus durations).
7. **No automatic scheduling** or order suggestions.
8. **Stability and support** complaints (crashes, a stale lock blocking launch).
9. **Steep learning curve**, many unexplained rules (the DOOD counting rules are a good
   example: correct but hard to read from the screen).
10. **Documentation drift**: the 2026 overview video still shows pre-2024 names and older
    conflict rules; the manual's archive holds beta pages with rules the current pages
    dropped (auto day break oversize rule). Users learn from out-of-date material.
11. **Session-only DOOD sort**, layouts that only recently became files, and templates that
    only arrived in July 2026.

Our answers: local first, no account, free, open file format with diff and merge, read-only
PDF and open exports for crew; then opt-in extras below.

## Competitor features MMS lacks (or does weakly)

Concepts only; details and sources in note 13.

- **Script revision merge** keeping tags and scene identity, draft compare (Filmustage,
  StudioBinder sync). **later**, high value.
- **Call sheets** generated from the schedule with distribution and confirmations
  (StudioBinder, SetHero, Yamdu, Gorilla add-on). For us: call sheet data export. **later**.
  Distribution: **skip** (needs a service).
- **Running timeline** from call time, day length and sunset warnings, push the tail of a
  long day to the next (CineSched, Call-Sheet-Tool, Storiara, Dramatify). **later**.
- **Schedule lock and drift report** per actor (CineSched). **later** (falls out of diff).
- **Auto fill or optimize order** (Dramatify, Filmustage AI, an open research project on
  travel cost). **later**, opt-in.
- **AI breakdown** (Filmustage, Gorilla). **skip** for now; maybe an opt-in local model
  later.
- **Sunrise and sunset by location** (Fuzzlecheck). **later**, opt-in, no network by default.
- **Rest period and cast conflict checks** (Fuzzlecheck). **later**.
- **Excel backup on every save**, restore from Excel (Gorilla). Ours: open JSON already, plus
  rolling backups. **v1**.
- **Export to MMS** formats (Gorilla, Filmustage). **skip** (native formats).
- **DOOD without holds option** (CineSched); custom codes (MMS 5). **later**.

## Open questions for the owner

1. **Drop rule**: one drop per element (our reading) or as many as fit? Run E-D2 first; it
   changes cast totals more than anything else.
2. **Drop gap default**: what minimum do you use in practice (7, 10, other), and should our
   built-in category default be off, as MMS seems to be?
3. **Worked total**: do line producers you work with expect MMS 10's worked total
   (including qualifying holidays and move days) or MMS 6's numbers? E-D7 will show what
   MMS 10 really does; which do you want in the budget exchange?
4. **Union rules**: do you want holds and drops checked against actor union consecutive
   employment rules (a warning layer), or just MMS's mechanical rules? (Default: MMS rules
   only.)
5. ~~"Extreme days"~~: dropped 2026-10-10. The kickoff guessed the owner might mention it;
   the owner never did and no source uses the term.
6. **Multiple day breaks** for a scene that runs over two days: do you use this, and how did
   MMS 6 show it on the DOOD?
7. **Fountain pagination**: 55 or 56 lines a page as our default (after E-I2)?
8. **Tagged `.fdx`**: can you make one from an invented script in Final Draft so we can see
   production tags? Only if you are fine with that.
9. **USS**: OK to support the open schedule standard as an interop format, given its
   unclear license wording?
10. **Default categories and colors**: our own list (informed by the open category id
    proposal), or do you want to define them? Strip colors follow industry convention; any
    house preferences?
11. **Call sheets**: in or out of scope for phase 3? A data export only, or a designed
    sheet?
12. **Running clock and day length warnings**: wanted as an opt-in extra?
13. **Time display**: h:mm by default with decimal hours as an option; does anyone on your
    shows use decimal?
14. **Scene sort**: natural order (2 before 10, 10 before 10A) as an opt-in, or default if
    MMS turns out to sort plainly (E-S4)?
15. **Recording session**: the wishlist in `engine-checks.md` is ordered by engine risk.
    Is that order fine, and when can the ZTEST checks run (you at the screen, or a computer
    use session with your OK)?
