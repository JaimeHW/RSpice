use rspice_core::{Engine, Netlist};

fn deck(fields: &str, definitions: &str, scoped: bool, alias: bool) -> String {
    let model = if alias { "alias" } else { "print_param_types" };
    let mut body = format!("A1 [in] {model} {fields}");
    if scoped {
        body = format!(".SUBCKT cell in\n{body}\n.ENDS\nX1 in cell");
    }
    format!(
        "* typed field shadowing\nV1 in 0 1\n{body}\n{definitions}\n.MODEL alias print_param_types(real=1)\n.END\n"
    )
}

#[test]
fn nonnumeric_instance_fields_do_not_fall_back_to_enclosing_numbers() {
    for (field, literal) in [
        ("string", "\"text\""),
        ("string_array", "[alpha]"),
        ("real_array", "[1 2]"),
        ("complex_array", "[<1 2>]"),
    ] {
        for read in [field.to_owned(), "dependent".into(), "read_field()".into()] {
            for scoped in [false, true] {
                for alias in [false, true] {
                    let definitions = format!(
                        ".PARAM {field}=7\n.GLOBAL_PARAM dependent={{{field}}}\n.FUNC read_field() {{{field}}}"
                    );
                    for consumer in [
                        format!("real={{{read}}}"),
                        format!("complex=<{{{read}}} 0>"),
                    ] {
                        let source = deck(
                            &format!("{consumer} {field}={literal}"),
                            &definitions,
                            scoped,
                            alias,
                        );
                        let netlist = Netlist::parse(&source).unwrap();
                        let error = Engine::default()
                            .build_circuit(&netlist)
                            .map(|_| ())
                            .expect_err(&source)
                            .to_string();
                        assert!(
                            error
                                .to_ascii_uppercase()
                                .contains(&field.to_ascii_uppercase()),
                            "{source}: {error}"
                        );
                        assert!(error.contains("nonnumeric"), "{source}: {error}");
                        assert!(!error.contains("cyclic"), "{source}: {error}");
                    }
                }
            }
        }
    }
}

#[test]
fn vector_and_complex_vector_consumers_cannot_read_numeric_string_fallbacks() {
    for fields in [
        "real_array=[{string}] string=\"text\"",
        "complex_array=[<{string} 0>] string=\"text\"",
        "real={string_array} string_array={payload}",
    ] {
        for scoped in [false, true] {
            let source = deck(
                fields,
                ".PARAM string=7 string_array=7 payload=\"[alpha]\"",
                scoped,
                false,
            );
            let error = Engine::default()
                .build_circuit(&Netlist::parse(&source).unwrap())
                .map(|_| ())
                .expect_err(&source)
                .to_string();
            assert!(
                error.to_ascii_uppercase().contains("STRING"),
                "{source}: {error}"
            );
        }
    }
}

#[test]
fn lazy_branches_and_field_self_references_keep_their_lexical_bindings() {
    for fields in [
        "real={if(1,2,string)} string=\"text\"",
        "real={read_formal(2)} string=\"text\"",
        "real_array=[{real_array+1}]",
        "complex=< {RE(complex)+1} {IMAG(complex)} >",
        "complex_array=[<{complex_array+1} 0>]",
    ] {
        for scoped in [false, true] {
            let source = deck(
                fields,
                ".PARAM string=7 real_array=7 complex={2+3j} complex_array=7\n.FUNC read_formal(string) {string}",
                scoped,
                false,
            );
            Engine::default()
                .build_circuit(&Netlist::parse(&source).unwrap())
                .unwrap();
        }
    }
}

#[test]
fn nonnumeric_masking_preserves_typed_cancellation_and_retry() {
    use rspice_core::{SimulationError, abort_signal::CountingAbort};
    for scoped in [false, true] {
        let source = deck(
            "real={read_value()} string=\"text\"",
            ".PARAM string=7\n.FUNC read_value() {if(1,4,string)}",
            scoped,
            true,
        );
        let netlist = Netlist::parse(&source).unwrap();
        let count = CountingAbort::new(usize::MAX);
        Engine::default()
            .build_circuit_with_abort(&netlist, &count)
            .unwrap();
        for limit in 0..count.count() {
            let abort = CountingAbort::new(limit);
            let error = Engine::default()
                .build_circuit_with_abort(&netlist, &abort)
                .map(|_| ())
                .expect_err("cancelled build");
            assert!(
                matches!(error, SimulationError::Aborted),
                "limit={limit}, scoped={scoped}: {error}"
            );
            assert_eq!(abort.count(), limit + 1);
            assert_eq!(abort.polls_after_abort(), 0);
        }
        Engine::default().build_circuit(&netlist).unwrap();
    }
}
