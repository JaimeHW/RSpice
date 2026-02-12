use super::super::DialogState;
use super::defaults::toggle_enabled_analysis;
use super::{app_simulation_analysis_options, Context, RSpiceApp, SIMULATION_ANALYSIS_CATEGORIES};

fn render_analysis_list(ui: &mut egui::Ui, dialogs: &mut DialogState, content_height: f32) {
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

                    for (category_name, analyses) in SIMULATION_ANALYSIS_CATEGORIES {
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
                            let selected = dialogs.sim_active_tab == idx;
                            let enabled = dialogs.enabled_analyses.contains(&idx);

                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(item_width, item_height),
                                egui::Sense::click(),
                            );

                            if selected {
                                ui.painter().rect_filled(rect, 4.0, selection_color);
                            } else if response.hovered() {
                                ui.painter().rect_filled(rect, 4.0, hover_color);
                            }

                            let checkbox_center = rect.left_center() + egui::vec2(16.0, 0.0);
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

                            let text_color = if selected {
                                egui::Color32::WHITE
                            } else {
                                text_color
                            };
                            ui.painter().text(
                                rect.left_center() + egui::vec2(34.0, 0.0),
                                egui::Align2::LEFT_CENTER,
                                name,
                                egui::FontId::proportional(13.0),
                                text_color,
                            );

                            if response.clicked() {
                                let click_pos = response.interact_pointer_pos().unwrap_or_default();
                                if checkbox_rect.contains(click_pos) {
                                    toggle_enabled_analysis(&mut dialogs.enabled_analyses, idx);
                                } else {
                                    dialogs.sim_active_tab = idx;
                                }
                            }
                            if response.double_clicked() {
                                toggle_enabled_analysis(&mut dialogs.enabled_analyses, idx);
                            }
                        }

                        ui.add_space(4.0);
                    }
                });
        },
    );
}

impl RSpiceApp {
    pub(in crate::common::app) fn render_simulation_setup_dialog(&mut self, ctx: &Context) {
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
                let content_height = (ui.available_height() - 60.0).max(200.0);

                ui.horizontal(|ui| {
                    render_analysis_list(ui, &mut self.state.dialogs, content_height);

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

    pub(super) fn render_analysis_options(&mut self, ui: &mut egui::Ui) {
        app_simulation_analysis_options::render_analysis_options(&mut self.state.dialogs, ui);
    }
}
