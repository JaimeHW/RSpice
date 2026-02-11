use super::{ConsoleMessage, RSpiceApp};
use egui::{Color32, Context};
use std::collections::HashSet;

use crate::common::simulation_analysis_tabs::SIMULATION_ANALYSIS_CATEGORIES;

#[path = "app_simulation_analysis_options.rs"]
mod app_simulation_analysis_options;

fn set_default_if_blank(field: &mut String, default: &str) {
    if field.trim().is_empty() {
        field.clear();
        field.push_str(default);
    }
}

fn toggle_enabled_analysis(enabled_analyses: &mut HashSet<usize>, index: usize) {
    if !enabled_analyses.insert(index) {
        enabled_analyses.remove(&index);
    }
}

impl RSpiceApp {
    pub(super) fn ensure_simulation_setup_defaults(&mut self) {
        // Transient
        set_default_if_blank(&mut self.state.dialogs.tran_stop, "1m");
        set_default_if_blank(&mut self.state.dialogs.tran_step, "10n");
        set_default_if_blank(&mut self.state.dialogs.tran_start, "0");
        set_default_if_blank(&mut self.state.dialogs.tran_maxstep, "auto");
        // AC
        set_default_if_blank(&mut self.state.dialogs.ac_fstart, "1");
        set_default_if_blank(&mut self.state.dialogs.ac_fstop, "1G");
        set_default_if_blank(&mut self.state.dialogs.ac_points, "101");
        set_default_if_blank(&mut self.state.dialogs.disto_f2_over_f1, "2.0");
        // DC
        set_default_if_blank(&mut self.state.dialogs.dc_source, "V1");
        set_default_if_blank(&mut self.state.dialogs.dc_start, "0");
        set_default_if_blank(&mut self.state.dialogs.dc_stop, "5");
        set_default_if_blank(&mut self.state.dialogs.dc_step, "0.01");
        set_default_if_blank(&mut self.state.dialogs.dc_source2, "V2");
        set_default_if_blank(&mut self.state.dialogs.dc_start2, "0");
        set_default_if_blank(&mut self.state.dialogs.dc_stop2, "3.3");
        set_default_if_blank(&mut self.state.dialogs.dc_step2, "0.1");
        // Noise
        set_default_if_blank(&mut self.state.dialogs.noise_output, "out");
        set_default_if_blank(&mut self.state.dialogs.noise_ref, "0");
        set_default_if_blank(&mut self.state.dialogs.noise_input, "V1");
        set_default_if_blank(&mut self.state.dialogs.noise_fstart, "1");
        set_default_if_blank(&mut self.state.dialogs.noise_fstop, "100Meg");
        // Pole-Zero
        set_default_if_blank(&mut self.state.dialogs.pz_input, "in");
        set_default_if_blank(&mut self.state.dialogs.pz_output, "out");
        // Sensitivity
        set_default_if_blank(&mut self.state.dialogs.sens_output, "V(out)");
        // Monte Carlo
        set_default_if_blank(&mut self.state.dialogs.mc_runs, "100");
        set_default_if_blank(&mut self.state.dialogs.mc_seed, "0");
        // PSS
        set_default_if_blank(&mut self.state.dialogs.pss_fund, "1Meg");
        set_default_if_blank(&mut self.state.dialogs.pss_harmonics, "10");
        set_default_if_blank(&mut self.state.dialogs.pss_maxiter, "100");
        // STB
        set_default_if_blank(&mut self.state.dialogs.stb_probe, "istb");
        set_default_if_blank(&mut self.state.dialogs.stb_fstart, "1");
        set_default_if_blank(&mut self.state.dialogs.stb_fstop, "100Meg");
        // Temperature
        set_default_if_blank(&mut self.state.dialogs.temp_start, "-40");
        set_default_if_blank(&mut self.state.dialogs.temp_stop, "125");
        set_default_if_blank(&mut self.state.dialogs.temp_step, "25");
    }

    pub(super) fn render_simulation_setup_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.simulation_dialog {
            return;
        }

        self.ensure_simulation_setup_defaults();

        let mut dialog_open = self.state.dialogs.simulation_dialog;
        let mut close_requested = false;

        egui::Window::new("Simulation Setup")
            .open(&mut dialog_open)
            .collapsible(false)
            .resizable(true)
            .default_width(700.0)
            .default_height(520.0)
            .min_width(600.0)
            .min_height(400.0)
            .show(ctx, |ui| {
                // Reserve space for footer action row.
                let content_height = (ui.available_height() - 60.0).max(200.0);

                ui.horizontal(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(180.0, content_height),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            ui.label(egui::RichText::new("Analyses").strong());
                            ui.separator();

                            egui::ScrollArea::vertical()
                                .id_salt("sim_list")
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    let item_width = ui.available_width() - 4.0;
                                    let item_height = 28.0;
                                    let header_height = 26.0;

                                    let selection_color = ui.visuals().selection.bg_fill;
                                    let hover_color = ui.visuals().widgets.hovered.bg_fill;
                                    let text_color = ui.visuals().text_color();
                                    let dim_color = text_color.gamma_multiply(0.6);
                                    let header_bg = ui.visuals().faint_bg_color;

                                    for (category_name, analyses) in SIMULATION_ANALYSIS_CATEGORIES
                                    {
                                        let (header_rect, _) = ui.allocate_exact_size(
                                            egui::vec2(item_width, header_height),
                                            egui::Sense::hover(),
                                        );
                                        ui.painter().rect_filled(header_rect, 0.0, header_bg);
                                        ui.painter().text(
                                            header_rect.left_center() + egui::vec2(8.0, 0.0),
                                            egui::Align2::LEFT_CENTER,
                                            *category_name,
                                            egui::FontId::proportional(11.0),
                                            dim_color,
                                        );

                                        for &(idx, name) in *analyses {
                                            let selected = self.state.dialogs.sim_active_tab == idx;
                                            let enabled =
                                                self.state.dialogs.enabled_analyses.contains(&idx);

                                            let (rect, response) = ui.allocate_exact_size(
                                                egui::vec2(item_width, item_height),
                                                egui::Sense::click(),
                                            );

                                            if selected {
                                                ui.painter().rect_filled(
                                                    rect,
                                                    4.0,
                                                    selection_color,
                                                );
                                            } else if response.hovered() {
                                                ui.painter().rect_filled(rect, 4.0, hover_color);
                                            }

                                            let checkbox_center =
                                                rect.left_center() + egui::vec2(16.0, 0.0);
                                            let box_size = 16.0;
                                            let checkbox_rect = egui::Rect::from_center_size(
                                                checkbox_center,
                                                egui::vec2(box_size, box_size),
                                            );

                                            if enabled {
                                                ui.painter().rect_filled(
                                                    checkbox_rect,
                                                    3.0,
                                                    egui::Color32::from_rgb(80, 160, 80),
                                                );
                                                let check_color = egui::Color32::WHITE;
                                                let s = box_size * 0.25;
                                                let c = checkbox_center;
                                                ui.painter().line_segment(
                                                    [
                                                        egui::pos2(c.x - s * 1.2, c.y),
                                                        egui::pos2(c.x - s * 0.3, c.y + s * 0.9),
                                                    ],
                                                    egui::Stroke::new(2.0, check_color),
                                                );
                                                ui.painter().line_segment(
                                                    [
                                                        egui::pos2(c.x - s * 0.3, c.y + s * 0.9),
                                                        egui::pos2(c.x + s * 1.2, c.y - s * 0.8),
                                                    ],
                                                    egui::Stroke::new(2.0, check_color),
                                                );
                                            } else {
                                                ui.painter().rect_stroke(
                                                    checkbox_rect,
                                                    3.0,
                                                    egui::Stroke::new(1.5, dim_color),
                                                );
                                            }

                                            let text_col = if selected {
                                                egui::Color32::WHITE
                                            } else {
                                                text_color
                                            };
                                            ui.painter().text(
                                                rect.left_center() + egui::vec2(34.0, 0.0),
                                                egui::Align2::LEFT_CENTER,
                                                name,
                                                egui::FontId::proportional(13.0),
                                                text_col,
                                            );

                                            if response.clicked() {
                                                let click_pos = response
                                                    .interact_pointer_pos()
                                                    .unwrap_or_default();
                                                if checkbox_rect.contains(click_pos) {
                                                    toggle_enabled_analysis(
                                                        &mut self.state.dialogs.enabled_analyses,
                                                        idx,
                                                    );
                                                } else {
                                                    self.state.dialogs.sim_active_tab = idx;
                                                }
                                            }
                                            if response.double_clicked() {
                                                toggle_enabled_analysis(
                                                    &mut self.state.dialogs.enabled_analyses,
                                                    idx,
                                                );
                                            }
                                        }

                                        ui.add_space(4.0);
                                    }
                                });
                        },
                    );

                    ui.separator();

                    ui.allocate_ui_with_layout(
                        egui::vec2(480.0, content_height),
                        egui::Layout::top_down(egui::Align::LEFT),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_salt("sim_opts")
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    self.render_analysis_options(ui);
                                });
                        },
                    );
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Cancel").clicked() {
                            close_requested = true;
                        }
                        if ui.button("OK").clicked() {
                            close_requested = true;
                        }
                    });
                });
            });

        self.state.dialogs.simulation_dialog = dialog_open && !close_requested;
    }

    /// Render the analysis options for the simulation dialog
    pub(super) fn render_analysis_options(&mut self, ui: &mut egui::Ui) {
        app_simulation_analysis_options::render_analysis_options(&mut self.state.dialogs, ui);
    }

    /// Render the global simulation options dialog.
    ///
    /// This dialog mirrors commercial SPICE options editors:
    /// - tabbed organization by option category
    /// - strict parsing and validation on apply
    /// - quick presets for common workflows
    /// - live SPICE `.OPTIONS` preview
    pub(super) fn render_simulation_options_dialog(&mut self, ctx: &Context) {
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

                ui.horizontal_wrapped(|ui| {
                    let tabs = [
                        "Accuracy",
                        "Convergence",
                        "Algorithm",
                        "Limits",
                        "Temperature",
                    ];
                    for (idx, tab_name) in tabs.iter().enumerate() {
                        let selected =
                            self.state.dialogs.simulation_options_state.active_tab == idx;
                        if ui.selectable_label(selected, *tab_name).clicked() {
                            self.state.dialogs.simulation_options_state.active_tab = idx;
                        }
                    }
                });

                ui.separator();
                ui.add_space(4.0);

                let preview_text = match self.state.dialogs.simulation_options_state.to_options() {
                    Ok(opts) => opts.to_spice_options(),
                    Err(_) => self
                        .state
                        .dialogs
                        .simulation_options_config
                        .to_spice_options(),
                };
                // Keep preview fixed at bottom, with a dedicated reserved region.
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
                                match self.state.dialogs.simulation_options_state.active_tab {
                                    0 => {
                                        ui.heading("Accuracy");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_accuracy_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("RELTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.reltol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("RESIDUAL_RELTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.residual_reltol,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("ABSTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.abstol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("VNTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.vntol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("IABSTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.iabstol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("CHGTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.chgtol)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });
                                    }
                                    1 => {
                                        ui.heading("Convergence");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_convergence_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("ITL1");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.itl1)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("ITL4");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.itl4)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("GMIN");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.gmin)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });

                                        ui.add_space(8.0);
                                        ui.checkbox(
                                            &mut opts.gmin_stepping,
                                            "Enable GMIN stepping",
                                        );
                                        ui.checkbox(
                                            &mut opts.source_stepping,
                                            "Enable source stepping",
                                        );
                                        ui.checkbox(
                                            &mut opts.pseudo_transient,
                                            "Enable pseudo-transient continuation",
                                        );

                                        ui.add_space(8.0);
                                        ui.horizontal(|ui| {
                                            ui.label("Damping strategy");
                                            let damping_options =
                                                crate::simulation::dialog::DampingStrategy::all();
                                            let selected = damping_options
                                                [opts.damping.min(damping_options.len() - 1)]
                                            .display_name();
                                            egui::ComboBox::from_id_salt("sim_opts_damping")
                                                .selected_text(selected)
                                                .show_ui(ui, |ui| {
                                                    for (idx, damping) in
                                                        damping_options.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut opts.damping,
                                                            idx,
                                                            damping.display_name(),
                                                        );
                                                    }
                                                });
                                        });
                                    }
                                    2 => {
                                        ui.heading("Algorithm");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;

                                        ui.horizontal(|ui| {
                                            ui.label("Integration method");
                                            let methods =
                                                crate::simulation::dialog::IntegrationMethod::all();
                                            let selected = methods
                                                [opts.method.min(methods.len() - 1)]
                                            .display_name();
                                            egui::ComboBox::from_id_salt("sim_opts_method")
                                                .selected_text(selected)
                                                .show_ui(ui, |ui| {
                                                    for (idx, method) in methods.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut opts.method,
                                                            idx,
                                                            method.display_name(),
                                                        );
                                                    }
                                                });
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Matrix solver");
                                            let solvers =
                                                crate::simulation::dialog::MatrixSolver::all();
                                            let selected = solvers
                                                [opts.solver.min(solvers.len() - 1)]
                                            .display_name();
                                            egui::ComboBox::from_id_salt("sim_opts_solver")
                                                .selected_text(selected)
                                                .show_ui(ui, |ui| {
                                                    for (idx, solver) in solvers.iter().enumerate()
                                                    {
                                                        ui.selectable_value(
                                                            &mut opts.solver,
                                                            idx,
                                                            solver.display_name(),
                                                        );
                                                    }
                                                });
                                        });

                                        ui.add_space(8.0);
                                        ui.checkbox(
                                            &mut opts.bypass_enabled,
                                            "Enable model bypass",
                                        );
                                        egui::Grid::new("sim_opts_bypass_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("Bypass RELTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.bypass_reltol,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("Bypass ABSTOL");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.bypass_abstol,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });
                                    }
                                    3 => {
                                        ui.heading("Limits");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_limits_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("Minimum timestep");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.min_timestep,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("Maximum timestep");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.max_timestep,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("Timestep reduction factor");
                                                ui.add(
                                                    egui::TextEdit::singleline(
                                                        &mut opts.timestep_factor,
                                                    )
                                                    .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });
                                    }
                                    4 => {
                                        ui.heading("Temperature and Output");
                                        ui.add_space(6.0);
                                        let opts = &mut self.state.dialogs.simulation_options_state;
                                        egui::Grid::new("sim_opts_temperature_grid")
                                            .num_columns(2)
                                            .spacing([20.0, 8.0])
                                            .show(ui, |ui| {
                                                ui.label("TEMP (C)");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.temp)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();

                                                ui.label("TNOM (C)");
                                                ui.add(
                                                    egui::TextEdit::singleline(&mut opts.tnom)
                                                        .desired_width(140.0),
                                                );
                                                ui.end_row();
                                            });

                                        ui.add_space(8.0);
                                        ui.checkbox(
                                            &mut opts.verbose,
                                            "Enable verbose convergence diagnostics",
                                        );
                                        ui.checkbox(
                                            &mut opts.save_internals,
                                            "Save internal node/branch data when supported",
                                        );
                                    }
                                    _ => {}
                                }

                                if !self.state.dialogs.simulation_options_errors.is_empty() {
                                    ui.add_space(8.0);
                                    ui.group(|ui| {
                                        ui.label(
                                            egui::RichText::new("Validation issues")
                                                .color(Color32::from_rgb(255, 120, 120))
                                                .strong(),
                                        );
                                        for error in &self.state.dialogs.simulation_options_errors {
                                            ui.label(
                                                egui::RichText::new(format!("- {}", error))
                                                    .color(Color32::from_rgb(255, 120, 120)),
                                            );
                                        }
                                    });
                                }
                            });
                    },
                );

                ui.add_space(8.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("SPICE .OPTIONS preview").strong());
                    ui.add_space(4.0);
                    let mut preview_buf = preview_text;
                    ui.add_enabled(
                        false,
                        egui::TextEdit::multiline(&mut preview_buf)
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
                        let preset = crate::simulation::dialog::SimulationOptions::default();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Fast").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::fast();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Accurate").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::accurate();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Robust").clicked() {
                        let preset = crate::simulation::dialog::SimulationOptions::robust();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&preset);
                        self.state.dialogs.simulation_options_errors.clear();
                    }
                    if ui.button("Revert").clicked() {
                        let current = self.state.dialogs.simulation_options_config.clone();
                        self.state.dialogs.simulation_options_state =
                            crate::simulation::dialog::OptionsDialogState::from_options(&current);
                        self.state.dialogs.simulation_options_errors.clear();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ok_clicked = ui.button("OK").clicked();
                        let apply_clicked = ui.button("Apply").clicked();
                        if ui.button("Cancel").clicked() {
                            close_requested = true;
                        }

                        if apply_clicked || ok_clicked {
                            let parsed = self.state.dialogs.simulation_options_state.to_options();
                            let validated = match parsed {
                                Ok(opts) => match opts.validate() {
                                    Ok(()) => Some(opts),
                                    Err(validation_errors) => {
                                        self.state.dialogs.simulation_options_errors =
                                            validation_errors
                                                .into_iter()
                                                .map(|err| err.to_string())
                                                .collect();
                                        None
                                    }
                                },
                                Err(parse_errors) => {
                                    self.state.dialogs.simulation_options_errors = parse_errors;
                                    None
                                }
                            };

                            if let Some(opts) = validated {
                                self.state.dialogs.simulation_options_config = opts.clone();
                                self.state.dialogs.simulation_options_state =
                                    crate::simulation::dialog::OptionsDialogState::from_options(
                                        &opts,
                                    );
                                self.state.dialogs.simulation_options_errors.clear();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn make_test_app() -> RSpiceApp {
        RSpiceApp {
            state: super::super::AppState::default(),
            first_frame: false,
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::common::file_workflow::NativeFileWorkflowIo),
        }
    }

    #[test]
    fn test_set_default_if_blank_updates_only_blank_values() {
        let mut value = "   ".to_string();
        set_default_if_blank(&mut value, "default");
        assert_eq!(value, "default");

        let mut existing = "keep".to_string();
        set_default_if_blank(&mut existing, "default");
        assert_eq!(existing, "keep");
    }

    #[test]
    fn test_toggle_enabled_analysis_toggles_membership() {
        let mut enabled = HashSet::new();
        toggle_enabled_analysis(&mut enabled, 7);
        assert!(enabled.contains(&7));
        toggle_enabled_analysis(&mut enabled, 7);
        assert!(!enabled.contains(&7));
    }

    #[test]
    fn test_simulation_analysis_categories_have_unique_indices() {
        let mut seen = HashSet::new();
        for (category_name, analyses) in SIMULATION_ANALYSIS_CATEGORIES {
            assert!(!category_name.trim().is_empty());
            assert!(!analyses.is_empty());
            for &(index, name) in *analyses {
                assert!(!name.trim().is_empty());
                assert!(
                    seen.insert(index),
                    "duplicate analysis index found in categories: {}",
                    index
                );
            }
        }
    }

    #[test]
    fn test_ensure_simulation_setup_defaults_fills_each_field_independently() {
        let mut app = make_test_app();
        app.state.dialogs.tran_stop = "5m".to_string();
        app.state.dialogs.ac_points = "501".to_string();
        app.state.dialogs.tran_step.clear();
        app.state.dialogs.noise_input = "   ".to_string();
        app.state.dialogs.temp_start.clear();

        app.ensure_simulation_setup_defaults();

        assert_eq!(app.state.dialogs.tran_stop, "5m");
        assert_eq!(app.state.dialogs.ac_points, "501");
        assert_eq!(app.state.dialogs.tran_step, "10n");
        assert_eq!(app.state.dialogs.noise_input, "V1");
        assert_eq!(app.state.dialogs.temp_start, "-40");
    }
}
