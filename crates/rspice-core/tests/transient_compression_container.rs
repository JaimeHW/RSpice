//! The compressed transient result is a complete result container.
//!
//! Compression is a declared numerical approximation of *waveform samples*.
//! It is not permission to drop a channel, a unit, an owner, an event trace, a
//! validity mask, a post-process product, or the identity of the run that
//! produced the result. These tests compare a compressed run against the
//! uncompressed run of the same deck and require everything except the
//! discarded samples to be identical.

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{
    CompressionConfig, Engine, TransientChannelAvailability, TransientChannelRole,
    TransientChannelSample, TransientChannelUnit, TransientResult, TransientResultCompressed,
    TransientSampleAbsence, evaluate_transient_post_results,
};
use rspice_core::{Netlist, SimulationConfig, Value};

/// One deck exercising every analog channel family plus XSPICE digital and
/// real event traces in the same run.
///
/// A bare `.SAVE` name selects an XSPICE event vector; a typed `V(node)`
/// deliberately does not. The save set here therefore names the digital and
/// real event nodes bare, keeps some analog channels and leaves others out, so
/// the fixture also covers a deliberately not-projected channel.
const MIXED_INVENTORY_DECK: &str = "\
compressed transient container inventory\n\
vin a 0 pwl(0 0 1n 0 1.1n 3.3)\n\
a_adc [a] [d] adc\n\
a_dac [d] [drv] dac\n\
.model adc adc_bridge (in_low=0.8 in_high=2.0 rise_delay=1p fall_delay=1p)\n\
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)\n\
r1 drv out 1k\n\
c1 out 0 1p\n\
aobs out rnode obs\n\
.model obs v_to_real (gain=2)\n\
VDD dd 0 5\n\
M1 dd drv 0 0 NM W=10u L=1u\n\
.model NM NMOS (LEVEL=1 VTO=1 KP=100u)\n\
VMEM memory 0 0.2\n\
.model MRM MEMRISTOR LEVEL=2 RON=50 ROFF=1k\n\
YMEMRISTOR MR1 memory 0 MRM IVRELATION=1\n\
.save D RNODE V(A) V(DRV) V(OUT) V(DD) V(MEMORY) I(VIN) I(VDD) @M1[gm]\n\
.end\n";

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn compression() -> CompressionConfig {
    CompressionConfig {
        abs_tol: 1.0e-6,
        rel_tol: 1.0e-4,
        enabled: true,
        maximum_retained_interval: 0.0,
    }
}

fn run_pair(
    deck: &str,
    tstop: Value,
    max_step: Value,
) -> (TransientResult, TransientResultCompressed) {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = engine();
    let uncompressed = engine
        .run_tran(&netlist, tstop, max_step)
        .expect("uncompressed transient solves");
    let compressed = engine
        .run_tran_compressed(&netlist, tstop, max_step, compression())
        .expect("compressed transient solves");
    (uncompressed, compressed)
}

/// Index of one retained time inside the accepted grid, proving the retained
/// grid is a subset of the accepted one rather than a resampling of it.
fn accepted_index(accepted: &[Value], time: Value) -> usize {
    accepted
        .binary_search_by(|candidate| candidate.total_cmp(&time))
        .unwrap_or_else(|_| panic!("retained time {time:.17e} is not an accepted solver point"))
}

#[test]
fn compressed_inventory_matches_uncompressed_for_every_channel_family() {
    let (uncompressed, compressed) = run_pair(MIXED_INVENTORY_DECK, 20.0e-9, 0.1e-9);

    assert!(
        !uncompressed.digital_traces.is_empty(),
        "the fixture must exercise digital event traces"
    );
    assert!(
        !uncompressed.real_traces.is_empty(),
        "the fixture must exercise real event traces"
    );
    assert!(
        !uncompressed.device_op_traces.is_empty(),
        "the fixture must exercise device operating-point traces"
    );
    assert!(
        !uncompressed.store_traces.is_empty(),
        "the fixture must exercise typed device store traces"
    );
    assert!(
        compressed.time.len() < uncompressed.time.len(),
        "the fixture must actually decimate: {} of {} points retained",
        compressed.time.len(),
        uncompressed.time.len()
    );

    // Event traces are never decimated: interpolation is undefined for them.
    assert_eq!(compressed.digital_traces, uncompressed.digital_traces);
    assert_eq!(compressed.real_traces, uncompressed.real_traces);

    // Names, order, units and owners survive.
    assert_eq!(compressed.node_names(), uncompressed.node_names);
    assert_eq!(compressed.branch_names(), uncompressed.branch_names);
    assert_eq!(compressed.num_nodes(), uncompressed.num_nodes);
    let expected_channels = uncompressed.voltages.len()
        + uncompressed.branch_currents.len()
        + uncompressed.device_op_traces.len()
        + uncompressed.store_traces.len();
    assert_eq!(compressed.channels.len(), expected_channels);

    for channel in &compressed.channels {
        let descriptor = &channel.descriptor;
        assert_eq!(descriptor.value_type(), "real");
        assert_eq!(descriptor.shape(), "scalar");
        match descriptor.role() {
            TransientChannelRole::NodeVoltage { .. } => {
                assert_eq!(*descriptor.unit(), TransientChannelUnit::Volt);
            }
            TransientChannelRole::BranchCurrent { .. } => {
                assert_eq!(*descriptor.unit(), TransientChannelUnit::Ampere);
            }
            TransientChannelRole::DeviceObservable { .. }
            | TransientChannelRole::DeviceStore { .. } => {
                assert_eq!(*descriptor.unit(), TransientChannelUnit::Unspecified);
            }
        }
    }

    // Every retained sample is the accepted sample, bit for bit.
    let indices = compressed
        .time
        .iter()
        .map(|time| accepted_index(&uncompressed.time, *time))
        .collect::<Vec<_>>();
    for (retained, &index) in compressed.step_sizes.iter().zip(&indices) {
        assert_eq!(retained.to_bits(), uncompressed.step_sizes[index].to_bits());
    }
    let mut compared_families = 0usize;
    for channel in &compressed.channels {
        if channel.availability != TransientChannelAvailability::Available {
            continue;
        }
        let source: &[Value] = match channel.descriptor.role() {
            TransientChannelRole::NodeVoltage { node, .. } => uncompressed
                .try_voltage_waveform_named(node)
                .expect("node channel exists uncompressed"),
            TransientChannelRole::BranchCurrent { branch } => uncompressed
                .try_branch_current_waveform_named(branch)
                .expect("branch channel exists uncompressed"),
            TransientChannelRole::DeviceObservable { device, parameter } => uncompressed
                .try_device_op_waveform_named(device, parameter)
                .expect("device channel exists uncompressed"),
            TransientChannelRole::DeviceStore { store } => uncompressed
                .try_store_waveform_named(store)
                .expect("store channel exists uncompressed"),
        };
        compared_families += 1;
        for (sample, &index) in channel.samples.iter().zip(&indices) {
            match sample {
                TransientChannelSample::Value(value) => assert_eq!(
                    value.to_bits(),
                    source[index].to_bits(),
                    "retained sample of '{}' at accepted index {index} was rewritten",
                    channel.descriptor.canonical_name()
                ),
                TransientChannelSample::Absent(_) => assert!(
                    !source[index].is_finite(),
                    "'{}' claims an absence at accepted index {index}, which holds {}",
                    channel.descriptor.canonical_name(),
                    source[index]
                ),
            }
        }
    }
    assert!(compared_families >= 4, "every family must be compared");
    let not_projected = compressed
        .channels
        .iter()
        .filter(|channel| channel.availability == TransientChannelAvailability::NotProjected)
        .collect::<Vec<_>>();
    assert!(
        !not_projected.is_empty(),
        "the fixture must exercise a deliberately not-projected channel"
    );
    for channel in not_projected {
        assert!(
            !channel.descriptor.canonical_name().is_empty(),
            "a not-projected channel keeps its identity"
        );
        assert!(channel.samples.is_empty());
    }
}

#[test]
fn compressed_expansion_restores_the_uncompressed_inventory_shape() {
    let (uncompressed, compressed) = run_pair(MIXED_INVENTORY_DECK, 20.0e-9, 0.1e-9);
    let expanded = compressed
        .clone()
        .try_into_transient()
        .expect("a fully present container expands");

    assert_eq!(expanded.node_names, uncompressed.node_names);
    assert_eq!(expanded.branch_names, uncompressed.branch_names);
    assert_eq!(expanded.num_nodes, uncompressed.num_nodes);
    assert_eq!(expanded.digital_traces, uncompressed.digital_traces);
    assert_eq!(expanded.real_traces, uncompressed.real_traces);
    assert_eq!(
        expanded
            .device_op_traces
            .iter()
            .map(|trace| (trace.device_name.clone(), trace.parameter.clone()))
            .collect::<Vec<_>>(),
        uncompressed
            .device_op_traces
            .iter()
            .map(|trace| (trace.device_name.clone(), trace.parameter.clone()))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        expanded
            .store_traces
            .iter()
            .map(|trace| trace.name.clone())
            .collect::<Vec<_>>(),
        uncompressed
            .store_traces
            .iter()
            .map(|trace| trace.name.clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(expanded.time, compressed.time);
}

/// `.FFT`, `.FOUR` and transient `.MEASURE` all read the exact accepted
/// trajectory, so a compressed run publishes the same numbers as an
/// uncompressed one.
const POST_RESULT_DECK: &str = "\
compressed transient post-process parity\n\
V1 in 0 SIN(0 1 100k)\n\
R1 in out 1k\n\
C1 out 0 1n\n\
.four 100k v(out)\n\
.fft v(out) np=64\n\
.meas tran vmax MAX v(out)\n\
.meas tran vmin MIN v(out)\n\
.tran 100n 20u\n\
.end\n";

#[test]
fn compressed_post_results_match_the_uncompressed_trajectory() {
    let netlist = Netlist::parse(POST_RESULT_DECK).expect("post-process deck parses");
    let engine = engine();
    let uncompressed = engine
        .run_tran(&netlist, 20.0e-6, 100.0e-9)
        .expect("uncompressed transient solves");
    let compressed = engine
        .run_tran_compressed(&netlist, 20.0e-6, 100.0e-9, compression())
        .expect("compressed transient solves");

    let expected = evaluate_transient_post_results(
        &netlist,
        &uncompressed,
        engine.config().resource_limits,
        &NoAbort,
    )
    .expect("post-process products evaluate on the accepted trajectory");

    assert!(!expected.fft.is_empty(), ".FFT coverage is non-vacuous");
    assert!(
        !expected.fourier.is_empty(),
        ".FOUR coverage is non-vacuous"
    );
    assert!(
        !expected.measurements.is_empty(),
        ".MEASURE coverage is non-vacuous"
    );
    assert!(
        compressed.time.len() < uncompressed.time.len(),
        "the fixture must actually decimate"
    );

    assert_eq!(compressed.post_results, expected);
    assert_eq!(compressed.post_results.fft, uncompressed.fft_results);
    for measurement in &compressed.post_results.measurements {
        assert!(
            measurement.value.is_some(),
            "measurement '{}' did not evaluate: {:?}",
            measurement.name,
            measurement.error
        );
    }
}

/// An `--allow-nonfinite` run is compressible: the non-finite samples become
/// typed absences instead of blocking compression entirely, and the validity
/// mask says which retained samples they are.
#[test]
fn a_non_finite_analog_sample_compresses_as_a_typed_absence() {
    let netlist = Netlist::parse(
        "non-finite compression policy\nV1 out 0 1\nR1 out 0 1k\n.tran 1u 10u\n.end\n",
    )
    .expect("deck parses");
    let engine = engine();
    let mut result = engine
        .run_tran(&netlist, 10.0e-6, 1.0e-6)
        .expect("transient solves");
    assert!(result.time.len() > 3);
    result.voltages[0][1] = Value::INFINITY;

    let compressed = engine
        .compress_transient_result_with_abort(
            &netlist,
            &result,
            &CompressionConfig::none(),
            &NoAbort,
        )
        .expect("a non-finite sample is representable as an absence");
    let channel = &compressed.channels[0];
    assert_eq!(
        channel.samples[1],
        TransientChannelSample::Absent(TransientSampleAbsence::NonFinite)
    );
    assert!(
        !channel.validity()[1],
        "the validity mask marks the absent sample"
    );
    assert!(channel.dense_values().is_none());
    assert!(
        channel.validity().iter().filter(|valid| **valid).count() == channel.samples.len() - 1,
        "only the injected sample is absent"
    );

    let error = compressed
        .try_into_transient()
        .expect_err("a non-finite absence cannot become a plain transient sample");
    assert!(
        error.contains("non-finite") && error.contains("v(out)"),
        "unexpected error: {error}"
    );
}

/// `.OPTIONS OUTPUT INITIAL_INTERVAL` composes with compression: every
/// accepted sample the authored lattice reads is mandatory, and the retained
/// grid replays the same rows.
const INTERVAL_SCHEDULE_DECK: &str = "\
compressed transient interval-output composition\n\
V1 in 0 PWL(0 0 1u 1 20u 1)\n\
R1 in out 1k\n\
C1 out 0 1n\n\
.options output initial_interval=1u\n\
.tran 100n 20u\n\
.end\n";

#[test]
fn compression_retains_every_sample_the_interval_lattice_reads() {
    let netlist = Netlist::parse(INTERVAL_SCHEDULE_DECK).expect("interval schedule deck parses");
    assert!(
        netlist.options.output_interval_schedule.is_some(),
        "the fixture must author an INITIAL_INTERVAL lattice"
    );
    let engine = engine();
    let uncompressed = engine
        .run_tran(&netlist, 20.0e-6, 100.0e-9)
        .expect("uncompressed transient solves");
    let compressed = engine
        .run_tran_compressed(&netlist, 20.0e-6, 100.0e-9, compression())
        .expect("an authored interval lattice composes with compression");

    assert!(
        compressed.time.len() < uncompressed.time.len(),
        "the fixture must actually decimate: {} of {} points retained",
        compressed.time.len(),
        uncompressed.time.len()
    );

    let limits = engine.config().resource_limits;
    let accepted_projection = uncompressed
        .output_projection(
            &[],
            netlist.options.output_interval_schedule.as_ref(),
            0.0,
            *uncompressed.time.last().expect("accepted samples exist"),
            limits.max_analysis_points,
        )
        .expect("the accepted grid projects the authored lattice");
    let expanded = compressed
        .clone()
        .try_into_transient()
        .expect("the compressed container expands");
    let retained_projection = expanded
        .output_projection(
            &[],
            netlist.options.output_interval_schedule.as_ref(),
            0.0,
            *expanded.time.last().expect("retained samples exist"),
            limits.max_analysis_points,
        )
        .expect("the retained grid projects the same authored lattice");

    assert_eq!(
        retained_projection
            .times()
            .iter()
            .map(|time| time.to_bits())
            .collect::<Vec<_>>(),
        accepted_projection
            .times()
            .iter()
            .map(|time| time.to_bits())
            .collect::<Vec<_>>(),
        "the compressed result no longer reproduces the authored output lattice"
    );

    let accepted_rows = accepted_projection
        .project(
            uncompressed
                .try_voltage_waveform_named("out")
                .expect("the output node is solved"),
        )
        .expect("the accepted waveform projects");
    let retained_rows = retained_projection
        .project(
            expanded
                .try_voltage_waveform_named("out")
                .expect("the retained output node exists"),
        )
        .expect("the retained waveform projects");
    assert_eq!(accepted_rows.len(), retained_rows.len());
    for (index, (accepted, retained)) in accepted_rows.iter().zip(&retained_rows).enumerate() {
        assert_eq!(
            accepted.to_bits(),
            retained.to_bits(),
            "authored output row {index} moved under compression"
        );
    }
}
