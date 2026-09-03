//! Iteration bounds for cooperative cancellation.
//!
//! `advanced_abort.rs` proves each analysis refuses to start once an abort is
//! already set. That is the entry-point contract and it says nothing about the
//! only property a user can feel: how much work happens between asking a run
//! to stop and it stopping.
//!
//! These tests measure that directly. Each one first counts how many times a
//! completed run polls its abort source, which is a lower bound on how finely
//! that analysis is instrumented, then cancels at a poll in the *interior* of
//! that count and asserts the run stopped at exactly that poll and polled no
//! further. An analysis that polls only at its entry point fails the first
//! assertion; one that keeps working after being told to stop fails the last.
//!
//! The numbers are deterministic because the fixtures are: fixed decks, fixed
//! sweeps, fixed trial counts, and — through [`serialized`] — a compute pool
//! pinned to one thread, so a poll sequence is not a function of core count.

use rspice_core::abort_signal::CountingAbort;
use rspice_core::analysis::fourier::{FourierAnalysis, FourierConfig, FourierError};
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::stb::StbConfig;
use rspice_core::analysis::{Distribution, PssConfig};
use rspice_core::engine::{
    CompressionConfig, Engine, SimulationConfig, SimulationError, TransientCheckpointEncoding,
};
use rspice_core::execution::{DeckPlan, DeckPlanError, MaterializedRunError};
use rspice_core::netlist::Netlist;
use rspice_core::{ResourceLimits, Value};

/// A small RC network driven by a megahertz sine: linear, convergent, and
/// cheap enough that every analysis below finishes in milliseconds.
fn fixture() -> (Engine, Netlist) {
    let netlist = Netlist::parse(
        "iteration bound fixture\n\
         .param rval=1k\n\
         v1 in 0 sin(0 1 1meg) ac 1\n\
         r1 in out {rval}\n\
         c1 out 0 1n\n\
         .end\n",
    )
    .expect("fixture parses");
    (Engine::new(SimulationConfig::default()), netlist)
}

/// Run `body` with the compute pool pinned to a single thread.
///
/// Several sweeps parallelize their points. Under a work-stealing pool the
/// poll sequence depends on how many workers there are: once one of them
/// observes the cancellation the rest each poll once more before returning,
/// which is correct behavior and impossible to assert an exact number on. One
/// thread makes the count the analysis's own.
fn serialized<T: Send>(body: impl FnOnce() -> T + Send) -> T {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .expect("single-threaded compute pool")
        .install(body)
}

/// How many times one *completed* run polls its abort source.
///
/// The baseline has to succeed, or the measured count describes how far a
/// rejected request got rather than how finely the analysis is instrumented,
/// and every bound derived from it would be measuring nothing.
#[track_caller]
fn poll_count<T: Send, E: std::fmt::Debug + Send>(
    what: &str,
    run: impl FnOnce(&CountingAbort) -> Result<T, E> + Send,
) -> usize {
    let counter = CountingAbort::new(usize::MAX);
    let outcome = serialized(|| run(&counter));
    assert!(
        outcome.is_ok(),
        "{what} did not complete without cancellation, so its poll count measures \
         nothing: {:?}",
        outcome.err()
    );
    counter.count()
}

/// The interior poll to cancel at, given a measured total.
///
/// Halfway is past every entry-point check and short of the tail, so an abort
/// observed here can only have come from the analysis's own loop.
fn interior(total: usize) -> usize {
    total / 2
}

/// Assert `run` stops at exactly the poll that first reports cancellation.
#[track_caller]
fn assert_stops_exactly_at(
    what: &str,
    threshold: usize,
    run: impl FnOnce(&CountingAbort) -> bool + Send,
) {
    let abort = CountingAbort::new(threshold);
    let cancelled = serialized(|| run(&abort));
    assert!(
        cancelled,
        "{what} did not report cancellation when its abort fired at poll {}",
        threshold + 1
    );
    assert_eq!(
        abort.observed_at(),
        Some(threshold + 1),
        "{what} never reached the poll that cancels it ({} polls total)",
        abort.count()
    );
    assert_eq!(
        abort.polls_after_abort(),
        0,
        "{what} kept working after observing cancellation: {} polls, expected {}",
        abort.count(),
        threshold + 1
    );
}

/// Assert an analysis polls often enough for an interior cancellation to
/// exist at all, which is what distinguishes a bounded loop from one that is
/// only checked on the way in.
#[track_caller]
fn assert_polls_repeatedly(what: &str, total: usize) {
    assert!(
        total >= 4,
        "{what} polled its abort source {total} time(s); a loop checked only at \
         its entry point leaves cancellation latency unbounded"
    );
}

fn aborted<T>(result: Result<T, SimulationError>) -> bool {
    matches!(result, Err(SimulationError::Aborted))
}

//=============================================================================
// Time domain
//=============================================================================

#[test]
fn transient_stops_inside_its_step_and_newton_loops() {
    let (engine, netlist) = fixture();
    let total = poll_count("transient", |abort| {
        engine.run_tran_with_abort(&netlist, 2.0e-5, 2.0e-8, abort)
    });
    assert_polls_repeatedly("transient", total);
    assert_stops_exactly_at("transient", interior(total), |abort| {
        aborted(engine.run_tran_with_abort(&netlist, 2.0e-5, 2.0e-8, abort))
    });
}

#[test]
fn dc_sweep_stops_inside_its_point_loop() {
    let (engine, netlist) = fixture();
    let total = poll_count("DC sweep", |abort| {
        engine.run_dc_sweep_with_abort(&netlist, "v1", 0.0, 5.0, 0.05, abort)
    });
    assert_polls_repeatedly("DC sweep", total);
    assert_stops_exactly_at("DC sweep", interior(total), |abort| {
        aborted(engine.run_dc_sweep_with_abort(&netlist, "v1", 0.0, 5.0, 0.05, abort))
    });
}

//=============================================================================
// Frequency domain
//=============================================================================

fn decade_sweep(points: usize) -> Vec<Value> {
    (0..points)
        .map(|index| 10.0_f64.powf(1.0 + 6.0 * index as Value / points as Value))
        .collect()
}

#[test]
fn ac_stops_inside_its_frequency_loop() {
    let (engine, netlist) = fixture();
    let frequencies = decade_sweep(200);
    let total = poll_count("AC", |abort| {
        engine.run_ac_with_abort(&netlist, &frequencies, abort)
    });
    assert_polls_repeatedly("AC", total);
    assert_stops_exactly_at("AC", interior(total), |abort| {
        aborted(engine.run_ac_with_abort(&netlist, &frequencies, abort))
    });
}

#[test]
fn noise_stops_inside_its_frequency_loop() {
    let (engine, netlist) = fixture();
    let frequencies = decade_sweep(120);
    let total = poll_count("noise", |abort| {
        engine.run_noise_with_abort(&netlist, 2, &frequencies, 300.15, abort)
    });
    assert_polls_repeatedly("noise", total);
    assert_stops_exactly_at("noise", interior(total), |abort| {
        aborted(engine.run_noise_with_abort(&netlist, 2, &frequencies, 300.15, abort))
    });
}

#[test]
fn distortion_stops_inside_its_frequency_loop() {
    // `.disto` measures the response to an explicit distortion excitation, so
    // the source carries `distof1`; without it the request is refused before
    // any sweep starts and there is no loop to bound.
    let netlist = Netlist::parse(
        "distortion iteration bound\n\
         v1 in 0 sin(0 1 1meg) ac 1 distof1 1\n\
         r1 in out 1k\n\
         c1 out 0 1n\n\
         .end\n",
    )
    .expect("distortion fixture parses");
    let engine = Engine::new(SimulationConfig::default());
    let frequencies = decade_sweep(60);
    let total = poll_count("distortion", |abort| {
        engine.run_distortion_with_abort(&netlist, &frequencies, None, abort)
    });
    assert_polls_repeatedly("distortion", total);
    assert_stops_exactly_at("distortion", interior(total), |abort| {
        aborted(engine.run_distortion_with_abort(&netlist, &frequencies, None, abort))
    });
}

#[test]
fn stability_stops_inside_its_sweep() {
    // STB needs a loop to break, so this is a closed feedback amplifier with
    // an explicit zero-volt probe source rather than the open RC above.
    let netlist = Netlist::parse(
        "stability iteration bound\n\
         EAMP out 0 in 0 10\n\
         VPROBE out fb 0\n\
         RF fb in 10k\n\
         RIN in 0 1k\n\
         .end\n",
    )
    .expect("STB fixture parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = || {
        StbConfig::new()
            .with_sweep(10.0, 1.0e6, 60)
            .with_probe("VPROBE")
            .with_nyquist(true)
    };

    let total = poll_count("STB", |abort| {
        engine.run_stb_with_abort(&netlist, config(), abort)
    });
    assert_polls_repeatedly("STB", total);
    assert_stops_exactly_at("STB", interior(total), |abort| {
        aborted(engine.run_stb_with_abort(&netlist, config(), abort))
    });
}

/// S-parameter extraction owns two loops of its own — one per driven port,
/// one per frequency of the projection that reads the port voltages back — on
/// top of the caller's AC solves.
#[test]
fn s_parameter_extraction_stops_inside_its_port_and_projection_loops() {
    let netlist = Netlist::parse(
        "s-parameter iteration bound\n\
         P1 p1 0 PORT=1 Z0=50 AC 1\n\
         R1 p1 p2 50\n\
         P2 p2 0 PORT=2 Z0=50\n\
         .ac lin 1 1 1\n\
         .end\n",
    )
    .expect("S-parameter fixture parses");
    let ports = rspice_core::analysis::s_param::collect_ports(&netlist).expect("ports collect");
    let engine = Engine::new(SimulationConfig::default());
    let frequencies = decade_sweep(80);
    let extract = |abort: &CountingAbort| {
        rspice_core::analysis::s_param::extract_s_matrix_with_abort(
            &netlist,
            &ports,
            &frequencies,
            |driven| {
                engine
                    .run_ac_with_abort(driven, &frequencies, abort)
                    .map_err(|error| error.to_string())
            },
            abort,
        )
    };

    let total = poll_count("S-parameter extraction", extract);
    assert_polls_repeatedly("S-parameter extraction", total);

    let threshold = interior(total);
    let abort = CountingAbort::new(threshold);
    let outcome = serialized(|| extract(&abort));
    // The abort can be seen either by the extraction's own loops or by the AC
    // solve it drives; both are cancellations, and neither may be reported as
    // a bad measurement.
    assert!(
        matches!(
            outcome,
            Err(rspice_core::analysis::s_param::ExtractError::Aborted)
                | Err(rspice_core::analysis::s_param::ExtractError::AcSolve(_))
        ),
        "S-parameter extraction did not stop when cancelled: {outcome:?}"
    );
    assert_eq!(abort.observed_at(), Some(threshold + 1));
    assert_eq!(
        abort.polls_after_abort(),
        0,
        "S-parameter extraction kept working after observing cancellation"
    );
}

#[test]
fn pole_zero_stops_inside_its_root_search() {
    let (engine, netlist) = fixture();
    let total = poll_count("pole-zero", |abort| {
        engine.run_pz_ports_with_abort(&netlist, 1, None, 2, None, false, true, true, abort)
    });
    assert_polls_repeatedly("pole-zero", total);
    assert_stops_exactly_at("pole-zero", interior(total), |abort| {
        aborted(
            engine.run_pz_ports_with_abort(&netlist, 1, None, 2, None, false, true, true, abort),
        )
    });
}

#[test]
fn sensitivity_stops_inside_its_parameter_sweep() {
    let (engine, netlist) = fixture();
    let frequencies = decade_sweep(40);
    let dc_total = poll_count("DC sensitivity", |abort| {
        engine.run_sensitivity_with_abort(&netlist, 2, "rval", 1.0e3, None, abort)
    });
    assert_polls_repeatedly("DC sensitivity", dc_total);
    assert_stops_exactly_at("DC sensitivity", interior(dc_total), |abort| {
        aborted(engine.run_sensitivity_with_abort(&netlist, 2, "rval", 1.0e3, None, abort))
    });

    let ac_total = poll_count("AC sensitivity", |abort| {
        engine.run_sensitivity_ac_with_abort(&netlist, 2, "rval", 1.0e3, &frequencies, None, abort)
    });
    assert_polls_repeatedly("AC sensitivity", ac_total);
    assert_stops_exactly_at("AC sensitivity", interior(ac_total), |abort| {
        aborted(engine.run_sensitivity_ac_with_abort(
            &netlist,
            2,
            "rval",
            1.0e3,
            &frequencies,
            None,
            abort,
        ))
    });
}

//=============================================================================
// Periodic / RF
//=============================================================================

#[test]
fn periodic_steady_state_stops_inside_its_shooting_loop() {
    let (engine, netlist) = fixture();
    let total = poll_count("PSS", |abort| {
        engine.run_pss_with_abort(&netlist, PssConfig::new(1.0e6), abort)
    });
    assert_polls_repeatedly("PSS", total);
    assert_stops_exactly_at("PSS", interior(total), |abort| {
        aborted(engine.run_pss_with_abort(&netlist, PssConfig::new(1.0e6), abort))
    });
}

#[test]
fn harmonic_balance_stops_inside_its_newton_loop() {
    let (engine, netlist) = fixture();
    let total = poll_count("harmonic balance", |abort| {
        engine.run_hb_with_abort(&netlist, HbConfig::new(1.0e6), abort)
    });
    assert_polls_repeatedly("harmonic balance", total);
    assert_stops_exactly_at("harmonic balance", interior(total), |abort| {
        aborted(engine.run_hb_with_abort(&netlist, HbConfig::new(1.0e6), abort))
    });
}

#[test]
fn periodic_ac_stops_inside_its_sideband_loop() {
    let (engine, netlist) = fixture();
    // A bare `PacConfig` names no fundamental, input, or output, so it is
    // refused before any loop starts. This is the fully specified request an
    // authored `.pac` produces.
    let config = || {
        PacConfig::new()
            .with_fundamental(1.0e6)
            .with_sweep(1.0e3, 1.0e5, 32)
            .with_sweep_type(PacSweepType::Linear)
            .with_sidebands(-2, 2)
            .with_input_source("v1")
            .with_output_node("out")
    };

    let total = poll_count("PAC", |abort| {
        engine.run_pac_with_abort(&netlist, config(), abort)
    });
    assert_polls_repeatedly("PAC", total);
    assert_stops_exactly_at("PAC", interior(total), |abort| {
        aborted(engine.run_pac_with_abort(&netlist, config(), abort))
    });
}

#[test]
fn periodic_noise_stops_inside_its_offset_loop() {
    let (engine, netlist) = fixture();
    let offsets = decade_sweep(20);
    let total = poll_count("PNoise", |abort| {
        engine.run_pnoise_with_abort(&netlist, 1.0e6, &offsets, "out", None, None, 3, abort)
    });
    assert_polls_repeatedly("PNoise", total);
    assert_stops_exactly_at("PNoise", interior(total), |abort| {
        aborted(
            engine.run_pnoise_with_abort(&netlist, 1.0e6, &offsets, "out", None, None, 3, abort),
        )
    });
}

//=============================================================================
// Statistical and parametric
//=============================================================================

const MONTE_CARLO_TRIALS: usize = 64;

fn monte_carlo_engine(workers: usize) -> Engine {
    // Monte Carlo runs its trials on scoped OS threads of its own rather than
    // through the shared compute pool, so its worker count is what `serialized`
    // cannot pin.
    let mut config = SimulationConfig::default();
    config.resource_limits.max_parallel_workers = workers;
    Engine::new(config)
}

fn run_monte_carlo<'a>(
    engine: &'a Engine,
    netlist: &'a Netlist,
) -> impl Fn(&CountingAbort) -> Result<rspice_core::analysis::MonteCarloResult, SimulationError> + 'a
{
    move |abort| {
        engine.run_monte_carlo_with_options_and_abort(
            netlist,
            MONTE_CARLO_TRIALS,
            2,
            Distribution::Gaussian { sigma: 0.01 },
            None,
            abort,
        )
    }
}

#[test]
fn monte_carlo_stops_inside_its_trial_loop() {
    let (_, netlist) = fixture();
    let engine = monte_carlo_engine(1);
    let run = run_monte_carlo(&engine, &netlist);

    let total = poll_count("Monte Carlo", &run);
    assert_polls_repeatedly("Monte Carlo", total);
    assert_stops_exactly_at("Monte Carlo", interior(total), |abort| aborted(run(abort)));
}

/// The parallel trial loop cannot stop at the exact poll that cancelled it —
/// each worker still in flight polls once more on its way out — so what it
/// owes is a bound rather than a number: no worker starts another trial, so
/// the overrun cannot exceed one poll per remaining trial.
#[test]
fn parallel_monte_carlo_stops_within_one_poll_per_worker() {
    let (_, netlist) = fixture();
    let engine = monte_carlo_engine(4);
    let run = run_monte_carlo(&engine, &netlist);

    let total = poll_count("parallel Monte Carlo", &run);
    assert_polls_repeatedly("parallel Monte Carlo", total);

    let threshold = interior(total);
    let abort = CountingAbort::new(threshold);
    assert!(
        aborted(run(&abort)),
        "the parallel trial loop must report cancellation"
    );
    assert_eq!(abort.observed_at(), Some(threshold + 1));
    assert!(
        abort.polls_after_abort() <= MONTE_CARLO_TRIALS,
        "the parallel trial loop kept working for {} polls after cancellation, past \
         the one-poll-per-remaining-trial bound of {MONTE_CARLO_TRIALS}",
        abort.polls_after_abort()
    );
}

#[test]
fn parametric_step_stops_inside_its_value_loop() {
    let (engine, netlist) = fixture();
    let values = (0..64)
        .map(|index| 1.0e3 + index as Value)
        .collect::<Vec<_>>();
    let total = poll_count("parametric step", |abort| {
        engine.run_step_with_abort(&netlist, "rval", &values, abort)
    });
    assert_polls_repeatedly("parametric step", total);
    assert_stops_exactly_at("parametric step", interior(total), |abort| {
        aborted(engine.run_step_with_abort(&netlist, "rval", &values, abort))
    });
}

//=============================================================================
// Post-processing
//=============================================================================

#[test]
fn fourier_stops_inside_its_transform() {
    let samples = 4_097usize;
    let time = (0..samples)
        .map(|index| index as Value / (samples - 1) as Value)
        .collect::<Vec<_>>();
    let values = time
        .iter()
        .map(|time| (2.0 * std::f64::consts::PI * time).sin())
        .collect::<Vec<_>>();
    let analysis = || FourierAnalysis::new(FourierConfig::new(1.0).with_harmonics(16));

    let total = poll_count("Fourier", |abort| {
        analysis().analyze_with_abort(&time, &values, abort)
    });
    assert_polls_repeatedly("Fourier", total);
    assert_stops_exactly_at("Fourier", interior(total), |abort| {
        matches!(
            analysis().analyze_with_abort(&time, &values, abort),
            Err(FourierError::Aborted)
        )
    });
}

/// The `.FFT` evaluation that runs after a transient is instrumented in its
/// own right, which this proves by cancelling past the point where the same
/// deck without `.fft` has already finished.
#[test]
fn fft_evaluation_stops_after_the_transient_it_follows() {
    let engine = Engine::new(SimulationConfig::default());
    let deck = |fft: &str| {
        Netlist::parse(&format!(
            "fft iteration bound\n\
             v1 in 0 sin(0 1 100k)\n\
             r1 in out 1k\n\
             c1 out 0 1n\n\
             .options fft fftout=1\n\
             {fft}\
             .end\n"
        ))
        .expect("FFT fixture parses")
    };
    let plain = deck("");
    let with_fft = deck(".fft v(out) np=256 window=rect freq=100k\n");

    let transient_only = poll_count("transient without .fft", |abort| {
        engine.run_tran_with_abort(&plain, 1.0e-4, 1.0e-7, abort)
    });
    let with_transform = poll_count("transient with .fft", |abort| {
        engine.run_tran_with_abort(&with_fft, 1.0e-4, 1.0e-7, abort)
    });
    assert!(
        with_transform > transient_only,
        "the transform polled no more than the transient alone ({with_transform} vs \
         {transient_only}), so `.fft` evaluation is uncancellable"
    );

    assert_stops_exactly_at("FFT evaluation", transient_only, |abort| {
        aborted(engine.run_tran_with_abort(&with_fft, 1.0e-4, 1.0e-7, abort))
    });
}

/// Compression scans every retained signal after the solve, so cancelling
/// past the plain transient's poll count lands inside the compressor.
#[test]
fn compression_stops_after_the_transient_it_compresses() {
    let (engine, netlist) = fixture();
    let compression = || CompressionConfig {
        enabled: true,
        abs_tol: 1.0e-9,
        rel_tol: 1.0e-6,
        maximum_retained_interval: 0.0,
    };

    let transient_only = poll_count("uncompressed transient", |abort| {
        engine.run_tran_with_abort(&netlist, 2.0e-5, 2.0e-8, abort)
    });
    let with_compression = poll_count("compressed transient", |abort| {
        engine.run_tran_compressed_with_abort(&netlist, 2.0e-5, 2.0e-8, compression(), abort)
    });
    assert!(
        with_compression > transient_only,
        "compression polled no more than the transient alone ({with_compression} vs \
         {transient_only}), so the compressor is uncancellable"
    );

    assert_stops_exactly_at("compression", transient_only, |abort| {
        aborted(engine.run_tran_compressed_with_abort(
            &netlist,
            2.0e-5,
            2.0e-8,
            compression(),
            abort,
        ))
    });
}

#[test]
fn checkpoint_serialization_stops_inside_its_encoder() {
    // The encoder polls once per serialized row, so the fixture is a ladder
    // wide enough that "halfway through the encoding" is inside a row loop
    // rather than between two header fields.
    let mut source = String::from("checkpoint iteration bound\nv1 n0 0 sin(0 1 1meg)\n");
    for index in 0..40 {
        source.push_str(&format!("r{index} n{index} n{} 1k\n", index + 1));
        source.push_str(&format!("c{index} n{} 0 1n\n", index + 1));
    }
    source.push_str(".end\n");
    let netlist = Netlist::parse(&source).expect("checkpoint fixture parses");
    let engine = Engine::new(SimulationConfig::default());

    let (_, checkpoint) = engine
        .run_tran_checkpointed_with_abort(
            &netlist,
            2.0e-5,
            2.0e-8,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("checkpointed transient completes");

    for (what, encoding) in [
        ("checkpoint text encoding", None),
        (
            "checkpoint packed encoding",
            Some(TransientCheckpointEncoding::Packed),
        ),
    ] {
        let total = poll_count(what, |abort| match encoding {
            None => checkpoint.to_text_with_abort(abort).map(|_| ()),
            Some(encoding) => checkpoint.to_bytes_with_abort(encoding, abort).map(|_| ()),
        });
        assert_polls_repeatedly(what, total);
        assert_stops_exactly_at(what, interior(total), |abort| match encoding {
            None => aborted(checkpoint.to_text_with_abort(abort)),
            Some(encoding) => aborted(checkpoint.to_bytes_with_abort(encoding, abort)),
        });
    }
}

//=============================================================================
// Run-axis materialization
//=============================================================================

fn axis_deck() -> Netlist {
    Netlist::parse(
        "axis iteration bound\n\
         .param rval=1k\n\
         v1 in 0 1\n\
         r1 in out {rval}\n\
         r2 out 0 1k\n\
         .step param rval 1k 8k 100\n\
         .temp 0 25 50 75\n\
         .op\n\
         .end\n",
    )
    .expect("axis fixture parses")
}

#[test]
fn coordinate_enumeration_stops_inside_its_cartesian_product() {
    let netlist = axis_deck();
    let limits = ResourceLimits::default();
    let plan =
        DeckPlan::from_netlist_with_abort(&netlist, &limits, &rspice_core::abort_signal::NoAbort)
            .expect("axis deck plans");

    let total = poll_count("coordinate enumeration", |abort| {
        plan.coordinates_with_abort(&limits, abort)
    });
    assert_polls_repeatedly("coordinate enumeration", total);
    assert_stops_exactly_at("coordinate enumeration", interior(total), |abort| {
        matches!(
            plan.coordinates_with_abort(&limits, abort),
            Err(DeckPlanError::Aborted)
        )
    });
}

#[test]
fn run_materialization_stops_inside_its_projection() {
    let netlist = axis_deck();
    let limits = ResourceLimits::default();
    let engine = Engine::new(SimulationConfig::default());
    let plan =
        DeckPlan::from_netlist_with_abort(&netlist, &limits, &rspice_core::abort_signal::NoAbort)
            .expect("axis deck plans");
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(
            &netlist,
            &plan,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("axis materializer");

    let total = poll_count("run materialization", |abort| {
        materializer.materialize_run_with_abort(0, abort)
    });
    assert_polls_repeatedly("run materialization", total);
    assert_stops_exactly_at("run materialization", interior(total), |abort| {
        matches!(
            materializer.materialize_run_with_abort(0, abort),
            Err(MaterializedRunError::Aborted)
        )
    });
}
