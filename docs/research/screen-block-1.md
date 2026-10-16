# Screen block 1: runbook

One unattended block of screen time: real-app checks in the reference product, then a live
check of our own app. Approve it once at the Mac and walk away. Expected length: about 60
to 90 minutes.

Clean-room: our own observations of normal UI use, recorded in our words; throwaway ZTEST
files only; exports go to the gitignored `/imports/ztest/`. Never open the owner's files or
the Open Recent entries that aren't ZTEST.

## Before starting (with the owner present)

1. Owner says go. Check the display is awake: `pmset -g assertions` shows UserIsActive 1.
2. Take the shared screen lock (`~/.claude/coordination/screen-lock.json`, holder
   `method-scheduling`, until +120 min, renew every 30 min) and request keep-awake.
3. Ask for full-screen control while the owner is still there, and **don't call
   release_full_control until the block ends**. The approval lasts until then.
4. Build the app first (`cargo build -p method-schedule-app`) and wrap it in the minimal `Method Schedule.app`
   bundle (HANDOFF.md, "Waiting for a live run").

## Part A: reference app (about 50 minutes)

Setup, as in runs 1 and 2:
- New schedule on this device, `ZTEST Block1`, saved in Documents. Import
  `~/Documents/ZTEST Lantern Creek.fdx`. Move every Boneyard strip to Default. Start
  date Mon 2027-03-01. Auto day breaks at 0 pages 1/8 (one strip a day).
- Cast and days: ADA 1, 11, 21; BRUNO 4, 7; CLEO 1, 11; DARIO 1, 11; EVE 1, 2, 3;
  FINN 5.

Checks, in order. Read the DOOD screen after each one and record cells and totals.

| Step | Check | Do | Record |
|---|---|---|---|
| A1 | E-D14 row order | Board IDs: CLEO `1`, ADA `2`, DARIO `10`, BRUNO `9`; EVE and FINN none | DOOD row order |
| A2 | E-L4 board IDs | Set FINN `07`, then DARIO `7`; set EVE `1a` while CLEO is `1` and ADA `2` | Which are refused, and the message |
| A3 | E-L4 names | Elements, Add `ada` to Cast | Accepted or refused |
| A4 | E-D6 | EVE holds on. Travel event for EVE on Mon Mar 1 (a work day) | Mar 1 cell, EVE worked |
| A5 | E-D22 | ADA holds and drops, minimum 7. Fitting for ADA on her pickup day (day 11) | Cell code on that day, and on the drop day |
| A6 | E-D23 | DARIO holds on. Mark Sat Mar 6 a holiday | Mar 6 cell, DARIO worked |
| A7 | E-C9 | Mark Sat Mar 13 a working day | Calendar look, day numbers after it, DOOD cell |
| A8 | E-E3 | Unavailable event for BRUNO on a hold day; camera test event for CLEO on a hold day | Codes and counts |
| A9 | E-D20 | DOOD Summary report, export Excel | Every column, especially Holiday |
| A10 | E-L5 | Try to rename and to move Cast Members; list the built-in categories and which can't be changed | Allowed or not |
| A11 | E-L6 | Merge FINN into a new Props element `LAMP`, then merge EVE into FINN within Cast | Allowed across categories? What survives |
| A12 | E-L7 | Move DARIO (`10`) to Stunts where an element already has `10`; then to a category with auto ID on | Refused, renumbered or kept |
| A13 | E-L8 | Lock FINN at `04` (if FINN survived A11, else BRUNO); Renumber from 1 | New IDs, and whether 4 was skipped |
| A14 | E-L9 | Turn on auto ID and holds as Cast defaults; use the apply-to-existing option if there is one | Which settings reach existing elements |
| A15 | E-B3 | In a new ZTEST schedule from `ZTEST Lengths.fdx`: shoot times 3:00, 3:00, 2:00 on three strips, a banner of 1:30 after the second, auto breaks by time 7:30 | Day contents and totals |

After the last step: export the DOOD Summary and the one-line to Excel into Documents,
then move them into `/imports/ztest/`. Close the schedule. Leave the app on its start
screen.

## Part B: our app (about 50 minutes)

Open `~/ZTEST/ZTEST-Lantern.mtsc` (rebuilt with the CLI as in HANDOFF.md) in `Method Schedule.app`.

| Step | Check |
|---|---|
| B1 | Elements tab: add a category, edit an element's board ID and rules, merge two, renumber, undo each |
| B2 | Day out of days grid: codes match `method-schedule dood`; sticky columns while scrolling; category picker |
| B3 | Calendar: mark a holiday and a company travel day, add a fitting event; board dates and the DOOD follow |
| B4 | Reports tab: each of the five reports previews; Print opens the system print sheet (cancel it) |
| B5 | Save as PDF into `~/ZTEST/`; open the PDF and check pages, headers and page numbers |
| B6 | File > Export, Excel and CSV, into `~/ZTEST/`; Export for Budget; open each file |
| B7 | Board at 1,500 strips (`method-schedule sample ~/ZTEST/big.mtsc`, start date and breaks set): fast trackpad scrolling (blank edges?), holding Down, Option-Down across day breaks and into the Boneyard, long banner text, VoiceOver on the selected row (decision 24) |
| B8 | Quit cleanly; `Recovery/` holds no lock file afterwards |
| B9 | Explain this count (decision 26): on the Day out of days tab click a Cast row's Total, then its Work and Hold; the panel's totals match the row and `method-schedule explain`. Click a name, Up and Down move the selection (and past 80 rows in `big.mtsc` it stays in view), Enter and ⌘I explain it. Click a date in the panel: the grid scrolls to that column and marks it. Undo an edit with the panel open: it is worked out again. Escape closes it |
| B10 | File > What Changed…: pick a copy of the file with a renamed element, a moved strip and a holiday; the groups and sentences match `method-schedule diff COPY FILE`. Change the title, then What Changed Since Saved lists it; after Save it says No changes. Cancel in the Open dialog does nothing |
| B11 | Merge Changes From… with the file saved twice (so it has a backup) and a copy made between the two saves, edited elsewhere: the base is "Found automatically" as the newest backup, with its reason. Copy the file again after the last save and touch its date back (`touch -t`): the base falls back to the file on disk with the warning. Pick another backup and the file on disk from the list |
| B12 | Merge, continued: a field both copies changed shows Yours, Theirs and Was; Keep theirs works the preview out again and keeps focus on the choice. Merge applies it, the status line says what happened, one Undo takes it all back, and the result opens again after Save. An Untitled schedule and the file's own file are refused in plain words |
| B13 | Drag and drop (decision 29): drag one strip within a day, across a day break, to another sub-board and into the Boneyard; the drop line shows where it lands, Escape cancels, one Undo puts it back. Shift-click a run and Command-click a second strip, drag both. Drag a day break toward the Boneyard: the line turns red and the drop is refused with a sentence. In `big.mtsc` hold a dragged strip at the bottom edge: the board scrolls by itself, new rows draw, and the drop lands where the line shows. Option-Up and Option-Down move a Shift-selected block |
| B14 | Banners (decision 30): Board > Insert Banner Above (⌘B) and Below (⇧⌘B) with text and `0:45`; the day's time on its break grows by it; double-click or Enter edits it; Delete asks, then removes it; a long banner text ends in an ellipsis; a refused duration (`90`) is said in the dialog |
| B15 | Boards and sub-boards (decision 31): Board > New Board, Duplicate Board, Boards and Sub-boards: add, rename, reorder, delete (second click confirms) boards and sub-boards; the Board picker switches the board; the Sub-board picker changes the Elements, Day out of days, Calendar and Reports tabs, File > Export and Export for Budget; a start date on the second sub-board gives its own dates; deleting a sub-board sends its scenes to the Boneyard; Undo brings each back |
| B16 | Sort Strips (decision 32): on a sub-board with banners and day breaks, sort by scene (with `10A`, `A1`), then set descending: banners on top, breaks gone, one Undo restores it all; sort the Boneyard |
| B17 | Find (decision 34): Edit > Find (⌘F) opens the panel at the right with focus in its box; type a set name: results appear as you type, in board order, with place and context. Match case, Whole word, Look in (Set only, Element names) and Where (the sub-board picked) narrow them. Click a sheet result: the Breakdown tab opens on that sheet with the field focused and marked; an element result opens the Elements tab on it; a banner result selects it on the Board (on a second board too); an event result opens the Calendar on its date. Enter and Shift+Enter in the box step through without leaving it; ⌘G and ⇧⌘G do the same from the board and from a Breakdown field. Check the keys reach the page in WKWebView (⌘F, ⇧⌘F, ⌘G, ⇧⌘G while a text field has focus, and ⌘F not opening a system find bar). Escape and Close shut it |
| B18 | Replace (decision 34): Edit > Replace (⇧⌘F); every result checked, All results a dash after unchecking one; Replace changes only the checked ones, the panel says how many, the results drop out, one Undo brings them all back and the panel searches again. Refusals in plain words, nothing changed: an element renamed to another's name in its category (any case), Cast renamed, a line break pasted into a Set, a category renamed to an existing one. Edit a field so a checked result stops matching, then Replace: refused as stale |
| B19 | Conflicts (decision 34): with two sub-boards dated over the same week sharing an actor, View > Conflicts (⌘7): the tab's red badge shows the count, each conflict names the actor, date and both sub-boards' days and scenes; click a scene (Board tab, its strip selected, the right sub-board picked), the date (Calendar of that sub-board on the date). Add a fitting for the actor on the other sub-board's working date: it shows under Informational, not as a conflict. Clear one sub-board's start date: it moves to Sub-boards not compared and its link puts the cursor in its start date. Move a strip so the clash goes away: the badge and the open tab follow; Undo brings it back |
| B20 | Red flags, managing (decision 36): Elements tab, pick ADA, Red Flag Types… adds Unavailable and Second job, rename one, Up and Down, Delete an unused one (second click). Add Red Flag…: Unavailable, From and To two scheduled dates, hard; then Second job, not before a later date, soft, with a note. `2027-3-2` and a To before From are refused in the dialog; Edit changes one in place; Delete asks. Delete Unavailable in the types dialog: refused with the sentence. Edit > Red Flag Types… opens the same dialog. One Undo per change, and after Save `method-schedule redflags FILE` agrees |
| B21 | Red flags, marks (decision 36): the flagged scenes' strips show the red ⚑ and right edge; hover gives the clash; VoiceOver reads the mark's name on the selected row. Calendar: the dates marked, the side panel lists the clashes. Day out of days: red outlines on those cells, the clash on hover. A fitting for ADA on a flagged non-shoot date marks the date and the cell. View > Red Flags (⌘8): badge count matches `method-schedule redflags FILE`, follows the Sub-board picker and moving a strip off the date; scene, date, event and name links go to the Board strip, the Calendar date and the Elements tab. In `big.mtsc` with a few open-ended flags, holding Down and dragging stay smooth |
| B22 | Merge and Duplicate (decision 37): select scenes 2 and 3 (Shift-click) and Board > Merge Strips…: the dialog lists the joined scenes, summed pages, elements gained and the set and time of day left out; Cancel changes nothing; Merge Strips leaves one strip at scene 2's place, selected, with the status line naming it; one Undo brings both back. Board > Duplicate Strip with two scenes selected: each copy right after its original, both selected; Breakdown > Duplicate Sheet (menu and button) shows the copy; one Undo each. A second board keeps its own order for the copies (right after the original, or the Boneyard's end) |
| B23 | Cut, copy and paste (decision 38): select two scenes and a day break, Edit > Copy (and ⌘C): the menu item is enabled with no text selected; paste into TextEdit gives a tab-separated line per strip. Select a strip lower down and ⌘V: copies land below it, selected, one Undo. ⌘X on a scene: it dims with a dashed edge; ⌘V below another strip moves it (no new sheet), the marks go. Cut on Board 1, show Board 2, paste: copies, with the sentence about the originals. Select a Boneyard scene and paste a day break: refused with the sentence. In the Breakdown tab click KIT, Command-click TOVE, ⌘C, Next, ⌘V: both added; ⌘X then ⌘V on another sheet moves them. In a Breakdown field and the Find box, ⌘X, ⌘C and ⌘V still edit the text. The Edit menu's Paste is enabled on the board only when strips are on the clipboard |
| B24 | Strip Layout (decision 39): Board > Strip Layout…: turn on Synopsis and Location, turn off I/E, make Synopsis Widest, move Location up under Scene, Save: the board's headings and strips follow, each still one line high with long synopses ending in an ellipsis; scrolling and dragging in `big.mtsc` stay smooth; one Undo puts the six back. The Reports tab's one-line follows the layout (synopsis in its own column, not under the set); Print and Save as PDF show the same columns. Save, reopen: the layout is back; `method-schedule diff` against the earlier copy names the strip layout change. Default Layout restores the six. Duplicate Board keeps the layout; New Board starts with the default |

## After the block

1. Release full control, the app locks and the screen lock. MMS on its start screen.
2. Write behavior note 17 (Part A) and add Part B to docs/APP.md "Live checks".
3. Update the engine checks table, add a decision for every rule that changed, change the
   engine and tests to match, and commit.
