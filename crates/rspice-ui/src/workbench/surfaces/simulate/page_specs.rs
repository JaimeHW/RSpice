//! Requirements & specifications.
//!
//! The limits a result is judged against. A specification bounds a named
//! measurement, so the page's honest subject is the join between the two: a
//! limit whose measurement no analysis produces is unmapped, and saying so
//! before dispatch is the whole value of the page.

use egui::Ui;

use crate::state::workspace::SimulationPlanPayload;
use crate::state::{SpecEntry, SpecPointScope};
use crate::workbench::RSpiceApp;
use crate::workbench::state::SpecificationEvidenceFilter;

use crate::ui::widgets::{Button, select};

use super::page_kit::{
    Tone, card, card_body, card_head_row, card_note, card_row, card_with_head, field_pair,
    filter_field, filter_row, ledger_head, ledger_row, row_matches, rule_row,
};
use super::workflows::commit_plan_change;

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

/// The process corners the current run set actually reaches.
///
/// Read from the declared space rather than from a fixed corner list, because
/// a specification scoped to a corner the run set does not declare is a claim
/// nothing will ever answer. A run set with no process axis still runs one
/// corner — the plan's reference section — so that is what it declares.
fn declared_process_corners(app: &RSpiceApp) -> Vec<String> {
    use crate::simulation::run_set::RunSetDimensionKind;

    let run_set = &app.state.sim_setup.corner.run_set;
    match run_set.enabled_dimension_of(RunSetDimensionKind::ProcessSection) {
        Some(dimension) if !dimension.values.is_empty() => dimension
            .values
            .iter()
            .map(|value| value.lexical.trim().to_owned())
            .collect(),
        _ => vec![
            app.state
                .sim_setup
                .reference_pvt
                .process
                .short_name()
                .to_owned(),
        ],
    }
}

/// The corners a specification names that the run set does not declare.
///
/// Recomputed every frame rather than recorded at authoring time: the run set
/// is edited independently of the specifications, so a scope that was sound
/// when it was written goes stale without the specification changing at all.
fn undeclared_corners(spec: &SpecEntry, declared: &[String]) -> Vec<String> {
    spec.scope
        .named_corners()
        .iter()
        .filter(|corner| {
            !declared
                .iter()
                .any(|declared| declared.eq_ignore_ascii_case(corner))
        })
        .cloned()
        .collect()
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

    /// Whether this outcome belongs to the class the registry is showing.
    ///
    /// This is the page's single classification of evidence: the head's
    /// counts are taken from it as well, so the row a filter keeps and the
    /// number the head reports can never disagree.
    fn in_class(&self, class: SpecificationEvidenceFilter) -> bool {
        match class {
            SpecificationEvidenceFilter::All => true,
            SpecificationEvidenceFilter::Failing => {
                matches!(self, Self::Fail { .. } | Self::MeasurementFailed)
            }
            SpecificationEvidenceFilter::WithoutEvidence => matches!(self, Self::None),
            SpecificationEvidenceFilter::Partial => self.is_partial(),
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

/// One registry row, resolved before the card so the table can be filtered
/// and the head can report how much of it survived.
struct RegistryRow {
    definition: String,
    limit: String,
    result: String,
    tone: Tone,
    margin: String,
    shown: bool,
}

fn registry(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let specs = &payload.specs;
    let declared = declared_process_corners(app);
    let query = app.state.workbench.specification_filter.clone();
    let class = app.state.workbench.specification_evidence_filter;
    let mut passing = 0usize;
    let mut failing = 0usize;
    let mut unmapped = 0usize;
    let mut partial = 0usize;
    let mut unreachable_scope = 0usize;
    let rows: Vec<RegistryRow> = specs
        .iter()
        .map(|spec| {
            let evidence = evidence_for(app, spec);
            if matches!(evidence, Evidence::Pass { .. }) {
                passing += 1;
            }
            if evidence.in_class(SpecificationEvidenceFilter::Failing) {
                failing += 1;
            }
            if evidence.in_class(SpecificationEvidenceFilter::WithoutEvidence) {
                unmapped += 1;
            }
            if evidence.in_class(SpecificationEvidenceFilter::Partial) {
                partial += 1;
            }
            // A scope the run set cannot reach outranks "no evidence" in the
            // cell, because the two have different fixes: one needs a run, the
            // other needs the scope or the run set changed.
            let missing = undeclared_corners(spec, &declared);
            let (result, tone) = if missing.is_empty() {
                evidence.cell(spec)
            } else {
                unreachable_scope += 1;
                (
                    format!("scoped to {} · not in the run set", missing.join(" · ")),
                    Tone::Warn,
                )
            };
            let definition = if spec.expression.trim().is_empty() {
                "not retained · imported before source was kept".to_owned()
            } else {
                spec.expression.clone()
            };
            let limit = super::output_evidence::specification_limit(spec);
            let margin = margin_label(spec, &evidence);
            let shown = evidence.in_class(class)
                && row_matches(
                    &query,
                    &[
                        spec.measurement.as_str(),
                        definition.as_str(),
                        limit.as_str(),
                        result.as_str(),
                        margin.as_str(),
                    ],
                );
            RegistryRow {
                definition,
                limit,
                result,
                tone,
                margin,
                shown,
            }
        })
        .collect();
    let shown = rows.iter().filter(|row| row.shown).count();
    // A partially-covered pass is reported separately rather than folded into
    // the pass count. Reading "4 pass" when three of those were decided by one
    // point of a forty-five point sweep is exactly the false sign-off this
    // page exists to prevent.
    let mut status = format!("{passing} pass · {failing} fail · {unmapped} without evidence");
    if partial > 0 {
        status.push_str(&format!(" · {partial} judged on the worst of several points"));
    }
    if unreachable_scope > 0 {
        status.push_str(&format!(
            " · {unreachable_scope} scoped to corners the run set does not declare"
        ));
    }
    // The verdict counts stay over the whole registry: they are what the plan
    // is judged by, and a head that counted only the visible rows would report
    // a sign-off the filter had produced.
    if shown != specs.len() {
        status.push_str(&format!(" · showing {shown} of {}", specs.len()));
    }
    let tone = if failing > 0 {
        Tone::Error
    } else if unmapped > 0 || partial > 0 || unreachable_scope > 0 {
        Tone::Warn
    } else {
        Tone::Ok
    };
    let selected = app.state.workbench.selected_specification.clone();
    let classes: Vec<String> = SpecificationEvidenceFilter::ALL
        .iter()
        .map(|class| (*class).label().to_owned())
        .collect();
    let mut pick = None;
    let mut author = false;
    let mut picked_class = None;
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
            // Withheld over an empty registry: neither a search box nor an
            // evidence class has anything to narrow when nothing judges the
            // plan yet.
            if !specs.is_empty() {
                filter_row(ui, |ui| {
                    filter_field(
                        ui,
                        "simulation-plan.spec-registry.filter",
                        "Filter measurement, definition or limit…",
                        &mut app.state.workbench.specification_filter,
                    );
                    picked_class = select(
                        ui,
                        "simulation-plan.spec-registry.scope",
                        "Evidence class",
                        class.label(),
                        &classes,
                        175.0,
                    );
                });
            }
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
            // A plan with no requirements and a view narrowed to none are
            // opposite facts: the first says nothing judges this plan, the
            // second says the limits are there and the reader is looking at a
            // slice of them. One row cannot stand for both.
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
            } else if shown == 0 {
                let held = format!("{} in this plan · widen the filter", specs.len());
                ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        ("No specification matches the filter", Tone::Warn),
                        (held.as_str(), Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                        ("—", Tone::Neutral),
                    ],
                    false,
                );
            }
            for (spec, row) in specs.iter().zip(&rows) {
                if !row.shown {
                    continue;
                }
                if ledger_row(
                    ui,
                    &REGISTRY_COLUMNS,
                    &[
                        (spec.measurement.as_str(), Tone::Accent),
                        (row.definition.as_str(), Tone::Neutral),
                        (row.limit.as_str(), Tone::Neutral),
                        (row.result.as_str(), row.tone),
                        (row.margin.as_str(), row.tone),
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
                 does not speak for points the run never measured. A limit narrowed to nominal or \
                 to named corners is answered only by measurements the executor attributed to a \
                 point, so an unattributed result never satisfies one. The filter and the evidence \
                 class narrow this table only; the counts above are the whole plan's, because a \
                 sign-off cannot be produced by hiding the rows that fail. Authoring lives in the \
                 specification editor, which owns the limits this page reads.",
            );
        },
    );
    if author {
        open_specification_editor(app);
    }
    if let Some(index) = picked_class {
        app.state.workbench.specification_evidence_filter = SpecificationEvidenceFilter::ALL[index];
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
fn margin_label(spec: &SpecEntry, evidence: &Evidence) -> String {
    let Some(value) = evidence.value() else {
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

/// The scopes the "Applies to" control offers, each with the exact value it
/// commits.
///
/// One entry per corner the run set declares rather than a fixed corner list,
/// so the control cannot offer a corner the plan will not run. A scope already
/// held that none of the derived entries reproduces — several corners at once,
/// or a corner the run set has since dropped — is appended so that opening the
/// record and picking nothing cannot quietly rewrite the requirement.
fn scope_options(app: &RSpiceApp, current: &SpecPointScope) -> Vec<(String, SpecPointScope)> {
    let points = app.state.sim_setup.corner.run_set.point_count();
    let mut options = vec![
        (
            format!("All {points} PVT points"),
            SpecPointScope::AllPoints,
        ),
        ("Nominal only".to_owned(), SpecPointScope::Nominal),
    ];
    options.extend(declared_process_corners(app).into_iter().map(|corner| {
        (
            format!("Corner {corner} only"),
            SpecPointScope::SelectedCorners {
                corners: vec![corner],
            },
        )
    }));
    if !options.iter().any(|(_, scope)| scope == current) {
        options.push((scope_label(current), current.clone()));
    }
    options
}

/// A scope spelled the way the control shows it.
fn scope_label(scope: &SpecPointScope) -> String {
    match scope {
        SpecPointScope::AllPoints => "All PVT points".to_owned(),
        SpecPointScope::Nominal => "Nominal only".to_owned(),
        SpecPointScope::SelectedCorners { corners } => {
            format!("Corners {}", corners.join(" · "))
        }
    }
}

fn selected_record(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
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
        })
        .cloned();
    let Some(spec) = selected.as_ref() else {
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
    let options = scope_options(app, &spec.scope);
    let option_labels: Vec<String> = options.iter().map(|(label, _)| label.clone()).collect();
    let current_scope = options
        .iter()
        .find(|(_, scope)| *scope == spec.scope)
        .map_or_else(|| scope_label(&spec.scope), |(label, _)| label.clone());
    let missing = undeclared_corners(spec, &declared_process_corners(app));
    let mut picked = None;
    card(ui, &title, Some((result.as_str(), tone)), |ui| {
        card_body(ui, |ui| {
            // The scope is a control rather than a read-only row because it is
            // part of the requirement: narrowing it changes which points the
            // limit is a claim about, so it commits as a plan transaction like
            // every other specification edit.
            field_pair(
                ui,
                ("Applies to", &mut |ui: &mut Ui, width: f32| {
                    picked = select(
                        ui,
                        "simulation-plan.spec-editor.scope",
                        "Applies to",
                        &current_scope,
                        &option_labels,
                        width,
                    );
                }),
                None,
            );
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
            rule_row(ui, "Margin", &margin_label(spec, &evidence));
            if !missing.is_empty() {
                card_note(
                    ui,
                    &format!(
                        "This limit is scoped to {}, which the current run set does not declare, \
                         so no run can produce evidence for it. Widen the scope or add the corner \
                         to the run set.",
                        missing.join(" · ")
                    ),
                );
            }
        });
    });
    if let Some(index) = picked
        && options[index].1 != spec.scope
    {
        commit_scope(app, &spec.measurement, options[index].1.clone());
    }
}

/// Adopt a scope change as one plan-configuration transaction.
///
/// The specs vector is replaced whole because that is the only mutation the
/// workspace exposes for it; matching is case-insensitive for the same reason
/// selection is.
pub(super) fn commit_scope(app: &mut RSpiceApp, measurement: &str, scope: SpecPointScope) {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return;
    };
    let detail = format!(
        "Scoped specification {measurement} to {}.",
        scope_label(&scope).to_lowercase()
    );
    let measurement = measurement.to_owned();
    commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
        let mut specs = workspace.active_specs(plan_id).to_vec();
        let target = specs
            .iter_mut()
            .find(|spec| spec.measurement.eq_ignore_ascii_case(&measurement))
            .ok_or_else(|| format!("specification {measurement} no longer exists"))?;
        target.scope = scope;
        target.validate()?;
        workspace.replace_active_specs(plan_id, specs);
        Ok(())
    });
}

/// How much of the dataset the reported value speaks for.
///
/// Spelled out rather than implied, because "judged" and "judged against one
/// of forty-five points" are different claims about the same number. The count
/// is the one *in the specification's scope*, which is the only honest number:
/// a limit narrowed to one corner was never judged against the rest.
fn evidence_coverage(app: &RSpiceApp, spec: &SpecEntry) -> String {
    let Some(run) = super::output_evidence::selected_output_dataset(&app.state.simulation) else {
        return "no dataset loaded".to_owned();
    };
    let within = if spec.scope.is_all_points() {
        String::new()
    } else {
        format!(" in {}", scope_label(&spec.scope).to_lowercase())
    };
    match super::output_evidence::measurement_in_output_dataset(run, spec) {
        None => format!("no attributed measurement of this name{within}"),
        Some(evidence) if evidence.is_complete_coverage() => {
            format!("the only measurement of this name{within}")
        }
        Some(evidence) => format!(
            "worst of {} retained measurements of this name{within}",
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
