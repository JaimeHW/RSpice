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
                .err()
                .expect(&deck);
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
            .err()
            .expect(ports);
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
