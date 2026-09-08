use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

#[test]
fn real_modulo_preserves_both_operand_jacobians() {
    for body in ["I(p,n)<+V(p,n)%V(m,n);", "r=V(p,n)%V(m,n); I(p,n)<+r;"] {
        let fixture = DeviceFixture::compile(&format!(
            "module remainder(p,n,m); inout p,n,m; electrical p,n,m; real r; analog begin {body} end endmodule"
        ));
        let mut device = fixture.device("R", &[1, 0, 2]);
        for a in [-10.0_f64, -1.25, 1.25, 10.0] {
            for b in [-3.75_f64, 3.75] {
                device.update_voltages(&[a, b]);
                assert_eq!(device.try_evaluate().unwrap()[0], a % b);
                let mut slopes = [0.0; 2];
                device
                    .try_stamp(
                        &[a, b],
                        |row, col, value| {
                            if row == 0 {
                                slopes[col] += value;
                            }
                        },
                        |_, _| {},
                    )
                    .unwrap();
                let expected = [1.0, -(a / b).trunc()];
                for axis in 0..2 {
                    assert_eq!(
                        slopes[axis], expected[axis],
                        "{body}, a={a}, b={b}, axis={axis}"
                    );
                    let mut plus = [a, b];
                    let mut minus = plus;
                    let step = 1.0e-5;
                    plus[axis] += step;
                    minus[axis] -= step;
                    let difference = (plus[0] % plus[1] - minus[0] % minus[1]) / (2.0 * step);
                    assert!((slopes[axis] - difference).abs() < 1.0e-9);
                }
            }
        }
    }
}

#[test]
fn real_modulo_nested_ddx_and_readback_preserve_piecewise_slopes() {
    for (left, right, left_scale, right_scale) in [
        (
            "10.0+V(p,n)*V(p,n)*V(p,n)",
            "2.0+V(p,n)*V(p,n)*V(p,n)",
            1.0,
            1.0,
        ),
        ("10.0", "2.0+V(p,n)*V(p,n)*V(p,n)", 0.0, 1.0),
        ("10.0+V(p,n)*V(p,n)*V(p,n)", "2.0", 1.0, 0.0),
    ] {
        let fixture = DeviceFixture::compile(&format!(
            "module remainder_ddx(p,n); inout p,n; electrical p,n; real a,b,r,d1,d2,d3; analog begin a={left}; b={right}; r=a%b; d1=ddx(r,V(p,n)); d2=ddx(d1,V(p,n)); d3=ddx(d2,V(p,n)); I(p,n)<+d2; end endmodule"
        ));
        let mut device = fixture.device("R", &[1, 0]);
        for v in [-0.75_f64, 0.5, 1.25] {
            let a = 10.0 + left_scale * v.powi(3);
            let b = 2.0 + right_scale * v.powi(3);
            let scale = left_scale - (a / b).trunc() * right_scale;
            device.update_voltages(&[v]);
            assert!((device.try_evaluate().unwrap()[0] - scale * 6.0 * v).abs() < 1.0e-10);
            let mut slope = 0.0;
            device
                .try_stamp(
                    &[v],
                    |row, col, value| {
                        if row == 0 && col == 0 {
                            slope += value;
                        }
                    },
                    |_, _| {},
                )
                .unwrap();
            assert!(
                (slope - scale * 6.0).abs() < 1.0e-10,
                "{left} % {right} at {v}: {slope}"
            );
            fixture.observe(&mut device);
            for (name, expected) in [
                ("r", a % b),
                ("d1", scale * 3.0 * v * v),
                ("d2", scale * 6.0 * v),
                ("d3", scale * 6.0),
            ] {
                let actual = device.variable(name).unwrap();
                assert!(
                    (actual - expected).abs() < 1.0e-10,
                    "{name} at {v}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn real_modulo_reactive_jacobian_preserves_both_operands() {
    let fixture = DeviceFixture::compile(
        "module reactive_remainder(p,n,m); inout p,n,m; electrical p,n,m; analog I(p,n)<+ddt(V(p,n)%V(m,n)); endmodule",
    );
    let mut device = fixture.device("R", &[1, 0, 2]);
    device.try_set_analysis_type(1).unwrap();
    for (a, b) in [(-10.0_f64, 3.75_f64), (10.0, -3.75), (1.25, 3.75)] {
        let mut slopes = [0.0; 2];
        device
            .try_stamp_reactive(&[a, b], |row, col, value| {
                if row == 0 {
                    slopes[col] += value;
                }
            })
            .unwrap();
        assert_eq!(slopes, [1.0, -(a / b).trunc()]);
    }
}

#[test]
fn real_modulo_constant_divisor_keeps_finite_slope_when_quotient_overflows() {
    let fixture = DeviceFixture::compile(
        "module tiny_divisor(p,n); inout p,n; electrical p,n; parameter real b=1.0e-300; analog I(p,n)<+V(p,n)%b; endmodule",
    );
    let mut device = fixture.device("R", &[1, 0]);
    let mut slope = 0.0;
    device
        .try_stamp(
            &[1.0e300],
            |row, col, value| {
                if row == 0 && col == 0 {
                    slope += value;
                }
            },
            |_, _| {},
        )
        .unwrap();
    assert_eq!(slope, 1.0);
}

#[test]
fn real_modulo_zero_divisor_reports_an_evaluation_error() {
    for numerator in ["V(p,n)", "0.0"] {
        let fixture = DeviceFixture::compile(&format!(
            "module zero_divisor(p,n); inout p,n; electrical p,n; parameter real b=0.0; analog I(p,n)<+{numerator}%b; endmodule"
        ));
        let mut device = fixture.device("R", &[1, 0]);
        device.update_voltages(&[1.0]);
        assert!(device.try_evaluate().is_err(), "{numerator} % 0 must fail");
    }
}

fn evaluate(device: &mut VerilogADevice) -> f64 {
    device.update_voltages(&[0.0]);
    device.try_evaluate().expect("evaluation succeeds")[0]
}

#[test]
fn tiny_nonzero_reals_use_exact_equality_and_truth_rules() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module exact_real_semantics(p, n);
    inout p, n;
    electrical p, n;
    parameter real eps = 1.0e-16;
    analog I(p, n) <+ ((eps == 0.0) ? 1.0 : 0.0) + (eps ? 10.0 : 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device).to_bits(), 10.0_f64.to_bits());
}

#[test]
fn positive_and_negative_zero_are_false_and_equal() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module signed_zero_semantics(p, n);
    inout p, n;
    electrical p, n;
    parameter real value = -0.0;
    analog I(p, n) <+ (value ? 100.0 : 1.0)
                       + ((value == 0.0) ? 10.0 : 0.0)
                       + ((value != 0.0) ? 1000.0 : 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device).to_bits(), 11.0_f64.to_bits());
}

#[test]
fn operator_chains_preserve_floating_point_association() {
    let chain = " + 1.0e16 + 1.0 - 1.0e16".repeat(16);
    let model = DeviceFixture::compile(&format!(
        "module operator_association(p, n);\n\
         inout p, n; electrical p, n;\n\
         analog I(p, n) <+ V(p, n){chain};\nendmodule\n"
    ));
    let mut device = model.device("A1", &[1, 0]);

    // Each authored left-associated group loses the unit before subtraction.
    // Reassociating constants across the chain would produce sixteen instead.
    assert_eq!(evaluate(&mut device).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn operator_materialization_preserves_left_to_right_function_effects() {
    let model = DeviceFixture::compile(
        r#"
module operator_effects(p, n);
    inout p, n;
    electrical p, n;
    integer count;
    analog function integer bump;
        inout counter;
        integer counter;
        begin
            counter = counter + 1;
            bump = counter;
        end
    endfunction
    analog begin
        count = 0;
        I(p, n) <+ bump(count) - bump(count);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device).to_bits(), (-1.0_f64).to_bits());
}
