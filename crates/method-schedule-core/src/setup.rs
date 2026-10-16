//! What a schedule keeps about its reports and its day out of days, apart from the
//! schedule itself: the production info printed in report headers, the list of reports
//! with each one's options (their order, named copies, the simple designer's columns), and
//! the day out of days code letters.
//!
//! All of it is saved in the schedule file (docs/FORMAT.md) and left out while it is at
//! its default, so files that never touched it read and write back byte for byte.
//! Report layouts stay our own (decisions.md, entry 23); this only says what a report
//! shows, never how another product lays it out.

use crate::model::{
    Align, CategoryId, ElementId, Error, Schedule, StripColumn, StripField, StripLayout, StripWidth,
};
use crate::Date;

/// The production info a report header can show, beside the schedule's title: the
/// reference app's list of fields (behavior note 19), each plain text. The title itself
/// is the schedule's.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Production {
    pub company: String,
    pub director: String,
    pub producer: String,
    pub unit_production_manager: String,
    pub assistant_director: String,
    pub art_director: String,
    pub set_dresser: String,
    pub script_date: String,
    pub production_number: String,
    pub preparer: String,
    pub timing: String,
    pub episode: String,
    pub custom_1: String,
    pub custom_2: String,
    pub custom_3: String,
    pub custom_4: String,
}

impl Production {
    pub fn is_empty(&self) -> bool {
        Self::FIELDS
            .iter()
            .all(|f| self.get(f).is_none_or(str::is_empty))
    }

    /// The field names, as the file and the app name them, in the dialog's order.
    pub const FIELDS: [&'static str; 16] = [
        "company",
        "director",
        "producer",
        "unit_production_manager",
        "assistant_director",
        "art_director",
        "set_dresser",
        "script_date",
        "production_number",
        "preparer",
        "timing",
        "episode",
        "custom_1",
        "custom_2",
        "custom_3",
        "custom_4",
    ];

    /// The longest value a field keeps.
    pub const LIMIT: usize = 200;

    /// A field's name as a person reads it (`Unit production manager`).
    pub fn label(field: &str) -> &'static str {
        match field {
            "company" => "Company",
            "director" => "Director",
            "producer" => "Producer",
            "unit_production_manager" => "Unit production manager",
            "assistant_director" => "Assistant director",
            "art_director" => "Art director",
            "set_dresser" => "Set dresser",
            "script_date" => "Script date",
            "production_number" => "Production number",
            "preparer" => "Prepared by",
            "timing" => "Timing",
            "episode" => "Episode",
            "custom_1" => "Custom 1",
            "custom_2" => "Custom 2",
            "custom_3" => "Custom 3",
            "custom_4" => "Custom 4",
            _ => "",
        }
    }

    pub fn get(&self, field: &str) -> Option<&str> {
        Some(match field {
            "company" => &self.company,
            "director" => &self.director,
            "producer" => &self.producer,
            "unit_production_manager" => &self.unit_production_manager,
            "assistant_director" => &self.assistant_director,
            "art_director" => &self.art_director,
            "set_dresser" => &self.set_dresser,
            "script_date" => &self.script_date,
            "production_number" => &self.production_number,
            "preparer" => &self.preparer,
            "timing" => &self.timing,
            "episode" => &self.episode,
            "custom_1" => &self.custom_1,
            "custom_2" => &self.custom_2,
            "custom_3" => &self.custom_3,
            "custom_4" => &self.custom_4,
            _ => return None,
        })
    }

    pub fn get_mut(&mut self, field: &str) -> Option<&mut String> {
        Some(match field {
            "company" => &mut self.company,
            "director" => &mut self.director,
            "producer" => &mut self.producer,
            "unit_production_manager" => &mut self.unit_production_manager,
            "assistant_director" => &mut self.assistant_director,
            "art_director" => &mut self.art_director,
            "set_dresser" => &mut self.set_dresser,
            "script_date" => &mut self.script_date,
            "production_number" => &mut self.production_number,
            "preparer" => &mut self.preparer,
            "timing" => &mut self.timing,
            "episode" => &mut self.episode,
            "custom_1" => &mut self.custom_1,
            "custom_2" => &mut self.custom_2,
            "custom_3" => &mut self.custom_3,
            "custom_4" => &mut self.custom_4,
            _ => return None,
        })
    }

    /// Sets `field` to `value`, trimmed and cut at [`Production::LIMIT`] characters.
    pub fn set(&mut self, field: &str, value: &str) -> Result<(), Error> {
        let slot = self
            .get_mut(field)
            .ok_or_else(|| Error::Broken(format!("{field:?} is not a production field")))?;
        *slot = value.trim().chars().take(Self::LIMIT).collect();
        Ok(())
    }
}

/// The kinds of report. Each kind is in every schedule's report list at least once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReportKind {
    OneLine,
    Shooting,
    /// The breakdown report grouped by strip: a block per scene with its elements.
    Breakdown,
    /// The breakdown report grouped by shoot day: one element block per category per day.
    BreakdownDay,
    Dood,
    Elements,
    ElementDetail,
    Totals,
    Calendar,
    /// The one-line with a cast ID grid: a fixed row per cast member, a column per scene.
    Vertical,
    /// Every sub-board of the board merged by date, each unit headed by its name.
    MultiUnit,
    Conflicts,
    RedFlags,
}

impl ReportKind {
    /// Every kind, in the default list order.
    pub const ALL: [ReportKind; 13] = [
        ReportKind::OneLine,
        ReportKind::Shooting,
        ReportKind::Breakdown,
        ReportKind::BreakdownDay,
        ReportKind::Dood,
        ReportKind::Elements,
        ReportKind::ElementDetail,
        ReportKind::Totals,
        ReportKind::Calendar,
        ReportKind::Vertical,
        ReportKind::MultiUnit,
        ReportKind::Conflicts,
        ReportKind::RedFlags,
    ];

    /// The name the file and the app use.
    pub fn name(self) -> &'static str {
        match self {
            ReportKind::OneLine => "one_line",
            ReportKind::Shooting => "shooting",
            ReportKind::Breakdown => "breakdown",
            ReportKind::BreakdownDay => "breakdown_day",
            ReportKind::Dood => "dood",
            ReportKind::Elements => "elements",
            ReportKind::ElementDetail => "element_detail",
            ReportKind::Totals => "totals",
            ReportKind::Calendar => "calendar",
            ReportKind::Vertical => "vertical",
            ReportKind::MultiUnit => "multi_unit",
            ReportKind::Conflicts => "conflicts",
            ReportKind::RedFlags => "red_flags",
        }
    }

    pub fn from_name(name: &str) -> Option<ReportKind> {
        Self::ALL.into_iter().find(|k| k.name() == name)
    }

    /// The default title.
    pub fn label(self) -> &'static str {
        match self {
            ReportKind::OneLine => "One-line schedule",
            ReportKind::Shooting => "Shooting schedule",
            ReportKind::Breakdown => "Breakdown by strip",
            ReportKind::BreakdownDay => "Breakdown by day",
            ReportKind::Dood => "Day out of days",
            ReportKind::Elements => "Elements",
            ReportKind::ElementDetail => "Element detail",
            ReportKind::Totals => "Daily category totals",
            ReportKind::Calendar => "Calendar",
            ReportKind::Vertical => "Vertical one-line",
            ReportKind::MultiUnit => "Multi-unit one-line",
            ReportKind::Conflicts => "Conflicts",
            ReportKind::RedFlags => "Red flags",
        }
    }

    /// Landscape unless the options say otherwise.
    pub fn landscape_by_default(self) -> bool {
        matches!(
            self,
            ReportKind::Dood | ReportKind::Calendar | ReportKind::Vertical
        )
    }

    /// The scene fields a report of this kind shows when its options name none.
    pub fn default_columns(self) -> &'static [StripField] {
        use StripField::*;
        match self {
            // The breakdown by day had none, so its scene rows printed empty (RPT-06,
            // decision 161).
            ReportKind::Breakdown | ReportKind::BreakdownDay | ReportKind::Shooting => {
                &[Scene, IntExt, Set, TimeOfDay, Pages]
            }
            ReportKind::ElementDetail => &[Scene, IntExt, Set, TimeOfDay, Pages],
            // The one-line and multi-unit follow the board's strip layout.
            _ => &[],
        }
    }
}

/// Where a report starts a new page (decision 100; behavior note 19).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PageBreak {
    #[default]
    None,
    /// Each shoot day on a new page (the old `page_per_day`).
    Day,
    /// Each week on a new page: a day whose week differs from the day before's.
    Week,
}

impl PageBreak {
    pub const ALL: [PageBreak; 3] = [PageBreak::None, PageBreak::Day, PageBreak::Week];

    pub fn name(self) -> &'static str {
        match self {
            PageBreak::None => "none",
            PageBreak::Day => "day",
            PageBreak::Week => "week",
        }
    }

    pub fn from_name(name: &str) -> Option<PageBreak> {
        Self::ALL.into_iter().find(|p| p.name() == name)
    }
}

/// The most element category columns a report adds.
pub const MAX_ELEMENT_COLUMNS: usize = 10;

/// The report's type size; also a strip layout's and a banner's (decision 87).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextSize {
    Small,
    #[default]
    Normal,
    Large,
}

impl TextSize {
    pub const ALL: [TextSize; 3] = [TextSize::Small, TextSize::Normal, TextSize::Large];

    pub fn name(self) -> &'static str {
        match self {
            TextSize::Small => "small",
            TextSize::Normal => "normal",
            TextSize::Large => "large",
        }
    }

    pub fn from_name(name: &str) -> Option<TextSize> {
        Self::ALL.into_iter().find(|t| t.name() == name)
    }
}

/// One column of a report's own layout (the report designer, decisions.md, entry 90): a
/// scene field and where and how it prints, as a strip layout column has it (entry 87),
/// plus a text size of its own. A field on a later line sits under the first line's
/// columns and covers `span` of them: a merged cell. Its width follows the board's strip
/// layout, as the report's columns always did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReportColumn {
    pub field: StripField,
    /// 0 for the first line, up to 3.
    pub line: u8,
    /// How many of the first line's columns it covers: 1 on the first line.
    pub span: u8,
    pub align: Align,
    pub bold: bool,
    pub italic: bool,
    /// `None`: the report's text size.
    pub size: Option<TextSize>,
}

impl ReportColumn {
    /// `field` on the first line, plain, at the report's text size.
    pub fn of(field: StripField) -> ReportColumn {
        ReportColumn {
            field,
            line: 0,
            span: 1,
            align: Align::Auto,
            bold: false,
            italic: false,
            size: None,
        }
    }

    /// Whether it is only its field: on the first line, nothing else set.
    pub fn is_plain(&self) -> bool {
        *self == ReportColumn::of(self.field)
    }

    /// `set` or `synopsis (line 2, 3 columns, centered, bold, large)`: its field's name and
    /// what isn't plain, in words.
    pub fn text(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if self.line > 0 {
            parts.push(format!("line {}", self.line + 1));
        }
        if self.span > 1 {
            parts.push(format!("{} columns", self.span));
        }
        if self.align != Align::Auto {
            parts.push(self.align.name().to_string());
        }
        if self.bold {
            parts.push("bold".to_string());
        }
        if self.italic {
            parts.push("italic".to_string());
        }
        if let Some(t) = self.size {
            parts.push(t.name().to_string());
        }
        if parts.is_empty() {
            self.field.name().to_string()
        } else {
            format!("{} ({})", self.field.name(), parts.join(", "))
        }
    }

    /// The strip layout column it prints as, at `width`.
    pub fn strip_column(&self, width: StripWidth) -> StripColumn {
        StripColumn {
            width,
            line: self.line,
            span: self.span,
            align: self.align,
            bold: self.bold,
            italic: self.italic,
            ..StripColumn::of(self.field)
        }
    }
}

/// What one report shows. Each kind uses the options that make sense for it and ignores
/// the rest. The defaults are what the reports always showed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportOptions {
    /// Only shoot days (or dates) from this date on.
    pub from: Option<Date>,
    /// Only shoot days (or dates) up to this date, inclusive.
    pub to: Option<Date>,
    /// `None`: the kind's default.
    pub landscape: Option<bool>,
    /// Show each scene's synopsis.
    pub synopsis: bool,
    /// Cast by name rather than by board ID.
    pub cast_names: bool,
    /// Where a new page starts: never, each shoot day, or each week.
    pub page_break: PageBreak,
    /// The line heading each shoot day (the report's shoot day strips).
    pub shoot_days: bool,
    /// The line ending each shoot day with its totals (the report's day breaks).
    pub day_breaks: bool,
    /// A last line with the report's scene count and pages.
    pub total_count: bool,
    /// A column per category, after the scene columns, listing each scene's elements of
    /// it (behavior note 19's element category picker).
    pub element_columns: Vec<CategoryId>,
    /// The named strip layout the scene columns follow (none: the report's own columns,
    /// else the kind's usual ones).
    pub layout: Option<crate::layouts::LayoutId>,
    /// Show banners.
    pub banners: bool,
    /// Only these categories (empty: all), in the schedule's category order.
    pub categories: Vec<CategoryId>,
    /// Only these elements (empty: all): the day out of days rows, the element detail,
    /// and the multi-unit one-line's days with a scene holding one of them.
    pub elements: Vec<ElementId>,
    /// Strip colors (interior day, exterior night and so on) on scene lines.
    pub colors: bool,
    /// Red flag marks on scene lines.
    pub icons: bool,
    /// The header layout it prints with (decision 99).
    pub header: crate::headers::HeaderId,
    pub text: TextSize,
    /// The scene fields shown, in order, with their lines, merged cells and looks (the
    /// report designer). Empty: the kind's default, or the board's strip layout for the
    /// one-line reports.
    pub columns: Vec<ReportColumn>,
    /// The day out of days as if no element had holds (and so no drops).
    pub no_holds: bool,
    /// The calendar with Monday to Friday only.
    pub weekdays_only: bool,
}

impl Default for ReportOptions {
    fn default() -> ReportOptions {
        ReportOptions {
            from: None,
            to: None,
            landscape: None,
            synopsis: true,
            cast_names: false,
            page_break: PageBreak::None,
            shoot_days: true,
            day_breaks: true,
            total_count: false,
            element_columns: Vec::new(),
            layout: None,
            banners: true,
            categories: Vec::new(),
            elements: Vec::new(),
            colors: false,
            icons: true,
            header: crate::headers::HeaderId::FULL,
            text: TextSize::Normal,
            columns: Vec::new(),
            no_holds: false,
            weekdays_only: false,
        }
    }
}

impl ReportOptions {
    /// Whether `d` is inside the date range (a day with no date always is).
    pub fn in_range(&self, d: Option<Date>) -> bool {
        match d {
            None => true,
            Some(d) => self.from.is_none_or(|f| d >= f) && self.to.is_none_or(|t| d <= t),
        }
    }

    /// Whether the category filter lets `c` through.
    pub fn shows_category(&self, c: CategoryId) -> bool {
        self.categories.is_empty() || self.categories.contains(&c)
    }

    /// What is wrong with these options, if anything.
    pub fn problem(&self) -> Option<String> {
        if let (Some(f), Some(t)) = (self.from, self.to) {
            if t < f {
                return Some(format!("the date range ends ({t}) before it starts ({f})"));
            }
        }
        let twice = |v: &[u32]| {
            let mut seen = std::collections::HashSet::new();
            v.iter().any(|x| !seen.insert(*x))
        };
        if twice(&self.categories.iter().map(|c| c.0).collect::<Vec<_>>()) {
            return Some("a category is picked twice".to_string());
        }
        if twice(&self.elements.iter().map(|c| c.0).collect::<Vec<_>>()) {
            return Some("an element is picked twice".to_string());
        }
        if twice(&self.element_columns.iter().map(|c| c.0).collect::<Vec<_>>()) {
            return Some("a category column is picked twice".to_string());
        }
        if self.element_columns.len() > MAX_ELEMENT_COLUMNS {
            return Some(format!(
                "a report has {MAX_ELEMENT_COLUMNS} category columns at most"
            ));
        }
        if let Some(c) = self.columns.iter().find(|c| !c.field.of_sheet()) {
            return Some(format!(
                "{} is for strip layouts, not report columns",
                c.field.label()
            ));
        }
        let cols: Vec<u32> = self
            .columns
            .iter()
            .map(|c| {
                StripField::ALL
                    .iter()
                    .position(|x| *x == c.field)
                    .unwrap_or(0) as u32
            })
            .collect();
        if twice(&cols) {
            return Some("a column is shown twice".to_string());
        }
        if !self.columns.is_empty() {
            // The same lines and merged cells a strip layout may have.
            let lay = StripLayout::of(
                self.columns
                    .iter()
                    .map(|c| c.strip_column(c.field.default_width()))
                    .collect(),
            );
            if let Some(why) = lay.problem() {
                return Some(format!("the report's columns: {why}"));
            }
        }
        None
    }
}

/// One entry of the report list: a named report of a kind with its options.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportSetup {
    pub name: String,
    pub kind: ReportKind,
    pub options: ReportOptions,
}

impl ReportSetup {
    pub fn of(kind: ReportKind) -> ReportSetup {
        ReportSetup {
            name: kind.label().to_string(),
            kind,
            options: ReportOptions::default(),
        }
    }

    pub fn landscape(&self) -> bool {
        self.options
            .landscape
            .unwrap_or_else(|| self.kind.landscape_by_default())
    }
}

/// The report list a schedule starts with: every kind once, with its default options.
pub fn default_reports() -> Vec<ReportSetup> {
    ReportKind::ALL.into_iter().map(ReportSetup::of).collect()
}

/// The longest report name.
pub const NAME_LIMIT: usize = 200;

/// The day out of days code letters (inventory section 10: MMS 5 let each be changed).
/// The defaults are the reference product's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DoodCodes {
    pub start: String,
    pub work: String,
    pub finish: String,
    pub hold: String,
    pub drop: String,
    pub pickup: String,
    pub travel: String,
    pub fitting: String,
    pub rehearsal: String,
    pub holiday: String,
    /// A company travel day.
    pub company_move: String,
}

impl Default for DoodCodes {
    fn default() -> DoodCodes {
        DoodCodes {
            start: "S".into(),
            work: "W".into(),
            finish: "F".into(),
            hold: "H".into(),
            drop: "D".into(),
            pickup: "P".into(),
            travel: "TR".into(),
            fitting: "FT".into(),
            rehearsal: "R".into(),
            holiday: "/".into(),
            company_move: "M".into(),
        }
    }
}

impl DoodCodes {
    /// The keys, as the file and the app name them, in display order.
    pub const KEYS: [&'static str; 11] = [
        "start",
        "work",
        "finish",
        "hold",
        "drop",
        "pickup",
        "travel",
        "fitting",
        "rehearsal",
        "holiday",
        "company_move",
    ];

    pub fn get(&self, key: &str) -> Option<&str> {
        Some(match key {
            "start" => &self.start,
            "work" => &self.work,
            "finish" => &self.finish,
            "hold" => &self.hold,
            "drop" => &self.drop,
            "pickup" => &self.pickup,
            "travel" => &self.travel,
            "fitting" => &self.fitting,
            "rehearsal" => &self.rehearsal,
            "holiday" => &self.holiday,
            "company_move" => &self.company_move,
            _ => return None,
        })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut String> {
        Some(match key {
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
            "company_move" => &mut self.company_move,
            _ => return None,
        })
    }

    pub fn is_default(&self) -> bool {
        *self == DoodCodes::default()
    }

    /// An event's code (`TR`, `FT`, `R`, as `EventKind::code` gives it) in these letters.
    pub fn event(&self, code: &str) -> String {
        match code {
            "TR" => self.travel.clone(),
            "FT" => self.fitting.clone(),
            "R" => self.rehearsal.clone(),
            other => other.to_string(),
        }
    }

    /// What is wrong with these letters: each is 1 to 3 letters, digits or `/`, and no
    /// two are the same (ignoring case), so every code reads one way.
    pub fn problem(&self) -> Option<String> {
        let mut seen: Vec<String> = Vec::new();
        for key in Self::KEYS {
            let v = self.get(key).unwrap_or_default();
            let n = v.chars().count();
            if n == 0 || n > 3 {
                return Some(format!(
                    "the {} code must be 1 to 3 characters",
                    key_label(key)
                ));
            }
            if !v.chars().all(|c| c.is_alphanumeric() || c == '/') {
                return Some(format!(
                    "the {} code {v:?} may hold only letters, digits and /",
                    key_label(key)
                ));
            }
            let up = v.to_uppercase();
            if seen.contains(&up) {
                return Some(format!("the code {v:?} is used twice"));
            }
            seen.push(up);
        }
        None
    }
}

impl DoodCodes {
    /// Every letter in order, `S W F H D P TR FT R / M`.
    pub fn text(&self) -> String {
        Self::KEYS
            .iter()
            .map(|k| self.get(k).unwrap_or_default())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// A saved report list in a few words: the names in order, `(default)` for none saved.
pub fn reports_text(list: &[ReportSetup]) -> String {
    if list.is_empty() {
        return "(default)".to_string();
    }
    list.iter()
        .map(|r| r.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

/// What changed between two report lists, a phrase each: reports added, removed or
/// renamed (matched by place among their kind), options changed, and the order.
pub fn report_changes(a: &[ReportSetup], b: &[ReportSetup]) -> Vec<String> {
    let mut out = Vec::new();
    for k in ReportKind::ALL {
        let xa: Vec<&ReportSetup> = a.iter().filter(|r| r.kind == k).collect();
        let xb: Vec<&ReportSetup> = b.iter().filter(|r| r.kind == k).collect();
        // Same name first, then by place among the kind.
        let mut left: Vec<&ReportSetup> = Vec::new();
        let mut right: Vec<&ReportSetup> = xb.clone();
        for r in &xa {
            if let Some(i) = right.iter().position(|o| o.name == r.name) {
                let o = right.remove(i);
                if o.options != r.options {
                    out.push(format!("report {:?} options changed", r.name));
                }
            } else {
                left.push(r);
            }
        }
        for (i, r) in left.iter().enumerate() {
            match right.get(i) {
                Some(o) => {
                    out.push(format!("report {:?} renamed {:?}", r.name, o.name));
                    if o.options != r.options {
                        out.push(format!("report {:?} options changed", o.name));
                    }
                }
                None => out.push(format!("report {:?} removed", r.name)),
            }
        }
        for o in right.iter().skip(left.len()) {
            out.push(format!("report {:?} added", o.name));
        }
    }
    let kinds = |l: &[ReportSetup]| l.iter().map(|r| r.kind).collect::<Vec<_>>();
    if out.is_empty() && kinds(a) != kinds(b) {
        out.push("report order changed".to_string());
    }
    out
}

/// `company_move` as a person reads it.
pub fn key_label(key: &str) -> &str {
    match key {
        "company_move" => "company travel",
        k => k,
    }
}

// ---------------------------------------------------------------------------
// The schedule's report list

impl Schedule {
    /// The report list as the Reports tab shows it: the saved list, or the default one,
    /// with any kind it lacks added at the end with its defaults.
    pub fn report_list(&self) -> Vec<ReportSetup> {
        if self.reports.is_empty() {
            return default_reports();
        }
        let mut out = self.reports.clone();
        for k in ReportKind::ALL {
            if !out.iter().any(|r| r.kind == k) {
                out.push(ReportSetup::of(k));
            }
        }
        out
    }

    /// Saves `list` as the report list (left empty, and out of the file, when it is the
    /// default). Refused when a name is empty, too long or used twice, or an option
    /// doesn't hold together.
    pub fn set_report_list(&mut self, list: Vec<ReportSetup>) -> Result<(), Error> {
        let mut list = list;
        for r in &mut list {
            r.name = r.name.trim().to_string();
            if r.options.landscape == Some(r.kind.landscape_by_default()) {
                r.options.landscape = None;
            }
        }
        check_reports(self, &list)?;
        self.reports = if list == default_reports() {
            Vec::new()
        } else {
            list
        };
        Ok(())
    }

    /// Replaces report `index` of the list.
    pub fn set_report(&mut self, index: usize, setup: ReportSetup) -> Result<(), Error> {
        let mut list = self.report_list();
        let slot = list.get_mut(index).ok_or_else(no_report)?;
        if slot.kind != setup.kind {
            return Err(Error::Broken("a report can't change its kind".to_string()));
        }
        *slot = setup;
        self.set_report_list(list)
    }

    /// Adds a copy of report `index` right after it, named `<name> copy` (or `copy 2`
    /// and on, to keep names unique). Returns its index.
    pub fn duplicate_report(&mut self, index: usize) -> Result<usize, Error> {
        let mut list = self.report_list();
        let mut copy = list.get(index).ok_or_else(no_report)?.clone();
        // The name is cut before the number, so a long name still ends in its number.
        let fit = |tail: &str| crate::model::fit_name(&copy.name, tail, NAME_LIMIT);
        let mut name = fit(" copy");
        let mut n = 2;
        while list.iter().any(|r| crate::model::same_name(&r.name, &name)) {
            name = fit(&format!(" copy {n}"));
            n += 1;
        }
        copy.name = name;
        list.insert(index + 1, copy);
        self.set_report_list(list)?;
        Ok(index + 1)
    }

    /// Moves report `index` to place `to` in the list.
    pub fn move_report(&mut self, index: usize, to: usize) -> Result<(), Error> {
        let mut list = self.report_list();
        if index >= list.len() || to >= list.len() {
            return Err(no_report());
        }
        let r = list.remove(index);
        list.insert(to, r);
        self.set_report_list(list)
    }

    /// Deletes report `index`. The last report of a kind stays.
    pub fn delete_report(&mut self, index: usize) -> Result<(), Error> {
        let mut list = self.report_list();
        let kind = list.get(index).ok_or_else(no_report)?.kind;
        if list.iter().filter(|r| r.kind == kind).count() < 2 {
            return Err(Error::InUse(format!(
                "{} is the only {} report",
                list[index].name,
                kind.label()
            )));
        }
        list.remove(index);
        self.set_report_list(list)
    }

    /// Drops ids that no longer name a category, element or strip layout from every
    /// report's filters, category columns and layout pick, and a deleted category from
    /// every strip layout's element count.
    pub(crate) fn tidy_report_filters(&mut self) {
        let cats: std::collections::HashSet<CategoryId> =
            self.categories.iter().map(|c| c.id).collect();
        let els: std::collections::HashSet<ElementId> =
            self.elements.iter().map(|e| e.id).collect();
        let strips: std::collections::HashSet<u32> = self
            .layouts
            .layouts
            .iter()
            .filter(|l| matches!(l.body, crate::layouts::LayoutBody::Strips(_)))
            .map(|l| l.id.0)
            .collect();
        for r in &mut self.reports {
            r.options.categories.retain(|c| cats.contains(c));
            r.options.element_columns.retain(|c| cats.contains(c));
            r.options.elements.retain(|e| els.contains(e));
            if r.options.layout.is_some_and(|l| !strips.contains(&l.0)) {
                r.options.layout = None;
            }
        }
        if self.reports == default_reports() {
            self.reports.clear();
        }
        // An element count of a deleted category counts every element again (decision
        // 147).
        for b in &mut self.boards {
            b.strip_layout.tidy_counts(&cats);
        }
        for l in &mut self.layouts.layouts {
            if let crate::layouts::LayoutBody::Strips(x) = &mut l.body {
                x.tidy_counts(&cats);
            }
        }
    }
}

fn no_report() -> Error {
    Error::Broken("that report is not in the list".to_string())
}

/// The report list's consistency rules (also part of `Schedule::check`).
pub(crate) fn check_reports(s: &Schedule, list: &[ReportSetup]) -> Result<(), Error> {
    for (i, r) in list.iter().enumerate() {
        if r.name.trim().is_empty() {
            return Err(Error::Broken("a report has no name".to_string()));
        }
        if r.name.chars().count() > NAME_LIMIT {
            return Err(Error::Broken(format!(
                "report name {:?} is longer than {NAME_LIMIT} characters",
                r.name
            )));
        }
        if list[..i]
            .iter()
            .any(|o| crate::model::same_name(&o.name, &r.name))
        {
            return Err(Error::DuplicateName(r.name.clone()));
        }
        if let Some(why) = r.options.problem() {
            return Err(Error::Broken(format!("report {:?}: {why}", r.name)));
        }
        if let Some(c) = r
            .options
            .categories
            .iter()
            .find(|c| s.category(**c).is_none())
        {
            return Err(Error::Broken(format!(
                "report {:?} picks missing category {}",
                r.name, c.0
            )));
        }
        if let Some(c) = r
            .options
            .element_columns
            .iter()
            .find(|c| s.category(**c).is_none())
        {
            return Err(Error::Broken(format!(
                "report {:?} has a column of missing category {}",
                r.name, c.0
            )));
        }
        if let Some(id) = r.options.layout {
            match s.layouts.get(id) {
                Some(crate::layouts::SavedLayout {
                    body: crate::layouts::LayoutBody::Strips(_),
                    ..
                }) => {}
                _ => {
                    return Err(Error::Broken(format!(
                        "report {:?} picks layout {}, which is not a strip layout",
                        r.name, id.0
                    )))
                }
            }
        }
        if let Some(e) = r.options.elements.iter().find(|e| s.element(**e).is_none()) {
            return Err(Error::Broken(format!(
                "report {:?} picks missing element {}",
                r.name, e.0
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Review: a 200-character name once came back as itself, a duplicate.
    #[test]
    fn duplicate_a_report_with_a_long_name() {
        let mut s = Schedule::new("T");
        let mut list = s.report_list();
        list[0].name = "x".repeat(NAME_LIMIT);
        s.set_report_list(list).unwrap();
        s.duplicate_report(0).unwrap();
        s.duplicate_report(0).unwrap();
        let list = s.report_list();
        assert!(list[2].name.ends_with("x copy"), "{}", list[2].name);
        assert!(list[1].name.ends_with("x copy 2"), "{}", list[1].name);
        assert_eq!(list[1].name.chars().count(), NAME_LIMIT);
    }

    #[test]
    fn the_list_starts_as_every_kind_and_stays_out_of_the_schedule_at_the_default() {
        let mut s = Schedule::new("ZTEST Setup");
        assert_eq!(s.report_list().len(), ReportKind::ALL.len());
        assert!(s.reports.is_empty());
        let at = s.duplicate_report(0).unwrap();
        assert_eq!(at, 1);
        assert_eq!(s.report_list()[1].name, "One-line schedule copy");
        assert_eq!(s.report_list()[1].kind, ReportKind::OneLine);
        let at2 = s.duplicate_report(0).unwrap();
        assert_eq!(s.report_list()[at2].name, "One-line schedule copy 2");
        assert!(!s.reports.is_empty());
        // Deleting the copies brings back the default, which leaves the file.
        s.delete_report(1).unwrap();
        s.delete_report(1).unwrap();
        assert!(s.reports.is_empty());
        // The last of a kind stays.
        assert!(matches!(s.delete_report(0), Err(Error::InUse(_))));
    }

    #[test]
    fn options_names_and_order_are_checked() {
        let mut s = Schedule::new("ZTEST Setup");
        s.move_report(0, 2).unwrap();
        assert_eq!(s.report_list()[2].kind, ReportKind::OneLine);
        let mut r = s.report_list()[2].clone();
        r.name = "  Shooting schedule ".into();
        assert!(matches!(
            s.set_report(2, r.clone()),
            Err(Error::DuplicateName(_))
        ));
        r.name = "Cast one-line".into();
        r.options.from = "2027-03-05".parse().ok();
        r.options.to = "2027-03-01".parse().ok();
        assert!(s.set_report(2, r.clone()).is_err());
        r.options.to = None;
        r.options.categories = vec![CategoryId(999)];
        assert!(s.set_report(2, r.clone()).is_err());
        r.options.categories = vec![s.cast().unwrap()];
        s.set_report(2, r.clone()).unwrap();
        assert_eq!(s.report_list()[2].name, "Cast one-line");
        let mut k = r.clone();
        k.kind = ReportKind::Dood;
        assert!(s.set_report(2, k).is_err());
        assert!(s.check().is_ok());
    }

    #[test]
    fn missing_kinds_come_back_at_the_end() {
        let mut s = Schedule::new("ZTEST Setup");
        s.reports = vec![ReportSetup::of(ReportKind::Calendar)];
        let list = s.report_list();
        assert_eq!(list[0].kind, ReportKind::Calendar);
        assert_eq!(list.len(), ReportKind::ALL.len());
    }

    #[test]
    fn report_changes_say_what_happened() {
        let mut s = Schedule::new("ZTEST Setup");
        let a = s.report_list();
        s.duplicate_report(0).unwrap();
        let mut r = s.report_list()[2].clone();
        r.options.synopsis = false;
        s.set_report(2, r).unwrap();
        let b = s.report_list();
        assert_eq!(
            report_changes(&a, &b),
            [
                "report \"One-line schedule copy\" added",
                "report \"Shooting schedule\" options changed"
            ]
        );
        let mut d = default_reports();
        d.swap(0, 1);
        assert_eq!(
            report_changes(&default_reports(), &d),
            ["report order changed"]
        );
        assert_eq!(reports_text(&[]), "(default)");
        assert_eq!(DoodCodes::default().text(), "S W F H D P TR FT R / M");
    }

    #[test]
    fn diff_lists_setup_changes() {
        let a = Schedule::new("ZTEST Setup");
        let mut b = a.clone();
        b.production.producer = "P. Roducer".into();
        b.duplicate_report(0).unwrap();
        b.dood_codes.hold = "X".into();
        let d = crate::diff::diff(&a, &b);
        let t: Vec<&str> = d.changes.iter().map(|c| c.text.as_str()).collect();
        assert_eq!(
            t,
            [
                "Producer \"\" -> \"P. Roducer\"",
                "Report \"One-line schedule copy\" added",
                "Day out of days hold code \"H\" -> \"X\""
            ]
        );
        assert!(d.changes.iter().all(|c| c.subject.group() == "Schedule"));
    }

    #[test]
    fn code_letters_must_read_one_way() {
        let mut c = DoodCodes::default();
        assert_eq!(c.problem(), None);
        c.hold = "w".into();
        assert!(c.problem().unwrap().contains("used twice"));
        c.hold = "".into();
        assert!(c.problem().is_some());
        c.hold = "H H".into();
        assert!(c.problem().is_some());
        c.hold = "HLD".into();
        assert_eq!(c.problem(), None);
        assert_eq!(c.event("TR"), "TR");
        c.travel = "T".into();
        assert_eq!(c.event("TR"), "T");
    }

    #[test]
    fn filters_follow_deleted_categories_and_elements() {
        let mut s = Schedule::new("ZTEST Setup");
        let props = s.add_category("Props");
        let lamp = s.add_element(props, "Lamp").unwrap();
        let mut r = s.report_list()[0].clone();
        r.options.categories = vec![props];
        r.options.elements = vec![lamp];
        s.set_report(0, r).unwrap();
        s.delete_element(lamp, false).unwrap();
        assert!(s.check().is_ok());
        assert!(s.report_list()[0].options.elements.is_empty());
        s.delete_category(props).unwrap();
        assert!(s.check().is_ok());
        assert!(s.reports.is_empty());
    }
}
