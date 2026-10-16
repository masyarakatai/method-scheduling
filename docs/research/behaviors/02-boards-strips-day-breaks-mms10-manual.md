# Behavior note 02: boards, sub-boards, strips and day breaks (MMS 10 manual)

Source: public MMS 10 manual, Board section (board, managing strips, banners, day breaks,
strip display, right-click menu, board find, print and download), plus the archived MMS 10
beta pages still in the site's search index (stripboard, segments, day breaks), read
October 2026. Current app with beta-era notes where marked.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

## Model

- A schedule holds one set of breakdown sheets. Each sheet has exactly one strip.
- A **board** is one arrangement of all the strips. A schedule can have many boards (story
  order, "actor scenario", "location scenario" and so on).
- A board holds **sub-boards** (called segments before Dec 2024). Typical use: first and
  second unit running at the same time, episode blocks, prep and post phases.
- Every board has a **Boneyard** sub-board. Strips there count as unscheduled and its start
  date is ignored. Each board has its own Boneyard.
- A new board starts with every strip in its Boneyard.
- Board and sub-board names are unique within the schedule (the beta said unique within the
  board for segments; the current page says schedule). Engine check E-S1 (cheap).
- Boards can be duplicated. Sub-boards cannot.
- Deleting a board deletes only the arrangement, never sheets. The beta said deleting a
  segment moves its strips to the Boneyard; the current page doesn't say. Assume the
  same.
- Day breaks and banners belong to one board. They are not shared across boards.
- Each sub-board carries a start date and its own off days.

## New strips

- A new or duplicated sheet lands right after the selected strip in the active sub-board of
  the active board. In every other board it lands at the end of that board's Boneyard.
- Imported sheets land at the end of the Boneyard in every board.
- An MMS 6 file opened in MMS 10 starts with all strips in the Boneyard.

## Moving strips

- Drag one or more selected strips (shift, cmd or ctrl click to multi-select). A line marks
  the drop point.
- Cut then paste: pasted strips go below the selected strip.
- Copy works on every strip type (shoot strips, banners, day breaks). The clipboard lasts
  until you switch board or schedule.
- Pasting into an empty sub-board or the Boneyard selects the first pasted strip.
- Move To sends the selection to another sub-board (at its end, we assume). Engine check
  E-S2.
- Drag across split panes works only between sub-boards of the same board.
- Merge: two or more selected strips become one sheet. A confirmation comes first. What
  merges and how (scene numbers joined, eighths summed, elements unioned) is not described.
  Engine check E-S3.
- Delete strip deletes its sheet everywhere, after a confirmation.

## Sorting

- Up to three sort keys from the scene header fields (the overview video says scene order
  plus two more).
- A sort pulls every banner to the top, removes all day breaks from the sub-board, then
  orders strips ascending.
- The beta manual said the sort was plain alphanumeric and promised smarter scene number
  handling later. Episode sorts numerically (EP10 after EP9, no episode last) since 10.15.8.
  Whether scene numbers now sort naturally (2 before 10, 10 before 10A) is open. E-S4.

## Banners

- Free text strips for notes (title, company move, lunch, "production day 1" and so on).
- Each banner can carry a duration. That duration is added into the day's estimated time
  total.
- Per banner: font style, size, alignment, color; thin or standard height (thin is half).
- Inserted above or below the selected strip; when several are selected, the last selected
  one decides.

## Day breaks

- A day break ends a shoot day. It shows the day number (count of day breaks so far in the
  sub-board), the date, total pages and total estimated (shoot) time of the strips above
  it back to the previous break. Optional fields: sunrise, sunset, screen time total,
  custom text, spacing, element quantity.
- Since 10.13 a **shoot day strip** sits above each day break, linked to it, showing the
  production's shoot day count and the full date. They move together. A day break moved to
  the Boneyard keeps its shoot day strip but loses its number.
- Day breaks can be moved, deleted and even parked in the Boneyard like strips.
- Sunrise on a break is the earliest sunrise of the day's sheets, sunset the latest. No
  values shows 00:00.

### Automatic day breaks

- Four options: by max page eighths, by max shoot time, by max screen time, or remove all.
- Applying always removes the sub-board's existing day breaks first, then inserts new ones.
- The archived beta page adds two rules the current page dropped: each day takes strips
  (and banners) only up to the max, and a single strip bigger than the max gets a day of
  its own. We take those as the rule until the real app says otherwise. E-B1, E-B2.
- 10.14.14 fixed a bug where bulk day breaks missed some strips, so it covers the whole sub-board,
  not just from the selection.
- MMS 6 (note 08) offered the same by time per day or max pages per day. Same idea.

### Questions the manual does not answer

- Is the max inclusive (a day of exactly the max is allowed)? E-B1.
- Do banner durations count toward a shoot-time max? E-B3.
- Is a last partial day closed with a day break, or left open after the last strip? E-B6.
- What happens to strips below the last day break: are they scheduled (DOOD, calendar)?
  E-B6.
- Two day breaks in a row (an empty day). MMS 5 had an "allow multiple day breaks"
  switch for scenes that take more than one day. Not in the MMS 10 manual. E-B5.

## Strip designer (display only)

- Fields a strip can show: scene, I/E, set, elements of a category, custom text, script day,
  ID list of a category, pages, script page, eighths, location, unit, shoot time, screen time,
  spacer, episode, sequence, shooting date, shooting day, D/N, element quantity, sunrise,
  sunset, comments.
- Up to four rows of fields. Cells merge when the same field sits in neighboring cells.
  Scale-to-content wraps text.
- Strip colors keyed by I/E and time of day. Font color per strip type since 10.14.14.
- Lines: none, all, vertical, horizontal; solid or dotted.
- Several named layouts per schedule; the board and the one-line report share one layout
  library and edits sync both ways (10.14.14).
- Show or hide total strips, banners and shoot day strips per sub-board (session setting).

## What this means for Method Schedule

1. Board, sub-board, Boneyard, strip, banner, day break as plain data with stable ids.
   Property test: any sequence of moves keeps every sheet on every board exactly once.
   [missing] v1.
2. Auto day breaks by eighths, shoot time and screen time as pure functions over a strip
   list, with the oversize-strip rule. [missing] v1.
3. Sort that strips day breaks and lifts banners, like MMS. Natural scene sort as an opt-in
   setting until E-S4 is checked. [missing] v1.
4. Strip designer: v1 ships a fixed sensible layout plus a field picker; the full designer
   is later.
