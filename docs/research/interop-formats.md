# Interop formats: Fountain, Final Draft, open schedule standard, DOOD and one-line exports

Status: research notes, October 2026. Nothing here is built yet.

Clean-room note: functional descriptions in our own words, from open specs, public sample
data files, READMEs and docs pages, and public articles. No open-source code was read (FDX
and Fountain parsers: READMEs and API docs only). No vendor native file was opened or
researched (`.mmsx`, `.msd`, `.sex`, `.mbl`, layout and template files are all off limits).
Sample files we looked at stay in the gitignored `imports/research-cache/interop/`; none are
copied into the repo. We do not reproduce any vendor's default category list here.

Contents:

1. Fountain
2. Final Draft `.fdx`
3. Pagination and eighths
4. Universal Schedule Standard (USS) and category ids (UCID)
5. DOOD exports across tools
6. One-line and board exports across tools
7. What MMS reads and writes (for comparison)
8. Recommendations
9. Open questions

---

## 1. Fountain

Spec: https://fountain.io/syntax/ (version 1.1, 2014). Plain text, open, implement from the
spec. Fallback for anything unmatched is action.

What matters for import (scene headings and speaking characters, kickoff section 5):

- **Strip the boneyard first.** `/* ... */` is the only construct that spans blank lines
  and is ignored on output; a whole scene can hide inside it.
- **Scene heading, detected**: a line starting with INT, EXT, EST, INT./EXT, INT/EXT or I/E
  (any case) then a dot or a space, with a blank line before and after.
- **Scene heading, forced**: one leading dot then a letter or digit. Two or more dots
  (an ellipsis) do not force.
- **Not a heading**: a leading `!` forces action.
- **Scene number**: optional, at the end of the heading between two `#`. Letters, digits,
  dashes and dots allowed (1, 1A, A1, I-1-A, 110A). Strip it from the set text.
- **Character cue**: a line all in capitals (at least one letter), blank line before, no
  blank line after. An extension in parentheses may follow on the same line (V.O., O.S.,
  free text, lowercase allowed). CONT'D is just an extension. Forced with a leading `@`,
  which keeps mixed case and non-Latin names. Dual dialogue: a `^` at the end of the second
  cue.
- **Speaking character** for import = a cue followed by dialogue. Normalize the name by
  dropping extensions and a trailing `^`.
- **Traps**: an all-caps action line followed by more text reads as a cue plus dialogue
  (writers fix it with `!`). Transitions are all caps ending in `TO:` with blank lines
  around, or forced with `>`; don't take them as cues. `>TEXT<` is centered action.
- **Other elements** (ignore for scheduling): title page (`Key: value` lines at the very
  top, values may continue on indented lines; keys we can keep: title, credit, author,
  source, draft date, contact), notes `[[ ]]`, sections `#`, `##` and deeper (don't print),
  synopses (`=` lines; could feed our synopsis field as an opt-in), page break `===`,
  lyrics `~`, emphasis markers.
- A non-standard extension in one parser uses trailing `#word` tags on lines. It clashes
  with scene numbers; don't support it.
- Splitting a heading into our fields: prefix to I/E (map INT./EXT and I/E to our
  "both" value), the rest up to the last ` - ` to set, after it to time of day. Keep the raw
  heading text too.

No breakdown tags and no page lengths in Fountain, so eighths need our own pagination
(section 3).

## 2. Final Draft `.fdx`

No public spec. Shape learned from public sample data: six samples with Fountain twins in
https://github.com/rsdoiel/fdx (testdata folder only; the repo's code is AGPL and was not
read; the test data's license is not stated, so it stays out of our repo), three samples
from fountain.io's downloads, and a category reference file in the UCID repo. API docs of an
MIT parser (lexington) show only the minimal tree.

- **Root**: an XML declaration (UTF-8, `standalone="no"`), then a `FinalDraft` element with
  a document type of script, a template flag and a version number (1, 3 and 4 seen; one
  article shows 5).
- **Content**: a flat list of paragraphs, each with a type and one or more text runs. Join
  the runs before testing anything.
- **Paragraph types seen**: scene heading, action, character, dialogue, parenthetical,
  transition, general, shot, plus custom names (a "singing" type in one sample). Accept any
  string. Shot is laid out like a heading but should not start a scene.
- **Scene number**: an optional `Number` attribute on scene heading paragraphs, a string
  (letter suffixes allowed by the numbering scheme). Several samples have none. The same
  attribute name means something else on tag categories.
- **Scene properties**: the first child of a heading paragraph carries the scene's length,
  its starting page and a title. Every heading in all nine samples had a length. Format:
  whole pages and eighths as text: `1/8` to `7/8`, `1`, `1 4/8`, `3 6/8`. Smallest seen
  1/8. This is the best source of eighths we have; use it as is.
- **Dual dialogue**: a general paragraph wraps a dual dialogue element holding both
  speakers' cue and dialogue paragraphs. A walker that only reads top-level paragraphs
  misses those cues. An open-source scheduler's README records a past bug where this
  dropped every later scene: make it a test.
- **Text run attributes** in production files: font, size, style (a plus-joined set like
  bold, italic, underline, all caps), colors in 48-bit hex, revision id. Keep whitespace
  inside runs (curly quotes and trailing spaces occur).
- **Blocks after the content** (ignore for import, but don't choke): header and footer,
  page layout (margins in points, page size in inches), element settings (fonts, indents,
  spacing, pagination behavior per type), title page (positioned paragraphs, not key and
  value), smart-type vocabulary lists (characters, extensions, scene intros, locations,
  times of day: handy hints for heading parsing), more and continued settings, locked
  pages, revisions (named color sets with a mark), cast list, scene numbering options,
  script notes, watermark settings.
- **Production tags**: the only public example is a category list (each category with an
  id, name, number, color and style) inside a tag data block. No public sample shows how a
  tagged word links to a category. Treat tag reading as unknown until the owner can share a
  tagged `.fdx` made from an invented script.
- **Hostile files**: no sample has a DTD or entity, but refuse any DOCTYPE, any entity and
  any external reference; cap file size, depth, attribute count and run count. The UCID
  reference file is itself not well-formed (stray closing tag), so expect broken files:
  strict mode by default, with a clear error naming the line.

What MMS takes from an `.fdx` (manual): headings and speaking characters only; tags come
only through the vendor's own tagged export. Our default matches that; reading FDX tags
would be an opt-in extra.

## 3. Pagination and eighths

- Industry: page lengths in whole pages plus eighths, never reduced (4/8, not 1/2).
- No published algorithm exists. Conventions found: 12-point Courier, 1.5 inch left
  margin, about 1 inch on the other sides; 55 lines a page (Fountain's publisher), 56 (a
  producer's rule of thumb, 8 eighths of 7 lines), up to 60 in one studio style guide, and
  52 to 57 observed. Fade In's docs say Final Draft's own pagination is idiosyncratic and
  varies with version and font metrics.
- Our rule: for `.fdx`, take each scene's length from the file. For Fountain, paginate
  ourselves with documented settings (lines per page default 56, one eighth = 7 lines,
  rounding up to the next eighth, minimum 1/8 per scene), then compare against MMS on the
  same script exported to `.fdx` from an editor (engine check E-I2). Settings saved in the
  file.
- Store lengths as integer eighths. Parse `N`, `N/8`, `N M/8`, and `M/8` with M over 8
  (normalize). Display `1 3/8`, `5/8`, `2`.

## 4. Universal Schedule Standard (USS) and category ids (UCID)

Repos: https://github.com/UniversalScheduleStandard/UniversalScheduleStandard and
.../UniversalCategoryIdentification. Read: READMEs, the docs page, two sample `.uss` files,
the category list file. Version 1.0.0 (2022), small fixes in June 2026, 19 stars. Backed by a
group of production software companies.

**License conflict, flag to the owner**: the USS README says CC BY-ND 4.0, its LICENSE file
holds CC BY 4.0 text, and an earlier badge said BY-NC-ND. The UCID repo is inconsistent the
same three ways. Reading and writing the format is ordinary interop; republishing a changed
copy of the spec or samples would need the license sorted. Don't vendor their files.

Shape (JSON, `.uss`):

- Root object with one key holding the schedule. Header keys always present (null when
  unknown): id, author, company, created, description, episode and episode name, name,
  project, schedule color and date, script color and date, season, source, format version.
- **Breakdowns** (one per scene or banner): id, banner text, comments, created,
  description, element ids, pages (a decimal multiple of one eighth: 0.125 for 1/8; fractions
  as text forbidden), scene number as a string, script page as a string, duration in
  milliseconds, and a type (scene or banner).
- I/E, time of day, set, unit, location and script day are not fields: they are elements
  in categories with fixed category ids.
- **Categories**: id, created, name, category id number. Ids 0 to 49 are reserved for
  header-style categories (set, I/E, time of day, script day, unit, sequence, location),
  100 to 499 for breakdown categories; custom ones go in 50 to 99 and from 500 up.
  Renaming is fine; reusing an id for a different intent is not.
- **Elements**: id, category id, created, days off (weekday numbers, 0 is Sunday), drop
  minimum, board id string, events, and flags for in-DOOD, drop allowed, hold allowed, id
  locked; linked element ids; name. Missing DOOD flags default to true on import.
- **Stripboards** (scenarios): id, name, calendar id, boards. **Boards** (sub-boards,
  including a Boneyard by name): id, name, an ordered list of breakdown ids where a nested
  list is one shoot day. A flat list means no day breaks. No day break objects, numbers or
  notes: day number and date come from position and the calendar. Across a stripboard's
  boards every breakdown must appear.
- **Calendars**: id, weekday days off, events (start date, single day off, other named
  events such as holiday, fitting, rehearsal, travel).
- No DOOD table: computed by the reader.
- Extensions: custom keys start with an underscore.
- **The samples disagree with the docs** in several places (different key names for the
  board list and for the drop count, "strip" instead of "scene", a different date offset
  style, holidays stored as plain events, a duplicated event id). A reader must accept both
  spellings; a writer should follow the docs.

Fit with our model: close. Gaps on their side: no day break or banner text per day, no
company travel (move) day, no per-sub-board start date beyond the calendar, no quantities,
no sunrise, sunset, screen time or script day as fields. Gaps on ours: none (we can map
everything they have).

## 5. DOOD exports across tools

See behavior note 12 for codes and counting. Export formats found:

| Tool | DOOD export | Notes |
|---|---|---|
| MMS 10 | DOOD report PDF; DOOD summary PDF and Excel | Per category. Layout not public. Owner exports from a ZTEST schedule will tell us columns. |
| Gorilla | Excel | Days worked feed its budgeting app. |
| Filmustage | PDF, xlsx | Fewer codes; first, last and span summary (sister repo). |
| StudioBinder | PDF; CSV advertised | Columns undocumented. |
| Celtx | download or print | Four codes only. |
| SetHero, Yamdu | read MMS PDF reports | Native files closed, so they parse PDFs. |

Typical content of a DOOD export, across tools: element id (board ID), name, category, one
code per day, start and finish dates, separate counts for work, hold, travel (and in some,
idle, holiday, rehearsal, fitting, drop days), total, sometimes rate and pay. Prep and wrap
never appear.

Our DOOD CSV (proposal, our own column order): `category, board_id, element, start, finish,
worked, held, travel, fitting, rehearsal, holiday, move, dropped_days, total_mms`, then one
column per date (ISO `YYYY-MM-DD` headers) holding our code. A second "long" CSV: one row per
element per date. Counts stay separate; `total_mms` is MMS's worked plus held as defined in
note 01, labeled as such.

## 6. One-line and board exports across tools

- Typical one-line columns (Scouty, StudioBinder, SetHero, Jungle): scene number, I/E, set,
  time of day, short description, pages in eighths, cast IDs, location, sometimes script
  day and estimated time; day rows with day number, date and page total. One encyclopedia
  stub claims one-liners omit cast and location; every working source includes them.
- MMS 10: one-line to PDF and Excel (with element quantities since July 2026); board to
  Excel or PDF; multi-unit merge by date.
- Our one-line CSV and xlsx: one row per strip, banner or day break, with a `row_type`
  column, day number and date on every scene row, pages as text (`1 3/8`) plus an integer
  eighths column for math.

## 7. What MMS reads and writes (for comparison only)

| Direction | Format | Open to us |
|---|---|---|
| Read | `.fdx` (headings, speaking characters) | Yes (open XML) |
| Read | vendor screenwriter files, tagged export `.sex` | No |
| Read | MMS 6 `.msd` | No |
| Write | PDF (all reports, board, calendar) | Output only |
| Write | Excel (one-line, element list, DOOD summary, element detail, breakdown, board) | Yes, as samples to compare against |
| Write | `.mbl` budget library, `.msd`, templates, layouts | No |

## 8. Recommendations

1. Fountain import first (open spec, easy fixtures), then `.fdx`. Headings and speaking
   characters by default; Fountain synopses and FDX tags as opt-in extras. v1.
2. FDX: lengths from the file; refuse DTDs and entities; handle dual dialogue; join runs;
   strict mode with good errors. v1.
3. Fountain eighths: documented pagination settings, checked against MMS (E-I2). v1.
4. USS: lenient reader and strict writer as an interop format, after the owner settles the
   license question. later. Its nested-array day model and category ids are a good
   reference for our own format design, but our file keeps explicit day break and banner
   objects.
5. DOOD and one-line as CSV and xlsx with every count separate and ISO dates. v1.
6. Budget exchange `.days.json` as in kickoff section 7. later, after the budgeting side
   agrees.

## 9. Open questions

1. Can the owner make a production-tagged `.fdx` from an invented script, so we can see
   the tag structure? (Only with their copy of Final Draft, invented text only.)
2. USS license: is reading and writing it fine with the owner, and do we want to credit it?
3. Lines per page default for Fountain: 55 or 56? Answered: 56, with a setting (decision 195).
4. Do we want an "idle" code and a configurable code alphabet as opt-ins?
5. Does the owner's line producer tool already read any DOOD CSV shape we should match?
