use rspice_core::abort_signal::CountingAbort;
use rspice_core::engine::SimulationError;
use rspice_core::{Engine, Netlist};

const DEVICES: [(&str, &str, &str, &str); 7] = [
    ("diode", "D(IS=VALUE)", "V1 out 0 .2\nD1 out 0 device", "1u"),
    (
        "bjt",
        "NPN(IS=VALUE)",
        "V1 out 0 1\nVb base 0 .5\nQ1 out base 0 device",
        "1p",
    ),
    (
        "mos",
        "NMOS(LEVEL=1 VTO=.4 KP=VALUE)",
        "V1 out 0 .1\nVg gate 0 1\nM1 out gate 0 0 device L=1u W=1u",
        "1m",
    ),
    (
        "jfet",
        "NJF(VTO=-2 BETA=VALUE)",
        "V1 out 0 1\nJ1 out 0 0 device",
        "1m",
    ),
    (
        "mesfet",
        "NMF(VTO=-2 BETA=VALUE)",
        "V1 out 0 1\nZ1 out 0 0 device",
        "1m",
    ),
    (
        "vbic",
        "NPN(LEVEL=11 IS=VALUE IBEI=1e-18 IBCI=1e-18 RCX=1)",
        "V1 out 0 1\nVb base 0 .5\nQ1 out base 0 device",
        "1p",
    ),
    (
        "bsim3",
        "NMOS(LEVEL=9 VERSION=3.3 VTH0=.4 U0=VALUE)",
        "V1 out 0 .1\nVg gate 0 1\nM1 out gate 0 0 device L=1u W=1u",
        "5",
    ),
];

#[test]
fn deferred_native_model_values_match_equivalent_numeric_cards() {
    for (family, model, circuit, value) in DEVICES {
        let current = |value: &str| {
            let source = format!(
                "* native model values\n{circuit}\n.MODEL device {}\n.END\n",
                model.replace("VALUE", value)
            );
            let netlist = Netlist::parse(&source).unwrap();
            Engine::default()
                .run_dc_op(&netlist)
                .unwrap()
                .branch_current_named("V1")
                .unwrap()
        };
        let expected = current(value);
        let actual = current(&format!("{{TEMP*0+{value}}}"));
        assert!(
            (actual - expected).abs() < expected.abs() * 1e-10 + 1e-15,
            "{family}: expected {expected}, got {actual}"
        );
    }
}

#[test]
fn invalid_deferred_native_values_cannot_silently_select_device_defaults() {
    for (family, model, circuit, _) in DEVICES {
        let source = format!(
            "* invalid native model value\n{circuit}\n.MODEL device {}\n.END\n",
            model.replace("VALUE", "{TEMP+missing}")
        );
        let netlist = Netlist::parse(&source).unwrap();
        let result = Engine::default().build_circuit(&netlist);
        assert!(
            result.is_err(),
            "{family}: invalid model expression was accepted"
        );
    }
}

#[test]
fn construction_expression_work_observes_the_callers_abort_signal() {
    let mut functions = String::from(".FUNC f0() {1}\n");
    for index in 1..=16 {
        functions.push_str(&format!(
            ".FUNC f{index}() {{f{}()+f{}()}}\n",
            index - 1,
            index - 1
        ));
    }
    for body in [
        ".MODEL rm R(RSH={TEMP+f16()})\nR2 out 0 rm L=1 W=1",
        ".MODEL cm C(CJ={TEMP+f16()})\nC1 out 0 cm L=1 W=1",
        ".MODEL dd D(IS={TEMP*0+f16()*1p})\nD1 out 0 dd",
    ] {
        let netlist = Netlist::parse(&format!(
            "* cancelled construction\n{functions}V1 out 0 .2\nR1 out 0 1k\n{body}\n.END\n"
        ))
        .unwrap();
        let abort = CountingAbort::new(256);
        let result = Engine::default().build_circuit_with_abort(&netlist, &abort);
        assert!(
            matches!(result, Err(SimulationError::Aborted)),
            "{body}: result {:?}, {} polls",
            result.as_ref().err(),
            abort.count()
        );
        assert_eq!(abort.polls_after_abort(), 0, "{body}");
    }
}

#[test]
fn native_model_values_follow_each_instances_temperature() {
    for (family, model, circuit, value) in DEVICES {
        for (override_card, temp) in [("TEMP=47", 47.0), ("DTEMP=10", 37.0)] {
            let circuit = circuit.replace(" device", &format!(" device {override_card}"));
            let run = |value: &str| {
                let source = format!(
                    "* instance model temperature\n{circuit}\n.MODEL device {}\n.END\n",
                    model.replace("VALUE", value)
                );
                let netlist = Netlist::parse(&source).unwrap();
                Engine::default()
                    .run_dc_op(&netlist)
                    .unwrap()
                    .branch_current_named("V1")
                    .unwrap()
            };
            let expected = run(&format!("{{{temp}*{value}}}"));
            let actual = run(&format!("{{TEMP*{value}}}"));
            assert!(
                (actual - expected).abs() < expected.abs() * 1e-10 + 1e-15,
                "{family} {override_card}: expected {expected}, got {actual}"
            );
        }
    }
}

#[test]
fn native_model_values_reject_non_real_and_non_finite_expressions() {
    for (_, model, circuit, _) in DEVICES {
        for expression in ["{TEMP*0+sqrt(-1)}", "{TEMP*0+1/0}"] {
            let netlist = Netlist::parse(&format!(
                "* invalid scalar model\n{circuit}\n.MODEL device {}\n.END\n",
                model.replace("VALUE", expression)
            ))
            .unwrap();
            assert!(
                Engine::default().build_circuit(&netlist).is_err(),
                "{model}: {expression}"
            );
        }
    }
}

#[test]
fn one_deferred_model_keeps_distinct_instance_temperatures() {
    let run = |model: &str, cards: &str| {
        let source =
            format!("* shared temperature model\nV1 a 0 .2\nV2 b 0 .2\n{cards}\n{model}\n.END\n");
        Engine::default()
            .run_dc_op(&Netlist::parse(&source).unwrap())
            .unwrap()
    };
    let expected = run(
        ".MODEL first D(IS=37u)\n.MODEL second D(IS=47u)",
        "D1 a 0 first TEMP=37\nD2 b 0 second TEMP=47",
    );
    let actual = run(
        ".MODEL shared D(IS={TEMP*1u})",
        "D1 a 0 shared TEMP=37\nD2 b 0 shared TEMP=47",
    );
    for source in ["V1", "V2"] {
        let expected = expected.branch_current_named(source).unwrap();
        let actual = actual.branch_current_named(source).unwrap();
        assert!(
            (actual - expected).abs() < expected.abs() * 1e-10 + 1e-15,
            "{source}: expected {expected}, got {actual}"
        );
    }
}

#[test]
fn native_model_expressions_use_the_model_nominal_temperature() {
    for (family, model, circuit, value) in DEVICES {
        let run = |expression: &str, nominal: &str| {
            let card = model
                .replace("VALUE", expression)
                .replace(')', &format!(" TNOM={nominal})"));
            let source =
                format!("* model nominal temperature\n{circuit}\n.MODEL device {card}\n.END\n");
            Engine::default()
                .run_dc_op(&Netlist::parse(&source).unwrap())
                .unwrap()
                .branch_current_named("V1")
                .unwrap()
        };
        let expected = run(&format!("{{57*{value}}}"), "57");
        for nominal in ["57", "{TEMP+30}"] {
            let actual = run(&format!("{{TNOM*{value}}}"), nominal);
            assert!(
                (actual - expected).abs() < expected.abs() * 1e-10 + 1e-15,
                "{family} {nominal}: expected {expected}, got {actual}"
            );
        }
    }
}
