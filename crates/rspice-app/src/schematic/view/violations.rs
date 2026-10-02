//! DRC/ERC violation markers on the canvas.
//!
//! After a check runs, every violation with a resolvable location gets a
//! screen-constant badge at the offending spot — a severity-colored
//! triangle with an exclamation tick. Hovering a badge shows the message
//! and the suggested fix. Markers hide as soon as the topology changes
//! (stale markers lie); the docbar pill flips to "ERC stale" instead.

use egui::Painter;

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app_state::AppState;
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation};

use super::viewport::Viewport;
use rspice_schematic_editor::view::violations as markers;

/// Jump the view to the next (`step` = 1) or previous (−1) finding,
/// ordered worst severity first. Selects the offending object where one
/// exists and centers the canvas on the anchor; node/global findings have
/// no spot and stay in the console and pill.
pub(crate) fn cycle_violation(state: &mut AppState, step: isize) {
    let Some(result) = &state.dialogs.drc_results else {
        state.push_user_message(ConsoleMessage::info(
            "No check results — run design checks first",
        ));
        return;
    };
    if state.dialogs.drc_checked_version != state.schematic.topology_version() {
        state.push_user_message(ConsoleMessage::warning(
            "Check results are stale — re-run design checks",
        ));
        return;
    }

    // Owned snapshot of the anchored findings so the borrows drop before
    // the selection/viewport mutations below.
    let mut anchored: Vec<(DrcSeverity, crate::diagnostics::LogAnchor, String)> = result
        .violations()
        .iter()
        .filter_map(|violation| {
            finding_anchor(state, violation)
                .map(|anchor| (violation.severity, anchor, violation.message.clone()))
        })
        .collect();
    if anchored.is_empty() {
        state.push_user_message(ConsoleMessage::info(
            "No findings with a canvas anchor — see the console summary",
        ));
        return;
    }
    // Worst first; stable within a severity.
    anchored.sort_by(|a, b| b.0.cmp(&a.0));

    let len = anchored.len() as isize;
    let index = match state.dialogs.drc_cycle {
        // First jump lands on the worst finding, not the second one.
        None if step >= 0 => 0,
        None => (len - 1) as usize,
        Some(current) => ((current as isize + step).rem_euclid(len)) as usize,
    };
    state.dialogs.drc_cycle = Some(index);

    let (severity, anchor, message) = anchored.swap_remove(index);
    state.jump_to_log_anchor(anchor);
    state.push_user_message(ConsoleMessage::info(format!(
        "Finding {}/{} — {} · {}",
        index + 1,
        len,
        severity.display_name(),
        message,
    )));
}

/// What a cycled finding selects on arrival.
/// Resolve a finding to a console jump target — shared by the check
/// runner's per-finding rows and anything else that wants click-to-source.
pub(crate) fn finding_anchor(
    state: &AppState,
    violation: &DrcViolation,
) -> Option<crate::diagnostics::LogAnchor> {
    location_anchor(state, &violation.location)
}

/// Where one finding location points, when it points anywhere.
///
/// Every surface that offers "go to this finding" resolves through here — the
/// canvas badges, the console's per-finding rows, and the validated-save
/// report, which carries a location rather than a whole violation. One
/// resolver keeps them from sending the author to three different places for
/// one location.
pub(crate) fn location_anchor(
    state: &AppState,
    location: &DrcLocation,
) -> Option<crate::diagnostics::LogAnchor> {
    if let DrcLocation::SymbolPin {
        reference,
        pin_name,
        point,
    } = location
    {
        return Some(crate::diagnostics::LogAnchor::Symbol {
            reference: reference.clone(),
            pin_name: pin_name.clone(),
            point: *point,
        });
    }

    let world = markers::anchor(&super::schematic_design_view(state), location)?;
    let (component, wire) = match location {
        DrcLocation::Component { id, .. } => (Some(*id), None),
        DrcLocation::Wire { id } => (None, Some(*id)),
        _ => (None, None),
    };
    Some(crate::diagnostics::LogAnchor::Schematic {
        x: world.x,
        y: world.y,
        component,
        wire,
    })
}

/// Draw only check results that remain current for the active schematic.
pub(super) fn draw_violation_markers(painter: &Painter, viewport: &Viewport, state: &AppState) {
    let Some(result) = &state.dialogs.drc_results else {
        return;
    };
    // Stale results stay in the dialog/console but vanish from the canvas.
    if state.dialogs.drc_checked_version != state.schematic.topology_version() {
        return;
    }

    markers::draw_violation_markers(
        painter,
        viewport,
        &super::schematic_design_view(state),
        result.violations(),
        state.ui.canvas_hover,
    );
}
