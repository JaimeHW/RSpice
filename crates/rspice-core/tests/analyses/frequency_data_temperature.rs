//! A frequency table has one physical temperature for circuit and noise laws.
use rspice_core::{Engine, Netlist};

fn deck(columns: &str, rows: &str, options: &str) -> Netlist {
    Netlist::parse(&format!(
        "Thermal table\nV1 in 0 AC 1\nR1 in out 1k tc1=.01 tnom=27\nC1 out 0 1u\n{options}\n.data points {columns}\n{rows}\n.enddata\n.end\n"
    ))
    .unwrap()
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs() * 1e-9 + 1e-30,
        "{actual:e} != {expected:e}"
    );
}

fn check(engine: &Engine, netlist: &Netlist, temperatures: &[f64], fallback: f64) {
    let ac = engine.run_ac_table(netlist, "points").unwrap();
    let noise = engine
        .run_noise_table_named_with_input_source(netlist, "out", None, "V1", "points", fallback)
        .unwrap();
    for ((ac, noise), &temperature) in ac.points.iter().zip(&noise.points).zip(temperatures) {
        let resistance = 1000.0 * (1.0 + 0.01 * (temperature - 27.0));
        let wrc = std::f64::consts::TAU * ac.frequency * resistance * 1e-6;
        let out = ac
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .unwrap();
        close(ac.voltages[out].re, 1.0 / (1.0 + wrc * wrc));
        close(ac.voltages[out].im, -wrc / (1.0 + wrc * wrc));
        close(noise.voltages[out].re, ac.voltages[out].re);
        close(noise.voltages[out].im, ac.voltages[out].im);
        close(
            noise.input_referred_density,
            4.0 * 1.380649e-23 * (temperature + 273.15) * resistance,
        );
        close(
            noise.output_noise_density,
            noise.input_referred_density / (1.0 + wrc * wrc),
        );
    }
    assert_eq!(ac.points.len(), temperatures.len());
    assert_eq!(noise.points.len(), temperatures.len());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn temperature_coordinates_do_not_require_an_authored_temperature_binding() {
    let netlist = deck("FREQ TEMP", "100 27\n10 127\n100 -23", "");
    assert_eq!(netlist.options.temp, None);
    assert_eq!(netlist.params.get("TEMP"), Some(27.0));
    let base = Engine::default();
    check(&base, &netlist, &[27.0, 127.0, -23.0], 999.0);
    check(
        &base.resolved_for_netlist(&netlist),
        &netlist,
        &[27.0, 127.0, -23.0],
        999.0,
    );
    let (rows, _) = base.run_ac_data(&netlist, "points").unwrap();
    for (row, temperature) in rows.iter().zip([27.0, 127.0, -23.0]) {
        assert_eq!(row.options.temp, Some(temperature));
        assert_eq!(row.params.get("TEMP"), Some(temperature));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_temperature_coordinates_override_the_resolved_run_temperature() {
    let netlist = deck("FREQ TEMP", "100 27\n10 127\n100 -23", ".options temp=27");
    let base = Engine::default();
    let mut config = base.resolved_for_netlist(&netlist).config().clone();
    config.temperature = 350.0;
    let engine = base.try_resolved_with_config(config).unwrap();
    check(&engine, &netlist, &[27.0, 127.0, -23.0], 999.0);
    assert_eq!(engine.config().temperature, 350.0);
    assert_eq!(netlist.options.temp, Some(27.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unchanged_authored_temperature_does_not_replace_the_resolved_run_policy() {
    let netlist = deck("FREQ", "100\n10", ".options temp=27");
    let base = Engine::default();
    let mut config = base.resolved_for_netlist(&netlist).config().clone();
    config.temperature = 400.15;
    let engine = base.try_resolved_with_config(config).unwrap();
    check(&engine, &netlist, &[127.0, 127.0], 999.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn noise_fallback_heats_the_circuit_as_well_as_the_noise_sources() {
    let netlist = deck("FREQ", "100\n10", "");
    let engine = Engine::default();
    let result = engine
        .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", 400.15)
        .unwrap();
    let (_, legacy) = engine
        .run_noise_data_named_with_input_source(&netlist, "out", None, "V1", "points", 400.15)
        .unwrap();
    for (point, compatibility) in result.points.iter().zip(legacy) {
        let wrc = std::f64::consts::TAU * point.frequency * 2000.0 * 1e-6;
        let density = 4.0 * 1.380649e-23 * 400.15 * 2000.0;
        close(point.input_referred_density, density);
        close(point.output_noise_density, density / (1.0 + wrc * wrc));
        assert_eq!(
            point.input_referred_density,
            compatibility.input_referred_density
        );
        assert_eq!(point.voltages, compatibility.voltages);
    }
    assert!(
        engine
            .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "points", -1.0)
            .is_err()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unresolved_rows_recompute_parameter_dependent_temperature() {
    let netlist = deck(
        "FREQ ambient",
        "100 27\n10 127",
        ".param ambient=27\n.options temp={ambient}",
    );
    check(&Engine::default(), &netlist, &[27.0, 127.0], f64::NAN);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn physical_temperature_survives_executed_control_options_and_legacy_replay() {
    use rspice_core::engine::ControlCircuit;
    use rspice_core::execution::control::ControlCommand;
    let netlist = deck("FREQ TEMP", "100 27\n10 127", ".options temp=27");
    let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
    let engine = Engine::default();
    circuit
        .execute(
            &engine,
            &ControlCommand {
                name: "option".into(),
                arguments: "temp=77 reltol=.002".into(),
                line: 1,
            },
            &netlist.params,
            &rspice_core::NoAbort,
        )
        .unwrap();
    let resolved = engine.resolved_for_netlist(circuit.netlist());
    check(&resolved, circuit.netlist(), &[27.0, 127.0], 999.0);
    let (rows, legacy) = resolved.run_ac_data(circuit.netlist(), "points").unwrap();
    let compact = resolved.run_ac_table(circuit.netlist(), "points").unwrap();
    for (index, row) in rows.iter().enumerate() {
        assert_eq!(row.options.temp, Some([27.0, 127.0][index]));
        assert_eq!(row.options.temp, row.params.get("TEMP"));
        assert_eq!(row.options.reltol, Some(0.002));
        assert_eq!(legacy[index].voltages, compact.points[index].voltages);
    }
    assert_eq!(circuit.netlist().options.temp, Some(77.0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn control_replay_overrides_only_options_changed_by_executed_commands() {
    use rspice_core::engine::ControlCircuit;
    use rspice_core::execution::control::ControlCommand;
    let netlist = deck(
        "FREQ ambient",
        "100 27\n10 127",
        ".param ambient=27\n.options temp={ambient}",
    );
    for (option, expected) in [
        ("reltol=.002", [27.0, 127.0]),
        ("temp=77 reltol=.002", [77.0, 77.0]),
    ] {
        let mut circuit = ControlCircuit::new(netlist.clone()).unwrap();
        let engine = Engine::default();
        circuit
            .execute(
                &engine,
                &ControlCommand {
                    name: "option".into(),
                    arguments: option.into(),
                    line: 1,
                },
                &netlist.params,
                &rspice_core::NoAbort,
            )
            .unwrap();
        check(&engine, circuit.netlist(), &expected, 999.0);
        let (rows, _) = engine.run_ac_data(circuit.netlist(), "points").unwrap();
        for (row, temperature) in rows.iter().zip(expected) {
            assert_eq!(row.options.temp, Some(temperature));
            assert_eq!(row.options.reltol, Some(0.002));
        }
    }
}
