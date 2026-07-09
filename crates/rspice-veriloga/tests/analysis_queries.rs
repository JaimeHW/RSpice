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
