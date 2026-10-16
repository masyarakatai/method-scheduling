# Behavior note 13: user pain points and competitor features

Source: public vendor support and pricing pages for MMS (ep.com support FAQ, legacy MMS 6
FAQ, product and shop pages), search-result snippets of six Reddit threads (the threads
themselves could not be opened: Reddit blocks fetchers and the browser pane), two Slashdot
reviews, Shadow Magic's video description (`YHTowtlR2_M`), comparison pages from Jungle
Software, Filmustage, StudioBinder and Storiara, the storyflow and Storiara 2026 roundups,
competitor help and pricing pages, and READMEs of three open-source projects (CineSched,
Call-Sheet-Tool, stripboard). Read October 2026. Raw pages in
`imports/research-cache/painpoints/`.
Clean-room: behavior in our own words, no copied text, the source stays outside the repo.
Forum opinions are summarized, never quoted. Open-source projects: READMEs only.

## MMS facts that shape the pain

- MMS 6 is end of life: updates stopped July 31, 2025, support ended Dec 31, 2025. One
  machine activation at a time.
- MMS 10 is subscription only (monthly or yearly, with academic and multi-seat plans), with
  required account sign-in and mandatory multi-factor authentication, cloud backup on save,
  and paid seats for every collaborator on a shared file. The vendor says offline work
  syncs later.

## Pain points (rough order of how often they came up)

1. Moving from a bought license to a subscription; unaffordable in weaker currencies.
2. MMS 10 felt worse than MMS 6 to some long-time users; one AD hit a deal-breaker and went
   back (details unreadable). Much of the 2026 release work is "MMS 6 parity", which agrees.
3. No call sheets. ADs move data into spreadsheet call sheet templates or other tools, and
   third parties sell exporters for this hand-off.
4. Shallow script import: headings and speaking characters only unless the script was
   pre-tagged in the screenwriting app.
5. Script revisions: no merge of a new draft into an existing breakdown (competitors
   market this; no direct user quote found).
6. Collaboration tied to paid seats and installs; no read-only viewer for crew.
7. Learning curve and dated feel (MMS 6).
8. Stability and support (crashes, a stale lock file blocking launch, slow support).
9. No running clock on strips (call time plus durations giving clock times per scene).
10. No automatic scheduling or optimization.
11. Weak DOOD and report export in MMS 6 (MMS 10 added on-screen DOOD and Excel).

Little or no evidence found for complaints about scene sorting, keyboard flow, speed or
Mac support.

## Competitor features worth knowing (concepts only)

- **StudioBinder** (web): stripboard with banners, day breaks, times; scenario versions;
  tagging in the script view with edits syncing to the breakdown; DOOD, one-liner, sides;
  call sheets sent by email and text with read tracking.
- **Celtx** (web): board fed from script and breakdown; end-of-day strips with eighths and
  time; insert a day break every N scenes or pages; DOOD with four codes.
- **Gorilla** (desktop): multi-key sort, horizontal or vertical strips, DOOD for any
  category, union performer report, actor database, Excel backup on every save, export to
  MMS, budget link with DOOD totals.
- **Yamdu** (web and mobile): breakdown, schedule, DOOD, call sheets; imports Movie Magic.
- **SetHero** (web): call sheets with confirmations; an MMS to Excel exporter.
- **Filmustage** (web): AI breakdown; AI schedule reordering around constraints; revision
  upload that keeps tags and compares drafts; in-app script edits recount eighths; exports
  to the vendor's tagged and budget formats.
- **Fuzzlecheck** (desktop): sunrise and sunset by location, cast conflict and rest period
  checks, automatic boards, time-limited licenses.
- **Dramatify, Storiara** (web): auto-fill days up to a max day length; block a drop that
  would push a day too long.
- **CineSched** (open source, macOS, GPL; README only): calendar plus board, running
  timeline from call to wrap, FDX, Fountain and Highland import, availability and blackout
  checks, a "lock schedule" snapshot that reports later drift per actor, DOOD with an option
  to leave out holds, call sheets, plain JSON file. Its README mentions a past bug where
  dual dialogue in an FDX dropped all later scenes on import: a test case for us.
- **Call-Sheet-Tool** (open source, local web app; README only): duration-based day timing,
  push the tail of a long day to the next, 12-hour and sunset warnings, snapshot before
  writing, version history, trash with restore.
- **stripboard** (open source research code; README only): shoot order as a minimum-travel
  path across locations, region split for big maps.

## What this means for Method Schedule

Our core answer is local first, no account, free: that addresses pains 1, 2 (partly), 6 and
8. Candidate opt-in extras, in rough value order:

1. Script revision merge: diff a new Fountain or FDX draft against the old one, keep tags
   and scene identity, show adds, omits and length changes. later (phase 3), high value.
2. Running clock on strips from a call time, with day-length warnings. later.
3. Call sheet data export (per day: scenes, cast with IDs, elements, times) as CSV and
   xlsx, not a designed call sheet. later.
4. Schedule lock and drift report (compare against a saved snapshot). Falls out of diff
   and merge. later.
5. Local order suggestions (group by set or location). later, opt-in.
6. A local backup copy in an open format on every save. v1 (rolling backups already planned).
7. Read-only exports for crew (PDF, plus our open file). v1.
