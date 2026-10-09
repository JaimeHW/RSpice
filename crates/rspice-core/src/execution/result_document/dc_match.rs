//! Reconcile mismatch report quantities before exporting or comparing them.

use super::*;
use crate::analysis::dcmatch::DcMatchScope;
use rspice_veriloga_runtime::arithmetic::ScaledValue;

fn invalid(detail: impl Into<String>) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "DC mismatch report",
        detail: detail.into(),
    }
}

fn close(actual: f64, expected: f64) -> bool {
    if actual == expected {
        return true;
    }
    if !actual.is_finite() || !expected.is_finite() || actual == 0.0 || expected == 0.0 {
        return false;
    }
    let scale = actual.abs().max(expected.abs());
    // No dimensionful absolute floor. Two adjacent subnormal roundings can
    // differ when a scope's sigma is retained before recombining the scopes.
    (actual / scale - expected / scale).abs() <= 64.0 * f64::EPSILON
        || (actual.is_sign_negative() == expected.is_sign_negative()
            && actual.to_bits().abs_diff(expected.to_bits()) <= 2)
}

fn product(left: f64, right: f64) -> Result<f64, ResultDocumentError> {
    let extended = ScaledValue::new(left).multiply(ScaledValue::new(right));
    let result = extended.binary64();
    if !result.is_finite() || (result == 0.0 && !extended.is_zero()) {
        return Err(invalid(
            "derived product exceeds finite representable precision",
        ));
    }
    Ok(result)
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let ResultPayload::DcMatch(payload) = &document.payload else {
        return Ok(());
    };
    check_abort(abort)?;
    // Basic payload validation has already checked finiteness, names, and
    // retained/evaluated counts. This pass checks the relationships between them.
    if document.point_count != 0 || !document.axes.is_empty() || !document.signals.is_empty() {
        return Err(invalid(
            "DC mismatch is a scalar report without primary series",
        ));
    }
    if payload.sigma_multiplier <= 0.0
        || payload.sigma_total < 0.0
        || payload.sigma_mismatch < 0.0
        || payload.sigma_process < 0.0
    {
        return Err(invalid(
            "sigmas must be nonnegative and the multiplier positive",
        ));
    }
    if !close(
        payload.sigma_total,
        libm::hypot(payload.sigma_mismatch, payload.sigma_process),
    ) {
        return Err(invalid(
            "total sigma disagrees with the two statistical scopes",
        ));
    }
    let quoted = product(payload.sigma_multiplier, payload.sigma_total)?;
    let mut unit = None;
    for (name, expected) in [
        ("nominal_value", payload.nominal_value),
        ("sigma_total", payload.sigma_total),
        ("sigma_mismatch", payload.sigma_mismatch),
        ("sigma_process", payload.sigma_process),
        ("quoted_sigma", quoted),
    ] {
        let scalar = document
            .scalars
            .iter()
            .find(|scalar| scalar.name == name)
            .ok_or_else(|| invalid(format!("missing '{name}' scalar")))?;
        if !matches!(scalar.value, ScalarValue::Real { value: Some(value) } if close(value, expected))
        {
            return Err(invalid(format!(
                "'{name}' scalar disagrees with the retained report"
            )));
        }
        if !matches!(scalar.unit, Some(SignalUnit::Volt | SignalUnit::Ampere))
            || unit.is_some_and(|unit| scalar.unit.as_ref() != Some(unit))
        {
            return Err(invalid(
                "output scalars must share the same voltage or current unit",
            ));
        }
        unit = scalar.unit.as_ref();
    }
    let mut identities = BTreeSet::new();
    for entry in &payload.contributors {
        check_abort(abort)?;
        if !identities.insert((
            entry.scope.tag(),
            entry.instance.to_ascii_lowercase(),
            entry.parameter.to_ascii_lowercase(),
        )) {
            return Err(invalid("duplicate statistical contributor identity"));
        }
        if entry.sigma_parameter < 0.0
            || !close(
                entry.contribution,
                product(entry.sensitivity, entry.sigma_parameter)?,
            )
        {
            return Err(invalid(
                "contributor displacement disagrees with sensitivity and parameter sigma",
            ));
        }
        if payload.sigma_total == 0.0 && entry.share != 0.0 {
            return Err(invalid(
                "zero total variance requires zero contributor shares",
            ));
        }
        if entry.contribution == 0.0 && entry.share != 0.0 {
            return Err(invalid("zero displacement requires a zero variance share"));
        }
    }
    let mut independent_variance = ScaledValue::new(0.0);
    for (scope, sigma, correlations) in [
        (
            DcMatchScope::Mismatch,
            payload.sigma_mismatch,
            payload.applied_correlations_mismatch,
        ),
        (
            DcMatchScope::Process,
            payload.sigma_process,
            payload.applied_correlations_process,
        ),
    ] {
        if correlations != 0 {
            // The covariance matrix is not retained. Correlated Euler shares
            // can be negative or greater than one; do not treat them as fractions.
            continue;
        }
        let one = ScaledValue::new(1.0);
        let squares = payload
            .contributors
            .iter()
            .filter(|entry| entry.scope == scope)
            .take_while(|_| !abort.is_aborted())
            .map(|entry| {
                let contribution = ScaledValue::new(entry.contribution);
                [contribution, contribution, one]
            });
        let variance = ScaledValue::sum_triple_products_ratio(squares, [[one; 3]].into_iter())
            .map_err(|_| invalid("independent variance exceeds arithmetic precision"))?;
        check_abort(abort)?;
        let retained_sigma = variance.sqrt().binary64();
        independent_variance = independent_variance.plus(variance);
        if !retained_sigma.is_finite()
            || (!close(sigma, retained_sigma)
                && (payload.contributors.len() == payload.evaluated_contributors
                    || sigma < retained_sigma))
        {
            return Err(invalid(
                "independent scope sigma disagrees with its retained contributors",
            ));
        }
        if payload.contributors.iter().any(|entry| {
            entry.scope == scope
                && (entry.share < 0.0 || (entry.share > 1.0 && !close(entry.share, 1.0)))
        }) {
            return Err(invalid(
                "independent variance shares must be between zero and one",
            ));
        }
    }
    let total_variance = if payload.contributors.len() == payload.evaluated_contributors
        && payload.applied_correlations_mismatch == 0
        && payload.applied_correlations_process == 0
    {
        Some(independent_variance)
    } else if payload.sigma_total >= f64::MIN_POSITIVE || payload.sigma_total == 0.0 {
        let sigma = ScaledValue::new(payload.sigma_total);
        Some(sigma.multiply(sigma))
    } else {
        // A rounded subnormal sigma cannot recover the omitted covariance
        // accurately enough to check individual shares. Its scope totals and
        // the independent shares' bounds are still checked above.
        None
    };
    if let Some(total) = total_variance {
        for entry in &payload.contributors {
            check_abort(abort)?;
            let correlations = match entry.scope {
                DcMatchScope::Mismatch => payload.applied_correlations_mismatch,
                DcMatchScope::Process => payload.applied_correlations_process,
            };
            if correlations != 0 {
                continue;
            }
            let contribution = ScaledValue::new(entry.contribution);
            let share = if total.is_zero() {
                ScaledValue::new(0.0)
            } else {
                contribution.multiply(contribution).divide(total)
            };
            let expected = share.binary64();
            if !expected.is_finite()
                || (expected == 0.0 && !share.is_zero())
                || !close(entry.share, expected)
            {
                return Err(invalid(
                    "independent variance share disagrees with its displacement and total variance",
                ));
            }
        }
    }
    Ok(())
}
