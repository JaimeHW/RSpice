//! Device-model specialist surface.
//!
//! The surface is a projection over the transactional editor state and the
//! persisted model-definition/qualification domains. It never supplies sample
//! model facts when those records are absent, and it does not turn a review
//! affordance into a release or qualification mutation.

mod promotion;
mod qualification;

use promotion::*;
use qualification::*;

use std::collections::{BTreeMap, BTreeSet};

use egui::{Align, Align2, Color32, Id, Layout, Pos2, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::state::model_library::{
    ApprovalDecision, CompatibilityDisposition, ConsumerChange, LicenseScope, LookupInterpolation,
    ModelDefinitionMetadata, ModelQualificationState, ModelReleaseCandidate,
    ModelSectionQualification, ModelSourceEvidenceBinding, ParameterDefinition, ParameterSource,
    QualificationAnalysis, QualificationEvidence, QualificationPlatform, QualificationPlatformRun,
    QualificationProbe, QualificationSample, QualificationSuite, QualificationVector,
    QualificationVectorDispositionCause, QualificationVectorRequiredAction,
    StatisticalDistribution, StatisticalHierarchyScope, TemperatureExtrapolationPolicy,
    TemperatureLawRepresentation,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize};
use crate::workbench::RSpiceApp;

use crate::workbench::commands::{CommandAvailability, ModelEditorWorkflow, active_model_editor_workflow, close_model_editor_workflow, prepare_model_editor_workflow, request_model_editor_workflow, set_model_editor_workflow_error};
use crate::workbench::commands::vocabulary::Command;
use super::super::design_system::{self, WorkbenchIcon};
use crate::workbench::documents::model_editor::{
    ModelComparisonDisposition, ModelEditorSection, ModelParameterKind,
    QualificationAuthoringAnalysis, QualificationAuthoringProbe, QualificationAuthoringSample,
};

const SPECIALIST_HEADER_H: f32 = 82.0;
const SPECIALIST_HEADER_MEDIUM_H: f32 = 124.0;
const OUTER_IDENTITY_H: f32 = 60.0;
const METRICS_H: f32 = 68.0;
const NAV_W: f32 = 214.0;
const NAV_ROW_H: f32 = 36.0;
const SECTION_HEADER_H: f32 = 70.0;
const TABLE_HEADER_H: f32 = 27.0;
const TABLE_MIN_W: f32 = 760.0;
const QUALIFICATION_FIELD_ROW_H: f32 = 54.0;
const QUALIFICATION_MULTILINE_ROW_H: f32 = 118.0;
const QUALIFICATION_STATUS_H: f32 = 86.0;
const QUALIFICATION_ERROR_H: f32 = 50.0;
const QUALIFICATION_PLAN_STATE_ID: &str = "model-editor-qualification-plan-state";

const SPECIALIST_SUMMARY: &str = "Inspect inherited parameters, sections, geometry equations, statistical groups, temperature behavior, tests, documentation, and release promotion.";
const EVIDENCE_CONTRACT: &str = "Source definition, parameter schema, inheritance, units, bounds, equations, sections, statistics, compilation, tests, provenance, and approvals form one model revision.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SurfaceAction {
    SelectSection(ModelEditorSection),
    OpenParameterSchema,
    OpenNewSection,
    OpenCorrelationMatrix,
    OpenTemperaturePreview,
    OpenQualificationPlan,
    OpenPromotionReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SectionSpec {
    section: ModelEditorSection,
    title: &'static str,
    description: &'static str,
    action_label: Option<&'static str>,
}

const SECTION_SPECS: [SectionSpec; 6] = [
    SectionSpec {
        section: ModelEditorSection::Parameters,
        title: "Typed parameters, inheritance, units, and bounds",
        description: "Search and edit model parameters with source, equation, unit, and consumer visibility.",
        action_label: Some("Parameter schema…"),
    },
    SectionSpec {
        section: ModelEditorSection::Sections,
        title: "Process sections and inheritance graph",
        description: "Show exact override deltas instead of copying complete parameter blocks.",
        action_label: Some("New section…"),
    },
    SectionSpec {
        section: ModelEditorSection::Statistics,
        title: "Process, mismatch, hierarchy, and correlation groups",
        description: "Statistical ownership and correlation are explicit and validated against device geometry.",
        action_label: Some("Correlation matrix…"),
    },
    SectionSpec {
        section: ModelEditorSection::Temperature,
        title: "Temperature equations, interpolation, and valid range",
        description: "Inspect parameter temperature dependence and out-of-range behavior.",
        action_label: Some("Plot temperature law…"),
    },
    SectionSpec {
        section: ModelEditorSection::Tests,
        title: "Golden vectors, platform parity, and regression",
        description: "Candidate models cannot be promoted without reference and runtime-parity evidence.",
        action_label: Some("Qualification plan…"),
    },
    SectionSpec {
        section: ModelEditorSection::Release,
        title: "Documentation, review, compatibility, and promotion",
        description: "Promote a content-addressed model revision with explicit consumer impact.",
        action_label: None,
    },
];

#[derive(Debug, Clone)]
struct PersistedModelRecords {
    metadata: Option<ModelDefinitionMetadata>,
    qualification: Option<ModelQualificationState>,
    metadata_error: Option<String>,
    qualification_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetricProjection {
    label: &'static str,
    value: String,
    detail: String,
    tone: MetricTone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetricTone {
    Neutral,
    Good,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QualificationRow {
    suite: String,
    vectors: String,
    desktop: String,
    wasm: String,
    reference_error: String,
    status: String,
    tone: MetricTone,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum QualificationDeleteTarget {
    #[default]
    None,
    Vector,
    Suite,
}

/// Dialog-local qualification selection and transaction buffers. These values
/// are presentation state only: persisted suite, vector, evidence, and
/// disposition records remain owned by the qualification domain.
#[derive(Debug, Clone)]
struct QualificationPlanUiState {
    selected_suite_id: String,
    selected_vector_id: String,
    edit_open: bool,
    edit_name: String,
    delete_target: QualificationDeleteTarget,
    disposition_open: bool,
    disposition_id: String,
    disposition_reason: String,
    disposition_cause: QualificationVectorDispositionCause,
    disposition_action: QualificationVectorRequiredAction,
    action_error: Option<String>,
}

impl Default for QualificationPlanUiState {
    fn default() -> Self {
        Self {
            selected_suite_id: String::new(),
            selected_vector_id: String::new(),
            edit_open: false,
            edit_name: String::new(),
            delete_target: QualificationDeleteTarget::None,
            disposition_open: false,
            disposition_id: String::new(),
            disposition_reason: String::new(),
            disposition_cause: QualificationVectorDispositionCause::Failed,
            disposition_action: QualificationVectorRequiredAction::Rerun,
            action_error: None,
        }
    }
}

#[derive(Debug, Clone)]
struct DisplayCell {
    text: String,
    mono: bool,
    tone: MetricTone,
}

impl DisplayCell {
    fn plain(value: impl Into<String>) -> Self {
        Self {
            text: value.into(),
            mono: false,
            tone: MetricTone::Neutral,
        }
    }

    fn mono(value: impl Into<String>) -> Self {
        Self {
            text: value.into(),
            mono: true,
            tone: MetricTone::Neutral,
        }
    }

    fn toned(value: impl Into<String>, tone: MetricTone) -> Self {
        Self {
            text: value.into(),
            mono: false,
            tone,
        }
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    crate::workbench::documents::model_editor::advance_qualification_execution(app);
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        ui.ctx().request_repaint();
    }
    let t = Tokens::get(ui.ctx());
    let size = ui.available_size().max(Vec2::splat(1.0));
    let (viewport, _) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(viewport, 0.0, t.color.bg_app);
    let mut surface = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(viewport)
            .layout(Layout::top_down(Align::Min)),
    );
    surface.spacing_mut().item_spacing = Vec2::ZERO;

    let records = persisted_records(app);
    let current_section = app.state.workbench.model_editor.active_section;
    let mut requested_action = None;

    outer_identity_header(&mut surface, app);
    specialist_header(&mut surface, app, &records);

    if app.state.workbench.model_editor.draft.is_none() {
        design_system::empty_state(
            &mut surface,
            WorkbenchIcon::Sliders,
            "No project-owned model candidate is open",
            "Open an editable project model from Model & PDK catalog. Built-in and external model sources remain read-only.",
        );
    } else {
        let metrics = project_metrics(app, &records);
        metric_strip(&mut surface, &metrics);
        workspace_body(
            &mut surface,
            app,
            &records,
            current_section,
            &mut requested_action,
        );
    }

    if let Some(action) = requested_action {
        apply_action(app, action);
    }
    if active_model_editor_workflow(app).is_some() {
        show_model_editor_workflow(ui.ctx(), app, &records);
    } else {
        show_inspection_dialogs(ui.ctx(), app, &records);
    }
}

fn outer_identity_header(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), OUTER_IDENTITY_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    bottom_rule(ui, rect, t.color.border);
    let action_w = 178.0_f32.min((rect.width() * 0.34).max(1.0));
    let copy = Rect::from_min_max(
        Pos2::new(rect.left() + 14.0, rect.top()),
        Pos2::new(
            (rect.right() - action_w - 24.0).max(rect.left() + 80.0),
            rect.bottom(),
        ),
    );
    paint_elided(
        ui,
        copy,
        "MODELS · TYPED PARAMETERS · VERSIONED DEFINITIONS",
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        9.0,
    );
    paint_elided(
        ui,
        copy,
        "Device model and parameter editor",
        theme::sans(14.0, FontWeight::SemiBold),
        t.color.text,
        29.0,
    );

    let button_rect = Rect::from_center_size(
        Pos2::new(rect.right() - action_w * 0.5 - 12.0, rect.center().y),
        Vec2::new(action_w, t.metrics.ctl_h),
    );
    let mut button_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let enabled = app.state.workbench.model_editor.draft.is_some();
    let response = Button::new("Validate model candidate")
        .accent()
        .enabled(enabled)
        .min_width(action_w)
        .max_width(action_w)
        .accessible_label(if enabled {
            "Validate model candidate"
        } else {
            "Validate model candidate. Unavailable: no project-owned model candidate is open"
        })
        .show(&mut button_ui)
        .on_disabled_hover_text("No project-owned model candidate is open.");
    if response.clicked() {
        request_model_editor_workflow(app, ModelEditorWorkflow::ValidateCandidate);
    }
}

fn persisted_records(app: &RSpiceApp) -> PersistedModelRecords {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return PersistedModelRecords {
            metadata: None,
            qualification: None,
            metadata_error: None,
            qualification_error: None,
        };
    };
    let metadata = Some(draft.metadata.clone());
    let qualification = Some(draft.qualification.clone());
    let metadata_error = draft.definition().err().map(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    });
    let qualification_error = draft
        .qualification
        .validate_for_model(&draft.name)
        .err()
        .map(|error| error.to_string());
    PersistedModelRecords {
        metadata,
        qualification,
        metadata_error,
        qualification_error,
    }
}

fn action_button(
    ui: &mut Ui,
    label: &str,
    width: f32,
    enabled: bool,
    blocker: Option<&str>,
    value: SurfaceAction,
    action: &mut Option<SurfaceAction>,
) {
    let accessible = if enabled {
        label.to_owned()
    } else {
        format!("{label}. Unavailable: {}", blocker.unwrap_or("unavailable"))
    };
    let response = Button::new(label)
        .enabled(enabled)
        .min_width(width)
        .max_width(width)
        .accessible_label(&accessible)
        .show(ui);
    let response = if !enabled && let Some(blocker) = blocker {
        response.on_hover_text(blocker)
    } else {
        response
    };
    if enabled && response.clicked() {
        *action = Some(value);
    }
}

fn specialist_header(ui: &mut Ui, app: &RSpiceApp, records: &PersistedModelRecords) {
    let t = Tokens::get(ui.ctx());
    let available_width = ui.available_width().max(1.0);
    let medium_layout = available_width > 820.0 && available_width <= 1120.0;
    let header_h = specialist_header_height(available_width);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(available_width, header_h), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    bottom_rule(ui, rect, t.color.border);

    let mark = Rect::from_min_size(
        Pos2::new(rect.left() + 14.0, rect.top() + 14.0),
        Vec2::splat(48.0),
    );
    ui.painter().rect(
        mark,
        t.radius,
        t.color.bg_panel_2,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        mark.center(),
        Align2::CENTER_CENTER,
        "M",
        theme::mono(tokens::FS_4, FontWeight::SemiBold),
        t.color.accent,
    );

    let left = mark.right() + 12.0;
    let status_w = 174.0_f32.min(rect.width() * 0.26);
    let purpose_w = if rect.width() > 1120.0 {
        (rect.width() * 0.21).clamp(190.0, 280.0)
    } else {
        0.0
    };
    let status_left = rect.right() - status_w - 10.0;
    let purpose_right = status_left - 12.0;
    let purpose_left = purpose_right - purpose_w;
    let text_right = if purpose_w > 0.0 {
        purpose_left - 12.0
    } else {
        status_left - 8.0
    }
    .max(left + 80.0);
    let text_clip = Rect::from_min_max(
        Pos2::new(left, rect.top()),
        Pos2::new(text_right, rect.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        Pos2::new(left, rect.top() + 9.0),
        Align2::LEFT_TOP,
        "COMPACT MODEL · MACROMODEL · BEHAVIORAL",
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
    let title = app.state.workbench.model_editor.draft.as_ref().map_or_else(
        || "No model candidate".to_owned(),
        |draft| format!("{} model candidate", draft.name),
    );
    ui.painter().with_clip_rect(text_clip).text(
        Pos2::new(left, rect.top() + 28.0),
        Align2::LEFT_TOP,
        title,
        theme::sans(14.0, FontWeight::SemiBold),
        t.color.text,
    );
    let summary_bottom = if medium_layout {
        rect.top() + 72.0
    } else {
        (rect.top() + 78.0).min(rect.bottom())
    };
    let summary_rect = Rect::from_min_max(
        Pos2::new(left, rect.top() + 50.0),
        Pos2::new(text_right, summary_bottom),
    );
    let summary = ui.painter().layout(
        SPECIALIST_SUMMARY.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        summary_rect.width().max(1.0),
    );
    ui.painter()
        .with_clip_rect(summary_rect)
        .galley(summary_rect.min, summary, t.color.text_dim);

    if purpose_w > 0.0 {
        let purpose_rect = Rect::from_min_max(
            Pos2::new(purpose_left, rect.top() + 10.0),
            Pos2::new(purpose_right, rect.bottom() - 10.0),
        );
        ui.painter().vline(
            purpose_rect.left(),
            purpose_rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
        let purpose_text = purpose_rect.shrink2(Vec2::new(12.0, 0.0));
        paint_elided(
            ui,
            purpose_text,
            "PURPOSE",
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_faint,
            0.0,
        );
        paint_elided(
            ui,
            purpose_text,
            "Engineering document",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
            15.0,
        );
        let boundary_rect = Rect::from_min_max(
            Pos2::new(purpose_text.left(), purpose_text.top() + 30.0),
            purpose_text.max,
        );
        let boundary = ui.painter().layout(
            "Owns the active definition and exposes downstream effects without duplicating source evidence."
                .to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
            boundary_rect.width().max(1.0),
        );
        ui.painter().with_clip_rect(boundary_rect).galley(
            boundary_rect.min,
            boundary,
            t.color.text_faint,
        );
    }

    if medium_layout {
        let purpose_rect = Rect::from_min_max(
            Pos2::new(left, rect.top() + 82.0),
            Pos2::new(status_left - 8.0, rect.bottom() - 7.0),
        );
        paint_elided(
            ui,
            purpose_rect,
            "PURPOSE",
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_faint,
            4.0,
        );
        paint_elided(
            ui,
            purpose_rect,
            "Engineering document",
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
            17.0,
        );
        paint_elided(
            ui,
            purpose_rect,
            "Owns the active definition and exposes downstream effects without duplicating source evidence.",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            30.0,
        );
    }

    let (status, tone) = candidate_status(app, records);
    let status_rect = Rect::from_min_max(
        Pos2::new(status_left, rect.top()),
        Pos2::new(rect.right() - 14.0, rect.bottom()),
    );
    let status_color = tone_color(&t, tone);
    let status_y = rect.top() + 16.0;
    ui.painter().circle_filled(
        Pos2::new(status_rect.left() + 4.0, status_y),
        2.5,
        status_color,
    );
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let label =
        design_system::elide_text(ui, &status, &font, (status_rect.width() - 14.0).max(1.0));
    ui.painter().text(
        Pos2::new(status_rect.left() + 13.0, status_y),
        Align2::LEFT_CENTER,
        label,
        font,
        status_color,
    );
}

fn specialist_header_height(width: f32) -> f32 {
    if width > 820.0 && width <= 1120.0 {
        SPECIALIST_HEADER_MEDIUM_H
    } else {
        SPECIALIST_HEADER_H
    }
}

fn candidate_status(app: &RSpiceApp, records: &PersistedModelRecords) -> (String, MetricTone) {
    if let Some(error) = records.metadata_error.as_ref() {
        return (format!("metadata invalid · {error}"), MetricTone::Error);
    }
    if let Some(error) = records.qualification_error.as_ref() {
        return (
            format!("qualification invalid · {error}"),
            MetricTone::Error,
        );
    }
    let editor = &app.state.workbench.model_editor;
    if let Some(diagnostic) = editor.diagnostics.first() {
        return (diagnostic.message.clone(), MetricTone::Error);
    }
    if editor.validation.is_some() {
        return ("candidate validated".to_owned(), MetricTone::Good);
    }
    ("validation required".to_owned(), MetricTone::Warning)
}

fn project_metrics(app: &RSpiceApp, records: &PersistedModelRecords) -> Vec<MetricProjection> {
    let draft = app.state.workbench.model_editor.draft.as_ref();
    let parameter_count = records.metadata.as_ref().map_or_else(
        || draft.map_or(0, |draft| draft.parameters.len()),
        |metadata| metadata.parameters.len(),
    );
    let (declared, inherited, model_overrides) =
        records.metadata.as_ref().map_or((0, 0, 0), |metadata| {
            metadata
                .parameters
                .iter()
                .fold((0, 0, 0), |mut counts, parameter| {
                    match parameter.source {
                        ParameterSource::Declared { .. } => counts.0 += 1,
                        ParameterSource::Inherited { .. } => counts.1 += 1,
                        ParameterSource::Overridden { .. } => counts.2 += 1,
                    }
                    counts
                })
        });
    let section_overrides = records.metadata.as_ref().map_or(0, |metadata| {
        metadata
            .sections
            .iter()
            .map(|section| section.overrides.len())
            .sum::<usize>()
    });
    let overridden = model_overrides + section_overrides;
    let section_count = records
        .metadata
        .as_ref()
        .map_or(0, |value| value.sections.len());
    let section_detail = records.metadata.as_ref().map_or_else(
        || "no persisted section metadata".to_owned(),
        |metadata| {
            if metadata.sections.is_empty() {
                "no named sections".to_owned()
            } else {
                metadata
                    .sections
                    .iter()
                    .map(|section| section.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" · ")
            }
        },
    );
    let qualification = qualification_totals(app, records);
    vec![
        MetricProjection {
            label: "Parameters",
            value: parameter_count.to_string(),
            detail: if records.metadata.is_some() {
                format!("{declared} declared · {inherited} inherited · {overridden} override")
            } else {
                "source values · schema unavailable".to_owned()
            },
            tone: if records.metadata_error.is_some() {
                MetricTone::Error
            } else {
                MetricTone::Neutral
            },
        },
        MetricProjection {
            label: "Sections",
            value: section_count.to_string(),
            detail: section_detail,
            tone: if records.metadata_error.is_some() {
                MetricTone::Error
            } else {
                MetricTone::Neutral
            },
        },
        MetricProjection {
            label: "Tests",
            value: format!("{} / {}", qualification.desktop_passed, qualification.total),
            detail: qualification.desktop_detail,
            tone: qualification.desktop_tone,
        },
        MetricProjection {
            label: "Parity",
            value: format!("{} / {}", qualification.wasm_passed, qualification.total),
            detail: qualification.wasm_detail,
            tone: qualification.wasm_tone,
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct QualificationTotals {
    total: usize,
    desktop_passed: usize,
    wasm_passed: usize,
    desktop_detail: String,
    wasm_detail: String,
    desktop_tone: MetricTone,
    wasm_tone: MetricTone,
}

fn qualification_totals(app: &RSpiceApp, records: &PersistedModelRecords) -> QualificationTotals {
    let Some(qualification) = records.qualification.as_ref() else {
        return QualificationTotals {
            total: 0,
            desktop_passed: 0,
            wasm_passed: 0,
            desktop_detail: "no bound qualification evidence".to_owned(),
            wasm_detail: "no bound qualification evidence".to_owned(),
            desktop_tone: MetricTone::Warning,
            wasm_tone: MetricTone::Warning,
        };
    };
    if records.qualification_error.is_some() {
        return QualificationTotals {
            total: qualification
                .suites
                .iter()
                .map(|suite| suite.vectors.len())
                .sum(),
            desktop_passed: 0,
            wasm_passed: 0,
            desktop_detail: "persisted qualification state is invalid".to_owned(),
            wasm_detail: "persisted qualification state is invalid".to_owned(),
            desktop_tone: MetricTone::Error,
            wasm_tone: MetricTone::Error,
        };
    }
    let source = app.state.workbench.model_editor.draft.as_ref();
    let mut total = 0;
    let mut desktop_passed = 0;
    let mut wasm_passed = 0;
    for suite in &qualification.suites {
        total += suite.vectors.len();
        if let Some(evidence) = bound_suite_evidence(qualification, suite, source) {
            desktop_passed += evidence
                .vector_outcomes
                .iter()
                .filter(|outcome| platform_outcome_passed(outcome, QualificationPlatform::Desktop))
                .count();
            wasm_passed += evidence
                .vector_outcomes
                .iter()
                .filter(|outcome| {
                    platform_outcome_passed(outcome, QualificationPlatform::WebAssembly)
                })
                .count();
        }
    }
    QualificationTotals {
        total,
        desktop_passed,
        wasm_passed,
        desktop_detail: if total == 0 {
            "no qualification vectors".to_owned()
        } else if desktop_passed == total {
            "desktop source-bound evidence".to_owned()
        } else {
            format!("{} unresolved or failed", total - desktop_passed)
        },
        wasm_detail: if total == 0 {
            "no qualification vectors".to_owned()
        } else if wasm_passed == total {
            "WebAssembly source-bound evidence".to_owned()
        } else {
            format!("{} unresolved or failed", total - wasm_passed)
        },
        desktop_tone: pass_tone(desktop_passed, total),
        wasm_tone: pass_tone(wasm_passed, total),
    }
}

fn pass_tone(passed: usize, total: usize) -> MetricTone {
    if total == 0 {
        MetricTone::Warning
    } else if passed == total {
        MetricTone::Good
    } else {
        MetricTone::Warning
    }
}

fn metric_strip(ui: &mut Ui, metrics: &[MetricProjection]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), METRICS_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    bottom_rule(ui, rect, t.color.border);
    let column_w = rect.width() / metrics.len().max(1) as f32;
    for (index, metric) in metrics.iter().enumerate() {
        let cell = Rect::from_min_max(
            Pos2::new(rect.left() + index as f32 * column_w, rect.top()),
            Pos2::new(rect.left() + (index + 1) as f32 * column_w, rect.bottom()),
        );
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        let clip = cell.shrink2(Vec2::new(12.0, 0.0));
        paint_elided(
            ui,
            clip,
            metric.label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            8.0,
        );
        paint_elided(
            ui,
            clip,
            &metric.value,
            theme::mono(tokens::FS_3, FontWeight::SemiBold),
            tone_color(&t, metric.tone),
            25.0,
        );
        paint_elided(
            ui,
            clip,
            &metric.detail,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
            47.0,
        );
    }
}

fn workspace_body(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
    current_section: ModelEditorSection,
    action: &mut Option<SurfaceAction>,
) {
    let t = Tokens::get(ui.ctx());
    let size = ui.available_size().max(Vec2::splat(1.0));
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    let nav_w = NAV_W.min((rect.width() * 0.31).max(150.0));
    let nav_rect = Rect::from_min_max(rect.min, Pos2::new(rect.left() + nav_w, rect.bottom()));
    let content_rect = Rect::from_min_max(Pos2::new(nav_rect.right(), rect.top()), rect.max);
    ui.painter().rect_filled(nav_rect, 0.0, t.color.bg_panel);
    ui.painter().rect_filled(content_rect, 0.0, t.color.bg_app);
    ui.painter().vline(
        nav_rect.right(),
        nav_rect.y_range(),
        Stroke::new(1.0, t.color.border),
    );

    let mut nav = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(nav_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    nav.spacing_mut().item_spacing = Vec2::ZERO;
    render_navigation(&mut nav, current_section, action);

    let mut content = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    content.spacing_mut().item_spacing = Vec2::ZERO;
    render_section(&mut content, app, records, current_section, action);
}

fn render_navigation(ui: &mut Ui, current: ModelEditorSection, action: &mut Option<SurfaceAction>) {
    let viewport_h = ui.available_height().max(1.0);
    ScrollArea::vertical()
        .id_salt("model-editor-section-navigation")
        .auto_shrink([false, false])
        .max_height(viewport_h)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            let t = Tokens::get(ui.ctx());
            for (index, spec) in SECTION_SPECS.iter().enumerate() {
                let (rect, response) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width().max(1.0), NAV_ROW_H),
                    Sense::click(),
                );
                let active = current == spec.section;
                let fill = if active {
                    t.color.bg_active
                } else if response.hovered() {
                    t.color.bg_hover
                } else {
                    Color32::TRANSPARENT
                };
                ui.painter().rect_filled(rect, 0.0, fill);
                bottom_rule(ui, rect, t.color.border);
                if active {
                    ui.painter().rect_filled(
                        Rect::from_min_max(rect.min, Pos2::new(rect.left() + 2.0, rect.bottom())),
                        0.0,
                        t.color.accent,
                    );
                }
                ui.painter().text(
                    Pos2::new(rect.left() + 11.0, rect.center().y),
                    Align2::LEFT_CENTER,
                    format!("{:02}", index + 1),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    if active {
                        t.color.accent
                    } else {
                        t.color.text_faint
                    },
                );
                ui.painter().text(
                    Pos2::new(rect.left() + 44.0, rect.center().y),
                    Align2::LEFT_CENTER,
                    spec.section.label(),
                    theme::sans(
                        tokens::FS_0,
                        if active {
                            FontWeight::SemiBold
                        } else {
                            FontWeight::Regular
                        },
                    ),
                    if active {
                        t.color.text
                    } else {
                        t.color.text_dim
                    },
                );
                theme::paint_focus_ring(ui, &response, rect);
                response.widget_info(|| {
                    egui::WidgetInfo::selected(
                        egui::WidgetType::Button,
                        true,
                        active,
                        spec.section.label(),
                    )
                });
                if response.clicked() && !active {
                    *action = Some(SurfaceAction::SelectSection(spec.section));
                }
            }

            const NOTE_H: f32 = 119.0;
            let rows_h = SECTION_SPECS.len() as f32 * NAV_ROW_H;
            ui.add_space((viewport_h - rows_h - NOTE_H).max(0.0));
            navigation_evidence_note(ui, NOTE_H);
        });
}

fn navigation_evidence_note(ui: &mut Ui, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (note_rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), height),
        Sense::hover(),
    );
    ui.painter().hline(
        note_rect.x_range(),
        note_rect.top(),
        Stroke::new(1.0, t.color.border),
    );
    let clip = note_rect.shrink2(Vec2::new(11.0, 8.0));
    paint_elided(
        ui,
        clip,
        "EVIDENCE CONTRACT",
        theme::mono(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_faint,
        0.0,
    );
    let galley = ui.painter().layout(
        EVIDENCE_CONTRACT.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        clip.width(),
    );
    ui.painter().with_clip_rect(clip).galley(
        Pos2::new(clip.left(), clip.top() + 22.0),
        galley,
        t.color.text_dim,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(format!("Evidence contract. {EVIDENCE_CONTRACT}"));
    });
}

fn render_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
    section: ModelEditorSection,
    action: &mut Option<SurfaceAction>,
) {
    let spec = section_spec(section);
    section_header(ui, app, spec, records, action);
    let height = ui.available_height().max(1.0);
    ScrollArea::vertical()
        .id_salt(("model-editor-section", section.label()))
        .auto_shrink([false, false])
        .max_height(height)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            match section {
                ModelEditorSection::Parameters => parameters_table(ui, app, records),
                ModelEditorSection::Sections => sections_table(ui, records),
                ModelEditorSection::Statistics => statistics_table(ui, records),
                ModelEditorSection::Temperature => temperature_table(ui, records),
                ModelEditorSection::Tests => qualification_table(ui, app, records),
                ModelEditorSection::Release => release_panel(ui, app, records, action),
            }
        });
}

fn section_spec(section: ModelEditorSection) -> &'static SectionSpec {
    SECTION_SPECS
        .iter()
        .find(|spec| spec.section == section)
        .expect("all model editor sections have a fixed surface contract")
}

fn section_header(
    ui: &mut Ui,
    app: &RSpiceApp,
    spec: &SectionSpec,
    records: &PersistedModelRecords,
    action: &mut Option<SurfaceAction>,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), SECTION_HEADER_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    bottom_rule(ui, rect, t.color.border);

    let action_w = if spec.action_label.is_some() {
        154.0
    } else {
        0.0
    };
    let copy_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 12.0, rect.top()),
        Pos2::new(
            (rect.right() - action_w - 16.0).max(rect.left() + 80.0),
            rect.bottom(),
        ),
    );
    paint_elided(
        ui,
        copy_rect,
        &format!("MODEL · {}", spec.section.label().to_uppercase()),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        9.0,
    );
    paint_elided(
        ui,
        copy_rect,
        spec.title,
        theme::sans(13.0, FontWeight::SemiBold),
        t.color.text,
        29.0,
    );
    paint_elided(
        ui,
        copy_rect,
        spec.description,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        51.0,
    );

    let Some(label) = spec.action_label else {
        return;
    };
    let (mut enabled, mut blocker, value) = section_action_state(spec.section, records);
    if app.state.workbench.safe_mode.project_read_only()
        && matches!(
            spec.section,
            ModelEditorSection::Parameters
                | ModelEditorSection::Sections
                | ModelEditorSection::Statistics
        )
    {
        enabled = false;
        blocker = Some("The project is open read-only.");
    }
    let button_rect = Rect::from_center_size(
        Pos2::new(rect.right() - 82.0, rect.center().y),
        Vec2::new(144.0, Tokens::get(ui.ctx()).metrics.ctl_h),
    );
    let mut button_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    action_button(
        &mut button_ui,
        label,
        144.0,
        enabled,
        blocker,
        value,
        action,
    );
}

fn section_action_state(
    section: ModelEditorSection,
    records: &PersistedModelRecords,
) -> (bool, Option<&'static str>, SurfaceAction) {
    match section {
        ModelEditorSection::Parameters => (
            records.metadata.is_some(),
            Some("An open model candidate is required."),
            SurfaceAction::OpenParameterSchema,
        ),
        ModelEditorSection::Sections => (
            records.metadata.is_some(),
            Some("An open model candidate is required before adding a section."),
            SurfaceAction::OpenNewSection,
        ),
        ModelEditorSection::Statistics => (
            records.metadata.as_ref().is_some_and(|metadata| {
                !metadata.statistics.correlation_matrices.is_empty()
                    || metadata
                        .statistics
                        .variables
                        .iter()
                        .any(|variable| variable.correlation_group.is_some())
            }),
            Some("Declare at least one correlated statistical variable first."),
            SurfaceAction::OpenCorrelationMatrix,
        ),
        ModelEditorSection::Temperature => (
            records.metadata.as_ref().is_some_and(|metadata| {
                !metadata.temperature_laws.is_empty() && records.metadata_error.is_none()
            }),
            Some("A valid temperature law is required before plotting."),
            SurfaceAction::OpenTemperaturePreview,
        ),
        ModelEditorSection::Tests => (
            records.qualification.is_some(),
            Some("An open model candidate is required."),
            SurfaceAction::OpenQualificationPlan,
        ),
        ModelEditorSection::Release => (
            false,
            Some("Release review is owned by the Release panel."),
            SurfaceAction::OpenPromotionReview,
        ),
    }
}

fn parameters_table(ui: &mut Ui, app: &mut RSpiceApp, records: &PersistedModelRecords) {
    const COLUMNS: [(&str, f32); 6] = [
        ("Parameter", 0.16),
        ("Value", 0.19),
        ("Unit", 0.10),
        ("Source", 0.20),
        ("Bounds", 0.17),
        ("Status", 0.18),
    ];
    let metadata = records.metadata.as_ref().map(parameter_metadata_index);
    let diagnostics = app.state.workbench.model_editor.diagnostics.clone();
    let changed = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .and_then(|draft| draft.delta().ok())
        .map(|delta| {
            delta
                .added_parameters
                .into_iter()
                .chain(delta.changed_parameters)
                .map(|value| value.to_lowercase())
                .collect::<BTreeSet<_>>()
        })
        .unwrap_or_default();
    let selected = app.state.workbench.model_editor.selected_parameter;
    let read_only = app.state.workbench.safe_mode.project_read_only();
    let mut changed_value = false;
    let mut selected_parameter = selected;

    ScrollArea::horizontal()
        .id_salt("model-editor-parameters-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, &COLUMNS);
            let Some(draft) = app.state.workbench.model_editor.draft.as_mut() else {
                table_empty(ui, width, "No open model parameter source.");
                return;
            };
            if draft.parameters.is_empty() {
                table_empty(ui, width, "The open model source declares no parameters.");
                return;
            }
            for (index, parameter) in draft.parameters.iter_mut().enumerate() {
                let meta = metadata
                    .as_ref()
                    .and_then(|values| values.get(&parameter.name.to_lowercase()).copied());
                let diagnostic = diagnostics
                    .iter()
                    .find(|value| value.field.starts_with(&format!("parameters[{index}]")));
                let tone = if diagnostic.is_some() {
                    MetricTone::Error
                } else if changed.contains(&parameter.name.to_lowercase()) {
                    MetricTone::Warning
                } else if meta.is_some() {
                    MetricTone::Good
                } else {
                    MetricTone::Warning
                };
                let status = diagnostic.map_or_else(
                    || match tone {
                        MetricTone::Warning if meta.is_none() => "schema unavailable".to_owned(),
                        MetricTone::Warning => "changed".to_owned(),
                        MetricTone::Good => "declared".to_owned(),
                        _ => "review".to_owned(),
                    },
                    |value| value.message.clone(),
                );
                let unit = if parameter.unit.is_empty() {
                    "not declared"
                } else {
                    parameter.unit.as_str()
                };
                let source = meta.map_or_else(
                    || "project source".to_owned(),
                    |value| parameter_source_label(&value.source),
                );
                let bounds = match (
                    parameter.lower_bound.as_str(),
                    parameter.upper_bound.as_str(),
                ) {
                    ("", "") => "not declared".to_owned(),
                    (lower, "") => format!("≥ {lower}"),
                    ("", upper) => format!("≤ {upper}"),
                    (lower, upper) => format!("{lower}…{upper}"),
                };
                let row = interactive_table_row(ui, width, &COLUMNS, selected == Some(index));
                paint_cell(ui, row[0], &DisplayCell::mono(&parameter.name));
                let edit_rect = row[1].shrink2(Vec2::new(5.0, 4.0));
                let mut edit_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(edit_rect)
                        .layout(Layout::left_to_right(Align::Center)),
                );
                let inherited = meta.is_some_and(|parameter| {
                    matches!(&parameter.source, ParameterSource::Inherited { .. })
                });
                let response = edit_ui
                    .add_enabled_ui(!read_only && !inherited, |ui| {
                        ui.add_sized(
                            edit_rect.size(),
                            egui::TextEdit::singleline(&mut parameter.value)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        )
                    })
                    .inner;
                let response = if inherited {
                    response.on_hover_text(
                        "Open Parameter schema… and add an explicit typed value for the target process section before editing an inherited value.",
                    )
                } else if read_only {
                    response.on_hover_text("The project is open read-only.")
                } else {
                    response
                };
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_label(format!("Value for {}", parameter.name));
                });
                if response.changed() {
                    changed_value = true;
                }
                if response.clicked() || response.has_focus() {
                    selected_parameter = Some(index);
                }
                paint_cell(ui, row[2], &DisplayCell::plain(unit));
                paint_cell(ui, row[3], &DisplayCell::plain(source));
                paint_cell(ui, row[4], &DisplayCell::mono(bounds));
                paint_cell(ui, row[5], &DisplayCell::toned(status, tone));
            }
        });
    app.state.workbench.model_editor.selected_parameter = selected_parameter;
    if changed_value {
        app.state
            .workbench
            .model_editor
            .invalidate_candidate_evidence();
    }
}

fn parameter_metadata_index(
    metadata: &ModelDefinitionMetadata,
) -> BTreeMap<String, &ParameterDefinition> {
    metadata
        .parameters
        .iter()
        .map(|parameter| (parameter.name.to_lowercase(), parameter))
        .collect()
}

fn parameter_source_label(source: &ParameterSource) -> String {
    match source {
        ParameterSource::Declared { source } => source.clone(),
        ParameterSource::Inherited { from_section } => format!("inherited · {from_section}"),
        ParameterSource::Overridden { section } => format!("override · {section}"),
    }
}

fn sections_table(ui: &mut Ui, records: &PersistedModelRecords) {
    const COLUMNS: [(&str, f32); 5] = [
        ("Section", 0.16),
        ("Parent", 0.16),
        ("Overrides", 0.14),
        ("Model files", 0.30),
        ("Qualification", 0.24),
    ];
    let rows = records.metadata.as_ref().map_or_else(Vec::new, |metadata| {
        metadata
            .sections
            .iter()
            .map(|section| {
                let (qualification, tone) = section_qualification(&section.qualification);
                vec![
                    DisplayCell::mono(&section.name),
                    DisplayCell::plain(section.parent.as_deref().unwrap_or("base")),
                    DisplayCell::mono(section.overrides.len().to_string()),
                    DisplayCell::plain(model_files_label(&section.model_files)),
                    DisplayCell::toned(qualification, tone),
                ]
            })
            .collect()
    });
    render_table(
        ui,
        "model-editor-sections-table",
        &COLUMNS,
        &rows,
        "No persisted process-section metadata is available for this model.",
    );
}

fn section_qualification(value: &ModelSectionQualification) -> (String, MetricTone) {
    match value {
        ModelSectionQualification::Unqualified => ("unqualified".to_owned(), MetricTone::Warning),
        ModelSectionQualification::Pending => ("pending".to_owned(), MetricTone::Warning),
        ModelSectionQualification::Qualified { evidence_digest } => {
            evidence_digest.as_ref().map_or(
                (
                    "invalid · qualification evidence not retained".to_owned(),
                    MetricTone::Error,
                ),
                |digest| {
                    (
                        format!("qualified · {}", short_identity(digest)),
                        MetricTone::Good,
                    )
                },
            )
        }
        ModelSectionQualification::Failed { summary } => {
            (format!("failed · {summary}"), MetricTone::Error)
        }
    }
}

fn model_files_label(files: &[crate::state::model_library::ModelFileIdentity]) -> String {
    match files {
        [] => "not declared".to_owned(),
        [file] => format!("{} · r{}", file.display_name, file.revision),
        files => format!("{} retained files", files.len()),
    }
}

fn statistics_table(ui: &mut Ui, records: &PersistedModelRecords) {
    const COLUMNS: [(&str, f32); 5] = [
        ("Variable", 0.18),
        ("Distribution", 0.25),
        ("σ / bounds", 0.18),
        ("Correlation group", 0.22),
        ("Hierarchy", 0.17),
    ];
    let rows = records.metadata.as_ref().map_or_else(Vec::new, |metadata| {
        metadata
            .statistics
            .variables
            .iter()
            .map(|variable| {
                let (distribution, extent) = distribution_labels(&variable.distribution);
                vec![
                    DisplayCell::mono(&variable.name),
                    DisplayCell::plain(distribution),
                    DisplayCell::mono(extent),
                    DisplayCell::plain(
                        variable
                            .correlation_group
                            .as_deref()
                            .unwrap_or("independent"),
                    ),
                    DisplayCell::plain(hierarchy_label(&variable.hierarchy)),
                ]
            })
            .collect()
    });
    render_table(
        ui,
        "model-editor-statistics-table",
        &COLUMNS,
        &rows,
        "No persisted statistical variables are declared for this model.",
    );
}

fn distribution_labels(distribution: &StatisticalDistribution) -> (String, String) {
    match distribution {
        StatisticalDistribution::Normal { sigma } => {
            ("normal".to_owned(), format!("σ {}", sigma.get()))
        }
        StatisticalDistribution::Lognormal { sigma_fraction } => (
            "lognormal".to_owned(),
            format!("σ fraction {}", sigma_fraction.get()),
        ),
        StatisticalDistribution::Uniform { lower, upper } => (
            "uniform".to_owned(),
            format!("{}…{}", lower.get(), upper.get()),
        ),
        StatisticalDistribution::Discrete { outcomes } => (
            "discrete".to_owned(),
            format!("{} outcomes", outcomes.len()),
        ),
    }
}

fn hierarchy_label(value: &StatisticalHierarchyScope) -> &'static str {
    match value {
        StatisticalHierarchyScope::Global => "global",
        StatisticalHierarchyScope::Wafer => "wafer",
        StatisticalHierarchyScope::Die => "die",
        StatisticalHierarchyScope::Instance => "instance",
        StatisticalHierarchyScope::MatchedDeviceGroup => "matched-device group",
    }
}

fn temperature_table(ui: &mut Ui, records: &PersistedModelRecords) {
    const COLUMNS: [(&str, f32); 5] = [
        ("Quantity", 0.18),
        ("Equation / table", 0.30),
        ("Reference", 0.16),
        ("Range", 0.20),
        ("Extrapolation", 0.16),
    ];
    let rows = records.metadata.as_ref().map_or_else(Vec::new, |metadata| {
        metadata
            .temperature_laws
            .iter()
            .map(|law| {
                let representation = match &law.representation {
                    TemperatureLawRepresentation::Equation { expression } => expression.clone(),
                    TemperatureLawRepresentation::LookupTable {
                        interpolation,
                        points,
                    } => format!(
                        "{} · {} points",
                        interpolation_label(*interpolation),
                        points.len()
                    ),
                };
                vec![
                    DisplayCell::plain(&law.quantity),
                    DisplayCell::mono(representation),
                    DisplayCell::mono(format!("{} °C", law.reference_temperature_c.get())),
                    DisplayCell::mono(format!(
                        "{}…{} °C",
                        law.valid_range.minimum_c.get(),
                        law.valid_range.maximum_c.get()
                    )),
                    DisplayCell::plain(extrapolation_label(law.extrapolation)),
                ]
            })
            .collect()
    });
    render_table(
        ui,
        "model-editor-temperature-table",
        &COLUMNS,
        &rows,
        "No persisted temperature laws are declared for this model.",
    );
}

fn extrapolation_label(value: TemperatureExtrapolationPolicy) -> &'static str {
    match value {
        TemperatureExtrapolationPolicy::BlockOutsideRange => "block outside",
        TemperatureExtrapolationPolicy::Warn => "warning",
        TemperatureExtrapolationPolicy::Clamp => "clamp",
    }
}

fn interpolation_label(value: LookupInterpolation) -> &'static str {
    match value {
        LookupInterpolation::Linear => "linear",
        LookupInterpolation::LogLinear => "log-linear",
        LookupInterpolation::Step => "step",
    }
}

fn qualification_table(ui: &mut Ui, app: &RSpiceApp, records: &PersistedModelRecords) {
    const COLUMNS: [(&str, f32); 6] = [
        ("Suite", 0.18),
        ("Vectors", 0.12),
        ("Desktop", 0.17),
        ("WASM", 0.17),
        ("Reference error", 0.18),
        ("Status", 0.18),
    ];
    let rows = qualification_rows(app, records)
        .into_iter()
        .map(|row| {
            vec![
                DisplayCell::plain(row.suite),
                DisplayCell::mono(row.vectors),
                DisplayCell::plain(row.desktop),
                DisplayCell::plain(row.wasm),
                DisplayCell::mono(row.reference_error),
                DisplayCell::toned(row.status, row.tone),
            ]
        })
        .collect::<Vec<_>>();
    render_table(
        ui,
        "model-editor-qualification-table",
        &COLUMNS,
        &rows,
        "No persisted golden-vector suites or platform evidence are available for this model.",
    );
}

fn qualification_rows(app: &RSpiceApp, records: &PersistedModelRecords) -> Vec<QualificationRow> {
    let Some(state) = records.qualification.as_ref() else {
        return Vec::new();
    };
    let source = app.state.workbench.model_editor.draft.as_ref();
    state
        .suites
        .iter()
        .map(|suite| {
            let total = suite.vectors.len();
            let evidence = bound_suite_evidence(state, suite, source);
            let desktop = bound_platform_run(state, suite, source, QualificationPlatform::Desktop);
            let wasm = bound_platform_run(state, suite, source, QualificationPlatform::WebAssembly);
            let worst = evidence.and_then(worst_relative_error);
            let invalid_state = records.qualification_error.is_some();
            let (status, tone) = if invalid_state {
                ("invalid persisted state".to_owned(), MetricTone::Error)
            } else if evidence.is_none() {
                (
                    "awaiting cross-platform evidence".to_owned(),
                    MetricTone::Warning,
                )
            } else if evidence.is_some_and(|evidence| evidence.passed) {
                ("pass".to_owned(), MetricTone::Good)
            } else {
                let unresolved = evidence.map_or(total, |evidence| {
                    evidence
                        .vector_outcomes
                        .iter()
                        .filter(|outcome| !outcome.passed)
                        .count()
                });
                (
                    format!("{unresolved} unresolved or failed"),
                    MetricTone::Warning,
                )
            };
            QualificationRow {
                suite: suite.name.clone(),
                vectors: total.to_string(),
                desktop: platform_run_summary(desktop),
                wasm: platform_run_summary(wasm),
                reference_error: worst
                    .map(format_percentage)
                    .unwrap_or_else(|| "not available".to_owned()),
                status,
                tone,
            }
        })
        .collect()
}

fn bound_platform_run<'a>(
    state: &'a ModelQualificationState,
    suite: &QualificationSuite,
    draft: Option<&crate::workbench::documents::model_editor::ModelEditorDraft>,
    platform: QualificationPlatform,
) -> Option<&'a QualificationPlatformRun> {
    let draft = draft?;
    if draft.definition_is_dirty() {
        return None;
    }
    exact_platform_run(
        state,
        suite,
        &draft.model_name,
        draft.source_id,
        draft.base_source_digest,
        draft.base_source_revision,
        platform,
    )
}

fn exact_platform_run<'a>(
    state: &'a ModelQualificationState,
    suite: &QualificationSuite,
    model_id: &str,
    source_id: crate::product::ModelSourceId,
    source_digest: crate::product::ContentDigest,
    source_revision: crate::product::ObjectRevision,
    platform: QualificationPlatform,
) -> Option<&'a QualificationPlatformRun> {
    let mut matches = state.platform_runs.iter().filter(|run| {
        run.platform == platform
            && run.suite_id.eq_ignore_ascii_case(&suite.id)
            && run.suite_revision == suite.revision
            && run.source.model_id.eq_ignore_ascii_case(model_id)
            && run.source.source_id == Some(source_id)
            && run.source.source_digest == source_digest
            && run.source.source_revision == source_revision
    });
    let first = matches.next()?;
    matches.next().is_none().then_some(first)
}

fn platform_run_summary(run: Option<&QualificationPlatformRun>) -> String {
    run.map_or_else(
        || "no run".to_owned(),
        |run| {
            format!(
                "{} pass",
                run.vector_outcomes
                    .iter()
                    .filter(|outcome| outcome.outcome.passed)
                    .count()
            )
        },
    )
}

fn bound_suite_evidence<'a>(
    state: &'a ModelQualificationState,
    suite: &QualificationSuite,
    draft: Option<&crate::workbench::documents::model_editor::ModelEditorDraft>,
) -> Option<&'a QualificationEvidence> {
    let draft = draft?;
    if draft.definition_is_dirty() {
        return None;
    }
    let mut matches = state.evidence.iter().filter(|evidence| {
        evidence.suite_id.eq_ignore_ascii_case(&suite.id)
            && evidence.suite_revision == suite.revision
            && evidence
                .source
                .model_id
                .eq_ignore_ascii_case(&draft.model_name)
            && evidence.source.source_id == Some(draft.source_id)
            && evidence.source.source_digest == draft.base_source_digest
            && evidence.source.source_revision == draft.base_source_revision
    });
    let first = matches.next()?;
    matches.next().is_none().then_some(first)
}

fn platform_outcome_passed(
    outcome: &crate::state::model_library::QualificationVectorOutcome,
    platform: QualificationPlatform,
) -> bool {
    outcome
        .platforms
        .iter()
        .find(|value| value.platform == platform)
        .is_some_and(|value| value.passed)
}

fn worst_relative_error(evidence: &QualificationEvidence) -> Option<f64> {
    evidence
        .vector_outcomes
        .iter()
        .flat_map(|vector| &vector.platforms)
        .flat_map(|platform| &platform.references)
        .map(|reference| reference.relative_error.get())
        .max_by(f64::total_cmp)
}

fn format_percentage(value: f64) -> String {
    format!("{:.4}%", value * 100.0)
}

fn release_panel(
    ui: &mut Ui,
    app: &RSpiceApp,
    records: &PersistedModelRecords,
    action: &mut Option<SurfaceAction>,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let candidate = current_candidate(app, records);
    let split = width >= 720.0;
    let left_w = if split { width * 0.5 } else { width };
    let facts = release_facts(candidate);
    let facts_h = facts.len() as f32 * 35.0;
    let note_h = 154.0;
    let height = if split {
        facts_h.max(note_h)
    } else {
        facts_h + note_h
    };
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let facts_rect = Rect::from_min_max(
        rect.min,
        Pos2::new(
            rect.left() + left_w,
            if split {
                rect.bottom()
            } else {
                rect.top() + facts_h
            },
        ),
    );
    let note_rect = if split {
        Rect::from_min_max(Pos2::new(facts_rect.right(), rect.top()), rect.max)
    } else {
        Rect::from_min_max(Pos2::new(rect.left(), facts_rect.bottom()), rect.max)
    };
    ui.painter().rect_filled(facts_rect, 0.0, t.color.bg_panel);
    ui.painter().rect_filled(note_rect, 0.0, t.color.bg_panel);
    if split {
        ui.painter().vline(
            facts_rect.right(),
            facts_rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    } else {
        ui.painter().hline(
            note_rect.x_range(),
            note_rect.top(),
            Stroke::new(1.0, t.color.border),
        );
    }
    let mut facts_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(facts_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    facts_ui.spacing_mut().item_spacing = Vec2::ZERO;
    for (label, value, tone) in facts {
        release_fact_row(&mut facts_ui, &label, &value, tone);
    }

    let note_clip = note_rect.shrink2(Vec2::new(12.0, 10.0));
    let note = "Promotion creates a new immutable model revision. Existing projects remain pinned until an explicit dependency update and qualification review.";
    let galley = ui.painter().layout(
        note.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        note_clip.width(),
    );
    ui.painter()
        .with_clip_rect(note_clip)
        .galley(note_clip.min, galley, t.color.text_dim);
    let button_rect = Rect::from_min_size(
        Pos2::new(note_clip.left(), note_rect.bottom() - 48.0),
        Vec2::new(170.0, Tokens::get(ui.ctx()).metrics.ctl_h),
    );
    let mut button_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(button_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let base_enabled = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_some_and(|draft| !draft.definition_is_dirty())
        && records.qualification_error.is_none();
    action_button(
        &mut button_ui,
        "Review model promotion",
        170.0,
        base_enabled,
        Some("Save a valid model definition and repair any qualification diagnostics first."),
        SurfaceAction::OpenPromotionReview,
        action,
    );
}

fn exact_passing_evidence_available(app: &RSpiceApp, records: &PersistedModelRecords) -> bool {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return false;
    };
    if draft.definition_is_dirty() {
        return false;
    }
    let Ok(source) = ModelSourceEvidenceBinding::try_new_project_bound(
        draft.model_name.clone(),
        draft.source_id,
        draft.base_source_digest,
        draft.base_source_revision,
    ) else {
        return false;
    };
    let Some(qualification) = records.qualification.as_ref() else {
        return false;
    };
    let Ok(suites) = qualification.exact_suites_for_source(&source) else {
        return false;
    };
    !suites.is_empty()
        && suites.into_iter().all(|suite| {
            qualification
                .evidence
                .iter()
                .any(|evidence| evidence.passed && evidence.validate_bound(suite, &source).is_ok())
        })
}

fn current_candidate<'a>(
    app: &'a RSpiceApp,
    _records: &PersistedModelRecords,
) -> Option<&'a ModelReleaseCandidate> {
    let draft = app.state.workbench.model_editor.draft.as_ref()?;
    if draft.definition_is_dirty() {
        return None;
    }
    let mut matches = draft.qualification.candidates.iter().filter(|candidate| {
        candidate
            .identity
            .model_id
            .eq_ignore_ascii_case(&draft.model_name)
            && candidate.source.source_id == Some(draft.source_id)
            && candidate.source.source_digest == draft.base_source_digest
            && candidate.source.source_revision == draft.base_source_revision
    });
    let first = matches.next()?;
    matches.next().is_none().then_some(first)
}

fn release_facts(candidate: Option<&ModelReleaseCandidate>) -> Vec<(String, String, MetricTone)> {
    let Some(candidate) = candidate else {
        return vec![
            (
                "Documentation".to_owned(),
                "no release candidate".to_owned(),
                MetricTone::Warning,
            ),
            (
                "License".to_owned(),
                "no release candidate".to_owned(),
                MetricTone::Warning,
            ),
            (
                "Consumer impact".to_owned(),
                "no release candidate".to_owned(),
                MetricTone::Warning,
            ),
            (
                "Compatibility".to_owned(),
                "no release candidate".to_owned(),
                MetricTone::Warning,
            ),
        ];
    };
    let documentation = if candidate.documentation.validate_complete().is_ok() {
        ("complete".to_owned(), MetricTone::Good)
    } else {
        (
            format!(
                "{} required documents declared",
                candidate.documentation.declarations.len()
            ),
            MetricTone::Warning,
        )
    };
    let license = candidate.license.as_ref().map_or_else(
        || ("not declared".to_owned(), MetricTone::Warning),
        |license| {
            (
                format!(
                    "{} · {}",
                    license.expression,
                    license_scope_label(license.scope)
                ),
                if license.validate_for_release().is_ok() {
                    MetricTone::Good
                } else {
                    MetricTone::Warning
                },
            )
        },
    );
    let impact = candidate.consumer_impact.as_ref().map_or_else(
        || ("not assessed".to_owned(), MetricTone::Warning),
        |impact| {
            (
                format!(
                    "{} consumers · {}",
                    impact.affected_consumer_ids.len(),
                    consumer_change_label(impact.change)
                ),
                if impact.validate_for_release().is_ok() {
                    MetricTone::Good
                } else {
                    MetricTone::Warning
                },
            )
        },
    );
    let compatibility = candidate.compatibility.as_ref().map_or_else(
        || ("not assessed".to_owned(), MetricTone::Warning),
        |compatibility| {
            let ready = candidate
                .consumer_impact
                .as_ref()
                .is_some_and(|impact| compatibility.validate_for_release(impact).is_ok());
            let declared = compatibility.platforms.len();
            let unresolved = compatibility
                .platforms
                .iter()
                .filter(|platform| platform.disposition != CompatibilityDisposition::Compatible)
                .count();
            (
                if ready {
                    "desktop · WASM complete".to_owned()
                } else {
                    format!("{declared}/2 platforms declared · {unresolved} incompatible")
                },
                if ready {
                    MetricTone::Good
                } else {
                    MetricTone::Warning
                },
            )
        },
    );
    vec![
        ("Documentation".to_owned(), documentation.0, documentation.1),
        ("License".to_owned(), license.0, license.1),
        ("Consumer impact".to_owned(), impact.0, impact.1),
        ("Compatibility".to_owned(), compatibility.0, compatibility.1),
    ]
}

fn consumer_change_label(value: ConsumerChange) -> &'static str {
    match value {
        ConsumerChange::NoImpact => "no impact",
        ConsumerChange::Compatible => "compatible",
        ConsumerChange::Breaking => "breaking",
    }
}

fn license_scope_label(value: LicenseScope) -> &'static str {
    match value {
        LicenseScope::FoundryProject => "foundry project",
        LicenseScope::OrganizationInternal => "organization internal",
        LicenseScope::Redistributable => "redistributable",
    }
}

fn release_fact_row(ui: &mut Ui, label: &str, value: &str, tone: MetricTone) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 35.0), Sense::hover());
    bottom_rule(ui, rect, t.color.border);
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left() + 11.0, rect.top()),
        Pos2::new(rect.left() + rect.width() * 0.38, rect.bottom()),
    );
    let value_rect = Rect::from_min_max(
        Pos2::new(label_rect.right() + 8.0, rect.top()),
        Pos2::new(rect.right() - 10.0, rect.bottom()),
    );
    paint_cell(ui, label_rect, &DisplayCell::plain(label));
    paint_cell(ui, value_rect, &DisplayCell::toned(value, tone));
}

fn apply_action(app: &mut RSpiceApp, action: SurfaceAction) {
    match action {
        SurfaceAction::SelectSection(section) => {
            app.state.workbench.model_editor.active_section = section;
        }
        SurfaceAction::OpenParameterSchema => {
            app.state.workbench.model_editor.begin_parameter_schema();
        }
        SurfaceAction::OpenNewSection => {
            app.state.workbench.model_editor.begin_new_section();
        }
        SurfaceAction::OpenCorrelationMatrix => {
            app.state
                .workbench
                .model_editor
                .begin_correlation_matrix_edit();
        }
        SurfaceAction::OpenTemperaturePreview => {
            app.state.workbench.model_editor.temperature_preview_open = true;
        }
        SurfaceAction::OpenQualificationPlan => {
            app.state.workbench.model_editor.qualification_plan_open = true;
        }
        SurfaceAction::OpenPromotionReview => {
            app.state.workbench.model_editor.begin_promotion_review();
        }
    }
}

fn show_inspection_dialogs(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    parameter_schema_dialog(ctx, app, records);
    new_section_dialog(ctx, app, records);
    correlation_matrix_dialog(ctx, app, records);
    temperature_preview_dialog(ctx, app, records);
    qualification_plan_dialog(ctx, app, records);
    promotion_review_dialog(ctx, app, records);
    release_comparison_dialog(ctx, app, records);
}

fn show_model_editor_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    let Some(request) = active_model_editor_workflow(app) else {
        return;
    };
    match request.workflow {
        ModelEditorWorkflow::SaveRevision => save_revision_workflow(ctx, app, &request),
        ModelEditorWorkflow::ValidateCandidate => {
            validation_review_workflow(ctx, app, records, &request);
        }
        ModelEditorWorkflow::RunQualification => {
            qualification_run_workflow(ctx, app, records, &request);
        }
    }
}

fn save_revision_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    request: &super::super::commands::ModelEditorWorkflowRequest,
) {
    let writable = app.state.project_lifecycle.project_open
        && !app.state.workbench.safe_mode.project_read_only()
        && app
            .state
            .workbench
            .model_editor
            .qualification_execution
            .is_none();
    let dirty = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .is_some_and(|draft| draft.is_dirty());
    let choice = Dialog::new(
        "MODELS · VERSIONED DEFINITION",
        "Save model revision",
        "Save reviewed revision",
    )
    .description("Review the exact candidate delta and source binding before publishing one guarded project/model revision.")
    .size(DialogSize::Transaction)
    .fixed_height(430.0)
    .primary_enabled(writable && dirty)
    .secondary("Cancel")
    .show(ctx, |ui| {
        workflow_purpose_note(ui, "Saving creates one versioned engineering revision. Existing immutable releases and qualification evidence are never rewritten.");
        dialog_subheading(ui, "Candidate revision contract");
        render_table(
            ui,
            "model-save-review-contract",
            &[("Field", 0.34), ("Reviewed value", 0.66)],
            &save_review_rows(app),
            "No project-owned model candidate is open.",
        );
        if let Some(error) = request.error.as_deref() {
            blocker_note(ui, error);
        } else if !writable {
            blocker_note(
                ui,
                if app.state.workbench.safe_mode.project_read_only() {
                    "The project is open read-only. Inspection remains available, but this revision cannot be saved."
                } else {
                    "Finish or cancel the active qualification run before saving this revision."
                },
            );
        } else if !dirty {
            blocker_note(ui, "The candidate has no semantic changes to save.");
        }
    });
    match choice {
        DialogChoice::Primary if writable && dirty => {
            match crate::workbench::documents::model_editor::save_open_candidate(app) {
                Ok(revision) => {
                    app.state.push_user_message(
                        crate::diagnostics::ConsoleMessage::info(format!(
                            "Model revision r{} was saved with its exact source and qualification state.",
                            revision.get()
                        )),
                    );
                    close_model_editor_workflow();
                }
                Err(error) => set_model_editor_workflow_error(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => close_model_editor_workflow(),
        DialogChoice::None | DialogChoice::Primary | DialogChoice::Ghost => {}
    }
}

fn save_review_rows(app: &RSpiceApp) -> Vec<Vec<DisplayCell>> {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return Vec::new();
    };
    let delta = draft.delta().ok();
    let definition_delta = delta.as_ref().map_or_else(
        || "definition cannot be projected".to_owned(),
        |delta| {
            let parameter_changes = delta.added_parameters.len()
                + delta.removed_parameters.len()
                + delta.changed_parameters.len();
            format!(
                "{parameter_changes} parameter change(s) · identity {} · description {}",
                if delta.identity_changed {
                    "changed"
                } else {
                    "stable"
                },
                if delta.description_changed {
                    "changed"
                } else {
                    "stable"
                }
            )
        },
    );
    vec![
        vec![
            DisplayCell::plain("Model identity"),
            DisplayCell::mono(format!("{}/{}", draft.library_name, draft.model_name)),
        ],
        vec![
            DisplayCell::plain("Source binding"),
            DisplayCell::mono(format!(
                "r{} · {}",
                draft.base_source_revision.get(),
                short_identity(&draft.base_source_digest.to_string())
            )),
        ],
        vec![
            DisplayCell::plain("Definition delta"),
            DisplayCell::plain(definition_delta),
        ],
        vec![
            DisplayCell::plain("Qualification delta"),
            DisplayCell::plain(if draft.qualification_is_dirty() {
                "changed · retained with this revision"
            } else {
                "unchanged"
            }),
        ],
        vec![
            DisplayCell::plain("Publication"),
            DisplayCell::plain("guarded project transaction · undoable revision"),
        ],
    ]
}

fn validation_review_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
    request: &super::super::commands::ModelEditorWorkflowRequest,
) {
    let prepared = request.prepared;
    let choice = Dialog::new(
        "MODELS · SCHEMA · PHYSICS · PLATFORM",
        "Validate model candidate",
        if prepared { "Close report" } else { "Run validation" },
    )
    .description("Review source identity, typed schema, physical bounds, qualification structure, and retained platform evidence without changing the model definition.")
    .size(DialogSize::Transaction)
    .fixed_height(470.0)
    ;
    let choice = if prepared {
        choice.show(ctx, |ui| {
            validation_review_body(ui, app, records, request, prepared);
        })
    } else {
        choice.secondary("Cancel").show(ctx, |ui| {
            validation_review_body(ui, app, records, request, prepared);
        })
    };
    match choice {
        DialogChoice::Primary if !prepared => {
            crate::workbench::documents::model_editor::validate_open_candidate(app);
            prepare_model_editor_workflow();
        }
        DialogChoice::Primary | DialogChoice::Secondary | DialogChoice::Cancelled => {
            close_model_editor_workflow();
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn validation_review_body(
    ui: &mut Ui,
    app: &RSpiceApp,
    records: &PersistedModelRecords,
    request: &super::super::commands::ModelEditorWorkflowRequest,
    prepared: bool,
) {
    workflow_purpose_note(
        ui,
        "Validation is bound to the exact candidate source and current project revision. A failed check cannot be accepted as release evidence.",
    );
    dialog_subheading(ui, "Validation report");
    render_table(
        ui,
        "model-validation-review-contract",
        &[("Check", 0.34), ("Observed", 0.44), ("State", 0.22)],
        &validation_review_rows(app, records, prepared),
        if prepared {
            "No model validation report is available."
        } else {
            "Run validation to produce an exact candidate report."
        },
    );
    if let Some(error) = request.error.as_deref() {
        blocker_note(ui, error);
    } else if !app.state.workbench.model_editor.diagnostics.is_empty() {
        blocker_note(
            ui,
            &app.state
                .workbench
                .model_editor
                .diagnostics
                .iter()
                .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
                .collect::<Vec<_>>()
                .join(" · "),
        );
    }
}

fn validation_review_rows(
    app: &RSpiceApp,
    records: &PersistedModelRecords,
    prepared: bool,
) -> Vec<Vec<DisplayCell>> {
    if !prepared {
        let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
            return Vec::new();
        };
        return vec![
            vec![
                DisplayCell::plain("Candidate source"),
                DisplayCell::mono(format!(
                    "{} · r{} · {}",
                    draft.model_name,
                    draft.base_source_revision.get(),
                    short_identity(&draft.base_source_digest.to_string())
                )),
                DisplayCell::toned("ready", MetricTone::Neutral),
            ],
            vec![
                DisplayCell::plain("Validation scope"),
                DisplayCell::plain(
                    "source identity · typed schema · physical bounds · qualification structure · platform evidence",
                ),
                DisplayCell::toned("pending", MetricTone::Warning),
            ],
        ];
    }
    let valid = app.state.workbench.model_editor.validation.is_some();
    let definition_valid =
        records.metadata_error.is_none() && app.state.workbench.model_editor.diagnostics.is_empty();
    let qualification_valid = records.qualification_error.is_none();
    let qualification = qualification_totals(app, records);
    let parity_complete = qualification.total > 0
        && qualification.desktop_passed == qualification.total
        && qualification.wasm_passed == qualification.total;
    vec![
        validation_row(
            "Source identity and digest",
            if valid {
                "exact source and project revisions bound"
            } else {
                "binding rejected"
            },
            valid,
        ),
        validation_row(
            "Typed schema and physical bounds",
            if definition_valid {
                "all declared values valid"
            } else {
                "one or more definition diagnostics"
            },
            definition_valid,
        ),
        validation_row(
            "Qualification contract",
            if qualification_valid {
                "persisted structure valid"
            } else {
                "qualification structure invalid"
            },
            qualification_valid,
        ),
        validation_row(
            "Desktop / WebAssembly parity",
            &format!(
                "{} / {} desktop · {} / {} WebAssembly",
                qualification.desktop_passed,
                qualification.total,
                qualification.wasm_passed,
                qualification.total
            ),
            parity_complete,
        ),
    ]
}

fn validation_row(check: &str, observed: &str, passed: bool) -> Vec<DisplayCell> {
    vec![
        DisplayCell::plain(check),
        DisplayCell::plain(observed),
        DisplayCell::toned(
            if passed { "pass" } else { "review" },
            if passed {
                MetricTone::Good
            } else {
                MetricTone::Warning
            },
        ),
    ]
}

fn qualification_run_workflow(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
    request: &super::super::commands::ModelEditorWorkflowRequest,
) {
    let runnable = Command::ModelRunQualificationTests.is_enabled(app);
    let choice = Dialog::new(
        "MODELS · TEST VECTORS · PLATFORM PARITY",
        "Run model qualification tests",
        "Queue qualification tests",
    )
    .description("Confirm the exact clean source, executable suites, current runtime target, and evidence-retention contract before dispatch.")
    .size(DialogSize::Transaction)
    .fixed_height(430.0)
    .primary_enabled(runnable)
    .secondary("Cancel")
    .show(ctx, |ui| {
        workflow_purpose_note(ui, "Qualification publishes only complete suite runs. Cross-platform evidence is assembled only from exact matching Desktop and WebAssembly source bindings.");
        dialog_subheading(ui, "Execution contract");
        render_table(
            ui,
            "model-qualification-run-contract",
            &[("Field", 0.34), ("Reviewed value", 0.66)],
            &qualification_run_rows(app, records),
            "No executable qualification plan is available.",
        );
        if let Some(error) = request.error.as_deref() {
            blocker_note(ui, error);
        } else if !runnable {
            let reason = match Command::ModelRunQualificationTests.availability(app) {
                CommandAvailability::Disabled(reason) => reason,
                CommandAvailability::Available | CommandAvailability::Hidden => {
                    "Qualification is unavailable in this context."
                }
            };
            blocker_note(ui, reason);
        }
    });
    match choice {
        DialogChoice::Primary if runnable => {
            match crate::workbench::documents::model_editor::start_qualification_execution(app) {
                Ok(()) => close_model_editor_workflow(),
                Err(error) => set_model_editor_workflow_error(error),
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => close_model_editor_workflow(),
        DialogChoice::None | DialogChoice::Primary | DialogChoice::Ghost => {}
    }
}

fn qualification_run_rows(
    app: &RSpiceApp,
    _records: &PersistedModelRecords,
) -> Vec<Vec<DisplayCell>> {
    let Some(draft) = app.state.workbench.model_editor.draft.as_ref() else {
        return Vec::new();
    };
    let Ok(source) = ModelSourceEvidenceBinding::try_new_project_bound(
        draft.model_name.clone(),
        draft.source_id,
        draft.base_source_digest,
        draft.base_source_revision,
    ) else {
        return Vec::new();
    };
    let exact_suites = draft
        .qualification
        .exact_suites_for_source(&source)
        .unwrap_or_default();
    let vectors = exact_suites
        .iter()
        .map(|suite| suite.vectors.len())
        .sum::<usize>();
    let exact_evidence = exact_suites
        .iter()
        .flat_map(|suite| {
            draft
                .qualification
                .evidence
                .iter()
                .filter(|evidence| evidence.validate_bound(suite, &source).is_ok())
        })
        .count();
    let target = if cfg!(target_arch = "wasm32") {
        "WebAssembly runtime"
    } else {
        "Local desktop runtime"
    };
    vec![
        vec![
            DisplayCell::plain("Model scope"),
            DisplayCell::mono(format!(
                "{} · r{}",
                draft.model_name,
                draft.base_source_revision.get()
            )),
        ],
        vec![
            DisplayCell::plain("Executable expansion"),
            DisplayCell::plain(format!(
                "{} suite(s) · {vectors} vector(s)",
                exact_suites.len()
            )),
        ],
        vec![
            DisplayCell::plain("Runtime target"),
            DisplayCell::plain(target),
        ],
        vec![
            DisplayCell::plain("Validation"),
            DisplayCell::toned(
                if app.state.workbench.model_editor.validation.is_some() {
                    "current source-bound report"
                } else {
                    "validation required"
                },
                if app.state.workbench.model_editor.validation.is_some() {
                    MetricTone::Good
                } else {
                    MetricTone::Warning
                },
            ),
        ],
        vec![
            DisplayCell::plain("Retained evidence"),
            DisplayCell::plain(format!(
                "{} existing cross-platform record(s) · immutable completed suites",
                exact_evidence
            )),
        ],
        vec![
            DisplayCell::plain("Promotion policy"),
            DisplayCell::plain("independent review required after completion"),
        ],
    ]
}

fn workflow_purpose_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (ui.available_width() - 28.0).max(1.0),
    );
    let height = (galley.size().y + 18.0).max(42.0);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), height),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().rect_filled(
        Rect::from_min_max(rect.min, Pos2::new(rect.left() + 2.0, rect.bottom())),
        0.0,
        t.color.accent,
    );
    ui.painter().galley(
        Pos2::new(rect.left() + 12.0, rect.top() + 9.0),
        galley,
        t.color.text_dim,
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(text);
    });
}

fn parameter_schema_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    _records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.parameter_schema_open {
        return;
    }
    let writable = !app.state.workbench.safe_mode.project_read_only();
    let choice = Dialog::new(
        "MODEL · TYPED PARAMETER SCHEMA",
        "Parameter schema",
        "Close",
    )
    .description("Edit typed declarations and exact process-section overrides. Changes remain transactional until Save model revision succeeds.")
    .size(DialogSize::Transaction)
    .fixed_height(680.0)
    .show(ctx, |ui| {
        section_override_editor(ui, app, writable);
        parameter_schema_editor(ui, app, writable);
    });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        app.state.workbench.model_editor.parameter_schema_open = false;
    }
}

fn section_override_editor(ui: &mut Ui, app: &mut RSpiceApp, writable: bool) {
    let (sections, parameters) = app.state.workbench.model_editor.draft.as_ref().map_or_else(
        || (Vec::new(), Vec::new()),
        |draft| {
            (
                draft
                    .metadata
                    .sections
                    .iter()
                    .map(|section| section.name.clone())
                    .collect::<Vec<_>>(),
                draft
                    .metadata
                    .parameters
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect::<Vec<_>>(),
            )
        },
    );
    dialog_subheading(ui, "Process-section value override");
    if sections.is_empty() || parameters.is_empty() {
        promotion_note(
            ui,
            None,
            if sections.is_empty() {
                "Create a named process section before authoring section-specific parameter values."
            } else {
                "The model declares no typed parameters that can be overridden."
            },
        );
        return;
    }

    let [section_rect, parameter_rect, value_rect] = qualification_three_rects(ui);
    let mut selection_changed = false;
    {
        let editor = &mut app.state.workbench.model_editor;
        selection_changed |= section_override_combo(
            ui,
            section_rect,
            "Process section",
            "model-parameter-schema-section",
            &mut editor.parameter_schema_section,
            &sections,
            writable,
        );
        selection_changed |= section_override_combo(
            ui,
            parameter_rect,
            "Parameter",
            "model-parameter-schema-parameter",
            &mut editor.parameter_schema_parameter,
            &parameters,
            writable,
        );
    }
    if selection_changed {
        app.state
            .workbench
            .model_editor
            .refresh_parameter_schema_override_editor();
    }
    qualification_text_field(
        ui,
        value_rect,
        "Section value",
        &mut app
            .state
            .workbench
            .model_editor
            .parameter_schema_override_value,
        true,
        writable,
    );

    let override_exists = app
        .state
        .workbench
        .model_editor
        .parameter_schema_override_exists();
    let (actions_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), 38.0),
        Sense::hover(),
    );
    let mut actions = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(actions_rect.shrink2(Vec2::new(0.0, 4.0)))
            .layout(Layout::left_to_right(Align::Center)),
    );
    actions.spacing_mut().item_spacing.x = 6.0;
    if Button::new(if override_exists {
        "Update override"
    } else {
        "Add override"
    })
    .accent()
    .enabled(writable)
    .show(&mut actions)
    .clicked()
    {
        app.state
            .workbench
            .model_editor
            .commit_parameter_schema_override();
    }
    if Button::new("Remove override")
        .enabled(writable && override_exists)
        .show(&mut actions)
        .on_disabled_hover_text(if override_exists {
            "Section overrides cannot be changed while the project is read-only."
        } else {
            "The selected parameter inherits its value and has no override to remove."
        })
        .clicked()
    {
        app.state
            .workbench
            .model_editor
            .remove_parameter_schema_override();
    }
    let error = app
        .state
        .workbench
        .model_editor
        .parameter_schema_override_error
        .clone();
    promotion_note(
        ui,
        error.as_deref(),
        if override_exists {
            "The selected value is an explicit section delta. Removing it restores inherited resolution through the declared parent graph."
        } else {
            "The selected value currently resolves through the section's parent graph; Add override creates one explicit typed delta."
        },
    );
}

fn section_override_combo(
    ui: &mut Ui,
    rect: Rect,
    label: &str,
    salt: &str,
    value: &mut String,
    options: &[String],
    enabled: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let label_rect = Rect::from_min_max(rect.min, Pos2::new(rect.right(), rect.top() + 16.0));
    paint_elided(
        ui,
        label_rect,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        0.0,
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top() + 20.0),
        Pos2::new(rect.right(), rect.bottom()),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let before = value.clone();
    let response = field
        .add_enabled_ui(enabled, |ui| {
            egui::ComboBox::from_id_salt(salt)
                .selected_text(value.as_str())
                .width(field_rect.width())
                .show_ui(ui, |ui| {
                    for option in options {
                        ui.selectable_value(value, option.clone(), option);
                    }
                })
        })
        .inner;
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_label(label);
        });
    *value != before
}

fn parameter_schema_editor(ui: &mut Ui, app: &mut RSpiceApp, writable: bool) {
    const COLUMNS: [(&str, f32); 5] = [
        ("Parameter", 0.16),
        ("Type", 0.14),
        ("Unit", 0.13),
        ("Bounds", 0.23),
        ("Description", 0.34),
    ];
    let mut changed = false;
    ScrollArea::horizontal()
        .id_salt("model-editor-schema-dialog")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, &COLUMNS);
            let Some(draft) = app.state.workbench.model_editor.draft.as_mut() else {
                table_empty(ui, width, "No open model parameter schema.");
                return;
            };
            if draft.parameters.is_empty() {
                table_empty(ui, width, "The candidate declares no parameters.");
                return;
            }
            for (index, parameter) in draft.parameters.iter_mut().enumerate() {
                let cells = interactive_table_row(ui, width, &COLUMNS, false);
                paint_cell(ui, cells[0], &DisplayCell::mono(&parameter.name));
                let type_rect = cells[1].shrink2(Vec2::new(4.0, 3.0));
                let mut type_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(type_rect)
                        .layout(Layout::left_to_right(Align::Center)),
                );
                let before_kind = parameter.kind;
                let type_response = type_ui
                    .add_enabled_ui(writable, |ui| {
                        egui::ComboBox::from_id_salt(("model-parameter-kind", index))
                            .selected_text(match parameter.kind {
                                ModelParameterKind::Numeric => "numeric",
                                ModelParameterKind::String => "string",
                            })
                            .width((type_rect.width() - 24.0).max(1.0))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut parameter.kind,
                                    ModelParameterKind::Numeric,
                                    "numeric",
                                );
                                ui.selectable_value(
                                    &mut parameter.kind,
                                    ModelParameterKind::String,
                                    "string",
                                );
                            })
                    })
                    .inner;
                ui.ctx()
                    .accesskit_node_builder(type_response.response.id, |node| {
                        node.set_label(format!("Type for {}", parameter.name));
                    });
                changed |= parameter.kind != before_kind;
                changed |= schema_text_edit_enabled(
                    ui,
                    cells[2],
                    &mut parameter.unit,
                    &format!("Unit for {}", parameter.name),
                    writable,
                );
                changed |= bounds_edit_enabled(
                    ui,
                    cells[3],
                    &mut parameter.lower_bound,
                    &mut parameter.upper_bound,
                    &parameter.name,
                    writable,
                );
                changed |= schema_text_edit_enabled(
                    ui,
                    cells[4],
                    &mut parameter.description,
                    &format!("Description for {}", parameter.name),
                    writable,
                );
            }
        });
    if changed {
        app.state
            .workbench
            .model_editor
            .invalidate_candidate_evidence();
    }
}

fn schema_text_edit(ui: &mut Ui, rect: Rect, value: &mut String, label: &str) -> bool {
    schema_text_edit_enabled(ui, rect, value, label, true)
}

fn schema_text_edit_enabled(
    ui: &mut Ui,
    rect: Rect,
    value: &mut String,
    label: &str,
    enabled: bool,
) -> bool {
    let edit_rect = rect.shrink2(Vec2::new(4.0, 3.0));
    schema_text_edit_exact(ui, edit_rect, value, label, enabled)
}

fn schema_text_edit_exact(
    ui: &mut Ui,
    edit_rect: Rect,
    value: &mut String,
    label: &str,
    enabled: bool,
) -> bool {
    let mut edit_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(edit_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let response = edit_ui
        .add_enabled_ui(enabled, |ui| {
            ui.add_sized(
                edit_rect.size(),
                egui::TextEdit::singleline(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            )
        })
        .inner;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
    });
    response.changed()
}

fn bounds_edit_enabled(
    ui: &mut Ui,
    rect: Rect,
    lower: &mut String,
    upper: &mut String,
    parameter_name: &str,
    enabled: bool,
) -> bool {
    let inner = rect.shrink2(Vec2::new(4.0, 3.0));
    let gap = 4.0;
    let field_width = ((inner.width() - gap) * 0.5).max(1.0);
    let lower_rect = Rect::from_min_size(inner.min, Vec2::new(field_width, inner.height()));
    let upper_rect = Rect::from_min_size(
        Pos2::new(lower_rect.right() + gap, inner.top()),
        Vec2::new(field_width, inner.height()),
    );
    schema_text_edit_exact(
        ui,
        lower_rect,
        lower,
        &format!("Lower bound for {parameter_name}"),
        enabled,
    ) | schema_text_edit_exact(
        ui,
        upper_rect,
        upper,
        &format!("Upper bound for {parameter_name}"),
        enabled,
    )
}

fn new_section_dialog(ctx: &egui::Context, app: &mut RSpiceApp, records: &PersistedModelRecords) {
    if !app.state.workbench.model_editor.new_section_open {
        return;
    }
    let writable = !app.state.workbench.safe_mode.project_read_only();
    let primary_enabled = writable && !app.state.workbench.model_editor.new_section_name.is_empty();
    let choice = Dialog::new(
        "MODEL · PROCESS SECTION",
        "New process section",
        "Add section",
    )
    .description(
        "Create one validated inheritance node. The section remains part of the candidate until Save model revision commits it atomically.",
    )
    .fixed_height(300.0)
    .secondary("Cancel")
    .primary_enabled(primary_enabled)
    .show(ctx, |ui| {
        ui.set_enabled(writable);
        model_text_field(
            ui,
            "Section name",
            "Case-insensitive SPICE section identity",
            &mut app.state.workbench.model_editor.new_section_name,
        );
        let parent_names = records
            .metadata
            .as_ref()
            .map(|metadata| {
                metadata
                    .sections
                    .iter()
                    .map(|section| section.name.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        model_parent_field(
            ui,
            &mut app.state.workbench.model_editor.new_section_parent,
            &parent_names,
        );
        if let Some(error) = app
            .state
            .workbench
            .model_editor
            .new_section_error
            .as_deref()
        {
            blocker_note(ui, error);
        }
    });
    match choice {
        DialogChoice::Primary => {
            app.state.workbench.model_editor.commit_new_section();
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            app.state.workbench.model_editor.new_section_open = false;
            app.state.workbench.model_editor.new_section_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn model_text_field(ui: &mut Ui, label: &str, help: &str, value: &mut String) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 54.0), Sense::hover());
    let label_rect = Rect::from_min_max(
        Pos2::new(rect.left(), rect.top()),
        Pos2::new(rect.left() + 190.0_f32.min(width * 0.34), rect.bottom()),
    );
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_top(),
        Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().with_clip_rect(label_rect).text(
        Pos2::new(label_rect.left(), label_rect.top() + 19.0),
        Align2::LEFT_TOP,
        help,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let field_rect = Rect::from_min_max(
        Pos2::new(label_rect.right() + 10.0, rect.top()),
        Pos2::new(rect.right(), rect.top() + t.metrics.ctl_h),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let response = field.add_sized(
        field_rect.size(),
        egui::TextEdit::singleline(value).font(theme::mono(tokens::FS_0, FontWeight::Regular)),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(label);
    });
}

fn model_parent_field(ui: &mut Ui, value: &mut String, parents: &[String]) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 54.0), Sense::hover());
    let label_w = 190.0_f32.min(width * 0.34);
    ui.painter().with_clip_rect(rect).text(
        rect.left_top(),
        Align2::LEFT_TOP,
        "Parent section",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter()
        .with_clip_rect(Rect::from_min_max(
            rect.min,
            Pos2::new(rect.left() + label_w, rect.bottom()),
        ))
        .text(
            Pos2::new(rect.left(), rect.top() + 19.0),
            Align2::LEFT_TOP,
            "Base model when no parent is selected",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
    let field_rect = Rect::from_min_max(
        Pos2::new(rect.left() + label_w + 10.0, rect.top()),
        Pos2::new(rect.right(), rect.top() + t.metrics.ctl_h),
    );
    let mut field = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(field_rect)
            .layout(Layout::left_to_right(Align::Center)),
    );
    let response = egui::ComboBox::from_id_salt("model-editor-new-section-parent")
        .selected_text(if value.is_empty() {
            "Base model"
        } else {
            value.as_str()
        })
        .width(field_rect.width())
        .show_ui(&mut field, |ui| {
            ui.selectable_value(value, String::new(), "Base model");
            for parent in parents {
                ui.selectable_value(value, parent.clone(), parent);
            }
        });
    ui.ctx()
        .accesskit_node_builder(response.response.id, |node| {
            node.set_label("Parent section");
        });
}

fn correlation_matrix_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    _records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.correlation_matrix_open {
        return;
    }
    let writable = !app.state.workbench.safe_mode.project_read_only();
    let choice = Dialog::new(
        "MODEL · STATISTICAL CORRELATION",
        "Correlation matrix",
        "Apply matrix",
    )
    .description("Edit the candidate's ordered correlation coefficients. Symmetric entries are kept synchronized and the complete positive-semidefinite matrix is validated atomically.")
    .size(DialogSize::Transaction)
    .fixed_height(680.0)
    .secondary("Cancel")
    .primary_enabled(writable)
    .show(ctx, |ui| {
        ui.set_enabled(writable);
        let matrices = app
            .state
            .workbench
            .model_editor
            .correlation_matrix_candidates
            .clone();
        if matrices.is_empty() {
            table_empty(
                ui,
                ui.available_width().max(1.0),
                "No correlation matrix is persisted.",
            );
            return;
        }
        ScrollArea::horizontal()
            .id_salt("model-editor-correlation-matrix-scroll")
            .auto_shrink([false, true])
            .show(ui, |ui| {
        for (matrix_index, matrix) in matrices.iter().enumerate() {
            dialog_subheading(ui, &matrix.group);
            let mut columns = vec![("Variable".to_owned(), 0.24)];
            let remaining = 0.76 / matrix.variables.len().max(1) as f32;
            columns.extend(
                matrix
                    .variables
                    .iter()
                    .map(|name| (name.clone(), remaining)),
            );
            let borrowed = columns
                .iter()
                .map(|(name, width)| (name.as_str(), *width))
                .collect::<Vec<_>>();
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, &borrowed);
            if matrix.variables.is_empty() {
                table_empty(ui, width, "The matrix contains no variables.");
                continue;
            }
            for (row_index, name) in matrix.variables.iter().enumerate() {
                let cells = interactive_table_row(ui, width, &borrowed, false);
                paint_cell(ui, cells[0], &DisplayCell::mono(name));
                for column_index in 0..matrix.variables.len() {
                    if row_index == column_index {
                        paint_cell(ui, cells[column_index + 1], &DisplayCell::mono("1"));
                        continue;
                    }
                    let Some(current) = app
                        .state
                        .workbench
                        .model_editor
                        .correlation_matrix_edits
                        .get(matrix_index)
                        .and_then(|rows| rows.get(row_index))
                        .and_then(|row| row.get(column_index))
                        .cloned()
                    else {
                        paint_cell(
                            ui,
                            cells[column_index + 1],
                            &DisplayCell::toned("missing", MetricTone::Error),
                        );
                        continue;
                    };
                    let mut value = current;
                    if schema_text_edit(
                        ui,
                        cells[column_index + 1],
                        &mut value,
                        &format!(
                            "Correlation {} to {} in {}",
                            matrix.variables[row_index],
                            matrix.variables[column_index],
                            matrix.group
                        ),
                    )
                        && let Some(rows) = app
                            .state
                            .workbench
                            .model_editor
                            .correlation_matrix_edits
                            .get_mut(matrix_index)
                    {
                        rows[row_index][column_index] = value.clone();
                        rows[column_index][row_index] = value;
                    }
                }
            }
        }
            });
        if let Some(error) = app
            .state
            .workbench
            .model_editor
            .correlation_matrix_error
            .as_deref()
        {
            blocker_note(ui, error);
        }
    });
    match choice {
        DialogChoice::Primary => {
            app.state
                .workbench
                .model_editor
                .commit_correlation_matrix_edit();
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            app.state.workbench.model_editor.correlation_matrix_open = false;
            app.state
                .workbench
                .model_editor
                .correlation_matrix_candidates
                .clear();
            app.state
                .workbench
                .model_editor
                .correlation_matrix_edits
                .clear();
            app.state.workbench.model_editor.correlation_matrix_error = None;
        }
        DialogChoice::None | DialogChoice::Ghost => {}
    }
}

fn temperature_preview_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.temperature_preview_open {
        return;
    }
    let choice = Dialog::new(
        "MODEL · TEMPERATURE LAW",
        "Temperature-law plot",
        "Close",
    )
    .description(
        "Evaluate the candidate's declared equation or lookup table over its complete valid range using typed model parameters.",
    )
    .show(ctx, |ui| {
        let Some(metadata) = records.metadata.as_ref() else {
            table_empty(
                ui,
                ui.available_width().max(1.0),
                "No temperature-law metadata is available.",
            );
            return;
        };
        if metadata.temperature_laws.is_empty() {
            table_empty(
                ui,
                ui.available_width().max(1.0),
                "No temperature laws are declared.",
            );
            return;
        }
        for law in &metadata.temperature_laws {
            dialog_subheading(ui, &law.quantity);
            match temperature_samples(metadata, law) {
                Ok(samples) => temperature_plot(ui, law, &samples),
                Err(error) => blocker_note(ui, &error),
            }
        }
    });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        app.state.workbench.model_editor.temperature_preview_open = false;
    }
}

fn temperature_samples(
    metadata: &ModelDefinitionMetadata,
    law: &crate::state::model_library::TemperatureLawDefinition,
) -> Result<Vec<(f64, f64)>, String> {
    const SAMPLE_COUNT: usize = 121;
    let minimum = law.valid_range.minimum_c.get();
    let span = law.valid_range.maximum_c.get() - minimum;
    (0..SAMPLE_COUNT)
        .map(|index| {
            let temperature = minimum + span * index as f64 / (SAMPLE_COUNT - 1) as f64;
            metadata
                .evaluate_temperature_law(&law.quantity, temperature)
                .map(|evaluation| (temperature, evaluation.value.get()))
                .map_err(|error| error.to_string())
        })
        .collect()
}

fn temperature_plot(
    ui: &mut Ui,
    law: &crate::state::model_library::TemperatureLawDefinition,
    samples: &[(f64, f64)],
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(320.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 190.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let plot = Rect::from_min_max(
        Pos2::new(rect.left() + 48.0, rect.top() + 14.0),
        Pos2::new(rect.right() - 14.0, rect.bottom() - 30.0),
    );
    let minimum_t = law.valid_range.minimum_c.get();
    let maximum_t = law.valid_range.maximum_c.get();
    let (mut minimum_y, mut maximum_y) = samples.iter().fold(
        (f64::INFINITY, f64::NEG_INFINITY),
        |(minimum, maximum), (_, value)| (minimum.min(*value), maximum.max(*value)),
    );
    if (maximum_y - minimum_y).abs() <= f64::EPSILON {
        let padding = maximum_y.abs().max(1.0) * 0.05;
        minimum_y -= padding;
        maximum_y += padding;
    }
    for division in 0..=4 {
        let fraction = division as f32 / 4.0;
        let y = egui::lerp(plot.bottom()..=plot.top(), fraction);
        ui.painter().hline(
            plot.x_range(),
            y,
            Stroke::new(1.0, t.color.border.gamma_multiply(0.55)),
        );
    }
    let points = samples
        .iter()
        .map(|(temperature, value)| {
            let x_fraction = ((*temperature - minimum_t) / (maximum_t - minimum_t)) as f32;
            let y_fraction = ((*value - minimum_y) / (maximum_y - minimum_y)) as f32;
            Pos2::new(
                egui::lerp(plot.left()..=plot.right(), x_fraction),
                egui::lerp(plot.bottom()..=plot.top(), y_fraction),
            )
        })
        .collect::<Vec<_>>();
    for pair in points.windows(2) {
        ui.painter()
            .line_segment([pair[0], pair[1]], Stroke::new(1.6, t.color.accent));
    }
    ui.painter().text(
        Pos2::new(plot.left(), rect.bottom() - 17.0),
        Align2::LEFT_CENTER,
        format!("{minimum_t} °C"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(plot.right(), rect.bottom() - 17.0),
        Align2::RIGHT_CENTER,
        format!("{maximum_t} °C"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(plot.left() - 7.0, plot.top()),
        Align2::RIGHT_TOP,
        format!("{maximum_y:.6e}"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        Pos2::new(plot.left() - 7.0, plot.bottom()),
        Align2::RIGHT_BOTTOM,
        format!("{minimum_y:.6e}"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn release_comparison_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    records: &PersistedModelRecords,
) {
    if !app.state.workbench.model_editor.comparison_open {
        return;
    }
    let releases = records
        .qualification
        .as_ref()
        .map(|state| {
            state
                .releases
                .iter()
                .map(|release| {
                    (
                        release.identity.id.clone(),
                        release.identity.version.clone(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if app
        .state
        .workbench
        .model_editor
        .comparison_release_id
        .is_empty()
        && let Some((release_id, _)) = releases.first()
    {
        app.state.workbench.model_editor.comparison_release_id = release_id.clone();
    }
    let selected_release = app
        .state
        .workbench
        .model_editor
        .comparison_release_id
        .clone();
    let comparison = app.state.workbench.model_editor.draft.as_ref().map_or_else(
        || Err("No project-owned model candidate is open".to_owned()),
        |draft| draft.compare_release(&selected_release),
    );
    let choice = Dialog::new(
        "MODELS · SEMANTIC AND NUMERICAL DELTA",
        "Compare model with approved release",
        "Close",
    )
    .description("Compare the open candidate with an immutable release using retained source bytes and exact qualification evidence.")
    .size(DialogSize::Transaction)
    .fixed_height(680.0)
    .show(ctx, |ui| {
        let (selector_rect, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width().max(1.0), 48.0),
            Sense::hover(),
        );
        ui.painter().text(
            selector_rect.left_top(),
            Align2::LEFT_TOP,
            "Approved release",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            Tokens::get(ui.ctx()).color.text_dim,
        );
        let field_rect = Rect::from_min_max(
            Pos2::new(selector_rect.left(), selector_rect.top() + 18.0),
            selector_rect.max,
        );
        let selected_label = releases
            .iter()
            .find(|(id, _)| id.eq_ignore_ascii_case(&selected_release))
            .map_or("No retained release".to_owned(), |(id, version)| {
                format!("{id} · {version}")
            });
        let mut selector = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(field_rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        let response = egui::ComboBox::from_id_salt("model-release-comparison-selector")
            .selected_text(selected_label)
            .width(field_rect.width())
            .show_ui(&mut selector, |ui| {
                for (id, version) in &releases {
                    ui.selectable_value(
                        &mut app.state.workbench.model_editor.comparison_release_id,
                        id.clone(),
                        format!("{id} · {version}"),
                    );
                }
            });
        ui.ctx()
            .accesskit_node_builder(response.response.id, |node| {
                node.set_label("Approved model release to compare");
            });

        const COLUMNS: [(&str, f32); 5] = [
            ("Parameter / behavior", 0.22),
            ("Approved release", 0.20),
            ("Candidate", 0.20),
            ("Effect", 0.22),
            ("Status", 0.16),
        ];
        let rows = comparison.as_ref().map_or_else(
            |_| Vec::new(),
            |rows| {
                rows.iter()
                .map(|row| {
                    let (status, tone) = match row.disposition {
                        ModelComparisonDisposition::Unchanged => ("unchanged", MetricTone::Good),
                        ModelComparisonDisposition::Improved => ("improved", MetricTone::Good),
                        ModelComparisonDisposition::Review => ("review", MetricTone::Warning),
                        ModelComparisonDisposition::Blocking => ("blocking", MetricTone::Error),
                    };
                    vec![
                        DisplayCell::mono(&row.item),
                        DisplayCell::mono(&row.released),
                        DisplayCell::mono(&row.candidate),
                        DisplayCell::plain(&row.effect),
                        DisplayCell::toned(status, tone),
                    ]
                })
                .collect()
            },
        );
        render_table(
            ui,
            "model-editor-release-comparison",
            &COLUMNS,
            &rows,
            comparison.as_ref().err().map_or(
                "No immutable model release is persisted for comparison.",
                String::as_str,
            ),
        );
    });
    if matches!(choice, DialogChoice::Primary | DialogChoice::Cancelled) {
        app.state.workbench.model_editor.comparison_open = false;
    }
}

fn dialog_subheading(ui: &mut Ui, title: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), 30.0),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().text(
        Pos2::new(rect.left() + 9.0, rect.center().y),
        Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
}

fn blocker_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let galley = ui.painter().layout(
        text.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        (ui.available_width() - 24.0).max(1.0),
    );
    let height = (galley.size().y + 20.0).max(40.0);
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width().max(1.0), height),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().galley(
        Pos2::new(rect.left() + 12.0, rect.top() + 10.0),
        galley,
        t.color.text_dim,
    );
}

fn render_table(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    columns: &[(&str, f32)],
    rows: &[Vec<DisplayCell>],
    empty: &str,
) {
    ScrollArea::horizontal()
        .id_salt(id)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, columns);
            if rows.is_empty() {
                table_empty(ui, width, empty);
                return;
            }
            for row in rows {
                let rects = table_row(ui, width, columns, false);
                for (rect, cell) in rects.into_iter().zip(row) {
                    paint_cell(ui, rect, cell);
                }
            }
        });
}

fn table_header(ui: &mut Ui, width: f32, columns: &[(&str, f32)]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, TABLE_HEADER_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    let cells = split_columns(rect, columns);
    for (index, ((label, _), cell)) in columns.iter().zip(cells).enumerate() {
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        paint_cell(
            ui,
            cell,
            &DisplayCell {
                text: label.to_uppercase(),
                mono: false,
                tone: MetricTone::Neutral,
            },
        );
    }
    bottom_rule(ui, rect, t.color.border);
}

fn table_row(ui: &mut Ui, width: f32, columns: &[(&str, f32)], selected: bool) -> Vec<Rect> {
    let height = Tokens::get(ui.ctx()).metrics.row_h;
    table_row_with_height(ui, width, columns, selected, height)
}

fn interactive_table_row(
    ui: &mut Ui,
    width: f32,
    columns: &[(&str, f32)],
    selected: bool,
) -> Vec<Rect> {
    let t = Tokens::get(ui.ctx());
    table_row_with_height(ui, width, columns, selected, t.metrics.ctl_h + 8.0)
}

fn table_row_with_height(
    ui: &mut Ui,
    width: f32,
    columns: &[(&str, f32)],
    selected: bool,
    height: f32,
) -> Vec<Rect> {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
    let fill = if selected {
        t.color.bg_active
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_app
    };
    ui.painter().rect_filled(rect, 0.0, fill);
    bottom_rule(ui, rect, t.color.border);
    let cells = split_columns(rect, columns);
    for cell in cells.iter().skip(1) {
        ui.painter().vline(
            cell.left(),
            cell.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    cells
}

fn table_empty(ui: &mut Ui, width: f32, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 52.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    bottom_rule(ui, rect, t.color.border);
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let message = design_system::elide_text(ui, message, &font, (rect.width() - 20.0).max(1.0));
    ui.painter().text(
        Pos2::new(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        message,
        font,
        t.color.text_dim,
    );
}

fn split_columns(rect: Rect, columns: &[(&str, f32)]) -> Vec<Rect> {
    let total = columns
        .iter()
        .map(|(_, share)| share.max(0.0))
        .sum::<f32>()
        .max(f32::EPSILON);
    let mut left = rect.left();
    columns
        .iter()
        .enumerate()
        .map(|(index, (_, share))| {
            let right = if index + 1 == columns.len() {
                rect.right()
            } else {
                left + rect.width() * share.max(0.0) / total
            };
            let cell =
                Rect::from_min_max(Pos2::new(left, rect.top()), Pos2::new(right, rect.bottom()));
            left = right;
            cell
        })
        .collect()
}

fn paint_cell(ui: &Ui, rect: Rect, cell: &DisplayCell) {
    let t = Tokens::get(ui.ctx());
    let inner = rect.shrink2(Vec2::new(8.0, 0.0));
    let font = if cell.mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    let text = design_system::elide_text(ui, &cell.text, &font, inner.width());
    ui.painter().with_clip_rect(inner).text(
        inner.left_center(),
        Align2::LEFT_CENTER,
        text,
        font,
        tone_color(&t, cell.tone),
    );
}

fn paint_elided(
    ui: &Ui,
    rect: Rect,
    text: &str,
    font: egui::FontId,
    color: Color32,
    y_offset: f32,
) {
    let text = design_system::elide_text(ui, text, &font, rect.width());
    ui.painter().with_clip_rect(rect).text(
        Pos2::new(rect.left(), rect.top() + y_offset),
        Align2::LEFT_TOP,
        text,
        font,
        color,
    );
}

fn tone_color(t: &Tokens, tone: MetricTone) -> Color32 {
    match tone {
        MetricTone::Neutral => t.color.text,
        MetricTone::Good => t.color.ok,
        MetricTone::Warning => t.color.warn,
        MetricTone::Error => t.color.err,
    }
}

fn bottom_rule(ui: &Ui, rect: Rect, color: Color32) {
    ui.painter()
        .hline(rect.x_range(), rect.bottom() - 0.5, Stroke::new(1.0, color));
}

fn short_identity(value: &str) -> String {
    const PREFIX: usize = 8;
    const SUFFIX: usize = 6;
    if value.chars().count() <= PREFIX + SUFFIX + 1 {
        return value.to_owned();
    }
    let start = value.chars().take(PREFIX).collect::<String>();
    let end = value
        .chars()
        .rev()
        .take(SUFFIX)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("{start}…{end}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_contract_matches_the_six_mockup_tabs() {
        assert_eq!(SECTION_SPECS.len(), 6);
        assert_eq!(
            SECTION_SPECS.map(|spec| spec.section),
            ModelEditorSection::ALL
        );
        assert_eq!(
            SECTION_SPECS[0].title,
            "Typed parameters, inheritance, units, and bounds"
        );
        assert_eq!(SECTION_SPECS[4].action_label, Some("Qualification plan…"));
        assert_eq!(SECTION_SPECS[5].action_label, None);
    }

    #[test]
    fn column_projection_is_stable_and_exactly_fills_the_row() {
        let columns = [("A", 0.16), ("B", 0.19), ("C", 0.65)];
        let rect = Rect::from_min_size(Pos2::new(11.0, 7.0), Vec2::new(913.0, 28.0));
        let cells = split_columns(rect, &columns);
        assert_eq!(cells.len(), columns.len());
        assert_eq!(cells.first().unwrap().left(), rect.left());
        assert_eq!(cells.last().unwrap().right(), rect.right());
        assert!(
            cells
                .windows(2)
                .all(|pair| pair[0].right() == pair[1].left())
        );
    }

    #[test]
    fn fixed_workspace_geometry_does_not_depend_on_selected_section() {
        for section in ModelEditorSection::ALL {
            assert_eq!(section_spec(section).section, section);
            assert_eq!(NAV_ROW_H, 36.0);
            assert_eq!(SECTION_HEADER_H, 70.0);
            assert_eq!(TABLE_HEADER_H, 27.0);
        }
    }

    #[test]
    fn specialist_header_preserves_the_mockup_ownership_breakpoint() {
        assert_eq!(SPECIALIST_HEADER_H, 82.0);
        assert_eq!(SPECIALIST_HEADER_MEDIUM_H, 124.0);
        assert_eq!(specialist_header_height(820.0), SPECIALIST_HEADER_H);
        assert_eq!(specialist_header_height(821.0), SPECIALIST_HEADER_MEDIUM_H);
        assert_eq!(
            specialist_header_height(1_120.0),
            SPECIALIST_HEADER_MEDIUM_H
        );
        assert_eq!(specialist_header_height(1_121.0), SPECIALIST_HEADER_H);
    }

    #[test]
    fn navigation_scroll_extent_always_contains_all_tabs_and_evidence() {
        const NOTE_H: f32 = 119.0;
        let required = SECTION_SPECS.len() as f32 * NAV_ROW_H + NOTE_H;
        assert_eq!(required, 335.0);
        assert!(
            required > 220.0,
            "short workspaces must scroll, not clip tabs"
        );
    }

    #[test]
    fn identity_elision_preserves_both_collision_significant_ends() {
        assert_eq!(short_identity("short"), "short");
        assert_eq!(
            short_identity("0123456789abcdef0123456789abcdef"),
            "01234567…abcdef"
        );
    }

    #[test]
    fn qualification_progress_combines_suite_and_vector_progress() {
        assert_eq!(qualification_progress_fraction(0, 0, 0, 0), 0.0);
        assert_eq!(qualification_progress_fraction(0, 2, 1, 4), 0.125);
        assert_eq!(qualification_progress_fraction(1, 2, 0, 4), 0.5);
        assert_eq!(qualification_progress_fraction(1, 2, 4, 4), 1.0);
        assert_eq!(qualification_progress_fraction(4, 2, 0, 0), 1.0);
    }

    #[test]
    fn qualification_form_geometry_is_independent_of_combo_choices() {
        assert_eq!(QUALIFICATION_FIELD_ROW_H, 54.0);
        assert_eq!(QUALIFICATION_MULTILINE_ROW_H, 118.0);
        assert_eq!(QUALIFICATION_STATUS_H, 86.0);
        assert_eq!(QUALIFICATION_ERROR_H, 50.0);
    }

    #[test]
    fn qualification_authoring_exposes_every_solver_analysis_domain() {
        let analyses = [
            QualificationAuthoringAnalysis::DcOperatingPoint,
            QualificationAuthoringAnalysis::DcSweep,
            QualificationAuthoringAnalysis::AcSweep,
            QualificationAuthoringAnalysis::Noise,
            QualificationAuthoringAnalysis::Transient,
        ];
        for analysis in analyses {
            assert!(!qualification_authoring_probe_options(analysis).is_empty());
            assert!(!qualification_authoring_sample_options(analysis).is_empty());
        }
        assert_eq!(
            qualification_authoring_probe_options(QualificationAuthoringAnalysis::AcSweep).len(),
            10
        );
        assert_eq!(
            qualification_authoring_probe_options(QualificationAuthoringAnalysis::Noise).len(),
            5
        );
        assert_eq!(
            qualification_authoring_probe_options(QualificationAuthoringAnalysis::Transient).len(),
            3
        );
    }

    #[test]
    fn qualification_authoring_normalizes_probe_and_sample_without_resizing_form() {
        let mut fields = crate::workbench::documents::model_editor::QualificationAuthoringDraft {
            analysis: QualificationAuthoringAnalysis::Noise,
            probe: QualificationAuthoringProbe::NodeVoltage,
            sample: QualificationAuthoringSample::OperatingPoint,
            ..Default::default()
        };
        qualification_normalize_authoring_domains(&mut fields);
        assert_eq!(fields.probe, QualificationAuthoringProbe::FrequencyValue);
        assert_eq!(
            fields.sample,
            QualificationAuthoringSample::FirstFrequencyPoint
        );
        assert!(!qualification_probe_requires_target(fields.probe));
        assert!(!qualification_sample_requires_index(fields.sample));
        assert!(qualification_sample_requires_index(
            QualificationAuthoringSample::FrequencyPoint
        ));
    }

    #[test]
    fn qualification_selection_falls_back_deterministically() {
        let mut state = ModelQualificationState::default();
        state.suites.push(QualificationSuite {
            schema_version: crate::state::model_library::MODEL_QUALIFICATION_SCHEMA_VERSION,
            id: "suite-b".to_owned(),
            name: "Suite B".to_owned(),
            revision: crate::product::ObjectRevision::INITIAL,
            vectors: Vec::new(),
        });
        let mut plan = QualificationPlanUiState {
            selected_suite_id: "missing".to_owned(),
            selected_vector_id: "missing".to_owned(),
            ..Default::default()
        };
        normalize_qualification_selection(&state, &mut plan);
        assert_eq!(plan.selected_suite_id, "suite-b");
        assert!(plan.selected_vector_id.is_empty());
    }

    #[test]
    fn retained_platform_runs_project_before_cross_platform_evidence_exists() {
        let source_digest = crate::product::ContentDigest::from_bytes([0x51; 32]);
        let source_revision = crate::product::ObjectRevision::INITIAL;
        let source_id = crate::product::ModelSourceId::from_namespace(
            uuid::Uuid::from_u128(0x795d_0456_80da_46b3_95f5_a327_aa67_323a),
            b"surface-platform-run",
        );
        let suite = QualificationSuite {
            schema_version: crate::state::model_library::MODEL_QUALIFICATION_SCHEMA_VERSION,
            id: "dc-parity".to_owned(),
            name: "DC parity".to_owned(),
            revision: crate::product::ObjectRevision::INITIAL,
            vectors: Vec::new(),
        };
        let source = crate::state::model_library::ModelSourceEvidenceBinding {
            model_id: "nch_qualified".to_owned(),
            source_id: Some(source_id),
            source_digest,
            source_revision,
        };
        let desktop = QualificationPlatformRun {
            schema_version: crate::state::model_library::MODEL_QUALIFICATION_SCHEMA_VERSION,
            platform: QualificationPlatform::Desktop,
            source,
            suite_id: suite.id.clone(),
            suite_revision: suite.revision,
            vector_outcomes: vec![
                crate::state::model_library::QualificationPlatformVectorOutcome {
                    vector_id: "nominal".to_owned(),
                    input_digest: source_digest,
                    outcome: crate::state::model_library::PlatformQualificationOutcome {
                        platform: QualificationPlatform::Desktop,
                        references: Vec::new(),
                        failure: None,
                        passed: true,
                    },
                },
            ],
            passed: true,
        };
        let mut state = ModelQualificationState::default();
        state.platform_runs.push(desktop);

        let retained_desktop = exact_platform_run(
            &state,
            &suite,
            "NCH_QUALIFIED",
            source_id,
            source_digest,
            source_revision,
            QualificationPlatform::Desktop,
        );
        let missing_wasm = exact_platform_run(
            &state,
            &suite,
            "nch_qualified",
            source_id,
            source_digest,
            source_revision,
            QualificationPlatform::WebAssembly,
        );

        assert!(state.evidence.is_empty());
        assert_eq!(platform_run_summary(retained_desktop), "1 pass");
        assert!(missing_wasm.is_none());
        assert_eq!(platform_run_summary(missing_wasm), "no run");
        assert!(
            exact_platform_run(
                &state,
                &suite,
                "nch_qualified",
                source_id,
                crate::product::ContentDigest::from_bytes([0x52; 32]),
                source_revision,
                QualificationPlatform::Desktop,
            )
            .is_none()
        );
        assert!(
            exact_platform_run(
                &state,
                &suite,
                "nch_qualified",
                crate::product::ModelSourceId::new(),
                source_digest,
                source_revision,
                QualificationPlatform::Desktop,
            )
            .is_none()
        );
    }

    #[test]
    fn cancelling_qualification_authoring_discards_every_partial_field() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.model_editor.begin_qualification_suite();
        let fields = &mut app.state.workbench.model_editor.qualification_authoring;
        fields.suite_id = "partial-suite".to_owned();
        fields.vector_id = "partial-vector".to_owned();
        fields.executable_input = "V1 in 0 1".to_owned();
        fields.analysis = QualificationAuthoringAnalysis::DcSweep;
        fields.probe = QualificationAuthoringProbe::BranchCurrent;
        fields.sample = QualificationAuthoringSample::SweepPoint;
        fields.error = Some("partial error".to_owned());

        cancel_qualification_authoring(&mut app);

        let editor = &app.state.workbench.model_editor;
        assert!(!editor.qualification_authoring_open);
        assert!(editor.qualification_authoring.suite_id.is_empty());
        assert!(editor.qualification_authoring.vector_id.is_empty());
        assert!(editor.qualification_authoring.executable_input.is_empty());
        assert_eq!(
            editor.qualification_authoring.analysis,
            QualificationAuthoringAnalysis::DcOperatingPoint
        );
        assert_eq!(
            editor.qualification_authoring.probe,
            QualificationAuthoringProbe::NodeVoltage
        );
        assert_eq!(
            editor.qualification_authoring.sample,
            QualificationAuthoringSample::OperatingPoint
        );
        assert!(editor.qualification_authoring.error.is_none());
    }
}
