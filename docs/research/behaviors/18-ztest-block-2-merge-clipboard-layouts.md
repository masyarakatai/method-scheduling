# Behavior note 18: screen block 2, merging strips, cut/copy/paste, strip layouts (real app)

Source: our own checks in the real reference app, 2026-10-11, on throwaway schedules
(`ZTEST Lantern Creek` and `ZTEST Block1`). Results read from the board, the merge
dialog, the strip designer and the DOOD.
Clean-room: behavior in our own words, no copied text, no screenshots kept, the test files
stay outside the repo. This note outranks notes 01 to 17 wherever they disagree.

## Merging strips (E-S18)

- With scene 3 clicked first and scene 2 added with Command-click, the merge dialog names
  scene 3 as the strip whose fields are used: **the first selected strip survives**, not
  the topmost.
- The dialog says the merge applies on every board, the first selected strip's fields
  are used, and the scenes and elements of all the selected strips go into the result.
- After the merge the strip read `3, 2`: **scene numbers join in selection order**,
  the survivor's first. It kept scene 3's set and place on the board, its pages were
  the sum (2/8), and the day scene 2 was on was left empty.
- A Merge button sits in the board toolbar and in the strip's context menu; it is
  enabled with two or more strips selected. Undo takes the merge back.

## Cut, copy and paste of strips (E-S19, E-S20, E-S21)

- Strips are cut, copied and pasted from **the strip's context menu** (right-click),
  which offers Edit, Cut, Copy, Paste, Delete Strip(s), Merge, Move To, Insert Daybreak
  and Insert Banner. The Edit menu's Cut and Paste did nothing to a selected strip.
- **Cut leaves the strip where it is**, outlined in red. Paste on another strip moves
  it **below that strip**; the strip count stays the same and the moved strip is
  selected.
- **Copy** also outlines the source strip. Paste below another strip makes **a new
  sheet** (the strip count went from 26 to 27) with **the same scene number** (`3`,
  not `3A`). After a copy's paste the selection stayed on the strip pasted below; the
  new strip was not selected.
- Paste is greyed out until something is cut or copied.
- No Duplicate command was found in the Schedule menu or the strip's context menu.
- Not checked: pasting on another board, and where a pasted copy appears on other boards.

## Strip layouts (E-S22)

- The strip designer works on **named layouts kept in a list**: the schedule had
  `Default` and `One-line Schedule`, with a command to add a new one. A layout has a
  **type**: strips, day breaks or shoot day headers.
- The default strip layout has **two lines**. First line: scene number, I/E, set, day or
  night, script day, cast (names), cast IDs, page eighths, unit. Second line: synopsis
  under most of the width, location, shoot time.
- Options: show red flag and conflict marks, scale the height, bold and italic,
  alignment, lines between fields and their style, font size.
- Fields offered to add include a spacer, custom text, ID lists and element lists by
  category, comments, episode, screen time, script page, sequence, shooting date,
  shooting day, element quantity, sunrise and sunset.

## An event on a start day (E-D6, follow-up to note 17 A4)

- The cell's full text (shown on hover) was `TRSW`: **the event's code followed by the
  whole work code** (`SW`), not only the marks. So a fitting on a pickup day reads
  `FTPW`, and on a plain work day `FTW`.

## Still to check

- Pasting strips on another board.
- Whether a layout belongs to a board or is picked per board from the list, and whether
  the one-line report uses its own layout.
- Where a banner goes with nothing selected.
