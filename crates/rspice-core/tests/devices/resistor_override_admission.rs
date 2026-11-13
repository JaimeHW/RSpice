//! Resistor overrides must change an implemented parameter or fail explicitly.
use rspice_core::{Engine, Netlist, NoAbort, ResourceLimits, SimulationError};

fn out_voltage(netlist: &Netlist) -> f64 {
    let result = Engine::default().run_dc_op(netlist).unwrap();
    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    result.node_voltages[out]
}

fn close(actual: f64, expected: f64) {
    // Ordinary DC retains the default 1 pS shunt conductance.
    assert!(
        (actual - expected).abs() < expected.abs() * 2e-8,
        "{actual} != {expected}"
    );
}

fn override_row(netlist: &Netlist, name: &str, value: f64) -> Result<Netlist, SimulationError> {
    Engine::create_perturbed_netlist_multi_with_limits_and_abort(
        netlist,
        &[(name.into(), value)],
        ResourceLimits::default(),
        &NoAbort,
    )
    .map(|(row, _)| row)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unknown_resistor_overrides_fail_without_mutating_the_source() {
    let source = "Overrides\nI1 0 out 1m\nR1 out 0 1k\n.end\n";
    let netlist = Netlist::parse(source).unwrap();
    for name in ["R1:bogus", "R1:resistnace", "R1:TNOM"] {
        let error = override_row(&netlist, name, 2000.0).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Unsupported resistor step parameter"),
            "{error}"
        );
    }
    assert_eq!(netlist.source_text.as_deref(), Some(source));
    close(out_voltage(&netlist), 1.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn resistor_alias_overrides_replace_the_authored_binding() {
    let netlist = Netlist::parse("Aliases\nI1 0 out 1m\nR1 out 0 1k MULT=2\n.end\n").unwrap();
    for name in ["R1:M", "R1:mult"] {
        let row = override_row(&netlist, name, 4.0).unwrap();
        close(out_voltage(&row), 0.25);
    }
    let temperature = Netlist::parse(
        "Temperature\nI1 0 out 1m\nR1 out 0 1k TC1=.001\n.options temp=127 tnom=27\n.end\n",
    )
    .unwrap();
    let row = override_row(&temperature, "R1:TC", 0.002).unwrap();
    close(out_voltage(&row), 1.2);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn noise_alias_override_changes_the_resistors_published_spectrum() {
    let netlist =
        Netlist::parse("Noise override\nI1 0 out AC 1\nR1 out 0 1k NOISY=1\n.end\n").unwrap();
    let engine = Engine::default();
    let spectrum = |row: &Netlist| {
        engine
            .run_noise_named_with_input_source(row, "out", None, "I1", &[10.0], 300.15)
            .unwrap()[0]
            .output_noise_density
    };
    assert!(spectrum(&netlist) > 0.0);
    let quiet = override_row(&netlist, "R1:NOISE", 0.0).unwrap();
    assert_eq!(spectrum(&quiet), 0.0);
}
