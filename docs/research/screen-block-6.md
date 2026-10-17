# Screen block 6 (2026-10-17, results)

One block of screen time, the owner at the Mac for the approvals, then unattended. Debug
`Method Schedule.app` (`cargo tauri build --debug --bundles app`, registered with Launch
Services so `.mtsc` and `.mtst` open from the system), on `~/ZTEST/ZTEST-Lantern.mtsc` (a
copy kept as `ZTEST-Lantern-before-blk6.mtsc`). The reference app on its ZTEST file, look
only. The shared screen lock was held for the block and released after it.

## Method Schedule: passed

| Check | Result |
|---|---|
| G167 | A day break shows the sun from a located sheet: scene 4 at ZTEST LOT (34.0522, -118.2437, UTC-8), Board > Day Breaks with sunrise and sunset on: day 4 reads "Sunrise 06:18 · Sunset 17:51" (right for Los Angeles on 4 March). The status line says why Locations is empty when no sheet has a location. |
| G168 | The calendar's category filter list closes on Escape. |
| G169 | The replace box reads "Replace with". Its tooltip didn't show on hover in the real window; the page test holds its text. |
| G170 | Locations: degrees and offset typed, Return sets them (offset normalized to -08:00), one click on Close closes. |
| G171 | Export Call Sheet Data: the save panel opens in the schedule's folder with the name filled in. |
| J179 to J182 | Outside changes, every branch: an unreadable file on disk is named as such with only Keep Mine and Later; Later puts up the standing "autosave is not writing into it" line; no unsaved changes offers Load Theirs, which loads, and one Undo takes it back; unsaved changes on both sides offers Merge, whose preview showed the one incoming change and no conflicts; Keep Mine saves ours and puts theirs in the backups; File > Revert to Backup lists backups newest first with their distance in changes, reverts, and Undo takes it back. |
| K183 | Two windows: copy two strips in one, Paste is on in the other, the paste brings the strips, the day break between them and their elements; one Undo removes them. |
| K185 (screen) | The \| rule after a field draws on every strip. The _ rule under the first field showed only in the column heading: bug 4 below, fixed. |
| K186 | Breakdown Designer: a field chip dragged to another row moves there and the Breakdown tab follows (Sunrise on the first row). One earlier attempt closed the dialog mid-drag; not seen again. |
| K187 (screen) | A banner in Monospaced draws in that face on the board. |
| K188 | The printed board (Save Board as PDF) fits 25 shoot days on 2 Letter pages, a strip line in about a screen line, the day break's sun included. |
| M193 | The daylight saving pop-up applies at once on a placed row: with the start moved to April, US and Canada gives 06:34 and 19:17, None 05:34 and 18:17. |
| M194 | Strip Layout, Category elements: picking a category (Cast, the only one in the file) lists its elements by name as a column. |
| Look A, decision 204 to 208 | The new top bar, segmented views, footer status line, set-apart scene cell and the three looks (Settings > Look: Paper, Graphite, Ledger) all render in the real window; Graphite's light day breaks keep their sun text readable. |
| Print Call Sheet, decision 207 | Saves a one-page Letter PDF with the day, the sun from the location, the scene, the cast and the timeline's note; "--:--" and "No call time set" without a call time. |
| Help menu, decisions 209 to 214 | About (icon, version, website, copyright), What's New, Report a Bug, Licenses and Notices (all 446 crates), Check for Updates ("Couldn't check for updates" while no release exists), Check for Updates Automatically ticked by default. |
| Template from the system | Opening a `.mtst` (as a Finder double-click does) asks about the open window's unsaved changes, then opens an Untitled schedule with the template's setup and no scenes. |

## Found and fixed (decision 218)

1. A disabled primary button (Replace in Find and Replace) was faded text on a faded accent, unreadable.
2. The calendar's Prep starts and Wrap ends fields cut their hint to "YYYY-MM-D".
3. In Ledger, the white button style also painted primary buttons, leaving white text on white (Settings' Save).
4. The set-apart scene cell (decision 206) drew its rule with a shadow that outranked a field's \| and _ rules, so the first field's rules never showed on strips.

## Noted, then changed (the owner's answers, decision 219)

- Opening a file from the system replaces the window's schedule (after asking about unsaved changes) instead of opening a new window, though the app keeps one window per schedule (decision 88). A design question for the owner.
- Licenses and Notices shows the third-party notices as raw Markdown.
- The board stops at its maximum width (1,100 px), leaving the rest of a wide window empty.

## Not run

- K184 (Find with a board beside the board), M192 (needs a day with shoot times past wrap), Layouts and Red Flag Types rename then click (G170's second half), K185 and K187 on paper, and the reference-app engine checks E-L5 and E-S34. Queued for the next block.

## The reference app, look only

- Its strip designer has a "Scale height" checkbox (no "scale to content" wording); ticking it changed nothing visible in the preview. Font size offers Default, Small, X-Small, Medium, Large and X-Large (ours: small, normal, large).
- Its banner editor has font style (bold, italic), font size (the same six), banner size, alignment, duration and a 40-color palette, and no font family choice: our Serif, Sans serif and Monospaced (decision 187) go beyond it.
- An empty banner left by closing its editor was deleted again; the file is otherwise as it was.
