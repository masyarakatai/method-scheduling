//! The layout library (inventory section 5, behavior note 18, decision 78): named layouts
//! kept with the schedule, each of a type (scene strips, day breaks or shoot day strips),
//! that a board picks from, and that the one-line report may pick from on its own.
//!
//! A board keeps its own strip layout and day look (decisions 39 and 67) and uses them
//! until it picks a named layout of that type; picking none goes back to its own. So
//! files from before the library read and write back unchanged, and every place that
//! draws a board asks [`Schedule::strip_layout_of`] and [`Schedule::day_look_of`].

use std::collections::HashSet;

use crate::model::{
    fit_name, same_name, BoardId, CategoryId, DayLook, Error, Schedule, StripLayout,
};

/// A named layout's id. Ids come from the schedule's `next_id`, as every other id does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayoutId(pub u32);

/// What a named layout lays out.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayoutBody {
    /// Scene strips: the fields, their widths and lines.
    Strips(StripLayout),
    /// Day breaks: what they show besides the day and its totals (screen time, sunrise,
    /// sunset, text; `shoot_day_strip` is not used).
    DayBreaks(DayLook),
    /// The shoot day strip heading each day: shown whenever a board picks one, with
    /// its `text` after the day and date (the other switches are not used).
    ShootDay(DayLook),
}

/// The three types, by the names the file and the app use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayoutKind {
    Strips,
    DayBreaks,
    ShootDay,
}

impl LayoutKind {
    pub const ALL: [LayoutKind; 3] = [
        LayoutKind::Strips,
        LayoutKind::DayBreaks,
        LayoutKind::ShootDay,
    ];

    pub fn name(self) -> &'static str {
        match self {
            LayoutKind::Strips => "strips",
            LayoutKind::DayBreaks => "day_breaks",
            LayoutKind::ShootDay => "shoot_day",
        }
    }

    pub fn from_name(name: &str) -> Option<LayoutKind> {
        LayoutKind::ALL.into_iter().find(|k| k.name() == name)
    }

    /// `strip layout`, `day break layout`, `shoot day layout`: for sentences.
    pub fn label(self) -> &'static str {
        match self {
            LayoutKind::Strips => "strip layout",
            LayoutKind::DayBreaks => "day break layout",
            LayoutKind::ShootDay => "shoot day layout",
        }
    }
}

impl LayoutBody {
    pub fn kind(&self) -> LayoutKind {
        match self {
            LayoutBody::Strips(_) => LayoutKind::Strips,
            LayoutBody::DayBreaks(_) => LayoutKind::DayBreaks,
            LayoutBody::ShootDay(_) => LayoutKind::ShootDay,
        }
    }

    pub fn problem(&self) -> Option<String> {
        match self {
            LayoutBody::Strips(l) => l.problem(),
            LayoutBody::DayBreaks(d) | LayoutBody::ShootDay(d) => d.problem(),
        }
    }

    /// For sentences (What Changed, merges); categories are named from `s`.
    pub fn text(&self, s: &Schedule) -> String {
        match self {
            LayoutBody::Strips(l) => l.text(s),
            LayoutBody::DayBreaks(d) => d.text(),
            LayoutBody::ShootDay(d) if d.text.is_empty() => "day and date".to_string(),
            LayoutBody::ShootDay(d) => format!("day and date, {:?}", d.text),
        }
    }
}

/// One named layout.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SavedLayout {
    pub id: LayoutId,
    /// Unique among the layouts, ignoring case.
    pub name: String,
    pub body: LayoutBody,
}

/// The schedule's layouts, in the order they are listed, and the strip layout the
/// one-line report uses (none: the board's).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Library {
    pub layouts: Vec<SavedLayout>,
    pub one_line: Option<LayoutId>,
}

impl Library {
    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty() && self.one_line.is_none()
    }

    pub fn get(&self, id: LayoutId) -> Option<&SavedLayout> {
        self.layouts.iter().find(|l| l.id == id)
    }

    /// For sentences: `Wide (strip layout), Night shoots (day break layout)`; categories
    /// are named from `s`.
    pub fn text(&self, s: &Schedule) -> String {
        if self.layouts.is_empty() {
            return "none".to_string();
        }
        let mut out: Vec<String> = self
            .layouts
            .iter()
            .map(|l| {
                format!(
                    "{:?} ({}: {})",
                    l.name,
                    l.body.kind().label(),
                    l.body.text(s)
                )
            })
            .collect();
        if let Some(l) = self.one_line.and_then(|id| self.get(id)) {
            out.push(format!("one-line report uses {:?}", l.name));
        }
        out.join(", ")
    }
}

/// The named layouts a board picks (none: its own).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LayoutUse {
    pub strips: Option<LayoutId>,
    pub day_breaks: Option<LayoutId>,
    pub shoot_day: Option<LayoutId>,
}

impl LayoutUse {
    pub fn is_default(&self) -> bool {
        *self == LayoutUse::default()
    }

    pub fn get(&self, kind: LayoutKind) -> Option<LayoutId> {
        match kind {
            LayoutKind::Strips => self.strips,
            LayoutKind::DayBreaks => self.day_breaks,
            LayoutKind::ShootDay => self.shoot_day,
        }
    }

    fn slot(&mut self, kind: LayoutKind) -> &mut Option<LayoutId> {
        match kind {
            LayoutKind::Strips => &mut self.strips,
            LayoutKind::DayBreaks => &mut self.day_breaks,
            LayoutKind::ShootDay => &mut self.shoot_day,
        }
    }

    /// For sentences, naming the layouts from `lib`.
    pub fn text(&self, lib: &Library) -> String {
        let name = |id: Option<LayoutId>| {
            id.and_then(|i| lib.get(i))
                .map_or("its own".to_string(), |l| format!("{:?}", l.name))
        };
        format!(
            "strips {}, day breaks {}, shoot day {}",
            name(self.strips),
            name(self.day_breaks),
            name(self.shoot_day)
        )
    }
}

/// The most layouts a schedule keeps.
pub const MAX_LAYOUTS: usize = 200;
/// The longest layout name, in characters.
pub const LAYOUT_NAME_LIMIT: usize = 200;

impl Schedule {
    /// The strip layout board `board` draws its scene strips with: the named one it
    /// picked, else its own. The default layout for an unknown board.
    pub fn strip_layout_of(&self, board: BoardId) -> StripLayout {
        let Some(b) = self.board(board) else {
            return StripLayout::default();
        };
        match b.uses.strips.and_then(|id| self.layouts.get(id)) {
            Some(SavedLayout {
                body: LayoutBody::Strips(l),
                ..
            }) => l.clone(),
            _ => b.strip_layout.clone(),
        }
    }

    /// What board `board`'s day breaks show, and its shoot day strip: its own day look,
    /// with the extras of the day break layout it picked and the shoot day strip (with
    /// its text) of the shoot day layout it picked. `shoot_text` is the shoot day
    /// layout's text.
    pub fn day_look_of(&self, board: BoardId) -> (DayLook, String) {
        let Some(b) = self.board(board) else {
            return (DayLook::default(), String::new());
        };
        let mut look = b.day_look.clone();
        if let Some(SavedLayout {
            body: LayoutBody::DayBreaks(d),
            ..
        }) = b.uses.day_breaks.and_then(|id| self.layouts.get(id))
        {
            look = DayLook {
                shoot_day_strip: look.shoot_day_strip,
                ..d.clone()
            };
        }
        let mut shoot_text = String::new();
        if let Some(SavedLayout {
            body: LayoutBody::ShootDay(d),
            ..
        }) = b.uses.shoot_day.and_then(|id| self.layouts.get(id))
        {
            look.shoot_day_strip = true;
            shoot_text = d.text.clone();
        }
        (look, shoot_text)
    }

    /// The strip layout the one-line report uses for board `board`: the named one the
    /// report picked, else the board's.
    pub fn one_line_layout_of(&self, board: BoardId) -> StripLayout {
        match self.layouts.one_line.and_then(|id| self.layouts.get(id)) {
            Some(SavedLayout {
                body: LayoutBody::Strips(l),
                ..
            }) => l.clone(),
            _ => self.strip_layout_of(board),
        }
    }

    /// The sheet fields a report of `kind` with options `o` shows for board `board`, in
    /// layout order (every line, spacers and custom text left out), as the printed
    /// report picks them (decision 100): its own columns, else its picked strip layout,
    /// else for the one-line the one-line's pick or the board's layout (the multi-unit
    /// one-line the board's), else the kind's usual fields.
    pub fn report_fields(
        &self,
        board: BoardId,
        kind: crate::setup::ReportKind,
        o: &crate::setup::ReportOptions,
    ) -> Vec<crate::model::StripField> {
        self.report_columns(board, kind, o)
            .into_iter()
            .map(|c| c.field)
            .collect()
    }

    /// [`Self::report_fields`] as columns: a layout's columns keep their category pick
    /// and switch, so a category's elements shows in a report that follows a layout as
    /// on the board (decision 194); a report's own columns and its kind's usual fields are
    /// plain.
    pub fn report_columns(
        &self,
        board: BoardId,
        kind: crate::setup::ReportKind,
        o: &crate::setup::ReportOptions,
    ) -> Vec<crate::model::StripColumn> {
        use crate::model::{StripColumn, StripField};
        use crate::setup::ReportKind;
        let picked = o.layout.and_then(|id| match self.layouts.get(id) {
            Some(SavedLayout {
                body: LayoutBody::Strips(l),
                ..
            }) => Some(l.clone()),
            _ => None,
        });
        let columns: Vec<StripColumn> = if !o.columns.is_empty() {
            o.columns.iter().map(|c| StripColumn::of(c.field)).collect()
        } else if let Some(l) = picked {
            l.columns
        } else if kind == ReportKind::OneLine {
            self.one_line_layout_of(board).columns
        } else if kind == ReportKind::MultiUnit {
            self.strip_layout_of(board).columns
        } else {
            kind.default_columns()
                .iter()
                .map(|f| StripColumn::of(*f))
                .collect()
        };
        columns
            .into_iter()
            .filter(|c| !matches!(c.field, StripField::Spacer | StripField::Text))
            .collect()
    }

    /// A layout name, trimmed: not empty, at most 200 characters, and no other layout
    /// (`except` aside) has it, ignoring case.
    pub fn layout_name(&self, name: &str, except: Option<LayoutId>) -> Result<String, Error> {
        let t = name.trim();
        if t.is_empty() {
            return Err(Error::EmptyName);
        }
        if t.chars().count() > LAYOUT_NAME_LIMIT {
            return Err(Error::Broken(
                "a layout name is at most 200 characters".to_string(),
            ));
        }
        if self
            .layouts
            .layouts
            .iter()
            .any(|l| Some(l.id) != except && same_name(&l.name, t))
        {
            return Err(Error::DuplicateName(t.to_string()));
        }
        Ok(t.to_string())
    }

    /// Adds a named layout at the end of the list. Returns its id.
    pub fn add_layout(&mut self, name: &str, body: LayoutBody) -> Result<LayoutId, Error> {
        if self.layouts.layouts.len() >= MAX_LAYOUTS {
            return Err(Error::Broken(format!("at most {MAX_LAYOUTS} layouts")));
        }
        if let Some(why) = body.problem() {
            return Err(Error::Broken(why));
        }
        if let LayoutBody::Strips(x) = &body {
            self.counts_known(x)?;
        }
        let name = self.layout_name(name, None)?;
        let id = LayoutId(self.fresh());
        self.layouts.layouts.push(SavedLayout { id, name, body });
        Ok(id)
    }

    pub fn rename_layout(&mut self, id: LayoutId, name: &str) -> Result<(), Error> {
        let name = self.layout_name(name, Some(id))?;
        self.layout_mut(id)?.name = name;
        Ok(())
    }

    /// Changes what a named layout lays out; its type stays.
    pub fn set_layout_body(&mut self, id: LayoutId, body: LayoutBody) -> Result<(), Error> {
        if let Some(why) = body.problem() {
            return Err(Error::Broken(why));
        }
        if let LayoutBody::Strips(x) = &body {
            self.counts_known(x)?;
        }
        let l = self.layout_mut(id)?;
        if l.body.kind() != body.kind() {
            return Err(Error::Broken(format!(
                "{:?} is a {}",
                l.name,
                l.body.kind().label()
            )));
        }
        l.body = body;
        Ok(())
    }

    /// Moves a named layout to place `to` in the list.
    pub fn move_layout(&mut self, id: LayoutId, to: usize) -> Result<(), Error> {
        let list = &mut self.layouts.layouts;
        let i = list
            .iter()
            .position(|l| l.id == id)
            .ok_or_else(|| Error::Broken(format!("no layout {}", id.0)))?;
        let l = list.remove(i);
        list.insert(to.min(list.len()), l);
        Ok(())
    }

    /// Deletes a named layout. Boards that picked it go back to their own; the one-line
    /// report back to the board's; a report that picked it to its own columns.
    pub fn delete_layout(&mut self, id: LayoutId) -> Result<(), Error> {
        let i = self
            .layouts
            .layouts
            .iter()
            .position(|l| l.id == id)
            .ok_or_else(|| Error::Broken(format!("no layout {}", id.0)))?;
        self.layouts.layouts.remove(i);
        if self.layouts.one_line == Some(id) {
            self.layouts.one_line = None;
        }
        for b in &mut self.boards {
            for k in LayoutKind::ALL {
                let slot = b.uses.slot(k);
                if *slot == Some(id) {
                    *slot = None;
                }
            }
        }
        // A report that picked it follows its own columns again (decision 100).
        self.tidy_report_filters();
        Ok(())
    }

    /// The boards that pick named layout `id`, of any type, in board order.
    pub fn boards_using(&self, id: LayoutId) -> impl Iterator<Item = &crate::model::Board> + '_ {
        self.boards
            .iter()
            .filter(move |b| LayoutKind::ALL.iter().any(|k| b.uses.get(*k) == Some(id)))
    }

    /// Board `board` picks named layout `id` of type `kind` (none: its own again).
    pub fn use_layout(
        &mut self,
        board: BoardId,
        kind: LayoutKind,
        id: Option<LayoutId>,
    ) -> Result<(), Error> {
        if let Some(id) = id {
            let l = self
                .layouts
                .get(id)
                .ok_or_else(|| Error::Broken(format!("no layout {}", id.0)))?;
            if l.body.kind() != kind {
                return Err(Error::Broken(format!(
                    "{:?} is not a {}",
                    l.name,
                    kind.label()
                )));
            }
        }
        let b = self
            .boards
            .iter_mut()
            .find(|b| b.id == board)
            .ok_or(Error::NoSuchBoard(board))?;
        *b.uses.slot(kind) = id;
        Ok(())
    }

    /// The one-line report picks strip layout `id` (none: the board's).
    pub fn use_one_line_layout(&mut self, id: Option<LayoutId>) -> Result<(), Error> {
        if let Some(id) = id {
            match self.layouts.get(id) {
                Some(SavedLayout {
                    body: LayoutBody::Strips(_),
                    ..
                }) => {}
                Some(l) => {
                    return Err(Error::Broken(format!("{:?} is not a strip layout", l.name)))
                }
                None => return Err(Error::Broken(format!("no layout {}", id.0))),
            }
        }
        self.layouts.one_line = id;
        Ok(())
    }

    /// Adds `layouts` (from a layout file) to the list with new ids; a name already used
    /// gets ` 2`, ` 3`... Returns the new ids.
    pub fn import_layouts(
        &mut self,
        layouts: Vec<(String, LayoutBody)>,
    ) -> Result<Vec<LayoutId>, Error> {
        let mut out = Vec::new();
        for (name, body) in layouts {
            let base = name.trim().to_string();
            let base = if base.is_empty() {
                "Layout".to_string()
            } else {
                base
            };
            let mut name = base.clone();
            let mut n = 2u32;
            loop {
                match self.layout_name(&name, None) {
                    Ok(_) => break,
                    // A taken name gets a number, the name cut so the number fits.
                    Err(Error::DuplicateName(_)) => {
                        name = fit_name(&base, &format!(" {n}"), LAYOUT_NAME_LIMIT);
                        n += 1;
                    }
                    Err(e) => return Err(e),
                }
            }
            // A layout file names an element count's category by name, and the reader
            // gave it this schedule's id, or none where it has no such category
            // (decision 147); one still unknown here counts every element.
            let body = match body {
                LayoutBody::Strips(mut x) => {
                    let cats: HashSet<CategoryId> = self.categories.iter().map(|c| c.id).collect();
                    x.tidy_counts(&cats);
                    LayoutBody::Strips(x)
                }
                other => other,
            };
            out.push(self.add_layout(&name, body)?);
        }
        Ok(out)
    }

    fn layout_mut(&mut self, id: LayoutId) -> Result<&mut SavedLayout, Error> {
        self.layouts
            .layouts
            .iter_mut()
            .find(|l| l.id == id)
            .ok_or_else(|| Error::Broken(format!("no layout {}", id.0)))
    }

    /// What `check` asks of the library: ids unique and below `next_id` (into `ids`),
    /// names unique ignoring case, bodies sound, and every pick naming a layout of its
    /// type.
    pub(crate) fn check_layouts(&self, ids: &mut crate::model::IdSet<u32>) -> Result<(), Error> {
        let broken = |why: String| Err(Error::Broken(why));
        if self.layouts.layouts.len() > MAX_LAYOUTS {
            return broken(format!("more than {MAX_LAYOUTS} layouts"));
        }
        for (i, l) in self.layouts.layouts.iter().enumerate() {
            if l.id.0 == 0 || l.id.0 >= self.next_id || !ids.insert(l.id.0) {
                return broken(format!(
                    "layout id {} is used twice or out of range",
                    l.id.0
                ));
            }
            if l.name.trim().is_empty() || l.name.chars().count() > LAYOUT_NAME_LIMIT {
                return broken("a layout has an empty or long name".to_string());
            }
            if self.layouts.layouts[..i]
                .iter()
                .any(|o| same_name(&o.name, &l.name))
            {
                return broken(format!("two layouts are named {:?}", l.name));
            }
            if let Some(why) = l.body.problem() {
                return broken(format!("layout {:?}: {why}", l.name));
            }
            if let LayoutBody::Strips(x) = &l.body {
                if let Some(c) = x.counted().find(|c| self.category(*c).is_none()) {
                    return broken(format!(
                        "layout {:?}: its element count counts a missing category {}",
                        l.name, c.0
                    ));
                }
            }
        }
        let fits = |id: Option<LayoutId>, kind: LayoutKind| {
            id.is_none_or(|id| self.layouts.get(id).is_some_and(|l| l.body.kind() == kind))
        };
        if !fits(self.layouts.one_line, LayoutKind::Strips) {
            return broken(
                "the one-line report picks a layout that isn't a strip layout".to_string(),
            );
        }
        for b in &self.boards {
            for k in LayoutKind::ALL {
                if !fits(b.uses.get(k), k) {
                    return broken(format!(
                        "board {:?} picks a {} that isn't one",
                        b.name,
                        k.label()
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{StripColumn, StripField};

    fn narrow(fields: &[StripField]) -> StripLayout {
        StripLayout::of(fields.iter().copied().map(StripColumn::of).collect())
    }

    #[test]
    fn report_fields_follow_the_reports_columns_then_its_layout() {
        use crate::setup::{ReportColumn, ReportKind, ReportOptions};
        let mut s = Schedule::new("ZTEST");
        let b = s.boards[0].id;
        let mut o = ReportOptions::default();
        // The one-line follows the board's layout; others their usual fields.
        let board: Vec<StripField> = StripLayout::default()
            .columns
            .iter()
            .map(|c| c.field)
            .collect();
        assert_eq!(s.report_fields(b, ReportKind::OneLine, &o), board);
        assert_eq!(
            s.report_fields(b, ReportKind::Shooting, &o),
            ReportKind::Shooting.default_columns()
        );
        // A picked layout, spacers and custom text left out.
        let lay = s
            .add_layout(
                "Mine",
                LayoutBody::Strips(narrow(&[
                    StripField::Set,
                    StripField::Spacer,
                    StripField::Synopsis,
                ])),
            )
            .unwrap();
        o.layout = Some(lay);
        assert_eq!(
            s.report_fields(b, ReportKind::Shooting, &o),
            [StripField::Set, StripField::Synopsis]
        );
        // The report's own columns win.
        o.columns = vec![ReportColumn::of(StripField::Unit)];
        assert_eq!(
            s.report_fields(b, ReportKind::OneLine, &o),
            [StripField::Unit]
        );
    }

    /// Review: a taken 200-character name once renamed itself forever.
    #[test]
    fn import_layouts_with_a_long_taken_name_ends() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut s = Schedule::new("T");
            let long = "x".repeat(LAYOUT_NAME_LIMIT);
            s.add_layout(&long, LayoutBody::DayBreaks(DayLook::default()))
                .unwrap();
            let r = s.import_layouts(vec![(
                long.clone(),
                LayoutBody::DayBreaks(DayLook::default()),
            )]);
            tx.send(r.map(|ids| s.layouts.get(ids[0]).unwrap().name.clone()))
                .unwrap();
        });
        let got = rx
            .recv_timeout(std::time::Duration::from_secs(3))
            .expect("import_layouts never returned (endless renaming loop)")
            .unwrap();
        assert_eq!(got.chars().count(), LAYOUT_NAME_LIMIT);
        assert!(got.ends_with("x 2"), "{got}");
    }

    #[test]
    fn a_board_picks_a_named_layout_and_falls_back_to_its_own() {
        let mut s = Schedule::new("ZTEST");
        let b = s.boards[0].id;
        let wide = s
            .add_layout(
                "Wide",
                LayoutBody::Strips(narrow(&[StripField::Scene, StripField::Synopsis])),
            )
            .unwrap();
        assert!(s
            .add_layout(" wide ", LayoutBody::Strips(StripLayout::default()))
            .is_err());
        assert_eq!(
            s.strip_layout_of(b),
            StripLayout::default(),
            "its own until it picks"
        );
        assert_eq!(s.boards_using(wide).count(), 0);
        s.use_layout(b, LayoutKind::Strips, Some(wide)).unwrap();
        assert_eq!(s.boards_using(wide).map(|x| x.id).collect::<Vec<_>>(), [b]);
        assert_eq!(s.strip_layout_of(b).columns.len(), 2);
        assert_eq!(
            s.one_line_layout_of(b).columns.len(),
            2,
            "the one-line follows"
        );
        // The one-line report picks its own.
        let mine = s
            .add_layout("One-line", LayoutBody::Strips(narrow(&[StripField::Scene])))
            .unwrap();
        s.use_one_line_layout(Some(mine)).unwrap();
        assert_eq!(s.one_line_layout_of(b).columns.len(), 1);
        // A day break layout and a shoot day layout.
        let night = s
            .add_layout(
                "Night",
                LayoutBody::DayBreaks(DayLook {
                    sunset: true,
                    text: "Night work".into(),
                    ..DayLook::default()
                }),
            )
            .unwrap();
        let shoot = s
            .add_layout(
                "Shoot day",
                LayoutBody::ShootDay(DayLook {
                    text: "Call 07:00".into(),
                    ..DayLook::default()
                }),
            )
            .unwrap();
        assert!(
            s.use_layout(b, LayoutKind::Strips, Some(night)).is_err(),
            "wrong type"
        );
        s.use_layout(b, LayoutKind::DayBreaks, Some(night)).unwrap();
        s.use_layout(b, LayoutKind::ShootDay, Some(shoot)).unwrap();
        let (look, text) = s.day_look_of(b);
        assert!(look.sunset && look.shoot_day_strip);
        assert_eq!(
            (look.text.as_str(), text.as_str()),
            ("Night work", "Call 07:00")
        );
        s.check().unwrap();
        // Deleting a layout sends its users back to their own.
        s.delete_layout(wide).unwrap();
        s.delete_layout(mine).unwrap();
        assert_eq!(s.strip_layout_of(b), StripLayout::default());
        assert_eq!(s.layouts.one_line, None);
        s.check().unwrap();
        // A changed body keeps its type.
        assert!(s
            .set_layout_body(night, LayoutBody::Strips(StripLayout::default()))
            .is_err());
        // Imports get fresh ids and free names.
        let ids = s
            .import_layouts(vec![(
                "Night".into(),
                LayoutBody::DayBreaks(DayLook::default()),
            )])
            .unwrap();
        assert_eq!(s.layouts.get(ids[0]).unwrap().name, "Night 2");
        s.check().unwrap();
    }
}
