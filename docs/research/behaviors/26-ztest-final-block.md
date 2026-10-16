# Behavior note 26: the final screen block (real app)

Source: our own checks in the real reference app, 2026-10-12, on `ZTEST Lantern Creek`,
plus our app in its real window. Every change was undone. Clean-room: behavior in our own
words, no copied text, no screenshots kept. This note outranks notes 01 to 25.

## Reference app

- **Sheet images** (BRK-16): the breakdown sheet itself has no images area; scene images
  show through the breakdown beside images view (note 22).
- **Stepping** (BRK-09, BRK-10): first, previous, next, last, in board order only; no
  stepping by sheet or scene number (BRK-10 is from the older version).
- **Sheet number and decimal time** (BRK-17, BRK-18): not in the breakdown designer's
  field list or anywhere on the sheet; older-version features only.
- **A new sheet** (BRD-04) goes right after the current sheet, in its day.
- **Off days on the board** (DOD-15): the board's repeating off days are shown read-only,
  with a link to the calendar where they are changed; the board, the calendar and the
  day out of days share one working week.
- **Day out of days code letters** (DOD-16): no setting for them anywhere.

## Our app in its real window

- Red flags tab: elements by check box, dates on a small calendar, one flag per element
  per run of dates (two elements, two runs: four flags); one undo.
- Vertical one-line: scene columns with turned text, dark day columns, a cast row per
  member in board ID order. **The day columns' text is cut off** ("End of day 1, Mon
  2027-03-..."): it needs to fit or wrap.
- The Red flags tab's small calendar opens on today's month, not the schedule's: it should
  open on the sub-board's start month.
- Breakdown Designer: show, width, up and down per field; hiding a field reflows the
  rows; one undo. The breakdown shows a conflict line for the sheet.
- Quit left no recovery lock.
