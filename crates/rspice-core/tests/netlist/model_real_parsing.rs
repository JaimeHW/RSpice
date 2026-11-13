use rspice_core::{Engine, Netlist};

#[test]
fn eager_model_fields_reject_complex_expressions_aliases_and_literals() {
    for model in ["D(IS=VALUE)", "R(RSH=VALUE)", "gain(gain=VALUE)"] {
        for value in ["{2+sqrt(-1)}", "{2+1e-300j}", "z", "-z", "(z)", "1j"] {
            let source = format!(
                "* real model fields\n.PARAM z={{2+sqrt(-1)}}\n.MODEL device {}\n.END\n",
                model.replace("VALUE", value)
            );
            let error = Netlist::parse(&source)
                .expect_err("non-real model value must fail")
                .to_string();
            assert!(error.contains("real value"), "{model}, {value}: {error}");
        }
    }
}

#[test]
fn forward_model_fields_cannot_drop_an_imaginary_component() {
    for value in ["z", "{z}", "{later()}"] {
        let source = format!(
            "* forward real model\n.MODEL device D(IS={value})\n\
             .PARAM z={{2+1e-300j}}\n.FUNC later() {{z}}\n.END\n"
        );
        let error = Netlist::parse(&source)
            .expect_err("non-real model value must fail")
            .to_string();
        assert!(
            error.contains("IS") && error.contains("real value"),
            "{error}"
        );
    }
}

#[test]
fn vector_entries_and_complex_literal_components_require_real_values() {
    for field in [
        "real_array=[0 {2+1e-300j}]",
        "complex=<{2+1e-300j} 1>",
        "complex=<1 {2+1e-300j}>",
        "complex_array=[<1 {2+1e-300j}>]",
    ] {
        let source =
            format!("* typed model fields\n.MODEL device print_param_types({field})\n.END\n");
        let error = Netlist::parse(&source)
            .expect_err("non-real model value must fail")
            .to_string();
        assert!(error.contains("real value"), "{field}: {error}");
    }
}

#[test]
fn model_fields_keep_explicit_complex_projections_and_complex_pairs() {
    let source = "* valid model fields\n.PARAM z={2+3j}\n\
        .MODEL device print_param_types(real={RE(z)} complex=<{RE(z)} {IMG(z)}>\n\
        + real_array=[{RE(z)} {IMG(z)}] complex_array=[<{RE(z)} {IMG(z)}>])\n.END\n";
    let netlist = Netlist::parse(source).unwrap();
    let model = &netlist.models[0];
    assert!(
        model
            .params
            .iter()
            .any(|(name, value)| name == "REAL" && *value == 2.0)
    );
    assert!(
        model
            .real_vector_params
            .iter()
            .any(|(_, values)| values == &[2.0, 3.0])
    );
    assert!(
        model
            .string_params
            .iter()
            .any(|(_, value)| value == "<2 3>")
    );
}

#[test]
fn scoped_model_fields_do_not_bypass_real_value_validation() {
    for (model, instance) in [
        ("D(IS={z})", "D1 out 0 device"),
        ("R(RSH={z})", "R1 out 0 device L=1 W=1"),
        ("gain(gain={z})", "A1 out aux device"),
        ("pwl(x_array=[0 1] y_array=[0 {z}])", "A1 out aux device"),
        ("print_param_types(complex=<1 {z}>)", "A1 [out] device"),
        (
            "print_param_types(complex_array=[<1 {z}>])",
            "A1 [out] device",
        ),
    ] {
        let source = format!(
            "* scoped real fields\n.SUBCKT child out PARAMS: z=2\n{instance}\n\
             .MODEL device {model}\n.ENDS\nX1 out child z={{2+1e-300j}}\n.END\n"
        );
        let netlist = Netlist::parse(&source).unwrap();
        let error = Engine::default()
            .build_circuit(&netlist)
            .expect_err("non-real scoped model value must fail")
            .to_string();
        assert!(error.contains("real value"), "{model}: {error}");
    }
}
