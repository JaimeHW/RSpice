//! Parameters consumed by immutable structure and compiled discrete behavior.
use rspice_veriloga::{CompilerOptions, NoPipelineControl, ScalarParameterValue, VerilogACompiler};

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
}

const CONSUMERS: &[(&str, &str)] = &[
    (
        "digital behavior",
        "reg [7:0] q; initial q=N; analog I(p)<+N;",
    ),
    ("packed digital shape", "reg [N-1:0] q; analog I(p)<+N;"),
    (
        "digital array",
        "reg [7:0] cells[0:N-1]; initial cells[0]=1; analog I(p)<+cells[0];",
    ),
    (
        "promoted numeric array",
        "integer cells[0:N-1]; initial cells[0]=1; analog I(p)<+cells[0];",
    ),
    (
        "analog local array",
        "analog begin:scope real cells[0:N-1]; cells[0]=N; I(p)<+cells[0]; end",
    ),
    (
        "analog numeric array",
        "real cells[0:N-1]; analog begin cells[0]=N; I(p)<+cells[0]; end",
    ),
];

#[test]
fn compiled_consumers_guard_transitive_numeric_dependencies() {
    let compiler = compiler();
    let mut failures = Vec::new();
    for &(label, body) in CONSUMERS {
        let source = format!(
            "module guarded(p); inout p; electrical p; parameter integer BASE=2; aliasparam SIZE=BASE; parameter real GAIN=1; parameter integer N=BASE+1; {body} endmodule"
        );
        let report = match compiler.compile_runtime(&source, None) {
            Ok(report) => report,
            Err(error) => {
                failures.push(format!("{label}: {error}"));
                continue;
            }
        };
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "guarded",
            report.model.clone(),
            &report.canonical_ir,
            &[1],
        )
        .unwrap();
        assert!(device.try_set_parameter("GAIN", 2.0).unwrap(), "{label}");
        let specialized = compiler
            .specialize_mixed_runtime_typed(
                &report.canonical_ir,
                &[("SIZE", ScalarParameterValue::Integer(4))],
                &NoPipelineControl,
            )
            .unwrap();
        assert_eq!(
            specialized
                .model
                .parameters
                .iter()
                .find(|p| p.name == "BASE")
                .unwrap()
                .elaboration_value,
            Some(4.0),
            "{label}"
        );
        assert_shape(&specialized, label, 5);
        if !device
            .try_set_parameter("SIZE", 4.0)
            .is_err_and(|error| error.to_string().contains("specialize the source"))
        {
            failures.push(format!(
                "{label}: accepted a value change that invalidates compiled content"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn compiled_consumers_guard_supplied_state_in_shape_defaults() {
    let compiler = compiler();
    let mut failures = Vec::new();
    for &(label, body) in CONSUMERS {
        let source = format!(
            "module guarded(p); inout p; electrical p; parameter integer BASE=2; aliasparam SIZE=BASE; parameter real GAIN=1; parameter integer N=$param_given(SIZE)?3:2; {body} endmodule"
        );
        let report = match compiler.compile_runtime(&source, None) {
            Ok(report) => report,
            Err(error) => {
                failures.push(format!("{label}: {error}"));
                continue;
            }
        };
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "guarded",
            report.model.clone(),
            &report.canonical_ir,
            &[1],
        )
        .unwrap();
        let specialized = compiler
            .specialize_mixed_runtime_typed(
                &report.canonical_ir,
                &[("SIZE", ScalarParameterValue::Integer(2))],
                &NoPipelineControl,
            )
            .unwrap();
        assert_eq!(
            specialized.model.parameters[0].elaboration_given,
            Some(true),
            "{label}"
        );
        assert_eq!(
            specialized.model.parameters[0].elaboration_value, None,
            "presence alone must not freeze BASE"
        );
        assert_shape(&specialized, label, 3);
        if !device
            .try_set_parameter("SIZE", 2.0)
            .is_err_and(|error| error.to_string().contains("specialize the source"))
        {
            failures.push(format!(
                "{label}: accepted a presence change that invalidates compiled content"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn assert_shape(report: &rspice_veriloga::RuntimeCompileReport, label: &str, n: u32) {
    match label {
        "packed digital shape" => assert_eq!(
            report
                .canonical_ir
                .digital
                .signals
                .iter()
                .find(|s| s.name == "q")
                .unwrap()
                .width,
            n
        ),
        "digital array" | "promoted numeric array" => assert_eq!(
            report.canonical_ir.digital.arrays[0].bounds,
            (0, i64::from(n) - 1)
        ),
        "analog numeric array" | "analog local array" => {
            assert_eq!(report.model.num_variables, n as usize)
        }
        _ => {}
    }
}

#[test]
fn lexical_variables_do_not_freeze_or_impersonate_public_parameters() {
    let compiler = compiler();
    let report=compiler.compile_runtime("module shadow(p); inout p; electrical p; parameter integer N=2; reg [7:0] q; initial begin:scope integer N; N=4; q=N; end analog I(p)<+N; endmodule",None).unwrap();
    assert_eq!(report.model.parameters[0].elaboration_value, None);
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "shadow",
        report.model.clone(),
        &report.canonical_ir,
        &[1],
    )
    .unwrap();
    assert!(device.try_set_parameter("N", 4.0).unwrap());
    assert!(compiler.compile_runtime("module reversed(p); inout p; electrical p; parameter integer N=2; analog begin:scope real cells[0:N-1]; integer N; N=4; I(p)<+N; end endmodule",None).is_err());
    assert!(compiler.compile_runtime("module shadow(p); inout p; electrical p; parameter integer N=2; analog begin:scope integer N; real cells[0:N-1]; N=4; I(p)<+N; end endmodule",None).is_err());
}
