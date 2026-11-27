//! Device-model specialist surface.
//!
//! The surface is a projection over the transactional editor state and the
//! persisted model-definition domain. It never supplies sample model facts
//! when those records are absent.

mod schema_editors;
mod workflows;

use schema_editors::*;
use workflows::*;

use std::collections::{BTreeMap, BTreeSet};

use egui::{Align, Align2, Color32, Layout, Pos2, Rect, ScrollArea, Sense, Stroke, Ui, Vec2};

use crate::state::model_library::{
    LookupInterpolation, ModelDefinitionMetadata, ParameterDefinition, ParameterSource,
    StatisticalDistribution, StatisticalHierarchyScope, TemperatureExtrapolationPolicy,
    TemperatureLawRepresentation,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, Problem, paint_problem_mark};
use crate::workbench::framework::feature::Cx;
use crate::workbench::models::ModelsFeature;

use super::super::design_system::{self, WorkbenchIcon};
use crate::workbench::documents::model_editor::{ModelEditorSection, ModelParameterKind};
use crate::workbench::models::model_editor_workflow::{
    ModelEditorWorkflow, active_model_editor_workflow, close_model_editor_workflow,
    prepare_model_editor_workflow, request_model_editor_workflow, set_model_editor_workflow_error,
};

/// The candidate band: one line, the open candidate's name and whatever is
/// wrong with it. It was 82 px when it also carried a monogram, a kicker, a
/// sentence restating the six sections below it, and a blurb about what the
/// document owns.
const SPECIALIST_HEADER_H: f32 = 46.0;
const OUTER_IDENTITY_H: f32 = 60.0;
const NAV_W: f32 = 214.0;
const NAV_ROW_H: f32 = 36.0;
/// The section band: its kicker line and the section's title. It was 70 px
/// when a sentence under the title described what the section below it is
/// for, and the rule has come up by exactly that sentence's ink and lead, so
/// the title keeps the padding under it that it always had.
const SECTION_HEADER_H: f32 = 51.0;
const TABLE_HEADER_H: f32 = 27.0;
const TABLE_MIN_W: f32 = 760.0;

/// Side of a problem mark painted in front of one of this surface's values,
/// and the advance from its left edge to where the text after it begins.
///
/// Read from the kit, which is what makes a marked line and an unmarked one
/// beside it differ by exactly the advance.
use crate::ui::widgets::{PROBLEM_MARK_ADVANCE as MARK_ADVANCE, PROBLEM_MARK_SIDE as MARK_SIDE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SurfaceAction {
    SelectSection(ModelEditorSection),
    OpenParameterSchema,
    OpenNewSection,
    OpenCorrelationMatrix,
    OpenTemperaturePreview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SectionSpec {
    section: ModelEditorSection,
    title: &'static str,
    action_label: Option<&'static str>,
}

const SECTION_SPECS: [SectionSpec; 4] = [
    SectionSpec {
        section: ModelEditorSection::Parameters,
        title: "Typed parameters, inheritance, units, and bounds",
        action_label: Some("Parameter schema…"),
    },
    SectionSpec {
        section: ModelEditorSection::Sections,
        title: "Process sections and inheritance graph",
        action_label: Some("New section…"),
    },
    SectionSpec {
        section: ModelEditorSection::Statistics,
        title: "Process, mismatch, hierarchy, and correlation groups",
        action_label: Some("Correlation matrix…"),
    },
    SectionSpec {
        section: ModelEditorSection::Temperature,
        title: "Temperature equations, interpolation, and valid range",
        action_label: Some("Plot temperature law…"),
    },
];

#[derive(Debug, Clone)]
struct PersistedModelRecords {
    metadata: Option<ModelDefinitionMetadata>,
    metadata_error: Option<String>,
}

/// What a verdict column says about one row.
///
/// Only a column whose job is to judge its rows reaches for a tone, and only
/// for the two verdicts that need acting on. A row that passed is painted
/// exactly like a row that states a fact, because a passing row is the normal
/// case and the normal case is not decorated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MetricTone {
    Neutral,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DisplayCell {
    text: String,
    mono: bool,
    tone: MetricTone,
    mark: Option<Problem>,
}

impl DisplayCell {
    fn plain(value: impl Into<String>) -> Self {
        Self {
            text: value.into(),
            mono: false,
            tone: MetricTone::Neutral,
            mark: None,
        }
    }

    fn mono(value: impl Into<String>) -> Self {
        Self {
            text: value.into(),
            mono: true,
            tone: MetricTone::Neutral,
            mark: None,
        }
    }

    /// A verdict cell, in the tone its column judges by.
    fn toned(value: impl Into<String>, tone: MetricTone) -> Self {
        Self {
            text: value.into(),
            mono: false,
            tone,
            mark: None,
        }
    }
}

impl ModelsFeature {
    /// Draw the device-model editor route's surface in the feature's
    /// context, with the workflow and inspection dialogs it opens.
    pub fn show_model_editor(cx: &mut Cx<'_, ModelsFeature>, ui: &mut Ui) {
        show(ui, cx);
    }
}

fn show(ui: &mut Ui, cx: &mut Cx<'_, ModelsFeature>) {
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

    let records = persisted_records(cx);
    let current_section = cx.state.model_editor.active_section;
    let mut requested_action = None;

    outer_identity_header(&mut surface, cx);
    specialist_header(&mut surface, cx, &records);

    if cx.state.model_editor.draft.is_none() {
        design_system::empty_state(
            &mut surface,
            WorkbenchIcon::Sliders,
            "No project-owned model candidate is open",
            "Open an editable project model from Model & PDK catalog. Built-in and external model sources remain read-only.",
        );
    } else {
        workspace_body(
            &mut surface,
            cx,
            &records,
            current_section,
            &mut requested_action,
        );
    }

    if let Some(action) = requested_action {
        apply_action(cx, action);
    }
    if active_model_editor_workflow(cx.state.model_editor_reads(cx.model, cx.platform)).is_some() {
        show_model_editor_workflow(ui.ctx(), cx, &records);
    } else {
        show_inspection_dialogs(ui.ctx(), cx, &records);
    }
}

fn outer_identity_header(ui: &mut Ui, cx: &Cx<'_, ModelsFeature>) {
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
        "Device model and parameter editor",
        theme::sans(14.0, FontWeight::SemiBold),
        t.color.text,
        21.0,
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
    let enabled = cx.state.model_editor.draft.is_some();
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
        request_model_editor_workflow(
            cx.state.model_editor_reads(cx.model, cx.platform),
            ModelEditorWorkflow::ValidateCandidate,
        );
    }
}

fn persisted_records(cx: &Cx<'_, ModelsFeature>) -> PersistedModelRecords {
    let Some(draft) = cx.state.model_editor.draft.as_ref() else {
        return PersistedModelRecords {
            metadata: None,
            metadata_error: None,
        };
    };
    let metadata = Some(draft.metadata.clone());
    let metadata_error = draft.definition().err().map(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.field, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ")
    });
    PersistedModelRecords {
        metadata,
        metadata_error,
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
        response.on_disabled_hover_text(blocker)
    } else {
        response
    };
    if enabled && response.clicked() {
        *action = Some(value);
    }
}

fn specialist_header(ui: &mut Ui, cx: &Cx<'_, ModelsFeature>, records: &PersistedModelRecords) {
    let t = Tokens::get(ui.ctx());
    let available_width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(available_width, SPECIALIST_HEADER_H),
        Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    bottom_rule(ui, rect, t.color.border);

    let left = rect.left() + 14.0;
    let status_w = 174.0_f32.min(rect.width() * 0.26);
    let status_left = rect.right() - status_w - 10.0;
    let text_right = (status_left - 8.0).max(left + 80.0);
    let text_clip = Rect::from_min_max(
        Pos2::new(left, rect.top()),
        Pos2::new(text_right, rect.bottom()),
    );
    let title = cx.state.model_editor.draft.as_ref().map_or_else(
        || "No model candidate".to_owned(),
        |draft| format!("{} model candidate", draft.name),
    );
    let title_font = theme::sans(14.0, FontWeight::SemiBold);
    let title = design_system::elide_text(ui, &title, &title_font, text_clip.width().max(1.0));
    ui.painter().with_clip_rect(text_clip).text(
        Pos2::new(left, rect.center().y),
        Align2::LEFT_CENTER,
        title,
        title_font,
        t.color.text,
    );

    let Some((status, problem)) = candidate_status(cx, records) else {
        return;
    };
    let status_rect = Rect::from_min_max(
        Pos2::new(status_left, rect.top()),
        Pos2::new(rect.right() - 14.0, rect.bottom()),
    );
    let status_y = rect.center().y;
    let mut text_left = status_rect.left();
    let mut available = (status_rect.width() - 1.0).max(1.0);
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                Pos2::new(text_left + MARK_SIDE * 0.5, status_y),
                Vec2::splat(MARK_SIDE),
            ),
            problem,
        );
        text_left += MARK_ADVANCE;
        available = (available - MARK_ADVANCE).max(1.0);
    }
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let label = design_system::elide_text(ui, &status, &font, available);
    ui.painter().text(
        Pos2::new(text_left, status_y),
        Align2::LEFT_CENTER,
        label,
        font,
        t.color.text_dim,
    );
}

/// What the header says about the open candidate, and `None` when there is
/// nothing to say: a validated candidate is the normal state, and the normal
/// state is not announced.
fn candidate_status(
    cx: &Cx<'_, ModelsFeature>,
    records: &PersistedModelRecords,
) -> Option<(String, Option<Problem>)> {
    if let Some(error) = records.metadata_error.as_ref() {
        return Some((
            format!("metadata invalid · {error}"),
            Some(Problem::Failure),
        ));
    }
    let editor = &cx.state.model_editor;
    if let Some(diagnostic) = editor.diagnostics.first() {
        return Some((diagnostic.message.clone(), Some(Problem::Failure)));
    }
    if editor.validation.is_some() {
        return None;
    }
    Some(("Not validated".to_owned(), None))
}

/// Section overrides count with the model-level overrides because both replace
/// an inherited typed value; the split is over the persisted schema, not the
/// draft, so an unsaved edit never moves a parameter between the three.
fn parameter_source_split(metadata: &ModelDefinitionMetadata) -> (usize, usize, usize) {
    let (declared, inherited, model_overrides) =
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
            });
    let section_overrides = metadata
        .sections
        .iter()
        .map(|section| section.overrides.len())
        .sum::<usize>();
    (declared, inherited, model_overrides + section_overrides)
}

/// The one scalar a rail entry's own section owns. A section whose page states
/// its counts per row carries none, so the rail never restates a table column.
fn navigation_meta(
    cx: &Cx<'_, ModelsFeature>,
    records: &PersistedModelRecords,
    section: ModelEditorSection,
) -> Option<String> {
    match section {
        ModelEditorSection::Parameters => {
            // Without persisted schema the open source's own values are the
            // only parameters the section can list, so they are what it counts.
            let count = records.metadata.as_ref().map_or_else(
                || {
                    cx.state
                        .model_editor
                        .draft
                        .as_ref()
                        .map_or(0, |draft| draft.parameters.len())
                },
                |metadata| metadata.parameters.len(),
            );
            Some(count.to_string())
        }
        ModelEditorSection::Sections => Some(
            records
                .metadata
                .as_ref()
                .map_or(0, |metadata| metadata.sections.len())
                .to_string(),
        ),
        ModelEditorSection::Statistics | ModelEditorSection::Temperature => None,
    }
}

/// The aggregate a section owns but cannot state in any one of its rows. A fact
/// the rows already carry is never repeated in the header.
fn section_header_fact(
    records: &PersistedModelRecords,
    section: ModelEditorSection,
) -> Option<String> {
    match section {
        ModelEditorSection::Parameters => {
            let metadata = records.metadata.as_ref()?;
            let (declared, inherited, overridden) = parameter_source_split(metadata);
            Some(format!(
                "{declared} declared · {inherited} inherited · {overridden} override"
            ))
        }
        ModelEditorSection::Sections
        | ModelEditorSection::Statistics
        | ModelEditorSection::Temperature => None,
    }
}

fn workspace_body(
    ui: &mut Ui,
    cx: &mut Cx<'_, ModelsFeature>,
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
    render_navigation(&mut nav, cx, records, current_section, action);

    let mut content = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(content_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    content.spacing_mut().item_spacing = Vec2::ZERO;
    render_section(&mut content, cx, records, current_section, action);
}

fn render_navigation(
    ui: &mut Ui,
    cx: &Cx<'_, ModelsFeature>,
    records: &PersistedModelRecords,
    current: ModelEditorSection,
    action: &mut Option<SurfaceAction>,
) {
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
                // The entry's own count is laid out first: the label is elided
                // against what the meta leaves, never painted under it.
                let meta = navigation_meta(cx, records, spec.section);
                let accessible = meta.as_ref().map_or_else(
                    || spec.section.label().to_owned(),
                    |value| format!("{} · {value}", spec.section.label()),
                );
                let meta = meta.map(|value| {
                    ui.painter().layout_no_wrap(
                        value,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        t.color.text_dim,
                    )
                });
                let label_font = theme::sans(
                    tokens::FS_0,
                    if active {
                        FontWeight::SemiBold
                    } else {
                        FontWeight::Regular
                    },
                );
                let meta_w = meta
                    .as_ref()
                    .map_or(0.0, |galley| galley.rect.width() + 10.0);
                let label = design_system::elide_text(
                    ui,
                    spec.section.label(),
                    &label_font,
                    (rect.width() - 44.0 - 11.0 - meta_w).max(1.0),
                );
                ui.painter().text(
                    Pos2::new(rect.left() + 44.0, rect.center().y),
                    Align2::LEFT_CENTER,
                    label,
                    label_font,
                    if active {
                        t.color.text
                    } else {
                        t.color.text_dim
                    },
                );
                if let Some(galley) = meta {
                    ui.painter().galley(
                        Pos2::new(
                            rect.right() - 11.0 - galley.rect.width(),
                            rect.center().y - galley.rect.height() * 0.5,
                        ),
                        galley,
                        t.color.text_dim,
                    );
                }
                theme::paint_focus_ring(ui, &response, rect);
                response.widget_info(|| {
                    egui::WidgetInfo::selected(
                        egui::WidgetType::Button,
                        true,
                        active,
                        accessible.clone(),
                    )
                });
                if response.clicked() && !active {
                    *action = Some(SurfaceAction::SelectSection(spec.section));
                }
            }
        });
}

fn render_section(
    ui: &mut Ui,
    cx: &mut Cx<'_, ModelsFeature>,
    records: &PersistedModelRecords,
    section: ModelEditorSection,
    action: &mut Option<SurfaceAction>,
) {
    let spec = section_spec(section);
    section_header(ui, cx, spec, records, action);
    let height = ui.available_height().max(1.0);
    ScrollArea::vertical()
        .id_salt(("model-editor-section", section.label()))
        .auto_shrink([false, false])
        .max_height(height)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            match section {
                ModelEditorSection::Parameters => parameters_table(ui, cx, records),
                ModelEditorSection::Sections => sections_table(ui, records),
                ModelEditorSection::Statistics => statistics_table(ui, records),
                ModelEditorSection::Temperature => temperature_table(ui, records),
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
    cx: &Cx<'_, ModelsFeature>,
    spec: &SectionSpec,
    records: &PersistedModelRecords,
    action: &mut Option<SurfaceAction>,
) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
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
    // The section's aggregate shares the kicker line: it is elided first so the
    // kicker gives up width to it rather than overprinting it.
    let fact = section_header_fact(records, spec.section);
    let fact_galley = fact.as_ref().map(|text| {
        let font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let text = design_system::elide_text(ui, text, &font, (copy_rect.width() * 0.66).max(1.0));
        ui.painter().layout_no_wrap(text, font, t.color.text_dim)
    });
    let kicker_rect = Rect::from_min_max(
        copy_rect.min,
        Pos2::new(
            copy_rect.right()
                - fact_galley
                    .as_ref()
                    .map_or(0.0, |galley| galley.rect.width() + 12.0),
            copy_rect.bottom(),
        ),
    );
    paint_elided(
        ui,
        kicker_rect,
        &format!("Model · {}", spec.section.label()),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
        9.0,
    );
    if let Some(galley) = fact_galley {
        ui.painter().with_clip_rect(copy_rect).galley(
            Pos2::new(
                copy_rect.right() - galley.rect.width(),
                copy_rect.top() + 9.0,
            ),
            galley,
            t.color.text_dim,
        );
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(match fact.as_ref() {
            Some(fact) => format!("{}. {}. {fact}", spec.section.label(), spec.title),
            None => format!("{}. {}", spec.section.label(), spec.title),
        });
    });
    paint_elided(
        ui,
        copy_rect,
        spec.title,
        theme::sans(13.0, FontWeight::SemiBold),
        t.color.text,
        29.0,
    );

    let Some(label) = spec.action_label else {
        return;
    };
    let (mut enabled, mut blocker, value) = section_action_state(spec.section, records);
    if cx.model.safe_mode.project_read_only()
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
    }
}

fn parameters_table(ui: &mut Ui, cx: &mut Cx<'_, ModelsFeature>, records: &PersistedModelRecords) {
    const COLUMNS: [(&str, f32); 6] = [
        ("Parameter", 0.16),
        ("Value", 0.19),
        ("Unit", 0.10),
        ("Source", 0.20),
        ("Bounds", 0.17),
        ("Status", 0.18),
    ];
    let metadata = records.metadata.as_ref().map(parameter_metadata_index);
    let diagnostics = cx.state.model_editor.diagnostics.clone();
    let changed = cx
        .state
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
    let selected = cx.state.model_editor.selected_parameter;
    let read_only = cx.model.safe_mode.project_read_only();
    let mut changed_value = false;
    let mut selected_parameter = selected;

    ScrollArea::horizontal()
        .id_salt("model-editor-parameters-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(TABLE_MIN_W);
            table_header(ui, width, &COLUMNS);
            let Some(draft) = cx.state.model_editor.draft.as_mut() else {
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
                // Only the rejection is attached to the field it rejects. A
                // parameter that reads back exactly as the schema declares it
                // leaves the cell empty: "declared" on every untouched row is
                // a constant, and the rows that differ are what the column is
                // for.
                let status = diagnostic.map_or_else(
                    || {
                        if meta.is_none() {
                            "schema unavailable".to_owned()
                        } else if changed.contains(&parameter.name.to_lowercase()) {
                            "changed".to_owned()
                        } else {
                            String::new()
                        }
                    },
                    |value| value.message.clone(),
                );
                let tone = if diagnostic.is_some() {
                    MetricTone::Error
                } else {
                    MetricTone::Neutral
                };
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
    cx.state.model_editor.selected_parameter = selected_parameter;
    if changed_value {
        cx.state.model_editor.invalidate_candidate_evidence();
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
    const COLUMNS: [(&str, f32); 4] = [
        ("Section", 0.20),
        ("Parent", 0.20),
        ("Overrides", 0.16),
        ("Model files", 0.44),
    ];
    let rows = records.metadata.as_ref().map_or_else(Vec::new, |metadata| {
        metadata
            .sections
            .iter()
            .map(|section| {
                vec![
                    DisplayCell::mono(&section.name),
                    DisplayCell::plain(section.parent.as_deref().unwrap_or("base")),
                    DisplayCell::mono(section.overrides.len().to_string()),
                    DisplayCell::plain(model_files_label(&section.model_files)),
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

fn apply_action(cx: &mut Cx<'_, ModelsFeature>, action: SurfaceAction) {
    match action {
        SurfaceAction::SelectSection(section) => {
            cx.state.model_editor.active_section = section;
        }
        SurfaceAction::OpenParameterSchema => {
            cx.state.model_editor.begin_parameter_schema();
        }
        SurfaceAction::OpenNewSection => {
            cx.state.model_editor.begin_new_section();
        }
        SurfaceAction::OpenCorrelationMatrix => {
            cx.state.model_editor.begin_correlation_matrix_edit();
        }
        SurfaceAction::OpenTemperaturePreview => {
            cx.state.model_editor.temperature_preview_open = true;
        }
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
    id: impl std::hash::Hash + std::fmt::Debug,
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
        paint_cell(ui, cell, &DisplayCell::plain(*label));
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
    let mut text_left = inner.left();
    let mut available = inner.width();
    if let Some(problem) = cell.mark {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                Pos2::new(text_left + MARK_SIDE * 0.5, inner.center().y),
                Vec2::splat(MARK_SIDE),
            ),
            problem,
        );
        text_left += MARK_ADVANCE;
        available = (available - MARK_ADVANCE).max(1.0);
    }
    let text = design_system::elide_text(ui, &cell.text, &font, available);
    ui.painter().with_clip_rect(inner).text(
        Pos2::new(text_left, inner.center().y),
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
mod tests;
