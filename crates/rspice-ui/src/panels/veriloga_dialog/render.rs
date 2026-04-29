use std::path::PathBuf;

use egui::{Context, RichText, Ui, Window};

use super::compile::{poll_compile, start_compile};
use super::options::VerilogADialogOptions;
use super::state::VerilogALoadDialogState;
use super::types::{CompilationState, ErrorSeverity, VerilogADialogResult};

/// Render the Verilog-A model loading dialog.
///
/// Returns the dialog result indicating user action.
pub fn render_veriloga_load_dialog(
    ctx: &Context,
    state: &mut VerilogALoadDialogState,
) -> VerilogADialogResult {
    if !state.open {
        return VerilogADialogResult::None;
    }

    poll_compile(state);

    let mut result = VerilogADialogResult::None;
    let mut should_close = false;

    Window::new("Load Verilog-A Model")
        .resizable(true)
        .collapsible(false)
        .default_width(500.0)
        .default_height(400.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 8.0;

            render_file_selection(ui, state);

            ui.add_space(4.0);
            egui::CollapsingHeader::new("Compiler Options")
                .default_open(state.show_advanced_options)
                .show(ui, |ui| {
                    state.show_advanced_options = true;
                    render_compiler_options(ui, &mut state.options);
                });

            ui.add_space(4.0);
            render_compilation_status(ui, state);

            ui.add_space(8.0);
            ui.separator();
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                        result = VerilogADialogResult::Cancelled;
                    }

                    if state.is_success() && ui.button("Add to Library").clicked() {
                        result = VerilogADialogResult::AddToLibrary;
                        should_close = true;
                    }
                });
            });
        });

    if should_close {
        state.close();
    }

    result
}

fn render_file_selection(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.strong("File:");
            ui.add_space(8.0);

            let text_edit = egui::TextEdit::singleline(&mut state.file_path_text)
                .desired_width(350.0)
                .hint_text("Path to .va file...");
            if ui.add(text_edit).changed() {
                sync_file_path_text(state);
            }

            if ui.button("Browse...").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("Verilog-A", &["va", "vams"])
                    .pick_file()
            {
                state.set_file_path(path);
            }
        });

        if let Some(path) = &state.file_path {
            if path.exists() {
                let file_name = path
                    .file_name()
                    .map(|s| s.to_string_lossy())
                    .unwrap_or_default();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("File: ").color(egui::Color32::GRAY));
                    ui.label(RichText::new(file_name).strong());
                });
            } else {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "File not found");
                });
            }
        }
    });
}

fn sync_file_path_text(state: &mut VerilogALoadDialogState) {
    if state.file_path_text.is_empty() {
        state.file_path = None;
    } else {
        state.file_path = Some(PathBuf::from(&state.file_path_text));
    }
    state.errors.clear();
    state.compiled_module = None;
    state.compiled_artifact = None;
    state.compiled_dependencies = None;
    state.compilation_state = CompilationState::Idle;
}

fn render_compilation_status(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    match state.compilation_state {
        CompilationState::Idle => {
            ui.horizontal(|ui| {
                let can_compile = state.can_compile();
                if ui
                    .add_enabled(can_compile, egui::Button::new("Compile"))
                    .clicked()
                {
                    start_compile(state);
                }

                if !can_compile && state.file_path.is_none() {
                    ui.label(
                        RichText::new("Select a file first")
                            .italics()
                            .color(egui::Color32::GRAY),
                    );
                }
            });
        }
        CompilationState::Compiling => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Compiling...");
            });
        }
        CompilationState::Success => render_success_section(ui, state),
        CompilationState::Failed => render_error_section(ui, state),
    }
}

fn render_compiler_options(ui: &mut Ui, options: &mut VerilogADialogOptions) {
    ui.horizontal(|ui| {
        ui.checkbox(&mut options.strict_mode, "Strict LRM mode");
        ui.add_space(16.0);
        ui.checkbox(&mut options.enable_ams, "Enable Verilog-AMS");
    });

    ui.add_space(4.0);
    ui.label(RichText::new("Include Paths:").strong().size(11.0));
    render_include_paths(ui, options);

    ui.add_space(4.0);
    ui.label(RichText::new("Preprocessor Defines:").strong().size(11.0));
    render_defines(ui, options);
}

fn render_include_paths(ui: &mut Ui, options: &mut VerilogADialogOptions) {
    let mut remove_idx = None;
    for (idx, path) in options.include_paths.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(path.to_string_lossy().to_string());
            if ui.small_button("X").clicked() {
                remove_idx = Some(idx);
            }
        });
    }
    if let Some(idx) = remove_idx {
        options.remove_include_path(idx);
    }

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(&mut options.new_include_path)
                .desired_width(250.0)
                .hint_text("Add include path..."),
        );
        if ui.button("+").clicked() && !options.new_include_path.is_empty() {
            options.add_include_path(PathBuf::from(&options.new_include_path));
            options.new_include_path.clear();
        }
    });
}

fn render_defines(ui: &mut Ui, options: &mut VerilogADialogOptions) {
    let mut remove_def_idx = None;
    for (idx, (name, value)) in options.defines.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(format!("{} = {}", name, value));
            if ui.small_button("X").clicked() {
                remove_def_idx = Some(idx);
            }
        });
    }
    if let Some(idx) = remove_def_idx {
        options.remove_define(idx);
    }

    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(&mut options.new_define_name)
                .desired_width(100.0)
                .hint_text("Name"),
        );
        ui.label("=");
        ui.add(
            egui::TextEdit::singleline(&mut options.new_define_value)
                .desired_width(100.0)
                .hint_text("Value"),
        );
        if ui.button("+").clicked() && !options.new_define_name.is_empty() {
            options.add_define(
                options.new_define_name.clone(),
                options.new_define_value.clone(),
            );
            options.new_define_name.clear();
            options.new_define_value.clear();
        }
    });
}

fn render_success_section(ui: &mut Ui, state: &VerilogALoadDialogState) {
    ui.colored_label(
        egui::Color32::from_rgb(100, 200, 100),
        "Compilation successful!",
    );

    if let Some(module) = &state.compiled_module {
        ui.add_space(4.0);
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Module:");
                ui.label(&module.name);
            });
            ui.horizontal(|ui| {
                ui.strong("Ports:");
                ui.label(module.ports.join(", "));
            });

            if !module.parameters.is_empty() {
                ui.add_space(4.0);
                ui.label(RichText::new("Parameters:").strong().size(11.0));
                egui::ScrollArea::vertical()
                    .max_height(120.0)
                    .show(ui, |ui| {
                        for param in &module.parameters {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.label(&param.name);
                                ui.label("=");
                                ui.label(&param.default_value);
                                let range = param.range_str();
                                if !range.is_empty() {
                                    ui.label(RichText::new(range).color(egui::Color32::GRAY));
                                }
                            });
                        }
                    });
            }

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{} internal nodes, {} variables",
                        module.internal_nodes, module.num_variables
                    ))
                    .color(egui::Color32::GRAY)
                    .size(10.0),
                );
            });

            if let Some(deps) = &state.compiled_dependencies {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} dependency file(s) captured for compile cache",
                            deps.len()
                        ))
                        .color(egui::Color32::GRAY)
                        .size(10.0),
                    );
                });
            }
        });
    }
}

fn render_error_section(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    ui.colored_label(
        egui::Color32::from_rgb(255, 100, 100),
        "Compilation failed!",
    );

    if !state.errors.is_empty() {
        ui.add_space(4.0);
        egui::ScrollArea::vertical()
            .max_height(150.0)
            .show(ui, |ui| {
                for err in &state.errors {
                    ui.horizontal(|ui| {
                        let icon = match err.severity {
                            ErrorSeverity::Error => RichText::new("[E]").color(egui::Color32::RED),
                            ErrorSeverity::Warning => {
                                RichText::new("[W]").color(egui::Color32::YELLOW)
                            }
                            ErrorSeverity::Note => RichText::new("[N]").color(egui::Color32::GRAY),
                        };
                        ui.label(icon);

                        let loc = err.location_str();
                        if !loc.is_empty() {
                            ui.label(RichText::new(format!("{}:", loc)).color(egui::Color32::GRAY));
                        }
                        ui.label(&err.message);
                    });
                }
            });
    }

    ui.add_space(4.0);
    if ui.button("Retry Compilation").clicked() {
        start_compile(state);
    }
}
