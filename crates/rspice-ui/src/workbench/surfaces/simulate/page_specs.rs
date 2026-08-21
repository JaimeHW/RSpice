//! Requirements & specifications.
//!
//! The limits a result is judged against. A specification bounds a named
//! measurement, so the page's honest subject is the join between the two: a
//! limit whose measurement no analysis produces is unmapped, and saying so
//! before dispatch is the whole value of the page.

use egui::Ui;

use crate::state::workspace::SimulationPlanPayload;
use crate::state::{
    MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy,
    RegressionSpecificationPolicy, SpecEntry, SpecPointScope, SpecificationDefinition,
    SpecificationPolicy, SpecificationRole,
};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::{SimulationPage, SpecificationEvidenceFilter};

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

fn governed_definition<'a>(
    payload: &'a SimulationPlanPayload,
    spec: &SpecEntry,
) -> Option<&'a SpecificationDefinition> {
    payload.specification_definitions.iter().find(|definition| {
        definition
            .measurement
            .eq_ignore_ascii_case(&spec.measurement)
    })
}

fn role_label(role: SpecificationRole) -> &'static str {
    match role {
        SpecificationRole::Blocking => "blocking",
        SpecificationRole::Review => "review",
        SpecificationRole::Informational => "informational",
    }
}

fn plan_payload(app: &RSpiceApp) -> SimulationPlanPayload {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| app.state.workspace.plan_data(plan_id).cloned())
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

    let run_set = &app.state.sim_setup.run_set;
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
fn evidence_for(
    app: &RSpiceApp,
    spec: &SpecEntry,
    definition: Option<&SpecificationDefinition>,
) -> Evidence {
    let Some(run) = super::output_evidence::selected_plan_dataset(app) else {
        return Evidence::None;
    };
    if let Some(verdict) = run.specification_verdicts().and_then(|verdicts| {
        verdicts.iter().find(|verdict| {
            definition.map_or_else(
                || {
                    verdict
                        .measurement()
                        .eq_ignore_ascii_case(&spec.measurement)
                },
                |definition| verdict.specification_id() == Some(definition.id),
            )
        })
    }) {
        let points = usize::try_from(verdict.evidence_count()).unwrap_or(usize::MAX);
        return match verdict.status() {
            crate::state::SpecificationVerdictStatus::Pass => {
                verdict
                    .worst_value()
                    .map_or(Evidence::MeasurementFailed, |value| Evidence::Pass {
                        value,
                        points,
                    })
            }
            crate::state::SpecificationVerdictStatus::BoundFailure => {
                verdict
                    .worst_value()
                    .map_or(Evidence::MeasurementFailed, |value| Evidence::Fail {
                        value,
                        points,
                    })
            }
            crate::state::SpecificationVerdictStatus::MeasurementFailure => {
                Evidence::MeasurementFailed
            }
            crate::state::SpecificationVerdictStatus::MissingEvidence => Evidence::None,
        };
    }
    match super::output_evidence::measurement_in_output_dataset_for_definition(
        run, spec, definition,
    ) {
        None => Evidence::None,
        Some(evidence) if !evidence.measurement_passed => Evidence::MeasurementFailed,
        Some(evidence)
            if guard_banded_margin(spec, definition, evidence.value)
                .is_none_or(|margin| margin >= 0.0) =>
        {
            Evidence::Pass {
                value: evidence.value,
                points: evidence.retained_measurements,
            }
        }
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

/// How much of a specification's scope its producing analysis can answer.
///
/// A scope and a participation can be known to disagree before anything is
/// dispatched: the scope names points, the producer declares which points it
/// runs at, and a bound scoped outside that set can never be judged there. So
/// it is stated on the row as an advisory rather than left to be discovered as
/// missing evidence after a run.
///
/// `None` where there is nothing to warn about — no declared space, or a
/// producer that covers every point the scope asks for.
fn coverage_advisory(
    resolved: &super::participation::PlanParticipation,
    spec: &SpecEntry,
    definition: Option<&SpecificationDefinition>,
) -> Option<String> {
    if !resolved.has_axes {
        return None;
    }
    let demanded = resolved.scope_demands(&spec.scope);
    if demanded.is_empty() {
        return None;
    }
    let covered = resolved.covered_by(
        definition.and_then(|entry| entry.producing_analysis),
        &demanded,
    );
    (covered < demanded.len()).then(|| format!("\u{25b3} {covered}/{} pts", demanded.len()))
}

fn registry(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let specs = &payload.specs;
    let declared = declared_process_corners(app);
    // Resolved once for the registry rather than per row: each resolution
    // re-expands the declared space, and this page draws every frame.
    let resolved = super::participation::PlanParticipation::resolve(app);
    let query = app.state.workbench.specification_filter.clone();
    let class = app.state.workbench.specification_evidence_filter;
    let mut passing = 0usize;
    let mut failing = 0usize;
    let mut unmapped = 0usize;
    let mut partial = 0usize;
    let mut unreachable_scope = 0usize;
    let mut short_coverage = 0usize;
    let rows: Vec<RegistryRow> = specs
        .iter()
        .map(|spec| {
            let governed = governed_definition(payload, spec);
            let evidence = evidence_for(app, spec, governed);
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
            let (mut result, mut tone) = if missing.is_empty() {
                evidence.cell(spec)
            } else {
                unreachable_scope += 1;
                (
                    format!("scoped to {} · not in the run set", missing.join(" · ")),
                    Tone::Warn,
                )
            };
            // Leads the cell rather than trailing it. A verdict read without
            // its coverage is the false sign-off this page exists to prevent,
            // and a cell that elides puts the trailing half out of sight.
            if let Some(advisory) = coverage_advisory(&resolved, spec, governed) {
                short_coverage += 1;
                result = format!("{advisory} · {result}");
                if tone == Tone::Ok || tone == Tone::Neutral {
                    tone = Tone::Warn;
                }
            }
            let expression = if spec.expression.trim().is_empty() {
                "not retained · imported before source was kept".to_owned()
            } else {
                spec.expression.clone()
            };
            let definition = governed.map_or_else(
                || expression.clone(),
                |definition| {
                    format!(
                        "{} · {} · {}",
                        definition.requirement_key,
                        role_label(definition.role),
                        expression
                    )
                },
            );
            let limit = super::output_evidence::specification_limit(spec);
            let margin = margin_label(spec, governed, &evidence);
            let shown = evidence.in_class(class)
                && row_matches(
                    &query,
                    &[
                        spec.measurement.as_str(),
                        definition.as_str(),
                        limit.as_str(),
                        result.as_str(),
                        margin.as_str(),
                        governed.map_or("", |definition| definition.requirement_name.as_str()),
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
        status.push_str(&format!(
            " · {partial} judged on the worst of several points"
        ));
    }
    if unreachable_scope > 0 {
        status.push_str(&format!(
            " · {unreachable_scope} scoped to corners the run set does not declare"
        ));
    }
    if short_coverage > 0 {
        status.push_str(&format!(
            " · {short_coverage} scoped wider than the analysis that produces them runs"
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
    } else if unmapped > 0 || partial > 0 || unreachable_scope > 0 || short_coverage > 0 {
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
        // The limits are plan-owned data with one editor; this page reads
        // them and hands off rather than giving the same records a second
        // author.
        crate::workbench::documents::result_document::open_specification_editor(&mut app.state);
    }
    if let Some(index) = picked_class {
        app.state.workbench.specification_evidence_filter = SpecificationEvidenceFilter::ALL[index];
    }
    if let Some(measurement) = pick {
        app.state.workbench.selected_specification = Some(measurement);
    }
}

/// Distance from the nearest bound, in the specification's own unit.
///
/// Reported only where a scalar result and a scalar bound both exist; a
/// waveform specification has no margin, and inventing one would be a claim
/// the plan cannot support.
fn margin_label(
    spec: &SpecEntry,
    definition: Option<&SpecificationDefinition>,
    evidence: &Evidence,
) -> String {
    let Some(value) = evidence.value() else {
        return "—".to_owned();
    };
    let Some(margin) = guard_banded_margin(spec, definition, value) else {
        return "—".to_owned();
    };
    format!("{margin:+.6} {}", spec.unit)
}

fn guard_banded_margin(
    spec: &SpecEntry,
    definition: Option<&SpecificationDefinition>,
    value: f64,
) -> Option<f64> {
    let margin = match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => (value - minimum).min(maximum - value),
        (Some(minimum), None) => value - minimum,
        (None, Some(maximum)) => maximum - value,
        (None, None) => return None,
    };
    Some(
        margin
            - definition
                .and_then(|definition| definition.guard_band)
                .unwrap_or(0.0),
    )
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
    let points = app.state.sim_setup.run_set.point_count();
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
                .map(|spec| (spec.clone(), governed_definition(payload, spec).cloned()))
        });
    let Some((spec, governed)) = selected.as_ref() else {
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
    let evidence = evidence_for(app, spec, governed.as_ref());
    let coverage = evidence_coverage(app, spec, governed.as_ref());
    let (result, tone) = evidence.cell(spec);
    let options = scope_options(app, &spec.scope);
    let option_labels: Vec<String> = options.iter().map(|(label, _)| label.clone()).collect();
    let current_scope = options
        .iter()
        .find(|(_, scope)| *scope == spec.scope)
        .map_or_else(|| scope_label(&spec.scope), |(label, _)| label.clone());
    let missing = undeclared_corners(spec, &declared_process_corners(app));
    let stable_identity = governed.as_ref().map_or_else(
        || "legacy row · migrate on edit".to_owned(),
        |definition| definition.id.to_string(),
    );
    let requirement_key = governed.as_ref().map_or("legacy row", |definition| {
        definition.requirement_key.as_str()
    });
    let requirement_name = governed
        .as_ref()
        .map_or(spec.measurement.as_str(), |definition| {
            definition.requirement_name.as_str()
        });
    let requirement_role = governed
        .as_ref()
        .map_or("legacy blocking", |definition| role_label(definition.role));
    // The producer is a plan instance, not a string. Resolving it against the
    // plan here decides both what the row says and whether the jump to it can
    // land: an id the plan no longer owns is a stale binding, and saying so is
    // more use than printing the identity that no longer resolves.
    let producer_id = governed
        .as_ref()
        .and_then(|definition| definition.producing_analysis);
    let producer_position = producer_id.and_then(|id| {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .and_then(|plan| {
                plan.instances()
                    .iter()
                    .position(|instance| instance.id() == id)
                    .map(|index| (index, plan.instances()[index].kind().code()))
            })
    });
    let producing_analysis = match (producer_id, producer_position) {
        (None, _) => "any exact producer of this measurement".to_owned(),
        (Some(id), None) => format!("{id} · no longer in this plan"),
        (Some(id), Some((index, code))) => format!("#{} · {code} · {id}", index + 1),
    };
    let guard_band = governed
        .as_ref()
        .and_then(|definition| definition.guard_band)
        .map_or_else(
            || "none".to_owned(),
            |value| format!("{value:.6} {}", spec.unit),
        );
    let source = governed
        .as_ref()
        .and_then(|definition| definition.source.as_ref())
        .map_or_else(
            || "authored in this plan".to_owned(),
            |source| {
                format!(
                    "{}:{} · {} · {}",
                    source.logical_path, source.row, source.imported_revision, source.source_digest
                )
            },
        );
    let waiver = governed
        .as_ref()
        .and_then(|definition| definition.waiver.as_ref())
        .map_or_else(
            || "none".to_owned(),
            |waiver| {
                format!(
                    "{} · {} · {}",
                    waiver.reference, waiver.owner, waiver.rationale
                )
            },
        );
    // Why the evidence hop cannot land, when it cannot. Restated from what the
    // coverage row already says, because a limit is judged against the dataset
    // this plan produced and no other.
    let evidence_block = if super::output_evidence::selected_plan_dataset(app).is_none() {
        Some(if app.state.simulation.active_run().is_some() {
            "The active dataset was not produced by this plan, so it holds no evidence for this \
             limit."
        } else {
            "No run has been retained, so there is no evidence to open."
        })
    } else {
        crate::workbench::documents::result_document::viewer_unavailability_reason(
            &app.state,
            crate::workbench::ResultViewer::Specs,
        )
    };
    let mut picked = None;
    let mut open_evidence = false;
    let mut select_producer = false;
    card(ui, &title, Some((result.as_str(), tone)), |ui| {
        card_body(ui, |ui| {
            rule_row(ui, "Stable requirement identity", &stable_identity);
            rule_row(ui, "Requirement key", requirement_key);
            rule_row(ui, "Requirement name", requirement_name);
            rule_row(ui, "Role", requirement_role);
            rule_row(ui, "Producing analysis", &producing_analysis);
            // The row above identifies the instance; this selects it. Both
            // halves are needed: the identity is what a receipt is checked
            // against, and the selection is what a reader wants next.
            if producer_id.is_some() {
                let producer = Button::new("Select producing analysis")
                    .enabled(producer_position.is_some())
                    .show(ui);
                if producer_position.is_some() {
                    select_producer |= producer
                        .on_hover_text(
                            "Select this instance on the Analyses page of this simulation plan",
                        )
                        .clicked();
                } else {
                    producer.on_hover_text(
                        "This requirement names an analysis instance the plan no longer owns, so \
                         there is nothing to select. Rebind the producer to repair it.",
                    );
                }
            }
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
            // The coverage row states what the dataset holds for this limit;
            // this opens exactly that, in the viewer whose whole subject is
            // specification evidence, with this measurement carried across.
            let open = Button::new("Open evidence in Results")
                .enabled(evidence_block.is_none())
                .show(ui);
            match evidence_block {
                None => {
                    open_evidence |= open
                        .on_hover_text(
                            "Open the Specs viewer on this plan's retained dataset with this \
                             measurement selected",
                        )
                        .clicked();
                }
                Some(reason) => {
                    open.on_hover_text(reason);
                }
            }
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
            rule_row(ui, "Acceptance guard band", &guard_band);
            rule_row(ui, "Requirement source", &source);
            rule_row(ui, "Waiver / disposition", &waiver);
            rule_row(
                ui,
                "Guard-banded margin",
                &margin_label(spec, governed.as_ref(), &evidence),
            );
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
    // Both hops carry the selection they were drawn for. `selected_specification`
    // is the one owner of "which limit is being read", so the Results viewer
    // arrives on the same row this card describes without a second copy of it.
    if select_producer && let Some(id) = producer_id {
        app.state.workbench.active_analysis_instance = Some(id);
        Command::SimulationPage(SimulationPage::Analyses).execute(app);
    } else if open_evidence {
        app.state.workbench.selected_specification = Some(spec.measurement.clone());
        Command::ResultViewer(crate::workbench::ResultViewer::Specs).execute(app);
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
fn evidence_coverage(
    app: &RSpiceApp,
    spec: &SpecEntry,
    definition: Option<&SpecificationDefinition>,
) -> String {
    let Some(run) = super::output_evidence::selected_plan_dataset(app) else {
        return if app.state.simulation.active_run().is_some() {
            "active dataset was not produced by this plan".to_owned()
        } else {
            "no dataset loaded".to_owned()
        };
    };
    let within = if spec.scope.is_all_points() {
        String::new()
    } else {
        format!(" in {}", scope_label(&spec.scope).to_lowercase())
    };
    match super::output_evidence::measurement_in_output_dataset_for_definition(
        run, spec, definition,
    ) {
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

fn monte_carlo_policy_label(policy: &MonteCarloSpecificationGate) -> String {
    match policy {
        MonteCarloSpecificationGate::NotGated => "Not gated".to_owned(),
        MonteCarloSpecificationGate::YieldAtLeast { percent } => {
            format!("Yield at least {percent}%")
        }
    }
}

fn evaluation_policy(ui: &mut Ui, app: &mut RSpiceApp, payload: &SimulationPlanPayload) {
    let dataset = super::output_evidence::selected_plan_dataset(app).map_or_else(
        || {
            app.state.simulation.active_run().map_or_else(
                || "none · no dataset is loaded".to_owned(),
                |run| format!("Run {} · belongs to another plan or a manual deck", run.id),
            )
        },
        |run| {
            if run.prepared_receipt().is_some() {
                format!("Run {} · this plan · immutable", run.id)
            } else {
                format!(
                    "Run {} · legacy dataset · plan ownership was not retained",
                    run.id
                )
            }
        },
    );
    let acceptance = super::output_evidence::selected_plan_dataset(app).map_or_else(
        || {
            if app.state.simulation.active_run().is_some() {
                "not evaluated · active dataset does not belong to this plan".to_owned()
            } else {
                "not evaluated · no dataset is loaded".to_owned()
            }
        },
        |run| {
            if run.specification_verdicts().is_none() {
                "pending · the selected run has no terminal verdict".to_owned()
            } else if run.specification_acceptance_is_blocked() {
                "blocked by its frozen requirement policy".to_owned()
            } else {
                "not blocked by its frozen requirement policy".to_owned()
            }
        },
    );
    let policy = payload.specification_policy.clone();
    let nominal_options = vec![
        "Block plan acceptance".to_owned(),
        "Record disposition for review".to_owned(),
    ];
    let missing_options = vec![
        "Fail closed".to_owned(),
        "Report unmapped for review".to_owned(),
    ];
    let regression_options = vec![
        "Limit and waveform evidence".to_owned(),
        "Limit evidence only".to_owned(),
    ];
    let monte_carlo_options = [
        "Not gated".to_owned(),
        "Yield at least 95%".to_owned(),
        "Yield at least 99%".to_owned(),
        "Yield at least 99.9%".to_owned(),
    ];
    let nominal_current = match policy.nominal_failure {
        NominalFailurePolicy::Block => nominal_options[0].as_str(),
        NominalFailurePolicy::RecordDisposition => nominal_options[1].as_str(),
    };
    let missing_current = match policy.missing_measurement {
        MissingMeasurementPolicy::FailClosed => missing_options[0].as_str(),
        MissingMeasurementPolicy::ReportUnmapped => missing_options[1].as_str(),
    };
    let regression_current = match policy.regression {
        RegressionSpecificationPolicy::LimitAndWaveform => regression_options[0].as_str(),
        RegressionSpecificationPolicy::LimitOnly => regression_options[1].as_str(),
    };
    let monte_carlo_current = monte_carlo_policy_label(&policy.monte_carlo);
    let mut picked_nominal = None;
    let mut picked_missing = None;
    let mut picked_regression = None;
    let mut picked_monte_carlo = None;
    let policy_tone = if matches!(
        policy.missing_measurement,
        MissingMeasurementPolicy::FailClosed
    ) && matches!(policy.nominal_failure, NominalFailurePolicy::Block)
    {
        ("fail closed", Tone::Ok)
    } else {
        ("review dispositions enabled", Tone::Warn)
    };
    card(ui, "Evaluation policy", Some(policy_tone), |ui| {
        card_body(ui, |ui| {
            rule_row(ui, "Evidence dataset", &dataset);
            rule_row(ui, "Dataset acceptance", &acceptance);
            rule_row(
                ui,
                "Specifications in this plan",
                &payload.specs.len().to_string(),
            );
            field_pair(
                ui,
                ("Nominal failure", &mut |ui: &mut Ui, width: f32| {
                    picked_nominal = select(
                        ui,
                        "simulation-plan.spec-policy.nominal-failure",
                        "Nominal failure",
                        nominal_current,
                        &nominal_options,
                        width,
                    );
                }),
                Some(("Missing measurement", &mut |ui: &mut Ui, width: f32| {
                    picked_missing = select(
                        ui,
                        "simulation-plan.spec-policy.missing-measurement",
                        "Missing measurement",
                        missing_current,
                        &missing_options,
                        width,
                    );
                })),
            );
            field_pair(
                ui,
                ("Monte Carlo gate", &mut |ui: &mut Ui, width: f32| {
                    picked_monte_carlo = select(
                        ui,
                        "simulation-plan.spec-policy.monte-carlo",
                        "Monte Carlo gate",
                        &monte_carlo_current,
                        &monte_carlo_options,
                        width,
                    );
                }),
                Some(("Regression evidence", &mut |ui: &mut Ui, width: f32| {
                    picked_regression = select(
                        ui,
                        "simulation-plan.spec-policy.regression",
                        "Regression evidence",
                        regression_current,
                        &regression_options,
                        width,
                    );
                })),
            );
        });
        card_note(
            ui,
            "This policy is plan-owned and frozen into every prepared run. Changing it creates \
                 a new plan revision; historical verdicts continue to use the exact policy in their \
                 immutable run receipt.",
        );
    });

    let mut edited = policy.clone();
    if let Some(index) = picked_nominal {
        edited.nominal_failure = match index {
            0 => NominalFailurePolicy::Block,
            _ => NominalFailurePolicy::RecordDisposition,
        };
    }
    if let Some(index) = picked_missing {
        edited.missing_measurement = match index {
            0 => MissingMeasurementPolicy::FailClosed,
            _ => MissingMeasurementPolicy::ReportUnmapped,
        };
    }
    if let Some(index) = picked_regression {
        edited.regression = match index {
            0 => RegressionSpecificationPolicy::LimitAndWaveform,
            _ => RegressionSpecificationPolicy::LimitOnly,
        };
    }
    if let Some(index) = picked_monte_carlo {
        edited.monte_carlo = match index {
            0 => MonteCarloSpecificationGate::NotGated,
            1 => MonteCarloSpecificationGate::YieldAtLeast { percent: 95.0 },
            2 => MonteCarloSpecificationGate::YieldAtLeast { percent: 99.0 },
            _ => MonteCarloSpecificationGate::YieldAtLeast { percent: 99.9 },
        };
    }
    if !edited.bitwise_eq(&policy) {
        commit_specification_policy(app, edited);
    }
}

pub(super) fn commit_specification_policy(app: &mut RSpiceApp, policy: SpecificationPolicy) {
    let Ok(plan_id) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.id())
    else {
        return;
    };
    let detail = format!(
        "Updated specification evaluation policy: nominal {}, missing {}, Monte Carlo {}, regression {}.",
        match policy.nominal_failure {
            NominalFailurePolicy::Block => "blocks acceptance",
            NominalFailurePolicy::RecordDisposition => "records a review disposition",
        },
        match policy.missing_measurement {
            MissingMeasurementPolicy::FailClosed => "fails closed",
            MissingMeasurementPolicy::ReportUnmapped => "reports unmapped evidence",
        },
        monte_carlo_policy_label(&policy.monte_carlo),
        match policy.regression {
            RegressionSpecificationPolicy::LimitAndWaveform =>
                "requires limit and waveform evidence",
            RegressionSpecificationPolicy::LimitOnly => "requires limit evidence",
        }
    );
    commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
        policy.validate()?;
        let payload = workspace
            .plan_data_mut(plan_id)
            .ok_or_else(|| format!("simulation plan {plan_id} has no active payload"))?;
        payload.specification_policy = policy;
        Ok(())
    });
}
