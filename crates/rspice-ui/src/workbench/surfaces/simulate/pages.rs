//! Setup-page routing and the shared page heading.
//!
//! The analyses page owns the ordered plan; every other route owns exactly one
//! plan-scoped concern. Routing lives here so the surface entry point stays a
//! scroll host and each page states only what it owns.

use egui::Ui;

use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;
use crate::workbench::state::SimulationPage;

use super::super::super::design_system::{heading, workspace_title_row};
use super::page_kit::setup_page;

/// Render the selected setup page. The analyses route is handled by the
/// surface itself and never reaches here.
pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp, page: SimulationPage) {
    workspace_title_row(ui, |ui| page_heading(ui, app, page));
    setup_page(ui, |ui| match page {
        SimulationPage::Analyses => {}
        SimulationPage::Variables => super::page_variables::show(ui, app),
        SimulationPage::Outputs => super::page_outputs::show(ui, app),
        SimulationPage::Specifications => super::page_specs::show(ui, app),
        SimulationPage::RunSet => super::page_runset::show(ui, app),
        SimulationPage::Models => super::page_models::show(ui, app),
        SimulationPage::Solver => super::page_solver::show(ui, app),
        SimulationPage::Save => super::page_save::show(ui, app),
    });
}

/// Eyebrow, title, description, and the page's own primary action.
///
/// The eyebrow states the page's authority in the plan, derived from the same
/// state the page edits — never a static caption.
fn page_heading(ui: &mut Ui, app: &mut RSpiceApp, page: SimulationPage) {
    let eyebrow = eyebrow(app, page);
    ui.horizontal(|ui| {
        let available = ui.available_width();
        let action = primary_action(page);
        let action_width = if action.is_some() { 190.0 } else { 0.0 };
        ui.allocate_ui_with_layout(
            egui::vec2((available - action_width).max(1.0), 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                heading(ui, &eyebrow, page.title(), page.description());
            },
        );
        if let Some((label, command)) = action {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if Button::new(label).accent().show(ui).clicked() {
                    command(app);
                }
            });
        }
    });
}

fn eyebrow(app: &RSpiceApp, page: SimulationPage) -> String {
    let plan = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.instances().len())
        .unwrap_or_default();
    match page {
        SimulationPage::Analyses => format!("PLAN · {plan} ANALYSES"),
        SimulationPage::Variables => "PARAMETERIZATION · PLAN SCOPE".to_owned(),
        SimulationPage::Outputs => "SAVED SIGNALS · EXPRESSIONS".to_owned(),
        SimulationPage::Specifications => "REQUIREMENTS · ACCEPTANCE LIMITS".to_owned(),
        SimulationPage::RunSet => "RUN SPACE · PROCESS · TEMPERATURE · VARIATION".to_owned(),
        SimulationPage::Models => "MODEL CLOSURE · SECTION BINDING · QUALIFICATION".to_owned(),
        SimulationPage::Solver => format!(
            "NUMERICS · {} · DESIGN CONTRACT",
            app.state.sim_setup.options.preset_label().to_uppercase()
        ),
        SimulationPage::Save => "RESULT STORAGE · STREAMING · RETENTION".to_owned(),
    }
}

type PageCommand = fn(&mut RSpiceApp);

fn primary_action(page: SimulationPage) -> Option<(&'static str, PageCommand)> {
    match page {
        SimulationPage::Variables => Some(("Add variable…", open_variable_dialog)),
        SimulationPage::Outputs => Some(("Add output…", open_output_dialog)),
        SimulationPage::Analyses
        | SimulationPage::Specifications
        | SimulationPage::RunSet
        | SimulationPage::Models
        | SimulationPage::Solver
        | SimulationPage::Save => None,
    }
}

fn open_variable_dialog(app: &mut RSpiceApp) {
    app.state.workbench.simulation_workflow = Some(
        crate::workbench::state::SimulationWorkflowDialog::DesignVariable(
            crate::workbench::state::DesignVariableDraft::default(),
        ),
    );
}

fn open_output_dialog(app: &mut RSpiceApp) {
    app.state.workbench.simulation_workflow = Some(
        crate::workbench::state::SimulationWorkflowDialog::SavedOutput(
            crate::workbench::state::SavedOutputDraft::default(),
        ),
    );
}
