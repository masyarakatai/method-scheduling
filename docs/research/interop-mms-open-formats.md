# Every MMS import and export, and which ones are open

Status: research notes, October 2026, for MMS 10 up to 10.15a (10.15.12, Sept 24 2026).
Owner's goal: support every import and export the reference app offers that uses a
non-proprietary format. Clean-room: our own words from public pages only; no native
vendor file (`.mmsx`, `.msd`, `.sex`, `.mbl`, layout, report or template files) was
opened, downloaded or described, and none of their insides were researched. Nothing here
has been checked in the real app yet; section 5 lists what to open in screen block 3
(row A25).

Builds on `interop-formats.md` section 7 and `mms-feature-inventory.md` sections 1, 2,
11 and 12.

## 1. Sources

| Key | URL | What it gave |
|---|---|---|
| M-IDX | https://mms-docs.ep.com/search/search_index.json | The whole current manual as text (fetched Oct 12 2026) |
| M-IMP | https://mms-docs.ep.com/Breakdown/ImportingBreakdownSheets.html | Script import: `.fdx`, `.sex`, the vendor screenwriter's files |
| M-EXP | https://mms-docs.ep.com/ScheduleFiles/ExportingScheduleData.html | File menu exports: MMS 6 and the budget library |
| M-BRD | https://mms-docs.ep.com/Board/PrintDownload.html | Board: Excel or PDF, print all or selected |
| M-CAL | https://mms-docs.ep.com/Calendars/ExportingPrinting.html | Calendar: PDF or print |
| M-RPT | https://mms-docs.ep.com/Reports/Reports.html | Reports: PDF for all, Excel on some |
| M-RPTn | https://mms-docs.ep.com/Reports/OnelineSchedule.html, BreakdownReport.html, DOODSummary.html, ElementsReport.html, ElementDetail.html | Which reports offer Excel |
| M-RF | https://mms-docs.ep.com/RedFlag/ElementRedFlags.html | Red flag view: PDF or print |
| M-IMG | https://mms-docs.ep.com/Images.html and Elements/ManagingElements.html | Element and scene images added from the computer |
| M-RN | https://mms-docs.ep.com/ReleaseNotes.html | Version and date of each path |
| EP-FAQ | https://ep.com/support/movie-magic-scheduling | Support answers: script import, MMS 6 both ways, layout, report and template files, PDFs for unlicensed crew |
| MMS5 | https://archive.org/stream/manualzilla-id-5897802/5897802_djvu.txt | MMS 5 manual (2009): `.sex` and Final Draft 8 `.fdx` import, PDF of reports, exports to older vendor apps and the budgeting app |
| MMB | https://mmb-docs.ep.com/Setup/mms6_import.html | The budgeting app reads only the budget library file from scheduling |
| FIN | https://help.filmustage.com/en/articles/4802061-export-to-movie-magic-and-gorilla-software | A competitor writes the vendor's files to get into MMS (all closed formats) |
| FAD | https://fadeinpro.com/kb/content/1/85/en/export-to-movie-magic-scheduling-or-movie-magic-budgeting.html | Fade In: the scheduling export format is closed and undocumented, so it can't write it |
| SCW | https://techsupport.screenplay.com/hc/en-us/articles/115000961891-How-do-you-export-a-script-to-Movie-Magic-Scheduling | The vendor screenwriter writes the scheduling export |
| YAM | https://support.yamdu.com/en/articles/58206-import-movie-magic-schedule-to-yamdu | Yamdu imports an MMS schedule as a PDF printed with its own strip layout |
| SAT | https://saturation.io/integrations/movie-magic-scheduling | Saturation reads native files and DOOD PDFs |
| SH | https://help.sethero.com/en/articles/1513681-can-i-import-data-from-movie-magic-scheduling | SetHero takes an Excel file made by a third-party converter, not by MMS |
| SB | https://www.studiobinder.com/movie-magic-scheduling-into-call-sheets/ | StudioBinder: retype the schedule; no file path |

Searched with no result: Scenechronize, Cast & Crew, Showbiz Budgeting, Gorilla's own
pages, Celtx, Highland and WriterDuet reading or writing anything MMS specific. Nothing
public says MMS 10 reads or writes CSV, tab-delimited text, HTML, XML, iCalendar (`.ics`),
plain-text or PDF scripts, Fountain, or any element or cast list import.

## 2. Every path

Open: yes means a documented open format or a common standard. Ours is what Method
Schedule has today (docs/APP.md, docs/MCP.md, docs/FORMAT.md, docs/EXCHANGE.md,
docs/CALLSHEET.md and `crates/`).

| # | Dir | What | Format | Open | Since | Source | Ours |
|---|---|---|---|---|---|---|---|
| 1 | In | Script, headings and speaking characters | Final Draft `.fdx` (XML) | Yes | 10.4.22 (Nov 2023); MMS 5 read FD 8 files | M-IMP, M-RN, MMS5 | Have (IMP-01, IMP-03) |
| 2 | In | Tagged script with elements and categories | vendor scheduling export `.sex` | No | 10.4.22; MMS 5 | M-IMP, FAD | Skip (IMP-02); we read tags from `.fdx` instead (IMP-04) |
| 3 | In | Script from the vendor's screenwriter | vendor screenwriter file | No | manual, version not given | M-IMP | Skip (IMP-02) |
| 4 | In | MMS 6 schedule | `.msd` | No | 1.4.17 (Oct 2023) | M-RN, EP-FAQ | Skip (FIL-02) |
| 5 | In | Board, report and header layouts | vendor layout and report files | No | 10.14.14 and earlier | EP-FAQ, M-RN | Skip; ours are open JSON (FIL-05) |
| 6 | In | Templates | vendor template file | No | 10.14.14 | EP-FAQ, M-RN | Skip; ours is `.mtst` JSON (FIL-04) |
| 7 | In | Element and scene images | image files from the computer (types not listed) | Yes (common image types) | 10.15.8 (Aug 2026) | M-IMG, M-RN | Have: PNG, JPEG, GIF, WebP (ELM-14). Whether MMS takes more (TIFF, HEIC, BMP) is unknown: gap IMP-09 to confirm |
| 8 | Out | Every report, print and save | PDF | Yes | 10.4 era on; every report page | M-RPT | Have (EXP-01) |
| 9 | Out | One-line schedule | Excel `.xlsx` | Yes (OOXML) | 10.7.53 (Sept 2024) | M-RN, M-RPTn | Have (EXP-02), but see row 10 |
| 10 | Out | One-line Excel with element quantities, matching the one-line layout | Excel | Yes | 10.14.14 (Jul 2026) | M-RN | Missing: our one-line sheet has cast IDs only, no other categories or quantities. Gap EXP-09 |
| 11 | Out | DOOD summary | Excel | Yes | by 10.11 (version not given) | M-RPTn | Have (EXP-02); we also export the full grid, which MMS does only as PDF |
| 12 | Out | Element list (with Set and Location values since 10.14.14) | Excel | Yes | by 10.11; values 10.14.14 | M-RPTn, M-RN | Have (EXP-02) |
| 13 | Out | Element detail | Excel | Yes | by 10.11 | M-RPTn | Have (EXP-03) |
| 14 | Out | Breakdown report, by strip | Excel | Yes | 10.11 (Dec 2025) | M-RPTn, M-RN | Have (EXP-03) |
| 15 | Out | Breakdown report grouped by shoot day | Excel | Yes | 10.11 or later | M-RPTn, EP-FAQ | Missing: our by-day report (RPT-06) prints and saves as PDF, but the xlsx and CSV are by strip only. Gap EXP-10 |
| 16 | Out | Breakdown data "with working formulas" | Excel | Yes | 10.11 | M-RN | Missing: our workbooks hold values only, never formulas. Gap EXP-11 |
| 17 | Out | PDFs "with embedded metadata for searchability" | PDF | Yes | 10.11 | M-RN | Partial at most: our PDFs carry real text; document properties (title, subject) not set on purpose yet. Gap EXP-12 |
| 18 | Out | Board | Excel and PDF; print all or selected | Yes | PDF 10.11; Excel undated | M-BRD, M-RN | Have (EXP-03, BRD-20) |
| 19 | Out | Calendar view | PDF and print, weekdays only or full week | Yes | 10.11 | M-CAL, M-RN | Have (CAL-09) |
| 20 | Out | Calendar report | PDF and print | Yes | 10.9.14 (May 2025) | M-RN | Have (RPT-07) |
| 21 | Out | Red flag view, filtered by element | PDF and print | Yes | 10.12 (Feb 2026) | M-RF, M-RN | Have (RFL-03) |
| 22 | Out | Multi-unit one-line merged by date, element filter | PDF | Yes | 10.13 (May 2026) | M-RPTn, M-RN | Have (RPT-12) |
| 23 | Out | Vertical one-line with cast ID grid | PDF | Yes | 10.15a (Sept 2026) | M-RN | Have (RPT-10) |
| 24 | Out | Shooting schedule, DOOD report, daily category total, conflict, red flag report | PDF only (Excel hidden where not offered, 10.12) | Yes | various | M-RPT, M-RN | Have as PDF; ours add xlsx and CSV for several |
| 25 | Out | MMS 6 schedule | `.msd` | No | 10.6 (Apr 2024) | M-EXP, M-RN | Skip (EXP-04) |
| 26 | Out | Budget library for the vendor's budgeting app | `.mbl` | No | 10.4.22 | M-EXP, MMB | Skip (EXP-05); ours is the open `.days.json` (EXP-06) |
| 27 | Out | Templates to send by email | vendor template file | No | 10.14.14 | M-RN | Skip; ours is `.mtst` |
| 28 | Both | Shared cloud schedule, change history | vendor cloud | No | 10.12 | M-RN | Skip; ours: diff and merge of open files |
| 29 | Both | Copy and paste of strips, banners, elements | in-app clipboard (cleared on board or file change) | Unknown | 10.8.33, 10.13 | M-RN | Ours is in-app too. Whether MMS puts text on the system clipboard is unknown: question Q6 |

Other products, for the record:

- **Budgeting**: only the closed `.mbl` goes from scheduling to the vendor's budgeting
  app (MMB). The 2009 manual shows the same. Saturation reads native files and DOOD PDFs
  (SAT). No public Showbiz Budgeting or Cast & Crew exchange with MMS was found.
- **Call sheet and production tools**: Yamdu takes a PDF printed from MMS with its own
  strip layout (YAM); StudioBinder says to retype (SB); SetHero needs a third-party
  converter's Excel (SH). So the practical open hand-off from MMS today is its PDFs and
  Excel files. That argues for our PDFs keeping real, selectable text in a steady order,
  and for our xlsx column names staying stable.
- **Writing apps into MMS**: Final Draft `.fdx` is the only open door. Fade In says the
  scheduling export format is closed (FAD); competitors that reach MMS write the closed
  formats (FIN). We already read `.fdx` with tags and Fountain, which MMS doesn't.
- **Older MMS**: MMS 5 read `.sex` and Final Draft 8 `.fdx`, printed reports to PDF and
  exported only to older vendor schedulers and the budgeting app (MMS5). No open data
  export appears there either.

## 3. Gap list: open formats MMS supports that we don't

| Id | Gap | Why | Priority | Size |
|---|---|---|---|---|
| EXP-09 | One-line xlsx and CSV with element columns and quantities as the one-line layout shows them (one column per shown category, `ID NAME xN`) | One-line Excel is the most handed-around file; MMS added quantities in 10.14.14 and our EXP-02 claims parity on it | v1 | S: the one-line report already knows its layout's categories; add columns in `export/tables.rs::one_line` |
| EXP-10 | Breakdown by shoot day as xlsx and CSV (one row per day per element, with quantity and how many scenes list it) | MMS exports the by-day breakdown to Excel; it is the call sheet base, and our engine already builds it (RPT-06, `report_data`) | later | S |
| EXP-11 | Workbook totals as real formulas (page and day totals on the one-line, totals column on the DOOD summary and grid) next to the stored values | MMS 10.11 says its Excel carries working formulas; people edit these sheets after export. Keep our rule that text never becomes a formula; only cells we generate do | later | M: formula cells in our own xlsx writer plus cached values so viewers without recalculation still show numbers |
| EXP-12 | PDF document properties: title, subject (schedule, report, board, sub-board, date range), creator, so PDFs index and search well | MMS 10.11 claims searchable metadata; downstream tools read MMS PDFs (YAM, SAT) | later | S on macOS (the PDF writer takes properties); check Windows and Linux print paths |
| IMP-09 | Image types beyond PNG, JPEG, GIF, WebP for element and scene images, only if MMS takes them (TIFF, HEIC, BMP are the likely ones) | MMS 10.15.8 takes "images from your computer"; the types are not documented | later | S per type if the web view shows it; HEIC may need conversion (M) |

Not gaps (MMS has no open path we lack): script import (MMS reads only `.fdx` openly; we
also read Fountain and `.fdx` tags), any data import (MMS has none documented for CSV,
Excel, elements or cast lists), CSV (MMS writes none; we do), calendar files (`.ics`: MMS
has none), HTML, XML or plain-text reports (none in MMS 10). Each of those would be a
beyond-MMS extra, not a parity row; the owner can pick them from
`mms-feature-inventory.md` "Competitor features MMS lacks".

## 4. What to keep in mind when building

- Any added xlsx columns go at the end of a sheet or get new names, so scripts that read
  our earlier exports keep working.
- Formulas (EXP-11): write the formula and its cached value; never let schedule text
  reach a formula cell; CSV stays values only.
- PDF properties (EXP-12): no personal data beyond the production info the user typed.

## 5. Questions to confirm in the real app (screen block 3, A25)

Open each on a ZTEST schedule only, read the menus and dialogs, write the answers in our
own words in behavior note 19.

1. **File > Import** (or Import Script): every file type the open dialog allows; does it
   accept Fountain, PDF, plain text, Celtx, Fade In, Highland or WriterDuet files; is
   there any element, cast or CSV import anywhere in File or the Elements tab.
2. **File > Export**: the full list (MMS 6, budget library, anything else such as Excel,
   CSV, calendar or XML).
3. **Reports tab**, for each report in the list (one-line, vertical one-line, shooting
   schedule, DOOD report, DOOD summary, element list, element detail, breakdown by strip
   and by day, daily category total, calendar, conflict, red flag): which export choices
   show (PDF, Excel, others). For the one-line Excel: which columns, and how quantities
   and categories appear. For the by-day breakdown Excel: the row shape. Open one saved
   Excel file in a spreadsheet app and note whether totals are formulas or values.
4. **Saved PDF**: in Preview, Tools > Show Inspector: title, subject, author, keywords;
   is the text selectable.
5. **Board tab** export icon and print icon; **Calendar tab** export and print icons;
   **Red flag** view export: the choices each offers (any image, Excel or `.ics`).
6. **Copy strips** on the board, then paste into TextEdit: does any text arrive.
7. **Element > Images > Add**: which file types the dialog allows (try a TIFF and a HEIC
   from `~/ZTEST` if present; don't add anything else).
8. **Email or share** commands anywhere in File or Reports (beyond the shared cloud file).
