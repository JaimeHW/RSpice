use crate::common::app::DialogState;

pub(super) fn render_analysis_options(dialogs: &mut DialogState, ui: &mut egui::Ui) {
    match dialogs.sim_active_tab {
        0 => {
            dialogs.op_state.render(ui);
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
                            egui::TextEdit::singleline(&mut dialogs.tran_stop).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Step Time:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.tran_step).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Start Time:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.tran_start)
                                .desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Max Step:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.tran_maxstep)
                                .desired_width(120.0),
                        );
                        ui.end_row();
                    });
            });
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label(egui::RichText::new("Options").strong());
                ui.add_space(5.0);
                ui.checkbox(&mut dialogs.tran_uic, "Use Initial Conditions (UIC)");
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
                            egui::TextEdit::singleline(&mut dialogs.ac_fstart).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Stop Frequency:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.ac_fstop).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Points/Decade:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.ac_points).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Sweep Type:");
                        let sweep_types = ["Decade", "Octave", "Linear"];
                        egui::ComboBox::from_id_salt("ac_sweep")
                            .selected_text(sweep_types[dialogs.ac_sweep_type])
                            .show_ui(ui, |ui| {
                                for (idx, name) in sweep_types.iter().enumerate() {
                                    if ui
                                        .selectable_label(dialogs.ac_sweep_type == idx, *name)
                                        .clicked()
                                    {
                                        dialogs.ac_sweep_type = idx;
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
                            egui::TextEdit::singleline(&mut dialogs.ac_fstart).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Stop Frequency:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.ac_fstop).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Points/Decade:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.ac_points).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Sweep Type:");
                        let sweep_types = ["Decade", "Octave", "Linear"];
                        egui::ComboBox::from_id_salt("disto_sweep")
                            .selected_text(sweep_types[dialogs.ac_sweep_type])
                            .show_ui(ui, |ui| {
                                for (idx, name) in sweep_types.iter().enumerate() {
                                    if ui
                                        .selectable_label(dialogs.ac_sweep_type == idx, *name)
                                        .clicked()
                                    {
                                        dialogs.ac_sweep_type = idx;
                                    }
                                }
                            });
                        ui.end_row();
                        ui.label("f2/f1 Ratio:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.disto_f2_over_f1)
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
                            egui::TextEdit::singleline(&mut dialogs.dc_source).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Start:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.dc_start).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Stop:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.dc_stop).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Step:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.dc_step).desired_width(120.0),
                        );
                        ui.end_row();
                    });
            });
            ui.add_space(10.0);
            ui.checkbox(&mut dialogs.dc_nested, "Enable Nested Sweep");
            if dialogs.dc_nested {
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
                                egui::TextEdit::singleline(&mut dialogs.dc_source2)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Start:");
                            ui.add(
                                egui::TextEdit::singleline(&mut dialogs.dc_start2)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Stop:");
                            ui.add(
                                egui::TextEdit::singleline(&mut dialogs.dc_stop2)
                                    .desired_width(120.0),
                            );
                            ui.end_row();
                            ui.label("Step:");
                            ui.add(
                                egui::TextEdit::singleline(&mut dialogs.dc_step2)
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
                            egui::TextEdit::singleline(&mut dialogs.noise_output)
                                .desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Reference:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.noise_ref).desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Input Source:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.noise_input)
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
                            egui::TextEdit::singleline(&mut dialogs.noise_fstart)
                                .desired_width(120.0),
                        );
                        ui.end_row();
                        ui.label("Stop Freq:");
                        ui.add(
                            egui::TextEdit::singleline(&mut dialogs.noise_fstop)
                                .desired_width(120.0),
                        );
                        ui.end_row();
                    });
            });
        }
        5 => {
            dialogs.pz_state.render(ui);
        }
        6 => {
            dialogs.sens_state.render(ui);
        }
        7 => {
            dialogs.mc_state.render(ui);
        }
        8 => {
            dialogs.pss_state.render(ui);
        }
        9 => {
            dialogs.stb_state.render(ui);
        }
        10 => {
            dialogs.temp_state.render(ui);
        }
        11 => {
            dialogs.hb_state.render(ui);
        }
        12 => {
            dialogs.sp_state.render(ui);
        }
        13 => {
            dialogs.pac_state.render(ui);
        }
        14 => {
            dialogs.pnoise_state.render(ui);
        }
        15 => {
            dialogs.pxf_state.render(ui);
        }
        16 => {
            dialogs.pstb_state.render(ui);
        }
        17 => {
            dialogs.xf_state.render(ui);
        }
        18 => {
            dialogs.corner_state.render(ui);
        }
        19 => {
            dialogs.envelope_state.render(ui);
        }
        20 => {
            dialogs.fourier_state.render(ui);
        }
        21 => {
            dialogs.reliability_state.render(ui);
        }
        22 => {
            dialogs.optimization_state.render(ui);
        }
        23 => {
            dialogs.soa_state.render(ui);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render_with_dialogs(dialogs: &mut DialogState) {
        let ctx = egui::Context::default();
        let raw_input = egui::RawInput::default();
        let _ = ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_analysis_options(dialogs, ui);
            });
        });
    }

    #[test]
    fn test_render_analysis_options_unknown_tab_is_noop() {
        let mut dialogs = DialogState::default();
        dialogs.sim_active_tab = usize::MAX;
        dialogs.tran_stop = "7m".to_string();

        render_with_dialogs(&mut dialogs);

        assert_eq!(dialogs.tran_stop, "7m".to_string());
    }

    #[test]
    fn test_render_analysis_options_transient_keeps_existing_values() {
        let mut dialogs = DialogState::default();
        dialogs.sim_active_tab = 1;
        dialogs.tran_stop = "8m".to_string();
        dialogs.tran_step = "5n".to_string();
        dialogs.tran_start = "1u".to_string();
        dialogs.tran_maxstep = "10n".to_string();

        render_with_dialogs(&mut dialogs);

        assert_eq!(dialogs.tran_stop, "8m".to_string());
        assert_eq!(dialogs.tran_step, "5n".to_string());
        assert_eq!(dialogs.tran_start, "1u".to_string());
        assert_eq!(dialogs.tran_maxstep, "10n".to_string());
    }
}
