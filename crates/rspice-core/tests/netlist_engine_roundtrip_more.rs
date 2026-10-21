//! Netlist checks whose evidence is the circuit the engine builds or solves
//! from the parse.
//!
//! The parser-side assertions stay in the netlist module's own unit tests.
//! The half that needs `Engine` lives here, outside the crate, so `netlist`
//! never reaches up to the engine to check itself.

use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{
    NetlistParseOptions, RandomState, StatisticalParamMode, source_map_for_editor,
};
use rspice_core::{Engine, Netlist, SimulationConfig, Value};

fn xyce_options() -> NetlistParseOptions {
    NetlistParseOptions {
        expression_dialect: ExpressionDialect::Xyce,
        ..Default::default()
    }
}

fn parse_xyce(source: &str) -> Netlist {
    Netlist::parse_with_options(source, xyce_options()).expect("Xyce fixture parses")
}

/// The authored deck builds without the `.PREPROCESS ADDRESISTORS` resistors,
/// and its materialized copy builds with both of them.
#[test]
fn add_resistors_copy_builds_with_its_generated_resistors() {
    let netlist = parse_xyce(
        "RC connectivity\n\
         .PARAM RH=2k\n\
         .PREPROCESS ADDRESISTORS NODCPATH {RH*2}\n\
         .PREPROCESS ADDRESISTORS ONETERMINAL 1meg\n\
         R1 1 2 1k\n\
         C1 2 0 1u\n\
         .END\n",
    );
    let original_circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("first-run circuit builds");
    assert_eq!(original_circuit.device_count(), 2);

    let materialized = netlist
        .materialize_xyce_add_resistors()
        .expect("derived copy materializes");
    let generated_circuit = Engine::default()
        .build_circuit(&materialized.netlist)
        .expect("derived circuit builds");
    assert_eq!(generated_circuit.device_count(), 4);
}

/// A materialized zero, negative or open resistor still builds.
#[test]
fn materialized_zero_negative_and_open_resistors_build() {
    for (raw, expected) in [("0", 0.0), ("-1", -1.0), ("1e999", Value::INFINITY)] {
        let source = format!(
            "finite resistor\n.PREPROCESS ADDRESISTORS ONETERMINAL {raw}\nV1 1 0 1\n.END\n"
        );
        let materialized = parse_xyce(&source)
            .materialize_xyce_add_resistors()
            .expect("canonical resistor value is retained");
        assert_eq!(materialized.report.generated[0].resistance, expected);
        Engine::default()
            .build_circuit(&materialized.netlist)
            .expect("materialized zero, negative, and open resistors build safely");
    }
}

#[test]
fn nested_instance_arguments_preserve_sequential_random_bindings() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for mode in [StatisticalParamMode::Sample, StatisticalParamMode::Nominal] {
            let make = |nested| {
                let instances = if nested {
                    "Xtop out 0 wrapper\n.subckt wrapper a b z=999\nX1 a b cell z={aunif(2,0.25)} next={z+1}\nX2 a b cell z={aunif(3,0.5)} next={z+1}\n.ends"
                } else {
                    "X1 out 0 cell z={aunif(2,0.25)} next={z+1}\nX2 out 0 cell z={aunif(3,0.5)} next={z+1}"
                };
                Netlist::parse_with_options(&format!(
                    "Sequential argument draws\n{instances}\n.subckt cell a b z=99 next=99\n.param sampled={{aunif(5,0.5)}}\nR1 a b {{next}}\nR2 a b {{sampled}}\n.ends\n.end"
                ), NetlistParseOptions {
                    expression_dialect: dialect,
                    statistical_mode: mode,
                    statistical_seed: Some(793),
                    ..Default::default()
                }).unwrap()
            };
            let direct = make(false);
            let nested = make(true);
            let engine = Engine::default();
            let direct_circuit = engine.build_circuit(&direct).unwrap();
            let next_draw = direct.params.random().next_uniform();
            let nested_circuit = engine.build_circuit(&nested).unwrap();
            let nested_resistances = &nested_circuit.resistor_storage().reported_resistances;
            assert_eq!(
                nested_resistances,
                &direct_circuit.resistor_storage().reported_resistances,
                "{dialect:?} {mode:?}"
            );
            assert_eq!(nested.params.random().next_uniform(), next_draw);
            let expected_stream = RandomState::new(793);
            let sample = |mean, spread| match mode {
                StatisticalParamMode::Sample => mean + spread * expected_stream.next_symmetric(),
                StatisticalParamMode::Nominal => mean,
            };
            let expected = [
                sample(2.0, 0.25) + 1.0,
                sample(5.0, 0.5),
                sample(3.0, 0.5) + 1.0,
                sample(5.0, 0.5),
            ];
            assert_eq!(*nested_resistances, expected);
            assert_eq!(next_draw, expected_stream.next_uniform());
            assert!((2.75..=3.25).contains(&nested_resistances[0]));
            assert!((3.5..=4.5).contains(&nested_resistances[2]));
        }
    }
}

/// `.PREPROCESS REMOVEUNUSED` drops a subcircuit resistor whose resolved
/// terminals are one net, so the built circuit never creates that net.
#[test]
fn post_flatten_filter_omits_the_resolved_subcircuit_resistor_from_the_circuit() {
    let netlist = Netlist::parse_with_options(
        "resolved actual equality\n\
         .PREPROCESS REMOVEUNUSED R\n\
         .SUBCKT CELL a b\n\
         R_BODY a b 1\n\
         .ENDS CELL\n\
         X1 n n CELL\n\
         .END\n",
        xyce_options(),
    )
    .expect("subcircuit fixture parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("circuit build omits the newly redundant flattened resistor");
    assert!(circuit.get_node_by_name("n").is_none());
}

/// The same filter drops a passive and every helper its parasitics
/// synthesized as one batch, so none of them creates the shorted net.
#[test]
fn post_flatten_filter_omits_the_resolved_passive_batch_from_the_circuit() {
    let netlist = Netlist::parse_with_options(
        "resolved parasitic batch equality\n\
         .PREPROCESS REMOVEUNUSED R\n\
         .SUBCKT CELL a b\n\
         R_BODY a b 1 RSER=2 RPAR=3 CPAR=4\n\
         .ENDS CELL\n\
         X1 n n CELL\n\
         .END\n",
        xyce_options(),
    )
    .expect("passive parasitic subcircuit fixture parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("circuit build omits the rejected passive batch");
    assert!(circuit.get_node_by_name("n").is_none());
}

/// An instance argument written as an expression reaches the controlled
/// source inside the subcircuit with its complete value, whether the cell is
/// instantiated directly or through a wrapper that rebinds `p`.
#[test]
fn instance_argument_expressions_reach_the_circuit() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        for (expression, combined) in [
            ("2^p", 8.0),
            ("5%p", 2.0),
            ("1<p", 1.0),
            ("3==p", 1.0),
            ("0||p", 1.0),
            ("1j", 1.0),
        ] {
            for nested in [false, true] {
                let instance = if nested {
                    format!(
                        "Xtop in out wrapper p=3\n.subckt wrapper in out p=9\nXleaf in out cell q={expression}\n.ends"
                    )
                } else {
                    format!("Xtop in out cell q={expression}")
                };
                let netlist = Netlist::parse_with_options(
                    &format!("Parameter field expressions\n.param p=3\nV1 in 0 1\n{instance}\n.subckt cell in out q=99\nE1 out 0 in 0 {{real(q)+img(q)}}\n.ends\n.end"),
                    NetlistParseOptions { expression_dialect: dialect, ..Default::default() },
                ).unwrap();
                let result = Engine::default().run_dc_op(&netlist).unwrap();
                assert_eq!(
                    result.voltage(2),
                    combined,
                    "{dialect:?} nested={nested}: {expression}"
                );
            }
        }
    }
}

/// The editor's parameter scope inside a subcircuit gives the values the
/// engine builds that subcircuit's resistors from.
#[test]
fn editor_and_engine_agree_on_subcircuit_parameter_scope() {
    let src = "deck\n\
               .param g=1\n\
               .subckt cell in out params: g=5\n\
               .param k={g*2}\n\
               R1 in out {k}\n\
               R2 in out {t}\n\
               .param k={g*3}\n\
               .ends\n\
               .param t={g*2}\n\
               V1 n 0 1\n\
               X1 n 0 cell\n\
               .end\n";

    let map = source_map_for_editor(src);
    let netlist = Netlist::parse(src).expect("deck parses");
    let circuit = Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("deck builds");
    let resistors = circuit.resistor_storage();
    for (resistor, local) in [("X1.R1", "k"), ("X1.R2", "t")] {
        let index = resistors
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(resistor))
            .expect("resistor exists");
        let engine = resistors.conductances[index].recip();
        let editor = map.param_context_in(Some("cell")).get(local).unwrap();
        assert!(
            (engine / editor - 1.0).abs() < 1e-12,
            "the editor ({editor}) and the engine ({engine}) disagree on {local} inside the subcircuit"
        );
    }
}

#[test]
fn circuit_build_revalidates_programmatically_changed_sidecar_state() {
    let mut netlist = Netlist::parse(
        "build validation\n\
         V1 1 0 1\n\
         .IC V(1)=1\n\
         .NODESET V(1)=2\n\
         .OP\n\
         .END\n",
    )
    .expect("ngspice parse permits both modes");
    netlist
        .params
        .set_expression_dialect(ExpressionDialect::Xyce);
    let error = Engine::default()
        .build_circuit(&netlist)
        .expect_err("builder enforces Xyce startup conflict");
    assert!(
        error
            .to_string()
            .contains("Cannot set both .IC and .NODESET simultaneously")
    );
}
