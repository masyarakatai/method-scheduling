# Behavior note 06: MMS 10 release history (what changed when)

Source: public MMS 10 release notes, beta 1.4.17 (Oct 2023) to 10.15a (Sept 24, 2026), read
October 2026 from the manual's search index.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

Why this matters: dated features tell us which behaviors are new (and may still shift),
which came back from MMS 6 ("parity" items), and which older tutorials describe a different
app. When a tutorial and the current app disagree, check the date here first.

## Timeline (only what touches scheduling behavior)

| Release | Date | Behavior added or changed |
|---|---|---|
| Beta 1.4.17 | Oct 2023 | New native format, MMS 6 import, breakdown, quick add, stripboards with segments, calendar with multi-day events, interactive DOOD with hold and drop rules, conflict feedback across segments and events, auto-numbered board IDs. |
| Beta 10.4.22 | Nov 2023 | Strip designer, first reports (one-line, shooting schedule, DOOD, elements), undo and redo, script import (.fdx and tagged export), budget library export, episode field. |
| Beta 10.5.40 | Feb 2024 | Double-click strip opens its sheet, built-in header fields in the DOOD, cloud backup, warning on corrupt MMS 6 files. |
| Beta 10.6 | Apr 2024 | Export to MMS 6, MMS 6 shortcuts, stacked one-line strips, strip colors editable, strips on calendar, field layout on sheets, element linking, notes, events from elements. |
| 10.7 | Jul 2024 | Split screen, events default to the start date when no day breaks exist, multi-select elements per category, hide empty categories, element list report. |
| 10.7.53 | Sept 2024 | Drag-drop strip and report designers, one-line Excel export, find and replace, **autosave**. |
| 10.7.99 | Nov 2024 | Merge elements, shortcut list in Help, blank value find and replace, weekend exclusion on the calendar. |
| 10.8.20 | Dec 2024 | **Renames**: stripboard to board, segment to sub-board. Multi-episode and multi-unit sub-boards, conflicts across units, custom calendar view. |
| 10.8.33 | Jan 2025 | Recent files, copy and paste banners, drag events, set and location elements can link to anything, report order in the left rail. |
| 10.8.42 | Feb 2025 | Daily category quantity report, landscape reports. |
| 10.9.14 | May 2025 | **Red flag manager** (global across boards), app-wide date format, calendar report. |
| 10.10.7 | Sept 2025 | Report date ranges, scenes and pages per day on the calendar. |
| 10.11 | Dec 2025 | Advanced find and replace, board and calendar print, episodic features, Excel export of breakdown data. |
| 10.12 | Feb 2026 | Shared files with change history, calendar filtering, resource counts on strips, red flag details on elements, calendar and board split. |
| 10.13 | May 2026 | Four-row strips, dotted lines, **shoot day strips** above day breaks, sunrise and sunset fields, copy shoot strips, insert day break above or below, DOOD **horizontal view**, element days and dates tab, **per-sheet use-in-daily-quantity**, merged multi-unit one-line. |
| 10.13a | Jun 2026 | **DOOD multi-key sort** (session only), shoot day designer in reports, page and day counts for set and location elements. |
| 10.14.14 | Jul 2026 | **Templates**, layout files, shared board and one-line layouts, header layouts, red flags split from conflicts (two calendar views), breakdown report grouped by day, strip font color, thin banners, line controls. Fixes: bulk day breaks now apply to all strips; autosave no longer misses rapid edits. |
| 10.15.8 | Aug 2026 | Images on elements and scenes, separate conflict and red flag reports, icon visibility per layout, daily category total parity, estimated time on calendar, partial-match find on set and location, **episode sort numeric**, layouts load with missing categories. |
| 10.15a | Sept 2026 | One-line vertical report, per-category daily quantity switch (cast off by default), red flags on built-in categories. |

## Readings for our engine

- The DOOD engine and hold and drop rules existed from the first beta. They are the most
  settled part of the app. The DOOD rules page is the newest wording.
- Auto day breaks had a bug fixed in July 2026 (did not cover all strips). Any recording
  older than 10.14.14 may show a partial result.
- "Conflict" meant two things before July 2026 (cross-unit clash and availability clash).
  Recordings before 10.14.14 mix them.
- Templates did not exist in MMS 10 until July 2026; older tutorials say to copy a whole
  schedule instead.
- The 2026 overview video (note 07) still says stripboard and segment, so it was recorded
  before Dec 2024 even though it was published in April 2026.
- Many 2026 items are labeled MMS 6 parity. Users moving from MMS 6 expect those
  behaviors; it is a good checklist for us too.

## What this means for Method Schedule

1. Treat items from 10.13 on as the freshest and most likely to change. Re-check them
   before the release candidate.
2. The parity list is a ready-made backlog of what MMS 6 users missed: templates, layouts as
   files, daily quantities, vertical one-line, breakdown by day.
