//! Textual DC/TRAN expansion must retain tables owned by other consumers.
use rspice_core::engine::{ControlAnalysisResult, ControlCircuit};
use rspice_core::execution::control::{ControlLimits, ControlProgram};
use rspice_core::netlist::{NetlistParseOptions, multi_run::try_expand_multi_run};
use rspice_core::{Engine, Netlist, NoAbort};

const NETWORK: &str = "Frequency ownership\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.data points FREQ\n10\n100\n10\n.enddata\n";

fn check_ac(netlist: &Netlist) {
    let result = Engine::default().run_ac_table(netlist, "points").unwrap();
    assert_eq!(result.columns[0].values, [10.0, 100.0, 10.0]);
    for point in result.points {
        let index = point
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        let wrc = std::f64::consts::TAU * point.frequency * 0.001;
        assert!((point.voltages[index].re - 1.0 / (1.0 + wrc * wrc)).abs() < 1e-10);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn declarative_frequency_tables_survive_multi_run_expansion() {
    let decks = try_expand_multi_run(&format!(
        "{NETWORK}.ac data=points\n.noise V(out) V1 data=points\n.end\n"
    ))
    .unwrap();
    assert_eq!(decks.len(), 1);
    let netlist = Netlist::parse(&decks[0].source).unwrap();
    check_ac(&netlist);
    let noise = Engine::default()
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", 300.15)
        .unwrap();
    assert_eq!(noise.columns[0].values, [10.0, 100.0, 10.0]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn runtime_selected_tables_survive_expansion_and_retained_control_parsing() {
    let decks = try_expand_multi_run(&format!(
        "{NETWORK}.control\nac data=points\nnoise V(out) V1 data=points\n.endc\n.end\n"
    ))
    .unwrap();
    let netlist = Netlist::parse_with_options(
        &decks[0].source,
        NetlistParseOptions {
            retain_control_script: true,
            ..NetlistParseOptions::default()
        },
    )
    .unwrap();
    assert_eq!(netlist.data_tables.len(), 1);
    let program = ControlProgram::parse_deck_with_abort(
        netlist.control_script.as_ref().unwrap().text(),
        ControlLimits::default(),
        &NoAbort,
    )
    .unwrap();
    let mut session = program.start(netlist.params.clone());
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    let engine = Engine::default();
    while let Some(command) = session.next_command(&mut circuit, &NoAbort).unwrap() {
        circuit
            .execute(&engine, &command, session.variables(), &NoAbort)
            .unwrap();
    }
    assert!(matches!(
        circuit.datasets()[0].result,
        ControlAnalysisResult::AcTable(_)
    ));
    assert!(matches!(
        circuit.datasets()[1].result,
        ControlAnalysisResult::NoiseTable(_)
    ));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn textual_sweeps_consume_only_their_own_tables_in_each_alter_variant() {
    let source = format!(
        "{NETWORK}.data bias V1\n1\n2\n.enddata\n.dc data=bias\n.ac data=points\n.alter hotter\n.options temp=127\n.end\n"
    );
    let decks = try_expand_multi_run(&source).unwrap();
    assert_eq!(decks.len(), 4);
    for deck in decks {
        assert!(!deck.source.contains(".data bias"));
        let netlist = Netlist::parse(&deck.source).unwrap();
        assert_eq!(netlist.data_tables.len(), 1);
        assert_eq!(netlist.data_tables[0].name, "points");
        check_ac(&netlist);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_table_shared_by_textual_and_typed_consumers_is_not_removed() {
    for selector in [
        ".ac data=points",
        ".noise V(out) V1 DATA=points",
        ".ac\n+ DATA\n+ = points",
        ".control\nac data=points\n.endc",
    ] {
        let source = format!("{NETWORK}.dc data=points\n{selector}\n.end\n");
        let decks = try_expand_multi_run(&source).unwrap();
        assert_eq!(decks.len(), 3);
        for deck in decks {
            let netlist = Netlist::parse_with_options(
                &deck.source,
                NetlistParseOptions {
                    retain_control_script: true,
                    ..NetlistParseOptions::default()
                },
            )
            .unwrap();
            check_ac(&netlist);
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn runtime_table_continuation_rows_survive_textual_expansion() {
    let source = "Mixed tables\n.param supply=1\nV1 out 0 {supply} AC 1\nR1 out 0 1k\n.data bias supply\n1\n2\n.enddata\n.data points FREQ\n100\n+ 10\n.enddata\n.dc data=bias\n.ac data=points\n.end\n";
    let decks = try_expand_multi_run(source).unwrap();
    assert_eq!(decks.len(), 2);
    for deck in decks {
        let netlist = Netlist::parse(&deck.source).unwrap();
        assert_eq!(netlist.data_tables.len(), 1);
        assert_eq!(
            netlist
                .frequency_data_table_points("points")
                .unwrap()
                .iter()
                .map(|point| point.frequency)
                .collect::<Vec<_>>(),
            [100.0, 10.0]
        );
        assert!(deck.source.contains("+ 10"));
    }
}
