//! Compile Verilog-A — the egui implementation of
//! `design/app/volta-dialogs-v2.html` §3.
//!
//! One vertical flow: source → options → status. The status zone is a
//! state machine (idle / compiling / success / failed) like the license
//! dialog: compiling shows the elapsed clock over a sweeping progress
//! bar, success shows the module table with port/parameter counts, and
//! failure is a typed problem list with mono file:line locations.
//! Compile lives in the footer as the secondary action; the primary
//! ("Add to library") stays disabled until a compile succeeds.

use std::path::PathBuf;

use egui::{Context, Rect, Sense, Stroke, Ui, vec2};

use super::compile::{poll_compile, start_compile};
use super::options::VerilogADialogOptions;
use super::state::VerilogALoadDialogState;
use super::types::{CompilationState, ErrorSeverity, VerilogADialogResult};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, check_row, chip, mono_input};

/// Render the Verilog-A compile dialog.
///
/// Returns the dialog result indicating user action.
pub fn render_veriloga_load_dialog(
    ctx: &Context,
    state: &mut VerilogALoadDialogState,
) -> VerilogADialogResult {
    if !state.open {
        return VerilogADialogResult::None;
    }

    // Pump the background compile; on the frame it finishes, stamp the
    // wall-clock for the "compiled in X s" line.
    let was_compiling = matches!(state.compilation_state, CompilationState::Compiling);
    poll_compile(state);
    let now = ctx.input(|i| i.time);
    if was_compiling
        && !matches!(state.compilation_state, CompilationState::Compiling)
        && let Some(started) = state.compile_started_at.take()
    {
        state.last_compile_secs = Some((now - started).max(0.0));
    }

    let mut result = VerilogADialogResult::None;
    let mut should_close = false;

    let hint = footer_hint(state);
    let compile_label = match state.compilation_state {
        CompilationState::Idle => "Compile",
        CompilationState::Compiling => "Compiling…",
        CompilationState::Success | CompilationState::Failed => "Recompile",
    };

    let choice = Dialog::new("Library", "Compile Verilog-A", "Add to library")
        .size(DialogSize::Md)
        .secondary(compile_label)
        .ghost("Cancel")
        .hint(&hint)
        .primary_enabled(state.is_success())
        // The browser build's source editor owns the Enter key.
        .primary_on_enter(cfg!(not(target_arch = "wasm32")))
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;

            render_source_section(ui, state);
            ui.add_space(10.0);
            render_options_section(ui, state);
            ui.add_space(10.0);
            render_status_section(ui, state, now);
        });

    match choice {
        DialogChoice::Primary => {
            if state.is_success() {
                result = VerilogADialogResult::AddToLibrary;
                should_close = true;
            }
        }
        DialogChoice::Secondary => {
            // start_compile owns the no-file/bad-extension guards and
            // reports them as typed problems.
            if !matches!(state.compilation_state, CompilationState::Compiling) {
                state.compile_started_at = Some(now);
                state.last_compile_secs = None;
                start_compile(state);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            result = VerilogADialogResult::Cancelled;
            should_close = true;
        }
        DialogChoice::None => {}
    }

    if should_close {
        state.close();
    }

    result
}

/// Footer meta, by compile state: cache totals when idle, live status
/// otherwise.
fn footer_hint(state: &VerilogALoadDialogState) -> String {
    match state.compilation_state {
        CompilationState::Idle => match state.cache_stats {
            Some((entries, bytes)) if entries > 0 => format!(
                "compile cache: {entries} module{} · {}",
                if entries == 1 { "" } else { "s" },
                fmt_bytes(bytes)
            ),
            _ => "compile validates before the model can join the library".to_owned(),
        },
        CompilationState::Compiling => "compiling…".to_owned(),
        CompilationState::Success => "compiled ok — ready to add".to_owned(),
        CompilationState::Failed => {
            let errors = state
                .errors
                .iter()
                .filter(|e| e.severity == ErrorSeverity::Error)
                .count();
            format!("{errors} error{}", if errors == 1 { "" } else { "s" })
        }
    }
}

// ---------------------------------------------------------------------------
// source + options
// ---------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
fn render_source_section(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    caption(ui, "SOURCE");
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), t.metrics.row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let browse_reserve = 84.0;
            let width = (ui.available_width() - browse_reserve).max(80.0);
            if mono_input(ui, &mut state.file_path_text, width).changed() {
                sync_file_path_text(state);
            }
            if Button::new("Browse…").ghost().show(ui).clicked()
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

/// The browser build has no filesystem: the module text is pasted into a
/// mono editor and compiled from memory (`include is not resolved).
#[cfg(target_arch = "wasm32")]
fn render_source_section(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    let c = Tokens::get(ui.ctx()).color;

    caption(ui, "SOURCE");
    let response = ui.add(
        egui::TextEdit::multiline(&mut state.source_text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .hint_text("paste Verilog-A source — `module … endmodule`")
            .desired_rows(10)
            .desired_width(f32::INFINITY),
    );
    if response.changed() {
        state.reset_compile_outcome();
    }
    ui.add_space(2.0);
    ui.label(
        egui::RichText::new(
            "compiles the pasted text — `include directives are not resolved in the browser",
        )
        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
        .color(c.text_faint),
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn sync_file_path_text(state: &mut VerilogALoadDialogState) {
    if state.file_path_text.is_empty() {
        state.file_path = None;
    } else {
        state.file_path = Some(PathBuf::from(&state.file_path_text));
    }
    state.reset_compile_outcome();
}

fn render_options_section(ui: &mut Ui, state: &mut VerilogALoadDialogState) {
    caption(ui, "OPTIONS");
    check_row(ui, "Strict LRM mode", &mut state.options.strict_mode);
    check_row(ui, "Enable Verilog-AMS", &mut state.options.enable_ams);

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let n_extra = state.options.include_paths.len() + state.options.defines.len();
        let label = if n_extra > 0 {
            format!("include paths · defines ({n_extra})")
        } else {
            "include paths · defines".to_owned()
        };
        if chip(ui, &label, state.show_advanced_options).clicked() {
            state.show_advanced_options = !state.show_advanced_options;
        }
    });
    if state.show_advanced_options {
        ui.add_space(4.0);
        render_advanced_options(ui, &mut state.options);
    }
}

fn render_advanced_options(ui: &mut Ui, options: &mut VerilogADialogOptions) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

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

// ---------------------------------------------------------------------------
// status zone — the idle / compiling / success / failed state machine
// ---------------------------------------------------------------------------

fn render_status_section(ui: &mut Ui, state: &mut VerilogALoadDialogState, now: f64) {
    match state.compilation_state {
        CompilationState::Idle => {
            caption(ui, "STATUS");
            inset(ui, |ui| {
                let c = Tokens::get(ui.ctx()).color;
                ui.label(
                    egui::RichText::new(
                        "Not compiled yet — Compile validates the module before it can join the library.",
                    )
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_faint),
                );
            });
        }
        CompilationState::Compiling => render_compiling_pane(ui, state, now),
        CompilationState::Success => render_success_pane(ui, state),
        CompilationState::Failed => render_failed_pane(ui, state),
    }
}

fn render_compiling_pane(ui: &mut Ui, state: &VerilogALoadDialogState, now: f64) {
    let c = Tokens::get(ui.ctx()).color;
    caption(ui, "STATUS");
    inset(ui, |ui| {
        let file = state
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "module".to_owned());
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("compiling {file}…"))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_dim),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let elapsed = state.compile_started_at.map_or(0.0, |t0| (now - t0).max(0.0));
                ui.label(
                    egui::RichText::new(format!("{elapsed:.1} s"))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            });
        });
        ui.add_space(8.0);
        progress_sweep(ui, now);
    });
    // The clock and the sweep animate; keep frames coming while we wait —
    // at animation rate, not vsync, since this runs for the whole compile.
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(33));
}

/// Indeterminate progress: a thin track with an accent segment sweeping
/// left to right (the spec's `.prog`).
fn progress_sweep(ui: &mut Ui, now: f64) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 3.0), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 1.5, c.bg_hover);
    let seg_w = rect.width() * 0.28;
    let span = rect.width() + seg_w;
    let phase = ((now * 0.45) % 1.0) as f32;
    let x0 = rect.left() - seg_w + span * phase;
    let seg = Rect::from_min_max(
        egui::pos2(x0.max(rect.left()), rect.top()),
        egui::pos2((x0 + seg_w).min(rect.right()), rect.bottom()),
    );
    if seg.width() > 1.0 {
        painter.rect_filled(seg, 1.5, c.accent);
    }
}

fn render_success_pane(ui: &mut Ui, state: &VerilogALoadDialogState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    caption(ui, "MODULES");
    let Some(module) = &state.compiled_module else {
        return;
    };

    inset(ui, |ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 24.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                badge(ui, "OK", c.ok);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&module.name)
                        .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                        .color(c.text),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut meta = format!(
                        "{} port{} · {} param{}",
                        module.ports.len(),
                        if module.ports.len() == 1 { "" } else { "s" },
                        module.parameters.len(),
                        if module.parameters.len() == 1 { "" } else { "s" },
                    );
                    if module.internal_nodes > 0 {
                        meta.push_str(&format!(" · {} internal", module.internal_nodes));
                    }
                    ui.label(
                        egui::RichText::new(meta)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_dim),
                    );
                });
            },
        );
        if !module.ports.is_empty() {
            ui.add_space(2.0);
            ui.label(
                egui::RichText::new(module.ports.join("  "))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        }
    });

    // The ok line: wall clock + dependency count.
    let mut line = String::new();
    if let Some(secs) = state.last_compile_secs {
        line.push_str(&format!("compiled in {secs:.1} s"));
    } else {
        line.push_str("compiled");
    }
    match state.compiled_dependencies.as_ref().map(Vec::len) {
        Some(1) => line.push_str(" · 1 include"),
        Some(n) if n > 1 => line.push_str(&format!(" · {n} includes")),
        _ => {}
    }
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(line)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(c.ok),
    );

    if !module.parameters.is_empty() {
        ui.add_space(8.0);
        caption(ui, "PARAMETERS");
        inset(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("va_params_scroll")
                .max_height(110.0)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 1.0;
                    for param in &module.parameters {
                        let range = param.range_str();
                        let line = if range.is_empty() {
                            format!("{} = {}", param.name, param.default_value)
                        } else {
                            format!("{} = {}  {}", param.name, param.default_value, range)
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

fn render_failed_pane(ui: &mut Ui, state: &VerilogALoadDialogState) {
    let errors = state
        .errors
        .iter()
        .filter(|e| e.severity == ErrorSeverity::Error)
        .count();
    let warnings = state
        .errors
        .iter()
        .filter(|e| e.severity == ErrorSeverity::Warning)
        .count();
    let mut title = format!("PROBLEMS · {errors} ERROR{}", if errors == 1 { "" } else { "S" });
    if warnings > 0 {
        title.push_str(&format!(
            " · {warnings} WARNING{}",
            if warnings == 1 { "" } else { "S" }
        ));
    }
    caption(ui, &title);

    inset(ui, |ui| {
        egui::ScrollArea::vertical()
            .id_salt("va_errors_scroll")
            .max_height(150.0)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for err in &state.errors {
                    problem_row(ui, err);
                }
            });
    });
}

/// One problem row: severity dot, mono location, dim message.
fn problem_row(ui: &mut Ui, err: &super::types::CompileErrorDisplay) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter();
    let dot_color = match err.severity {
        ErrorSeverity::Error => c.err,
        ErrorSeverity::Warning => c.warn,
        ErrorSeverity::Note => c.text_faint,
    };
    painter.circle_filled(egui::pos2(rect.left() + 5.0, rect.center().y), 3.0, dot_color);

    let mut x = rect.left() + 16.0;
    let location = err.location_str();
    if !location.is_empty() {
        let galley = ui.fonts(|f| {
            f.layout_no_wrap(
                location,
                theme::mono(tokens::FS_0, FontWeight::Medium),
                c.text,
            )
        });
        painter.galley(
            egui::pos2(x, rect.center().y - galley.size().y * 0.5),
            galley.clone(),
            c.text,
        );
        x += galley.size().x + 8.0;
    }

    let remaining = (rect.right() - x).max(20.0);
    let message = ui.fonts(|f| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            err.message.clone(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            egui::Color32::PLACEHOLDER,
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width(remaining);
        f.layout_job(job)
    });
    painter.galley(
        egui::pos2(x, rect.center().y - message.size().y * 0.5),
        message,
        c.text_dim,
    );

    if err.message.len() > 60 {
        response.on_hover_text(&err.message);
    }
}

// ---------------------------------------------------------------------------
// chrome
// ---------------------------------------------------------------------------

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
    ui.add_space(2.0);
}

/// The spec's `.inset` — a bordered inset card.
fn inset(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::none()
        .fill(c.bg_inset)
        .stroke(Stroke::new(1.0, c.border))
        .rounding(t.radius)
        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui);
        });
}

/// A bordered status badge (the spec's `.badge`).
fn badge(ui: &mut Ui, label: &str, color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        label,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(9.5, FontWeight::Medium),
            color,
            extra_letter_spacing: 0.06 * 9.5,
            ..Default::default()
        },
    );
    let galley = ui.fonts(|f| f.layout_job(job));
    let (rect, _) =
        ui.allocate_exact_size(vec2(galley.size().x + 12.0, 14.0), Sense::hover());
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        egui::Color32::TRANSPARENT,
        Stroke::new(1.0, color.gamma_multiply(0.5)),
    );
    painter.galley(
        egui::pos2(rect.left() + 6.0, rect.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
}

/// Compact byte count for the cache hint.
fn fmt_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    let b = bytes as f64;
    if b >= MB {
        format!("{:.1} MiB", b / MB)
    } else if b >= KB {
        format!("{:.0} KiB", b / KB)
    } else {
        format!("{bytes} B")
    }
}
