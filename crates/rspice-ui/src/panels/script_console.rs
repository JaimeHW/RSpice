//! Scripting Console Panel
//!
//! Integrated professional console for RSpice automation.

use crate::simulation::automation::{CommandOutput, ScriptExecutor};
use crate::state::SimulationState;
use egui::{Align, Color32, Layout, RichText, Ui};

/// State for the scripting console
#[derive(Clone, Default)]
pub struct ScriptConsoleState {
    pub input_buffer: String,
    pub history: Vec<ConsoleHistoryItem>,
    pub executor: ScriptExecutor,
}

#[derive(Clone, Default)]
pub struct ConsoleHistoryItem {
    pub command: String,
    pub output: CommandOutput,
}

/// Renders the Scripting Console Panel
pub fn render_script_console(
    ui: &mut Ui,
    state: &mut ScriptConsoleState,
    app_state: &mut SimulationState,
) {
    ui.heading("Automation Console");
    ui.add_space(4.0);

    // Scrollable History Area
    let history_height = ui.available_height() - 40.0;
    egui::ScrollArea::vertical()
        .max_height(history_height)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for item in &state.history {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(">").weak());
                    ui.label(&item.command);
                });

                let color = if item.output.success {
                    Color32::from_rgb(46, 204, 113) // Green
                } else {
                    Color32::from_rgb(231, 76, 60) // Red
                };

                ui.label(RichText::new(&item.output.message).color(color));
                ui.add_space(4.0);
            }
        });

    ui.separator();

    // Input Area
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        let response = ui.add(
            egui::TextEdit::singleline(&mut state.input_buffer)
                .hint_text("Enter command (e.g., 'run tran', 'help')...")
                .desired_width(f32::INFINITY)
                .lock_focus(true),
        );

        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let cmd = state.input_buffer.trim().to_string();
            if !cmd.is_empty() {
                let output = state.executor.execute_command(&cmd, app_state);
                state.history.push(ConsoleHistoryItem {
                    command: cmd,
                    output,
                });
                state.input_buffer.clear();
                response.request_focus();
            }
        }
    });
}
