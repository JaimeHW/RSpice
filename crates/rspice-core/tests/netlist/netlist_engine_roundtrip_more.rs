//! Netlist checks whose evidence is the circuit the engine builds or solves
//! from the parse.
//!
//! The parser-side assertions stay in the netlist module's own unit tests.
//! The half that needs `Engine` lives here, outside the crate, so `netlist`
//! never reaches up to the engine to check itself.

use rspice_core::circuit::VoltageSources;
use rspice_core::config::{ExpressionDialect, SpiceDialect};
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

/// Complex instance bindings reach every subcircuit scope, and the built
/// resistors and source carry the physical (real) value each binding
/// evaluates to, whether the cell is instantiated directly, through a
/// wrapper, or with its defaults.
#[test]
fn subcircuit_bindings_preserve_complex_scopes_and_physical_values() {
    for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
        let netlist = Netlist::parse_with_options(
            "Complex instance bindings
.param z={2+3j}
Xdirect d 0 cell z={z+1j} next={z+1j}
Xnested n 0 wrapper z={z}
Xdefault f 0 cell
.subckt wrapper a b z=100
.param local={z+2j}
Xchild a b cell z={local} next={z+1j}
.ends
.subckt cell a b z={1+2j} next={z+1j}
.param squared={z*z}
.func part(x) {img(x)}
R1 a b {1+part(z)}
R2 a b {1+img(squared)}
R3 a b 1 M={1+img(next)}
V1 inside b DC {1+img(next)} AC {1+img(next)}
.ends
.end",
            NetlistParseOptions {
                expression_dialect: dialect,
                ..Default::default()
            },
        )
        .unwrap();
        let circuit = Engine::default().build_circuit(&netlist).unwrap();
        for (scope, r1, r2, magnitude) in [
            ("Xdirect", 5.0, 17.0, 6.0),
            ("Xnested.Xchild", 6.0, 21.0, 7.0),
            ("Xdefault", 3.0, 5.0, 4.0),
        ] {
            for (name, conductance) in [("R1", 1.0 / r1), ("R2", 1.0 / r2), ("R3", magnitude)] {
                let name = format!("{scope}.{name}");
                let index = circuit
                    .resistor_storage()
                    .names
                    .iter()
                    .position(|actual| actual.eq_ignore_ascii_case(&name))
                    .unwrap();
                assert_eq!(
                    circuit.resistor_storage().conductances[index],
                    conductance,
                    "{dialect:?} {name}"
                );
            }
            let name = format!("{scope}.V1");
            let index = circuit
                .voltage_source_storage()
                .names
                .iter()
                .position(|actual| actual.eq_ignore_ascii_case(&name))
                .unwrap();
            assert_eq!(
                circuit.voltage_source_storage().dc_values[index],
                magnitude,
                "{dialect:?} {name}"
            );
            assert_eq!(
                circuit.voltage_source_storage().ac_magnitudes[index],
                magnitude,
                "{dialect:?} {name}"
            );
        }
    }
}

/// A Spectre statistical parameter reaches every built resistor and source
/// that reads it, through user functions, aliases, root redefinitions and
/// shadowing subcircuit parameters, with process and mismatch variation and
/// with enough user functions to change how the deck is expanded.
#[test]
fn statistical_parameter_expansion_reaches_the_built_circuit() {
    use rspice_core::netlist::{
        SpectreDistribution, SpectreSpread, SpectreStatisticalCoordinate, SpectreStatisticsPlan,
        SpectreVariation, SpectreVariationScope,
    };
    let mut plan = SpectreStatisticsPlan {
        variations: vec![SpectreVariation {
            bounds: None,
            line: 3,
            scope: SpectreVariationScope::Process,
            parameter: "rv".into(),
            distribution: SpectreDistribution::Gaussian,
            spread: SpectreSpread::StandardDeviation("5".into()),
            percent: false,
        }],
        correlations: vec![],
    };
    let coordinate = SpectreStatisticalCoordinate {
        seed: 41,
        monte_carlo_run: 9,
        temperature_celsius: 27.0,
        axes: vec![],
    };
    for (variation_scope, function_count) in [
        (SpectreVariationScope::Process, 0),
        (SpectreVariationScope::Process, 65),
        (SpectreVariationScope::Mismatch, 0),
        (SpectreVariationScope::Mismatch, 65),
    ] {
        plan.variations[0].scope = variation_scope;
        let mut definitions = String::new();
        for index in 0..function_count {
            definitions.push_str(&format!(".func unused{index}(x) {{x}}\n"));
        }
        let mut netlist = Netlist::parse(&format!(
            "statistical expansion\n.param rv=100\n.RSPICE_SPECTRE_STAT {}\n{definitions}\
             .func hidden(x) {{rv*x}}\n.func nested(x) {{hidden(x)+1}}\n\
             .param scale=2 top_alias={{scale*rv}} top_bare=top_alias\n\
             .param doubled={{2*rv}} bare_alias=doubled\n\
             .param z={{3+4J}}\n.func captured(x) {{scale*x+img(z)}}\n\
             .param captured_alias={{captured(rv)}} noise_capture={{rv+agauss(0,1,1)}}\n\
             .param scale=9 top_alias=123 root_nested={{bare_alias+rv}}\n\
             .param z=0\n.func captured(x) {{9*x}}\n\
             RROOT in 0 {{hidden(2)}}\nVSTAT stat 0 DC {{nested(1)}}\n\
             .subckt unit a b\n.param derived={{2*rv}}\n.param indirect={{nested(3)}}\n\
             R1 a b {{if(rv<1e6,derived,3*rv)}}\n\
             R2 a b {{hidden(2)}}\nR3 a b {{nested(3)}}\nR4 a b {{indirect}}\n\
             .model RSTAT R(R={{hidden(2)}})\nR5 a b 1 RSTAT\n\
             R6 a b {{bare_alias}}\nR7 a b {{root_nested}}\nR8 a b {{top_alias}}\n\
             R9 a b {{captured_alias}}\nR10 a b {{noise_capture}}\nR11 a b {{top_bare}}\n.ends\n\
             .subckt shadow a b rv=50\n.param local={{2*rv}}\n\
             R1 a b {{rv}}\nR2 a b {{local}}\nR3 a b {{bare_alias}}\n\
             R4 a b {{rv+bare_alias}}\n.model LOCAL R(R={{rv}})\nR5 a b 1 LOCAL\n\
             VS sv b DC {{rv+bare_alias}} AC rv local SIN(rv {{bare_alias}} 1k)\n\
             IS a b DC {{rv+bare_alias}} AC rv local PWL(0 rv 1m {{rv+bare_alias}})\n.ends\n\
             .subckt body_shadow a b\n.param rv=60\n\
             R1 a b {{rv}}\nR2 a b {{bare_alias}}\n\
             XCH a b shadow rv=80\n.ends\n\
             X1 in 0 unit\nX2 in 0 shadow\nX3 in 0 shadow rv=70\n\
             X4 in 0 body_shadow\nV1 in 0 1\n.end\n",
            plan.encode_internal()
        ))
        .expect("statistical deck parses");
        let process = plan.sample_process(&netlist.params, &coordinate).unwrap();
        let sample_for = |identity| {
            plan.sample_mismatch(&netlist.params, &process, identity, &coordinate)
                .unwrap()
                .get("RV")
                .or_else(|| process.get("RV"))
                .copied()
                .unwrap()
        };
        let sample = sample_for("X1");
        let root_sample = sample_for("RROOT");
        let source_sample = sample_for("VSTAT");
        let captured_noise = netlist.params.get("noise_capture").unwrap() - 100.0;
        let expected = if sample < 1e6 {
            2.0 * sample
        } else {
            3.0 * sample
        };
        netlist.spectre_statistical_coordinate = Some(coordinate.clone());
        let circuit = Engine::new(SimulationConfig::default())
            .build_circuit(&netlist)
            .expect("sampled hierarchical circuit builds");
        let mut failures = Vec::new();
        for (name, expected) in [
            ("RROOT", 2.0 * root_sample),
            ("X1.R1", expected),
            ("X1.R2", 2.0 * sample),
            ("X1.R3", 3.0 * sample + 1.0),
            ("X1.R4", 3.0 * sample + 1.0),
            ("X1.R5", 2.0 * sample),
            ("X1.R6", 2.0 * sample),
            ("X1.R7", 3.0 * sample),
            ("X1.R8", 123.0),
            // Readers of a root parameter that is redefined after them
            // read the last definition, as in ngspice-46: scale=9 and
            // z=0 reach captured(rv), and top_bare reads top_alias=123.
            ("X1.R9", 9.0 * sample),
            ("X1.R10", sample + captured_noise),
            ("X1.R11", 123.0),
            ("X2.R1", 50.0),
            ("X2.R2", 100.0),
            ("X2.R3", 2.0 * sample_for("X2")),
            ("X2.R4", 50.0 + 2.0 * sample_for("X2")),
            ("X2.R5", 50.0),
            ("X3.R1", 70.0),
            ("X3.R2", 140.0),
            ("X3.R3", 2.0 * sample_for("X3")),
            ("X3.R4", 70.0 + 2.0 * sample_for("X3")),
            ("X3.R5", 70.0),
            ("X4.R1", 60.0),
            ("X4.R2", 2.0 * sample_for("X4")),
            ("X4.XCH.R1", 80.0),
            ("X4.XCH.R2", 160.0),
            ("X4.XCH.R3", 2.0 * sample_for("X4.XCH")),
            ("X4.XCH.R4", 80.0 + 2.0 * sample_for("X4.XCH")),
            ("X4.XCH.R5", 80.0),
        ] {
            let index = circuit
                .resistor_storage()
                .names
                .iter()
                .position(|actual| actual.eq_ignore_ascii_case(name))
                .expect("resistor exists");
            let actual = circuit.resistor_storage().conductances[index].recip();
            let relative = (actual / expected - 1.0).abs();
            if !relative.is_finite() || relative >= 1e-14 {
                failures.push(format!(
                    "{variation_scope:?}, {function_count} extra functions, {name}: sampled resistance {actual}, expected {expected}"
                ));
            }
        }
        let source_index = circuit
            .voltage_source_storage()
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VSTAT"))
            .expect("source exists");
        let actual = circuit.voltage_source_storage().dc_values[source_index];
        let expected = source_sample + 1.0;
        let relative = (actual / expected - 1.0).abs();
        if !relative.is_finite() || relative >= 1e-14 {
            failures.push(format!(
                "{variation_scope:?}: sampled source {actual}, expected {expected}"
            ));
        }
        let voltage_sources = circuit.voltage_source_storage();
        let current_sources = circuit.current_source_storage();
        for (scope, local) in [("X2", 50.0), ("X3", 70.0), ("X4.XCH", 80.0)] {
            let sampled = sample_for(scope);
            for (kind, names, dc, magnitudes, phases, specs, peak_time) in [
                (
                    "VS",
                    &voltage_sources.names,
                    &voltage_sources.dc_values,
                    &voltage_sources.ac_magnitudes,
                    &voltage_sources.ac_phases,
                    &voltage_sources.source_specs,
                    0.25e-3,
                ),
                (
                    "IS",
                    &current_sources.names,
                    &current_sources.dc_values,
                    &current_sources.ac_magnitudes,
                    &current_sources.ac_phases,
                    &current_sources.source_specs,
                    1e-3,
                ),
            ] {
                let name = format!("{scope}.{kind}");
                let index = names
                    .iter()
                    .position(|actual| actual.eq_ignore_ascii_case(&name))
                    .unwrap();
                let spec = specs[index].as_ref().unwrap();
                let at_time = |time| {
                    VoltageSources::evaluate_source_spec_at_time_with_dialect(
                        spec,
                        time,
                        1e-9,
                        1.0,
                        SpiceDialect::Ngspice,
                    )
                };
                for (field, actual, expected) in [
                    ("DC", dc[index], local + 2.0 * sampled),
                    ("AC magnitude", magnitudes[index], local),
                    ("AC phase", phases[index], (2.0 * local).to_radians()),
                    ("waveform start", at_time(0.0), local),
                    ("waveform peak", at_time(peak_time), local + 2.0 * sampled),
                ] {
                    let relative = (actual / expected - 1.0).abs();
                    if !relative.is_finite() || relative >= 1e-14 {
                        failures.push(format!(
                            "{variation_scope:?}, {name} {field}: {actual}, expected {expected}"
                        ));
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
