//! Reconcile retained trials, campaign accounting, and confidence declarations.

use super::*;
use std::collections::BTreeMap;

fn invalid(detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "Monte Carlo report",
        detail: detail.into(),
    }
}

struct Scalars<'a>(BTreeMap<&'a str, &'a ResultScalar>);

impl Scalars<'_> {
    fn count(&self, name: &str) -> Result<Option<u64>, ResultDocumentError> {
        self.0
            .get(name)
            .map(|scalar| match scalar.value {
                ScalarValue::Count { value } => Ok(value),
                _ => Err(invalid(format!("'{name}' must be an exact unsigned count"))),
            })
            .transpose()
    }

    fn boolean(&self, name: &str) -> Result<Option<bool>, ResultDocumentError> {
        self.0
            .get(name)
            .map(|scalar| match scalar.value {
                ScalarValue::Boolean { value } => Ok(value),
                _ => Err(invalid(format!("'{name}' must be a boolean"))),
            })
            .transpose()
    }
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let ResultPayload::MonteCarlo(payload) = &document.payload else {
        return Ok(());
    };
    check_abort(abort)?;
    let scalars = Scalars(
        document
            .scalars
            .iter()
            .map(|scalar| (scalar.name.as_str(), scalar))
            .collect(),
    );
    let population = payload
        .successful_trial_indices
        .as_ref()
        .map(Vec::len)
        .or_else(|| {
            payload
                .statistics
                .first()
                .map(|variable| variable.samples.len())
        });
    let mut names = BTreeSet::new();
    for variable in &payload.statistics {
        check_abort(abort)?;
        if !names.insert(&variable.name) {
            return Err(invalid("duplicate case-sensitive variable identity"));
        }
        if population != Some(variable.samples.len()) {
            return Err(invalid(
                "variables do not cover the same retained trial population",
            ));
        }
        if variable.standard_deviation.is_some_and(|value| value < 0.0)
            || variable
                .minimum
                .zip(variable.maximum)
                .is_some_and(|(min, max)| min > max)
        {
            return Err(invalid(
                "standard deviation must be nonnegative and extrema ordered",
            ));
        }
        validate_statistics(variable, abort)?;
    }
    let completed = scalars.count("completed_runs")?;
    let failed = scalars.count("failed_runs")?;
    let stated_successful = scalars.count("successful_runs")?;
    let population = population.map(|count| count as u64);
    if stated_successful
        .zip(population)
        .is_some_and(|(stated, retained)| stated != retained)
    {
        return Err(invalid(
            "successful run count disagrees with retained trials",
        ));
    }
    let successful = stated_successful.or(population);
    if completed
        .zip(failed)
        .is_some_and(|(all, failed)| failed > all)
        || completed
            .zip(successful)
            .is_some_and(|(all, successful)| successful > all)
        || completed
            .zip(failed)
            .zip(successful)
            .is_some_and(|((all, failed), successful)| all.checked_sub(failed) != Some(successful))
    {
        return Err(invalid(
            "completed, failed, and successful run counts disagree",
        ));
    }
    let inferred_failures = failed.or_else(|| {
        completed
            .zip(successful)
            .and_then(|(all, successful)| all.checked_sub(successful))
    });
    if scalars
        .boolean("all_converged")?
        .zip(inferred_failures)
        .is_some_and(|(converged, failed)| converged != (failed == 0))
    {
        return Err(invalid("convergence flag disagrees with failed run count"));
    }
    if scalars
        .boolean("mean_confidence_conditional_on_success")?
        .zip(inferred_failures)
        .is_some_and(|(conditional, failed)| conditional != (failed != 0))
    {
        return Err(invalid(
            "confidence conditioning disagrees with failed run count",
        ));
    }
    // Older reports can retain absolute trial identities without retaining
    // the attempted range. Missing range metadata must not imply a zero start.
    let first = scalars.count("first_trial")?;
    let end = first
        .zip(completed)
        .map(|(first, count)| {
            first
                .checked_add(count)
                .ok_or_else(|| invalid("trial range exceeds the supported integer range"))
        })
        .transpose()?;
    if let Some(indices) = &payload.successful_trial_indices {
        for &index in indices {
            check_abort(abort)?;
            if first.is_some_and(|first| (index as u64) < first)
                || end.is_some_and(|end| index as u64 >= end)
            {
                return Err(invalid(
                    "trial identity lies outside the completed campaign range",
                ));
            }
        }
    }
    scalars.count("sampling_seed")?;
    scalars.count("mean_confidence_bootstrap_seed")?;
    if scalars
        .count("mean_confidence_bootstrap_resamples")?
        .is_some_and(|count| count < 2)
    {
        return Err(invalid(
            "bootstrap confidence requires at least two resamples",
        ));
    }
    if let Some(level) = scalars.0.get("mean_confidence_level_pct")
        && !matches!(level.value, ScalarValue::Real { value: Some(value) } if value > 0.0 && value < 100.0)
    {
        return Err(invalid(
            "confidence level must lie strictly between zero and 100 percent",
        ));
    }
    let mut confidence_fields = BTreeSet::new();
    for variable in &payload.statistics {
        check_abort(abort)?;
        let identity = wire::encode_hex(variable.name.as_bytes());
        let fields =
            ["lower", "upper", "state"].map(|field| format!("mean_confidence_{field}:{identity}"));
        let values = fields
            .each_ref()
            .map(|field| scalars.0.get(field.as_str()).copied());
        confidence_fields.extend(fields);
        if values.iter().all(Option::is_none) {
            continue;
        }
        let [Some(lower), Some(upper), Some(state)] = values else {
            return Err(invalid(
                "confidence bounds and availability must be declared together",
            ));
        };
        let (
            ScalarValue::Real { value: lower_value },
            ScalarValue::Real { value: upper_value },
            ScalarValue::Text { value: state },
        ) = (&lower.value, &upper.value, &state.value)
        else {
            return Err(invalid(
                "confidence bounds or availability use an invalid scalar type",
            ));
        };
        let samples = variable.samples.len();
        let valid = match state.as_str() {
            "available" => {
                samples >= 2
                    && lower_value
                        .zip(*upper_value)
                        .is_some_and(|(lower, upper)| lower <= upper)
            }
            "insufficient_samples" => samples < 2 && lower_value.is_none() && upper_value.is_none(),
            "unrepresentable" => samples >= 2 && lower_value.is_none() && upper_value.is_none(),
            _ => false,
        };
        if !valid {
            return Err(invalid(
                "confidence availability contradicts its bounds or sample population",
            ));
        }
        if lower.unit != upper.unit
            || variable
                .unit
                .as_ref()
                .is_some_and(|unit| lower.unit.as_ref() != Some(unit))
        {
            return Err(invalid("confidence bounds disagree with the output unit"));
        }
    }
    for scalar in &document.scalars {
        check_abort(abort)?;
        if [
            "mean_confidence_lower:",
            "mean_confidence_upper:",
            "mean_confidence_state:",
        ]
        .iter()
        .any(|prefix| scalar.name.starts_with(prefix))
            && !confidence_fields.contains(&scalar.name)
        {
            return Err(invalid(
                "confidence declaration refers to an unretained variable",
            ));
        }
    }
    Ok(())
}

/// Reconcile summaries with their retained evidence without fabricating any
/// missing sample or recomputing a producer's rounded moment estimates.
fn validate_statistics(
    variable: &MonteCarloVariableStatistics,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let mut observed_min = f64::INFINITY;
    let mut observed_max = f64::NEG_INFINITY;
    let mut observed = 0;
    for (index, sample) in variable.samples.iter().enumerate() {
        if index % 64 == 0 {
            check_abort(abort)?;
        }
        if let Some(value) = sample {
            observed_min = observed_min.min(*value);
            observed_max = observed_max.max(*value);
            observed += 1;
        }
    }
    let complete = observed != 0 && observed == variable.samples.len();
    if variable
        .minimum
        .is_some_and(|min| min > observed_min || complete && min != observed_min)
        || variable
            .maximum
            .is_some_and(|max| max < observed_max || complete && max != observed_max)
    {
        return Err(invalid("extrema disagree with retained samples"));
    }
    let (minimum, maximum) = if complete {
        (Some(observed_min), Some(observed_max))
    } else {
        (variable.minimum, variable.maximum)
    };
    if variable.mean.is_some_and(|mean| {
        minimum.is_some_and(|min| mean < min) || maximum.is_some_and(|max| mean > max)
    }) {
        return Err(invalid("mean lies outside the sample extrema"));
    }

    if variable.histogram.is_empty() {
        return if variable.bin_edges.is_empty() {
            Ok(())
        } else {
            Err(invalid("histogram edges have no corresponding bins"))
        };
    }
    // Payload shape/finite checks have already admitted exactly n+1 edges.
    // Repeated edges in legacy reports are allowed only when their counts
    // agree with the same half-open intervals used by current producers.
    let mut total = 0usize;
    let intervals = variable
        .bin_edges
        .iter()
        .zip(variable.bin_edges.iter().skip(1));
    for (index, (&count, (&lower, &upper))) in variable.histogram.iter().zip(intervals).enumerate()
    {
        if index % 64 == 0 {
            check_abort(abort)?;
        }
        if lower > upper {
            return Err(invalid("histogram edges must be ordered"));
        }
        if count != 0 && index + 1 < variable.histogram.len() && lower == upper {
            return Err(invalid(
                "an empty histogram interval cannot contain samples",
            ));
        }
        total = total
            .checked_add(count)
            .ok_or_else(|| invalid("histogram population overflows its count type"))?;
    }
    if total != variable.samples.len() {
        return Err(invalid(
            "histogram counts disagree with the retained trial population",
        ));
    }
    let (Some(&first), Some(&last)) = (variable.bin_edges.first(), variable.bin_edges.last())
    else {
        return Err(invalid("histogram has no bin edges"));
    };
    if minimum.is_some_and(|min| min < first) || maximum.is_some_and(|max| max > last) {
        return Err(invalid("histogram edges do not enclose the sample extrema"));
    }
    let mut remaining = variable.histogram.clone();
    for (index, sample) in variable.samples.iter().enumerate() {
        if index % 64 == 0 {
            check_abort(abort)?;
        }
        let Some(sample) = sample else { continue };
        if *sample < first || *sample > last {
            return Err(invalid("histogram edges do not enclose retained samples"));
        }
        let bin = variable.bin_edges.partition_point(|edge| edge <= sample) - 1;
        let bin = bin.min(remaining.len() - 1);
        let count = remaining
            .get_mut(bin)
            .ok_or_else(|| invalid("histogram sample has no corresponding bin"))?;
        *count = count
            .checked_sub(1)
            .ok_or_else(|| invalid("histogram bin counts disagree with retained samples"))?;
    }
    Ok(())
}
