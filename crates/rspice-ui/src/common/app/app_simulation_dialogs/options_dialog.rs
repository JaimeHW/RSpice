use egui::Ui;

use super::super::DialogState;
use super::{Color32, ConsoleMessage, Context, RSpiceApp};

const OPTIONS_TABS: [&str; 5] = [
    "Accuracy",
    "Convergence",
    "Algorithm",
    "Limits",
    "Temperature",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SimulationOptionsPreset {
    Default,
    Fast,
    Accurate,
    Robust,
}

impl SimulationOptionsPreset {
    fn to_options(self) -> crate::simulation::dialog::SimulationOptions {
        match self {
            Self::Default => crate::simulation::dialog::SimulationOptions::default(),
            Self::Fast => crate::simulation::dialog::SimulationOptions::fast(),
            Self::Accurate => crate::simulation::dialog::SimulationOptions::accurate(),
            Self::Robust => crate::simulation::dialog::SimulationOptions::robust(),
        }
    }
}

pub(super) fn apply_options_preset(dialogs: &mut DialogState, preset: SimulationOptionsPreset) {
    dialogs.simulation_options_state =
        crate::simulation::dialog::OptionsDialogState::from_options(&preset.to_options());
    dialogs.simulation_options_errors.clear();
}

pub(super) fn revert_options_dialog_state(dialogs: &mut DialogState) {
    let current = dialogs.simulation_options_config.clone();
    dialogs.simulation_options_state =
        crate::simulation::dialog::OptionsDialogState::from_options(&current);
    dialogs.simulation_options_errors.clear();
}

pub(super) fn parse_and_validate_options(
    dialogs: &mut DialogState,
) -> Option<crate::simulation::dialog::SimulationOptions> {
    let parsed = dialogs.simulation_options_state.to_options();
    match parsed {
        Ok(options) => match options.validate() {
            Ok(()) => Some(options),
            Err(validation_errors) => {
                dialogs.simulation_options_errors = validation_errors
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect();
                None
            }
        },
        Err(parse_errors) => {
            dialogs.simulation_options_errors = parse_errors;
            None
        }
    }
}

pub(super) fn commit_validated_options(
    dialogs: &mut DialogState,
    options: &crate::simulation::dialog::SimulationOptions,
) {
    dialogs.simulation_options_config = options.clone();
    dialogs.simulation_options_state =
        crate::simulation::dialog::OptionsDialogState::from_options(options);
    dialogs.simulation_options_errors.clear();
}

fn render_tab_selector(ui: &mut Ui, active_tab: &mut usize) {
    ui.horizontal_wrapped(|ui| {
        for (idx, tab_name) in OPTIONS_TABS.iter().enumerate() {
            let selected = *active_tab == idx;
            if ui.selectable_label(selected, *tab_name).clicked() {
                *active_tab = idx;
            }
        }
    });
}

fn render_accuracy_tab(ui: &mut Ui, opts: &mut crate::simulation::dialog::OptionsDialogState) {
    ui.heading("Accuracy");
    ui.add_space(6.0);
    egui::Grid::new("sim_opts_accuracy_grid")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("RELTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.reltol).desired_width(140.0));
            ui.end_row();

            ui.label("RESIDUAL_RELTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.residual_reltol).desired_width(140.0));
            ui.end_row();

            ui.label("ABSTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.abstol).desired_width(140.0));
            ui.end_row();

            ui.label("VNTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.vntol).desired_width(140.0));
            ui.end_row();

            ui.label("IABSTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.iabstol).desired_width(140.0));
            ui.end_row();

            ui.label("CHGTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.chgtol).desired_width(140.0));
            ui.end_row();
        });
}

fn render_convergence_tab(ui: &mut Ui, opts: &mut crate::simulation::dialog::OptionsDialogState) {
    ui.heading("Convergence");
    ui.add_space(6.0);
    egui::Grid::new("sim_opts_convergence_grid")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("ITL1");
            ui.add(egui::TextEdit::singleline(&mut opts.itl1).desired_width(140.0));
            ui.end_row();

            ui.label("ITL4");
            ui.add(egui::TextEdit::singleline(&mut opts.itl4).desired_width(140.0));
            ui.end_row();

            ui.label("GMIN");
            ui.add(egui::TextEdit::singleline(&mut opts.gmin).desired_width(140.0));
            ui.end_row();
        });

    ui.add_space(8.0);
    ui.checkbox(&mut opts.gmin_stepping, "Enable GMIN stepping");
    ui.checkbox(&mut opts.source_stepping, "Enable source stepping");
    ui.checkbox(
        &mut opts.pseudo_transient,
        "Enable pseudo-transient continuation",
    );

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("Damping strategy");
        let damping_options = crate::simulation::dialog::DampingStrategy::all();
        let selected = damping_options[opts.damping.min(damping_options.len() - 1)].display_name();
        egui::ComboBox::from_id_salt("sim_opts_damping")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (idx, damping) in damping_options.iter().enumerate() {
                    ui.selectable_value(&mut opts.damping, idx, damping.display_name());
                }
            });
    });
}

fn render_algorithm_tab(ui: &mut Ui, opts: &mut crate::simulation::dialog::OptionsDialogState) {
    ui.heading("Algorithm");
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label("Integration method");
        let methods = crate::simulation::dialog::IntegrationMethod::all();
        let selected = methods[opts.method.min(methods.len() - 1)].display_name();
        egui::ComboBox::from_id_salt("sim_opts_method")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (idx, method) in methods.iter().enumerate() {
                    ui.selectable_value(&mut opts.method, idx, method.display_name());
                }
            });
    });

    ui.horizontal(|ui| {
        ui.label("Matrix solver");
        let solvers = crate::simulation::dialog::MatrixSolver::all();
        let selected = solvers[opts.solver.min(solvers.len() - 1)].display_name();
        egui::ComboBox::from_id_salt("sim_opts_solver")
            .selected_text(selected)
            .show_ui(ui, |ui| {
                for (idx, solver) in solvers.iter().enumerate() {
                    ui.selectable_value(&mut opts.solver, idx, solver.display_name());
                }
            });
    });

    ui.add_space(8.0);
    ui.checkbox(&mut opts.bypass_enabled, "Enable model bypass");
    egui::Grid::new("sim_opts_bypass_grid")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("Bypass RELTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.bypass_reltol).desired_width(140.0));
            ui.end_row();

            ui.label("Bypass ABSTOL");
            ui.add(egui::TextEdit::singleline(&mut opts.bypass_abstol).desired_width(140.0));
            ui.end_row();
        });
}

fn render_limits_tab(ui: &mut Ui, opts: &mut crate::simulation::dialog::OptionsDialogState) {
    ui.heading("Limits");
    ui.add_space(6.0);
    egui::Grid::new("sim_opts_limits_grid")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("Minimum timestep");
            ui.add(egui::TextEdit::singleline(&mut opts.min_timestep).desired_width(140.0));
            ui.end_row();

            ui.label("Maximum timestep");
            ui.add(egui::TextEdit::singleline(&mut opts.max_timestep).desired_width(140.0));
            ui.end_row();

            ui.label("Timestep reduction factor");
            ui.add(egui::TextEdit::singleline(&mut opts.timestep_factor).desired_width(140.0));
            ui.end_row();
        });
}

fn render_temperature_tab(ui: &mut Ui, opts: &mut crate::simulation::dialog::OptionsDialogState) {
    ui.heading("Temperature and Output");
    ui.add_space(6.0);
    egui::Grid::new("sim_opts_temperature_grid")
        .num_columns(2)
        .spacing([20.0, 8.0])
        .show(ui, |ui| {
            ui.label("TEMP (C)");
            ui.add(egui::TextEdit::singleline(&mut opts.temp).desired_width(140.0));
            ui.end_row();

            ui.label("TNOM (C)");
            ui.add(egui::TextEdit::singleline(&mut opts.tnom).desired_width(140.0));
            ui.end_row();
        });

    ui.add_space(8.0);
    ui.checkbox(&mut opts.verbose, "Enable verbose convergence diagnostics");
    ui.checkbox(
        &mut opts.save_internals,
        "Save internal node/branch data when supported",
    );
}

fn render_validation_errors(ui: &mut Ui, dialogs: &DialogState) {
    if dialogs.simulation_options_errors.is_empty() {
        return;
    }

    ui.add_space(8.0);
    ui.group(|ui| {
        ui.label(
            egui::RichText::new("Validation issues")
                .color(Color32::from_rgb(255, 120, 120))
                .strong(),
        );
        for error in &dialogs.simulation_options_errors {
            ui.label(
                egui::RichText::new(format!("- {}", error)).color(Color32::from_rgb(255, 120, 120)),
            );
        }
    });
}

fn render_active_tab(ui: &mut Ui, dialogs: &mut DialogState) {
    match dialogs.simulation_options_state.active_tab {
        0 => render_accuracy_tab(ui, &mut dialogs.simulation_options_state),
        1 => render_convergence_tab(ui, &mut dialogs.simulation_options_state),
        2 => render_algorithm_tab(ui, &mut dialogs.simulation_options_state),
        3 => render_limits_tab(ui, &mut dialogs.simulation_options_state),
        4 => render_temperature_tab(ui, &mut dialogs.simulation_options_state),
        _ => {}
    }

    render_validation_errors(ui, dialogs);
}

fn options_preview_text(dialogs: &DialogState) -> String {
    match dialogs.simulation_options_state.to_options() {
        Ok(options) => options.to_spice_options(),
        Err(_) => dialogs.simulation_options_config.to_spice_options(),
    }
}

impl RSpiceApp {
    pub(in crate::common::app) fn render_simulation_options_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.simulation_options {
            return;
        }

        let mut dialog_open = self.state.dialogs.simulation_options;
        let mut close_requested = false;

        egui::Window::new("Simulation Options")
            .open(&mut dialog_open)
            .collapsible(false)
            .resizable(true)
            .default_width(760.0)
            .default_height(580.0)
            .min_width(680.0)
            .min_height(320.0)
            .show(ctx, |ui| {
                ui.label("Configure numerical accuracy, convergence, and algorithm behavior.");
                ui.label(egui::RichText::new("Changes apply to all new simulation runs.").weak());
                ui.add_space(6.0);

                render_tab_selector(
                    ui,
                    &mut self.state.dialogs.simulation_options_state.active_tab,
                );

                ui.separator();
                ui.add_space(4.0);

                let preview_text = options_preview_text(&self.state.dialogs);
                let preview_height = 170.0;
                let footer_height = 60.0;
                let content_height =
                    (ui.available_height() - preview_height - footer_height).max(0.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), content_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("sim_options_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                render_active_tab(ui, &mut self.state.dialogs);
                            });
                    },
                );

                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("SPICE .OPTIONS preview").strong());
                    ui.add_space(4.0);
                    let mut preview_buffer = preview_text;
                    ui.add_enabled(
                        false,
                        egui::TextEdit::multiline(&mut preview_buffer)
                            .font(egui::TextStyle::Monospace)
                            .desired_rows(6)
                            .desired_width(f32::INFINITY),
                    );
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    if ui.button("Default").clicked() {
                        apply_options_preset(
                            &mut self.state.dialogs,
                            SimulationOptionsPreset::Default,
                        );
                    }
                    if ui.button("Fast").clicked() {
                        apply_options_preset(
                            &mut self.state.dialogs,
                            SimulationOptionsPreset::Fast,
                        );
                    }
                    if ui.button("Accurate").clicked() {
                        apply_options_preset(
                            &mut self.state.dialogs,
                            SimulationOptionsPreset::Accurate,
                        );
                    }
                    if ui.button("Robust").clicked() {
                        apply_options_preset(
                            &mut self.state.dialogs,
                            SimulationOptionsPreset::Robust,
                        );
                    }
                    if ui.button("Revert").clicked() {
                        revert_options_dialog_state(&mut self.state.dialogs);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ok_clicked = ui.button("OK").clicked();
                        let apply_clicked = ui.button("Apply").clicked();
                        if ui.button("Cancel").clicked() {
                            close_requested = true;
                        }

                        if apply_clicked || ok_clicked {
                            if let Some(options) =
                                parse_and_validate_options(&mut self.state.dialogs)
                            {
                                commit_validated_options(&mut self.state.dialogs, &options);
                                self.state.push_user_message(ConsoleMessage::info(
                                    "Simulation options updated",
                                ));
                                if ok_clicked {
                                    close_requested = true;
                                }
                            }
                        }
                    });
                });
            });

        self.state.dialogs.simulation_options = dialog_open && !close_requested;
    }
}
