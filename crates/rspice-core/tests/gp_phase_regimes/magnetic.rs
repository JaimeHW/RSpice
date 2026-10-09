//! Coupled-winding event epochs against the two exact RL decay modes.
use super::*;
use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_perfect_coupling_preserves_null_flux_current_jumps_and_restart() {
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
            for (scale, ratio_squared) in [
                (1.0, 4.0),
                (1e-6, 4.0),
                (1.0, 9.0),
                (1e-6, 9.0),
                (1.0, 2.0),
                (1e-6, 2.0),
            ] {
                let turns = f64::sqrt(ratio_squared);
                for coupling in [-1.0, 1.0] {
                    let edge = 0.5 * scale;
                    let stop = 2.0 * scale;
                    let step = 0.005 * scale;
                    let deck = Netlist::parse(&format!(
                "perfect mutual flux\nV1 in 0 PWL(0 .5 {edge:e} .5 {edge:e} 1 {stop:e} 1)\nR1 in a 1\nL1 a 0 {scale:e}\nL2 b 0 {:e}\nR2 b 0 {ratio_squared}\nK1 L1 L2 {coupling}\nVC c 0 2\nVB base 0 PWL(0 .6 {edge:e} .6 {edge:e} .601 {stop:e} .601)\nQ1 c base 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF={:e} PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-5 ABSTOL=1e-15 VNTOL=1e-10 CHGTOL={:e}\n.end\n",
                ratio_squared * scale, 0.1 * scale, 1e-18 * scale,
            )).unwrap();
                    let mut config = SimulationConfig {
                        gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                        integration_method: method,
                        ..SimulationConfig::default().with_spice_dialect(dialect)
                    };
                    config.convergence_config.gmin_target = 0.0;
                    let engine = Engine::new(config);
                    let (full, checkpoints) = engine
                .run_tran_checkpoint_schedule_with_startup_mode(
                    &deck,
                    stop,
                    step,
                    TransientStartupMode::OperatingPoint,
                    &[edge],
                )
                .unwrap_or_else(|error| {
                    panic!("{dialect:?}/{method:?}, k={coupling}, turns={turns}, scale={scale:e}: {error}")
                });
                    let primary = full.try_branch_current_waveform_named("L1").unwrap();
                    let secondary = full.try_branch_current_waveform_named("L2").unwrap();
                    let a = full.try_voltage_waveform_named("a").unwrap();
                    let b = full.try_voltage_waveform_named("b").unwrap();
                    for (index, &time) in full.time.iter().enumerate() {
                        let (i1, i2) = if time < edge {
                            (0.5, 0.0)
                        } else {
                            let decay = (-(time - edge) / (2.0 * scale)).exp();
                            (1.0 - 0.25 * decay, -coupling * 0.25 / turns * decay)
                        };
                        assert!(
                            (primary[index] - i1).abs() < 3e-4,
                            "k={coupling}, scale={scale:e}, t={time:e}: I1={} expected {i1}",
                            primary[index]
                        );
                        assert!(
                            (secondary[index] - i2).abs() < 1.5e-4,
                            "k={coupling}, scale={scale:e}, t={time:e}: I2={} expected {i2}",
                            secondary[index]
                        );
                        // The rank-one inductance matrix requires v2 = turns*k*v1 at
                        // every finite point, including either side of the source edge.
                        assert!((b[index] - turns * coupling * a[index]).abs() < 1e-9);
                    }
                    let seam = full.time.iter().position(|time| *time == edge).unwrap();
                    // The jump lies in the nullspace of the full flux matrix. Neither
                    // winding current alone is a conserved magnetic state.
                    assert!((primary[seam] - 0.75).abs() < 1e-10);
                    assert!((secondary[seam] + coupling * 0.25 / turns).abs() < 1e-10);
                    assert!(
                        (primary[seam] + turns * coupling * secondary[seam] - 0.5).abs() < 1e-10
                    );
                    let checkpoint = TransientCheckpoint::from_bytes(
                        &checkpoints[0]
                            .checkpoint
                            .to_bytes(TransientCheckpointEncoding::Packed)
                            .unwrap(),
                    )
                    .unwrap();
                    let (resumed, _) = engine
                        .run_tran_resume(&deck, &checkpoint, stop, step)
                        .unwrap();
                    assert_eq!(resumed.time, full.time[seam..]);
                    for (actual, expected) in resumed
                        .voltages
                        .iter()
                        .chain(&resumed.branch_currents)
                        .zip(full.voltages.iter().chain(&full.branch_currents))
                    {
                        assert_eq!(actual, &expected[seam..]);
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_perfect_coupling_three_windings_preserve_both_voltage_constraints() {
    for scale in [1.0, 1e-6] {
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
        ] {
            let edge = 0.5 * scale;
            let stop = 2.0 * scale;
            let deck = Netlist::parse(&format!(
                "three perfect windings\nV1 in 0 PWL(0 .5 {edge:e} .5 {edge:e} 1 {stop:e} 1)\nR1 in a 1\nL1 a 0 {scale:e}\nL2 b 0 {:e}\nR2 b 0 4\nL3 d 0 {:e}\nR3 d 0 9\nK12 L1 L2 1\nK13 L1 L3 1\nK23 L2 L3 1\nVC c 0 2\nVB base 0 PWL(0 .6 {edge:e} .6 {edge:e} .601 {stop:e} .601)\nQ1 c base 0 qm\n.model qm NPN(IS=1e-16 BF=100 TF={:e} PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-5 ABSTOL=1e-15 VNTOL=1e-10 CHGTOL={:e}\n.end\n",
                4.0 * scale, 9.0 * scale, 0.1 * scale, 1e-18 * scale,
            )).unwrap();
            let mut config = SimulationConfig {
                gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                integration_method: method,
                ..SimulationConfig::default()
            };
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let (full, _) = engine
                .run_tran_checkpoint_schedule_with_startup_mode(
                    &deck,
                    stop,
                    0.005 * scale,
                    TransientStartupMode::OperatingPoint,
                    &[edge],
                )
                .unwrap_or_else(|error| panic!("{method:?}, scale={scale:e}: {error}"));
            let currents = ["L1", "L2", "L3"]
                .map(|name| full.try_branch_current_waveform_named(name).unwrap());
            let voltages =
                ["a", "b", "d"].map(|name| full.try_voltage_waveform_named(name).unwrap());
            for (index, &time) in full.time.iter().enumerate() {
                // The normalized rank-one matrix has one decay mode with
                // time constant 3*scale and two algebraic voltage constraints.
                let expected = if time < edge {
                    [0.5, 0.0, 0.0]
                } else {
                    let decay = (-(time - edge) / (3.0 * scale)).exp();
                    [1.0 - decay / 6.0, -decay / 12.0, -decay / 18.0]
                };
                for winding in 0..3 {
                    assert!((currents[winding][index] - expected[winding]).abs() < 2e-4);
                    assert!(
                        (voltages[winding][index] - (winding + 1) as f64 * voltages[0][index])
                            .abs()
                            < 1e-9
                    );
                }
            }
            let seam = full.time.iter().position(|&time| time == edge).unwrap();
            assert!((currents[0][seam] - 5.0 / 6.0).abs() < 1e-10);
            assert!((currents[1][seam] + 1.0 / 12.0).abs() < 1e-10);
            assert!((currents[2][seam] + 1.0 / 18.0).abs() < 1e-10);
            assert!(
                (currents[0][seam] + 2.0 * currents[1][seam] + 3.0 * currents[2][seam] - 0.5).abs()
                    < 1e-10
            );
        }
    }
}

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

// An ordinary flux-truncation retry can be much shorter than the output step.
// Its BJT charge Jacobian must not erase the finite source current, including
// when that current drives another circuit equation.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_small_step_preserves_source_current_and_cccs_feedback() {
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
            for polarity in [1.0, -1.0] {
                for control in ["L1", "R1"] {
                    let mut config = SimulationConfig::default().with_spice_dialect(dialect);
                    config.gp_transient_phase_model = GpTransientPhaseModel::NgspiceWeil;
                    config.integration_method = method;
                    config.convergence_config.gmin_target = 0.0;
                    let engine = Engine::new(config);
                    let source = Netlist::parse(&format!(
                        "small-step source current\nVC c 0 {}\nVB b 0 {}\nVD drive 0 DC 0 SIN(0 {} 1G)\nR1 drive coil 1k\nL1 coil 0 1u\nF1 b 0 {control} 1\nF2 copy 0 VB 1\nR2 copy 0 1k\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options device zeroresistancetol=1k\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) v(copy) i(vb) i(f1) i(f2)\n.end\n",
                        2.0*polarity, 0.7*polarity, 1e-3*polarity,
                        if polarity > 0.0 { "NPN" } else { "PNP" },
                    )).unwrap();
                    let (result, checkpoints) = engine
                        .run_tran_checkpoint_schedule_with_startup_mode(
                            &source,
                            50e-12,
                            2e-12,
                            TransientStartupMode::OperatingPoint,
                            &[25e-12],
                        )
                        .unwrap_or_else(|error| {
                            panic!("{dialect:?}/{method:?}/{polarity}/{control}: {error}")
                        });
                    let base = result.try_branch_current_waveform_named("vb").unwrap();
                    let forcing = result.try_branch_current_waveform_named("f1").unwrap();
                    let copy_current = result.try_branch_current_waveform_named("f2").unwrap();
                    let copy_voltage = result.try_voltage_waveform_named("copy").unwrap();
                    let voltage = result.try_voltage_waveform_named("b").unwrap();
                    let vt = thermal_voltage(dialect);
                    let current =
                        polarity * (diode(0.7, vt, dialect).0 / 100.0 + diode(-1.3, vt, dialect).0);
                    for (index, &time) in result.time.iter().enumerate() {
                        let expected = current + forcing[index];
                        assert!((voltage[index] - polarity * 0.7).abs() < 1e-10);
                        assert!(
                            (-base[index] - expected).abs() < 2e-11,
                            "{dialect:?}/{method:?}/{polarity}/{control} t={time:e}: {} != {expected:e}",
                            -base[index]
                        );
                        assert!((copy_current[index] - base[index]).abs() < 1e-15);
                        assert!((copy_voltage[index] - 1000.0 * expected).abs() < 2e-8);
                    }
                    let checkpoint = TransientCheckpoint::from_bytes(
                        &checkpoints[0]
                            .checkpoint
                            .to_bytes(TransientCheckpointEncoding::Packed)
                            .unwrap(),
                    )
                    .unwrap();
                    let (resumed, _) = engine
                        .run_tran_resume(&source, &checkpoint, 50e-12, 2e-12)
                        .unwrap();
                    let seam = result
                        .time
                        .iter()
                        .position(|&time| time == checkpoint.time)
                        .unwrap();
                    assert_eq!(resumed.time, result.time[seam..]);
                    for (actual, expected) in resumed
                        .voltages
                        .iter()
                        .chain(&resumed.branch_currents)
                        .zip(result.voltages.iter().chain(&result.branch_currents))
                    {
                        if expected.is_empty() {
                            assert!(actual.is_empty());
                            continue;
                        }
                        assert_eq!(actual.len(), expected.len() - seam);
                        for (&a, &e) in actual.iter().zip(&expected[seam..]) {
                            assert_eq!(a.to_bits(), e.to_bits());
                        }
                    }
                    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
                }
            }
        }
    }
}
