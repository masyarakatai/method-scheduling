# Behavior note 11: classroom tutorials (MMS 6, 2019, third party)

Source: two third-party classroom videos by a film school instructor, "How to Create a
Schedule using MMS" (`P6k_D3JIj3o`, 22 minutes, July 2019) and a shorter companion
(`hN7tPE_z0HY`, 16 minutes, April 2019), read from auto-generated caption tracks in
`imports/research-cache/captions/`. MMS 6 era, classroom framing. Notes 01 to 07 win.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.

## What happens

- New schedule from the default template file, found on disk inside the app's data folder.
- Import of a tagged export file fills header fields and elements. In the walkthrough an
  empty first sheet had to be deleted after import (the new file started with one blank
  sheet). Imported sheets showed up in the unscheduled area, then were dragged onto the
  board.
- The sheet navigation mode was switched from board order to scene order before review.
- Pages typed as whole pages and eighths in two boxes (0 and 5 for 5/8). Tab moves between
  them.
- Estimated time typed as hours and minutes in two boxes; the instructor's floor is one
  hour per scene for students.
- Unknown element names prompt to create.
- Schedule built mostly with two tools: insert day break and banners.
  - Banners used as a title, then "production day N" headers, then timed blocks (setup and
    breakfast, makeup and wardrobe, lunch, blocking and rehearsal, company move, wrap),
    each with a duration.
  - Inserting the day break after the last strip of a day shows the day total of all strip
    and banner durations above it. Without banner durations the day total is too low.

## What this teaches us

- Many users schedule "by banner": non-shooting time lives in banners with durations, and
  the day break total is their day length. Banner durations must sum into the day total
  exactly like strip shoot times.
- Classroom users rely on a starter template; we need a good built-in default (our own
  category list and colors).
- Schools are a real audience (they also appear in the vendor's academic pricing; see the
  feature inventory's pain points).

## What this means for Method Schedule

1. Day total time = sum of strip shoot times plus banner durations, in integer minutes.
   v1 (E-B3 checks whether auto day breaks by time also count banners).
2. A new file should not start with a stray blank sheet that import leaves behind. v1.
3. Two-box eighths entry (pages, eighths) with Tab between them, plus one-box entry such as
   "1 3/8" or "11/8". v1.
