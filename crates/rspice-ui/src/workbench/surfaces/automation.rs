//! Mockup-defined Automation and CI workspace backed by governed execution.

use egui::{Align, Layout, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::automation_workflow::ArtifactKind;
use crate::diagnostics::ConsoleMessage;
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::{MessageCatalog, MessageId, RSpiceApp, ScalarPreference};

use super::super::design_system::{
    WorkbenchIcon, code_inspector_property_list, code_inspector_section, code_workspace_heading,
    property_row, workspace_title_row,
};
use crate::workbench::documents::code_workspace::{
    AutomationExecutionState, CodeDiagnosticCollection, CodeEditorLanguage, CodeWorkspacePage,
    show_code_document_with_debug_versioned,
};

const INTERNAL_INSPECTOR_MIN_WIDTH: f32 = 270.0;
const INTERNAL_INSPECTOR_WIDTH: f32 = 330.0;
const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;
const EDITOR_SCROLLBAR_INSET: f32 = 14.0;
const TITLE_ACTION_STACK_BREAKPOINT: f32 = 720.0;

fn title_actions_stack(available_width: f32) -> bool {
    available_width <= TITLE_ACTION_STACK_BREAKPOINT
}

fn valid_automation_watch_expression(expression: &str) -> bool {
    !expression.trim().is_empty()
        && expression.len() <= rspice_automation_protocol::MAX_WATCH_EXPRESSION_BYTES
        && !expression.as_bytes().contains(&0)
}

fn visible_workspace_rect(ui: &Ui) -> egui::Rect {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .intersect(ui.max_rect())
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    handle_automation_file_drop(ui.ctx(), app);
    crate::workbench::documents::code_workspace::sync_automation_debugger(app);
    let t = Tokens::get(ui.ctx());
    let bundle = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .cloned();
    let runtime_stale = app
        .state
        .ui
        .code_workspace
        .automation
        .receipt
        .as_ref()
        .is_some_and(|receipt| {
            bundle.as_ref().is_none_or(|bundle| {
                receipt.token.project_id != app.state.workspace.project.id()
                    || receipt.token.revision != bundle.revision().get()
                    || receipt.token.content_digest != bundle.closure_digest()
            })
        });
    if runtime_stale {
        app.state.ui.code_workspace.automation = Default::default();
    }
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        title_row(ui, app);
        let Some(bundle) = bundle else {
            let messages = app.state.ui.messages();
            super::super::design_system::empty_state_with_actions(
                ui,
                WorkbenchIcon::Terminal,
                &messages.text(MessageId::AutomationNoSource),
                &messages.text(MessageId::AutomationNoSourceDescription),
                |ui| {
                    if Button::new(&messages.text(MessageId::CodeSourceCreateWorkspace))
                        .show(ui)
                        .clicked()
                        && let Err(error) = crate::workbench::documents::code_workspace::open_source_workspace_dialog(
                            app,
                            ProjectSourceLanguage::RSpiceAutomation,
                        )
                    {
                        app.state.push_user_message(ConsoleMessage::error(error));
                    }
                },
            );
            return;
        };
        let selected_path = match app.state.ui.code_workspace.automation.selected_file.clone() {
            Some(path) if bundle.contains_file(&path) => path,
            Some(_) => {
                app.state.ui.code_workspace.automation.selected_file = None;
                bundle.root().logical_path().to_owned()
            }
            None => bundle.root().logical_path().to_owned(),
        };
        let Some(mut source) = bundle.file_content(&selected_path).map(str::to_owned) else {
            let messages = app.state.ui.messages();
            super::super::design_system::empty_state(
                ui,
                WorkbenchIcon::File,
                &messages.text(MessageId::AutomationFileMissing),
                &messages.text(MessageId::AutomationFileMissingDescription),
            );
            return;
        };
        let runtime = &app.state.ui.code_workspace.automation;
        let diagnostics = runtime
            .diagnostic_token
            .filter(|token| {
                token.project_id == app.state.workspace.project.id()
                    && token.revision == bundle.revision().get()
                    && token.content_digest == bundle.closure_digest()
            })
            .map_or_else(
                || std::sync::Arc::new(CodeDiagnosticCollection::default()),
                |_| std::sync::Arc::clone(&runtime.diagnostics),
            );
        // Base the responsive split on the workspace pane, not the outer
        // viewport. Navigator and inspector docks can leave the editor with
        // substantially less room than the browser window itself.
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
                                        &bundle,
                                        &selected_path,
                                        &mut source,
                                        &diagnostics,
                                    )
                                });
                            ui.painter().hline(
                                code_pane.response.rect.x_range(),
                                code_pane.response.rect.bottom(),
                                Stroke::new(1.0, t.color.border),
                            );
                            egui::Frame::new().fill(t.color.bg_panel).show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                inspector(ui, app);
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
                &bundle,
                &selected_path,
                &mut source,
                &diagnostics,
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
                .id_salt("workbench.automation.inspector")
                .auto_shrink([false, false])
                .show(&mut inspector_ui, |ui| inspector(ui, app));
            ui.allocate_rect(available, Sense::hover());
        }
    });
}

fn handle_automation_file_drop(ctx: &egui::Context, app: &mut RSpiceApp) {
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
            .text(MessageId::AutomationDropOneSource);
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
        .unwrap_or_else(|| "dropped-source.py".to_owned());
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
        crate::workbench::documents::code_workspace::import_dropped_automation_source(
            app, name, contents,
        )
        .map(|_| ())
    });
    if let Err(error) = result {
        app.state.push_user_message(ConsoleMessage::error(format!(
            "Automation drop import failed: {error}"
        )));
    }
}

fn title_row(ui: &mut Ui, app: &mut RSpiceApp) {
    let messages = app.state.ui.messages();
    let eyebrow = messages.text(MessageId::AutomationHeadingEyebrow);
    let title = messages.text(MessageId::AutomationHeadingTitle);
    let description = messages.text(MessageId::AutomationHeadingDescription);
    let generated_netlist = messages.text(MessageId::CodeGeneratedNetlist);
    let validate = messages.text(MessageId::AutomationValidate);
    let dry_run = messages.text(MessageId::AutomationDryRun);
    let stack_actions = title_actions_stack(visible_workspace_rect(ui).width());
    workspace_title_row(ui, |ui| {
        if stack_actions {
            code_workspace_heading(ui, &eyebrow, &title, &description);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 18.0) * 0.25).max(1.0);
                generated_netlist_button(ui, app, width);
                validate_button(ui, app, width);
                dry_run_button(ui, app, width);
                run_button(ui, app, width);
            });
        } else {
            let run_label = run_button_label(app);
            let actions_width = title_button_width(ui, &generated_netlist, false)
                + 6.0
                + title_button_width(ui, &validate, false)
                + 6.0
                + title_button_width(ui, &dry_run, false)
                + 6.0
                + title_button_width(ui, &run_label, true);
            let heading_width = (ui.available_width() - actions_width - 12.0).max(1.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.allocate_ui_with_layout(
                    Vec2::new(heading_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(heading_width);
                        code_workspace_heading(ui, &eyebrow, &title, &description);
                    },
                );
                ui.add_space(6.0);
                generated_netlist_button(ui, app, 0.0);
                validate_button(ui, app, 0.0);
                dry_run_button(ui, app, 0.0);
                run_button(ui, app, 0.0);
            });
        }
    });
}

fn validate_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let busy = automation_busy(app);
    let workspace_available = automation_workspace_available(app);
    let messages = app.state.ui.messages();
    let label = messages.text(MessageId::AutomationValidate);
    let unavailable = messages.text(MessageId::AutomationWorkspaceRequired);
    let mut button = Button::new(&label).enabled(!busy && workspace_available);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    let response = button.show(ui);
    let response = if workspace_available {
        response
    } else {
        response.on_disabled_hover_text(unavailable)
    };
    if response.clicked() {
        crate::workbench::documents::code_workspace::validate_automation_workspace(app);
    }
}

fn dry_run_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let busy = automation_busy(app);
    let workspace_available = automation_workspace_available(app);
    let messages = app.state.ui.messages();
    let label = messages.text(MessageId::AutomationDryRun);
    let unavailable = messages.text(MessageId::AutomationWorkspaceRequired);
    let mut button = Button::new(&label).enabled(!busy && workspace_available);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    let response = button.show(ui);
    let response = if workspace_available {
        response
    } else {
        response.on_disabled_hover_text(unavailable)
    };
    if response.clicked() {
        crate::workbench::documents::code_workspace::dry_run_automation_workflow(app);
    }
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
    }
}

fn run_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let active = app.state.ui.code_workspace.automation.execution.is_active();
    let validating = app
        .state
        .ui
        .code_workspace
        .automation
        .pending_runtime_validation
        .is_some();
    let label = run_button_label(app);
    let workspace_available = automation_workspace_available(app);
    let unavailable = app
        .state
        .ui
        .messages()
        .text(MessageId::AutomationWorkspaceRequired);
    let mut button = Button::new(&label)
        .accent()
        .enabled(!validating && (active || workspace_available));
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    let response = button.show(ui);
    let response = if active || workspace_available {
        response
    } else {
        response.on_disabled_hover_text(unavailable)
    };
    if response.clicked() {
        if active {
            crate::workbench::documents::code_workspace::cancel_automation_workflow(app);
        } else {
            crate::workbench::documents::code_workspace::start_automation_workflow(app);
        }
    }
}

fn run_button_label(app: &RSpiceApp) -> String {
    let messages = app.state.ui.messages();
    if app
        .state
        .ui
        .code_workspace
        .automation
        .pending_runtime_validation
        .is_some()
    {
        messages.text(MessageId::AutomationValidating)
    } else if app.state.ui.code_workspace.automation.execution.is_active() {
        if app.state.ui.code_workspace.automation.cancel_requested {
            messages.text(MessageId::AutomationStopping)
        } else {
            messages.text(MessageId::AutomationStopWorkflow)
        }
    } else {
        messages.text(MessageId::AutomationRunWorkflow)
    }
}

fn automation_busy(app: &RSpiceApp) -> bool {
    let runtime = &app.state.ui.code_workspace.automation;
    runtime.execution.is_active()
        || runtime.pending_runtime_validation.is_some()
        || runtime.runtime_execution.is_some()
}

fn automation_workspace_available(app: &RSpiceApp) -> bool {
    app.state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .is_some()
}

fn automation_execution_label(
    execution: &AutomationExecutionState,
    messages: MessageCatalog,
) -> String {
    messages.text(match execution {
        AutomationExecutionState::Idle => MessageId::AutomationExecutionValidationRequired,
        AutomationExecutionState::PythonBroker { dry_run: true, .. } => {
            MessageId::AutomationExecutionPythonDryRun
        }
        AutomationExecutionState::PythonBroker { dry_run: false, .. } => {
            MessageId::AutomationExecutionPythonWorkflowRunning
        }
        AutomationExecutionState::PythonComplete { dry_run: true } => {
            MessageId::AutomationExecutionPythonDryRunPassed
        }
        AutomationExecutionState::PythonComplete { dry_run: false } => {
            MessageId::AutomationExecutionPythonWorkflowCompleted
        }
        AutomationExecutionState::AwaitingDispatch { .. } => {
            MessageId::AutomationExecutionPreflightAccepted
        }
        AutomationExecutionState::Running { .. } => MessageId::AutomationExecutionWorkflowRunning,
        AutomationExecutionState::Complete { passed: true, .. } => {
            MessageId::AutomationExecutionReleasePass
        }
        AutomationExecutionState::Complete { passed: false, .. } => {
            MessageId::AutomationExecutionReleaseFailed
        }
        AutomationExecutionState::Cancelled => MessageId::AutomationExecutionCancelled,
        AutomationExecutionState::Failed => MessageId::AutomationExecutionFailed,
    })
}

fn automation_debug_phase_label(
    phase: crate::workbench::documents::code_workspace::AutomationDebugPhase,
    messages: MessageCatalog,
) -> String {
    use crate::workbench::documents::code_workspace::AutomationDebugPhase;
    messages.text(match phase {
        AutomationDebugPhase::Inactive => MessageId::AutomationDebugInactive,
        AutomationDebugPhase::Paused => MessageId::AutomationDebugPaused,
        AutomationDebugPhase::Running => MessageId::AutomationDebugRunning,
        AutomationDebugPhase::Stopped => MessageId::AutomationDebugStopped,
        AutomationDebugPhase::Failed => MessageId::AutomationDebugFailed,
    })
}

fn automation_exception_policy_label(
    policy: crate::workbench::documents::code_workspace::AutomationExceptionPolicy,
    messages: MessageCatalog,
) -> String {
    use crate::workbench::documents::code_workspace::AutomationExceptionPolicy;
    messages.text(match policy {
        AutomationExceptionPolicy::All => MessageId::AutomationDebugAllExceptions,
        AutomationExceptionPolicy::Uncaught => MessageId::AutomationDebugUncaughtExceptions,
        AutomationExceptionPolicy::Never => MessageId::AutomationDebugNever,
    })
}

fn automation_breakpoint_kind_label(
    kind: crate::workbench::documents::code_workspace::AutomationBreakpointKind,
    messages: MessageCatalog,
) -> String {
    use crate::workbench::documents::code_workspace::AutomationBreakpointKind;
    messages.text(match kind {
        AutomationBreakpointKind::Stop => MessageId::AutomationBreakpointStop,
        AutomationBreakpointKind::Conditional => MessageId::AutomationBreakpointCondition,
        AutomationBreakpointKind::Logpoint => MessageId::AutomationBreakpointLogpoint,
        AutomationBreakpointKind::HitCount => MessageId::AutomationBreakpointHitCount,
    })
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

fn automation_editor_language(path: &str) -> CodeEditorLanguage {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".py") || lower.ends_with(".pyi") {
        CodeEditorLanguage::Python
    } else if lower.ends_with(".yaml") || lower.ends_with(".yml") {
        CodeEditorLanguage::Yaml
    } else if lower.ends_with(".toml") || lower.ends_with(".lock") {
        CodeEditorLanguage::Toml
    } else {
        CodeEditorLanguage::Automation
    }
}

fn code_pane(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    bundle: &crate::state::ProjectSourceBundle,
    selected_path: &str,
    source: &mut String,
    diagnostics: &CodeDiagnosticCollection,
) {
    let t = Tokens::get(ui.ctx());
    let legacy = bundle.root().logical_path().ends_with(".rspice");
    let language = automation_editor_language(selected_path);
    let editable = !automation_busy(app)
        && app
            .state
            .ui
            .code_workspace
            .automation
            .manifest
            .as_ref()
            .is_none_or(|manifest| {
                !manifest
                    .environment_lock_path
                    .eq_ignore_ascii_case(selected_path)
            })
        && crate::workbench::documents::code_workspace::source_document_is_editable(
            app,
            ProjectSourceLanguage::RSpiceAutomation,
            selected_path,
        );
    let editor_id = egui::Id::new(("workbench.automation.source", selected_path));
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
    let path_clip = egui::Rect::from_min_max(
        egui::pos2(toolbar.left() + 8.0, toolbar.top()),
        egui::pos2(toolbar.right() - 8.0, toolbar.bottom()),
    );
    let runtime = &app.state.ui.code_workspace.automation;
    let validated = runtime.receipt.as_ref().is_some_and(|receipt| {
        receipt.token.project_id == app.state.workspace.project.id()
            && receipt.token.revision == bundle.revision().get()
            && receipt.token.content_digest == bundle.closure_digest()
    });
    let messages = app.state.ui.messages();
    let (status, color) = if runtime.pending_runtime_validation.is_some() {
        (
            messages.text(MessageId::AutomationStatusValidating),
            t.color.info,
        )
    } else if runtime.execution.is_active() {
        (
            automation_execution_label(&runtime.execution, messages),
            t.color.info,
        )
    } else if matches!(
        runtime.execution,
        AutomationExecutionState::Complete { passed: true, .. }
    ) {
        (
            automation_execution_label(&runtime.execution, messages),
            t.color.ok,
        )
    } else if matches!(
        runtime.execution,
        AutomationExecutionState::Complete { passed: false, .. } | AutomationExecutionState::Failed
    ) {
        (
            automation_execution_label(&runtime.execution, messages),
            t.color.err,
        )
    } else if matches!(runtime.execution, AutomationExecutionState::Cancelled) {
        (
            automation_execution_label(&runtime.execution, messages),
            t.color.warn,
        )
    } else if validated {
        (
            messages.text(MessageId::AutomationStatusValidated),
            t.color.ok,
        )
    } else {
        (
            messages.text(MessageId::AutomationStatusModified),
            t.color.warn,
        )
    };
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
        selected_path,
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
        editable,
        true,
    );
    if crate::workbench::documents::text_editor_commands::take_find_in_source_bundle_request(
        ui, editor_id,
    ) && let Err(error) = crate::workbench::documents::code_workspace::open_source_search(
        app,
        ProjectSourceLanguage::RSpiceAutomation,
        selected_path,
    ) {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::error(error));
    }
    let interaction = egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        let debuggable = language == CodeEditorLanguage::Python;
        let interaction = if debuggable && !legacy {
            let selected_document_id = bundle.document_id(selected_path).map(|id| id.as_uuid());
            let breakpoints = app
                .state
                .ui
                .code_workspace
                .automation
                .debug
                .breakpoints
                .iter()
                .filter(|breakpoint| {
                    breakpoint.enabled && Some(breakpoint.document_id) == selected_document_id
                })
                .map(|breakpoint| breakpoint.line)
                .collect::<Vec<_>>();
            let current_line = app
                .state
                .ui
                .code_workspace
                .automation
                .debug
                .selected_frame_id
                .and_then(|frame_id| {
                    app.state
                        .ui
                        .code_workspace
                        .automation
                        .debug
                        .call_stack
                        .iter()
                        .find(|frame| {
                            frame.frame_id == frame_id
                                && Some(frame.document_id) == selected_document_id
                        })
                        .map(|frame| frame.line)
                });
            show_code_document_with_debug_versioned(
                ui,
                ("workbench.automation.source", selected_path),
                source,
                bundle.revision().get(),
                language,
                diagnostics,
                Some(selected_path),
                editable,
                messages,
                &breakpoints,
                current_line,
            )
        } else {
            crate::workbench::documents::code_workspace::show_code_document_interaction_versioned(
                ui,
                ("workbench.automation.source", selected_path),
                source,
                bundle.revision().get(),
                language,
                diagnostics,
                Some(selected_path),
                editable,
                messages,
            )
        };
        if let Some(line) = interaction.breakpoint_toggled {
            crate::workbench::documents::code_workspace::toggle_automation_breakpoint(app, line);
        }
        if interaction.changed {
            match app.state.workspace.replace_project_source_bundle_file(
                bundle.id(),
                selected_path,
                source.clone(),
            ) {
                Ok(true) => {
                    crate::workbench::documents::code_workspace::invalidate_automation_evidence(
                        app,
                    );
                }
                Ok(false) => {}
                Err(error) => {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                            "Could not update {}: {error}",
                            selected_path
                        )))
                }
            }
        }
        interaction
    }).inner;
    app.state.ui.code_workspace.source_carets.insert(
        (
            ProjectSourceLanguage::RSpiceAutomation,
            selected_path.to_owned(),
        ),
        interaction.cursor_char_index,
    );
    let selection_key = (
        ProjectSourceLanguage::RSpiceAutomation,
        selected_path.to_owned(),
    );
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
        match crate::workbench::documents::code_workspace::open_language_tools(
            app,
            ProjectSourceLanguage::RSpiceAutomation,
            selected_path,
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
    ) && let Err(error) = crate::workbench::documents::code_workspace::open_language_tools(
        app,
        ProjectSourceLanguage::RSpiceAutomation,
        selected_path,
        interaction.cursor_char_index,
    ) {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::error(error));
    }
}

fn inspector(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let messages = app.state.ui.messages();
    ui.set_width(ui.available_width());
    let manifest = app.state.ui.code_workspace.automation.manifest.clone();
    let preview = execution_preview(app);
    let runtime_status = app
        .automation_runtime_unavailable_reason()
        .unwrap_or_else(|| messages.text(MessageId::AutomationManagedRuntimeVerified));
    let task_count = preview.task_points.to_string();
    let plan_kind = messages.text(if preview.release_ready {
        MessageId::AutomationReleasePlan
    } else {
        MessageId::AutomationNonSignOff
    });
    let preview_status = messages.format(
        MessageId::AutomationTaskSummary,
        &[("count", &task_count), ("plan_kind", &plan_kind)],
    );
    let not_validated = messages.text(MessageId::AutomationNotValidated);

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationRuntimeApi),
        None,
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationPython),
                    manifest
                        .as_ref()
                        .map_or(not_validated.as_str(), |manifest| {
                            manifest.python_version.as_str()
                        }),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationRspiceApi),
                    manifest
                        .as_ref()
                        .map_or(not_validated.as_str(), |manifest| {
                            manifest.api_version.as_str()
                        }),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationEnvironment),
                    manifest
                        .as_ref()
                        .map_or(not_validated.as_str(), |manifest| {
                            manifest.environment_digest.as_str()
                        }),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationPlatform),
                    &runtime_status,
                );
            });
        },
    );

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationPermissions),
        None,
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationProjectFiles),
                    manifest
                        .as_ref()
                        .map_or(not_validated.as_str(), |manifest| {
                            manifest.project_files.as_str()
                        }),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationArtifactDirectory),
                    manifest
                        .as_ref()
                        .map_or(not_validated.as_str(), |manifest| {
                            manifest.artifact_directory.as_str()
                        }),
                );
                let deny = messages.text(MessageId::AutomationDeny);
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationNetwork),
                    manifest
                        .as_ref()
                        .map_or(deny.as_str(), |manifest| manifest.network.as_str()),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationProcessSpawn),
                    manifest
                        .as_ref()
                        .map_or(deny.as_str(), |manifest| manifest.process_spawn.as_str()),
                );
                let redact = messages.text(MessageId::AutomationRedact);
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationSecretLogging),
                    manifest
                        .as_ref()
                        .map_or(redact.as_str(), |manifest| manifest.secret_logging.as_str()),
                );
            });
        },
    );

    let diagnostic_count = app.state.ui.code_workspace.automation.diagnostics.len();
    let diagnostic_status = messages.text(if diagnostic_count == 0 {
        MessageId::AutomationClear
    } else {
        MessageId::AutomationActionRequired
    });
    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationDiagnostics),
        Some((
            &diagnostic_status,
            if diagnostic_count == 0 {
                t.color.ok
            } else {
                t.color.err
            },
        )),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationCurrentFindings),
                    &diagnostic_count.to_string(),
                );
                let validation_status = messages.text(
                    if app.state.ui.code_workspace.automation.receipt.is_some() {
                        MessageId::AutomationExactRevisionRetained
                    } else {
                        MessageId::AutomationRequired
                    },
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationValidation),
                    &validation_status,
                );
            });
        },
    );
    let preview_tone = if preview.release_ready {
        t.color.accent
    } else {
        t.color.warn
    };
    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationExecutionPreview),
        Some((&preview_status, preview_tone)),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationProjectRevision),
                    &app.state.workspace.project.revision().get().to_string(),
                );
                let slots = preview.slots.to_string();
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationRunTarget),
                    &messages.format(MessageId::AutomationLocalSlots, &[("slots", &slots)]),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationPvtPoints),
                    &preview.pvt_points.to_string(),
                );
                let release_checks = preview.release_checks.to_string();
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationReleaseChecks),
                    &messages.format(
                        MessageId::AutomationConfiguredCount,
                        &[("count", &release_checks)],
                    ),
                );
            });
        },
    );

    debugger(ui, app);

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationArtifacts),
        None,
        |ui| {
            ui.add_space(4.0);
            artifact_row(
                ui,
                app,
                ArtifactKind::JunitXml,
                &messages.text(MessageId::AutomationArtifactCi),
            );
            artifact_row(
                ui,
                app,
                ArtifactKind::SummaryJson,
                &messages.text(MessageId::AutomationArtifactMachine),
            );
            artifact_row(
                ui,
                app,
                ArtifactKind::VerificationPdf,
                &messages.text(MessageId::AutomationArtifactReport),
            );
            ui.add_space(7.0);
        },
    );

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationSessionSafety),
        None,
        |ui| {
            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(10, 9))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(
                                messages.text(MessageId::AutomationManagedRuntimeSafety),
                            )
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                        )
                        .wrap(),
                    );
                });
        },
    );
}

fn debugger(ui: &mut Ui, app: &mut RSpiceApp) {
    use crate::workbench::documents::code_workspace::{
        AutomationBreakpointKind, AutomationDebugPhase, AutomationExceptionPolicy, AutomationWatch,
    };

    let t = Tokens::get(ui.ctx());
    let messages = app.state.ui.messages();
    let redacted = messages.text(MessageId::AutomationRedacted);
    let phase = app.state.ui.code_workspace.automation.debug.phase;
    let tone = match phase {
        AutomationDebugPhase::Paused => t.color.warn,
        AutomationDebugPhase::Running => t.color.info,
        AutomationDebugPhase::Failed => t.color.err,
        AutomationDebugPhase::Inactive | AutomationDebugPhase::Stopped => t.color.text_dim,
    };
    let phase_label = automation_debug_phase_label(phase, messages);
    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationDebug),
        Some((&phase_label, tone)),
        |ui| {
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::splat(4.0);
                let can_start = matches!(
                    phase,
                    AutomationDebugPhase::Inactive
                        | AutomationDebugPhase::Stopped
                        | AutomationDebugPhase::Failed
                );
                let start = messages.text(MessageId::AutomationDebugStart);
                if Button::new(&start).enabled(can_start).show(ui).clicked() {
                    crate::workbench::documents::code_workspace::start_automation_debugger(app);
                }
                let continue_label = messages.text(MessageId::AutomationDebugContinue);
                if Button::new(&continue_label)
                    .enabled(phase == AutomationDebugPhase::Paused)
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::continue_automation_debugger(app);
                }
                let pause = messages.text(
                    if app.state.ui.code_workspace.automation.debug.pause_requested {
                        MessageId::AutomationDebugPausePending
                    } else {
                        MessageId::AutomationDebugPause
                    },
                );
                if Button::new(&pause)
                    .enabled(phase == AutomationDebugPhase::Running)
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::pause_automation_debugger(app);
                }
                let step_over = messages.text(MessageId::AutomationDebugStepOver);
                if Button::new(&step_over)
                    .enabled(phase == AutomationDebugPhase::Paused)
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::step_over_automation_debugger(app);
                }
                let step_into = messages.text(MessageId::AutomationDebugStepInto);
                if Button::new(&step_into)
                    .enabled(phase == AutomationDebugPhase::Paused)
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::step_into_automation_debugger(app);
                }
                let step_out = messages.text(MessageId::AutomationDebugStepOut);
                if Button::new(&step_out)
                    .enabled(
                        phase == AutomationDebugPhase::Paused
                            && app
                                .state
                                .ui
                                .code_workspace
                                .automation
                                .debug
                                .call_stack
                                .len()
                                > 1,
                    )
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::step_out_automation_debugger(app);
                }
                let restart = messages.text(MessageId::AutomationDebugRestart);
                if Button::new(&restart)
                    .enabled(phase != AutomationDebugPhase::Inactive)
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::restart_automation_debugger(app);
                }
                let stop = messages.text(MessageId::AutomationDebugStop);
                if Button::new(&stop)
                    .enabled(matches!(
                        phase,
                        AutomationDebugPhase::Paused | AutomationDebugPhase::Running
                    ))
                    .show(ui)
                    .clicked()
                {
                    crate::workbench::documents::code_workspace::stop_automation_debugger(app);
                }
            });
            ui.add_space(7.0);
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationDebugCurrentLine),
                    &app.state
                        .ui
                        .code_workspace
                        .automation
                        .debug
                        .current_line
                        .map_or_else(|| "—".to_owned(), |line| line.to_string()),
                );
                property_row(
                    ui,
                    &messages.text(MessageId::AutomationDebugExceptionPolicy),
                    &automation_exception_policy_label(
                        app.state
                            .ui
                            .code_workspace
                            .automation
                            .debug
                            .exception_policy,
                        messages,
                    ),
                );
            });

            let mut policy = app
                .state
                .ui
                .code_workspace
                .automation
                .debug
                .exception_policy;
            egui::ComboBox::from_id_salt("automation.debug.exceptions")
                .selected_text(automation_exception_policy_label(policy, messages))
                .width(ui.available_width().max(1.0))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut policy,
                        AutomationExceptionPolicy::All,
                        messages.text(MessageId::AutomationDebugAllExceptions),
                    );
                    ui.selectable_value(
                        &mut policy,
                        AutomationExceptionPolicy::Uncaught,
                        messages.text(MessageId::AutomationDebugUncaughtExceptions),
                    );
                    ui.selectable_value(
                        &mut policy,
                        AutomationExceptionPolicy::Never,
                        messages.text(MessageId::AutomationDebugNever),
                    );
                });
            app.state
                .ui
                .code_workspace
                .automation
                .debug
                .exception_policy = policy;

            ui.add_space(7.0);
            ui.label(
                egui::RichText::new(messages.text(MessageId::AutomationBreakpoints))
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
            let breakpoints = app
                .state
                .ui
                .code_workspace
                .automation
                .debug
                .breakpoints
                .clone();
            let mut remove_breakpoint = None;
            let mut breakpoints_changed = false;
            for breakpoint in breakpoints {
                ui.horizontal(|ui| {
                    let mut enabled = breakpoint.enabled;
                    if ui.checkbox(&mut enabled, "").changed()
                        && let Some(current) = app
                            .state
                            .ui
                            .code_workspace
                            .automation
                            .debug
                            .breakpoints
                            .iter_mut()
                            .find(|current| {
                                current.document_id == breakpoint.document_id
                                    && current.line == breakpoint.line
                            })
                    {
                        current.enabled = enabled;
                        breakpoints_changed = true;
                    }
                    ui.label(
                        egui::RichText::new(format!(
                            "{}:{}",
                            breakpoint.source_path, breakpoint.line
                        ))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .small_button(messages.text(MessageId::AutomationRemove))
                            .clicked()
                        {
                            remove_breakpoint = Some((breakpoint.document_id, breakpoint.line));
                        }
                        let mut kind = breakpoint.kind;
                        egui::ComboBox::from_id_salt((
                            "automation.breakpoint.kind",
                            breakpoint.document_id,
                            breakpoint.line,
                        ))
                        .selected_text(automation_breakpoint_kind_label(kind, messages))
                        .width(92.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut kind,
                                AutomationBreakpointKind::Stop,
                                automation_breakpoint_kind_label(
                                    AutomationBreakpointKind::Stop,
                                    messages,
                                ),
                            );
                            ui.selectable_value(
                                &mut kind,
                                AutomationBreakpointKind::Conditional,
                                automation_breakpoint_kind_label(
                                    AutomationBreakpointKind::Conditional,
                                    messages,
                                ),
                            );
                            ui.selectable_value(
                                &mut kind,
                                AutomationBreakpointKind::Logpoint,
                                automation_breakpoint_kind_label(
                                    AutomationBreakpointKind::Logpoint,
                                    messages,
                                ),
                            );
                            ui.selectable_value(
                                &mut kind,
                                AutomationBreakpointKind::HitCount,
                                automation_breakpoint_kind_label(
                                    AutomationBreakpointKind::HitCount,
                                    messages,
                                ),
                            );
                        });
                        if kind != breakpoint.kind
                            && let Some(current) = app
                                .state
                                .ui
                                .code_workspace
                                .automation
                                .debug
                                .breakpoints
                                .iter_mut()
                                .find(|current| {
                                    current.document_id == breakpoint.document_id
                                        && current.line == breakpoint.line
                                })
                        {
                            current.kind = kind;
                            if kind == AutomationBreakpointKind::HitCount && current.hit_count == 0
                            {
                                current.hit_count = 1;
                            }
                            breakpoints_changed = true;
                        }
                    });
                });
                let active = app
                    .state
                    .ui
                    .code_workspace
                    .automation
                    .debug
                    .breakpoints
                    .iter()
                    .find(|current| {
                        current.document_id == breakpoint.document_id
                            && current.line == breakpoint.line
                    })
                    .cloned()
                    .unwrap_or(breakpoint);
                match active.kind {
                    AutomationBreakpointKind::Stop => {}
                    AutomationBreakpointKind::Conditional | AutomationBreakpointKind::Logpoint => {
                        let mut expression = active.expression.clone();
                        let hint = if active.kind == AutomationBreakpointKind::Conditional {
                            messages.text(MessageId::AutomationPythonCondition)
                        } else {
                            messages.text(MessageId::AutomationLogMessageHint)
                        };
                        if ui
                            .add(
                                egui::TextEdit::singleline(&mut expression)
                                    .hint_text(&hint)
                                    .desired_width(ui.available_width()),
                            )
                            .changed()
                            && let Some(current) = app
                                .state
                                .ui
                                .code_workspace
                                .automation
                                .debug
                                .breakpoints
                                .iter_mut()
                                .find(|current| {
                                    current.document_id == active.document_id
                                        && current.line == active.line
                                })
                        {
                            current.expression = expression;
                            breakpoints_changed = true;
                        }
                    }
                    AutomationBreakpointKind::HitCount => {
                        ui.horizontal(|ui| {
                            ui.label(messages.text(MessageId::AutomationPauseAfter));
                            let mut hit_count = active.hit_count.max(1);
                            if ui
                                .add(egui::DragValue::new(&mut hit_count).range(1..=u64::MAX))
                                .changed()
                                && let Some(current) = app
                                    .state
                                    .ui
                                    .code_workspace
                                    .automation
                                    .debug
                                    .breakpoints
                                    .iter_mut()
                                    .find(|current| {
                                        current.document_id == active.document_id
                                            && current.line == active.line
                                    })
                            {
                                current.hit_count = hit_count;
                                breakpoints_changed = true;
                            }
                            let observed = active.observed_hits.to_string();
                            ui.label(
                                messages.format(
                                    MessageId::AutomationHitSummary,
                                    &[("count", &observed)],
                                ),
                            );
                        });
                    }
                }
            }
            if let Some((document_id, line)) = remove_breakpoint {
                crate::workbench::documents::code_workspace::remove_automation_breakpoint(
                    app,
                    document_id,
                    line,
                );
            } else if breakpoints_changed {
                crate::workbench::documents::code_workspace::sync_automation_debugger(app);
            }

            let frames = app
                .state
                .ui
                .code_workspace
                .automation
                .debug
                .call_stack
                .clone();
            if !frames.is_empty() {
                ui.add_space(7.0);
                ui.label(
                    egui::RichText::new(messages.text(MessageId::AutomationCallStack))
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_faint),
                );
                let selected = app
                    .state
                    .ui
                    .code_workspace
                    .automation
                    .debug
                    .selected_frame_id;
                for frame in frames {
                    if ui
                        .selectable_label(
                            selected == Some(frame.frame_id),
                            format!("{}  {}:{}", frame.name, frame.source_path, frame.line),
                        )
                        .clicked()
                    {
                        crate::workbench::documents::code_workspace::select_automation_debug_frame(
                            app,
                            frame.frame_id,
                        );
                    }
                }
            }

            let values = app.state.ui.code_workspace.automation.debug.locals.clone();
            if !values.is_empty() {
                ui.add_space(7.0);
                ui.label(
                    egui::RichText::new(messages.text(MessageId::AutomationLocals))
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_faint),
                );
                for value in values {
                    property_row(
                        ui,
                        &value.name,
                        if value.redacted {
                            &redacted
                        } else {
                            &value.display_value
                        },
                    );
                }
            }

            let globals = app.state.ui.code_workspace.automation.debug.globals.clone();
            if !globals.is_empty() {
                ui.add_space(7.0);
                ui.label(
                    egui::RichText::new(messages.text(MessageId::AutomationGlobals))
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_faint),
                );
                for value in globals {
                    property_row(
                        ui,
                        &value.name,
                        if value.redacted {
                            &redacted
                        } else {
                            &value.display_value
                        },
                    );
                }
            }

            ui.add_space(7.0);
            ui.label(
                egui::RichText::new(messages.text(MessageId::AutomationWatches))
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_faint),
            );
            ui.horizontal(|ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(
                        &mut app.state.ui.code_workspace.automation.debug.watch_input,
                    )
                    .hint_text(messages.text(MessageId::AutomationExpression))
                    .desired_width((ui.available_width() - 48.0).max(60.0)),
                );
                let add = messages.text(MessageId::AutomationAdd);
                let submit = Button::new(&add).show(ui).clicked()
                    || (response.lost_focus()
                        && ui.input(|input| input.key_pressed(egui::Key::Enter)));
                if submit {
                    let expression = app
                        .state
                        .ui
                        .code_workspace
                        .automation
                        .debug
                        .watch_input
                        .trim()
                        .to_owned();
                    let valid = valid_automation_watch_expression(&expression);
                    let duplicate = app
                        .state
                        .ui
                        .code_workspace
                        .automation
                        .debug
                        .watches
                        .iter()
                        .any(|watch| watch.expression == expression);
                    if valid && !duplicate {
                        app.state.ui.code_workspace.automation.debug.watches.push(
                            AutomationWatch {
                                expression,
                                value: String::new(),
                                error: None,
                            },
                        );
                        app.state
                            .ui
                            .code_workspace
                            .automation
                            .debug
                            .watch_input
                            .clear();
                        crate::workbench::documents::code_workspace::refresh_automation_debugger(
                            app,
                        );
                    }
                }
            });
            for watch in app.state.ui.code_workspace.automation.debug.watches.clone() {
                property_row(
                    ui,
                    &watch.expression,
                    watch.error.as_deref().unwrap_or(&watch.value),
                );
            }

            let output = app
                .state
                .ui
                .code_workspace
                .automation
                .debug
                .output
                .iter()
                .rev()
                .take(5)
                .cloned()
                .collect::<Vec<_>>();
            if !output.is_empty() {
                ui.add_space(7.0);
                ui.label(
                    egui::RichText::new(messages.text(MessageId::AutomationStructuredOutput))
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_faint),
                );
                for record in output.into_iter().rev() {
                    property_row(
                        ui,
                        &format!("{} · {}", record.sequence, record.event),
                        &record.detail,
                    );
                }
            }
            ui.add_space(7.0);
        },
    );
}

struct ExecutionPreview {
    task_points: usize,
    pvt_points: usize,
    release_checks: usize,
    slots: u32,
    release_ready: bool,
}

fn execution_preview(app: &RSpiceApp) -> ExecutionPreview {
    use crate::simulation::plan::AnalysisKind;
    let (enabled, corner_enabled) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            let enabled = plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
                .count();
            let corner = plan
                .instances()
                .iter()
                .any(|instance| instance.enabled() && instance.kind() == AnalysisKind::Corner);
            (enabled, corner)
        })
        .unwrap_or_default();
    let pvt_points = if corner_enabled {
        app.state
            .sim_setup
            .corner
            .to_config(
                &app.state.sim_setup.run_set,
                app.state.sim_setup.reference_pvt,
            )
            .map_or(0, |config| config.num_corners())
    } else {
        0
    };
    let release_checks = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
        .map_or(0, |plan_id| app.state.workspace.active_specs(plan_id).len());
    ExecutionPreview {
        task_points: crate::workbench::documents::code_workspace::automation_task_count(app)
            .unwrap_or_else(|_| enabled.saturating_mul(pvt_points.max(1))),
        pvt_points,
        release_checks,
        slots: app
            .state
            .ui
            .preferences
            .scalar(ScalarPreference::LocalParallelSlots),
        release_ready: corner_enabled && pvt_points > 0 && release_checks > 0,
    }
}

fn artifact_row(ui: &mut Ui, app: &mut RSpiceApp, kind: ArtifactKind, meta: &str) {
    let t = Tokens::get(ui.ctx());
    let available = app
        .state
        .ui
        .code_workspace
        .automation
        .artifacts
        .get(kind)
        .is_some();
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), 28.0),
        if available {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let accessible_label = format!("Export {}", kind.file_name());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, available, &accessible_label)
    });
    if response.hovered() && available {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let icon = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 15.0, rect.center().y),
        Vec2::splat(14.0),
    );
    WorkbenchIcon::File.paint(
        ui.painter(),
        icon,
        if available {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
    ui.painter().text(
        egui::pos2(rect.left() + 28.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        kind.file_name(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        if available {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        meta,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect.shrink(2.0));
    let keyboard_activate = available
        && response.has_focus()
        && ui.ctx().input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                || input.consume_key(egui::Modifiers::NONE, egui::Key::Space)
        });
    if (response.clicked() || keyboard_activate) && available {
        crate::workbench::documents::code_workspace::export_automation_artifact(app, kind);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_actions_follow_workspace_width_not_viewport_width() {
        assert!(title_actions_stack(TITLE_ACTION_STACK_BREAKPOINT));
        assert!(!title_actions_stack(TITLE_ACTION_STACK_BREAKPOINT + 1.0));
    }

    #[test]
    fn watch_envelope_validation_defers_python_grammar_to_the_managed_runtime() {
        assert!(valid_automation_watch_expression("value +"));
        assert!(!valid_automation_watch_expression("   \n"));
        assert!(!valid_automation_watch_expression("value\0suffix"));
        assert!(valid_automation_watch_expression(
            &"x".repeat(rspice_automation_protocol::MAX_WATCH_EXPRESSION_BYTES)
        ));
        assert!(!valid_automation_watch_expression(&"x".repeat(
            rspice_automation_protocol::MAX_WATCH_EXPRESSION_BYTES + 1
        )));
    }
}
