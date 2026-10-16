# Behavior note 21: screen block 3c, image types, strip actions, keys, elements (real app)

Source: our own checks in the real reference app, 2026-10-12, a 20-minute full-screen
block on `ZTEST Lantern Creek`, plus our own app in its real window. Test images were made
for the purpose (`~/ZTEST/imgtest`, one small picture saved in each format).
Clean-room: behavior in our own words, no copied text, no screenshots kept.
This note outranks notes 01 to 20 wherever they disagree.

## Image types (IMP-09)

- An element's editor has an Images tab with an Add Image button and a drop area.
- The file picker allows **PNG, JPEG, GIF, WebP, TIFF, HEIC and BMP**, and refuses PDF.

## The element editor (ELM-04 and others)

- Tabs: DOOD rules (allow holds, allow drop and pickup, minimum days between drop and
  pickup, 7 here), linked elements, properties, scenes, red flags, days and dates, images.
- Above the tabs: name, board ID, category, lock ID, and **exclude from board as a
  checkbox on the element itself** (ours is a category setting).
- The Elements tab has two lists: categories, and "built-in" (the header fields).
  Columns: board ID, name, occurrences, pages, days, start, finish, conflicts, red flag.

## Strip actions (A9, A12, E-S23)

- The strip's context menu: Edit, Cut, Copy, Paste, Delete Strip(s), Merge, Move To
  (other sub-boards and the Boneyard), Insert Daybreak (above, below), Insert Banner.
- Delete Strip(s) asks first: the selected strips and their breakdown sheets are deleted
  from every board in the schedule; Cancel is the default. As ours (decision 54).

## Keys (E-K1, KEY-01)

- File: new schedule Cmd N, open Cmd O, open shared Shift Cmd O, save Cmd S, save as
  Shift Cmd S, close the active schedule Cmd W.
- Edit: the standard undo, redo, cut, copy, paste and select all; advanced find and
  replace has no key.
- Schedule: Breakdown Cmd period, Board Cmd U, Calendar Shift Cmd D, Reports Cmd P; DOOD,
  Elements and Red Flags have none.
- Design and View: none.

## Our app in its real window (live)

- Board: 22 px lines, the conflict mark on a strip and the Conflicts badge, Break by.
- The right-click menu: every item present; Paste and Merge Strips disabled with one
  strip and an empty clipboard; Move To lists the other sub-board and the Boneyard; a move
  to the Boneyard scrolls to it and selects the strip; one Undo puts it back.
- Breakdown: compact header in three rows, opens on the strip selected on the Board,
  Images row with Add Image.
