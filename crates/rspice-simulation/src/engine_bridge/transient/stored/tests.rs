use rspice_core::abort_signal::NoAbort;
use rspice_simulation_contract::config::{AnalysisConfig, TransientAnalysisConfig};

use super::super::super::SimulationInput;
use super::super::{convert_transient_result, netlist_for_transient_config};
use super::*;
use crate::results::SimulationResult;
use crate::results_store::tests::{FREE_SPACE, quota};

/// A pulse into an RC low-pass, a branch current, a measurement over the output window, and an
/// output start that falls between two accepted steps.
const DECK: &str = "stored transient
V1 in 0 PULSE(0 1 0 1u 1u 50u 100u)
R1 in out 1k
C1 out 0 10n
.tran 0.1u 300u 33.3u
.meas tran vmax MAX v(out)
.end
";

fn config(stop_time: f64, start_time: f64, max_timestep: Option<f64>) -> TransientAnalysisConfig {
    TransientAnalysisConfig {
        stop_time,
        step_time: 1e-7,
        start_time,
        max_timestep,
        uic: false,
    }
}

/// The deck run in memory and converted as the in-memory route converts it, with the
/// convergence evidence that route captures over the whole accepted trajectory.
fn in_memory(
    bridge: &EngineBridge,
    deck: &str,
    config: &TransientAnalysisConfig,
) -> (
    SimulationResult,
    rspice_results::convergence_quality::TransientConvergenceEvidence,
    Vec<f64>,
) {
    let analysis = AnalysisConfig::Transient(config.clone());
    let input = SimulationInput {
        config: &analysis,
        netlist_str: deck,
        source_path: None,
        supply_corner: None,
        environment: None,
    };
    let materialized = bridge.materialize_input(&input, &NoAbort).unwrap();
    let netlist = netlist_for_transient_config(&materialized, config, &NoAbort).unwrap();
    let max_step = config
        .resolved_maximum_step(netlist.parse_dialect())
        .unwrap();
    let engine = bridge.engine_for_netlist(&netlist);
    let run = engine
        .run_tran_with_abort(&netlist, config.stop_time, max_step, &NoAbort)
        .unwrap();
    let full_time = run.time.clone();
    let convergence = rspice_results::convergence_quality::TransientConvergenceEvidence::capture(
        engine.convergence_quality(),
        &run.time,
        &NoAbort,
    )
    .unwrap();
    let converted = convert_transient_result(&netlist, run, config.start_time, &NoAbort).unwrap();
    (converted, convergence, full_time)
}

fn bits(values: &[f64]) -> Vec<u64> {
    values.iter().map(|value| value.to_bits()).collect()
}

#[test]
fn a_stored_transient_holds_the_samples_the_in_memory_route_returns_without_reading_them() {
    let bridge = EngineBridge::new();
    let config = config(300e-6, 33.3e-6, None);
    let (expected, expected_convergence, _) = in_memory(&bridge, DECK, &config);
    let actual = bridge
        .run(&AnalysisConfig::Transient(config.clone()), DECK)
        .unwrap();
    let (
        SimulationResult::Transient {
            time,
            waveforms,
            measurements,
            convergence,
            events,
            store,
            ..
        },
        SimulationResult::Transient {
            time: expected_time,
            waveforms: expected_waveforms,
            measurements: expected_measurements,
            events: expected_events,
            ..
        },
    ) = (actual, expected)
    else {
        panic!("both routes return a transient");
    };
    let receipt = store.expect("the desktop route keeps its rows in a store");
    assert!(receipt.path.exists());
    assert!(receipt.stopped.is_none());
    assert!(time.is_stored());
    // The authored start falls between two steps: the axis starts there.
    assert_eq!(time.len(), expected_time.len());
    assert_eq!(time.first().copied(), Some(33.3e-6));
    assert_eq!(bits(&time), bits(&expected_time));
    let mut names = waveforms.keys().collect::<Vec<_>>();
    names.sort();
    let mut expected_names = expected_waveforms.keys().collect::<Vec<_>>();
    expected_names.sort();
    assert_eq!(names, expected_names);
    assert!(names.iter().any(|name| name.starts_with("I(")));
    for (name, waveform) in &waveforms {
        let expected = &expected_waveforms[name];
        assert!(
            waveform.y_values.is_stored(),
            "{name} is a window of the store"
        );
        assert!(
            WaveformValues::ptr_eq(&waveform.x_values, &time),
            "{name} shares the run's one axis"
        );
        assert_eq!(waveform.y_unit, expected.y_unit, "{name}");
        assert_eq!(bits(&waveform.y_values), bits(&expected.y_values), "{name}");
    }
    assert_eq!(
        format!("{measurements:?}"),
        format!("{expected_measurements:?}")
    );
    assert_eq!(measurements.len(), 1);
    assert_eq!(format!("{events:?}"), format!("{expected_events:?}"));
    assert_eq!(
        format!("{:?}", convergence.as_deref()),
        format!("{:?}", Some(&expected_convergence))
    );
    let path = receipt.path.clone();
    drop((time, waveforms));
    assert!(!path.exists(), "the last waveform took its segment with it");
}

/// A comparator's digital output on a pulse train: its history goes to the store.
const MIXED: &str = "stored events
V1 in 0 PULSE(0 1 0 1u 1u 5u 10u)
R1 in 0 1k
a_adc [in] [dig] adc
.model adc adc_bridge(in_low=0.4 in_high=0.6)
.tran 0.1u 100u
.end
";

#[test]
fn a_stored_transient_keeps_its_event_histories_in_its_store_and_reads_them_as_returned_in_memory()
{
    let bridge = EngineBridge::new();
    let same = |a: &crate::results::DigitalEventPoint, b: &crate::results::DigitalEventPoint| {
        a.value_code == b.value_code
    };
    for start in [0.0, 33.3e-6] {
        let config = config(100e-6, start, None);
        let (expected, _, _) = in_memory(&bridge, MIXED, &config);
        let actual = bridge
            .run(&AnalysisConfig::Transient(config.clone()), MIXED)
            .unwrap();
        let (
            SimulationResult::Transient { events, store, .. },
            SimulationResult::Transient {
                events: expected, ..
            },
        ) = (actual, expected)
        else {
            panic!("both routes return a transient");
        };
        assert!(store.is_some());
        assert_eq!(events.digital.len(), expected.digital.len());
        assert!(!events.digital.is_empty());
        for (trace, whole) in events.digital.iter().zip(&expected.digital) {
            assert_eq!(trace.node_name, whole.node_name);
            assert!(trace.points.is_stored() && !whole.points.is_stored());
            assert_eq!(trace.points.len(), whole.points.len(), "start {start}");
            assert!(whole.points.len() > 10);
            for (t0, t1) in [(start, 100e-6), (40e-6, 45e-6), (52e-6, 52e-6), (0.0, 1e-6)] {
                assert_eq!(
                    trace.points.around(t0, t1, same).unwrap(),
                    whole.points.around(t0, t1, same).unwrap(),
                    "start {start}, window {t0} ..= {t1}"
                );
            }
            assert_eq!(*trace.points, *whole.points);
        }
    }
}

#[test]
fn a_run_whose_rows_cannot_fit_is_refused_before_it_solves_and_leaves_no_segment() {
    FREE_SPACE.with(|free| *free.borrow_mut() = Some(quota(4096)));
    let error = EngineBridge::new()
        .run(
            &AnalysisConfig::Transient(config(300e-6, 0.0, Some(1e-8))),
            DECK,
        )
        .unwrap_err();
    FREE_SPACE.with(|free| *free.borrow_mut() = None);
    let SimulationError::ResultStorage(ResultStorageFailure::Insufficient {
        rows,
        needed,
        available,
        folder,
    }) = &error
    else {
        panic!("a typed storage refusal, got {error}");
    };
    // 300 us at a 10 ns ceiling, and six values a row.
    assert_eq!(*rows, 30_000, "{error}");
    assert!(*needed > 4096, "{error}");
    assert_eq!(*available, 4096, "{error}");
    assert!(folder.contains("rspice-results"), "{error}");
    let message = error.to_string();
    assert!(message.starts_with("Results storage full: "), "{message}");
    assert!(message.contains("refused before solving"), "{message}");
    assert!(message.contains("has 4.00 KiB free"), "{message}");
}

#[test]
fn a_volume_that_fills_stops_the_run_keeping_the_rows_it_wrote_and_names_the_stop() {
    // Room for the preflight's 30,000 rows of six values (1,738,800 bytes), and for less than
    // the run writes once each step is at most 10 ns: the watch keeps a closing reserve free.
    let config = config(300e-6, 0.0, Some(1e-8));
    let (_, _, full_time) = in_memory(&EngineBridge::new(), DECK, &config);
    FREE_SPACE.with(|free| {
        *free.borrow_mut() = Some(quota(1_760_000));
    });
    let result = EngineBridge::new().run(&AnalysisConfig::Transient(config), DECK);
    FREE_SPACE.with(|free| *free.borrow_mut() = None);
    let SimulationResult::Transient {
        time,
        waveforms,
        store,
        ..
    } = result.unwrap()
    else {
        panic!("a transient");
    };
    let receipt = store.unwrap();
    let stop = receipt.stopped.as_ref().expect("the volume filled");
    let kept = usize::try_from(receipt.rows).unwrap();
    assert!(
        kept > 0 && kept < full_time.len(),
        "{kept} of {}",
        full_time.len()
    );
    assert_eq!(time.len(), kept);
    assert_eq!(stop.at, Some(full_time[kept - 1]));
    assert_eq!(bits(&time), bits(&full_time[..kept]));
    assert!(
        waveforms
            .values()
            .all(|waveform| waveform.y_values.len() == kept)
    );
}
