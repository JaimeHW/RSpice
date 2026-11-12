//! Mockup-defined Automation and CI workspace backed by governed execution.

use egui::{Align, Layout, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::automation_workflow::ArtifactKind;
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::CodeFeature;
use crate::workbench::code::commands::{AutomationRequest, CodeRequest};
use crate::workbench::framework::feature::Cx;
use crate::workbench::{MessageCatalog, MessageId};

use super::super::design_system::inspector::muted_inspector_copy;
use super::super::design_system::{
    Problem, WorkbenchIcon, code_inspector_section, heading, paint_problem_mark, panel_bleed,
    panel_body, panel_inset, property_row, property_row_height, section_block_gap,
    subsection_header, workspace_title_row,
};
use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::workbench::documents::code_workspace::{
    AutomationExecutionState, CodeEditorLanguage, CodeWorkspaceRuntimeState, EditorFindings,
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

/// The Automation workspace's source bundle, when the project has one.
fn automation_bundle(model: &Model) -> Option<&crate::state::ProjectSourceBundle> {
    model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
}

/// The file the page shows from `bundle`: the one selected while the bundle
/// still holds it, else its root.
fn shown_path(
    bundle: &crate::state::ProjectSourceBundle,
    code: &CodeWorkspaceRuntimeState,
) -> String {
    match code.automation.selected_file.as_deref() {
        Some(path) if bundle.contains_file(path) => path.to_owned(),
        _ => bundle.root().logical_path().to_owned(),
    }
}

/// The editor the page shows the file at `selected_path` in: it writes the
/// file only while the runtime is idle, never the environment lock the
/// manifest names, and only where the project lets the source be written.
fn page_editor(
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
    platform: &PlatformState,
    selected_path: &str,
) -> crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
    let language = automation_editor_language(selected_path);
    let editable = !automation_busy(code)
        && code.automation.manifest.as_ref().is_none_or(|manifest| {
            !manifest
                .environment_lock_path
                .eq_ignore_ascii_case(selected_path)
        })
        && crate::workbench::documents::code_workspace::source_document_is_editable(
            model,
            code,
            platform,
            ProjectSourceLanguage::RSpiceAutomation,
            selected_path,
        );
    crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
        id: egui::Id::new(("workbench.automation.source", selected_path)),
        editable,
        // The project index navigates Python and the Automation script;
        // run plans and manifests are data.
        navigable: matches!(
            language,
            CodeEditorLanguage::Python | CodeEditorLanguage::Automation
        ),
    }
}

/// Before the page draws, in the code feature's context: drop the runtime's
/// evidence once it no longer matches the workspace's closure, and put the
/// page's editor in front, before the shell takes the dispatcher's answers
/// the page's context holds, so they are the answers the editor gets; the
/// page publishes it again as it draws it.
pub(in crate::workbench) fn prepare(cx: &mut Cx<'_, CodeFeature>) {
    let bundle = automation_bundle(cx.model);
    let runtime_stale = cx
        .state
        .code_workspace
        .automation
        .receipt
        .as_ref()
        .is_some_and(|receipt| {
            bundle.as_ref().is_none_or(|bundle| {
                receipt.token.project_id != cx.model.workspace.content.project.id()
                    || receipt.token.revision != bundle.revision().get()
                    || receipt.token.content_digest != bundle.closure_digest()
            })
        });
    if runtime_stale {
        cx.state.code_workspace.automation = Default::default();
    }
    let Some(bundle) = bundle else {
        return;
    };
    let selected_path = shown_path(bundle, &cx.state.code_workspace);
    if bundle.file_content(&selected_path).is_some() {
        cx.wb.active_source_editor = Some(page_editor(
            cx.model,
            &cx.state.code_workspace,
            cx.platform,
            &selected_path,
        ));
    }
}

/// Whether the page's editor's right-click menu is open, or may open on
/// what this frame carries. The page's context holds the menu's answers
/// only then, so the shell reads this before it lends the context.
pub(in crate::workbench) fn source_menu_open(
    ctx: &egui::Context,
    model: &Model,
    code: &CodeWorkspaceRuntimeState,
) -> bool {
    automation_bundle(model).is_some_and(|bundle| {
        crate::workbench::documents::virtual_text_editor::context_menu_may_show(
            ctx,
            crate::workbench::documents::code_workspace::source_editor_id(
                ProjectSourceLanguage::RSpiceAutomation,
                bundle.id(),
                &shown_path(bundle, code),
            ),
        )
    })
}

/// Draw the Automation page in the code feature's context, once it has
/// prepared. The context holds its menu's answers while
/// [`source_menu_open`] says the menu is open or opening.
pub(in crate::workbench) fn show(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) {
    // A file dropped on the page is imported once the page has drawn.
    let dropped = ui.ctx().input(|input| input.raw.dropped_files.clone());
    if !dropped.is_empty() {
        cx.wb
            .outbox
            .push(CodeRequest::Automation(AutomationRequest::ImportDropped(
                dropped,
            )));
    }
    let t = Tokens::get(ui.ctx());
    let bundle = cx
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .cloned();
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        title_row(ui, cx);
        let Some(bundle) = bundle else {
            let messages = cx.wb.messages();
            super::super::design_system::empty_state_with_actions(
                ui,
                WorkbenchIcon::Terminal,
                &messages.text(MessageId::AutomationNoSource),
                &messages.text(MessageId::AutomationNoSourceDescription),
                |ui| {
                    if Button::new(&messages.text(MessageId::CodeSourceCreateWorkspace))
                        .show(ui)
                        .clicked()
                    {
                        cx.wb
                            .outbox
                            .push(CodeRequest::Automation(AutomationRequest::CreateWorkspace));
                    }
                },
            );
            return;
        };
        let selected_path = match cx.state.code_workspace.automation.selected_file.clone() {
            Some(path) if bundle.contains_file(&path) => path,
            Some(_) => {
                cx.state.code_workspace.automation.selected_file = None;
                bundle.root().logical_path().to_owned()
            }
            None => bundle.root().logical_path().to_owned(),
        };
        let Some(mut source) = bundle.file_content(&selected_path).map(str::to_owned) else {
            let messages = cx.wb.messages();
            super::super::design_system::empty_state(
                ui,
                WorkbenchIcon::File,
                &messages.text(MessageId::AutomationFileMissing),
                &messages.text(MessageId::AutomationFileMissingDescription),
            );
            return;
        };
        let runtime = &cx.state.code_workspace.automation;
        let diagnostics = runtime
            .diagnostic_token
            .filter(|token| {
                token.project_id == cx.model.workspace.content.project.id()
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
                                        cx,
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
                                inspector(ui, cx);
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
                cx,
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
                .show(&mut inspector_ui, |ui| inspector(ui, cx));
            ui.allocate_rect(available, Sense::hover());
        }
    });
}

fn title_row(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) {
    let messages = cx.wb.messages();
    let title = messages.text(MessageId::AutomationHeadingTitle);
    let generated_netlist = messages.text(MessageId::CodeGeneratedNetlist);
    let validate = messages.text(MessageId::AutomationValidate);
    let dry_run = messages.text(MessageId::AutomationDryRun);
    let stack_actions = title_actions_stack(visible_workspace_rect(ui).width());
    workspace_title_row(ui, |ui| {
        if stack_actions {
            heading(ui, &title, None);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 18.0) * 0.25).max(1.0);
                generated_netlist_button(ui, cx, width);
                validate_button(ui, cx, width);
                dry_run_button(ui, cx, width);
                run_button(ui, cx, width);
            });
        } else {
            let run_label = run_button_label(&cx.state.code_workspace, cx.wb.messages());
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
                        heading(ui, &title, None);
                    },
                );
                ui.add_space(6.0);
                generated_netlist_button(ui, cx, 0.0);
                validate_button(ui, cx, 0.0);
                dry_run_button(ui, cx, 0.0);
                run_button(ui, cx, 0.0);
            });
        }
    });
}

fn validate_button(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>, width: f32) {
    let busy = automation_busy(&cx.state.code_workspace);
    let workspace_available = automation_workspace_available(cx.model);
    let messages = cx.wb.messages();
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
        cx.wb
            .outbox
            .push(CodeRequest::Automation(AutomationRequest::Validate));
    }
}

fn dry_run_button(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>, width: f32) {
    let busy = automation_busy(&cx.state.code_workspace);
    let workspace_available = automation_workspace_available(cx.model);
    let messages = cx.wb.messages();
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
        cx.wb
            .outbox
            .push(CodeRequest::Automation(AutomationRequest::DryRun));
    }
}

fn generated_netlist_button(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>, width: f32) {
    let label = cx.wb.messages().text(MessageId::CodeGeneratedNetlist);
    let mut button = Button::new(&label);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        cx.wb.outbox.push(CodeRequest::Automation(
            AutomationRequest::OpenGeneratedNetlist,
        ));
    }
}

fn run_button(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>, width: f32) {
    let active = cx.state.code_workspace.automation.execution.is_active();
    let validating = cx
        .state
        .code_workspace
        .automation
        .pending_runtime_validation
        .is_some();
    let label = run_button_label(&cx.state.code_workspace, cx.wb.messages());
    let workspace_available = automation_workspace_available(cx.model);
    let unavailable = cx
        .wb
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
        let request = if active {
            AutomationRequest::Stop
        } else {
            AutomationRequest::Run
        };
        cx.wb.outbox.push(CodeRequest::Automation(request));
    }
}

fn run_button_label(code: &CodeWorkspaceRuntimeState, messages: MessageCatalog) -> String {
    if code.automation.pending_runtime_validation.is_some() {
        messages.text(MessageId::AutomationValidating)
    } else if code.automation.execution.is_active() {
        if code.automation.cancel_requested {
            messages.text(MessageId::AutomationStopping)
        } else {
            messages.text(MessageId::AutomationStopWorkflow)
        }
    } else {
        messages.text(MessageId::AutomationRunWorkflow)
    }
}

fn automation_busy(code: &CodeWorkspaceRuntimeState) -> bool {
    let runtime = &code.automation;
    runtime.execution.is_active()
        || runtime.pending_runtime_validation.is_some()
        || runtime.runtime_execution.is_some()
}

fn automation_workspace_available(model: &Model) -> bool {
    model
        .workspace
        .content
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
    cx: &mut Cx<'_, CodeFeature>,
    bundle: &crate::state::ProjectSourceBundle,
    selected_path: &str,
    source: &mut String,
    diagnostics: &CodeDiagnosticCollection,
) {
    let t = Tokens::get(ui.ctx());
    let legacy = bundle.root().logical_path().ends_with(".rspice");
    let language = automation_editor_language(selected_path);
    let editor = page_editor(
        cx.model,
        &cx.state.code_workspace,
        cx.platform,
        selected_path,
    );
    let editable = editor.editable;
    let editor_id = editor.id;
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
    let runtime = &cx.state.code_workspace.automation;
    let validated = runtime.receipt.as_ref().is_some_and(|receipt| {
        receipt.token.project_id == cx.model.workspace.content.project.id()
            && receipt.token.revision == bundle.revision().get()
            && receipt.token.content_digest == bundle.closure_digest()
    });
    let messages = cx.wb.messages();
    // One live dot while the runtime is working, one mark when a workflow
    // came back failed, and the words themselves the rest of the time. A
    // validated script is the ordinary case and says nothing.
    let live = runtime.pending_runtime_validation.is_some() || runtime.execution.is_active();
    let failed = matches!(
        runtime.execution,
        AutomationExecutionState::Complete { passed: false, .. } | AutomationExecutionState::Failed
    );
    let status = if runtime.pending_runtime_validation.is_some() {
        messages.text(MessageId::AutomationStatusValidating)
    } else if matches!(runtime.execution, AutomationExecutionState::Idle) {
        if validated {
            String::new()
        } else {
            messages.text(MessageId::AutomationNotValidated)
        }
    } else {
        automation_execution_label(&runtime.execution, messages)
    };
    let status_galley = ui.painter().layout_no_wrap(
        status,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let toolbar_content_right = toolbar.right() - EDITOR_SCROLLBAR_INSET;
    let status_mark_x =
        toolbar_content_right - status_galley.size().x - crate::ui::widgets::PROBLEM_MARK_ADVANCE;
    if live {
        ui.painter().circle_filled(
            egui::pos2(status_mark_x, toolbar.center().y),
            2.5,
            t.color.accent,
        );
    } else if failed {
        paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(status_mark_x, toolbar.center().y),
                Vec2::splat(TOOLBAR_MARK_SIDE),
            ),
            Problem::Failure,
        );
    }
    ui.painter().galley(
        egui::pos2(
            toolbar_content_right - status_galley.size().x - 8.0,
            toolbar.center().y - status_galley.size().y * 0.5,
        ),
        status_galley.clone(),
        t.color.text_dim,
    );
    // Keep even unusually long logical paths out of the status. The full path
    // remains available from the file tree.
    let path_right = (status_mark_x - 13.0).max(path_clip.left());
    let path_clip =
        egui::Rect::from_min_max(path_clip.min, egui::pos2(path_right, path_clip.bottom()));
    ui.painter().with_clip_rect(path_clip).text(
        egui::pos2(toolbar.left() + 8.0, toolbar.center().y),
        egui::Align2::LEFT_CENTER,
        selected_path,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let search_input =
        crate::workbench::documents::text_editor_commands::take_find_in_source_bundle_request(
            ui, editor_id,
        );
    let preceding_input = search_input
        .as_ref()
        .map(|input| input.preceding_scope(ui.ctx()));
    super::netlist::publish_source_editor(ui.ctx(), cx.wb, editor);
    let interaction = egui::Frame::new().fill(t.color.bg_inset).show(ui, |ui| {
        let debuggable = language == CodeEditorLanguage::Python;
        let interaction = if debuggable && !legacy {
            let selected_document_id = bundle.document_id(selected_path).map(|id| id.as_uuid());
            let breakpoints = cx.state.code_workspace
                .automation
                .debug
                .breakpoints
                .iter()
                .filter(|breakpoint| {
                    breakpoint.enabled && Some(breakpoint.document_id) == selected_document_id
                })
                .map(|breakpoint| breakpoint.line)
                .collect::<Vec<_>>();
            let current_line = cx.state.code_workspace
                .automation
                .debug
                .selected_frame_id
                .and_then(|frame_id| {
                    cx.state.code_workspace
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
                EditorFindings::Current(diagnostics),
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
                EditorFindings::Current(diagnostics),
                Some(selected_path),
                editable,
                messages,
            )
        };
        if let Some(line) = interaction.breakpoint_toggled {
            cx.wb
                .outbox
                .push(CodeRequest::Automation(AutomationRequest::ToggleBreakpoint(line)));
        }
        if interaction.changed {
            match cx.model.workspace.content.replace_project_source_bundle_file(
                bundle.id(),
                selected_path,
                source.clone(),
            ) {
                Ok(true) => {
                    crate::workbench::documents::code_workspace::invalidate_automation_evidence_in(
                        &mut cx.state.code_workspace,
                        cx.own,
                    );
                }
                Ok(false) => {}
                Err(error) => {
                    cx.wb.prompt(format!("Could not update {selected_path}: {error}"));
                    // The editor goes back to the source as it is stored.
                    crate::workbench::documents::virtual_text_editor::resynchronize(
                        ui.ctx(),
                        crate::workbench::documents::code_workspace::source_editor_id(
                            crate::state::ProjectSourceLanguage::RSpiceAutomation,
                            bundle.id(),
                            selected_path,
                        ),
                    );
                }
            }
        }
        interaction
    }).inner;
    if let Some(announcement) = interaction.announcement.clone() {
        cx.wb.prompt(announcement);
    }
    cx.state.code_workspace.source_carets.insert(
        (bundle.id(), selected_path.to_owned()),
        interaction.cursor_char_index,
    );
    let selection_key = (bundle.id(), selected_path.to_owned());
    if let Some(range) = interaction.selected_char_range {
        cx.state
            .code_workspace
            .source_selections
            .insert(selection_key, range);
    } else {
        cx.state
            .code_workspace
            .source_selections
            .remove(&selection_key);
    }
    // The right-click menu's rows read the page's context, which holds the
    // menu's answers while the menu is open or opening; the row chosen runs
    // once the page has drawn.
    let choice = crate::workbench::chrome::title_bar::source_editor_context_menu_over(
        &interaction.response,
        |ctx| {
            super::netlist::source_menu::rows(crate::workbench::code::SOURCE_EDITOR_MENU, cx, ctx)
        },
    );
    if let Some(choice) = choice {
        cx.execute(choice.command());
    }
    if let Some(step) = interaction.unanswered_history {
        match cx.state.code_workspace.source_edit_history.step(
            &mut cx.model.workspace.content,
            bundle.id(),
            step,
        ) {
            Ok(true) => {
                crate::workbench::documents::code_workspace::invalidate_automation_evidence_in(
                    &mut cx.state.code_workspace,
                    cx.own,
                );
            }
            Ok(false) => {}
            Err(error) => cx.wb.prompt(error),
        }
    }
    if crate::workbench::documents::text_editor_commands::take_format_document_request(
        ui, editor_id,
    ) {
        // Formatting is carried out once the page has drawn. A format the
        // palette, a menu or a shortcut asked for is taken a frame after it
        // was asked, in a frame no input began, so this frame asks for the
        // next at once to draw what formatting wrote.
        ui.ctx().request_repaint();
        cx.wb
            .outbox
            .push(CodeRequest::Automation(AutomationRequest::FormatDocument {
                bundle_id: bundle.id(),
                path: selected_path.to_owned(),
            }));
    }
    drop(preceding_input);
    if let Some(input) = search_input {
        // The search binds to the source revisions and the selection once
        // the page has drawn, after preceding edits.
        cx.wb
            .outbox
            .push(CodeRequest::Automation(AutomationRequest::FindInBundle {
                path: selected_path.to_owned(),
            }));
        input.route(
            ui.ctx(),
            crate::workbench::documents::code_workspace::source_search_query_id(),
        );
    }
}

fn inspector(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) {
    ui.set_width(ui.available_width());
    panel_body(ui, |ui| inspector_sections(ui, cx));
}

fn inspector_sections(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) {
    let messages = cx.wb.messages();
    let manifest = cx.state.code_workspace.automation.manifest.clone();
    let preview = execution_preview(cx.model);
    let runtime_status =
        crate::workbench::documents::code_workspace::automation_runtime_unavailable_reason(cx.own)
            .unwrap_or_else(|| messages.text(MessageId::AutomationManagedRuntimeVerified));
    let task_count = preview.task_points.to_string();
    // What the preview would run is a count. Whether that plan would also
    // qualify as a release is a disclaimer, and the Release checks row below
    // already says how many of them are configured.
    let preview_status = messages.format(
        if preview.task_points == 1 {
            MessageId::AutomationTaskSummarySingular
        } else {
            MessageId::AutomationTaskSummary
        },
        &[("count", &task_count)],
    );
    let not_validated = messages.text(MessageId::AutomationNotValidated);

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationRuntimeApi),
        None,
        None,
        |ui| {
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
        },
    );

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationPermissions),
        None,
        None,
        |ui| {
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
        },
    );

    let diagnostic_count = cx.state.code_workspace.automation.diagnostics.len();
    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationDiagnostics),
        // "Current findings" is the next row down, so the header carries the
        // mark and nothing else. No findings is the ordinary case: a header
        // that says "clear" is a word spent on the absence of a problem.
        None,
        (diagnostic_count > 0).then_some(Problem::Failure),
        |ui| {
            property_row(
                ui,
                &messages.text(MessageId::AutomationCurrentFindings),
                &diagnostic_count.to_string(),
            );
            let validation_status =
                messages.text(if cx.state.code_workspace.automation.receipt.is_some() {
                    MessageId::AutomationExactRevisionRetained
                } else {
                    MessageId::AutomationRequired
                });
            property_row(
                ui,
                &messages.text(MessageId::AutomationValidation),
                &validation_status,
            );
        },
    );
    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationExecutionPreview),
        Some(&preview_status),
        None,
        |ui| {
            property_row(
                ui,
                &messages.text(MessageId::AutomationProjectRevision),
                &cx.model
                    .workspace
                    .content
                    .project
                    .revision()
                    .get()
                    .to_string(),
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
        },
    );

    debugger(ui, cx);

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationArtifacts),
        None,
        None,
        |ui| {
            artifact_row(
                ui,
                cx,
                ArtifactKind::JunitXml,
                &messages.text(MessageId::AutomationArtifactCi),
            );
            artifact_row(
                ui,
                cx,
                ArtifactKind::SummaryJson,
                &messages.text(MessageId::AutomationArtifactMachine),
            );
            artifact_row(
                ui,
                cx,
                ArtifactKind::VerificationPdf,
                &messages.text(MessageId::AutomationArtifactReport),
            );
        },
    );

    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationSessionSafety),
        None,
        None,
        |ui| {
            muted_inspector_copy(
                ui,
                &messages.text(MessageId::AutomationManagedRuntimeSafety),
            );
        },
    );
}

fn debugger(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>) {
    use crate::workbench::documents::code_workspace::{
        AutomationBreakpointKind, AutomationDebugPhase, AutomationExceptionPolicy, AutomationWatch,
    };

    let t = Tokens::get(ui.ctx());
    let messages = cx.wb.messages();
    let redacted = messages.text(MessageId::AutomationRedacted);
    let phase = cx.state.code_workspace.automation.debug.phase;
    // Paused, running and stopped are where the session is, not something
    // wrong with it; only a failed session is marked.
    let problem = match phase {
        AutomationDebugPhase::Failed => Some(Problem::Failure),
        AutomationDebugPhase::Paused
        | AutomationDebugPhase::Running
        | AutomationDebugPhase::Inactive
        | AutomationDebugPhase::Stopped => None,
    };
    let phase_label = automation_debug_phase_label(phase, messages);
    code_inspector_section(
        ui,
        &messages.text(MessageId::AutomationDebug),
        Some(&phase_label),
        problem,
        |ui| {
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
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::StartDebugger));
                }
                let continue_label = messages.text(MessageId::AutomationDebugContinue);
                if Button::new(&continue_label)
                    .enabled(phase == AutomationDebugPhase::Paused)
                    .show(ui)
                    .clicked()
                {
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::ContinueDebugger));
                }
                let pause = messages.text(
                    if cx.state.code_workspace.automation.debug.pause_requested {
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
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::PauseDebugger));
                }
                let step_over = messages.text(MessageId::AutomationDebugStepOver);
                if Button::new(&step_over)
                    .enabled(phase == AutomationDebugPhase::Paused)
                    .show(ui)
                    .clicked()
                {
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::StepOver));
                }
                let step_into = messages.text(MessageId::AutomationDebugStepInto);
                if Button::new(&step_into)
                    .enabled(phase == AutomationDebugPhase::Paused)
                    .show(ui)
                    .clicked()
                {
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::StepInto));
                }
                let step_out = messages.text(MessageId::AutomationDebugStepOut);
                if Button::new(&step_out)
                    .enabled(
                        phase == AutomationDebugPhase::Paused
                            && cx.state.code_workspace.automation.debug.call_stack.len() > 1,
                    )
                    .show(ui)
                    .clicked()
                {
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::StepOut));
                }
                let restart = messages.text(MessageId::AutomationDebugRestart);
                if Button::new(&restart)
                    .enabled(phase != AutomationDebugPhase::Inactive)
                    .show(ui)
                    .clicked()
                {
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::RestartDebugger));
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
                    cx.wb
                        .outbox
                        .push(CodeRequest::Automation(AutomationRequest::StopDebugger));
                }
            });
            section_block_gap(ui);
            property_row(
                ui,
                &messages.text(MessageId::AutomationDebugCurrentLine),
                &cx.state
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
                    cx.state.code_workspace.automation.debug.exception_policy,
                    messages,
                ),
            );

            let mut policy = cx.state.code_workspace.automation.debug.exception_policy;
            crate::ui::widgets::Combo::from_id_salt("automation.debug.exceptions")
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
            cx.state.code_workspace.automation.debug.exception_policy = policy;

            let breakpoints = cx.state.code_workspace.automation.debug.breakpoints.clone();
            // A group header titles its rows; with no breakpoints set it
            // would title nothing.
            if !breakpoints.is_empty() {
                subsection_header(ui, &messages.text(MessageId::AutomationBreakpoints));
            }
            let mut remove_breakpoint = None;
            let mut breakpoints_changed = false;
            for breakpoint in breakpoints {
                ui.horizontal(|ui| {
                    let mut enabled = breakpoint.enabled;
                    if ui.checkbox(&mut enabled, "").changed()
                        && let Some(current) = cx
                            .state
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
                        crate::ui::widgets::Combo::from_id_salt((
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
                            && let Some(current) = cx
                                .state
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
                let active = cx
                    .state
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
                            && let Some(current) = cx
                                .state
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
                                && let Some(current) = cx
                                    .state
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
                cx.wb.outbox.push(CodeRequest::Automation(
                    AutomationRequest::RemoveBreakpoint { document_id, line },
                ));
            } else if breakpoints_changed {
                cx.wb
                    .outbox
                    .push(CodeRequest::Automation(AutomationRequest::SyncBreakpoints));
            }

            let frames = cx.state.code_workspace.automation.debug.call_stack.clone();
            if !frames.is_empty() {
                subsection_header(ui, &messages.text(MessageId::AutomationCallStack));
                let selected = cx.state.code_workspace.automation.debug.selected_frame_id;
                for frame in frames {
                    if ui
                        .selectable_label(
                            selected == Some(frame.frame_id),
                            format!("{}  {}:{}", frame.name, frame.source_path, frame.line),
                        )
                        .clicked()
                    {
                        cx.wb
                            .outbox
                            .push(CodeRequest::Automation(AutomationRequest::SelectFrame(
                                frame.frame_id,
                            )));
                    }
                }
            }

            let values = cx.state.code_workspace.automation.debug.locals.clone();
            if !values.is_empty() {
                subsection_header(ui, &messages.text(MessageId::AutomationLocals));
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

            let globals = cx.state.code_workspace.automation.debug.globals.clone();
            if !globals.is_empty() {
                subsection_header(ui, &messages.text(MessageId::AutomationGlobals));
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

            subsection_header(ui, &messages.text(MessageId::AutomationWatches));
            ui.horizontal(|ui| {
                // The field takes the row up to its button, which ends on
                // the inset.
                ui.spacing_mut().item_spacing.x = WATCH_ROW_GAP;
                let response = ui.add(
                    egui::TextEdit::singleline(
                        &mut cx.state.code_workspace.automation.debug.watch_input,
                    )
                    .hint_text(messages.text(MessageId::AutomationExpression))
                    .desired_width((ui.available_width() - WATCH_ADD_W - WATCH_ROW_GAP).max(60.0)),
                );
                let add = messages.text(MessageId::AutomationAdd);
                let submit = Button::new(&add).min_width(WATCH_ADD_W).show(ui).clicked()
                    || (response.lost_focus()
                        && ui.input(|input| input.key_pressed(egui::Key::Enter)));
                if submit {
                    let expression = cx
                        .state
                        .code_workspace
                        .automation
                        .debug
                        .watch_input
                        .trim()
                        .to_owned();
                    let valid = valid_automation_watch_expression(&expression);
                    let duplicate = cx
                        .state
                        .code_workspace
                        .automation
                        .debug
                        .watches
                        .iter()
                        .any(|watch| watch.expression == expression);
                    if valid && !duplicate {
                        cx.state
                            .code_workspace
                            .automation
                            .debug
                            .watches
                            .push(AutomationWatch {
                                expression,
                                value: String::new(),
                                error: None,
                            });
                        cx.state.code_workspace.automation.debug.watch_input.clear();
                        cx.wb
                            .outbox
                            .push(CodeRequest::Automation(AutomationRequest::RefreshWatches));
                    }
                }
            });
            for watch in cx.state.code_workspace.automation.debug.watches.clone() {
                property_row(
                    ui,
                    &watch.expression,
                    watch.error.as_deref().unwrap_or(&watch.value),
                );
            }

            let output = cx
                .state
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
                subsection_header(ui, &messages.text(MessageId::AutomationStructuredOutput));
                for record in output.into_iter().rev() {
                    property_row(
                        ui,
                        &format!("{} · {}", record.sequence, record.event),
                        &record.detail,
                    );
                }
            }
        },
    );
}

/// The watch row's Add button, and the gap between it and the field.
const WATCH_ADD_W: f32 = 48.0;
const WATCH_ROW_GAP: f32 = 6.0;

struct ExecutionPreview {
    task_points: usize,
    pvt_points: usize,
    release_checks: usize,
    /// Workers a run will actually get here: the reader's ceiling narrowed to
    /// the parallelism this host reports, which is the same arithmetic the
    /// engine does. Reporting the stored preference instead would print "0"
    /// for the default, which asks for every core rather than none.
    slots: usize,
}

fn execution_preview(model: &Model) -> ExecutionPreview {
    use crate::simulation::plan::AnalysisKind;
    let (enabled, corner_enabled) = model
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
        model
            .sim_setup
            .corner
            .to_config(&model.sim_setup.run_set, model.sim_setup.reference_pvt)
            .map_or(0, |config| config.num_corners())
    } else {
        0
    };
    let release_checks = model
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
        .map_or(0, |plan_id| {
            model.workspace.content.active_specs(plan_id).len()
        });
    ExecutionPreview {
        task_points: crate::workbench::documents::code_workspace::automation_task_count(model)
            .unwrap_or_else(|_| enabled.saturating_mul(pvt_points.max(1))),
        pvt_points,
        release_checks,
        slots: model
            .preferences
            .solver_worker_limit()
            .unwrap_or_else(|| rspice_core::ResourceLimits::default().max_parallel_workers)
            .min(std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)),
    }
}

/// An artifact row's file icon, on the panel inset, and the gap before its
/// name.
/// Side of the toolbar status's failure mark: the kit's own, so it matches
/// every other marked line in the product.
use crate::ui::widgets::PROBLEM_MARK_SIDE as TOOLBAR_MARK_SIDE;

const ARTIFACT_ICON_W: f32 = 14.0;
const ARTIFACT_ICON_GAP: f32 = 6.0;

fn artifact_row(ui: &mut Ui, cx: &mut Cx<'_, CodeFeature>, kind: ArtifactKind, meta: &str) {
    let t = Tokens::get(ui.ctx());
    let available = cx
        .state
        .code_workspace
        .automation
        .artifacts
        .get(kind)
        .is_some();
    let inset = panel_inset(ui);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), property_row_height(ui)),
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
        ui.painter()
            .rect_filled(panel_bleed(ui, rect), 0.0, t.color.bg_hover);
    }
    let icon = egui::Rect::from_min_size(
        egui::pos2(rect.left() + inset, rect.center().y - ARTIFACT_ICON_W * 0.5),
        Vec2::splat(ARTIFACT_ICON_W),
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
        egui::pos2(icon.right() + ARTIFACT_ICON_GAP, rect.center().y),
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
        egui::pos2(rect.right() - inset, rect.center().y),
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
        cx.wb
            .outbox
            .push(CodeRequest::Automation(AutomationRequest::ExportArtifact(
                kind,
            )));
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
