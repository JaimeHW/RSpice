use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

#[test]
fn integer_constants_and_defaults_agree_with_runtime_arithmetic() {
    for (expression, expected) in [
        ("5/2", 2.0),
        ("-5/2", -2.0),
        ("2147483647+1", -2147483648.0),
        ("2**31", -2147483648.0),
        ("2**-1", 0.0),
        ("(-1)**-3", -1.0),
        ("-(-2147483648)", -2147483648.0),
        ("8.0+(1/2)", 8.0),
        ("1/2.0", 0.5),
        ("(64'sd1+1)+2147483647", 2147483649.0),
        ("40'sh7fffffffff+1", -549755813888.0),
        ("(9007199254740992+1)>9007199254740992", 1.0),
    ] {
        for body in [
            format!("analog I(p,n)<+({expression});"),
            format!("parameter real result={expression}; analog I(p,n)<+result;"),
            format!("localparam real result={expression}; analog I(p,n)<+result;"),
        ] {
            let fixture = DeviceFixture::compile(&format!(
                "module constants(p,n); inout p,n; electrical p,n; {body} endmodule"
            ));
            let mut device = fixture.device("X", &[1, 0]);
            device.update_voltages(&[1.0]);
            assert_eq!(device.try_evaluate().unwrap()[0], expected, "{body}");
        }
    }
}

#[test]
fn integer_division_in_parameter_bounds_uses_the_effective_parameter_vector() {
    let fixture = DeviceFixture::compile(
        "module bounds(p,n); inout p,n; electrical p,n; parameter real value=2 from [0:limit/2]; parameter integer limit=5; analog I(p,n)<+value; endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    device.try_set_parameter("value", 2.25).unwrap();
    assert!(device.try_resolve_parameter_defaults().is_err());
    device.try_set_parameter("limit", 6.0).unwrap();
    device.try_resolve_parameter_defaults().unwrap();
}

#[test]
fn integer_domain_failures_are_reported_only_for_taken_expressions() {
    for operator in ["/", "%"] {
        let fixture = DeviceFixture::compile(&format!(
            "module domains(p,n); inout p,n; electrical p,n; integer a,b; analog begin a=5; b=V(p,n); I(p,n)<+(V(p,n)<0 ? 3 : a {operator} b); end endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        for (v, expected) in [
            (-1.0, Some(3.0)),
            (0.0, None),
            (2.0, Some(if operator == "/" { 2.0 } else { 1.0 })),
        ] {
            device.update_voltages(&[v]);
            let result = device.try_evaluate();
            if let Some(expected) = expected {
                assert_eq!(result.unwrap()[0], expected);
            } else {
                assert!(result.is_err(), "{operator} at {v}");
            }
        }
    }
}

#[test]
fn integer_arithmetic_keeps_its_type_inside_real_expressions() {
    for (operator, left, right, expected) in [
        ("/", 5.0, 2.0, 2.0),
        ("/", -5.0, 2.0, -2.0),
        ("/", 5.0, -2.0, -2.0),
        ("+", 2147483647.0, 1.0, -2147483648.0),
        ("-", -2147483648.0, 1.0, 2147483647.0),
        ("*", 2147483647.0, 2.0, -2.0),
        ("**", 2.0, 31.0, -2147483648.0),
        ("**", 2.0, -1.0, 0.0),
        ("**", -1.0, -3.0, -1.0),
        ("**", -1.0, -2.0, 1.0),
        ("%", -5.0, 2.0, -1.0),
    ] {
        let fixture = DeviceFixture::compile(&format!(
            "module typed_arithmetic(p,q,n); inout p,q,n; electrical p,q,n; integer a,b; analog begin a=V(p,n); b=V(q,n); I(p,n)<+(a {operator} b)+0.25*V(p,n); end endmodule"
        ));
        let mut device = fixture.device("X", &[1, 2, 0]);
        device.update_voltages(&[left, right]);
        assert_eq!(
            device.try_evaluate().unwrap()[0],
            expected + 0.25 * left,
            "{left} {operator} {right}"
        );
        let mut slope = 0.0;
        device
            .try_stamp(
                &[left, right],
                |row, column, value| {
                    if row == 0 && column == 0 {
                        slope += value;
                    }
                },
                |_, _| {},
            )
            .unwrap();
        assert_eq!(
            slope, 0.25,
            "integer arithmetic must not acquire a real tangent"
        );
    }
}

#[test]
fn dependent_integer_division_truncates_before_real_parameter_assignment() {
    let fixture = DeviceFixture::compile(
        "module typed_default(p,n); inout p,n; electrical p,n; parameter integer a=5,b=2; parameter real quotient=a/b; analog I(p,n)<+quotient*V(p,n); endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for (numerator, expected) in [(5.0, 2.0), (-5.0, -2.0), (7.0, 3.0)] {
        device.try_set_parameter("a", numerator).unwrap();
        device.try_resolve_parameter_defaults().unwrap();
        device.update_voltages(&[1.0]);
        assert_eq!(device.try_evaluate().unwrap()[0], expected);
    }
}

#[test]
fn integer_function_initializers_snapshot_inputs_and_restore_block_scope() {
    let fixture = DeviceFixture::compile(
        r#"
module integer_scopes(p,n);
inout p,n; electrical p,n;
analog function real snapshot;
    input x; real x;
    integer saved=x;
    begin
        x=x+10.0;
        begin : inner integer saved=x; x=saved+0.25; end
        snapshot=saved+x;
    end
endfunction
analog I(p,n)<+snapshot(V(p,n));
endmodule
"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-1.5_f64, 0.75, 2.5] {
        device.update_voltages(&[v]);
        assert_eq!(
            device.try_evaluate().unwrap()[0],
            v.round() + (v + 10.0).round() + 0.25
        );
    }
}

#[test]
fn integer_conversion_failures_are_checked_and_untaken_writes_are_skipped() {
    let fixture = DeviceFixture::compile(
        "module checked_integer(p,n); inout p,n; electrical p,n; integer q; analog begin q=0; if(V(p,n)>0.0) q=V(p,n); I(p,n)<+q; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for (v, fails) in [(1.5, false), (2147483647.5, true), (-2147483649.0, false)] {
        device.update_voltages(&[v]);
        assert_eq!(device.try_evaluate().is_err(), fails, "at {v}");
        if !fails {
            fixture.observe(&mut device);
            assert_eq!(
                device.variable("q"),
                Some(if v > 0.0 { v.round() } else { 0.0 })
            );
        }
    }
}

#[test]
fn nested_conditional_integer_array_writes_skip_invalid_conversions() {
    let fixture = DeviceFixture::compile(
        "module checked_array(p,n); inout p,n; electrical p,n; integer q[0:0],idx; analog begin idx=0; q[idx]=0; if(V(p,n)>0.0) begin if(V(p,n)<10.0) q[idx]=V(p,n); else q[idx]=3.0; end I(p,n)<+q[idx]; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for (v, expected) in [(1.5, 2.0), (3e9, 3.0), (-3e9, 0.0), (2.5, 3.0)] {
        device.update_voltages(&[v]);
        assert_eq!(device.try_evaluate().unwrap()[0], expected, "at {v}");
        fixture.observe(&mut device);
    }
}

#[test]
fn integer_assignments_round_values_and_drop_their_tangents() {
    let fixture = DeviceFixture::compile(
        "module integer_store(p,n); inout p,n; electrical p,n; integer rounded; real reported; analog begin rounded=V(p,n); reported=rounded+0.25*V(p,n); I(p,n)<+reported; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-2.5_f64, -1.5, -0.5, -0.49, 0.49, 0.5, 1.25, 1.5, 2.5] {
        device.update_voltages(&[v]);
        assert_eq!(
            device.try_evaluate().unwrap()[0],
            v.round() + 0.25 * v,
            "at {v}"
        );
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
        assert_eq!(slope, 0.25, "at {v}");
        fixture.observe(&mut device);
        assert_eq!(device.variable("rounded"), Some(v.round()));
    }
}

#[test]
fn integer_array_assignments_convert_constant_and_runtime_indices() {
    for target in ["q[-1]", "q[index]"] {
        let fixture = DeviceFixture::compile(&format!(
            "module integer_array(p,n); inout p,n; electrical p,n; integer q[-1:1],index; analog begin index=-1; {target}=V(p,n); I(p,n)<+q[-1]; end endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        for v in [-1.5_f64, 0.75, 2.5] {
            device.update_voltages(&[v]);
            assert_eq!(
                device.try_evaluate().unwrap()[0],
                v.round(),
                "{target} at {v}"
            );
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
            assert_eq!(slope, 0.0);
        }
    }
}

#[test]
fn integer_initializers_and_localparams_use_assignment_conversion() {
    let fixture = DeviceFixture::compile(
        "module integer_initializers(p,n); inout p,n; electrical p,n; localparam integer limit=2.5; integer startup=1.5; integer a[0:1]='{0.5,-0.5}; analog begin : scope integer local=V(p,n); I(p,n)<+startup+limit+a[0]-a[1]+local; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-1.5_f64, 0.75, 2.5] {
        device.update_voltages(&[v]);
        assert_eq!(device.try_evaluate().unwrap()[0], 7.0 + v.round(), "at {v}");
    }
}

#[test]
fn integer_function_arguments_outputs_and_returns_convert_at_each_write() {
    let fixture = DeviceFixture::compile(
        r#"
module integer_function(p,n);
inout p,n; electrical p,n;
real out, result;
analog function real from_integer;
    input x; integer x;
    from_integer=x+0.25;
endfunction
analog function integer to_integer;
    input x; real x;
    to_integer=x;
endfunction
analog function real output_integer;
    input x; real x;
    output y; integer y;
    begin y=x; output_integer=y+0.125; end
endfunction
analog begin
    result=from_integer(V(p,n))+to_integer(V(p,n))+output_integer(V(p,n),out);
    I(p,n)<+result+out;
end
endmodule
"#,
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-1.5_f64, 0.75, 2.5] {
        device.update_voltages(&[v]);
        assert_eq!(
            device.try_evaluate().unwrap()[0],
            4.0 * v.round() + 0.375,
            "at {v}"
        );
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
        assert_eq!(slope, 0.0);
    }
}

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
