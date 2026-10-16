# Behavior note 15: ZTEST run 2, events, company travel days, DOOD summary (real app)

Source: our own checks in the real reference app, 2026-10-10, continuing on the same
throwaway `ZTEST Lantern Creek` schedule as note 14 (holiday on Wed 2027-03-03 still in
place, EVE's holds turned off at the end of run 1). DOOD summary exported to Excel into
the gitignored `/imports/ztest/` and read by script.
Clean-room: behavior in our own words, no copied text, no screenshots kept, the test file
and export stay outside the repo.

## Events

- Event kinds offered when adding one: none, travel, fitting, unavailable, rehearsal,
  camera test and more. Only travel, fitting and rehearsal are marked as counting toward
  the day out of days. An event has a from and to date and a list of elements.
- **Fitting on a hold day** (E-D8): BRUNO, holds on, fitting on a day that was H. The cell
  shows the event code joined with H (`FTH`), and the day moved from held to worked:
  worked 2 to 3, held 2 to 1.
- **Travel inside a drop gap** (E-D11): CLEO had dropped across a 14-day gap. A travel day
  in the middle split it into gaps of 6 and 7, both under the minimum of 14, so the drop
  disappeared and the row became holds with `TRH` on the travel day. The holiday in the
  span, no longer inside a drop, counted as worked again (worked 4 = 2 work + travel +
  holiday, held 8).
- **Event before the start** (E-D10): FINN, holds off, fitting on Fri 2027-02-26, first
  work Mon 2027-03-08. The DOOD grew columns back to that week. FT counted as worked
  (worked 2). The start stayed on the strip day, which still showed SWF. Nothing between
  the fitting and the start was held.

## Company travel day

- Marking Wed 2027-03-17 as company travel (E-D17) took it out of the shoot day count:
  the day after it took the next day number, and later days reflowed.
- Inside ADA's held span (holds on, not in her drop) it showed `M` and counted as worked:
  worked 3 to 4, held unchanged at 6 because the reflow added one more held day at the end.
- Outside an element's span it shows nothing.

## DOOD summary export

- Columns: element name, Work, Hold, Holiday, Start, Finish, Total. One row per element in
  the chosen category, sorted by name in this run (none had board IDs).
- **Total = Work + Hold.** Work already includes counted events, holidays and company
  travel days. The Holiday column was empty for every row even though holidays counted;
  what fills it is an open question (E-D20).
- Start and finish are the first and last strip days, even when an event falls earlier.

## Still to check

An event on a day the element also works (E-D6), rehearsal's code, the unavailable kind,
what fills the summary's Holiday column, DOOD order once elements have board IDs, strips
below the last day break, auto day breaks with a strip over the limit, and the `.fdx`
length forms.
