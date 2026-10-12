//! The Verilog-A source editor.
//!
//! A Verilog-A module is a view of a cell, so its editor is a Design-workspace
//! document: [`show_in_design`] draws it there, editor only, while the
//! inspector dock states the module's facts. Until the project-wide source
//! workspace is retired, the Netlist workspace's Verilog-A page draws the same
//! editor with those facts in a column of its own ([`show`]). Both hosts edit
//! the bundle `code_workspace::active_source_owner` names, so they can never
//! disagree about which module is in front of the reader.

use egui::{Align, Layout, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::{MessageCatalog, MessageId, RSpiceApp};

use super::super::design_system::{code_workspace_heading, workspace_title_row};
use crate::workbench::documents::code_workspace::{
    CodeDiagnosticCollection, CodeEditorLanguage, CodeWorkspacePage,
    show_code_document_interaction_versioned,
};

const INTERNAL_INSPECTOR_MIN_WIDTH: f32 = 270.0;
const INTERNAL_INSPECTOR_WIDTH: f32 = 330.0;
const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;
const EDITOR_SCROLLBAR_INSET: f32 = 14.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 680.0;

fn title_actions_stack(available_width: f32) -> bool {
    available_width <= TITLE_ACTION_STACK_BREAKPOINT
}

fn visible_workspace_rect(ui: &Ui) -> egui::Rect {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .intersect(ui.max_rect())
}

/// Which workspace is drawing the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Host {
    /// The Design workspace, whose inspector dock states the module's facts.
    Design,
    /// The Netlist workspace's Verilog-A page, which states them itself.
    NetlistPage,
}

/// The Netlist workspace's Verilog-A page.
pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    show_hosted(ui, app, Host::NetlistPage);
}

/// A Verilog-A cell view, as the Design workspace's document.
pub fn show_in_design(ui: &mut Ui, app: &mut RSpiceApp) {
    show_hosted(ui, app, Host::Design);
}

fn show_hosted(ui: &mut Ui, app: &mut RSpiceApp, host: Host) {
    // Import and compile results are consumed by the frame loop, so a result
    // that lands while neither host is on screen is still published. Only
    // what needs this surface's pointer and context stays here.
    handle_veriloga_file_drop(ui.ctx(), app);
    compile_dialog_window(ui.ctx(), app);
    let t = Tokens::get(ui.ctx());
    let selected = crate::workbench::documents::code_workspace::selected_veriloga_source(app);
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let default_file_name = app
            .state
            .ui
            .messages()
            .text(MessageId::VerilogASourceDefault);
        let file_name = selected
            .as_ref()
            .map_or(default_file_name.as_str(), |source| {
                source.document().file_name()
            });
        // In Design the row names the module the tab holds; the compiler
        // page's own heading is the Netlist page's.
        let module_title = (host == Host::Design).then(|| {
            selected
                .as_ref()
                .ok()
                .and_then(|source| source.selected_module().map(str::to_owned))
                .unwrap_or_else(|| app.state.workspace.content.active_view.cell.clone())
        });
        title_row(ui, app, file_name, module_title.as_deref());
        let selected = match selected {
            Ok(selected) => selected,
            Err(error) => {
                missing_source(ui, app, error);
                return;
            }
        };
        let active_path =
            crate::workbench::documents::code_workspace::active_veriloga_file_path(app, &selected);
        let editor_language = if selected.bundle().role_for_path(&active_path)
            == Some(crate::state::ProjectSourceRole::VerilogABuildProfile)
        {
            CodeEditorLanguage::Toml
        } else {
            CodeEditorLanguage::VerilogA
        };
        let mut source = selected
            .bundle()
            .file_content(&active_path)
            .unwrap_or_else(|| selected.document().content())
            .to_owned();
        let diagnostics = current_diagnostics(app, &selected);
        if host == Host::Design {
            // The inspector dock states the module's facts, so the editor
            // takes the whole document well at every width. A second copy of
            // them beside or below it would only push the source off a phone.
            let available = ui.available_rect_before_wrap();
            let mut editor_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(available)
                    .layout(Layout::top_down(Align::Min)),
            );
            code_pane(
                &mut editor_ui,
                app,
                &selected,
                &active_path,
                &mut source,
                &diagnostics,
                editor_language,
            );
            ui.allocate_rect(available, Sense::hover());
            return;
        }
        // Match the mockup's 820 px content-column breakpoint. Docked panels
        // can narrow this surface while the outer viewport remains wide.
        let stacked = visible_workspace_rect(ui).width() <= 820.0;
        if stacked {
            let pane_size = visible_workspace_rect(ui).size().max(Vec2::splat(1.0));
            // The responsive viewport occupies the same document well on all
            // code pages. A parent-independent identity is safe because the
            // pages are mutually exclusive and prevents egui from treating a
            // tab switch as an unstable scrollbar replacement.
            ui.scope_builder(
                egui::UiBuilder::new().id(egui::Id::new("workbench.code-workspace.stacked-scope")),
                |ui| {
                    ScrollArea::vertical()
                        .id_salt("viewport")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            // A vertical ScrollArea intentionally exposes unbounded
                            // content height and can also report a wider child before
                            // its first sizing pass. Pin the child to the measured
                            // workspace pane so toolbars never lay out off-screen.
                            ui.set_width(pane_size.x);
                            let editor_height = pane_size.y;
                            let code_pane =
                                ui.allocate_ui(Vec2::new(pane_size.x, editor_height), |ui| {
                                    code_pane(
                                        ui,
                                        app,
                                        &selected,
                                        &active_path,
                                        &mut source,
                                        &diagnostics,
                                        editor_language,
                                    );
                                });
                            ui.painter().hline(
                                code_pane.response.rect.x_range(),
                                code_pane.response.rect.bottom(),
                                Stroke::new(1.0, t.color.border),
                            );
                            egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                inspector(ui, app, &selected);
                            });
                        });
                },
            );
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
            code_pane(
                &mut editor_ui,
                app,
                &selected,
                &active_path,
                &mut source,
                &diagnostics,
                editor_language,
            );
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
                .show(&mut inspector_ui, |ui| inspector(ui, app, &selected));
            ui.allocate_rect(available, Sense::hover());
        }
    });
}

fn compile_dialog_window(ctx: &egui::Context, app: &mut RSpiceApp) {
    let Some(draft) = app.state.ui.code_workspace.veriloga.compile_dialog.clone() else {
        return;
    };
    let messages = app.state.ui.messages();
    let choice = Dialog::new(
        messages.text(MessageId::VerilogACompileReviewEyebrow),
        messages.text(MessageId::VerilogACompileReviewTitle),
        messages.text(MessageId::VerilogACompileReviewPrimary),
    )
    .description(messages.text(MessageId::VerilogACompileReviewDescription))
    .size(DialogSize::Transaction)
    .initial_focus(DialogInitialFocus::BodyControl)
    .primary_enabled(draft.error.is_none())
    .ghost(messages.text(MessageId::CommonCancel))
    .show_with_initial_body_focus(ctx, |ui| {
        let t = Tokens::get(ctx);
        ScrollArea::vertical()
            .id_salt("workbench.veriloga.compile-review")
            .max_height((ctx.content_rect().height() - 220.0).max(240.0))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewIdentity),
                    |ui| {
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewPackage),
                            format!("{} {}", draft.package_name, draft.package_version),
                        );
                        let entry_module = if draft.selected_module.is_empty() {
                            messages.text(MessageId::VerilogACompileReviewAutomaticModule)
                        } else {
                            draft.selected_module.clone()
                        };
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewEntryModule),
                            entry_module,
                        );
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewProfile),
                            &draft.profile_path,
                        );
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewRevision),
                            draft.bundle_revision.to_string(),
                        );
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewClosureDigest),
                            draft.closure_digest.to_string(),
                        );
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewProfileDigest),
                            draft.profile_digest.to_string(),
                        );
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewOrder),
                    |ui| {
                        for (index, path) in draft.compile_order.iter().enumerate() {
                            ui.monospace(format!("{}. {path}", index + 1));
                        }
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewIncludePaths),
                    |ui| {
                        compile_review_values(ui, &draft.include_paths, &messages);
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewDefinitions),
                    |ui| {
                        if draft.definitions.is_empty() && draft.undefinitions.is_empty() {
                            ui.label(messages.text(MessageId::VerilogACompileReviewNone));
                        }
                        for (name, value) in &draft.definitions {
                            compile_review_row(
                                ui,
                                messages.text(MessageId::VerilogACompileReviewDefine),
                                format!("{name}={value}"),
                            );
                        }
                        for name in &draft.undefinitions {
                            compile_review_row(
                                ui,
                                messages.text(MessageId::VerilogACompileReviewUndefine),
                                name,
                            );
                        }
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewTargets),
                    |ui| {
                        compile_review_bool_row(
                            ui,
                            &messages,
                            MessageId::VerilogACompileReviewPortable,
                            true,
                        );
                        compile_review_bool_row(
                            ui,
                            &messages,
                            MessageId::VerilogACompileReviewGeneratedRust,
                            draft.generated_rust,
                        );
                        compile_review_bool_row(
                            ui,
                            &messages,
                            MessageId::VerilogACompileReviewNativeJit,
                            draft.native_x64_jit,
                        );
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewFallback),
                            messages.text(if draft.reject_fallback {
                                MessageId::VerilogACompileReviewRejectFallback
                            } else {
                                MessageId::VerilogACompileReviewAllowFallback
                            }),
                        );
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewChecks),
                    |ui| {
                        for (check, enabled) in &draft.checks {
                            let label = match check.as_str() {
                                "hidden-state" => MessageId::VerilogACompileReviewHiddenState,
                                "discontinuities" => {
                                    MessageId::VerilogACompileReviewDiscontinuities
                                }
                                "units-and-ranges" => MessageId::VerilogACompileReviewUnitsRanges,
                                "convergence" => MessageId::VerilogACompileReviewConvergence,
                                _ => MessageId::VerilogACompileReviewPortability,
                            };
                            compile_review_bool_row(ui, &messages, label, *enabled);
                        }
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewCellBindings),
                    |ui| {
                        if draft.cell_bindings.is_empty() {
                            ui.label(messages.text(MessageId::VerilogACompileReviewNone));
                        }
                        for (cell, module) in &draft.cell_bindings {
                            compile_review_row(ui, cell, module);
                        }
                    },
                );
                compile_review_section(
                    ui,
                    messages.text(MessageId::VerilogACompileReviewHistory),
                    |ui| {
                        compile_review_row(
                            ui,
                            messages.text(MessageId::VerilogACompileReviewHistoryCount),
                            draft.qualification_attempts,
                        );
                        if draft.recent_qualifications.is_empty() {
                            ui.label(messages.text(MessageId::VerilogACompileReviewNone));
                        }
                        for record in &draft.recent_qualifications {
                            ui.separator();
                            let disposition = match record.disposition {
                                crate::state::ProjectSourceQualificationDisposition::Passed => {
                                    messages.text(MessageId::VerilogACompileReviewPassed)
                                }
                                crate::state::ProjectSourceQualificationDisposition::Failed => {
                                    messages.text(MessageId::VerilogACompileReviewFailed)
                                }
                            };
                            compile_review_row(
                                ui,
                                messages.text(MessageId::VerilogACompileReviewAttempt),
                                format!(
                                    "#{} · {} · {}",
                                    record.sequence, disposition, record.selected_module
                                ),
                            );
                            compile_review_row(
                                ui,
                                messages.text(MessageId::VerilogACompileReviewRecordedAt),
                                record.recorded_at_unix_ms,
                            );
                            compile_review_row(
                                ui,
                                messages.text(MessageId::VerilogACompileReviewReportDigest),
                                record.report_digest,
                            );
                        }
                    },
                );
                if let Some(error) = draft.error.as_deref() {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(error).color(t.color.err));
                }
            });
        None
    });

    match choice {
        DialogChoice::Primary => {
            app.state.ui.code_workspace.veriloga.compile_dialog = Some(draft);
            if let Err(error) =
                crate::workbench::documents::code_workspace::commit_veriloga_compile_dialog(
                    app,
                    ctx.clone(),
                )
                && let Some(dialog) = app.state.ui.code_workspace.veriloga.compile_dialog.as_mut()
            {
                dialog.error = Some(error);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            app.state.ui.code_workspace.veriloga.compile_dialog = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {
            app.state.ui.code_workspace.veriloga.compile_dialog = Some(draft);
        }
    }
}

fn compile_review_section(ui: &mut Ui, title: impl std::fmt::Display, body: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new(title.to_string())
            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
            .color(t.color.text),
    );
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(10)
        .show(ui, body);
}

fn compile_review_row(ui: &mut Ui, label: impl std::fmt::Display, value: impl std::fmt::Display) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label.to_string());
        ui.monospace(value.to_string());
    });
}

fn compile_review_bool_row(
    ui: &mut Ui,
    messages: &MessageCatalog,
    label: MessageId,
    enabled: bool,
) {
    compile_review_row(
        ui,
        messages.text(label),
        messages.text(if enabled {
            MessageId::VerilogACompileReviewEnabled
        } else {
            MessageId::VerilogACompileReviewDisabled
        }),
    );
}

fn compile_review_values(ui: &mut Ui, values: &[String], messages: &MessageCatalog) {
    if values.is_empty() {
        ui.label(messages.text(MessageId::VerilogACompileReviewNone));
    } else {
        for value in values {
            ui.monospace(value);
        }
    }
}

fn handle_veriloga_file_drop(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.application_modal_open() {
        return;
    }
    let dropped = ctx.input(|input| input.raw.dropped_files.clone());
    if dropped.is_empty() {
        return;
    }
    if dropped.len() != 1 {
        let message = app
            .state
            .ui
            .messages()
            .text(MessageId::VerilogADropOneSource);
        app.state
            .push_user_message(ConsoleMessage::warning(message));
        return;
    }
    let file = &dropped[0];
    let name = (!file.name.trim().is_empty())
        .then(|| file.name.clone())
        .or_else(|| {
            file.path
                .as_deref()
                .and_then(std::path::Path::file_name)
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned)
        })
        .unwrap_or_else(|| "dropped.va".to_owned());
    let bytes = if let Some(bytes) = file.bytes.as_ref() {
        Ok(bytes.to_vec())
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            file.path
                .as_deref()
                .ok_or_else(|| "Dropped file has neither bytes nor a native path.".to_owned())
                .and_then(|path| std::fs::read(path).map_err(|error| error.to_string()))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err("Browser drop did not provide immutable file bytes.".to_owned())
        }
    };
    let result = bytes.and_then(|bytes| {
        if bytes.len() > crate::state::MAX_PROJECT_CODE_SOURCE_BYTES {
            return Err(format!(
                "Dropped source exceeds the supported {}-byte limit.",
                crate::state::MAX_PROJECT_CODE_SOURCE_BYTES
            ));
        }
        let contents = String::from_utf8(bytes)
            .map_err(|error| format!("Dropped source is not valid UTF-8: {error}"))?;
        crate::workbench::documents::code_workspace::import_dropped_veriloga_source(
            app, name, contents,
        )
    });
    if let Err(error) = result {
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Verilog-A drop import failed: {error}"
        )));
    }
}

/// The row above the editor: a heading, the cross-link to the generated deck,
/// and Compile.
///
/// With a `module` the row heads a Design document and names that module in
/// two short lines. Without one it heads the Netlist workspace's compiler page
/// with that page's description. The page heading describes a workspace; a
/// Design tab is one module, and three lines of workspace copy above it cost a
/// phone a fifth of its editor.
fn title_row(ui: &mut Ui, app: &mut RSpiceApp, file_name: &str, module: Option<&str>) {
    let messages = app.state.ui.messages();
    let generated_netlist = messages.text(MessageId::CodeGeneratedNetlist);
    let (eyebrow, title, description, heading_reserve) = match module {
        Some(module) => (
            messages.text(MessageId::VerilogAModuleEyebrow),
            module.to_owned(),
            None,
            200.0,
        ),
        None => (
            messages.text(MessageId::VerilogAHeadingEyebrow),
            messages.text(MessageId::VerilogAHeadingTitle),
            Some(messages.text(MessageId::VerilogAHeadingDescription)),
            280.0,
        ),
    };
    let heading = |ui: &mut Ui| match description.as_deref() {
        Some(description) => code_workspace_heading(ui, &eyebrow, &title, description),
        None => module_heading(ui, &eyebrow, &title),
    };
    let full_compile_label = compile_button_label(app, file_name);
    let actions_width = title_button_width(ui, &generated_netlist, false)
        + 6.0
        + title_button_width(ui, &full_compile_label, true);
    let visible_width = visible_workspace_rect(ui).width();
    let stack_actions =
        title_actions_stack(visible_width) || actions_width + heading_reserve > visible_width;
    workspace_title_row(ui, |ui| {
        // Open docks can make the workspace much narrower than the viewport.
        // Stack against the actual row width so actions never overlap.
        if stack_actions {
            heading(ui);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
                generated_netlist_button(ui, app, width);
                compile_button(ui, app, file_name, width, true);
            });
        } else {
            let actions_width = title_button_width(ui, &generated_netlist, false)
                + 6.0
                + title_button_width(ui, &full_compile_label, true);
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
                        heading(ui);
                    },
                );
                ui.add_space(6.0);
                generated_netlist_button(ui, app, 0.0);
                compile_button(ui, app, file_name, 0.0, false);
            });
        }
    });
}

/// The compiler page's eyebrow and title type, without its description: the
/// module name is the whole of what a Design tab's heading has to say.
fn module_heading(ui: &mut Ui, eyebrow: &str, module: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(eyebrow.to_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .extra_letter_spacing(0.09 * tokens::FS_0)
            .color(t.color.text_faint),
    );
    ui.add_space(3.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(module)
                .font(theme::mono(15.0, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .truncate(),
    );
}

fn generated_netlist_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let label = app
        .state
        .ui
        .messages()
        .text(MessageId::CodeGeneratedNetlist);
    let mut button = Button::new(&label);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        let _ =
            crate::workbench::documents::netlist_document::open_generated_primary(&mut app.state);
        app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
        // A cross-link, not a page switch: from Design the deck is in another
        // workspace, so the button goes there rather than flipping a page
        // nobody is looking at.
        if app.state.workbench.workspace != super::super::state::Workspace::Netlist {
            app.state
                .workbench
                .activate(super::super::state::Workspace::Netlist);
        }
    }
}

fn compile_button(ui: &mut Ui, app: &mut RSpiceApp, file_name: &str, width: f32, compact: bool) {
    use crate::workbench::commands::vocabulary::Command;

    let messages = app.state.ui.messages();
    let accessible_label = compile_button_label(app, file_name);
    let label = if compact {
        if app.state.ui.code_workspace.veriloga.pending.is_some() {
            messages.text(MessageId::VerilogACompiling)
        } else {
            messages.text(MessageId::VerilogACompileModel)
        }
    } else {
        accessible_label.clone()
    };
    let compile_enabled = Command::CompileVerilogA.is_enabled(app);
    let unavailable = messages.text(MessageId::VerilogAWorkspaceRequired);
    let button = Button::new(&label)
        .enabled(compile_enabled)
        .accessible_label(&accessible_label);
    let mut button = if compile_enabled {
        button.accent()
    } else {
        button
    };
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    let response = button.show(ui);
    let response = if compile_enabled {
        response
    } else {
        response.on_disabled_hover_text(unavailable)
    };
    if response.clicked() {
        Command::CompileVerilogA.execute(app);
    }
}

fn compile_button_label(app: &RSpiceApp, file_name: &str) -> String {
    let messages = app.state.ui.messages();
    let pending = app.state.ui.code_workspace.veriloga.pending.is_some();
    if pending {
        messages.format(
            MessageId::VerilogACompilingFile,
            &[("file_name", file_name)],
        )
    } else {
        messages.format(MessageId::VerilogACompileFile, &[("file_name", file_name)])
    }
}

fn title_button_width(ui: &Ui, label: &str, accent: bool) -> f32 {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(
        tokens::FS_1,
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
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
    active_path: &str,
    source: &mut String,
    diagnostics: &CodeDiagnosticCollection,
    editor_language: CodeEditorLanguage,
) {
    source_editor(
        ui,
        app,
        selected,
        active_path,
        source,
        diagnostics,
        editor_language,
    );
}

fn source_editor(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
    active_path: &str,
    source: &mut String,
    diagnostics: &CodeDiagnosticCollection,
    editor_language: CodeEditorLanguage,
) {
    let t = Tokens::get(ui.ctx());
    let editor_id = egui::Id::new((
        "workbench.veriloga.source",
        selected.bundle().id(),
        active_path,
    ));
    // A coarse pointer raises the Editor menu to the touch target; the strip
    // grows with it rather than letting the menu hang into the source.
    let (toolbar, _) = ui.allocate_exact_size(
        Vec2::new(
            ui.available_width(),
            EDITOR_TOOLBAR_HEIGHT.max(t.metrics.ctl_h),
        ),
        Sense::hover(),
    );
    ui.painter().rect_filled(toolbar, 0.0, t.color.bg_panel);
    ui.painter().hline(
        toolbar.x_range(),
        toolbar.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let path_clip = egui::Rect::from_min_max(
        egui::pos2(toolbar.left() + 8.0, toolbar.top()),
        egui::pos2(toolbar.right() - 8.0, toolbar.bottom()),
    );
    let messages = app.state.ui.messages();
    let (status, color) = crate::workbench::documents::code_workspace::veriloga_compile_status(
        &app.state.ui.code_workspace.veriloga,
        app.state.workspace.content.project.id(),
        selected,
        messages,
        &t,
    );
    let status_galley =
        ui.painter()
            .layout_no_wrap(status, theme::mono(tokens::FS_0, FontWeight::Medium), color);
    let toolbar_content_right = toolbar.right() - EDITOR_SCROLLBAR_INSET;
    ui.painter().circle_filled(
        egui::pos2(
            toolbar_content_right - status_galley.size().x - 17.0,
            toolbar.center().y,
        ),
        2.5,
        color,
    );
    ui.painter().galley(
        egui::pos2(
            toolbar_content_right - status_galley.size().x - 8.0,
            toolbar.center().y - status_galley.size().y * 0.5,
        ),
        status_galley.clone(),
        color,
    );
    let menu_right = toolbar_content_right - status_galley.size().x - 24.0;
    let menu_rect = egui::Rect::from_min_max(
        egui::pos2((menu_right - 64.0).max(toolbar.left() + 8.0), toolbar.top()),
        egui::pos2(menu_right, toolbar.bottom()),
    );
    // Keep even unusually long logical paths out of the command and status
    // regions. The full path remains available from the file tree.
    let path_right = (menu_rect.left() - 6.0).max(path_clip.left());
    let path_clip =
        egui::Rect::from_min_max(path_clip.min, egui::pos2(path_right, path_clip.bottom()));
    ui.painter().with_clip_rect(path_clip).text(
        egui::pos2(toolbar.left() + 8.0, toolbar.center().y),
        egui::Align2::LEFT_CENTER,
        active_path,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let mut menu_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(menu_rect)
            .layout(Layout::right_to_left(Align::Center)),
    );
    crate::workbench::documents::text_editor_commands::editor_command_menu(
        &mut menu_ui,
        editor_id,
        true,
        true,
    );
    let search_input =
        crate::workbench::documents::text_editor_commands::take_find_in_source_bundle_request(
            ui, editor_id,
        );
    let preceding_input = search_input
        .as_ref()
        .map(|input| input.preceding_scope(ui.ctx()));
    let interaction = egui::Frame::new()
        .fill(t.color.bg_inset)
        .show(ui, |ui| {
            let interaction = show_code_document_interaction_versioned(
                ui,
                (
                    "workbench.veriloga.source",
                    selected.bundle().id(),
                    active_path,
                ),
                source,
                selected.bundle().revision().get(),
                editor_language,
                diagnostics,
                Some(active_path),
                true,
                messages,
            );
            if interaction.changed {
                match crate::workbench::documents::code_workspace::replace_selected_veriloga_file(
                    app,
                    selected,
                    active_path,
                    source.clone(),
                ) {
                    Ok(true) => {}
                    Ok(false) => {}
                    Err(error) => {
                        app.state
                            .push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                                "Could not update {}: {error}",
                                active_path
                            )))
                    }
                }
            }
            interaction
        })
        .inner;
    // Every request below names the bundle this editor is showing, never a
    // language-wide default: Find, Format and the language tools must act on
    // the cell view in front of the reader.
    let bundle_id = selected.bundle().id();
    app.state.ui.code_workspace.source_carets.insert(
        (bundle_id, active_path.to_owned()),
        interaction.cursor_char_index,
    );
    let selection_key = (bundle_id, active_path.to_owned());
    if let Some(range) = interaction.selected_char_range {
        app.state
            .ui
            .code_workspace
            .source_selections
            .insert(selection_key, range);
    } else {
        app.state
            .ui
            .code_workspace
            .source_selections
            .remove(&selection_key);
    }
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui, editor_id,
    ) {
        match crate::workbench::documents::code_workspace::open_language_tools_in_bundle(
            app,
            crate::state::ProjectSourceLanguage::VerilogA,
            bundle_id,
            active_path,
            interaction.cursor_char_index,
        ) {
            Ok(()) => {
                let formatting_message = app
                    .state
                    .ui
                    .messages()
                    .text(MessageId::CodeFormattingParseGated);
                if let Some(tools) = app.state.ui.code_workspace.language_tools.as_mut() {
                    tools.view =
                        crate::workbench::documents::code_workspace::LanguageToolView::CodeActions;
                    tools.status = Some(formatting_message);
                }
            }
            Err(error) => app
                .state
                .push_user_message(crate::diagnostics::ConsoleMessage::error(error)),
        }
    }
    if crate::workbench::documents::text_editor_commands::take_source_language_tools_request(
        ui, editor_id,
    ) && let Err(error) =
        crate::workbench::documents::code_workspace::open_language_tools_in_bundle(
            app,
            crate::state::ProjectSourceLanguage::VerilogA,
            bundle_id,
            active_path,
            interaction.cursor_char_index,
        )
    {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::error(error));
    }
    drop(preceding_input);
    if let Some(input) = search_input {
        // Bind the search to source revisions and selection after preceding edits.
        if let Err(error) =
            crate::workbench::documents::code_workspace::open_source_search_in_bundle(
                app,
                crate::state::ProjectSourceLanguage::VerilogA,
                bundle_id,
                active_path,
            )
        {
            app.state
                .push_user_message(crate::diagnostics::ConsoleMessage::error(error));
        }
        input.route(
            ui.ctx(),
            crate::workbench::documents::code_workspace::source_search_query_id(),
        );
    }
}

fn current_diagnostics(
    app: &RSpiceApp,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) -> std::sync::Arc<CodeDiagnosticCollection> {
    crate::workbench::documents::code_workspace::veriloga_diagnostics(
        &app.state.ui.code_workspace.veriloga,
        app.state.workspace.content.project.id(),
        selected,
    )
}

/// The Netlist page's own inspector column. The Design workspace states the
/// same facts in its inspector dock, drawn by the same body.
fn inspector(
    ui: &mut Ui,
    app: &RSpiceApp,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) {
    crate::workbench::documents::code_workspace::show_veriloga_inspector(
        ui,
        &app.state.ui.code_workspace.veriloga,
        app.state.workspace.content.project.id(),
        selected,
        app.state.ui.messages(),
    );
}

/// The editor has no bundle to show.
///
/// A Verilog-A cell view without its bundle is a defect of that view, and the
/// resolver's error names it exactly; the project-wide source workspace is not
/// a remedy for it, so nothing here offers to create one. Only with no
/// Verilog-A view in front does the Netlist page offer to start that
/// workspace.
fn missing_source(ui: &mut Ui, app: &mut RSpiceApp, error: String) {
    let messages = app.state.ui.messages();
    if app.state.workspace.content.active_view_type() == crate::state::ViewType::VerilogA {
        super::super::design_system::empty_state(
            ui,
            super::super::design_system::WorkbenchIcon::VerilogA,
            &messages.text(MessageId::VerilogANoSource),
            &error,
        );
        return;
    }
    super::super::design_system::empty_state_with_actions(
        ui,
        super::super::design_system::WorkbenchIcon::Code,
        &messages.text(MessageId::VerilogANoSource),
        &messages.text(MessageId::VerilogANoSourceDescription),
        |ui| {
            if Button::new(&messages.text(MessageId::CodeSourceCreateWorkspace))
                .show(ui)
                .clicked()
                && let Err(error) =
                    crate::workbench::documents::code_workspace::open_source_workspace_dialog(
                        app,
                        crate::state::ProjectSourceLanguage::VerilogA,
                    )
            {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
            if Button::new(&messages.text(MessageId::CodeSourceImportRoot))
                .show(ui)
                .clicked()
                && let Err(error) =
                    crate::workbench::documents::code_workspace::request_veriloga_root_import(app)
            {
                app.state.push_user_message(ConsoleMessage::error(error));
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_actions_follow_workspace_width_not_viewport_width() {
        assert!(title_actions_stack(TITLE_ACTION_STACK_BREAKPOINT));
        assert!(!title_actions_stack(TITLE_ACTION_STACK_BREAKPOINT + 1.0));
    }
}
