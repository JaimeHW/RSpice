//! DRC/ERC violation markers on the canvas.
//!
//! After a check runs, every violation with a resolvable location gets a
//! screen-constant badge at the offending spot — a severity-colored
//! triangle with an exclamation tick. Hovering a badge shows the message
//! and the suggested fix. Once the topology changes the docbar pill flips
//! to "ERC stale", and only the markers of findings the edit did not touch
//! stay until the next check.

use egui::Painter;

use crate::workbench::app_state::{Model, model_location_anchor};
use rspice_design::drc::{DrcLocation, DrcSeverity};
use rspice_schematic_editor::session::visibility::SchematicReviewMarkerVisibility;

use super::parts::CanvasFacts;
use super::viewport::Viewport;
use rspice_schematic_editor::view::violations as markers;

/// Where one finding location in the drawing in front of the reader points,
/// when it points anywhere.
///
/// Every surface that offers "go to this finding" resolves through here or,
/// for a finding in another cell view, through [`model_location_anchor`] —
/// the canvas badges, the console's per-finding rows, the validated-save
/// report, which carries a location rather than a whole violation, and the
/// published rule findings. One resolver keeps them from sending the author
/// to three different places for one location.
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

/// Where the canvas marks the other objects the current finding involves:
/// each one the drawing in front of the reader draws on the sheet in view,
/// with the finding's severity.
///
/// The current finding may come from a check of another cellview — a loop
/// stated inside an instance of this one — so this asks the store for it
/// rather than the active drawing's own results.
pub fn related_marks(state: &CanvasFacts<'_>) -> Option<(DrcSeverity, Vec<crate::state::Point>)> {
    let subject = state.model.workspace.content.active_schematic_reference();
    let (severity, locations) = state.current_finding_related_marks(&subject)?;
    let view = state.design_view();
    let points = locations
        .iter()
        .filter_map(|location| markers::anchor(&view, location))
        .collect();
    Some((severity, points))
}

/// Draw only check results that remain current for the active schematic,
/// with the current finding's other objects marked beneath them.
pub(super) fn draw_violation_markers(
    painter: &Painter,
    viewport: &Viewport,
    state: &CanvasFacts<'_>,
) {
    if let Some((severity, points)) = related_marks(state) {
        markers::draw_related_marks(painter, viewport, &points, severity);
    }
    let checks = &state.model.design_checks;
    let Some(result) = &checks.drc_results else {
        return;
    };
    let view = state.design_view();
    if checks.drc_checked_version == state.model.schematic.topology_version() {
        markers::draw_violation_markers(
            painter,
            viewport,
            &view,
            result.violations(),
            state.design.canvas_hover,
        );
        return;
    }
    // Once the drawing changed, a finding whose site the edit left alone
    // keeps its marker until the next check; one whose part, pin or wire the
    // edit touched leaves the canvas, since the edit may have fixed it. Its
    // row stays in the console and the dialog either way.
    let untouched = result
        .violations()
        .iter()
        .zip(&checks.drc_sites)
        .filter(|(violation, site)| {
            markers::site_fingerprint(view.document, &violation.location) == **site
        })
        .map(|(violation, _)| violation.clone())
        .collect::<Vec<_>>();
    markers::draw_violation_markers(
        painter,
        viewport,
        &view,
        &untouched,
        state.design.canvas_hover,
    );
}
