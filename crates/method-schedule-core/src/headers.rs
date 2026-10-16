//! Report header layouts (decisions.md, entry 99; behavior note 19): named layouts kept in
//! the schedule, each saying whether a report prints a header at all, on the first page
//! only or on every page, its looks (bold, italic, alignment, a text size step) and the
//! fields it places in up to three rows. A report picks one by id.
//!
//! The list starts as three layouts that stand for the fixed headers reports had before
//! (Full, Compact and Title only, ids 1 to 3). It stays out of the file while it is that
//! list, and a report's pick of one of the three is written as the old header name, so
//! files from before read and write back byte for byte.

use std::collections::HashSet;

use crate::model::{Error, Schedule};
use crate::setup::{Production, TextSize};

/// A header layout's id. Ids 1 to 3 are the three the list starts with; new ones count on
/// from the highest in the list, never below 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HeaderId(pub u32);

impl HeaderId {
    /// The title, the production info and the board, sub-board and start date.
    pub const FULL: HeaderId = HeaderId(1);
    /// The title and the facts on one line.
    pub const COMPACT: HeaderId = HeaderId(2);
    /// The title only.
    pub const TITLE: HeaderId = HeaderId(3);

    /// The name a file wrote for one of the first three (`full`, `compact`, `title`).
    pub fn old_name(self) -> Option<&'static str> {
        match self.0 {
            1 => Some("full"),
            2 => Some("compact"),
            3 => Some("title"),
            _ => None,
        }
    }

    pub fn from_old_name(name: &str) -> Option<HeaderId> {
        match name {
            "full" => Some(HeaderId::FULL),
            "compact" => Some(HeaderId::COMPACT),
            "title" => Some(HeaderId::TITLE),
            _ => None,
        }
    }
}

impl Default for HeaderId {
    fn default() -> HeaderId {
        HeaderId::FULL
    }
}

/// One thing a header row shows.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum HeaderItem {
    /// Room that pushes what follows to the far side of the row.
    Space,
    /// Text of its own, the same on every report using the layout.
    Text(String),
    /// The schedule's title, printed larger.
    Title,
    ReportName,
    /// The date the report is printed.
    Today,
    Board,
    SubBoard,
    /// The sub-board's start date.
    StartDate,
    /// A production info field, by its key ([`Production::FIELDS`]).
    Production(&'static str),
}

impl HeaderItem {
    /// Every item a row can take, custom text aside, in the designer's order.
    pub fn all() -> Vec<HeaderItem> {
        let mut out = vec![
            HeaderItem::Space,
            HeaderItem::Title,
            HeaderItem::ReportName,
            HeaderItem::Today,
            HeaderItem::Board,
            HeaderItem::SubBoard,
            HeaderItem::StartDate,
        ];
        out.extend(Production::FIELDS.iter().map(|f| HeaderItem::Production(f)));
        out
    }

    /// Its name in the file and the app (`report_name`, `director`); custom text is `text`.
    pub fn name(&self) -> &'static str {
        match self {
            HeaderItem::Space => "space",
            HeaderItem::Text(_) => "text",
            HeaderItem::Title => "title",
            HeaderItem::ReportName => "report_name",
            HeaderItem::Today => "today",
            HeaderItem::Board => "board",
            HeaderItem::SubBoard => "sub_board",
            HeaderItem::StartDate => "start_date",
            HeaderItem::Production(f) => f,
        }
    }

    /// The item named `name`, custom text aside.
    pub fn from_name(name: &str) -> Option<HeaderItem> {
        HeaderItem::all().into_iter().find(|i| i.name() == name)
    }

    pub fn label(&self) -> &'static str {
        match self {
            HeaderItem::Space => "Space",
            HeaderItem::Text(_) => "Custom text",
            HeaderItem::Title => "Title",
            HeaderItem::ReportName => "Report name",
            HeaderItem::Today => "Today's date",
            HeaderItem::Board => "Board",
            HeaderItem::SubBoard => "Sub-board",
            HeaderItem::StartDate => "Start date",
            HeaderItem::Production(f) => Production::label(f),
        }
    }

    /// For sentences: `report name`, `text "Draft"`.
    pub fn text(&self) -> String {
        match self {
            HeaderItem::Text(t) => format!("text {t:?}"),
            other => other.label().to_lowercase(),
        }
    }
}

/// Where a header's rows sit across the page.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HeaderAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl HeaderAlign {
    pub const ALL: [HeaderAlign; 3] = [HeaderAlign::Left, HeaderAlign::Center, HeaderAlign::Right];

    pub fn name(self) -> &'static str {
        match self {
            HeaderAlign::Left => "left",
            HeaderAlign::Center => "center",
            HeaderAlign::Right => "right",
        }
    }

    pub fn from_name(name: &str) -> Option<HeaderAlign> {
        Self::ALL.into_iter().find(|a| a.name() == name)
    }
}

/// The most rows a header has.
pub const MAX_ROWS: usize = 3;
/// The most items a row holds.
pub const MAX_ITEMS: usize = 12;
/// The most header layouts a schedule keeps.
pub const MAX_HEADERS: usize = 50;
/// The longest custom text.
pub const TEXT_LIMIT: usize = 100;

/// One named header layout.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HeaderDesign {
    pub id: HeaderId,
    pub name: String,
    /// Print a header at all.
    pub include: bool,
    /// On the first page only; otherwise atop every page.
    pub first_page_only: bool,
    pub bold: bool,
    pub italic: bool,
    pub align: HeaderAlign,
    pub size: TextSize,
    /// Up to three rows of items, top to bottom.
    pub rows: Vec<Vec<HeaderItem>>,
}

impl HeaderDesign {
    /// A layout with `rows` and the usual looks: included, first page only, plain, left,
    /// normal size.
    pub fn of(id: HeaderId, name: &str, rows: Vec<Vec<HeaderItem>>) -> HeaderDesign {
        HeaderDesign {
            id,
            name: name.to_string(),
            include: true,
            first_page_only: true,
            bold: false,
            italic: false,
            align: HeaderAlign::Left,
            size: TextSize::Normal,
            rows,
        }
    }

    /// What is wrong with it alone, if anything.
    pub fn problem(&self) -> Option<String> {
        if self.name.trim().is_empty() {
            return Some("a header layout has no name".to_string());
        }
        if self.name.chars().count() > crate::setup::NAME_LIMIT {
            return Some(format!(
                "header layout name {:?} is longer than {} characters",
                self.name,
                crate::setup::NAME_LIMIT
            ));
        }
        if self.id.0 == 0 {
            return Some("a header layout has id 0".to_string());
        }
        if self.rows.len() > MAX_ROWS {
            return Some(format!("a header has {MAX_ROWS} rows at most"));
        }
        if self.rows.iter().any(|r| r.len() > MAX_ITEMS) {
            return Some(format!("a header row holds {MAX_ITEMS} fields at most"));
        }
        for item in self.rows.iter().flatten() {
            if let HeaderItem::Text(t) = item {
                if t.chars().count() > TEXT_LIMIT || t.contains(['\n', '\r']) {
                    return Some(format!(
                        "a header's custom text is one line of {TEXT_LIMIT} characters at most"
                    ));
                }
            }
        }
        None
    }

    /// Its parts for sentences: `Full: first page only; title | company, director`.
    pub fn text(&self) -> String {
        let mut looks: Vec<&str> = Vec::new();
        if !self.include {
            looks.push("no header");
        }
        looks.push(if self.first_page_only {
            "first page only"
        } else {
            "every page"
        });
        if self.bold {
            looks.push("bold");
        }
        if self.italic {
            looks.push("italic");
        }
        if self.align != HeaderAlign::Left {
            looks.push(self.align.name());
        }
        if self.size != TextSize::Normal {
            looks.push(self.size.name());
        }
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(HeaderItem::text)
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .collect();
        format!("{}: {}; {}", self.name, looks.join(", "), rows.join(" | "))
    }
}

/// The list a schedule starts with: the three headers reports had before (decision 59) as
/// layouts.
pub fn default_headers() -> Vec<HeaderDesign> {
    use HeaderItem::*;
    vec![
        HeaderDesign::of(
            HeaderId::FULL,
            "Full",
            vec![
                vec![Title],
                vec![
                    Production("company"),
                    Production("director"),
                    Production("producer"),
                ],
                vec![ReportName, Board, SubBoard, StartDate],
            ],
        ),
        HeaderDesign {
            bold: true,
            ..HeaderDesign::of(
                HeaderId::COMPACT,
                "Compact",
                vec![vec![Title, ReportName, Board, SubBoard]],
            )
        },
        HeaderDesign::of(HeaderId::TITLE, "Title only", vec![vec![Title]]),
    ]
}

/// A header list in a few words: the names in order, `(default)` for none saved.
pub fn headers_text(list: &[HeaderDesign]) -> String {
    if list.is_empty() {
        return "(default)".to_string();
    }
    list.iter()
        .map(|h| h.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

/// The header list's consistency rules (also part of `Schedule::check`): at least one,
/// at most [`MAX_HEADERS`], ids and names unique (names ignoring case), each sound.
pub(crate) fn check_headers(list: &[HeaderDesign]) -> Result<(), Error> {
    if list.len() > MAX_HEADERS {
        return Err(Error::Broken(format!(
            "more than {MAX_HEADERS} header layouts"
        )));
    }
    let mut ids = HashSet::new();
    for (i, h) in list.iter().enumerate() {
        if let Some(why) = h.problem() {
            return Err(Error::Broken(why));
        }
        if !ids.insert(h.id) {
            return Err(Error::Broken(format!(
                "header layout id {} is used twice",
                h.id.0
            )));
        }
        if list[..i]
            .iter()
            .any(|o| crate::model::same_name(&o.name, &h.name))
        {
            return Err(Error::DuplicateName(h.name.clone()));
        }
    }
    Ok(())
}

impl Schedule {
    /// The header layouts in order: the saved list, or the three the list starts with.
    pub fn header_list(&self) -> Vec<HeaderDesign> {
        if self.header_layouts.is_empty() {
            default_headers()
        } else {
            self.header_layouts.clone()
        }
    }

    /// The layout a report picking `id` prints with: that one, or the list's first when
    /// it is gone (a report keeps its pick; deleting a layout never touches reports).
    pub fn header_of(&self, id: HeaderId) -> HeaderDesign {
        let list = self.header_list();
        list.iter()
            .find(|h| h.id == id)
            .cloned()
            .unwrap_or_else(|| list[0].clone())
    }

    /// Saves `list` as the header layouts (left empty, and out of the file, when it is
    /// the default list). Names are trimmed. Refused when it is empty or breaks a rule.
    pub fn set_header_list(&mut self, list: Vec<HeaderDesign>) -> Result<(), Error> {
        let mut list = list;
        if list.is_empty() {
            return Err(Error::InUse(
                "the last header layout can't be deleted".to_string(),
            ));
        }
        for h in &mut list {
            h.name = h.name.trim().to_string();
        }
        check_headers(&list)?;
        self.header_layouts = if list == default_headers() {
            Vec::new()
        } else {
            list
        };
        Ok(())
    }

    /// Adds `design` (its id is ignored) at the end with a fresh id, which it returns.
    /// The id is past every layout's and every id a report still picks, so a report that
    /// kept the pick of a deleted layout (it prints with the first) is never handed the
    /// new one without asking.
    pub fn add_header(&mut self, design: HeaderDesign) -> Result<HeaderId, Error> {
        let mut list = self.header_list();
        let picked = self.report_list().iter().map(|r| r.options.header.0).max();
        let id = HeaderId(
            list.iter()
                .map(|h| h.id.0)
                .chain(picked)
                .max()
                .unwrap_or(0)
                .max(3)
                + 1,
        );
        list.push(HeaderDesign { id, ..design });
        self.set_header_list(list)?;
        Ok(id)
    }

    /// Replaces layout `design.id`.
    pub fn set_header(&mut self, design: HeaderDesign) -> Result<(), Error> {
        let mut list = self.header_list();
        let slot = list
            .iter_mut()
            .find(|h| h.id == design.id)
            .ok_or_else(|| Error::Broken(format!("no header layout {}", design.id.0)))?;
        *slot = design;
        self.set_header_list(list)
    }

    /// Deletes layout `id`; the last one stays. Reports that picked it print with the
    /// list's first.
    pub fn delete_header(&mut self, id: HeaderId) -> Result<(), Error> {
        let mut list = self.header_list();
        let i = list
            .iter()
            .position(|h| h.id == id)
            .ok_or_else(|| Error::Broken(format!("no header layout {}", id.0)))?;
        list.remove(i);
        self.set_header_list(list)
    }

    /// Moves layout `id` to place `to`.
    pub fn move_header(&mut self, id: HeaderId, to: usize) -> Result<(), Error> {
        let mut list = self.header_list();
        let i = list
            .iter()
            .position(|h| h.id == id)
            .ok_or_else(|| Error::Broken(format!("no header layout {}", id.0)))?;
        let h = list.remove(i);
        list.insert(to.min(list.len()), h);
        self.set_header_list(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_starts_as_the_three_old_headers_and_stays_out_at_the_default() {
        let mut s = Schedule::new("ZTEST Headers");
        assert_eq!(s.header_list().len(), 3);
        assert!(s.header_layouts.is_empty());
        assert_eq!(s.header_of(HeaderId::COMPACT).name, "Compact");
        let mut d = HeaderDesign::of(
            HeaderId(0),
            " Call sheet top ",
            vec![vec![
                HeaderItem::Title,
                HeaderItem::Space,
                HeaderItem::Today,
            ]],
        );
        d.align = HeaderAlign::Center;
        let id = s.add_header(d.clone()).unwrap();
        assert_eq!(id, HeaderId(4));
        assert_eq!(s.header_of(id).name, "Call sheet top");
        assert!(s.add_header(d.clone()).is_err(), "a name used twice");
        s.delete_header(id).unwrap();
        assert!(s.header_layouts.is_empty(), "back to the default list");
        // A pick of a deleted layout prints with the first.
        s.delete_header(HeaderId::FULL).unwrap();
        assert_eq!(s.header_of(HeaderId::FULL).id, HeaderId::COMPACT);
        // Ids go on from the highest, never below 4.
        assert_eq!(s.add_header(d).unwrap(), HeaderId(4));
        s.delete_header(HeaderId::COMPACT).unwrap();
        s.delete_header(HeaderId::TITLE).unwrap();
        assert!(matches!(s.delete_header(HeaderId(4)), Err(Error::InUse(_))));
        assert!(s.check().is_ok());

        // A report keeps the pick of a deleted layout and prints with the first; a new
        // layout takes a fresh id, never the deleted one, so the report doesn't change.
        let mut s = Schedule::new("ZTEST Headers");
        let top = HeaderDesign::of(HeaderId(0), "Top", vec![vec![HeaderItem::Title]]);
        let id = s.add_header(top.clone()).unwrap();
        let mut list = s.report_list();
        list[0].options.header = id;
        s.set_report_list(list).unwrap();
        s.delete_header(id).unwrap();
        assert_eq!(s.report_list()[0].options.header, id);
        assert_eq!(s.header_of(id).id, HeaderId::FULL);
        let next = s.add_header(top).unwrap();
        assert_ne!(next, id);
        assert_eq!(
            s.header_of(s.report_list()[0].options.header).id,
            HeaderId::FULL
        );
        assert!(s.check().is_ok());
    }

    #[test]
    fn rows_items_and_text_are_bounded() {
        let mut s = Schedule::new("ZTEST Headers");
        let mut d = HeaderDesign::of(HeaderId(0), "Big", vec![vec![]; 4]);
        assert!(s.add_header(d.clone()).is_err());
        d.rows = vec![vec![HeaderItem::Space; MAX_ITEMS + 1]];
        assert!(s.add_header(d.clone()).is_err());
        d.rows = vec![vec![HeaderItem::Text("two\nlines".into())]];
        assert!(s.add_header(d.clone()).is_err());
        d.rows = vec![vec![HeaderItem::Text("x".repeat(TEXT_LIMIT))]];
        assert!(s.add_header(d.clone()).is_ok());
        assert_eq!(
            HeaderItem::from_name("art_director"),
            Some(HeaderItem::Production("art_director"))
        );
        assert_eq!(HeaderItem::from_name("text"), None);
        assert_eq!(
            default_headers()[1].text(),
            "Compact: first page only, bold; title, report name, board, sub-board"
        );
    }

    #[test]
    fn diff_and_merge_see_header_layouts() {
        let a = Schedule::new("ZTEST Headers");
        let mut b = a.clone();
        b.add_header(HeaderDesign::of(
            HeaderId(0),
            "Top",
            vec![vec![HeaderItem::Title]],
        ))
        .unwrap();
        let mut c = a.clone();
        let mut full = c.header_of(HeaderId::FULL);
        full.bold = true;
        c.set_header(full).unwrap();
        let d = crate::diff::diff(&a, &b);
        assert!(
            d.changes
                .iter()
                .any(|x| x.text == "Header layout \"Top\" added"),
            "{:?}",
            d.changes
        );
        let d = crate::diff::diff(&a, &c);
        assert!(d.changes[0].text.contains("first page only, bold"));
        let m = crate::merge::merge(&a, &b, &a);
        assert_eq!(m.schedule.header_list().len(), 4);
        // Two different changes conflict; ours by default.
        let m = crate::merge::merge(&a, &b, &c);
        assert_eq!(m.conflicts.len(), 1);
        assert_eq!(m.schedule.header_list().len(), 4);
    }
}
