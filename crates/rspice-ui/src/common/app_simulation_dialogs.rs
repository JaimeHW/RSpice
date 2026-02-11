use super::{ConsoleMessage, RSpiceApp};
use egui::{Color32, Context};

impl RSpiceApp {
    /// Render the analysis options for the simulation dialog
    pub(super) fn render_analysis_options(&mut self, ui: &mut egui::Ui) {
        match self.state.dialogs.sim_active_tab {
            0 => {
                self.state.dialogs.op_state.render(ui);
            }
            1 => {
                ui.heading("Transient Analysis");
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new("Time-domain analysis for switching and dynamic circuits")
                        .weak(),
                );
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Time Parameters").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("tran_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Stop Time:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_stop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Step Time:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_step)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Start Time:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_start)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Max Step:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.tran_maxstep)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Options").strong());
                    ui.add_space(5.0);
                    ui.checkbox(
                        &mut self.state.dialogs.tran_uic,
                        "Use Initial Conditions (UIC)",
                    );
                });
            }
            2 => {
                ui.heading("AC Analysis");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Small-signal frequency response analysis").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Frequency Range").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("ac_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Start Frequency:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_fstart)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop Frequency:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_fstop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Points/Decade:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_points)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Sweep Type:");
                            let sweep_types = ["Decade", "Octave", "Linear"];
                            egui::ComboBox::from_id_salt("ac_sweep")
                                .selected_text(sweep_types[self.state.dialogs.ac_sweep_type])
                                .show_ui(ui, |ui| {
                                    for (idx, name) in sweep_types.iter().enumerate() {
                                        if ui
                                            .selectable_label(
                                                self.state.dialogs.ac_sweep_type == idx,
                                                *name,
                                            )
                                            .clicked()
                                        {
                                            self.state.dialogs.ac_sweep_type = idx;
                                        }
                                    }
                                });
                            ui.end_row();
                        });
                });
            }
            24 => {
                ui.heading("DISTO Analysis");
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new(
                        "Transfer-based harmonic/intermodulation distortion estimates versus frequency",
                    )
                    .weak(),
                );
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Frequency Sweep").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("disto_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Start Frequency:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_fstart)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop Frequency:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_fstop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Points/Decade:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.ac_points)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Sweep Type:");
                            let sweep_types = ["Decade", "Octave", "Linear"];
                            egui::ComboBox::from_id_salt("disto_sweep")
                                .selected_text(sweep_types[self.state.dialogs.ac_sweep_type])
                                .show_ui(ui, |ui| {
                                    for (idx, name) in sweep_types.iter().enumerate() {
                                        if ui
                                            .selectable_label(
                                                self.state.dialogs.ac_sweep_type == idx,
                                                *name,
                                            )
                                            .clicked()
                                        {
                                            self.state.dialogs.ac_sweep_type = idx;
                                        }
                                    }
                                });
                            ui.end_row();
                            ui.label("f2/f1 Ratio:");
                            ui.add(
                                egui::TextEdit::singleline(
                                    &mut self.state.dialogs.disto_f2_over_f1,
                                )
                                .desired_width(120.0)
                                .hint_text("auto"),
                            );
                            ui.end_row();
                        });
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new("Note: DISTO uses nonlinear HB extraction when available and falls back to linearized AC estimates if HB is unsupported for the circuit.")
                            .size(11.0)
                            .weak(),
                    );
                });
            }
            3 => {
                ui.heading("DC Sweep");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("DC parameter sweep analysis").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Primary Sweep").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("dc_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Source:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_source)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Start:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_start)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_stop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Step:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.dc_step)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
                ui.add_space(10.0);
                ui.checkbox(&mut self.state.dialogs.dc_nested, "Enable Nested Sweep");
                if self.state.dialogs.dc_nested {
                    ui.add_space(5.0);
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Secondary Sweep").strong());
                        ui.add_space(5.0);
                        egui::Grid::new("dc_grid2")
                            .num_columns(2)
                            .spacing([20.0, 6.0])
                            .show(ui, |ui| {
                                ui.label("Source:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_source2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                                ui.label("Start:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_start2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                                ui.label("Stop:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_stop2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                                ui.label("Step:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.state.dialogs.dc_step2)
                                        .desired_width(120.0),
                                );
                                ui.end_row();
                            });
                    });
                }
            }
            4 => {
                ui.heading("Noise Analysis");
                ui.add_space(5.0);
                ui.label(egui::RichText::new("Spectral noise analysis").weak());
                ui.add_space(15.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Node Configuration").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("noise_grid")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Output Node:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_output)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Reference:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_ref)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Input Source:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_input)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.label(egui::RichText::new("Frequency Range").strong());
                    ui.add_space(5.0);
                    egui::Grid::new("noise_freq")
                        .num_columns(2)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.label("Start Freq:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_fstart)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop Freq:");
                            ui.add(
                                egui::TextEdit::singleline(&mut self.state.dialogs.noise_fstop)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                        });
                });
            }
            5 => {
                self.state.dialogs.pz_state.render(ui);
            }
            6 => {
                self.state.dialogs.sens_state.render(ui);
            }
            7 => {
                self.state.dialogs.mc_state.render(ui);
            }
            8 => {
                self.state.dialogs.pss_state.render(ui);
            }
            9 => {
                self.state.dialogs.stb_state.render(ui);
            }
            10 => {
                self.state.dialogs.temp_state.render(ui);
            }
            11 => {
                self.state.dialogs.hb_state.render(ui);
            }
            12 => {
                self.state.dialogs.sp_state.render(ui);
            }
            13 => {
                self.state.dialogs.pac_state.render(ui);
            }
            14 => {
                self.state.dialogs.pnoise_state.render(ui);
            }
            15 => {
                self.state.dialogs.pxf_state.render(ui);
            }
            16 => {
                self.state.dialogs.pstb_state.render(ui);
            }
            17 => {
                self.state.dialogs.xf_state.render(ui);
            }
            18 => {
                self.state.dialogs.corner_state.render(ui);
            }
            19 => {
                self.state.dialogs.envelope_state.render(ui);
            }
            20 => {
                self.state.dialogs.fourier_state.render(ui);
            }
            21 => {
                self.state.dialogs.reliability_state.render(ui);
            }
            22 => {
                self.state.dialogs.optimization_state.render(ui);
            }
            23 => {
                self.state.dialogs.soa_state.render(ui);
            }
            _ => {}
        }
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
