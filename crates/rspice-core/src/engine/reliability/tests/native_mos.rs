use super::*;
use crate::analysis::reliability::AgingParameterScale;
use crate::engine::{SimulationConfig, SpiceDialect};

fn aging(parameters: &[(&str, AgingParameterUpdate, f64)]) -> ReliabilityRunRequest {
    let mut r = request();
    r.study.mission.truncate(1);
    r.target_years = vec![1.0 / SECONDS_PER_AGING_YEAR];
    r.study.bindings[0].device = "X1.M1".into();
    let AgingLaw::EquivalentTimePower {
        clock_gate_exponent,
        parameters: entries,
        ..
    } = &mut r.study.model_pack.models[0].law
    else {
        unreachable!()
    };
    *clock_gate_exponent = 0.0;
    *entries = parameters
        .iter()
        .map(|(name, update, shift)| AgingParameterScale {
            parameter: (*name).into(),
            update: *update,
            scale_at_reference_time: *shift,
        })
        .collect();
    r
}

fn deck(level: u32, parameters: &str, instance: &str) -> String {
    format!(
        "Native MOS aging\n.param SUP=1\nVS s 0 {{SUP}}\nVG g 0 0\nVD d 0 .2\nX1 d g s cell\nM2 d g s s PM W=20u L=2u\n.subckt cell d g s\nM1 d g s s PM W=10u L=1u {instance}\n.ends\n.model PM.1 PMOS (LEVEL={level} LMIN=.5u LMAX=3u VTHO=-.4 VTH0=-.2 U0=.05 LU0=.002 WU0=.003 PU0=.004 VSAT=80000 RDSW=100 {parameters})\n.end\n"
    )
}

fn compare_points(actual: &ReliabilityOperatingPoint, expected: &crate::solver::SimulationResult) {
    for (key, value) in &actual.device_observables {
        let expected = expected.try_dc_observable_named(key).unwrap();
        assert!(
            (value - expected).abs() <= 5e-10 * expected.abs().max(1e-14),
            "{key}: {value} != {expected}"
        );
    }
    for (name, value) in &actual.voltages {
        close(*value, expected.try_voltage_named(name).unwrap(), 2e-12);
    }
}

fn explicit_aged(deck: &str, replacements: &[(&str, String)]) -> Netlist {
    let original = deck
        .lines()
        .find(|line| line.starts_with(".model"))
        .unwrap();
    let mut card = original.replace("PM.1", "AGED");
    for (old, new) in replacements {
        card = card.replace(old, new);
    }
    let deck = deck
        .replace("M1 d g s s PM", "M1 d g s s AGED")
        .replace(".end\n", &format!("{card}\n.end\n"));
    Netlist::parse(&deck).unwrap()
}

#[test]
fn native_bsim_aging_matches_explicit_cards_bins_and_circuit_temperature() {
    for (level, dialect, external) in [
        (8, SpiceDialect::Ngspice, false),
        (49, SpiceDialect::Ngspice, false),
        (9, SpiceDialect::Xyce, false),
        (49, SpiceDialect::Xyce, false),
        (14, SpiceDialect::Ngspice, false),
        (54, SpiceDialect::Ngspice, true),
    ] {
        let suffix = if external {
            "RDSMOD=1 RDW=120 RSW=80 RDWMIN=5 RSWMIN=6"
        } else {
            ""
        };
        let instance = if level == 14 || level == 54 {
            "DTEMP=40 NF=2 MULU0=.8 DELVTO=.01"
        } else {
            "DTEMP=40 MULU0=.8 DELVTO=.01"
        };
        let version = if level == 14 || level == 54 {
            "4.8.3"
        } else if dialect == SpiceDialect::Xyce {
            "3.2.2"
        } else {
            "3.3.0"
        };
        let source = deck(level, &format!("{suffix} VERSION=\"{version}\""), instance);
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        let mut params = vec![
            ("VTH0", AgingParameterUpdate::Additive, -0.03),
            ("U0", AgingParameterUpdate::Relative, -0.2),
            ("VSAT", AgingParameterUpdate::Relative, -0.1),
        ];
        let mut replacements = vec![
            ("VTH0=-.2", "VTH0=-.23".into()),
            ("U0=.05", "U0=.04".into()),
            ("VSAT=80000", "VSAT=72000".into()),
        ];
        if external {
            params.extend([
                ("RDW", AgingParameterUpdate::Relative, 0.2),
                ("RSW", AgingParameterUpdate::Relative, 0.25),
                ("RDWMIN", AgingParameterUpdate::Additive, 1.0),
                ("RSWMIN", AgingParameterUpdate::Additive, 2.0),
            ]);
            replacements.extend([
                ("RDW=120", "RDW=144".into()),
                ("RSW=80", "RSW=100".into()),
                ("RDWMIN=5", "RDWMIN=6".into()),
                ("RSWMIN=6", "RSWMIN=8".into()),
            ]);
        } else {
            params.push(("RDSW", AgingParameterUpdate::Relative, 0.2));
            replacements.push(("RDSW=100", "RDSW=120".into()));
        }
        let r = aging(&params);
        let result = engine
            .run_reliability_with_abort(&Netlist::parse(&source).unwrap(), &r, &NoAbort)
            .unwrap_or_else(|e| panic!("{level} {dialect:?}: {e}"));
        close(
            result.stress.phases[0].devices[0].samples[0].temperature_k,
            300.15,
            1e-12,
        );
        let vth = result.aged[0]
            .parameters
            .iter()
            .find(|p| p.parameter == "VTH0")
            .unwrap();
        assert_eq!(vth.fresh_value, -0.2, "VTH0 precedes alias VTHO");
        assert_eq!(vth.compact_model, "PM.1");
        let expected = engine
            .run_dc_op_with_abort(&explicit_aged(&source, &replacements), &NoAbort)
            .unwrap();
        compare_points(&result.aged[0].operating_point, &expected);
        result
            .validate_retained_payload_with_abort(&Default::default(), &NoAbort)
            .unwrap();
        let limits = Default::default();
        let (meta, buffer) = result
            .clone()
            .into_transfer_parts_with_abort(&limits, &NoAbort)
            .unwrap();
        assert_eq!(
            ReliabilityRunResult::from_transfer_parts_with_abort(meta, buffer, &limits, &NoAbort)
                .unwrap(),
            result
        );
    }
}

#[test]
fn native_bsim_aging_keeps_fresh_mobility_units_across_the_inference_boundary() {
    for level in [8, 54] {
        for (fresh, aged, encoded, factor) in [(2.0, 0.2, 0.00002, 1e-4), (0.2, 2.2, 22000.0, 1e4)]
        {
            let source = deck(level, "", if level == 54 { "NF=2" } else { "" })
                .replace("U0=.05", &format!("U0={fresh}"));
            let r = aging(&[("U0", AgingParameterUpdate::Additive, aged - fresh)]);
            let engine =
                Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice));
            let result = engine
                .run_reliability_with_abort(&Netlist::parse(&source).unwrap(), &r, &NoAbort)
                .unwrap();
            let replacements = [
                (format!("U0={fresh}"), format!("U0={encoded}")),
                ("LU0=.002".into(), format!("LU0={}", 0.002 * factor)),
                ("WU0=.003".into(), format!("WU0={}", 0.003 * factor)),
                ("PU0=.004".into(), format!("PU0={}", 0.004 * factor)),
            ];
            let replacements: Vec<_> = replacements
                .iter()
                .map(|(a, b)| (a.as_str(), b.clone()))
                .collect();
            let expected = engine
                .run_dc_op_with_abort(&explicit_aged(&source, &replacements), &NoAbort)
                .unwrap();
            compare_points(&result.aged[0].operating_point, &expected);
            close(result.aged[0].parameters[0].fresh_value, fresh, 1e-15);
            close(result.aged[0].parameters[0].aged_value, aged, 1e-15);
        }
    }
}

#[test]
fn native_bsim_aging_rejects_inactive_resistance_coordinates_and_invalid_binned_values() {
    let engine = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice));
    for (suffix, parameter) in [
        ("RDSMOD=1 RDW=120 RSW=80", "RDSW"),
        ("RDSMOD=0 RDW=120", "RDW"),
    ] {
        let circuit = Netlist::parse(&deck(54, suffix, "")).unwrap();
        assert!(
            engine
                .run_reliability_with_abort(
                    &circuit,
                    &aging(&[(parameter, AgingParameterUpdate::Relative, 0.1)]),
                    &NoAbort
                )
                .unwrap_err()
                .to_string()
                .contains("no qualified parameter mapping")
        );
    }
    let circuit = Netlist::parse(&deck(8, "", "")).unwrap();
    assert!(
        engine
            .run_reliability_with_abort(
                &circuit,
                &aging(&[("U0", AgingParameterUpdate::Additive, -1.0)]),
                &NoAbort
            )
            .unwrap_err()
            .to_string()
            .contains("outside its native domain")
    );
    let circuit = Netlist::parse(&deck(54, "RDSWMIN=1 LRDSWMIN=100", "")).unwrap();
    assert!(
        engine
            .run_reliability_with_abort(
                &circuit,
                &aging(&[("RDSWMIN", AgingParameterUpdate::Additive, -2.0)]),
                &NoAbort
            )
            .unwrap_err()
            .to_string()
            .contains("outside its native domain")
    );
    // Ngspice LEVEL=9 is MOS9, not the Xyce BSIM3 family.
    let circuit = Netlist::parse(&deck(9, "", "")).unwrap();
    assert!(
        engine
            .run_reliability_with_abort(
                &circuit,
                &aging(&[("VTH0", AgingParameterUpdate::Additive, -0.01)]),
                &NoAbort
            )
            .is_err()
    );
}
