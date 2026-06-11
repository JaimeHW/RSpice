//! Verilog-A loader on the modal primitive: file pick, compiler options,
//! compile status with typed error list, and Add-to-Library as the primary
//! action once compilation succeeds.

use std::path::PathBuf;

use egui::{Context, Ui};

use super::compile::{poll_compile, start_compile};
use super::options::VerilogADialogOptions;
use super::state::VerilogALoadDialogState;
use super::types::{CompilationState, ErrorSeverity, VerilogADialogResult};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, check_row, kv_row, mono_input,
};

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

    let hint = match state.compilation_state {
        CompilationState::Idle => "compile to validate".to_owned(),
        CompilationState::Compiling => "compiling…".to_owned(),
        CompilationState::Success => "compiled ok".to_owned(),
        CompilationState::Failed => {
            let errors = state
                .errors
                .iter()
                .filter(|e| e.severity == ErrorSeverity::Error)
                .count();
            format!("{errors} error{}", if errors == 1 { "" } else { "s" })
        }
    };

    let choice = Dialog::new("Library", "Load Verilog-A model", "Add to library")
        .size(DialogSize::Md)
        .ghost("Cancel")
        .hint(&hint)
        .primary_enabled(state.is_success())
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;

            render_file_selection(ui, state);
            ui.add_space(8.0);
            render_compiler_options(ui, &mut state.options);
            ui.add_space(8.0);
            render_compilation_status(ui, state);
        });

    match choice {
        DialogChoice::Primary => {
            if state.is_success() {
                result = VerilogADialogResult::AddToLibrary;
                should_close = true;
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            result = VerilogADialogResult::Cancelled;
            should_close = true;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }

    if should_close {
        state.close();
    }

    result
}

/// Faint mono section caption.
fn caption(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        text,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_faint,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
}

fn render_file_selection(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), t.metrics.row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) =
                ui.allocate_exact_size(egui::vec2(92.0, t.metrics.row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                "Source file",
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            let browse_reserve = 76.0;
            let width = (ui.available_width() - browse_reserve).max(80.0);
            if mono_input(ui, &mut state.file_path_text, width).changed() {
                sync_file_path_text(state);
            }
            // Native file pickers don't exist in the browser build; the
            // mono path input above remains the entry point there.
            #[cfg(not(target_arch = "wasm32"))]
            if Button::new("Browse").ghost().show(ui).clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("Verilog-A", &["va", "vams"])
                    .pick_file()
            {
                state.set_file_path(path);
            }
        },
    );

    if let Some(path) = &state.file_path
        && !path.exists()
    {
        ui.label(
            egui::RichText::new("File not found")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.err),
        );
    }
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
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    match state.compilation_state {
        CompilationState::Idle => {
            if Button::new("Compile")
                .accent()
                .enabled(state.can_compile())
                .min_width(ui.available_width())
                .show(ui)
                .clicked()
            {
                start_compile(state);
            }
        }
        CompilationState::Compiling => {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(
                    egui::RichText::new("Compiling…")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                );
            });
        }
        CompilationState::Success => render_success_section(ui, state),
        CompilationState::Failed => render_error_section(ui, state),
    }
}

fn render_compiler_options(ui: &mut Ui, options: &mut VerilogADialogOptions) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    caption(ui, "COMPILER");
    check_row(ui, "Strict LRM mode", &mut options.strict_mode);
    check_row(ui, "Enable Verilog-AMS", &mut options.enable_ams);

    ui.add_space(6.0);
    caption(ui, "INCLUDE PATHS");
    let mut remove_idx = None;
    for (idx, path) in options.include_paths.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.label(
                egui::RichText::new(path.to_string_lossy().to_string())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_dim),
            );
            if Button::new("remove").ghost().show(ui).clicked() {
                remove_idx = Some(idx);
            }
        });
    }
    if let Some(idx) = remove_idx {
        options.remove_include_path(idx);
    }
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let width = (ui.available_width() - 60.0).max(80.0);
        mono_input(ui, &mut options.new_include_path, width);
        if Button::new("Add").ghost().show(ui).clicked() && !options.new_include_path.is_empty() {
            options.add_include_path(PathBuf::from(&options.new_include_path));
            options.new_include_path.clear();
        }
    });

    ui.add_space(6.0);
    caption(ui, "PREPROCESSOR DEFINES");
    let mut remove_def_idx = None;
    for (idx, (name, value)) in options.defines.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            ui.label(
                egui::RichText::new(format!("{name} = {value}"))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_dim),
            );
            if Button::new("remove").ghost().show(ui).clicked() {
                remove_def_idx = Some(idx);
            }
        });
    }
    if let Some(idx) = remove_def_idx {
        options.remove_define(idx);
    }
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        let width = ((ui.available_width() - 60.0) * 0.5 - 6.0).max(60.0);
        mono_input(ui, &mut options.new_define_name, width);
        mono_input(ui, &mut options.new_define_value, width);
        if Button::new("Add").ghost().show(ui).clicked() && !options.new_define_name.is_empty() {
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
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.label(
        egui::RichText::new("Compilation succeeded")
            .font(theme::sans(tokens::FS_1, FontWeight::Medium))
            .color(c.ok),
    );

    if let Some(module) = &state.compiled_module {
        ui.spacing_mut().item_spacing.y = 0.0;
        kv_row(ui, "Module", &module.name);
        kv_row(ui, "Ports", &module.ports.join(", "));
        kv_row(
            ui,
            "Internals",
            &format!(
                "{} nodes · {} variables",
                module.internal_nodes, module.num_variables
            ),
        );
        if let Some(deps) = &state.compiled_dependencies {
            kv_row(ui, "Dependencies", &deps.len().to_string());
        }

        if !module.parameters.is_empty() {
            ui.add_space(6.0);
            caption(ui, "PARAMETERS");
            egui::Frame::none()
                .fill(c.bg_inset)
                .stroke(egui::Stroke::new(1.0, c.border))
                .rounding(t.radius)
                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    egui::ScrollArea::vertical()
                        .id_salt("va_params_scroll")
                        .max_height(120.0)
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 1.0;
                            for param in &module.parameters {
                                let range = param.range_str();
                                let line = if range.is_empty() {
                                    format!("{} = {}", param.name, param.default_value)
                                } else {
                                    format!(
                                        "{} = {}  {}",
                                        param.name, param.default_value, range
                                    )
                                };
                                ui.label(
                                    egui::RichText::new(line)
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(c.text_dim),
                                );
                            }
                        });
                });
        }
    }
}

fn render_error_section(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    ui.label(
        egui::RichText::new("Compilation failed")
            .font(theme::sans(tokens::FS_1, FontWeight::Medium))
            .color(c.err),
    );

    if !state.errors.is_empty() {
        ui.add_space(4.0);
        egui::ScrollArea::vertical()
            .id_salt("va_errors_scroll")
            .max_height(150.0)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 1.0;
                for err in &state.errors {
                    let (tag, color) = match err.severity {
                        ErrorSeverity::Error => ("E", c.err),
                        ErrorSeverity::Warning => ("W", c.warn),
                        ErrorSeverity::Note => ("N", c.text_faint),
                    };
                    let loc = err.location_str();
                    let line = if loc.is_empty() {
                        format!("{tag}  {}", err.message)
                    } else {
                        format!("{tag}  {loc}: {}", err.message)
                    };
                    ui.label(
                        egui::RichText::new(line)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(color),
                    );
                }
            });
    }

    ui.add_space(6.0);
    if Button::new("Retry compilation")
        .min_width(ui.available_width())
        .show(ui)
        .clicked()
    {
        start_compile(state);
    }
}
