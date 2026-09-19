//! Original trial identities and explicit failures for statistical verdicts.

use super::*;
use crate::state::{FamilyMeasurementEvidence, FamilyMemberId, FamilyMemberMeasurements};
use rspice_core::analysis::monte_carlo::{MonteCarloResult, MonteCarloSampling};

pub(super) fn parameter_population(
    result: &MonteCarloResult,
    indices: &[usize],
    sampling: MonteCarloSampling,
    limit: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<FamilyMemberMeasurements>> {
    let mut names = result.variables.keys().collect::<Vec<_>>();
    names.sort();
    check_size(result.num_runs, names.len(), limit)?;
    let mut successful = 0;
    let mut members = Vec::with_capacity(result.num_runs);
    for index in 0..result.num_runs {
        poll_periodically(abort, index)?;
        let observed = indices.get(successful) == Some(&index);
        let measurements = names
            .iter()
            .enumerate()
            .map(|(column, name)| {
                poll_periodically(abort, column)?;
                let value = observed.then(|| result.variables[*name].samples[successful]);
                Ok(observation((*name).clone(), value))
            })
            .collect::<ServiceRunResult<Vec<_>>>()?;
        members.push(FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloSequenceTrial {
                index,
                seed: sampling.seed,
                policy: sampling.policy.into(),
            },
            measurements,
        ));
        successful += usize::from(observed);
    }
    Ok(members)
}

pub(super) fn include_deck_failures(
    successful: Vec<FamilyMemberMeasurements>,
    requested: usize,
    base_seed: u64,
    limit: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<FamilyMemberMeasurements>> {
    let names = successful
        .first()
        .map(|member| {
            member
                .measurements
                .iter()
                .map(|value| value.name.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    check_size(requested, names.len(), limit)?;
    let mut successful = successful.into_iter().peekable();
    let mut members = Vec::with_capacity(requested);
    for index in 0..requested {
        poll_periodically(abort, index)?;
        if successful
            .peek()
            .is_some_and(|member| member.member.index() == index)
        {
            members.push(successful.next().expect("observed trial"));
        } else {
            members.push(FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial {
                    index,
                    seed: trial_seed(base_seed, index),
                },
                names
                    .iter()
                    .enumerate()
                    .map(|(column, name)| {
                        poll_periodically(abort, column)?;
                        Ok(observation(name.clone(), None))
                    })
                    .collect::<ServiceRunResult<Vec<_>>>()?,
            ));
        }
    }
    Ok(members)
}

fn observation(name: String, value: Option<f64>) -> FamilyMeasurementEvidence {
    FamilyMeasurementEvidence {
        name,
        value,
        passed: value.is_some(),
        error: value
            .is_none()
            .then(|| "Monte Carlo trial did not converge".into()),
    }
}

fn check_size(rows: usize, columns: usize, limit: usize) -> ServiceRunResult<()> {
    // Include row identities as well as their observations before allocating.
    let requested = rows.saturating_mul(columns.saturating_add(1));
    if requested > limit {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            requested,
            limit,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monte_carlo_trial_evidence_checks_budget_before_allocation() {
        assert!(matches!(
            include_deck_failures(Vec::new(), usize::MAX, 7, 8, &NoAbort),
            Err(ServiceRunError::ResourceLimit(_))
        ));
    }
    #[test]
    fn monte_carlo_missing_deck_trials_keep_original_independent_seeds() {
        let observed = [0, 2]
            .into_iter()
            .map(|index| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloTrial {
                        index,
                        seed: trial_seed(7, index),
                    },
                    vec![observation("V(out)".into(), Some(index as f64))],
                )
            })
            .collect();
        let members = include_deck_failures(observed, 4, 7, 8, &NoAbort).unwrap();
        assert_eq!(members.len(), 4);
        for (index, member) in members.iter().enumerate() {
            assert_eq!(
                member.member,
                FamilyMemberId::MonteCarloTrial {
                    index,
                    seed: trial_seed(7, index)
                }
            );
            let value = &member.measurements[0];
            assert_eq!(value.is_measured(), index % 2 == 0);
            assert_eq!(value.error.is_some(), index % 2 != 0);
        }
    }
}
