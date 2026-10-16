# Feature review, 2026-10-13

Each review lane checked every `docs/parity/features.csv` row in its area end to end:
the engine, the file format, the command line, MCP and the app (its Rust commands and
the page in the harness), against the row's decisions, behavior notes and docs. Verdicts:
**ok** (works as the decisions say, covered by a test), **fixed** (a fix landed, with a
test where one can catch it), **gap** (left open; listed in docs/ROADMAP.md, M1).

## Boards and day breaks (BRD, DBK)

Checked on a 1,500-strip `sample`, a one-sheet sample, an empty schedule, a sub-board
with no strips, parked day breaks, a scene in parts, undo after every edit, and the page
in the harness (also `?big`, a real 1,500-strip view: 57 lines drawn, about 3 ms a
scroll step, no console errors). The ignored perf test
(`perf_edit_and_view_at_1500_strips`, release): view 1.21 ms, edit plus view plus JSON
2.34 ms at 1,500 strips (3.05 ms with red flags on most sheets), against 2.56 ms before
the review; the review changed nothing on the Rust side of the view.

| Row | Verdict | What was checked | Commit |
|---|---|---|---|
| BRD-01 | ok | Add, duplicate (independent copy), rename, reorder, delete boards; the last board can't go; one undo step each (`boards_and_sub_boards_are_managed`, `duplicated_board_is_independent`); harness Boards dialog | |
| BRD-02 | ok | A new sub-board starts on the date the app passes (today, D123); delete asks, sends scenes to the Boneyard, takes breaks and banners; the last one can't go | |
| BRD-03 | ok | Boneyard per board, parked breaks, banners refused there in the engine, the app, MCP and the strict reader | |
| BRD-04 | ok | A new sheet goes right after the current one on a sub-board (note 26), else at the Boneyard's end (D17); every other board's Boneyard too | |
| BRD-05 | ok | Drag within and between lists, a selection as one undo step, a drop on a day heading lands first in that day (note 25), the day left keeps its break | |
| BRD-06 | ok | Cut marks until the paste, copy makes new sheets, banners never paste into the Boneyard (`clipboard.rs` tests); harness context menu | |
| BRD-07 | fixed | Move To put strips at the end; note 24 says the top, with the selection going to the next strip where they were. Now so (D129); harness: a strip moved to the Boneyard lands first and the next row is selected | 9c2b0b3 |
| BRD-08 | ok | Delete on a scene asks, deletes its sheet on every board, one undo; a part of a scene in parts goes alone | |
| BRD-09 | fixed | Up to four rules, each once (D122); natural scene order; banners first, breaks dropped; empty and Boneyard sorts. The command line and MCP had no sort: `sort FILE FIELD[:desc],...` and `sort_strips` added, with tests (D131) | df5923d, d35e208 |
| BRD-10 | ok | Banners above or below the selection, durations, edit by double-click, undo (harness and `banners_are_inserted_edited_and_deleted`); CLI `banner`, MCP `insert_banner` | |
| BRD-11 | fixed | Bold, nine colors, one or two lines, size, alignment; editing the text keeps the style; unknown colors refused by the reader, the CLI and MCP. In-place editing and italic stay a deliberate difference (D126). MCP.md said `set_banner` works in the Boneyard, where banners never are: fixed | 6fb2f6a |
| BRD-12 | fixed | Find stepped through results but marked nothing on the board; note 24 has highlights and select all. Now every strip in the results is outlined while the panel is open, and Select on Board selects them (D130); harness checked, marks clear on close | 9c2b0b3 |
| BRD-13 | ok | A color per I/E and time of day pair, first match wins, text colors per strip type, saved and round-tripped (`strip_colors_*` tests) | |
| BRD-14 | ok | The field picker and widths; harness: a field moved to line 2 over three columns saves as one call and the board draws two-line strips | |
| BRD-15 | ok | Four lines, spans, alignment, bold, italic, size, text and spacer fields; the reader refuses explicit defaults (`the_strip_designer_*` tests) | |
| BRD-16 | ok | Named layouts of three kinds, picked per board and by the one-line, layout files load anywhere, deleting one sends its boards back to their own | |
| BRD-17 | ok | Day headings, breaks, totals and banners hidden and shown again in the harness; printing follows; kept in this browser only | |
| BRD-18 | fixed | Cast count and Elements fields work (`the_pages_layout_fields_are_the_engines`). Note 25: the reference app's element quantity field asks which elements it counts; ours counted every category on the board. Fixed after the review: the strip designer picks the elements an element count counts (D136); note 27 found it counts one category, which ours now does (`an_element_count_counts_its_category`, D147) | 24e64c3 |
| BRD-19 | ok | Double-click opens the sheet, or a banner's editor (harness) | |
| BRD-20 | ok | Print Board through the reports' print path with the board's layout, selected strips, Boneyard, paper (harness `__printed`) | |
| BRD-21 | ok | A pair's own text color wins over the scene text color; round-trips | |
| DBK-01 | ok | Day number, date, scenes, pages and time on each break; `days` in the CLI and MCP | |
| DBK-02 | ok | Screen time, sunrise, sunset, quantities and text, one undo step (`the_day_look_is_one_step_and_days_carry_their_extras`) | |
| DBK-03 | ok | The shoot day strip above each break, saved with the board, printed | |
| DBK-04 | ok | Breaks move by drag, keys and Move To, and delete without asking (D54) | |
| DBK-05 | ok | Parked breaks: no day, counted nowhere; a Boneyard sort drops them as a sub-board sort drops breaks; round-trip | |
| DBK-06 | ok | Above the topmost or below the bottommost selected strip; refused on a Boneyard selection | |
| DBK-07 | ok | Engine rules (close on reaching the limit, last day closed, a strip over the limit joins the open day); the CLI and MCP | |
| DBK-08 | fixed | Shoot time and screen time limits. The command line took a limit of 0 (a day per strip) where the app and MCP refuse it; now refused, with a test; "1 shoot days" and "1 strips" now read in the singular | 6fb2f6a |
| DBK-09 | ok | Remove All asks nothing, one undo step; an empty sub-board says there is nothing to remove | |
| DBK-10 | ok | Banner durations count toward a shoot time limit (`banners_count_toward_a_time_limit`) | |
| DBK-11 | fixed | Continue on Next Day, parts deleted alone, a part refused in the Boneyard. A file with parts had no format test: added (round-trip; a part in the Boneyard refused). Sorting a sub-board put a scene's parts side by side, so after breaking again they could share one day; fixed after the review: sort moves the parts as one block and the automatic breaks keep each part in its own day (`sorting_keeps_a_scenes_parts_together_and_breaking_keeps_them_apart`, D134) | 6fb2f6a, c30ba76 |

Counts: 25 ok, 5 fixed, 2 gaps (BRD-18, DBK-11) plus one more found on the way (the
strip designer's switches for the red flag and conflict marks, note 25, BRD-14; ours
always showed them). All three fixed after the review: BRD-18 and the switches in
24e64c3 (D136), DBK-11 in c30ba76 (D134).

Outside this area, SPL-06's note said "no highlight on the board"; with BRD-12's fix that
was no longer true, so the note now says the matches are outlined (D130). Its status is
unchanged.

## Import, breakdown and elements (IMP, BRK, ELM)

Checked with the command line on the Harbor Light fixture, an empty `.fdx`, a file that
is not XML, a scene of 1,200 pages (capped at 999), headings with no time of day or a
lower-case slug, cues that are only an extension, a re-import into the same schedule, a
new schedule and a 1,500-sheet `sample` (`elements` in 0.06 s); MCP through the test
fixture; the app's Rust commands with undo after each edit; the Breakdown and Elements
tabs in the harness (quick add, Category;Name, quantity, step by scene number, Delete
Sheet, merge, renumber with and without a sorted column), no console errors.

| Row | Verdict | What was checked | Commit |
|---|---|---|---|
| IMP-01 | ok | `fdx.rs` tests (lengths, entities, nesting, DOCTYPE and size refusals, mutated files never panic); CLI list and `--into` on odd, empty and non-XML files; a 1,200-page scene capped at 999 pages | |
| IMP-02 | ok | A skip row (vendor formats); nothing to check | |
| IMP-03 | ok | Speaking characters only, names matched ignoring case (`ana` joins `ANA`), no board IDs; a cue that is only `(V.O.)` adds nothing | |
| IMP-04 | ok | `reads_production_tags_only_when_asked` and the tagged import tests; CLI `--tags` and MCP `tags` go through the same function | |
| IMP-05 | fixed | New sheets at the end of every board's Boneyard. The app's count notice had no test, and the CLI said "1 sheets", "1 scenes", "1 elements": the notice is now a tested function, with a session test that a second import lands after what is there on every board, keeps a set title and is one undo step | 775ee59, 98e6a25 |
| IMP-06 | ok | `splits_heading_parts`, `I/E.`, `EXT./INT.` and a heading with no time of day; lengths as written (a bare `2` is 2/8, D12) | |
| IMP-07 | ok | `fountain.rs` tests and the fixture through the CLI | |
| IMP-08 | ok | A new schedule has no sheets; now under test (the session test above) | 98e6a25 |
| IMP-09 | ok | `sniff_image` by bytes for all seven types, whatever the name says | |
| BRK-01 | ok | Strict times and pages (`times_and_pages_are_read_strictly`), one undo step per field, refused values change nothing; MCP `set_sheet_fields` | |
| BRK-02 | ok | I/E and time of day offer the values other sheets use, deduplicated ignoring case | |
| BRK-03 | ok | Field order one step and checked; round trip | |
| BRK-04 | ok | The place line and its jump (`the_sheet_view_shows_fields_groups_place_and_neighbors`). The harness fake writes `Main` where the app writes `Main, day 1, Mon 2027-03-01`; fake only | |
| BRK-05 | fixed | After the current sheet on a sub-board, else the Boneyard's end. MCP had no way to add a sheet: `new_sheet` added, with a test | 980495d |
| BRK-06 | ok | Copies after each strip, every field and element (`duplicate_places_each_copy_after_its_strip_as_one_step`); CLI and MCP | |
| BRK-07 | ok | Merge preview and merge, selection order, dropped values (`merging_sheets_*`, `merge_strips_previews_*`); CLI and MCP | |
| BRK-08 | fixed | Asks first (D125). Stepping by scene or sheet number, Delete Sheet then showed the next sheet in board order; now the next in the order the tab steps by (harness: scene 9 deleted shows scene 4). MCP had no delete: `delete_sheet` added | 66cbb8d, 980495d |
| BRK-09 | ok | Board order, a scene in parts at its first part (`the_sheet_steps_by_the_order_asked_for`) | |
| BRK-10 | ok | By board order, sheet number or scene number; see BRK-08 for Delete in those orders | 66cbb8d |
| BRK-11 | fixed | Hide empty works in the harness and is remembered in the browser; it was page logic with no automated test. Fixed after the review: the rule is in `ui/js/logic.js`, run by `node --test` in CI (D137) | 24e64c3 |
| BRK-12 | ok | A box's Add matches within its category and adds a new name there (`quick_add_tags_creates_and_removes_elements`) | |
| BRK-13 | ok | Matches, then Enter adds the name in the category shown beside the field (New in). The reference app asks with a list (note 24); ours shows the category first, a difference D17 chose | |
| BRK-14 | ok | `clipboard.rs` tests: cut marks until the paste, paste once per element, one undo step | |
| BRK-15 | fixed | Quantities 1 to 99,999 and daily (tests in core and app). Enter in the Quantity dialog closed it and pressed the quantity button again, reopening it; the dialog code now keeps Enter in a field from reaching the opener | 2c0f4ad |
| BRK-16 | ok | `sheet_images_are_paths_with_thumbnails`; round trip | |
| BRK-17 | ok | Round trip, natural order, Go to Sheet Number; MCP `number` | |
| BRK-18 | fixed | Decimal time is tested in Rust (`show_decimal_hours`, typed decimal times). `Category;Name` in Quick add works in the harness (a new category is made); the split had no automated test, now in `logic.js` under `node --test` (D137) | 24e64c3 |
| BRK-19 | ok | `hidden_fields_and_widths_save_with_the_order`; round trip | |
| ELM-01 | fixed | Add, rename (cast too), reorder user ones, unique ignoring case, delete when empty (`categories_are_renamed_reordered_*`). MCP could only create one through `add_element`: `add_category`, `delete_category` and `name` and `position` on `set_category` added | 980495d |
| ELM-02 | ok | `a_new_schedule_lists_our_default_categories` in core and the format; CLI `new`, MCP `new_schedule` | |
| ELM-03 | ok | One more than the highest whole number, `07` counts as 7, `2A` ignored | |
| ELM-04 | ok | A default for new elements, Apply to Existing reaches the rest (D120); MCP `set_category` | |
| ELM-05 | ok | `category_defaults_apply_to_new_elements_only` | |
| ELM-06 | ok | `quantities_stay_with_their_elements`, the daily totals; MCP | |
| ELM-07 | ok | Add, rename, move, delete with values following, one or several lines (`custom_properties_belong_to_the_category`, the app's session test); MCP `set_category` | |
| ELM-08 | ok | `applying_defaults_is_one_step`, `defaults_apply_to_existing_elements_only_when_asked` | |
| ELM-09 | fixed | Eight header fields in the Elements tab and the DOOD (D118); APP.md still listed four: fixed | af9c6de |
| ELM-10 | fixed | Rename unique ignoring case. The engine let `add_element` make an element with a blank name (MCP could); now refused. MCP had no rename: `rename_element` added | 775ee59, 980495d |
| ELM-11 | fixed | Only the app checked that a board ID is letters and digits, at most 12; MCP `set_board_id` and `add_element` took `1 A`, `1-A` or 13 characters. The rule is in the engine now, with core and MCP tests | 775ee59 |
| ELM-12 | ok | Notes through the session and MCP `set_element_details` | |
| ELM-13 | ok | Values per element, lost on a move; Elements report, CLI and MCP `list_elements` | |
| ELM-14 | ok | `images_are_paths_beside_the_schedule`, `image_reads_are_careful` | |
| ELM-15 | ok | `the_elements_view_lists_categories_and_one_categorys_elements`; CLI `elements` on 1,500 sheets | |
| ELM-16 | fixed | Events, conflicts and red flags columns are tested in Rust; the column sort had no automated test (harness checked; see ELM-17), now in `logic.js` under `node --test` (D137) | 24e64c3 |
| ELM-17 | fixed | The dialog says "in the order shown", but with the table sorted by a column Renumber still went by board ID and name; the page now sends the order it shows (an order that misses or doubles an element is refused). Enter in its Start field reopened the dialog (as BRK-15). MCP had no renumber: `renumber_elements` added | af9c6de, 2c0f4ad, 980495d |
| ELM-18 | fixed | One category only (E-L6; the roadmap's owner question removed). A merge left locks naming the merged-away cast, so drift showed them dropped; the survivor now takes their place in locks. MCP had no merge: `merge_elements` added | c17a6a1, af89fc1, 980495d |
| ELM-19 | fixed | Refused when the name or board ID is taken (D45), keeps its ID; MCP had no move: `move_element` added | 980495d |
| ELM-20 | ok | Links through every level with a cycle guard, retroactive (`linked_elements_chain_*`); MCP | |
| ELM-21 | ok | `element_days` view test; no start date says so | |
| ELM-22 | ok | Its own setting decides (D120); round trip; MCP | |

Counts: 36 ok, 11 fixed, 3 gaps (BRK-11, BRK-18, ELM-16: page logic with no automated
test), all fixed after the review by the page logic tests (24e64c3, D137). MCP also gained `delete_element`, which had no row of its
own. The Enter fix is in the dialog code every dialog shares, so other dialogs that close
from a field are fixed too.

## Day out of days and reports (DOD, RPT)

Checked against behavior notes 14 to 17 (and 18 to 26 where a row cites them) and
decisions 10 to 12, 20, 47, 59, 116 to 121. Engine: every counting rule of notes 14 and
15 has a test in `dood.rs` (drop length and ties, holidays in and out of the span and the
drop, events before the start, events splitting a drop, company travel days, events on
work days, the shared working week, scenes spanning days, two strips on a day, strips
after the last day break and in the Boneyard). Command line: every report command on a
1,500-sheet `sample` (all under 0.1 s in release), an empty schedule and one with a start
date but no strips. MCP: `dood`, `one_line`, `report`, `report_list` and the setters
through the existing tests. App: each report's data in Rust tests, and in the harness
(port 8764) every report of the list previewed, printed and saved as PDF, the DOOD tab
with 300 rows by 140 dates, Weekends off (grid and vertical), Vertical, Sort, Codes, the H
box with undo, Explain, report Up and Copy, a header layout added; no console errors.

| Row | Verdict | What was checked | Commit |
|---|---|---|---|
| DOD-01 | fixed | Grid per board, sub-board and category, header fields as categories. The command line's `dood` text and the summary spreadsheet mixed every category in board ID order; now a category at a time (`dood::group_by_category`, D128) | f7db093 |
| DOD-02 | ok | Every code of notes 14, 15 and 18 (`SW`, `WF`, `SWF`, `SWD`, `PW`, `PWF`, `FTH`, `TRSW`, `FTPW`, `FTWF`, `/`, `M`) pinned in `dood.rs` tests | |
| DOD-03 | ok | Holds strictly between first and last work, weekends uncounted (E-D5) | |
| DOD-04 | fixed | One drop in the longest gap, ties to the earlier, minimum inclusive (E-D1, E-D2). A drop ending on an event day: the event code, H, then D or P, as the real app (E-D22, note 27, D139) | 4459c41 |
| DOD-05 | ok | Travel, fitting, rehearsal count as worked anywhere, split gaps, never move S or F; unavailable, camera test and post kinds count nowhere | |
| DOD-06 | ok | Holidays and company travel days count only inside a held span outside the drop, holds off counts neither (E-D3, E-D7, E-D13, E-D17, A6) | |
| DOD-07 | ok | Start and finish from strips only, with an event before the start (E-D10); events only give no start | |
| DOD-08 | ok | Summary Work, Hold, Start, Finish, Total = Work + Hold, no Holiday column (D117); the exchange keeps its holiday count and its total equals the summary's | |
| DOD-09 | ok | Columns from the first to the last date of the category's rows (D20), sticky heads, virtual rows past 80 (harness 300 x 140) | |
| DOD-10 | ok | Vertical view: a row per date, counts at the foot; Weekends off hides those rows (harness) | |
| DOD-11 | ok | Natural board ID order, no ID last, ties by name (E-D14) | |
| DOD-12 | fixed | Heading sorts. The Total heading sorted by work and was marked sorted with it; Total is no sort key, so it no longer sorts; a test checks each heading's key against the engine's (D128) | 02c61cf |
| DOD-13 | ok | Up to four keys, stable, blanks last both ways (`rows_sort_by_several_keys_stably_with_blanks_last`); CLI `--sort`, MCP sort | |
| DOD-14 | ok | H and D boxes call `set_element_field`, one undo step (harness: box, then Undo restores) | |
| DOD-15 | ok | One working week: a six-day week and a day off of its own move the days and the holds (new test) | 1ddc8ee |
| DOD-16 | ok | Code letters in the grid, reports, spreadsheets, CLI and MCP `report`; MCP `dood` keeps the usual letters on purpose, now said in MCP.md (D128) | f7db093 |
| DOD-17 | ok | Explain follows the cells (`explain.rs` reads `dood` and `gaps`); the harness panel opens from Work | |
| DOD-18 | ok | The reference app's six key names in the dialog, CLI and MCP (`sort_keys_take_the_reference_apps_words`) | |
| DOD-19 | fixed | Weekends box hides Saturday and Sunday columns and vertical rows, counts unchanged, remembered per viewer; the DOOD report follows it and pages by whole weeks from the week's first day, as the real app (note 27, D141) | |
| RPT-01 | fixed | One-line by day, banners, cast IDs, layout columns. An omitted scene was never struck: the page read an `omitted` the Rust never sent; now sent for report lines and board strips, and the CLI and MCP mark it (D128) | 4459c41, 11ee528 |
| RPT-02 | fixed | Shooting schedule by category; the omitted mark as RPT-01 | 4459c41, 11ee528 |
| RPT-03 | fixed | DOOD report a category at a time with its legend. The CLI grid export ignored `--sort` and `--no-holds`, a one-category grid took every category's dates, and an out of range report said there was nothing (D128) | f7db093, 1ddc8ee |
| RPT-04 | ok | Element list counts sheets and pages anywhere, days on the sub-board, omitted scenes left out | |
| RPT-05 | fixed | Breakdown by strip; the omitted mark as RPT-01 | 4459c41 |
| RPT-06 | ok | Breakdown by day: each element once with its scene count, daily quantities summed with partial marks (D91) | |
| RPT-07 | fixed | Calendar report in the user's week, weekdays only, red flags. Past two years it printed 24 months without a word; the date note now says so (D128) | b6702ad |
| RPT-08 | ok | Element detail with work dates and scenes, category and element filters, date range | |
| RPT-09 | ok | Daily category totals, quantities and partial totals, unused categories left out | |
| RPT-10 | fixed | Vertical one-line. A cast member left off the board had a row although it has no ID on the strips; now none (D128) | f7db093 |
| RPT-11 | fixed | Conflict and red flag reports with range and filters. A conflicts report whose range missed every conflict said the board had none (D128) | 1ddc8ee |
| RPT-12 | ok | Multi-unit merged by date, undated sub-boards named, element filter drops days | |
| RPT-13 | fixed | Date range, production header, synopsis, page per day, banners. With Cast names on, a cast member left off the board showed by name though not by ID; now neither (D128) | f7db093 |
| RPT-14 | ok | Category filter, strip colors, red flag marks, text size, header pick | |
| RPT-15 | ok | Designer columns on four lines, spans, looks, sizes; round trip and strict reader (`report_designer_columns_round_trip_and_plain_ones_stay_names`) | |
| RPT-16 | ok | Up, Down, Copy, Delete (the last of a kind refused); each report setup edit undoes and redoes as one step (new test) | 1ddc8ee |
| RPT-17 | ok | Every production field in the file, the dialog, CLI flags and MCP; round trip | |
| RPT-18 | fixed | Header layouts. A deleted layout's id was handed to the next new one, so a report still picking it printed with the new layout unasked; ids now go past every pick (D128), the harness fake too | d1b0310 |
| RPT-19 | ok | Shoot day lines, day breaks, total count, page per day or week, category columns, named strip layout (`headers_page_breaks_totals_and_category_columns`) | |
| RPT-20 | ok | Turned text, dark day columns with pages and time, production info beside the cast (D127 widths) | |

Counts: 27 ok, 12 fixed, 0 gaps among the rows. Also fixed on the way: main did not
build for a while (two lanes each added the same `count` helper to the CLI; f3b509e).
Left for the roadmap (M1): spreadsheets from the app follow none of a report's options
(range, filters, without holds), by D59, unconfirmed against the reference app; the DOOD
report prints every date whatever the Weekends box (D121), unconfirmed; the vertical
grid and Cast names leaving out a cast member off the board, and an event day at a
drop's edge showing only its code, are our plain readings to confirm in a screen block;
the DOOD tab's Weekends box and Vertical view had no automated page test (now under
`node --test`, 24e64c3, D137).
Settled in screen block 4 (behavior note 27): an event at a drop's edge takes the D or P
after its H (D139); the DOOD report follows the Weekends box and pages by whole weeks
(D141); the vertical one-line keeps a row for a cast member off the board (D142); the
spreadsheets ignore the report's range in the reference app too (D59 stands).

## Calendar, events and red flags (CAL, EVT, RFL)

Checked on the 1,500-sheet `sample` (CLI `calendar`, `conflicts`, `redflags`, `prep`,
`budget`, `check`; MCP over stdio: `add_red_flags`, `add_event`, `conflicts`,
`red_flags`, `delete_event`, `set_weekly_off`, `days`), the harbor fixture, an empty
schedule, an undated sub-board, holidays and company travel days, an omitted scene, a
weekly day off marked back to working, two dated sub-boards, nameless events of every
kind, undo and redo of each edit, and the page in the harness on port 8763 (Month and
Weeks, the date menu, a holiday and its star, a travel event on a work day raising the
conflict badge, dragging the event and a strip to other dates and undoing both, All and
None, the Red flags tab adding two runs of dates; no console errors, viewport left as it
was). The strict reader was probed with hand-edited files (`check`).

| Row | Verdict | What was checked | Commit |
|---|---|---|---|
| CAL-01 | fixed | Start date, weekly days off, days off, holidays, company travel, back to working (`marking_dates_reflows_the_board_one_step_each`, the calendar property test). The strict reader took a date listed as two kinds, a date twice, marks out of order and a week with all seven days off; `check` now refuses them (`Calendar::problem`, FORMAT.md Consistency). MCP could not set the working week: `set_weekly_off` added | dfc62c3, 5e6c630 |
| CAL-02 | ok | Shoot days skip off days, holidays and travel days; the board, the DOOD, conflicts (a holiday moving a sub-board's days onto a date another sub-board shoots) and clashes follow | db13056 |
| CAL-03 | ok | Week start and date format in `settings.json` (`settings_default_to_autosave_on_and_are_kept`); the month grid with Monday first keeps each date's kind and day (test added) | c9f7fc6 |
| CAL-04 | fixed | The month: six weeks, greyed dates outside it, Previous and Next across the year (`the_calendar_view_shows_a_month_of_marks_days_and_events`). APP.md said Sunday to Saturday only: fixed | 2436d07 |
| CAL-05 | ok | Strips, pages and length per date from Rust; Today, Go To and First Shoot Day in the harness; Go To's reading is now under `node --test` (D137) | 24e64c3 |
| CAL-06 | ok | No test moved a strip by its drop place: one now drops a strip on a later and an earlier date, each lands at the end of that day, the day left stays, two undos restore it; harness drag and undo | c9f7fc6 |
| CAL-07 | ok | The strips carry their elements; the category then element filter, checked in the harness, is now under `node --test` (D137) | 24e64c3 |
| CAL-08 | fixed | Strip Fields kept in the schedule (`calendar_fields_live_in_the_schedule`). APP.md still gave the old default (scene, set, pages): fixed | 2436d07 |
| CAL-09 | ok | The Calendar report with weekdays only or the full week (`breakdowns_detail_vertical_and_calendar`, `the_calendar_month_has_strips_drop_places_and_the_users_week`) | |
| CAL-10 | ok | CLI `prep` refuses a wrap before the prep start; MCP `set_prep_wrap`; the exchange writes them only when set (`prep_and_wrap_are_written_only_when_set`) | |
| CAL-11 | fixed | Round trip and left out at the default; `"calendar_fields": []` read as the default and wrote back without the key; refused now | 5e6c630 |
| CAL-12 | ok | The date menu from a right-click and Actions…: the current kind shown as now and off, holiday then undo (harness); each action is an edit already under test | |
| CAL-13 | fixed | Eight switches, All and None, kept in the schedule, one undo step (`the_view_switches_are_kept_in_the_schedule`); harness None and All. `"calendar_hidden": []` was read; refused now | 5e6c630 |
| CAL-14 | fixed | The weeks view ran from the first to the last date (`the_weeks_view_runs_through_the_schedule`), so an event two or more years from the shoot (a mistyped year) moved the first week there, and the 110-week cap could then leave the shoot days out. Fixed after the review: the span is the shoot's and the month's, widened by events within 26 weeks; farther ones are counted in a note (D135) | 2afe360 |
| CAL-15 | ok | Star and plane in both views whatever the switches (harness); the date kinds from Rust | |
| EVT-01 | fixed | Kind, optional name, dates, elements, one undo step (`events_are_saved_checked_and_deleted`). The longest event was 366 days in the app and 367 over MCP; the engine's `set_event` now holds the one limit (`MAX_EVENT_DAYS`). MCP could add an event but not delete one: `delete_event` added | 6ed71b1, dfc62c3 |
| EVT-02 | ok | Dragging an event keeps its length, one undo step (`an_event_moves_whole_as_one_step`); harness drag and undo; New Event from an element and a sheet opens the editor | |
| EVT-03 | ok | Unavailable and camera tests never count (`post_production_events_never_count_in_the_day_out_of_days` and the DOOD tests) and, new, never conflict on a work day | db13056 |
| EVT-04 | ok | TR, FT and R on the DOOD, `FTW` and the rest on work days (DOOD tests); harness DOOD | |
| EVT-05 | ok | One element on one date in two sub-boards, other boards never compared, undated sub-boards listed, header values left out on purpose (D124); conflicts follow holidays and omitted scenes (test added); CLI and MCP on the sample | db13056 |
| EVT-06 | fixed | A nameless event's conflict mark read "KIT at  on a work day" on strips and dates, and its clash link in the Conflicts tab was an empty button; every list now calls it by its kind (`Event::title`). The Breakdown tab's conflict line is now under test | 6ed71b1, db13056 |
| EVT-07 | ok | Travel, fittings and rehearsals on a work day are conflicts; unavailable, camera test, other and post-production never are (test over every kind); harness badge | db13056 |
| EVT-08 | ok | Ten colors or none, refused when unknown by the app, MCP and the reader; round trip (`properties_images_quantities_links_and_event_kinds_round_trip`). An explicit `"color": null` was still read; refused since the reader writes back what it read (D133) | a920ee3 |
| EVT-09 | ok | Ten post-production kinds in the file, the app and MCP; never in the DOOD, never a conflict | |
| RFL-01 | fixed | Flags per element with types, ranges and open ends (`work_on_a_flagged_date_clashes`, `edits_are_checked`); the reader refused bad dates and missing types, but read `"hard": false` and `"note": ""`, which it never writes; refused now | 5e6c630 |
| RFL-02 | fixed | Clash marks on the board, calendar, DOOD, Breakdown, Elements table and reports (`clashes_mark_strips_dates_cells_and_the_tab`). A clash at a nameless event showed "No name" on the Red flags tab and `event ""` in the command line: now its kind | 6ed71b1 |
| RFL-03 | ok | The table, filter, export and the Red flags report with the table (`red_flags_table_lists_every_flag`, `dood_elements_totals_and_the_flag_reports`) | |
| RFL-04 | ok | Standard types match the event kinds, renamed and recolored, never deleted, added to older files on request (`built_in_red_flag_types_match_the_event_kinds`, the MCP test) | |
| RFL-05 | fixed | Several elements and dates at once, runs joined, one undo step (`several_at_once_the_table_and_the_standard_types`, `red_flags_for_several_elements_and_dates_at_once`); harness two runs. With 1,000 elements and 1,000 dates each allowed, one click could add a million flags: at most 10,000 at once now, in the engine and the app. Picking differed from note 25: fixed after the review, a click replaces the pick, Command-click adds or removes, Shift-click a run (D138) | 2436d07, 24e64c3 |
| RFL-06 | ok | Delete Checked all or nothing; its undo and redo are now under test; APP.md's Not yet list still named this table: fixed. The harness's red flags export was put right by another lane (987fa8b) | 2436d07, f87d414 |

Counts: 19 ok, 10 fixed, 1 gap (CAL-14), with three more findings that belong to no
single row: the reference app picks red flag dates by replacing the pick on a click and
adding with Command (note 25), where ours toggled each date; the reader took `null` for
an optional key and `false` or `[]` for many later keys, which FORMAT.md's
explicit-default rule refuses (fixed here only for the calendar and red flag keys
above); and the reader took an event longer than 366 days, which no edit can make. All
fixed after the review: CAL-14 in 2afe360 (D135), the date picking in 24e64c3 (D138),
the reader across the whole file and the event length in a920ee3 (D133, with a test that
tries every key of a schedule with everything set as `null`, empty and left out).

## Files, exports, split screen, keys, saving and beyond (FIL, EXP, SPL, KEY, SAV, BYD)

Checked on a 1,500-sheet `sample`, a 30-sheet one, an empty schedule, a Boneyard-only
one (imported, never scheduled), Harbor Light scheduled, with day breaks removed, and
after a script revision that omits a scene; every command the CLI help lists run on each
of them (`every_command_in_the_help_runs` now keeps the help and a run of each command
in step), every MCP tool against MCP.md (`mcp_md_lists_every_tool`: the group table, the
counts line and a paragraph per tool), the page in the harness on port 8766 (every Plan
dialog, every split pane and Single View, the shortcut sheet and its filter, the strip
menu and Shift+F10, Delete on a scene and on a day break, undo, every report and format
in File > Export, Recovered schedules), no console errors.

Bugs fixed: the Day Timeline, Turnarounds and Rest and drift views wrote the word "null"
into the dialog whenever a day had no warnings, every day was checked or no actor was
unchanged (`replaceChildren` with a null child; a test now refuses one in every page
script); opening Find beside a split pane squeezed the board to 34 px; deleting a day
break, sheet, category, sub-board or board left call times, turnarounds, pins, omitted
marks and locks pointing at nothing in the file (a lock of a deleted board then failed in
Drift); an omitted scene's eighths reached the one-line Day Totals formulas while the
day's pages left them out; the harness refused the red flags export the app makes.

| Row | Verdict | What was checked | Commit |
|---|---|---|---|
| FIL-01 | ok | n/a: our file is the open `.mtsc` (EXP-07) | |
| FIL-02 | ok | n/a: native files are off limits | |
| FIL-03 | ok | One window per schedule: each window's schedule, title, close question and tokens apart (`docs.rs`), cascade (`windows.rs`), New Window and Open in New Window in the harness. No test found the window already holding a file, which opening it again relies on: added (`the_window_holding_a_file_is_found`, any spelling of the path). Moving strips between windows stays a gap (D86) | 35ca065 |
| FIL-04 | ok | Save as Template and New from Template (app test and harness), CLI `template` and `new --from`, a template refusing sheets, strips, banners and locks from every CLI write, MCP refusing every write to one, `.mtst` round trip. A CLI template could be written under any extension (`.mtsc` too); since 25f3bd9 `template` refuses an output not ending in `.mtst`, as the app and MCP (D132) | 25f3bd9 |
| FIL-05 | ok | Layout files saved and loaded (app test, CLI `--export` and `--import`); a file that is not a layout file, or empty, is refused with a sentence; a schedule with no named layouts says so instead of writing an empty file | |
| FIL-06 | ok | n/a: rolling backups and diff and merge stand in | |
| FIL-07 | ok | n/a | |
| FIL-08 | ok | Open Recent ordered, deduplicated, capped at 10, pruned and kept; same-named files told apart (`safety.rs` tests) | |
| FIL-09 | ok | 200 steps (`undo_history_is_capped`), every edit one step through `Session::edit`, undo and redo in the harness after Delete | |
| EXP-01 | ok | PDF of every report through the print path (other lanes' report tests; harness `__printed`) | |
| EXP-02 | ok | DOOD summary, grid, one-line and element list as xlsx and CSV, CLI and app, on every test schedule; DOOD needs a start date and says so | |
| EXP-03 | ok | Element detail, breakdown, breakdown by day and board as xlsx and CSV (`element_detail_breakdown_and_board`), every one from File > Export in the harness | |
| EXP-04 | ok | n/a | |
| EXP-05 | ok | n/a | |
| EXP-06 | fixed | The budget exchange must stay additive and is shared with Method Budget: nothing held the writer to EXCHANGE.md. A test now checks the written file has exactly the fields of its three tables, in their order, every optional one present, and that its example has the same shape. CLI `budget`, MCP `budget_exchange`, File > Export for Budget | 4811459 |
| EXP-07 | ok | Round trips, the strict reader, files without new keys reading back byte for byte (format tests), `fmt` idempotent on the 1,500-sheet sample | |
| EXP-08 | ok | n/a: skipped by the owner | |
| EXP-09 | fixed | One-line in layout and fixed columns, CLI and app. An omitted scene kept kind `scene` and its eighths, so the Day Totals formulas counted pages the day did not: now kind `omitted_scene` with no eighths in both column sets and the board sheet (`an_omitted_scene_is_listed_and_adds_nothing`; D91 and D103 say so) | 4bc7adf |
| EXP-10 | ok | Breakdown by day, CLI `--by-day` and File > Export; omitted scenes left out | |
| EXP-11 | ok | Working formulas with cached values (`totals_are_working_formulas`, `daily_totals_add_up_by_formula`) | |
| EXP-12 | ok | Title, subject, author and creator added to a PDF; empty ones left out; a PDF it can't update left alone (`pdf_info.rs`); macOS only | |
| SPL-01 | fixed | Breakdown beside the board follows the selection (harness), resizable, remembered. With the Find panel open the board was squeezed to 34 px: the pane's width now counts the Find panel and is worked out again when it opens or closes (harness: 274 px left at an 861 px window) | b9bd4db |
| SPL-02 | ok | Any board or sub-board beside the board (harness `board_view`); no drag between panes (as decided) | |
| SPL-03 | ok | The calendar beside the board (harness `calendar_view`); no drop onto its dates from the board (as decided) | |
| SPL-04 | ok | The selected scene's images beside the board (`sheet_images_are_paths_with_thumbnails`, harness) | |
| SPL-05 | ok | Find and replace on text fields (core `find.rs` tests, CLI `find` and `replace`, MCP `find` and `replace`) | |
| SPL-06 | ok | `--where` rows with AND and OR on the CLI, MCP and the panel; an empty find text lists the sheets | |
| SPL-07 | ok | Single View in the pane's head and Shift Cmd 0; the same View item again closes the pane (harness, every kind) | |
| SPL-08 | ok | Images beside the Breakdown tab, following the sheet (harness `get_sheet` per step) | |
| KEY-01 | ok | The shortcut table, the menu's accelerators and the reference keys agree (`menu.rs` tests) | |
| KEY-02 | ok | Go to Scene, Linked Elements, Add Red Flag, first and last sheet in the table (`the_reference_keys_are_here`) | |
| KEY-03 | ok | The strip menu's items and keys, Paste off with an empty clipboard, Escape closes it, Shift+F10 opens it (harness); the items and when each is off are now under `node --test` (D137) | 24e64c3 |
| KEY-04 | ok | Delete on a day break deletes at once, one call; Backspace on a scene asks first, Cancel changes nothing, undo brings the break back (harness); what Delete asks is now under `node --test` (D137) | 24e64c3 |
| KEY-05 | ok | Help > Keyboard Shortcuts lists 103 commands from the one table; the filter narrows to one, an unmatched filter shows none; Escape closes it (harness); the filter is now under `node --test` (D137) | 24e64c3 |
| SAV-01 | ok | Save, Save As, Untitled, close asks Save, Don't Save or Cancel (`discarding_and_leaving`, `title_and_questions_follow_the_voice`) | |
| SAV-02 | ok | Autosave after a quiet spell, never over a file changed elsewhere, the toggle kept in settings (`safety_app.rs`, `safety.rs` tests); File > Autosave is answered in Rust | |
| SAV-03 | ok | n/a | |
| SAV-04 | ok | n/a | |
| SAV-05 | ok | Recovery copies after a crash offered and opened, locks of live copies respected, a clean exit leaves no lock (tests); Recovered schedules dialog, Later (harness) | |
| BYD-01 | ok | A revision that drops a scene: matched, omitted, kept in place, counted nowhere (days, DOOD, budget, timeline, call sheet, element list); CLI `--dry-run` writes nothing; MCP `revise_script`; Plan > Import Script Revision | |
| BYD-02 | fixed | Call sheet JSON and CSV, CLI, MCP and Plan > Export Call Sheet Data; day 0, a day past the last and a sub-board with no days refused. A test now holds the JSON to the keys CALLSHEET.md lists, in alphabetical order | 4811459 |
| BYD-03 | ok | n/a: needs a service | |
| BYD-04 | fixed | Call times set and cleared, 25:00 refused, timelines without a call time count from the call, untimed scenes warned. The Day Timeline dialog showed "null" under every day without warnings: fixed, with a test over every page script. A deleted day break's call time stayed in the file: now tidied. Pushing past wrap to the next day is still to come | b9bd4db, 483ab30 |
| BYD-05 | fixed | Lock, relock refused, a blank name refused, drift and unlock of an unknown lock refused, drift after a reorder (harness). A lock of a deleted board stayed and failed in Drift: deleting a board now drops its locks (`deleting_what_the_plan_points_at_tidies_it`); the drift view's "null" fixed | 483ab30, b9bd4db |
| BYD-06 | fixed | Suggest and apply, a pinned strip keeps its place through apply (MCP test added for `pin_strips`), a missing strip refused. The count called "pinned" included omitted scenes and parts of a scene spanning days, so a schedule with nothing pinned said "1 pinned": the CLI, the dialog and MCP.md now say they stay put. Pins on deleted strips and merged sheets are tidied | 35ce5fa, 4bc7adf, 483ab30 |
| BYD-07 | ok | n/a | |
| BYD-08 | ok | Latitude past 90 refused, offsets from the longitude, "no located scene" without coordinates, published times (`solar.rs`). No daylight saving rules (as decided) | |
| BYD-09 | fixed | Turnarounds of 0 and 49 hours refused, an unknown category refused, the long day limit of 12 hours written as the default (left out). The Turnarounds and Rest dialog showed "null" whenever every work day was checked: fixed. A deleted category's turnaround stayed in the file: now tidied | b9bd4db, 483ab30 |
| BYD-10 | ok | A backup before every save that replaces a file, the last 5 distinct kept (`backups_keep_the_last_few_distinct_versions`) | |
| BYD-11 | ok | n/a | |
| BYD-12 | ok | DOOD without holds (`without_holds_counts_work_and_events_only`), CLI `--no-holds`, the DOOD tab and report option | |

Also in this lane: the CLI's counts read "1 shoot day", "1 day break", "1 strip", "1
scene" (`count`); `remove_day_breaks` and `pin_strips` were the two MCP tools no test
called; docs/APP.md's "Not yet" list was brought up to date (4d6ab52), with the two
windows, split screen and beyond-the-reference-app items added.

Gaps left, each in docs/ROADMAP.md, M1:
- Page logic with no automated test here: the split pane's toggles and width, Delete
  asking for scenes only, the strip menu's items, the shortcut sheet's filter. Fixed
  after the review: under `node --test` in CI (24e64c3, D137).
- Moving strips between windows (FIL-03, D86); pushing what runs past wrap to the next
  day (BYD-04); daylight saving rules for sunrise and sunset (BYD-08). Still open.
- The command line worked on the first board for most commands (MCP tools take a board).
  Fixed after the review: every one-board command takes `--board NAME` (25f3bd9, D132).

Counts: 43 ok (12 of them n/a rows; FIL-03 with a test added), 8 fixed, 0 gap rows; the gaps left inside rows are
in docs/ROADMAP.md, M1.
