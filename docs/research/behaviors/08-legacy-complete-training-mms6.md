# Behavior note 08: legacy complete training (MMS 6 era, vendor video)

Source: vendor video "Legacy Movie Magic Scheduling Complete Video Training" (`ohwgc8hwhbA`,
58 minutes, Feb 2018, MMS 6), read from the auto-generated English caption track fetched
without the video into the gitignored `imports/research-cache/captions/`. MMS 6 era: older
than everything in notes 01 to 07, which win where they differ.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.
The trainer's sample productions are not reused; examples below are invented.

## Breakdown practice (useful domain knowledge, still true)

- New schedule from a template; the stock app shipped a default and a French template.
- Production info (title, key staff) only shows on printouts and reports.
- Script import from the screenwriting app needs the script tagged there first. The trainer
  doesn't trust tagging done by others and breaks down by hand.
- Sheet number is just an index; the scene field is what matters.
  - One scene split across places: A, B, C suffixes on the scene number.
  - Several scenes on one set: a list or range in one sheet.
  - Intercut action: one sheet per thread, each listing alternate scene numbers.
- I/E has interior, exterior and both; users add their own values (an "underwater" option
  for an invented sea film). Time of day: day, night, morning, evening; stick to day or
  night unless the script needs sunrise or sunset.
- Each I/E and time of day pair has its own strip color, set in one grid; text color too.
  The defaults follow industry convention.
- Pages are whole pages and eighths, never reduced (one and a half pages is 1 4/8).
  Schedulers pad short but heavy scenes (a one-line action beat counted as 5/8).
- Synopsis: short enough to fit the strip.
- Script day: D1, D2 and so on per story day regardless of elapsed time; N1 for the night
  of day 1. Used by wardrobe and makeup for continuity.
- Unit, sequence and location often blank at first. Line producers sometimes use sequence
  to flag expensive stretches.
- Estimated time in hours and minutes; many ADs skip it and plan by pages.
- Comments printed on only one report then; notes printed everywhere.
- Typing a new element name and pressing Enter asks to create it; ending the name with a
  semicolon creates it with no prompt.
- Background convention: one element per group with the count in the name ("Neighbors (10)")
  in the MMS 6 era. MMS 10 imports such names as typed (note 05) and has a real quantity.

## Cleanup and IDs

- Element manager: fix a misspelled element once and every sheet updates; merge two
  spellings, choosing the survivor.
- Board IDs: sort cast by occurrence, most first, renumber from 1 in that order. Contract
  billing can force a number; move that actor to the top, then lock the ID so renumbering
  never changes it. Vehicles get IDs too.

## Scheduling

- Click the colored bar at the top of a sheet to jump to its strip.
- Several layouts (horizontal and vertical); vertical ones show board IDs as columns, handy
  for spotting an actor's run.
- Keep a story-order board untouched; duplicate it into a working draft. Unlimited boards
  for scenarios.
- First pass: sort by set, then time of day, then I/E.
- Day break after the last strip of a day; the break shows day number, date, pages and time
  estimate. Totals update as strips move above it.
- **Auto schedule** (MMS 6 name): give time per day or max pages per day; day breaks drop
  in for the whole board as a starting point; delete all day breaks to start over.
- Banners for company moves with an optional duration that adds to the day total (an
  invented example: two 3-hour scenes plus a 1:30 move shows 7:30). Banners also serve as
  titles, lunches, rehearsals.
- Calendar manager: several named calendars per schedule, each with days off, holidays, and
  production dates. Only the production start date moves the schedule; prep start and end
  of post only print. Switching the board's calendar re-dates every day break. Weekend
  days can be opened up one at a time.
- Calendar view: drag strips or whole days to other dates.
- Boneyard: a holding drawer, per board. Uses: parking unplaceable scenes, a palette to build
  from, a second unit, or removing shot scenes during production.

## Printing and DOOD

- Print view matches the screen; ranges by date or scene; options for Boneyard, banners,
  day breaks. First page lists cast with board IDs.
- Reports: breakdown sheets, cast list, one-line, several shooting schedule variants (one
  carries comments), images, DOOD.
- DOOD reading: SW, W, WF, SWF. Gaps between work days for a lead are costly (pay or lose the
  actor). Same for picture vehicles.

## Where this disagrees with MMS 10

| Topic | MMS 6 (this video) | MMS 10 (wins) |
|---|---|---|
| Calendars | Named calendars, board picks one | Start date and off days per sub-board |
| Boneyard | Separate window per board | A sub-board in each board |
| Auto day breaks | Time or pages per day | Eighths, shoot time or screen time; replaces existing |
| Background counts | Count in the element name | Quantity field plus daily quantity rules |

## What this means for Method Schedule

1. Scene field accepts lists, ranges and lettered parts; sorting must cope (E-S4).
2. Lock and renumber exactly as described. [missing] v1.
3. Keep "duplicate board as scenario" cheap. [missing] v1.
4. Semicolon-to-create is a nice keyboard touch; consider it as an opt-in. later.
