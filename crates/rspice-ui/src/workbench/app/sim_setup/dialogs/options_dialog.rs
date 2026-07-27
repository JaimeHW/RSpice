//! Simulation Options — engine numerics on the modal primitive: underline
//! tabs, preset chips, draft buffers with validation, and a live
//! `.OPTIONS` preview in an inset mono well.
//!
//! The dialog edits `SimSetupState::options_draft` and commits to
//! `SimSetupState::options` on OK/Apply — cancel never mutates the
//! effective options.

use egui::{Context, Ui};

use super::{ConsoleMessage, RSpiceApp};
use crate::workbench::app::SimSetupState;
use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::simulation::dialog::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogSize, check_row, chip, choice_row, dialog_tabs, input_row,
};

const TABS: &[&str] = &[
    "Accuracy",
    "Convergence",
    "Algorithm",
    "Limits",
    "Temperature",
];

/// Parse the draft and validate; on failure the errors land in
/// `options_errors` and `None` is returned.
fn parse_and_validate_options(setup: &mut SimSetupState) -> Option<SimulationOptions> {
    match setup.options_draft.to_options() {
        Ok(options) => match options.validate() {
            Ok(()) => {
                setup.options_errors.clear();
                Some(options)
            }
            Err(validation_errors) => {
                setup.options_errors = validation_errors
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect();
                None
            }
        },
        Err(parse_errors) => {
            setup.options_errors = parse_errors;
            None
        }
    }
}

/// Make `options` effective and refresh the draft from it.
fn commit_validated_options(setup: &mut SimSetupState, options: &SimulationOptions) {
    setup.commit_options(options);
    let active_tab = setup.options_draft.active_tab;
    setup.options_draft = OptionsDialogState::from_options(options);
    setup.options_draft.active_tab = active_tab;
    setup.options_errors.clear();
}

fn commit_options_transaction(
    app: &mut RSpiceApp,
    options: &SimulationOptions,
    close_dialog: bool,
) -> Result<bool, String> {
    let current_bytes = serde_json::to_vec(&app.state.sim_setup.options)
        .map_err(|error| format!("Could not compare the current solver options: {error}"))?;
    let requested_bytes = serde_json::to_vec(options)
        .map_err(|error| format!("Could not compare the requested solver options: {error}"))?;
    if current_bytes == requested_bytes {
        if close_dialog {
            app.state.sim_setup.options_open = false;
        }
        return Ok(false);
    }
    let mut candidate = app.state.sim_setup.clone();
    commit_validated_options(&mut candidate, options);
    let receipt = candidate
        .commit_active_plan_configuration_change("Updated simulation solver options.")
        .map_err(|error| error.to_string())?;
    if close_dialog {
        candidate.options_open = false;
    }
    app.state.sim_setup = candidate;
    app.invalidate_simulation_preflight();
    app.state.workbench.analysis_lifecycle_status = format!(
        "Configuration receipt #{} · revision {} to {} · {}",
        receipt.sequence(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.detail()
    );
    Ok(true)
}

/// Preset chips — each replaces the whole draft.
fn presets_row(ui: &mut Ui, setup: &mut SimSetupState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        let presets: [(&str, SimulationOptions); 4] = [
            ("default", SimulationOptions::default()),
            ("fast", SimulationOptions::fast()),
            ("accurate", SimulationOptions::accurate()),
            ("robust", SimulationOptions::robust()),
        ];
        for (label, options) in presets {
            if chip(ui, label, false).clicked() {
                let active_tab = setup.options_draft.active_tab;
                setup.options_draft = OptionsDialogState::from_options(&options);
                setup.options_draft.active_tab = active_tab;
                setup.options_errors.clear();
            }
        }
    });
}

fn accuracy_tab(ui: &mut Ui, opts: &mut OptionsDialogState) {
    input_row(ui, "RELTOL", &mut opts.reltol);
    input_row(ui, "Residual rel", &mut opts.residual_reltol);
    input_row(ui, "ABSTOL", &mut opts.abstol);
    input_row(ui, "VNTOL", &mut opts.vntol);
    input_row(ui, "IABSTOL", &mut opts.iabstol);
    input_row(ui, "CHGTOL", &mut opts.chgtol);
}

fn convergence_tab(ui: &mut Ui, opts: &mut OptionsDialogState) {
    input_row(ui, "ITL1 (DC)", &mut opts.itl1);
    input_row(ui, "ITL4 (tran)", &mut opts.itl4);
    input_row(ui, "GMIN", &mut opts.gmin);
    check_row(ui, "GMIN stepping", &mut opts.gmin_stepping);
    check_row(ui, "Source stepping", &mut opts.source_stepping);
    check_row(ui, "Pseudo-transient", &mut opts.pseudo_transient);
    let names: Vec<&str> = DampingStrategy::all()
        .iter()
        .map(|strategy| strategy.display_name())
        .collect();
    choice_row(ui, "Damping", &names, &mut opts.damping);
}

fn algorithm_tab(ui: &mut Ui, opts: &mut OptionsDialogState) {
    let methods: Vec<&str> = IntegrationMethod::all()
        .iter()
        .map(|method| method.display_name())
        .collect();
    choice_row(ui, "Integration", &methods, &mut opts.method);
    let solvers: Vec<&str> = MatrixSolver::all()
        .iter()
        .map(|solver| solver.display_name())
        .collect();
    choice_row(ui, "Solver", &solvers, &mut opts.solver);
    check_row(ui, "Model bypass", &mut opts.bypass_enabled);
    if opts.bypass_enabled {
        input_row(ui, "Bypass rel", &mut opts.bypass_reltol);
        input_row(ui, "Bypass abs", &mut opts.bypass_abstol);
    }
}

fn typed_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    kind: QuantityInputKind,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    let response = input_row(ui, label, value);
    if response.lost_focus()
        && let Ok(parsed) = parse_ui_quantity(value, kind, policy, locale)
    {
        let schema_value = if kind == QuantityInputKind::Temperature {
            parsed - 273.15
        } else {
            parsed
        };
        *value = format!("{schema_value:.17e}");
    }
}

fn limits_tab(
    ui: &mut Ui,
    opts: &mut OptionsDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    typed_input_row(
        ui,
        "Min step",
        &mut opts.min_timestep,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    typed_input_row(
        ui,
        "Max step",
        &mut opts.max_timestep,
        QuantityInputKind::Time,
        policy,
        locale,
    );
    input_row(ui, "Cut factor", &mut opts.timestep_factor);
}

fn temperature_tab(
    ui: &mut Ui,
    opts: &mut OptionsDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    typed_input_row(
        ui,
        "TEMP",
        &mut opts.temp,
        QuantityInputKind::Temperature,
        policy,
        locale,
    );
    typed_input_row(
        ui,
        "TNOM",
        &mut opts.tnom,
        QuantityInputKind::Temperature,
        policy,
        locale,
    );
    check_row(ui, "Verbose diagnostics", &mut opts.verbose);
    check_row(ui, "Save internal nodes", &mut opts.save_internals);
}

/// The `.OPTIONS` preview in an inset mono well — live from the draft,
/// falling back to the effective options while the draft is unparseable.
fn options_preview(ui: &mut Ui, setup: &SimSetupState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let text = match setup.options_draft.to_options() {
        Ok(options) => options.to_spice_options(),
        Err(_) => setup.options.to_spice_options(),
    };

    let mut header = egui::text::LayoutJob::default();
    header.append(
        ".OPTIONS PREVIEW",
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: c.text_faint,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(header);
    ui.add_space(4.0);
    egui::Frame::NONE
        .fill(c.bg_inset)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_dim),
            );
        });
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_simulation_options_dialog(&mut self, ctx: &Context) {
        if !self.state.sim_setup.options_open {
            return;
        }

        let error_count = self.state.sim_setup.options_errors.len();
        let hint = if error_count > 0 {
            format!(
                "{error_count} issue{} to fix",
                if error_count == 1 { "" } else { "s" }
            )
        } else {
            "applies to new runs".to_owned()
        };

        let choice = {
            let quantity_policy = self.state.ui.preferences.quantity_presentation_policy();
            let number_locale = self.state.ui.number_locale;
            let setup = &mut self.state.sim_setup;
            Dialog::new("Simulate", "Simulation options", "OK")
                .description(
                    "Review and validate accuracy, convergence, algorithm, limits, and temperature options applied to new simulation runs.",
                )
                .size(DialogSize::WideWorkflow)
                .secondary("Apply")
                .ghost("Revert")
                .hint(&hint)
                .show(ctx, |ui| {
                    let t = Tokens::get(ui.ctx());
                    let c = t.color;
                    ui.spacing_mut().item_spacing.y = 2.0;

                    presets_row(ui, setup);
                    ui.add_space(8.0);

                    let mut active = setup.options_draft.active_tab;
                    dialog_tabs(ui, TABS, &mut active);
                    setup.options_draft.active_tab = active;
                    match active {
                        0 => accuracy_tab(ui, &mut setup.options_draft),
                        1 => convergence_tab(ui, &mut setup.options_draft),
                        2 => algorithm_tab(ui, &mut setup.options_draft),
                        3 => limits_tab(
                            ui,
                            &mut setup.options_draft,
                            quantity_policy,
                            number_locale,
                        ),
                        _ => temperature_tab(
                            ui,
                            &mut setup.options_draft,
                            quantity_policy,
                            number_locale,
                        ),
                    }

                    if !setup.options_errors.is_empty() {
                        ui.add_space(8.0);
                        for error in &setup.options_errors {
                            ui.label(
                                egui::RichText::new(error.as_str())
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(c.err),
                            );
                        }
                    }

                    ui.add_space(12.0);
                    options_preview(ui, setup);
                })
        };

        match choice {
            DialogChoice::Primary | DialogChoice::Secondary => {
                if let Some(options) = parse_and_validate_options(&mut self.state.sim_setup) {
                    match commit_options_transaction(
                        self,
                        &options,
                        choice == DialogChoice::Primary,
                    ) {
                        Ok(true) => self
                            .state
                            .push_user_message(ConsoleMessage::info("Simulation options updated")),
                        Ok(false) => {}
                        Err(error) => self
                            .state
                            .sim_setup
                            .options_errors
                            .push(format!("Options were not committed: {error}")),
                    }
                }
            }
            DialogChoice::Ghost => {
                let setup = &mut self.state.sim_setup;
                let current = setup.options.clone();
                let active_tab = setup.options_draft.active_tab;
                setup.options_draft = OptionsDialogState::from_options(&current);
                setup.options_draft.active_tab = active_tab;
                setup.options_errors.clear();
            }
            DialogChoice::Cancelled => {
                self.state.sim_setup.options_open = false;
            }
            DialogChoice::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_options_commit_advances_the_plan_and_invalidates_preflight() {
        let mut app = RSpiceApp::test_instance();
        let (plan_id, source_revision) = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| (plan.id(), plan.revision()))
            .expect("default plan");
        let topology_root = app.state.workspace.simulation_root_reference().key();
        let topology_revision = app.state.schematic.topology_version();
        let topology_closure = vec![(topology_root.to_ascii_lowercase(), topology_revision)];
        app.state.workbench.preflight.report = Some(crate::workbench::state::PreflightReport {
            project_revision: app.state.workspace.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: Some(plan_id),
            simulation_plan_revision: Some(source_revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        });
        let mut options = app.state.sim_setup.options.clone();
        options.reltol *= 2.0;

        assert!(
            commit_options_transaction(&mut app, &options, false)
                .expect("validated options commit atomically")
        );

        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            source_revision.next().expect("revision advances")
        );
        assert_eq!(app.state.sim_setup.options.reltol, options.reltol);
        assert!(app.state.workbench.preflight.report.is_none());
    }

    #[test]
    fn unchanged_solver_options_do_not_fabricate_a_plan_revision() {
        let mut app = RSpiceApp::test_instance();
        let revision = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .revision();
        let options = app.state.sim_setup.options.clone();

        assert!(!commit_options_transaction(&mut app, &options, false).expect("no-op succeeds"));
        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            revision
        );
    }
}
