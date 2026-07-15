//! Transient checkpoint/restore: segmented runs must continue the same
//! trajectory the unsegmented run follows, file round-trips must be exact,
//! and mismatched state must be refused loudly.

use std::sync::Arc;

use rspice_core::analysis::IntegrationMethod;
#[cfg(feature = "veriloga-builtins")]
use rspice_core::engine::ConvergenceConfig;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect, TransientCheckpoint};
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
fn fixed_gear2_resume_uses_be_before_real_history_bdf2() {
    const TAU: f64 = 1.0e-3;
    const PRE_RESUME_STEP: f64 = 100.0e-6;
    const SPLIT: f64 = 500.0e-6;
    const RESUME_STEP: f64 = 1.0e-3;

    let netlist = Netlist::parse(
        "\
* checkpoint fixed-Gear2 order restart
r1 out 0 1k
c1 out 0 1u ic=1
.tran 100u 3m uic
.end
",
    )
    .expect("fixed-Gear2 checkpoint deck parses");
    let first_engine = Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::Gear2,
        transient_initial_timestep: Some(PRE_RESUME_STEP),
        ..Default::default()
    });
    let (_, checkpoint) = first_engine
        .run_tran_checkpointed(&netlist, SPLIT, PRE_RESUME_STEP)
        .expect("fixed-Gear2 first segment completes");

    let resume_grid = Arc::new(vec![SPLIT, SPLIT + RESUME_STEP, SPLIT + 2.0 * RESUME_STEP]);
    let resume_engine = Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::Gear2,
        transient_initial_timestep: Some(RESUME_STEP),
        locked_time_grid: Some(resume_grid),
        ..Default::default()
    });
    let (resumed, _) = resume_engine
        .run_tran_resume(
            &netlist,
            &checkpoint,
            SPLIT + 2.0 * RESUME_STEP,
            RESUME_STEP,
        )
        .expect("fixed-Gear2 resumed segment completes");
    let trace = &resumed.voltages[out_index(&resumed)];

    assert_eq!(resumed.time.len(), 3, "resume grid has two real intervals");
    assert_eq!(resumed.time[0].to_bits(), checkpoint.time.to_bits());

    let first_dt = resumed.time[1] - resumed.time[0];
    let expected_be = trace[0] / (1.0 + first_dt / TAU);
    assert!(
        (trace[1] - expected_be).abs() <= 1.0e-12,
        "first resumed Gear2 interval must be backward Euler: expected {expected_be:e}, got {:e}",
        trace[1]
    );

    let second_dt = resumed.time[2] - resumed.time[1];
    let ratio = second_dt / first_dt;
    let a0 = (1.0 + 2.0 * ratio) / (1.0 + ratio);
    let a1 = 1.0 + ratio;
    let a2 = -(ratio * ratio) / (1.0 + ratio);
    let expected_bdf2 = (a1 * trace[1] + a2 * trace[0]) / (a0 + second_dt / TAU);
    assert!(
        (trace[2] - expected_bdf2).abs() <= 1.0e-12,
        "second resumed Gear2 interval must use the real first interval as BDF2 history: expected {expected_bdf2:e}, got {:e}",
        trace[2]
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
fn xyce_signal_history_modes_survive_segmented_disk_checkpoints() {
    for selector in [2, 3] {
        let deck = format!(
            "\
* Xyce NEWLTE signal-history checkpoint bench
vzero in 0 0
r1 in out 1k
c1 out 0 100n ic=1
.options timeint reltol=1e-5 abstol=1e-7 newlte={selector}
.tran 10u 1m uic
.end
"
        );
        let netlist = Netlist::parse(&deck).expect("NEWLTE checkpoint deck parses");
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..Default::default()
        });

        let full = engine
            .run_tran(&netlist, 1.0e-3, 20.0e-6)
            .expect("unsegmented NEWLTE run completes");
        let full_out = out_index(&full);
        let (_, checkpoint) = engine
            .run_tran_checkpointed(&netlist, 0.4e-3, 20.0e-6)
            .expect("NEWLTE first segment completes");

        let checkpoint_text = checkpoint.to_text();
        assert!(
            checkpoint_text.contains(&format!("lte_reference_mode {selector}\n")),
            "checkpoint records the resolved NEWLTE={selector} provenance"
        );
        let local_count = checkpoint_text
            .lines()
            .find_map(|line| line.strip_prefix("lte_signal_local "))
            .expect("checkpoint contains the signal-local vector header")
            .parse::<usize>()
            .expect("signal-local count is numeric");
        if selector == 2 {
            assert_eq!(local_count, 0, "NEWLTE=2 stores one global reference");
        } else {
            assert!(local_count > 0, "NEWLTE=3 stores per-variable references");
        }

        let path = std::env::temp_dir().join(format!(
            "rspice_newlte{selector}_checkpoint_{}.ckpt",
            std::process::id()
        ));
        checkpoint.save(&path).expect("NEWLTE checkpoint saves");
        let loaded = TransientCheckpoint::load(&path).expect("NEWLTE checkpoint loads");
        let _ = std::fs::remove_file(&path);
        assert_eq!(checkpoint, loaded, "NEWLTE checkpoint round-trip is exact");

        let (memory_resume, _) = engine
            .run_tran_resume(&netlist, &checkpoint, 1.0e-3, 20.0e-6)
            .expect("NEWLTE checkpoint resumes from memory");
        let (disk_resume, _) = engine
            .run_tran_resume(&netlist, &loaded, 1.0e-3, 20.0e-6)
            .expect("NEWLTE checkpoint resumes from disk");
        assert_eq!(memory_resume.time, disk_resume.time);
        assert!(
            memory_resume.voltages[out_index(&memory_resume)]
                .iter()
                .zip(&disk_resume.voltages[out_index(&disk_resume)])
                .all(|(memory, disk)| memory.to_bits() == disk.to_bits()),
            "NEWLTE={selector} disk resume is bit-identical to memory resume"
        );

        let mut worst = 0.0_f64;
        let resumed_out = out_index(&memory_resume);
        for sample in 1..=20 {
            let time = 0.4e-3 + sample as f64 * 30.0e-6;
            let expected = interpolate(&full.time, &full.voltages[full_out], time);
            let actual = interpolate(
                &memory_resume.time,
                &memory_resume.voltages[resumed_out],
                time,
            );
            worst = worst.max((expected - actual).abs());
        }
        assert!(
            worst < 5.0e-3,
            "NEWLTE={selector} segmented trajectory tracks the full run (worst |delta|={worst})"
        );
    }
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
            // `astate` returns an accepted-sample history value, so inserting
            // an arbitrary segment endpoint changes its adaptive sample grid.
            // Split at the source's existing falling-edge breakpoint to compare
            // checkpoint continuation without introducing a new timepoint.
            31.0e-9,
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

#[cfg(feature = "veriloga-builtins")]
#[test]
fn generated_veriloga_checkpoint_preserves_reactive_history_and_provenance() {
    use rspice_core::device::veriloga_generated::builtins;

    const STEP: f64 = 250.0e-12;
    const SPLIT_INDEX: usize = 80;
    const STOP_INDEX: usize = 160;
    const SPLIT: f64 = SPLIT_INDEX as f64 * STEP;
    const STOP: f64 = STOP_INDEX as f64 * STEP;

    assert!(
        builtins::builtin_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case("DIODE_CMC")),
        "the canonical generated CMC diode must be present"
    );

    let netlist = Netlist::parse(
        "\
* generated Verilog-A checkpoint bench: junction-charge history
vin in 0 pulse(0 1 0 1n 1n 8n 20n)
r1 in out 1k
d1 out 0 dcmc
.model dcmc d level=2002
.tran 250p 40n
.end
",
    )
    .expect("generated CMC diode checkpoint deck parses");
    let engine = Engine::new(SimulationConfig {
        tolerance: 1.0e-13,
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(STEP),
        convergence_config: ConvergenceConfig::default()
            .with_voltage_tolerances(1.0e-12, 1.0e-14)
            .with_current_tolerance(1.0e-15)
            .with_residual_reltol(1.0e-12),
        locked_time_grid: Some(Arc::new(
            (0..=STOP_INDEX).map(|index| index as f64 * STEP).collect(),
        )),
        ..Default::default()
    });

    let full = engine
        .run_tran(&netlist, STOP, STEP)
        .expect("unsegmented generated CMC diode run completes");
    let full_out = out_index(&full);
    let (first, checkpoint) = engine
        .run_tran_checkpointed(&netlist, SPLIT, STEP)
        .expect("generated CMC diode first segment completes");

    let checkpoint_text = checkpoint.to_text();
    assert!(
        checkpoint_text.contains("generated_veriloga_state_available 1\n"),
        "current checkpoints record generated-state availability"
    );
    assert!(
        checkpoint_text.contains("generated_veriloga_states 1\n"),
        "the generated diode instance is serialized"
    );
    assert!(
        checkpoint_text.contains("ddt_state 5\n"),
        "the CMC diode's five accepted ddt histories are serialized"
    );
    let ddt_rows = checkpoint_text
        .lines()
        .skip_while(|line| *line != "ddt_state 5")
        .skip(1)
        .take(5)
        .collect::<Vec<_>>();
    assert_eq!(ddt_rows.len(), 5);
    assert!(
        ddt_rows.iter().any(|row| {
            let fields = row.split_whitespace().collect::<Vec<_>>();
            fields.len() == 4
                && fields[3] == "1"
                && fields[..3]
                    .iter()
                    .any(|value| value.parse::<f64>().is_ok_and(|value| value != 0.0))
        }),
        "the accepted diode-charge history is initialized and nonzero at the checkpoint"
    );

    let checkpoint_path = std::env::temp_dir().join(format!(
        "rspice_generated_veriloga_checkpoint_{}.ckpt",
        std::process::id()
    ));
    checkpoint
        .save(&checkpoint_path)
        .expect("generated checkpoint saves");
    let disk_checkpoint =
        TransientCheckpoint::load(&checkpoint_path).expect("generated checkpoint loads");
    let _ = std::fs::remove_file(&checkpoint_path);
    assert_eq!(
        checkpoint, disk_checkpoint,
        "generated persistent state round-trips exactly through disk"
    );

    let (memory_resume, _) = engine
        .run_tran_resume(&netlist, &checkpoint, STOP, STEP)
        .expect("generated checkpoint resumes from memory");
    let (disk_resume, _) = engine
        .run_tran_resume(&netlist, &disk_checkpoint, STOP, STEP)
        .expect("generated checkpoint resumes from disk");
    assert_eq!(memory_resume.time, disk_resume.time);
    assert!(
        memory_resume.voltages[out_index(&memory_resume)]
            .iter()
            .zip(&disk_resume.voltages[out_index(&disk_resume)])
            .all(|(memory, disk)| memory.to_bits() == disk.to_bits()),
        "disk and in-memory generated-state resumes are bit-identical"
    );

    let resumed_out = out_index(&memory_resume);
    let mut worst = 0.0_f64;
    for sample in 1..=20 {
        let time = SPLIT + sample as f64 * 1.0e-9;
        let expected = interpolate(&full.time, &full.voltages[full_out], time);
        let actual = interpolate(
            &memory_resume.time,
            &memory_resume.voltages[resumed_out],
            time,
        );
        worst = worst.max((expected - actual).abs());
    }
    assert!(
        worst < 1.0e-12,
        "generated checkpoint continuation tracks the unsegmented trajectory (worst |delta|={worst})"
    );
    assert_eq!(
        first.voltages[out_index(&first)]
            .last()
            .expect("first segment has a seam sample")
            .to_bits(),
        memory_resume.voltages[resumed_out][0].to_bits(),
        "generated checkpoint carries the seam solution bit-exactly"
    );

    let upgraded_legacy_text = checkpoint_text
        .lines()
        .take_while(|line| !line.starts_with("generated_veriloga_state_available "))
        .collect::<Vec<_>>()
        .join("\n")
        + "\ngenerated_veriloga_state_available 0\ngenerated_veriloga_states 0\n";
    let legacy = TransientCheckpoint::from_text(&upgraded_legacy_text)
        .expect("the upgraded legacy checkpoint parses with unavailable generated state");
    let legacy_error = engine
        .run_tran_resume(&netlist, &legacy, STOP, STEP)
        .expect_err("legacy checkpoints cannot silently reset generated reactive history");
    assert!(
        legacy_error
            .to_string()
            .contains("does not contain generated Verilog-A persistent state"),
        "legacy refusal identifies missing generated persistent state: {legacy_error}"
    );

    let generated_header = checkpoint_text
        .lines()
        .find(|line| line.starts_with("generated_veriloga_state "))
        .expect("checkpoint contains generated instance provenance");
    let identity = generated_header
        .split_whitespace()
        .nth(3)
        .expect("generated checkpoint header contains a model identity");
    assert_ne!(identity, "0".repeat(64));
    let wrong_identity_text = checkpoint_text.replacen(identity, &"0".repeat(64), 1);
    let wrong_identity = TransientCheckpoint::from_text(&wrong_identity_text)
        .expect("syntactically valid checkpoint with stale model identity parses");
    let identity_error = engine
        .run_tran_resume(&netlist, &wrong_identity, STOP, STEP)
        .expect_err("checkpoint state from a different generated artifact must be refused");
    assert!(
        identity_error.to_string().contains("model identity"),
        "identity refusal identifies generated-model provenance: {identity_error}"
    );
}
