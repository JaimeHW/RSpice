//! Specification report rows over retained run evidence and frozen requirements.
use super::SpecEntry;
use crate::{
    analysis_result::AnalysisResult,
    family_metadata::AnalysisResultFamilyMetadata,
    run::SimulationRun,
    specification_verdict::{SpecificationVerdict, SpecificationVerdictStatus},
    waveform::RetainedWaveform,
};
use std::{borrow::Cow, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecResultStatus {
    Pass,
    Fail,
    Unbound,
    Missing,
    Invalid,
}

impl SpecResultStatus {
    pub const fn label(self) -> &'static str {
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
pub struct SpecResultRow {
    pub measurement: String,
    pub expression: String,
    pub value: Option<f64>,
    pub limit: String,
    pub margin: Option<f64>,
    pub unit: String,
    pub is_bounded: bool,
    pub source_analysis_index: Option<usize>,
    pub worst_corner: Option<String>,
    pub status: SpecResultStatus,
    pub detail: String,
}

#[derive(Debug, Clone)]
struct MeasurementCandidate<'a, W> {
    unit_error: Option<String>,
    native_unit: Option<String>,
    analysis_index: usize,
    analysis: &'a AnalysisResult<W>,
    value: Option<f64>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SpecSummary {
    pub passing: usize,
    pub bounded: usize,
    pub failures: usize,
    pub unavailable: usize,
}

pub fn summarize_rows(rows: &[SpecResultRow]) -> SpecSummary {
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

fn signed_margin(spec: &SpecEntry, value: f64) -> Option<f64> {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => Some((value - minimum).min(maximum - value)),
        (Some(minimum), None) => Some(value - minimum),
        (None, Some(maximum)) => Some(maximum - value),
        (None, None) => None,
    }
}

fn exact_corner_label<W>(analysis: &AnalysisResult<W>) -> Option<String> {
    match analysis.family_metadata.as_ref() {
        Some(AnalysisResultFamilyMetadata::Corner { corner_labels, .. })
            if corner_labels.len() == 1 =>
        {
            corner_labels.first().cloned()
        }
        _ => None,
    }
}

fn measurement_candidates<'a, A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &'a SimulationRun<A>,
    name: &str,
    unit: &str,
) -> Vec<MeasurementCandidate<'a, W>> {
    run.analyses
        .iter()
        .map(AsRef::as_ref)
        .enumerate()
        .flat_map(|(analysis_index, analysis)| {
            analysis
                .scalar_evidence(name)
                .into_iter()
                .map(move |evidence| {
                    let converted = evidence.value_in_unit(unit);
                    MeasurementCandidate {
                        analysis_index,
                        analysis,
                        native_unit: evidence
                            .unit
                            .as_ref()
                            .and_then(|unit| unit.symbol())
                            .map(str::to_owned),
                        unit_error: converted.as_ref().err().cloned(),
                        value: converted.ok().flatten(),
                    }
                })
        })
        .collect()
}

fn candidates_share_source_lineage<W: AsRef<RetainedWaveform>>(
    candidates: &[MeasurementCandidate<'_, W>],
) -> bool {
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

/// Project one legacy measurement; governed run reports use [`result_rows`].
pub fn result_row<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &SimulationRun<A>,
    measurement: String,
    spec: Option<&SpecEntry>,
) -> SpecResultRow {
    let candidates = measurement_candidates(
        run,
        &measurement,
        spec.map_or("", |entry| entry.unit.as_str()),
    );
    // The one spelling of a bound. This table formatted its own with a
    // plot-axis formatter, so a megahertz limit read "≥ 1.000 M Hz" here and
    // "≥ 1M Hz" on the studio page that authored it — a number a reader has to
    // translate before holding it against a datasheet.
    let limit = spec.map_or_else(|| "\u{2014}".to_owned(), SpecEntry::limit_text);
    let mut unit = spec.map_or_else(String::new, |entry| entry.unit.clone());
    if unit.trim().is_empty()
        && let Some(native) = candidates
            .first()
            .and_then(|candidate| candidate.native_unit.as_ref())
        && candidates
            .iter()
            .all(|candidate| candidate.native_unit.as_ref() == Some(native))
    {
        unit = native.clone();
    }
    let expression = spec.map_or_else(String::new, |entry| entry.expression.clone());
    let is_bounded = spec.is_some_and(|entry| entry.min.is_some() || entry.max.is_some());
    if let Some(candidate) = candidates
        .iter()
        .find(|candidate| candidate.unit_error.is_some())
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
            detail: candidate.unit_error.clone().unwrap_or_default(),
        };
    }
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
        let candidate = &candidates[0];
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
    let candidate = worst.next().expect("minimum came from one candidate");
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
pub fn resolved_specifications<'a, A>(
    run: &'a SimulationRun<A>,
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
fn run_judgment<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &SimulationRun<A>,
) -> Option<RunSpecificationJudgment<'_>> {
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
            run.analyses.iter().map(AsRef::as_ref),
        )),
        provisional: true,
    })
}

/// Project the resolved requirements and retained measurements in their original order.
/// Resolve the run's frozen contract with [`resolved_specifications`] first.
pub fn result_rows<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &SimulationRun<A>,
    specs: &[SpecEntry],
) -> Vec<SpecResultRow> {
    let mut rows = Vec::new();
    let mut seen = HashSet::new();
    for spec in specs {
        if seen.insert(spec.measurement.to_ascii_lowercase()) {
            rows.push(result_row(run, spec.measurement.clone(), Some(spec)));
        }
    }
    for analysis in run.analyses.iter().map(AsRef::as_ref) {
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
                run.analyses.iter().map(AsRef::as_ref).position(|analysis| {
                    analysis.provenance().is_some_and(|provenance| {
                        provenance.authored_source_instance_id() == source
                    })
                })
            });
            row.worst_corner = row
                .source_analysis_index
                .and_then(|index| run.analyses.get(index).map(AsRef::as_ref))
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
            if verdict.status() == SpecificationVerdictStatus::MeasurementFailure
                && !row.unit.trim().is_empty()
            {
                let unit_error = run
                    .analyses
                    .iter()
                    .map(AsRef::as_ref)
                    .filter(|analysis| {
                        analysis.provenance().is_some_and(|provenance| {
                            Some(provenance.authored_source_instance_id())
                                == verdict.source_instance_id()
                        })
                    })
                    .find_map(|analysis| {
                        if let Some(worst) = verdict.worst_member() {
                            let evidence = analysis
                                .family_metadata
                                .as_ref()?
                                .member_measurements()
                                .iter()
                                .find(|member| &member.member == worst)?
                                .evidence_for(verdict.measurement())?;
                            evidence
                                .unit
                                .as_ref()?
                                .convert_value(evidence.value?, &row.unit)
                                .err()
                        } else {
                            analysis
                                .scalar_evidence(verdict.measurement())
                                .iter()
                                .find_map(|evidence| evidence.value_in_unit(&row.unit).err())
                        }
                    });
                if let Some(error) = unit_error {
                    row.detail.push(' ');
                    row.detail.push_str(&error);
                }
            }
        }
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_margin_is_positive_inside_and_negative_outside_each_bound_shape() {
        let two_sided = SpecEntry {
            measurement: "gain".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(10.0),
            max: Some(20.0),
            unit: "dB".to_owned(),
            scope: crate::specification::SpecPointScope::AllPoints,
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
}
