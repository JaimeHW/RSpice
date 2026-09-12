//! Numerical verification of compiled device evaluation and MNA stamping.
//!
//! These tests check the companion-model math against hand-derived values:
//! the engine solves A*V = z, so a device must stamp G into all rows of its
//! KCL pair and -/+ Ieq = -/+(I - G*V) into the RHS.

#[cfg(not(feature = "native"))]
use rspice_veriloga::codegen::{BytecodeProgram, Instruction};
use rspice_veriloga::device::VerilogADevice;
use std::collections::HashMap;

mod support;

use support::DeviceFixture;

fn compile(source: &str) -> DeviceFixture {
    DeviceFixture::compile(source)
}

#[test]
fn thermal_voltage_spellings_preserve_precision_and_subnormal_range() {
    // Oracles round the exact SI rational 1380649/16021766340 times each
    // binary64 input. Multiplication by the rounded scale may differ by one ULP.
    let cases = [
        (5e-324, 0x0000000000000000_u64),
        (1e-318, 0x0000000000000011_u64),
        (1e-310, 0x0000000067f5e2f6_u64),
        (1e-300, 0x00ce41a9117cd314_u64),
        (2.2250738585072014e-308, 0x0000005a5bfa538b_u64),
        (1.0, 0x3f1696fe94e2cf9e_u64),
        (300.15, 0x3f9a7c55c97686ec_u64),
        (1e+300, 0x7d60dda58c073969_u64),
        (1.7976931348623157e+308, 0x7f1696fe94e2cf9d_u64),
    ];
    assert_eq!(
        rspice_veriloga_runtime::THERMAL_VOLTAGE_PER_K.to_bits(),
        0x3f1696fe94e2cf9e
    );
    for expression in [
        "$vt()",
        "$vt",
        "$thermal_vt()",
        "$vt($temperature)",
        "$thermal_vt($temperature)",
        "$vt(V(p))",
    ] {
        for assigned in [false, true] {
            let statement = if assigned {
                format!("y={expression}; I(p)<+y;")
            } else {
                format!("I(p)<+{expression};")
            };
            let fixture = compile(&format!(
                "module thermal_range(p); inout p; electrical p; real y; analog begin {statement} end endmodule"
            ));
            let mut device = fixture.device("THERMAL", &[1]);
            for (temperature, expected) in cases {
                device.try_set_temperature(temperature).unwrap();
                device.update_voltages(&[temperature]);
                let actual = device.try_evaluate().unwrap()[0];
                assert!(
                    actual.to_bits().abs_diff(expected) <= 1,
                    "{expression}, assigned={assigned}, T={temperature:e}: {actual:e}, expected bits {expected:x}"
                );
                let runtime =
                    rspice_veriloga_runtime::GeneratedEvalContext::new(&[], temperature, 1);
                let mut vm = rspice_veriloga::vm::VmContext::new(1);
                vm.temperature = temperature;
                assert_eq!(
                    actual.to_bits(),
                    runtime.thermal_voltage().to_bits(),
                    "{expression}, T={temperature:e}"
                );
                assert_eq!(actual.to_bits(), vm.vt().to_bits());
                if assigned {
                    fixture.observe(&mut device);
                    assert_eq!(device.variable("y").unwrap().to_bits(), actual.to_bits());
                }
            }
        }
    }
}

#[test]
fn ddx_laplace_system_quantities_preserve_values_jacobians_and_readback() {
    use rspice_veriloga::vm::IntegrationCoefficients;

    let thermal_scale = 8.617333262145177e-5;
    let ambient_vt = 330.0 * thermal_scale;
    // A `None` factor means the query reports the current time. `$abstime` is
    // seconds-valued; `$realtime` reports the same instant in the owning
    // module's time unit (VAMS-2023 9.10), which is the default 1 ns timing
    // here because these sources declare no `timescale.
    let default_time_unit = rspice_veriloga::time_scale::ModuleTimeScale::default()
        .unit_seconds()
        .expect("the default module timing is valid");
    for (input, factor) in [
        ("$abstime*V(p)*V(p)*V(p)", None),
        ("$realtime*V(p)*V(p)*V(p)", None),
        ("$temperature*V(p)*V(p)*V(p)", Some(330.0)),
        ("$mfactor*V(p)*V(p)*V(p)", Some(1.0)),
        ("$param_given(gain)*V(p)*V(p)*V(p)", Some(1.0)),
        ("$port_connected(p)*V(p)*V(p)*V(p)", Some(1.0)),
        ("analysis(\"tran\")*V(p)*V(p)*V(p)", Some(1.0)),
        ("$vt()*V(p)*V(p)*V(p)", Some(ambient_vt)),
        ("$thermal_vt()*V(p)*V(p)*V(p)", Some(ambient_vt)),
        ("$vt(V(p))*V(p)*V(p)", Some(thermal_scale)),
        ("$thermal_vt(V(p)*V(p))*V(p)", Some(thermal_scale)),
        ("$vt(V(p)*V(p)*V(p))", Some(thermal_scale)),
    ] {
        for order in [1, 2] {
            let mut value = format!("laplace_nd({input}, '{{1.0,0.5}}, '{{1.0,0.25}})");
            for _ in 0..order {
                value = format!("ddx({value},V(p))");
            }
            let fixture = compile(&format!(
                "module query_derivative(p); inout p; electrical p; parameter real gain=2; real y;
                analog begin y={value}; I(p)<+y; end endmodule"
            ));
            let mut device = fixture.device("QUERY", &[1]);
            device.set_temperature(330.0);
            assert!(device.set_parameter("gain", 3.0));
            device.update_voltages(&[0.0]);
            device.try_evaluate().unwrap();
            device.advance_state();
            device.set_analysis_type(2);
            let mut time = 0.0;
            for (voltage, timestep) in [(0.25_f64, 0.125), (0.4, 0.25)] {
                time += timestep;
                device.set_time(time);
                device.set_timestep(timestep);
                let a = 1.0 / timestep;
                device.set_integration_coefficients(IntegrationCoefficients {
                    active: true,
                    derivative_scale: a,
                    previous_value_scale: a,
                    older_value_scale: 0.0,
                    previous_derivative_scale: 0.0,
                });
                let reported_time = if input.starts_with("$realtime") {
                    time / default_time_unit
                } else {
                    time
                };
                let scale = factor.unwrap_or(reported_time) * (1.0 + 0.5 * a) / (1.0 + 0.25 * a);
                let (expected, slope) = if order == 1 {
                    (3.0 * scale * voltage.powi(2), 6.0 * scale * voltage)
                } else {
                    (6.0 * scale * voltage, 6.0 * scale)
                };
                device.update_voltages(&[voltage]);
                let current = device.try_evaluate().unwrap()[0];
                let (matrix, _) = collect_stamps(&mut device, &[voltage]);
                assert!(
                    (current / expected - 1.0).abs() < 1e-12,
                    "{value}: {current} != {expected}"
                );
                assert!(
                    (matrix[&(0, 0)] / slope - 1.0).abs() < 1e-12,
                    "{value}: {matrix:?} != {slope}"
                );
                fixture.observe(&mut device);
                assert!(
                    (device.variable("y").unwrap() / expected - 1.0).abs() < 1e-12,
                    "{value}"
                );
                device.advance_state();
            }
        }
    }
}

#[test]
fn ddx_laplace_thermal_arguments_keep_quotient_dependencies() {
    let k = 8.617333262145177e-5;
    for (input, scale) in [
        ("V(p)*V(p)*V(p)/$vt(V(p))", 1.0 / k),
        ("$vt(V(p)*V(p)*V(p))/V(p)", k),
    ] {
        let fixture = compile(&format!(
            "module thermal_quotient(p); inout p; electrical p;
            analog I(p)<+ddx(laplace_nd({input}, '{{1.0}}, '{{1.0,0.25}}),V(p)); endmodule"
        ));
        let mut device = fixture.device("QUOTIENT", &[1]);
        for voltage in [0.25, 0.5, 2.0] {
            device.update_voltages(&[voltage]);
            let current = device.try_evaluate().unwrap()[0];
            let (matrix, _) = collect_stamps(&mut device, &[voltage]);
            assert!(
                (current / (2.0 * scale * voltage) - 1.0).abs() < 1e-12,
                "{input}: {current}"
            );
            assert!(
                (matrix[&(0, 0)] / (2.0 * scale) - 1.0).abs() < 1e-12,
                "{input}: {matrix:?}"
            );
        }
    }
}

#[test]
fn ddx_laplace_thermal_arguments_keep_mixed_electrical_derivatives() {
    let k = 8.617333262145177e-5;
    for mixed in [false, true] {
        let first = "ddx(laplace_nd($vt(V(p)*V(q))*V(p), '{1.0,0.5}, '{1.0,0.25}),V(p))";
        let value = if mixed {
            format!("ddx({first},V(q))")
        } else {
            first.to_owned()
        };
        let fixture = compile(&format!(
            "module thermal_mixed(p,q); inout p,q; electrical p,q; real y;
            analog begin y={value}; I(p)<+y; end endmodule"
        ));
        let mut device = fixture.device("MIXED", &[1, 2]);
        for (p, q) in [(0.25, 0.75), (2.0, 0.125), (0.5, 0.5)] {
            let (expected, dp, dq) = if mixed {
                (2.0 * k * p, 2.0 * k, 0.0)
            } else {
                (2.0 * k * p * q, 2.0 * k * q, 2.0 * k * p)
            };
            device.update_voltages(&[p, q]);
            let current = device.try_evaluate().unwrap()[0];
            let (matrix, _) = collect_stamps(&mut device, &[p, q]);
            assert!(
                (current / expected - 1.0).abs() < 1e-12,
                "{value}: {current}"
            );
            assert!(
                (matrix[&(0, 0)] / dp - 1.0).abs() < 1e-12,
                "{value}: {matrix:?}"
            );
            let actual_dq = matrix.get(&(0, 1)).copied().unwrap_or(0.0);
            assert!(
                (actual_dq - dq).abs() <= 1e-12 * dq.abs(),
                "{value}: {matrix:?}"
            );
            fixture.observe(&mut device);
            assert!(
                (device.variable("y").unwrap() / expected - 1.0).abs() < 1e-12,
                "{value}"
            );
        }
    }
}

#[test]
fn ddx_laplace_repeated_state_actions_preserve_values_and_jacobians() {
    for (expression, expected_current, expected_slope) in [
        ("laplace_nd(V(p)*V(p), '{1.0,0.5}, '{1.0,0.25})", 0.5, 2.0),
        ("laplace_zp(V(p)*V(p), '{-2.0,0.0}, '{-4.0,0.0})", 0.5, 2.0),
        (
            "laplace_nd(V(p)*V(p), '{1.0,0.5}, '{1.0,0.25}) + laplace_nd(V(q), '{3.0}, '{1.0,0.5})",
            0.5,
            2.0,
        ),
        (
            "laplace_nd(V(p)*V(p), '{1.0,0.5}, '{1.0,0.25}) * laplace_nd(V(p), '{1.0}, '{1.0,0.5})",
            0.1875,
            1.5,
        ),
        (
            "laplace_nd(laplace_nd(V(p)*V(p), '{1.0}, '{1.0,0.5}), '{1.0}, '{1.0,0.25})",
            0.5,
            2.0,
        ),
    ] {
        for assigned in [false, true] {
            let value = format!("ddx({expression},V(p))");
            let body = if assigned {
                format!("y={value}; I(p)<+y;")
            } else {
                format!("I(p)<+{value};")
            };
            let fixture = compile(&format!(
                "module derivative_filters(p,q); inout p,q; electrical p,q; real y; analog begin {body} end endmodule"
            ));
            let mut device = fixture.device("DDX", &[1, 2]);
            device.update_voltages(&[0.25, 0.75]);
            let current = device.try_evaluate().unwrap()[0];
            assert!(
                (current / expected_current - 1.0).abs() < 1e-12,
                "{body}: {current}"
            );
            let (matrix, _) = collect_stamps(&mut device, &[0.25, 0.75]);
            assert!(
                (matrix.get(&(0, 0)).copied().unwrap_or(0.0) / expected_slope - 1.0).abs() < 1e-12,
                "{body}: {matrix:?}"
            );
            assert_eq!(matrix.get(&(0, 1)).copied().unwrap_or(0.0), 0.0, "{body}");
        }
    }
}

#[test]
fn ddx_laplace_transient_matches_primal_with_fixed_accepted_history() {
    use rspice_veriloga::vm::IntegrationCoefficients;

    for expression in [
        "laplace_nd(V(p)*V(p), '{1.0,0.5}, '{1.0,0.25})",
        "laplace_zp(V(p)*V(p), '{-2.0,0.0}, '{-4.0,0.0})",
        "laplace_zd(V(p)*V(p), '{-2.0,0.0}, '{1.0,0.25})",
        "laplace_np(V(p)*V(p), '{1.0,0.5}, '{-4.0,0.0})",
        "laplace_nd(V(p)*V(p), '{1.0,0.5}, '{1.0,0.25}) + laplace_nd(V(q), '{3.0}, '{1.0,0.5})",
        "laplace_nd(V(p)*V(p), '{1.0,0.5}, '{1.0,0.25}) * laplace_nd(V(p), '{1.0}, '{1.0,0.5})",
        "laplace_nd(laplace_nd(V(p)*V(p), '{1.0}, '{1.0,0.5}), '{1.0}, '{1.0,0.25})",
    ] {
        for assigned in [false, true] {
            let make = |value: &str| {
                let body = if assigned {
                    format!("y={value}; I(p)<+y;")
                } else {
                    format!("I(p)<+{value};")
                };
                compile(&format!(
                    "module filter_history(p,q); inout p,q; electrical p,q; real y; analog begin {body} end endmodule"
                ))
            };
            let primal_fixture = make(expression);
            let derivative_fixture = make(&format!("ddx({expression},V(p))"));
            let mut primal = primal_fixture.device("PRIMAL", &[1, 2]);
            let mut derivative = derivative_fixture.device("DERIVATIVE", &[1, 2]);
            for device in [&mut primal, &mut derivative] {
                device.update_voltages(&[0.0, 0.0]);
                device.try_evaluate().unwrap();
                device.advance_state();
                device.set_analysis_type(2);
            }
            // Vary companion gains and retain nonzero accepted histories to
            // expose accidental integration of derivative inputs.
            let mut time = 0.0;
            for (voltage, timestep) in [(0.25, 0.125), (0.4, 0.25), (-0.1, 0.0625)] {
                time += timestep;
                for device in [&mut primal, &mut derivative] {
                    device.set_time(time);
                    device.set_timestep(timestep);
                    device.set_integration_coefficients(IntegrationCoefficients {
                        active: true,
                        derivative_scale: 1.0 / timestep,
                        previous_value_scale: 1.0 / timestep,
                        older_value_scale: 0.0,
                        previous_derivative_scale: 0.0,
                    });
                }
                let sample = |device: &mut VerilogADevice, v| {
                    device.update_voltages(&[v, 0.75]);
                    device.try_evaluate().unwrap()[0]
                };
                let epsilon = 1e-5;
                let expected = (sample(&mut primal, voltage + epsilon)
                    - sample(&mut primal, voltage - epsilon))
                    / (2.0 * epsilon);
                let expected_slope = (sample(&mut derivative, voltage + epsilon)
                    - sample(&mut derivative, voltage - epsilon))
                    / (2.0 * epsilon);
                let current = sample(&mut derivative, voltage);
                let (matrix, _) = collect_stamps(&mut derivative, &[voltage, 0.75]);
                let slope = matrix.get(&(0, 0)).copied().unwrap_or(0.0);
                let context = format!("{expression}, assigned={assigned}, time={time}");
                assert!(
                    (current - expected).abs() < 1e-8,
                    "{context}: {current} != {expected}"
                );
                assert!(
                    (slope - expected_slope).abs() < 1e-8,
                    "{context}: slope {slope} != {expected_slope}"
                );
                assert_eq!(
                    matrix.get(&(0, 1)).copied().unwrap_or(0.0),
                    0.0,
                    "{context}"
                );
                sample(&mut primal, voltage);
                sample(&mut derivative, voltage);
                primal.advance_state();
                derivative.advance_state();
            }
        }
    }
}

#[test]
fn nested_ddx_laplace_preserves_curvature_jacobian_and_observation() {
    use rspice_veriloga::vm::IntegrationCoefficients;

    for (operator, numerator, denominator) in [
        ("laplace_nd", "'{1.0,0.5}", "'{1.0,0.25}"),
        ("laplace_zp", "'{-2.0,0.0}", "'{-4.0,0.0}"),
        ("laplace_zd", "'{-2.0,0.0}", "'{1.0,0.25}"),
        ("laplace_np", "'{1.0,0.5}", "'{-4.0,0.0}"),
    ] {
        for assigned in [false, true] {
            let value =
                format!("ddx(ddx({operator}(V(p)*V(p)*V(p),{numerator},{denominator}),V(p)),V(p))");
            let body = if assigned {
                format!("y={value}; I(p)<+y;")
            } else {
                format!("I(p)<+{value};")
            };
            let fixture = compile(&format!(
                "module filter_curvature(p); inout p; electrical p; real y; analog begin {body} end endmodule"
            ));
            let mut device = fixture.device("CURVATURE", &[1]);
            let check = |device: &mut VerilogADevice, voltage: f64, gain: f64| {
                let expected = 6.0 * voltage * gain;
                device.update_voltages(&[voltage]);
                let current = device.try_evaluate().unwrap()[0];
                assert!(
                    (current - expected).abs() < 1e-12,
                    "{body}: {current} != {expected}"
                );
                let (matrix, _) = collect_stamps(device, &[voltage]);
                assert!(
                    (matrix[&(0, 0)] - 6.0 * gain).abs() < 1e-12,
                    "{body}: {matrix:?}"
                );
                if assigned {
                    fixture.observe(device);
                    assert!(
                        (device.variable("y").unwrap() - expected).abs() < 1e-12,
                        "{body}"
                    );
                }
            };
            for voltage in [-0.75, 0.0, 1.25] {
                check(&mut device, voltage, 1.0);
            }
            check(&mut device, 0.0, 1.0);
            device.advance_state();
            device.set_analysis_type(2);
            let mut time = 0.0;
            for (voltage, timestep) in [(0.25, 0.125), (-0.4, 0.25)] {
                time += timestep;
                let scale = 1.0 / timestep;
                device.set_time(time);
                device.set_timestep(timestep);
                device.set_integration_coefficients(IntegrationCoefficients {
                    active: true,
                    derivative_scale: scale,
                    previous_value_scale: scale,
                    older_value_scale: 0.0,
                    previous_derivative_scale: 0.0,
                });
                // H(a) = (1 + a/2)/(1 + a/4), where a is the
                // integration rule's current-value derivative coefficient.
                check(
                    &mut device,
                    voltage,
                    (1.0 + 0.5 * scale) / (1.0 + 0.25 * scale),
                );
                device.advance_state();
            }
        }
    }
}

#[test]
fn laplace_complex_and_origin_roots_match_coefficient_forms() {
    for (zeros, poles, numerator, denominator, dc_gain) in [
        (
            "'{-1.0,1.0,-1.0,-1.0}",
            "'{-2.0,2.0,-2.0,-2.0}",
            "'{1.0,1.0,0.5}",
            "'{1.0,0.5,0.125}",
            1.0,
        ),
        (
            "'{0.0,0.0}",
            "'{-4.0,0.0}",
            "'{0.0,1.0}",
            "'{1.0,0.25}",
            0.0,
        ),
    ] {
        let mut responses = Vec::new();
        for (operator, n, d) in [
            ("laplace_zp", zeros, poles),
            ("laplace_zd", zeros, denominator),
            ("laplace_np", numerator, poles),
            ("laplace_nd", numerator, denominator),
        ] {
            let fixture = compile(&format!(
                "module normalized_roots(p,n); inout p,n; electrical p,n;
                analog I(p,n)<+{operator}(V(p,n),{n},{d}); endmodule"
            ));
            let mut device = fixture.device("ROOTS", &[1, 0]);
            device.update_voltages(&[0.25]);
            assert_eq!(device.try_evaluate().unwrap()[0], 0.25 * dc_gain);
            device.set_analysis_type(1);
            let mut values = Vec::new();
            for frequency in [0.0, 0.125, 1.0] {
                let mut terms = Vec::new();
                device
                    .try_stamp_small_signal_complex(&[0.25], frequency, |r, c, re, im| {
                        assert_eq!((r, c), (0, 0));
                        terms.push([re, im]);
                    })
                    .unwrap();
                assert_eq!(terms.len(), 1);
                values.push(terms[0]);
            }
            responses.push(values);
        }
        assert!(responses.iter().all(|values| values == &responses[3]));
    }
}

#[test]
fn laplace_root_forms_match_normalized_dc_ac_and_transient_responses() {
    // LRM 4.5.11: each spelling represents (1+s/2)/(1+s/4).
    for (operator, arguments) in [
        ("laplace_zp", "'{-2.0,0.0}, '{-4.0,0.0}"),
        ("laplace_zd", "'{-2.0,0.0}, '{1.0,0.25}"),
        ("laplace_np", "'{1.0,0.5}, '{-4.0,0.0}"),
        ("laplace_nd", "'{1.0,0.5}, '{1.0,0.25}"),
    ] {
        for assigned in [false, true] {
            let call = format!("{operator}(V(p,n)*V(p,n), {arguments})");
            let body = if assigned {
                format!("y={call}; I(p,n)<+y;")
            } else {
                format!("I(p,n)<+{call};")
            };
            let fixture = compile(&format!(
                "module normalized_filter(p,n); inout p,n; electrical p,n; real y; analog begin {body} end endmodule"
            ));
            let mut device = fixture.device("NORMALIZED", &[1, 0]);
            let bias = 0.25;
            let expected_current = bias * bias;
            let expected_slope = 2.0 * bias;
            device.update_voltages(&[bias]);
            assert!(
                (device.try_evaluate().unwrap()[0] / expected_current - 1.0).abs() < 1e-12,
                "{body}"
            );
            let (matrix, _) = collect_stamps(&mut device, &[bias]);
            assert!(
                (matrix[&(0, 0)] / expected_slope - 1.0).abs() < 1e-12,
                "{body}"
            );
            device.set_analysis_type(1);
            for frequency in [0.0, 0.125, 1.0, 1e-200, 1e200] {
                // H = 2 - 1/(1+j*w/4), evaluated without squaring a huge w.
                let x = 0.25 * std::f64::consts::TAU * frequency;
                let (inv_re, inv_im) = if x <= 1.0 {
                    (1.0 / (1.0 + x * x), -x / (1.0 + x * x))
                } else {
                    let inverse = 1.0 / x;
                    (
                        inverse * inverse / (1.0 + inverse * inverse),
                        -inverse / (1.0 + inverse * inverse),
                    )
                };
                let expected = [expected_slope * (2.0 - inv_re), -expected_slope * inv_im];
                let mut terms = Vec::new();
                device
                    .try_stamp_small_signal_complex(&[bias], frequency, |r, c, re, im| {
                        assert_eq!((r, c), (0, 0));
                        terms.push([re, im]);
                    })
                    .unwrap();
                assert_eq!(terms.len(), 1);
                for (actual, expected) in terms[0].into_iter().zip(expected) {
                    if expected == 0.0 {
                        assert_eq!(actual, 0.0);
                    } else {
                        assert!(
                            (actual / expected - 1.0).abs() < 1e-12,
                            "{body}, f={frequency}: {actual} != {expected}"
                        );
                    }
                }
            }
            let mut transient = fixture.device("STEP", &[1, 0]);
            transient.update_voltages(&[0.0]);
            assert_eq!(transient.try_evaluate().unwrap()[0], 0.0);
            transient.advance_state();
            transient.set_analysis_type(2);
            transient.set_time(0.125);
            transient.set_timestep(0.125);
            transient.set_integration_coefficients(rspice_veriloga::vm::IntegrationCoefficients {
                active: true,
                derivative_scale: 8.0,
                previous_value_scale: 8.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            });
            transient.update_voltages(&[bias]);
            let expected = (5.0 / 3.0) * bias * bias;
            assert!(
                (transient.try_evaluate().unwrap()[0] / expected - 1.0).abs() < 1e-12,
                "{body}"
            );
        }
    }
}

#[test]
fn filter_null_zeros_preserve_assigned_and_direct_device_responses() {
    for operator in ["laplace_zp", "laplace_zd", "zi_zp", "zi_zd"] {
        let denominator = if operator.ends_with("zp") {
            "'{-1.0,0.0}"
        } else {
            "'{1.0,1.0}"
        };
        let timing = if operator.starts_with("zi_") {
            ",0.25"
        } else {
            ""
        };
        let gain = if operator.starts_with("zi_") {
            0.5
        } else {
            1.0
        };
        let mut responses = Vec::new();
        for assigned in [false, true] {
            let call = format!("{operator}(V(p,n)*V(p,n),, {denominator}{timing})");
            let body = if assigned {
                format!("y={call}; I(p,n)<+y;")
            } else {
                format!("I(p,n)<+{call};")
            };
            let fixture = compile(&format!(
                "module null_filter(p,n); inout p,n; electrical p,n; real y;
                 analog begin {body} end endmodule"
            ));
            let mut device = fixture.device("NULL", &[1, 0]);
            let bias = 0.25;
            device.update_voltages(&[bias]);
            let current = device.try_evaluate().unwrap()[0];
            assert!(
                (current / (gain * bias * bias) - 1.0).abs() < 1e-12,
                "{operator}: {current}"
            );
            let (matrix, _) = collect_stamps(&mut device, &[bias]);
            assert!((matrix[&(0, 0)] / (2.0 * gain * bias) - 1.0).abs() < 1e-12);
            device.set_analysis_type(1);
            let mut values = Vec::new();
            for frequency in [0.0, 0.125, 0.373] {
                let mut terms = Vec::new();
                device
                    .try_stamp_small_signal_complex(&[bias], frequency, |r, c, re, im| {
                        assert_eq!((r, c), (0, 0));
                        terms.push([re, im]);
                    })
                    .unwrap();
                assert_eq!(terms.len(), 1);
                assert!(terms[0].iter().all(|value| value.is_finite()));
                values.push(terms[0]);
            }
            responses.push(values);
        }
        assert_eq!(responses[0], responses[1], "{operator}");
    }
}

#[test]
fn filter_root_array_order_preserves_compilation_and_ac_stamps() {
    let [a, b, c] = [
        -1.0,
        -1.0 - 50.0 * f64::EPSILON,
        -1.0 - 100.0 * f64::EPSILON,
    ];
    let ordered = [
        (a, 1.0),
        (a, -1.0),
        (b, 1.0),
        (b, -1.0),
        (c, 1.0),
        (c, -1.0),
    ];
    let permuted = [
        (a, 1.0),
        (b, -1.0),
        (b, 1.0),
        (c, -1.0),
        (c, 1.0),
        (a, -1.0),
    ];
    for operator in ["laplace_zp", "zi_zp"] {
        let mut responses = Vec::new();
        for roots in [ordered, permuted] {
            let roots = roots
                .into_iter()
                .flat_map(|(re, im)| [re, im])
                .map(|v| format!("{v:.17e}"))
                .collect::<Vec<_>>()
                .join(",");
            let timing = if operator == "zi_zp" { ",0.25" } else { "" };
            let fixture = compile(&format!(
                "module paired_filter(p,n); inout p,n; electrical p,n; real y;
                 analog begin y={operator}(V(p,n), , '{{{roots}}}{timing});
                 I(p,n)<+y; end endmodule"
            ));
            let mut device = fixture.device("PAIRS", &[1, 0]);
            device.set_analysis_type(1);
            let mut values = Vec::new();
            for frequency in [0.0, 0.125, 0.373] {
                let mut terms = Vec::new();
                device
                    .try_stamp_small_signal_complex(&[0.25], frequency, |r, c, re, im| {
                        assert_eq!((r, c), (0, 0));
                        terms.push([re, im]);
                    })
                    .unwrap();
                assert_eq!(terms.len(), 1);
                values.push(terms[0]);
            }
            responses.push(values);
        }
        for (ordered, permuted) in responses[0].iter().zip(&responses[1]) {
            for (&expected, &actual) in ordered.iter().zip(permuted) {
                if expected == 0.0 {
                    assert_eq!(actual, 0.0)
                } else {
                    assert!(
                        (actual / expected - 1.0).abs() <= 128.0 * f64::EPSILON,
                        "{operator}: {actual} vs {expected}"
                    );
                }
            }
        }
    }
}

#[test]
fn laplace_ac_stamps_preserve_final_components_across_intermediate_range_limits() {
    let fixture = compile(
        "module ranged_filter(p,n); inout p,n; electrical p,n; real y;
         analog begin y=laplace_nd(V(p,n), '{1.0,0.1,0.3}, '{1.0,1.0,1.0});
         I(p,n)<+y; end endmodule",
    );
    let mut device = fixture.device("AC", &[1, 0]);
    device.set_analysis_type(1);
    for (frequency, expected_real, expected_imaginary) in [
        (1e-200, 1.0, -0.9 * (std::f64::consts::TAU * 1e-200)),
        (1e200, 0.3, 0.2 / (std::f64::consts::TAU * 1e200)),
    ] {
        let mut terms = Vec::new();
        device
            .try_stamp_small_signal_complex(&[0.25], frequency, |r, c, re, im| {
                terms.push((r, c, re, im));
            })
            .unwrap();
        assert_eq!(terms.len(), 1);
        let (row, column, real, imaginary) = terms[0];
        assert_eq!((row, column), (0, 0));
        assert!((real / expected_real - 1.0).abs() <= 16.0 * f64::EPSILON);
        assert!((imaginary / expected_imaginary - 1.0).abs() <= 16.0 * f64::EPSILON);
    }

    let fixture = compile(
        "module small_response(p,n); inout p,n; electrical p,n;
         analog I(p,n)<+laplace_nd(V(p,n), '{1.0}, '{1.0,1.0,1.0}); endmodule",
    );
    let mut device = fixture.device("UNDERFLOW", &[1, 0]);
    device.set_analysis_type(1);
    let mut terms = 0;
    let error = device
        .try_stamp_small_signal_complex(&[0.25], 1e200, |_, _, _, _| terms += 1)
        .unwrap_err();
    assert!(error.to_string().contains("underflows"));
    assert_eq!(terms, 0);
}

#[test]
fn homogeneous_math_device_values_and_gradients_preserve_extreme_scales() {
    for op in ["hypot", "atan2"] {
        for derivative in 0..3 {
            let expression = format!("{op}(V(p),V(q))");
            let expression = match derivative {
                1 => format!("ddx({expression},V(p))"),
                2 => format!("ddx({expression},V(q))"),
                _ => expression,
            };
            let fixture = compile(&format!(
                "module planar(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
            ));
            let mut device = fixture.device("X", &[1, 2]);
            for scale in [1e-200, 1e-100, 1.0, 1e100, 1e200, 8e307] {
                for (a, b) in [(-1.0_f64, 2.0_f64), (0.0, -2.0), (1.0, -1.0), (1.0, 1.0)] {
                    let (p, q) = (a * scale, b * scale);
                    let expected = if op == "hypot" {
                        let r = a.hypot(b);
                        [p.hypot(q), a / r, b / r]
                    } else {
                        let d = a * a + b * b;
                        [p.atan2(q), (b / d) / scale, (-a / d) / scale]
                    };
                    device.update_voltages(&[p, q]);
                    let value = device
                        .try_evaluate()
                        .unwrap_or_else(|error| panic!("{expression} at {p:e},{q:e}: {error}"))[0];
                    let mut pairs = vec![(value, expected[derivative])];
                    // Atan2 Hessians at the smallest biases exceed f64's
                    // range. Check both mixed partials on representable scales.
                    if derivative == 0 || (1e-100..=1e100).contains(&scale) {
                        let d = a * a + b * b;
                        let hessian = if op == "hypot" {
                            let r = a.hypot(b);
                            let factor = (1.0 / (r * r * r)) / scale;
                            [b * b * factor, -a * b * factor, a * a * factor]
                        } else {
                            let factor = ((1.0 / (d * d)) / scale) / scale;
                            [
                                -2.0 * a * b * factor,
                                (a * a - b * b) * factor,
                                2.0 * a * b * factor,
                            ]
                        };
                        let slopes = match derivative {
                            1 => [hessian[0], hessian[1]],
                            2 => [hessian[1], hessian[2]],
                            _ => [expected[1], expected[2]],
                        };
                        let (matrix, _) = collect_stamps(&mut device, &[p, q]);
                        for (column, slope) in slopes.into_iter().enumerate() {
                            pairs.push((matrix.get(&(0, column)).copied().unwrap_or(0.0), slope));
                        }
                    }
                    for (actual, expected) in pairs {
                        if expected == 0.0 {
                            assert_eq!(actual, expected, "{expression} at {p:e},{q:e}");
                        } else {
                            assert!(
                                (actual / expected - 1.0).abs() < 1e-12,
                                "{expression} at {p:e},{q:e}: expected {expected:e}, got {actual:e}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn ac_quotient_preserves_common_scale_and_dynamic_phase() {
    for gain in [-1e200, -1e-200, 1e-308, 1e-200, 1.0, 1e200] {
        for dynamic in [false, true] {
            for assigned in [false, true] {
                let input = if dynamic { "V(p)+ddt(V(p))" } else { "V(p)" };
                let denominator = format!("{gain:e}*({input})");
                let body = if assigned {
                    format!("r={denominator}; I(p)<+{gain:e}/r;")
                } else {
                    format!("I(p)<+{gain:e}/({denominator});")
                };
                let fixture = compile(&format!(
                    "module quotient_ac(p); inout p; electrical p; real r; analog begin {body} end endmodule"
                ));
                for bias in [-0.01_f64, 0.01] {
                    for omega in [1.0, 10.0] {
                        let mut device = fixture.device("AC", &[1]);
                        device.set_analysis_type(1);
                        let mut admittance = (0.0, 0.0);
                        device
                            .try_stamp_small_signal_complex(
                                &[bias],
                                omega / std::f64::consts::TAU,
                                |row, col, re, im| {
                                    assert_eq!((row, col), (0, 0));
                                    admittance.0 += re;
                                    admittance.1 += im;
                                },
                            )
                            .unwrap_or_else(|error| {
                                panic!("{body}, bias={bias}, omega={omega}: {error}")
                            });
                        let slope = -1.0 / (bias * bias);
                        assert!(
                            (admittance.0 / slope - 1.0).abs() < 1e-11,
                            "{body}: {admittance:?}"
                        );
                        if dynamic {
                            assert!(
                                (admittance.1 / (slope * omega) - 1.0).abs() < 1e-11,
                                "{body}: {admittance:?}"
                            );
                        } else {
                            assert_eq!(admittance.1, 0.0, "{body}");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn quotient_range_preserves_values_and_curvature() {
    for (expression, bias, expected) in [
        ("ddx(1e308/(1e308*V(p)),V(p))", 0.01, [-1e4, 2e6]),
        ("ddx(1.6e308*V(p)/(3*V(p)-1),V(p))", 1.0, [-4e307, 1.2e308]),
        ("ddx(V(p)/V(p),V(p))", 1e-309, [0.0, 0.0]),
        ("ddx((2*V(p))/(3*V(p)),V(p))", 1e-309, [0.0, 0.0]),
        ("ddx((5*V(p))/(7*V(p)),V(p))", -1e-309, [0.0, 0.0]),
        ("ddx((2*V(p))/(3*V(p)),V(p))", 1e-100, [0.0, 0.0]),
        ("ddx((5*V(p))/(7*V(p)),V(p))", 1.0, [0.0, 0.0]),
        ("ddx(1e308/(1e200+1e-200*V(p)),V(p))", 0.0, [-1e-292, 0.0]),
    ] {
        let fixture = compile(&format!(
            "module quotient(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
        ));
        let mut device = fixture.device("X", &[1]);
        device.update_voltages(&[bias]);
        let current = device
            .try_evaluate()
            .unwrap_or_else(|error| panic!("{expression}: {error}"))[0];
        let (matrix, _) = collect_stamps(&mut device, &[bias]);
        for (actual, expected) in [current, matrix.get(&(0, 0)).copied().unwrap_or(0.0)]
            .into_iter()
            .zip(expected)
        {
            if expected == 0.0 {
                assert_eq!(actual, expected, "{expression}");
            } else {
                assert!(
                    (actual / expected - 1.0).abs() < 1e-12,
                    "{expression}: {actual:e} != {expected:e}"
                );
            }
        }
    }
}

#[test]
fn quotient_mixed_partials_preserve_representable_results() {
    for derivative in 0..3 {
        let expression = "1e200*V(p)*V(p)/(V(q)*V(q))";
        let expression = match derivative {
            1 => format!("ddx({expression},V(p))"),
            2 => format!("ddx({expression},V(q))"),
            _ => expression.into(),
        };
        let fixture = compile(&format!(
            "module quotient(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
        ));
        let mut device = fixture.device("X", &[1, 2]);
        for p in [-2.0, 0.0, 0.75] {
            for q in [-1e150, -1e100, 1e50, 1e100, 1e150] {
                let scale = (1e200 / q) / q;
                let value = scale * p * p;
                let dp = 2.0 * scale * p;
                let dq = (-2.0 * value) / q;
                let expected = match derivative {
                    1 => [dp, 2.0 * scale, (-2.0 * dp) / q],
                    2 => [dq, (-2.0 * dp) / q, (-3.0 * dq) / q],
                    _ => [value, dp, dq],
                };
                device.update_voltages(&[p, q]);
                let value = device.try_evaluate().unwrap()[0];
                let (matrix, _) = collect_stamps(&mut device, &[p, q]);
                for (actual, expected) in [
                    value,
                    matrix.get(&(0, 0)).copied().unwrap_or(0.0),
                    matrix.get(&(0, 1)).copied().unwrap_or(0.0),
                ]
                .into_iter()
                .zip(expected)
                {
                    if expected == 0.0 {
                        assert_eq!(actual, expected);
                    } else {
                        assert!(
                            (actual / expected - 1.0).abs() < 1e-12,
                            "{expression}, p={p:e}, q={q:e}: expected {expected:e}, got {actual:e}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn hypot_input_gains_preserve_finite_full_hessian() {
    for (a, b, p, q) in [
        (1e-200_f64, 1e-200_f64, 1.0, 1.0),
        (1e200, 1e200, 1.0, 1.0),
        (1e150, 1e-150, 1e-150, 1e150),
        (1e-150, 1e150, 1e150, 1e-150),
    ] {
        let radius = (a * p).hypot(b * q);
        // Coordinates are equal, so the analytic Hessian is
        // [a*a, -a*b; -a*b, b*b] / (2*radius).
        for (axis, gain, slopes) in [
            ("p", a, [(a / radius) * a / 2.0, -(a / radius) * b / 2.0]),
            ("q", b, [-(b / radius) * a / 2.0, (b / radius) * b / 2.0]),
        ] {
            let fixture = compile(&format!(
                "module gain(p,q); inout p,q; electrical p,q; analog I(p)<+ddx(hypot({a:e}*V(p),{b:e}*V(q)),V({axis})); endmodule"
            ));
            let mut device = fixture.device("X", &[1, 2]);
            device.update_voltages(&[p, q]);
            let value = device.try_evaluate().unwrap()[0];
            let (matrix, _) = collect_stamps(&mut device, &[p, q]);
            for (actual, expected) in [value, matrix[&(0, 0)], matrix[&(0, 1)]].into_iter().zip([
                gain / std::f64::consts::SQRT_2,
                slopes[0],
                slopes[1],
            ]) {
                assert!(
                    (actual / expected - 1.0).abs() < 1e-12,
                    "a={a:e}, b={b:e}, axis={axis}: expected {expected:e}, got {actual:e}"
                );
            }
        }
    }
}

#[test]
fn hypot_shared_operand_keeps_its_large_value_and_finite_gradient() {
    let fixture = compile(
        "module shared(p); inout p; electrical p; analog I(p)<+hypot(V(p),V(p)); endmodule",
    );
    let mut device = fixture.device("X", &[1]);
    for p in [-1e308_f64, 1e308, -1e-200, 1e-200] {
        device.update_voltages(&[p]);
        let value = device.try_evaluate().unwrap()[0];
        assert!((value / p.hypot(p) - 1.0).abs() < 1e-12);
        let (matrix, _) = collect_stamps(&mut device, &[p]);
        let expected = p.signum() * std::f64::consts::SQRT_2;
        assert!((matrix[&(0, 0)] / expected - 1.0).abs() < 1e-12);
    }
}

#[test]
fn hypot_parameter_defaults_preserve_range_after_overrides() {
    let fixture = compile(
        "module defaults(p); inout p; electrical p; parameter real x=1e308; parameter real r=hypot(x,x); analog I(p)<+r; endmodule",
    );
    let mut device = fixture.device("X", &[1]);
    for x in [1e308_f64, -1e308, 1e-200, -1e-200] {
        assert!(device.set_parameter("x", x));
        device.resolve_parameter_defaults();
        let value = device.try_evaluate().unwrap()[0];
        assert!((value / x.hypot(x) - 1.0).abs() < 1e-12);
    }
}

#[test]
fn hypot_dynamic_operands_preserve_history_and_small_signal_slopes() {
    for body in [
        "I(p)<+hypot(1+ddt(V(p)),2+ddt(V(q)));",
        "r=hypot(1+ddt(V(p)),2+ddt(V(q))); I(p)<+r;",
    ] {
        let fixture = compile(&format!(
            "module dynamic(p,q); inout p,q; electrical p,q; real r; analog begin {body} end endmodule",
        ));
        let mut device = fixture.device("X", &[1, 2]);
        device.update_voltages(&[1.0, 2.0]);
        assert!((device.try_evaluate().unwrap()[0] - 5.0_f64.sqrt()).abs() < 1e-12);
        device.advance_state();
        device.set_analysis_type(2);
        device.set_timestep(0.5);
        // Repeated and alternate Newton candidates must use the accepted charges.
        for (p, q) in [(2.0, 3.0), (3.0, 4.0), (2.0, 3.0)] {
            let (x, y) = (1.0_f64 + (p - 1.0) / 0.5, 2.0_f64 + (q - 2.0) / 0.5);
            let r = x.hypot(y);
            device.update_voltages(&[p, q]);
            assert!((device.try_evaluate().unwrap()[0] - r).abs() < 1e-12);
            let (matrix, _) = collect_stamps(&mut device, &[p, q]);
            assert!((matrix[&(0, 0)] - x / r / 0.5).abs() < 1e-12);
            assert!((matrix[&(0, 1)] - y / r / 0.5).abs() < 1e-12);
        }
        device.advance_state();
        device.update_voltages(&[3.0, 4.0]);
        assert!((device.try_evaluate().unwrap()[0] - 5.0).abs() < 1e-12);

        let mut ac = fixture.device("AC", &[1, 2]);
        ac.set_analysis_type(1);
        let mut entries = HashMap::new();
        ac.try_stamp_small_signal_complex(
            &[1.0, 2.0],
            1.0 / std::f64::consts::TAU,
            |row, col, re, im| {
                let entry = entries.entry((row, col)).or_insert((0.0, 0.0));
                entry.0 += re;
                entry.1 += im;
            },
        )
        .unwrap();
        for column in 0..2 {
            let (re, im) = entries[&(0, column)];
            assert!(re.abs() < 1e-12);
            assert!((im - (column + 1) as f64 / 5.0_f64.sqrt()).abs() < 1e-12);
        }
    }
}

#[cfg(not(feature = "native"))]
#[test]
fn ddx_array_self_assignment_preserves_values_through_aliasing_indices() {
    for (target, operand) in [("q[idx]", "q[2]"), ("q[2]", "q[idx]"), ("q[idx]", "q[idx]")] {
        let fixture = compile(&format!(
            "module array_self(p,n); inout p,n; electrical p,n; real q[2:3]; integer idx; analog begin idx=2; q[2]=V(p,n)*V(p,n)*V(p,n)*V(p,n); q[3]=0; {target}=ddx({operand},V(p,n)); I(p,n)<+q[2]; end endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        for v in [-0.75_f64, 0.0, 1.25] {
            device.update_voltages(&[v]);
            let current = device.try_evaluate().unwrap()[0];
            assert!(
                (current - 4.0 * v.powi(3)).abs() < 1e-10,
                "{target}={operand}: {current}"
            );
            let (matrix, _) = collect_stamps(&mut device, &[v]);
            assert!((matrix.get(&(0, 0)).copied().unwrap_or(0.0) - 12.0 * v * v).abs() < 1e-10);
        }
    }
    let fixture = compile(
        "module index_self(p,n); inout p,n; electrical p,n; real q[0:2]; analog begin q[0]=0.5*V(p,n)*V(p,n); q[1]=0; q[2]=0; q[ddx(q[0],V(p,n))]=1+2*V(p,n); I(p,n)<+q[0]; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for (v, current, slope) in [(0.0, 1.0, 2.0), (1.0, 0.5, 1.0), (2.0, 2.0, 2.0)] {
        device.update_voltages(&[v]);
        assert_eq!(device.try_evaluate().unwrap()[0], current);
        let (matrix, _) = collect_stamps(&mut device, &[v]);
        assert_eq!(matrix[&(0, 0)], slope);
    }
}

#[test]
fn ddx_self_assignment_preserves_the_primal_value_and_jacobian() {
    let fixture = compile(
        "module self_primal(p,n); inout p,n; electrical p,n; real x; analog begin x=V(p,n)*V(p,n)*V(p,n)*V(p,n); x=ddx(x,V(p,n)); I(p,n)<+x; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-0.75_f64, 0.0, 1.25] {
        device.update_voltages(&[v]);
        let current = device.try_evaluate().unwrap()[0];
        assert!(
            (current - 4.0 * v.powi(3)).abs() < 1e-10,
            "at {v}: {current}"
        );
        let (matrix, _) = collect_stamps(&mut device, &[v]);
        assert!((matrix.get(&(0, 0)).copied().unwrap_or(0.0) - 12.0 * v * v).abs() < 1e-10);
    }
}

#[test]
fn nested_ddx_array_loop_distinguishes_overwritten_elements() {
    let fixture = compile(
        "module array_loop(p,n); inout p,n; electrical p,n; parameter integer count=3; real q[1:2]; integer k; analog begin for(k=0;k<count;k=k+1) begin q[1]=V(p,n)*V(p,n)*V(p,n); q[2]=ddx(q[1],V(p,n)); end I(p,n)<+ddx(q[2],V(p,n)); end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-0.75, 0.0, 1.25] {
        device.update_voltages(&[v]);
        assert!((device.try_evaluate().unwrap()[0] - 6.0 * v).abs() < 1e-10);
        let (matrix, _) = collect_stamps(&mut device, &[v]);
        assert!((matrix[&(0, 0)] - 6.0).abs() < 1e-10);
    }
}

#[test]
fn nested_ddx_observation_preserves_loop_carried_derivatives() {
    let fixture = compile(
        "module loop_readback(p,n); inout p,n; electrical p,n; parameter integer count=3; real x,reported; integer k; analog begin x=1; for(k=0;k<count;k=k+1) x=x*V(p,n); reported=ddx(ddx(x,V(p,n)),V(p,n)); I(p,n)<+reported; end endmodule",
    );
    let mut device = fixture.device("X", &[1, 0]);
    for v in [-0.75, 0.0, 1.25] {
        device.update_voltages(&[v]);
        assert!((device.try_evaluate().unwrap()[0] - 6.0 * v).abs() < 1e-10);
        fixture.observe(&mut device);
        assert!((device.variable("reported").unwrap() - 6.0 * v).abs() < 1e-10);
    }
}

#[cfg(feature = "native")]
#[test]
fn nested_ddx_readback_refuses_unimplemented_simultaneous_shadow_writes() {
    for (tail, expected) in [
        ("reported=ddx(x,V(p,n)); I(p,n)<+reported;", 7.5),
        ("I(p,n)<+V(p,n);", 1.25),
    ] {
        let fixture = compile(&format!(
            "module self_readback(p,n); inout p,n; electrical p,n; real x,reported; analog begin x=V(p,n)*V(p,n)*V(p,n); x=ddx(x,V(p,n)); {tail} end endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        device.update_voltages(&[1.25]);
        assert_eq!(device.try_evaluate().unwrap(), vec![expected]);
        let error = device
            .observe_variables(&fixture.canonical_ir)
            .unwrap_err()
            .to_string();
        assert!(error.contains("simultaneous ddx self-update"), "{error}");
    }
}

#[test]
fn nested_ddx_transcendentals_and_report_only_values_keep_higher_orders() {
    for body in [
        "x=ddx(ddx(ddx(exp(V(p,n)),V(p,n)),V(p,n)),V(p,n));",
        "x=exp(V(p,n)); x=ddx(x,V(p,n)); x=ddx(x,V(p,n)); x=ddx(x,V(p,n));",
        "x=exp(V(p,n)); for(k=0;k<3;k=k+1) x=ddx(x,V(p,n));",
    ] {
        let fixture = compile(&format!(
            "module transcendental(p,n); inout p,n; electrical p,n; real x, reported; integer k; analog begin {body} reported=ddx(ddx(exp(V(p,n)),V(p,n)),V(p,n)); I(p,n)<+x; end endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        for voltage in [-0.75_f64, 0.0, 1.25] {
            device.update_voltages(&[voltage]);
            let values = device.try_evaluate().unwrap();
            assert!(
                (values[0] - voltage.exp()).abs() < 1e-10,
                "{body}: {values:?}"
            );
            let (matrix, _) = collect_stamps(&mut device, &[voltage]);
            assert!(
                (matrix[&(0, 0)] - voltage.exp()).abs() < 1e-10,
                "{body}: {matrix:?}"
            );
            #[cfg(not(feature = "native"))]
            {
                fixture.observe(&mut device);
                assert!((device.variable("reported").unwrap() - voltage.exp()).abs() < 1e-10);
            }
        }
    }
}

#[test]
fn nested_ddx_mathematical_values_and_jacobians_follow_analytic_derivatives() {
    for (expression, expected) in [
        (
            "sin(V(p,n))",
            (|x: f64| [-x.sin(), -x.cos()]) as fn(f64) -> [f64; 2],
        ),
        ("cos(V(p,n))", |x| [-x.cos(), x.sin()]),
        ("exp(V(p,n))", |x| [x.exp(), x.exp()]),
        ("sqrt(V(p,n))", |x| {
            [-0.25 / x.powf(1.5), 0.375 / x.powf(2.5)]
        }),
        ("ln(V(p,n))", |x| [-1.0 / x.powi(2), 2.0 / x.powi(3)]),
        ("pow(V(p,n),3.5)", |x| {
            [8.75 * x.powf(1.5), 13.125 * x.sqrt()]
        }),
        ("V(p,n)**3.5", |x| [8.75 * x.powf(1.5), 13.125 * x.sqrt()]),
        ("(2.0+V(p,n))/(1.0+V(p,n))", |x| {
            [2.0 / (1.0 + x).powi(3), -6.0 / (1.0 + x).powi(4)]
        }),
        ("min(V(p,n)*V(p,n)*V(p,n),0.5)", |x| {
            if x * x * x < 0.5 {
                [6.0 * x, 6.0]
            } else {
                [0.0, 0.0]
            }
        }),
        ("max(V(p,n)*V(p,n)*V(p,n),0.5)", |x| {
            if x * x * x > 0.5 {
                [6.0 * x, 6.0]
            } else {
                [0.0, 0.0]
            }
        }),
        ("hypot(V(p,n),2.0)", |x| {
            [
                4.0 / (x * x + 4.0).powf(1.5),
                -12.0 * x / (x * x + 4.0).powf(2.5),
            ]
        }),
        ("atan2(V(p,n),2.0)", |x| {
            [
                -4.0 * x / (x * x + 4.0).powi(2),
                (12.0 * x * x - 16.0) / (x * x + 4.0).powi(3),
            ]
        }),
        ("hypot(V(p,n),V(p,n)+3.0)", |x| {
            let d = 2.0 * x * x + 6.0 * x + 9.0;
            [9.0 / d.powf(1.5), -27.0 * (2.0 * x + 3.0) / d.powf(2.5)]
        }),
        ("atan2(V(p,n),V(p,n)+3.0)", |x| {
            let d = 2.0 * x * x + 6.0 * x + 9.0;
            let dp = 4.0 * x + 6.0;
            [
                -3.0 * dp / d.powi(2),
                6.0 * dp * dp / d.powi(3) - 12.0 / d.powi(2),
            ]
        }),
    ] {
        let fixture = compile(&format!(
            "module nested_math(p,n); inout p,n; electrical p,n; analog I(p,n)<+ddx(ddx({expression},V(p,n)),V(p,n)); endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        for voltage in [0.25, 0.75, 1.25, 3.0, 5.0] {
            device.update_voltages(&[voltage]);
            let current = device.try_evaluate().unwrap()[0];
            let (matrix, _) = collect_stamps(&mut device, &[voltage]);
            let jacobian = matrix.get(&(0, 0)).copied().unwrap_or(0.0);
            for (actual, expected) in [current, jacobian].into_iter().zip(expected(voltage)) {
                assert!(
                    (actual - expected).abs() <= 1e-10 * expected.abs().max(1.0),
                    "{expression}, V={voltage}: expected {expected}, got {actual}"
                );
            }
        }
    }
}

#[test]
fn recursive_ddx_in_a_runtime_loop_is_diagnosed() {
    let source = "module recursive(p,n); inout p,n; electrical p,n; real x; integer k; analog begin x=exp(V(p,n)); for(k=0;k<V(p,n);k=k+1) x=ddx(x,V(p,n)); I(p,n)<+x; end endmodule";
    let error = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(source, None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("unbounded derivative order"), "{error}");
}

#[test]
fn nested_ddx_preserves_variable_array_and_self_assignment_jacobians() {
    for body in [
        "analog I(p,n)<+ddx(ddx(V(p,n)*V(p,n)*V(p,n),V(p,n)),V(p,n));",
        "real x,y; analog begin x=V(p,n)*V(p,n)*V(p,n); y=ddx(x,V(p,n)); I(p,n)<+ddx(y,V(p,n)); end",
        "real x; analog begin x=V(p,n)*V(p,n)*V(p,n); x=ddx(x,V(p,n)); I(p,n)<+ddx(x,V(p,n)); end",
        "real q[2:3]; integer idx; analog begin idx=2; q[idx]=V(p,n)*V(p,n)*V(p,n); I(p,n)<+ddx(ddx(q[idx],V(p,n)),V(p,n)); end",
        "parameter integer count=3; real x; integer k; analog begin x=0; for(k=0;k<count;k=k+1) x=x+V(p,n)*V(p,n)*V(p,n); I(p,n)<+ddx(ddx(x,V(p,n)),V(p,n))/3; end",
        "real x,y; analog begin x=V(p,n)*V(p,n)*V(p,n)*V(p,n); y=ddx(ddx(x,V(p,n)),V(p,n)); I(p,n)<+ddx(y,V(p,n))/4; end",
    ] {
        let fixture = compile(&format!(
            "module nested_runtime(p,n); inout p,n; electrical p,n; {body} endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        for voltage in [-0.8, 0.0, 1.3] {
            device.update_voltages(&[voltage]);
            let values = device.try_evaluate().unwrap();
            assert!(
                (values[0] - 6.0 * voltage).abs() < 1e-10,
                "{body}: {values:?}"
            );
            let (matrix, rhs) = collect_stamps(&mut device, &[voltage]);
            assert!((matrix[&(0, 0)] - 6.0).abs() < 1e-10, "{body}: {matrix:?}");
            assert!(
                rhs.values().all(|value| value.abs() < 1e-10),
                "{body}: {rhs:?}"
            );
        }
    }
}

#[test]
fn reactive_stamping_holds_external_derivative_coefficients_at_the_bias_point() {
    for (expression, capacitances, current, driven_derivative, control_derivative) in [
        ("V(p,n)*ddt(V(p,n))", [3.0, -2.0], 12.0, 10.0, 0.0),
        ("V(c,n)*ddt(V(p,n))", [2.0, -4.0], 8.0, 4.0, 4.0),
        ("ddt(V(p,n))/V(c,n)", [0.5, -0.25], 2.0, 1.0, -1.0),
        (
            "((V(c,n)>0)?2.0:4.0)*ddt(V(p,n))",
            [2.0, 4.0],
            8.0,
            4.0,
            0.0,
        ),
        (
            "(2.0+V(c,n))*ddt(V(p,n)*V(p,n))",
            [24.0, 8.0],
            64.0,
            48.0,
            16.0,
        ),
    ] {
        let fixture = compile(&format!(
            "module weighted_derivative(p,n,c); inout p,n,c; electrical p,n,c; analog I(p,n)<+{expression}; endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0, 2]);
        device.try_set_analysis_type(1).unwrap();
        for (bias, capacitance) in [[3.0, 2.0], [-2.0, -4.0]].into_iter().zip(capacitances) {
            let mut matrix = HashMap::new();
            device
                .try_stamp_reactive(&bias, |row, column, value| {
                    *matrix.entry((row, column)).or_insert(0.0) += value;
                })
                .unwrap();
            assert_eq!(
                matrix.get(&(0, 0)).copied().unwrap_or(0.0),
                capacitance,
                "{expression}: {matrix:?}"
            );
            assert_eq!(
                matrix.get(&(0, 1)).copied().unwrap_or(0.0),
                0.0,
                "a coefficient outside ddt creates no control-port capacitance: {expression}: {matrix:?}"
            );
        }

        // The transient Newton derivative must still include dk/dx * ddt(q).
        device.try_set_analysis_type(0).unwrap();
        device.update_voltages(&[1.0, 4.0]);
        assert_eq!(device.try_evaluate().unwrap(), vec![0.0]);
        device.advance_state();
        device.try_set_analysis_type(2).unwrap();
        device.set_timestep(0.5);
        device.update_voltages(&[3.0, 2.0]);
        assert_eq!(
            device.try_evaluate().unwrap(),
            vec![current],
            "{expression}"
        );
        let (matrix, _) = collect_stamps(&mut device, &[3.0, 2.0]);
        assert_eq!(matrix[&(0, 0)], driven_derivative, "{expression}");
        assert_eq!(
            matrix.get(&(0, 1)).copied().unwrap_or(0.0),
            control_derivative,
            "{expression}"
        );
    }
}

#[test]
fn analysis_continuation_retargets_only_matching_devices_without_initialization() {
    let fixture = compile(
        r#"module continued(p,n);
inout p,n; electrical p,n;
real scale;
analog initial scale=sqrt(301.0-$temperature);
analog I(p,n)<+scale*V(p,n);
endmodule"#,
    );
    let mut initial = fixture.device("X1", &[1, 0]);
    initial.try_set_temperature(300.0).unwrap();
    initial.try_begin_analysis(0).unwrap();
    let accepted = initial.checkpoint_state().unwrap();
    let mut rebuilt = fixture.device("x1", &[2, 0]);
    rebuilt.try_set_temperature(302.0).unwrap();
    rebuilt.try_set_analysis_type(0).unwrap();
    for bad_source in [
        {
            let mut state = accepted.clone();
            state.instance_name = "X2".into();
            state
        },
        {
            let mut state = accepted.clone();
            state.source_digest = "different source".into();
            state
        },
        {
            let mut state = accepted.clone();
            state.accepted.variables.clear();
            state
        },
    ] {
        assert!(rebuilt.prepare_analysis_continuation(&bad_source).is_err());
    }
    let mut rebuilt = rebuilt.prepare_analysis_continuation(&accepted).unwrap();
    rebuilt.update_voltages(&[0.0, 3.0]);
    assert_eq!(rebuilt.try_evaluate().unwrap(), vec![3.0]);
}

#[test]
fn cached_device_construction_still_validates_the_supplied_artifact() {
    let fixture = compile(
        r#"`include "disciplines.vams"
module cache_integrity(p,n);
inout p,n; electrical p,n;
analog I(p,n) <+ V(p,n)/4.0;
endmodule"#,
    );
    let model = std::sync::Arc::new(fixture.model);
    let construct = |artifact: &_| {
        VerilogADevice::try_new_with_canonical_ir("X", model.clone(), artifact, &[1, 0])
    };
    let mut first = construct(&fixture.canonical_ir).unwrap();
    first.update_voltages(&[8.0]);
    assert_eq!(first.try_evaluate().unwrap(), vec![2.0]);

    let mut stale_schema = fixture.canonical_ir.clone();
    stale_schema.metadata.schema_version = 0;
    assert!(construct(&stale_schema).is_err());

    let mut changed_graph = fixture.canonical_ir.clone();
    changed_graph.mir.equations.clear();
    assert!(construct(&changed_graph).is_err());

    let mut next = construct(&fixture.canonical_ir).unwrap();
    next.update_voltages(&[12.0]);
    assert_eq!(next.try_evaluate().unwrap(), vec![3.0]);
}

#[test]
fn custom_flow_access_reads_the_branch_unknown() {
    for (expression, expected) in [
        ("TestQ(b)", 7.0),
        ("TestQ(n,p)", -7.0),
        ("TestQ(b) * TestU(b)", 14.0),
        ("ddx(TestQ(b)*TestQ(b), TestQ(b))", 14.0),
        ("ddx(TestQ(b)*TestQ(b), TestQ(n,p))", -14.0),
        (
            "ddx(ddx(TestQ(b)*TestQ(b)*TestU(b)*TestU(b),TestQ(b)),TestU(b))",
            56.0,
        ),
        (
            "ddx(ddx(TestQ(b)*TestQ(b)*TestU(b)*TestU(b),TestU(n,p)),TestQ(b))",
            -56.0,
        ),
    ] {
        // The reversed unnamed probe must read an unnamed source. Named
        // branches are distinct even when their endpoints are identical.
        let unnamed_flow = expression.contains("TestQ(n,p)");
        let expression = if unnamed_flow {
            expression.replace("TestQ(b)", "TestQ(p,n)")
        } else {
            expression.to_owned()
        };
        let source_probe = if unnamed_flow {
            "TestU(p,n)"
        } else {
            "TestU(b)"
        };
        let source = format!(
            r#"
`include "disciplines.vams"
nature TestPotential units="V"; access=TestU; abstol=1e-6; endnature
nature TestFlow units="A"; access=TestQ; abstol=1e-12; endnature
discipline testdisc potential TestPotential; flow TestFlow; enddiscipline
module probe(p,n,o);
inout p,n,o; testdisc p,n; electrical o;
branch(p,n) b;
analog begin {source_probe} <+ 2.0; I(o) <+ {expression}; end
endmodule
"#
        );
        let fixture = compile(&source);
        let mut device = fixture.device("X", &[1, 0, 2]);
        device.set_branch_current_indices(&[3]);
        device.update_all_voltages(&[2.0, 0.0, 7.0]);
        assert_eq!(
            device.try_evaluate().unwrap(),
            vec![2.0, expected],
            "{expression}"
        );
    }
}

#[test]
fn the_same_nature_can_have_different_roles_in_different_disciplines() {
    let fixture = compile(
        r#"
`include "disciplines.vams"
nature Shared units="U"; access=Sense; abstol=1e-6; endnature
discipline potential_only potential Shared; enddiscipline
discipline flow_only flow Shared; enddiscipline
module probe(p,n,o);
inout p,n,o; potential_only p,n; electrical o;
analog begin Sense(p,n) <+ 2.0; I(o) <+ Sense(p,n); end
endmodule
"#,
    );
    let mut device = fixture.device("X", &[1, 0, 2]);
    device.set_branch_current_indices(&[3]);
    device.update_all_voltages(&[2.0, 0.0, 7.0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![2.0, 2.0]);
}

#[test]
fn custom_flow_read_has_the_correct_jacobian_columns() {
    let fixture = compile(
        r#"
`include "disciplines.vams"
nature TestPotential units="V"; access=TestU; abstol=1e-6; endnature
nature TestFlow units="A"; access=TestQ; abstol=1e-12; endnature
discipline testdisc potential TestPotential; flow TestFlow; enddiscipline
module probe(p,n,o);
inout p,n,o; testdisc p,n; electrical o;
analog begin TestU(p,n) <+ 2.0; I(o) <+ TestQ(p,n)*TestU(p,n); end
endmodule
"#,
    );
    let mut device = fixture.device("X", &[1, 0, 2]);
    device.set_branch_current_indices(&[3]);
    let (matrix, _) = collect_stamps(&mut device, &[2.0, 0.0, 7.0]);
    assert_eq!(matrix[&(1, 0)], 7.0, "d(output flow)/d(potential)");
    assert_eq!(matrix[&(1, 2)], 2.0, "d(output flow)/d(branch flow)");
}

/// Collect matrix and RHS stamps into maps for inspection
fn collect_stamps(
    device: &mut VerilogADevice,
    voltages: &[f64],
) -> (HashMap<(usize, usize), f64>, HashMap<usize, f64>) {
    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    let mut rhs: HashMap<usize, f64> = HashMap::new();
    device.stamp(
        voltages,
        |row, col, value| *matrix.entry((row, col)).or_insert(0.0) += value,
        |node, value| *rhs.entry(node).or_insert(0.0) += value,
    );
    (matrix, rhs)
}

const RESISTOR: &str = r#"
`include "disciplines.vams"
module res2(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

#[test]
fn resistor_stamps_companion_form() {
    let model = compile(RESISTOR);
    // p -> circuit node 1, n -> ground
    let mut device = model.device("R1", &[1, 0]);

    // Node 1 at 4 V
    let (matrix, rhs) = collect_stamps(&mut device, &[4.0]);

    // G = 1/r = 0.5 at (0,0); rows/cols touching ground are dropped
    assert_eq!(matrix.len(), 1);
    assert!((matrix[&(0, 0)] - 0.5).abs() < 1e-12);

    // Linear element: Ieq = I - G*V = 2 - 0.5*4 = 0
    let total_rhs: f64 = rhs.values().map(|v| v.abs()).sum();
    assert!(total_rhs < 1e-12, "linear resistor must have zero Ieq");
}

#[test]
fn floating_resistor_stamps_all_four_positions() {
    let model = compile(RESISTOR);
    // Both terminals on non-ground circuit nodes 1 and 2
    let mut device = model.device("R1", &[1, 2]);

    let (matrix, _rhs) = collect_stamps(&mut device, &[3.0, 1.0]);

    // Full two-terminal conductance pattern
    assert!((matrix[&(0, 0)] - 0.5).abs() < 1e-12);
    assert!((matrix[&(0, 1)] + 0.5).abs() < 1e-12);
    assert!((matrix[&(1, 0)] + 0.5).abs() < 1e-12);
    assert!((matrix[&(1, 1)] - 0.5).abs() < 1e-12);
}

#[test]
fn nonlinear_companion_rhs_matches_analytic() {
    // I = 2*V^2 through an intermediate variable (exercises the
    // shadow-variable chain rule): dI/dV = 4V
    let model = compile(
        r#"
`include "disciplines.vams"
module sqlaw(p, n);
    inout p, n;
    electrical p, n;
    real gm;
    analog begin
        gm = 2.0 * V(p, n);
        I(p, n) <+ gm * V(p, n);
    end
endmodule
"#,
    );
    let mut device = model.device("Q1", &[1, 0]);

    let v = 3.0;
    let (matrix, rhs) = collect_stamps(&mut device, &[v]);

    // G = dI/dV = 4*V = 12
    assert!(
        (matrix[&(0, 0)] - 12.0).abs() < 1e-9,
        "chain rule through variables must reach the Jacobian, got {}",
        matrix[&(0, 0)]
    );

    // Ieq = I - G*V = 18 - 36 = -18; rhs[p] -= Ieq => +18
    assert!((rhs[&0] - 18.0).abs() < 1e-9, "got rhs {:?}", rhs);
}

#[test]
fn conditional_branches_select_correct_equation() {
    let model = compile(
        r#"
`include "disciplines.vams"
module piecewise(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (V(p, n) > 1.0)
            I(p, n) <+ 2.0 * V(p, n);
        else
            I(p, n) <+ V(p, n);
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();
    let total: f64 = currents.iter().sum();
    assert!((total - 6.0).abs() < 1e-12, "V=3 selects the 2*V branch");

    let mut device = model.device("X2", &[1, 0]);
    device.update_voltages(&[0.5]);
    let currents = device.evaluate();
    let total: f64 = currents.iter().sum();
    assert!((total - 0.5).abs() < 1e-12, "V=0.5 selects the V branch");
}

#[test]
fn internal_node_voltage_is_readable() {
    let model = compile(
        r#"
`include "disciplines.vams"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r = 1.0 from (0:inf);
    analog begin
        I(p, mid) <+ (V(p) - V(mid)) / r;
        I(mid, n) <+ V(mid, n) / r;
    end
endmodule
"#,
    );
    assert_eq!(model.internal_nodes, 1);

    let mut device = model.device("D1", &[1, 0]);
    // Internal node mapped to circuit node 2
    device.set_internal_node_indices(&[2]);
    // node1 = 2 V, node2 (internal mid) = 0.5 V
    device.update_all_voltages(&[2.0, 0.5]);
    let currents = device.evaluate();

    // I(p,mid) = (2 - 0.5)/1 = 1.5 ; I(mid,n) = 0.5/1 = 0.5
    assert!((currents[0] - 1.5).abs() < 1e-12, "got {:?}", currents);
    assert!((currents[1] - 0.5).abs() < 1e-12, "got {:?}", currents);
}

#[test]
fn single_ended_access_references_global_ground() {
    // V(p) must read the potential of p against ground, NOT against
    // terminal 0 of the device
    let model = compile(
        r#"
`include "disciplines.vams"
module gprobe(a, b);
    inout a, b;
    electrical a, b;
    analog I(a, b) <+ V(b);
endmodule
"#,
    );
    // a -> node1, b -> node2
    let mut device = model.device("G1", &[1, 2]);
    device.update_voltages(&[5.0, 1.25]);
    let currents = device.evaluate();
    assert!(
        (currents[0] - 1.25).abs() < 1e-12,
        "V(b) must be 1.25 (potential vs ground), got {}",
        currents[0]
    );
}

#[test]
fn named_branch_evaluation() {
    let model = compile(
        r#"
`include "disciplines.vams"
module br_res(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) res;
    analog I(res) <+ V(res) / 4.0;
endmodule
"#,
    );
    let mut device = model.device("B1", &[1, 0]);
    device.update_voltages(&[2.0]);
    let currents = device.evaluate();
    assert!((currents[0] - 0.5).abs() < 1e-12);
}

#[test]
fn capacitor_ddt_backward_euler() {
    let model = compile(
        r#"
`include "disciplines.vams"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-6 from (0:inf);
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
    );
    let mut device = model.device("C1", &[1, 0]);

    // DC: charge recorded, current is zero
    device.update_voltages(&[1.0]);
    let dc = device.evaluate();
    assert!(dc[0].abs() < 1e-18, "ddt must be 0 at DC, got {}", dc[0]);
    device.advance_state();

    // Transient step: V goes 1.0 -> 2.0 over dt=1us
    // i = C*dV/dt = 1e-6 * 1.0 / 1e-6 = 1.0
    device.set_analysis_type(2);
    device.set_timestep(1e-6);
    device.update_voltages(&[2.0]);
    let tr = device.evaluate();
    assert!(
        (tr[0] - 1.0).abs() < 1e-9,
        "backward-Euler capacitor current, got {}",
        tr[0]
    );

    // Jacobian must contain the companion conductance C/dt = 1.0
    let (matrix, _rhs) = collect_stamps(&mut device, &[2.0]);
    assert!(
        (matrix[&(0, 0)] - 1.0).abs() < 1e-9,
        "companion conductance C/dt, got {}",
        matrix[&(0, 0)]
    );
}

#[test]
fn ddx_computes_partial_derivative() {
    let model = compile(
        r#"
`include "disciplines.vams"
module ddxm(p, n);
    inout p, n;
    electrical p, n;
    real g;
    analog begin
        g = ddx(V(p) * V(p), V(p));
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );
    let mut device = model.device("DX1", &[1, 0]);
    // V(p) = 3 => ddx(V(p)^2, V(p)) = 2*V(p) = 6; I = 6 * 3 = 18
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();
    assert!((currents[0] - 18.0).abs() < 1e-9, "got {}", currents[0]);
}

#[test]
fn ddx_accepts_named_branch_probe() {
    let model = compile(
        r#"
`include "disciplines.vams"
module ddxbr(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) sense;
    real g, h;
    analog begin
        g = ddx(V(<sense>) * V(<sense>), V(<sense>));
        h = ddx(V(sense) * V(sense), V(sense));
        I(p, n) <+ 0.5 * (g + h) * V(p, n);
    end
endmodule
"#,
    );
    let mut device = model.device("DXB1", &[1, 0]);
    device.update_voltages(&[3.0]);
    let currents = device.evaluate();

    model.observe(&mut device);
    assert!((device.variable("g").unwrap() - 6.0).abs() < 1e-12);
    assert!((device.variable("h").unwrap() - 6.0).abs() < 1e-12);
    assert!((currents[0] - 18.0).abs() < 1e-9, "got {}", currents[0]);
}

#[test]
fn voltage_contribution_stamps_branch_unknown() {
    let model = compile(
        r#"
`include "disciplines.vams"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real level = 1.5;
    analog V(p, n) <+ level;
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1, "one branch unknown");

    // p -> node1 (row 0), n -> node2 (row 1), branch unknown -> node3 (row 2)
    let mut device = model.device("V1", &[1, 2]);
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = collect_stamps(&mut device, &[0.0, 0.0, 0.0]);

    // Structural coupling: KCL rows gain the branch column, the branch row
    // reads the node potentials
    assert!((matrix[&(0, 2)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(1, 2)] + 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 0)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 1)] + 1.0).abs() < 1e-12);

    // Branch row RHS carries the source value: V(p) - V(n) = 1.5
    assert!((rhs[&2] - 1.5).abs() < 1e-12, "rhs: {rhs:?}");
}

#[test]
fn impedance_form_resistor_via_voltage_contribution() {
    // V(p,n) <+ I(p,n) * r is a resistor written in impedance form;
    // the branch row must read V(p) - V(n) - r*i_br = 0
    let model = compile(
        r#"
`include "disciplines.vams"
module zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1);

    let mut device = model.device("Z1", &[1, 2]);
    device.set_branch_current_indices(&[3]);

    // Branch current solution value 1 mA
    let (matrix, _rhs) = collect_stamps(&mut device, &[1.0, 0.5, 1e-3]);

    // Constitutive row: dE/di = r stamps -r at (branch, branch)
    assert!(
        (matrix[&(2, 2)] + 2000.0).abs() < 1e-9,
        "got {:?}",
        matrix.get(&(2, 2))
    );
}

#[test]
fn try_stamp_reports_missing_branch_current_solution_slot() {
    let model = compile(
        r#"
`include "disciplines.vams"
module zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    );

    let mut device = model.device("Z1", &[1, 2]);
    device.set_branch_current_indices(&[3]);

    let err = device
        .try_stamp(&[1.0, 0.5], |_, _, _| {}, |_, _| {})
        .expect_err("missing branch-current solution slot must be reported");

    assert!(
        err.to_string()
            .contains("missing branch-current solution slot"),
        "unexpected error: {err}"
    );
}

#[test]
fn mode_disabled_voltage_contribution_leaves_branch_open() {
    // A potential contribution under a parameter-only guard must leave
    // the branch OPEN when disabled, not short it to zero volts
    let model = compile(
        r#"
`include "disciplines.vams"
module modal(p, n);
    inout p, n;
    electrical p, n;
    parameter integer shorted = 0;
    analog begin
        if (shorted > 0)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
    );
    assert_eq!(model.branch_sources.len(), 1);

    // Disabled (default): branch row pinned to zero current, no coupling
    let mut device = model.device("M1", &[1, 2]);
    device.set_branch_current_indices(&[3]);
    let (matrix, _) = collect_stamps(&mut device, &[1.0, 0.0, 0.0]);
    assert!((matrix[&(2, 2)] - 1.0).abs() < 1e-12, "identity pin");
    assert!(!matrix.contains_key(&(0, 2)), "no KCL coupling when open");

    // Enabled: structural short V(p)-V(n)=0
    let mut device = model.device("M2", &[1, 2]);
    device.set_branch_current_indices(&[3]);
    device.set_parameter("shorted", 1.0);
    device.resolve_parameter_defaults();
    let (matrix, _) = collect_stamps(&mut device, &[1.0, 0.0, 0.0]);
    assert!((matrix[&(0, 2)] - 1.0).abs() < 1e-12);
    assert!((matrix[&(2, 0)] - 1.0).abs() < 1e-12);
}

#[test]
fn user_function_device_evaluates() {
    let model = compile(
        r#"
`include "disciplines.vams"
module fres(p, n);
    inout p, n;
    electrical p, n;
    analog function real conduct;
        input v;
        begin
            if (v > 0.0)
                conduct = 2.0 * v;
            else
                conduct = v;
        end
    endfunction
    analog I(p, n) <+ conduct(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("F1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 4.0).abs() < 1e-12);
    device.update_voltages(&[-1.0]);
    assert!((device.evaluate()[0] + 1.0).abs() < 1e-12);
}

#[test]
fn dependent_parameter_defaults_track_overrides() {
    let model = compile(
        r#"
`include "disciplines.vams"
module wres(p, n);
    inout p, n;
    electrical p, n;
    parameter real w = 1.0 from (0:inf);
    parameter real rs = 10.0 / w from (0:inf);
    analog I(p, n) <+ V(p, n) / rs;
endmodule
"#,
    );

    // Default w=1 => rs defaults to 10; I = V/10
    let mut device = model.device("W1", &[1, 0]);
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);

    // Override w=2 => rs default must recompute to 5; I = V/5
    let mut device = model.device("W2", &[1, 0]);
    device.set_parameter("w", 2.0);
    device.resolve_parameter_defaults();
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 1.0).abs() < 1e-12);

    // Explicit rs wins over its default regardless of w
    let mut device = model.device("W3", &[1, 0]);
    device.set_parameter("w", 2.0);
    device.set_parameter("rs", 50.0);
    device.resolve_parameter_defaults();
    device.update_voltages(&[5.0]);
    assert!((device.evaluate()[0] - 0.1).abs() < 1e-12);
}

#[test]
#[cfg(not(feature = "native"))]
fn try_new_reports_dependent_parameter_default_runtime_errors() {
    let mut model = compile(
        r#"
`include "disciplines.vams"
module bad_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real w = 1.0 from (0:inf);
    parameter real r = 10.0 / w from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );
    model.parameters[1].default_program = Some(BytecodeProgram {
        instructions: vec![Instruction::PushParam(99)],
    });

    let err = model
        .try_device("BD1", &[1, 0])
        .expect_err("checked construction must report invalid dependent parameter defaults");
    let text = err.to_string();
    assert!(
        text.contains("parameter") || text.contains("Invalid instruction"),
        "diagnostic should identify the dependent default failure, got: {text}"
    );
}

#[test]
fn param_given_reflects_instance_overrides() {
    let model = compile(
        r#"
`include "disciplines.vams"
module pg(p, n);
    inout p, n;
    electrical p, n;
    parameter real rknob = 1.0 from (0:inf);
    real geff;
    analog begin
        if ($param_given(rknob))
            geff = 1.0 / rknob;
        else
            geff = 0.25;
        I(p, n) <+ geff * V(p, n);
    end
endmodule
"#,
    );

    // Not given: the model's fallback conductance applies
    let mut device = model.device("P1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);

    // Given: the explicit value applies even though it equals the default
    let mut device = model.device("P2", &[1, 0]);
    device.set_parameter("rknob", 1.0);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 2.0).abs() < 1e-12);
}

#[test]
fn port_connected_reflects_omitted_trailing_terminal() {
    let model = compile(
        r#"
`include "disciplines.vams"

module optional_port_probe(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    analog I(p, n) <+ ($port_connected(opt) ? 10.0 : 1.0) * V(p, n);
endmodule
"#,
    );

    let mut omitted = model.device("X1", &[1, 0]);
    omitted.update_voltages(&[2.0]);
    assert!((omitted.evaluate()[0] - 2.0).abs() < 1e-12);

    let mut grounded = model.device("X2", &[1, 0, 0]);
    grounded.update_voltages(&[2.0]);
    assert!((grounded.evaluate()[0] - 20.0).abs() < 1e-12);
}

#[test]
fn runtime_array_index_errors_do_not_evaluate_as_zero() {
    let model = compile(
        r#"
`include "disciplines.vams"

module runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    parameter integer nseg = 5;
    real w[1:4];
    integer i;
    real total;
    analog begin
        total = 0.0;
        for (i = 1; i <= nseg; i = i + 1) begin
            w[i] = 0.001 * i;
            total = total + w[i];
        end
        I(p, n) <+ total * V(p, n);
    end
endmodule
"#,
    );

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut device = model.device("X1", &[1, 0]);
        device.update_voltages(&[1.0]);
        let _ = device.evaluate();
    }));

    assert!(
        result.is_err(),
        "runtime array bounds errors must not be converted into a numeric current"
    );
}

#[test]
fn try_stamp_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module stamp_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    let err = device
        .try_stamp(&[1.0], |_, _, _| {}, |_, _| {})
        .expect_err("checked stamping must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn try_compute_jacobian_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module jac_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-3;
        I(p, n) <+ w[i] * V(p, n);
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[1.0]);
    let err = device
        .try_compute_jacobian()
        .expect_err("checked Jacobian evaluation must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn try_stamp_reactive_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module reactive_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = (V(p, n) > 0.5) ? 5 : 1;
        w[i] = 1.0e-6;
        I(p, n) <+ ddt(w[i] * V(p, n));
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.set_analysis_type(1);
    let err = device
        .try_stamp_reactive(&[1.0], |_, _, _| {})
        .expect_err("checked reactive stamping must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn try_noise_sources_reports_runtime_array_index_errors() {
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_runtime_oob(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = analysis("noise") ? 5 : 1;
        w[i] = 1.0e-18;
        I(p, n) <+ V(p, n) * 1.0e-3 + white_noise(w[i], "bad");
    end
endmodule
"#,
    );

    // The index is in range everywhere except noise, so the instance is a
    // working device for every other analysis and must construct.
    let device = model
        .try_device("X1", &[1, 0])
        .expect("an index that only leaves the array under noise must still construct");

    // Selecting noise is where the model leaves its array, and which call
    // notices depends on the backend: the interpreter re-evaluates static
    // conditions as the analysis type changes, while the native backend gets
    // there during the evaluation itself. Either landing is the same fault, so
    // take the whole switch-and-evaluate as one step.
    let mut probe = device.clone();
    let err = probe
        .try_set_analysis_type(3)
        .and_then(|()| probe.try_noise_sources(&[0.0]).map(|_| ()))
        .expect_err("checked noise evaluation must report runtime array bounds errors");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );

    // The grouped path is the one the engine uses, and it must report the
    // model's own fault rather than the planning failure behind it.
    let mut probe = device.clone();
    let err = probe
        .try_set_analysis_type(3)
        .and_then(|()| {
            probe
                .try_noise_processes_at_frequency(&[0.0], 1.0e3)
                .map(|_| ())
        })
        .expect_err("the grouped path must report the same runtime array bounds error");
    let text = err.to_string();
    assert!(
        text.contains("Array index 5") || text.contains("[1:4]"),
        "diagnostic should identify the runtime array bounds error, got: {text}"
    );
}

#[test]
fn grouped_noise_without_a_runtime_plan_fails_closed_at_noise_time() {
    // Same unlowerable shape as above, but every index the model can produce
    // is inside the array, so nothing faults at run time. What is left is the
    // planning failure itself, and it must surface as an error rather than as
    // PSDs taken from the scalar path, which has neither the CFG's activation
    // nor its reaching definitions.
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_runtime_indexed(p, n);
    inout p, n;
    electrical p, n;
    real w[1:4];
    integer i;
    analog begin
        i = analysis("noise") ? 3 : 1;
        w[i] = 1.0e-18;
        I(p, n) <+ V(p, n) * 1.0e-3 + white_noise(w[i], "bad");
    end
endmodule
"#,
    );

    let mut device = model
        .try_device("X1", &[1, 0])
        .expect("runtime-indexed noise metadata must not block construction");

    // Everything that does not read noise metadata still works.
    device.update_voltages(&[1.0]);
    device
        .try_compute_jacobian()
        .expect("the operating point is unaffected by unlowerable noise metadata");

    device
        .try_set_analysis_type(3)
        .expect("noise analysis configures");
    let err = device
        .try_noise_processes_at_frequency(&[0.0], 1.0e3)
        .expect_err("an unlowerable grouped-noise plan must fail closed at noise time");
    let text = err.to_string();
    assert!(
        text.contains("canonical grouped-noise CFG lowering failed")
            && text.contains("run-time array index")
            && text.contains("array access expression"),
        "diagnostic should identify every unsupported runtime-indexed metadata operation, got: {text}"
    );
}

#[test]
fn analysis_aliases_match_generated_runtime_semantics() {
    let model = compile(
        r#"
`include "disciplines.vams"

module analysis_aliases(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ analysis("op")
        + 2.0 * analysis("smallsig")
        + 4.0 * analysis("smallsignal")
        + 8.0 * analysis("small_signal");
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    for (analysis_type, expected) in [(0, 1.0), (1, 14.0), (2, 0.0), (3, 14.0), (4, 0.0)] {
        device.set_analysis_type(analysis_type);
        device.update_voltages(&[0.0]);
        let currents = device.evaluate();
        assert_eq!(
            currents[0].to_bits(),
            f64::to_bits(expected),
            "analysis_type: {analysis_type}, currents: {currents:?}"
        );
    }
}

#[test]
fn analysis_queries_and_event_filters_preserve_physical_identity_across_phases() {
    use rspice_veriloga_runtime::{AnalogAnalysisPhase, analysis_query_mask};
    let model = compile(
        r#"module phases(p,n);
inout p,n; electrical p,n;
real event_scope;
analog begin
    @(initial_step("tran")) event_scope=1;
    @(initial_step("ic")) event_scope=2;
    @(initial_step("static")) event_scope=4;
    I(p,n)<+analysis("dc") + 2*analysis("ac") + 4*analysis("tran")
        + 8*analysis("noise") + 16*analysis("ic") + 32*analysis("static")
        + 64*analysis("smallsig") + 512*analysis("nodeset")
        + 1024*event_scope + 8192*analysis("unknown_analysis");
end
endmodule"#,
    );
    for analysis in 0..=4 {
        let mut device = model.device("X1", &[1, 0]);
        device.try_begin_analysis(analysis).unwrap();
        for phase in [
            AnalogAnalysisPhase::Point,
            AnalogAnalysisPhase::Equilibrium,
            AnalogAnalysisPhase::Nodeset,
        ] {
            device.try_set_analysis_phase(phase).unwrap();
            device.try_set_analysis_step(true, false).unwrap();
            device.update_voltages(&[0.0]);
            let expected = (analysis_query_mask(analysis, phase, false, false) & 0x3ff)
                + match analysis {
                    2 => 1024,
                    4 => 2048,
                    _ => 0,
                };
            assert_eq!(
                device.evaluate()[0],
                f64::from(expected),
                "analysis {analysis}, phase {phase:?}"
            );
        }
    }
}

#[test]
fn phase_change_does_not_replay_analog_initial() {
    use rspice_veriloga_runtime::AnalogAnalysisPhase::{Equilibrium, Point};
    let model = compile(
        r#"module initialize_phase(p,n);
inout p,n; electrical p,n; real saved;
analog initial saved=analysis("static") ? 100 : 200;
analog I(p,n)<+saved+analysis("static");
endmodule"#,
    );
    let mut device = model.device("X1", &[1, 0]);
    device.try_begin_analysis_in_phase(1, Equilibrium).unwrap();
    device.update_voltages(&[0.0]);
    assert_eq!(device.evaluate()[0], 101.0);
    device.try_set_analysis_phase(Point).unwrap();
    assert_eq!(device.evaluate()[0], 100.0);
    device.try_begin_analysis_in_phase(1, Point).unwrap();
    assert_eq!(device.evaluate()[0], 200.0);
}

#[test]
fn try_noise_sources_preserves_flicker_and_table_metadata() {
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_metadata(p, n);
    inout p, n;
    electrical p, n;
    parameter real s = 1.0e-18;
    parameter real ex = 2.0;
    analog begin
        I(p, n) <+ flicker_noise(s, ex, "fl");
        I(p, n) <+ noise_table('{1.0, 2.0e-18, 10.0, 4.0e-18}, "tbl");
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.set_analysis_type(3);
    let sources = device
        .try_noise_sources(&[0.0])
        .expect("checked noise metadata");

    let flicker = sources
        .iter()
        .find(|source| source.name == "fl")
        .expect("flicker source");
    assert!((flicker.psd - 1.0e-18).abs() < 1.0e-30);
    assert_eq!(flicker.exponent, Some(2.0));
    assert!(flicker.table.is_none());

    let table = sources
        .iter()
        .find(|source| source.name == "tbl")
        .expect("table source");
    assert_eq!(table.psd, 1.0);
    assert_eq!(
        table
            .table
            .as_ref()
            .map(|(points, log)| (points.len(), *log)),
        Some((2, false))
    );
}

#[test]
fn try_noise_sources_evaluates_current_probe_psd() {
    let model = compile(
        r#"
`include "disciplines.vams"

module noise_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n) * 2.0e-3
            + white_noise(abs(I(p, n)) * 4.0, "shot");
    end
endmodule
"#,
    );

    let mut device = model.device("X1", &[1, 0]);
    device.set_internal_node_indices(&[2]);
    device.set_analysis_type(3);
    let sources = device
        .try_noise_sources(&[3.0, -0.006])
        .expect("checked current-probe noise evaluation");

    let shot = sources
        .iter()
        .find(|source| source.name == "shot")
        .expect("shot noise source");
    assert_eq!(shot.node_pos, 2);
    assert_eq!(shot.node_neg, 0);
    assert!((shot.psd - 2.4e-2).abs() < 1.0e-15, "shot={shot:?}");
}

#[test]
fn idtmod_wraps_the_integral() {
    // Phase accumulator: phi = idtmod(rate, 0, 1) folds into [0, 1)
    let model = compile(
        r#"
`include "disciplines.vams"
module phase(p, n);
    inout p, n;
    electrical p, n;
    parameter real rate = 1.0e6;
    real phi;
    analog begin
        phi = idtmod(rate, 0.0, 1.0);
        I(p, n) <+ phi * 1.0e-3;
    end
endmodule
"#,
    );
    let mut device = model.device("PH1", &[1, 0]);
    device.update_voltages(&[0.0]);

    // DC: integral sits at its initial condition
    assert!(device.evaluate()[0].abs() < 1e-18);
    device.advance_state();

    // rate * dt = 0.25 per step; the fourth step wraps 1.0 -> 0.0
    device.set_analysis_type(2);
    device.set_timestep(0.25e-6);
    let mut phases = Vec::new();
    for _ in 0..6 {
        let current = device.evaluate()[0];
        phases.push(current / 1.0e-3);
        device.advance_state();
    }
    let expected = [0.25, 0.5, 0.75, 0.0, 0.25, 0.5];
    for (i, (got, want)) in phases.iter().zip(expected).enumerate() {
        assert!(
            (got - want).abs() < 1e-9,
            "step {i}: phase {got} != {want} (all: {phases:?})"
        );
    }
}

#[test]
fn jacobian_matches_finite_difference_for_diode() {
    let model = compile(
        r#"
`include "disciplines.vams"
module diode(a, c);
    inout a, c;
    electrical a, c;
    parameter real is_sat = 1e-14 from (0:inf);
    analog I(a, c) <+ is_sat * (limexp(V(a, c) / $vt) - 1.0);
endmodule
"#,
    );

    let bias = 0.6;
    let delta = 1e-7;

    let mut device = model.device("D1", &[1, 0]);
    device.update_voltages(&[bias]);
    let i0 = device.evaluate()[0];
    let (matrix, _) = collect_stamps(&mut device, &[bias]);
    let g_analytic = matrix[&(0, 0)];

    let mut device2 = model.device("D2", &[1, 0]);
    device2.update_voltages(&[bias + delta]);
    let i1 = device2.evaluate()[0];

    let g_fd = (i1 - i0) / delta;
    let rel_err = ((g_analytic - g_fd) / g_fd).abs();
    assert!(
        rel_err < 1e-3,
        "analytic {} vs finite-difference {} (rel err {})",
        g_analytic,
        g_fd,
        rel_err
    );
}

#[test]
fn ac_intermediate_range_survives_indexed_assignments_and_multiplicity() {
    for gain in [1e-200, 1e200] {
        let fixture = compile(&format!(
            "module ranged_assignment(p); inout p; electrical p;
             real r[0:1]; integer k; analog begin
             r[0]={gain:e}*(V(p)+ddt(V(p)));
             for(k=0;k<1;k=k+1) r[k+1]=r[k];
             I(p)<+{gain:e}/r[1]; end endmodule"
        ));
        for omega in [1e-200, 1e200] {
            let mut device = fixture.device("RANGE", &[1]);
            device.set_analysis_type(1);
            let mut admittance = (0.0, 0.0);
            device
                .try_stamp_small_signal_complex(
                    &[0.01],
                    omega / std::f64::consts::TAU,
                    |_, _, real, imaginary| {
                        admittance.0 += real;
                        admittance.1 += imaginary;
                    },
                )
                .unwrap();
            assert!((admittance.0 / -1e4 - 1.0).abs() < 1e-12);
            assert!((admittance.1 / (-1e4 * omega) - 1.0).abs() < 1e-12);
        }

        let fixture = compile(&format!(
            "module ranged_multiplicity(p); inout p; electrical p;
             analog I(p)<+{gain:e}*(V(p)+ddt(V(p))); endmodule"
        ));
        let mut device = fixture.device("MULTIPLICITY", &[1]);
        device.set_analysis_type(1);
        device.try_set_multiplicity(1.0 / gain).unwrap();
        let mut admittance = (0.0, 0.0);
        device
            .try_stamp_small_signal_complex(
                &[0.01],
                gain / std::f64::consts::TAU,
                |_, _, real, imaginary| {
                    admittance.0 += real;
                    admittance.1 += imaginary;
                },
            )
            .unwrap();
        assert!((admittance.0 - 1.0).abs() < 1e-12);
        assert!((admittance.1 / gain - 1.0).abs() < 1e-12);
        if gain > 1.0 {
            device.try_set_multiplicity(1.0).unwrap();
            let mut published = 0;
            assert!(
                device
                    .try_stamp_small_signal_complex(
                        &[0.01],
                        gain / std::f64::consts::TAU,
                        |_, _, _, _| published += 1,
                    )
                    .is_err()
            );
            assert_eq!(
                published, 0,
                "unrepresentable final entries must not be published"
            );
        }
    }
}

#[test]
fn noise_only_integrator_metadata_uses_its_canonical_state_sites() {
    for (index, body) in [
        r#"analog I(p,n)<+V(p,n)
    +white_noise(1.0+ddt(V(p,n)),"white")
    +flicker_noise(2.0+idt(V(p,n),3.0),3.0+ddt(2.0*V(p,n)),"flicker");"#,
        r#"analog begin
    w=white_noise(1.0+ddt(V(p,n)),"white");
    f=flicker_noise(2.0+idt(V(p,n),3.0),3.0+ddt(2.0*V(p,n)),"flicker");
    I(p,n)<+V(p,n)+w+f;
end"#,
        r#"analog I(p,n)<+ddt(V(p,n))
    +white_noise(1.0+ddt(V(p,n)),"white")
    +flicker_noise(2.0+idt(V(p,n),3.0),3.0+ddt(2.0*V(p,n)),"flicker");"#,
        r#"analog begin
    w=white_noise(1.0+ddt(V(p,n)),"white");
    f=flicker_noise(2.0+idt(V(p,n),3.0),3.0+ddt(2.0*V(p,n)),"flicker");
    q=ddt(V(p,n));
    I(p,n)<+q+w+f;
end"#,
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = compile(&format!(
            "module noise_only_state(p,n); inout p,n; electrical p,n; real w,f,q; {body} endmodule"
        ));
        let layout = rspice_veriloga::canonical_ir::CanonicalStateLayout::from_hir(
            &fixture.canonical_ir.hir,
        );
        let integral = layout
            .sites()
            .iter()
            .find(|site| site.kind == rspice_veriloga::canonical_ir::CanonicalStateOperator::Idt)
            .unwrap();
        assert!(fixture.noise_sources[1].psd_program.instructions.iter().any(|instruction| {
            matches!(instruction, rspice_veriloga::codegen::Instruction::IdtState(slot) if *slot == integral.slot as usize)
        }));
        let mut device = fixture.device("NOISE", &[1, 0]);
        device.try_set_analysis_type(3).unwrap();
        for frequency in [0.0, 1.0, 1e6] {
            let processes = device
                .try_noise_processes_at_frequency(&[2.0], frequency)
                .unwrap();
            assert_eq!(processes.len(), 2);
            assert_eq!(processes[0].psd, 1.0);
            assert_eq!(processes[0].exponent, None);
            assert_eq!(processes[1].psd, 5.0);
            assert_eq!(processes[1].exponent, Some(3.0));
        }
        device.try_set_analysis_type(0).unwrap();
        device.update_voltages(&[2.0]);
        device.try_evaluate().unwrap();
        device.try_advance_state().unwrap();
        device.try_set_analysis_type(2).unwrap();
        device.set_timestep(0.25);
        device.update_voltages(&[4.0]);
        assert_eq!(
            device.try_evaluate().unwrap()[0],
            if index < 2 { 4.0 } else { 8.0 }
        );
    }
}

#[test]
fn noise_powers_freeze_dynamic_values_without_touching_history() {
    for (dynamic, dc_value) in [
        ("ddt(V(p,n))", 0.0),
        ("idt(V(p,n),3.0)", 3.0),
        ("ddt(idt(V(p,n),3.0))", 0.0),
        ("idt(ddt(V(p,n)),3.0)", 3.0),
    ] {
        let fixture = compile(&format!(
            "module dynamic_noise_power(p,n); inout p,n; electrical p,n; real d; analog begin d={dynamic}; I(p,n)<+V(p,n)+d+white_noise(limexp(V(p,n)+d),\"input\"); end endmodule"
        ));
        let mut device = fixture.device("NOISE", &[1, 0]);
        device.update_voltages(&[2.0]);
        device.try_evaluate().unwrap();
        device.advance_state();
        device.try_set_analysis_type(3).unwrap();
        let history = device.checkpoint_state().unwrap();
        // The deterministic circuit contribution can itself contain an
        // integrator, whose small-signal transfer has a pole at zero frequency.
        for frequency in [1.0, 1e3, 1e6] {
            let processes = device
                .try_noise_processes_at_frequency(&[2.0], frequency)
                .unwrap_or_else(|error| panic!("{dynamic}: {error}"));
            assert_eq!(processes.len(), 1);
            let expected = (2.0_f64 + dc_value).exp();
            assert!((processes[0].psd / expected - 1.0).abs() <= 8.0 * f64::EPSILON);
            assert_eq!(device.checkpoint_state().unwrap(), history);
        }
    }
}

#[test]
fn integration_history_is_read_only_after_the_small_signal_operating_point() {
    use rspice_veriloga_runtime::AnalogAnalysisPhase::{Equilibrium, Point};
    for (operator, expected) in [
        ("ddt(V(p,n))", 0.0),
        ("idt(V(p,n),3.0)", 3.0),
        ("idtmod(V(p,n),3.0,2.0,0.0)", 1.0),
    ] {
        let fixture = compile(&format!(
            "module readonly_integrator(p,n); inout p,n; electrical p,n; analog I(p,n)<+{operator}; endmodule"
        ));
        for analysis in [1, 3] {
            let mut device = fixture.device("STATE", &[1, 0]);
            device
                .try_begin_analysis_in_phase(analysis, Equilibrium)
                .unwrap();
            device.update_voltages(&[2.0]);
            assert_eq!(device.try_evaluate().unwrap()[0], expected);
            device.try_advance_state().unwrap();
            device.try_set_analysis_phase(Point).unwrap();
            let accepted = device.checkpoint_state().unwrap();
            assert!(
                accepted
                    .accepted
                    .state_initialized
                    .iter()
                    .any(|&initialized| initialized)
            );
            for voltage in [3.0, 4.0] {
                device.update_voltages(&[voltage]);
                assert_eq!(device.try_evaluate().unwrap()[0], expected);
                assert_eq!(
                    device.checkpoint_state().unwrap(),
                    accepted,
                    "{operator}, analysis {analysis}"
                );
            }
        }
    }
}

#[test]
fn noise_injection_range_is_preserved_until_multiplicity_scaling() {
    for (gain, omega, multiplicity) in [(1e150, 1e200, 1e-200_f64), (1e-150, 1e-200, 1e200_f64)] {
        let fixture = compile(&format!(
            "module ranged_noise(p); inout p; electrical p; real x,y;
             analog begin x=white_noise(1.0,\"source\"); y={gain:e}*x;
             I(p)<+ddt(y); end endmodule"
        ));
        let mut device = fixture.device("NOISE", &[1]);
        device.set_analysis_type(3);
        device.try_set_multiplicity(multiplicity).unwrap();
        let processes = device
            .try_noise_processes_at_frequency(&[0.01], omega / std::f64::consts::TAU)
            .unwrap();
        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0].injections.len(), 1);
        assert_eq!(processes[0].psd, 1.0);
        let actual = processes[0].injections[0].gain;
        let expected = (gain * multiplicity.sqrt()) * omega;
        assert_eq!(actual.re, 0.0);
        assert!(
            (actual.im.abs() / expected - 1.0).abs() < 1e-12,
            "{actual:?} != {expected}"
        );
    }
}
