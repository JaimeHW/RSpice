//! The Specifications view's inspector with a specification selected: its
//! whole definition, one property to a row and every one editable, and under
//! it what the run says of it.
//!
//! A property is committed alone, as one plan change, when its field lets go
//! of the keyboard or its choice is made; a draft that does not make a valid
//! definition stays on the rows with the reason under them, and the plan
//! keeps what it had. Under the grid are facts only — how many points judged
//! the specification, where its worst value was, how many trials answered it
//! and the verdict — because every one of them is the run's, not the
//! reader's to change.

mod draft;
mod measurement_builder;
mod reference_import;

pub(crate) use draft::SpecDraft;

use egui::Ui;
use rspice_results::specification::{
    MeasurementReferenceSource, SpecPointScope, SpecificationDefinition, SpecificationRole,
};
use rspice_results::specification_verdict::SpecificationVerdictStatus;
use rspice_results::verification::{CellStatus, Row, RowStatus, Sheet};
use rspice_simulation_contract::run_set::RunSetDimensionKind;
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{
    Problem, property_row, property_row_combo, property_row_input_with_hint, property_row_problem,
    property_row_wrapped, section_header,
};

use crate::io::file_exchange::{self, FileKind};
use crate::product::AnalysisInstanceId;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::state::SpecificationInspectorDraft;
use crate::workbench::surfaces::verify::specification_edit::{
    definition_index, edit_specifications, plan_definition,
};
use crate::workbench::surfaces::verify::specifications::{status_word, worst_at};
use draft::{ComparisonDraftKind, measurement_authoring};
use reference_import::{ReferenceCompletion, ReferenceExchange};

pub(super) fn specifications(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let definition = cx
        .state
        .selected_specification
        .as_deref()
        .and_then(|measurement| plan_definition(cx.model, measurement));
    match definition {
        Some(definition) => specification(ui, cx, &definition),
        None => super::acceptance::acceptance(ui, cx),
    }
}

const SOURCES: [(bool, &str); 2] = [
    (false, "A result of the run"),
    (true, "Defined when the plan runs"),
];

const ROLES: [(SpecificationRole, &str); 3] = [
    (SpecificationRole::Blocking, "Blocking"),
    (SpecificationRole::Review, "Review"),
    (SpecificationRole::Informational, "Informational"),
];

fn specification(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
    definition: &SpecificationDefinition,
) {
    let revision = cx
        .model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.revision());
    let fresh = SpecificationInspectorDraft {
        specification: Some(definition.id),
        revision,
        draft: SpecDraft::default(),
    };
    if cx.state.verification.specification_draft.as_ref() != Some(&fresh) {
        cx.state.verification.specification_draft = Some(SpecificationInspectorDraft {
            draft: SpecDraft::from_definition(definition),
            ..fresh
        });
    }
    let analyses = analysis_options(cx.model);
    let scopes = scope_options(cx.model, &definition.scope);
    let Some(held) = cx.state.verification.specification_draft.as_mut() else {
        return;
    };
    let draft = &mut held.draft;

    section_header(ui, "Specification", None);
    let mut committed: Option<&str> = None;
    let mut commit = |property: &'static str, done: bool| {
        if done && committed.is_none() {
            committed = Some(property);
        }
    };
    commit("name", text(ui, "Name", &mut draft.requirement_name, ""));
    commit("key", text(ui, "Key", &mut draft.requirement_key, ""));
    commit(
        "measurement",
        text(ui, "Measurement", &mut draft.measurement, ""),
    );

    let kinds = ComparisonDraftKind::ALL
        .map(|kind| (kind, kind.label()))
        .to_vec();
    let before = draft.comparison;
    if pick(ui, "Comparison", &kinds, &mut draft.comparison) {
        if draft.comparison == ComparisonDraftKind::Tracked {
            // A tracked specification bounds nothing, so it keeps no guard
            // band to judge against.
            draft.guard_band.clear();
        }
        commit("comparison", draft.comparison != before);
    }
    let limits = match draft.comparison {
        ComparisonDraftKind::Tracked => None,
        ComparisonDraftKind::Minimum => Some(("Minimum", None)),
        ComparisonDraftKind::Maximum => Some(("Maximum", None)),
        ComparisonDraftKind::Range => Some(("Range from", Some("Range to"))),
        ComparisonDraftKind::EqualWithin => Some(("Target", Some("Tolerance"))),
    };
    if let Some((primary, secondary)) = limits {
        commit(
            "limit",
            text(ui, primary, &mut draft.primary_limit, "10meg, 1u, 3.3"),
        );
        if let Some(secondary) = secondary {
            commit(
                "limit",
                text(ui, secondary, &mut draft.secondary_limit, "10meg, 1u, 3.3"),
            );
        }
        commit(
            "guard band",
            text(ui, "Guard band", &mut draft.guard_band, "none"),
        );
    }
    commit(
        "unit",
        text(ui, "Unit", &mut draft.unit, "the measurement's own"),
    );
    commit("role", pick(ui, "Role", &ROLES, &mut draft.role));
    commit(
        "points it applies to",
        pick_scope(ui, &scopes, &mut draft.scope),
    );
    commit(
        "producing analysis",
        pick_analysis(ui, &analyses, &mut draft.producing_analysis),
    );
    commit(
        "measurement source",
        pick(ui, "Source", &SOURCES, &mut draft.define_measurement),
    );
    let expression = if draft.define_measurement {
        "Card"
    } else {
        "Expression"
    };
    commit(
        "expression",
        text(ui, expression, &mut draft.expression, ".MEAS card"),
    );
    commit(
        "measurement",
        measurement_authoring(ui, draft, &mut start_reference),
    );
    let invalid = draft.parse().err();
    if let Some(problem) = &invalid {
        let _ = crate::ui::widgets::problem_line(ui, Problem::Failure, problem);
    }

    if let (Some(property), None) = (committed, &invalid) {
        commit_draft(cx, definition, property);
    }

    verdict(ui, cx.model, &definition.measurement);
}

/// Commit the draft over `definition` when it changed it, and follow the
/// specification to its new measurement name when that changed.
fn commit_draft(
    cx: &mut Cx<'_, SimulateFeature>,
    definition: &SpecificationDefinition,
    property: &str,
) {
    let Some(Ok(Some(parsed))) = cx
        .state
        .verification
        .specification_draft
        .as_ref()
        .map(|held| held.draft.parse())
    else {
        return;
    };
    if parsed.bitwise_eq(definition) {
        return;
    }
    let id = definition.id;
    let measurement = definition.measurement.clone();
    let renamed = parsed.measurement.clone();
    let detail = format!(
        "Set the {property} of specification {}.",
        parsed.requirement_name
    );
    let committed = edit_specifications(cx, |definitions| {
        let index = definition_index(definitions, Some(id), &measurement)
            .ok_or_else(|| format!("{measurement} is no longer in the plan"))?;
        definitions[index] = parsed;
        Ok(detail)
    });
    match committed {
        Ok(_) => {
            cx.state.selected_specification = Some(renamed);
            cx.state.verification.action_problem = None;
        }
        Err(problem) => {
            cx.state.verification.action_problem = Some(format!(
                "The {property} of {} was not set: {problem}.",
                definition.requirement_name
            ));
        }
    }
}

/// One typed property. Returns whether the field let go of the keyboard,
/// which is when its words are committed.
fn text(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> bool {
    property_row_input_with_hint(ui, label, value, hint, false).lost_focus()
}

/// One property chosen from `options`, written back when another is picked.
fn pick<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, &str)],
    value: &mut T,
) -> bool {
    let mut selected = options
        .iter()
        .find(|(option, _)| *option == *value)
        .map(|(_, words)| (*words).to_owned())
        .unwrap_or_default();
    let shown = options
        .iter()
        .map(|(_, words)| ((*words).to_owned(), (*words).to_owned()))
        .collect::<Vec<_>>();
    let changed = property_row_combo(
        ui,
        label,
        ("verify.specification", label),
        &mut selected,
        &shown,
        true,
    );
    match options.iter().find(|(_, words)| *words == selected) {
        Some((option, _)) if changed && *option != *value => {
            *value = *option;
            true
        }
        _ => false,
    }
}

/// Which points the specification is judged at.
fn pick_scope(
    ui: &mut Ui,
    scopes: &[(String, SpecPointScope)],
    scope: &mut SpecPointScope,
) -> bool {
    let shown = scopes
        .iter()
        .map(|(words, _)| (words.clone(), words.clone()))
        .collect::<Vec<_>>();
    let mut selected = scopes
        .iter()
        .find(|(_, candidate)| *candidate == *scope)
        .map(|(words, _)| words.clone())
        .unwrap_or_default();
    let changed = property_row_combo(
        ui,
        "Applies to",
        "verify.specification.scope",
        &mut selected,
        &shown,
        true,
    );
    match scopes.iter().find(|(words, _)| *words == selected) {
        Some((_, picked)) if changed && *picked != *scope => {
            *scope = picked.clone();
            true
        }
        _ => false,
    }
}

/// Which of the plan's analyses must produce the measurement, or any.
fn pick_analysis(
    ui: &mut Ui,
    analyses: &[(AnalysisInstanceId, String)],
    producing: &mut Option<AnalysisInstanceId>,
) -> bool {
    let mut choices = vec![(None, "Any analysis".to_owned())];
    choices.extend(
        analyses
            .iter()
            .map(|(id, words)| (Some(*id), words.clone())),
    );
    if let Some(id) = *producing
        && !analyses.iter().any(|(candidate, _)| *candidate == id)
    {
        choices.push((Some(id), "An analysis no longer in the plan".to_owned()));
    }
    let key = |id: Option<AnalysisInstanceId>| id.map(|id| id.to_string()).unwrap_or_default();
    let shown = choices
        .iter()
        .map(|(id, words)| (key(*id), words.clone()))
        .collect::<Vec<_>>();
    let mut selected = key(*producing);
    let changed = property_row_combo(
        ui,
        "Produced by",
        "verify.specification.producer",
        &mut selected,
        &shown,
        true,
    );
    match choices.iter().find(|(id, _)| key(*id) == selected) {
        Some((id, _)) if changed && *id != *producing => {
            *producing = *id;
            true
        }
        _ => false,
    }
}

/// The plan's analyses, by identity and the name the plan lists them under.
fn analysis_options(model: &Model) -> Vec<(AnalysisInstanceId, String)> {
    model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            plan.instances()
                .iter()
                .enumerate()
                .map(|(index, instance)| {
                    (
                        instance.id(),
                        plan.instance_list_label(index)
                            .unwrap_or_else(|| instance.display_name().to_owned()),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

/// The point sets a specification can be judged at: every point, the nominal
/// one, or one process corner the run space declares — and `current`, named,
/// when it is none of those.
fn scope_options(model: &Model, current: &SpecPointScope) -> Vec<(String, SpecPointScope)> {
    let mut scopes = vec![
        ("Every point".to_owned(), SpecPointScope::AllPoints),
        ("The nominal point".to_owned(), SpecPointScope::Nominal),
    ];
    if let Some(dimension) = model
        .sim_setup
        .run_set
        .enabled_dimension_of(RunSetDimensionKind::ProcessSection)
    {
        scopes.extend(dimension.values.iter().map(|value| {
            let corner = value.lexical.trim().to_owned();
            (
                format!("Corner {corner}"),
                SpecPointScope::SelectedCorners {
                    corners: vec![corner],
                },
            )
        }));
    }
    if !scopes.iter().any(|(_, scope)| scope == current) {
        let words = match current {
            SpecPointScope::SelectedCorners { corners } => {
                format!("Corners {}", corners.join(" \u{b7} "))
            }
            SpecPointScope::AllPoints => "Every point".to_owned(),
            SpecPointScope::Nominal => "The nominal point".to_owned(),
        };
        scopes.push((words, current.clone()));
    }
    scopes
}

/// What the run says of the specification: its status, how many points
/// judged it, where its worst value was, how many trials answered it, and the
/// verdict.
fn verdict(ui: &mut Ui, model: &Model, measurement: &str) {
    section_header(ui, "Verdict", None);
    let Some(sheet) = model.simulation.verification_sheet() else {
        muted_inspector_copy(ui, "Nothing has been judged yet.");
        return;
    };
    if let Some(refusal) = sheet.refusal() {
        muted_inspector_copy(ui, &format!("Not judged: {refusal}."));
        return;
    }
    let Some(row) = sheet
        .rows
        .iter()
        .find(|row| row.measurement.eq_ignore_ascii_case(measurement))
    else {
        muted_inspector_copy(ui, "The run has not judged this specification.");
        return;
    };
    if let Some(status) = row.status {
        let problem = match status {
            RowStatus::Fail | RowStatus::Error => Some(Problem::Failure),
            RowStatus::Near | RowStatus::NotMeasured => Some(Problem::Warning),
            RowStatus::Pass | RowStatus::Waived | RowStatus::Tracked => None,
        };
        let _ = property_row_problem(ui, "Status", status_word(status), problem);
    }
    let _ = property_row(ui, "Points judged", &points_judged(sheet, row));
    if let Some(at) = row.worst.as_ref().and_then(|worst| worst_at(sheet, worst)) {
        let _ = property_row(ui, "Worst at", &at);
    }
    if let Some(statistics) = &row.statistics {
        let _ = property_row(ui, "Trials", &statistics.trials.len().to_string());
    }
    // The status above already says whether the limit held; what the verdict
    // adds is how many of the values it judged were in bound, or, with none,
    // why there were none.
    if let Some(verdict) = &row.verdict {
        let count = verdict.evidence_count();
        if count > 0 {
            let _ = property_row(
                ui,
                "In bound",
                &format!("{} of {count}", verdict.passing_evidence_count()),
            );
        } else {
            let words = match verdict.status() {
                SpecificationVerdictStatus::Pass => "Inside its limit",
                SpecificationVerdictStatus::BoundFailure => "Outside its limit",
                SpecificationVerdictStatus::MeasurementFailure => "The measurement failed",
                SpecificationVerdictStatus::MissingEvidence => "No value answered it",
            };
            let _ = property_row_wrapped(ui, "Verdict", words);
        }
    }
}

/// How many of the run's points judged the row: all of them, or how many of
/// how many when its scope leaves some out.
fn points_judged(sheet: &Sheet, row: &Row) -> String {
    let judged = row
        .cells
        .iter()
        .filter(|cell| !matches!(cell.status, CellStatus::Excluded | CellStatus::Absent))
        .count();
    if judged == sheet.points.len() {
        judged.to_string()
    } else {
        format!("{judged} of {}", sheet.points.len())
    }
}

/// A comparison table read in, or written out, through the shared file
/// picker, for a specification that references a result.
const TABLE: FileKind = FileKind {
    label: "Comparison tables",
    extensions: &["csv", "prn", "csd"],
    subject: "Reference table",
    fallback_name: "reference.csv",
};

/// The picker the reader opened, until it answers.
#[derive(Debug)]
struct PendingExchange {
    context: egui::Context,
    id: egui::Id,
    saving: bool,
}

impl Drop for PendingExchange {
    fn drop(&mut self) {
        file_exchange::discard_exchange(&self.context, self.id);
    }
}

impl ReferenceExchange for PendingExchange {
    fn poll(&self, ctx: &egui::Context) -> Option<Result<Option<ReferenceCompletion>, String>> {
        if self.saving {
            file_exchange::take_saved(ctx, self.id)
                .map(|outcome| outcome.map(|saved| saved.map(|_| ReferenceCompletion::Exported)))
        } else {
            file_exchange::take_opened(ctx, self.id).map(|outcome| {
                outcome.map(|opened| {
                    opened.map(|opened| ReferenceCompletion::Opened {
                        name: opened.name,
                        text: opened.text,
                    })
                })
            })
        }
    }
}

/// Open the picker: to read a table in, or to write the referenced one out.
fn start_reference(
    ctx: &egui::Context,
    source: Option<&MeasurementReferenceSource>,
) -> Result<Box<dyn ReferenceExchange>, String> {
    let pending = PendingExchange {
        context: ctx.clone(),
        id: egui::Id::new(uuid::Uuid::new_v4()),
        saving: source.is_some(),
    };
    let result = if let Some(source) = source {
        let name = std::path::Path::new(&source.logical_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(TABLE.fallback_name)
            .to_owned();
        file_exchange::save_file(
            ctx,
            pending.id,
            TABLE,
            name,
            source.contents.as_bytes().to_vec(),
        )
    } else {
        file_exchange::open_file(
            ctx,
            pending.id,
            TABLE,
            rspice_core::ResourceLimits::default().max_netlist_bytes,
        )
    };
    result.map(|()| Box::new(pending) as Box<dyn ReferenceExchange>)
}

#[cfg(test)]
mod tests;
