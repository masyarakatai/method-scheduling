# Method Schedule brand

How Method Schedule looks and sounds, in the app, on methodschedule.com, in the README and
anywhere else it shows up. It is one of a family with Method Budget (methodbudget.com):
the same voice, the same neutrals and type, the same way of drawing the mark and the
name, and an accent of its own. The tokens live in
[site/assets/tokens.css](../site/assets/tokens.css); the artwork is drawn by
[scripts/make-brand.py](../scripts/make-brand.py).

## The name

- Always **Method Schedule**, two words, both capitalized, in running text.
- Never shorten it to initials: they are ambiguous, and a name exists so the product is
  not mistaken for anything else. On second mention "the app" or "it" is fine.
- The command is `method-schedule`, in code style. Schedule files are `.mtsc` files
  (templates `.mtst`).
- The wordmark is set in lowercase (`method schedule`). That is artwork, not spelling: in
  text, write Method Schedule.
- Credit "the Method Schedule contributors". The copyright line is "Method Production Suite Team and the
  Method Schedule contributors", the same pattern the ArtCraft apps use. No personal,
  company or studio names in public material: not in pages, alt text, metadata or file names.

## Voice

Plain, confident, specific. Write for someone who schedules films, not for someone who
reads the code.

- Say what a person can do, in their words: "Ask why an element's counts are what they
  are", not "count provenance engine".
- Claim only what is built. Planned work is labelled as planned ("coming", "on the
  roadmap"). The feature list in `docs/features.toml` is checked against
  `docs/parity/features.csv` for exactly this reason.
- No hype: no "revolutionary", "powerful", "seamless", "blazing". A number beats an
  adjective.
- No em dashes anywhere. Use a colon, a comma, parentheses or a new sentence.
- No exclamation marks.
- Other products: one factual line saying Method Schedule is not affiliated, in the FAQ
  (and the README's note). Never a comparison, a dig, a screenshot or a logo. A file
  format another product defines is named only to say we read it, with a trademark line
  in the fine print.
- Examples use invented productions and invented people only.

## The mark

A stripboard. A column of strips, each with its first cell (the scene number) set apart,
and one solid bar across the column: the day break that closes a shooting day. The
strips are the color at half strength, the day break at full strength, and the gaps
between them are empty, so the mark sits on any background. It is the same box as Method
Budget's page (40 by 52 units), and half strength means the same in both: the working,
under the thing that counts.

It is drawn on a 64-unit grid: five rows 8 units tall with 3-unit gaps, the fourth the
day break. A separate 16-unit drawing (five rows 2 units tall, whole units only) is used
for favicons so it stays sharp at 16 and 32 pixels.

| File | Use |
|---|---|
| `site/assets/brand/mark.svg` | The mark, slate blue, for light backgrounds |
| `site/assets/brand/mark-dark.svg` | The mark, light slate blue, for dark backgrounds |
| `site/assets/brand/wordmark.svg` | Mark and name, for light backgrounds |
| `site/assets/brand/wordmark-dark.svg` | Mark and name, for dark backgrounds |
| `site/favicon.svg` | The 16-unit mark; switches color with the browser's scheme |
| `site/favicon.ico` | The 16-unit mark at 16 and 32 pixels |
| `site/apple-touch-icon.png` | 180 pixels, white mark on a full-bleed slate blue square |

Rules:

- Leave clear space around the mark of at least a quarter of its height.
- Don't recolor it outside the palette, outline it, add effects, rotate it, or add or
  remove strips.
- Smallest sizes: the mark at 16 pixels (use the favicon drawing below 24), the wordmark
  at 13 pixels tall (the same x-height as Method Budget's at 18).
- Use the `-dark` files on dark backgrounds. In HTML, a `<picture>` with a
  `prefers-color-scheme: dark` source does this; the site does.

### The wordmark

`method schedule` in lowercase, drawn from strokes (not typed in a font): geometric round
letters, one weight, a 100-unit x-height with tall ascenders. The letters it shares with
Method Budget's wordmark (m, e, t, h, o, d, u) are the same drawings; s, c and l are new
and follow the same rules. Being drawn, it looks the same everywhere and ships no font.
The mark stands as tall as the ascenders and sits on the baseline. The name is ink, the
mark slate blue.

Our wordmark has no descender (Method Budget's g has one), so its files are 162 units
tall where theirs are 224. To set the two names at the same x-height, our wordmark's
height is 162/224 of theirs: 19 pixels in a header where theirs is 26, 16 in a footer
where theirs is 22.

### The app icon

The desktop app's icon is the mark in white on a slate blue (`#2F5FB3`) rounded square,
drawn by `scripts/make-icons.py` from the same rows as `make-brand.py` (it loads that
script, so the two cannot drift), as Method Budget's icon is drawn from its mark. The
square is the mark's 64-unit grid with corners of 14.3 units, the day break full white,
the other strips white at 55% (a touch over the mark's half strength, which reads dim on
the solid blue). From 48 pixels up the square sits inside the macOS icon grid's margin
(100 of 1024 a side); at 16 and 32 pixels it is full bleed and draws the 16-unit favicon
rows, which land on whole pixels and stay sharp. Regenerate with
`python3 scripts/make-icons.py`.

Regenerate the SVGs, favicon and touch icon with `python3 scripts/make-brand.py`
(`--check` changes nothing and exits 1 when a file is out of date or a color pair below
fails its contrast).

## Color

The neutrals, the amber and the token names are Method Budget's, value for value: warm
paper and ink on the light side, the same dark surfaces on the other. That is what makes
the two sites read as one family, and it means the neutral part can move into one shared
tokens file (the suite plan) with no change here. What is ours is the accent: the desktop
app's own slate blue, in the same lightness band as Method Budget's teal (relative
luminance 0.12 against the teal's 0.14), so the two accents read as a pair.

Amber (`--note`, `--note-fill`) is the family's caution color, kept with Method Budget's
meaning: the Early alpha pill and notes that ask for care. It is never an accent.

| Token | Light | Dark | Use |
|---|---|---|---|
| `--bg` | `#F7F6F2` | `#111417` | Page (shared) |
| `--surface` | `#FFFFFF` | `#1B1F24` | Cards, panels (shared) |
| `--surface-2` | `#EFEDE7` | `#22272E` | Inset areas, table heads (shared) |
| `--ink` | `#1C2228` | `#E4E8EC` | Text (shared) |
| `--ink-soft` | `#3A424C` | `#C3CAD2` | Secondary text (shared) |
| `--muted` | `#59626D` | `#9AA3AD` | Captions, fine print (shared) |
| `--rule` | `#DEDBD3` | `#2B3138` | Dividers, decorative (shared) |
| `--rule-strong` | `#8F897C` | `#67717D` | Borders that carry meaning (shared) |
| `--accent` | `#2F5FB3` | `#7AA2F0` | Links, buttons, the mark (the app's own) |
| `--accent-strong` | `#244A8F` | `#A5C1F6` | Hover, counts |
| `--accent-ink` | `#FFFFFF` | `#0E1420` | Text on an accent fill |
| `--accent-soft` | `#E3EAF7` | `#1C2A44` | Selection, highlights |
| `--note` | `#8A5A00` | `#E3B341` | Amber: caution, notes (shared) |

The desktop app uses the same neutrals and accent under its own token names in
`crates/method-schedule-app/ui/styles.css` (look A, decision 204), plus the stripboard
colors below. Graphite and Ledger (decision 208) are optional looks in the app only; the
site and the brand stay Paper.

The home page's app picture also uses the app's stripboard colors as tokens
(`--strip-int-day`, `--strip-ext-day`, `--strip-int-night`, `--strip-ext-night`,
`--strip-banner`, `--break-bg`, `--break-ink`), so `site.css` still names no color of its
own.

### Contrast (WCAG 2.x)

Body text needs 4.5:1 (AA); borders that carry meaning need 3:1. Computed from
tokens.css by `python3 scripts/make-brand.py --contrast` (the same pairs as Method
Budget's table, plus accent on surface, accent-strong on accent-soft, and the
illustration's day break, exterior day strip and terminal); `--check` fails when one
drops below what it needs.

| Pair | Light | Dark | Needs |
|---|---|---|---|
| ink on bg | 14.84 | 15.01 | 4.5 |
| ink on surface-2 | 13.71 | 12.20 | 4.5 |
| ink-soft on surface-2 | 8.69 | 9.09 | 4.5 |
| muted on bg | 5.72 | 7.23 | 4.5 |
| muted on surface-2 | 5.29 | 5.88 | 4.5 |
| accent on bg | 5.70 | 7.26 | 4.5 |
| accent on surface | 6.16 | 6.51 | 4.5 |
| accent on surface-2 | 5.26 | 5.91 | 4.5 |
| accent on accent-soft | 5.10 | 5.64 | 4.5 |
| accent-strong on accent-soft | 7.09 | 7.90 | 4.5 |
| accent-ink on accent | 6.16 | 7.24 | 4.5 |
| ink on accent-soft | 13.28 | 11.65 | 4.5 |
| note on surface | 5.93 | 8.51 | 4.5 |
| rule-strong on bg | 3.22 | 3.73 | 3 |
| break-ink on break-bg | 12.92 | 13.12 | 4.5 |
| ink on strip-ext-day | 14.47 | 9.97 | 4.5 |
| term-prompt on term-bg | 7.01 | 7.01 | 4.5 |
| term-dim on term-bg | 6.98 | 6.98 | 4.5 |

The terminal sample on the site is dark in both schemes (`--term-*` tokens, Method
Budget's with our accent as the prompt).

## Type

The system's own faces: San Francisco on a Mac, Segoe UI on Windows, the desktop's sans on
Linux; `ui-monospace` and its fallbacks for code. That is what the app uses, so nothing is
downloaded and the site matches the app on every machine. Figures in tables use tabular
numbers (`font-variant-numeric: tabular-nums`) so columns line up.

Headings are bold and tightly tracked (`letter-spacing: -.02em`); body text is 17 pixels
on 1.6 line height. The wordmark is artwork, so it needs no font. All of this is Method
Budget's.

## Imagery

- No stock photos and no other product's screens, logos or text.
- The home page's app picture is HTML drawn from the app's own layout with an invented
  schedule. Real screenshots replace it later: captures of invented productions
  (`method-schedule sample`), never a real one (docs/WEBSITE.md).
- The share card (`site/og.png`, 1200 by 630) is drawn from `site/og-card.html` by
  `scripts/make-og.sh`.
