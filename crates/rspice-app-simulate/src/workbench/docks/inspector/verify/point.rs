//! The Corners cell the reader selected: the conditions of its point, then
//! what that point gave the specification and the analysis it came from.

use egui::Ui;
use rspice_ui_kit::panels::{Problem, section_header};

use crate::workbench::app_state::Model;
use crate::workbench::design_system::property_row;
use crate::workbench::state::VerificationSessionState;
use crate::workbench::surfaces::verify::{FactSection, point_facts};

/// Each section's title and its label and value rows.
pub(super) fn facts(ui: &mut Ui, sections: &[FactSection]) {
    for section in sections {
        section_header(ui, &section.title, None);
        for (label, value) in &section.rows {
            property_row(ui, label, value);
        }
    }
}

pub(super) fn show(ui: &mut Ui, model: &Model, session: &VerificationSessionState) {
    let failed = model.simulation.active_run().and_then(|run| {
        run.analyses.iter().find(|analysis| {
            analysis.analysis_type == crate::state::AnalysisType::Corner && !analysis.success
        })
    });
    if let Some(analysis) = failed {
        let message = analysis.error_message.as_deref().map_or_else(
            || "The corner run failed.".to_owned(),
            |error| format!("The corner run failed: {error}"),
        );
        crate::ui::widgets::problem_line(ui, Problem::Failure, &message);
    }
    facts(ui, &point_facts(model, session));
}
