//! Tools that add and delete sheets and edit elements and categories as the app's
//! Breakdown and Elements tabs do: new and delete sheet, rename, delete, merge, move and
//! renumber elements, add and delete categories (features BRK-05, BRK-08, ELM-01, ELM-10,
//! ELM-17 to ELM-19). Renaming and moving a category go through `set_category`.

use method_schedule_core::model::{CategoryId, ElementId, Place, Sheet, SheetId};
use method_schedule_core::reports::element_list;
use serde_json::json;

use super::super::files::edit;
use super::super::schema::{array, boolean, integer, name, object};
use super::write::done;
use super::{board_prop, id_prop, obj, path_prop, pick_board, quoted, revision_prop, Args, Ctx};
use super::{Tool, ToolError, ToolResult};

pub fn tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "new_sheet",
            title: "New sheet",
            description: "Adds an empty breakdown sheet, as the Breakdown tab's New Sheet \
                          does: right after sheet `after` when that sheet is on a sub-board \
                          of the board, otherwise at the end of the Boneyard; every other \
                          board gets it at the end of its Boneyard. Fill it in with \
                          set_sheet_fields. Returns the new sheet's id.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        board_prop(),
                        ("after", id_prop("The sheet to put it after.")),
                    ],
                    &["path", "revision"],
                )
            },
            run: new_sheet,
        },
        Tool {
            name: "delete_sheet",
            title: "Delete a sheet",
            description: "Deletes a breakdown sheet and its strip on every board. Its \
                          elements stay in the schedule; day breaks and banners around it \
                          stay where they are.",
            mutating: true,
            destructive: true,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("sheet", id_prop("The sheet id.")),
                    ],
                    &["path", "revision", "sheet"],
                )
            },
            run: delete_sheet,
        },
        Tool {
            name: "rename_element",
            title: "Rename an element",
            description: "Renames an element. Names are unique in a category, ignoring case.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("element", id_prop("The element id.")),
                        ("name", name("The new name.")),
                    ],
                    &["path", "revision", "element", "name"],
                )
            },
            run: rename_element,
        },
        Tool {
            name: "delete_element",
            title: "Delete an element",
            description: "Deletes an element: it leaves every sheet and event. An element \
                          linked to others, or linked from them, is refused unless `unlink` \
                          is true, which drops its links both ways first.",
            mutating: true,
            destructive: true,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("element", id_prop("The element id.")),
                        ("unlink", boolean("Drop its links first. Default false.")),
                    ],
                    &["path", "revision", "element"],
                )
            },
            run: delete_element,
        },
        Tool {
            name: "merge_elements",
            title: "Merge elements",
            description: "Merges elements of one category into `survivor`, which keeps its \
                          name, board ID and rules: every sheet and event that listed any of \
                          them lists the survivor once, their links, quantities and red flags \
                          follow, and the others are deleted. Elements of different \
                          categories are refused, as in the reference app.",
            mutating: true,
            destructive: true,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("survivor", id_prop("The element to keep.")),
                        (
                            "others",
                            array(
                                "The elements merged into it.",
                                id_prop("An element id."),
                                500,
                            ),
                        ),
                    ],
                    &["path", "revision", "survivor", "others"],
                )
            },
            run: merge_elements,
        },
        Tool {
            name: "move_element",
            title: "Move an element to another category",
            description: "Moves an element to another category. Its name and its board ID \
                          must be free there, or the move is refused. It keeps its board ID \
                          and lock (one without an ID keeps none, also in a category \
                          that numbers automatically), its rules, links and sheets; \
                          its custom property values are lost.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("element", id_prop("The element id.")),
                        ("category", id_prop("The category id to move it to.")),
                    ],
                    &["path", "revision", "element", "category"],
                )
            },
            run: move_element,
        },
        Tool {
            name: "renumber_elements",
            title: "Renumber a category's board IDs",
            description: "Gives a category's elements board IDs from `start` (1 by default), \
                          in the order list_elements gives them (board ID, then name). \
                          Locked IDs stay and their numbers are skipped; IDs with letters \
                          stay; every other element, with or without an ID, gets the next \
                          free number.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("category", id_prop("The category id.")),
                        ("start", integer("The first number. Default 1.", 1, 99_999)),
                    ],
                    &["path", "revision", "category"],
                )
            },
            run: renumber_elements,
        },
        Tool {
            name: "add_category",
            title: "Add a category",
            description: "Adds an element category at the end. Names are unique, ignoring \
                          case: a name already used is refused.",
            mutating: true,
            destructive: false,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("name", name("The category's name.")),
                    ],
                    &["path", "revision", "name"],
                )
            },
            run: add_category,
        },
        Tool {
            name: "delete_category",
            title: "Delete a category",
            description: "Deletes a category that holds no elements. The cast category is \
                          built in and can't be deleted.",
            mutating: true,
            destructive: true,
            schema: || {
                object(
                    vec![
                        path_prop(),
                        revision_prop(),
                        ("category", id_prop("The category id.")),
                    ],
                    &["path", "revision", "category"],
                )
            },
            run: delete_category,
        },
    ]
}

fn new_sheet(ctx: &Ctx, a: &Args) -> ToolResult {
    let after = a.u32("after").map(SheetId);
    let e = edit(ctx, a, |s| {
        let board = pick_board(s, a)?;
        if let Some(x) = after {
            s.sheet(x)
                .ok_or_else(|| ToolError::not_found(format!("no sheet {}", x.0)))?;
        }
        let at = after
            .and_then(|x| s.sheet_place(board, x))
            .filter(|p| p.sub_board.is_some())
            .map(|p| Place {
                sub_board: p.sub_board,
                index: p.index + 1,
            })
            .unwrap_or(Place {
                sub_board: None,
                index: usize::MAX,
            });
        Ok((board, s.add_sheet_at(Sheet::default(), board, at)?))
    })?;
    let (board, sheet) = e.out;
    let place = e.schedule.sheet_place(board, sheet);
    Ok(done(
        &e,
        &format!("Added sheet {}", sheet.0),
        obj(vec![
            ("sheet", json!(sheet.0)),
            ("board", json!(board.0)),
            (
                "sub_board",
                json!(place.and_then(|p| p.sub_board).map(|x| x.0)),
            ),
            ("index", json!(place.map(|p| p.index))),
        ]),
    ))
}

fn delete_sheet(ctx: &Ctx, a: &Args) -> ToolResult {
    let sheet = SheetId(a.req_u32("sheet")?);
    let e = edit(ctx, a, |s| Ok(s.delete_sheet(sheet)?))?;
    Ok(done(
        &e,
        &format!("Deleted sheet {}", sheet.0),
        obj(vec![("sheet", json!(sheet.0))]),
    ))
}

fn rename_element(ctx: &Ctx, a: &Args) -> ToolResult {
    let id = ElementId(a.req_u32("element")?);
    let to = a.req("name")?.to_string();
    let e = edit(ctx, a, |s| Ok(s.rename_element(id, &to)?))?;
    let now = e.schedule.element(id).expect("renamed").name.clone();
    Ok(done(
        &e,
        &format!("Renamed element {} to {}", id.0, quoted(&now)),
        obj(vec![("element", json!(id.0)), ("name", json!(now))]),
    ))
}

fn delete_element(ctx: &Ctx, a: &Args) -> ToolResult {
    let id = ElementId(a.req_u32("element")?);
    let unlink = a.opt_bool("unlink").unwrap_or(false);
    let e = edit(ctx, a, |s| Ok(s.delete_element(id, unlink)?))?;
    Ok(done(
        &e,
        &format!("Deleted element {}", id.0),
        obj(vec![("element", json!(id.0))]),
    ))
}

fn merge_elements(ctx: &Ctx, a: &Args) -> ToolResult {
    let survivor = ElementId(a.req_u32("survivor")?);
    let others: Vec<ElementId> = a.u32s("others").into_iter().map(ElementId).collect();
    if !others.iter().any(|o| *o != survivor) {
        return Err(ToolError::invalid(
            "`others` needs at least one element besides the survivor",
        ));
    }
    let e = edit(ctx, a, |s| Ok(s.merge_elements(survivor, &others)?))?;
    let sheets = e
        .schedule
        .sheets
        .iter()
        .filter(|h| h.elements.contains(&survivor))
        .count();
    let mut merged: Vec<u32> = others
        .iter()
        .filter(|o| **o != survivor)
        .map(|o| o.0)
        .collect();
    merged.dedup();
    Ok(done(
        &e,
        &format!("Merged {} into element {}", merged.len(), survivor.0),
        obj(vec![
            ("survivor", json!(survivor.0)),
            ("merged", json!(merged)),
            ("sheets", json!(sheets)),
        ]),
    ))
}

fn move_element(ctx: &Ctx, a: &Args) -> ToolResult {
    let id = ElementId(a.req_u32("element")?);
    let to = CategoryId(a.req_u32("category")?);
    let e = edit(ctx, a, |s| Ok(s.move_element(id, to)?))?;
    let el = e.schedule.element(id).expect("moved");
    Ok(done(
        &e,
        &format!("Moved element {} to category {}", id.0, to.0),
        obj(vec![
            ("element", json!(id.0)),
            ("category", json!(el.category.0)),
            ("board_id", json!(el.board_id)),
        ]),
    ))
}

fn renumber_elements(ctx: &Ctx, a: &Args) -> ToolResult {
    let category = CategoryId(a.req_u32("category")?);
    let start = a.u32("start").unwrap_or(1);
    let e = edit(ctx, a, |s| {
        s.category(category)
            .ok_or_else(|| ToolError::not_found(format!("no category {}", category.0)))?;
        // The list's order is board ID, then name, on any sub-board; the first serves.
        let order: Vec<ElementId> = match s
            .boards
            .first()
            .and_then(|b| b.sub_boards.first().map(|sb| (b.id, sb.id)))
        {
            Some((b, sb)) => element_list(s, b, sb)?
                .into_iter()
                .filter(|l| l.category == category)
                .map(|l| l.element)
                .collect(),
            None => Vec::new(),
        };
        Ok(s.renumber(category, start, &order)?)
    })?;
    let ids: Vec<_> = e
        .schedule
        .elements
        .iter()
        .filter(|x| x.category == category)
        .map(|x| json!({"element": x.id.0, "board_id": x.board_id}))
        .collect();
    Ok(done(
        &e,
        &format!("Renumbered category {} from {start}", category.0),
        obj(vec![
            ("category", json!(category.0)),
            ("elements", json!(ids)),
        ]),
    ))
}

fn add_category(ctx: &Ctx, a: &Args) -> ToolResult {
    let n = a.req("name")?.trim().to_string();
    let e = edit(ctx, a, |s| {
        if n.is_empty() {
            return Err(ToolError::invalid("a category needs a name"));
        }
        if s.category_named(&n).is_some() {
            return Err(method_schedule_core::model::Error::DuplicateCategory(n.clone()).into());
        }
        Ok(s.add_category(&n))
    })?;
    Ok(done(
        &e,
        &format!("Added category {}", quoted(&n)),
        obj(vec![("category", json!(e.out.0)), ("name", json!(n))]),
    ))
}

fn delete_category(ctx: &Ctx, a: &Args) -> ToolResult {
    let id = CategoryId(a.req_u32("category")?);
    let e = edit(ctx, a, |s| Ok(s.delete_category(id)?))?;
    Ok(done(
        &e,
        &format!("Deleted category {}", id.0),
        obj(vec![("category", json!(id.0))]),
    ))
}
