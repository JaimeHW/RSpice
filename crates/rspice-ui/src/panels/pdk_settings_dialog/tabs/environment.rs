use super::super::model::PdkSettingsDialogState;
use egui::{RichText, Ui};

/// Render the Environment tab
pub(in crate::panels::pdk_settings_dialog) fn render_environment_tab(
    ui: &mut Ui,
    state: &mut PdkSettingsDialogState,
) {
    ui.heading("Environment Variables");
    ui.add_space(4.0);

    ui.label(
        RichText::new(
            "Define environment variable overrides for path expansion (e.g., $PDK_HOME).",
        )
        .color(egui::Color32::GRAY)
        .size(11.0),
    );

    ui.add_space(8.0);

    let mut remove_key: Option<String> = None;
    let env_entries: Vec<(String, String)> = state
        .config
        .env_overrides()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    for (name, value) in &env_entries {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(RichText::new(format!("${}", name)).strong().monospace());
            ui.label("=");
            ui.label(RichText::new(value).monospace());

            let resolved = state.config.expand_path(value);
            if resolved != *value {
                ui.label(
                    RichText::new(format!("→ {}", resolved))
                        .color(egui::Color32::GRAY)
                        .size(10.0),
                );
            }

            if ui.small_button("🗑").on_hover_text("Remove").clicked() {
                remove_key = Some(name.clone());
            }
        });
    }

    if let Some(key) = remove_key {
        state.remove_env_override(&key);
    }

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("$");
        ui.add(
            egui::TextEdit::singleline(&mut state.new_env_name)
                .desired_width(120.0)
                .hint_text("VAR_NAME"),
        );
        ui.label("=");
        ui.add(
            egui::TextEdit::singleline(&mut state.new_env_value)
                .desired_width(250.0)
                .hint_text("value or path"),
        );

        if ui.button("Add").clicked() && !state.new_env_name.is_empty() {
            state.add_env_override(state.new_env_name.clone(), state.new_env_value.clone());
        }
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    egui::CollapsingHeader::new("System Environment Variables")
        .default_open(false)
        .show(ui, |ui| {
            ui.label(
                RichText::new("Read-only view of relevant system environment variables.")
                    .color(egui::Color32::GRAY)
                    .size(11.0),
            );
            ui.add_space(4.0);

            let common_vars = [
                "PDK_HOME",
                "PDK_ROOT",
                "MY_TECH",
                "SPICE_LIB_DIR",
                "MODEL_PATH",
            ];
            for var in common_vars {
                if let Ok(value) = std::env::var(var) {
                    ui.horizontal(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("${}", var)).monospace());
                        ui.label("=");
                        ui.label(RichText::new(&value).size(11.0));
                    });
                }
            }
        });
}
