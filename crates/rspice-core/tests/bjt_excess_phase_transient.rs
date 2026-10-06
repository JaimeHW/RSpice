//! Public GP excess-phase admission, continuation and invalid-parameter contracts.
//! Independent accuracy oracles also run through the public APIs in physical_dispatch_tests.

use rspice_core::engine::{
    CompressionConfig, Engine, SimulationConfig, SpiceDialect, TransientStartupMode,
};
use rspice_core::{Netlist, SimulationError};

fn deck(parameters: &str) -> Netlist {
    Netlist::parse(&format!(
        "GP transient phase\nVC c 0 2\nVB b 0 DC .7 SIN(.7 1u 1G)\nQ1 c b 0 model\n.model model NPN IS=1e-16 BF=100 BR=1 {parameters}\n.options RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 GMIN=0\n.end\n"
    ))
    .unwrap()
}

fn assert_noncausal(error: SimulationError, instance: &str) {
    assert!(
        matches!(error, SimulationError::ParameterDomain(_)),
        "{error}"
    );
    let message = error.to_string();
    assert!(
        message.contains(instance)
            && message.contains("PTF")
            && message.contains("nonnegative causal delay"),
        "{message}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_phase_runs_both_polarities_models_dialects_and_startup_modes() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for model in [
            rspice_core::GpTransientPhaseModel::ExactDelay,
            rspice_core::GpTransientPhaseModel::NgspiceWeil,
        ] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.gp_transient_phase_model = model;
            let engine = Engine::new(config);
            for private in ["", "RB=100 RBM=20 IRB=1e-5 RE=1 RC=2"] {
                for phase in ["90", "21"] {
                    for kind in ["NPN", "PNP"] {
                        let sign = if kind == "PNP" { "-" } else { "" };
                        let text = format!(
                            "GP transient phase\nVC c 0 {sign}2\nVB b 0 DC {sign}.7 SIN({sign}.7 {sign}1u 1G)\nQ1 c b 0 model\n.model model {kind} IS=1e-16 BF=100 BR=1 TF=1n PTF={phase} {private}\n.options RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 GMIN=0\n.end\n"
                        );
                        let source = Netlist::parse(&text).unwrap();
                        for startup in [
                            TransientStartupMode::OperatingPoint,
                            TransientStartupMode::Uic,
                        ] {
                            let result = engine.run_tran_with_startup_mode(&source, 2e-9, 4e-12, startup)
                                .unwrap_or_else(|error| panic!("{dialect:?}/{model:?}/{kind}/{phase}/{private}/{startup:?}: {error}"));
                            assert_eq!(result.time.last(), Some(&2e-9));
                            assert!(
                                result
                                    .voltages
                                    .iter()
                                    .chain(&result.branch_currents)
                                    .flatten()
                                    .all(|value| value.is_finite())
                            );
                        }
                    }
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_phase_support_covers_preflight_checkpoint_compression_and_continuation() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let source = deck("TF=1n PTF=21 RB=100 RBM=20 IRB=1e-5");
    for model in [
        rspice_core::GpTransientPhaseModel::ExactDelay,
        rspice_core::GpTransientPhaseModel::NgspiceWeil,
    ] {
        let config = SimulationConfig {
            gp_transient_phase_model: model,
            ..SimulationConfig::default()
        };
        let engine = Engine::new(config);
        assert!(
            engine
                .preflight_transient_checkpoint(&source)
                .unwrap()
                .is_resumable()
        );
        let (full, scheduled) = engine
            .run_tran_checkpoint_schedule_with_startup_mode(
                &source,
                2e-9,
                4e-12,
                TransientStartupMode::OperatingPoint,
                &[1e-9],
            )
            .unwrap();
        assert_eq!(scheduled.len(), 1);
        for encoding in [
            TransientCheckpointEncoding::Packed,
            TransientCheckpointEncoding::Unpacked,
        ] {
            let checkpoint = &scheduled[0].checkpoint;
            let restored =
                TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&source, &restored, 2e-9, 4e-12)
                .unwrap();
            let seam = full
                .time
                .iter()
                .position(|time| *time == checkpoint.time)
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
        let (first, checkpoint) = engine.run_tran_checkpointed(&source, 1e-9, 4e-12).unwrap();
        let (continued, final_checkpoint) = engine
            .run_tran_resume(&source, &checkpoint, 2e-9, 4e-12)
            .unwrap();
        assert_eq!(continued.time.first(), first.time.last());
        assert_eq!(final_checkpoint.time, 2e-9);
        let compressed = engine
            .run_tran_compressed(&source, 2e-9, 4e-12, CompressionConfig::default())
            .unwrap();
        assert_eq!(compressed.time.last(), Some(&2e-9));
        compressed.validate().unwrap();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_noncausal_phase_refuses_before_startup_and_checkpoint_publication() {
    let source = deck("TF=1n PTF=-21");
    for model in [
        rspice_core::GpTransientPhaseModel::ExactDelay,
        rspice_core::GpTransientPhaseModel::NgspiceWeil,
    ] {
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: model,
            ..SimulationConfig::default()
        });
        assert_noncausal(
            engine.preflight_transient_checkpoint(&source).unwrap_err(),
            "Q1",
        );
        assert_noncausal(
            engine
                .run_tran_checkpointed(&source, 2e-9, 4e-12)
                .unwrap_err(),
            "Q1",
        );
        for startup in [
            TransientStartupMode::OperatingPoint,
            TransientStartupMode::Uic,
        ] {
            assert_noncausal(
                engine
                    .run_tran_with_startup_mode(&source, 2e-9, 4e-12, startup)
                    .unwrap_err(),
                "Q1",
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_zero_phase_preserves_transient_and_checkpoint_behavior() {
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice));
    for private in ["", "RB=100 RBM=20 IRB=1e-5 RE=1 RC=2"] {
        for (base, disabled) in [("TF=1n", "TF=1n PTF=-0"), ("TF=0", "TF=0 PTF=90")] {
            let a = deck(&format!("{base} {private}"));
            let b = deck(&format!("{disabled} {private}"));
            assert!(
                engine
                    .preflight_transient_checkpoint(&b)
                    .unwrap()
                    .is_resumable()
            );
            let a = engine.run_tran(&a, 2e-9, 4e-12).unwrap();
            let (b, checkpoint) = engine.run_tran_checkpointed(&b, 2e-9, 4e-12).unwrap();
            assert_eq!(a.time, b.time);
            assert_eq!(a.voltages, b.voltages);
            assert_eq!(a.branch_currents, b.branch_currents);
            assert_eq!(checkpoint.time, 2e-9);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_unused_phase_model_does_not_block_an_unaffected_circuit() {
    let source = Netlist::parse(
        "unused phase model\nV1 in 0 1\nR1 in 0 1k\n.model unused NPN TF=1n PTF=90\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    assert!(
        engine
            .preflight_transient_checkpoint(&source)
            .unwrap()
            .is_resumable()
    );
    engine.run_tran(&source, 1e-9, 1e-10).unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_hierarchical_noncausal_phase_names_the_elaborated_instance() {
    let source = Netlist::parse(
        "hierarchical phase\nVC c 0 2\nVB b 0 .7\nXstage c b stage\n.subckt stage c b\nQphase c b 0 local\n.model local NPN TF=1n PTF=-21\n.ends\n.end\n",
    ).unwrap();
    let engine = Engine::default();
    let error = engine.run_tran(&source, 1e-9, 1e-10).unwrap_err();
    assert!(
        error
            .to_string()
            .to_ascii_lowercase()
            .contains("xstage.qphase")
    );
    assert_noncausal(error, "PTF");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_exact_transport_refines_toward_the_independent_delayed_exponential() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let vt = match dialect {
            SpiceDialect::Ngspice => 1.380_648_52e-23 * 300.15 / 1.602_176_620_8e-19,
            SpiceDialect::Xyce => 1.380_622_6e-23 * 300.15 / 1.602_191_8e-19,
            SpiceDialect::BestAvailable => 1.380_649e-23 * 300.15 / 1.602_176_634e-19,
        };
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let source = Netlist::parse(&format!(
                "Independent delayed transport\nVC c 0 {}\nVB b 0 SIN({} {} 1G)\nQ1 c b 0 model\n.model model {kind} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232 TNOM=27\n.options TEMP=27 GMIN=0 RELTOL=.01 ABSTOL=1e-15 VNTOL=1e-9\n.end\n",
                polarity*2.0,polarity*0.6,polarity*0.01,
            )).unwrap();
            let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
            let mut errors = Vec::new();
            for step in [8e-11, 8e-12] {
                let result = engine.run_tran(&source, 4.37e-9, step).unwrap();
                let collector = result.try_branch_current_waveform_named("vc").unwrap();
                let mut error = 0.0_f64;
                for (&time, &actual) in result.time.iter().zip(collector) {
                    let base = 0.6 + 0.01 * (std::f64::consts::TAU * 1e9 * time).sin();
                    let delayed_base =
                        0.6 + 0.01 * (std::f64::consts::TAU * 1e9 * (time - 1e-9).max(0.0)).sin();
                    // QB=1, no parasitic resistance or collector charge. TF
                    // stores charge at the ideal-clamped base; only forward
                    // collector transport is delayed. Reverse transport is
                    // instantaneous and BR=1 contributes the second term.
                    let expected = -polarity
                        * (1e-16 * (delayed_base / vt).exp_m1()
                            - 2e-16 * ((base - 2.0) / vt).exp_m1());
                    error = error.max((actual - expected).abs());
                }
                errors.push(error);
            }
            assert!(errors[1] < 1e-9, "{dialect:?}/{kind}: {errors:?}");
            assert!(
                errors[1] < errors[0] * 0.6,
                "refinement did not improve {dialect:?}/{kind}: {errors:?}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_phase_preserves_nominal_delay_at_temperature_with_area_and_multiplicity() {
    use rspice_core::GpTransientPhaseModel::{ExactDelay, NgspiceWeil};
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let k_over_q = match dialect {
            SpiceDialect::Ngspice => 1.380_648_52e-23 / 1.602_176_620_8e-19,
            SpiceDialect::Xyce => 1.380_622_6e-23 / 1.602_191_8e-19,
            SpiceDialect::BestAvailable => 1.380_649e-23 / 1.602_176_634e-19,
        };
        // TTF1 is a native ngspice extension; Xyce correctly rejects it.
        let tf_temperature = if dialect == SpiceDialect::Xyce {
            ""
        } else {
            "TTF1=.001"
        };
        for model in [ExactDelay, NgspiceWeil] {
            let reltol = if model == ExactDelay { 1e-6 } else { 1e-9 };
            for (temperature, area, multiplier) in [(-40.0, 0.2, 3.0), (125.0, 2.0, 0.5)] {
                for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
                    let source = Netlist::parse(&format!(
                        "Temperature and scaled phase\nVC c 0 {}\nVB b 0 SIN({} {} 1G)\nVE e 0 0\nQ1 c b e model AREA={area} M={multiplier}\n.model model {kind} IS=1e-16 EG=0 XTI=0 BF=100 BR=1 TF=1n {tf_temperature} PTF=57.29577951308232 TNOM=27\n.options TEMP={temperature} GMIN=0 RELTOL={reltol} ABSTOL=1e-18 VNTOL=1e-10\n.end\n",
                        polarity * 2.0, polarity * 0.6, polarity * 0.01,
                    )).unwrap();
                    let mut config = SimulationConfig {
                        gp_transient_phase_model: model,
                        // A discrete filter's oracle is grid-specific. Use a
                        // deterministic grid, as in the recorded ngspice tests,
                        // while the exact-delay oracle exercises adaptation.
                        locked_time_grid: (model == NgspiceWeil).then(|| {
                            let mut times: Vec<_> =
                                (0..=1092).map(|i| f64::from(i) * 4e-12).collect();
                            times.push(4.37e-9);
                            std::sync::Arc::new(times)
                        }),
                        ..SimulationConfig::default().with_spice_dialect(dialect)
                    };
                    // GMIN=0 disables junction leakage; this independent
                    // isolated-device oracle also excludes the distinct
                    // numerical nodal/continuation conditioning floor.
                    config.convergence_config.gmin_target = 0.0;
                    let engine = Engine::new(config);
                    let result = engine.run_tran(&source, 4.37e-9, 8e-12)
                        .unwrap_or_else(|error| panic!("{dialect:?}/{model:?}/{kind}/T={temperature}/AREA={area}/M={multiplier}: {error}"));
                    let collector = result.try_branch_current_waveform_named("vc").unwrap();
                    let base_current = result.try_branch_current_waveform_named("vb").unwrap();
                    let emitter = result.try_branch_current_waveform_named("ve").unwrap();
                    let vt = k_over_q * (temperature + 273.15);
                    let saturation = area * multiplier * 1e-16;
                    let forward = |time: f64| {
                        let base = 0.6 + 0.01 * (std::f64::consts::TAU * 1e9 * time).sin();
                        saturation * (base / vt).exp_m1()
                    };
                    let mut previous_time = 0.0;
                    let mut previous_step = 0.0;
                    let mut previous = forward(0.0);
                    let mut older = previous;
                    let mut error = 0.0_f64;
                    let mut peak = 0.0_f64;
                    let mut worst = (0.0, 0.0, 0.0);
                    let mut base_error = 0.0_f64;
                    let mut base_peak = 0.0_f64;
                    for (index, &time) in result.time.iter().enumerate() {
                        // EG=XTI=0 isolates the independently known thermal
                        // voltage and AREA*M current scaling. TTF1 changes
                        // diffusion charge, but the phase delay remains 1 ns.
                        let delayed = if model == ExactDelay {
                            forward((time - 1e-9).max(0.0))
                        } else if index == 0 {
                            previous
                        } else {
                            // ngspice 46 bjtload.c:593-612, evaluated from
                            // the observed accepted grid, without core helpers.
                            let step = time - previous_time;
                            let ratio = if previous_step == 0.0 {
                                1.0
                            } else {
                                step / previous_step
                            };
                            let a = step / 1e-9;
                            let next = (previous * (1.0 + ratio + 3.0 * a) - older * ratio
                                + forward(time) * 3.0 * a * a)
                                / (1.0 + 3.0 * a + 3.0 * a * a);
                            older = previous;
                            previous = next;
                            previous_time = time;
                            previous_step = step;
                            next
                        };
                        let base = 0.6 + 0.01 * (std::f64::consts::TAU * 1e9 * time).sin();
                        // GP uses a cubic reverse-junction continuation.
                        // ngspice bjttemp.c:168-201 also applies the default
                        // AREAB=AREA factor a second time to common-IS BC
                        // current; Xyce's common-IS law uses AREA*M once.
                        let reverse = if dialect == SpiceDialect::Xyce {
                            -saturation
                        } else {
                            -saturation
                                * area
                                * (1.0 + (3.0 * vt / ((base - 2.0) * std::f64::consts::E)).powi(3))
                        };
                        let expected = -polarity * (delayed - 2.0 * reverse);
                        if index != 0 {
                            let tf = if dialect == SpiceDialect::Xyce {
                                1e-9
                            } else {
                                1e-9 * (1.0 + 0.001 * (temperature - 27.0))
                            };
                            let rate = 0.01
                                * std::f64::consts::TAU
                                * 1e9
                                * (std::f64::consts::TAU * 1e9 * time).cos();
                            let expected_base = -polarity
                                * (forward(time) / 100.0
                                    + reverse
                                    + tf * saturation * (base / vt).exp() / vt * rate);
                            base_error =
                                base_error.max((base_current[index] - expected_base).abs());
                            base_peak = base_peak.max(expected_base.abs());
                        }
                        if (collector[index] - expected).abs() > error {
                            error = (collector[index] - expected).abs();
                            worst = (time, collector[index], expected);
                        }
                        peak = peak.max(expected.abs());
                        // Match the authored solver budget, including the
                        // startup charge-rate recovery at tiny lead currents.
                        assert!(
                            (collector[index] + base_current[index] + emitter[index]).abs()
                                < 1e-18
                                    + reltol
                                        * (collector[index].abs()
                                            + base_current[index].abs()
                                            + emitter[index].abs()),
                            "{dialect:?}/{model:?}/{kind}/T={temperature}/t={time:e}: terminal KCL {}, {}, {}",
                            collector[index],
                            base_current[index],
                            emitter[index]
                        );
                    }
                    let relative_bound = if model == ExactDelay { 5e-4 } else { 1e-9 };
                    assert!(
                        error < 1e-17 + peak * relative_bound,
                        "{dialect:?}/{model:?}/{kind}/T={temperature}/AREA={area}/M={multiplier}: error={error:e}, peak={peak:e}, worst={worst:?}"
                    );
                    // Includes first-order startup/restart intervals. A stale
                    // OneStep residual across hybrid Gear2 intervals previously
                    // introduced mA-scale ringing and eventual step collapse.
                    assert!(
                        base_error < 1e-15 + 0.02 * base_peak,
                        "{dialect:?}/{model:?}/{kind}/T={temperature}: base charge-rate error={base_error:e}, peak={base_peak:e}"
                    );
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_phase_cancellation_does_not_contaminate_a_later_run() {
    use rspice_core::abort_signal::CountingAbort;
    let source = deck("TF=1n PTF=21 RB=100 RBM=20 IRB=1e-5");
    for model in [
        rspice_core::GpTransientPhaseModel::ExactDelay,
        rspice_core::GpTransientPhaseModel::NgspiceWeil,
    ] {
        let engine = Engine::new(SimulationConfig {
            gp_transient_phase_model: model,
            ..SimulationConfig::default()
        });
        let baseline = engine.run_tran(&source, 2e-9, 4e-12).unwrap();
        let abort = CountingAbort::new(500);
        assert!(matches!(
            engine.run_tran_with_abort(&source, 2e-9, 4e-12, &abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(abort.observed_at(), Some(501));
        assert_eq!(abort.polls_after_abort(), 0);
        let after = engine.run_tran(&source, 2e-9, 4e-12).unwrap();
        assert_eq!(baseline.time, after.time);
        assert_eq!(baseline.voltages, after.voltages);
        assert_eq!(baseline.branch_currents, after.branch_currents);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_unresolvable_transport_arrival_is_not_coalesced_or_silently_removed() {
    let source = deck("TF=1n PTF=1e-200");
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice));
    let error = engine
        .run_tran(&source, 2e-9, 4e-12)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("phase arrival") && error.contains("minimum step"),
        "{error}"
    );
    // The Weil approximation carries a finite filter state, not a physical
    // delayed event. Its scaled coefficients also cover this zero-delay limit.
    let engine = Engine::new(SimulationConfig {
        gp_transient_phase_model: rspice_core::GpTransientPhaseModel::NgspiceWeil,
        ..SimulationConfig::default()
    });
    let result = engine.run_tran(&source, 2e-9, 4e-12).unwrap();
    assert_eq!(result.time.last(), Some(&2e-9));
    assert!(
        result
            .branch_currents
            .iter()
            .flatten()
            .all(|value| value.is_finite())
    );
}
