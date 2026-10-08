//! Coupled-winding event epochs against the two exact RL decay modes.
use super::*;
use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_events_preserve_mutual_flux_modes_and_exact_checkpoint_continuation() {
    let edge = 0.5e-9;
    let stop = 3e-9;
    let step = 2e-12;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
        ] {
            let mut config = SimulationConfig {
                gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                integration_method: method,
                ..SimulationConfig::default().with_spice_dialect(dialect)
            };
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            for coupling in [-0.5, 0.5] {
                let deck = Netlist::parse(&format!(
                    "GP event mutual flux\nV1 in 0 PWL(0 .5 .5n .5 .5n 1 3n 1)\nR1 in a 1k\nL1 a 0 1u\nL2 b 0 4u\nR2 b 0 4k\nK1 L1 L2 {coupling}\nVC c 0 2\nVB base 0 PWL(0 .6 .5n .6 .5n .601 3n .601)\nQ1 c base 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF=1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-5 ABSTOL=1e-15 VNTOL=1e-10 CHGTOL=1e-26\n.end\n",
                )).unwrap();
                let (full, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &deck,
                        stop,
                        step,
                        TransientStartupMode::OperatingPoint,
                        &[edge, 1.6e-9],
                    )
                    .unwrap_or_else(|error| panic!("{dialect:?}/{method:?}/{coupling}: {error}"));
                assert_eq!(checkpoints.len(), 2);
                let primary = full.try_branch_current_waveform_named("L1").unwrap();
                let secondary = full.try_branch_current_waveform_named("L2").unwrap();
                let a = full.try_voltage_waveform_named("a").unwrap();
                let b = full.try_voltage_waveform_named("b").unwrap();
                // R1/L1 = R2/L2 = 1/ns. Scaling each current by sqrt(L)
                // diagonalizes the flux matrix into modes 1 +/- k.
                for (index, &time) in full.time.iter().enumerate() {
                    let elapsed = (time - edge).max(0.0) * 1e9;
                    let plus = (-elapsed / (1.0 + coupling)).exp();
                    let minus = (-elapsed / (1.0 - coupling)).exp();
                    let i1 = 0.5e-3 + 0.25e-3 * (2.0 - plus - minus);
                    let i2 = 0.125e-3 * (minus - plus);
                    // A 2 ps ceiling resolves the fastest 0.5 ns mode with
                    // at least 250 intervals, including first-order steps.
                    assert!(
                        (primary[index] - i1).abs() < 1e-6,
                        "{dialect:?}/{method:?}/{coupling} at {time:e}: L1={} expected {i1}",
                        primary[index]
                    );
                    assert!(
                        (secondary[index] - i2).abs() < 0.5e-6,
                        "{dialect:?}/{method:?}/{coupling} at {time:e}: L2={} expected {i2}",
                        secondary[index]
                    );
                    assert!((b[index] + 4e3 * secondary[index]).abs() < 1e-9);
                    if time != edge {
                        let drive = if time < edge { 0.5 } else { 1.0 };
                        assert!((a[index] + 1e3 * primary[index] - drive).abs() < 1e-9);
                    }
                }
                for retained in checkpoints {
                    let checkpoint = TransientCheckpoint::from_bytes(
                        &retained
                            .checkpoint
                            .to_bytes(TransientCheckpointEncoding::Packed)
                            .unwrap(),
                    )
                    .unwrap();
                    let (resumed, _) = engine
                        .run_tran_resume(&deck, &checkpoint, stop, step)
                        .unwrap();
                    let seam = full
                        .time
                        .iter()
                        .position(|time| *time == checkpoint.time)
                        .unwrap();
                    assert_eq!(
                        resumed.time,
                        full.time[seam..],
                        "{dialect:?}/{method:?}/{coupling}"
                    );
                    for (actual, expected) in resumed
                        .voltages
                        .iter()
                        .chain(&resumed.branch_currents)
                        .zip(full.voltages.iter().chain(&full.branch_currents))
                    {
                        assert_eq!(
                            actual,
                            &expected[seam..],
                            "{dialect:?}/{method:?}/{coupling}"
                        );
                    }
                }
            }
        }
    }
}
