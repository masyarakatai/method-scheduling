//! Fuzz tests for the script readers (decision 175): seeded scripts in the shape of the
//! ZTEST scripts (`scripts/make-ztest-fdx.py`), as Final Draft and as Fountain, mutated
//! thousands of times. A reader never panics, refuses or reads, and what it reads imports
//! into a schedule that passes `check`, every sheet a length in range. Inputs stay small
//! (a mutation adds a few hundred bytes at most); large inputs that would show a reader
//! slower than linear have a test of their own.
//!
//! More rounds: `METHOD_SCHEDULE_FUZZ_ROUNDS=50`, and `METHOD_SCHEDULE_FUZZ_SEED` for
//! other cases (`crate::testgen`).

use crate::fountain::{ImportedScript, LINES_PER_PAGE};
use crate::model::MAX_SHEET_EIGHTHS;
use crate::revise::import_tagged;
use crate::testgen::{fuzz_rng, mutate_bytes, rounds, Rng};
use crate::{fdx, fountain, Schedule};

/// The ZTEST cast, as in `scripts/make-ztest-fdx.py`.
const CAST: &[&str] = &["ADA", "BRUNO", "CLEO", "DARIO", "EVE", "FINN"];
/// Length attributes in the forms the length run tried, and some no script should have.
const LENGTHS: &[&str] = &[
    "8/8",
    "1 0/8",
    "1",
    "4/8",
    "1 2/8",
    "2/8",
    "3/8",
    "5/8",
    "10/8",
    "2 1/8",
    "",
    "0",
    "-1",
    "0/8",
    "1/0",
    "abc",
    "999",
    "1000",
    "4294967296",
    "1 9/8",
    "7/16",
    " 3/8 ",
];
const PREFIXES: &[&str] = &["INT.", "EXT.", "INT./EXT.", "I/E", "EST.", ""];
const TIMES: &[&str] = &["DAY", "NIGHT", "CONTINUOUS", "", "LATER - DAY"];

fn pick<'a>(rng: &mut Rng, v: &[&'a str]) -> &'a str {
    v[rng.below(v.len() as u64) as usize]
}

fn heading(rng: &mut Rng, n: u64) -> String {
    let tod = pick(rng, TIMES);
    let room = format!("ROOM {n}");
    match (pick(rng, PREFIXES), tod) {
        ("", "") => room,
        ("", t) => format!("{room} - {t}"),
        (p, "") => format!("{p} {room}"),
        (p, t) => format!("{p} {room} - {t}"),
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn para(kind: &str, text: &str) -> String {
    format!(
        "<Paragraph Type=\"{kind}\">\n<Text>{}</Text>\n</Paragraph>\n",
        escape(text)
    )
}

/// A ZTEST-style Final Draft script: numbered scene headings with lengths in many forms,
/// action, the ZTEST cast speaking, now and then dual dialogue, a title page and
/// production tags. Returns the text and its number of scenes.
fn ztest_fdx(rng: &mut Rng) -> (String, usize) {
    let scenes = 1 + rng.below(25);
    let tagged = rng.below(3) == 0;
    let mut body = String::new();
    for n in 1..=scenes {
        let number = match rng.below(6) {
            0 => String::new(),
            1 => format!(" Number=\"{n}A\""),
            _ => format!(" Number=\"{n}\""),
        };
        let page = match rng.below(8) {
            0 => String::new(),
            1 => " Page=\"0\"".to_string(),
            _ => format!(" Page=\"{n}\""),
        };
        let length = pick(rng, LENGTHS);
        let text = if tagged && rng.below(2) == 0 {
            format!(
                "<Text>INT. </Text><Text TagNumber=\"{}\">ROOM {n}</Text><Text> - DAY</Text>",
                1 + rng.below(4)
            )
        } else {
            format!("<Text>{}</Text>", escape(&heading(rng, n)))
        };
        body.push_str(&format!(
            "<Paragraph{number} Type=\"Scene Heading\">\n\
             <SceneProperties Length=\"{length}\"{page} Title=\"\"/>\n{text}\n</Paragraph>\n"
        ));
        body.push_str(&para("Action", &format!("Room {n}. Nothing happens.")));
        for _ in 0..rng.below(4) {
            let name = pick(rng, CAST);
            if rng.below(6) == 0 {
                body.push_str("<Paragraph Type=\"General\">\n<DualDialogue>\n");
                body.push_str(&para("Character", name));
                body.push_str(&para("Dialogue", "Both at once."));
                body.push_str(&para("Character", pick(rng, CAST)));
                body.push_str(&para("Dialogue", "Both at once."));
                body.push_str("</DualDialogue>\n</Paragraph>\n");
            } else {
                let cue = match rng.below(4) {
                    0 => format!("{name} (V.O.)"),
                    _ => name.to_string(),
                };
                body.push_str(&para("Character", &cue));
                if rng.below(3) == 0 {
                    body.push_str(&para("Parenthetical", "(quietly)"));
                }
                body.push_str(&para("Dialogue", &format!("Line in room {n}.")));
            }
        }
    }
    let title = if rng.below(2) == 0 {
        format!(
            "<TitlePage>\n<Content>\n{}</Content>\n</TitlePage>\n",
            para("Text", "ZTEST Lantern Creek")
        )
    } else {
        String::new()
    };
    let tags = if tagged {
        "<TagData>\n<TagCategories>\n<TagCategory Id=\"c1\" Name=\"Props\" Number=\"7\"/>\n\
         <TagCategory Id=\"c2\" Name=\"Set Dressing\" Number=\"8\"/>\n</TagCategories>\n\
         <TagDefinitions>\n<TagDefinition CatId=\"c1\" Id=\"d1\" Label=\"LANTERN\"/>\n\
         <TagDefinition CatId=\"c2\" Id=\"d2\" Label=\"\"/>\n\
         <TagDefinition CatId=\"c2\" Id=\"d3\" Label=\"Boathouse\"/>\n</TagDefinitions>\n\
         <Tags>\n<Tag Number=\"1\"><DefId>d1</DefId></Tag>\n\
         <Tag Number=\"2\"><DefId>d2</DefId></Tag>\n\
         <Tag Number=\"3\"><DefId>d3</DefId></Tag>\n</Tags>\n</TagData>\n"
    } else {
        ""
    };
    let xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\" ?>\n\
         <FinalDraft DocumentType=\"Script\" Template=\"No\" Version=\"5\">\n{title}\
         <Content>\n{body}</Content>\n{tags}</FinalDraft>\n"
    );
    (xml, scenes as usize)
}

/// The same kind of script in Fountain: a title page, headings with scene numbers,
/// action, cues with extensions and parentheticals, page breaks, notes, boneyard and
/// sections. Returns the text and its number of scenes.
fn ztest_fountain(rng: &mut Rng) -> (String, usize) {
    let scenes = 1 + rng.below(25);
    let mut out = String::new();
    if rng.below(2) == 0 {
        out.push_str("Title: ZTEST Lantern Creek\nAuthor: Invented Writer\n\n");
    }
    for n in 1..=scenes {
        let number = if rng.below(2) == 0 {
            format!(" #{n}#")
        } else {
            String::new()
        };
        // A heading must have a prefix, or be forced with a period.
        let h = heading(rng, n);
        let h = if PREFIXES[..5].iter().any(|p| h.starts_with(p)) {
            h
        } else {
            format!(".{h}")
        };
        out.push_str(&format!("{h}{number}\n\n"));
        for _ in 0..rng.below(4) {
            match rng.below(8) {
                0 => out.push_str("===\n\n"),
                1 => out.push_str("[[A note that runs on.]]\n\n"),
                2 => out.push_str("/* Boneyard\n\nINT. CUT - DAY\n\n*/\n\n"),
                3 => out.push_str("# Act One\n\n= A synopsis.\n\n"),
                _ => {
                    let name = pick(rng, CAST);
                    let ext = if rng.below(4) == 0 { " (O.S.)" } else { "" };
                    let paren = if rng.below(3) == 0 { "(beat)\n" } else { "" };
                    out.push_str(&format!(
                        "Room {n}. Something happens over a long line of action.\n\n\
                         {name}{ext}\n{paren}Line in room {n}.\n\n"
                    ));
                }
            }
        }
    }
    (out, scenes as usize)
}

/// What a reader gives must import cleanly, with every sheet a length in range: from
/// `shortest` (1/8 for Fountain, decision 9; a Final Draft length is used as written, and
/// `0` is a length the reference app shows, behavior note 16) to a sheet's limit.
fn imports_sanely(script: &ImportedScript, shortest: u32, why: &str) {
    for sc in &script.scenes {
        assert!(sc.page >= 1, "{why}: page {}", sc.page);
        assert!(
            (shortest..=MAX_SHEET_EIGHTHS).contains(&sc.length.0),
            "{why}: length {}",
            sc.length.0
        );
    }
    let mut s = Schedule::new("ZTEST Fuzz");
    let cast = s.cast().expect("a new schedule has a cast");
    let sheets = import_tagged(&mut s, script, cast).unwrap_or_else(|e| panic!("{why}: {e}"));
    assert_eq!(sheets.len(), script.scenes.len(), "{why}");
    s.check().unwrap_or_else(|e| panic!("{why}: {e}"));
    for sh in &s.sheets {
        assert!(
            (shortest..=MAX_SHEET_EIGHTHS).contains(&sh.pages.0),
            "{why}"
        );
    }
}

#[test]
fn ztest_scripts_read_as_written() {
    let mut rng = fuzz_rng(0x27_fd);
    for case in 0..rounds(200) {
        let (xml, n) = ztest_fdx(&mut rng);
        let script =
            fdx::parse_with_tags(&xml).unwrap_or_else(|e| panic!("case {case}: {e}\n{xml}"));
        assert_eq!(script.scenes.len(), n, "case {case}");
        imports_sanely(&script, 0, &format!("fdx case {case}"));
        let (text, n) = ztest_fountain(&mut rng);
        let script = fountain::parse(&text);
        assert_eq!(script.scenes.len(), n, "case {case}\n{text}");
        imports_sanely(&script, 1, &format!("fountain case {case}"));
    }
}

const FDX_SNIPPETS: &[&[u8]] = &[
    b"<",
    b">",
    b"/>",
    b"&",
    b"&amp;",
    b"&#x;",
    b"&#0;",
    b"&#x110000;",
    b"\"",
    b"]]>",
    b"<!--",
    b"<![CDATA[",
    b"<?pi?>",
    b"<!DOCTYPE x>",
    b"</Paragraph>",
    b"<Paragraph Type=\"Scene Heading\">",
    b"<SceneProperties Length=\"99999\" Page=\"-3\"/>",
    b"<Text TagNumber=\"1\">",
    b"<DualDialogue>",
    b"\xC3",
    b"\xEF\xBB\xBF",
    b"\r\n",
];

const FOUNTAIN_SNIPPETS: &[&[u8]] = &[
    b"\n",
    b"\n\n",
    b"INT. ",
    b".",
    b"@",
    b"#",
    b"#1A#",
    b"===",
    b"[[",
    b"]]",
    b"/*",
    b"*/",
    b"(",
    b"^",
    b" - ",
    b"Title:",
    b"\r",
    b"\t",
    b"\xC3\xA9",
    b"\xE2\x80\xA8",
    b"\0",
];

#[test]
fn mutated_fdx_scripts_never_panic() {
    let mut rng = fuzz_rng(0xfd_f022);
    for case in 0..rounds(1500) {
        let (xml, _) = ztest_fdx(&mut rng);
        let mut bytes = xml.into_bytes();
        mutate_bytes(&mut rng, &mut bytes, FDX_SNIPPETS);
        let text = String::from_utf8_lossy(&bytes);
        let why = format!("fdx mutation case {case}");
        if let Ok(script) = fdx::parse(&text) {
            imports_sanely(&script, 0, &why);
        }
        if let Ok(script) = fdx::parse_with_tags(&text) {
            imports_sanely(&script, 0, &why);
        }
    }
}

#[test]
fn mutated_fountain_scripts_never_panic() {
    let mut rng = fuzz_rng(0xf0_022);
    for case in 0..rounds(1500) {
        let (text, _) = ztest_fountain(&mut rng);
        let mut bytes = text.into_bytes();
        mutate_bytes(&mut rng, &mut bytes, FOUNTAIN_SNIPPETS);
        let text = String::from_utf8_lossy(&bytes);
        imports_sanely(
            &fountain::parse(&text),
            1,
            &format!("fountain mutation case {case}"),
        );
    }
}

/// Large inputs built to trip a reader that is slower than linear: each is a few hundred
/// kilobytes, so a quadratic step would take minutes, not the fraction of a second these
/// take.
#[test]
fn large_scripts_read_in_linear_time() {
    let n = 100_000;
    let fountains = [
        "[[".repeat(n),
        "/*".repeat(n),
        "[[a]]\n".repeat(n),
        "\n".repeat(4 * n),
        "x".repeat(4 * n),
        "x ".repeat(2 * n),
        "INT. A - DAY\n\n".repeat(n / 4),
        "BOB\nHi.\n\n".repeat(n / 4),
        "===\na\n".repeat(n / 2),
        format!("Title: {}\n\nINT. A - DAY\n", "T ".repeat(n)),
    ];
    for text in &fountains {
        imports_sanely(&fountain::parse(text), 1, "a large Fountain script");
    }
    let head = "<?xml version=\"1.0\"?>\n<FinalDraft>\n<Content>\n";
    let tail = "</Content>\n</FinalDraft>\n";
    let fdxs = [
        format!("{head}{}{tail}", "<Paragraph/>".repeat(n)),
        format!(
            "{head}{}{tail}",
            "<Paragraph Type=\"Action\"><Text>a</Text></Paragraph>".repeat(n / 4)
        ),
        format!("{head}{}{tail}", "&amp;".repeat(n)),
        format!("{head}{}", "<Paragraph>".repeat(n)),
        format!("{head}<Paragraph a=\"{}\"/>{tail}", "x".repeat(4 * n)),
        format!("{head}{}{tail}", "<!-- c -->".repeat(n)),
    ];
    for text in &fdxs {
        if let Ok(script) = fdx::parse_with_tags(text) {
            imports_sanely(&script, 0, "a large Final Draft script");
        }
    }
}

/// Decision 175: a Fountain script whose page breaks put a scene past about 268 million
/// lines overflowed the length sum (a debug build panicked, a release build wrapped to
/// a wrong length). Lengths now count in 64 bits and stay in range.
#[test]
fn a_fountain_scene_far_down_the_script_keeps_a_sane_length() {
    let far = u32::MAX / 16 + 1;
    // Three pages are 24 eighths wherever they fall.
    let n = LINES_PER_PAGE;
    assert_eq!(fountain::length(far - 3 * n, far, n).0, 24);
    assert_eq!(fountain::length(0, u32::MAX, n).0, MAX_SHEET_EIGHTHS);
    assert_eq!(fountain::length(u32::MAX - 14, u32::MAX, n).0, 2);
}
