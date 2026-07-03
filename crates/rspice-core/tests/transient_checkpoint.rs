//! Transient checkpoint/restore: segmented runs must continue the same
//! trajectory the unsegmented run follows, file round-trips must be exact,
//! and mismatched state must be refused loudly.

use rspice_core::engine::{Engine, SimulationConfig, TransientCheckpoint};
use rspice_core::netlist::Netlist;
use rspice_core::xspice::{register_data_file, unregister_data_file};

/// Sine-driven RC: smooth, source phase depends on absolute time, so a
/// resume that mishandles t0 or the capacitor history shows up immediately.
const DECK: &str = "\
* checkpoint bench: sine-driven rc
vin in 0 sin(0 1 1meg)
r1 in out 1k
c1 out 0 159.155p
.tran 1n 2u
.end
";

const XSPICE_GAIN_DECK: &str = "\
* checkpoint bench: stateless xspice gain
vin in 0 sin(0 1 1meg)
a1 in out amp
.model amp gain (gain=2)
rload out 0 1k
.tran 1n 40n
.end
";

const XSPICE_INTEGRATOR_DECK: &str = "\
* checkpoint bench: stateful xspice integrator
vin in 0 sin(0 1 1meg)
a1 in out integ
.model integ int (gain=1 out_lower_limit=-10 out_upper_limit=10)
rload out 0 1k
.tran 1n 40n
.end
";

const TAU_STEP: f64 = 1e-9;

fn interpolate(time: &[f64], values: &[f64], t: f64) -> f64 {
    let idx = time.partition_point(|x| *x < t);
    if idx == 0 {
        return values[0];
    }
    if idx >= time.len() {
        return *values.last().unwrap();
    }
    let (t0, t1) = (time[idx - 1], time[idx]);
    let (v0, v1) = (values[idx - 1], values[idx]);
    if t1 == t0 {
        v0
    } else {
        v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    }
}

fn out_index(result: &rspice_core::engine::TransientResult) -> usize {
    result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present")
}

fn assert_segmented_xspice_deck_tracks(
    label: &str,
    deck: &str,
    tstop: f64,
    split: f64,
    step: f64,
    tolerance: f64,
) {
    let netlist = Netlist::parse(deck).unwrap_or_else(|err| panic!("{label} deck parses: {err}"));
    let engine = Engine::new(SimulationConfig::default());

    let full = engine
        .run_tran(&netlist, tstop, step)
        .unwrap_or_else(|err| panic!("{label} full run completes: {err}"));
    let full_out = out_index(&full);

    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, split, step)
        .unwrap_or_else(|err| panic!("{label} first segment completes: {err}"));
    let (second, _) = engine
        .run_tran_resume(&netlist, &checkpoint, tstop, step)
        .unwrap_or_else(|err| panic!("{label} resumed segment completes: {err}"));
    let second_out = out_index(&second);

    let mut worst = 0.0f64;
    let sample_step = (tstop - split) / 16.0;
    for k in 1..=16 {
        let t = split + (k as f64) * sample_step;
        let v_full = interpolate(&full.time, &full.voltages[full_out], t);
        let v_seg = interpolate(&second.time, &second.voltages[second_out], t);
        worst = worst.max((v_full - v_seg).abs());
    }
    assert!(
        worst < tolerance,
        "{label} checkpoint resume must track the full run (worst |delta| = {worst})"
    );

    let v_seam_first = *first.voltages[out_index(&first)].last().unwrap();
    let v_seam_second = second.voltages[second_out][0];
    assert_eq!(
        v_seam_first.to_bits(),
        v_seam_second.to_bits(),
        "{label} seam state is carried bit-exactly"
    );
}

#[test]
fn segmented_run_continues_the_unsegmented_trajectory() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    // Reference: one unsegmented run to 2 µs.
    let full = engine
        .run_tran(&netlist, 2e-6, TAU_STEP)
        .expect("full run completes");
    let full_out = out_index(&full);

    // Segmented: run to 1 µs, checkpoint, resume to 2 µs.
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-6, TAU_STEP)
        .expect("first segment completes");
    assert!(
        (checkpoint.time - 1e-6).abs() < 1e-9,
        "checkpoint lands at the segment end, got {}",
        checkpoint.time
    );
    let (second, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-6, TAU_STEP)
        .expect("resumed segment completes");

    let second_out = out_index(&second);
    assert!(
        (second.time[0] - checkpoint.time).abs() < 1e-15,
        "resumed run starts at the checkpoint time"
    );

    // The resumed trajectory must track the unsegmented one. Compare at
    // sample times across the second segment; the restart introduces only
    // integration-tolerance-level differences.
    let mut worst = 0.0f64;
    for k in 1..=40 {
        let t = 1.0e-6 + (k as f64) * 24e-9;
        let v_full = interpolate(&full.time, &full.voltages[full_out], t);
        let v_seg = interpolate(&second.time, &second.voltages[second_out], t);
        worst = worst.max((v_full - v_seg).abs());
    }
    assert!(
        worst < 2e-3,
        "segmented trajectory must track the unsegmented run (worst |Δ| = {worst})"
    );

    // Continuity at the seam: the resumed first point equals the first
    // segment's last point exactly (same solution vector).
    let v_seam_first = *first.voltages[out_index(&first)].last().unwrap();
    let v_seam_second = second.voltages[second_out][0];
    assert_eq!(
        v_seam_first.to_bits(),
        v_seam_second.to_bits(),
        "seam state is carried bit-exactly"
    );
}

#[test]
fn checkpoint_file_round_trip_resumes_identically() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-6, TAU_STEP)
        .expect("segment completes");

    let path = std::env::temp_dir().join("rspice_checkpoint_roundtrip_test.ckpt");
    checkpoint.save(&path).expect("checkpoint saves");
    let loaded = TransientCheckpoint::load(&path).expect("checkpoint loads");
    let _ = std::fs::remove_file(&path);
    assert_eq!(checkpoint, loaded, "file round-trip is exact");

    let (from_memory, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 1.5e-6, TAU_STEP)
        .expect("resume from memory");
    let (from_file, _) = engine
        .run_tran_resume(&netlist, &loaded, 1.5e-6, TAU_STEP)
        .expect("resume from file");

    assert_eq!(from_memory.time.len(), from_file.time.len());
    let out = out_index(&from_memory);
    assert!(
        from_memory.voltages[out]
            .iter()
            .zip(&from_file.voltages[out_index(&from_file)])
            .all(|(a, b)| a.to_bits() == b.to_bits()),
        "file-loaded checkpoint resumes bit-identically"
    );
}

#[test]
fn mismatched_netlist_is_refused() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let other = Netlist::parse(
        "\
* different circuit
vin in 0 dc 1
r1 in out 2k
c1 out 0 100p
.tran 1n 1u
.end
",
    )
    .expect("other deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 0.5e-6, TAU_STEP)
        .expect("segment completes");

    let err = engine
        .run_tran_resume(&other, &checkpoint, 1e-6, TAU_STEP)
        .expect_err("mismatched netlist must be refused");
    let message = format!("{err}");
    assert!(
        message.contains("different netlist"),
        "diagnostic names the problem: {message}"
    );
}

#[test]
fn resume_requires_a_later_stop_time() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-6, TAU_STEP)
        .expect("segment completes");

    let err = engine
        .run_tran_resume(&netlist, &checkpoint, 0.5e-6, TAU_STEP)
        .expect_err("earlier tstop must be refused");
    assert!(format!("{err}").contains("must exceed"));
}

#[test]
fn stateless_xspice_checkpoint_resume_tracks_unsegmented_gain() {
    let netlist = Netlist::parse(XSPICE_GAIN_DECK).expect("XSPICE gain deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let full = engine
        .run_tran(&netlist, 40e-9, TAU_STEP)
        .expect("full XSPICE gain run completes");
    let full_out = out_index(&full);

    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 20e-9, TAU_STEP)
        .expect("first XSPICE gain segment can run");
    let (second, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 40e-9, TAU_STEP)
        .expect("stateless XSPICE gain resumes");

    let second_out = out_index(&second);
    let mut worst = 0.0f64;
    for k in 1..=12 {
        let t = 20.0e-9 + (k as f64) * 1.5e-9;
        let v_full = interpolate(&full.time, &full.voltages[full_out], t);
        let v_seg = interpolate(&second.time, &second.voltages[second_out], t);
        worst = worst.max((v_full - v_seg).abs());
    }
    assert!(
        worst < 1e-9,
        "stateless XSPICE gain checkpoint resume must track the full run (worst |Î”| = {worst})"
    );

    let v_seam_first = *first.voltages[out_index(&first)].last().unwrap();
    let v_seam_second = second.voltages[second_out][0];
    assert_eq!(
        v_seam_first.to_bits(),
        v_seam_second.to_bits(),
        "stateless XSPICE seam state is carried bit-exactly"
    );
}

#[test]
fn stateful_xspice_checkpoint_resume_tracks_unsegmented_integrator() {
    let netlist = Netlist::parse(XSPICE_INTEGRATOR_DECK).expect("XSPICE int deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let full = engine
        .run_tran(&netlist, 40e-9, TAU_STEP)
        .expect("full XSPICE int run completes");
    let full_out = out_index(&full);

    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 20e-9, TAU_STEP)
        .expect("first XSPICE int segment can run");
    let (second, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 40e-9, TAU_STEP)
        .expect("stateful XSPICE int resumes");

    let second_out = out_index(&second);
    let mut worst = 0.0f64;
    for k in 1..=12 {
        let t = 20.0e-9 + (k as f64) * 1.5e-9;
        let v_full = interpolate(&full.time, &full.voltages[full_out], t);
        let v_seg = interpolate(&second.time, &second.voltages[second_out], t);
        worst = worst.max((v_full - v_seg).abs());
    }
    assert!(
        worst < 1e-9,
        "stateful XSPICE int checkpoint resume must track the full run (worst |delta| = {worst})"
    );

    let v_seam_first = *first.voltages[out_index(&first)].last().unwrap();
    let v_seam_second = second.voltages[second_out][0];
    assert_eq!(
        v_seam_first.to_bits(),
        v_seam_second.to_bits(),
        "stateful XSPICE seam state is carried bit-exactly"
    );
}

#[test]
fn stateful_analog_xspice_checkpoint_resume_tracks_additional_models() {
    let cases = [
        (
            "d_dt",
            "\
* checkpoint bench: differentiator
vin in 0 sin(0 1 1meg)
a1 in out diff
.model diff d_dt (gain=1e-6 out_lower_limit=-10 out_upper_limit=10)
rload out 0 1k
.tran 0.5n 60n
.end
",
            60.0e-9,
            30.0e-9,
            0.5e-9,
            1.0e-6,
        ),
        (
            "hyst",
            "\
* checkpoint bench: hysteresis
vin in 0 pulse(0 2 0 1n 1n 20n 40n)
a1 in out h
.model h hyst (in_low=0.5 in_high=1.5 hyst=0.1 out_lower_limit=0 out_upper_limit=5 input_domain=0.01)
rload out 0 1k
.tran 1n 90n
.end
",
            90.0e-9,
            45.0e-9,
            1.0e-9,
            1.0e-9,
        ),
        (
            "slew",
            "\
* checkpoint bench: slew
vin in 0 pulse(0 1 0 1n 1n 20n 40n)
a1 in out sl
.model sl slew (rise_slope=1e8 fall_slope=1e8)
rload out 0 1k
.tran 1n 90n
.end
",
            90.0e-9,
            45.0e-9,
            1.0e-9,
            1.0e-9,
        ),
        (
            "astate",
            "\
* checkpoint bench: analog state return
vin in 0 pulse(0 1 0 1n 1n 10n 20n)
a1 in out ast
.model ast astate (astate_no=1)
rload out 0 1k
.tran 1n 60n
.end
",
            60.0e-9,
            30.0e-9,
            1.0e-9,
            1.0e-9,
        ),
    ];

    for (label, deck, tstop, split, step, tolerance) in cases {
        assert_segmented_xspice_deck_tracks(label, deck, tstop, split, step, tolerance);
    }
}

#[test]
fn event_driven_xspice_checkpoint_resume_is_refused_until_event_state_is_serialized() {
    let uri = "virtual://transient_checkpoint/event_state_blocker";
    register_data_file(uri, "0 0s\n1n 1s\n").expect("register virtual d_source data");
    let deck = format!(
        "\
* xspice event checkpoint boundary
a_src [d] src
a_dac [d] [out] dac
.model src d_source (input_file=\"{uri}\")
.model dac dac_bridge (out_low=0 out_high=5 out_undef=2.5 t_rise=1p t_fall=1p)
rload out 0 1k
.tran 100p 2n
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("XSPICE event deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let (_, checkpoint) = engine
        .run_tran_checkpointed(&netlist, 1e-9, 100e-12)
        .expect("first XSPICE event segment can run");
    let err = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-9, 100e-12)
        .expect_err("event-driven XSPICE checkpoint resume must be refused");
    let _ = unregister_data_file(uri);
    let message = format!("{err}");
    assert!(
        message.contains("XSPICE")
            && message.contains("event node values")
            && message.contains("Run XSPICE transient decks unsegmented"),
        "diagnostic should explain the unsupported checkpoint boundary: {message}"
    );
}
