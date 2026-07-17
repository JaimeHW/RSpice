//! Mockup-defined Automation and CI workspace backed by governed execution.

use egui::{Align, Layout, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::automation_workflow::ArtifactKind;
use crate::common::RSpiceApp;
use crate::state::ProjectSourceLanguage;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::ScalarPreference;

use super::super::code_workspace::{
    AutomationExecutionState, CodeEditorLanguage, CodeWorkspacePage, show_code_editor,
};
use super::super::design_system::{
    WorkbenchIcon, code_inspector_property_list, code_inspector_section, code_workspace_heading,
    property_row, workspace_title_row,
};

const INTERNAL_INSPECTOR_MIN_WIDTH: f32 = 270.0;
const INTERNAL_INSPECTOR_WIDTH: f32 = 330.0;
const EDITOR_TOOLBAR_HEIGHT: f32 = 33.0;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    super::super::code_workspace::poll_automation_workflow(app);
    let t = Tokens::get(ui.ctx());
    let document = app
        .state
        .workspace
        .project_sources
        .get(ProjectSourceLanguage::RSpiceAutomation)
        .cloned();
    let runtime_stale = app
        .state
        .ui
        .code_workspace
        .automation
        .receipt
        .as_ref()
        .is_some_and(|receipt| {
            document.as_ref().is_none_or(|document| {
                receipt.token.project_id != app.state.workspace.project.id()
                    || receipt.token.revision != document.revision().get()
                    || receipt.token.content_digest != document.content_digest()
            })
        });
    if runtime_stale {
        app.state.ui.code_workspace.automation = Default::default();
    }
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ui.set_min_size(ui.available_size());
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        title_row(ui, app);
        let Some(document) = document else {
            super::super::design_system::empty_state(
                ui,
                WorkbenchIcon::Terminal,
                "No Automation source",
                "This project does not own an Automation workflow document.",
            );
            return;
        };
        let mut source = document.content().to_owned();
        let runtime = &app.state.ui.code_workspace.automation;
        let diagnostics = runtime
            .diagnostic_token
            .filter(|token| {
                token.project_id == app.state.workspace.project.id()
                    && token.revision == document.revision().get()
                    && token.content_digest == document.content_digest()
            })
            .map_or_else(Vec::new, |_| runtime.diagnostics.clone());
        let stacked = ui.ctx().content_rect().width() <= 820.0;
        if stacked {
            ScrollArea::vertical()
                .id_salt("workbench.automation.stacked")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    let editor_height = ui.clip_rect().height().max(1.0);
                    let code_pane = ui
                        .allocate_ui(Vec2::new(ui.available_width(), editor_height), |ui| {
                            code_pane(ui, app, &document, &mut source, &diagnostics)
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
                .id_salt("workbench.automation.inspector")
                .auto_shrink([false, false])
                .show(&mut inspector_ui, |ui| inspector(ui, app));
            ui.allocate_rect(available, Sense::hover());
        }
    });
}

fn title_row(ui: &mut Ui, app: &mut RSpiceApp) {
    let phone = ui.ctx().content_rect().width() <= 560.0;
    workspace_title_row(ui, |ui| {
        if phone {
            code_workspace_heading(
                ui,
                "AUTOMATION · LOCAL SCRIPT · REPRODUCIBLE BATCH",
                "Automation and CI console",
                "Compose project-aware run plans, exports, comparisons and machine-readable release gates.",
            );
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                let width = ((ui.available_width() - 6.0) * 0.5).max(1.0);
                generated_netlist_button(ui, app, width);
                run_button(ui, app, width);
            });
        } else {
            let run_label = run_button_label(app);
            let actions_width = title_button_width(ui, "Generated netlist", false)
                + 6.0
                + title_button_width(ui, run_label, true);
            let heading_width = (ui.available_width() - actions_width - 12.0).max(1.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.allocate_ui_with_layout(
                    Vec2::new(heading_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        code_workspace_heading(
                            ui,
                            "AUTOMATION · LOCAL SCRIPT · REPRODUCIBLE BATCH",
                            "Automation and CI console",
                            "Compose project-aware run plans, exports, comparisons and machine-readable release gates.",
                        );
                    },
                );
                ui.add_space(6.0);
                generated_netlist_button(ui, app, 0.0);
                run_button(ui, app, 0.0);
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

fn run_button(ui: &mut Ui, app: &mut RSpiceApp, width: f32) {
    let active = app.state.ui.code_workspace.automation.execution.is_active();
    let label = run_button_label(app);
    let mut button = Button::new(label).accent().enabled(!active);
    if width > 0.0 {
        button = button.min_width(width).max_width(width);
    }
    if button.show(ui).clicked() {
        super::super::code_workspace::start_automation_workflow(app);
    }
}

fn run_button_label(app: &RSpiceApp) -> &'static str {
    if app.state.ui.code_workspace.automation.execution.is_active() {
        "Workflow running…"
    } else {
        "Run workflow"
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
    let runtime = &app.state.ui.code_workspace.automation;
    let validated = runtime.receipt.as_ref().is_some_and(|receipt| {
        receipt.token.project_id == app.state.workspace.project.id()
            && receipt.token.revision == document.revision().get()
            && receipt.token.content_digest == document.content_digest()
    });
    let (status, color) = if runtime.execution.is_active() {
        (runtime.execution.label(), t.color.info)
    } else if matches!(
        runtime.execution,
        AutomationExecutionState::Complete { passed: true, .. }
    ) {
        (runtime.execution.label(), t.color.ok)
    } else if matches!(
        runtime.execution,
        AutomationExecutionState::Complete { passed: false, .. } | AutomationExecutionState::Failed
    ) {
        (runtime.execution.label(), t.color.err)
    } else if validated {
        ("validated", t.color.ok)
    } else {
        ("modified · validation required", t.color.warn)
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
            "workbench.automation.source",
            source,
            CodeEditorLanguage::Automation,
            diagnostics,
        ) {
            match app
                .state
                .workspace
                .replace_project_source(ProjectSourceLanguage::RSpiceAutomation, source.clone())
            {
                Ok(true) => {
                    let runtime = &mut app.state.ui.code_workspace.automation;
                    runtime.receipt = None;
                    runtime.diagnostics.clear();
                    runtime.diagnostic_token = None;
                    runtime.execution = AutomationExecutionState::Idle;
                    runtime.artifacts.clear();
                    runtime.last_error = None;
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

fn inspector(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.set_width(ui.available_width());
    let preview = execution_preview(app);
    let preview_status = format!(
        "{} tasks · {}",
        preview.task_points,
        if preview.release_ready {
            "release plan"
        } else {
            "non-sign-off"
        }
    );
    let preview_tone = if preview.release_ready {
        t.color.accent
    } else {
        t.color.warn
    };
    code_inspector_section(
        ui,
        "Execution preview",
        Some((&preview_status, preview_tone)),
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    "Project revision",
                    &app.state.workspace.project.revision().get().to_string(),
                );
                property_row(
                    ui,
                    "Run target",
                    &format!("local · {} slots", preview.slots),
                );
                property_row(ui, "PVT points", &preview.pvt_points.to_string());
                property_row(
                    ui,
                    "Release checks",
                    &format!("{} configured", preview.release_checks),
                );
            });
        },
    );

    code_inspector_section(ui, "Artifacts", None, |ui| {
        ui.add_space(4.0);
        artifact_row(ui, app, ArtifactKind::JunitXml, "CI");
        artifact_row(ui, app, ArtifactKind::SummaryJson, "machine");
        artifact_row(ui, app, ArtifactKind::VerificationPdf, "report");
        ui.add_space(7.0);
    });

    code_inspector_section(ui, "Security", None, |ui| {
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(10, 9))
            .show(ui, |ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new("Scripts run with project-scoped file access. Remote targets require named authorization before any design data is transmitted.")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
    });
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
            .to_config()
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
        task_points: enabled.saturating_mul(pvt_points),
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
        .iter()
        .any(|artifact| artifact.kind() == kind);
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
        super::super::code_workspace::export_automation_artifact(app, kind);
    }
}
