//! What the inspector says about the analysis a reader has selected.
//!
//! Two sections: the analysis itself — what it is called, what kind it is,
//! what it is configured to do — and the execution context every analysis in
//! the plan resolves against. Both are read-only reports over the stable plan
//! and the session, so this module borrows the application once and renders,
//! rather than owning any part of the plan's authority.

use egui::Ui;

use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;

use super::super::super::design_system::{StatusMark, property_row, property_row_status};
use super::section_header;

/// What this instance's relationship to the run set actually is.
///
/// Enabled/Excluded is only half of it: an enabled instance also declares *how
/// much* of the declared space it visits, and that declaration is a control the
/// Analyses page now offers. Reporting only the enabled bit made an instance
/// scoped to the nominal point read exactly like one running the whole matrix.
/// The wording is [`AnalysisRunAt::label`]'s, which is the spelling the control
/// and the receipt both use.
fn run_set_participation(instance: &crate::simulation::plan::AnalysisInstance) -> String {
    if instance.enabled() {
        format!("Enabled \u{00b7} {}", instance.run_at().label())
    } else {
        "Excluded".to_owned()
    }
}

pub(super) fn simulate(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            let legacy_kind = crate::simulation::plan::AnalysisKind::from_legacy_index(
                app.state.workbench.active_analysis,
            );
            plan.instances()
                .iter()
                .find(|instance| {
                    Some(instance.id()) == app.state.workbench.active_analysis_instance
                })
                .or_else(|| {
                    legacy_kind.and_then(|kind| {
                        plan.instances()
                            .iter()
                            .find(|instance| instance.kind() == kind)
                    })
                })
                .or_else(|| plan.instances().first())
                .map(|instance| {
                    (
                        instance.id(),
                        instance.kind(),
                        instance.display_name().to_owned(),
                        instance.draft().clone(),
                        run_set_participation(instance),
                        instance.dependencies().len(),
                        instance.modified_revision(),
                    )
                })
        });

    section_header(ui, "Selected analysis", None);
    let Some((id, kind, name, draft, run_set, dependency_count, revision)) = selected else {
        property_row(ui, "Selection", "No analysis instances in this plan");
        return;
    };
    app.state.workbench.active_analysis_instance = Some(id);
    app.state.workbench.active_analysis = kind.legacy_index();
    // Name over kind over identity: three rows that were two, because in a plan
    // holding three transients the kind was not telling the reader which one
    // the rest of this section is describing.
    property_row(ui, "Name", &name);
    property_row(ui, "Type", kind.label());
    property_row(ui, "Instance", &id.to_string());
    property_row(
        ui,
        "Configuration",
        &app.state.sim_setup.analysis_draft_summary(&draft),
    );
    property_row(ui, "Run set", &run_set);
    property_row(ui, "Revision", &revision.get().to_string());
    property_row(ui, "Prerequisites", &dependency_count.to_string());
    if let Some(error) = app.state.sim_setup.analysis_draft_validation_error(&draft) {
        property_row_status(
            ui,
            "Validation",
            &error,
            Tokens::get(ui.ctx()).color.err,
            StatusMark::Failure,
        );
    } else {
        property_row_status(
            ui,
            "Validation",
            "Configuration valid",
            Tokens::get(ui.ctx()).color.ok,
            StatusMark::Success,
        );
    }
    section_header(ui, "Execution context", None);
    property_row(
        ui,
        "Corner",
        app.state.sim_setup.reference_pvt.process.short_name(),
    );
    property_row(
        ui,
        "Temperature",
        &format!(
            "{} °C",
            app.state.sim_setup.reference_pvt.temperature_celsius
        ),
    );
    property_row(
        ui,
        "Enabled analyses",
        &app.state
            .sim_setup
            .enabled_analysis_instance_count()
            .to_string(),
    );
    property_row(
        ui,
        "Engine status",
        if app.state.simulation.cancellation_is_pending() {
            "Stopping"
        } else if matches!(
            app.state.simulation.active_execution_lifecycle(),
            Some(crate::state::SimulationRunLifecycle::Preparing)
        ) {
            "Preparing"
        } else if app.state.simulation.has_active_execution() {
            "Running"
        } else {
            "Ready"
        },
    );
}
