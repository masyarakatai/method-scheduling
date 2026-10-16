# Behavior note 01: day out of days rules (MMS 10 manual)

Source: public MMS 10 manual, Day-Out-of-Days and DOOD Rules pages, plus the Reports pages
for the DOOD report and DOOD summary and the release notes for 10.13 and 10.13a (read
October 2026 through the site's full-text search index). Current app, MMS 10.15a era.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.
Raw fetch lives in the gitignored `imports/research-cache/`.

This is the most important note for the engine. Every rule below still needs a check in the
real app (see `../engine-checks.md`).

## Scope and inputs

- A DOOD is computed for one board and one sub-board at a time, and one category at a time.
  Built-in header fields (set, location, unit, episode) can also be shown as DOOD
  "categories", so a set gets its own work days.
- Inputs: the strip order and day breaks of that sub-board, the sub-board's calendar (start
  date, working week, off days, holidays, company travel days), and events attached to
  elements in that sub-board.
- Hold and drop switches live on each element. A category can carry defaults for new
  elements. Changing the category default does not rewrite existing elements in MMS 10
  (the 2026 overview video says so; MMS 5 had an explicit "apply to existing" box, see
  note 10).
- The manual says rules are managed per sub-board. Unclear whether that means the hold and
  drop switches themselves can differ per sub-board, or only that the DOOD is computed per
  sub-board. Engine check E-D12.

## Codes and what each one means (our words)

- **Blank**: the element does nothing that day.
- **Off day**: a company day off from the calendar. Shown as an empty grey cell. Never a
  hold, never work (no strip can sit on it). Some events can still put a code on it.
- **W**: the element is in a strip scheduled that day.
- **S** (start): the first day the element is in a strip. Events before that day do not
  move the start. Shown joined with W as SW.
- **F** (finish): the last day the element is in a strip. Events after it do not move the
  finish. Shown as WF. A one-day element shows SWF.
- **H** (hold): only when the element allows holds. Every non-working day strictly between
  the first and last strip day gets H, except company off days (they stay grey, not held).
  Counted in the held total.
- **D** (drop): only when holds are allowed and drops are allowed. If there is a long
  enough gap between two work days, the element is released. The manual says the drop
  appears joined to a work code (WD, or SWD), so the D sits on the last work day before
  the gap, not on the first idle day. Days inside the drop are not counted at all.
- **P** (pick-up): the work day the element comes back after a drop, joined as PW (or PWF).
- **TR** travel, **FT** fitting, **R** rehearsal: from events of those types attached to the
  element.
  - Shown and counted whether or not holds or drops are allowed.
  - Shown even outside the first-to-last work span (before start, after finish).
  - Counted in the worked total.
  - Treated as work when the engine looks for a drop gap, so an event inside a gap can
    break it up and prevent the drop.
- **/** company holiday and **M** company travel (move) day:
  - Treated like a non-working day for strips (no strip can be scheduled on them).
  - Shown and counted as worked only when all of these hold: the element allows holds, it
    has at least one W day, the day falls between its first and last W day, and the day
    is not inside a drop.
  - Otherwise not shown and not counted.
  - Note the difference: M is the whole company traveling, TR is one element traveling.

## Event types and the DOOD

From the Events page: six event types exist. Travel, fitting, rehearsal and "unavailable"
show on the DOOD. "None" (a general event) and camera test do not. The DOOD rules page
lists codes for travel, fitting and rehearsal only, so the code an unavailable event shows
is not documented. Engine check E-D9.

Events belong to the sub-board they were added in.

## Totals

- Worked total: W days plus TR, FT, R days, plus holidays and move days that qualify.
- Held total: H days.
- Drop days: not counted anywhere.
- The DOOD summary report lists, per element, worked, held, travel and fitting totals
  (and more), without the day grid.
- The MMS 5 manual (note 10) defined the grand total as work plus hold plus company travel,
  with holiday kept as its own column. MMS 10 instead folds qualifying holidays into
  worked. The two disagree; MMS 10 wins until the real app says otherwise. Engine check
  E-D7.

## Layout and sorting

- Weeks run Sunday to Saturday across the top. One row per element.
- Default row order is "scene order" (order of first appearance, we assume). Engine check
  E-D14.
- Vertical (default) and horizontal views. Since 10.13.
- Multi-key sort, up to four keys, vertical view only, from 10.13a. Keys: board ID, name,
  first work day, last work day, worked days, held days. Each key picks its own direction.
  A key used above is not offered again below. Elements missing the value go last. Ties
  keep their previous order (a stable sort). The sort lives for the session only: it
  clears on close, on switching sub-board, or on leaving the DOOD tab, and is never saved.
- Working week setting is shared between DOOD and calendar. The manual names this as the
  first suspect when an MMS 10 DOOD differs from an old MMS 6 one.

## Drop placement: what the manual says and what it leaves open

- Drop needs holds on first. Then a minimum gap in **calendar days** (not working days).
- The engine places the drop by itself at the earliest gap long enough. Read literally, that could mean
  only one drop per element (the first long enough gap), with any later long gap shown as
  holds. Industry practice (actor contracts) usually allows one drop and pick-up per
  engagement, which fits that reading. Must be checked: E-D2.
- Not stated: whether the gap is measured from the last work day to the next work day
  (pick-up date minus drop date), or as the count of idle days strictly between them.
  E-D1.
- Not stated: whether off days, holidays and move days inside the gap count toward the
  calendar-day minimum. "Calendar days" suggests yes. E-D3.

## What this means for Method Schedule

1. DOOD as a pure function of (sub-board strips and day breaks, calendar, events, element
   switches), one fixture per rule above. [missing] v1.
2. Keep every total separate (worked, held, travel, fitting, rehearsal, holiday, move,
   drop days) and derive "worked" the MMS 10 way by default. [missing] v1.
3. An "explain this cell" view: for any code, say which rule produced it. Phase 3 extra.
4. Session-only multi-key sort with stable ordering and blanks last. [missing] later.
