use egui::{Context, RichText};

use super::{ConfirmationAction, ConfirmationResponse, RSpiceApp};

fn split_prompt_lines(prompt: &str) -> (&str, &str) {
    match prompt.split_once('\n') {
        Some((line1, line2)) => (line1, line2),
        None => (prompt, ""),
    }
}

fn confirmation_prompt_lines(action: Option<ConfirmationAction>) -> (&'static str, &'static str) {
    let prompt = action
        .as_ref()
        .map(ConfirmationAction::prompt_message)
        .unwrap_or(
            "The current schematic has unsaved changes.\nDo you want to save before continuing?",
        );
    split_prompt_lines(prompt)
}

impl RSpiceApp {
    pub(super) fn render_confirmation_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.confirmation_dialog.visible {
            return;
        }

        let mut response: Option<ConfirmationResponse> = None;

        egui::Area::new(egui::Id::new("save_confirmation_backdrop"))
            .order(egui::Order::Foreground)
            .anchor(egui::Align2::LEFT_TOP, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                // Dim backdrop to block interactions with the editor until confirmed.
                let screen = ui.ctx().screen_rect();
                ui.painter().rect_filled(
                    screen,
                    egui::Rounding::ZERO,
                    egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180),
                );
            });

        egui::Window::new("Save Changes?")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                ui.set_min_width(350.0);

                let pending_action = self.state.dialogs.confirmation_dialog.pending_action;
                let (prompt_line1, prompt_line2) = confirmation_prompt_lines(pending_action);

                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("!")
                            .size(28.0)
                            .color(egui::Color32::from_rgb(255, 191, 0)),
                    );
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        if let Some(action) = pending_action {
                            ui.label(RichText::new(action.dialog_title()).size(14.0).strong());
                        }
                        ui.add_space(4.0);
                        ui.label(prompt_line1);
                        if !prompt_line2.is_empty() {
                            ui.label(prompt_line2);
                        }
                    });
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Cancel").clicked() {
                            response = Some(ConfirmationResponse::Cancel);
                        }

                        ui.add_space(8.0);

                        if ui.button("Don't Save").clicked() {
                            response = Some(ConfirmationResponse::No);
                        }

                        ui.add_space(8.0);

                        if ui
                            .add(
                                egui::Button::new(RichText::new("Save").strong())
                                    .fill(egui::Color32::from_rgb(59, 130, 246)),
                            )
                            .clicked()
                        {
                            response = Some(ConfirmationResponse::Yes);
                        }
                    });
                });
            });

        if let Some(r) = response {
            self.handle_confirmation_response(r);
        }
    }
}

