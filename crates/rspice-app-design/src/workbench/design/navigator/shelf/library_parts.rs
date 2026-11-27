//! The shelf's library parts: every part the loaded model libraries offer
//! this shelf, armed on a click.

use super::*;

/// Complete one click on a library-part row.
pub(super) fn apply_library_part_row(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    row: LibraryPartRow,
    ctx: &egui::Context,
) {
    match row.action {
        LibraryPartAction::Arm(placement) => arm_library_part(model, wb, *placement, ctx),
        // A refused row renders disabled, so its click never arrives.
        LibraryPartAction::Refused(_) => {}
    }
}

/// One indexed part the shelf can offer, with what a click would do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryPartRow {
    pub part_id: String,
    /// The part's device class, in the index's vocabulary.
    pub(in crate::workbench::design::navigator) device: String,
    /// The project library holding the part, or nothing for a built-in one.
    pub(in crate::workbench::design::navigator) source: String,
    /// The meta column: the part's device class.
    pub meta: String,
    pub action: LibraryPartAction,
}

/// Where one library-part click goes.
///
/// The same fork the Models workspace shelf decides in its `place` module: a
/// definition a loaded library holds is armed directly, and a part that cannot
/// be drawn is refused with the sentence the disabled row carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryPartAction {
    /// Arm now: the definition is in this project's catalog — compiled in,
    /// or held by one of the project's model files.
    Arm(Box<PartPlacement>),
    /// No placement exists for this part; the sentence the disabled control
    /// carries.
    Refused(String),
}

/// Every part the loaded model libraries can offer this shelf.
///
/// The "Project library" section below reads the symbol-cell library, not the
/// model-library manager, so this section is the only door such a definition
/// has onto the canvas.
pub fn library_part_rows(model: &Model, query: &ShelfQuery) -> Vec<LibraryPartRow> {
    let libraries = model.model_library_manager.libraries_sorted();
    shelf_rows(&library_part_index(&libraries), &libraries, query)
}

/// Decides each indexed part's door, dropping the parts that have none.
///
/// Separated from [`library_part_rows`] so a test can feed it libraries
/// without a model.
pub(in crate::workbench::design::navigator) fn shelf_rows(
    index: &[LibraryPart],
    libraries: &[&ModelLibrary],
    query: &ShelfQuery,
) -> Vec<LibraryPartRow> {
    /// Rows the shelf lists before asking the user to narrow the search.
    ///
    /// A foundry library is large; a navigator column is not. The cap is a
    /// rendering decision, and the footer says when it bit.
    const SHELF_ROWS: usize = 200;

    index
        .iter()
        .filter(|part| {
            library_part_rank(query, &part.part_id, &part.device, part_source(part)).is_some()
        })
        .take(SHELF_ROWS)
        .filter_map(|part| library_part_row(libraries, part))
        .collect()
}

/// The address a part is searched by besides its number: the project library
/// holding it — the same column the Models workspace shelf searches. A
/// compiled-in part has none.
fn part_source(part: &LibraryPart) -> &str {
    if part.built_in { "" } else { &part.library }
}

/// How one library part answers a query. Its number and address are part
/// numbers and its device class supplies the words; it has no label beyond
/// its number.
pub(super) fn library_part_rank(
    query: &ShelfQuery,
    part_id: &str,
    device: &str,
    source: &str,
) -> Option<MatchRank> {
    query.rank(&ShelfTerms {
        label: "",
        keywords: device_class_keywords(device),
        part_numbers: &[part_id, source],
    })
}

/// One shelf row, with the route its click takes.
///
/// The part comes out of the same library set this frame read, so a library
/// that is not in it means the part and the set disagree — the part is dropped
/// rather than offered against bytes that are not there.
fn library_part_row(libraries: &[&ModelLibrary], part: &LibraryPart) -> Option<LibraryPartRow> {
    let holder = libraries
        .iter()
        .copied()
        .find(|library| library.name == part.library)?;
    let action = match plan_library_placement(holder, &part.part_id) {
        Ok(placement) => LibraryPartAction::Arm(Box::new(placement)),
        Err(reason) => LibraryPartAction::Refused(refusal_sentence(reason)),
    };
    Some(LibraryPartRow {
        part_id: part.part_id.clone(),
        device: part.device.clone(),
        source: part_source(part).to_owned(),
        // A part that is here and placeable is the normal case; where it
        // came from is the Models workspace's business.
        meta: part.device.clone(),
        action,
    })
}

/// The shelf section for the parts the unified model index lists.
pub(super) fn library_parts_section(
    ui: &mut Ui,
    model: &mut Model,
    rows: &[LibraryPartRow],
) -> Option<LibraryPartRow> {
    if rows.is_empty() {
        return None;
    }
    if !catalog_group_row(
        ui,
        "component-shelf-library-parts",
        ShelfGlyph::Icon(WorkbenchIcon::Models),
        "Library parts",
        rows.len(),
        false,
    ) {
        return None;
    }

    let mut requested = None;
    for row in rows {
        let placeable = !matches!(row.action, LibraryPartAction::Refused(_));
        let offered = ui
            .add_enabled_ui(placeable, |ui| {
                let response = nav_row_indented_response(
                    ui,
                    WorkbenchIcon::Models,
                    &row.part_id,
                    false,
                    Some(&row.meta),
                    2,
                );
                match &row.action {
                    // The row stays searchable and stays readable; only the
                    // action is refused, and the refusal says why.
                    LibraryPartAction::Refused(reason) => {
                        response.on_disabled_hover_text(reason.as_str());
                        None
                    }
                    LibraryPartAction::Arm(_) => {
                        Some(response.on_hover_text(format!("Click to arm {}", row.part_id)))
                    }
                }
            })
            .inner;
        // A refused part has no placement to pin, so it carries no pin menu
        // either: the rail must not fill with doors that lead nowhere.
        let Some(response) = offered else {
            continue;
        };
        if response.clicked() {
            requested = Some(row.clone());
        }
        shelf_pin_context_menu(
            &response,
            model,
            &ShelfEntry::LibraryPart(row.part_id.clone()),
        );
    }
    requested
}

/// Arms the cursor with a definition the project already holds.
///
/// The same completion the Models workspace shelf performs when it arms a
/// held part, minus that workspace's operation receipt: this click happened
/// on the canvas side, so the armed cursor is the whole story.
fn arm_library_part(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    placement: PartPlacement,
    ctx: &egui::Context,
) {
    let _ = crate::workbench::app::arm_part_placement(model, placement);
    finish_shelf_placement(wb, ctx);
}
