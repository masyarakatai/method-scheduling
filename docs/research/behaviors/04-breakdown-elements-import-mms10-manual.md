# Behavior note 04: breakdown sheets, categories, elements and script import (MMS 10 manual)

Source: public MMS 10 manual, Breakdown section (breakdown, add sheet, importing, scene
header, categories, elements, quick add, navigating), Elements section, Linking Elements,
Images, and release notes 10.6 to 10.15a. Read October 2026.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.
We do not copy the vendor's default category list; ours will be our own.

## Breakdown sheet header

All fields optional. "Element" type means a pick list shared across sheets (type a new
value and it becomes available everywhere); exactly one value per sheet.

| Field | Kind | Notes (our words) |
|---|---|---|
| Scenes | text | One scene, a list ("1, 3, 5"), part scenes ("12A"), or non-script work (EPK, stills). |
| I/E | pick list | Interior, exterior, both, plus user values. Drives strip color. |
| Set | pick list | Usually from the slugline. |
| Time of day | pick list | Day, night, morning, evening, dawn, dusk, plus user values. Drives strip color. |
| Sunrise, sunset | time | 24-hour HH:MM; sunrise hours 00 to 11, sunset 12 to 23. Since 10.13. |
| Pages | eighths | Whole pages plus eighths. Typing more than 8 eighths rolls the extra into pages. |
| Shoot time | number | Meant as hours:minutes but not enforced. MMS 6 "estimated time" lands here. |
| Synopsis | text | Short (tutorials say five or six words to fit a strip). |
| Location | pick list | The real place; independent of set. |
| Script pages | text | Page numbers, lists or ranges. |
| Script day | pick list | Story day ("D1", "N3", "1944"). |
| Screen time | number | Meant as minutes:seconds, not enforced. |
| Unit | pick list | Which unit shoots it. |
| Sequence | pick list | A story beat that groups scenes. |
| Episode | pick list | For series. |
| Comments | text | Notes. |

- Header field order can be rearranged per schedule.
- Next to the slugline the sheet shows its planned date in the active sub-board; double
  clicking it jumps to the strip.
- MMS 6 time display could be colon or decimal hours (note 09). Not in the MMS 10 manual.

## Categories

- Categories group elements (cast, background, props and so on). A new schedule ships with
  a built-in list. Built-in ones cannot be reordered; user ones can.
- Category names are unique. Renaming is allowed; clearing a name asks first.
- Category settings: auto-assign board IDs to new elements (next free whole number),
  exclude from board, default hold, default drop and minimum drop gap for new elements,
  "use in daily quantity", and custom element property fields.
- Changing defaults does not change existing elements (overview video). MMS 5 had a
  per-setting "apply to existing" box. Engine check E-L1.
- "Hide empty" hides categories with no elements on the current sheet.

## Elements

- An element belongs to one category. Name unique within the category (beta manual).
- Board ID: alphanumeric allowed ("1A", "3(m)"), unique in category (beta manual).
- Renumber: give unlocked elements consecutive whole numbers starting at a chosen integer,
  in the category's custom order, skipping numbers held by locked elements. Alphanumeric
  IDs are left alone.
- Lock or unlock IDs (single, bulk, all).
- Elements table columns: ID, name, occurrences (sheets using it), pages, work days, start
  and finish dates, events, conflicts. Built-in header categories (set, location) get
  page and day counts since 10.13a.
- Sort by any column, or a custom drag order.
- Merge two or more elements into one, picking the survivor; all sheets point to it.
- Move to another category: custom property values are lost (properties are per
  category). The overview video said all info carries over; newer manual wins.
- Edit window tabs: properties, DOOD switches, linked elements, notes, scenes (each sheet
  with its date in the active sub-board), red flags, days and dates (shoot day numbers and
  dates in the active sub-board), images.
- Quantity per element per sheet (for example 35 background). A per-sheet "use in daily
  quantity" box (on by default) controls whether that quantity adds to the day total, so a
  group reused in four scenes on one day isn't counted four times. A per-category switch
  hides quantities entirely; cast is off by default. Partial totals get an asterisk.
- Copy, cut, paste elements between sheets.

## Quick add

- A single field under the header. Type part of a name, pick from matches with arrow keys
  and Enter. If nothing matches, an add option appears; then pick or create the category
  and press Enter. Focus stays in the field for the next one.
- MMS 6 trick: ending a typed name with a semicolon created it without the confirm prompt
  (note 08).

## Linked elements

- Link element A to element B: adding A to a sheet also adds B.
- For series, choose all episodes or specific ones.
- A "retroactive" box applies the link to sheets that already hold A.
- Deleting a link stops future linking only; nothing already added is removed.
- Since 10.8.33 set and location elements can link to elements of any category (a set
  always brings its dressing).
- MMS 5 had preferences for inserting linked elements automatically or with a prompt, and
  removing them with confirmation (note 10). Not in the MMS 10 manual. E-L2.

## Sheets

- New sheet goes after the current one in the active sub-board; elsewhere to the end of the
  Boneyard. The New Sheet menu also duplicates, merges and deletes.
- Duplicate copies header and elements (used to split one scene into two sheets).
- Navigation steps through sheets in the order of the active sub-board. MMS 5 also offered
  stepping by sheet number or by scene number (note 09).
- Delete is permanent apart from undo. MMS 5 had automatic or manual sheet numbering.

## Script import

- Reads Final Draft `.fdx`, the vendor screenwriter's files, and the vendor tagged-export
  file (`.sex`, proprietary, off limits to us).
- From a plain script: scene headings (sluglines) and speaking characters only.
- From a tagged export: also tagged elements, matched by name to existing elements;
  unknown names are created; unknown categories are created.
- Every new sheet goes to the end of the Boneyard in every board. A count of imported
  sheets is shown.
- Not documented: whether pages (eighths) come from the `.fdx` scene properties, how the
  slugline splits into I/E, set and time of day, how scene numbers are taken, and what a
  second import of a revised script does (duplicates, we suspect). E-I1 to E-I4.

## Images

- Images on elements (several, each with a note, one marked for reports) and on scenes
  through split panes. They travel inside the schedule file. 10.15.8.

## What this means for Method Schedule

1. Every header field above, eighths stored as an integer count of eighths, times as
   integer minutes or seconds, with the "not enforced" free text kept only as an import
   fallback. [missing] v1.
2. Categories with defaults, elements with board IDs, lock, renumber, merge, move.
   [missing] v1.
3. Quick add by keyboard. [missing] v1.
4. Linked elements with the retroactive option. [missing] later.
5. Daily quantity rules. [missing] later.
6. Images. skip for v1 (file size and scope), later as a sidecar folder.
