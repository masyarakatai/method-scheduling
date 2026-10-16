//! A color per red flag type (decisions.md, entry 84): its marks on the board, the
//! calendar, the Breakdown tab, the Red flags tab and an element's flags are drawn in it.

use tauri::{Runtime, WebviewWindow};

use method_schedule_core::model::{RedFlagTypeId, Rgb};
use method_schedule_core::Schedule;

use crate::app::{finish, CmdResult};
use crate::board::EditResult;
use crate::docs::Doc;

/// Sets red flag type `kind`'s color: `#rrggbb`, or empty for none (the usual red).
pub fn set_color(s: &mut Schedule, kind: u32, color: &str) -> EditResult {
    let color = color.trim();
    let parsed = if color.is_empty() {
        None
    } else {
        Some(Rgb::parse(color).ok_or_else(|| format!("{color:?} is not a color like #cc3333."))?)
    };
    let t = s
        .red_flag_types
        .iter_mut()
        .find(|t| t.id == RedFlagTypeId(kind))
        .ok_or("That red flag type is no longer in the schedule.")?;
    t.color = parsed;
    Ok(())
}

/// Red Flag Types > a type's color, one undo step.
#[tauri::command(rename_all = "snake_case")]
pub fn set_red_flag_type_color<R: Runtime>(
    window: WebviewWindow<R>,
    state: Doc,
    red_flag_type: u32,
    color: String,
) -> CmdResult {
    let mut s = state.lock();
    s.edit(|x| set_color(x, red_flag_type, &color))?;
    finish(&window, &s, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::Session;
    use method_schedule_core::model::RedFlagTypeId;

    #[test]
    fn a_type_gets_a_color_and_loses_it() {
        let mut s = Session::new();
        let mut id = 0;
        s.edit(|x| {
            id = x
                .add_red_flag_type("Second job")
                .map_err(|e| e.to_string())?
                .0;
            Ok(())
        })
        .unwrap();
        s.edit(|x| set_color(x, id, "#3366CC")).unwrap();
        let of = |s: &Session| s.schedule().red_flag_type(RedFlagTypeId(id)).unwrap().color;
        assert_eq!(of(&s), Some(Rgb(0x3366cc)));
        assert_eq!(
            crate::redflags::types(s.schedule())
                .iter()
                .find(|t| t.id == id)
                .unwrap()
                .color
                .as_deref(),
            Some("#3366cc")
        );
        assert!(s.edit(|x| set_color(x, id, "blue")).is_err());
        assert!(s.edit(|x| set_color(x, 9999, "#000000")).is_err());
        s.edit(|x| set_color(x, id, "")).unwrap();
        assert_eq!(of(&s), None);
    }
}
