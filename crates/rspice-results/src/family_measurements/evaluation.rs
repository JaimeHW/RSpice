//! One measurement evaluated on every member of a result family, and what the
//! members' values add up to.
//!
//! A family is measured the way ADE Assembler and ADS measure one: the
//! expression is evaluated once per member, on that member's own data, and the
//! family's answer is the distribution of those values — never the expression
//! evaluated once over a curve that happens to string the members together.
//! `avg(V(out))` over a Monte Carlo family is ten averages, one per trial, and
//! their mean and spread; it is not the average of a line drawn through ten
//! trials.
//!
//! What a member supplies depends on how the family was retained:
//!
//! - an expanded family (a PVT point, a Monte Carlo trial or a `.STEP` point
//!   solved as its own task) gives each member its own waveforms, so any
//!   measurement a single run takes, a member takes;
//! - an in-analysis sweep or corner family retains one sample per member in
//!   shared waveforms, so a member is one row of them;
//! - an in-analysis Monte Carlo family retains each trial's drawn variables and
//!   `.MEAS` results, so a member is those named numbers.
//!
//! A member whose expression has no finite value keeps the reason. It is not
//! dropped from the count a yield is judged over: a member the measurement
//! could not take is not a member that met the specification.

use crate::analysis_result::AnalysisResult;
use crate::calculator::ast::CalculatorExpr;
use crate::calculator::evaluator::{self, CalcValue, EvaluationContext, EvaluationError};
use crate::calculator::retained::WaveformsContext;
use crate::calculator::value::RealValue;
use crate::family_measurements::FamilyMemberId;
use crate::family_metadata::AnalysisResultFamilyMetadata;
use crate::population::{self, ColumnKind, PopulationRequirements};
use crate::waveform::RetainedWaveform;
use crate::yield_analysis::YieldSpec;

/// Where one member's data lives.
pub enum MemberSource<'a, W> {
    /// The member's own retained waveforms: one solved point of an expanded
    /// family.
    Waveforms(&'a [W]),
    /// One row of waveforms the family's members share, one sample each.
    Row { waveforms: &'a [W], row: usize },
    /// Named numbers: a Monte Carlo trial's drawn variables and measurements.
    Scalars(Vec<(String, f64)>),
}

/// One member of a family, named the way a verdict names it.
pub struct FamilyMember<'a, W> {
    /// The member's place in the family's own order.
    pub index: usize,
    pub label: String,
    pub source: MemberSource<'a, W>,
}

/// What one member's evaluation gave.
#[derive(Debug, Clone, PartialEq)]
pub struct MemberValue {
    pub index: usize,
    pub label: String,
    /// The member's finite value, or why it has none.
    pub value: Result<f64, String>,
}

/// A measurement evaluated once per family member.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyMeasurement {
    pub members: Vec<MemberValue>,
}

/// The distribution of a family measurement's finite member values.
///
/// The standard deviation is the sample one, divided by n − 1, as the
/// population sheet, the histogram inspector and every Monte Carlo statistic
/// in the product already state it; below two values it is not defined, and
/// is `None` rather than zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FamilyStatistics {
    /// Members with a finite value.
    pub count: usize,
    /// Members the measurement could not take.
    pub unmeasured: usize,
    pub mean: f64,
    pub std_dev: Option<f64>,
    pub min: f64,
    pub max: f64,
}

/// A family measurement judged against a lower and/or upper limit.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyYield {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub passed: usize,
    /// Every member, measured or not. An unmeasured member fails.
    pub total: usize,
    /// Positions in [`FamilyMeasurement::members`] of the members that
    /// failed, in family order.
    pub failing: Vec<usize>,
}

impl FamilyYield {
    /// The passing share, in percent.
    #[must_use]
    pub fn percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * self.passed as f64 / self.total as f64
        }
    }
}

impl FamilyMeasurement {
    /// The finite member values, in family order.
    #[must_use]
    pub fn values(&self) -> Vec<f64> {
        self.members
            .iter()
            .filter_map(|member| member.value.as_ref().ok().copied())
            .collect()
    }

    /// Mean, standard deviation, minimum and maximum over the members that
    /// measured. `None` when none did.
    #[must_use]
    pub fn statistics(&self) -> Option<FamilyStatistics> {
        let values = self.values();
        Some(FamilyStatistics {
            count: values.len(),
            unmeasured: self.members.len() - values.len(),
            mean: population::mean(&values)?,
            std_dev: population::std_dev(&values),
            min: values.iter().copied().fold(f64::INFINITY, f64::min),
            max: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        })
    }

    /// The members inside `[lower, upper]`, either bound optional, judged as
    /// every other yield in the product is judged ([`YieldSpec::evaluates`]).
    /// `None` without any bound, or with bounds that admit nothing.
    #[must_use]
    pub fn yield_against(&self, lower: Option<f64>, upper: Option<f64>) -> Option<FamilyYield> {
        let spec = match (lower, upper) {
            (Some(lower), Some(upper)) if lower <= upper => YieldSpec::range("", lower, upper, ""),
            (Some(lower), None) => YieldSpec::lower("", lower, ""),
            (None, Some(upper)) => YieldSpec::upper("", upper, ""),
            _ => return None,
        };
        if [lower, upper]
            .into_iter()
            .flatten()
            .any(|bound| !bound.is_finite())
        {
            return None;
        }
        let failing: Vec<usize> = self
            .members
            .iter()
            .enumerate()
            .filter(|(_, member)| {
                !member
                    .value
                    .as_ref()
                    .is_ok_and(|value| spec.evaluates(*value))
            })
            .map(|(position, _)| position)
            .collect();
        Some(FamilyYield {
            lower,
            upper,
            passed: self.members.len() - failing.len(),
            total: self.members.len(),
            failing,
        })
    }
}

/// Evaluate `expression` once on each of `members`.
pub fn evaluate_members<W: AsRef<RetainedWaveform>>(
    expression: &CalculatorExpr,
    members: &[FamilyMember<'_, W>],
) -> FamilyMeasurement {
    FamilyMeasurement {
        members: members
            .iter()
            .map(|member| MemberValue {
                index: member.index,
                label: member.label.clone(),
                value: evaluate_member(expression, &member.source),
            })
            .collect(),
    }
}

fn evaluate_member<W: AsRef<RetainedWaveform>>(
    expression: &CalculatorExpr,
    source: &MemberSource<'_, W>,
) -> Result<f64, String> {
    let value = match source {
        MemberSource::Waveforms(waveforms) => {
            evaluator::evaluate(expression, &WaveformsContext::new(waveforms))
        }
        MemberSource::Row { waveforms, row } => {
            let rows = [*row];
            WaveformsContext::new(waveforms)
                .with_sample_projection(&rows, None)
                .and_then(|context| evaluator::evaluate(expression, &context))
        }
        MemberSource::Scalars(values) => evaluator::evaluate(expression, &NamedValues(values)),
    };
    finite_scalar(
        value
            .and_then(CalcValue::into_real)
            .map_err(|error| error.to_string())?,
    )
}

/// A member's number. A row member reads each signal as a one-sample trace,
/// and that one sample is the member's value of it.
fn finite_scalar(value: RealValue) -> Result<f64, String> {
    let value = match value {
        RealValue::Scalar(value) => value,
        RealValue::Waveform(_, samples) if samples.len() == 1 => samples[0],
        RealValue::Waveform(..) => {
            return Err(
                "The expression is a trace; reduce it with avg(), rms() or another scalar \
                 function."
                    .to_owned(),
            );
        }
    };
    if value.is_finite() {
        Ok(value)
    } else {
        Err("The expression has no finite value.".to_owned())
    }
}

/// A Monte Carlo trial's named numbers, matched the way every other
/// measurement join matches them: case-insensitively.
struct NamedValues<'a>(&'a [(String, f64)]);

impl EvaluationContext for NamedValues<'_> {
    fn get_waveform(
        &self,
        signal: &str,
        dataset: Option<&str>,
    ) -> Result<CalcValue, EvaluationError> {
        if dataset.is_some() {
            return Err(EvaluationError::IdentifierNotFound(format!(
                "{signal} of another dataset"
            )));
        }
        self.0
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(signal))
            .map(|(_, value)| CalcValue::from(RealValue::Scalar(*value)))
            .ok_or_else(|| {
                EvaluationError::IdentifierNotFound(format!("{signal} is not retained per trial"))
            })
    }
}

/// The members of the family `analysis` belongs to, or `None` when it
/// belongs to none.
///
/// `siblings` are the other analyses of the same run. An analysis whose run
/// solved several points of the one authored analysis (each with its own
/// execution identity) is a member of that expanded family, and so is each of
/// its siblings; an analysis that retains a sweep, corner or Monte Carlo
/// family in itself is that family. A lone analysis is not a family of one.
pub fn family_members<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    siblings: impl IntoIterator<Item = &'a AnalysisResult<W>>,
) -> Option<Vec<FamilyMember<'a, W>>> {
    if let Some(members) = in_analysis_members(analysis) {
        return Some(members);
    }
    let authored = analysis.provenance()?.authored_source_instance_id();
    let expanded: Vec<&AnalysisResult<W>> = siblings
        .into_iter()
        .filter(|sibling| {
            sibling.success
                && sibling.analysis_type == analysis.analysis_type
                && sibling
                    .provenance()
                    .is_some_and(|provenance| provenance.authored_source_instance_id() == authored)
        })
        .collect();
    (expanded.len() >= 2).then(|| {
        expanded
            .into_iter()
            .enumerate()
            .map(|(index, member)| FamilyMember {
                index,
                label: member
                    .provenance()
                    .and_then(|provenance| provenance.pvt_point())
                    .map_or_else(|| member.label.clone(), |point| point.label()),
                source: MemberSource::Waveforms(&member.waveforms),
            })
            .collect()
    })
}

fn in_analysis_members<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
) -> Option<Vec<FamilyMember<'_, W>>> {
    if !analysis.success {
        return None;
    }
    let metadata = analysis.family_metadata.as_ref()?;
    metadata.validate_for(analysis.analysis_type).ok()?;
    let rows = |labels: Vec<String>| -> Option<Vec<FamilyMember<'_, W>>> {
        let shared = analysis
            .waveforms
            .iter()
            .all(|waveform| waveform.as_ref().y.len() == labels.len());
        (shared && !labels.is_empty() && !analysis.waveforms.is_empty()).then(|| {
            labels
                .into_iter()
                .enumerate()
                .map(|(row, label)| FamilyMember {
                    index: row,
                    label,
                    source: MemberSource::Row {
                        waveforms: &analysis.waveforms,
                        row,
                    },
                })
                .collect()
        })
    };
    match metadata {
        AnalysisResultFamilyMetadata::Parametric { sweep_values, .. } => rows(
            sweep_values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    FamilyMemberId::SweepPoint {
                        index,
                        value: *value,
                    }
                    .label()
                })
                .collect(),
        ),
        AnalysisResultFamilyMetadata::Corner {
            corner_labels,
            temperatures_c,
            x_values,
            x_label,
            ..
        } => {
            let corners: Vec<String> = corner_labels
                .iter()
                .zip(temperatures_c)
                .map(|(label, temperature)| format!("{label} · {temperature} °C"))
                .collect();
            // A corner family laid along a swept axis has several rows per
            // corner, and only the swept value tells them apart.
            let repeated = corners
                .iter()
                .enumerate()
                .any(|(row, label)| corners[..row].contains(label));
            rows(if repeated {
                corners
                    .into_iter()
                    .zip(x_values)
                    .map(|(corner, x)| format!("{corner} · {x_label} {x}"))
                    .collect()
            } else {
                corners
            })
        }
        AnalysisResultFamilyMetadata::MonteCarlo { .. } => {
            let population = population::build(analysis, PopulationRequirements::Retained(&[]))?;
            // A sampled variable belongs to a trial only while the retained
            // rows are the sampled ordinals; otherwise it is another trial's.
            let columns: Vec<_> = population
                .columns
                .iter()
                .filter(|column| {
                    column.kind == ColumnKind::Measurement || population.variables_paired
                })
                .collect();
            Some(
                population
                    .trials
                    .iter()
                    .enumerate()
                    .map(|(row, trial)| FamilyMember {
                        index: row,
                        label: trial.label.clone(),
                        source: MemberSource::Scalars(
                            columns
                                .iter()
                                .filter_map(|column| {
                                    column
                                        .values
                                        .get(row)
                                        .copied()
                                        .flatten()
                                        .map(|value| (column.name.clone(), value))
                                })
                                .collect(),
                        ),
                    })
                    .collect(),
            )
        }
        AnalysisResultFamilyMetadata::Optimization { .. }
        | AnalysisResultFamilyMetadata::Soa { .. }
        | AnalysisResultFamilyMetadata::PeriodicNoise { .. }
        | AnalysisResultFamilyMetadata::SParameter { .. } => None,
    }
}

#[cfg(test)]
#[path = "evaluation_tests.rs"]
mod tests;
