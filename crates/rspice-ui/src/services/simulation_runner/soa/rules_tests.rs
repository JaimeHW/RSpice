//! Numerical and scope regressions for configurable terminal-stress rules.

use super::*;

fn rule(parameter: SoAParameter, limit: f64, devices: &[&str], models: &[&str]) -> SoaRuleConfig {
    SoaRuleConfig {
        voltage_basis: Default::default(),
        parameter,
        max_value: limit,
        devices: devices.iter().map(|name| (*name).to_owned()).collect(),
        models: models.iter().map(|name| (*name).to_owned()).collect(),
    }
}

fn config(rules: Vec<SoaRuleConfig>) -> SoaRunConfig {
    SoaRunConfig {
        stop_time: 1e-8,
        step_time: 1e-10,
        check_vgs_max: false,
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules,
        ..Default::default()
    }
}

#[test]
fn soa_additional_voltage_and_current_rules_retain_complete_physical_evidence() {
    let deck = "Scoped stress\nVd d 0 3\nVg g 0 2\nVc c 0 2\nVb b 0 0.6\nM1 d g 0 0 NM W=10u L=1u\nQ1 c b 0 NPN\n.model NM NMOS LEVEL=1 VTO=1 KP=1m LAMBDA=0\n.model NPN NPN IS=1e-14 BF=100\n.save V(d)\n.end\n";
    let data = run_soa_analysis_with_config_and_source_path_and_abort(
        deck,
        &config(vec![
            rule(SoAParameter::Vgd, 0.8, &[], &[]),
            rule(SoAParameter::Vbc, 1.0, &[], &[]),
            rule(SoAParameter::Id, 0.004, &[], &[]),
            rule(SoAParameter::Ic, 1e-6, &[], &[]),
        ]),
        None,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(data.evaluations.len(), 4);
    for (parameter, expected) in [
        (SoAParameter::Vgd, 1.0),
        (SoAParameter::Vbc, 1.4),
        (SoAParameter::Id, 0.005),
    ] {
        let result = data
            .evaluations
            .iter()
            .find(|entry| entry.parameter == parameter)
            .unwrap();
        assert!(
            (result.worst_actual_value - expected).abs() < 1e-8,
            "{result:?}"
        );
    }
    let collector = data
        .evaluations
        .iter()
        .find(|entry| entry.parameter == SoAParameter::Ic)
        .unwrap();
    assert!(
        collector.worst_actual_value > 1e-5 && collector.worst_actual_value < 1e-3,
        "{collector:?}"
    );
    assert!(
        data.evaluations
            .iter()
            .all(|entry| entry.sample_count == data.time.len() as u64)
    );
    for trace in &data.stress_history {
        assert_eq!(trace.values.len(), data.time.len());
        assert_eq!(
            trace.unit,
            if matches!(trace.parameter, SoAParameter::Id | SoAParameter::Ic) {
                "A"
            } else {
                "V"
            }
        );
    }
}

#[test]
fn soa_current_limits_measure_accepted_displacement_current_inside_subcircuits() {
    let deck = "SOA charge stress\nVd d 0 0\nVg g 0 PWL(0 -1 1n -1 6n -0.5 10n -0.5)\nX1 d g CELL\n.subckt CELL d g\nM1 d g 0 0 NM W=1u L=1u\n.model NM NMOS LEVEL=1 VTO=1 KP=1u CGDO=1u\n.ends CELL\n.save V(d)\n.end\n";
    let mut cfg = config(vec![rule(SoAParameter::Id, 50e-6, &["X1:M1"], &[])]);
    cfg.observation.models = vec!["CELL::NM".into()];
    let full =
        run_soa_analysis_with_config_and_source_path_and_abort(deck, &cfg, None, &NoAbort).unwrap();
    assert_eq!(full.evaluations.len(), 1);
    let stress = &full.evaluations[0];
    // Channel is off. The gate-drain overlap alone supplies about 100 uA
    // (1 pF * 0.5 V / 5 ns), which a DC-only current report would lose.
    assert!(
        stress.worst_actual_value > 90e-6 && stress.worst_actual_value < 110e-6,
        "{stress:?}"
    );
    assert!(full.stress_history[0].values[0] < 1e-10);
    let settled = run_soa_analysis_with_config_and_source_path_and_abort(
        deck,
        &SoaRunConfig {
            observation: SoaObservationConfig {
                start_time: 8e-9,
                ..Default::default()
            },
            ..cfg
        },
        None,
        &NoAbort,
    )
    .unwrap();
    assert!(
        settled.evaluations[0].worst_actual_value < 1e-8,
        "{:?}",
        settled.evaluations
    );
}

#[test]
fn soa_scoped_rules_override_defaults_without_ambiguous_precedence() {
    let deck = "SOA scopes\nVd d 0 1\nVg g 0 2\nM1 d g 0 0 NM1\nM2 d g 0 0 NM2\n.model NM1 NMOS LEVEL=1\n.model NM2 NMOS LEVEL=1\n.end\n";
    let cfg = SoaRunConfig {
        rules: vec![
            rule(SoAParameter::Vgs, 3.0, &["m1"], &[]),
            rule(SoAParameter::Vgs, 1.0, &[], &["nm2"]),
        ],
        stop_time: 1e-8,
        step_time: 1e-9,
        ..Default::default()
    };
    let result =
        run_soa_analysis_with_config_and_source_path_and_abort(deck, &cfg, None, &NoAbort).unwrap();
    assert_eq!(result.evaluations.len(), 4);
    for (name, limit) in [("M1", 3.0), ("M2", 1.0)] {
        assert_eq!(
            result
                .evaluations
                .iter()
                .find(|entry| entry.device_id == name && entry.parameter == SoAParameter::Vgs)
                .unwrap()
                .limit_value,
            limit
        );
    }
    let mut conflict = cfg.clone();
    conflict.rules.push(rule(SoAParameter::Vgs, 2.0, &[], &[]));
    assert!(
        run_soa_analysis_with_config_and_source_path_and_abort(deck, &conflict, None, &NoAbort)
            .unwrap_err()
            .to_string()
            .contains("overlapping")
    );
    for unmatched in [
        rule(SoAParameter::Ic, 1e-3, &["M1"], &[]),
        rule(SoAParameter::Id, 1e-3, &["typo"], &[]),
    ] {
        assert!(
            run_soa_analysis_with_config_and_source_path_and_abort(
                deck,
                &config(vec![unmatched]),
                None,
                &NoAbort
            )
            .is_err()
        );
    }
}

#[test]
fn soa_checks_grounded_devices_without_retaining_unrelated_voltage_columns() {
    let deck = "Grounded SOA device\nV1 unrelated 0 1\nR1 unrelated 0 1k\nM1 0 0 0 0 NM W=10u L=1u\n.model NM NMOS LEVEL=1 VTO=1 KP=1m\n.save V(unrelated)\n.end\n";
    let mut cfg = config(vec![
        rule(SoAParameter::Vgs, 1.0, &["M1"], &[]),
        rule(SoAParameter::Id, 1e-6, &["M1"], &[]),
    ]);
    cfg.observation.devices = vec!["M1".into()];
    cfg.observation.start_time = 5e-9;
    let data =
        run_soa_analysis_with_config_and_source_path_and_abort(deck, &cfg, None, &NoAbort).unwrap();
    assert!(!data.time.is_empty());
    assert!(data.time[0] >= 5e-9);
    assert!(data.violations.is_empty());
    assert_eq!(data.evaluations.len(), 2);
    assert!(data.convergence.is_some());
    for trace in &data.stress_history {
        assert_eq!(trace.values.len(), data.time.len());
        assert!(trace.values.iter().all(|value| value.abs() < 1e-12));
    }
    for evaluation in &data.evaluations {
        assert_eq!(evaluation.sample_count, data.time.len() as u64);
        assert_eq!(evaluation.device_id, "M1");
    }
}

#[test]
fn soa_directional_overrides_preserve_the_other_default_side_and_explicit_bounds() {
    let netlist = rspice_core::netlist::parse_netlist(
        "limits\nM1 d g 0 0 NM\nM2 d g 0 0 NM\n.model NM NMOS LEVEL=1\n.end\n",
    )
    .unwrap();
    let mut cfg = SoaRunConfig {
        check_vds_max: false,
        check_vbe_max: false,
        check_vce_max: false,
        rules: vec![rule(SoAParameter::VgsNegative, 0.0, &["M1"], &[])],
        ..Default::default()
    };
    let resolved = rules::resolve(
        &netlist.elements,
        &cfg,
        &Default::default(),
        &Default::default(),
        &NoAbort,
    )
    .unwrap();
    let limits = |name: &str, rows: &Vec<(usize, SoADefinition)>| {
        rows.iter()
            .find(|(index, _)| netlist.elements[*index].name == name)
            .unwrap()
            .1
            .limits
            .iter()
            .map(|l| (l.parameter, l.max_value))
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    assert_eq!(
        limits("M1", &resolved),
        std::collections::BTreeMap::from([
            (SoAParameter::VgsPositive, 1.8),
            (SoAParameter::VgsNegative, 0.0),
        ])
    );
    assert_eq!(
        limits("M2", &resolved),
        std::collections::BTreeMap::from([(SoAParameter::Vgs, 1.8)])
    );
    cfg.rules.push(rule(SoAParameter::Vgs, 3.0, &["M1"], &[]));
    let resolved = rules::resolve(
        &netlist.elements,
        &cfg,
        &Default::default(),
        &Default::default(),
        &NoAbort,
    )
    .unwrap();
    assert_eq!(
        limits("M1", &resolved),
        std::collections::BTreeMap::from([
            (SoAParameter::Vgs, 3.0),
            (SoAParameter::VgsNegative, 0.0),
        ])
    );
    cfg.rules
        .push(rule(SoAParameter::VgsNegative, 2.0, &[], &["NM"]));
    assert!(
        rules::resolve(
            &netlist.elements,
            &cfg,
            &Default::default(),
            &Default::default(),
            &NoAbort
        )
        .unwrap_err()
        .to_string()
        .contains("overlapping")
    );
}

#[test]
fn soa_terminal_current_probes_capture_advanced_mos_charging_current() {
    for model in ["LEVEL=49", "LEVEL=54 VERSION=4.8"] {
        let deck = format!(
            "Advanced MOS charge stress\nVd d 0 0\nVg g 0 PWL(0 -1 1n -1 6n -0.5 10n -0.5)\nX1 d g CELL\n.subckt CELL d g\nM1 d g 0 0 NM W=1u L=1u\n.model NM NMOS {model} CGDO=1u CGSO=0\n.ends CELL\n.end\n"
        );
        let cfg = config(vec![
            rule(SoAParameter::IdNegative, 50e-6, &["X1:M1"], &[]),
            rule(SoAParameter::IdPositive, 50e-6, &["X1:M1"], &[]),
            rule(SoAParameter::IgPositive, 50e-6, &["X1:M1"], &[]),
            rule(SoAParameter::Is, 50e-6, &["X1:M1"], &[]),
        ]);
        let data =
            run_soa_analysis_with_config_and_source_path_and_abort(&deck, &cfg, None, &NoAbort)
                .unwrap();
        for parameter in [SoAParameter::IdNegative, SoAParameter::IgPositive] {
            let evidence = data
                .evaluations
                .iter()
                .find(|e| e.parameter == parameter)
                .unwrap();
            assert!(
                evidence.worst_actual_value > 90e-6 && evidence.worst_actual_value < 120e-6,
                "{model}: {evidence:?}"
            );
            assert_eq!(evidence.sample_count, data.time.len() as u64);
            assert_eq!(evidence.unit, "A");
        }
        // The off channel must not masquerade as the 100 uA overlap current.
        let opposite = data
            .evaluations
            .iter()
            .find(|e| e.parameter == SoAParameter::IdPositive)
            .unwrap();
        assert!(opposite.worst_actual_value < 1e-7, "{model}: {opposite:?}");
    }
}

#[test]
fn soa_bulk_current_limits_measure_signed_body_junction_charging() {
    for (kind, sign) in [("NMOS", -1.0), ("PMOS", 1.0)] {
        let deck = format!(
            "Body charging\nVb b 0 PWL(0 0 1n 0 6n {} 10n {})\nM1 0 0 0 b MM W=1u L=1u\n.model MM {kind} LEVEL=1 VTO={} KP=0 CBD=1p CBS=0 MJ=0 IS=1e-30\n.end\n",
            sign * 0.5,
            sign * 0.5,
            -sign * 100.0
        );
        let direction = if sign < 0.0 {
            SoAParameter::IbulkNegative
        } else {
            SoAParameter::IbulkPositive
        };
        let cfg = config(vec![rule(direction, 50e-6, &["M1"], &[])]);
        let result =
            run_soa_analysis_with_config_and_source_path_and_abort(&deck, &cfg, None, &NoAbort)
                .unwrap();
        let evaluation = &result.evaluations[0];
        assert!(
            evaluation.worst_actual_value > 90e-6 && evaluation.worst_actual_value < 110e-6,
            "{kind}: {evaluation:?}"
        );
        assert_eq!(evaluation.unit, "A");
        assert_eq!(evaluation.sample_count, result.time.len() as u64);
        assert!(result.stress_history[0].values[0] < 1e-10);
        assert!(result.stress_history[0].values.last().unwrap() < &1e-8);
    }
}

#[test]
fn soa_body_rules_do_not_invent_floating_or_compact_device_pins() {
    for deck in [
        "Floating body\nVd d 0 1\nVg g 0 1\nM1 d g 0 0 MM W=4u L=1u\n.model MM NMOS LEVEL=56 CAPMOD=2\n.end\n",
        "Compact VDMOS\nVd d 0 1\nVg g 0 1\nM1 d g 0 MM\n.model MM VDMOS NCHAN VTO=2 KP=1\n.end\n",
    ] {
        let error = run_soa_analysis_with_config_and_source_path_and_abort(
            deck,
            &config(vec![rule(SoAParameter::Ibulk, 1.0, &["M1"], &[])]),
            None,
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("matches no applicable device"),
            "{error}"
        );
    }
}

#[test]
fn soa_diode_and_substrate_current_limits_capture_junction_charging() {
    for (element, model, parameter) in [
        (
            "D1 n 0 DM",
            ".model DM D IS=1e-30 CJO=1p M=0",
            SoAParameter::IaNegative,
        ),
        (
            "Q1 0 0 0 n QM",
            ".model QM NPN IS=1e-30 CJS=1p MJS=0",
            SoAParameter::IsubNegative,
        ),
    ] {
        let deck = format!(
            "Junction current\nV1 n 0 PWL(0 0 1n 0 6n -0.5 10n -0.5)\n{element}\n{model}\n.end\n"
        );
        let cfg = config(vec![rule(parameter, 50e-6, &[], &[])]);
        let result =
            run_soa_analysis_with_config_and_source_path_and_abort(&deck, &cfg, None, &NoAbort)
                .unwrap();
        let evaluation = &result.evaluations[0];
        assert!(
            evaluation.worst_actual_value > 90e-6 && evaluation.worst_actual_value < 110e-6,
            "{element}: {evaluation:?}"
        );
        assert_eq!(evaluation.unit, "A");
        assert_eq!(evaluation.sample_count, result.time.len() as u64);
        assert!(result.stress_history[0].values[0] < 1e-10);
        assert!(result.stress_history[0].values.last().unwrap() < &1e-8);
    }
}

#[test]
fn soa_substrate_rules_reject_thermal_or_implicit_bjt_terminals() {
    for (element, model) in [
        ("Q1 c b 0 0 QM", ".model QM NPN LEVEL=11 IS=1e-16 BF=100"),
        ("Q1 c b 0 QM", ".model QM NPN IS=1e-14 BF=100"),
    ] {
        let deck =
            format!("BJT terminal eligibility\nV1 c 0 1\nV2 b 0 0.6\n{element}\n{model}\n.end\n");
        let error = run_soa_analysis_with_config_and_source_path_and_abort(
            &deck,
            &config(vec![rule(SoAParameter::Isub, 1.0, &["Q1"], &[])]),
            None,
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("matches no applicable device"),
            "{error}"
        );
    }
}

#[test]
fn soa_intrinsic_scope_requires_every_applicable_device_to_supply_observation() {
    let netlist = rspice_core::Netlist::parse(
        "Intrinsic scope\nM1 d g 0 0 NM\nM2 d g 0 0 NM\n.model NM NMOS\n.end\n",
    )
    .unwrap();
    let elements = rspice_core::netlist::flatten_netlist_with_models(&netlist)
        .unwrap()
        .elements;
    let mut voltage = rule(SoAParameter::Vgs, 1.0, &[], &[]);
    voltage.voltage_basis = SoaVoltageBasis::IntrinsicNodes;
    let mut invalid = voltage.clone();
    invalid.parameter = SoAParameter::Id;
    assert!(
        invalid
            .validate()
            .unwrap_err()
            .contains("only to voltage rules")
    );
    let layouts = HashMap::from([(
        "M1".into(),
        terminals::TerminalLayout {
            intrinsic_voltages: 1u128 << SoAParameter::Vgs as u32,
            ..Default::default()
        },
    )]);
    let error = rules::resolve(
        &elements,
        &config(vec![voltage]),
        &layouts,
        &Default::default(),
        &NoAbort,
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("M2") && error.to_string().contains("intrinsic"),
        "{error}"
    );
}
