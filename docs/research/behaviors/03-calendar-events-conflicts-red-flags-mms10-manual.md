# Behavior note 03: calendar, events, conflicts and red flags (MMS 10 manual)

Source: public MMS 10 manual, Calendars section (calendar, navigating, events, conflict
handling, filtering, building a schedule), Red Flag section, the "Coming from MMS 6" page on
what changed, and release notes 10.7 to 10.15a; archived beta calendar pages from the same
search index. Read October 2026.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

## Calendar basics

- The calendar shows one sub-board of one board at a time. Picking a sub-board there also
  switches it in every other tab.
- Each sub-board has its own **start date**: the first day any scheduled strip in it can
  be worked. Set from the calendar (right-click a day, or a start field).
- Day 1's date is the start date (overview video). Each later day break takes the next
  working day.
- **Working week**: the days of the week that are working days, set per sub-board from the
  calendar's settings. The week start day (Sunday or Monday) and the date format are
  app-wide preferences. The DOOD shares the working week.
- Each day can be marked: working day, off day, holiday, company travel ("co travel"), and
  events can be added. "Working day" cancels an off day, holiday or travel mark.
- **Off days**, **holidays** and **company travel days** all block strips: no shoot day can
  land on them. Strips reflow around them (the next day break moves to the next working
  day). The overview video shows a day break skipping a travel day.
- Holidays and travel days differ only in the DOOD (/ and M codes, note 01).
- Repeating off days (for example every Saturday and Sunday) come from the working week;
  single dates can be flipped back to working.
- In MMS 5 and 6 a calendar was its own object (several per schedule, each with days off
  and production dates) and a stripboard pointed at one calendar. MMS 10 folds this into
  each sub-board. MMS 10 wins.

## Viewing and building on the calendar

- Month view, scroll or step by month, jump to first or last scheduled strip, go to date,
  today.
- View toggles: strips, scenes and pages per day, estimated day length (from the day break),
  red flags, red flag clashes, conflicts.
- Calendar strip designer for what a strip shows on a day.
- Filter by category and elements: only strips holding those elements show. Filters are per
  sub-board and last until the schedule closes.
- Drag a strip to another day to reschedule it. Whole days could be dragged in MMS 6.
  Calendar and board can sit side by side and strips drag from board to a calendar day.
- Print and PDF, weekdays only or full week. Calendar report needs day breaks.

## Events

- An event has: optional name (defaults to its type), a date range, a type, a color, and
  zero or more elements.
- Types: none (general), travel, fitting, unavailable, rehearsal, camera test.
  Travel, fitting, unavailable and rehearsal show on the DOOD; none and camera test do not.
- Events belong to the sub-board where they were created.
- If the sub-board has no day breaks yet, a new event defaults to the start date.
- Events can be dragged to other days (10.8.33). Several on one day collapse into a
  "more" list.
- Events can be created from an element's edit window and from the breakdown sheet.
- Company holidays and travel days do not affect events.
- The 2026 overview video calls events what MMS 6 called red flags. That matches MMS 5 and
  6, where the built-in "DOOD" red flag types (travel, fitting, rehearsal) were what put
  TR, FT and R on the DOOD (note 09). Current MMS 10 has both events and red flags again.

## Conflicts

- A conflict is one active element in strips scheduled on the same shooting date in two or
  more sub-boards of the same board. That is the whole definition in the current manual.
- Events never cause conflicts (current manual). The overview video (recorded on an older
  build, it still says "segment") said overlapping events also flagged a conflict. Newer
  manual wins.
- Conflicts are warnings only. Nothing blocks the schedule.
- Shown on: a global list for the current board, the calendar day, the strip, the DOOD cell,
  the elements table. Clicking explains why.
- Only sub-boards with day breaks can produce conflicts (conflict report page).
- "Active element" is not defined. Probably elements not excluded from the board, or
  elements on scheduled strips. Engine check E-E3.

## Red flags

- A red flag marks an element unavailable (or constrained) on a date or range. Types are a
  user list (add, delete, reorder); some built-in "DOOD" types can't be deleted.
- Red flags are global across boards. Several flags per element per day are allowed (MMS 6
  allowed one).
- A **red flag clash**: a flagged element is in a strip scheduled on a flagged date. Shown
  on the board strip, breakdown sheet, calendar, DOOD, elements tab and in reports.
- Two calendar views: the standard one only marks dates where the flagged element is
  actually scheduled (MMS 6 behavior), and a list view marks every flagged date.
- When a clash and a conflict hit the same strip, both markers show.
- Reports: a conflict report (one row per conflicting strip: date, element and board ID,
  shoot day, scene with set and time of day, other sub-board) and a red flag report (same
  shape, flag in place of the shoot day). Conflicts on flagged elements go to the red flag
  report only.
- Icons can be turned off per layout for a clean handout.
- MMS 6 red flags open in MMS 10 as red flags, one per day, not as events.

## What this means for Method Schedule

1. Per sub-board calendar: start date, working week, per-date overrides (off, holiday,
   travel, working). Shoot day N maps to the Nth working date on or after the start date.
   [missing] v1.
2. Events with types and element lists, per sub-board, feeding the DOOD. [missing] v1.
3. Conflict detection across sub-boards of one board as a pure function. [missing] v1.
4. Red flags (availability) and clashes. [missing] later (phase 2), but keep the data shape
   in the v1 file format so adding it is additive.
