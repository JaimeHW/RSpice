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
///
/// The finding it lands on becomes *the* current finding — the same selection
/// the Problems grid draws its current row from, held by canonical identity in
/// the diagnostic store. It used to be an index into the list this function
/// sorts for itself, which no other surface could read and which a
/// republished batch silently repointed.
pub(crate) fn cycle_violation(state: &mut AppState, step: isize) {
    // A symbol view's findings are the pin checks', which address a pin on a
    // symbol rather than a spot in a drawing — and they are published as
    // records, so the tour reads them from the store and names each one by
    // the identity the grid names it by.
    if state.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
        if !state.cycle_active_symbol_pin_finding(step) {
            state.push_user_message(ConsoleMessage::info(
                "No symbol pin findings — run the pin checks first",
            ));
        }
        return;
    }
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

    // What the published findings are called, in the order the result holds
    // them. Read rather than derived here: the store is where a finding's
    // identity is minted, and a second derivation of it is a second identity.
    let identities = state.canvas_finding_identities();
    // Owned snapshot of the anchored findings so the borrows drop before
    // the selection/viewport mutations below.
    let mut anchored: Vec<AnchoredFinding> = result
        .violations()
        .iter()
        .enumerate()
        .filter_map(|(ordinal, violation)| {
            finding_anchor(state, violation).map(|anchor| AnchoredFinding {
                severity: violation.severity,
                identity: identities.get(ordinal).copied(),
                anchor,
                message: violation.message.clone(),
            })
        })
        .collect();
    if anchored.is_empty() {
        state.push_user_message(ConsoleMessage::info(
            "No findings with a canvas anchor — see the console summary",
        ));
        return;
    }
    // Worst first; stable within a severity.
    anchored.sort_by(|a, b| b.severity.cmp(&a.severity));

    let len = anchored.len() as isize;
    // Where the tour stands is wherever the current finding sits in it. A
    // selection made on the grid — or on a finding this tour cannot visit,
    // such as a netlist error — therefore does not strand the tour: it starts
    // from the worst finding, which is where it starts from a cold session.
    let standing = state.diagnostics.current_finding().and_then(|current| {
        anchored
            .iter()
            .position(|found| found.identity == Some(current))
    });
    let index = match standing {
        // First jump lands on the worst finding, not the second one.
        None if step >= 0 => 0,
        None => (len - 1) as usize,
        Some(current) => ((current as isize + step).rem_euclid(len)) as usize,
    };

    let found = anchored.swap_remove(index);
    // A finding with no published record leaves nothing current rather than
    // leaving the previous finding standing, which the tour has just left.
    state.diagnostics.set_current_finding(found.identity);
    state.jump_to_log_anchor(found.anchor);
    state.push_user_message(ConsoleMessage::info(format!(
        "Finding {}/{} — {} · {}",
        index + 1,
        len,
        found.severity.display_name(),
        found.message,
    )));
}

/// One finding the tour can visit: what it is called, how bad it is, where it
/// is, and what it says.
struct AnchoredFinding {
    severity: DrcSeverity,
    /// What the store calls it, where the checks published a record for it.
    identity: Option<uuid::Uuid>,
    anchor: crate::diagnostics::LogAnchor,
    message: String,
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
    state: &AppState,
    location: &DrcLocation,
) -> Option<crate::diagnostics::LogAnchor> {
    location_anchor_in(
        state,
        &state.workspace.content.active_schematic_reference(),
        location,
    )
}

/// Where one finding location in `cell_view` points, when it points anywhere.
///
/// The drawing in front of the reader is asked through its live editor, and
/// any other cell view through the working copy the workspace holds for it,
/// so a finding about a view that is not open still says where it is. The
/// anchor carries the view, and following it opens that view first.
pub(crate) fn location_anchor_in(
    state: &AppState,
    cell_view: &crate::state::CellViewRef,
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

    let key = cell_view.key();
    let world =
        if key.eq_ignore_ascii_case(&state.workspace.content.active_schematic_reference().key()) {
            markers::anchor(&super::schematic_design_view(state), location)?
        } else {
            let (_, schematic) = state
                .workspace
                .content
                .schematic_buffers
                .iter()
                .find(|(buffer, _)| buffer.eq_ignore_ascii_case(&key))?;
            let view = rspice_schematic_editor::view::design_view::DesignView {
                document: schematic.document(),
                canvas_cache: None,
                sheet_catalog: state
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&key),
                review_markers: state.ui.schematic_visibility.review_markers,
            };
            markers::anchor(&view, location)?
        };
    let (component, wire) = match location {
        DrcLocation::Component { id, .. } => (Some(*id), None),
        DrcLocation::Wire { id } => (None, Some(*id)),
        _ => (None, None),
    };
    Some(crate::diagnostics::LogAnchor::Schematic {
        cell_view: cell_view.clone(),
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
