//! Mockup-defined Verilog-A compiler workspace.

use egui::{Align, Layout, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::state::ProjectSourceLanguage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::code_workspace::{
    CodeEditorLanguage, CodeEditorSeverity, CodeWorkspacePage, TargetQualification,
    show_code_editor,
};
use super::super::design_system::{
    StatusMark, code_inspector_property_list, code_inspector_section, code_workspace_heading,
    property_row, property_row_status, property_row_toned, workspace_title_row,
};

const INTERNAL_INSPECTOR_MIN_WIDTH: f32 = 270.0;
const INTERNAL_INSPECTOR_WIDTH: f32 = 330.0;
const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 560.0;

fn title_actions_stack(available_width: f32) -> bool {
    available_width <= TITLE_ACTION_STACK_BREAKPOINT
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    super::super::code_workspace::poll_veriloga_import(app);
    super::super::code_workspace::poll_veriloga_compile(app);
    let t = Tokens::get(ui.ctx());
    let document = app
        .state
        .workspace
        .project_sources
        .get(ProjectSourceLanguage::VerilogA)
        .cloned();
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let file_name = document
            .as_ref()
            .map_or("Verilog-A source", |source| source.file_name());
        title_row(ui, app, file_name);
        let Some(document) = document else {
            super::super::design_system::empty_state(
                ui,
                super::super::design_system::WorkbenchIcon::Code,
                "No Verilog-A source",
                "This project does not own a Verilog-A source document.",
            );
            return;
        };
        let mut source = document.content().to_owned();
        let diagnostics = current_diagnostics(app, &document);
        let stacked = ui.ctx().content_rect().width() <= 820.0;
        if stacked {
            ScrollArea::vertical()
                .id_salt("workbench.veriloga.stacked")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    let editor_height = ui.clip_rect().height().max(1.0);
                    let code_pane =
                        ui.allocate_ui(Vec2::new(ui.available_width(), editor_height), |ui| {
                            code_pane(ui, app, &document, &mut source, &diagnostics);
                        });
                    ui.painter().hline(
                        code_pane.response.rect.x_range(),
                        code_pane.response.rect.bottom(),
                        Stroke::new(1.0, t.color.border),
                    );
                    egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        inspector(ui, app, &document);
                    });
                });
        } else {
            let available = ui.available_rect_before_wrap();
            let inspector_width = INTERNAL_INSPECTOR_WIDTH
                .min((available.width() * 0.42).max(INTERNAL_INSPECTOR_MIN_WIDTH));
            let editor_rect = egui::Rect::from_min_max(
                available.min,
                egui::pos2(available.right() - inspector_width, available.bottom()),
            );
            let inspector_rect = egui::Rect::from_min_max(
                egui::pos2(editor_rect.right(), available.top()),
                available.max,
            );
            ui.painter()
                .rect_filled(inspector_rect, 0.0, t.color.bg_panel);
            let mut editor_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(editor_rect)
                    .layout(Layout::top_down(Align::Min)),
            );
            code_pane(&mut editor_ui, app, &document, &mut source, &diagnostics);
            ui.painter().vline(
                editor_rect.right(),
                editor_rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
            let mut inspector_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(inspector_rect)
                    .layout(Layout::top_down(Align::Min)),
            );
            ScrollArea::vertical()
                .id_salt("workbench.veriloga.inspector")
                .auto_shrink([false, false])
                .show(&mut inspector_ui, |ui| inspector(ui, app, &document));
            ui.allocate_rect(available, Sense::hover());
        }
    });
}

fn title_row(ui: &mut Ui, app: &mut RSpiceApp, file_name: &str) {
    workspace_title_row(ui, |ui| {
        // Open docks can make the workspace much narrower than the viewport.
        // Stack against the actual row width so actions never overlap.
        if title_actions_stack(ui.available_width()) {
            code_workspace_heading(
                ui,
                "VERILOG-A · SEMANTIC COMPILE · MULTI-RUNTIME",
                "Behavioral-model compiler",
                "Canonical IR, interpreted WebAssembly, native-JIT preview and generated-Rust qualification paths.",
            );
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
                generated_netlist_button(ui, app, width);
                compile_button(ui, app, file_name, width);
            });
        } else {
            let compile_label = compile_button_label(app, file_name);
            let actions_width = title_button_width(ui, "Generated netlist", false)
                + 6.0
                + title_button_width(ui, &compile_label, true);
            let heading_width = (ui.available_width() - actions_width - 12.0).max(1.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.allocate_ui_with_layout(
                    Vec2::new(heading_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        // `allocate_ui_with_layout` is content-sized unless
                        // the child claims its width. The mockup's flex-grow
                        // heading must consume the measured remainder so the
                        // action group stays flush right.
                        ui.set_width(heading_width);
                        code_workspace_heading(
                            ui,
                            "VERILOG-A · SEMANTIC COMPILE · MULTI-RUNTIME",
                            "Behavioral-model compiler",
                            "Canonical IR, interpreted WebAssembly, native-JIT preview and generated-Rust qualification paths.",
                        );
                    },
                );
                ui.add_space(6.0);
                generated_netlist_button(ui, app, 0.0);
                compile_button(ui, app, file_name, 0.0);
            });
        }
    });
}

fn generated_netlist_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let mut button = Button::new("Generated netlist");
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        let _ = super::super::netlist_document::open_generated_primary(&mut app.state);
        app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    }
}

fn compile_button(ui: &mut Ui, app: &mut RSpiceApp, file_name: &str, width: f32) {
    let label = compile_button_label(app, file_name);
    let pending = app.state.ui.code_workspace.veriloga.pending.is_some();
    let mut button = Button::new(&label).accent().enabled(!pending);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        super::super::code_workspace::start_veriloga_compile(app, ui.ctx().clone());
    }
}

fn compile_button_label(app: &RSpiceApp, file_name: &str) -> String {
    let pending = app.state.ui.code_workspace.veriloga.pending.is_some();
    if pending {
        format!("Compiling {file_name}…")
    } else {
        format!("Compile {file_name}")
    }
}

fn title_button_width(ui: &Ui, label: &str, accent: bool) -> f32 {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(
        tokens::FS_0,
        if accent {
            FontWeight::SemiBold
        } else {
            FontWeight::Regular
        },
    );
    let content = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, t.color.text)
        .size()
        .x
        + 20.0;
    content.max(if t.metrics.ctl_h >= 44.0 { 44.0 } else { 0.0 })
}

fn code_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    document: &crate::state::ProjectSourceDocument,
    source: &mut String,
    diagnostics: &[super::super::code_workspace::CodeEditorDiagnostic],
) {
    let t = Tokens::get(ui.ctx());
    let (toolbar, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), EDITOR_TOOLBAR_HEIGHT),
        Sense::hover(),
    );
    ui.painter().rect_filled(toolbar, 0.0, t.color.bg_panel);
    ui.painter().hline(
        toolbar.x_range(),
        toolbar.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(toolbar.left() + 8.0, toolbar.center().y),
        egui::Align2::LEFT_CENTER,
        document.file_name(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let compiling = app.state.ui.code_workspace.veriloga.pending.is_some();
    let receipt_current = app
        .state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .is_some_and(|receipt| {
            receipt.token.project_id == app.state.workspace.project.id()
                && receipt.token.revision == document.revision().get()
                && receipt.token.content_digest == document.content_digest()
        });
    let (status, color) = if compiling {
        ("compiling", t.color.info)
    } else if receipt_current {
        ("semantic checks pass", t.color.ok)
    } else {
        ("modified · compile required", t.color.warn)
    };
    let status_galley = ui.painter().layout_no_wrap(
        status.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        color,
    );
    ui.painter().circle_filled(
        egui::pos2(
            toolbar.right() - status_galley.size().x - 17.0,
            toolbar.center().y,
        ),
        2.5,
        color,
    );
    ui.painter().galley(
        egui::pos2(
            toolbar.right() - status_galley.size().x - 8.0,
            toolbar.center().y - status_galley.size().y * 0.5,
        ),
        status_galley,
        color,
    );
    egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        if show_code_editor(
            ui,
            "workbench.veriloga.source",
            source,
            CodeEditorLanguage::VerilogA,
            diagnostics,
        ) {
            match app
                .state
                .workspace
                .replace_project_source(ProjectSourceLanguage::VerilogA, source.clone())
            {
                Ok(true) => {
                    app.state.ui.code_workspace.veriloga.receipt = None;
                    app.state.ui.code_workspace.veriloga.last_failure.clear();
                    app.state.ui.code_workspace.veriloga.last_failure_token = None;
                }
                Ok(false) => {}
                Err(error) => app
                    .state
                    .push_user_message(crate::common::ConsoleMessage::error(format!(
                        "Could not update {}: {error}",
                        document.file_name()
                    ))),
            }
        }
    });
}

fn current_diagnostics(
    app: &RSpiceApp,
    document: &crate::state::ProjectSourceDocument,
) -> Vec<super::super::code_workspace::CodeEditorDiagnostic> {
    let state = &app.state.ui.code_workspace.veriloga;
    current_receipt(app, document)
        .map(|receipt| receipt.diagnostics.clone())
        .unwrap_or_else(|| {
            state
                .last_failure_token
                .filter(|token| {
                    token.project_id == app.state.workspace.project.id()
                        && token.revision == document.revision().get()
                        && token.content_digest == document.content_digest()
                })
                .map_or_else(Vec::new, |_| state.last_failure.clone())
        })
}

fn current_receipt<'a>(
    app: &'a RSpiceApp,
    document: &crate::state::ProjectSourceDocument,
) -> Option<&'a super::super::code_workspace::VerilogACompileReceipt> {
    app.state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .filter(|receipt| {
            receipt.token.project_id == app.state.workspace.project.id()
                && receipt.token.revision == document.revision().get()
                && receipt.token.content_digest == document.content_digest()
        })
}

fn inspector(ui: &mut Ui, app: &RSpiceApp, document: &crate::state::ProjectSourceDocument) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(ui.available_width());
    code_inspector_section(ui, "Build targets", None, |ui| {
        code_inspector_property_list(ui, |ui| {
            if let Some(receipt) = current_receipt(app, document) {
                property_row_status(
                    ui,
                    "Semantic IR",
                    "canonical",
                    t.color.ok,
                    StatusMark::Success,
                );
                if receipt.bytecode_available {
                    property_row_status(
                        ui,
                        "Bytecode VM",
                        "available",
                        t.color.ok,
                        StatusMark::Success,
                    );
                } else {
                    property_row_status(
                        ui,
                        "Bytecode VM",
                        "unavailable",
                        t.color.err,
                        StatusMark::Failure,
                    );
                }
                target_row(ui, "Native x64 JIT", &receipt.native_jit);
                target_row(ui, "WASM interpreter", &receipt.wasm_interpreter);
                target_row(ui, "Generated Rust", &receipt.generated_rust);
            } else {
                property_row(ui, "Semantic IR", "compile required");
                property_row(ui, "Bytecode VM", "compile required");
                property_row(ui, "Native x64 JIT", "compile required");
                property_row(ui, "WASM interpreter", "compile required");
                property_row(ui, "Generated Rust", "compile required");
            }
        });
    });

    let diagnostics = current_diagnostics(app, document);
    let error_count = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == CodeEditorSeverity::Error)
        .count();
    let advisory_count = diagnostics.len().saturating_sub(error_count);
    let diagnostic_status = if error_count > 0 {
        format!(
            "{error_count} error{}",
            if error_count == 1 { "" } else { "s" }
        )
    } else if advisory_count > 0 {
        format!(
            "{advisory_count} advisor{}",
            if advisory_count == 1 { "y" } else { "ies" }
        )
    } else {
        "clean".to_owned()
    };
    let diagnostic_tone = if error_count > 0 {
        t.color.err
    } else if advisory_count > 0 {
        t.color.warn
    } else {
        t.color.ok
    };
    code_inspector_section(
        ui,
        "Diagnostics",
        Some((&diagnostic_status, diagnostic_tone)),
        |ui| {
            if diagnostics.is_empty() {
                muted_row(ui, "No compiler diagnostics for the current source.");
            } else {
                for diagnostic in diagnostics {
                    diagnostic_row(ui, &diagnostic);
                }
            }
        },
    );

    code_inspector_section(ui, "ABI contract", None, |ui| {
        code_inspector_property_list(ui, |ui| {
            if let Some(receipt) = current_receipt(app, document) {
                property_row(ui, "Analog ports", &receipt.analog_ports.to_string());
                property_row(ui, "Noise sources", &receipt.noise_sources.to_string());
                property_row(ui, "State variables", &receipt.state_variables.to_string());
            } else {
                property_row(ui, "Analog ports", "—");
                property_row(ui, "Noise sources", "—");
                property_row(ui, "State variables", "—");
            }
        });
    });
}

fn target_row(ui: &mut Ui, label: &str, target: &TargetQualification) {
    let t = Tokens::get(ui.ctx());
    let (value, tone, mark) = match target {
        TargetQualification::Available => (
            "available".to_owned(),
            t.color.ok,
            Some(StatusMark::Success),
        ),
        TargetQualification::Preview => (
            "preview".to_owned(),
            t.color.warn,
            Some(StatusMark::Warning),
        ),
        TargetQualification::QualificationOnly => {
            ("qualification only".to_owned(), t.color.text, None)
        }
        TargetQualification::Unsupported(reason) => (
            format!("unsupported · {reason}"),
            t.color.err,
            Some(StatusMark::Failure),
        ),
        TargetQualification::Failed(reason) => (
            format!("failed · {reason}"),
            t.color.err,
            Some(StatusMark::Failure),
        ),
    };
    if let Some(mark) = mark {
        property_row_status(ui, label, &value, tone, mark);
    } else {
        property_row_toned(ui, label, &value, tone);
    }
}

fn muted_row(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

fn diagnostic_row(ui: &mut Ui, diagnostic: &super::super::code_workspace::CodeEditorDiagnostic) {
    let t = Tokens::get(ui.ctx());
    let tone = match diagnostic.severity {
        CodeEditorSeverity::Info => t.color.info,
        CodeEditorSeverity::Warning => t.color.warn,
        CodeEditorSeverity::Error => t.color.err,
    };
    let shown = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 7.0;
                let (icon, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                super::super::design_system::WorkbenchIcon::Warning.paint(ui.painter(), icon, tone);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 3.0;
                    ui.label(
                        egui::RichText::new(&diagnostic.message)
                            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text),
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&diagnostic.detail)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        )
                        .wrap(),
                    );
                });
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_actions_follow_workspace_width_not_viewport_width() {
        assert!(title_actions_stack(560.0));
        assert!(!title_actions_stack(561.0));
    }
}
