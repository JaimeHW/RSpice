//! Explicit voltage ports must retain their type through circuit construction.
use rspice_core::netlist::{
    ElementKind, XspicePort, flatten_netlist_with_models, reduce_supernode_topology,
};
use rspice_core::{Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_voltage_cannot_silently_become_a_digital_connection() {
    for port in ["%v in", "%v(in)", "%v([in])"] {
        for deck in [
            format!("Typed voltage\nA1 {port} out d_buffer\n.end\n"),
            format!(
                "Typed voltage in hierarchy\n.subckt cell in out\nA1 {port} out d_buffer\n.ends\nX1 a b cell\n.end\n"
            ),
        ] {
            let netlist = Netlist::parse(&deck).unwrap();
            let error = Engine::default()
                .build_circuit(&netlist)
                .map(|_| ())
                .expect_err(&deck);
            assert!(error.to_string().contains("explicit Voltage"), "{error}");
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_voltage_is_rejected_on_current_only_vector_outputs() {
    for ports in ["%v(out)", "%v([a b])", "[%v(a) %i(b)]"] {
        let netlist = Netlist::parse(&format!(
            "Current-only outputs\nA1 null null {ports} seegen\n.end\n"
        ))
        .unwrap();
        let error = Engine::default()
            .build_circuit(&netlist)
            .map(|_| ())
            .expect_err(ports);
        assert!(error.to_string().contains("explicit Voltage"), "{error}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_voltage_type_survives_hierarchy_and_supernode_reduction() {
    let mut netlist = Netlist::parse(
        "Typed voltage hierarchy\n.subckt cell a b\nA1 %v(a) %v(b) gain gain=2\n.ends\nV1 in 0 3\nX1 in mid cell\nRshort mid out 0\nRload out 0 2\n.end\n"
    ).unwrap();
    let flattened = flatten_netlist_with_models(&netlist).unwrap();
    let reduced = reduce_supernode_topology(flattened.elements.clone(), 0.0);
    for elements in [&flattened.elements, &reduced.elements] {
        let ports = elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Xspice { ports, .. } => Some(ports),
                _ => None,
            })
            .unwrap();
        assert_eq!(ports[0], XspicePort::ExplicitVoltage("IN".to_string()));
        assert!(matches!(&ports[1], XspicePort::ExplicitVoltage(_)));
        assert!(ports.iter().all(XspicePort::is_analog));
        assert_eq!(ports[0].node_names(), ["IN"]);
    }
    netlist.options.topology_supernode = Some(true);
    let op = Engine::default().run_dc_op(&netlist).unwrap();
    let output_name = reduced
        .elements
        .iter()
        .find_map(|element| {
            if let ElementKind::Xspice { ports, .. } = &element.kind
                && let XspicePort::ExplicitVoltage(name) = &ports[1]
            {
                Some(name.as_str())
            } else {
                None
            }
        })
        .unwrap();
    let out = op
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case(output_name))
        .unwrap();
    assert!((op.node_voltages[out] - 6.0).abs() < 1e-9, "{op:?}");
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn compact_voltage_vectors_retain_explicit_types_and_bare_defaults() {
    let netlist = Netlist::parse("Compact voltage\nA1 %v([a b]) c summer\n.end\n").unwrap();
    let ElementKind::Xspice { ports, .. } = &netlist.elements[0].kind else {
        panic!("code model")
    };
    assert_eq!(
        ports,
        &[
            XspicePort::ExplicitVoltage("A".to_string()),
            XspicePort::ExplicitVoltage("B".to_string()),
            XspicePort::Analog("C".to_string()),
        ]
    );
}

struct TableFile(&'static str);

impl TableFile {
    fn linear(path: &'static str) -> Self {
        let mut data = String::from("4\n4\n0 1 2 3\n0 1 2 3\n");
        for y in 0..4 {
            for x in 0..4 {
                data.push_str(&format!("{} ", 2 * x + 3 * y));
            }
            data.push('\n');
        }
        rspice_core::xspice::register_data_file(path, data).unwrap();
        Self(path)
    }
}

impl Drop for TableFile {
    fn drop(&mut self) {
        let _ = rspice_core::xspice::unregister_data_file(self.0);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_table_voltage_outputs_preserve_dc_ac_polarity_and_load_independence() {
    let file = TableFile::linear("virtual://voltage-ports/dc-ac");
    for (port, dc, ac) in [
        ("%v(out)", 3.5, 2.0),
        ("%vd[out ref]", 13.5, 2.0),
        ("%vd[ref out]", 6.5, -2.0),
    ] {
        for load in [2.0, 10.0] {
            let deck = format!(
                "Explicit table voltage\nVX x 0 DC 1 AC 1\nVY y 0 0.5\nVREF ref 0 10\nA1 x y {port} cm\n.model cm table2d(file=\"{}\" order=2)\nRLOAD out 0 {load}\n.end\n",
                file.0
            );
            let netlist = Netlist::parse(&deck).unwrap();
            let engine = Engine::default();
            let op = engine.run_dc_op(&netlist).unwrap();
            let out = op
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            assert!(
                (op.node_voltages[out] - dc).abs() < 1e-9,
                "{port}, load={load}: {op:?}"
            );
            let points = engine.run_ac(&netlist, &[1.0, 1e6]).unwrap();
            for point in points {
                let out = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                assert!(
                    (point.voltages[out].re - ac).abs() < 1e-9,
                    "{port}: {point:?}"
                );
                assert!(point.voltages[out].im.abs() < 1e-9, "{port}: {point:?}");
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn explicit_table_voltage_outputs_follow_transient_inputs() {
    let file = TableFile::linear("virtual://voltage-ports/transient");
    for (port, reference, polarity) in [
        ("%v(out)", 0.0, 1.0),
        ("%vd[out ref]", 10.0, 1.0),
        ("%vd[ref out]", 10.0, -1.0),
    ] {
        let netlist = Netlist::parse(&format!(
            "Transient table voltage\nVX x 0 PWL(0 0 1u 1 2u 1)\nVY y 0 0.5\nVREF ref 0 10\nA1 x y {port} cm\n.model cm table2d(file=\"{}\" order=2)\nRLOAD out 0 2\nCLOAD out 0 1n\n.end\n", file.0
        )).unwrap();
        let result = Engine::default().run_tran(&netlist, 2e-6, 1e-7).unwrap();
        let out = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!(result.time.len() > 10);
        assert!((result.time.last().unwrap() - 2e-6).abs() < 1e-15);
        for (time, value) in result.time.iter().zip(&result.voltages[out]) {
            let expected = reference + polarity * (1.5 + 2.0 * (time / 1e-6).min(1.0));
            assert!(
                (value - expected).abs() < 1e-7,
                "{port}: t={time}, value={value}, expected={expected}"
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn vector_voltage_selection_preserves_neighboring_current_outputs() {
    let path = "virtual://voltage-ports/mixed-vector";
    rspice_core::xspice::register_data_file(path, "0 1 2 3 4\n1e-6 2 3 4 5\n").unwrap();
    let _file = TableFile(path);
    for ports in ["[%v(a) %vd[0 b] %i(c) d]", "%v(a) %vd[0 b] %i(c) d"] {
        let netlist = Netlist::parse(&format!(
            "Mixed vector outputs\nA1 {ports} fs\n.model fs filesource(file=\"{path}\")\nRA a 0 2\nRB b 0 2\nRC c 0 2\nRD d 0 2\n.end\n"
        )).unwrap();
        let result = Engine::default().run_tran(&netlist, 1e-6, 1e-7).unwrap();
        for (name, offset, gain) in [
            ("a", 1.0, 1.0),
            ("b", 2.0, -1.0),
            ("c", 3.0, -2.0),
            ("d", 4.0, 1.0),
        ] {
            let node = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            for (time, value) in result.time.iter().zip(&result.voltages[node]) {
                let expected = gain * (offset + time / 1e-6);
                assert!(
                    (value - expected).abs() < 1e-7,
                    "{ports}, {name}: t={time}, value={value}, expected={expected}"
                );
            }
        }
    }
}
