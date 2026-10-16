# Behavior note 24: parity marathon, part 1 (real app)

Source: our own checks in the real reference app, 2026-10-12, a long full-screen block on
`ZTEST Lantern Creek`, going through every feature not yet checked there. Every change
was undone or reversed; nothing was saved apart from the app's own autosave of reversed
edits. Clean-room: behavior in our own words, no copied text, no screenshots kept.
This note outranks notes 01 to 23 wherever they disagree.

## Breakdown

- **Hide empty** (BRK-11) leaves only the categories with elements on the sheet.
- **Adding in a category box** (BRK-12): typing a new name in a box offers to add it there.
  Each element row in a box has a "use in daily quantity" check box.
- **Quick add with an unknown name** (BRK-13) asks for the category from a list.
- **An element's menu** (right-click): cut, copy, paste, edit element, remove from sheet,
  and a quantity field (BRK-15: quantity is per sheet). **Command V anywhere on another
  sheet pastes** the copied element there (BRK-14).
- **Next** steps through sheets in board order (BRK-09). The title gains an asterisk for
  unsaved changes.
- **Delete Sheet deletes at once, without asking**; a short notice says it was deleted;
  Undo brings it back (BRK-08). Ours asks first, on purpose.

## Board

- **Find on the board** (BRD-12): a bar with a field picker (comments, time of day,
  element, episode, I/E, location, scene number, script day, and more), a "containing"
  box, a count of matches, previous and next, and select all. Matches are highlighted.
- **Sort** (BRD-09): rules in a list, each a field; **at most four rules**; no ascending or
  descending choice.
- **A day break's menu**: cut, copy, paste, delete, merge, move to, insert day break,
  insert banner. A **day break can be moved to the Boneyard** (DBK-05); there it shows as
  an end of day with no number or date, no pages and no time.
- **Bulk day break actions** (beside New Day Break, DBK-08, DBK-09): insert by page eighths
  (whole pages and eighths per day), by shooting time, by screen time, or remove all day
  breaks. **Remove all does not ask**, and the shoot day headers go with the breaks;
  Undo restores them.
- **New Banner** (BRD-11) edits the banner in place on the board, above the selected
  strip, with a text box and a duration in hours and minutes; a style panel has bold and
  italic, a font size, a banner size, alignment, a duration switch and a palette of about
  fifty colors.
- **Sub-boards** (BRD-02): the sub-board picker lists the sub-boards, the Boneyard, add
  sub-board and reorder. A **new sub-board's start date is today**. Deleting a sub-board
  asks, then sends its scenes to the Boneyard; its day breaks go.
- **Move To another sub-board puts the strip at the top** of that sub-board, and the
  selection moves to the next strip where it was.
- **A strip in a conflict shows a red warning mark**; the calendar day shows one too.

## Conflicts (EVT-05)

- With scene 2 moved to a second sub-board starting the same day as the first, the
  toolbar count and its list ("conflicts and red flags for the board") named **every
  element on both strips that day, header values included**: the actor, the prop, the
  I/E value and the time of day value. Each entry links to the strip in each sub-board.
  Ours counts real elements only.

## Elements

- **The built-in list** (ELM-09) shows every header field: time of day, episode, I/E,
  location, script day, sequence, set and unit, each value with occurrences, pages, days,
  start and finish, unused pick list values included. Ours covers set, location, unit and
  episode only.
- **The cast category comes with properties**: full name, address, phone, agent, agent's
  phone, rate, per, and notes (ELM-07, ELM-12, ELM-13). A category's properties have a
  name and a type (single line or multi-line), reorder by dragging, add, edit, delete.
- **A category's settings are defaults for new elements**: automatic board IDs, exclude
  from board, allow hold days, use in daily quantity. Exclude from board itself is an
  element setting (note 21). Ours applies the category switch to all its elements.

## Calendar, DOOD, reports, files

- **Dragging a strip to another date on the calendar** (CAL-06) puts it at the end of that
  day's strips; the day it left stays, empty.
- **The DOOD's View menu** hides or shows Saturdays and Sundays. A button switches to the
  vertical view (DOD-10): dates down, elements across, worked and held totals at the top.
- **Reports**: each report has a menu (edit, duplicate, delete, save as a report layout
  file) and a handle to reorder (RPT-16). The DOOD report pages by weeks with date, day of
  the week and shooting day rows; the summary has work, hold, holiday (empty), start,
  finish, total; the element list has board ID, name, occurrences, pages, days; element
  detail lists each element's scenes, sets, start, finish, work days and occurrences;
  daily category totals list each day's strips and a total for each category counted
  daily (background actors in this schedule).
- **Advanced find and replace** opens beside the breakdown: find and find and replace,
  rows of field, category, operator and value joined by and or or. Operators: equal or not
  equal for categories; equal, starts with, contains for text.
- **Several open schedules** share one window and are switched from the title's menu
  ("currently open schedules"). Ours opens a window per schedule.
- **New Schedule with autosave on asks to save it at once** ("save as required" while
  autosave is on); cancelling discards the new schedule. Ours keeps it untitled with a
  recovery copy.
