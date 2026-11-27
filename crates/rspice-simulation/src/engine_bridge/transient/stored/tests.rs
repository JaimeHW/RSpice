use rspice_core::abort_signal::NoAbort;
use rspice_simulation_contract::config::{AnalysisConfig, TransientAnalysisConfig};

use super::super::super::SimulationInput;
use super::super::{convert_transient_result, netlist_for_transient_config};
use super::*;
use crate::results::SimulationResult;
use crate::results_store::tests::{FREE_SPACE, quota};

/// A pulse into an RC low-pass, a branch current, measurements over the output window (an
/// extreme, an average, an integral, a rise time and a value found at a crossing, the last two
/// read in more than one pass), and an output start that falls between two accepted steps.
const DECK: &str = "stored transient
V1 in 0 PULSE(0 1 0 1u 1u 50u 100u)
R1 in out 1k
C1 out 0 10n
.tran 0.1u 300u 33.3u
.meas tran vmax MAX v(out)
.meas tran vavg AVG v(out) FROM=50u TO=250u
.meas tran charge INTEG i(V1)
.meas tran trise TRIG v(out) VAL=0.1 RISE=2 TARG v(out) VAL=0.5 RISE=2
.meas tran vfound FIND v(out) WHEN v(in)=0.5 FALL=2
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

/// What runs as a stored run's solve returns, if anything does.
type AfterSolve = Option<Box<dyn Fn(&std::path::Path)>>;

thread_local! {
    /// How many times a stored run on this thread was read back whole.
    pub(super) static READ_BACKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many chunks of the store a read-back on this thread has read.
    pub(super) static CHUNKS_READ_BACK: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
    /// What runs as a stored run's solve returns, before anything reads the run back.
    pub(super) static AFTER_SOLVE: std::cell::RefCell<AfterSolve> =
        const { std::cell::RefCell::new(None) };
}

/// The deck's `.ASSERT` reports over its whole accepted trajectory, as the in-memory route
/// checks them.
fn in_memory_assertions(
    bridge: &EngineBridge,
    deck: &str,
    config: &TransientAnalysisConfig,
) -> Vec<rspice_core::analysis::AssertionReport> {
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
    let run = bridge
        .engine_for_netlist(&netlist)
        .run_tran_with_abort(&netlist, config.stop_time, max_step, &NoAbort)
        .unwrap();
    rspice_core::analysis::evaluate_tran_assertions(&netlist, &run, &NoAbort).unwrap()
}

#[test]
fn a_stored_transient_is_checked_and_measured_from_its_store_as_the_in_memory_route_checks_it() {
    let bridge = EngineBridge::new();
    let config = config(300e-6, 33.3e-6, None);
    // Checks over the whole run, before the output start included: a band, one inside a window,
    // one that must last, a condition on two nodes and one on a branch current.
    let asserted = DECK.replace(
        ".end\n",
        ".ASSERT high EXPR='v(out)' MAX=0.4\n\
         .ASSERT early EXPR='v(out)' MIN=0.05 FROM=20u TO=120u\n\
         .ASSERT held EXPR='v(out)' MAX=0.3 DURATION=20u\n\
         .ASSERT apart COND='v(in)-v(out) < 0.5 || v(out) > 0.2'\n\
         .ASSERT drawn EXPR='i(V1)' MIN=-0.0006\n\
         .end\n",
    );
    let (expected, _, _) = in_memory(&bridge, &asserted, &config);
    let expected_assertions = in_memory_assertions(&bridge, &asserted, &config);
    assert!(
        expected_assertions
            .iter()
            .any(|report| !report.violations.is_empty()),
        "the cards find violations to compare"
    );
    READ_BACKS.with(|count| count.set(0));
    let actual = bridge
        .run(&AnalysisConfig::Transient(config.clone()), &asserted)
        .unwrap();
    assert_eq!(
        READ_BACKS.with(std::cell::Cell::get),
        0,
        "neither the checks nor the measurements read the run back whole"
    );
    let (
        SimulationResult::Transient {
            measurements,
            checks,
            store,
            ..
        },
        SimulationResult::Transient {
            measurements: expected_measurements,
            ..
        },
    ) = (actual, expected)
    else {
        panic!("both routes return a transient");
    };
    assert!(store.is_some());
    assert_eq!(
        format!("{:?}", checks.assertions),
        format!("{expected_assertions:?}")
    );
    assert_eq!(
        format!("{measurements:?}"),
        format!("{expected_measurements:?}")
    );
    // A card the rows cannot answer reads the run back, and words its report as in memory.
    let unanswered = DECK.replace(".end\n", ".ASSERT gone EXPR='v(nowhere)' MAX=1\n.end\n");
    let expected_assertions = in_memory_assertions(&bridge, &unanswered, &config);
    READ_BACKS.with(|count| count.set(0));
    let SimulationResult::Transient { checks, .. } = bridge
        .run(&AnalysisConfig::Transient(config.clone()), &unanswered)
        .unwrap()
    else {
        panic!("a transient");
    };
    assert_eq!(READ_BACKS.with(std::cell::Cell::get), 1);
    assert!(checks.assertions[0].error.is_some());
    assert_eq!(
        format!("{:?}", checks.assertions),
        format!("{expected_assertions:?}")
    );
}

/// Stop pressed once this thread's read-back has read `chunks` chunks of the store.
struct StopAfterChunks(usize);

impl rspice_core::abort_signal::AbortSignal for StopAfterChunks {
    fn is_aborted(&self) -> bool {
        CHUNKS_READ_BACK.with(std::cell::Cell::get) >= self.0
    }
}

/// A measurement that does not stream (a continuous equation) reads the run back a chunk of the
/// store at a time, and measures every statement as the in-memory route does; Stop pressed while
/// it reads ends the read where it is.
#[test]
fn a_measurement_that_does_not_stream_reads_the_run_back_a_chunk_at_a_time() {
    let bridge = EngineBridge::new();
    // A step small enough for a run of several chunks of rows.
    let config = config(300e-6, 33.3e-6, Some(0.02e-6));
    let deck = DECK.replace(".end\n", ".meas tran doubled EQN {2*v(out)}\n.end\n");
    let (expected, _, full_time) = in_memory(&bridge, &deck, &config);
    READ_BACKS.with(|count| count.set(0));
    CHUNKS_READ_BACK.with(|count| count.set(0));
    let actual = bridge
        .run(&AnalysisConfig::Transient(config.clone()), &deck)
        .unwrap();
    assert_eq!(READ_BACKS.with(std::cell::Cell::get), 1);
    let chunks = CHUNKS_READ_BACK.with(std::cell::Cell::get);
    assert_eq!(
        chunks,
        full_time.len().div_ceil(4096),
        "{} rows read a chunk at a time",
        full_time.len()
    );
    assert!(chunks > 2, "{chunks} chunks");
    let (
        SimulationResult::Transient {
            measurements,
            store,
            ..
        },
        SimulationResult::Transient {
            measurements: expected_measurements,
            ..
        },
    ) = (actual, expected)
    else {
        panic!("both routes return a transient");
    };
    assert!(store.is_some());
    assert!(
        measurements
            .iter()
            .any(|measurement| measurement.name.eq_ignore_ascii_case("doubled")),
        "{measurements:?}"
    );
    assert_eq!(
        format!("{measurements:?}"),
        format!("{expected_measurements:?}")
    );

    CHUNKS_READ_BACK.with(|count| count.set(0));
    let stopped = bridge.run_with_abort(
        &AnalysisConfig::Transient(config.clone()),
        &deck,
        &StopAfterChunks(2),
    );
    assert!(
        matches!(stopped, Err(SimulationError::Aborted)),
        "{:?}",
        stopped.map(|_| "a result")
    );
    assert_eq!(CHUNKS_READ_BACK.with(std::cell::Cell::get), 2);
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
    assert_eq!(time.read_whole().unwrap().first().copied(), Some(33.3e-6));
    assert_eq!(
        bits(&time.read_whole().unwrap()),
        bits(&expected_time.read_whole().unwrap())
    );
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
        assert_eq!(
            bits(&waveform.y_values.read_whole().unwrap()),
            bits(&expected.y_values.read_whole().unwrap()),
            "{name}"
        );
    }
    assert_eq!(
        format!("{measurements:?}"),
        format!("{expected_measurements:?}")
    );
    assert_eq!(measurements.len(), 5);
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
    assert_eq!(bits(&time.read_whole().unwrap()), bits(&full_time[..kept]));
    assert!(
        waveforms
            .values()
            .all(|waveform| waveform.y_values.len() == kept)
    );
}

/// Stop pressed as the solve ends answers there: the run reports that it stopped, reads nothing
/// back and leaves no segment, where it used to read every row and device of the run back first.
#[test]
fn stop_pressed_as_the_solve_ends_stops_the_run_before_it_is_read_back() {
    let stop = std::sync::Arc::new(rspice_core::abort_signal::AtomicAbort::new());
    let pressed = std::sync::Arc::clone(&stop);
    let segment = std::sync::Arc::new(std::sync::Mutex::new(None));
    let written = std::sync::Arc::clone(&segment);
    AFTER_SOLVE.with(|hook| {
        *hook.borrow_mut() = Some(Box::new(move |path: &std::path::Path| {
            *written.lock().unwrap() = Some(path.to_path_buf());
            pressed.set();
        }));
    });
    READ_BACKS.with(|count| count.set(0));
    let result = EngineBridge::new().run_with_abort(
        &AnalysisConfig::Transient(config(300e-6, 0.0, None)),
        DECK,
        &*stop,
    );
    AFTER_SOLVE.with(|hook| *hook.borrow_mut() = None);
    assert!(
        matches!(result, Err(SimulationError::Aborted)),
        "a run stopped as its solve ended is a stopped run, got {:?}",
        result.map(|_| "a result")
    );
    assert_eq!(READ_BACKS.with(std::cell::Cell::get), 0);
    let segment = segment
        .lock()
        .unwrap()
        .clone()
        .expect("the run solved into a segment");
    assert!(!segment.exists(), "{} is left behind", segment.display());
}

/// A sink that keeps how each run it was given ended, and whether its close was told the run
/// had been stopped.
#[derive(Default)]
struct Endings(Vec<(rspice_core::result_sink::RunStatus, bool)>);

impl ClosesUnlessStopped for Endings {
    fn end_unless(
        &mut self,
        status: rspice_core::result_sink::RunStatus,
        stopped: &dyn Fn() -> bool,
    ) -> Result<rspice_core::result_sink::SinkReceipt, SinkStop> {
        self.0.push((status, stopped()));
        Ok(rspice_core::result_sink::SinkReceipt::default())
    }
}

impl rspice_core::result_sink::ResultSink for Endings {
    fn begin(
        &mut self,
        _schema: &rspice_core::result_sink::ResultSchema,
        _meta: &rspice_core::result_sink::RunMeta,
    ) -> Result<(), SinkStop> {
        Ok(())
    }

    fn row(&mut self, _x: f64, _values: &[f64]) -> Result<(), SinkStop> {
        Ok(())
    }

    fn amend_open_row(&mut self, _row: u64, _column: u32, _value: f64) -> Result<(), SinkStop> {
        Ok(())
    }

    fn events(
        &mut self,
        _changes: &[rspice_core::result_sink::EventChange],
    ) -> Result<(), SinkStop> {
        Ok(())
    }

    fn end(
        &mut self,
        status: rspice_core::result_sink::RunStatus,
    ) -> Result<rspice_core::result_sink::SinkReceipt, SinkStop> {
        self.end_unless(status, &|| false)
    }
}

#[test]
fn a_stopped_run_is_neither_closed_nor_waited_for_and_every_other_ending_closes() {
    use rspice_core::abort_signal::ImmediateAbort;
    use rspice_core::result_sink::{ResultSink, RunStatus as Ending, StopReason};
    let cancelled = Ending::Stopped(StopReason::Cancelled);
    let mut endings = Endings::default();
    let refused = UnclosedOnStop {
        sink: &mut endings,
        abort: &ImmediateAbort,
    }
    .end(cancelled.clone());
    assert!(
        refused.is_err(),
        "a stopped run's segment is deleted, not closed"
    );
    // A run that completed as Stop was pressed starts its close, told the run was stopped so
    // nothing waits for it; a run cancelled with no Stop pending (its engine's own stop)
    // closes as before.
    UnclosedOnStop {
        sink: &mut endings,
        abort: &ImmediateAbort,
    }
    .end(Ending::Completed)
    .expect("a completed run closes");
    UnclosedOnStop {
        sink: &mut endings,
        abort: &NoAbort,
    }
    .end(cancelled.clone())
    .expect("a run nobody stopped closes");
    // The close of the run that completed is told Stop was pressed, so it is not waited for.
    assert_eq!(
        endings.0,
        vec![(Ending::Completed, true), (cancelled, false)]
    );
}
