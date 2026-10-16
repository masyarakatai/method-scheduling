//! The file shape of `method_schedule_core::setup`: the production info, the report list
//! and the day out of days code letters (docs/FORMAT.md, "Production", "Reports" and
//! "Day out of days codes"). Each is left out at its default, and so is every field of
//! it, so files without them read and write back byte for byte.

use serde::{Deserialize, Serialize};

use method_schedule_core::headers::{HeaderAlign, HeaderDesign, HeaderId, HeaderItem};
use method_schedule_core::layouts::LayoutId;
use method_schedule_core::model::{CategoryId, ElementId, StripField};
use method_schedule_core::setup::{
    DoodCodes, PageBreak, Production, ReportColumn, ReportKind, ReportOptions, ReportSetup,
    TextSize,
};

use crate::{
    date, default_given, in_align, in_size, is_false, is_true, out_align, yes, FormatError,
};

/// The production info: each field left out when empty (decisions.md, entries 59 and 97).
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileProduction {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    company: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    director: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    producer: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    unit_production_manager: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    assistant_director: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    art_director: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    set_dresser: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    script_date: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    production_number: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    preparer: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    timing: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    episode: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    custom_1: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    custom_2: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    custom_3: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    custom_4: String,
}

pub(crate) fn out_production(p: &Production) -> Option<FileProduction> {
    (!p.is_empty()).then(|| FileProduction {
        company: p.company.clone(),
        director: p.director.clone(),
        producer: p.producer.clone(),
        unit_production_manager: p.unit_production_manager.clone(),
        assistant_director: p.assistant_director.clone(),
        art_director: p.art_director.clone(),
        set_dresser: p.set_dresser.clone(),
        script_date: p.script_date.clone(),
        production_number: p.production_number.clone(),
        preparer: p.preparer.clone(),
        timing: p.timing.clone(),
        episode: p.episode.clone(),
        custom_1: p.custom_1.clone(),
        custom_2: p.custom_2.clone(),
        custom_3: p.custom_3.clone(),
        custom_4: p.custom_4.clone(),
    })
}

pub(crate) fn in_production(p: Option<FileProduction>) -> Production {
    p.map_or_else(Production::default, |p| Production {
        company: p.company,
        director: p.director,
        producer: p.producer,
        unit_production_manager: p.unit_production_manager,
        assistant_director: p.assistant_director,
        art_director: p.art_director,
        set_dresser: p.set_dresser,
        script_date: p.script_date,
        production_number: p.production_number,
        preparer: p.preparer,
        timing: p.timing,
        episode: p.episode,
        custom_1: p.custom_1,
        custom_2: p.custom_2,
        custom_3: p.custom_3,
        custom_4: p.custom_4,
    })
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileReport {
    name: String,
    kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    to: Option<String>,
    /// `portrait` or `landscape`; left out at the kind's default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    orientation: Option<String>,
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    synopsis: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    cast_names: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    page_per_day: bool,
    /// `week`; a page per day stays `page_per_day` (decision 100).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    page_break: Option<String>,
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    shoot_days: bool,
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    day_breaks: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    total_count: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    element_columns: Vec<u32>,
    /// A named strip layout's id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    layout: Option<u32>,
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    banners: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    categories: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    elements: Vec<u32>,
    #[serde(default, skip_serializing_if = "is_false")]
    colors: bool,
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    icons: bool,
    /// `compact` or `title` for two of the three layouts the list starts with (as before
    /// decision 99), else a header layout's id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    header: Option<FileHeaderPick>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    columns: Vec<FileReportColumn>,
    #[serde(default, skip_serializing_if = "is_false")]
    no_holds: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    weekdays_only: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum FileHeaderPick {
    Name(String),
    Id(u32),
}

/// A report column: its field's name when it is only that (files from before the report
/// designer read and write back unchanged), else an object (decisions.md, entry 90).
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FileReportColumn {
    Field(String),
    Cell(FileReportCell),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileReportCell {
    field: String,
    /// On the second line; later lines are `line` 3 or 4, as in a strip layout.
    #[serde(default, skip_serializing_if = "is_false")]
    second_line: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    line: Option<u8>,
    /// How many of the first line's columns it covers (a merged cell), left out at 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    span: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    bold: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    italic: bool,
    /// `small`, `normal` or `large`; left out to follow the report's `text`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size: Option<String>,
}

fn out_column(c: &ReportColumn) -> FileReportColumn {
    if c.is_plain() {
        return FileReportColumn::Field(c.field.name().to_string());
    }
    FileReportColumn::Cell(FileReportCell {
        field: c.field.name().to_string(),
        second_line: c.line == 1,
        line: (c.line > 1).then_some(c.line + 1),
        span: (c.span != 1).then_some(c.span),
        align: out_align(c.align),
        bold: c.bold,
        italic: c.italic,
        size: c.size.map(|t| t.name().to_string()),
    })
}

fn in_column(f: FileReportColumn) -> Result<ReportColumn, FormatError> {
    let field = |name: &str| {
        StripField::from_name(name).ok_or_else(|| bad(format!("{name:?} is not a field")))
    };
    let c = match f {
        FileReportColumn::Field(name) => return Ok(ReportColumn::of(field(&name)?)),
        FileReportColumn::Cell(c) => c,
    };
    let line = match (c.second_line, c.line) {
        (false, None) => 0,
        (true, None) => 1,
        (false, Some(n @ 3..=4)) => n - 1,
        (_, Some(n)) => {
            return Err(bad(format!(
                "{n} is not a report line (3 or 4; the second is second_line)"
            )))
        }
    };
    if c.span == Some(1) {
        return Err(default_given("span", "1"));
    }
    let size = match c.size.as_deref() {
        None => None,
        Some(t) => {
            Some(TextSize::from_name(t).ok_or_else(|| bad(format!("{t:?} is not a text size")))?)
        }
    };
    let out = ReportColumn {
        field: field(&c.field)?,
        line,
        span: c.span.unwrap_or(1),
        align: in_align(c.align.as_deref())?,
        bold: c.bold,
        italic: c.italic,
        size,
    };
    if out.is_plain() {
        return Err(bad(format!(
            "report column {:?} sets nothing but its field; write it as the name",
            c.field
        )));
    }
    Ok(out)
}

pub(crate) fn out_reports(list: &[ReportSetup]) -> Vec<FileReport> {
    list.iter()
        .map(|r| {
            let o = &r.options;
            FileReport {
                name: r.name.clone(),
                kind: r.kind.name().to_string(),
                from: o.from.map(|d| d.to_string()),
                to: o.to.map(|d| d.to_string()),
                orientation: o
                    .landscape
                    .filter(|l| *l != r.kind.landscape_by_default())
                    .map(|l| if l { "landscape" } else { "portrait" }.to_string()),
                synopsis: o.synopsis,
                cast_names: o.cast_names,
                page_per_day: o.page_break == PageBreak::Day,
                page_break: (o.page_break == PageBreak::Week).then(|| "week".to_string()),
                shoot_days: o.shoot_days,
                day_breaks: o.day_breaks,
                total_count: o.total_count,
                element_columns: o.element_columns.iter().map(|c| c.0).collect(),
                layout: o.layout.map(|l| l.0),
                banners: o.banners,
                categories: o.categories.iter().map(|c| c.0).collect(),
                elements: o.elements.iter().map(|e| e.0).collect(),
                colors: o.colors,
                icons: o.icons,
                header: (o.header != HeaderId::default()).then(|| match o.header.old_name() {
                    Some(n) => FileHeaderPick::Name(n.to_string()),
                    None => FileHeaderPick::Id(o.header.0),
                }),
                text: (o.text != TextSize::default()).then(|| o.text.name().to_string()),
                columns: o.columns.iter().map(out_column).collect(),
                no_holds: o.no_holds,
                weekdays_only: o.weekdays_only,
            }
        })
        .collect()
}

fn bad(why: String) -> FormatError {
    FormatError::BadValue(why)
}

pub(crate) fn in_reports(list: Vec<FileReport>) -> Result<Vec<ReportSetup>, FormatError> {
    list.into_iter()
        .map(|f| {
            let kind = ReportKind::from_name(&f.kind)
                .ok_or_else(|| bad(format!("{:?} is not a report kind", f.kind)))?;
            let landscape = match f.orientation.as_deref() {
                None => None,
                Some("landscape") => Some(true),
                Some("portrait") => Some(false),
                Some(o) => return Err(bad(format!("{o:?} is not an orientation"))),
            };
            if landscape == Some(kind.landscape_by_default()) {
                let o = f.orientation.as_deref().unwrap_or_default();
                return Err(default_given(
                    &format!("a {} report's orientation", f.kind),
                    o,
                ));
            }
            let header = match f.header {
                None => HeaderId::default(),
                Some(FileHeaderPick::Name(h)) => match HeaderId::from_old_name(&h) {
                    Some(x) if x == HeaderId::default() => return Err(default_given("header", &h)),
                    Some(x) => x,
                    None => return Err(bad(format!("{h:?} is not a header layout"))),
                },
                Some(FileHeaderPick::Id(n)) if n <= 3 => {
                    return Err(bad(format!(
                        "header {n} is one of the first three layouts; write its name"
                    )))
                }
                Some(FileHeaderPick::Id(n)) => HeaderId(n),
            };
            let page_break = match (f.page_per_day, f.page_break.as_deref()) {
                (false, None) => PageBreak::None,
                (true, None) => PageBreak::Day,
                (false, Some("week")) => PageBreak::Week,
                (true, Some(_)) => {
                    return Err(bad(
                        "a report has both page_per_day and page_break".to_string()
                    ))
                }
                (false, Some(p)) => {
                    return Err(bad(format!(
                        "{p:?} is not a page break (week; a day is page_per_day)"
                    )))
                }
            };
            let text = in_size(f.text.as_deref())?;
            let columns = f
                .columns
                .into_iter()
                .map(in_column)
                .collect::<Result<_, _>>()?;
            Ok(ReportSetup {
                name: f.name,
                kind,
                options: ReportOptions {
                    from: f.from.as_deref().map(date).transpose()?,
                    to: f.to.as_deref().map(date).transpose()?,
                    landscape,
                    synopsis: f.synopsis,
                    cast_names: f.cast_names,
                    page_break,
                    shoot_days: f.shoot_days,
                    day_breaks: f.day_breaks,
                    total_count: f.total_count,
                    element_columns: f.element_columns.into_iter().map(CategoryId).collect(),
                    layout: f.layout.map(LayoutId),
                    banners: f.banners,
                    categories: f.categories.into_iter().map(CategoryId).collect(),
                    elements: f.elements.into_iter().map(ElementId).collect(),
                    colors: f.colors,
                    icons: f.icons,
                    header,
                    text,
                    columns,
                    no_holds: f.no_holds,
                    weekdays_only: f.weekdays_only,
                },
            })
        })
        .collect()
}

/// A report header layout (decisions.md, entry 99).
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileHeader {
    id: u32,
    name: String,
    /// The layout prints no header.
    #[serde(default, skip_serializing_if = "is_false")]
    hidden: bool,
    /// Atop every page rather than the first only.
    #[serde(default, skip_serializing_if = "is_false")]
    every_page: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    bold: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    italic: bool,
    /// `center` or `right`; left out for left.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    align: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    rows: Vec<Vec<FileHeaderItem>>,
}

/// A header item: its name, or `{"text": ...}` for custom text.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FileHeaderItem {
    Name(String),
    Text(FileHeaderText),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileHeaderText {
    text: String,
}

pub(crate) fn out_header(h: &HeaderDesign) -> FileHeader {
    FileHeader {
        id: h.id.0,
        name: h.name.clone(),
        hidden: !h.include,
        every_page: !h.first_page_only,
        bold: h.bold,
        italic: h.italic,
        align: (h.align != HeaderAlign::Left).then(|| h.align.name().to_string()),
        size: (h.size != TextSize::Normal).then(|| h.size.name().to_string()),
        rows: h
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|i| match i {
                        HeaderItem::Text(t) => {
                            FileHeaderItem::Text(FileHeaderText { text: t.clone() })
                        }
                        other => FileHeaderItem::Name(other.name().to_string()),
                    })
                    .collect()
            })
            .collect(),
    }
}

pub(crate) fn in_header(f: FileHeader) -> Result<HeaderDesign, FormatError> {
    let align = match f.align.as_deref() {
        None => HeaderAlign::Left,
        Some("left") => return Err(default_given("a header's align", "left")),
        Some(a) => HeaderAlign::from_name(a)
            .ok_or_else(|| bad(format!("{a:?} is not a header alignment")))?,
    };
    let rows = f
        .rows
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|i| match i {
                    FileHeaderItem::Text(t) => Ok(HeaderItem::Text(t.text)),
                    FileHeaderItem::Name(n) => HeaderItem::from_name(&n)
                        .ok_or_else(|| bad(format!("{n:?} is not a header field"))),
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(HeaderDesign {
        id: HeaderId(f.id),
        name: f.name,
        include: !f.hidden,
        first_page_only: !f.every_page,
        bold: f.bold,
        italic: f.italic,
        align,
        size: in_size(f.size.as_deref())?,
        rows,
    })
}

#[derive(Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub(crate) struct FileDoodCodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    start: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    work: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    finish: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    drop: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pickup: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    travel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    fitting: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rehearsal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    holiday: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    company_move: Option<String>,
}

impl FileDoodCodes {
    fn slot(&mut self, key: &str) -> &mut Option<String> {
        match key {
            "start" => &mut self.start,
            "work" => &mut self.work,
            "finish" => &mut self.finish,
            "hold" => &mut self.hold,
            "drop" => &mut self.drop,
            "pickup" => &mut self.pickup,
            "travel" => &mut self.travel,
            "fitting" => &mut self.fitting,
            "rehearsal" => &mut self.rehearsal,
            "holiday" => &mut self.holiday,
            _ => &mut self.company_move,
        }
    }
}

pub(crate) fn out_codes(c: &DoodCodes) -> Option<FileDoodCodes> {
    if c.is_default() {
        return None;
    }
    let d = DoodCodes::default();
    let mut f = FileDoodCodes::default();
    for key in DoodCodes::KEYS {
        let v = c.get(key).unwrap_or_default();
        if Some(v) != d.get(key) {
            *f.slot(key) = Some(v.to_string());
        }
    }
    Some(f)
}

pub(crate) fn in_codes(f: Option<FileDoodCodes>) -> DoodCodes {
    let mut c = DoodCodes::default();
    if let Some(mut f) = f {
        for key in DoodCodes::KEYS {
            if let (Some(v), Some(slot)) = (f.slot(key).take(), c.get_mut(key)) {
                *slot = v;
            }
        }
    }
    c
}
