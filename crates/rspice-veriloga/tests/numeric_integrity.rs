mod support;

use rspice_veriloga::vm::VmError;
use support::DeviceFixture;

#[test]
fn switched_sources_retain_the_last_kind_and_expose_the_solved_current() {
    for (body, conductance) in [
        ("if (mode) V(p)<+2*I(p); else I(p)<+3*V(p);", 3.0),
        ("if (!mode) V(p)<+2*I(p); else I(p)<+3*V(p);", 0.5),
        ("if (V(p)>0) V(p)<+2*I(p); else I(p)<+3*V(p);", 0.5),
        ("if (V(p)<0) V(p)<+2*I(p); else I(p)<+3*V(p);", 3.0),
        (
            "if (mode>0) V(p)<+2*I(p); else if(mode<0) I(p)<+3*V(p);",
            0.0,
        ),
        (
            "value=gain; V(p)<+value*I(p); value=5; I(p)<+value*V(p); value=3; V(p)<+value*I(p);",
            1.0 / 3.0,
        ),
        ("I(p)<+5*V(p); for(j=0;j<2;j=j+1) V(p)<+gain*I(p);", 0.25),
        (
            "V(p)<+2*I(p); I(p)<+5*V(p); V(p)<+3*I(p); V(p)<+4*I(p);",
            1.0 / 7.0,
        ),
        (
            "I(p)<+2*V(p); V(p)<+3*I(p); I(p)<+4*V(p); I(p)<+5*V(p);",
            9.0,
        ),
        (
            "V(0,p)<+2*I(0,p); I(p)<+5*V(p); V(0,p)<+3*I(0,p); V(p)<+4*I(p);",
            1.0 / 7.0,
        ),
    ] {
        let source = format!(
            "module switched(p,q); inout p,q; electrical p,q; parameter integer mode=0; localparam real gain=mode+2; real value; integer j; analog begin I(q)<+I(p); {body} end endmodule"
        );
        let fixture = DeviceFixture::compile(&source);
        let internal_end = 2 + fixture.internal_nodes;
        let dimension = internal_end + fixture.branch_sources.len();
        let mut device = fixture.device("SWITCHED", &[1, 2]);
        device.set_internal_node_indices(&(3..=internal_end).collect::<Vec<_>>());
        device.set_branch_current_indices(&((internal_end + 1)..=dimension).collect::<Vec<_>>());
        let mut matrix = vec![vec![0.0; dimension]; dimension];
        let mut rhs = vec![0.0; dimension];
        let mut bias = vec![0.0; dimension];
        bias[0] = 1.0;
        device
            .try_stamp(&bias, |r, c, v| matrix[r][c] += v, |r, v| rhs[r] += v)
            .unwrap();
        for pivot in (2..dimension).rev() {
            assert!(matrix[pivot][pivot].abs() > 0.1, "{body}: {matrix:?}");
            let (rows, tail) = matrix.split_at_mut(pivot);
            for (row, coefficients) in rows.iter_mut().enumerate() {
                let factor = coefficients[pivot] / tail[0][pivot];
                for (value, pivot_value) in coefficients[..pivot].iter_mut().zip(&tail[0][..pivot])
                {
                    *value -= factor * pivot_value;
                }
                rhs[row] -= factor * rhs[pivot];
            }
        }
        for row in [0, 1] {
            assert!(
                (matrix[row][0] - conductance).abs() < 1e-12,
                "{body}: row {row}, expected {conductance}, matrix={matrix:?}"
            );
        }
        assert!(
            rhs.iter().all(|value| value.abs() < 1e-12),
            "{body}: {rhs:?}"
        );
    }
}

#[test]
fn switch_branches_preserve_complex_admittance_after_source_replacement() {
    use num_complex::Complex64;
    for potential in [false, true] {
        let source = if potential {
            "I(p)<+3*V(p)+ddt(2*V(p)); V(p)<+2*I(p)+ddt(4*I(p));"
        } else {
            "V(p)<+2*I(p)+ddt(4*I(p)); I(p)<+3*V(p)+ddt(2*V(p));"
        };
        let fixture = DeviceFixture::compile(&format!(
            "module switched(p); inout p; electrical p; analog begin {source} end endmodule"
        ));
        let mut device = fixture.device("AC_SWITCH", &[1]);
        device.set_internal_node_indices(&[2]);
        device.try_begin_analysis(1).unwrap();
        for omega in [0.0, 0.25, 2.0] {
            let mut matrix = [[Complex64::default(); 2]; 2];
            device
                .try_stamp_small_signal_complex(
                    &[1.0, 0.0],
                    omega / std::f64::consts::TAU,
                    |r, c, re, im| matrix[r][c] += Complex64::new(re, im),
                )
                .unwrap();
            let admittance = matrix[0][0] - matrix[0][1] * matrix[1][0] / matrix[1][1];
            let expected = if potential {
                Complex64::new(2.0, 4.0 * omega).inv()
            } else {
                Complex64::new(3.0, 2.0 * omega)
            };
            assert!(
                (admittance - expected).norm() < 1e-12,
                "{source}: omega={omega}, Y={admittance}, expected {expected}, matrix={matrix:?}"
            );
        }
    }
}

#[test]
fn hierarchical_switch_branches_preserve_localparams_and_independent_named_sources() {
    for (branch, declaration) in [("p,n", ""), ("sense", "branch(p,n) sense;")] {
        let source=format!("module child(p,n,q); inout p,n,q; electrical p,n,q; parameter real gain=1; localparam real resistance=2*gain; {declaration}
            analog begin I(q,n)<+I({branch}); I({branch})<+7*V(p,n); V({branch})<+resistance*I({branch}); end endmodule
            module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r;
            child #(.gain(1)) a(p,n,q); child #(.gain(2)) b(p,n,r); endmodule");
        assert_hierarchy_gains(&source, [0.75, 0.5, 0.25]);
    }
}

#[test]
fn overwritten_switch_contributions_still_validate_their_executed_rhs() {
    let fixture = DeviceFixture::compile(
        "module switched(p); inout p; electrical p; analog begin I(p)<+ln(V(p)); V(p)<+I(p); end endmodule",
    );
    let dimension = 1 + fixture.internal_nodes + fixture.branch_sources.len();
    let mut device = fixture.device("SWITCHED", &[1]);
    device.set_internal_node_indices(&(2..=(1 + fixture.internal_nodes)).collect::<Vec<_>>());
    device.set_branch_current_indices(
        &((2 + fixture.internal_nodes)..=dimension).collect::<Vec<_>>(),
    );
    let mut bias = vec![0.0; dimension];
    bias[0] = -1.0;
    assert!(
        device.try_stamp(&bias, |_, _, _| {}, |_, _| {}).is_err(),
        "changing source kind must not erase a domain error in an executed contribution"
    );
}

#[test]
fn declared_ground_preserves_reference_topology_and_conductance() {
    for (declarations, body) in [
        ("electrical p,g; ground g;", "I(p,g)<+0.5*V(p,g);"),
        ("ground g; electrical p,g;", "I(p,g)<+0.5*V(p,g);"),
        ("electrical p; ground electrical g;", "I(p,g)<+0.5*V(p,g);"),
        ("thermal p,g; ground g;", "Pwr(p,g)<+0.5*Temp(p,g);"),
        (
            "thermal p; ground electrical g; branch(g,p) heat;",
            "Pwr(heat)<+0.5*Temp(heat);",
        ),
    ] {
        assert_potential_conductance(
            &format!(
                "module parallel(p); inout p; {declarations} analog begin {body} end endmodule"
            ),
            0,
            0.5,
        );
    }
    assert_potential_conductance(
        "module resistor(p,n); inout p,n; electrical p,n; analog I(p,n)<+0.5*V(p,n); endmodule
         module parallel(p); inout p; electrical p,g; ground g; resistor r(p,g); endmodule",
        0,
        0.5,
    );
}

#[test]
fn potential_sources_preserve_named_and_instance_branch_identity() {
    for (declarations, body, branches, conductance) in [
        (
            "branch(p) a,b;",
            "V(a)<+2*I(a); V(b)<+3*I(b);",
            2,
            5.0 / 6.0,
        ),
        ("branch(p) a;", "V(a)<+2*I(a); V(p)<+3*I(p);", 2, 5.0 / 6.0),
        (
            "ground g; branch(p) a; branch(g,p) b;",
            "V(a)<+2*I(a); V(b)<+3*I(b);",
            2,
            5.0 / 6.0,
        ),
        (
            "branch(p) a,b;",
            "V(a)<+2*I(a)+I(b); V(b)<+I(a)+3*I(b);",
            2,
            0.6,
        ),
        ("branch(p) a;", "V(a)<+2*I(a); V(a)<+3*I(a);", 1, 0.2),
        ("ground g;", "V(p)<+2*I(p); V(g,p)<+3*I(g,p);", 1, 0.2),
        (
            "ground g; parameter integer enabled=1;",
            "V(p)<+2*I(p); if(enabled) V(g,p)<+3*I(g,p);",
            1,
            0.2,
        ),
        (
            "ground g; parameter integer enabled=0;",
            "V(p)<+2*I(p); if(enabled) V(g,p)<+3*I(g,p);",
            1,
            0.5,
        ),
        (
            "branch(p) a,b;",
            "V(a)<+2*I(a); V(b)<+2*I(b)+I(<p>)-I(a);",
            2,
            5.0 / 6.0,
        ),
        (
            "branch(p) a,b;",
            "V(a): V(p)==2*I(a); V(b): V(p)==3*I(b);",
            2,
            5.0 / 6.0,
        ),
        (
            "branch(p) a,b;",
            "V(a)<+I(a)+ddx(I(a)*I(b),I(b)); V(b)<+3*I(b)+ddx(I(a)*I(a),I(b));",
            2,
            5.0 / 6.0,
        ),
    ] {
        assert_potential_conductance(
            &format!(
                "module parallel(p); inout p; electrical p; {declarations} analog begin {body} end endmodule"
            ),
            branches,
            conductance,
        );
    }
    assert_potential_conductance(
        "module resistor(p); inout p; electrical p; parameter real r=2; analog V(p)<+r*I(p); endmodule
         module parallel(p); inout p; electrical p; resistor #(.r(2)) a(p); resistor #(.r(3)) b(p); endmodule",
        2,
        5.0 / 6.0,
    );
}

fn assert_potential_conductance(source: &str, branches: usize, conductance: f64) {
    let report = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(source, Some("parallel"))
        .unwrap();
    let fixture = DeviceFixture {
        model: report.model,
        canonical_ir: report.canonical_ir,
    };
    assert_eq!(fixture.branch_sources.len(), branches, "{source}");
    assert_eq!(fixture.internal_nodes, 0, "{source}");
    let dimension = 1 + branches;
    let mut device = fixture.device("PARALLEL", &[1]);
    device.set_branch_current_indices(&(2..=dimension).collect::<Vec<_>>());
    let mut matrix = vec![vec![0.0; dimension]; dimension];
    let mut rhs = vec![0.0; dimension];
    let mut bias = vec![0.0; dimension];
    bias[0] = 1.0;
    device
        .try_stamp(&bias, |r, c, v| matrix[r][c] += v, |r, v| rhs[r] += v)
        .unwrap();
    for pivot in (1..dimension).rev() {
        let (rows, tail) = matrix.split_at_mut(pivot);
        let pivot_row = &tail[0];
        assert!(pivot_row[pivot].abs() > 0.1, "{source}: {pivot_row:?}");
        for (row, coefficients) in rows.iter_mut().enumerate() {
            let factor = coefficients[pivot] / pivot_row[pivot];
            for (value, pivot_value) in coefficients[..pivot].iter_mut().zip(&pivot_row[..pivot]) {
                *value -= factor * pivot_value;
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }
    assert!(
        (matrix[0][0] - conductance).abs() < 1e-12,
        "{source}: {matrix:?}"
    );
    assert!(rhs.iter().all(|v| v.abs() < 1e-12), "{source}: {rhs:?}");
}

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
fn hierarchical_flow_probes_retain_each_instances_branch() {
    for (declaration, branch, probe, expected_q, expected_r) in [
        ("", "p,n", "I(p,n)", 3.0, 6.0),
        ("", "p,n", "I(<p>)", 3.0, 6.0),
        ("", "p,n", "I(n,p)", -3.0, -6.0),
        ("branch(p,n) sense;", "sense", "I(sense)", 3.0, 6.0),
        ("", "p,n", "ddx(I(p,n)*I(p,n),I(n,p))", -6.0, -12.0),
    ] {
        let source = format!(
            "module child(p,n,q); inout p,n,q; electrical p,n,q; parameter real gain=1; {declaration}
             analog begin if(gain>0) I({branch})<+gain*V(p,n); I(q,n)<+3*({probe}); end endmodule
             module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r;
             child #(.gain(1)) a(p,n,q); child #(.gain(2)) b(p,n,r); endmodule"
        );
        assert_hierarchy_gains(&source, [3.0, expected_q, expected_r]);
    }
}

fn assert_hierarchy_gains(source: &str, expected_gains: [f64; 3]) {
    let report = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(source, Some("top"))
        .unwrap();
    let fixture = DeviceFixture {
        model: report.model,
        canonical_ir: report.canonical_ir,
    };
    let dimension = 3 + fixture.model.internal_nodes;
    let mut device = fixture.device("HIERARCHY", &[1, 0, 2, 3]);
    device.set_internal_node_indices(&(4..=dimension).collect::<Vec<_>>());
    let mut matrix = vec![vec![0.0; dimension]; dimension];
    let mut rhs = vec![0.0; dimension];
    let mut bias = vec![0.0; dimension];
    bias[0] = 1.0;
    device
        .try_stamp(&bias, |r, c, v| matrix[r][c] += v, |r, v| rhs[r] += v)
        .unwrap();
    for pivot in (3..dimension).rev() {
        let (rows, tail) = matrix.split_at_mut(pivot);
        let pivot_row = &tail[0];
        assert!(pivot_row[pivot].abs() > 0.1);
        for (row, coefficients) in rows.iter_mut().enumerate() {
            let factor = coefficients[pivot] / pivot_row[pivot];
            for (value, pivot_value) in coefficients[..pivot].iter_mut().zip(&pivot_row[..pivot]) {
                *value -= factor * pivot_value;
            }
            rhs[row] -= factor * rhs[pivot];
        }
    }
    for (row, expected) in expected_gains.into_iter().enumerate() {
        assert!(
            (matrix[row][0] - expected).abs() < 1e-12,
            "{source}: row {row}, matrix={matrix:?}"
        );
    }
    assert!(
        rhs.iter().all(|value| value.abs() < 1e-12),
        "{source}: rhs={rhs:?}"
    );
}

#[test]
fn hierarchical_port_flows_include_children_and_preserve_boundary_identity() {
    for (port, q, r) in [("p", 3.0, 6.0), ("n", -0.75, -1.5)] {
        assert_hierarchy_gains(&format!(
            "module leaf(p,n); inout p,n; electrical p,n; parameter real gain=1; analog I(p,n)<+gain*V(p,n); endmodule
             module child(p,n,q); inout p,n,q; electrical p,n,q; parameter real gain=1;
             leaf #(.gain(gain)) inner(p,n); analog I(q,n)<+3*I(<{port}>); endmodule
             module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r;
             child #(.gain(1)) a(p,n,q); child #(.gain(2)) b(p,n,r); endmodule"
        ), [3.0, q, r]);
    }
    assert_hierarchy_gains(
        "module child(p,n,r); inout p,n,r; electrical p,n,r; analog begin I(p,n)<+2*V(p,n); I(r,n)<+3*I(p,n); end endmodule
         module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r; child a(p,n,r);
         analog begin I(p,n)<+4*V(p,n); I(q,n)<+I(p,n); end endmodule",
        [6.0, 4.0, 6.0],
    );
    // A child's distinct ports can connect to one parent net. The child's
    // port probe still measures its own boundary, while the parent's net
    // current sums the two equal and opposite branch endpoints to zero.
    assert_hierarchy_gains(
        "module child(p,n,q); inout p,n,q; electrical p,n,q; analog begin I(p,n)<+V(p); I(q)<+3*I(<p>); end endmodule
         module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r; child a(p,p,q); analog I(r)<+I(<p>); endmodule",
        [0.0, 3.0, 0.0],
    );
    // Unobserved sources also cancel without discarding their evaluation.
    assert_hierarchy_gains(
        "module child(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p); endmodule
         module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r; child a(p,p);
         analog begin I(q)<+V(p); I(r)<+I(<p>); end endmodule",
        [0.0, 1.0, 0.0],
    );
    for probe in ["3*I(<p>)", "3*V(driver)"] {
        assert_hierarchy_gains(
            &format!(
                "module child(p,n,q,driver); inout p,n,q,driver; electrical p,n,q,driver;
                 analog begin I(p,n)<+V(driver); I(q)<+{probe}; end endmodule
                 module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r;
                 child a(0,0,q,p); analog I(r)<+I(<p>); endmodule"
            ),
            [0.0, 3.0, 0.0],
        );
    }
    assert_hierarchy_gains(
        "module child(p,n,q); inout p,n,q; electrical p,n,q; analog I(q)<+V(p,n); endmodule
         module top(p,n,q,r); inout p,n,q,r; electrical p,n,q,r; child a(p,p,q); endmodule",
        [0.0, 0.0, 0.0],
    );
}

#[test]
fn tied_hierarchy_ports_preserve_cancelled_source_validation() {
    let report = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(
            "module child(p,n,driver); inout p,n,driver; electrical p,n,driver;
         analog I(p,n)<+1/(V(driver)-V(driver)); endmodule
         module top(p); inout p; electrical p; child a(0,0,p); endmodule",
            Some("top"),
        )
        .unwrap();
    let fixture = DeviceFixture {
        model: report.model,
        canonical_ir: report.canonical_ir,
    };
    assert_eq!(fixture.model.internal_state_nodes.len(), 1);
    let mut device = fixture.device("CANCELLED", &[1]);
    device.set_internal_node_indices(&[2]);
    assert!(
        device
            .try_stamp(&[1.0, 0.0], |_, _, _| {}, |_, _| {})
            .is_err()
    );
}

#[test]
fn hierarchy_keeps_source_free_flow_and_potential_probe_rejection() {
    let error = rspice_veriloga::VerilogACompiler::default().compile_runtime(
        "module child(p,n,q); inout p,n,q; electrical p,n,q; analog I(q,n)<+I(p,n)+V(p,n); endmodule
         module top(p,n,q); inout p,n,q; electrical p,n,q; child a(p,n,q); endmodule", Some("top")
    ).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("cannot probe both flow and potential"),
        "{error}"
    );
}

#[test]
fn authored_sources_and_flow_probes_require_distinct_nets() {
    for body in [
        "I(p,p)<+1;",
        "V(p,p)<+1;",
        "I(q)<+I(p,p);",
        "I(q)<+I(g1);",
        "V(0)<+1;",
    ] {
        let error = rspice_veriloga::VerilogACompiler::default().compile_runtime(&format!(
            "module invalid(p,q); inout p,q; electrical p,q; ground g1,g2; analog begin {body} end endmodule"
        ), None).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("branch endpoints must name distinct nets"),
            "{body}: {error}"
        );
    }
}

#[test]
fn coincident_potential_reads_retain_compact_model_compatibility() {
    let fixture = DeviceFixture::compile(
        "module compatible(p,q); inout p,q; electrical p,q; ground g1,g2;
         analog I(q)<+V(p,p)+V(g1,g2)+V(g1); endmodule",
    );
    let mut device = fixture.device("COMPATIBLE", &[1, 2]);
    assert_eq!(device.try_evaluate().unwrap(), vec![0.0]);
    let mut matrix = [[0.0; 2]; 2];
    let mut rhs = [0.0; 2];
    device
        .try_stamp(
            &[2.0, 0.0],
            |row, col, value| matrix[row][col] += value,
            |row, value| rhs[row] += value,
        )
        .unwrap();
    assert_eq!(matrix, [[0.0; 2]; 2]);
    assert_eq!(rhs, [0.0; 2]);
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
