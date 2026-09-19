//! Statistical population validation for both directions of worker transport.

use super::*;

pub(super) fn validate(result: &WorkerSimulationResult) -> Result<(), String> {
    if let WorkerSimulationResult::MonteCarlo {
        seed,
        runs_requested,
        member_measurements,
        variables,
        runs_completed,
        num_failures,
        ..
    } = result
    {
        let evidence_values = member_measurements.iter().fold(0usize, |count, member| {
            count
                .saturating_add(member.measurements.len())
                .saturating_add(2)
        });
        let values = variables.iter().fold(evidence_values, |count, variable| {
            count.saturating_add(
                variable.estimated_numeric_payload_bytes() / std::mem::size_of::<f64>(),
            )
        });
        if values > MAX_WORKER_F64_VALUES {
            return Err("Monte Carlo population exceeds the worker numeric payload limit".into());
        }
        crate::state::FamilyMemberMeasurements::validate_monte_carlo_sequence(
            member_measurements,
            *seed,
            *runs_requested,
            *runs_completed,
            *num_failures,
            variables
                .iter()
                .map(|variable| (variable.name.as_str(), variable.samples.as_slice())),
        )?;
        for variable in variables {
            if let Some(confidence) = variable.mean_confidence {
                if variable.samples.len() != *runs_completed {
                    return Err(
                        "Monte Carlo confidence sample count disagrees with successful runs".into(),
                    );
                }
                confidence.validate(variable.samples.len(), *num_failures)?;
            }
        }
    }
    Ok(())
}
