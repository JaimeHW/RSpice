//! SPECS — the active retained dataset's governed measurement results.
//!
//! Each row binds an authored `.MEAS` expression and project-owned limit to
//! the exact retained value and worst source in the active immutable dataset.
//! Ambiguous lineage, missing results, and invalid evaluations fail closed;
//! the viewer and inspector share this same projection. Bounds and authored
//! expressions persist with the workspace ([`SpecEntry`]), while the docbar
//! opens their inline editor.

use std::collections::HashSet;

use egui::Ui;

use crate::quantity::engineering::parse_engineering_value;
use crate::state::{
    AnalysisResultFamilyMetadata, AnalysisType, SimulationRun, SimulationRunLifecycle, SpecEntry,
};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;
use crate::workbench::design_system::{StatusMark, WorkbenchIcon, icon_button, paint_status_mark};

use super::{ResultViewer, well_hint};

const SPEC_COLUMN_WIDTHS: [f32; 7] = [160.0, 200.0, 116.0, 150.0, 140.0, 168.0, 92.0];
const SPEC_TABLE_GUTTER: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpecResultStatus {
    Pass,
    Fail,
    Unbound,
    Missing,
    Invalid,
}

impl SpecResultStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Unbound => "no spec",
            Self::Missing => "no result",
            Self::Invalid => "invalid",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SpecResultRow {
    measurement: String,
    expression: String,
    value: Option<f64>,
    limit: String,
    margin: Option<f64>,
    unit: String,
    is_bounded: bool,
    source_analysis_index: Option<usize>,
    worst_corner: Option<String>,
    status: SpecResultStatus,
    detail: String,
}

#[derive(Debug, Clone, Copy)]
struct MeasurementCandidate<'a> {
    analysis_index: usize,
    analysis: &'a crate::state::AnalysisResult,
    value: Option<f64>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct SpecSummary {
    passing: usize,
    bounded: usize,
    failures: usize,
    unavailable: usize,
}

fn summarize_rows(rows: &[SpecResultRow]) -> SpecSummary {
    SpecSummary {
        passing: rows
            .iter()
            .filter(|row| row.is_bounded && row.status == SpecResultStatus::Pass)
            .count(),
        bounded: rows.iter().filter(|row| row.is_bounded).count(),
        failures: rows
            .iter()
            .filter(|row| row.is_bounded && row.status == SpecResultStatus::Fail)
            .count(),
        unavailable: rows
            .iter()
            .filter(|row| {
                row.is_bounded
                    && matches!(
                        row.status,
                        SpecResultStatus::Missing | SpecResultStatus::Invalid
                    )
            })
            .count(),
    }
}

/// One spec row being edited, kept as text so partial input survives
/// frames. Parsed (engineering notation allowed) on apply.
#[derive(Debug, Clone, Default)]
pub struct SpecDraft {
    pub measurement: String,
    pub expression: String,
    pub min: String,
    pub max: String,
    pub unit: String,
}

impl SpecDraft {
    fn from_entry(entry: &SpecEntry) -> Self {
        let fmt = |bound: Option<f64>| bound.map(|v| format!("{v}")).unwrap_or_default();
        Self {
            measurement: entry.measurement.clone(),
            expression: entry.expression.clone(),
            min: fmt(entry.min),
            max: fmt(entry.max),
            unit: entry.unit.clone(),
        }
    }

    /// Parse into a spec entry. `Err` carries the offending field label.
    fn parse(&self) -> Result<Option<SpecEntry>, &'static str> {
        let name = self.measurement.trim();
        if name.is_empty() {
            return Ok(None); // blank rows are simply dropped
        }
        let bound = |text: &str, field| -> Result<Option<f64>, &'static str> {
            let text = text.trim();
            if text.is_empty() {
                return Ok(None);
            }
            parse_engineering_value(text).map(Some).map_err(|_| field)
        };
        Ok(Some(SpecEntry {
            measurement: name.to_owned(),
            expression: self.expression.trim().to_owned(),
            min: bound(&self.min, "min")?,
            max: bound(&self.max, "max")?,
            unit: self.unit.trim().to_owned(),
        }))
    }
}

/// Measurements present in the active dataset but not yet bound by the
/// active requirement contract. The editor never discovers names from a
/// different historical dataset.
fn untracked_measurements(state: &AppState) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    if let Some(run) = state.simulation.active_run() {
        for analysis in &run.analyses {
            for measurement in &analysis.measurements {
                if seen.insert(measurement.name.to_ascii_lowercase()) {
                    names.push(measurement.name.clone());
                }
            }
        }
    }
    names
        .into_iter()
        .filter(|name| {
            !state
                .workspace
                .specs
                .iter()
                .any(|spec| spec.measurement.eq_ignore_ascii_case(name))
        })
        .collect()
}

fn signed_margin(spec: &SpecEntry, value: f64) -> Option<f64> {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => Some((value - minimum).min(maximum - value)),
        (Some(minimum), None) => Some(value - minimum),
        (None, Some(maximum)) => Some(maximum - value),
        (None, None) => None,
    }
}

fn exact_corner_label(analysis: &crate::state::AnalysisResult) -> Option<String> {
    match analysis.family_metadata.as_ref() {
        Some(AnalysisResultFamilyMetadata::Corner { corner_labels, .. })
            if corner_labels.len() == 1 =>
        {
            corner_labels.first().cloned()
        }
        _ => None,
    }
}

fn measurement_candidates<'a>(run: &'a SimulationRun, name: &str) -> Vec<MeasurementCandidate<'a>> {
    run.analyses
        .iter()
        .enumerate()
        .flat_map(|(analysis_index, analysis)| {
            analysis
                .measurements
                .iter()
                .filter(move |measurement| measurement.name.eq_ignore_ascii_case(name))
                .map(move |measurement| MeasurementCandidate {
                    analysis_index,
                    analysis,
                    value: measurement.value.filter(|value| value.is_finite()),
                })
        })
        .collect()
}

fn candidates_share_source_lineage(candidates: &[MeasurementCandidate<'_>]) -> bool {
    if candidates.len() <= 1 {
        return true;
    }

    let first = candidates[0].analysis;
    if let Some(first_provenance) = first.provenance() {
        return candidates.iter().all(|candidate| {
            candidate.analysis.provenance().is_some_and(|provenance| {
                provenance.authored_source_instance_id()
                    == first_provenance.authored_source_instance_id()
                    && provenance.source_revision() == first_provenance.source_revision()
            })
        });
    }

    // Migrated corner results can predate prepared-task provenance. Repeated
    // result identity plus typed corner metadata is the only exact lineage
    // evidence available; ordinary legacy analyses remain ambiguous.
    matches!(
        first.family_metadata,
        Some(AnalysisResultFamilyMetadata::Corner { .. })
    ) && candidates.iter().all(|candidate| {
        candidate.analysis.id == first.id
            && candidate.analysis.analysis_type == first.analysis_type
            && matches!(
                candidate.analysis.family_metadata,
                Some(AnalysisResultFamilyMetadata::Corner { .. })
            )
    })
}

fn result_row(run: &SimulationRun, measurement: String, spec: Option<&SpecEntry>) -> SpecResultRow {
    let candidates = measurement_candidates(run, &measurement);
    let limit = spec.map_or_else(|| "—".to_owned(), rail_text);
    let unit = spec.map_or_else(String::new, |entry| entry.unit.clone());
    let expression = spec.map_or_else(String::new, |entry| entry.expression.clone());
    let is_bounded = spec.is_some_and(|entry| entry.min.is_some() || entry.max.is_some());
    if candidates.is_empty() {
        return SpecResultRow {
            measurement,
            expression,
            value: None,
            limit,
            margin: None,
            unit,
            is_bounded,
            source_analysis_index: None,
            worst_corner: None,
            status: SpecResultStatus::Missing,
            detail: "No retained analysis in this dataset evaluated the measurement.".to_owned(),
        };
    }

    let Some(spec) = spec.filter(|entry| entry.min.is_some() || entry.max.is_some()) else {
        if candidates.len() != 1 {
            return SpecResultRow {
                measurement,
                expression,
                value: None,
                limit,
                margin: None,
                unit,
                is_bounded,
                source_analysis_index: None,
                worst_corner: None,
                status: SpecResultStatus::Invalid,
                detail: format!(
                    "{} retained analyses publish this unbound measurement; select a source before treating one value as authoritative.",
                    candidates.len()
                ),
            };
        }
        let candidate = candidates[0];
        return SpecResultRow {
            measurement,
            expression,
            value: candidate.value,
            limit,
            margin: None,
            unit,
            is_bounded,
            source_analysis_index: Some(candidate.analysis_index),
            worst_corner: exact_corner_label(candidate.analysis),
            status: if candidate.value.is_some() {
                SpecResultStatus::Unbound
            } else {
                SpecResultStatus::Invalid
            },
            detail: if candidate.value.is_some() {
                "A retained value exists, but no requirement bound is configured.".to_owned()
            } else {
                "The retained measurement evaluation failed.".to_owned()
            },
        };
    };

    if !candidates_share_source_lineage(&candidates) {
        return SpecResultRow {
            measurement,
            expression,
            value: None,
            limit,
            margin: None,
            unit,
            is_bounded,
            source_analysis_index: None,
            worst_corner: None,
            status: SpecResultStatus::Invalid,
            detail: format!(
                "{} retained analyses publish this measurement from different or unproven source lineages; bind one source before evaluating the requirement.",
                candidates.len()
            ),
        };
    }

    if let Some(candidate) = candidates
        .iter()
        .find(|candidate| candidate.value.is_none())
    {
        return SpecResultRow {
            measurement,
            expression,
            value: None,
            limit,
            margin: None,
            unit,
            is_bounded,
            source_analysis_index: Some(candidate.analysis_index),
            worst_corner: exact_corner_label(candidate.analysis),
            status: SpecResultStatus::Invalid,
            detail: "At least one retained measurement evaluation failed; no numeric verdict was invented."
                .to_owned(),
        };
    }

    let worst_margin = candidates
        .iter()
        .filter_map(|candidate| candidate.value.and_then(|value| signed_margin(spec, value)))
        .min_by(f64::total_cmp);
    let Some(worst_margin) = worst_margin else {
        return SpecResultRow {
            measurement,
            expression,
            value: None,
            limit,
            margin: None,
            unit,
            is_bounded,
            source_analysis_index: None,
            worst_corner: None,
            status: SpecResultStatus::Invalid,
            detail: "The configured requirement has no evaluable numeric bound.".to_owned(),
        };
    };
    let mut worst = candidates.iter().filter(|candidate| {
        candidate
            .value
            .and_then(|value| signed_margin(spec, value))
            .is_some_and(|margin| margin.total_cmp(&worst_margin).is_eq())
    });
    let candidate = *worst.next().expect("minimum came from one candidate");
    let source_is_unique = worst.next().is_none();
    let value = candidate.value;
    SpecResultRow {
        measurement,
        expression,
        value,
        limit,
        margin: Some(worst_margin),
        unit,
        is_bounded,
        source_analysis_index: source_is_unique.then_some(candidate.analysis_index),
        worst_corner: source_is_unique
            .then(|| exact_corner_label(candidate.analysis))
            .flatten(),
        status: if worst_margin >= 0.0 {
            SpecResultStatus::Pass
        } else {
            SpecResultStatus::Fail
        },
        detail: if source_is_unique {
            "Worst retained value for the active dataset.".to_owned()
        } else {
            "Several retained sources tie for the worst margin; no corner identity was guessed."
                .to_owned()
        },
    }
}

fn result_rows(run: &SimulationRun, specs: &[SpecEntry]) -> Vec<SpecResultRow> {
    let mut rows = Vec::new();
    let mut seen = HashSet::new();
    for spec in specs {
        if seen.insert(spec.measurement.to_ascii_lowercase()) {
            rows.push(result_row(run, spec.measurement.clone(), Some(spec)));
        }
    }
    for analysis in &run.analyses {
        for measurement in &analysis.measurements {
            if seen.insert(measurement.name.to_ascii_lowercase()) {
                rows.push(result_row(run, measurement.name.clone(), None));
            }
        }
    }
    rows
}

fn lifecycle_label(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy authority",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "streaming",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "immutable",
        SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "aborted",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

fn table_width() -> f32 {
    SPEC_COLUMN_WIDTHS.iter().sum::<f32>() + SPEC_TABLE_GUTTER
}

fn column_rect(row: egui::Rect, index: usize) -> egui::Rect {
    let left = row.left() + SPEC_COLUMN_WIDTHS[..index].iter().sum::<f32>();
    egui::Rect::from_min_size(
        egui::pos2(left, row.top()),
        egui::vec2(SPEC_COLUMN_WIDTHS[index], row.height()),
    )
}

fn spec_table_row_height(control_height: f32) -> f32 {
    control_height.max(28.0)
}

fn paint_clipped_table_text(
    ui: &Ui,
    clip_rect: egui::Rect,
    align: egui::Align2,
    text: impl ToString,
    font: egui::FontId,
    color: egui::Color32,
) {
    if clip_rect.width() <= 0.0 || clip_rect.height() <= 0.0 {
        return;
    }
    let position = if align == egui::Align2::RIGHT_CENTER {
        clip_rect.right_center()
    } else {
        clip_rect.left_center()
    };
    ui.painter()
        .with_clip_rect(clip_rect)
        .text(position, align, text, font, color);
}

fn value_text(row: &SpecResultRow) -> String {
    row.value
        .map_or_else(|| "—".to_owned(), |value| fmt_si(value, &row.unit, 4))
}

fn margin_text(row: &SpecResultRow) -> String {
    row.margin.map_or_else(
        || {
            if row.status == SpecResultStatus::Unbound {
                "unbound".to_owned()
            } else {
                "—".to_owned()
            }
        },
        |margin| fmt_si(margin, &row.unit, 4),
    )
}

fn row_accessibility_label(row: &SpecResultRow) -> String {
    format!(
        "Measurement {}; expression {}; value {}; limit {}; margin {}; worst corner {}; status {}; {}",
        row.measurement,
        if row.expression.is_empty() {
            "not retained"
        } else {
            &row.expression
        },
        value_text(row),
        row.limit,
        margin_text(row),
        row.worst_corner.as_deref().unwrap_or("not available"),
        row.status.label(),
        row.detail
    )
}

fn paint_table_header(ui: &mut Ui, content_width: f32, row_height: f32) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (header, response) =
        ui.allocate_exact_size(egui::vec2(content_width, row_height), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            "Specification result columns: measurement, expression, value, spec, margin, worst corner, status",
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label("Specification result column headers");
    });
    ui.painter().rect_filled(header, 0.0, c.bg_elevated);
    ui.painter().hline(
        header.x_range(),
        header.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border_strong),
    );
    for (index, label) in [
        "MEASUREMENT",
        "EXPRESSION",
        "VALUE",
        "SPEC",
        "MARGIN",
        "WORST CORNER",
        "STATUS",
    ]
    .iter()
    .enumerate()
    {
        let cell = column_rect(header, index);
        let cell_response = ui.interact(cell, response.id.with(index), egui::Sense::hover());
        cell_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), *label)
        });
        ui.ctx().accesskit_node_builder(cell_response.id, |node| {
            node.set_role(egui::accesskit::Role::ColumnHeader);
            node.set_label(*label);
        });
        paint_clipped_table_text(
            ui,
            cell.shrink2(egui::vec2(9.0, 0.0)),
            if matches!(index, 2 | 4) {
                egui::Align2::RIGHT_CENTER
            } else {
                egui::Align2::LEFT_CENTER
            },
            label,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }
}

fn paint_result_row(
    ui: &mut Ui,
    row: &SpecResultRow,
    row_index: usize,
    content_width: f32,
    row_height: f32,
    max_margin: f64,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(content_width, row_height), egui::Sense::hover());
    let accessible_label = row_accessibility_label(row);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(accessible_label);
    });
    if row_index % 2 == 1 {
        ui.painter()
            .rect_filled(rect, 0.0, c.bg_panel.gamma_multiply(0.35));
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border.gamma_multiply(0.65)),
    );

    let expression = if row.expression.is_empty() {
        "—"
    } else {
        &row.expression
    };
    let value = value_text(row);
    let margin = margin_text(row);
    let cell_values = [
        row.measurement.as_str(),
        expression,
        value.as_str(),
        row.limit.as_str(),
        margin.as_str(),
        row.worst_corner.as_deref().unwrap_or("—"),
        row.status.label(),
    ];
    for (index, label) in cell_values.iter().enumerate() {
        if index == 5 && row.source_analysis_index.is_some() {
            continue;
        }
        let cell = column_rect(rect, index);
        let cell_response = ui.interact(
            cell,
            response.id.with(("cell", index)),
            egui::Sense::hover(),
        );
        cell_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), *label)
        });
        ui.ctx().accesskit_node_builder(cell_response.id, |node| {
            node.set_role(egui::accesskit::Role::Cell);
            node.set_label(*label);
        });
    }

    paint_clipped_table_text(
        ui,
        column_rect(rect, 0).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        &row.measurement,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        c.text,
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, 1).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        expression,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if row.expression.is_empty() {
            c.text_faint
        } else {
            c.text_dim
        },
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, 2).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::RIGHT_CENTER,
        &value,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        if row.status == SpecResultStatus::Fail {
            c.err
        } else {
            c.text
        },
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, 3).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        &row.limit,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_dim,
    );

    let margin_cell = column_rect(rect, 4).shrink2(egui::vec2(9.0, 5.0));
    if let Some(value) = row.margin {
        let fraction = (value.abs() / max_margin).clamp(0.04, 1.0) as f32;
        let color = if value >= 0.0 { c.ok } else { c.err };
        ui.painter().rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(margin_cell.left(), margin_cell.bottom() - 3.0),
                egui::vec2(margin_cell.width() * fraction, 3.0),
            ),
            0.0,
            color,
        );
        paint_clipped_table_text(
            ui,
            margin_cell,
            egui::Align2::RIGHT_CENTER,
            &margin,
            theme::mono(tokens::FS_1, FontWeight::Regular),
            color,
        );
    } else {
        paint_clipped_table_text(
            ui,
            margin_cell,
            egui::Align2::RIGHT_CENTER,
            &margin,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    let corner_cell = column_rect(rect, 5).shrink2(egui::vec2(6.0, 2.0));
    let focused = if let Some(analysis_index) = row.source_analysis_index {
        let label = row.worst_corner.as_deref().unwrap_or("Open source");
        let button = ui.put(
            corner_cell,
            egui::Button::new(
                egui::RichText::new(label).font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            )
            .frame(false),
        );
        button.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!(
                    "Open retained source analysis for {}{}",
                    row.measurement,
                    row.worst_corner
                        .as_ref()
                        .map_or_else(String::new, |corner| format!(" at {corner}"))
                ),
            )
        });
        button.clicked().then_some(analysis_index)
    } else {
        paint_clipped_table_text(
            ui,
            corner_cell,
            egui::Align2::LEFT_CENTER,
            "—",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
        None
    };

    let status_cell = column_rect(rect, 6).shrink2(egui::vec2(9.0, 0.0));
    let (status_color, mark) = match row.status {
        SpecResultStatus::Pass => (c.ok, StatusMark::Success),
        SpecResultStatus::Fail | SpecResultStatus::Invalid => (c.err, StatusMark::Failure),
        SpecResultStatus::Unbound | SpecResultStatus::Missing => {
            (c.text_faint, StatusMark::Neutral)
        }
    };
    paint_status_mark(
        &ui.painter().with_clip_rect(status_cell),
        egui::Rect::from_center_size(
            egui::pos2(status_cell.left() + 5.0, status_cell.center().y),
            egui::Vec2::splat(8.0),
        ),
        mark,
        status_color,
    );
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(status_cell.left() + 14.0, status_cell.top()),
            status_cell.right_bottom(),
        ),
        egui::Align2::LEFT_CENTER,
        row.status.label(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        status_color,
    );
    focused
}

fn show_table_shell(
    ui: &mut Ui,
    rows: &[SpecResultRow],
    salt: impl std::hash::Hash + Copy + std::fmt::Debug,
    empty_message: Option<&str>,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let row_height = spec_table_row_height(t.metrics.ctl_h);
    let content_width = ui.available_width().max(table_width());
    let max_margin = rows
        .iter()
        .filter_map(|row| row.margin)
        .map(f64::abs)
        .fold(0.0_f64, f64::max)
        .max(f64::EPSILON);
    let mut focused = None;
    let table = egui::Frame::NONE.show(ui, |ui| {
        egui::ScrollArea::horizontal()
            .id_salt(("rspice.results.specification-table-x", salt))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_min_width(content_width);
                paint_table_header(ui, content_width, row_height);
                egui::ScrollArea::vertical()
                    .id_salt(("rspice.results.specification-table-y", salt))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if rows.is_empty() {
                            let height = ui.available_height().max(120.0);
                            ui.allocate_ui_with_layout(
                                egui::vec2(content_width, height),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.set_min_height(height);
                                    well_hint(
                                        ui,
                                        empty_message.unwrap_or("No specification evidence"),
                                    );
                                },
                            );
                        } else {
                            for (index, row) in rows.iter().enumerate() {
                                focused = paint_result_row(
                                    ui,
                                    row,
                                    index,
                                    content_width,
                                    row_height,
                                    max_margin,
                                )
                                .or(focused);
                            }
                        }
                    });
            });
    });
    table.response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            "Active dataset specification results",
        )
    });
    ui.ctx().accesskit_node_builder(table.response.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_label("Active dataset specification results");
    });
    focused
}

fn paint_summary(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    accessible_label: String,
    detail_color: egui::Color32,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 40.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
    ui.painter().rect_filled(rect, 0.0, c.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border_strong),
    );
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 10.0, rect.top()),
            egui::pos2(rect.center().x, rect.bottom()),
        ),
        egui::Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_1, FontWeight::Medium),
        c.text,
    );
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(rect.center().x, rect.top()),
            egui::pos2(rect.right() - 10.0, rect.bottom()),
        ),
        egui::Align2::RIGHT_CENTER,
        detail,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        detail_color,
    );
}

/// Render the active dataset's specification evidence as the upgraded
/// seven-column engineering table (or the inline contract editor).
pub fn show(ui: &mut Ui, state: &mut AppState) {
    if state.ui.results.spec_drafts.is_some() {
        show_editor(ui, state);
        return;
    }

    let c = Tokens::get(ui.ctx()).color;
    let Some(run) = state.simulation.active_run() else {
        let bounded = state
            .workspace
            .specs
            .iter()
            .filter(|spec| spec.min.is_some() || spec.max.is_some())
            .count();
        paint_summary(
            ui,
            "Specifications",
            &format!("0 / {bounded} pass · no active dataset"),
            format!(
                "Specifications: zero of {bounded} bounded measurements pass; no active dataset"
            ),
            c.warn,
        );
        show_table_shell(
            ui,
            &[],
            "no-active-dataset",
            Some(
                "No active dataset — select a retained run or run the simulation to evaluate specifications",
            ),
        );
        return;
    };

    let run_id = run.id;
    let dataset_id = run.dataset_id;
    let lifecycle = run.lifecycle;
    let rows = result_rows(run, &state.workspace.specs);
    let summary = summarize_rows(&rows);
    let passing = summary.passing;
    let bounded = summary.bounded;
    let unavailable = summary.unavailable;
    let unavailable_text = (unavailable > 0)
        .then(|| format!(" · {unavailable} unavailable"))
        .unwrap_or_default();
    paint_summary(
        ui,
        &format!("Specifications · Run #{run_id}"),
        &format!(
            "{passing} / {bounded} pass{unavailable_text} · {} · dataset {dataset_id}",
            lifecycle_label(lifecycle)
        ),
        format!(
            "Specifications for run {run_id}, dataset {dataset_id}, {}: {passing} of {bounded} bounded measurements pass; {unavailable} unavailable",
            lifecycle_label(lifecycle)
        ),
        if lifecycle == SimulationRunLifecycle::Completed && unavailable == 0 {
            c.text_dim
        } else {
            c.warn
        },
    );
    let focus_analysis = show_table_shell(
        ui,
        &rows,
        dataset_id,
        Some(
            "No measurements — add .MEAS statements, run the simulation, then bind requirement limits",
        ),
    );

    if let Some(analysis_index) = focus_analysis {
        let viewer = state
            .simulation
            .active_run()
            .and_then(|run| run.analyses.get(analysis_index))
            .map(|analysis| source_viewer(state, analysis.analysis_type))
            .unwrap_or(ResultViewer::Manifest);
        if state.simulation.select_analysis(analysis_index) {
            state.ui.results.viewer = viewer;
            state.ui.results.clear_cursors();
        }
    }
}

fn source_viewer(state: &AppState, analysis_type: AnalysisType) -> ResultViewer {
    let candidate = match analysis_type {
        AnalysisType::DcOp => ResultViewer::Op,
        AnalysisType::Ac | AnalysisType::Pac | AnalysisType::Stb | AnalysisType::Pstb => {
            ResultViewer::Bode
        }
        AnalysisType::Fourier | AnalysisType::HarmonicBalance => ResultViewer::Fft,
        AnalysisType::Sensitivity => ResultViewer::Contribution,
        AnalysisType::Tf | AnalysisType::Pxf | AnalysisType::Qpxf => ResultViewer::TransferFunction,
        AnalysisType::PoleZero => ResultViewer::PoleZero,
        _ => ResultViewer::Waves,
    };
    if super::viewer_availability(state, candidate).available {
        candidate
    } else {
        ResultViewer::Manifest
    }
}

/// The rail text: `≥ min · ≤ max unit` with whichever bounds exist.
fn rail_text(spec: &SpecEntry) -> String {
    let mut parts = Vec::new();
    if let Some(min) = spec.min {
        parts.push(format!("≥ {}", fmt_si(min, "", 3).trim()));
    }
    if let Some(max) = spec.max {
        parts.push(format!("≤ {}", fmt_si(max, "", 3).trim()));
    }
    if parts.is_empty() {
        return "—".to_owned();
    }
    let mut text = parts.join(" · ");
    if !spec.unit.is_empty() {
        text.push(' ');
        text.push_str(&spec.unit);
    }
    text
}

/// The inline spec editor: one text row per spec, engineering notation
/// accepted in the bound fields, with discovered-measurement shortcuts.
fn show_editor(ui: &mut Ui, state: &mut AppState) {
    let untracked = untracked_measurements(state);
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some(drafts) = state.ui.results.spec_drafts.as_mut() else {
        return;
    };
    let mut remove: Option<usize> = None;

    egui::ScrollArea::both()
        .id_salt("rspice.results.specs-edit")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(
                        "Bounds accept engineering notation (10meg, 1u, 3.3). \
                         Empty bound = unbounded side. Expression is retained authored source, never inferred from results.",
                    )
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
                );
            });
            ui.add_space(6.0);

            // Column captions.
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                for (label, width) in [
                    ("MEASUREMENT", 160.0),
                    ("EXPRESSION", 240.0),
                    ("MIN", 100.0),
                    ("MAX", 100.0),
                    ("UNIT", 70.0),
                ] {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(width, 18.0), egui::Sense::hover());
                    ui.painter().text(
                        egui::pos2(rect.left() + 4.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    );
                }
            });

            for (idx, draft) in drafts.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    let field = |ui: &mut Ui, text: &mut String, width: f32, hint: &str| {
                        ui.add(
                            egui::TextEdit::singleline(text)
                                .desired_width(width)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .hint_text(hint),
                        );
                    };
                    field(ui, &mut draft.measurement, 160.0, "meas name");
                    field(ui, &mut draft.expression, 240.0, ".MEAS expression");
                    field(ui, &mut draft.min, 100.0, "—");
                    field(ui, &mut draft.max, 100.0, "—");
                    field(ui, &mut draft.unit, 70.0, "");
                    if icon_button(
                        ui,
                        WorkbenchIcon::Close,
                        "Remove spec",
                        false,
                        egui::vec2(22.0, 22.0),
                    )
                    .clicked()
                    {
                        remove = Some(idx);
                    }
                    if let Err(field) = draft.parse() {
                        ui.label(
                            egui::RichText::new(format!("invalid {field}"))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.err),
                        );
                    }
                });
                ui.add_space(2.0);
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                if ui.button("Add spec").clicked() {
                    drafts.push(SpecDraft::default());
                }
            });

            // One-click drafts for measurements seen in runs but unbound.
            let missing: Vec<&String> = untracked
                .iter()
                .filter(|name| {
                    !drafts
                        .iter()
                        .any(|d| d.measurement.eq_ignore_ascii_case(name))
                })
                .collect();
            if !missing.is_empty() {
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("discovered:")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let mut add: Option<String> = None;
                    for name in missing {
                        if ui.small_button(name.as_str()).clicked() {
                            add = Some(name.clone());
                        }
                    }
                    if let Some(name) = add {
                        drafts.push(SpecDraft {
                            measurement: name,
                            ..Default::default()
                        });
                    }
                });
            }
        });

    if let Some(idx) = remove
        && let Some(drafts) = state.ui.results.spec_drafts.as_mut()
    {
        drafts.remove(idx);
    }
}

/// Apply the open editor's drafts to the workspace. Returns false (and
/// leaves the editor open) when a bound fails to parse.
pub fn apply_drafts(state: &mut AppState) -> bool {
    let Some(drafts) = state.ui.results.spec_drafts.clone() else {
        return true;
    };
    let mut specs = Vec::with_capacity(drafts.len());
    for draft in &drafts {
        match draft.parse() {
            Ok(Some(entry)) => specs.push(entry),
            Ok(None) => {}
            Err(_) => return false,
        }
    }
    let Ok(plan_id) = state
        .sim_setup
        .stable_analysis_plan()
        .map(crate::simulation::plan::SimulationPlan::id)
    else {
        return false;
    };
    let mut workspace = state.workspace.clone();
    workspace.replace_active_specs(plan_id, specs);
    if workspace.validate_simulation_configuration().is_err() {
        return false;
    }
    let mut setup = state.sim_setup.clone();
    if setup
        .commit_active_plan_configuration_change("Updated output specifications.")
        .is_err()
    {
        return false;
    }
    state.workspace = workspace;
    state.sim_setup = setup;
    state.workbench.preflight = Default::default();
    state.ui.results.spec_drafts = None;
    true
}

/// Open the editor seeded from the current workspace specs.
pub fn open_editor(state: &mut AppState) {
    let drafts = state
        .workspace
        .specs
        .iter()
        .map(SpecDraft::from_entry)
        .collect();
    state.ui.results.spec_drafts = Some(drafts);
}

/// Right panel: the same active-dataset projection shown in the document.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let specs = &state.workspace.specs;
    if specs.is_empty() {
        section_header(ui, "Specs", None);
        measurement_table(ui, &[("Bounds", "none defined")]);
        return;
    }

    let Some(run) = state.simulation.active_run() else {
        section_header(ui, "Specs · active dataset", None);
        let bounded = specs
            .iter()
            .filter(|spec| spec.min.is_some() || spec.max.is_some())
            .count()
            .to_string();
        let specs_n = specs.len().to_string();
        measurement_table(
            ui,
            &[
                ("Dataset", "none selected"),
                ("Specs", specs_n.as_str()),
                ("Bounded", bounded.as_str()),
                ("Unavailable", bounded.as_str()),
            ],
        );
        return;
    };

    let rows = result_rows(run, specs);
    let summary = summarize_rows(&rows);
    let bounded = summary.bounded;
    let passing = summary.passing;
    let failures = summary.failures;
    let unavailable = summary.unavailable;
    let worst = rows
        .iter()
        .filter(|row| row.is_bounded && row.status == SpecResultStatus::Fail)
        .filter_map(|row| row.margin.map(|margin| (row, margin)))
        .min_by(|(_, left), (_, right)| left.total_cmp(right));

    section_header(ui, "Specs · active dataset", None);
    let specs_n = specs.len().to_string();
    let run_n = format!("run #{}", run.id);
    let bounded_s = bounded.to_string();
    let passing_s = passing.to_string();
    let fails_s = failures.to_string();
    let unavailable_s = unavailable.to_string();
    measurement_table(
        ui,
        &[
            ("Dataset", run_n.as_str()),
            ("Specs", specs_n.as_str()),
            ("Bounded", bounded_s.as_str()),
            ("Passing", passing_s.as_str()),
            ("Failures", fails_s.as_str()),
            ("Unavailable", unavailable_s.as_str()),
        ],
    );

    if let Some((row, _)) = worst {
        ui.add_space(8.0);
        section_header(ui, "Worst violation", None);
        let value_s = value_text(row);
        let margin_s = margin_text(row);
        let source_s = row.worst_corner.as_deref().unwrap_or("source unavailable");
        measurement_table(
            ui,
            &[
                (row.measurement.as_str(), value_s.as_str()),
                ("Margin", margin_s.as_str()),
                ("Source", source_s),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SpecDraft, SpecResultStatus, apply_drafts, result_row, result_rows,
        row_accessibility_label, signed_margin, spec_table_row_height, summarize_rows, table_width,
    };
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::state::{
        AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultProvenance, AnalysisType,
        SimulationRun, SpecEntry,
    };

    #[test]
    fn matrix_rows_follow_desktop_and_touch_control_contracts() {
        assert_eq!(spec_table_row_height(25.0), 28.0);
        assert_eq!(spec_table_row_height(32.0), 32.0);
        assert_eq!(spec_table_row_height(44.0), 44.0);
        assert_eq!(spec_table_row_height(48.0), 48.0);
    }

    #[test]
    fn legacy_specification_deserialization_does_not_invent_an_expression() {
        let spec: SpecEntry =
            serde_json::from_str(r#"{"measurement":"gain","min":1.0,"max":2.0,"unit":"V/V"}"#)
                .expect("legacy specification remains readable");

        assert!(spec.expression.is_empty());
        assert!(
            !serde_json::to_string(&spec)
                .expect("migrated specification serializes")
                .contains("expression")
        );
    }

    #[test]
    fn signed_margin_is_positive_inside_and_negative_outside_each_bound_shape() {
        let two_sided = SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(10.0),
            max: Some(20.0),
            unit: "dB".to_owned(),
        };
        assert_eq!(signed_margin(&two_sided, 12.0), Some(2.0));
        assert_eq!(signed_margin(&two_sided, 22.5), Some(-2.5));

        let minimum = SpecEntry {
            max: None,
            ..two_sided.clone()
        };
        assert_eq!(signed_margin(&minimum, 13.0), Some(3.0));
        let maximum = SpecEntry {
            min: None,
            max: Some(20.0),
            ..two_sided
        };
        assert_eq!(signed_margin(&maximum, 18.0), Some(2.0));
    }

    #[test]
    fn bounded_row_uses_the_exact_worst_retained_source_and_corner() {
        let mut run = SimulationRun::new(4);
        let source = AnalysisInstanceId::new();
        for (value, corner) in [(0.8, "TT · 27 °C"), (1.2, "SS · 125 °C")] {
            run.add_analysis(
                AnalysisResult::new(1, AnalysisType::Corner, corner)
                    .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                        x_values: vec![1.0],
                        x_label: "corner".to_owned(),
                        x_unit: String::new(),
                        temperatures_c: vec![27.0],
                        corner_labels: vec![corner.to_owned()],
                        failed_corners: 0,
                    })
                    .with_provenance(
                        AnalysisResultProvenance::new(
                            source,
                            ObjectRevision::INITIAL,
                            ContentDigest::from_bytes([0x51; 32]),
                            Vec::new(),
                        )
                        .expect("corner provenance is valid"),
                    )
                    .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
            );
        }
        let spec = SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(0.0),
            max: Some(1.0),
            unit: "V/V".to_owned(),
        };

        let row = result_row(&run, "gain".to_owned(), Some(&spec));

        assert_eq!(row.value, Some(1.2));
        assert!(
            row.margin
                .is_some_and(|margin| (margin + 0.2).abs() < 1.0e-12)
        );
        assert_eq!(row.status, SpecResultStatus::Fail);
        assert_eq!(row.source_analysis_index, Some(1));
        assert_eq!(row.worst_corner.as_deref(), Some("SS · 125 °C"));
    }

    #[test]
    fn ambiguous_unbound_measurement_never_selects_an_arbitrary_value() {
        let mut run = SimulationRun::new(5);
        for value in [1.0, 2.0] {
            run.add_analysis(
                AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                    .with_measurements(vec![rspice_core::MeasureResult::success("delay", value)]),
            );
        }

        let row = result_row(&run, "delay".to_owned(), None);

        assert_eq!(row.status, SpecResultStatus::Invalid);
        assert_eq!(row.value, None);
        assert_eq!(row.source_analysis_index, None);
        assert!(row.detail.contains("2 retained analyses"));
    }

    #[test]
    fn seven_column_geometry_is_stable_and_does_not_depend_on_row_state() {
        assert_eq!(super::SPEC_COLUMN_WIDTHS.len(), 7);
        assert_eq!(table_width(), 1042.0);
    }

    #[test]
    fn bounded_missing_measurements_remain_in_the_requirement_denominator() {
        let run = SimulationRun::new(6);
        let spec = SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(1.0),
            max: None,
            unit: "V/V".to_owned(),
        };

        let rows = result_rows(&run, &[spec]);
        let summary = summarize_rows(&rows);

        assert_eq!(summary.bounded, 1);
        assert_eq!(summary.passing, 0);
        assert_eq!(summary.failures, 0);
        assert_eq!(summary.unavailable, 1);
        assert_eq!(rows[0].status, SpecResultStatus::Missing);
    }

    #[test]
    fn bounded_cross_analysis_name_collision_fails_closed_without_lineage() {
        let mut run = SimulationRun::new(7);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Ac, "AC")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 2.0)]),
        );
        let spec = SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(0.0),
            max: Some(3.0),
            unit: "V/V".to_owned(),
        };

        let row = result_row(&run, "gain".to_owned(), Some(&spec));

        assert_eq!(row.status, SpecResultStatus::Invalid);
        assert_eq!(row.value, None);
        assert_eq!(row.source_analysis_index, None);
        assert!(row.detail.contains("different or unproven source lineages"));
    }

    #[test]
    fn rows_preserve_declared_contract_order_then_first_retained_order() {
        let mut run = SimulationRun::new(8);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_measurements(vec![
                rspice_core::MeasureResult::success("zeta", 1.0),
                rspice_core::MeasureResult::success("beta", 2.0),
                rspice_core::MeasureResult::success("alpha", 3.0),
            ]),
        );
        let specs = [
            SpecEntry {
                measurement: "zeta".to_owned(),
                expression: "max V(z)".to_owned(),
                min: None,
                max: Some(2.0),
                unit: "V".to_owned(),
            },
            SpecEntry {
                measurement: "alpha".to_owned(),
                expression: "max V(a)".to_owned(),
                min: None,
                max: Some(4.0),
                unit: "V".to_owned(),
            },
        ];

        let names: Vec<_> = result_rows(&run, &specs)
            .into_iter()
            .map(|row| row.measurement)
            .collect();

        assert_eq!(names, ["zeta", "alpha", "beta"]);
    }

    #[test]
    fn row_accessibility_carries_every_visible_engineering_value() {
        let mut run = SimulationRun::new(9);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.5)]),
        );
        let spec = SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(1.0),
            max: Some(2.0),
            unit: "V/V".to_owned(),
        };
        let row = result_row(&run, "gain".to_owned(), Some(&spec));

        let label = row_accessibility_label(&row);

        assert!(label.contains("expression max V(out)"));
        assert!(label.contains(&format!("value {}", super::value_text(&row))));
        assert!(label.contains(&format!("limit {}", row.limit)));
        assert!(label.contains(&format!("margin {}", super::margin_text(&row))));
        assert!(label.contains("status pass"));
    }

    #[test]
    fn applying_drafts_commits_the_active_plan_owned_specification() {
        let mut state = crate::workbench::AppState::default();
        let plan_id = state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .id();
        let source_revision = state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .revision();
        state.ui.results.spec_drafts = Some(vec![SpecDraft {
            measurement: "gain_db".to_owned(),
            expression: "max db(V(out))".to_owned(),
            min: "20".to_owned(),
            max: "40".to_owned(),
            unit: "dB".to_owned(),
        }]);

        assert!(apply_drafts(&mut state));

        let owned = state
            .workspace
            .active_plan_data(plan_id)
            .expect("active plan payload");
        assert_eq!(owned.specs.len(), 1);
        assert_eq!(owned.specs[0].measurement, "gain_db");
        assert_eq!(owned.specs[0].expression, "max db(V(out))");
        assert_eq!(state.workspace.specs, owned.specs);
        assert!(
            state
                .sim_setup
                .stable_analysis_plan()
                .expect("default plan")
                .revision()
                > source_revision
        );
        assert!(state.ui.results.spec_drafts.is_none());
    }
}
