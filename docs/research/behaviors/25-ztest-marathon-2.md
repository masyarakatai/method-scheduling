# Behavior note 25: parity marathon, part 2 (real app)

Source: our own checks in the real reference app, 2026-10-12, continuing note 24 on
`ZTEST Lantern Creek`. Every change was undone or reversed. Clean-room: behavior in our
own words, no copied text, no screenshots kept. This note outranks notes 01 to 24.

## Linked elements (ELM-20, E-L2)

- An element's linked elements tab: a search to add elements (from any category), each
  with an episodes choice ("all"), a remove button, and a **link retroactively** check box.
- **Links are one way**: linking a prop to an actor does not show the actor on the prop.
- **Without retroactive, linking adds nothing to sheets** that already have the main
  element.
- **Links chain**: actor to prop, prop to second prop; adding the actor to a sheet added
  both props. One undo removed all three.

## Daily quantities (BRK-15, ELM-06, E-L11)

- A new background actor on a sheet has "use in daily quantity" on; the element's own
  day out of days default for that category is holds off.
- The daily category totals report counted it as **1 with no quantity set**.

## Header values (E-L12)

- Typing `room 2` in a sheet's set offered the existing `ROOM 2` and used it: **values
  match ignoring case**.

## Renumbering (ELM-17, E-L8)

- With one element locked at 2, renumbering from 1 gave 1, (2 locked), 3, 4, 5, 6: **a
  locked number is skipped**.

## Deleting

- Deleting an element in use asks first (cancel is the default).
- Deleting a sub-board asks first.

## Day out of days summary (E-D20)

- A holiday inside held spans counts in work, and **the summary's holiday column stays
  empty**. The reference app never fills it.

## Calendar print (CAL-09)

- Print opens the month as a one-page PDF in a preview window with save and print.

## Reports

- Each report's settings menu: edit, duplicate, delete, save as a report layout file;
  a layout menu: show report name, show board and sub-board, maximum font size (small,
  default, large), portrait or landscape.
- The conflicts report reads "no conflicts" and can't be printed or exported then.

## Strip designer (BRD-14, BRD-16, BRD-18)

- Fields are dragged in from the list; clicking only selects. **Element quantity** asks
  which elements (any, picked from the categories) it counts.
- The board picks its layout by name from the shared list (default, one-line schedule),
  with "edit layout in strip designer". The strip designer's red flags and conflicts
  switches decide whether the marks show on strips.

## Not found

- **No way to continue a scene on the next day** in the strip menus (DBK-11); the older
  manuals' "multiple day breaks for a scene" isn't in this version's menus.

## Files

- The cloud backup dialog lists a backup per save of each schedule with recover and
  delete; on by default.

## Added later in the same block

- **Drag on the board** (BRD-05): a strip dragged up to the first day lands at the top of
  it; the day it left keeps its break with no strips and 0 pages.
- **The day out of days category picker** (DOOD-01) lists every category with its count.
- **A travel red flag does not count in the day out of days** (E-E15): the actor's work
  and hold totals stayed the same; a flag mark shows beside the name.
- **Picking dates for a red flag**: a click replaces the pick, Command-click adds; three
  dates (two following, one apart) made **two flags, one per run of following dates**
  (E-E16).
- **Import Script greys out Fountain and plain text files**: the reference app reads
  neither (IMP-07; ours is an extra).
