use super::*;
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};
use rspice_core::engine::MonteCarloStudyConfig;
use rspice_core::{Engine, Netlist};

fn fixture() -> StudyMonteCarloCheckpoint {
    let engine = Engine::default();
    let netlist = Netlist::parse("Journal\n.param r=1k\nV1 in 0 1\nR1 in 0 {r}\n.end\n").unwrap();
    let names = vec!["gain".to_owned(), "bounded".to_owned()];
    let study = MonteCarloStudyConfig::new(3, 37, names.clone());
    let mut numerical = engine
        .new_monte_carlo_checkpoint(&netlist, &study, [7; 32], &NoAbort)
        .unwrap();
    engine
        .run_monte_carlo_measurements_checkpointed_with_abort(
            &netlist,
            &study,
            [7; 32],
            &mut numerical,
            &NoAbort,
            |_, _, index, _| {
                if index == 1 {
                    Err(rspice_core::SimulationError::Circuit(
                        "failed to converge".into(),
                    ))
                } else {
                    Ok(vec![-0.0, f64::MIN_POSITIVE])
                }
            },
            |_| Ok(()),
        )
        .unwrap();
    let observations = numerical
        .completed_indices()
        .map(|index| {
            let row = if let Some(values) = numerical.trial(index).unwrap() {
                names
                    .iter()
                    .enumerate()
                    .map(|(column, name)| FamilyMeasurementEvidence {
                        unit: None,
                        name: name.clone(),
                        value: Some(values[column]),
                        passed: column == 0,
                        error: (column == 1).then(|| "GOAL not met".into()),
                    })
                    .collect()
            } else {
                failed_observations(&names, "failed to converge")
            };
            (index, row)
        })
        .collect();
    StudyMonteCarloCheckpoint::capture(
        &numerical,
        &names,
        &observations,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap()
}

#[test]
fn studio_monte_carlo_checkpoint_envelope_rejects_missing_verdicts_corruption_and_conflicts() {
    let original = fixture();
    let limits = ResourceLimits::default();
    let bytes = original.to_bytes_with_limits(limits, &NoAbort).unwrap();
    let decoded =
        StudyMonteCarloCheckpoint::from_bytes_with_limits(&bytes, limits, &NoAbort).unwrap();
    assert_eq!(decoded, original);
    assert_eq!(
        decoded.observations[&0][0].value.unwrap().to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        decoded.observations[&0][1].value.unwrap().to_bits(),
        f64::MIN_POSITIVE.to_bits()
    );
    assert!(!decoded.observations[&1][0].passed);
    for mutation in 0..4 {
        let mut malformed = bytes.clone();
        match mutation {
            0 => malformed[0] ^= 1,
            1 => {
                malformed[MAGIC.len()] = 9;
                let size = malformed.len() - 32;
                let digest = Sha256::digest(&malformed[..size]);
                malformed[size..].copy_from_slice(&digest);
            }
            2 => {
                malformed.truncate(malformed.len() - 34);
                let digest = Sha256::digest(&malformed);
                malformed.extend_from_slice(&digest);
            }
            _ => {
                malformed.truncate(malformed.len() - 32);
                malformed.push(0);
                let digest = Sha256::digest(&malformed);
                malformed.extend_from_slice(&digest);
            }
        }
        assert!(
            StudyMonteCarloCheckpoint::from_bytes_with_limits(&malformed, limits, &NoAbort)
                .is_err()
        );
    }
    let mut missing = original.clone();
    missing.observations.remove(&0);
    assert!(missing.to_bytes_with_limits(limits, &NoAbort).is_err());
    let mut altered = original.clone();
    altered.observations.get_mut(&0).unwrap()[1].error = Some("different failure".into());
    let mut merged = original.clone();
    assert!(
        merged
            .merge_with_limits(&altered, limits, &NoAbort)
            .is_err()
    );
    assert_eq!(merged, original);
    assert!(
        merged
            .merge_with_limits(&original, limits, &ImmediateAbort)
            .is_err()
    );
    assert_eq!(merged, original);
    let mut tiny = limits;
    tiny.max_external_data_bytes = 16;
    assert!(matches!(
        StudyMonteCarloCheckpoint::from_bytes_with_limits(&bytes, tiny, &NoAbort),
        Err(SimulationError::ResourceLimit { .. })
    ));
    assert!(matches!(
        original.to_bytes_with_limits(tiny, &NoAbort),
        Err(SimulationError::ResourceLimit { .. })
    ));
    let mut tiny = limits;
    tiny.max_result_values = 1;
    assert!(merged.merge_with_limits(&original, tiny, &NoAbort).is_err());
    assert_eq!(merged, original);
}

#[test]
fn measurement_units_survive_checkpoint_resume_and_conflicting_units_refuse_merge() {
    use rspice_core::analysis::MeasurementUnit;
    let limits = ResourceLimits::default();
    let mut original = fixture();
    let old_bytes = original.to_bytes_with_limits(limits, &NoAbort).unwrap();
    assert_eq!(old_bytes[MAGIC.len()], 1);
    assert_eq!(
        StudyMonteCarloCheckpoint::from_bytes_with_limits(&old_bytes, limits, &NoAbort).unwrap(),
        original
    );
    original.observations.get_mut(&0).unwrap()[0].unit = Some(MeasurementUnit::Known("V".into()));
    original.observations.get_mut(&0).unwrap()[1].unit = Some(MeasurementUnit::Unknown);
    let bytes = original.to_bytes_with_limits(limits, &NoAbort).unwrap();
    assert_eq!(bytes[MAGIC.len()], 2);
    let decoded =
        StudyMonteCarloCheckpoint::from_bytes_with_limits(&bytes, limits, &NoAbort).unwrap();
    assert_eq!(decoded, original);
    let mut conflicting = decoded.clone();
    conflicting.observations.get_mut(&0).unwrap()[0].unit =
        Some(MeasurementUnit::Known("A".into()));
    assert!(
        original
            .merge_with_limits(&conflicting, limits, &NoAbort)
            .is_err()
    );
}
