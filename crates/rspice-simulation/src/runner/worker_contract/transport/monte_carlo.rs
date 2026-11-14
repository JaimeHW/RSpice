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
        rspice_results::family_measurements::FamilyMemberMeasurements::validate_monte_carlo_sequence(
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
