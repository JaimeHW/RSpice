//! DRC/ERC violation markers on the canvas.
//!
//! After a check runs, every violation with a resolvable location gets a
//! screen-constant badge at the offending spot — a severity-colored
//! triangle with an exclamation tick. Hovering a badge shows the message
//! and the suggested fix. Markers hide as soon as the topology changes
//! (stale markers lie); the docbar pill flips to "ERC stale" instead.

use egui::Painter;

use crate::workbench::app_state::{AppState, Model, model_location_anchor};
use rspice_design::drc::{DrcLocation, DrcSeverity};
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::viewport::Viewport;
use rspice_schematic_editor::view::violations as markers;

#[cfg(test)]
pub(crate) mod tests;

/// Where one finding location in the drawing in front of the reader points,
/// when it points anywhere.
///
/// Every surface that offers "go to this finding" resolves through here or
/// through [`location_anchor_in`] — the canvas badges, the console's
/// per-finding rows, the validated-save report, which carries a location
/// rather than a whole violation, and the published rule findings. One
/// resolver keeps them from sending the author to three different places for
/// one location.
pub(crate) fn location_anchor(
    model: &Model,
    review_markers: SchematicReviewMarkerVisibility,
    location: &DrcLocation,
) -> Option<crate::diagnostics::LogAnchor> {
    model_location_anchor(
        model,
        review_markers,
        &model.workspace.content.active_schematic_reference(),
        location,
    )
}

/// Where one finding location in `cell_view` points, when it points anywhere.
///
/// The drawing in front of the reader is asked through its live editor, and
/// any other cell view through the working copy the workspace holds for it,
/// so a finding about a view that is not open still says where it is. The
/// anchor carries the view and the sheet the object is drawn on, and
/// following it opens that view and brings that sheet forward first.
pub(crate) fn location_anchor_in(
    state: &AppState,
    cell_view: &crate::state::CellViewRef,
    location: &DrcLocation,
) -> Option<crate::diagnostics::LogAnchor> {
    model_location_anchor(
        &state.model,
        state.design.schematic_visibility.review_markers,
        cell_view,
        location,
    )
}

/// Where the canvas marks the other objects the current finding involves:
/// each one the drawing in front of the reader draws on the sheet in view,
/// with the finding's severity.
///
/// The current finding may come from a check of another cellview — a loop
/// stated inside an instance of this one — so this asks the store for it
/// rather than the active drawing's own results.
pub(crate) fn related_marks(state: &AppState) -> Option<(DrcSeverity, Vec<crate::state::Point>)> {
    let subject = state.model.workspace.content.active_schematic_reference();
    let (severity, locations) = state.current_finding_related_marks(&subject)?;
    let view = super::schematic_design_view(state);
    let points = locations
        .iter()
        .filter_map(|location| markers::anchor(&view, location))
        .collect();
    Some((severity, points))
}

/// Draw only check results that remain current for the active schematic,
/// with the current finding's other objects marked beneath them.
pub(super) fn draw_violation_markers(painter: &Painter, viewport: &Viewport, state: &AppState) {
    if let Some((severity, points)) = related_marks(state) {
        markers::draw_related_marks(painter, viewport, &points, severity);
    }
    let Some(result) = &state.model.design_checks.drc_results else {
        return;
    };
    // Stale results stay in the dialog/console but vanish from the canvas.
    if state.model.design_checks.drc_checked_version != state.model.schematic.topology_version() {
        return;
    }

    markers::draw_violation_markers(
        painter,
        viewport,
        &super::schematic_design_view(state),
        result.violations(),
        state.design.canvas_hover,
    );
}
