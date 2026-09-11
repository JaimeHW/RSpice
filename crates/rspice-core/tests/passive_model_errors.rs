use rspice_core::{Engine, Netlist, SimulationConfig, SpiceDialect};

#[test]
fn negative_explicit_capacitance_cannot_select_a_model_default() {
    for element in ["C1 in 0 -1n", "C1 in 0 -1n CM", "C1 in 0 CM C=-1n"] {
        let source =
            format!("Invalid capacitance\nV1 in 0 1\n{element}\n.model CM C(C=1n)\n.end\n");
        let netlist = Netlist::parse(&source).unwrap();
        let error = Engine::default()
            .build_circuit(&netlist)
            .expect_err("negative capacitance");
        assert!(
            matches!(error, rspice_core::SimulationError::ParameterDomain(_)),
            "{element}: {error}"
        );
    }
}

#[test]
fn nonpositive_explicit_inductance_cannot_select_a_model_default() {
    for element in ["L1 in 0 0 LM", "L1 in 0 -1m LM", "L1 in 0 LM L=-1m"] {
        let source = format!("Invalid inductance\nV1 in 0 1\n{element}\n.model LM L(L=1m)\n.end\n");
        let netlist = Netlist::parse(&source).unwrap();
        let error = Engine::default()
            .build_circuit(&netlist)
            .err()
            .unwrap_or_else(|| panic!("{element}: invalid explicit value selected a fallback"));
        assert!(
            matches!(error, rspice_core::SimulationError::ParameterDomain(_)),
            "{element}: {error}"
        );
    }
}

#[test]
fn infinite_explicit_reactive_values_cannot_select_model_defaults() {
    for (prefix, model) in [("C", "C(C=1n)"), ("L", "L(L=1m)")] {
        for invalid in [f64::INFINITY, f64::NEG_INFINITY] {
            let mut netlist = Netlist::parse(&format!(
                "SDK value\nV1 in 0 1\n{prefix}1 in 0 MOD\n.model MOD {model}\n.end\n"
            ))
            .unwrap();
            let element = netlist
                .elements
                .iter_mut()
                .find(|element| element.name == format!("{prefix}1"))
                .unwrap();
            match &mut element.kind {
                rspice_core::netlist::ElementKind::Capacitor { value, .. }
                | rspice_core::netlist::ElementKind::Inductor { value, .. } => *value = invalid,
                _ => unreachable!(),
            }
            let error = Engine::default()
                .build_circuit(&netlist)
                .err()
                .unwrap_or_else(|| {
                    panic!("{prefix}1={invalid}: infinity selected a model fallback")
                });
            assert!(
                matches!(error, rspice_core::SimulationError::Circuit(_)),
                "arithmetic failure must not establish a parameter boundary: {error}"
            );
        }
    }
}

#[test]
fn reactive_scaling_range_failures_do_not_establish_parameter_domains() {
    for element in ["C1 in 0 1e308 SCALE=2", "L1 in 0 1e-200 SCALE=1e-200"] {
        let netlist =
            Netlist::parse(&format!("Scaling range\nV1 in 0 1\n{element}\n.end\n")).unwrap();
        let error = Engine::default()
            .build_circuit(&netlist)
            .expect_err("unrepresentable effective value");
        assert!(
            matches!(error, rspice_core::SimulationError::Circuit(_)),
            "{element}: {error}"
        );
    }
}

#[test]
fn invalid_model_expressions_cannot_silently_select_passive_defaults() {
    let models = [
        (
            "capacitor width",
            "C1 in 0 CM L=1u",
            ".model CM C(CJ=1 DEFW={MISSING})",
        ),
        (
            "capacitor length",
            "C1 in 0 CM W=1u",
            ".model CM C(CJ=1 DEFL={MISSING})",
        ),
        (
            "capacitor TC1",
            "C1 in 0 CM",
            ".model CM C(C=1n TC1={MISSING})",
        ),
        (
            "capacitor TC2",
            "C1 in 0 CM",
            ".model CM C(C=1n TC2={MISSING})",
        ),
        (
            "resistor width",
            "R1 in 0 RM L=1u",
            ".model RM R(RSH=100 DEFW={MISSING})",
        ),
        (
            "resistor noise width",
            "R1 in 0 1k RM",
            ".model RM R(KF=1e-10 DEFW={MISSING})",
        ),
    ];
    for (name, element, model) in models {
        let source = format!("{name}\nV1 in 0 1\n{element}\n{model}\n.temp 80\n.end\n");
        let netlist = Netlist::parse(&source).expect("model expression parses");
        let error = Engine::default()
            .build_circuit(&netlist)
            .err()
            .unwrap_or_else(|| panic!("{name}: invalid model expression was silently accepted"))
            .to_string();
        assert!(
            error.contains("MISSING"),
            "{name}: lost the expression error: {error}"
        );
    }
}

#[test]
fn explicit_instance_values_do_not_evaluate_unused_model_fallbacks() {
    for (element, model) in [
        (
            "C1 in 0 CM TC1=0 TC2=0",
            ".model CM C(C=1n TC1={MISSING} TC2={MISSING})",
        ),
        (
            "C1 in 0 CM W=1u L=2u",
            ".model CM C(CJ=1 DEFW={MISSING} DEFL={MISSING})",
        ),
        (
            "R1 in 0 RM W=1u L=2u",
            ".model RM R(RSH=100 DEFW={MISSING})",
        ),
    ] {
        let netlist =
            Netlist::parse(&format!("override\nV1 in 0 1\n{element}\n{model}\n.end\n")).unwrap();
        Engine::default()
            .build_circuit(&netlist)
            .unwrap_or_else(|error| panic!("{element}: unused fallback evaluated: {error}"));
    }
}

#[test]
fn xyce_capacitor_default_width_preserves_expression_errors_and_instance_precedence() {
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..SimulationConfig::default()
    });
    for (width, should_build) in [("", false), ("W=2u", true)] {
        let netlist = Netlist::parse(&format!(
            "Xyce default width\nV1 in 0 1\nC1 in 0 CM L=1u {width}\n.model CM C(CJ=1 DEFW={{MISSING}})\n.end\n"
        )).unwrap();
        match engine.build_circuit(&netlist) {
            Ok(_) => assert!(
                should_build,
                "invalid DEFW expression was silently accepted"
            ),
            Err(error) => {
                assert!(!should_build, "explicit width must override DEFW: {error}");
                assert!(error.to_string().contains("MISSING"), "{error}");
            }
        }
    }
}
