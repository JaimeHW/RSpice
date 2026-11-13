use rspice_core::{Engine, Netlist};

fn reject(source: &str) {
    let error = match Netlist::parse(source) {
        Err(error) => error.to_string(),
        Ok(netlist) => match Engine::default().build_circuit(&netlist) {
            Err(error) => error.to_string(),
            Ok(_) => panic!("non-real instance field was accepted: {source}"),
        },
    };
    assert!(error.contains("real value"), "{source}: {error}");
}

#[test]
fn xspice_scalar_overrides_reject_complex_values_without_truncation() {
    for value in [
        "{2+sqrt(-1)}",
        "{2+1e-300j}",
        "z",
        "-z",
        "1j",
        "-.5j",
        "2+1j",
    ] {
        reject(&format!(
            "* scalar override\n.PARAM z={{2+1e-300j}}\nV1 in 0 1\n\
             A1 in out gain gain={value}\nR1 out 0 1k\n.END\n"
        ));
    }
}

#[test]
fn xspice_vector_and_pair_components_require_real_values() {
    for field in [
        "real_array=[0 {z}]",
        "real_array=[0 1j]",
        "integer_array=[0 {z}]",
        "complex=<{z} 1>",
        "complex=<1 {z}>",
        "complex=<1 1j>",
        "complex_array=[<1 {z}>]",
    ] {
        reject(&format!(
            "* typed override\n.PARAM z={{2+1e-300j}}\nV1 in 0 1\n\
             A1 [in] print_param_types {field}\n.END\n"
        ));
    }
}

#[test]
fn scoped_xspice_fields_cannot_bypass_real_value_validation() {
    for field in [
        "real={z}",
        "real=z",
        "real_array=[0 {z}]",
        "integer_array=[0 {z}]",
        "complex=<{z} 1>",
        "complex=<1 {z}>",
        "complex_array=[<1 {z}>]",
    ] {
        reject(&format!(
            "* scoped typed override\n.SUBCKT cell in PARAMS: z=2\n\
             A1 [in] print_param_types {field}\n.ENDS\n\
             X1 in cell z={{2+1e-300j}}\nV1 in 0 1\n.END\n"
        ));
    }
}

#[test]
fn scoped_physical_scalars_reject_complex_results() {
    for device in ["R1 in 0 {z}", "C1 in 0 {z}", "R1 in 0 1k L={z}"] {
        reject(&format!(
            "* scoped physical field\n.SUBCKT cell in PARAMS: z=2\n{device}\n\
             .ENDS\nX1 in cell z={{2+1e-300j}}\n.END\n"
        ));
    }
}

#[test]
fn explicit_projection_preserves_gain_in_root_and_nested_instances() {
    let body = "A1 in out gain gain={IMG(z)}\nR1 out 0 1k";
    for devices in [
        format!(".PARAM z={{2+3j}}\n{body}"),
        format!(".SUBCKT cell in out PARAMS: z=0\n{body}\n.ENDS\nX1 in out cell z={{2+3j}}"),
    ] {
        let netlist =
            Netlist::parse(&format!("* projected gain\nV1 in 0 1\n{devices}\n.END\n")).unwrap();
        let result = Engine::default().run_dc_op(&netlist).unwrap();
        assert!((result.try_voltage_named("out").unwrap() - 3.0).abs() < 1e-12);
    }
}

#[test]
fn complex_pairs_keep_both_components_after_scoped_projection() {
    let body = "A1 [in] print_param_types complex=<{RE(z)} {IMG(z)}> real_array=[{RE(z)} {IMG(z)}]";
    let source = format!(
        "* complex pair\n.SUBCKT cell in PARAMS: z=0\n{body}\n.ENDS\nX1 in cell z={{2+3j}}\n.END\n"
    );
    let netlist = Netlist::parse(&source).unwrap();
    let flattened = rspice_core::netlist::flatten_netlist_with_models(&netlist).unwrap();
    let rspice_core::netlist::ElementKind::Xspice {
        string_params,
        real_vector_params,
        ..
    } = &flattened.elements[0].kind
    else {
        panic!("expected XSPICE instance")
    };
    assert!(
        string_params
            .iter()
            .any(|(name, value)| name == "COMPLEX" && value == "<2 3>")
    );
    assert!(
        real_vector_params
            .iter()
            .any(|(_, value)| value == &[2.0, 3.0])
    );
    Engine::default().build_circuit(&netlist).unwrap();
}
