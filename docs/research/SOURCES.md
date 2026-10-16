# Research sources: scheduling knowledge and tutorials

Status: compiled and link-checked October 2026. Every URL below was opened (HTTP fetch,
scrape, or video metadata lookup) unless the access note says otherwise.

Clean-room line for this whole file: these are places to **learn from**. Restate what you
learn in our own words in a behavior note. Never copy text, screenshots, report layouts,
icons, sample data, bundled lists or code from any of them. Vendor material teaches
behavior only. Open-source code is for reading docs and concepts only; never paste or port
code, whatever the license.

How to read the entries:

- **Depth** 1 to 5: 1 = a definition or teaser, 3 = a solid walkthrough of one topic,
  5 = reference-grade, covers the area end to end.
- **Access**: how a scraper fares. "curl ok" means a plain HTTP fetch returns the page.
  "firecrawl only" means plain fetches get blocked but the firecrawl scraper works.
  "browser only" means both get blocked; use the Browser pane and read it by hand.
- **Version**: the MMS version shown, where it matters. Newer recordings outrank older
  ones; the real app outranks all of them.
- Video durations, upload dates and caption status come from public video metadata
  (no video was downloaded). "auto-en" means only machine captions exist; "manual en"
  means a human caption track exists.

---

## Top 20 to scrape first

| # | Source | Why first |
|---|---|---|
| 1 | MMS manual full-text index: https://mms-docs.ep.com/search/search_index.json | The whole current manual (about 98 pages) as one JSON file. One fetch gives every page's text for search and note taking. |
| 2 | MMS manual, Coming from MMS 6 section: https://mms-docs.ep.com/ComingFromMMS6/WhatChanged.html plus BringingFilesIn, LayoutsAndTemplates, ReportsAndPrinting | The vendor's own map from MMS 6 terms and features to MMS 10. Bridges the old tutorials to the current app. |
| 3 | MMS manual, DOOD pages: https://mms-docs.ep.com/DayOutofDays/DayOutofDays.html and https://mms-docs.ep.com/DayOutofDays/DOODRules.html | The counting rules (work, hold, drop, events) our engine has to match. |
| 4 | MMS manual, Board section: https://mms-docs.ep.com/Board/Board.html, DayBreaks.html, BannerStrips.html, ManagingStrips.html, StripDisplay.html | Boards, sub-boards, Boneyard, day breaks, auto day breaks, banners. |
| 5 | MMS release notes: https://mms-docs.ep.com/ReleaseNotes.html | Dated feature history of MMS 10 (latest seen: 10.15a, Sept 2026), including "MMS 6 parity" items that tell us which old behaviors came back. |
| 6 | Video: Legacy MMS Complete Video Training, `ohwgc8hwhbA` (58:07, 2018) | The deepest single walkthrough of MMS 6, 22 chapters from import to DOOD. |
| 7 | Video: MMS 10 Overview, `X0GX_EUNe88` (20:00, Apr 2026) | Current app, vendor-made, human English captions. Already saved locally per the kickoff file. |
| 8 | MMS 5 user manual, full text: https://archive.org/stream/manualzilla-id-5897802/5897802_djvu.txt | Complete older manual as plain OCR text. Best written source for MMS 5 and 6 era concepts (element manager, design layouts, shortcut keys). |
| 9 | EP Scheduling user manual PDF (about 2006, 139 pages): http://assistantdirectors.com/art/movies/2006/eps_manual.pdf | Even older edition. Shows the long-standing core: breakdown sheets, element linking, stripboard, calendar, DOOD. |
| 10 | Vendor short legacy videos (2009, MMS 4 and 5): Red Flags parts 1 and 2, Boneyard, Hold Days, Custom Daybreaks, Custom Text Banners (IDs in the video table) | Focused three to eight minute demos of exactly the features we must match. |
| 11 | Video: How to Create a Schedule using MMS, `P6k_D3JIj3o` (22:02, 2019) | Clean classroom-style end-to-end build of a schedule in MMS 6, chaptered. |
| 12 | Script Breakdown and Film Scheduling course PDF: https://www.actioncutprint.com/sbfs/SBFS-Course12.pdf | 160-page free course ebook by a veteran 1st AD. Breakdown practice and schedule design from the working side. |
| 13 | Jungle Software DOOD article: https://www.junglesoftware.com/great_articles/What-is-a-DOOD-report.php | Plain explanation of DOOD codes plus hold, drop and pick-up practice tied to actor union rules. |
| 14 | StudioBinder DOOD explainer: https://www.studiobinder.com/blog/day-out-of-days-reports-explained/ | Widely read DOOD primer with layout description and codes. |
| 15 | StudioBinder stripboard guide: https://www.studiobinder.com/blog/shooting-schedule-stripboard/ and video `w9KmYOUNwKA` (7:51, human captions) | Strip colors, day breaks, grouping logic, in concept form. |
| 16 | Fountain syntax: https://fountain.io/syntax/ | The open screenplay spec we will import. |
| 17 | FDX sample files with Fountain twins: https://github.com/rsdoiel/fdx (testdata folder, six samples) | Real `.fdx` files next to the same script in Fountain and text. Learn the XML shape from the data, not the code (AGPL). |
| 18 | Universal Schedule Standard: https://github.com/UniversalScheduleStandard/UniversalScheduleStandard | An open JSON format proposal for schedules and breakdowns (CC BY 4.0), with two sample files. Directly relevant to our own file and export design. |
| 19 | Script numbering and revision conventions: https://filmustage.com/blog/script-revision-colors-markup-history/ and https://johnaugust.com/2007/renumbering | A-scenes, OMITTED, locked pages, revision color order. Rules our scene numbering must respect. |
| 20 | FSU film school production handbook: https://fsufilmhandbook.com/the-schedule/ and https://fsufilmhandbook.com/1st-ad-responsibilities-m1/ | How a film school teaches the schedule, prep and the 1st AD's paperwork. |

---

## Counts per category

| Category | Count |
|---|---|
| A. Vendor docs, current MMS 10 | 11 |
| B. Vendor docs and manuals, legacy MMS 4 to 6 | 6 |
| C. Courses and training offerings | 7 |
| D. YouTube, vendor channel and playlists | 3 channel/playlist entries, 20 videos |
| E. YouTube, third-party MMS tutorials | 16 |
| F. YouTube, general scheduling concepts | 17 |
| G. Written guides and blogs | 30 |
| H. Film school and production handbooks | 6 |
| I. Unions, guilds and training programs | 5 |
| J. Forums, Reddit and Q&A | 11 |
| K. Books | 9 |
| L. Open formats, specs and conventions | 14 |
| M. Open-source projects | 15 |
| N. Competitor public docs | 9 |
| O. Sources checked and set aside | 8 |

---

## A. Vendor docs, current MMS 10

All on the vendor's public manual site (static MkDocs site, curl ok, no login). Version:
MMS 10, kept current (release notes run to Sept 2026). Clean-room: behavior only, never
copy help text, labels beyond a few-word term, screenshots or report layouts.

| Source | Type | Covers | Depth | Access |
|---|---|---|---|---|
| https://mms-docs.ep.com/search/search_index.json | Full-text JSON of the manual | Every page's text in one file | 5 | curl ok. Best first fetch. The sitemap.xml has no usable URLs, so use this or the nav links. |
| https://mms-docs.ep.com/contents.html (and any page's left nav) | Manual table of contents | Page list: Getting Started, Schedule Files, Shared File, Breakdown, Elements, Linking Elements, Red Flag, Board, Calendars, Day Out of Days, Reports, Split Screen, Images, Coming from MMS 6, Release Notes | 3 | curl ok |
| https://mms-docs.ep.com/Breakdown/Breakdown.html, SceneHeaderInformation.html, Categories.html, Elements.html, QuickAddElements.html, ImportingBreakdownSheets.html, AddBreakdownSheet.html, NavigatingSheets.html | Manual pages | Breakdown sheet header fields, eighths, categories, element entry, script import (.fdx), sheet order | 5 | curl ok |
| https://mms-docs.ep.com/Elements/Elements.html, ManagingCategories.html, ManagingElements.html; https://mms-docs.ep.com/LinkingElements/LinkingElements.html | Manual pages | Element manager equivalent, board IDs, linked elements | 4 | curl ok |
| https://mms-docs.ep.com/Board/Board.html, DayBreaks.html, BannerStrips.html, BoardFind.html, ManagingStrips.html, StripDisplay.html, PrintDownload.html, RightClickContextMenu.html | Manual pages | Boards and sub-boards, Boneyard, strips, auto day breaks, banners, strip layout, print and Excel export | 5 | curl ok |
| https://mms-docs.ep.com/Calendars/Calendar.html, BuildingaSchedule.html, Events.html, ConflictHandling.html, FilteringtheCalendar.html, NavigatingtheCalendar.html, ExportingPrinting.html; https://mms-docs.ep.com/RedFlag/RedFlag.html, ElementRedFlags.html, ManagingRedFlagTypes.html | Manual pages | Working week, off days, holidays, events, conflicts, red flags (availability limits) | 5 | curl ok |
| https://mms-docs.ep.com/DayOutofDays/DayOutofDays.html, DOODRules.html | Manual pages | DOOD codes and counting rules | 5 | curl ok |
| https://mms-docs.ep.com/Reports/Reports.html plus OnelineSchedule, OnelineVertical, ShootingSchedule, DOODReport, DOODSummary, ElementsReport, ElementDetail, BreakdownReport, DailyCategoryTotal, CalendarReport, ConflictReport, RedFlagReport (.html) | Manual pages | Report list, options, which export to Excel | 4 | curl ok. Do not copy report designs. |
| https://mms-docs.ep.com/ReleaseNotes.html | Release notes | Version history of MMS 10 with dates, MMS 6 parity items | 4 | curl ok |

Also on the same site: ScheduleFiles (files, templates, export, cloud backup), SharedFile
(multi-user), SplitScreen (four split views plus find and replace), Images.html.

Other vendor pages:

| Source | Type | Covers | Depth | Access |
|---|---|---|---|---|
| https://www.ep.com/movie-magic-scheduling/ | Product page | Marketing feature list, current positioning | 1 | curl ok |
| https://www.ep.com/support/movie-magic-scheduling/ | Support FAQ | Install, license, templates, chat support | 2 | curl ok |

## B. Vendor docs and manuals, legacy MMS 4 to 6

| Source | Type | Covers | Version | Depth | Access | Clean-room note |
|---|---|---|---|---|---|---|
| https://mms-docs.ep.com/ComingFromMMS6/WhatChanged.html, BringingFilesIn.html, LayoutsAndTemplates.html, ReportsAndPrinting.html | Vendor migration guide | Term map MMS 6 to 10 (Stripboard to Board, Element Manager to Elements and so on), red flag changes, no schedule password, running both versions, opening `.msd` | 6 vs 10 | 4 | curl ok | Behavior only. Opening `.msd` is done in MMS, never by us. |
| https://archive.org/stream/manualzilla-id-5897802/5897802_djvu.txt | Full user manual, OCR text | Menus, toolbars, shortcut keys, templates, breakdown, element manager, design layouts, stripboard, calendar, reports | MMS 5 (copyright to 2014) | 5 | curl ok | Archived copy of a vendor manual: read for concepts, never quote. OCR is noisy in tables. |
| http://assistantdirectors.com/art/movies/2006/eps_manual.pdf | Full user manual PDF, 139 pages | Getting started, setup preferences, element quick entry, categories, breakdown sheets (merge, duplicate), calendar, stripboard, banners, day breaks, layouts, reports, DOOD, import and export, advanced find | EP Scheduling, about 2006 (MMS 4 era) | 5 | firecrawl only (plain fetch returns HTTP 406 from a server filter) | Third-party hosted copy of a vendor manual. Read, restate, never quote. |
| https://www.ep.com/support/legacy-movie-magic-scheduling/ | Vendor FAQ | MMS 6 end of life: updates stopped 31 July 2025, support ended 31 Dec 2025; migration to MMS 10 | 6 | 2 | curl ok | Context only. |
| https://howtofilmschool.com/creating-a-schedule-using-movie-magic-scheduling/ | Third-party walkthrough | Highlighting elements, breakdown sheets, stripboard in older MMS | 5 or 6 (inferred) | 2 | curl ok | Fine to learn from. |
| Vendor legacy short videos | Video | See section D | 4 and 5 | 3 | video | See section D. |

## C. Courses and training offerings

Mostly gated (signup or payment). Listed so a human can take them; an agent usually cannot
scrape the lesson content.

| Source | Type | Covers | Depth | Access |
|---|---|---|---|---|
| https://academy.ep.com/new-movie-magic-scheduling-10 | Vendor course (Skilljar), about 1 hour | Getting started, breakdown sheets, stripboards, calendar, reports, downloadable materials | 3 | Landing page curl ok; lessons need an account |
| https://www.ep.com/ep-store/movie-magic-scheduling-course/ | Vendor course store page | Same academy course; page lists a price and a free coupon | 2 | curl ok |
| https://www.ep.com/academy/ | Vendor academy hub | Course catalog | 1 | curl ok |
| https://creativepathwayscanada.com/opportunities/skills-training/movie-magic-scheduling-basics/ | Third-party class listing (2 hours) | Intro MMS class outline | 1 | curl ok |
| https://www.uclaextension.edu/entertainment/film-tv/course/movie-magic-budgeting-film-tv-x-479088 | University extension course page | Budgeting and scheduling course outline | 1 | curl ok |
| https://moviesoftwaretraining.com/ | Private trainer site | MMS and Final Draft training for ADs and PMs | 1 | curl ok |
| https://www.studiobinder.com/tutorials/breakdown/intro-to-scheduling/ | Free competitor course | Lesson series on breakdown to stripboard to schedule | 3 | curl ok |

## D. YouTube, vendor channel and playlists

Channel: https://www.youtube.com/@BroadcastEP (channel ID `UCSHML_Xzj1a3OS2l6jBVbkg`).
Search inside the channel: https://www.youtube.com/@BroadcastEP/search?query=scheduling
Playlist "Movie Magic Scheduling" (MMS 10): https://www.youtube.com/playlist?list=PLb7fVmferYHi6rUiyarLyUFFG_ZgWfj14 (one video only).
A user-curated mixed playlist that collects the legacy MMS training plus other tools:
https://www.youtube.com/playlist?list=PLwS1GcslyqPU4zdGRF_ELwviovO1g407J (49 videos, many off topic).

Clean-room for all videos: watch, take bullet notes in our words, never save frames into
the repo. Version notes: "applies to version 4 and up" appears in many 2009 descriptions.

| Video ID | Title (short) | Length | Uploaded | Version | Captions | Depth | Note |
|---|---|---|---|---|---|---|---|
| `X0GX_EUNe88` | MMS Overview | 20:00 | 2026-04 | 10 | manual en + auto | 4 | Current app tour: stripboard, calendar, day breaks, import, banners, breakdown, sorting, DOOD, events, elements, reports. |
| `ohwgc8hwhbA` | Legacy MMS Complete Video Training | 58:07 | 2018-02 | 6 | auto-en, 22 chapters | 5 | Production info, import, breakdown, strip colors, elements, category, image and element managers, stripboard, sorting, day breaks, banners, calendar, Boneyard, printing, DOOD. |
| `4_NZX-JL8Ss` | Red Flags part 1 | 3:49 | 2009-05 | 5 | auto-en | 3 | Basics of element availability flags. |
| `kwc-cX9dyz4` | Red Flags part 2 | 7:43 | 2009-05 | 5 | auto-en | 3 | Deeper red flag behavior. |
| `KnX9eS7VHtg` | The Boneyard | 3:11 | 2009-05 | 5 | auto-en | 3 | Unscheduled strip holding area, new in MMS 5. |
| `WARV77dZDdw` | Hold Days | 3:48 | 2009-09 | 4+ | auto-en | 3 | Setting holds for a whole category. |
| `m5sOn_EYQU4` | Custom Daybreaks | 4:42 | 2009-08 | 4+ | auto-en | 3 | Which fields a day break strip shows. |
| `0WLy1paFMq4` | Custom Text Banners | 3:23 | 2009-06 | 4+ | auto-en | 2 | Per-banner text styling. |
| `EGGonnWvSrE` | Estimated Time Preferences | 2:34 | 2009-09 | 4+ | auto-en | 2 | Time shown as decimal or hours:minutes. |
| `dg8P_Utlb6w` | Element Quick Entry | 4:11 | 2009-07 | 4 and 5 | auto-en | 3 | Power-user element entry. |
| `K471ZX095oI` | Navigation Shortcuts | 4:59 | 2009-06 | 4+ | auto-en | 3 | Keyboard flow; useful for matching shortcuts. |
| `4VP5zDJnkR0` | Direct Insert | 3:04 | 2009-05 | 5 | auto-en | 2 | Inserting elements straight from sheets. |
| `qMv9X3Q66m4` | Direct Insert Enhancements | 3:09 | 2009-11 | 5 | auto-en | 2 | Revised direct insert flow. |
| `Fv0ZCIFGZ6g` | Color Settings | 3:12 | 2009-07 | 4+ | auto-en | 3 | I/E and day/night choices that drive strip colors. |
| `GjfO4NrWI2Y` | Storyboards | 3:13 | 2009-05 | 5 | auto-en | 2 | Images on breakdown sheets. |
| `6OI7KlYNkqI` | Copy/Paste Elements | 3:52 | 2009-10 | 4+ | auto-en | 2 | Moving elements between sheets. |
| `EyIkkHWCIbs` | Moving Elements | 3:47 | 2009-08 | 4+ | auto-en | 2 | Moving elements between categories. |
| `ugP-PR3WU2w` | Print Scaling | 3:14 | 2009-05 | 5 | auto-en | 2 | Fitting the board to pages. |
| `dpgDg0WJ2mY` | Import MMS Elements into MMB | 4:10 | 2009-05 | MMS 5 / MMB 7 | auto-en | 3 | Budget hand-off concept (categories and day counts). |
| `zEtUs927Qmg` | Moving a License | 6:00 | about 2011 | n/a | not checked | 1 | Licensing only; skip. |

## E. YouTube, third-party MMS tutorials

| Video ID | Title (short) | Channel | Length | Uploaded | Version | Captions | Depth | Note |
|---|---|---|---|---|---|---|---|---|
| `P6k_D3JIj3o` | How to Create a Schedule using MMS | Christian Hurtak | 22:02 | 2019-07 | 6 (inferred) | auto-en, 12 chapters | 4 | Template, import, scene info, estimated times, elements, Boneyard, building and finishing the schedule. Classroom pace. |
| `hN7tPE_z0HY` | How to Create a Schedule Using MMS | Christian Hurtak | 15:40 | 2019-04 | 6 (inferred) | auto-en, 9 chapters | 3 | Categories, company move, wrap time, banners. |
| `o04hd7VRBBA` | MMS tutorial | Kyle Bergersen (Univ. of Oklahoma media program) | 10:19 | 2020-02 | 6 (inferred) | auto-en | 3 | Script export, default template, import, adding elements. |
| `tZwEZ5Zrrsg` | Movie Magic: Scheduling | Jose Luis Madrigal Jr. | 1:10:01 | 2017-07 | 6 (inferred) | auto-en | 3 | Long lecture-style session on basics and breakdown; slow but thorough. |
| `wH5fDAUq1Oc` | How to use Movie Magic for film scheduling | Moonshot Initiative | 7:10 | 2021-01 | 6 (inferred) | auto-en | 2 | Working producer's quick tour; practitioner framing. |
| `7-e6PzF2D-I` | How to adjust layouts in MMS | Go For Production | 4:13 | 2018-08 | 6 | auto-en | 2 | Strip layout editing. |
| `dQROTXcNngA` | MM Scheduling: Stripboard | Edward Lavelle | 13:15 | 2019-02 | 6 (inferred) | none | 2 | Stripboard only; no captions. |
| `nkV25PERLC0` | Open a New Schedule or Demo File | Palm Beach State Film | 1:35 | 2019-01 | 6 | none | 1 | Part of a school series; very short. |
| `57DuJwICbwQ` | Nuevo Movie Magic Scheduling (2024) MMS X | Taller Movie Magic Espanol | 12:40 | 2024-10 | 6 vs 10 | auto-en (Spanish audio) | 3 | Side-by-side of MMS 6 and the new MMS 10 interface. |
| `OReDmABBr2Q` | First steps, loading scenes (Spanish) | Taller Movie Magic Espanol | 10:54 | 2015-11 | 5 or 6 | manual es | 2 | Spanish series; also strip colors, category manager, element manager, printing in the same playlist: https://www.youtube.com/playlist?list=PLi7NqWXBhHxPcC-ZyCHL4LNKCy2GbqQjZ |
| `GtQkGAuVN18` | Stripboard Manager (Spanish) | Taller Movie Magic Espanol | 8:27 | 2015-11 | 5 or 6 | manual es | 2 | As above. |
| `EO7Q3xHJ3d8` | Calendar Manager (Spanish) | Taller Movie Magic Espanol | 5:50 | 2015-11 | 5 or 6 | manual es | 2 | As above. |
| `yTM-Ud01wgY` | Intro to MMS, module 1: Breakdown Sheets (Portuguese) | ADA-RS/BRASIL | 22:50 | 2020-10 | 6 | auto-en | 3 | Detailed breakdown sheet walkthrough by an AD association. |
| `YHTowtlR2_M` | Intro to call sheet workflow for MMS 10 and later | Shadow Magic | 1:56 | 2026-02 | 10 | auto-en | 1 | Pain points moving MMS 10 data into a spreadsheet call sheet; series intro. |
| `wF4E2g7bcRM` | MMS: Exporting to Excel | SetHero | 3:25 | 2020-12 | 6 | auto-en, 7 chapters | 2 | How users get schedule data out of MMS 6 into spreadsheets. |
| `Eqzg5U3-QaQ` | Script breakdown, scheduling and budgeting, tutorial 4 of 4 | yschutzer | 10:40 | 2008-11 | 4 or 5 | auto-en | 2 | Old but shows the breakdown to budget chain. |

## F. YouTube, general scheduling concepts

| Video ID | Title (short) | Channel | Length | Uploaded | Captions | Depth | Note |
|---|---|---|---|---|---|---|---|
| `w9KmYOUNwKA` | How to Make a Shooting Schedule Using a Stripboard | StudioBinder | 7:51 | 2016-12 | manual en | 3 | Stripboard logic in a web tool. |
| `9IyI1YAVBW8` | How to Create a Shooting Schedule Using Stripboards | StudioBinder | 13:58 | 2017-01 | manual en | 3 | Longer version with grouping strategy. |
| `U6I1tMgjW-I` | The Pre-Production Process Explained | StudioBinder | 28:48 | 2023-12 | manual en | 3 | Where breakdown and schedule sit in prep. |
| `L8lF9TxxhlM` | What is a Day Out of Days Report? | StudioBinder Academy | 4:05 | 2023-09 | auto-en | 2 | DOOD basics. |
| `-HGkzRuJHL0` | How to Make a Production Schedule for Film | Luis M Pena | 20:56 | 2023-07 | auto-en, 8 chapters | 3 | Spreadsheet stripboard from scratch; good for first principles. |
| `IMep2s_T89c` | Script Breakdown 101 | RocketJump Film School | 13:40 | 2016-02 | manual en | 3 | Categories, color coding, eighths; very popular primer. |
| `dL7v7rc8AKs` | Script Lining | Cinemanium | 4:08 | 2017-02 | auto-en | 2 | Lining the script before breakdown. |
| `XnjjDQUyZ2o` | How To Do a Script Breakdown | Media Production Pro | 5:48 | 2021-06 | auto-en | 2 | Breakdown for budget and schedule. |
| `t2Li4VODxlU` | 1st AD Pre-production part 1: script breakdown | chinfat | 30:40 | 2024-01 | auto-en, 11 chapters | 3 | Full breakdown pass in Celtx from an AD view. |
| `edT67z1j_ZA` | 1st AD Pre-production part 2: scheduling | chinfat | 34:23 | 2024-01 | auto-en, 10 chapters | 3 | Building a schedule and reports in Celtx. |
| `py1drqU2zWA` | Lesson 44: Day out of Days | junglesoftwaretv | 9:59 | 2016-03 | auto-en | 3 | DOOD in Gorilla; codes and holds. |
| `F-12ujXLFbo` | What is a Day Out of Days Report? | Filmmaking Stuff | 4:01 | 2020-06 | auto-en | 2 | DOOD basics. |
| `-_RiVqrlpjA` | Reading the Stripboard Schedule | Filmmaking Stuff | 4:22 | 2020-07 | none | 2 | Reading a board on set. |
| `jBVCLIk3edg` | DOOD, one-liner, shooting schedule, script colors | arvold. | 3:45 | 2015-10 | auto-en | 2 | Quick definitions of the report family. |
| `_Wvczjxq5Qk` | How to Make a Shooting Schedule | The Nerdy Filmmaker | 7:48 | 2022-08 | auto-en, 10 chapters | 2 | Indie scheduling heuristics. |
| `9moWzpfAeZ0` | Tagging for Production | Final Draft | 6:12 | 2018-09 | manual en | 2 | Breakdown tagging inside the screenwriting app, which feeds scheduling import. |
| `UVjkRQHj8JU` | Overview of CineSched | Chris Tempel | 16:57 | 2026-03 | auto-en, 15 chapters | 2 | Open-source scheduling app tour (see section M). |

## G. Written guides and blogs

Most are vendor marketing blogs for competing tools: good for concepts and vocabulary, thin
on edge cases. Clean-room: fine to learn from, never copy text or images. All curl ok
unless noted.

| Source | Covers | Depth | Access |
|---|---|---|---|
| https://www.studiobinder.com/blog/day-out-of-days-reports-explained/ | DOOD codes, layout, purpose | 3 | curl ok |
| https://www.studiobinder.com/blog/shooting-schedule-stripboard/ | Stripboard, strip colors, day breaks | 3 | curl ok |
| https://www.studiobinder.com/blog/online-scene-breakdown-production-scheduling/ | Breakdown steps, eighths | 3 | curl ok |
| https://www.studiobinder.com/blog/what-is-a-shooting-schedule/ | Shooting schedule purpose and contents | 2 | curl ok |
| https://www.studiobinder.com/blog/what-is-a-call-sheet/ | Call sheet anatomy | 3 | curl ok |
| https://nofilmschool.com/shooting-schedule | Designing a shooting schedule, eighths | 2 | curl ok |
| https://www.wrapbook.com/blog/script-breakdown | Breakdown process, color guide | 3 | curl ok |
| https://www.wrapbook.com/blog/shooting-schedule | Shooting schedule steps | 2 | curl ok |
| https://www.wrapbook.com/blog/day-out-of-days-report | DOOD guide | 2 | curl ok |
| https://blog.celtx.com/script-breakdown/ | Breakdown how-to | 2 | curl ok |
| https://www.junglesoftware.com/great_articles/What-is-a-DOOD-report.php | DOOD codes, holds, drops, pick-ups, union rule context | 3 | curl ok |
| https://junglesoftware.com/day-out-of-days-report/ | DOOD in production | 2 | curl ok |
| https://junglesoftware.com/what-is-a-stripboard/ | Stripboard basics | 2 | curl ok |
| https://junglesoftware.com/how-to-read-a-shooting-schedule/ | Reading page counts, day breaks, workload | 3 | curl ok |
| https://junglesoftware.com/script-breakdown-film/ | Breakdown steps | 2 | curl ok |
| https://junglesoftware.com/final-draft-to-film-scheduling/ | FDX import to schedule workflow | 2 | curl ok |
| https://www.thegreenshot.io/uncategorized/day-out-of-days-dood-schedule-film-production/ | Building a DOOD, status codes | 3 | curl ok |
| https://sethero.com/blog/film-shooting-schedule-stripboard-template/ | Stripboard rows, day-break rows | 2 | curl ok |
| https://sethero.com/blog/article/day-out-of-days-film-set-terminology/ | DOOD term, who makes it | 1 | curl ok |
| https://sethero.com/blog/film-production-scheduling-7-stages-for-producers/ | Pages per day ranges, scheduling stages | 2 | curl ok |
| https://filmustage.com/blog/what-are-eighths-in-a-script-and-how-do-they-help-plan-production/ | Page eighths | 2 | curl ok |
| https://blog.staffmeup.com/understanding-day-out-of-days-reports-and-stripboards-in-film-production/ | DOOD and stripboards | 2 | curl ok |
| https://firstdraftfilmworks.com/blog/the-complete-guide-to-film-scheduling-master-movie-magic-scheduling/ | MMS-centric scheduling guide | 2 | curl ok; generic, possibly machine-written |
| https://www.dummies.com/article/academics-the-arts/performing-arts/film/the-art-of-scheduling-a-film-199344/ | Strip order, calendar thinking | 2 | curl ok |
| https://www.tumblr.com/goingforpicture/16526489708/8ths | Eighths explained by a working AD blog | 2 | curl ok |
| https://www.scouty.com/en-us/blog/how-to-create-a-one-liner-schedule | One-liner contents | 2 | curl ok |
| https://la411.com/blog/post/the-anatomy-of-a-call-sheet-how-to-make-a-call-sheet-for-film-and-tv | Call sheet sections, including day X of Y | 3 | curl ok |
| https://johnwellsproductions.com/how-to-read-a-call-sheet | Reading a real TV call sheet | 2 | curl ok |
| https://www.showmeshorts.co.nz/education/filmmaker-resources/live-or-die-by-your-shooting-schedule | Pages-per-day practice for shorts | 2 | curl ok |
| https://storiara.com/blog/how-many-script-pages-can-you-shoot-in-a-day | Pages-per-day reasoning | 2 | curl ok |

Filmmaker IQ was searched: its catalog has no scheduling course, so it is not listed.

## H. Film school and production handbooks

| Source | Type | Covers | Depth | Access | Clean-room note |
|---|---|---|---|---|---|
| https://www.actioncutprint.com/sbfs/SBFS-Course12.pdf | Free course ebook, 160 pages | Script breakdown, schedule design, minimizing moves and setups, daily prep schedule, AD duties | 4 | curl ok (PDF) | Author's copyrighted text; notes only. |
| https://fsufilmhandbook.com/the-schedule/ | Film school handbook page | How the school schedules its productions | 3 | curl ok | Fine to learn from. |
| https://fsufilmhandbook.com/1st-ad-responsibilities-m1/ and https://fsufilmhandbook.com/2nd-ad-responsibilities-mfa/ | Handbook pages | AD paperwork and duties | 3 | curl ok | As above. |
| https://fsufilmhandbook.com/call-sheet/ and https://fsufilmhandbook.com/daily-prep-schedule/ | Handbook pages | Call sheet, prep schedule | 2 | curl ok | As above. The site's WordPress search API lists more pages. |
| https://fm.hunter.cuny.edu/handbook/production2/pre/SHOOTING_SCHEDULE.pdf | One-page school form | Columns of a simple shooting schedule: date, scene, D/N, I/E, location, description, cast | 2 | curl ok | Use as a column-concept reference only. |
| https://moody.utexas.edu/sites/default/files/sites/communication.utexas.edu/files/attachments/utla/F16_-_HHW_Syllabus.pdf | University syllabus | Production course reading list naming the classic scheduling and AD books | 2 | curl ok | Source for the book list. |

## I. Unions, guilds and training programs

No public DGA trainee reading list was found; the programs publish overviews, not
syllabi. Union contract text is the authority for hold, drop and turnaround rules.

| Source | Covers | Depth | Access |
|---|---|---|---|
| https://www.dga.org/resources/training-program | DGA AD training program overview (paid on-set days plus seminars) | 1 | curl ok |
| https://www.dgatrainingprogram.org/program/ | NY program expectations (math, accounting, paperwork skills) | 2 | curl ok |
| https://www.trainingplan.org/ | LA AD training program site | 1 | curl ok |
| https://www.dga.org/contracts/agreements | DGA agreements index (AD work rules, paperwork duties) | 3 | curl ok; PDFs are long |
| https://www.sagaftra.org/contracts-industry-resources/contracts/2023-tvtheatrical-contracts | Actor contract hub; consecutive employment and drop and pick-up rules that DOOD holds model | 4 | browser only (HTTP 403 to fetchers) |

## J. Forums, Reddit and Q&A

Reddit blocks both curl and firecrawl (HTTP 403, firecrawl refuses the domain). Read these
in the Browser pane. Titles and gist below come from search-index snippets; open each before
relying on it. Clean-room: practitioner opinions are fine to learn from; do not quote
posts in public docs.

| Source | Gist | Depth | Access |
|---|---|---|---|
| https://www.reddit.com/r/Filmmakers/comments/1jumoz3/calling_ads_need_your_notes_on_the_new_movie/ | ADs' notes on MMS 10 versus MMS 6: what changed, workarounds (one-line report headers, report editing) | 4 | browser only |
| https://www.reddit.com/r/FilmTVBudgeting/comments/1ayyfgq/adding_time_to_mm_scheduling_strips/ | Running clock from estimated times on strips | 3 | browser only |
| https://www.reddit.com/r/Filmmakers/comments/faa16v/movie_magic_scheduling_6_pdf_print_help/ | MMS 6 breakdown sheet print and custom category | 2 | browser only |
| https://www.reddit.com/r/Filmmakers/comments/ac83dd/ads_of_redditwhat_programs_do_you_use_for/ | What ADs use for features and why | 3 | browser only |
| https://www.reddit.com/r/Filmmakers/comments/1m0aloq/movie_magic_scheduling_alternatives/ | Alternatives users want and why | 2 | browser only |
| https://www.reddit.com/r/filmmaking/comments/1isf817/could_we_make_an_affordable_or_open_source/ | Demand for an open-source scheduler with a calendar view | 2 | browser only |
| https://www.reddit.com/r/Filmmakers/comments/1rmy41c/i_made_a_scheduling_app_help_me_make_it_better/ | Indie dev's app and the MMS pain points that drove it | 2 | browser only |
| https://www.reddit.com/r/Filmmakers/comments/zfn7wd/how_to_estimate_how_many_pages_can_be_shot_in_a/ | Pages-per-day estimates from working crew | 2 | browser only |
| https://www.reddit.com/r/Filmmakers/comments/ri5ic4/scheduling_question/ | Scheduling a flash-forward scene; AD advice | 2 | browser only |
| https://www.reddit.com/r/Screenwriting/comments/sdm9d2/locked_scene_number_debate_new_scenes_before/ | A-numbering for scenes inserted before scene 1 | 2 | browser only |
| https://www.dvxuser.com/threads/free-and-open-source-scheduling-and-budgeting-software.5712817/ | Forum thread announcing open-source scheduling tools | 2 | curl ok |

Stack Exchange: searched; no film-production site with real scheduling depth exists
(Movies & TV Stack Exchange is about watching films). Skip.

## K. Books

For the owner to buy if wanted. Do not download copies; several pirated PDFs turn up on
document-sharing sites and must be ignored.

| Title | Author | Edition seen | Why |
|---|---|---|---|
| Film Scheduling: Or, How Long Will It Take to Shoot Your Movie? | Ralph S. Singleton | 2nd ed., Lone Eagle, 1991 (1st ed. 1984) | The classic step-by-step scheduling text: breakdown sheets, strips, board, DOOD. Verified on Google Books. |
| Film Scheduling / Film Budgeting Workbook | Ralph S. Singleton | Lone Eagle | Companion exercises. |
| Scheduling and Budgeting Your Film: A Panic-Free Guide | Paula Landry | Focal Press, 2012 (a 2nd edition is referenced) | Modern line-producing view of scheduling. |
| The Film Director's Team | Alain Silver and Elizabeth Ward | Silman-James Press | The AD department and its paperwork; long a trainee staple. |
| Film Production Management 101 | Deborah S. Patz | 2nd ed., Michael Wiese Productions, 2010 | Breakdown, schedule and paperwork flow for PMs and coordinators. |
| The Complete Film Production Handbook | Eve Light Honthaner | 4th ed., Focal Press | Forms and procedures, including scheduling paperwork. |
| Producer to Producer | Maureen A. Ryan | 3rd ed. | Low-budget producing, includes scheduling chapters. |
| Film Production Management | Bastian Cleve | Focal Press | Production management text often paired with Singleton on syllabi. |
| Script Breakdown and Film Scheduling (ebook course) | Peter D. Marshall | Online course, free PDF in section H | Practitioner course. |

## L. Open formats, specs and conventions

| Source | Type | Covers | Depth | Access | License and clean-room note |
|---|---|---|---|---|---|
| https://fountain.io/syntax/ | Spec | Scene headings, forced headings, scene numbers, characters, dialogue, title page, boneyard, notes, sections, page breaks | 5 | curl ok | Open spec. Implement from the spec. |
| https://fountain.io/developers/ and https://fountain.io/apps/ | Spec site pages | Reference implementations and apps list | 2 | curl ok | Reference code is MIT; we still write our own. |
| https://github.com/rsdoiel/fdx (testdata/) | FDX sample files | Six `.fdx` samples with matching `.fountain`, `.txt`, `.fadein` | 4 | curl ok | Code is AGPL-3.0: never read for porting. Sample files show the XML shape; keep them out of our repo unless their license allows. |
| https://pkg.go.dev/github.com/lapingvino/lexington/fdx | Parser API docs | FDX root, Content, Paragraph Type, Text structure | 3 | curl ok | MIT. Docs show element names only. |
| https://github.com/wildwinter/screenplay-tools | Parser README | Fountain and FDX parsing to one model (several languages) | 3 | curl ok | MIT. README only. |
| https://laper.ai/blog/fountain-format-vs-fdx-screenplay-files/ | Article | FDX skeleton, editor support matrix | 2 | curl ok | Fine to learn from. |
| https://blog.castandcrew.com/final-draft-interview | Interview | Why FDX is open, text-based XML | 1 | curl ok | Context. |
| https://kb.finaldraft.com/hc/en-us/articles/27810301418132-How-do-I-number-scenes | Vendor KB | Scene numbering, keeping existing numbers, A-scenes | 3 | firecrawl only (403 to curl) | Behavior only. |
| https://github.com/UniversalScheduleStandard/UniversalScheduleStandard | Open format proposal | JSON schedule and breakdown format, two `.uss` samples | 4 | curl ok | CC BY 4.0. Strong candidate to read or write as an interop format; credit if we adopt it. |
| https://github.com/UniversalScheduleStandard/UniversalCategoryIdentification | Open proposal | Standard IDs for breakdown categories | 3 | curl ok | Check license before reuse. |
| https://johnaugust.com/2007/renumbering | Article | Scene number formatting when scenes move | 2 | curl ok | Fine to learn from. |
| https://storiara.com/blog/how-to-number-a-film-script-for-production | Article | A and B scenes, numbering in both margins | 2 | curl ok | Fine to learn from. |
| https://filmustage.com/blog/script-revision-colors-markup-history/ | Article | Revision color order, A-pages, OMITTED, asterisks, breakdown color code | 3 | curl ok | Fine to learn from. |
| Report and layout examples: https://sethero.com/film-production-templates/ (one-liner, DOOD, call sheet templates), https://www.studiobinder.com/blog/downloads/film-production-shooting-schedule-template/, https://www.onassemble.com/template-library/shooting-schedule-template | Template galleries | Typical one-liner, shooting schedule, DOOD and call sheet columns | 2 | curl ok (downloads need signup) | Learn the common columns; never copy a template's design. |

## M. Open-source projects

Concepts only. Never copy code. Licenses checked through the GitHub API.

| Repo | License | Activity | Covers | Depth | Note |
|---|---|---|---|---|---|
| https://github.com/ChrisTempel/CineSched | GPL-3.0 (LICENSE file; API reports unknown) | active, Oct 2026 | macOS scheduling app: FDX import, stripboard, calendar, one-line PDF, call sheets | 3 | Closest open competitor. Read README and watch `UVjkRQHj8JU`. Copyleft: never borrow code. |
| https://github.com/sthielmann/Call-Sheet-Tool | MIT | Aug 2026 | Local-first call sheet, shooting schedule, stripboard | 2 | Small. |
| https://github.com/koosoli/OpenShotDesigner | unknown | Oct 2026 | Lined script, shot planning, call sheets for ADs and script supervisors | 2 | Check license before reading further. |
| https://github.com/ruisnow-e/stripboard | MIT | Sept 2026 | Shoot order optimization to cut company moves | 2 | Interesting for an optional "suggest order" extra. |
| https://github.com/hvaler/stripboard-dev | Apache-2.0 | Sept 2026 | Constraint-solver scheduling experiment | 1 | Concept only. |
| https://github.com/OfficialSquidProductions/production-desk-updates | MIT | Oct 2026 | Local breakdown and scheduler tool | 1 | Very new. |
| https://github.com/ggvfx/film-breakdown-assistant | MIT | Apr 2026 | Script parsing to scenes and elements | 1 | Concept only. |
| https://github.com/survos/screenplay-parser | none stated | 2017 | Auto breakdown from FDX, Celtx, Fountain | 1 | Old; no license, so do not read code. |
| https://github.com/nyousefi/Fountain | MIT | 2023 | Reference Fountain implementation | 3 | Spec authors' code. |
| https://github.com/mattdaly/Fountain.js | MIT | 2024 | JavaScript Fountain parser | 2 | Concept only. |
| https://github.com/piersdeseilligny/betterfountain | MIT | Sept 2026 | Fountain editor with stats (scene lengths, characters) | 2 | Has page and length estimation ideas. |
| https://github.com/ifrost/afterwriting-labs | none found via API | Jan 2026 | Fountain to PDF with statistics | 2 | Check license before reading code. |
| https://github.com/trelby/trelby | GPL-2.0 | Oct 2025 | Screenwriting app with FDX and Fountain import and export, reports | 2 | Copyleft. |
| https://github.com/LaPingvino/lexington | MIT | Oct 2026 | Fountain and FDX toolkit | 2 | See L. |
| https://github.com/wildwinter/screenplay-tools | MIT | Oct 2026 | Fountain and FDX parsing | 2 | See L. |

## N. Competitor public docs

| Source | Covers | Depth | Access |
|---|---|---|---|
| https://support.celtx.com/hc/en-us/articles/225674908-Day-out-of-Days-Report | DOOD grid (rows assets, columns days, SW/W/WF codes), sorting | 3 | firecrawl only (403 to curl) |
| https://support.celtx.com/hc/en-us/articles/360059004274-Professional-Stripboards-and-Scheduling | Stripboard, end-of-day strips | 3 | firecrawl only |
| https://support.celtx.com/hc/en-us/sections/360000544254-Schedule-Reports | Index of schedule and report articles | 2 | firecrawl only |
| https://junglesoftware.com/gorilla-scheduling/ | Gorilla feature list: breakdown sheets, one-liners, DOOD, shooting schedules | 2 | curl ok |
| https://yamdu.com/en/learn/preproduction/shooting-planning/day-out-of-days/ | DOOD for all resource types, not just cast | 2 | curl ok |
| https://yamdu.com/en/videos/proproducers/ | Video series on schedules, stripboards, DOOD | 2 | curl ok (video) |
| https://www.celtx.com/product/pre-production/schedule/ | Celtx schedule product page | 1 | curl ok |
| https://storyflow.so/blog/best-film-scheduling-software-2026 and https://storiara.com/blog/top-10-film-scheduling-software | 2026 tool roundups; quick map of the competitive field | 2 | curl ok; vendor-written |
| https://www.videomaker.com/article/r10/18653-jungle-software-gorilla-scheduling-and-budgeting-6-review/ | Independent review of a desktop competitor | 2 | curl ok |

## O. Sources checked and set aside

| Source | Why set aside |
|---|---|
| https://www.scribd.com/doc/48106422/Movie-Magic-Scheduling-5-User-Manual | Same MMS 5 manual as the archive.org text; Scribd walls content behind login. Use archive.org instead. |
| https://www.scribd.com/document/920871941/New-Movie-Magic-Scheduling-10-Coursebook | Appears to be paid vendor course material reposted. Do not use. |
| Scribd uploads of the Singleton and Landry books | Likely pirated. Do not use; buy the books. |
| https://support.amazonstudios.com/hc/en-us/articles/43421616282772-Movie-Magic-Scheduling-Useful-User-Guide-Links | Studio help page that only links to the vendor manual; blocked (403). Nothing new. |
| https://www.academia.edu/17644682/PRODUCTION_MANAGEMENT_AND_SCHEDULLING | Login wall (403). Low value. |
| https://www.filmmakingstuff.com/filmmaking-lesson-9-break-down-and-schedule/ | Blocked (403) to fetchers; the same author's videos are listed in F. |
| https://www.scribd.com/document/245599028/Prod-Plan-18544 | A film school syllabus (breakdown, board, DOOD, budget assignments) behind Scribd's wall; the UT syllabus in H covers the same ground. |
| Facebook and Instagram posts (vendor webinars, reels) | Login walls; no stable text. Search YouTube for the same webinar titles instead. |
