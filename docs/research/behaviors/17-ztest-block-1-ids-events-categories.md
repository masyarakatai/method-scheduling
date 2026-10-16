# Behavior note 17: screen block 1, board IDs, events on work days, categories (real app)

Source: our own checks in the real reference app, 2026-10-11, on throwaway schedules
(`ZTEST Block1`, and `ZTEST Lantern Creek` from run 2). Runbook items A1 to A15 in
docs/research/screen-block-1.md. Results read from the board, the DOOD and the Elements
screens.
Clean-room: behavior in our own words, no copied text, no screenshots kept, the test files
stay outside the repo. This note outranks notes 01 to 16 wherever they disagree.

## Board IDs

- **A1, order.** The DOOD and the element lists go by board ID ascending, numbers by
  value (1, 2, 9, 10). An element with no ID goes last.
- **A2, the text is kept and the order is natural.** `07` and `7` may both exist; the ID
  is kept exactly as typed. The order is natural: the leading number first, then the
  rest, so 1, 1a, 2, 7, 07, 9. `1a` sorts between 1 and 2, not after every whole
  number. Two elements with the same ID order by name.
- **A13, renumber.** Renumbering goes in the list's order and leaves a locked ID where it
  is. Whether a number held by a locked element is skipped was not settled.
- **A14, the next automatic ID is one more than the highest**, not the lowest free
  number: with IDs up to 9 and a gap below, the new element got 10.

## Names

- **A3.** A second element whose name differs only in case (`ada` beside `ADA`) in the
  same category is refused; the name is said to be in use in the category.

## Events and the DOOD

- **A4, an event on a work day.** A travel day on a date the element works shows the
  event's code in the cell (the travel code, with the start mark when it is the first
  day) and a warning mark. It counts as a conflict: the toolbar's conflict count goes up
  and the element's row in Elements shows it. The day is counted once, as one worked
  day.
- **A5.** A fitting on the pickup day shows the fitting code with the pickup mark and the
  warning. Totals do not change.
- **A6.** A holiday on a Saturday inside a held span shows the holiday mark and counts as
  worked (as note 15).
- **A7.** Marking that Saturday as a working day makes it a shoot day and the later days
  move along; a fitting that then falls on a hold day shows the fitting code with the
  hold mark.
- **A8.** Unavailable and camera test events neither show nor count; the cell stays a
  hold.
- **A9.** The DOOD summary's holiday column stays empty even when holidays are counted
  in the span.

## Categories

- **A10.** The cast category can be renamed. A category's settings are: assign board IDs
  automatically, leave off the board, allow hold days, allow drop and pickup, and use in
  the daily quantity. Changing them offers no way to apply them to existing elements.
- **A14.** Category settings apply to elements made after the change only.

## Merge and move

- **A11, merging elements.** Only the selected elements are offered as the survivor, so
  a merge stays inside one category. The survivor keeps its board ID, the scenes of all
  of them are combined, and the others' events move to it.
- **A12.** Moving an element into a category where its board ID is already taken is
  refused with a message.

## Automatic day breaks by time

- **A15.** With a limit of 7:30, a banner's time counts toward the day: 3:00, a 1:30
  banner and 3:00 make 7:30 and close the day (as note 16: a day closes on reaching the
  limit).

## Banners

- New Banner goes **above** the selected strip.

## Still to check

- A13: whether renumbering skips a number a locked element holds.
- The code shown for an event on a start or finish day in the summary columns.
