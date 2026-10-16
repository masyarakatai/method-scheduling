//! The report list and the settings the schedule keeps for reports and the day out of
//! days (decision 59): each report's name, order and options, the production info, the
//! code letters and a sub-board's prep start and wrap end. Plus the user's own display
//! preferences (week start, date format, what a calendar strip shows), which live in the
//! app's `settings.json`, not in the schedule.
//!
//! Every schedule change is one undo step through `Session::edit`; options travel from
//! the page as plain values and are read strictly here (typed dates through
//! `calendar::typed_date`, names of fields, header layouts and sizes checked by name).

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Runtime, WebviewWindow};

use method_schedule_core::headers::{HeaderAlign, HeaderDesign, HeaderId, HeaderItem};
use method_schedule_core::layouts::{LayoutBody, LayoutId};
use method_schedule_core::model::{Align, CategoryId, ElementId, Schedule, StripField};
use method_schedule_core::setup::{
    PageBreak, ReportColumn, ReportOptions, ReportSetup, TextSize, NAME_LIMIT,
};

use crate::app::{finish, AppState, CmdResult};
use crate::board::EditResult;
use crate::calendar::typed_date;
use crate::docs::Doc;
use crate::view::NamedDto;

/// One report of the list, as the Reports tab shows it.
#[derive(Clone, Debug, Serialize)]
pub struct ReportItemDto {
    pub index: usize,
    pub name: String,
    /// The kind's name (`one_line`, `breakdown`...) and its usual title.
    pub kind: &'static str,
    pub kind_label: &'static str,
    pub landscape: bool,
    pub options: OptionsDto,
}

/// A report's options as the page edits them.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OptionsDto {
    /// Dates as typed; `""` for none.
    pub from: String,
    pub to: String,
    /// `None`: the kind's default.
    pub landscape: Option<bool>,
    pub synopsis: bool,
    pub cast_names: bool,
    /// `none`, `day` or `week`.
    pub page_break: String,
    pub shoot_days: bool,
    pub day_breaks: bool,
    pub total_count: bool,
    /// Category ids: a column each after the scene columns.
    pub element_columns: Vec<u32>,
    /// A named strip layout's id; `None`: the report's own columns.
    pub layout: Option<u32>,
    pub banners: bool,
    pub categories: Vec<u32>,
    pub elements: Vec<u32>,
    pub colors: bool,
    pub icons: bool,
    /// The header layout's id (decision 99).
    pub header: u32,
    pub text: String,
    pub columns: Vec<ColumnDto>,
    pub no_holds: bool,
    pub weekdays_only: bool,
}

/// One column of a report as the page edits it (the report designer, decisions.md, entry
/// 89): a field's name alone, or the field with its line (1 to 4), the first line's
/// columns it covers, its alignment, bold, italic and text size (`""`: the report's).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColumnDto {
    Field(String),
    Cell(CellDto),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CellDto {
    pub field: String,
    pub line: u8,
    pub span: u8,
    pub align: String,
    pub bold: bool,
    pub italic: bool,
    pub size: String,
}

impl Default for CellDto {
    fn default() -> CellDto {
        CellDto {
            field: String::new(),
            line: 1,
            span: 1,
            align: "auto".to_string(),
            bold: false,
            italic: false,
            size: String::new(),
        }
    }
}

impl ColumnDto {
    pub fn of(c: &ReportColumn) -> ColumnDto {
        ColumnDto::Cell(CellDto {
            field: c.field.name().to_string(),
            line: c.line + 1,
            span: c.span,
            align: c.align.name().to_string(),
            bold: c.bold,
            italic: c.italic,
            size: c.size.map_or(String::new(), |t| t.name().to_string()),
        })
    }

    /// The column, read strictly.
    pub fn read(&self) -> EditResult<ReportColumn> {
        let field = |f: &str| {
            StripField::from_name(f)
                .filter(|f| f.of_sheet())
                .ok_or_else(|| format!("{f:?} is not a field."))
        };
        let c = match self {
            ColumnDto::Field(f) => return Ok(ReportColumn::of(field(f)?)),
            ColumnDto::Cell(c) => c,
        };
        if !(1..=4).contains(&c.line) {
            return Err(format!(
                "Line {} is not a line of a report (1 to 4).",
                c.line
            ));
        }
        let align = Align::from_name(&c.align)
            .ok_or_else(|| format!("{:?} is not an alignment.", c.align))?;
        let size = match c.size.as_str() {
            "" => None,
            t => Some(TextSize::from_name(t).ok_or_else(|| format!("{t:?} is not a text size."))?),
        };
        Ok(ReportColumn {
            field: field(&c.field)?,
            line: c.line - 1,
            span: if c.line == 1 { 1 } else { c.span },
            align,
            bold: c.bold,
            italic: c.italic,
            size,
        })
    }
}

impl OptionsDto {
    pub fn of(o: &ReportOptions) -> OptionsDto {
        OptionsDto {
            from: o.from.map(|d| d.to_string()).unwrap_or_default(),
            to: o.to.map(|d| d.to_string()).unwrap_or_default(),
            landscape: o.landscape,
            synopsis: o.synopsis,
            cast_names: o.cast_names,
            page_break: o.page_break.name().to_string(),
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
            header: o.header.0,
            text: o.text.name().to_string(),
            columns: o.columns.iter().map(ColumnDto::of).collect(),
            no_holds: o.no_holds,
            weekdays_only: o.weekdays_only,
        }
    }

    /// The options, read strictly.
    pub fn read(&self) -> EditResult<ReportOptions> {
        let date = |what: &str, v: &str| -> EditResult<Option<method_schedule_core::Date>> {
            let v = v.trim();
            if v.is_empty() {
                return Ok(None);
            }
            typed_date(v).map(Some).ok_or_else(|| {
                format!("The {what} date {v:?} is not a date. Type it as 3/1/2027 or 2027-03-01.")
            })
        };
        let from = date("From", &self.from)?;
        let to = date("To", &self.to)?;
        if let (Some(f), Some(t)) = (from, to) {
            if t < f {
                return Err(format!("The date range ends ({t}) before it starts ({f})."));
            }
        }
        let page_break = PageBreak::from_name(&self.page_break)
            .ok_or_else(|| format!("{:?} is not a page break.", self.page_break))?;
        let text = TextSize::from_name(&self.text)
            .ok_or_else(|| format!("{:?} is not a text size.", self.text))?;
        let columns = self
            .columns
            .iter()
            .map(ColumnDto::read)
            .collect::<Result<Vec<_>, _>>()?;
        if self.categories.len() > 500
            || self.elements.len() > 5000
            || columns.len() > 13
            || self.element_columns.len() > 50
        {
            return Err("Too many picked.".to_string());
        }
        Ok(ReportOptions {
            from,
            to,
            landscape: self.landscape,
            synopsis: self.synopsis,
            cast_names: self.cast_names,
            page_break,
            shoot_days: self.shoot_days,
            day_breaks: self.day_breaks,
            total_count: self.total_count,
            element_columns: self
                .element_columns
                .iter()
                .copied()
                .map(CategoryId)
                .collect(),
            layout: self.layout.map(LayoutId),
            banners: self.banners,
            categories: self.categories.iter().copied().map(CategoryId).collect(),
            elements: self.elements.iter().copied().map(ElementId).collect(),
            colors: self.colors,
            icons: self.icons,
            header: HeaderId(self.header),
            text,
            columns,
            no_holds: self.no_holds,
            weekdays_only: self.weekdays_only,
        })
    }
}

/// What the Reports tab needs to list and edit reports.
#[derive(Clone, Debug, Serialize)]
pub struct ReportsDto {
    pub reports: Vec<ReportItemDto>,
    pub title: String,
    pub company: String,
    pub director: String,
    pub producer: String,
    /// Every production info field in the dialog's order: `[key, label, value]`.
    pub production: Vec<(&'static str, &'static str, String)>,
    /// Every category, and every element by category (`ID NAME`), for the filters.
    pub categories: Vec<NamedDto>,
    pub elements: Vec<crate::view::ElementDto>,
    /// The scene fields a report can show, `[name, label]`, in order.
    pub fields: Vec<(&'static str, &'static str)>,
    /// The header layouts in order (decision 99), and what a header row can hold,
    /// `[name, label]` (custom text aside).
    pub headers: Vec<HeaderDto>,
    pub header_items: Vec<(&'static str, &'static str)>,
    /// The named strip layouts a report can follow.
    pub strip_layouts: Vec<NamedDto>,
}

/// A header layout as the page edits it: id 0 for a new one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeaderDto {
    pub id: u32,
    pub name: String,
    pub include: bool,
    pub first_page_only: bool,
    pub bold: bool,
    pub italic: bool,
    /// `left`, `center` or `right`.
    pub align: String,
    /// `small`, `normal` or `large`.
    pub size: String,
    pub rows: Vec<Vec<HeaderItemDto>>,
}

/// A header item: its name (`report_name`, `director`), or custom text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeaderItemDto {
    Name(String),
    Text(HeaderTextDto),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeaderTextDto {
    pub text: String,
}

impl HeaderDto {
    pub fn of(h: &HeaderDesign) -> HeaderDto {
        HeaderDto {
            id: h.id.0,
            name: h.name.clone(),
            include: h.include,
            first_page_only: h.first_page_only,
            bold: h.bold,
            italic: h.italic,
            align: h.align.name().to_string(),
            size: h.size.name().to_string(),
            rows: h
                .rows
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|i| match i {
                            HeaderItem::Text(t) => {
                                HeaderItemDto::Text(HeaderTextDto { text: t.clone() })
                            }
                            other => HeaderItemDto::Name(other.name().to_string()),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    /// The layout, read strictly.
    pub fn read(&self) -> EditResult<HeaderDesign> {
        let align = HeaderAlign::from_name(&self.align)
            .ok_or_else(|| format!("{:?} is not an alignment.", self.align))?;
        let size = TextSize::from_name(&self.size)
            .ok_or_else(|| format!("{:?} is not a text size.", self.size))?;
        let rows = self
            .rows
            .iter()
            .map(|r| {
                r.iter()
                    .map(|i| match i {
                        HeaderItemDto::Text(t) => Ok(HeaderItem::Text(t.text.trim().to_string())),
                        HeaderItemDto::Name(n) => HeaderItem::from_name(n)
                            .ok_or_else(|| format!("{n:?} is not a header field.")),
                    })
                    .collect::<Result<Vec<_>, String>>()
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(HeaderDesign {
            id: HeaderId(self.id),
            name: self.name.clone(),
            include: self.include,
            first_page_only: self.first_page_only,
            bold: self.bold,
            italic: self.italic,
            align,
            size,
            rows,
        })
    }
}

pub fn reports_dto(s: &Schedule) -> ReportsDto {
    let reports = s
        .report_list()
        .iter()
        .enumerate()
        .map(|(index, r)| ReportItemDto {
            index,
            name: r.name.clone(),
            kind: r.kind.name(),
            kind_label: r.kind.label(),
            landscape: r.landscape(),
            options: OptionsDto::of(&r.options),
        })
        .collect();
    let mut elements = Vec::new();
    for c in &s.categories {
        let mut els: Vec<_> = s.elements.iter().filter(|e| e.category == c.id).collect();
        els.sort_by(|a, b| {
            method_schedule_core::reports::id_order(a.board_id.as_deref(), b.board_id.as_deref())
                .then_with(|| {
                    a.name
                        .to_ascii_uppercase()
                        .cmp(&b.name.to_ascii_uppercase())
                })
        });
        elements.extend(els.into_iter().map(|e| crate::view::ElementDto {
            id: e.id.0,
            name: e.name.clone(),
            board_id: e.board_id.clone().unwrap_or_default(),
            category: c.name.clone(),
            quantity: None,
            daily: None,
        }));
    }
    ReportsDto {
        reports,
        title: s.title.clone(),
        company: s.production.company.clone(),
        director: s.production.director.clone(),
        producer: s.production.producer.clone(),
        production: method_schedule_core::setup::Production::FIELDS
            .iter()
            .map(|f| {
                (
                    *f,
                    method_schedule_core::setup::Production::label(f),
                    s.production.get(f).unwrap_or_default().to_string(),
                )
            })
            .collect(),
        categories: s
            .categories
            .iter()
            .map(|c| NamedDto {
                id: c.id.0,
                name: c.name.clone(),
            })
            .collect(),
        elements,
        fields: StripField::SHEET
            .iter()
            .map(|f| (f.name(), f.label()))
            .collect(),
        headers: s.header_list().iter().map(HeaderDto::of).collect(),
        header_items: HeaderItem::all()
            .iter()
            .map(|i| (i.name(), i.label()))
            .collect(),
        strip_layouts: s
            .layouts
            .layouts
            .iter()
            .filter(|l| matches!(l.body, LayoutBody::Strips(_)))
            .map(|l| NamedDto {
                id: l.id.0,
                name: l.name.clone(),
            })
            .collect(),
    }
}

fn say(e: method_schedule_core::model::Error) -> String {
    use method_schedule_core::model::Error;
    match e {
        Error::DuplicateName(n) => format!("A report named {n:?} is already in the list."),
        Error::InUse(why) => format!("{} can't be deleted: it is the last of its kind.", {
            why.split(" is the only").next().unwrap_or(&why).to_string()
        }),
        Error::Broken(why) => {
            let mut c = why.chars();
            match c.next() {
                Some(f) => format!("{}{}.", f.to_uppercase(), c.as_str()),
                None => "That change was refused.".to_string(),
            }
        }
        other => other.to_string(),
    }
}

/// Saves report `index` with `name` and `options`.
pub fn apply_report(
    s: &mut Schedule,
    index: usize,
    name: &str,
    options: &OptionsDto,
) -> EditResult {
    let list = s.report_list();
    let old = list
        .get(index)
        .ok_or("That report is no longer in the list.")?;
    let name = name.trim();
    if name.is_empty() {
        return Err("A report needs a name.".to_string());
    }
    if name.chars().count() > NAME_LIMIT {
        return Err(format!("A report name is {NAME_LIMIT} characters at most."));
    }
    let setup = ReportSetup {
        name: name.to_string(),
        kind: old.kind,
        options: options.read()?,
    };
    // A header layout picked here must be in the list (one deleted since stays picked
    // only on reports that had it).
    if setup.options.header != old.options.header
        && !s.header_list().iter().any(|h| h.id == setup.options.header)
    {
        return Err("That header layout is no longer in the list.".to_string());
    }
    s.set_report(index, setup).map_err(say)
}

/// Saves header layout `design`: a new one at the end when its id is 0, else in place.
pub fn apply_header(s: &mut Schedule, design: &HeaderDto) -> EditResult<u32> {
    let mut h = design.read()?;
    h.name = h.name.trim().to_string();
    if h.name.is_empty() {
        return Err("A header layout needs a name.".to_string());
    }
    if h.id.0 == 0 {
        return s.add_header(h).map(|id| id.0).map_err(header_say);
    }
    let id = h.id.0;
    s.set_header(h).map_err(header_say)?;
    Ok(id)
}

fn header_say(e: method_schedule_core::model::Error) -> String {
    use method_schedule_core::model::Error;
    match e {
        Error::DuplicateName(n) => format!("A header layout named {n:?} is already in the list."),
        Error::InUse(_) => "The last header layout can't be deleted.".to_string(),
        other => say(other),
    }
}

pub fn apply_production(s: &mut Schedule, field: &str, value: &str) -> EditResult {
    let value: String = value.trim().chars().take(200).collect();
    if field == "title" {
        if value.is_empty() {
            return Err("The schedule needs a title.".to_string());
        }
        s.title = value;
        return Ok(());
    }
    s.production
        .set(field, &value)
        .map_err(|_| format!("{field:?} is not a production field."))
}

pub fn apply_codes(
    s: &mut Schedule,
    codes: &std::collections::BTreeMap<String, String>,
) -> EditResult {
    let mut c = s.dood_codes.clone();
    for (k, v) in codes {
        let slot = c
            .get_mut(k)
            .ok_or_else(|| format!("{k:?} is not a day out of days code."))?;
        *slot = v.trim().to_string();
    }
    if let Some(why) = c.problem() {
        let mut ch = why.chars();
        let first = ch
            .next()
            .map(|f| f.to_uppercase().to_string())
            .unwrap_or_default();
        return Err(format!("{first}{}.", ch.as_str()));
    }
    s.dood_codes = c;
    Ok(())
}

/// Sets a sub-board's prep start and wrap end, as typed (empty clears).
pub fn apply_prep_wrap(s: &mut Schedule, sub: u32, prep: &str, wrap: &str) -> EditResult {
    let read = |what: &str, v: &str| -> EditResult<Option<method_schedule_core::Date>> {
        let v = v.trim();
        if v.is_empty() {
            return Ok(None);
        }
        typed_date(v).map(Some).ok_or_else(|| {
            format!("The {what} {v:?} is not a date. Type it as 3/1/2027 or 2027-03-01.")
        })
    };
    let (p, w) = (read("prep start", prep)?, read("wrap end", wrap)?);
    if let (Some(p), Some(w)) = (p, w) {
        if w < p {
            return Err(format!("Wrap can't end ({w}) before prep starts ({p})."));
        }
    }
    let (b, sb) = {
        let (b, sb) = crate::view::sub_ref(s, sub)?;
        (b.id, sb.id)
    };
    let cal = s.calendar_mut(b, sb).map_err(|e| e.to_string())?;
    cal.prep_start = p;
    cal.wrap_end = w;
    Ok(())
}

// ---------------------------------------------------------------------------
// Commands

fn edit<R: Runtime>(
    window: &WebviewWindow<R>,
    state: &AppState,
    f: impl FnOnce(&mut Schedule) -> EditResult,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(f)?;
    finish(window, &s, None)
}

/// The report list, the production info and what the filters can pick.
#[tauri::command]
pub fn report_list(state: Doc) -> CmdResult<ReportsDto> {
    Ok(reports_dto(state.lock().schedule()))
}

#[tauri::command]
pub fn save_report<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    index: usize,
    name: String,
    options: OptionsDto,
) -> CmdResult {
    edit(&window, &state, |s| apply_report(s, index, &name, &options))
}

/// Adds a copy right after report `index`; the view's notice names it.
#[tauri::command]
pub fn duplicate_report<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    index: usize,
) -> CmdResult {
    let mut s = state.lock();
    let at = s.edit(|x| x.duplicate_report(index).map_err(say))?;
    let name = s
        .schedule()
        .report_list()
        .get(at)
        .map_or(String::new(), |r| r.name.clone());
    finish(&window, &s, Some(format!("Added {name}.")))
}

#[tauri::command]
pub fn move_report<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    index: usize,
    to: usize,
) -> CmdResult {
    edit(&window, &state, |s| s.move_report(index, to).map_err(say))
}

#[tauri::command]
pub fn delete_report<R: Runtime>(window: WebviewWindow<R>, state: Doc, index: usize) -> CmdResult {
    edit(&window, &state, |s| s.delete_report(index).map_err(say))
}

/// Saves a header layout (id 0: a new one); the notice names it.
#[tauri::command]
pub fn save_header_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    design: HeaderDto,
) -> CmdResult {
    let mut s = state.lock();
    let added = design.id == 0;
    s.edit(|x| apply_header(x, &design).map(|_| ()))?;
    let note = added.then(|| format!("Added header layout {}.", design.name.trim()));
    finish(&window, &s, note)
}

#[tauri::command]
pub fn delete_header_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    id: u32,
) -> CmdResult {
    edit(&window, &state, |s| {
        s.delete_header(HeaderId(id)).map_err(header_say)
    })
}

#[tauri::command]
pub fn move_header_layout<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    id: u32,
    to: usize,
) -> CmdResult {
    edit(&window, &state, |s| {
        s.move_header(HeaderId(id), to).map_err(header_say)
    })
}

/// `field`: `title` or a production field's key (`company`, `art_director`...).
#[tauri::command]
pub fn set_production<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    field: String,
    value: String,
) -> CmdResult {
    edit(&window, &state, |s| apply_production(s, &field, &value))
}

/// `codes`: the letters to change, by key (`hold`, `company_move`...).
#[tauri::command]
pub fn set_dood_codes<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    codes: std::collections::BTreeMap<String, String>,
) -> CmdResult {
    edit(&window, &state, |s| apply_codes(s, &codes))
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_prep_wrap<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    sub_board: u32,
    prep_start: String,
    wrap_end: String,
) -> CmdResult {
    edit(&window, &state, |s| {
        apply_prep_wrap(s, sub_board, &prep_start, &wrap_end)
    })
}

/// Whether `old`, the user's per-user calendar fields from before they were kept in the
/// schedule, is a pick of the user's own (not empty, not that preference's default of
/// scene, set and pages), so the Calendar starts from it (decision 102).
pub fn old_calendar_pick(old: &[StripField]) -> bool {
    !old.is_empty()
        && old.iter().enumerate().all(|(i, f)| !old[..i].contains(f))
        && old != [StripField::Scene, StripField::Set, StripField::Pages]
}

/// Sets what a calendar strip shows, by field name.
pub fn apply_calendar_fields(s: &mut Schedule, fields: &[String]) -> EditResult {
    if fields.len() > method_schedule_core::model::MAX_CALENDAR_FIELDS {
        return Err("A calendar strip shows 13 fields at most.".to_string());
    }
    let fields = fields
        .iter()
        .map(|x| {
            StripField::from_name(x)
                .filter(|f| f.of_sheet())
                .ok_or_else(|| format!("{x:?} is not a field."))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if fields.is_empty() {
        return Err("A calendar strip shows at least one field.".to_string());
    }
    s.set_calendar_fields(fields)
        .map_err(|_| "Each field can show once on a calendar strip.".to_string())
}

/// Calendar > Strip Fields: what a strip shows on a date, kept in the schedule.
#[tauri::command]
pub fn set_calendar_fields<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    fields: Vec<String>,
) -> CmdResult {
    edit(&window, &state, |s| apply_calendar_fields(s, &fields))
}

// ---------------------------------------------------------------------------
// The user's preferences (settings.json; not in the schedule)

/// How the user likes dates and the calendar shown.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Prefs {
    /// `sun` or `mon`.
    pub week_start: String,
    /// `iso` (2027-03-01), `us` (3/1/2027) or `long` (Mon 1 Mar 2027).
    pub date_format: String,
    /// What a strip shows on a calendar date: strip layout fields, in order.
    pub calendar_fields: Vec<String>,
    /// Lines on a page when a Fountain script is imported, 40 to 70 (decision 195).
    pub fountain_lines: u32,
    /// The app's look (decision 208): `paper` (the default, light or dark with the
    /// system), `graphite` (always dark) or `ledger` (always light, full strip colors).
    pub theme: String,
}

/// The looks the app has (decision 208), the default first.
pub const THEMES: [&str; 3] = ["paper", "graphite", "ledger"];

impl Default for Prefs {
    fn default() -> Prefs {
        Prefs {
            week_start: "sun".to_string(),
            date_format: "iso".to_string(),
            calendar_fields: vec!["scene".into(), "set".into(), "pages".into()],
            fountain_lines: method_schedule_core::fountain::LINES_PER_PAGE,
            theme: THEMES[0].to_string(),
        }
    }
}

impl Prefs {
    /// The preferences, read strictly.
    pub fn checked(&self) -> Result<Prefs, String> {
        if !["sun", "mon"].contains(&self.week_start.as_str()) {
            return Err(format!(
                "{:?} is not a week start (sun or mon).",
                self.week_start
            ));
        }
        if !["iso", "us", "long"].contains(&self.date_format.as_str()) {
            return Err(format!("{:?} is not a date format.", self.date_format));
        }
        let mut seen = Vec::new();
        for f in &self.calendar_fields {
            if !StripField::from_name(f).is_some_and(|f| f.of_sheet()) {
                return Err(format!("{f:?} is not a field."));
            }
            if seen.contains(f) {
                return Err(format!("{f:?} is shown twice."));
            }
            seen.push(f.clone());
        }
        if seen.is_empty() {
            return Err("A calendar strip shows at least one field.".to_string());
        }
        use method_schedule_core::fountain::{MAX_LINES_PER_PAGE, MIN_LINES_PER_PAGE};
        if !(MIN_LINES_PER_PAGE..=MAX_LINES_PER_PAGE).contains(&self.fountain_lines) {
            return Err(format!(
                "Fountain lines per page are {MIN_LINES_PER_PAGE} to {MAX_LINES_PER_PAGE}."
            ));
        }
        if !THEMES.contains(&self.theme.as_str()) {
            return Err(format!("{:?} is not a look the app has.", self.theme));
        }
        Ok(self.clone())
    }
}

#[tauri::command]
pub fn preferences<R: Runtime>(app: AppHandle<R>) -> Prefs {
    app.state::<crate::safety_app::Safety>().prefs()
}

/// The event every window hears when the preferences change; the payload is the new
/// preferences. Each window redraws with them (they are the user's, not one schedule's).
pub const PREFS_CHANGED: &str = "prefs-changed";

#[tauri::command]
pub fn set_preferences<R: Runtime>(app: AppHandle<R>, prefs: Prefs) -> Result<Prefs, String> {
    let p = prefs.checked()?;
    app.state::<crate::safety_app::Safety>().set_prefs(&p);
    let _ = app.emit(PREFS_CHANGED, &p);
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(s: &Schedule, i: usize) -> OptionsDto {
        OptionsDto::of(&s.report_list()[i].options)
    }

    /// The harness's fake (harness/fake-tauri.js) refuses with these words too: each pair
    /// is a piece of Rust's message that the fake's source holds as written.
    #[test]
    fn the_harness_fake_refuses_in_the_same_words() {
        let fake = include_str!("../harness/fake-tauri.js");
        let mut s = Schedule::new("ZTEST Reports");
        let sub = s.boards[0].sub_boards[0].id.0;
        let code =
            |v: &str| std::collections::BTreeMap::from([("hold".to_string(), v.to_string())]);
        let o = opts(&s, 0);
        let first = s.report_list()[0].name.clone();
        let cases = [
            (
                apply_codes(&mut s, &code("")),
                " code must be 1 to 3 characters.",
            ),
            (
                apply_codes(&mut s, &code("a!")),
                " may hold only letters, digits and /.",
            ),
            (apply_codes(&mut s, &code("S")), " is used twice."),
            (
                apply_prep_wrap(&mut s, sub, "", "soon"),
                " is not a date. Type it as 3/1/2027 or 2027-03-01.",
            ),
            (apply_prep_wrap(&mut s, sub, "soon", ""), "The prep start"),
            (apply_prep_wrap(&mut s, sub, "", "soon"), "The wrap end"),
            (
                apply_report(&mut s, 99, "X", &o),
                "That report is no longer in the list.",
            ),
            (
                apply_report(&mut s, 0, &"x".repeat(201), &o),
                "A report name is 200 characters at most.",
            ),
            (apply_report(&mut s, 1, &first, &o), "A report named "),
            (
                apply_report(&mut s, 1, &first, &o),
                " is already in the list.",
            ),
            (
                s.delete_report(99).map_err(say),
                "That report is not in the list.",
            ),
            (
                s.move_report(99, 0).map_err(say),
                "That report is not in the list.",
            ),
        ];
        for (said, piece) in cases {
            let said = said.unwrap_err();
            assert!(said.contains(piece), "Rust said {said:?}, not {piece:?}");
            assert!(fake.contains(piece), "the fake never says {piece:?}");
        }
    }

    #[test]
    fn options_read_strictly_and_save_as_one_report() {
        let mut s = Schedule::new("ZTEST Reports");
        let mut o = opts(&s, 0);
        o.from = "3/1/2027".into();
        o.to = "2027-03-05".into();
        o.synopsis = false;
        o.columns = vec![
            ColumnDto::Field("scene".into()),
            ColumnDto::Field("set".into()),
        ];
        o.header = 2;
        apply_report(&mut s, 0, "  Week one ", &o).unwrap();
        let r = &s.report_list()[0];
        assert_eq!(r.name, "Week one");
        assert_eq!(r.options.from, "2027-03-01".parse().ok());
        assert_eq!(
            r.options.columns,
            [
                ReportColumn::of(StripField::Scene),
                ReportColumn::of(StripField::Set)
            ]
        );
        assert_eq!(r.options.header, HeaderId::COMPACT);
        // Round trip through the page's shape.
        assert_eq!(opts(&s, 0).read().unwrap(), r.options);

        let mut bad = o.clone();
        bad.from = "3/1/27".into();
        assert!(apply_report(&mut s, 0, "x", &bad)
            .unwrap_err()
            .contains("not a date"));
        bad = o.clone();
        bad.to = "2027-02-01".into();
        assert!(apply_report(&mut s, 0, "x", &bad)
            .unwrap_err()
            .contains("before it starts"));
        bad = o.clone();
        bad.header = 99;
        assert!(apply_report(&mut s, 0, "x", &bad)
            .unwrap_err()
            .contains("no longer in the list"));
        bad = o.clone();
        bad.page_break = "month".into();
        assert!(apply_report(&mut s, 0, "x", &bad).is_err());
        bad = o.clone();
        bad.columns = vec![
            ColumnDto::Field("scene".into()),
            ColumnDto::Field("scene".into()),
        ];
        assert!(apply_report(&mut s, 0, "x", &bad).is_err());
        // The report designer: a merged cell under two columns, bold and large; a later
        // line wider than the first is refused.
        let mut cells = o.clone();
        cells.columns.push(ColumnDto::Cell(CellDto {
            field: "synopsis".into(),
            line: 2,
            span: 2,
            bold: true,
            size: "large".into(),
            ..CellDto::default()
        }));
        apply_report(&mut s, 0, "Week one", &cells).unwrap();
        let syn = s.report_list()[0].options.columns[2];
        assert_eq!((syn.line, syn.span, syn.bold), (1, 2, true));
        assert_eq!(syn.size, Some(TextSize::Large));
        assert_eq!(opts(&s, 0).read().unwrap(), s.report_list()[0].options);
        if let ColumnDto::Cell(c) = &mut cells.columns[2] {
            c.span = 3;
        }
        let err = apply_report(&mut s, 0, "Week one", &cells).unwrap_err();
        assert!(err.contains("covers 3 columns"), "{err}");
        assert!(apply_report(&mut s, 0, " ", &o).is_err());
        let err = apply_report(&mut s, 0, "Shooting schedule", &o).unwrap_err();
        assert!(err.contains("already in the list"), "{err}");
        assert!(apply_report(&mut s, 99, "x", &o).is_err());
        let last = s
            .report_list()
            .iter()
            .position(|r| r.kind == method_schedule_core::setup::ReportKind::Calendar)
            .unwrap();
        let err = s.delete_report(last).map_err(say).unwrap_err();
        assert!(err.contains("last of its kind"), "{err}");
        assert!(!err.contains('!'));
    }

    #[test]
    fn header_layouts_save_read_strictly_and_reports_pick_them() {
        let mut s = Schedule::new("ZTEST Reports");
        let dto = reports_dto(&s);
        assert_eq!(dto.headers.len(), 3);
        assert!(dto.header_items.iter().any(|(k, _)| *k == "set_dresser"));
        let mut h = dto.headers[0].clone();
        h.id = 0;
        h.name = " Wide top ".into();
        h.align = "center".into();
        h.rows = vec![vec![
            HeaderItemDto::Name("title".into()),
            HeaderItemDto::Name("space".into()),
            HeaderItemDto::Text(HeaderTextDto {
                text: " Draft ".into(),
            }),
        ]];
        let id = apply_header(&mut s, &h).unwrap();
        assert_eq!(id, 4);
        let back = &reports_dto(&s).headers[3];
        assert_eq!(back.name, "Wide top");
        assert_eq!(
            back.rows[0][2],
            HeaderItemDto::Text(HeaderTextDto {
                text: "Draft".into()
            })
        );
        assert!(apply_header(&mut s, &h).unwrap_err().contains("already"));
        let mut bad = h.clone();
        bad.name = "Other".into();
        bad.align = "justify".into();
        assert!(apply_header(&mut s, &bad).is_err());
        bad.align = "left".into();
        bad.rows = vec![vec![HeaderItemDto::Name("writer".into())]];
        assert!(apply_header(&mut s, &bad).is_err());
        let mut o = opts(&s, 0);
        o.header = id;
        apply_report(&mut s, 0, "One-line schedule", &o).unwrap();
        assert_eq!(s.report_list()[0].options.header, HeaderId(id));
        s.delete_header(HeaderId(id)).unwrap();
        // The report keeps its pick and prints with the first layout.
        assert_eq!(s.header_of(HeaderId(id)).name, "Full");
    }

    #[test]
    fn calendar_fields_live_in_the_schedule() {
        let mut s = Schedule::new("ZTEST Calendar");
        assert_eq!(
            s.calendar_strip_fields(),
            method_schedule_core::model::DEFAULT_CALENDAR_FIELDS
        );
        apply_calendar_fields(&mut s, &["set".into(), "scene".into()]).unwrap();
        assert_eq!(s.calendar_fields, [StripField::Set, StripField::Scene]);
        assert!(apply_calendar_fields(&mut s, &[]).is_err());
        assert!(apply_calendar_fields(&mut s, &["spacer".into()]).is_err());
        assert!(apply_calendar_fields(&mut s, &["set".into(), "set".into()]).is_err());
        // The older per-user pick counts only when the user changed it.
        assert!(!old_calendar_pick(&[
            StripField::Scene,
            StripField::Set,
            StripField::Pages
        ]));
        assert!(old_calendar_pick(&[StripField::Scene, StripField::Cast]));
        assert!(!old_calendar_pick(&[]));
    }

    #[test]
    fn production_codes_and_prep() {
        let mut s = Schedule::new("ZTEST Reports");
        apply_production(&mut s, "company", " ZTEST Pictures ").unwrap();
        assert_eq!(s.production.company, "ZTEST Pictures");
        apply_production(&mut s, "title", "ZTEST Two").unwrap();
        assert_eq!(s.title, "ZTEST Two");
        assert!(apply_production(&mut s, "title", " ").is_err());
        assert!(apply_production(&mut s, "writer", "x").is_err());
        apply_production(&mut s, "art_director", " A. Director ").unwrap();
        assert_eq!(s.production.art_director, "A. Director");
        let dto = reports_dto(&s);
        assert_eq!(dto.production.len(), 16);
        assert_eq!(
            dto.production[5],
            ("art_director", "Art director", "A. Director".to_string())
        );

        let mut c = std::collections::BTreeMap::new();
        c.insert("hold".to_string(), "W".to_string());
        let err = apply_codes(&mut s, &c).unwrap_err();
        assert!(err.ends_with("used twice."), "{err}");
        c.insert("hold".to_string(), "HD".to_string());
        apply_codes(&mut s, &c).unwrap();
        assert_eq!(s.dood_codes.hold, "HD");
        c.insert("nope".to_string(), "X".to_string());
        assert!(apply_codes(&mut s, &c).is_err());

        let sub = crate::view::first_sub(&s);
        apply_prep_wrap(&mut s, sub, "1/4/2027", "").unwrap();
        assert_eq!(
            s.boards[0].sub_boards[0].calendar.prep_start,
            "2027-01-04".parse().ok()
        );
        assert!(apply_prep_wrap(&mut s, sub, "2027-02-01", "2027-01-01").is_err());
        assert!(apply_prep_wrap(&mut s, sub, "soon", "").is_err());
        assert!(s.check().is_ok());
    }

    #[test]
    fn preferences_are_checked() {
        let p = Prefs::default();
        assert!(p.checked().is_ok());
        let mut q = p.clone();
        q.week_start = "tue".into();
        assert!(q.checked().is_err());
        q = p.clone();
        q.date_format = "eu".into();
        assert!(q.checked().is_err());
        q = p.clone();
        q.calendar_fields = vec![];
        assert!(q.checked().is_err());
        q.calendar_fields = vec!["scene".into(), "scene".into()];
        assert!(q.checked().is_err());
        // Fountain lines per page (decision 195): 56 by default, 40 to 70, refused in
        // the harness fake's words too.
        assert_eq!(p.fountain_lines, 56);
        let fake = include_str!("../harness/fake-tauri.js");
        for (n, ok) in [(40, true), (70, true), (55, true), (39, false), (71, false)] {
            q = p.clone();
            q.fountain_lines = n;
            match q.checked() {
                Ok(_) => assert!(ok, "{n}"),
                Err(e) => {
                    assert!(!ok, "{n}");
                    assert!(fake.contains(&e), "{e}");
                }
            }
        }
    }

    /// Every report setup edit is one undo step, and redo brings it back exactly:
    /// options, copies, order, deletes, header layouts, production info, code letters.
    #[test]
    fn each_report_edit_undoes_and_redoes_as_one_step() {
        use crate::session::Session;
        let mut s = Session::new();
        let mut o = opts(s.schedule(), 0);
        o.synopsis = false;
        type Step = Box<dyn Fn(&mut Schedule) -> EditResult>;
        let steps: Vec<Step> = vec![
            Box::new(move |x| apply_report(x, 0, "Week one", &o)),
            Box::new(|x| x.duplicate_report(0).map(|_| ()).map_err(say)),
            Box::new(|x| x.move_report(1, 3).map_err(say)),
            Box::new(|x| x.delete_report(3).map_err(say)),
            Box::new(|x| {
                let mut h = HeaderDto::of(&x.header_list()[0]);
                h.id = 0;
                h.name = "Mine".into();
                apply_header(x, &h).map(|_| ())
            }),
            Box::new(|x| apply_production(x, "director", "A. Director")),
            Box::new(|x| {
                apply_codes(
                    x,
                    &std::collections::BTreeMap::from([("hold".to_string(), "HD".to_string())]),
                )
            }),
        ];
        for step in &steps {
            let before = s.schedule().clone();
            s.edit(|x| step(x)).unwrap();
            let after = s.schedule().clone();
            assert_ne!(before, after);
            assert!(s.undo());
            assert_eq!(*s.schedule(), before);
            assert!(s.redo());
            assert_eq!(*s.schedule(), after);
        }
    }
}
