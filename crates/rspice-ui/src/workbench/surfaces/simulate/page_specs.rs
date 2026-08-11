//! Requirements & specifications.
//!
//! The limits a result is judged against. A specification bounds a named
//! measurement, so the page's honest subject is the join between the two: a
//! limit whose measurement no analysis produces is unmapped, and saying so
//! before dispatch is the whole value of the page.

use egui::Ui;

use crate::state::SpecEntry;
use crate::state::workspace::SimulationPlanPayload;
use crate::workbench::RSpiceApp;

use crate::ui::widgets::Button;

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, ledger_head,
    ledger_row, rule_row,
};

const REGISTRY_COLUMNS: [f32; 5] = [0.24, 0.26, 0.18, 0.16, 0.16];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let payload = plan_payload(app);
    registry(ui, app, &payload);
    card_row(
        ui,
        app,
        |ui, app| selected_record(ui, app, &payload),
        |ui, app| evaluation_policy(ui, app, &payload),
    );
}

fn plan_payload(app: &RSpiceApp) -> SimulationPlanPayload {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.active_plan_data(plan_id).cloned())
        .unwrap_or_default()
}

/// Resolved evidence for one specification against the active dataset.
///
/// `points` on a judged outcome is how many retained measurements of that name
/// the dataset holds. It is carried through so the page can say whether the
/// number it shows is the dataset's answer or one point out of many — the
/// difference between a verdict and a sample.
enum Evidence {
    Pass { value: f64, points: usize },
    Fail { value: f64, points: usize },
    MeasurementFailed,
    None,
}

impl Evidence {
    fn value(&self) -> Option<f64> {
        match self {
            Self::Pass { value, .. } | Self::Fail { value, .. } => Some(*value),
            Self::MeasurementFailed | Self::None => None,
        }
    }

    /// True when the dataset holds more than one measurement of this name, so
    /// the reported value is the worst of a set rather than a lone answer.
    fn is_partial(&self) -> bool {
        match self {
            Self::Pass { points, .. } | Self::Fail { points, .. } => *points > 1,
            Self::MeasurementFailed | Self::None => false,
        }
    }

    fn cell(&self, spec: &SpecEntry) -> (String, Tone) {
        match self {
            Self::Pass { value, points } if *points > 1 => (
                format!("{value:.6} {} · worst of {points}", spec.unit),
                Tone::Warn,
            ),
            Self::Fail { value, points } if *points > 1 => (
                format!("{value:.6} {} · worst of {points}", spec.unit),
                Tone::Error,
            ),
            Self::Pass { value, .. } => (format!("{value:.6} {}", spec.unit), Tone::Ok),
            Self::Fail { value, .. } => (format!("{value:.6} {}", spec.unit), Tone::Error),
            Self::MeasurementFailed => ("measurement failed".to_owned(), Tone::Error),
            Self::None => ("no evidence".to_owned(), Tone::Warn),
        }
    }
}

/// Resolve one specification against the active dataset.
///
/// The dataset join itself is owned by [`super::output_evidence`]; this only
/// applies the specification's own bound to the value it found.
fn evidence_for(app: &RSpiceApp, spec: &SpecEntry) -> Evidence {
    let Some(run) = super::output_evidence::selected_output_dataset(&app.state.simulation) else {
        return Evidence::None;
    };
    match super::output_evidence::measurement_in_output_dataset(run, spec) {
        None => Evidence::None,
        Some(evidence) if !evidence.measurement_passed => Evidence::MeasurementFailed,
        Some(evidence) if spec.passes(evidence.value) => Evidence::Pass {
            value: evidence.value,
            points: evidence.retained_measurements,
        },
        Some(evidence) => Evidence::Fail {
            value: evidence.value,
            points: evidence.retained_measurements,
        },
    }
}

fn registry(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let specs = &payload.specs;
    let mut passing = 0usize;
    let mut failing = 0usize;
    let mut unmapped = 0usize;
    let mut partial = 0usize;
    let rows: Vec<(String, Tone)> = specs
        .iter()
        .map(|spec| {
            let evidence = evidence_for(app, spec);
            match evidence {
                Evidence::Pass { .. } => passing += 1,
                Evidence::Fail { .. } | Evidence::MeasurementFailed => failing += 1,
                Evidence::None => unmapped += 1,
            }
            if evidence.is_partial() {
                partial += 1;
            }
            evidence.cell(spec)
        })
        .collect();
    // A partially-covered pass is reported separately rather than folded into
    // the pass count. Reading "4 pass" when three of those were decided by one
    // point of a forty-five point sweep is exactly the false sign-off this
    // page exists to prevent.
    let mut status = format!("{passing} pass · {failing} fail · {unmapped} without evidence");
    if partial > 0 {
        status.push_str(&format!(" · {partial} judged on the worst of several points"));
    }
    let tone = if failing > 0 {
        Tone::Error
    } else if unmapped > 0 || partial > 0 {
        Tone::Warn
    } else {
        Tone::Ok
    };
    let selected = app.state.workbench.selected_specification.clone();
    let mut pick = None;
    let mut author = false;
    card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Specification registry",
                Some((status.as_str(), tone)),
                |ui| {
                    author = Button::new("Author specifications…").show(ui).clicked();
                },
            );
        },
        |ui| {
            ledger_head(
                ui,
                &REGISTRY_COLUMNS,
                // The value is the worst attributed measurement of this name
                // the dataset holds, judged against this specification bound.
                // The cell says how many points it was chosen from, because
                // the worst of what was retained is not the worst of a space
                // the runners did not measure at every point.
                &["Measurement", "Definition", "Limit", "Result", "Margin"],
            );
            if specs.is_empty() {
                ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        ("No specifications", Tone::Neutral),
                        ("nothing judges this plan's results", Tone::Warn),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                    ],
                    false,
                );
            }
            for (spec, (result, result_tone)) in specs.iter().zip(&rows) {
                let definition = if spec.expression.trim().is_empty() {
                    "not retained · imported before source was kept".to_owned()
                } else {
                    spec.expression.clone()
                };
                let margin = margin_label(app, spec);
                if ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        (spec.measurement.as_str(), Tone::Accent),
                        (definition.as_str(), Tone::Neutral),
                        (
                            super::output_evidence::specification_limit(spec).as_str(),
                            Tone::Neutral,
                        ),
                        (result.as_str(), *result_tone),
                        (margin.as_str(), *result_tone),
                    ],
                    selected.as_deref() == Some(spec.measurement.as_str()),
                )
                .clicked()
                {
                    pick = Some(spec.measurement.clone());
                }
            }
            card_note(
                ui,
                "A limit is evaluated against the active dataset's measurement of the same name. \
                 A specification with no matching measurement is reported as being without \
                 evidence — it is never treated as passing. Where the dataset holds several \
                 measurements of one name — a corner, temperature or Monte Carlo run set — the \
                 result shown is the worst of them against this limit, marked with the count it \
                 was chosen from. That verdict is sound for every point the dataset retained; it \
                 does not speak for points the run never measured. Authoring lives in the \
                 specification editor, which owns the limits this page reads.",
            );
        },
    );
    if author {
        open_specification_editor(app);
    }
    if let Some(measurement) = pick {
        app.state.workbench.selected_specification = Some(measurement);
    }
}

/// Route to the surface that owns specification authoring.
///
/// The limits are plan-owned data with one editor; duplicating that editor
/// here would give the same records two authors. The page reads them and
/// hands off.
fn open_specification_editor(app: &mut RSpiceApp) {
    app.state.ui.results.viewer = crate::workbench::ResultViewer::Specs;
    crate::workbench::documents::result_document::open_specification_editor(&mut app.state);
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
}

/// Distance from the nearest bound, in the specification's own unit.
///
/// Reported only where a scalar result and a scalar bound both exist; a
/// waveform specification has no margin, and inventing one would be a claim
/// the plan cannot support.
fn margin_label(app: &RSpiceApp, spec: &SpecEntry) -> String {
    let Some(value) = evidence_for(app, spec).value() else {
        return "—".to_owned();
    };
    let margin = match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => (value - minimum).min(maximum - value),
        (Some(minimum), None) => value - minimum,
        (None, Some(maximum)) => maximum - value,
        (None, None) => return "—".to_owned(),
    };
    format!("{margin:+.6} {}", spec.unit)
}

fn selected_record(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
    // Resolved case-insensitively, which is how a measurement name is matched
    // everywhere else: a specification whose case changed is still the same
    // requirement, and one that was deleted resolves to nothing rather than to
    // whichever requirement now sits in its place.
    let selected = app
        .state
        .workbench
        .selected_specification
        .as_deref()
        .and_then(|measurement| {
            payload
                .specs
                .iter()
                .find(|spec| spec.measurement.eq_ignore_ascii_case(measurement))
        });
    let Some(spec) = selected else {
        card(
            ui,
            "Selected specification",
            Some(("none selected", Tone::Neutral)),
            |ui| {
                card_note(
                    ui,
                    "Select a row above to see the bound, the measurement that supplies its value, \
                     and the evidence resolved from the active dataset.",
                );
            },
        );
        return;
    };
    let title = format!("Selected specification · {}", spec.measurement);
    let evidence = evidence_for(app, spec);
    let coverage = evidence_coverage(app, spec);
    let (result, tone) = evidence.cell(spec);
    card(ui, &title, Some((result.as_str(), tone)), |ui| {
        card_body(ui, |ui| {
            rule_row(ui, "Evidence coverage", &coverage);
            rule_row(
                ui,
                "Limit",
                &super::output_evidence::specification_limit(spec),
            );
            rule_row(
                ui,
                "Measurement definition",
                if spec.expression.trim().is_empty() {
                    "not retained by the import that created this row"
                } else {
                    spec.expression.as_str()
                },
            );
            rule_row(ui, "Display unit", &spec.unit);
            rule_row(ui, "Margin", &margin_label(app, spec));
        });
    });
}

/// How much of the dataset the reported value speaks for.
///
/// Spelled out rather than implied, because "judged" and "judged against one
/// of forty-five points" are different claims about the same number.
fn evidence_coverage(app: &RSpiceApp, spec: &SpecEntry) -> String {
    let Some(run) = super::output_evidence::selected_output_dataset(&app.state.simulation) else {
        return "no dataset loaded".to_owned();
    };
    match super::output_evidence::measurement_in_output_dataset(run, spec) {
        None => "no attributed measurement of this name".to_owned(),
        Some(evidence) if evidence.is_complete_coverage() => {
            "the dataset's only measurement of this name".to_owned()
        }
        Some(evidence) => format!(
            "worst of {} retained measurements of this name",
            evidence.retained_measurements
        ),
    }
}

fn evaluation_policy(ui: &mut Ui, app: &RSpiceApp, payload: &SimulationPlanPayload) {
    let dataset = app.state.simulation.active_run().map_or_else(
        || "none · no dataset is loaded".to_owned(),
        |run| format!("Run {} · immutable", run.id),
    );
    card(
        ui,
        "Evaluation policy",
        Some(("fail closed", Tone::Ok)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Evidence dataset", &dataset);
                rule_row(
                    ui,
                    "Specifications in this plan",
                    &payload.specs.len().to_string(),
                );
                rule_row(
                    ui,
                    "Missing measurement",
                    "reported as without evidence · never passed",
                );
                rule_row(
                    ui,
                    "Failed measurement",
                    "the specification fails, whatever the retained value was",
                );
            });
            card_note(
                ui,
                "Judging is deliberately one-directional: a specification can only pass when a \
                 measurement of the same name succeeded and its value satisfies the bound. Every \
                 other outcome — no measurement, a failed one, a non-finite value — is not a pass.",
            );
        },
    );
}
