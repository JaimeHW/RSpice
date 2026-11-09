mod support;

use support::DeviceFixture;

#[test]
fn analysis_accepts_multiple_names_and_unknown_extensions() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module analysis_query_list(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ analysis("dc", "tran", "vendor_private")
                     + 10.0 * analysis("unknown_vendor_mode");
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[0.0]);

    for (analysis, expected) in [(0, 1.0_f64), (1, 0.0), (2, 1.0), (3, 0.0), (4, 0.0)] {
        device.set_analysis_type(analysis);
        let current = device
            .try_evaluate()
            .expect("analysis query evaluation succeeds")[0];
        assert_eq!(current.to_bits(), expected.to_bits(), "analysis {analysis}");
    }
}

#[test]
fn analysis_rejects_non_string_arguments() {
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let error = compiler
        .compile(
            r#"
`include "disciplines.vams"
module invalid_analysis_query(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ analysis("dc", 1.0);
endmodule
"#,
        )
        .expect_err("non-string analysis query must be rejected");
    assert!(error.to_string().contains("string"), "{error}");
}

/// `$simparam$str` (LRM 2.4 Table 9-27): `"module"` is the enclosing module's
/// name, fixed at elaboration, so a comparison with it folds. A compiled model
/// is shared by every instance and run, so the other names are refused by
/// name. The call once reached code generation as an unsupported feature.
#[test]
fn simparam_str_answers_the_module_name_and_refuses_the_rest_by_name() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module named_cell(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (($simparam$str("module") == "named_cell") ? 2.0 : 1.0)
                     + (($simparam$str("module") != "other") ? 10.0 : 0.0);
endmodule
"#,
    );
    let mut device = model.device("S1", &[1, 0]);
    device.update_voltages(&[0.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 12.0);

    let compiler = rspice_veriloga::VerilogACompiler::default();
    for (query, reason) in [
        ("instance", "no instance name or hierarchical path"),
        ("path", "no instance name or hierarchical path"),
        ("cwd", "cannot hold one"),
        ("analysis_name", "analysis(\"tran\")"),
        ("analysis_type", "analysis(\"tran\")"),
        ("vendor", "the only name answered is \"module\""),
    ] {
        let error = compiler
            .compile(&format!(
                "module q(p, n); inout p, n; electrical p, n;
                 analog I(p, n) <+ ($simparam$str(\"{query}\") == \"x\") ? 1.0 : 0.0;
                 endmodule"
            ))
            .expect_err("an unanswered $simparam$str name is refused")
            .to_string();
        assert!(
            error.contains(&format!("$simparam$str(\"{query}\") is not supported"))
                && error.contains(reason),
            "{query}: {error}"
        );
    }
}
