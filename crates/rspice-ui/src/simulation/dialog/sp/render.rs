use super::{SpDialogState, SpPortDialogState};
use egui::Ui;

impl SpDialogState {
    /// Render dialog content
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();

        ui.heading("S-Parameter Analysis");
        ui.add_space(4.0);
        ui.label(egui::RichText::new("RF/microwave network characterization (Sij matrix)").weak());
        ui.add_space(12.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("Frequency Range").strong());
            ui.add_space(4.0);

            egui::Grid::new("sp_freq_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Start:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.start_freq)
                            .desired_width(120.0)
                            .hint_text("1Meg"),
                    );
                    ui.end_row();

                    ui.label("Stop:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.stop_freq)
                            .desired_width(120.0)
                            .hint_text("10G"),
                    );
                    ui.end_row();

                    ui.label("Points:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.num_points)
                            .desired_width(120.0)
                            .hint_text("10"),
                    );
                    ui.end_row();

                    ui.label("Sweep:");
                    let sweeps = ["Decade", "Octave", "Linear"];
                    egui::ComboBox::from_id_salt("sp_sweep")
                        .selected_text(sweeps[self.sweep_type_idx])
                        .show_ui(ui, |ui| {
                            for (idx, name) in sweeps.iter().enumerate() {
                                if ui
                                    .selectable_label(self.sweep_type_idx == idx, *name)
                                    .clicked()
                                {
                                    self.sweep_type_idx = idx;
                                }
                            }
                        });
                    ui.end_row();
                });
        });

        ui.add_space(8.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new("Reference Impedance").strong());
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Z0:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.z0)
                        .desired_width(80.0)
                        .hint_text("50"),
                );
                ui.label("Ω");
            });
        });

        ui.add_space(8.0);
        self.render_ports(ui);

        ui.add_space(8.0);
        self.render_options(ui);

        if let Ok(config) = self.to_config() {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(format!(
                    "{}-port | ~{} freq points",
                    config.num_ports(),
                    config.total_points()
                ))
                .size(10.0)
                .color(egui::Color32::from_rgb(120, 125, 135)),
            );
        }
    }

    fn render_ports(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Port Configuration").strong());
            ui.add_space(4.0);

            let mut remove_index = None;
            for (idx, port) in self.ports.iter_mut().enumerate() {
                let hint = Self::default_port_node(idx);
                ui.horizontal(|ui| {
                    ui.label(format!("Port {}:", idx + 1));
                    ui.add(
                        egui::TextEdit::singleline(&mut port.node_pos)
                            .desired_width(110.0)
                            .hint_text(hint.as_str()),
                    );
                    ui.checkbox(&mut port.differential, "Differential");
                    ui.checkbox(&mut port.z0_override, "Z0");
                    if port.z0_override {
                        ui.add(
                            egui::TextEdit::singleline(&mut port.z0)
                                .desired_width(70.0)
                                .hint_text("50"),
                        );
                        ui.label("ohm");
                    }
                    if idx >= 2 && ui.small_button("Remove").clicked() {
                        remove_index = Some(idx);
                    }
                });
                if port.differential {
                    ui.horizontal(|ui| {
                        ui.add_space(55.0);
                        ui.label("Neg:");
                        ui.add(
                            egui::TextEdit::singleline(&mut port.node_neg)
                                .desired_width(110.0)
                                .hint_text("0"),
                        );
                    });
                }
                ui.add_space(4.0);
            }
            if let Some(idx) = remove_index {
                self.ports.remove(idx);
            }

            if ui.button("+ Add Port").clicked() {
                let node = Self::default_port_node(self.ports.len());
                self.ports.push(SpPortDialogState::single_ended(node));
            }
        });
    }

    fn render_options(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Options").strong());
            ui.add_space(4.0);

            ui.checkbox(&mut self.do_noise, "Include Noise Analysis (NF)");
            ui.checkbox(&mut self.touchstone_export, "Export Touchstone (.sNp)");
            if self.touchstone_export {
                ui.horizontal(|ui| {
                    ui.label("Touchstone version:");
                    egui::ComboBox::from_id_salt("sp_touchstone_version")
                        .selected_text(format!("v{}", self.touchstone_version.clamp(1, 2)))
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(self.touchstone_version == 1, "v1")
                                .clicked()
                            {
                                self.touchstone_version = 1;
                            }
                            if ui
                                .selectable_label(self.touchstone_version != 1, "v2")
                                .clicked()
                            {
                                self.touchstone_version = 2;
                            }
                        });
                });
            }
        });
    }
}
