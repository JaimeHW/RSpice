use rspice_core::{Engine, Netlist, SimulationConfig, SpiceDialect};

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
