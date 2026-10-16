//! Banners on the command line and over MCP: their style (decisions.md, entries 65 and
//! 87: bold, a palette color, one or two lines high, a text size and an alignment; entry
//! 187: a font family) read
//! and set the same way on both, and the `banners` and `banner` commands.

use method_schedule_core::model::{
    Align, Banner, BannerColor, BannerFont, BannerStyle, Minutes, Place, StripId, StripKind,
    TextSize,
};
use serde_json::{json, Value};

use super::{chosen_board, first, flag, hm, load, save};

/// The longest banner text, in characters (the app's limit).
pub const MAX_TEXT: usize = 200;
/// The longest banner duration, in minutes: a day.
pub const MAX_MINUTES: u32 = 24 * 60;

/// The alignments a banner takes. Left is the usual way.
pub const ALIGNS: [&str; 3] = ["left", "center", "right"];

/// A style change: each part given is set, the rest kept. `color` `Some(None)` clears it.
#[derive(Default)]
pub struct StyleChange<'a> {
    pub bold: Option<bool>,
    pub color: Option<Option<&'a str>>,
    pub tall: Option<bool>,
    pub size: Option<&'a str>,
    pub align: Option<&'a str>,
    pub font: Option<&'a str>,
}

impl StyleChange<'_> {
    pub fn apply(&self, st: &mut BannerStyle) -> Result<(), String> {
        if let Some(b) = self.bold {
            st.bold = b;
        }
        if let Some(c) = self.color {
            st.color = match c.map(str::trim).filter(|c| !c.is_empty() && *c != "none") {
                None => None,
                Some(c) => Some(BannerColor::from_text(c).ok_or_else(|| {
                    format!(
                        "{c:?} is not a banner color ({}, a lowercase #rrggbb, or none)",
                        BannerColor::NAMED
                            .map(|c| c.name().unwrap_or_default())
                            .join(", ")
                    )
                })?),
            };
        }
        if let Some(t) = self.tall {
            st.tall = t;
        }
        if let Some(sz) = self.size {
            st.size = TextSize::from_name(sz)
                .ok_or_else(|| format!("{sz:?} is not a text size (small, normal, large)"))?;
        }
        if let Some(a) = self.align {
            // Left and the usual way are one for a banner, as in the app.
            st.align = match Align::from_name(a) {
                Some(Align::Left | Align::Auto) => Align::Auto,
                Some(x) => x,
                None => {
                    return Err(format!(
                        "{a:?} is not a banner alignment (left, center, right)"
                    ))
                }
            };
        }
        if let Some(f) = self.font {
            st.font = BannerFont::from_name(f)
                .ok_or_else(|| format!("{f:?} is not a banner font (usual, serif, sans, mono)"))?;
        }
        Ok(())
    }
}

/// A banner's alignment by name: left for the usual way.
pub fn align_name(a: Align) -> &'static str {
    match a {
        Align::Auto | Align::Left => "left",
        x => x.name(),
    }
}

/// A banner's style as JSON: `{bold, color (null for the board's), tall, size, align,
/// font}`.
pub fn style_json(st: &BannerStyle) -> Value {
    json!({
        "bold": st.bold,
        "color": st.color.map(BannerColor::text),
        "tall": st.tall,
        "size": st.size.name(),
        "align": align_name(st.align),
        "font": st.font.name(),
    })
}

/// A banner's text, checked: trimmed, not empty, at most [`MAX_TEXT`] characters.
pub fn text(t: &str) -> Result<String, String> {
    let t = t.trim();
    if t.is_empty() {
        return Err("a banner needs text".to_string());
    }
    if t.chars().count() > MAX_TEXT {
        return Err(format!("a banner is at most {MAX_TEXT} characters"));
    }
    Ok(t.to_string())
}

/// A banner's duration, checked: at most a day.
pub fn minutes(m: u32) -> Result<Minutes, String> {
    if m > MAX_MINUTES {
        return Err(format!("a banner lasts at most {MAX_MINUTES} minutes"));
    }
    Ok(Minutes(m))
}

/// The style flags: `--bold`, `--no-bold`, `--color NAME|none`, `--height 1|2`,
/// `--size small|normal|large`, `--align left|center|right`, `--font usual|serif|sans|mono`.
fn style_flags(args: &[String]) -> Result<StyleChange<'_>, String> {
    let bold = match (has(args, "--bold"), has(args, "--no-bold")) {
        (true, true) => return Err("--bold and --no-bold together".to_string()),
        (true, false) => Some(true),
        (false, true) => Some(false),
        (false, false) => None,
    };
    let tall = match flag(args, "--height") {
        None => None,
        Some("1") => Some(false),
        Some("2") => Some(true),
        Some(h) => return Err(format!("--height {h:?}: 1 or 2 lines")),
    };
    Ok(StyleChange {
        bold,
        color: flag(args, "--color").map(Some),
        tall,
        size: flag(args, "--size"),
        align: flag(args, "--align"),
        font: flag(args, "--font"),
    })
}

fn has(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

/// One banner as a line: strip id, where, duration, style and text.
fn line(where_: &str, id: StripId, b: &Banner) -> String {
    format!(
        "{:>6}  {where_:<10} {:>6}  {:<30} {}\n",
        id.0,
        hm(b.duration),
        b.style.text(),
        b.text
    )
}

/// `banners FILE`: every banner of the board (`--board`, the first by default), its sub-boards in order then the
/// Boneyard, with strip id, duration and style.
pub fn banners_cmd(args: &[String]) -> Result<String, String> {
    let [path] = args else {
        return Err("usage: method-schedule banners FILE".to_string());
    };
    let s = load(path)?;
    let b = chosen_board(&s)?;
    let mut out = String::new();
    let lists = b
        .sub_boards
        .iter()
        .map(|x| (x.name.as_str(), &x.strips))
        .chain(std::iter::once(("Boneyard", &b.boneyard)));
    for (name, strips) in lists {
        for st in strips {
            if let StripKind::Banner(bn) = &st.kind {
                out.push_str(&line(name, st.id, bn));
            }
        }
    }
    if out.is_empty() {
        out.push_str("no banners\n");
    }
    Ok(out)
}

/// `banner FILE --add TEXT [--at N] [--minutes N] [style]` inserts a banner on the first
/// sub-board; `banner FILE STRIP [--text T] [--minutes N] [style]` changes one, keeping
/// what is not given. Saves, then prints the banner.
pub fn banner_cmd(args: &[String]) -> Result<String, String> {
    let usage = "usage: method-schedule banner FILE (--add TEXT [--at N] | STRIP [--text TEXT]) \
                 [--minutes N] [--bold | --no-bold] [--color NAME|none] [--height 1|2] \
                 [--size small|normal|large] [--align left|center|right] \
                 [--font usual|serif|sans|mono]";
    let path = args.first().ok_or(usage)?;
    let rest = &args[1..];
    let mut s = load(path)?;
    let (board, sb) = first(&s)?;
    let style = style_flags(rest)?;
    let mins = flag(rest, "--minutes")
        .map(|m| {
            m.parse::<u32>()
                .map_err(|_| format!("--minutes {m:?}: a whole number of minutes"))
                .and_then(minutes)
        })
        .transpose()?;
    let id = if let Some(t) = flag(rest, "--add") {
        let mut b = Banner {
            text: text(t)?,
            duration: mins.unwrap_or(Minutes(0)),
            style: BannerStyle::default(),
        };
        style.apply(&mut b.style)?;
        let index = match flag(rest, "--at") {
            None => usize::MAX,
            Some(n) => n
                .parse::<usize>()
                .map_err(|_| format!("--at {n:?}: a position from 0"))?,
        };
        let at = Place {
            sub_board: Some(sb),
            index: index.min(chosen_board(&s)?.sub_boards[0].strips.len()),
        };
        s.insert_strip(board, StripKind::Banner(b), at)
            .map_err(|e| e.to_string())?
    } else {
        let strip = rest.first().filter(|a| !a.starts_with("--")).ok_or(usage)?;
        let id = StripId(
            strip
                .parse()
                .map_err(|_| format!("{strip:?} is not a strip id"))?,
        );
        let on = chosen_board(&s)?;
        let old = on
            .sub_boards
            .iter()
            .flat_map(|x| &x.strips)
            .chain(&on.boneyard)
            .find(|x| x.id == id)
            .ok_or_else(|| format!("the board {:?} has no strip {}", on.name, id.0))?;
        let StripKind::Banner(old) = &old.kind else {
            return Err(format!("strip {} is not a banner", id.0));
        };
        let mut b = old.clone();
        if let Some(t) = flag(rest, "--text") {
            b.text = text(t)?;
        }
        if let Some(m) = mins {
            b.duration = m;
        }
        style.apply(&mut b.style)?;
        s.set_banner(board, id, b).map_err(|e| e.to_string())?;
        id
    };
    s.check().map_err(|e| e.to_string())?;
    save(path, &s)?;
    let b = chosen_board(&s)?;
    let found = b
        .sub_boards
        .iter()
        .map(|x| (x.name.as_str(), &x.strips))
        .chain(std::iter::once(("Boneyard", &b.boneyard)))
        .find_map(|(name, strips)| {
            strips
                .iter()
                .find(|x| x.id == id)
                .and_then(|x| match &x.kind {
                    StripKind::Banner(bn) => Some(line(name, id, bn)),
                    _ => None,
                })
        });
    Ok(found.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_changes_apply_and_refuse() {
        let mut st = BannerStyle::default();
        StyleChange {
            bold: Some(true),
            color: Some(Some("teal")),
            tall: Some(true),
            size: Some("large"),
            align: Some("center"),
            font: Some("serif"),
        }
        .apply(&mut st)
        .unwrap();
        assert_eq!(
            st.text(),
            "bold, teal, two lines, large text, centered, serif"
        );
        assert_eq!(style_json(&st)["font"], "serif");
        assert_eq!(style_json(&st)["color"], "teal");
        // Left is the usual way; none clears the color; what is not given is kept.
        StyleChange {
            color: Some(Some("none")),
            align: Some("left"),
            ..Default::default()
        }
        .apply(&mut st)
        .unwrap();
        assert_eq!(st.align, Align::Auto);
        assert_eq!(st.color, None);
        assert!(st.bold && st.tall);
        assert_eq!(style_json(&st)["align"], "left");
        assert_eq!(style_json(&st)["color"], Value::Null);
        for bad in [
            StyleChange {
                color: Some(Some("mauve")),
                ..Default::default()
            },
            StyleChange {
                size: Some("huge"),
                ..Default::default()
            },
            StyleChange {
                align: Some("justify"),
                ..Default::default()
            },
            StyleChange {
                font: Some("Papyrus"),
                ..Default::default()
            },
        ] {
            let before = st;
            assert!(bad.apply(&mut st).is_err());
            assert_eq!(st, before);
        }
        assert!(text("  ").is_err());
        assert!(text(&"x".repeat(MAX_TEXT + 1)).is_err());
        assert!(minutes(MAX_MINUTES + 1).is_err());
    }
}
