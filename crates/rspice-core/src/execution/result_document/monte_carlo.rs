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
