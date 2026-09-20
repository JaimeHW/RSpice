//! Validate complete parameter-stream trial evidence at every persistence boundary.

use super::{FamilyMemberId, FamilyMemberMeasurements};
use std::collections::{BTreeMap, HashSet};

impl FamilyMemberMeasurements {
    /// Legacy independent-trial records may contain only a partial roster.
    /// Sequence records require a complete population with exact sample attribution.
    pub(crate) fn validate_monte_carlo_sequence<'a>(
        members: &[Self],
        seed: u64,
        requested: usize,
        completed: usize,
        failures: usize,
        variables: impl IntoIterator<Item = (&'a str, &'a [f64])>,
    ) -> Result<(), String> {
        if !members.iter().any(|member| {
            matches!(
                member.member,
                FamilyMemberId::MonteCarloSequenceTrial { .. }
            )
        }) {
            return Ok(());
        }
        let invalid = || {
            "Monte Carlo sequence evidence disagrees with the retained trial population".to_owned()
        };
        if requested == 0
            || members.len() != requested
            || completed.checked_add(failures) != Some(requested)
        {
            return Err(invalid());
        }
        let mut columns = BTreeMap::new();
        for (name, samples) in variables {
            if name.trim().is_empty()
                || samples.len() != completed
                || samples.iter().any(|value| !value.is_finite())
                || columns.insert(name.to_ascii_lowercase(), samples).is_some()
            {
                return Err(invalid());
            }
        }
        if columns.is_empty() {
            return Err(invalid());
        }
        let first_trial = members[0].member.index();
        if first_trial.checked_add(requested).is_none() {
            return Err(invalid());
        }
        let mut successful = 0;
        let mut policy = None;
        for (position, member) in members.iter().enumerate() {
            let FamilyMemberId::MonteCarloSequenceTrial {
                index,
                seed: recorded_seed,
                policy: recorded_policy,
            } = &member.member
            else {
                return Err(invalid());
            };
            if *index != first_trial + position
                || *recorded_seed != seed
                || !matches!(
                    recorded_policy.as_str(),
                    "parameter-xoroshiro128plus-2018-v1"
                        | "spectre-coordinate-splitmix64-v1"
                        | "deck-expressions-and-spectre-coordinate-v1"
                )
                || policy.is_some_and(|value| value != recorded_policy.as_str())
            {
                return Err(invalid());
            }
            policy = Some(recorded_policy.as_str());
            if member.measurements.len() != columns.len() {
                return Err(invalid());
            }
            let observed = member.measurements[0].value.is_some();
            let mut names = HashSet::with_capacity(columns.len());
            for measurement in &member.measurements {
                let name = measurement.name.to_ascii_lowercase();
                if !names.insert(name.clone()) {
                    return Err(invalid());
                }
                let Some(samples) = columns.get(&name) else {
                    return Err(invalid());
                };
                if observed {
                    let Some(value) = measurement.value else {
                        return Err(invalid());
                    };
                    // A computed value that misses GOAL/TOL or FAILVALUE is
                    // a valid population sample with a failing verdict.
                    let verdict_valid = if measurement.passed {
                        measurement.error.is_none()
                    } else {
                        measurement
                            .error
                            .as_ref()
                            .is_some_and(|error| !error.trim().is_empty())
                    };
                    if !verdict_valid
                        || !value.is_finite()
                        || samples.get(successful).map(|sample| sample.to_bits())
                            != Some(value.to_bits())
                    {
                        return Err(invalid());
                    }
                } else if measurement.value.is_some()
                    || measurement.passed
                    || measurement
                        .error
                        .as_ref()
                        .is_none_or(|error| error.trim().is_empty())
                {
                    return Err(invalid());
                }
            }
            successful += usize::from(observed);
        }
        if successful != completed {
            return Err(invalid());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::FamilyMeasurementEvidence;

    fn population() -> Vec<FamilyMemberMeasurements> {
        [Some(0.9), None, Some(1.1)]
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                FamilyMemberMeasurements::new(
                    FamilyMemberId::MonteCarloSequenceTrial {
                        index,
                        seed: u64::MAX,
                        policy: "parameter-xoroshiro128plus-2018-v1".into(),
                    },
                    vec![FamilyMeasurementEvidence {
                        name: "V(out)".into(),
                        value,
                        passed: value.is_some(),
                        error: value.is_none().then(|| "did not converge".into()),
                    }],
                )
            })
            .collect()
    }
    fn validate(members: &[FamilyMemberMeasurements]) -> Result<(), String> {
        FamilyMemberMeasurements::validate_monte_carlo_sequence(
            members,
            u64::MAX,
            3,
            2,
            1,
            [("V(out)", [0.9, 1.1].as_slice())],
        )
    }
    #[test]
    fn monte_carlo_sequence_validation_rejects_lost_or_reattributed_trials() {
        let original = population();
        validate(&original).unwrap();
        let mut resumed = original.clone();
        for member in &mut resumed {
            if let FamilyMemberId::MonteCarloSequenceTrial { index, .. } = &mut member.member {
                *index += 37;
            }
        }
        validate(&resumed).unwrap();
        resumed.swap(0, 1);
        assert!(validate(&resumed).is_err());
        for mutation in 0..10 {
            let mut members = original.clone();
            match mutation {
                0 => {
                    members.remove(1);
                }
                1 => {
                    members.swap(0, 2);
                }
                2 => {
                    if let FamilyMemberId::MonteCarloSequenceTrial { seed, .. } =
                        &mut members[0].member
                    {
                        *seed = 7;
                    }
                }
                3 => {
                    if let FamilyMemberId::MonteCarloSequenceTrial { policy, .. } =
                        &mut members[0].member
                    {
                        *policy = "spectre-coordinate-splitmix64-v1".into();
                    }
                }
                4 => {
                    members[0].measurements[0].value = Some(1.1);
                }
                5 => {
                    members[1].measurements[0].passed = true;
                }
                6 => {
                    members[1].measurements[0].error = None;
                }
                7 => {
                    members[0].measurements[0].name = "V(other)".into();
                }
                8 => {
                    members[0].member = FamilyMemberId::MonteCarloTrial {
                        index: 0,
                        seed: u64::MAX,
                    };
                }
                _ => {
                    members[1].measurements.clear();
                }
            }
            assert!(validate(&members).is_err(), "mutation {mutation}");
        }
    }
}
