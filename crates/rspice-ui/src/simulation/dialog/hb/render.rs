use super::{HbDialogState, HbToneDialogState};
use egui::Ui;

impl HbDialogState {
    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("Harmonic Balance Analysis");
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new("Large-signal frequency-domain steady-state for RF circuits")
                .weak(),
        );
        ui.add_space(12.0);

        // Fundamental Tone
        ui.group(|ui| {
            ui.label(egui::RichText::new("Fundamental Tone").strong());
            ui.add_space(4.0);

            egui::Grid::new("hb_fund_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Frequency:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fundamental)
                            .desired_width(120.0)
                            .hint_text("1G"),
                    );
                    ui.end_row();

                    ui.label("Harmonics:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.harmonics)
                            .desired_width(120.0)
                            .hint_text("9"),
                    );
                    ui.end_row();

                    ui.label("Tone Name:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fundamental_name)
                            .desired_width(120.0)
                            .hint_text("tone1"),
                    );
                    ui.end_row();

                    ui.label("Source:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.fundamental_source)
                            .desired_width(120.0)
                            .hint_text("V1"),
                    );
                    ui.end_row();

                    ui.label("Oversample:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.oversample)
                            .desired_width(120.0)
                            .hint_text("2"),
                    );
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Additional tones.
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Additional Tones").strong());
                if ui.small_button("+ Add Tone").clicked() {
                    self.additional_tones.push(HbToneDialogState {
                        frequency: "900Meg".to_string(),
                        harmonics: "5".to_string(),
                        name: format!("tone{}", self.additional_tones.len() + 2),
                        source: String::new(),
                    });
                }
            });
            ui.add_space(4.0);

            if self.additional_tones.is_empty() {
                ui.label(
                    egui::RichText::new("No additional tones configured")
                        .small()
                        .weak(),
                );
            } else {
                let mut remove_index = None;
                for (idx, tone) in self.additional_tones.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("Tone {}:", idx + 2));
                        ui.label("F");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.frequency)
                                .desired_width(80.0)
                                .hint_text("900Meg"),
                        );
                        ui.label("H");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.harmonics)
                                .desired_width(40.0)
                                .hint_text("5"),
                        );
                        ui.label("Name");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.name)
                                .desired_width(70.0)
                                .hint_text("LO"),
                        );
                        ui.label("Source");
                        ui.add(
                            egui::TextEdit::singleline(&mut tone.source)
                                .desired_width(70.0)
                                .hint_text("VLO"),
                        );
                        if ui.small_button("Remove").clicked() {
                            remove_index = Some(idx);
                        }
                    });
                }
                if let Some(idx) = remove_index {
                    self.additional_tones.remove(idx);
                }
            }
        });

        ui.add_space(8.0);

        // Convergence
        ui.group(|ui| {
            ui.label(egui::RichText::new("Convergence").strong());
            ui.add_space(4.0);

            egui::Grid::new("hb_conv_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Rel. Tolerance:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.reltol)
                            .desired_width(120.0)
                            .hint_text("1e-6"),
                    );
                    ui.end_row();

                    ui.label("Max Iterations:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.maxiter)
                            .desired_width(120.0)
                            .hint_text("100"),
                    );
                    ui.end_row();

                    ui.label("Damping:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.damping)
                            .desired_width(120.0)
                            .hint_text("1.0"),
                    );
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        // Solver Options
        ui.group(|ui| {
            ui.label(egui::RichText::new("Solver Options").strong());
            ui.add_space(4.0);

            egui::Grid::new("hb_solver_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Solver:");
                    let solvers = ["Newton-Raphson", "Krylov (GMRES)"];
                    egui::ComboBox::from_id_salt("hb_solver")
                        .selected_text(solvers[self.solver_idx])
                        .show_ui(ui, |ui| {
                            for (idx, name) in solvers.iter().enumerate() {
                                if ui.selectable_label(self.solver_idx == idx, *name).clicked() {
                                    self.solver_idx = idx;
                                }
                            }
                        });
                    ui.end_row();

                    if self.solver_idx == 1 {
                        ui.label("GMRES Restart:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.gmres_restart)
                                .desired_width(120.0)
                                .hint_text("30"),
                        );
                        ui.end_row();
                    }
                });

            ui.add_space(4.0);
            ui.checkbox(
                &mut self.source_stepping,
                "Enable Source Stepping (for difficult convergence)",
            );
        });

        // Info footer
        if let Ok(config) = self.to_config() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!(
                    "Spectral components: {} | FFT size: {}",
                    config.num_spectral_components(),
                    config.fft_size()
                ))
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 135)),
            );
        }
    }
}
