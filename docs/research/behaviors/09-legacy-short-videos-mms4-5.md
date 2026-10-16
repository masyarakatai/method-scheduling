# Behavior note 09: legacy tips videos (MMS 4 and 5, 2009)

Source: seven vendor "tips and tricks" videos from 2009, read from auto-generated English
caption tracks fetched without the videos into `imports/research-cache/captions/`:
Hold Days (`WARV77dZDdw`), Custom Daybreaks (`m5sOn_EYQU4`), The Boneyard (`KnX9eS7VHtg`),
Red Flags part 1 (`4_NZX-JL8Ss`) and part 2 (`kwc-cX9dyz4`), Navigation Shortcuts
(`K471ZX095oI`), Estimated Time Preferences (`EGGonnWvSrE`). MMS 4 and 5 era, the oldest
material in this set. Anything in notes 01 to 07 wins.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

## Hold days (bulk)

- Problem shown: a finished schedule where no cast had holds turned on.
- The category manager had, per default setting, an "apply to existing elements" box.
  Ticking holds plus that box turned holds on for every element in the category at once;
  without it, only new elements got the default.
- Then individual exceptions were switched off one by one in the element manager.
- The resulting DOOD showed long runs of H for everyone, which the host called a mess to
  clean up: holds make idle days visible and costly, which is the point.
- MMS 10 (note 07) applies category defaults to new elements only and the manual shows no
  apply-to-existing switch. Engine check E-L1.

## Custom day break text

- Day break text was a template string mixing literal text with placeholders (day number,
  date, pages, estimated time). Removing the date gave a cleaner board while dates were
  still unsettled.
- Alignment, font and style for day breaks, banners and area headers lived in the layout
  settings.
- MMS 10 does this with a field designer instead (note 02).

## Boneyard (MMS 5)

- Called the Boneyard in classic MMS, renamed to "unscheduled area" in MMS 4, then a
  separate window again in MMS 5.
- Moving strips: cut then select a target position then paste (paste needs a selected
  position), or right-click move to Boneyard (goes to its end) and move to stripboard (goes
  to the board's end).

## Red flags (MMS 5)

- New in MMS 5: per element availability limits on dates.
- Entry: pick category, one or more elements, a date or dragged range, a flag type, an
  optional note.
- Dragging a strip onto a flagged date for an element in it showed a flag on the strip at
  once; dragging it off cleared it. Double-clicking the flag highlighted the matching
  constraint. A flag with a plus meant several constraints that day.
- Flag types were a user list, with usage counts. Three built-in types (travel, rehearsal,
  fitting, each tagged for the DOOD) could not be edited or deleted.
- Those three types were what put travel, rehearsal and fitting codes on the DOOD. A DOOD
  preference had to be ticked for them to print (off by default).
- **A fitting flagged before the first shoot day extended the DOOD to start on that earlier
  date**, so the fitting showed in its own column ahead of day 1. This matches the MMS 10
  rule that events show outside the work span. Engine check E-D10 (does MMS 10 add date
  columns before the start date?).
- The red flag field had to be added to custom strip layouts by hand; a layout copy without
  it gave a clean print.

## Navigation shortcuts (MMS 5)

- The breakdown sheet could step by sheet number, by active board order, or by scene number.
- The colored strip at the top of a sheet jumped to that strip on the board; double-click a
  strip to go back to its sheet.
- Right-click on the board had go-to items: first, previous, next strip, Boneyard.
- Cmd or Ctrl plus 5 toggled focus between Boneyard and board; 9 and 0 stepped through
  boards.

## Estimated time display

- Estimated time could show as hours:minutes or decimal hours (3:30 or 3.5), a schedule
  preference. Converting showed the same duration both ways; day totals followed.
- MMS 10's shoot time field is unvalidated text-like number (note 04). Engine check E-B8
  for how MMS 10 sums it.

## What this means for Method Schedule

1. Store durations as integer minutes; display as h:mm by default with decimal hours as an
   option. [missing] v1.
2. Offer "apply to existing" for category defaults. v1.
3. Events before the start date must widen the DOOD date range. v1, pending E-D10.
4. Sheet navigation by board order (default), sheet number or scene number. later.
