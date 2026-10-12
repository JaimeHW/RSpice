//! The Verilog-A source editor.
//!
//! A Verilog-A module is a view of a cell, so its editor is a Design-workspace
//! document: [`show_in_design`] draws it, editor only, while the inspector
//! dock states the module's facts. It edits the bundle
//! `code_workspace::active_source_owner` names, which is the bundle every
//! command, dock and dialog resolves too.

use egui::{Align, Layout, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::diagnostics::ConsoleMessage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::{MessageCatalog, MessageId, RSpiceApp};

use super::super::design_system::workspace_title_row;
use crate::workbench::documents::code_workspace::{
    CodeDiagnosticCollection, CodeEditorLanguage, show_code_document_interaction_versioned,
};

const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;
const EDITOR_SCROLLBAR_INSET: f32 = 14.0;

/// A Verilog-A cell view, as the Design workspace's document.
pub fn show_in_design(ui: &mut Ui, app: &mut RSpiceApp) {
    // Import and compile results are consumed by the frame loop, so a result
    // that lands while the editor is off screen is still published. Only what
    // needs this surface's pointer and context stays here.
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
        let module = selected
            .as_ref()
            .ok()
            .and_then(|source| source.selected_module().map(str::to_owned))
            .unwrap_or_else(|| app.state.workspace.content.active_view.cell.clone());
        title_row(ui, app, file_name, &module);
        let selected = match selected {
            Ok(selected) => selected,
            Err(error) => {
                missing_source(ui, app, &error);
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
        // The inspector dock states the module's facts, so the editor takes
        // the whole document well at every width. A second copy of them beside
        // or below it would only push the source off a phone.
        let available = ui.available_rect_before_wrap();
        let mut editor_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(available)
                .layout(Layout::top_down(Align::Min)),
        );
        source_editor(
            &mut editor_ui,
            app,
            &selected,
            &active_path,
            &mut source,
            &diagnostics,
            editor_language,
        );
        ui.allocate_rect(available, Sense::hover());
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

/// The row above the editor: the module the tab holds, in two short lines.
///
/// The toolbar owns Compile and the cross-link to the generated deck. Where it
/// keeps only its first tools, on a phone, the row carries those two itself,
/// under the heading.
fn title_row(ui: &mut Ui, app: &mut RSpiceApp, file_name: &str, module: &str) {
    let eyebrow = app
        .state
        .ui
        .messages()
        .text(MessageId::VerilogAModuleEyebrow);
    let viewport = ui.ctx().content_rect();
    let carries_actions =
        super::super::layout::toolbar_tool_limit(viewport.width(), viewport.height()).is_some();
    workspace_title_row(ui, |ui| {
        module_heading(ui, &eyebrow, module);
        if !carries_actions {
            return;
        }
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
            generated_netlist_button(ui, app, width);
            compile_button(ui, app, file_name, width);
        });
    });
}

/// The kicker and the module name: all a Design tab's heading has to say.
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
    if Button::new(&label)
        .min_width(width)
        .max_width(width)
        .show(ui)
        .clicked()
    {
        crate::workbench::documents::netlist_document::show_generated_deck(&mut app.state);
    }
}

fn compile_button(ui: &mut Ui, app: &mut RSpiceApp, file_name: &str, width: f32) {
    use crate::workbench::commands::vocabulary::Command;

    let messages = app.state.ui.messages();
    let accessible_label = compile_button_label(app, file_name);
    let label = if app.state.ui.code_workspace.veriloga.pending.is_some() {
        messages.text(MessageId::VerilogACompiling)
    } else {
        messages.text(MessageId::VerilogACompileModel)
    };
    let compile_enabled = Command::CompileVerilogA.is_enabled(app);
    let unavailable = messages.text(MessageId::VerilogACompileUnavailable);
    let button = Button::new(&label)
        .enabled(compile_enabled)
        .accessible_label(&accessible_label)
        .min_width(width)
        .max_width(width);
    let button = if compile_enabled {
        button.accent()
    } else {
        button
    };
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
        true,
    );
    // Replacing the file from disk is this menu's explicit, confirmed
    // transaction; File > Import never does it.
    if crate::workbench::documents::text_editor_commands::take_replace_source_request(ui, editor_id)
        && let Err(error) =
            crate::workbench::documents::code_workspace::request_veriloga_source_replacement(
                app,
                active_path,
            )
    {
        app.state.push_user_message(ConsoleMessage::warning(error));
    }
    interface_strip(ui, &mut app.state, selected);
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

/// The line under the toolbar that states a finding about the module's
/// interface: its ports differ from the cell's schematic, they could not be
/// written, or a hand-edited symbol has drifted from them, with Update symbol
/// beside the drift. With nothing to say it takes no room at all.
fn interface_strip(
    ui: &mut Ui,
    state: &mut crate::workbench::app_state::AppState,
    selected: &crate::workbench::documents::code_workspace::SelectedVerilogASource,
) {
    use crate::workbench::documents::code_workspace::{
        VerilogAInterfaceFinding, interface_finding, interface_mismatch_detail, symbol_drift_text,
        veriloga_receipt,
    };

    let crate::state::ProjectSourceOwner::CellView { reference } = selected.bundle().owner() else {
        return;
    };
    let t = Tokens::get(ui.ctx());
    let messages = state.ui.messages();
    let veriloga = &state.ui.code_workspace.veriloga;
    let receipt = veriloga_receipt(veriloga, state.workspace.content.project.id(), selected);
    let Some(finding) = interface_finding(veriloga, receipt, reference) else {
        return;
    };
    let (text, tone, detail, offers_update) = match finding {
        VerilogAInterfaceFinding::SchematicMismatch { schematic, module } => (
            messages.text(MessageId::VerilogAInterfaceMismatchStrip),
            t.color.err,
            Some(interface_mismatch_detail(reference, schematic, module)),
            false,
        ),
        VerilogAInterfaceFinding::Refused(reason) => (
            messages.format(
                MessageId::VerilogAInterfaceRefusedStrip,
                &[("reason", reason)],
            ),
            t.color.warn,
            None,
            false,
        ),
        VerilogAInterfaceFinding::SymbolDrift(drift) => (
            messages.format(
                MessageId::VerilogASymbolStrip,
                &[("drift", &symbol_drift_text(drift, messages))],
            ),
            t.color.warn,
            None,
            true,
        ),
    };
    let (strip, response) = ui.allocate_exact_size(
        Vec2::new(
            ui.available_width(),
            EDITOR_TOOLBAR_HEIGHT.max(t.metrics.ctl_h),
        ),
        Sense::hover(),
    );
    // The strip shares the toolbar's fill, so the rule above it is what keeps
    // a finding from reading as a second row of tools.
    ui.painter().rect_filled(strip, 0.0, t.color.bg_panel);
    for y in [strip.top() + 0.5, strip.bottom()] {
        ui.painter()
            .hline(strip.x_range(), y, Stroke::new(1.0, t.color.border));
    }
    // The toolbar's left inset, and its status text's right edge.
    let content = strip.shrink2(Vec2::new(8.0, 0.0));
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(egui::Rect::from_min_max(
                content.min,
                egui::pos2(content.right() - EDITOR_SCROLLBAR_INSET, content.bottom()),
            ))
            .layout(Layout::right_to_left(Align::Center)),
    );
    let update = offers_update
        && Button::new(&messages.text(MessageId::VerilogAUpdateSymbol))
            .show(&mut actions)
            .clicked();
    let text_right = actions.min_rect().left().min(content.right()) - 8.0;
    let icon = egui::Rect::from_center_size(
        egui::pos2(content.left() + 7.0, content.center().y),
        Vec2::splat(14.0),
    );
    crate::workbench::design_system::WorkbenchIcon::Warning.paint(ui.painter(), icon, tone);
    let text_clip = egui::Rect::from_min_max(
        egui::pos2(icon.right() + 7.0, strip.top()),
        egui::pos2(text_right.max(icon.right() + 7.0), strip.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        egui::pos2(text_clip.left(), strip.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        tone,
    );
    if let Some(detail) = detail {
        response.on_hover_text(detail);
    }
    if update {
        state.update_active_veriloga_symbol();
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

/// The view has no bundle to show.
///
/// A Verilog-A cell view without its bundle is a defect of that view, and the
/// resolver's error names it exactly. Nothing here offers a remedy that is not
/// one: there is no project-wide source to fall back to.
fn missing_source(ui: &mut Ui, app: &RSpiceApp, error: &str) {
    super::super::design_system::empty_state(
        ui,
        super::super::design_system::WorkbenchIcon::VerilogA,
        &app.state.ui.messages().text(MessageId::VerilogANoSource),
        error,
    );
}
