use super::*;
use rspice_core::CurrentImpulseOwner;
use rspice_core::engine::TransientStartupMode;

fn deck(polarity: f64) -> Netlist {
    Netlist::parse(&format!(
        "GP controlled event\nVC c 0 {}\nVREF ref 0 2\nVB ctrl 0 DC {} SIN({} {} 1G)\nE1 b 0 drive ref 2\nE2 drive ref ctrl b 1\nVGREF gr 0 1.25\nVG gp gr DC {polarity} SIN(0 {polarity} 1G 0 0 90)\nG1 b 0 gp gr 1u\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10\n.save v(b) i(vc) i(e1) i(g1) i(vb) i(vg) i(e2)\n.end\n",
        2.0*polarity, 1.05*polarity, 1.05*polarity, 1.5e-6*polarity,
        if polarity>0.0 { "NPN" } else { "PNP" },
    )).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_voltage_controlled_sources_preserve_transport_charge_currents_and_restart() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig {
            gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
            ..SimulationConfig::default().with_spice_dialect(dialect)
        };
        // Ideal input-port current excludes the separately configured nodal
        // conditioning shunt; .options GMIN controls the junction floor only.
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for polarity in [1.0, -1.0] {
            let source = deck(polarity);
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let result = engine
                    .run_tran_with_startup_mode(&source, 2.5e-9, 4e-12, startup)
                    .unwrap();
                let uic = startup == TransientStartupMode::Uic;
                behavioral::check(&result, dialect, polarity, Some("g1"), "e1", uic);
                for name in ["vb", "vg", "e2"] {
                    for (&time, &current) in result
                        .time
                        .iter()
                        .zip(result.try_branch_current_waveform_named(name).unwrap())
                    {
                        assert!(
                            current.abs() < 1e-16,
                            "{dialect:?}/{polarity}/{startup:?}: {name} at {time:e}: {current:e}"
                        );
                    }
                }
                for trace in result.current_impulses.as_ref().unwrap() {
                    let CurrentImpulseOwner::Branch { branch_name } = &trace.owner else {
                        continue;
                    };
                    if ["vb", "vg", "g1", "e2"]
                        .iter()
                        .any(|name| branch_name.eq_ignore_ascii_case(name))
                    {
                        assert!(
                            trace
                                .points
                                .iter()
                                .all(|point| point.charge_coulombs == 0.0),
                            "unexpected impulse at {branch_name}"
                        );
                    }
                    if uic && branch_name.eq_ignore_ascii_case("e1") {
                        let initial = trace.points.iter().find(|point| point.time == 0.0).unwrap();
                        let expected =
                            -polarity * TF * diode(0.7, thermal_voltage(dialect), dialect).0;
                        assert!(
                            (initial.charge_coulombs - expected).abs()
                                < 1e-25 + expected.abs() * 1e-10
                        );
                    }
                }
            }
            let (_, checkpoint) = engine
                .run_tran_checkpointed(&source, 1.2e-9, 4e-12)
                .unwrap();
            let (resumed, _) = engine
                .run_tran_resume(&source, &checkpoint, 2.5e-9, 4e-12)
                .unwrap();
            behavioral::check(&resumed, dialect, polarity, Some("g1"), "e1", false);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_voltage_controlled_step_preserves_charge_impulse_and_transport_arrival() {
    // Resolve the roughly 2 fC nonlinear jump with explicit CHGTOL below;
    // the default charge floor is larger than the entire measured impulse.
    let jump = 0.5e-9;
    let arrival = jump + DELAY;
    let at_or_after =
        |time: f64, event: f64| time >= event || (time - event).abs() <= 8.0 * f64::EPSILON * event;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.gp_transient_phase_model = GpTransientPhaseModel::ExactDelay;
        config.convergence_config.gmin_target = 0.0;
        let engine = Engine::new(config);
        for polarity in [1.0, -1.0] {
            let source=Netlist::parse(&format!(
                "controlled voltage jump\nVC c 0 {}\nVB ctrl 0 PWL(0 {} .5n {} .5n {} 2n {})\nE1 b 0 ctrl 0 2\nG1 b 0 ctrl 0 1u\nQ1 c b 0 qm\n.model qm {} IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232\n.options GMIN=0 RELTOL=1e-7 VNTOL=1e-10 ABSTOL=1e-16 CHGTOL=1e-26\n.save v(b) i(vc) i(e1) i(g1) i(vb)\n.end\n",
                2.0*polarity, 0.35*polarity, 0.35*polarity,0.3505*polarity,0.3505*polarity,if polarity>0.0 { "NPN" } else { "PNP" },
            )).unwrap();
            let result = engine.run_tran(&source, 2e-9, 4e-12).unwrap();
            let b = result.try_voltage_waveform_named("b").unwrap();
            let base = result.try_branch_current_waveform_named("e1").unwrap();
            let collector = result.try_branch_current_waveform_named("vc").unwrap();
            let extra = result.try_branch_current_waveform_named("g1").unwrap();
            let vt = thermal_voltage(dialect);
            for (index, &time) in result.time.iter().enumerate() {
                let voltage = if at_or_after(time, jump) { 0.701 } else { 0.7 };
                let delayed = if at_or_after(time, arrival) {
                    0.701
                } else {
                    0.7
                };
                let reverse = diode(voltage - 2.0, vt, dialect).0;
                let expected_collector = diode(delayed, vt, dialect).0 - 2.0 * reverse;
                let forcing = 0.5e-6 * voltage;
                let expected_base = diode(voltage, vt, dialect).0 / 100.0 + reverse + forcing;
                assert!((polarity * b[index] - voltage).abs() < 1e-10);
                assert!(
                    (-polarity * collector[index] - expected_collector).abs() < 2e-11,
                    "{dialect:?}/{polarity}: collector at {time:e}"
                );
                assert!(
                    (-polarity * base[index] - expected_base).abs() < 2e-11,
                    "{dialect:?}/{polarity}: base at {time:e}: {} vs {expected_base:e}",
                    -polarity * base[index]
                );
                assert!((polarity * extra[index] - forcing).abs() < 1e-16);
            }
            for clock in [jump, arrival] {
                assert!(
                    result
                        .time
                        .iter()
                        .any(|&time| (time - clock).abs() <= 8.0 * f64::EPSILON * clock)
                );
            }
            let traces = result.current_impulses.as_ref().unwrap();
            assert!(traces.iter().all(|trace| trace.complete));
            let base_trace=traces.iter().find(|trace| matches!(&trace.owner,CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case("e1"))).unwrap();
            let impulse = base_trace
                .points
                .iter()
                .find(|point| point.time == jump)
                .unwrap()
                .charge_coulombs;
            let expected =
                -polarity * TF * (diode(0.701, vt, dialect).0 - diode(0.7, vt, dialect).0);
            assert!(
                (impulse - expected).abs() < 1e-25 + expected.abs() * 1e-9,
                "{dialect:?}/{polarity}: impulse {impulse:e}, expected {expected:e}, difference {:e}",
                impulse - expected
            );
            for name in ["vb", "g1"] {
                let trace=traces.iter().find(|trace| matches!(&trace.owner,CurrentImpulseOwner::Branch {branch_name} if branch_name.eq_ignore_ascii_case(name))).unwrap();
                assert!(
                    trace
                        .points
                        .iter()
                        .all(|point| point.charge_coulombs == 0.0)
                );
            }
        }
    }
}
