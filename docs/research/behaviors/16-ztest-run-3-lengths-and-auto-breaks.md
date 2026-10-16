# Behavior note 16: ZTEST run 3, script page lengths and automatic day breaks (real app)

Source: our own checks in the real reference app, 2026-10-10, on a fresh throwaway
schedule `ZTEST Lengths` built from an invented script
(`scripts/make-ztest-fdx.py --lengths`). Results read from the board.
Clean-room: behavior in our own words, no copied text, no screenshots kept, the test file
stays outside the repo. This note outranks notes 01 to 13 wherever they disagree.

## Page lengths from a script file (E-I4)

Each scene's length attribute was written in a different form. What the board showed:

| Written | Read as |
|---|---|
| `8/8` | 1 |
| `1 0/8` | 1 |
| `1` | 1/8 |
| `4/8` | 4/8 |
| `10/8` | 1 2/8 |
| `2 1/8` | 2 1/8 |

So more than eight eighths roll into pages, whole pages show without a fraction, and **a
bare number counts eighths**, not pages.

## Automatic day breaks by page eighths (E-B1, E-B2, E-B6)

Limit 1 page, strips in board order: 1, 1, 1/8, 4/8, 1 2/8, 2/8, 3/8, 5/8, 4/8, 4/8,
1 2/8, 2 1/8, then the schedule's own empty starting sheet (0).

Days the app made: [1], [1], [1/8 + 4/8 + 1 2/8 = 1 7/8], [2/8 + 3/8 + 5/8 = 1 2/8],
[4/8 + 4/8 = 1], [1 2/8], [2 1/8], [0].

- **A day closes as soon as its total reaches or passes the limit.** It is not kept
  under the limit: the strip that crosses it stays in the day. This replaces our
  research guess of "pack while the day stays at or under the limit".
- A strip at or over the limit on its own fills a day alone.
- A break closes the last day, even one holding only a zero-length sheet.
- Day dates followed the default working week from the board's start date.

## Still to check

Auto day breaks by shoot time with banners (E-B3), by screen time, an event on a work
day (E-D6), the summary's Holiday column (E-D20), DOOD order with board IDs (E-D14).
