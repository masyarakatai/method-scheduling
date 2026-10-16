//! CSV (RFC 4180) and tab-delimited text.
//!
//! - Records end in CRLF, the last one too. The header row comes first.
//! - CSV quotes a field only when it has to (a comma, a double quote, CR or LF in it) and
//!   doubles any double quote inside.
//! - Tab-delimited text never quotes, so tabs and line breaks inside a field become
//!   spaces.
//! - Whole numbers are plain digits, dates are `YYYY-MM-DD`.
//! - Text that a spreadsheet would run as a formula (starting with `=`, `+`, `-` or `@`,
//!   or a tab or CR) gets a leading `'`, unless it is a plain number such as `-5`.
//!   Schedule files can come from anyone, and opening an export must never run anything.
//! - A total that a workbook writes as a formula (`Cell::Formula`) is its value here: CSV
//!   carries values only.
//! - With `bom`, the file starts with a UTF-8 byte order mark, so Excel reads accented
//!   letters correctly.

use super::{Cell, Sheet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Comma,
    Tab,
}

const BOM: &str = "\u{feff}";

/// `sheet` as CSV or tab-delimited text, header row first.
pub fn write(sheet: &Sheet, delimiter: Delimiter, bom: bool) -> Vec<u8> {
    let mut out = String::new();
    if bom {
        out.push_str(BOM);
    }
    let header: Vec<Cell> = sheet
        .columns
        .iter()
        .map(|c| Cell::text(c.as_str()))
        .collect();
    for row in std::iter::once(&header).chain(&sheet.rows) {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                out.push(match delimiter {
                    Delimiter::Comma => ',',
                    Delimiter::Tab => '\t',
                });
            }
            let text = field_text(cell);
            match delimiter {
                Delimiter::Comma => push_csv_field(&mut out, &text),
                Delimiter::Tab => push_tsv_field(&mut out, &text),
            }
        }
        out.push_str("\r\n");
    }
    out.into_bytes()
}

/// A cell's text before quoting.
fn field_text(cell: &Cell) -> String {
    match cell {
        Cell::Empty => String::new(),
        Cell::Text(s) => neutralize(s),
        Cell::Int(n) => n.to_string(),
        Cell::Date(d) => d.to_string(),
        // Values only: a CSV never carries a formula.
        Cell::Formula(f) => f.value().to_string(),
    }
}

/// Text that would start a spreadsheet formula gets a leading `'`.
pub fn neutralize(s: &str) -> String {
    let risky = matches!(s.chars().next(), Some('=' | '+' | '-' | '@' | '\t' | '\r'));
    if risky && !is_plain_number(s) {
        format!("'{s}")
    } else {
        s.to_string()
    }
}

/// `-5`, `+12.50`, `-.5`: a sign, digits and at most one point.
fn is_plain_number(s: &str) -> bool {
    let body = s.strip_prefix(['-', '+']).unwrap_or(s);
    let mut digits = 0;
    let mut points = 0;
    for c in body.chars() {
        match c {
            '0'..='9' => digits += 1,
            '.' => points += 1,
            _ => return false,
        }
    }
    digits > 0 && points <= 1
}

fn push_csv_field(out: &mut String, s: &str) {
    if s.contains([',', '"', '\r', '\n']) {
        out.push('"');
        out.push_str(&s.replace('"', "\"\""));
        out.push('"');
    } else {
        out.push_str(s);
    }
}

fn push_tsv_field(out: &mut String, s: &str) {
    // CRLF is one break, so it becomes one space.
    let s = s.replace("\r\n", " ");
    out.extend(s.chars().map(|c| match c {
        '\t' | '\r' | '\n' => ' ',
        c => c,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet(rows: Vec<Vec<Cell>>) -> Sheet {
        Sheet {
            name: "Test",
            columns: vec!["Name".into(), "Days".into()],
            rows,
        }
    }

    fn csv(rows: Vec<Vec<Cell>>) -> String {
        String::from_utf8(write(&sheet(rows), Delimiter::Comma, false)).unwrap()
    }

    fn tsv(rows: Vec<Vec<Cell>>) -> String {
        String::from_utf8(write(&sheet(rows), Delimiter::Tab, false)).unwrap()
    }

    fn t(s: &str) -> Cell {
        Cell::text(s)
    }

    #[test]
    fn quotes_only_when_needed() {
        assert_eq!(
            csv(vec![vec![t("plain"), Cell::Int(3)]]),
            "Name,Days\r\nplain,3\r\n"
        );
        assert_eq!(
            csv(vec![
                vec![t("a, b"), Cell::Empty],
                vec![t("say \"hi\""), Cell::Empty],
                vec![t("two\nlines"), Cell::Empty],
                vec![t("cr\rhere"), Cell::Empty],
                vec![t("\"quoted\""), Cell::Empty],
                vec![t(" spaced "), Cell::Empty],
                vec![t("caf\u{e9} \u{1f3ac}"), Cell::Empty],
            ]),
            "Name,Days\r\n\
             \"a, b\",\r\n\
             \"say \"\"hi\"\"\",\r\n\
             \"two\nlines\",\r\n\
             \"cr\rhere\",\r\n\
             \"\"\"quoted\"\"\",\r\n\
             \x20spaced ,\r\n\
             caf\u{e9} \u{1f3ac},\r\n"
        );
    }

    #[test]
    fn empty_cells_and_rows_keep_their_commas() {
        assert_eq!(
            csv(vec![vec![Cell::Empty, Cell::Empty]]),
            "Name,Days\r\n,\r\n"
        );
        assert_eq!(csv(vec![]), "Name,Days\r\n");
    }

    #[test]
    fn numbers_and_dates_are_plain() {
        assert_eq!(
            csv(vec![
                vec![Cell::Int(42), Cell::Date("2027-03-01".parse().unwrap())],
                vec![Cell::Int(0), Cell::Int(u64::MAX)],
            ]),
            "Name,Days\r\n42,2027-03-01\r\n0,18446744073709551615\r\n"
        );
    }

    #[test]
    fn formula_text_is_neutralized() {
        assert_eq!(neutralize("=1+2"), "'=1+2");
        assert_eq!(neutralize("+SUM(A1)"), "'+SUM(A1)");
        assert_eq!(neutralize("@cmd"), "'@cmd");
        assert_eq!(neutralize("-CAST"), "'-CAST");
        assert_eq!(neutralize("- night shoot"), "'- night shoot");
        assert_eq!(neutralize("\tx"), "'\tx");
        assert_eq!(neutralize("\rx"), "'\rx");
        // Plain numbers and ordinary text stay as they are.
        assert_eq!(neutralize("-5"), "-5");
        assert_eq!(neutralize("+12.50"), "+12.50");
        assert_eq!(neutralize("-.5"), "-.5");
        assert_eq!(neutralize("INT/EXT"), "INT/EXT");
        assert_eq!(neutralize("1st AD"), "1st AD");
        assert_eq!(neutralize(""), "");
        // A neutralized field is still quoted when it needs to be.
        assert_eq!(
            csv(vec![vec![t("=HYPERLINK(\"x\",\"y\")"), Cell::Empty]]),
            "Name,Days\r\n\"'=HYPERLINK(\"\"x\"\",\"\"y\"\")\",\r\n"
        );
        // Headers go through the same path.
        let odd = Sheet {
            name: "Test",
            columns: vec!["=cmd".into()],
            rows: vec![vec![t("@SUM(1)")]],
        };
        assert_eq!(
            String::from_utf8(write(&odd, Delimiter::Comma, false)).unwrap(),
            "'=cmd\r\n'@SUM(1)\r\n"
        );
    }

    #[test]
    fn formulas_are_their_values() {
        assert_eq!(
            csv(vec![vec![
                super::super::Formula::sum(1, 0..2, 7),
                Cell::Int(1)
            ]]),
            "Name,Days\r\n7,1\r\n"
        );
    }

    #[test]
    fn bom_comes_first() {
        let bytes = write(&sheet(vec![]), Delimiter::Comma, true);
        assert_eq!(&bytes[..3], [0xEF, 0xBB, 0xBF]);
        assert_eq!(&bytes[3..], b"Name,Days\r\n");
    }

    #[test]
    fn tab_delimited_never_quotes() {
        assert_eq!(
            tsv(vec![
                vec![t("a, \"b\""), Cell::Int(5)],
                vec![t("tab\there"), Cell::Empty],
                vec![t("line\r\nbreak\nand\rmore"), Cell::Empty],
            ]),
            "Name\tDays\r\n\
             a, \"b\"\t5\r\n\
             tab here\t\r\n\
             line break and more\t\r\n"
        );
    }
}
