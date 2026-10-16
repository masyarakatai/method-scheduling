# Behavior note 12: DOOD practice across other tools and guides

Source: public articles and help pages read October 2026: Jungle Software's DOOD articles
(junglesoftware.com, two pages), StudioBinder's DOOD explainers (two posts), Wrapbook's DOOD
guide (Sept 2022), SetHero's DOOD glossary entry, TheGreenShot's DOOD article (Aug 2026),
Celtx's DOOD help article, Yamdu's DOOD concept page, and the sister repo's
`interop-formats.md` section 6. Raw pages in `imports/research-cache/interop/dood/`.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.
These are secondary sources about industry practice, not about MMS. MMS behavior (note 01)
wins for our defaults.

## Codes the industry uses

| Code | Meaning | Who uses it |
|---|---|---|
| SW (or S) | first work day | all |
| W | work | all |
| WF (or F, SF) | last work day | all; SetHero adds a session finish variant |
| SWF | one-day element | all |
| H | hold: idle but paid and on call | most |
| I | idle: not working, not paid, not released | Jungle, Wrapbook, TheGreenShot. MMS has no I |
| WD (or D) | work then drop | most; SetHero writes D alone |
| PW, PWF (or P) | pick-up | most |
| T or TR | travel | most |
| R, SR | rehearsal, start with rehearsal | Jungle, Wrapbook |
| FT | fitting | MMS; others mention fittings without a letter |
| grey or dash | company off day | most |

Celtx only shows SW, W, WF and SWF. Filmustage (per the sister repo) has no travel, rehearsal,
drop or pick-up codes.

## How holds and drops are decided

- Holds are computed from gaps between work days in every tool that has them. Wrapbook notes
  that whether the gap is hold or idle depends on the actor's deal, so it is a per-element
  switch (as in MMS).
- Drop threshold disagrees across sources: "seven or more days" (Jungle legend,
  StudioBinder, Wrapbook), "ten or sometimes fourteen days" (Jungle body text), "more than
  six" (SetHero), and the open schedule standard's samples use ten. MMS makes it a number on
  each element. We do the same, with the category default, and never hard-code a value.
- Jungle says an element may be dropped only once per schedule and the user picks which gap
  when two qualify. This supports reading MMS's "first slot that fits" as one drop per
  element, placed automatically at the first gap (E-D2).
- Real contract rules (actor union consecutive employment, drop and pick-up) are the true
  authority. The union site blocks fetchers, so we have not read them. Owner question.

## Totals and layouts

- Common totals: work, hold, travel; sometimes idle, holiday, rehearsal, start and finish
  dates, total days; some add rate and pay. MMS keeps travel, fitting and rehearsal inside
  worked (note 01); other guides don't say.
- Rows are elements (cast by board ID first). Some guides group principals, supporting and
  day players. Columns are shoot days (or calendar days with off days greyed).
- Several guides say a DOOD needs day breaks: without them a one-day actor can show on two
  days, or the report can't run at all (Gorilla, Celtx).
- DOOD for any category (props, vehicles, sets), not just cast, is standard in pro tools.
- The DOOD is the source for the actor union's daily performer report (Exhibit G in the US)
  and for budget day counts (Gorilla feeds days worked into its budgeting app).

## What this means for Method Schedule

1. Default to MMS's code set and rules. An opt-in "idle" code for elements with holds off
   but not released is a possible extra. later.
2. Drop threshold per element, default from the category, no built-in number beyond what
   the owner picks for our template. v1.
3. DOOD for any category including set and location. v1.
4. Budget exchange carries each count separately (kickoff section 7). v1 data, later UI.
