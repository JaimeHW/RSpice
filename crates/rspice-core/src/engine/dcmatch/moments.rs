//! Extended-range mismatch displacements, covariance, and variance allocation.

use super::*;
use rspice_veriloga_runtime::arithmetic::ScaledValue;

#[cfg(test)]
mod tests;

fn retained(quantity: &str, value: ScaledValue) -> Result<Value, SimulationError> {
    let result = value.binary64();
    if !result.is_finite() || (result == 0.0 && !value.is_zero()) {
        return Err(SimulationError::Circuit(format!(
            ".DCMATCH {quantity} exceeds finite representable precision"
        )));
    }
    Ok(result)
}

fn sum(
    terms: impl Iterator<Item = [ScaledValue; 3]> + Clone,
    abort: &dyn AbortSignal,
) -> Result<ScaledValue, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let one = ScaledValue::new(1.0);
    // A cancelled partial accumulator is discarded below. Each recovery pass
    // stops consuming the iterator promptly, including inside a large group.
    let result = ScaledValue::sum_triple_products_ratio(
        terms.take_while(|_| !abort.is_aborted()),
        [[one, one, one]].into_iter(),
    );
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    result.map_err(|error| {
        SimulationError::Circuit(format!(".DCMATCH covariance arithmetic: {error:?}"))
    })
}

pub(super) fn central_difference(
    up: Value,
    down: Value,
    sigma: Value,
) -> Result<Value, SimulationError> {
    if !sigma.is_finite() || sigma <= 0.0 {
        return Err(SimulationError::Circuit(
            ".DCMATCH derivative requires a finite positive parameter sigma".into(),
        ));
    }
    let one = ScaledValue::new(1.0);
    let difference = ScaledValue::sum_products_div(
        [[ScaledValue::new(up), one], [ScaledValue::new(-down), one]].into_iter(),
        ScaledValue::new(sigma).multiply(ScaledValue::new(2.0)),
    )
    .map_err(|error| {
        SimulationError::Circuit(format!(".DCMATCH derivative arithmetic: {error:?}"))
    })?;
    retained("sensitivity", difference)
}

pub(super) fn contributor(
    scope: DcMatchScope,
    instance: String,
    parameter: String,
    sigma_parameter: Value,
    sensitivity: Value,
) -> Result<DcMatchContributor, SimulationError> {
    let contribution = retained(
        "contributor displacement",
        ScaledValue::new(sensitivity).multiply(ScaledValue::new(sigma_parameter)),
    )?;
    Ok(DcMatchContributor {
        instance,
        parameter,
        scope,
        sigma_parameter,
        sensitivity,
        contribution,
        share: 0.0,
    })
}

const CANCELLATION_TOLERANCE: Value = 1e-12;

fn positive_variance(
    scope: DcMatchScope,
    variance: ScaledValue,
    magnitude: ScaledValue,
) -> Result<ScaledValue, SimulationError> {
    if variance.is_zero() {
        return Ok(ScaledValue::new(0.0));
    }
    if variance.is_finite() && !variance.binary64().is_sign_negative() {
        return Ok(variance);
    }
    if variance.is_finite()
        && !magnitude.is_zero()
        && -variance.divide(magnitude).binary64() <= CANCELLATION_TOLERANCE
    {
        return Ok(ScaledValue::new(0.0));
    }
    Err(SimulationError::Circuit(format!(
        ".DCMATCH formed a negative or invalid {} variance from a validated correlation matrix",
        scope.tag()
    )))
}

fn scope_variance(
    scope: DcMatchScope,
    contributors: &[DcMatchContributor],
    correlation: Option<&ScopeCorrelation>,
    allocations: &mut [ScaledValue],
    abort: &dyn AbortSignal,
) -> Result<ScaledValue, SimulationError> {
    let one = ScaledValue::new(1.0);
    let squares = contributors
        .iter()
        .filter(|entry| entry.scope == scope)
        .map(|entry| {
            let c = ScaledValue::new(entry.contribution);
            [c, c, one]
        });
    let magnitude = sum(squares, abort)?;
    let Some(correlation) = correlation else {
        for (index, entry) in contributors.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if entry.scope == scope {
                let c = ScaledValue::new(entry.contribution);
                allocations[index] = c.multiply(c);
            }
        }
        return Ok(magnitude);
    };
    let mut groups = BTreeMap::<&str, Vec<(usize, usize)>>::new();
    for (index, entry) in contributors.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if entry.scope == scope {
            groups
                .entry(&entry.instance)
                .or_default()
                .push((index, correlation.row(scope, &entry.parameter)?));
        }
    }
    let matrix = correlation.matrix.values();
    let term = |(i, row): (usize, usize), (j, column): (usize, usize)| {
        [
            ScaledValue::new(contributors[i].contribution),
            ScaledValue::new(matrix[row][column]),
            ScaledValue::new(contributors[j].contribution),
        ]
    };
    for members in groups.values() {
        for &left in members {
            allocations[left.0] = sum(members.iter().map(|&right| term(left, right)), abort)?;
        }
    }
    // Sum the original quadratic terms together. Summing rounded row
    // allocations could erase a small variance between large cancelling rows.
    let terms = groups.values().flat_map(|members| {
        members
            .iter()
            .flat_map(move |&left| members.iter().map(move |&right| term(left, right)))
    });
    positive_variance(scope, sum(terms, abort)?, magnitude)
}

pub(super) fn assemble(
    card: &DcMatchCard,
    output: String,
    nominal_value: Value,
    mut contributors: Vec<DcMatchContributor>,
    correlations: &ScopeCorrelations,
    abort: &dyn AbortSignal,
) -> Result<DcMatchResult, SimulationError> {
    let mut allocations = vec![ScaledValue::new(0.0); contributors.len()];
    let mismatch = scope_variance(
        DcMatchScope::Mismatch,
        &contributors,
        correlations.for_scope(DcMatchScope::Mismatch),
        &mut allocations,
        abort,
    )?;
    let process = scope_variance(
        DcMatchScope::Process,
        &contributors,
        correlations.for_scope(DcMatchScope::Process),
        &mut allocations,
        abort,
    )?;
    let total = mismatch.plus(process);
    if !total.is_zero() {
        for (entry, allocation) in contributors.iter_mut().zip(allocations) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            entry.share = retained("variance share", allocation.divide(total))?;
        }
    }
    contributors.sort_by(|left, right| {
        right
            .share
            .abs()
            .total_cmp(&left.share.abs())
            .then_with(|| left.scope.tag().cmp(right.scope.tag()))
            .then_with(|| left.instance.cmp(&right.instance))
            .then_with(|| left.parameter.cmp(&right.parameter))
    });
    let evaluated_contributors = contributors.len();
    contributors.retain(|entry| entry.share.abs() >= card.threshold);
    if card.contributor_limit > 0 {
        contributors.truncate(card.contributor_limit);
    }
    let sigma_total = retained("total sigma", total.sqrt())?;
    retained(
        "quoted sigma",
        ScaledValue::new(card.sigma_multiplier).multiply(ScaledValue::new(sigma_total)),
    )?;
    Ok(DcMatchResult {
        output,
        nominal_value,
        sigma_multiplier: card.sigma_multiplier,
        sigma_total,
        sigma_mismatch: retained("mismatch sigma", mismatch.sqrt())?,
        sigma_process: retained("process sigma", process.sqrt())?,
        contributors,
        evaluated_contributors,
        applied_correlations_mismatch: correlations.statements(DcMatchScope::Mismatch),
        applied_correlations_process: correlations.statements(DcMatchScope::Process),
    })
}
