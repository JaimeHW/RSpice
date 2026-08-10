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

use super::page_kit::{
    Tone, card, card_body, card_note, card_row, ledger_head, ledger_row, rule_row,
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
enum Evidence {
    Pass(f64),
    Fail(f64),
    MeasurementFailed,
    None,
}

impl Evidence {
    fn cell(&self, spec: &SpecEntry) -> (String, Tone) {
        match self {
            Self::Pass(value) => (format!("{value:.6} {}", spec.unit), Tone::Ok),
            Self::Fail(value) => (format!("{value:.6} {}", spec.unit), Tone::Error),
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
    match super::output_evidence::measurement_in_output_dataset(run, &spec.measurement) {
        None => Evidence::None,
        Some(evidence) if !evidence.measurement_passed => Evidence::MeasurementFailed,
        Some(evidence) if spec.passes(evidence.value) => Evidence::Pass(evidence.value),
        Some(evidence) => Evidence::Fail(evidence.value),
    }
}

fn registry(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let specs = &payload.specs;
    let mut passing = 0usize;
    let mut failing = 0usize;
    let mut unmapped = 0usize;
    let rows: Vec<(String, Tone)> = specs
        .iter()
        .map(|spec| {
            let evidence = evidence_for(app, spec);
            match evidence {
                Evidence::Pass(_) => passing += 1,
                Evidence::Fail(_) | Evidence::MeasurementFailed => failing += 1,
                Evidence::None => unmapped += 1,
            }
            evidence.cell(spec)
        })
        .collect();
    let status = format!("{passing} pass · {failing} fail · {unmapped} without evidence");
    let tone = if failing > 0 {
        Tone::Error
    } else if unmapped > 0 {
        Tone::Warn
    } else {
        Tone::Ok
    };
    let selected = app.state.workbench.selected_spec;
    let mut pick = None;
    card(
        ui,
        "Specification registry",
        Some((status.as_str(), tone)),
        |ui| {
            ledger_head(
                ui,
                &REGISTRY_COLUMNS,
                &[
                    "Measurement",
                    "Definition",
                    "Limit",
                    "Worst result",
                    "Margin",
                ],
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
            for (index, (spec, (result, result_tone))) in specs.iter().zip(&rows).enumerate() {
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
                    selected == Some(index),
                )
                .clicked()
                {
                    pick = Some(index);
                }
            }
            card_note(
                ui,
                "A limit is evaluated against the active dataset's measurement of the same name. \
                 A specification with no matching measurement is reported as being without \
                 evidence — it is never treated as passing.",
            );
        },
    );
    if let Some(index) = pick {
        app.state.workbench.selected_spec = Some(index);
    }
}

/// Distance from the nearest bound, in the specification's own unit.
///
/// Reported only where a scalar result and a scalar bound both exist; a
/// waveform specification has no margin, and inventing one would be a claim
/// the plan cannot support.
fn margin_label(app: &RSpiceApp, spec: &SpecEntry) -> String {
    let value = match evidence_for(app, spec) {
        Evidence::Pass(value) | Evidence::Fail(value) => value,
        Evidence::MeasurementFailed | Evidence::None => return "—".to_owned(),
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
    let selected = app
        .state
        .workbench
        .selected_spec
        .and_then(|index| payload.specs.get(index));
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
    let (result, tone) = evidence_for(app, spec).cell(spec);
    card(ui, &title, Some((result.as_str(), tone)), |ui| {
        card_body(ui, |ui| {
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
