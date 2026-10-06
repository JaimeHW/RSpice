//! Per-analysis transport storage includes live histories and retained copies.

use std::sync::Arc;

use rspice_core::engine::CompressionConfig;
use rspice_core::engine::TransientStartupMode;
use rspice_core::{
    Engine, GpTransientPhaseModel, Netlist, ResourceKind, SimulationConfig, SimulationError,
};

fn deck(devices: usize, phase: f64) -> Netlist {
    let mut text = format!(
        "GP history budget\nVC c 0 2\nVB b 0 .7\n.model qm NPN IS=1e-16 BF=100 TF=1n PTF={phase}\n"
    );
    for index in 0..devices {
        text.push_str(&format!("Q{index} c b 0 qm\n"));
    }
    text.push_str(".end\n");
    Netlist::parse(&text).unwrap()
}

fn config(limit: usize) -> SimulationConfig {
    let mut config = SimulationConfig {
        locked_time_grid: Some(Arc::new(
            (0..=1000).map(|index| f64::from(index) * 1e-12).collect(),
        )),
        ..SimulationConfig::default()
    };
    config.resource_limits.max_transport_history_bytes = limit;
    config
}

// Delay exceeds the run horizon, so every observed accepted point remains
// retained. Initial physical startup also keeps one sided knot and its known
// derivative order. Account for spare capacity independently from result size.
fn live_bytes(points: usize) -> usize {
    (points.max(4).next_power_of_two() + 2 * 4) * std::mem::size_of::<(f64, f64)>()
}

fn assert_limit(error: SimulationError, limit: usize) -> usize {
    let SimulationError::ResourceLimit(error) = error else {
        panic!("expected transport memory policy refusal, got {error}");
    };
    assert_eq!(error.resource, ResourceKind::TransportHistoryBytes);
    assert_eq!(error.limit, limit);
    assert!(error.requested > limit);
    error.requested
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn live_transport_storage_is_bounded_independently_of_waveform_retention() {
    let netlist = deck(1, 90.0);
    let baseline = Engine::new(config(usize::MAX))
        .run_tran(&netlist, 1e-9, 1e-12)
        .unwrap();
    assert!(baseline.time.len() >= 1001);
    let bytes = live_bytes(baseline.time.len());
    Engine::new(config(bytes))
        .run_tran(&netlist, 1e-9, 1e-12)
        .unwrap();
    let limited = Engine::new(config(bytes - 1));
    assert_eq!(
        assert_limit(
            limited.run_tran(&netlist, 1e-9, 1e-12).unwrap_err(),
            bytes - 1
        ),
        bytes
    );
    assert_limit(
        limited
            .run_tran_compressed(&netlist, 1e-9, 1e-12, CompressionConfig::default())
            .unwrap_err(),
        bytes - 1,
    );
    let rerun = Engine::new(config(bytes))
        .run_tran(&netlist, 1e-9, 1e-12)
        .unwrap();
    assert_eq!(rerun.time, baseline.time);
    assert_eq!(rerun.voltages, baseline.voltages);
    assert_eq!(rerun.branch_currents, baseline.branch_currents);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn the_history_budget_is_aggregate_across_devices_and_startup_modes() {
    let baseline = Engine::new(config(usize::MAX))
        .run_tran(&deck(1, 90.0), 1e-9, 1e-12)
        .unwrap();
    let bytes = live_bytes(baseline.time.len());
    let engine = Engine::new(config(bytes));
    for mode in [
        TransientStartupMode::OperatingPoint,
        TransientStartupMode::Uic,
    ] {
        assert_limit(
            engine
                .run_tran_with_startup_mode(&deck(2, 90.0), 1e-9, 1e-12, mode)
                .unwrap_err(),
            bytes,
        );
    }
    let rerun = engine.run_tran(&deck(1, 90.0), 1e-9, 1e-12).unwrap();
    assert_eq!(rerun.time, baseline.time);
    assert_eq!(rerun.voltages, baseline.voltages);
    assert_eq!(rerun.branch_currents, baseline.branch_currents);
    let no_records = Engine::new(config(0));
    assert_limit(
        no_records
            .run_tran(&deck(1, 90.0), 1e-9, 1e-12)
            .unwrap_err(),
        0,
    );
    no_records.run_tran(&deck(1, 0.0), 1e-9, 1e-12).unwrap();
    let mut weil = config(0);
    weil.gp_transient_phase_model = GpTransientPhaseModel::NgspiceWeil;
    Engine::new(weil)
        .run_tran(&deck(1, 90.0), 1e-9, 1e-12)
        .unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn checkpoint_capture_accounts_for_the_live_history_and_all_retained_copies() {
    let netlist = deck(1, 90.0);
    let baseline = Engine::new(config(usize::MAX))
        .run_tran(&netlist, 1e-9, 1e-12)
        .unwrap();
    let live = live_bytes(baseline.time.len());
    let copy = (baseline.time.len() + 2) * std::mem::size_of::<(f64, f64)>();
    let engine = Engine::new(config(live));
    assert_eq!(
        assert_limit(
            engine
                .run_tran_checkpointed(&netlist, 1e-9, 1e-12)
                .unwrap_err(),
            live
        ),
        live + copy
    );
    Engine::new(config(live + copy))
        .run_tran_checkpointed(&netlist, 1e-9, 1e-12)
        .unwrap();
    let limited = Engine::new(config(live + copy));
    assert_limit(
        limited
            .run_tran_checkpoint_schedule_with_startup_mode(
                &netlist,
                1e-9,
                1e-12,
                TransientStartupMode::OperatingPoint,
                &[2.5e-10, 5e-10, 7.5e-10],
            )
            .unwrap_err(),
        live + copy,
    );
    let (waveform, checkpoints) = Engine::new(config(4 * (live + copy)))
        .run_tran_checkpoint_schedule_with_startup_mode(
            &netlist,
            1e-9,
            1e-12,
            TransientStartupMode::OperatingPoint,
            &[2.5e-10, 5e-10, 7.5e-10],
        )
        .unwrap();
    assert_eq!(checkpoints.len(), 3);
    assert!(waveform.time.len() >= 1001);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn restoration_obeys_the_new_budget_without_mutating_the_input_checkpoint() {
    let netlist = deck(1, 90.0);
    let config = SimulationConfig::default();
    let engine = Engine::new(config.clone());
    let (_, checkpoint) = engine.run_tran_checkpointed(&netlist, 1e-9, 1e-12).unwrap();
    let encoded = checkpoint.to_text();
    let (baseline, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-9, 1e-12)
        .unwrap();
    let mut limited = config;
    limited.resource_limits.max_transport_history_bytes = 0;
    assert_limit(
        Engine::new(limited)
            .run_tran_resume(&netlist, &checkpoint, 2e-9, 1e-12)
            .unwrap_err(),
        0,
    );
    assert_eq!(checkpoint.to_text(), encoded);
    let (rerun, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 2e-9, 1e-12)
        .unwrap();
    assert_eq!(rerun.time, baseline.time);
    assert_eq!(rerun.voltages, baseline.voltages);
    assert_eq!(rerun.branch_currents, baseline.branch_currents);
}
