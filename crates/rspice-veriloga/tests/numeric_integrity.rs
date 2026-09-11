mod support;

use rspice_veriloga::vm::VmError;
use support::DeviceFixture;

#[test]
fn flow_probes_preserve_simultaneous_equations_and_jacobians() {
    for (declarations, body, expected_pp, expected_qp) in [
        ("", "I(p,n)<+2*V(p,n); I(q,n)<+3*I(p,n);", 2.0, 6.0),
        ("", "I(p,n)<+2*V(p,n); I(q,n)<+3*I(<p>);", 2.0, 6.0),
        ("", "I(p,n)<+2*V(p,n); I(q,n)<+3*I(<n>);", 2.0, -1.5),
        (
            "",
            "I(p,n)<+V(p,n)+0.1*I(q,n); I(q,n)<+0.2*I(p,n);",
            1.0 / 0.98,
            0.2 / 0.98,
        ),
        (
            "parameter integer enabled=0;",
            "if(enabled) I(p,n)<+10*V(p,n); I(p,n)<+2*V(p,n); I(q,n)<+3*I(p,n);",
            2.0,
            6.0,
        ),
        (
            "parameter integer enabled=1;",
            "if(enabled) I(p,n)<+10*V(p,n); I(p,n)<+2*V(p,n); I(q,n)<+3*I(p,n);",
            12.0,
            36.0,
        ),
        ("", "I(q,n)<+3*I(p,n); I(p,n)<+2*V(p,n);", 2.0, 6.0),
        ("", "I(p,n)<+V(p,n); I(p,n)<+0.1*I(p,n);", 1.0 / 0.9, 0.0),
        (
            "",
            "I(p,n)<+V(p,n); I(p,n)<+2*V(p,n); I(q,n)<+3*I(p,n);",
            3.0,
            9.0,
        ),
        ("", "I(p,n)<+2*V(p,n); I(q,n)<+3*I(n,p);", 2.0, -6.0),
        (
            "",
            "I(p,n)<+2*V(p,n); I(q,n)<+ddx(I(p,n)*I(p,n),I(p,n));",
            2.0,
            4.0,
        ),
        (
            "",
            "I(p,n)<+2*V(p,n); I(q,n)<+ddx(I(p,n)*I(p,n),I(n,p));",
            2.0,
            -4.0,
        ),
        (
            "branch (p,n) a,b;",
            "I(a)<+2*V(p,n); I(b)<+5*V(p,n); I(q,n)<+3*I(a)-I(b);",
            7.0,
            1.0,
        ),
    ] {
        let fixture = DeviceFixture::compile(&format!(
            "module probes(p,n,q); inout p,n,q; electrical p,n,q; {declarations} analog begin {body} end endmodule"
        ));
        assert!(!fixture.model.internal_state_nodes.is_empty(), "{body}");
        let mut device = fixture.device("PROBES", &[1, 0, 2]);
        let dimension = 2 + fixture.model.internal_nodes;
        device.set_internal_node_indices(&(3..=dimension).collect::<Vec<_>>());
        let mut matrix = vec![vec![0.0; dimension]; dimension];
        let mut rhs = vec![0.0; dimension];
        let mut bias = vec![0.0; dimension];
        bias[0] = 1.0;
        device
            .try_stamp(
                &bias,
                |r, c, value| matrix[r][c] += value,
                |r, value| rhs[r] += value,
            )
            .unwrap();
        // Eliminate the private current unknowns to compare the resulting
        // electrical Jacobian with the analytic source equations.
        for pivot in (2..dimension).rev() {
            assert!(matrix[pivot][pivot].abs() > 0.1, "{body}: {matrix:?}");
            let (rows, tail) = matrix.split_at_mut(pivot);
            let pivot_row = &tail[0];
            for (row, coefficients) in rows.iter_mut().enumerate() {
                let factor = coefficients[pivot] / pivot_row[pivot];
                for (value, pivot_value) in
                    coefficients[..pivot].iter_mut().zip(&pivot_row[..pivot])
                {
                    *value -= factor * pivot_value;
                }
                rhs[row] -= factor * rhs[pivot];
            }
        }
        assert!(
            (matrix[0][0] - expected_pp).abs() < 1e-12,
            "{body}: {matrix:?}"
        );
        assert!(
            (matrix[1][0] - expected_qp).abs() < 1e-12,
            "{body}: {matrix:?}"
        );
        assert!(
            rhs.iter().all(|value| value.abs() < 1e-12),
            "{body}: {rhs:?}"
        );
    }
}

#[test]
fn pure_flow_probe_enforces_zero_potential_without_inventing_a_current() {
    let fixture = DeviceFixture::compile(
        "module sensor(p,n,q); inout p,n,q; electrical p,n,q; analog I(q,n)<+3*I(p,n); endmodule",
    );
    let mut device = fixture.device("SENSOR", &[1, 0, 2]);
    device.set_internal_node_indices(&[3]);
    let mut matrix = [[0.0; 3]; 3];
    let mut rhs = [0.0; 3];
    device
        .try_stamp(
            &[0.0, 0.0, -2.0],
            |r, c, v| matrix[r][c] += v,
            |r, v| rhs[r] += v,
        )
        .unwrap();
    assert_eq!(
        matrix,
        [[0.0, 0.0, -1.0], [0.0, 0.0, -3.0], [-1.0, 0.0, 0.0]]
    );
    assert_eq!(rhs, [0.0; 3]);
    let error = rspice_veriloga::VerilogACompiler::default().compile_runtime("module illegal(p,n,q); inout p,n,q; electrical p,n,q; analog I(q,n)<+I(p,n)+V(p,n); endmodule", None).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("cannot probe both flow and potential")
    );
}

#[test]
fn flow_probe_feedback_preserves_limiter_correction_and_transient_history() {
    for transient in [false, true] {
        let source = if transient {
            "ddt(V(p,n))+0.1*I(p,n)"
        } else {
            "V(p,n)+0.1*pow($limit(I(p,n),0.25),2)"
        };
        let fixture = DeviceFixture::compile(&format!(
            "module feedback(p,n,q); inout p,n,q; electrical p,n,q; analog begin I(q,n)<+3*I(p,n); I(p,n)<+{source}; end endmodule"
        ));
        for multiplicity in [1.0, 3.0] {
            let mut device = fixture.device("FEEDBACK", &[1, 0, 2]);
            device.set_internal_node_indices(&[3]);
            device.set_multiplicity(multiplicity);
            if transient {
                device.set_analysis_type(2);
                device.set_timestep(1.0);
            }
            device
                .try_stamp(&[0.0; 3], |_, _, _| {}, |_, _| {})
                .unwrap();
            if transient {
                device.try_advance_state().unwrap();
                device.set_time(1.0);
                device
                    .try_stamp(&[1.0, 0.0, -1.0 / 0.9], |_, _, _| {}, |_, _| {})
                    .unwrap();
                device.try_advance_state().unwrap();
                device.set_time(2.0);
            }
            let mut matrix = [[0.0; 3]; 3];
            let mut rhs = [0.0; 3];
            device
                .try_stamp(
                    &[if transient { 2.0 } else { 1.0 }, 0.0, -2.0],
                    |r, c, value| matrix[r][c] += value,
                    |r, value| rhs[r] += value,
                )
                .unwrap();
            // Eliminate the current equation to check the physical Newton
            // companion against the analytic feedback equation.
            let denominator = if transient { 0.9 } else { 0.95 };
            let expected_rhs = if transient { 1.0 } else { 0.00625 };
            for (row, gain) in [(0, 1.0), (1, 3.0)] {
                let factor = matrix[row][2] / matrix[2][2];
                let conductance = matrix[row][0] - factor * matrix[2][0];
                let companion = rhs[row] - factor * rhs[2];
                let scale = multiplicity * gain / denominator;
                assert!(
                    (conductance - scale).abs() < 1e-12,
                    "{source}, m={multiplicity}: {matrix:?}"
                );
                assert!(
                    (companion - scale * expected_rhs).abs() < 1e-12,
                    "{source}, m={multiplicity}: {rhs:?}"
                );
            }
        }
    }
}

#[test]
fn tiny_nonzero_conductance_is_retained_in_the_jacobian() {
    let model = DeviceFixture::compile(
        "module tiny(p,n); inout p,n; electrical p,n; analog I(p,n)<+1e-40*V(p,n); endmodule",
    );
    let mut device = model.device("TINY", &[1, 0]);
    let mut conductance = 0.0;
    let mut rhs = 0.0;
    device
        .try_stamp(&[1.0], |_, _, g| conductance += g, |_, v| rhs += v)
        .unwrap();
    assert_eq!(conductance, 1e-40);
    assert_eq!(rhs, 0.0);
}

fn assert_numeric_error(error: VmError, expected_context: &str) {
    match error {
        VmError::InvalidNumericResult(message) => assert!(
            message.contains(expected_context),
            "expected numeric error containing {expected_context:?}, got {message:?}"
        ),
        other => panic!("expected invalid numeric result, got {other:?}"),
    }
}

#[test]
fn nonfinite_contributions_are_reported_instead_of_zeroed() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_value(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sqrt(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[-1.0]);

    let error = device
        .try_evaluate()
        .expect_err("NaN contribution must be a runtime diagnostic");
    assert_numeric_error(error, "contribution 0");
}

#[test]
fn nonfinite_jacobians_are_reported_instead_of_dropped() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sqrt(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_stamp(&[0.0], |_, _, _| {}, |_, _| {})
        .expect_err("infinite derivative must be a runtime diagnostic");
    assert_numeric_error(error, "Jacobian 0:0");
}

#[test]
fn nonfinite_reactive_jacobians_are_reported_instead_of_dropped() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_reactive_jacobian(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(sqrt(V(p, n)));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_stamp_reactive(&[0.0], |_, _, _| {})
        .expect_err("infinite reactive derivative must be a runtime diagnostic");
    assert_numeric_error(error, "reactive Jacobian 0:0");
}

#[test]
fn invalid_noise_power_is_reported_instead_of_suppressed() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_noise(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ white_noise(V(p, n), "invalid");
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_noise_sources(&[-1.0])
        .expect_err("negative noise power must be a runtime diagnostic");
    assert_numeric_error(error, "negative value");
}

#[test]
fn nonfinite_noise_exponents_are_reported() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module invalid_noise_exponent(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ flicker_noise(1.0, sqrt(V(p, n)), "invalid");
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    let error = device
        .try_noise_sources(&[-1.0])
        .expect_err("NaN noise exponent must be a runtime diagnostic");
    assert_numeric_error(error, "exponent");
}
