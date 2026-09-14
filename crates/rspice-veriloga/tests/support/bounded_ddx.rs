//! A parameter range bounds derivative storage without fixing its default.

use super::support::DeviceFixture;

fn source(range: &str, header: &str, body: &str) -> String {
    include_str!("../fixtures/bounded_derivative_loop.va")
        .replace("from [-2:6]", range)
        .replace("k=0;k<order;k=k+1", header)
        .replace("a[slot]=ddx(a[slot],V(p,n));", body)
}

#[test]
fn bounded_recursive_ddx_keeps_parameter_overrides_and_loop_exit() {
    for (range, header, cases) in [
        (
            "from [-2:6]",
            "k=0;k<order;k=k+1",
            vec![
                (-2, 0, 0),
                (0, 0, 0),
                (1, 1, 1),
                (4, 4, 4),
                (6, 6, 6),
                (2, 2, 2),
            ],
        ),
        (
            "from (-1:6)",
            "k=0;k<=order;k=k+2",
            vec![(0, 1, 2), (4, 3, 6), (5, 3, 6), (2, 2, 4)],
        ),
        (
            "from [-2:6]",
            "k=4;k>=order;k=k-2",
            vec![(6, 0, 4), (4, 1, 2), (-2, 4, -4), (1, 2, 0)],
        ),
    ] {
        let fixture =
            DeviceFixture::compile(&source(range, header, "a[slot]=ddx(a[slot],V(p,n));"));
        let mut device = fixture.device("BOUNDED", &[1, 0]);
        #[cfg(feature = "native")]
        assert!(device.is_using_native());
        for (order, iterations, exit) in cases {
            device.try_set_parameter("order", f64::from(order)).unwrap();
            device.try_resolve_parameter_defaults().unwrap();
            for slot in [0, 1] {
                device.try_set_parameter("slot", f64::from(slot)).unwrap();
                for voltage in [-0.4_f64, 0.0, 0.3] {
                    device.update_voltages(&[voltage]);
                    let expected = 2.0_f64.powi(iterations) * (2.0 * voltage).exp();
                    let current = device.try_evaluate().unwrap()[0];
                    assert!(
                        (current - expected).abs() < 1e-10 * expected,
                        "{header}: order={order}, slot={slot}, V={voltage}: {current} != {expected}"
                    );
                    let mut conductance = 0.0;
                    device.stamp(
                        &[voltage],
                        &mut |row, col, value| {
                            if row == 0 && col == 0 {
                                conductance += value;
                            }
                        },
                        &mut |_, _| {},
                    );
                    assert!(
                        (conductance - 2.0 * expected).abs() < 1e-10 * expected,
                        "{header}: order={order}: Jacobian={conductance}"
                    );
                    fixture.observe(&mut device);
                    assert_eq!(device.variable("k"), Some(f64::from(exit)));
                }
            }
        }
    }
}

#[test]
fn a_default_or_a_mutable_counter_does_not_prove_a_derivative_bound() {
    for (range, header, body) in [
        ("", "k=0;k<order;k=k+1", "a[slot]=ddx(a[slot],V(p,n));"),
        (
            "from [0:6]",
            "k=0;k<order;k=k+1",
            "a[slot]=ddx(a[slot],V(p,n)); k=k-1;",
        ),
        (
            "from [0:6]",
            "k=0;k<order;k=k-1",
            "a[slot]=ddx(a[slot],V(p,n));",
        ),
        (
            "from [0:6]",
            "k=0;k<V(p,n);k=k+1",
            "a[slot]=ddx(a[slot],V(p,n));",
        ),
        (
            "from [0:33]",
            "k=0;k<order;k=k+1",
            "a[slot]=ddx(a[slot],V(p,n));",
        ),
    ] {
        let error = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(&source(range, header, body), None)
            .unwrap_err()
            .to_string();
        assert!(error.contains("unbounded derivative order"), "{error}");
    }
}

#[test]
fn bounded_ddx_counter_initialization_uses_integer_assignment_rounding() {
    let fixture = DeviceFixture::compile(
        &source(
            "from [-2:-0.8]",
            "k=-0.6;k<=order;k=k+1",
            "a[slot]=ddx(a[slot],V(p,n));",
        )
        .replace("order=4", "order=-1"),
    );
    let mut device = fixture.device("ROUNDED", &[1, 0]);
    device.update_voltages(&[0.0]);
    assert_eq!(device.try_evaluate().unwrap(), vec![2.0]);
    fixture.observe(&mut device);
    assert_eq!(device.variable("k"), Some(0.0));
}

#[test]
fn bounded_ddx_checks_ranges_without_changing_the_previous_parameter() {
    let fixture = DeviceFixture::compile(include_str!("../fixtures/bounded_derivative_loop.va"));
    let mut device = fixture.device("RANGES", &[1, 0]);
    for (assignment, expected) in [(0.5, 2.0), (3.5, 16.0), (5.5, 64.0), (-1.5, 1.0)] {
        device.try_set_parameter("order", assignment).unwrap();
        device.try_resolve_parameter_defaults().unwrap();
        for invalid in [-3.0, 7.0, f64::NAN, f64::INFINITY] {
            assert!(device.try_set_parameter("order", invalid).is_err());
            device.update_voltages(&[0.0]);
            assert_eq!(device.try_evaluate().unwrap(), vec![expected]);
        }
    }
}

#[test]
fn an_output_function_cannot_invalidate_a_bounded_ddx_counter_proof() {
    let source = source("from [0:6]", "k=0;k<order;k=k+1", "a[slot]=rewind(ddx(a[slot],V(p,n)),k);")
        .replace("analog begin", "analog function real rewind; input x; inout counter; real x; integer counter; begin counter=counter-1; rewind=x; end endfunction\nanalog begin");
    let error = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(&source, None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("unbounded derivative order"), "{error}");
}

#[test]
fn bounded_ddx_preserves_body_guards_and_counter_reads() {
    let fixture = DeviceFixture::compile(&source(
        "from [0:6]",
        "k=0;k<order;k=k+1",
        "if(V(p,n)>0) a[slot]=(k+1)*ddx(a[slot],V(p,n));",
    ));
    let mut device = fixture.device("GUARDED", &[1, 0]);
    for order in [0_i32, 1, 4, 6, 2] {
        device.try_set_parameter("order", f64::from(order)).unwrap();
        for voltage in [-0.4_f64, 0.0, 0.3] {
            let gain = if voltage > 0.0 {
                2.0_f64.powi(order) * f64::from((1..=order).product::<i32>())
            } else {
                1.0
            };
            let expected = gain * (2.0 * voltage).exp();
            device.update_voltages(&[voltage]);
            assert!((device.try_evaluate().unwrap()[0] - expected).abs() < 1e-10 * expected);
            fixture.observe(&mut device);
            assert_eq!(device.variable("k"), Some(f64::from(order)));
        }
    }
}

#[cfg(any(not(feature = "native"), feature = "native-bytecode-contract-tests"))]
#[test]
fn bounded_ddx_executes_through_the_postfix_device() {
    let fixture = DeviceFixture::compile(include_str!("../fixtures/bounded_derivative_loop.va"));
    let mut device = fixture.try_bytecode_device("POSTFIX", &[1, 0]).unwrap();
    for order in [0_i32, 1, 4, 6, 2, -2] {
        device.try_set_parameter("order", f64::from(order)).unwrap();
        for voltage in [-0.4_f64, 0.0, 0.3] {
            let expected = 2.0_f64.powi(order.max(0)) * (2.0 * voltage).exp();
            device.update_voltages(&[voltage]);
            assert!((device.try_evaluate().unwrap()[0] - expected).abs() < 1e-10 * expected);
            let mut conductance = 0.0;
            device.stamp(
                &[voltage],
                &mut |row, col, value| {
                    if row == 0 && col == 0 {
                        conductance += value;
                    }
                },
                &mut |_, _| {},
            );
            assert!((conductance - 2.0 * expected).abs() < 1e-10 * expected);
        }
    }
}

#[test]
fn bounded_ddx_event_candidates_preserve_acceptance_and_observation() {
    let fixture = DeviceFixture::compile(
        "module held(p,n); inout p,n; electrical p,n;
        parameter integer order=4 from [0:6]; real x; integer k;
        analog begin @(initial_step) begin x=exp(2*V(p,n));
        for(k=0;k<order;k=k+1) x=ddx(x,V(p,n)); end I(p,n)<+x; end endmodule",
    );
    for order in [0_i32, 1, 4, 6, 2] {
        let mut device = fixture.device("HELD", &[1, 0]);
        device.try_set_parameter("order", f64::from(order)).unwrap();
        device.try_set_analysis_step(true, false).unwrap();
        for voltage in [-0.4_f64, 0.0, 0.3] {
            let expected = 2.0_f64.powi(order) * (2.0 * voltage).exp();
            device.update_voltages(&[voltage]);
            assert!((device.try_evaluate().unwrap()[0] - expected).abs() < 1e-10 * expected);
            let before = device.checkpoint_state().unwrap();
            fixture.observe(&mut device);
            assert!((device.variable("x").unwrap() - expected).abs() < 1e-10 * expected);
            assert_eq!(device.checkpoint_state().unwrap(), before);
        }
        device.try_advance_state().unwrap();
        device.try_set_analysis_step(false, false).unwrap();
        device.update_voltages(&[-0.7]);
        let held = 2.0_f64.powi(order) * 0.6_f64.exp();
        assert!((device.try_evaluate().unwrap()[0] - held).abs() < 1e-10 * held);
        let mut conductance = 0.0;
        device.stamp(
            &[-0.7],
            &mut |row, col, value| {
                if row == 0 && col == 0 {
                    conductance += value;
                }
            },
            &mut |_, _| {},
        );
        assert_eq!(conductance, 0.0, "accepted sample has zero live derivative");
    }
}
