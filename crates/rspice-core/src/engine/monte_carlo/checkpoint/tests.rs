//! Checkpoints must reproduce actual circuit trials, including interrupted gaps.
use super::*;
use crate::abort_signal::{AtomicAbort, ImmediateAbort, NoAbort};
use crate::analysis::monte_carlo::Distribution;
use std::sync::atomic::AtomicUsize;

fn engine(workers: usize) -> Engine {
    let mut config = crate::SimulationConfig::default();
    config.resource_limits.max_parallel_workers = workers;
    Engine::new(config)
}
fn fixture(variation: MonteCarloVariationSource) -> (Netlist, MonteCarloStudyConfig) {
    let r = if variation == MonteCarloVariationSource::DeckStatistics {
        "{aunif(1000,200)}"
    } else {
        "1000"
    };
    let netlist = Netlist::parse(&format!("MC checkpoint\n.param R={r}\nV1 in 0 1\nRS in out {{R}}\nRL out 0 1k\n.options GMIN=0\n.end\n")).unwrap();
    let mut study =
        MonteCarloStudyConfig::new(8, 31, vec!["output".into(), "draw".into(), "zero".into()]);
    study.first_trial = 7;
    study.variation_source = variation;
    study.distribution = Distribution::Gaussian { sigma: 0.2 };
    if variation == MonteCarloVariationSource::ParameterTolerance {
        study.parameter_filter = vec!["R".into()];
    }
    (netlist, study)
}
fn evaluate(
    engine: &Engine,
    trial: &Netlist,
    index: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    if index == 8 {
        return Err(SimulationError::Circuit("intentional trial failure".into()));
    }
    let point = engine.run_dc_op_with_abort(trial, abort)?;
    let output = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    Ok(vec![
        point.node_voltages[output],
        trial.params.get("R").unwrap(),
        -0.0,
    ])
}

#[test]
fn monte_carlo_checkpoint_resumes_and_pools_exact_trial_populations() {
    for variation in [
        MonteCarloVariationSource::ParameterTolerance,
        MonteCarloVariationSource::DeckStatistics,
    ] {
        let (netlist, study) = fixture(variation);
        let serial = engine(1);
        let reference = serial
            .run_monte_carlo_measurements_with_abort(&netlist, &study, &NoAbort, evaluate)
            .unwrap();
        let mut checkpoint = serial
            .new_monte_carlo_checkpoint(&netlist, &study, [1; 32], &NoAbort)
            .unwrap();
        let stop = AtomicAbort::new();
        let result = serial.run_monte_carlo_measurements_checkpointed_with_abort(
            &netlist,
            &study,
            [1; 32],
            &mut checkpoint,
            &stop,
            evaluate,
            |saved| {
                if saved.completed_trials() == 3 {
                    stop.set();
                }
                Ok(())
            },
        );
        assert!(matches!(result, Err(SimulationError::Aborted)));
        assert_eq!(
            checkpoint.completed_indices().collect::<Vec<_>>(),
            [7, 8, 9]
        );
        assert_eq!(checkpoint.trial(8), Some(None));
        assert_eq!(checkpoint.trial(10), None);
        let bytes = checkpoint
            .to_bytes_with_limits(ResourceLimits::default(), &NoAbort)
            .unwrap();
        checkpoint = MonteCarloCheckpoint::from_bytes_with_limits(
            &bytes,
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        assert_eq!(
            checkpoint.trial(7).unwrap().unwrap()[2].to_bits(),
            (-0.0_f64).to_bits()
        );
        let parallel = engine(2);
        let mut tail_study = study.clone();
        tail_study.first_trial = 11;
        tail_study.num_runs = 4;
        let mut tail = parallel
            .new_monte_carlo_checkpoint(&netlist, &tail_study, [1; 32], &NoAbort)
            .unwrap();
        parallel
            .run_monte_carlo_measurements_checkpointed_with_abort(
                &netlist,
                &tail_study,
                [1; 32],
                &mut tail,
                &NoAbort,
                evaluate,
                |_| Ok(()),
            )
            .unwrap();
        checkpoint
            .merge_with_limits(&tail, ResourceLimits::default(), &NoAbort)
            .unwrap();
        checkpoint
            .merge_with_limits(&tail, ResourceLimits::default(), &NoAbort)
            .unwrap();
        assert_eq!(
            checkpoint.completed_trials(),
            7,
            "overlap cannot add weight to the population"
        );
        let calls = AtomicUsize::new(0);
        let result = parallel
            .run_monte_carlo_measurements_checkpointed_with_abort(
                &netlist,
                &study,
                [1; 32],
                &mut checkpoint,
                &NoAbort,
                |engine, trial, index, abort| {
                    assert_eq!(index, 10);
                    calls.fetch_add(1, Ordering::SeqCst);
                    evaluate(engine, trial, index, abort)
                },
                |_| Ok(()),
            )
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            result.successful_trial_indices,
            reference.successful_trial_indices
        );
        assert_eq!(result.num_failures, 1);
        for (name, expected) in &reference.variables {
            let actual = &result.variables[name];
            assert_eq!(
                actual
                    .samples
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                expected
                    .samples
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>()
            );
            assert_eq!(actual.mean.to_bits(), expected.mean.to_bits());
            assert_eq!(actual.std_dev.to_bits(), expected.std_dev.to_bits());
            assert_eq!(actual.histogram, expected.histogram);
        }
        assert_eq!(
            format!("{:?}", result.confidence),
            format!("{:?}", reference.confidence)
        );
        let before = checkpoint.clone();
        for changed in 0..4 {
            let mut request = study.clone();
            let mut circuit = netlist.clone();
            let mut contract = [1; 32];
            match changed {
                0 => request.seed += 1,
                1 => request.measurements.swap(0, 1),
                2 => circuit
                    .ast_overlay
                    .parameters
                    .insert("R".into(), 1700.0)
                    .map(|_| ())
                    .unwrap_or(()),
                _ => contract[0] = 2,
            }
            let result = serial.run_monte_carlo_measurements_checkpointed_with_abort(
                &circuit,
                &request,
                contract,
                &mut checkpoint,
                &NoAbort,
                |_, _, _, _| panic!("mismatched population must not run"),
                |_| Ok(()),
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("population contract")
            );
            assert_eq!(checkpoint, before);
        }
    }
}

#[test]
fn monte_carlo_checkpoint_encoding_and_pooling_reject_corruption_and_limits() {
    let (netlist, study) = fixture(MonteCarloVariationSource::ParameterTolerance);
    let mut checkpoint = engine(1)
        .new_monte_carlo_checkpoint(&netlist, &study, [1; 32], &NoAbort)
        .unwrap();
    checkpoint
        .rows
        .insert(7, Some(vec![-0.0, Value::MIN_POSITIVE, Value::MAX]));
    checkpoint.rows.insert(8, None);
    let limits = ResourceLimits::default();
    let bytes = checkpoint.to_bytes_with_limits(limits, &NoAbort).unwrap();
    let restored = MonteCarloCheckpoint::from_bytes_with_limits(&bytes, limits, &NoAbort).unwrap();
    assert_eq!(
        restored.to_bytes_with_limits(limits, &NoAbort).unwrap(),
        bytes
    );
    let mut corrupt = bytes.clone();
    corrupt[MAGIC.len() + 4] ^= 1;
    assert!(
        MonteCarloCheckpoint::from_bytes_with_limits(&corrupt, limits, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("digest")
    );
    for case in 0..3 {
        let mut body = bytes[..bytes.len() - 32].to_vec();
        match case {
            0 => body.extend_from_slice(&[0]),
            1 => body[MAGIC.len()..MAGIC.len() + 4].copy_from_slice(&2_u32.to_le_bytes()),
            _ => {
                let at = MAGIC.len() + 4 + 32 + 8 + 8 + 8 + 1;
                body[at..at + 8].copy_from_slice(&Value::NAN.to_bits().to_le_bytes());
            }
        }
        let digest = blake3::hash(&body);
        body.extend_from_slice(digest.as_bytes());
        assert!(MonteCarloCheckpoint::from_bytes_with_limits(&body, limits, &NoAbort).is_err());
    }
    let mut tiny = limits;
    tiny.max_external_data_bytes = bytes.len() - 1;
    assert!(checkpoint.to_bytes_with_limits(tiny, &NoAbort).is_err());
    assert!(MonteCarloCheckpoint::from_bytes_with_limits(&bytes, tiny, &NoAbort).is_err());
    let mut conflict = checkpoint.clone();
    conflict.rows.get_mut(&7).unwrap().as_mut().unwrap()[0] = 0.0;
    let before = checkpoint.clone();
    assert!(
        checkpoint
            .merge_with_limits(&conflict, limits, &NoAbort)
            .is_err()
    );
    assert_eq!(
        checkpoint.to_bytes_with_limits(limits, &NoAbort).unwrap(),
        bytes
    );
    conflict = checkpoint.clone();
    conflict.rows.insert(9, Some(vec![0.0; 3]));
    tiny = limits;
    tiny.max_batch_runs = 2;
    assert!(
        checkpoint
            .merge_with_limits(&conflict, tiny, &NoAbort)
            .is_err()
    );
    assert!(
        checkpoint
            .merge_with_limits(&conflict, limits, &ImmediateAbort)
            .is_err()
    );
    assert_eq!(checkpoint, before);
}

#[test]
fn monte_carlo_checkpoint_keeps_committed_rows_when_publication_or_evaluation_stops() {
    let engine = engine(1);
    let (netlist, study) = fixture(MonteCarloVariationSource::ParameterTolerance);
    let mut checkpoint = engine
        .new_monte_carlo_checkpoint(&netlist, &study, [1; 32], &NoAbort)
        .unwrap();
    let error = engine
        .run_monte_carlo_measurements_checkpointed_with_abort(
            &netlist,
            &study,
            [1; 32],
            &mut checkpoint,
            &NoAbort,
            evaluate,
            |_| Err(invalid("disk full")),
        )
        .unwrap_err();
    assert!(error.to_string().contains("disk full"));
    assert_eq!(checkpoint.completed_indices().collect::<Vec<_>>(), [7]);
    struct Deadline(AtomicAbort);
    impl AbortSignal for Deadline {
        fn is_aborted(&self) -> bool {
            self.0.is_aborted()
        }
        fn abort_reason(&self) -> crate::abort_signal::AbortReason {
            crate::abort_signal::AbortReason::TimeLimit
        }
    }
    let deadline = Deadline(AtomicAbort::new());
    let result = engine.run_monte_carlo_measurements_checkpointed_with_abort(
        &netlist,
        &study,
        [1; 32],
        &mut checkpoint,
        &deadline,
        |engine, trial, index, abort| {
            assert_ne!(index, 7);
            if index == 10 {
                deadline.0.set();
                Err(SimulationError::Aborted)
            } else {
                evaluate(engine, trial, index, abort)
            }
        },
        |_| Ok(()),
    );
    assert!(matches!(result, Err(SimulationError::TimeLimitExceeded)));
    assert_eq!(
        checkpoint.completed_indices().collect::<Vec<_>>(),
        [7, 8, 9]
    );
    engine
        .run_monte_carlo_measurements_checkpointed_with_abort(
            &netlist,
            &study,
            [1; 32],
            &mut checkpoint,
            &NoAbort,
            |engine, trial, index, abort| {
                assert!(index >= 10);
                evaluate(engine, trial, index, abort)
            },
            |_| Ok(()),
        )
        .unwrap();
    assert_eq!(checkpoint.completed_trials(), 8);
}
