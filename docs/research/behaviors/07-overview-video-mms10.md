# Behavior note 07: MMS 10 overview tour (vendor video)

Source: vendor video "Movie Magic Scheduling Overview" (`X0GX_EUNe88`, 20 minutes, published
April 2026), read from the human English caption track saved by the owner at
`~/Desktop/Movie Magic Scheduling/`. No frames used. The narration still says "stripboard"
and "segment", so the footage predates the Dec 2024 rename (note 06). The current manual
wins where they differ.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

## What the tour shows

- **Home screen**: new schedule on this machine, cloud backups, open from disk. Account info
  and sign out at the top right. About box shows the version.
- **Top bar**: undo, redo, save, and a schedule switcher listing open schedules; an asterisk
  marks unsaved ones.
- **Tabs**: breakdown, board, calendar, DOOD, elements, reports. Every tab works on one
  board and one sub-board.
- **New board is empty**: its strips all sit in its Boneyard. Move them with right click,
  move to, the default sub-board. Moved strips land at the bottom.
- **Reordering**: drag, or cut and paste (paste goes after the selected strip).
- **Calendar**: board and sub-boards listed at the side; month arrows; today; go to date.
  Right-click a date to set the start date, mark working day, off day, company travel,
  holiday, or add an event. Strips can be shown on the calendar. The calendar is per
  sub-board, not shared. To test a change, duplicate the board. Repeating off days are set
  from the sub-board's menu.
- **Day breaks**: select the last strip of the day, add a day break, it appears below.
  Dates follow the calendar: the first day break gets the start date, and a travel day is
  skipped to the next working day. The calendar then shows day number and strip count per
  day.
- **Saving**: save or save as; closing an unsaved schedule offers save and close, discard
  and close, or cancel. An opened MMS 6 file shows unsaved until saved.
- **Boards keep their own day breaks** and banners. A sample had three boards (an actor
  scenario, a location scenario, scene order).
- **Script import**: from File; a count of new sheets; they land in every board's
  Boneyard. The narration says the script must be tagged for an automatic breakdown.
- **Banners**: notes for the day; can carry a duration that is added to the day's total
  time (a company move).
- **Board header**: start date (edited on the calendar), off days (view, edited on the
  calendar), move to, delete (confirms; deletes from all boards).
- **Bulk day breaks**: by page eighths, by time to shoot, by screen time, or remove all,
  then apply.
- **Elements on a sheet**: click under a category, pick or type a new name, Enter. The new
  element becomes available schedule-wide. Copy and paste elements across sheets.
- **Quick add**: type, arrow to a match, Enter; focus stays for the next. A new name asks
  for its category.
- **New sheet** opens after the current one; duplicate sheet splits a scene. New strips
  appear only in the current board; elsewhere they go to that board's Boneyard.
- **Sort**: pick a field (scene order sorts numerically), add up to two more rules.
- **DOOD screen**: pick a category; blank cells are idle days; company off days are grey
  columns. Edit an element to set allow holds, allow drop and pick-up, and the minimum days
  between drop and pick-up.
  - Holds: the days between start and finish when the element is idle but paid and on
    call.
  - Drop: when the gap is long enough, the element is released and not paid until picked
    up.
  - Category defaults can be set in the elements tab. They apply to new elements only and
    do not fill in existing ones.
- **Conflicts**: an icon marks an element scheduled in two segments on the same day **or with
  overlapping events**. The current manual says events never cause conflicts; manual wins.
  Conflicts also show on the calendar and in the elements tab, and never block anything.
- **Events**: described as what MMS 6 called red flags. Right-click a date, add event,
  optional name, date range, type, color, elements.
- **Elements tab**: board ID shows importance order; elements without an ID sort to the top
  or bottom. Lock IDs before bulk changes. Sort by column or custom drag order, then
  renumber from a chosen number; locked ones are skipped. Move to another category (the
  narration says all info carries over; the manual says custom properties are lost).
  Linked elements, notes, events from an element.
- **Reports** (four at the time): one-line (follows the board design), shooting schedule
  (strips with categories under them, can limit categories), DOOD (paged by groups of
  elements; can run for chosen elements), elements report (board ID, name, occurrences,
  page count, days). PDF or print.

## Disagreements with the current manual

| Topic | Video | Manual (wins) |
|---|---|---|
| Names | Stripboard, segment | Board, sub-board |
| Conflicts | Cross-segment or overlapping events | Cross-sub-board only; events never |
| Events vs red flags | Events replaced red flags | Both exist since 10.9.14 |
| Move element category | All info kept | Custom property values lost |
| Reports | Four | Twelve |

## What this means for Method Schedule

1. Day break dates skip travel days: shoot day N maps to the Nth working date. Confirms
   note 03.
2. Banner duration adds to the day total. [missing] v1.
3. Category DOOD defaults affect new elements only by default. Offer an explicit "apply to
   existing" action like MMS 5 did (one undo step). [missing] v1.
