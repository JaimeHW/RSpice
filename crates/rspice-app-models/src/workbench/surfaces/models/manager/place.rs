//! What placing one shelf row does.
//!
//! Browsing parts a schematic cannot receive is a catalogue, not a chooser, so
//! the shelf offers Place. There is exactly one way a part reaches the cursor:
//! [`crate::workbench::app::arm_part_placement`], taking a [`PartPlacement`]
//! planned from the loaded library that holds the part. Every row the shelf
//! lists is in such a library, so nothing needs retaining or publishing before
//! the cursor is armed.
//!
//! # Refusal is a sentence, not a missing button
//!
//! A control that vanishes teaches nothing. Every reason placement is
//! impossible is a string here, and the shelf renders the control disabled
//! carrying it.

use super::*;

use crate::state::part_placement::{PartPlacement, plan_library_placement, refusal_sentence};

use shelf::ShelfRow;

/// The placement this row would arm, or why it has none.
///
/// The error is the sentence a reader sees on the disabled control, so it says
/// what about *this part* cannot be drawn rather than that something failed.
pub(super) fn plan(manager: &ModelLibraryManager, row: &ShelfRow) -> Result<PartPlacement, String> {
    let library = manager.get_library(&row.part.library).ok_or_else(|| {
        format!(
            "Model library '{}' is no longer loaded in this session.",
            row.part.library
        )
    })?;
    plan_library_placement(library, &row.part.part_id).map_err(refusal_sentence)
}

/// Why this session cannot place anything at all right now.
///
/// Separate from [`plan`] because these are facts about the session, not about
/// the part: they change with which document is in front and whether the
/// project may be written, and they apply to every row on the page at once.
pub(super) fn session_block_reason(model: &Model) -> Option<&'static str> {
    // A placement is armed on a sheet's cursor, so there has to be a sheet.
    // The active view having no schematic buffer is exactly that condition:
    // a symbol view is in front, and a device has nowhere to land on one.
    if model.workspace.active_schematic().is_none() {
        return Some(
            "The view in front is not a schematic, so a placed part would have nowhere to land. Open a schematic first.",
        );
    }
    if model.safe_mode.project_read_only() {
        return Some("This project is open read-only, so it cannot receive an instance.");
    }
    None
}

/// Arms the cursor with one planned placement, and hands over the canvas.
///
/// Runs after the frame, from the workspace's own action queue, because a
/// placement ends in two things a render borrow cannot reach: the canvas
/// focus and the workspace receipt.
pub(super) fn arm(
    wb: &mut WorkbenchSession,
    state: &mut ModelsState,
    ctx: &egui::Context,
    placement: PartPlacement,
) {
    let armed = placement.armed_label();
    wb.outbox
        .push(crate::workbench::framework::requests::ArmPartPlacement(
            placement,
        ));
    rspice_schematic_editor::view::canvas::request_focus(ctx);
    apply_receipt(state, Ok(format!("{armed} is armed for placement.")));
}

/// Everything the shelf needs to offer Place on one row.
///
/// Assembled once per painted detail rather than per control so the placement
/// is planned exactly once for the row a reader is looking at.
pub(super) struct PlaceOffer {
    pub(super) placement: Option<Box<PartPlacement>>,
    pub(super) blocked: Option<String>,
}

impl PlaceOffer {
    pub(super) fn for_row(model: &Model, row: &ShelfRow) -> Self {
        match plan(&model.model_library_manager, row) {
            Err(reason) => Self {
                placement: None,
                blocked: Some(reason),
            },
            Ok(placement) => Self {
                placement: Some(Box::new(placement)),
                blocked: session_block_reason(model).map(str::to_owned),
            },
        }
    }
}
