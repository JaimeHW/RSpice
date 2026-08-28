//! SPECS — the active retained dataset's governed measurement results.
//!
//! Each row binds an authored `.MEAS` expression and project-owned limit to
//! the exact retained value and worst source in the active immutable dataset.
//! Ambiguous lineage, missing results, and invalid evaluations fail closed;
//! the viewer and inspector share this same projection. Bounds and authored
//! expressions persist with the workspace ([`SpecEntry`]), while the docbar
//! opens their inline editor.

use std::borrow::Cow;
use std::collections::HashSet;

use egui::Ui;

use crate::quantity::engineering::parse_engineering_value;
use crate::state::{
    AnalysisResultFamilyMetadata, SimulationRun, SimulationRunLifecycle, SpecEntry, SpecPointScope,
    SpecificationComparison, SpecificationDefinition, SpecificationRole, SpecificationVerdict,
    SpecificationVerdictStatus,
};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;
use crate::workbench::design_system::{StatusMark, WorkbenchIcon, icon_button, paint_status_mark};

use super::{ResultViewer, well_hint};

/// The width each column wants, and the floor it may not be squeezed below.
///
/// A pair per column rather than one number, because the two were the same
/// number and the table was therefore a fixed 1042 points wide however narrow
/// the pane was. Below that width the last column — STATUS, which is the
/// verdict the whole table exists to state — was simply cut off by the pane
/// edge, at 1600 as well as at 1000, and a horizontal scroll bar under a table
/// is not where a reader looks for a pass/fail.
///
/// The floors are what each column can still be read at: a measurement name
/// and an expression elide legibly, a value and a margin are right-aligned
/// numbers, and the status word does not elide at all — so it keeps its width
/// and the prose columns give theirs up.
const SPEC_COLUMNS: [(f32, f32); 7] = [
    (160.0, 104.0),
    (200.0, 112.0),
    (116.0, 84.0),
    (150.0, 96.0),
    (140.0, 88.0),
    (168.0, 96.0),
    (92.0, 92.0),
];
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ComparisonDraftKind {
    #[default]
    Tracked,
    Minimum,
    Maximum,
    Range,
    EqualWithin,
}

impl ComparisonDraftKind {
    const ALL: [Self; 5] = [
        Self::Tracked,
        Self::Minimum,
        Self::Maximum,
        Self::Range,
        Self::EqualWithin,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Tracked => "Track only",
            Self::Minimum => "At least",
            Self::Maximum => "At most",
            Self::Range => "Inside range",
            Self::EqualWithin => "Equals within tolerance",
        }
    }
}

/// One governed requirement being edited. Numeric values remain text so
/// partial engineering notation survives frames. `original` carries the
/// immutable identity plus imported source and waiver metadata that ordinary
/// scalar edits are not authorized to rewrite.
#[derive(Debug, Clone, Default)]
pub struct SpecDraft {
    original: Option<SpecificationDefinition>,
    pub requirement_key: String,
    pub requirement_name: String,
    pub measurement: String,
    pub expression: String,
    comparison: ComparisonDraftKind,
    /// Minimum/maximum limit, range minimum, or equality target.
    pub primary_limit: String,
    /// Range maximum or equality tolerance.
    pub secondary_limit: String,
    pub guard_band: String,
    pub unit: String,
    pub role: SpecificationRole,
    pub scope: SpecPointScope,
    pub producing_analysis: Option<crate::product::AnalysisInstanceId>,
}

impl SpecDraft {
    fn from_definition(definition: &SpecificationDefinition) -> Self {
        let (comparison, primary_limit, secondary_limit) = match definition.comparison {
            SpecificationComparison::Tracked => {
                (ComparisonDraftKind::Tracked, String::new(), String::new())
            }
            SpecificationComparison::Minimum { limit } => (
                ComparisonDraftKind::Minimum,
                limit.to_string(),
                String::new(),
            ),
            SpecificationComparison::Maximum { limit } => (
                ComparisonDraftKind::Maximum,
                limit.to_string(),
                String::new(),
            ),
            SpecificationComparison::Range { minimum, maximum } => (
                ComparisonDraftKind::Range,
                minimum.to_string(),
                maximum.to_string(),
            ),
            SpecificationComparison::EqualWithin { target, tolerance } => (
                ComparisonDraftKind::EqualWithin,
                target.to_string(),
                tolerance.to_string(),
            ),
        };
        Self {
            original: Some(definition.clone()),
            requirement_key: definition.requirement_key.clone(),
            requirement_name: definition.requirement_name.clone(),
            measurement: definition.measurement.clone(),
            expression: definition.expression.clone(),
            comparison,
            primary_limit,
            secondary_limit,
            guard_band: definition
                .guard_band
                .map(|value| value.to_string())
                .unwrap_or_default(),
            unit: definition.unit.clone(),
            role: definition.role,
            scope: definition.scope.clone(),
            producing_analysis: definition.producing_analysis,
        }
    }

    /// Parse into a complete governed definition. `Err` identifies the field
    /// or domain invariant that refused the draft.
    fn parse(&self) -> Result<Option<SpecificationDefinition>, String> {
        let name = self.measurement.trim();
        if name.is_empty() {
            return Ok(None); // blank rows are simply dropped
        }
        let required_value = |text: &str, field: &str| -> Result<f64, String> {
            let text = text.trim();
            if text.is_empty() {
                return Err(format!("{field} is required"));
            }
            parse_engineering_value(text).map_err(|_| format!("{field} is invalid"))
        };
        let comparison = match self.comparison {
            ComparisonDraftKind::Tracked => SpecificationComparison::Tracked,
            ComparisonDraftKind::Minimum => SpecificationComparison::Minimum {
                limit: required_value(&self.primary_limit, "minimum limit")?,
            },
            ComparisonDraftKind::Maximum => SpecificationComparison::Maximum {
                limit: required_value(&self.primary_limit, "maximum limit")?,
            },
            ComparisonDraftKind::Range => SpecificationComparison::Range {
                minimum: required_value(&self.primary_limit, "range minimum")?,
                maximum: required_value(&self.secondary_limit, "range maximum")?,
            },
            ComparisonDraftKind::EqualWithin => SpecificationComparison::EqualWithin {
                target: required_value(&self.primary_limit, "equality target")?,
                tolerance: required_value(&self.secondary_limit, "equality tolerance")?,
            },
        };
        let projected = SpecEntry {
            measurement: name.to_owned(),
            expression: self.expression.trim().to_owned(),
            min: None,
            max: None,
            unit: self.unit.trim().to_owned(),
            scope: self.scope.clone(),
        };
        let mut definition = self
            .original
            .clone()
            .unwrap_or_else(|| SpecificationDefinition::new_from_projection(&projected));
        let requirement_key = self.requirement_key.trim();
        let requirement_name = self.requirement_name.trim();
        if !requirement_key.is_empty() {
            definition.requirement_key = requirement_key.to_owned();
        }
        definition.requirement_name = if requirement_name.is_empty() {
            name.to_owned()
        } else {
            requirement_name.to_owned()
        };
        definition.measurement = name.to_owned();
        definition.expression = self.expression.trim().to_owned();
        definition.producing_analysis = self.producing_analysis;
        definition.comparison = comparison;
        definition.guard_band = if self.guard_band.trim().is_empty() {
            None
        } else {
            Some(required_value(&self.guard_band, "guard band")?)
        };
        definition.role = self.role;
        definition.scope = self.scope.clone();
        definition.unit = self.unit.trim().to_owned();
        definition.validate()?;
        Ok(Some(definition))
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
            for name in analysis.scalar_evidence_names() {
                if seen.insert(name.to_ascii_lowercase()) {
                    names.push(name);
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
                .scalar_evidence(name)
                .into_iter()
                .map(move |evidence| MeasurementCandidate {
                    analysis_index,
                    analysis,
                    value: evidence.value,
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
    // The one spelling of a bound. This table formatted its own with a
    // plot-axis formatter, so a megahertz limit read "≥ 1.000 M Hz" here and
    // "≥ 1M Hz" on the studio page that authored it — a number a reader has to
    // translate before holding it against a datasheet.
    let limit = spec.map_or_else(|| "\u{2014}".to_owned(), SpecEntry::limit_text);
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

/// The requirement set one retained run is judged against.
///
/// A dispatched run freezes the specifications it was prepared with into its
/// receipt, and that set — never the workspace's currently authored one — is
/// the contract every surface reading the run must present. Only a legacy
/// dataset from before prepared-run receipts has no frozen set, and only for
/// those is the workspace's live contract an honest fallback.
///
/// One function, because the sheet resolved this and the CSV export and the
/// hardcopy capture did not: they re-resolved against the live workspace, so a
/// limit edited after a completed run put a bound the run had never been
/// judged against onto the exported and printed row, beside the verdict the
/// run had actually earned.
fn resolved_specifications<'a>(
    run: &'a SimulationRun,
    workspace_specs: &'a [SpecEntry],
) -> Cow<'a, [SpecEntry]> {
    run.prepared_receipt().map_or_else(
        || Cow::Borrowed(workspace_specs),
        |receipt| {
            Cow::Owned(
                receipt
                    .specifications()
                    .iter()
                    .map(|specification| specification.entry().clone())
                    .collect(),
            )
        },
    )
}

/// The requirements the workspace's active retained dataset was judged
/// against, owned.
///
/// The whole-state spelling of [`resolved_specifications`], for the hardcopy
/// capture, which takes a presentation of the Results workspace rather than
/// drawing one. It captured `workspace.specs` directly, and both the printed
/// table and the printed document's identity derive from what it captured —
/// so a limit edited after a completed run restated itself on the page beside
/// the verdict the run had actually earned, and moved the identity of an
/// immutable result.
pub(crate) fn active_run_specifications(state: &AppState) -> Vec<SpecEntry> {
    state.simulation.active_run().map_or_else(
        || state.workspace.specs.clone(),
        |run| resolved_specifications(run, &state.workspace.specs).into_owned(),
    )
}

/// The verdicts one retained run presents, and whether they are final.
struct RunSpecificationJudgment<'a> {
    verdicts: Cow<'a, [SpecificationVerdict]>,
    provisional: bool,
}

/// The verdicts for a run at any point in its life, from the one judge.
///
/// A terminal run carries the verdicts it sealed. A run still streaming is
/// judged by the same function the seal will use, over the evidence retained
/// so far, and its rows say that they are provisional. Before this, the
/// streaming rows came from [`result_row`]'s projection, which knows nothing
/// of guard bands, point scope or producing-analysis binding and refuses any
/// measurement more than one prepared task published — so rows flipped the
/// instant a run completed, and an ordinary corner sweep read `invalid` until
/// it did. [`result_row`] now answers only for legacy datasets, which have no
/// frozen requirement to judge and never had a sealed verdict either.
fn run_judgment(run: &SimulationRun) -> Option<RunSpecificationJudgment<'_>> {
    if let Some(sealed) = run.specification_verdicts() {
        return Some(RunSpecificationJudgment {
            verdicts: Cow::Borrowed(sealed),
            provisional: false,
        });
    }
    let receipt = run.prepared_receipt()?;
    if receipt.specifications().is_empty() {
        return None;
    }
    Some(RunSpecificationJudgment {
        verdicts: Cow::Owned(SpecificationVerdict::evaluate(
            receipt.specifications(),
            &run.analyses,
        )),
        provisional: true,
    })
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
    if let Some(judgment) = run_judgment(run) {
        for verdict in judgment.verdicts.iter() {
            let Some(row) = rows
                .iter_mut()
                .find(|row| row.measurement.eq_ignore_ascii_case(verdict.measurement()))
            else {
                continue;
            };
            row.value = verdict.worst_value();
            row.margin = verdict.signed_margin();
            row.source_analysis_index = verdict.source_instance_id().and_then(|source| {
                run.analyses.iter().position(|analysis| {
                    analysis.provenance().is_some_and(|provenance| {
                        provenance.authored_source_instance_id() == source
                    })
                })
            });
            row.worst_corner = row
                .source_analysis_index
                .and_then(|index| run.analyses.get(index))
                .and_then(exact_corner_label);
            row.status = match verdict.status() {
                SpecificationVerdictStatus::Pass if !row.is_bounded => SpecResultStatus::Unbound,
                SpecificationVerdictStatus::Pass => SpecResultStatus::Pass,
                SpecificationVerdictStatus::BoundFailure => SpecResultStatus::Fail,
                SpecificationVerdictStatus::MeasurementFailure => SpecResultStatus::Invalid,
                SpecificationVerdictStatus::MissingEvidence => SpecResultStatus::Missing,
            };
            let requirement_identity = verdict.requirement_key().map_or_else(
                || {
                    verdict.specification_id().map_or_else(
                        || "legacy requirement".to_owned(),
                        |id| format!("requirement {id}"),
                    )
                },
                |key| format!("requirement {key}"),
            );
            let passing_population = verdict.specification_id().map_or_else(String::new, |_| {
                format!(
                    ", of which {} satisfied the guard-banded bound",
                    verdict.passing_evidence_count()
                )
            });
            // A judgment made while the run is still producing evidence is
            // the same judgment, over less of it. It says so rather than
            // presenting itself as the sealed one.
            let standing = if judgment.provisional {
                "Provisional verdict, not yet sealed,"
            } else {
                "Immutable terminal verdict"
            };
            row.detail = format!(
                "{standing} for {requirement_identity} over {} attributed measurement{}{} from the frozen run specification.",
                verdict.evidence_count(),
                if verdict.evidence_count() == 1 {
                    ""
                } else {
                    "s"
                },
                passing_population,
            );
        }
    }
    rows
}

/// Serialize the same worst-case projection rendered by the Specs sheet,
/// against the same requirements it was rendered against.
///
/// `workspace_specs` is the legacy fallback only: the run's own frozen
/// contract is resolved here rather than by the caller, so an export arm
/// holding the live workspace cannot write a bound the sheet never showed.
pub(crate) fn export_csv(
    run: &SimulationRun,
    workspace_specs: &[SpecEntry],
) -> super::ResultSheetCsv {
    let specs = resolved_specifications(run, workspace_specs);
    let rows = result_rows(run, &specs);
    let mut contents = String::from(
        "measurement,expression,value,minimum,maximum,limit,margin,unit,scope,worst_corner,status,detail\n",
    );
    for row in &rows {
        let spec = specs
            .iter()
            .find(|spec| spec.measurement.eq_ignore_ascii_case(&row.measurement));
        let value = row.value.map(|value| format!("{value:.17e}"));
        let minimum = spec
            .and_then(|spec| spec.min)
            .map(|value| format!("{value:.17e}"));
        let maximum = spec
            .and_then(|spec| spec.max)
            .map(|value| format!("{value:.17e}"));
        let margin = row.margin.map(|value| format!("{value:.17e}"));
        let scope = spec
            .map(|spec| serde_json::to_string(&spec.scope).unwrap_or_else(|_| "null".to_owned()))
            .unwrap_or_default();
        contents.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{}\n",
            super::csv_field(&row.measurement),
            super::csv_field(&row.expression),
            super::csv_field(value.as_deref().unwrap_or_default()),
            super::csv_field(minimum.as_deref().unwrap_or_default()),
            super::csv_field(maximum.as_deref().unwrap_or_default()),
            super::csv_field(&row.limit),
            super::csv_field(margin.as_deref().unwrap_or_default()),
            super::csv_field(&row.unit),
            super::csv_field(&scope),
            super::csv_field(row.worst_corner.as_deref().unwrap_or_default()),
            row.status.label(),
            super::csv_field(&row.detail),
        ));
    }
    super::ResultSheetCsv {
        default_name: "rspice-specifications.csv",
        detail: format!("{} specification rows", rows.len()),
        contents,
    }
}

/// Renderer-neutral table projection shared with semantic hardcopy. This is
/// deliberately run-level: the sheet judges every retained source, not only
/// whichever analysis happens to be selected globally.
///
/// `workspace_specs` is the legacy fallback only, exactly as in
/// [`export_csv`]: what is printed is the contract the run froze.
pub(crate) fn hardcopy_table(
    run: &SimulationRun,
    workspace_specs: &[SpecEntry],
) -> super::ResultSheetTable {
    let specs = resolved_specifications(run, workspace_specs);
    let result_rows = result_rows(run, &specs);
    let rows = result_rows
        .iter()
        .map(|row| {
            let spec = specs
                .iter()
                .find(|spec| spec.measurement.eq_ignore_ascii_case(&row.measurement));
            let bounds = spec.map_or_else(
                || "not specified".to_owned(),
                |spec| match (spec.min, spec.max) {
                    (Some(minimum), Some(maximum)) => {
                        format!(
                            "{} <= value <= {}",
                            exact_value(minimum),
                            exact_value(maximum)
                        )
                    }
                    (Some(minimum), None) => format!("value >= {}", exact_value(minimum)),
                    (None, Some(maximum)) => format!("value <= {}", exact_value(maximum)),
                    (None, None) => "tracked without bounds".to_owned(),
                },
            );
            let scope = spec
                .map(|spec| {
                    serde_json::to_string(&spec.scope).unwrap_or_else(|_| "unavailable".to_owned())
                })
                .unwrap_or_default();
            vec![
                row.measurement.clone(),
                row.expression.clone(),
                row.value.map_or_else(String::new, exact_value),
                bounds,
                row.margin.map_or_else(String::new, exact_value),
                row.unit.clone(),
                scope,
                row.worst_corner.clone().unwrap_or_default(),
                row.status.label().to_owned(),
                row.detail.clone(),
            ]
        })
        .collect();
    super::ResultSheetTable {
        title: format!("Specifications · {}", run.label),
        columns: [
            "Measurement",
            "Expression",
            "Value",
            "Bounds",
            "Margin",
            "Unit",
            "Scope",
            "Worst corner",
            "Status",
            "Detail",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
        rows,
    }
}

fn exact_value(value: f64) -> String {
    format!("{value:.17e}")
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

/// The narrowest the table can be laid out at without cutting a column off.
fn table_minimum_width() -> f32 {
    SPEC_COLUMNS.iter().map(|(_, floor)| floor).sum::<f32>() + SPEC_TABLE_GUTTER
}

/// The width the table lays itself out at when nothing squeezes it.
fn table_width() -> f32 {
    SPEC_COLUMNS.iter().map(|(want, _)| want).sum::<f32>() + SPEC_TABLE_GUTTER
}

/// What each column gets in a table `content_width` wide.
///
/// Above the preferred total every column has what it wants. Below it, the
/// shortfall is taken from the columns in proportion to how much each has to
/// give — its preferred width less its floor — so the columns that elide
/// legibly give up the room and the verdict column keeps all of it. Below the
/// floors nothing more can be taken and the caller's horizontal scroll is what
/// is left.
fn spec_columns(content_width: f32) -> [f32; 7] {
    let preferred = table_width();
    let mut widths = SPEC_COLUMNS.map(|(want, _)| want);
    if content_width >= preferred {
        return widths;
    }
    let shortfall = preferred - content_width.max(table_minimum_width());
    let give: f32 = SPEC_COLUMNS.iter().map(|(want, floor)| want - floor).sum();
    if shortfall <= 0.0 || give <= 0.0 {
        return widths;
    }
    for (width, (want, floor)) in widths.iter_mut().zip(SPEC_COLUMNS) {
        *width = want - shortfall * (want - floor) / give;
    }
    widths
}

fn column_rect(row: egui::Rect, widths: &[f32; 7], index: usize) -> egui::Rect {
    let left = row.left() + widths[..index].iter().sum::<f32>();
    egui::Rect::from_min_size(
        egui::pos2(left, row.top()),
        egui::vec2(widths[index], row.height()),
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
    let widths = spec_columns(content_width);
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
        let cell = column_rect(header, &widths, index);
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

/// The row a hop last scrolled this table to, and the dataset it was in.
///
/// The dataset is half the key. The memo held the measurement name alone, so
/// selecting a different run while the same limit stayed carried left the
/// memo satisfied and the scroll request suppressed: the marked row sat below
/// the fold of a table the reader had no reason to think had scrolled at all.
#[derive(Clone, PartialEq, Eq)]
struct CarriedRowArrival {
    dataset: Option<crate::product::DatasetId>,
    measurement: String,
}

fn paint_result_row(
    ui: &mut Ui,
    row: &SpecResultRow,
    row_index: usize,
    content_width: f32,
    row_height: f32,
    max_margin: f64,
    carried: bool,
    scroll_into_view: bool,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let widths = spec_columns(content_width);
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
        node.set_selected(carried);
    });
    if carried {
        // A hop from the studio names one limit; the table has to show which
        // one arrived, or the reader is left to find it by eye in a table the
        // navigation already resolved.
        ui.painter().rect_filled(rect, 0.0, c.bg_active);
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            c.accent,
        );
        // Scrolled to once per arrival, not every frame: the carried selection
        // outlives the hop, and a row that re-centres itself forever would
        // take the scroll bar away from the reader.
        if scroll_into_view {
            response.scroll_to_me(Some(egui::Align::Center));
        }
    } else if row_index % 2 == 1 {
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
        let cell = column_rect(rect, &widths, index);
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
        column_rect(rect, &widths, 0).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        &row.measurement,
        theme::mono(tokens::FS_1, FontWeight::Regular),
        c.text,
    );
    paint_clipped_table_text(
        ui,
        column_rect(rect, &widths, 1).shrink2(egui::vec2(9.0, 0.0)),
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
        column_rect(rect, &widths, 2).shrink2(egui::vec2(9.0, 0.0)),
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
        column_rect(rect, &widths, 3).shrink2(egui::vec2(9.0, 0.0)),
        egui::Align2::LEFT_CENTER,
        &row.limit,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_dim,
    );

    let margin_cell = column_rect(rect, &widths, 4).shrink2(egui::vec2(9.0, 5.0));
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

    let corner_cell = column_rect(rect, &widths, 5).shrink2(egui::vec2(6.0, 2.0));
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

    let status_cell = column_rect(rect, &widths, 6).shrink2(egui::vec2(9.0, 0.0));
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
    dataset: Option<crate::product::DatasetId>,
    salt: impl std::hash::Hash + Copy + std::fmt::Debug,
    empty_message: Option<&str>,
    carried: Option<&str>,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let row_height = spec_table_row_height(t.metrics.ctl_h);
    // Which row the table has to bring into view, once per arrival: a limit
    // carried into a dataset it has not yet been shown in.
    let arrival = carried
        .filter(|measurement| {
            rows.iter()
                .any(|row| row.measurement.eq_ignore_ascii_case(measurement))
        })
        .map(|measurement| CarriedRowArrival {
            dataset,
            measurement: measurement.to_ascii_lowercase(),
        });
    let memo = egui::Id::new("rspice.results.specification-table.scrolled-to");
    let is_new_arrival = arrival.as_ref().is_some_and(|arrival| {
        ui.ctx()
            .data(|data| data.get_temp::<CarriedRowArrival>(memo))
            .as_ref()
            != Some(arrival)
    });
    if is_new_arrival && let Some(arrival) = arrival {
        ui.ctx().data_mut(|data| data.insert_temp(memo, arrival));
    }
    // The pane's width, floored at what the columns can still be read in
    // rather than at what they would prefer. Floored at the preferred width,
    // the table was a fixed 1042 points and the pane simply cut the last
    // column — the verdict — off its right edge at every width below that.
    let content_width = ui.available_width().max(table_minimum_width());
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
                                // Matched case-insensitively for the same
                                // reason the studio's own selection is: a
                                // measurement name is one identity however it
                                // was typed.
                                let selected = carried.is_some_and(|measurement| {
                                    measurement.eq_ignore_ascii_case(&row.measurement)
                                });
                                focused = paint_result_row(
                                    ui,
                                    row,
                                    index,
                                    content_width,
                                    row_height,
                                    max_margin,
                                    selected,
                                    selected && is_new_arrival,
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
    // The band reads left to right, title then verdict, and whatever will not
    // fit is lost from the end. The detail used to be right-aligned into the
    // room the title left, which is an arrangement that elides the *head*: in
    // the 721-point pane the results workspace gives this document at the
    // 1000-point gate, "0 / 2 pass · 2 unavailable · immutable · dataset
    // <uuid>" arrived as "/ 2 pass · …" — the pass count, which is the one
    // thing the band exists to state, cut off by its own left edge. Making the
    // detail shorter is not the fix on its own; a longer verdict would put it
    // straight back, so the layout is what changed.
    let title_font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let title_width = ui
        .painter()
        .layout_no_wrap(title.to_owned(), title_font.clone(), c.text)
        .size()
        .x;
    let split = (rect.left() + 10.0 + title_width + 16.0).min(rect.right() - 10.0);
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 10.0, rect.top()),
            egui::pos2(split, rect.bottom()),
        ),
        egui::Align2::LEFT_CENTER,
        title,
        title_font,
        c.text,
    );
    paint_clipped_table_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(split, rect.top()),
            egui::pos2(rect.right() - 10.0, rect.bottom()),
        ),
        egui::Align2::LEFT_CENTER,
        detail,
        summary_detail_font(),
        detail_color,
    );
}

/// The face the band's verdict is set in.
///
/// Named because a pin that asks whether the line fits has to measure the line
/// that is actually painted; a font spelled twice is a pin that can pass over a
/// band that clips.
fn summary_detail_font() -> egui::FontId {
    theme::mono(tokens::FS_0, FontWeight::Regular)
}

/// The verdict the band states for one retained run.
///
/// The dataset identity is elided to its first eight characters, the way every
/// other surface that has to fit one beside prose elides it. That is not on its
/// own the fix for the band clipping — the layout above is — but it is the
/// difference between a reader seeing the whole verdict and seeing most of it.
fn summary_detail(
    passing: usize,
    bounded: usize,
    unavailable: usize,
    lifecycle: SimulationRunLifecycle,
    dataset: crate::product::DatasetId,
) -> String {
    let unavailable_text = if unavailable > 0 {
        format!(" \u{b7} {unavailable} unavailable")
    } else {
        String::new()
    };
    format!(
        "{passing} / {bounded} pass{unavailable_text} \u{b7} {} \u{b7} dataset {}",
        lifecycle_label(lifecycle),
        crate::product::short_identity(dataset),
    )
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
            None,
            "no-active-dataset",
            Some(
                "No active dataset — select a retained run or run the simulation to evaluate specifications",
            ),
            None,
        );
        return;
    };

    let run_id = run.id;
    let dataset_id = run.dataset_id;
    let lifecycle = run.lifecycle;
    let specs = resolved_specifications(run, &state.workspace.specs);
    let rows = result_rows(run, &specs);
    let summary = summarize_rows(&rows);
    let passing = summary.passing;
    let bounded = summary.bounded;
    let unavailable = summary.unavailable;
    paint_summary(
        ui,
        &format!("Specifications · Run #{run_id}"),
        &summary_detail(passing, bounded, unavailable, lifecycle, dataset_id),
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
    // The studio's Requirements page and this table read one fact — which
    // limit is being looked at — rather than each keeping a selection of its
    // own, so a hop from either side arrives on the row the other named.
    let carried = state.workbench.selected_specification.clone();
    let focus_analysis = show_table_shell(
        ui,
        &rows,
        Some(dataset_id),
        dataset_id,
        Some(
            "No measurements — add .MEAS statements, run the simulation, then bind requirement limits",
        ),
        carried.as_deref(),
    );

    if let Some(analysis_index) = focus_analysis {
        let viewer = state
            .simulation
            .active_run()
            .and_then(|run| run.analyses.get(analysis_index))
            .map_or(ResultViewer::Manifest, source_viewer);
        if state.simulation.select_analysis(analysis_index) {
            state.ui.results.viewer = viewer;
            state.ui.results.clear_cursors();
        }
    }
}

/// The sheet that shows this retained analysis, in tab order.
///
/// Through the shared compatibility predicate and the shared pair refinement,
/// never a table of its own. This module kept an analysis-type-to-viewer map
/// beside them, and it drifted in two ways at once: it named viewers by
/// analysis kind rather than by the evidence the analysis actually retained —
/// so a Monte Carlo campaign or an optimizer run resolved to `Waves` — and it
/// asked whether its answer was available of the *globally selected* analysis
/// rather than the one whose row was clicked, which for every analysis-scoped
/// sheet is a different question. Both dropped the reader on the manifest
/// instead of the source they had asked for.
///
/// The Specs sheet itself is excluded: it is the sheet the button was pressed
/// on, so it is not somewhere to open.
fn source_viewer(analysis: &crate::state::AnalysisResult) -> ResultViewer {
    ResultViewer::PRIMARY
        .into_iter()
        .filter(|viewer| {
            *viewer != ResultViewer::Specs
                && super::view_context::analysis_supports_viewer(*viewer, analysis)
        })
        .map(|viewer| super::project_viewer_for_analysis(viewer, analysis))
        .next()
        .unwrap_or(ResultViewer::Manifest)
}

/// The inline governed-requirement editor. Exact comparison kind, limits,
/// guard band, role, and durable requirement identity remain explicit rather
/// than being flattened through the legacy min/max projection.
fn show_editor(ui: &mut Ui, state: &mut AppState) {
    let untracked = untracked_measurements(state);
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let analysis_options = state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            plan.instances()
                .iter()
                .enumerate()
                .map(|(index, instance)| {
                    (
                        instance.id(),
                        format!(
                            "{} · {}",
                            plan.instance_list_label(index)
                                .unwrap_or_else(|| instance.display_name().to_owned()),
                            instance.id()
                        ),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut scope_options = vec![
        ("All PVT points".to_owned(), SpecPointScope::AllPoints),
        ("Nominal only".to_owned(), SpecPointScope::Nominal),
    ];
    if let Some(dimension) = state
        .sim_setup
        .run_set
        .enabled_dimension_of(crate::simulation::run_set::RunSetDimensionKind::ProcessSection)
    {
        scope_options.extend(dimension.values.iter().map(|value| {
            let corner = value.lexical.trim().to_owned();
            (
                format!("Corner {corner} only"),
                SpecPointScope::SelectedCorners {
                    corners: vec![corner],
                },
            )
        }));
    }

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
                        "Limits and guard bands accept engineering notation (10meg, 1u, 3.3). \
                         Requirement identity, comparison semantics, and authored expressions are retained exactly; results never rewrite them.",
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
                    ("REQUIREMENT KEY", 130.0),
                    ("REQUIREMENT NAME", 180.0),
                    ("MEASUREMENT", 150.0),
                    ("EXPRESSION", 230.0),
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
                let field = |ui: &mut Ui, text: &mut String, width: f32, hint: &str| {
                    ui.add(
                        egui::TextEdit::singleline(text)
                            .desired_width(width)
                            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                            .hint_text(hint),
                    );
                };
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    field(ui, &mut draft.requirement_key, 130.0, "SPEC-…");
                    field(ui, &mut draft.requirement_name, 180.0, "requirement name");
                    field(ui, &mut draft.measurement, 150.0, "measurement");
                    field(ui, &mut draft.expression, 230.0, ".MEAS expression");
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
                });
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    egui::ComboBox::from_id_salt(("spec-comparison", idx))
                        .selected_text(draft.comparison.label())
                        .width(170.0)
                        .show_ui(ui, |ui| {
                            for comparison in ComparisonDraftKind::ALL {
                                ui.selectable_value(
                                    &mut draft.comparison,
                                    comparison,
                                    comparison.label(),
                                );
                            }
                        });
                    let (primary_hint, secondary_hint) = match draft.comparison {
                        ComparisonDraftKind::Tracked => ("no limit", ""),
                        ComparisonDraftKind::Minimum => ("minimum", ""),
                        ComparisonDraftKind::Maximum => ("maximum", ""),
                        ComparisonDraftKind::Range => ("range minimum", "range maximum"),
                        ComparisonDraftKind::EqualWithin => ("target", "tolerance"),
                    };
                    field(ui, &mut draft.primary_limit, 110.0, primary_hint);
                    field(ui, &mut draft.secondary_limit, 110.0, secondary_hint);
                    field(ui, &mut draft.guard_band, 100.0, "guard band");
                    field(ui, &mut draft.unit, 80.0, "unit");
                    egui::ComboBox::from_id_salt(("spec-role", idx))
                        .selected_text(match draft.role {
                            SpecificationRole::Blocking => "Blocking",
                            SpecificationRole::Review => "Review",
                            SpecificationRole::Informational => "Informational",
                        })
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut draft.role,
                                SpecificationRole::Blocking,
                                "Blocking",
                            );
                            ui.selectable_value(
                                &mut draft.role,
                                SpecificationRole::Review,
                                "Review",
                            );
                            ui.selectable_value(
                                &mut draft.role,
                                SpecificationRole::Informational,
                                "Informational",
                            );
                        });
                    if let Err(error) = draft.parse() {
                        ui.label(
                            egui::RichText::new(error)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(c.err),
                        );
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("APPLIES TO")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let current_scope = scope_options
                        .iter()
                        .find(|(_, scope)| scope == &draft.scope)
                        .map_or_else(
                            || match &draft.scope {
                                SpecPointScope::SelectedCorners { corners } => {
                                    format!("Corners {}", corners.join(" · "))
                                }
                                SpecPointScope::AllPoints => "All PVT points".to_owned(),
                                SpecPointScope::Nominal => "Nominal only".to_owned(),
                            },
                            |(label, _)| label.clone(),
                        );
                    egui::ComboBox::from_id_salt(("spec-scope", idx))
                        .selected_text(current_scope)
                        .width(180.0)
                        .show_ui(ui, |ui| {
                            for (label, scope) in &scope_options {
                                ui.selectable_value(&mut draft.scope, scope.clone(), label);
                            }
                        });
                    ui.label(
                        egui::RichText::new("PRODUCING ANALYSIS")
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                    let current_producer = draft.producing_analysis.map_or_else(
                        || "Any exact producer".to_owned(),
                        |id| {
                            analysis_options
                                .iter()
                                .find(|(candidate, _)| *candidate == id)
                                .map_or_else(
                                    || format!("Missing analysis · {id}"),
                                    |(_, label)| label.clone(),
                                )
                        },
                    );
                    egui::ComboBox::from_id_salt(("spec-producer", idx))
                        .selected_text(current_producer)
                        .width(280.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut draft.producing_analysis,
                                None,
                                "Any exact producer",
                            );
                            for (id, label) in &analysis_options {
                                ui.selectable_value(
                                    &mut draft.producing_analysis,
                                    Some(*id),
                                    label,
                                );
                            }
                        });
                });
                if let Some(original) = draft.original.as_ref() {
                    let source = original.source.as_ref().map_or_else(
                        || "authored in this plan".to_owned(),
                        |source| {
                            format!(
                                "{}:{} · revision {} · digest {}",
                                source.logical_path,
                                source.row,
                                source.imported_revision,
                                source.source_digest
                            )
                        },
                    );
                    let waiver = original.waiver.as_ref().map_or_else(
                        || "none".to_owned(),
                        |waiver| {
                            format!(
                                "{} · owner {} · {}",
                                waiver.reference, waiver.owner, waiver.rationale
                            )
                        },
                    );
                    ui.horizontal_wrapped(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "Stable ID {} · source {source} · waiver/disposition {waiver}",
                                original.id
                            ))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_faint),
                        );
                    });
                }
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
                            requirement_name: name.clone(),
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
    workspace.replace_active_specification_definitions(plan_id, specs);
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
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(crate::simulation::plan::SimulationPlan::id)
        .map_or_else(Vec::new, |plan_id| {
            state
                .workspace
                .plan_data(plan_id)
                .filter(|payload| !payload.specification_definitions.is_empty())
                .map(|payload| {
                    payload
                        .specification_definitions
                        .iter()
                        .map(SpecDraft::from_definition)
                        .collect()
                })
                .unwrap_or_else(|| {
                    state
                        .workspace
                        .specs
                        .iter()
                        .enumerate()
                        .map(|(index, entry)| {
                            SpecificationDefinition::from_legacy(plan_id, index, entry)
                        })
                        .map(|definition| SpecDraft::from_definition(&definition))
                        .collect()
                })
        });
    state.ui.results.spec_drafts = Some(drafts);
}

/// Right panel: the same active-dataset projection shown in the document.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(run) = state.simulation.active_run() else {
        let specs = &state.workspace.specs;
        if specs.is_empty() {
            section_header(ui, "Specs", None);
            measurement_table(ui, &[("Bounds", "none defined")]);
            return;
        }
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

    // Results are immutable evidence. The side inspector must therefore use
    // the same frozen receipt requirements as the document, even after the
    // active plan is edited or switched.
    let specs = resolved_specifications(run, &state.workspace.specs);
    if specs.is_empty() {
        section_header(ui, "Specs · active dataset", None);
        measurement_table(
            ui,
            &[("Bounds", "none were frozen into this dataset's run receipt")],
        );
        return;
    }
    let rows = result_rows(run, &specs);
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
    // The results panel is where a spec table gets read into a sign-off
    // package, so it is where an unqualified model has to be visible. It
    // annotates the dataset and changes nothing else: the rows, the margins and
    // the worst violation are all still exactly what the run measured.
    // The stamp names its cause. It printed the verdict alone, which is what a
    // surface says when it knows the answer and not the reason — and the
    // reason is on the receipt, one call away, in the same words Verify and the
    // requirements page use.
    let run_n = run
        .prepared_receipt()
        .and_then(crate::state::PreparedRunReceipt::sign_off_blocker)
        .map_or_else(
            || format!("run #{}", run.id),
            |blocker| format!("run #{} · NOT SIGN-OFF · {blocker}", run.id),
        );
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
mod tests;
