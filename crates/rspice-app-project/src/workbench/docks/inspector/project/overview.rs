//! What the inspector beside the Project Overview adds under the project:
//! the deck a netlist-first project runs, or the design checks' findings.

use egui::Ui;
use rspice_ui_kit::panels::section_header;

use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::design_system::{property_row, property_row_problem, property_row_wrapped};
use crate::workbench::project::health::{self, DesignSubject};

use super::ProjectInspector;

/// A netlist-first project's deck and whether it checks; otherwise the
/// findings themselves, every one of them, each whole: the page's status row
/// states the checks' counts, and the panel scrolls.
pub(super) fn overview_findings(ui: &mut Ui, inspector: &ProjectInspector<'_>) {
    let model = inspector.model;
    if DesignSubject::of(model) == DesignSubject::SourceDeck {
        let deck = health::source_deck(model);
        section_header(ui, "Source deck", None);
        property_row(
            ui,
            "Deck",
            &health::deck_name(model).unwrap_or_else(|| "Unavailable".to_owned()),
        );
        property_row_problem(ui, "Validation", &deck.state, deck.tone.problem());
        return;
    }
    let checks = health::schematic_checks(model, inspector.facts);
    section_header(ui, "Problems", Some(&checks.state));
    match model.simulation_root_design_check_status(inspector.facts) {
        DesignCheckStatus::Current(receipt) => {
            for violation in receipt.result.violations() {
                property_row_wrapped(ui, "Finding", &violation.message);
            }
        }
        DesignCheckStatus::Unavailable { reason, .. } => {
            property_row_wrapped(ui, "Checks", &reason);
        }
        DesignCheckStatus::NotRun | DesignCheckStatus::Stale(_) => {}
    }
}
