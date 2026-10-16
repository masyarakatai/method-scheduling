//! Document properties for a PDF we printed: title, subject, author and creator, so a
//! saved report indexes and searches well (decisions.md, entry 96).
//!
//! The PDF is left as it is and an incremental update is added at its end (ISO 32000-1,
//! 7.5.6): a new document information dictionary, a cross-reference section naming it,
//! and a trailer pointing back at the old one with `/Prev`. Readers take the newest
//! trailer, so they see the new properties; the pages are untouched byte for byte.
//!
//! Only a file with a classic cross-reference table (the `xref` keyword, as macOS's
//! print system writes) and no encryption is updated; for anything else [`set_info`]
//! gives `None` and the caller keeps the file as it was. Text goes in as UTF-16BE hex
//! strings with a byte order mark, so any title reads back exactly and nothing in it can
//! end the string early.

/// The properties to set. An empty one is left out.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfInfo {
    pub title: String,
    pub subject: String,
    /// The production company from the schedule's production info.
    pub author: String,
    /// The program that made the document.
    pub creator: String,
}

/// The longest property kept, in characters.
const MAX_CHARS: usize = 500;

/// `pdf` with an incremental update setting `info`, or `None` when the file isn't one
/// this can update safely (see the module notes).
pub fn set_info(pdf: &[u8], info: &PdfInfo) -> Option<Vec<u8>> {
    let prev = last_startxref(pdf)?;
    if !pdf.get(prev..)?.starts_with(b"xref") {
        return None;
    }
    let trailer = trailer_dict(pdf, prev)?;
    if find(trailer, b"/Encrypt").is_some() {
        return None;
    }
    let size = int_after(trailer, b"/Size")?;
    let root = reference_after(trailer, b"/Root")?;
    let id = find(trailer, b"/ID").and_then(|at| bracketed(&trailer[at + 3..]));
    // Keep the old dictionary's creation date and producer when they are plain.
    let old = reference_after(trailer, b"/Info").and_then(|(n, g)| object(pdf, n, g));
    let keep: Vec<(&str, &[u8])> = ["/CreationDate", "/Producer"]
        .into_iter()
        .filter_map(|k| Some((k, old.and_then(|o| literal_after(o, k.as_bytes()))?)))
        .collect();

    let mut out = pdf.to_vec();
    if !out.ends_with(b"\n") && !out.ends_with(b"\r") {
        out.push(b'\n');
    }
    let at = out.len();
    let mut dict = String::from("<<");
    for (key, value) in [
        ("/Title", &info.title),
        ("/Subject", &info.subject),
        ("/Author", &info.author),
        ("/Creator", &info.creator),
    ] {
        let value = value.trim();
        if !value.is_empty() {
            dict.push_str(&format!(" {key} {}", hex_text(value)));
        }
    }
    out.extend_from_slice(format!("{size} 0 obj\n{dict}").as_bytes());
    for (key, raw) in keep {
        out.extend_from_slice(format!(" {key} ").as_bytes());
        out.extend_from_slice(raw);
    }
    out.extend_from_slice(b" >>\nendobj\n");
    let xref = out.len();
    out.extend_from_slice(
        format!(
            "xref\n{size} 1\n{at:010} 00000 n\r\ntrailer\n<< /Size {} /Root {} {} R /Info {size} 0 R /Prev {prev}",
            size + 1,
            root.0,
            root.1
        )
        .as_bytes(),
    );
    if let Some(id) = id {
        out.extend_from_slice(b" /ID ");
        out.extend_from_slice(id);
    }
    out.extend_from_slice(format!(" >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
    Some(out)
}

/// `<FEFF...>`: the text as UTF-16BE hex with a byte order mark, cut to [`MAX_CHARS`].
fn hex_text(s: &str) -> String {
    let s: String = s.chars().take(MAX_CHARS).collect();
    let mut out = String::from("<FEFF");
    for u in s.encode_utf16() {
        out.push_str(&format!("{u:04X}"));
    }
    out.push('>');
    out
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn rfind(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).rposition(|w| w == needle)
}

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | b'\n' | b'\x0c' | b'\0')
}

fn skip_space(s: &[u8]) -> &[u8] {
    let n = s.iter().take_while(|b| is_space(**b)).count();
    &s[n..]
}

/// A whole number at the start of `s` (after any white space), and the rest.
fn int(s: &[u8]) -> Option<(usize, &[u8])> {
    let s = skip_space(s);
    let n = s.iter().take_while(|b| b.is_ascii_digit()).count();
    if n == 0 || n > 12 {
        return None;
    }
    let v = std::str::from_utf8(&s[..n]).ok()?.parse().ok()?;
    Some((v, &s[n..]))
}

/// The offset after the file's last `startxref`.
fn last_startxref(pdf: &[u8]) -> Option<usize> {
    let at = rfind(pdf, b"startxref")?;
    let (n, _) = int(&pdf[at + 9..])?;
    (n < pdf.len()).then_some(n)
}

/// The trailer dictionary after the cross-reference table at `xref`, `<<` to its `>>`.
fn trailer_dict(pdf: &[u8], xref: usize) -> Option<&[u8]> {
    let from = xref + find(&pdf[xref..], b"trailer")? + 7;
    let rest = skip_space(&pdf[from..]);
    if !rest.starts_with(b"<<") {
        return None;
    }
    let mut depth = 0usize;
    let mut i = 0;
    while i + 1 < rest.len() {
        match &rest[i..i + 2] {
            b"<<" => {
                depth += 1;
                i += 2;
            }
            b">>" => {
                depth -= 1;
                i += 2;
                if depth == 0 {
                    return Some(&rest[..i]);
                }
            }
            _ => i += 1,
        }
    }
    None
}

/// What follows the key `key` (not a longer key that starts the same).
fn key_value<'a>(dict: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let mut from = 0;
    while let Some(at) = find(&dict[from..], key) {
        let end = from + at + key.len();
        // `/Info` must not match `/InfoX`.
        if dict.get(end).is_none_or(|b| !b.is_ascii_alphanumeric()) {
            return Some(&dict[end..]);
        }
        from = end;
    }
    None
}

fn int_after(dict: &[u8], key: &[u8]) -> Option<usize> {
    int(key_value(dict, key)?).map(|(n, _)| n)
}

/// `12 0 R` after `key`, as (12, 0).
fn reference_after(dict: &[u8], key: &[u8]) -> Option<(usize, usize)> {
    let (n, rest) = int(key_value(dict, key)?)?;
    let (g, rest) = int(rest)?;
    skip_space(rest).starts_with(b"R").then_some((n, g))
}

/// `[ ... ]` at the start of `s` (after white space), brackets included, with no nested
/// brackets (an `/ID` holds two strings).
fn bracketed(s: &[u8]) -> Option<&[u8]> {
    let s = skip_space(s);
    if !s.starts_with(b"[") {
        return None;
    }
    let end = s.iter().position(|b| *b == b']')?;
    (!s[1..end].contains(&b'[')).then_some(&s[..=end])
}

/// The body of object `n g` (the last one in the file with that number), to `endobj`.
fn object(pdf: &[u8], n: usize, g: usize) -> Option<&[u8]> {
    let head = format!("{n} {g} obj");
    let mut best = None;
    let mut from = 0;
    while let Some(at) = find(&pdf[from..], head.as_bytes()) {
        let start = from + at;
        if start == 0 || is_space(pdf[start - 1]) {
            best = Some(start);
        }
        from = start + head.len();
    }
    let start = best? + head.len();
    let end = start + find(&pdf[start..], b"endobj")?;
    Some(&pdf[start..end])
}

/// A plain literal string `( ... )` after `key`: no backslashes and no parentheses
/// inside, so it can be copied as it is.
fn literal_after<'a>(dict: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let s = skip_space(key_value(dict, key)?);
    if !s.starts_with(b"(") {
        return None;
    }
    let end = s.iter().position(|b| *b == b')')?;
    let inner = &s[1..end];
    (!inner.contains(&b'\\') && !inner.contains(&b'(') && inner.len() <= 200).then_some(&s[..=end])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small PDF with a classic cross-reference table, offsets worked out.
    fn sample(trailer_extra: &str) -> Vec<u8> {
        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>",
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>",
            "<< /Producer (macOS Quartz PDFContext) /CreationDate (D:20271001120000Z00'00') /Title (Old) >>",
        ];
        let mut out = b"%PDF-1.3\n".to_vec();
        let mut at = Vec::new();
        for (i, o) in objects.iter().enumerate() {
            at.push(out.len());
            out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
        }
        let xref = out.len();
        out.extend_from_slice(b"xref\n0 5\n0000000000 65535 f\r\n");
        for a in at {
            out.extend_from_slice(format!("{a:010} 00000 n\r\n").as_bytes());
        }
        out.extend_from_slice(
            format!(
                "trailer\n<< /Size 5 /Root 1 0 R /Info 4 0 R /ID [ <AB12> <CD34> ]{trailer_extra} >>\nstartxref\n{xref}\n%%EOF\n"
            )
            .as_bytes(),
        );
        out
    }

    fn info() -> PdfInfo {
        PdfInfo {
            title: "ZTEST Harbor (Light) - One-line \u{e9}".into(),
            subject: "One-line schedule; Board 1, Main".into(),
            author: "ZTEST Pictures".into(),
            creator: "Method Schedule".into(),
        }
    }

    fn unhex(s: &str) -> String {
        let hex = s.strip_prefix("<FEFF").unwrap().strip_suffix('>').unwrap();
        let units: Vec<u16> = (0..hex.len())
            .step_by(4)
            .map(|i| u16::from_str_radix(&hex[i..i + 4], 16).unwrap())
            .collect();
        String::from_utf16(&units).unwrap()
    }

    #[test]
    fn adds_an_update_with_the_properties() {
        let pdf = sample("");
        let out = set_info(&pdf, &info()).unwrap();
        // The original is untouched; the update follows it.
        assert_eq!(&out[..pdf.len()], &pdf[..]);
        let tail = std::str::from_utf8(&out[pdf.len()..]).unwrap();
        assert!(tail.starts_with("5 0 obj\n<< /Title <FEFF"), "{tail}");
        assert!(tail.ends_with("%%EOF\n"));
        // The new startxref points at the new xref, whose entry points at object 5.
        let xref = last_startxref(&out).unwrap();
        assert!(out[xref..].starts_with(b"xref\n5 1\n"));
        let entry = std::str::from_utf8(&out[xref + 9..xref + 19]).unwrap();
        let obj: usize = entry.parse().unwrap();
        assert!(out[obj..].starts_with(b"5 0 obj"));
        assert_eq!(
            &out[xref + 9..xref + 29],
            format!("{obj:010} 00000 n\r\n").as_bytes()
        );
        // The trailer keeps the root and the ID and points back at the old table.
        let old = last_startxref(&pdf).unwrap();
        let t = trailer_dict(&out, xref).unwrap();
        assert_eq!(int_after(t, b"/Size"), Some(6));
        assert_eq!(reference_after(t, b"/Root"), Some((1, 0)));
        assert_eq!(reference_after(t, b"/Info"), Some((5, 0)));
        assert_eq!(int_after(t, b"/Prev"), Some(old));
        assert!(find(t, b"/ID [ <AB12> <CD34> ]").is_some());
        // Each property reads back exactly; the old producer and date carry over.
        let body = std::str::from_utf8(object(&out, 5, 0).unwrap()).unwrap();
        let value = |key: &str| {
            let at = body.find(key).unwrap() + key.len() + 1;
            let end = body[at..].find('>').unwrap() + at + 1;
            unhex(&body[at..end])
        };
        assert_eq!(value("/Title"), info().title);
        assert_eq!(value("/Subject"), info().subject);
        assert_eq!(value("/Author"), "ZTEST Pictures");
        assert_eq!(value("/Creator"), "Method Schedule");
        assert!(body.contains("/Producer (macOS Quartz PDFContext)"));
        assert!(body.contains("/CreationDate (D:20271001120000Z00'00')"));
        // Updating again chains on.
        let twice = set_info(&out, &info()).unwrap();
        let t = trailer_dict(&twice, last_startxref(&twice).unwrap()).unwrap();
        assert_eq!(int_after(t, b"/Prev"), Some(xref));
        assert_eq!(int_after(t, b"/Size"), Some(7));
    }

    #[test]
    fn empty_properties_are_left_out() {
        let out = set_info(
            &sample(""),
            &PdfInfo {
                title: "T".into(),
                creator: "Method Schedule".into(),
                ..PdfInfo::default()
            },
        )
        .unwrap();
        let body = std::str::from_utf8(object(&out, 5, 0).unwrap()).unwrap();
        assert!(body.contains("/Title <FEFF0054>"));
        assert!(!body.contains("/Author") && !body.contains("/Subject"));
    }

    #[test]
    fn leaves_alone_what_it_cannot_update() {
        // Encrypted.
        assert_eq!(set_info(&sample(" /Encrypt 9 0 R"), &info()), None);
        // A cross-reference stream instead of a table.
        let mut stream = sample("");
        let at = rfind(&stream, b"startxref").unwrap();
        stream.truncate(at);
        stream.extend_from_slice(b"startxref\n9\n%%EOF\n");
        assert_eq!(set_info(&stream, &info()), None);
        // Not a PDF, or cut off.
        assert_eq!(set_info(b"hello", &info()), None);
        let pdf = sample("");
        assert_eq!(set_info(&pdf[..pdf.len() / 2], &info()), None);
        assert_eq!(set_info(b"", &info()), None);
    }

    #[test]
    fn text_is_utf16_hex() {
        assert_eq!(hex_text("A"), "<FEFF0041>");
        assert_eq!(hex_text("\u{1f3ac}"), "<FEFFD83CDFAC>");
        assert_eq!(unhex(&hex_text(") >> endobj")), ") >> endobj");
        assert_eq!(unhex(&hex_text(&"x".repeat(900))).len(), MAX_CHARS);
    }
}
