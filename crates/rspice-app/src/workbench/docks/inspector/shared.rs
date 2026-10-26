//! What more than one inspector section draws or reads alike: the action row
//! that closes a section, a counted noun, and the checks of the drawing in
//! front of the reader, which every section that inspects the drawing reports
//! the same way.

use egui::Ui;
use rspice_design::drc::{DrcSeverity, DrcViolation};
use rspice_ui_kit::panels::{Problem, panel_inset, section_block_gap};

use crate::workbench::app_state::Model;

/// The section body's action row: content-sized buttons that wrap, matching
/// the mockup's `.section-body.panel-action-stack` box and the design
/// system's own rule that an action row is a group, not a run-on.
///
/// The row takes the section's own step from the block above it and leaves
/// the closing step to the section, so a body that ends in buttons is framed
/// exactly like one that ends in a property list. It starts and stops on the
/// panel inset, like the rows above it.
const ACTION_ROW_GAP: f32 = 6.0;

pub fn action_stack(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    let inset = panel_inset(ui);
    let width = (ui.available_width() - 2.0 * inset).max(1.0);
    section_block_gap(ui);
    // Top-aligned: a centred row is a control's height tall before anything
    // is in it, and centred the wrapping group half that far down.
    ui.horizontal_top(|ui| {
        ui.add_space(inset);
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
            |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(ACTION_ROW_GAP, ACTION_ROW_GAP);
                body(ui);
            },
        );
    });
}

/// `count` things, as a row's value states them: "1 finding", "3 findings".
pub fn count_label(count: usize, noun: &str) -> String {
    format!("{count} {noun}{}", if count == 1 { "" } else { "s" })
}

/// The findings a check of the drawing as it stands reported for whatever
/// `targets` picks out. A check run against an earlier drawing reports none:
/// its findings may name objects that have since moved or gone.
pub fn current_findings(
    model: &Model,
    targets: impl Fn(&DrcViolation) -> bool,
) -> Vec<&DrcViolation> {
    if !checks_current(model) {
        return Vec::new();
    }
    model
        .design_checks
        .drc_results
        .as_ref()
        .map(|result| {
            result
                .violations()
                .iter()
                .filter(|violation| targets(violation))
                .collect()
        })
        .unwrap_or_default()
}

/// Whether a finding is about this drawing rather than about a cellview it
/// places.
fn in_this_drawing(violation: &DrcViolation) -> bool {
    !matches!(
        violation.location,
        rspice_design::drc::DrcLocation::InsideInstance { .. }
    )
}

/// The open terminals the sheet's Connectivity row counts.
///
/// All three findings are counted: an undriven input and an unloaded output
/// are unconnected pins too, and a row that quietly omitted them would
/// understate the sheet. Only this drawing's own are: a finding inside a
/// placed cell is stated once per occurrence, so counting those would make a
/// clean sheet that places one cell eight times report eight times that
/// cell's open pins as its own.
pub fn sheet_open_pin_findings(model: &Model) -> Vec<&DrcViolation> {
    current_findings(model, |violation| {
        in_this_drawing(violation)
            && matches!(
                violation.violation_type,
                rspice_design::drc::DrcViolationType::UnconnectedPin
                    | rspice_design::drc::DrcViolationType::UndrivenInput
                    | rspice_design::drc::DrcViolationType::UnloadedOutput
            )
    })
}

/// The floating nodes the sheet's row counts: this drawing's own, for the
/// reason [`sheet_open_pin_findings`] gives.
pub fn sheet_floating_net_findings(model: &Model) -> Vec<&DrcViolation> {
    current_findings(model, |violation| {
        in_this_drawing(violation)
            && violation.violation_type == rspice_design::drc::DrcViolationType::FloatingNode
    })
}

/// The problem a set of findings amounts to: a failure when any of them is
/// an error that counts, else a warning. A waived error is still listed, but
/// it no longer counts against the design — the Checks header and the run
/// gate do not count it — so it is not a failure here either.
pub fn findings_problem(findings: &[&DrcViolation]) -> Problem {
    if findings
        .iter()
        .any(|finding| finding.counts() && finding.severity >= DrcSeverity::Error)
    {
        Problem::Failure
    } else {
        Problem::Warning
    }
}

/// `true` when schematic checks were run against the drawing as it stands.
pub fn checks_current(model: &Model) -> bool {
    model.design_checks.drc_results.is_some()
        && model.design_checks.drc_checked_version == model.schematic.topology_version()
}

/// What the Checks header says, only when the check is not current and clean:
/// "not run" before any check, "stale" once the drawing has changed under
/// one, and the error count of a current check that found errors.
pub fn checks_status(model: &Model) -> Option<String> {
    let Some(result) = model.design_checks.drc_results.as_ref() else {
        return Some("not run".to_owned());
    };
    if !checks_current(model) {
        return Some("stale".to_owned());
    }
    let summary = result.summary();
    let blocking = summary.critical + summary.errors;
    (blocking > 0).then(|| count_label(blocking, "error"))
}
