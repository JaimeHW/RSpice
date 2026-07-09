mod support;

use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use support::DeviceFixture;

fn assert_unsupported(source: &str, expected_context: &str) {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect_err("unsupported syntax must not compile by being discarded");
    let message = error.to_string();
    assert!(
        message.contains(expected_context),
        "expected diagnostic containing {expected_context:?}, got {message:?}"
    );
}

#[test]
fn unsupported_module_instances_are_not_silently_discarded() {
    assert_unsupported(
        r#"
`include "disciplines.vams"
module unsupported_instance(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        "Unsupported module item: child",
    );
}

#[test]
fn unsupported_keyword_module_items_are_not_silently_discarded() {
    assert_unsupported(
        r#"
`include "disciplines.vams"
module unsupported_wire(p, n);
    inout p, n;
    electrical p, n;
    wire hidden;
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        "Unsupported module item: wire",
    );
}

#[test]
fn unsupported_nature_properties_are_not_silently_discarded() {
    assert_unsupported(
        r#"
nature custom_potential;
    access = CP;
    unsupported_property = 1;
endnature
module invalid_nature;
endmodule
"#,
        "Unsupported nature property: unsupported_property",
    );
}

#[test]
fn analog_functions_accept_direct_body_statements() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module direct_function_body(p, n);
    inout p, n;
    electrical p, n;

    analog function real square;
        input x;
        square = x * x;
    endfunction

    analog I(p, n) <+ square(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device.try_evaluate().expect("direct function evaluates"),
        vec![4.0]
    );
}
