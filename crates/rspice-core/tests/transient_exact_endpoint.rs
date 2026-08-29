//! A transient's requested horizon is a numerical contract: source/device
//! evaluation, accepted samples, scheduled saves, and continuation checkpoints
//! must all use the same bit-exact time.

use std::sync::Arc;

use rspice_core::engine::{
    Engine, SimulationConfig, TransientCheckpoint, TransientResult, TransientStartupMode,
};
use rspice_core::netlist::Netlist;
use rspice_core::numerics::integration::IntegrationMethod;

const PREVIOUS_TIME: f64 = f64::from_bits(0x3e02_db21_7f74_4098);
const STOP_TIME: f64 = f64::from_bits(0x3e42_ffcc_ca47_13bf);

const TIME_SOURCE_DECK: &str = "\
* bit-exact transient endpoint contract
Bclock clock 0 V={TIME}
Rload clock 0 1
.tran 1n 20n
.end
";

fn engine() -> Engine {
    Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(PREVIOUS_TIME),
        locked_time_grid: Some(Arc::new(vec![0.0, PREVIOUS_TIME, STOP_TIME])),
        ..Default::default()
    })
}

fn clock_index(result: &TransientResult) -> usize {
    result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("clock"))
        .expect("TIME source output node is present")
}

fn assert_exact_terminal_sample(result: &TransientResult, context: &str) {
    let terminal_time = result.time.last().expect("transient has a terminal sample");
    assert_eq!(
        terminal_time.to_bits(),
        STOP_TIME.to_bits(),
        "{context}: terminal sample uses the exact requested stop time"
    );
    assert!(
        result.time.windows(2).all(|pair| pair[0] < pair[1]),
        "{context}: accepted sample times are strictly increasing: {:?}",
        result.time
    );
    assert_eq!(
        result
            .time
            .iter()
            .filter(|time| time.to_bits() == STOP_TIME.to_bits())
            .count(),
        1,
        "{context}: the exact stop time occurs once"
    );

    let terminal_clock = result.voltages[clock_index(result)]
        .last()
        .expect("TIME source has a terminal value");
    assert_eq!(
        terminal_clock.to_bits(),
        STOP_TIME.to_bits(),
        "{context}: the source is evaluated at exact tstop rather than merely relabeled afterward"
    );
}

#[test]
fn final_time_is_canonical_across_run_checkpoint_schedule_and_resume() {
    let remaining = STOP_TIME - PREVIOUS_TIME;
    assert_ne!(
        (PREVIOUS_TIME + remaining).to_bits(),
        STOP_TIME.to_bits(),
        "the public oracle must retain its one-ULP subtraction/addition mismatch"
    );

    let netlist = Netlist::parse(TIME_SOURCE_DECK).expect("endpoint contract deck parses");
    let engine = engine();
    let max_step = STOP_TIME;

    let full = engine
        .run_tran(&netlist, STOP_TIME, max_step)
        .expect("ordinary transient completes");
    assert_exact_terminal_sample(&full, "ordinary run");
    assert_eq!(
        full.time[full.time.len() - 2].to_bits(),
        PREVIOUS_TIME.to_bits(),
        "the oracle's penultimate accepted point is exact"
    );
    assert_eq!(
        full.step_sizes
            .last()
            .expect("terminal step size")
            .to_bits(),
        remaining.to_bits(),
        "the terminal integration interval is the exact remaining duration"
    );

    let (scheduled_result, scheduled) = engine
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            STOP_TIME,
            max_step,
            TransientStartupMode::OperatingPoint,
            &[PREVIOUS_TIME, STOP_TIME],
        )
        .expect("scheduled checkpoint transient completes");
    assert_exact_terminal_sample(&scheduled_result, "scheduled checkpoint run");
    assert_eq!(scheduled.len(), 2, "both scheduled saves are captured");
    let scheduled_clock = clock_index(&scheduled_result);
    for (entry, expected_time) in scheduled.iter().zip([PREVIOUS_TIME, STOP_TIME]) {
        assert_eq!(entry.nominal_time.to_bits(), expected_time.to_bits());
        assert_eq!(
            entry.checkpoint.time.to_bits(),
            expected_time.to_bits(),
            "a save due on an accepted point captures that exact point"
        );
        assert_eq!(
            entry.checkpoint.solution[scheduled_clock].to_bits(),
            expected_time.to_bits(),
            "a scheduled checkpoint captures the solution evaluated at its exact accepted time"
        );
    }

    let (first_segment, seam_checkpoint) = engine
        .run_tran_checkpointed(&netlist, PREVIOUS_TIME, max_step)
        .expect("first checkpointed segment completes");
    assert_eq!(
        first_segment.time.last().expect("seam sample").to_bits(),
        PREVIOUS_TIME.to_bits()
    );
    assert_eq!(
        seam_checkpoint.time.to_bits(),
        PREVIOUS_TIME.to_bits(),
        "the resume seam is bit-exact"
    );
    let seam_clock = clock_index(&first_segment);
    assert_eq!(
        seam_checkpoint.solution[seam_clock].to_bits(),
        PREVIOUS_TIME.to_bits(),
        "the seam checkpoint solution is evaluated at the exact seam time"
    );
    let serialized_seam = TransientCheckpoint::from_text(&seam_checkpoint.to_text())
        .expect("the exact seam checkpoint survives serialization");

    let (resumed, final_checkpoint) = engine
        .run_tran_resume(&netlist, &serialized_seam, STOP_TIME, max_step)
        .expect("checkpoint continuation completes");
    assert_eq!(
        resumed.time.first().expect("resumed seam sample").to_bits(),
        PREVIOUS_TIME.to_bits()
    );
    assert_exact_terminal_sample(&resumed, "resumed run");
    assert_eq!(
        final_checkpoint.time.to_bits(),
        STOP_TIME.to_bits(),
        "the final continuation checkpoint uses exact tstop"
    );
    assert_eq!(
        final_checkpoint.solution[clock_index(&resumed)].to_bits(),
        STOP_TIME.to_bits(),
        "the final checkpoint solution is evaluated at exact tstop"
    );
}
