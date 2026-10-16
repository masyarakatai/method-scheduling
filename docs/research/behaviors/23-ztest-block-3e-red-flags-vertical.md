# Behavior note 23: screen block 3e, red flags and the vertical one-line (real app)

Source: our own checks in the real reference app, 2026-10-12, a short full-screen block
on `ZTEST Lantern Creek`, plus our app in its real window. No edits were kept.
Clean-room: behavior in our own words, no copied text, no screenshots kept.
This note outranks notes 01 to 22 wherever they disagree.

## Red flags (RFL-01 to RFL-03, E-E14)

- The Red Flags tab is one screen in three parts: a list of elements (a category picker,
  check boxes, so several elements at once), a small month calendar to pick dates
  (several at once, with a count of the dates picked), and the flag's type, a note and an
  add button. Below, a table of every flag: start, end, category, element, type, note,
  with delete for the checked rows, an element filter, export and print.
- **Red flag types and event types are one list**, with built-in entries: travel,
  fitting, rehearsal (each marked as counting in the day out of days), unavailable, camera
  test, post-production and the rest of the event kinds. So E-E14 is answered: there are
  built-in types. Ours starts with no red flag types.

## The vertical one-line (RPT-10, E-R9)

- The stripboard printed sideways: each strip a column with its text turned, day breaks
  as black columns carrying the day's total pages and time.
- No separate cover page: the left side of the same page holds producer, director,
  assistant director and script date, then the cast list as rows in board ID order.

## Our app in its real window (live)

- Calendar: Month and Weeks, the "On each date" switches with All and None, the date
  menu on a right-click (working day greyed as current, day off, company travel, holiday,
  add event, add red flag, set as start date); a holiday shows its star and later days
  move on; Undo puts it back.
- New event: kind, optional name, from and to, eleven colors, several elements from a
  filtered list. A travel event for two actors shows on the calendar and in the side
  panel, and the day out of days shows TR for both and counts it.

## Found in our app

- Empty date fields (prep starts, wrap ends) show a sample date as their hint
  (`2027-03-01`), which reads like a value. A hint should show the form (`YYYY-MM-DD`),
  not a date.
- Weeks view cells have a blank line between the day label and its strips.
