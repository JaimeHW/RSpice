use rspice_core::{Engine, Netlist};

#[test]
fn quoted_instance_vectors_reject_trailing_tokens() {
    for (name, value) in [
        ("real_array", "[1 2]"),
        ("string_array", "[alpha beta]"),
        ("complex_array", "[<1 2>]"),
    ] {
        for trailing in ["junk", " [3]", "+4", "]"] {
            let instance = format!("A1 [in] print_param_types {name}=\"{value}{trailing}\"");
            for source in [
                format!("* root literal\n{instance}\n.END\n"),
                format!("* scoped literal\n.SUBCKT cell in\n{instance}\n.ENDS\nX1 in cell\n.END\n"),
            ] {
                let error = Netlist::parse(&source)
                    .map(|_| ())
                    .expect_err(&source)
                    .to_string();
                assert!(error.contains("Unexpected token"), "{source}: {error}");
                assert!(
                    error.to_ascii_lowercase().contains(name),
                    "{source}: {error}"
                );
            }
        }
    }
}

#[test]
fn string_parameter_vectors_reject_trailing_tokens() {
    let source = "* string parameter\n.PARAM payload=\"[alpha beta]junk\"\n\
                  A1 [in] print_param_types string_array={payload}\n.END\n";
    let error = Netlist::parse(source)
        .map(|_| ())
        .expect_err(source)
        .to_string();
    assert!(error.contains("Unexpected token"), "{error}");
}

#[test]
fn complete_quoted_vectors_keep_supported_types_and_whitespace() {
    for fields in [
        "real_array=\" [1 2] \" string_array=\" [alpha beta] \" complex_array=\" [<1 2>] \"",
        "real_array=\"[]\" string_array=\"[alpha]\" complex_array=\"[<1 2>]\"",
    ] {
        let source = format!("* complete vectors\nA1 [in] print_param_types {fields}\n.END\n");
        let netlist = Netlist::parse(&source).unwrap();
        Engine::default().build_circuit(&netlist).unwrap();
    }
}
