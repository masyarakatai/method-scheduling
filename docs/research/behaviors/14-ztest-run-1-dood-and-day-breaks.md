# Behavior note 14: ZTEST run 1, day out of days and day breaks (real app)

Source: our own checks in the real reference app (MMS 10, current release on this Mac),
2026-10-10, on a throwaway schedule `ZTEST Lantern Creek` built from an invented script we
generated (`scripts/make-ztest-fdx.py`). Results read from the app's screens.
Clean-room: behavior in our own words, no copied text, no screenshots kept, the test file
stays outside the repo. This note outranks notes 01 to 13 wherever they disagree.

## Setup

- 25 invented one-page scenes, one per sheet, imported from our `.fdx`. Start date Mon
  2027-03-01, default working week. Automatic day breaks at 1/8 page a day, which put one
  strip on each day.
- Six invented cast members, each placed on the days one check needs: ADA, BRUNO, CLEO,
  DARIO, EVE, FINN.

## Import and setup findings

- A brand new schedule already holds one empty sheet on its first sub-board. After our
  import the board had 26 strips, and that empty sheet became the last shoot day.
- Imported sheets land in the Boneyard, in script order.
- Our `.fdx` gave each scene `Length="1"`; every sheet came in as **1/8**. Either that
  length form isn't read or a missing length becomes 1/8. Follow-up: try `8/8` and
  `1 0/8` (engine check E-I4).
- Imported speaking characters become cast elements **with no board ID**.
- A new element has holds off and drops off. Turning drops on shows a minimum gap of
  **1 day** by default.
- The default working week is Monday to Friday; day dates skip weekends.

## Day breaks

- Automatic breaks by page eighths with a limit of 1/8 put each 1/8 strip on its own day,
  so a strip that exactly reaches the limit fits (the limit is inclusive). E-B1 in part.
- Marking a working day as a holiday reflowed every later day onto the next working date.

## Day out of days

Codes seen: SW, W, WF, SWF (one-day element), H, SWD and WD (last work day before a
drop), PW and PWF (pickup day), and `/` for a holiday. Two totals per element: worked and
held.

- **Holds off**: the span from first to last work day is drawn as a plain bar with no
  codes; nothing between work days is counted.
- **Holds on** (E-D5): every working day strictly between the first and last work day gets
  H. Weekends inside the span stay off days, uncounted. Example: work Thu and Tue gives H
  on Fri and Mon, held 2, worked 2.
- **Drop gap** (E-D1): the gap is the count of calendar days strictly between two
  consecutive work days, weekends and holidays included. Work Mar 1 and Mar 15 is a gap of
  13: a minimum of 13 drops, a minimum of 14 holds. A gap drops when it is at least the
  minimum.
- **One drop per element** (E-D2). Other qualifying gaps show holds.
- **Which gap drops**: the **longest** qualifying gap, not the first. Work on days 1, 5,
  11 and 21 with a minimum of 7 (gaps of 3, 9 and 13 calendar days): the 13-day gap
  dropped and the 9-day gap held. Work on days 1, 11 and 18 (gaps 13 and 8): the first,
  longer gap dropped. Two equal gaps of 13: the earlier one dropped. So: longest wins,
  ties go to the earlier gap. (Not yet separated: longest by calendar days or by working
  days. In these runs both measures agreed.)
- Days inside a drop show nothing and count nowhere.
- **Holidays** (E-D3, E-D7, E-D13): a holiday counts toward a drop gap (a 13-day gap grew
  to 14 and then met a minimum of 14). With holds on and the holiday inside the work span,
  the holiday counts as **worked** (EVE: 3 work days plus the holiday, worked 4). Inside a
  drop it counts nowhere. Outside the span it counts nowhere. With holds off it counts
  nowhere (EVE back to worked 3), even though the span bar is drawn across it.

## Still to check

Events (travel, fitting, rehearsal) and whether they block a drop, company travel days,
strips below the last day break, longest gap by calendar or working days, auto day breaks
with a strip over the limit, the DOOD summary export's columns, and the `.fdx` length
forms.
