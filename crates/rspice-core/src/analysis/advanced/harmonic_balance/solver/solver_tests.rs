use super::*;
#[cfg(feature = "veriloga")]
use crate::device::veriloga::VerilogADevice;
#[cfg(feature = "veriloga")]
use rspice_veriloga::codegen::{
    BytecodeProgram, CompiledModel, CompiledParameter, Instruction,
    JacobianEntry as VaJacobianEntry, StampIndex, StampLocation, StampProgram,
};

#[cfg(feature = "veriloga")]
fn create_veriloga_resistor_model() -> CompiledModel {
    let value_program = BytecodeProgram {
        instructions: vec![
            Instruction::PushParam(0),      // g
            Instruction::PushVoltage(0, 1), // V(p, n)
            Instruction::Mul,
        ],
    };
    let g_pos = BytecodeProgram {
        instructions: vec![Instruction::PushParam(0)],
    };
    let g_neg = BytecodeProgram {
        instructions: vec![Instruction::PushParam(0), Instruction::Neg],
    };

    CompiledModel {
        name: "hb_va_resistor".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![CompiledParameter {
            name: "g".into(),
            default: 1e-3,
            min: Some(0.0),
            max: None,
        }],
        num_variables: 0,
        assignment_programs: vec![],
        stamp_programs: vec![StampProgram {
            stamp_locations: vec![
                StampLocation {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Ground,
                    sign: -1.0,
                },
                StampLocation {
                    row: StampIndex::Terminal(1),
                    col: StampIndex::Ground,
                    sign: 1.0,
                },
            ],
            value_program,
            jacobian_programs: vec![
                VaJacobianEntry {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Terminal(0),
                    program: g_pos.clone(),
                },
                VaJacobianEntry {
                    row: StampIndex::Terminal(0),
                    col: StampIndex::Terminal(1),
                    program: g_neg.clone(),
                },
                VaJacobianEntry {
                    row: StampIndex::Terminal(1),
                    col: StampIndex::Terminal(0),
                    program: g_neg,
                },
                VaJacobianEntry {
                    row: StampIndex::Terminal(1),
                    col: StampIndex::Terminal(1),
                    program: g_pos,
                },
            ],
        }],
        lookup_tables: vec![],
        internal_nodes: 0,
        branch_currents: 0,
        laplace_filters: vec![],
    }
}

#[cfg(feature = "veriloga")]
fn create_veriloga_resistor_device(name: &str, nodes: &[usize], g: f64) -> VerilogADevice {
    let mut device = VerilogADevice::new(name, create_veriloga_resistor_model(), nodes);
    let ok = device.set_parameter("g", g);
    assert!(ok, "expected parameter g to exist");
    device
}

#[test]
fn test_hb_solver_creation() {
    let config = HbConfig::new(1e9).with_harmonics(5);
    let solver = HbSolver::new(config, 3);

    assert_eq!(solver.num_nodes, 3);
    assert_eq!(solver.num_harmonics, 5);
}

#[test]
fn test_solver_state_creation() {
    let state = HbSolverState::new(3, 5);

    assert_eq!(state.x.len(), 3);
    assert_eq!(state.x[0].len(), 6); // 5 harmonics + DC
    assert!(!state.converged);
}

#[test]
fn test_solver_state_norms() {
    let mut state = HbSolverState::new(2, 2);

    // Set some values
    state.x[0][0] = Complex64::new(1.0, 0.0);
    state.x[0][1] = Complex64::new(0.0, 1.0);
    state.residual[1][0] = Complex64::new(3.0, 4.0); // |3+4j| = 5

    state.compute_residual_norm();
    assert!((state.residual_norm - 5.0).abs() < 1e-10);

    let sol_norm = state.solution_norm();
    assert!((sol_norm - 2.0_f64.sqrt()).abs() < 1e-10); // sqrt(1 + 1)
}

#[test]
fn test_initialize_diode_voltages_ignores_non_finite_ground_conductance() {
    let config = HbConfig::new(1e9).with_harmonics(1);
    let mut solver = HbSolver::new(config, 2);
    solver.g_matrix.push((0, 0, f64::NAN));
    solver.g_matrix.push((1, 1, 1e-3));

    let mut state = HbSolverState::new(2, 1);
    solver.initialize_diode_voltages(&mut state);

    assert!(state.x[0][0].re.is_finite());
    assert!(state.x[1][0].re.is_finite());
    assert!((state.x[1][0].re - 0.0).abs() < 1e-12);
}

#[test]
fn test_add_stamps() {
    let config = HbConfig::new(1e9);
    let mut solver = HbSolver::new(config, 2);

    solver.add_conductance(0, 1, 0.001);
    solver.add_capacitance(0, 0, 1e-12);

    assert_eq!(solver.g_matrix.len(), 1);
    assert_eq!(solver.c_matrix.len(), 1);
}

#[test]
fn test_set_sources() {
    let config = HbConfig::new(1e9).with_harmonics(3);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1.0);
    solver.set_ac_source(0, 0.5, 0.0);

    assert!((solver.source_spectra[0][0].re - 1.0).abs() < 1e-10);
    assert!((solver.source_spectra[0][1].re - 0.5).abs() < 1e-10);
}

#[test]
fn test_add_sources_accumulate() {
    let config = HbConfig::new(1e9).with_harmonics(3);
    let mut solver = HbSolver::new(config, 1);

    solver.add_dc_source(0, 1.0);
    solver.add_dc_source(0, 2.0);
    solver.add_ac_source(0, 3.0, 0.0);
    solver.add_ac_source(0, 1.0, 0.0);

    assert!((solver.source_spectra[0][0].re - 3.0).abs() < 1e-12);
    assert!((solver.source_spectra[0][1].re - 4.0).abs() < 1e-12);
}

#[test]
fn test_add_harmonic_source_accumulates_non_fundamental_components() {
    let config = HbConfig::new(1e9).with_harmonics(5);
    let mut solver = HbSolver::new(config, 1);

    solver.add_harmonic_source(0, 3, 1.0, 0.0);
    solver.add_harmonic_source(0, 3, 0.5, 0.0);

    assert!((solver.source_spectra[0][3].re - 1.5).abs() < 1e-12);
    assert!(solver.source_spectra[0][1].norm() < 1e-15);
}

#[test]
fn test_voltage_source_branch_harmonics_supports_arbitrary_indices() {
    let config = HbConfig::new(1e6).with_harmonics(6);
    let mut solver = HbSolver::new(config, 1);

    solver.add_voltage_source_branch_harmonics(
        1,
        0,
        0.5,
        &[(2, 1.0, 0.0), (5, 0.25, std::f64::consts::FRAC_PI_2)],
    );

    let branch = solver
        .voltage_source_branches
        .first()
        .expect("expected one voltage source branch");
    assert!((HbSolver::voltage_source_value_at_harmonic(branch, 0).re - 0.5).abs() < 1e-12);
    assert!((HbSolver::voltage_source_value_at_harmonic(branch, 2).norm() - 1.0).abs() < 1e-12);
    assert!((HbSolver::voltage_source_value_at_harmonic(branch, 5).norm() - 0.25).abs() < 1e-12);
    assert!(HbSolver::voltage_source_value_at_harmonic(branch, 1).norm() < 1e-15);
}

#[test]
fn test_solve_linear_with_voltage_source_branch() {
    let config = HbConfig::new(1e6).with_harmonics(3).with_tolerance(1e-12);
    let mut solver = HbSolver::new(config, 1);

    // Add a tiny capacitor so this remains a valid HB setup.
    solver.add_capacitance(0, 0, 1e-12);
    solver.add_voltage_source_branch(1, 0, 3.3);

    let mut state = HbSolverState::new(1, 3);
    solver
        .solve_linear(&mut state)
        .expect("linear solve should succeed");

    assert!(
        (state.x[0][0].re - 3.3).abs() < 1e-9,
        "ideal source should force the node DC voltage"
    );
}

#[test]
fn test_linear_residual_dc_only() {
    let config = HbConfig::new(1e9).with_harmonics(2);
    let mut solver = HbSolver::new(config, 1);

    // Simple resistor to ground: I = G*V, with 1V DC source
    solver.add_conductance(0, 0, 0.001); // 1k ohm
    solver.set_dc_source(0, 0.001); // 1mA = 1V / 1k

    let mut state = HbSolverState::new(1, 2);
    state.x[0][0] = Complex64::new(1.0, 0.0); // V = 1V

    solver.compute_linear_residual(&mut state);

    // Residual should be small: G*V - I = 0.001*1 - 0.001 = 0
    assert!(
        state.residual[0][0].norm() < 1e-10,
        "Residual: {}",
        state.residual[0][0]
    );
}

#[test]
fn test_solve_linear_simple() {
    let config = HbConfig::new(1e9).with_harmonics(1);
    let solver = HbSolver::new(config, 1);

    // Empty circuit (no stamps) with DC source
    // Should give zero solution
    let mut state = HbSolverState::new(1, 1);

    // This is degenerate, but should not panic
    let _ = solver.solve_linear(&mut state);
}

#[test]
fn test_complex_linear_solve() {
    let config = HbConfig::new(1e9);
    let solver = HbSolver::new(config, 2);

    // Simple 2x2 system
    let a = vec![
        vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.0)],
        vec![Complex64::new(1.0, 0.0), Complex64::new(3.0, 0.0)],
    ];
    let b = vec![Complex64::new(5.0, 0.0), Complex64::new(7.0, 0.0)];

    let x = solver.solve_complex_linear_system(&a, &b).unwrap();

    // Verify solution
    let r0 = a[0][0] * x[0] + a[0][1] * x[1] - b[0];
    let r1 = a[1][0] * x[0] + a[1][1] * x[1] - b[1];

    assert!(r0.norm() < 0.01, "Residual 0: {}", r0);
    assert!(r1.norm() < 0.01, "Residual 1: {}", r1);
}

#[test]
fn test_build_result() {
    let config = HbConfig::new(1e9).with_harmonics(3);
    let solver = HbSolver::new(config, 2);

    let mut state = HbSolverState::new(2, 3);
    state.converged = true;
    state.iteration = 5;
    state.residual_norm = 1e-10;

    let result = solver.build_result(&state);

    assert!(result.converged);
    assert_eq!(result.iterations, 5);
    assert_eq!(result.num_nodes(), 2);
    assert_eq!(result.num_harmonics, 3);
}

#[test]
fn test_hb_error_display() {
    let err = HbError::ConvergenceFailed {
        iterations: 50,
        residual: 1e-3,
    };
    let msg = format!("{}", err);
    assert!(msg.contains("50 iterations"));
}

// ==========================================================================
// Newton Solver Tests - Verification
// ==========================================================================

#[test]
fn test_nonlinear_device_instance_diode_creation() {
    let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);

    assert_eq!(diode.device_type, NonlinearDeviceType::Diode);
    assert_eq!(diode.terminals, vec![0, 1]);
    assert!((diode.params.is - 1e-14).abs() < 1e-20);
    assert!((diode.params.n - 1.0).abs() < 1e-10);
}

#[test]
fn test_nonlinear_device_instance_bjt_creation() {
    let bjt = NonlinearDeviceInstance::npn_bjt(0, 1, 2, 1e-15, 100.0);

    assert_eq!(bjt.device_type, NonlinearDeviceType::NpnBjt);
    assert_eq!(bjt.terminals, vec![0, 1, 2]);
    assert!((bjt.params.bf - 100.0).abs() < 1e-10);
}

#[test]
fn test_nonlinear_device_instance_jfet_and_switch_creation() {
    let njfet = NonlinearDeviceInstance::njfet(0, 1, 2, -2.0, 1e-3, 0.02);
    assert_eq!(njfet.device_type, NonlinearDeviceType::Njfet);
    assert_eq!(njfet.terminals, vec![0, 1, 2]);
    assert!((njfet.params.vth + 2.0).abs() < 1e-12);
    assert!((njfet.params.kp - 1e-3).abs() < 1e-15);

    let vsw = NonlinearDeviceInstance::voltage_switch(0, 1, 2, 3, 0.5, 0.0, 1.0, 1e9, 0.05);
    assert_eq!(vsw.device_type, NonlinearDeviceType::VoltageSwitch);
    assert_eq!(vsw.terminals, vec![0, 1, 2, 3]);
    assert!((vsw.params.vth - 0.5).abs() < 1e-12);
    assert!((vsw.params.ron - 1.0).abs() < 1e-12);
    assert!((vsw.params.roff - 1e9).abs() < 1.0);

    let isw = NonlinearDeviceInstance::current_switch(0, 1, 2, 3, 1e-3, 0.0, 2.0, 1e9, 1e-4, 1e6);
    assert_eq!(isw.device_type, NonlinearDeviceType::CurrentSwitch);
    assert_eq!(isw.terminals, vec![0, 1, 2, 3]);
    assert!((isw.params.vth - 1e-3).abs() < 1e-15);
    assert!((isw.params.control_gain - 1e6).abs() < 1e-6);
}

#[test]
fn test_jfet_evaluate_and_jacobian_kcl() {
    let jfet = NonlinearDeviceInstance::njfet(0, 1, 2, -2.0, 1e-3, 0.01);
    // vd=2.0V, vg=-0.5V, vs=0V => conductive region for NJF depletion device
    let voltages = vec![2.0, -0.5, 0.0];
    let currents = jfet.evaluate(&voltages);

    let sum: Value = currents.iter().map(|(_, i)| i).sum();
    assert!(sum.abs() < 1e-12, "JFET current KCL must close: {}", sum);

    let i_d = currents
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    let i_s = currents
        .iter()
        .find(|(n, _)| *n == 2)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(i_d < 0.0, "drain current should leave drain node");
    assert!(i_s > 0.0, "source current should enter source node");

    let jac = jfet.jacobian(&voltages);
    assert!(
        jac.iter()
            .any(|((r, c), v)| *r == 0 && *c == 1 && v.abs() > 0.0),
        "JFET Jacobian should include drain-gate transconductance"
    );
    assert!(
        jac.iter()
            .any(|((r, c), v)| *r == 2 && *c == 1 && v.abs() > 0.0),
        "JFET Jacobian should include source-gate coupling"
    );
}

#[test]
fn test_voltage_switch_evaluate_and_control_jacobian() {
    let vsw = NonlinearDeviceInstance::voltage_switch(0, 1, 2, 3, 1.0, 0.0, 1.0, 1e9, 0.05);
    // vp=1.0, vn=0.0, vcp=1.0, vcn=0.0 => threshold region (max control sensitivity)
    let voltages = vec![1.0, 0.0, 1.0, 0.0];
    let currents = vsw.evaluate(&voltages);
    let i_pos = currents
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    let i_neg = currents
        .iter()
        .find(|(n, _)| *n == 1)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(
        i_pos < 0.0 && i_neg > 0.0,
        "switch branch current direction"
    );
    assert!(
        (i_pos + i_neg).abs() < 1e-12,
        "switch branch current must satisfy KCL"
    );

    let jac = vsw.jacobian(&voltages);
    assert!(
        jac.iter()
            .any(|((r, c), v)| *r == 0 && *c == 2 && v.abs() > 0.0),
        "switch Jacobian should include control-positive coupling"
    );
    assert!(
        jac.iter()
            .any(|((r, c), v)| *r == 0 && *c == 3 && v.abs() > 0.0),
        "switch Jacobian should include control-negative coupling"
    );
}

#[test]
fn test_current_switch_evaluate_and_control_jacobian() {
    let isw = NonlinearDeviceInstance::current_switch(0, 1, 2, 3, 1e-3, 0.0, 1.0, 1e9, 1e-4, 1e6);
    // vp=1.0, vn=0.0, vcp=1e-9, vcn=0 => ictrl = 1mA at threshold
    let voltages = vec![1.0, 0.0, 1e-9, 0.0];
    let currents = isw.evaluate(&voltages);
    let i_pos = currents
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    let i_neg = currents
        .iter()
        .find(|(n, _)| *n == 1)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(i_pos < 0.0 && i_neg > 0.0);
    assert!((i_pos + i_neg).abs() < 1e-12);

    let jac = isw.jacobian(&voltages);
    assert!(
        jac.iter()
            .any(|((r, c), v)| *r == 0 && *c == 2 && v.abs() > 0.0),
        "current switch Jacobian should include control-positive coupling"
    );
    assert!(
        jac.iter()
            .any(|((r, c), v)| *r == 0 && *c == 3 && v.abs() > 0.0),
        "current switch Jacobian should include control-negative coupling"
    );
}

#[test]
fn test_diode_evaluate_forward_bias() {
    let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
    let vt: f64 = 0.02585;

    // Forward bias: 0.6V across diode (node 0 = 0.6V, node 1 = 0V)
    let voltages = vec![0.6, 0.0];
    let currents = diode.evaluate(&voltages);

    // Current should be positive and significant for forward bias
    let i_anode = currents
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    let i_cathode = currents
        .iter()
        .find(|(n, _)| *n == 1)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);

    // Current flows out of anode (negative) into cathode (positive)
    assert!(
        i_anode < 0.0,
        "Current should flow out of anode: {}",
        i_anode
    );
    assert!(
        i_cathode > 0.0,
        "Current should flow into cathode: {}",
        i_cathode
    );

    // KCL: currents should sum to zero
    let sum: Value = currents.iter().map(|(_, i)| i).sum();
    assert!(sum.abs() < 1e-20, "KCL violation: {}", sum);

    // Verify current magnitude is reasonable for 0.6V forward bias
    let expected_i = 1e-14_f64 * ((0.6_f64 / vt).exp() - 1.0);
    assert!(
        (i_cathode - expected_i).abs() / expected_i < 0.01,
        "Current should match Shockley equation: got {} expected {}",
        i_cathode,
        expected_i
    );
}

#[test]
fn test_diode_evaluate_reverse_bias() {
    let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);

    // Reverse bias: -5V across diode
    let voltages = vec![-5.0, 0.0];
    let currents = diode.evaluate(&voltages);

    // Current should be very small (approximately -Is)
    let i_cathode = currents
        .iter()
        .find(|(n, _)| *n == 1)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(
        i_cathode.abs() < 1e-13,
        "Reverse current should be ~Is: {}",
        i_cathode
    );
}

#[test]
fn test_diode_jacobian_positive_conductance() {
    let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);

    // At 0.6V forward bias
    let voltages = vec![0.6, 0.0];
    let jac = diode.jacobian(&voltages);

    // Self-conductance at anode should be positive
    let g_aa = jac
        .iter()
        .filter(|((i, j), _)| *i == 0 && *j == 0)
        .map(|(_, g)| *g)
        .sum::<Value>();
    assert!(g_aa > 0.0, "Self-conductance should be positive: {}", g_aa);
}

#[test]
fn test_newton_solver_diode_dc() {
    // Test Newton solver on simple diode DC circuit
    // Diode in series with resistor, DC current source

    let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(50);
    let mut solver = HbSolver::new(config, 2);

    // Node 0: anode of diode
    // Node 1: cathode of diode (to ground through resistor)

    // Current source: 1mA into node 0
    solver.set_dc_source(0, 1e-3);

    // Resistor from node 1 to ground: 100 ohms
    solver.add_conductance(1, 1, 0.01);

    // Diode from node 0 to node 1
    solver.add_diode(0, 1, 1e-14, 1.0);

    // Small conductance for numerical stability
    solver.add_conductance(0, 0, 1e-9);

    // Add capacitor to make it valid for HB
    solver.add_capacitance(0, 0, 1e-12);

    let mut state = HbSolverState::new(2, 1);
    // Initialize with small forward bias guess
    state.x[0][0] = Complex64::new(0.6, 0.0);
    state.x[1][0] = Complex64::new(0.1, 0.0);

    let result = solver.solve_newton(&mut state);
    assert!(result.is_ok(), "Newton should converge: {:?}", result);
    assert!(state.converged, "Should converge");

    // DC voltages should be physical
    let v0_dc = state.x[0][0].re;
    let v1_dc = state.x[1][0].re;
    let vd = v0_dc - v1_dc;

    // Diode voltage should be around 0.5-0.7V for 1mA
    assert!(
        vd > 0.4 && vd < 0.8,
        "Diode voltage should be ~0.6V: {}",
        vd
    );
}

#[test]
fn test_newton_solver_linear_fallback() {
    // Test that solve_newton works for linear circuits too (no devices)
    let config = HbConfig::new(1e6).with_harmonics(3);
    let mut solver = HbSolver::new(config, 2);

    // Simple RC circuit
    solver.add_conductance(0, 0, 0.001);
    solver.add_capacitance(1, 1, 1e-9);
    solver.add_conductance(0, 1, 0.0001);
    solver.set_dc_source(0, 1e-3);

    let mut state = HbSolverState::new(2, 3);

    let result = solver.solve_newton(&mut state);
    assert!(result.is_ok(), "Should solve linear circuit: {:?}", result);
}

#[test]
fn test_newton_solver_device_registration() {
    let config = HbConfig::new(1e9);
    let mut solver = HbSolver::new(config, 3);

    assert!(!solver.has_nonlinear_devices());

    solver.add_diode(0, 1, 1e-14, 1.0);
    assert!(solver.has_nonlinear_devices());

    solver.add_npn_bjt(1, 2, 0, 1e-15, 100.0);
    assert_eq!(solver.nonlinear_devices.len(), 2);
}

#[cfg(feature = "veriloga")]
#[test]
fn test_newton_solver_veriloga_device_registration() {
    let config = HbConfig::new(1e6).with_harmonics(1);
    let mut solver = HbSolver::new(config, 2);
    assert!(!solver.has_nonlinear_devices());

    let device = create_veriloga_resistor_device("RVA1", &[1, 2], 1e-3);
    solver.add_veriloga_device(device);
    assert!(solver.has_nonlinear_devices());
}

#[cfg(feature = "veriloga")]
#[test]
fn test_veriloga_residual_and_jacobian_consistency() {
    let config = HbConfig::new(1e6).with_harmonics(1);
    let mut solver = HbSolver::new(config, 2);
    solver.add_veriloga_device(create_veriloga_resistor_device("RVA1", &[1, 2], 2e-3));

    let mut state = HbSolverState::new(2, 1);
    state.x[0][0] = Complex64::new(1.25, 0.0);
    state.x[1][0] = Complex64::new(0.25, 0.0);

    solver.compute_full_residual_with_gmin(&mut state, 0.0);
    assert!(
        (state.residual[0][0].re + 2e-3).abs() < 1e-9,
        "expected node-0 residual to match -g*(v0-v1)"
    );
    assert!(
        (state.residual[1][0].re - 2e-3).abs() < 1e-9,
        "expected node-1 residual to match +g*(v0-v1)"
    );

    let jac = solver.build_full_jacobian_with_gmin(&state, 0.0);
    let h = solver.num_harmonics + 1;
    let d00 = jac[0][0].re;
    let d01 = jac[0][h].re;
    let d10 = jac[h][0].re;
    let d11 = jac[h][h].re;

    assert!((d00 + 2e-3).abs() < 1e-9, "dR0/dV0 should be -g");
    assert!((d01 - 2e-3).abs() < 1e-9, "dR0/dV1 should be +g");
    assert!((d10 - 2e-3).abs() < 1e-9, "dR1/dV0 should be +g");
    assert!((d11 + 2e-3).abs() < 1e-9, "dR1/dV1 should be -g");

    let mut perturbed = HbSolverState::new(2, 1);
    perturbed.x = state.x.clone();
    let eps = 1e-6;
    perturbed.x[0][0].re += eps;
    solver.compute_full_residual_with_gmin(&mut perturbed, 0.0);
    let fd = (perturbed.residual[0][0].re - state.residual[0][0].re) / eps;
    assert!(
        (fd - d00).abs() < 1e-6,
        "finite-difference Jacobian should match analytical"
    );
}

#[cfg(feature = "veriloga")]
#[test]
fn test_newton_solver_converges_with_veriloga_resistor() {
    let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(30);
    let mut solver = HbSolver::new(config, 1);
    solver.add_capacitance(0, 0, 1e-12);
    solver.set_dc_source(0, 1e-3);
    solver.add_veriloga_device(create_veriloga_resistor_device("RVA1", &[1, 0], 1e-3));

    let mut state = HbSolverState::new(1, 1);
    let result = solver.solve_newton(&mut state);
    assert!(result.is_ok(), "Verilog-A Newton solve should converge");
    assert!(
        (state.x[0][0].re - 1.0).abs() < 1e-6,
        "1mA into 1k equivalent should settle near 1V"
    );
}

#[test]
fn test_build_full_jacobian_linear_only() {
    let config = HbConfig::new(1e9).with_harmonics(2);
    let mut solver = HbSolver::new(config, 2);

    // Add some linear elements using proper MNA stamping
    // 100 ohm resistor from node 0 to ground (G = 0.01)
    solver.add_conductance(0, 0, 0.01);
    // 1k resistor between nodes 0 and 1 (full MNA stamp)
    solver.add_resistor(0, 1, 1000.0); // G = 0.001
    // 1pF capacitor at node 1
    solver.add_capacitance(1, 1, 1e-12);

    let state = HbSolverState::new(2, 2);
    let jac = solver.build_full_jacobian(&state);

    // Matrix is 2 nodes * 3 harmonics = 6x6
    assert_eq!(jac.len(), 6);
    assert_eq!(jac[0].len(), 6);

    // DC block (k=0): should have conductance stamps
    // With KCL convention (res = I_source - Y*V), Jacobian J = -Y
    // Node 0, harmonic 0: J = -(0.01 + 0.001) = -0.011
    let y00_dc = jac[0][0];
    assert!(
        (y00_dc.re - (-0.011)).abs() < 1e-10,
        "J(0,0) at DC should be -0.011: {}",
        y00_dc
    );

    // Node 1, harmonic 0: J = -0.001 from resistor
    let h = 3; // 2 harmonics + DC
    let y11_dc = jac[1 * h][1 * h]; // node 1, harmonic 0
    assert!(
        (y11_dc.re - (-0.001)).abs() < 1e-10,
        "J(1,1) at DC should be -0.001: {}",
        y11_dc
    );

    // Off-diagonal: J = -(-0.001) = +0.001 (negative of off-diagonal admittance)
    let y01_dc = jac[0][1 * h];
    assert!(
        (y01_dc.re - 0.001).abs() < 1e-10,
        "J(0,1) at DC should be +0.001: {}",
        y01_dc
    );
}

#[test]
fn test_build_full_jacobian_with_diode() {
    let config = HbConfig::new(1e9).with_harmonics(1);
    let mut solver = HbSolver::new(config, 2);

    // Resistor and diode
    solver.add_conductance(0, 0, 0.001);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_capacitance(1, 1, 1e-12);

    let mut state = HbSolverState::new(2, 1);
    state.x[0][0] = Complex64::new(0.6, 0.0);

    let jac = solver.build_full_jacobian(&state);

    // Jacobian should have nonlinear conductance added (with KCL sign: J = -Y)
    // At 0.6V, diode conductance is significant
    // Total J(0,0) = -(linear G + diode gd) < -0.001
    let y00 = jac[0][0];
    assert!(
        y00.re < -0.001,
        "Should have negative Jacobian with nonlinear contribution: {}",
        y00
    );
}

#[test]
fn test_newton_nmos_saturation() {
    let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(50);
    let mut solver = HbSolver::new(config, 3);

    // NMOS: Drain=0, Gate=1, Source=2
    // Setup: Vgs = 2V (above Vth=0.7), Vds = 3V (saturation)

    // Gate voltage source (node 1 at 2V)
    solver.add_conductance(1, 1, 1.0); // Very low resistance to enforce voltage
    solver.set_dc_source(1, 2.0); // 2V at gate

    // Drain load resistor
    solver.add_conductance(0, 0, 0.001); // 1k load
    solver.set_dc_source(0, 5e-3); // Current to set Vdd

    // Source to ground
    solver.add_conductance(2, 2, 0.1); // Low resistance

    // Capacitor for HB validity
    solver.add_capacitance(0, 0, 1e-12);

    // Add NMOS
    solver.add_nonlinear_device(NonlinearDeviceInstance::nmos(0, 1, 2, 2, 0.7, 2e-4));

    let mut state = HbSolverState::new(3, 1);
    state.x[0][0] = Complex64::new(3.0, 0.0);
    state.x[1][0] = Complex64::new(2.0, 0.0);
    state.x[2][0] = Complex64::new(0.1, 0.0);

    let result = solver.solve_newton(&mut state);
    // Newton should attempt to solve (may not converge perfectly for this
    // simplified test case, but should not panic)
    assert!(result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })));
}

#[test]
fn test_nonlinear_device_params_builders() {
    let diode_params = NonlinearDeviceParams::diode(2.5e-9, 1.7);
    assert!((diode_params.is - 2.5e-9).abs() < 1e-15);
    assert!((diode_params.n - 1.7).abs() < 1e-10);

    let bjt_params = NonlinearDeviceParams::bjt(1e-15, 150.0, 2.0, 100.0);
    assert!((bjt_params.bf - 150.0).abs() < 1e-10);
    assert!((bjt_params.br - 2.0).abs() < 1e-10);
    assert!((bjt_params.vaf - 100.0).abs() < 1e-10);

    let mos_params = NonlinearDeviceParams::mosfet(0.5, 5e-4, 0.02);
    assert!((mos_params.vth - 0.5).abs() < 1e-10);
    assert!((mos_params.kp - 5e-4).abs() < 1e-10);
    assert!((mos_params.lambda - 0.02).abs() < 1e-10);
}

#[test]
fn test_bjt_evaluate_forward_active() {
    let bjt = NonlinearDeviceInstance::npn_bjt(0, 1, 2, 1e-15, 100.0);

    // Forward active: Vbe = 0.7V, Vbc = -2V (C=3V, B=1V, E=0.3V)
    let voltages = vec![3.0, 1.0, 0.3];
    let currents = bjt.evaluate(&voltages);

    // Collector current should be positive and significant
    let ic = currents
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    let ib = currents
        .iter()
        .find(|(n, _)| *n == 1)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    let ie = currents
        .iter()
        .find(|(n, _)| *n == 2)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);

    // In forward active, Ic should be much larger than Ib
    assert!(
        ic.abs() > ib.abs() * 10.0,
        "Ic should be >> Ib: Ic={}, Ib={}",
        ic.abs(),
        ib.abs()
    );

    // KCL: Ic + Ib + Ie should equal 0
    let sum = ic + ib + ie;
    assert!(
        sum.abs() < 1e-12 * ic.abs().max(1e-20),
        "KCL violation: {}",
        sum
    );
}

#[test]
fn test_mosfet_regions() {
    let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 1e-4);

    // Cutoff: Vgs = 0.5V < Vth
    let v_cutoff = vec![2.0, 0.5, 0.0, 0.0];
    let i_cutoff = nmos.evaluate(&v_cutoff);
    let id_cutoff = i_cutoff
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(
        id_cutoff.abs() < 1e-12,
        "Should be in cutoff: {}",
        id_cutoff
    );

    // Triode: Vgs = 2V, Vds = 0.5V
    let v_triode = vec![0.5, 2.0, 0.0, 0.0];
    let i_triode = nmos.evaluate(&v_triode);
    let id_triode = i_triode
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(
        id_triode.abs() > 1e-6,
        "Should have current in triode: {}",
        id_triode
    );

    // Saturation: Vgs = 2V, Vds = 5V
    let v_sat = vec![5.0, 2.0, 0.0, 0.0];
    let i_sat = nmos.evaluate(&v_sat);
    let id_sat = i_sat
        .iter()
        .find(|(n, _)| *n == 0)
        .map(|(_, i)| *i)
        .unwrap_or(0.0);
    assert!(
        id_sat.abs() > id_triode.abs() * 0.5,
        "Saturation current should be similar: {}",
        id_sat
    );
}

#[test]
fn test_line_search_mechanism() {
    // Verify that line search is properly implemented by checking
    // that the method exists and can be called
    let config = HbConfig::new(1e9).with_harmonics(1);
    let mut solver = HbSolver::new(config, 2);
    solver.add_conductance(0, 0, 0.001);
    solver.add_capacitance(1, 1, 1e-12);
    solver.add_diode(0, 1, 1e-14, 1.0);

    let mut state = HbSolverState::new(2, 1);
    state.x[0][0] = Complex64::new(0.5, 0.0);

    // Create a small delta
    let delta_x = vec![
        vec![Complex64::new(0.1, 0.0), Complex64::new(0.0, 0.0)],
        vec![Complex64::new(-0.05, 0.0), Complex64::new(0.0, 0.0)],
    ];

    // Compute initial residual
    solver.compute_full_residual(&mut state);
    // Apply line search
    let result = solver.apply_line_search(&mut state, &delta_x);
    assert!(result.is_ok(), "Line search should not fail");

    // Residual should have been recomputed
    assert!(
        state.residual_norm.is_finite(),
        "Residual should be finite after line search"
    );
}

#[test]
fn test_solve_jacobian_system() {
    let config = HbConfig::new(1e9).with_harmonics(1);
    let solver = HbSolver::new(config, 2);

    // Create a simple 4x4 identity-like Jacobian (2 nodes * 2 harmonics)
    let jac = vec![
        vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.1, 0.0),
            Complex64::new(0.0, 0.0),
        ],
        vec![
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.1, 0.0),
        ],
        vec![
            Complex64::new(0.1, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 0.0),
        ],
        vec![
            Complex64::new(0.0, 0.0),
            Complex64::new(0.1, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ],
    ];

    let mut state = HbSolverState::new(2, 1);
    state.residual[0][0] = Complex64::new(1.0, 0.0);
    state.residual[0][1] = Complex64::new(0.5, 0.0);
    state.residual[1][0] = Complex64::new(0.2, 0.0);
    state.residual[1][1] = Complex64::new(0.1, 0.0);

    let result = solver.solve_jacobian_system(&jac, &state);
    assert!(result.is_ok(), "Jacobian solve should succeed");

    let delta_x = result.unwrap();
    assert_eq!(delta_x.len(), 2);
    assert_eq!(delta_x[0].len(), 2);
}

// =========================================================================
// Comprehensive Test Suite for HB Newton Solver
// =========================================================================

#[test]
fn test_newton_solver_diode_ac_with_harmonics() {
    // Test diode with AC excitation - should generate harmonics
    let config = HbConfig::new(1e6)
        .with_harmonics(5)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    // DC bias: 1mA into diode
    solver.set_dc_source(0, 1e-3);
    // AC excitation: 0.1mA at fundamental (10% modulation - realistic small-signal)
    solver.set_ac_source(0, 0.1e-3, 0.0);

    // Load resistor
    solver.add_conductance(1, 1, 0.01);
    // Diode from node 0 to node 1
    solver.add_diode(0, 1, 1e-14, 1.0);
    // Small GMIN for stability
    solver.add_conductance(0, 0, 1e-9);
    // Capacitor for HB
    solver.add_capacitance(0, 0, 1e-12);

    let mut state = HbSolverState::new(2, 5);
    state.x[0][0] = Complex64::new(0.6, 0.0);
    state.x[1][0] = Complex64::new(0.1, 0.0);

    let result = solver.solve_newton(&mut state);
    assert!(result.is_ok(), "Should converge with AC: {:?}", result);

    // Nonlinear diode should generate harmonics
    // DC component should dominate
    let dc_magnitude = state.x[0][0].norm();
    let ac_magnitude = state.x[0][1].norm();
    assert!(dc_magnitude > ac_magnitude, "DC should dominate over AC");
}

#[test]
fn test_newton_solver_npn_bjt_amplifier() {
    // Test NPN BJT common-emitter configuration
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 4);

    // Nodes: 0=Collector, 1=Base, 2=Emitter, 3=Vcc
    // BJT: C=0, B=1, E=2

    // Collector load resistor to Vcc (node 3)
    solver.add_conductance(0, 0, 0.001); // 1k load
    solver.add_conductance(3, 3, 1.0); // Enforce Vcc
    solver.set_dc_source(3, 5.0); // 5V supply

    // Base bias
    solver.add_conductance(1, 1, 0.0001); // 10k bias
    solver.set_dc_source(1, 0.7e-3); // Base current

    // Emitter to ground
    solver.add_conductance(2, 2, 0.01); // 100 ohm

    // Capacitors for HB
    solver.add_capacitance(0, 0, 1e-12);
    solver.add_capacitance(1, 1, 1e-12);

    // Add NPN BJT
    solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);

    let mut state = HbSolverState::new(4, 3);
    // Initial guess: typical operating point
    state.x[0][0] = Complex64::new(3.0, 0.0); // Collector
    state.x[1][0] = Complex64::new(0.7, 0.0); // Base
    state.x[2][0] = Complex64::new(0.1, 0.0); // Emitter
    state.x[3][0] = Complex64::new(5.0, 0.0); // Vcc

    let result = solver.solve_newton(&mut state);
    // Should either converge or reach iteration limit without panicking
    assert!(
        result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })),
        "Should handle BJT: {:?}",
        result
    );
}

#[test]
fn test_newton_solver_poor_initial_guess() {
    // Test that source stepping helps with poor initial guess
    let config = HbConfig::new(1e6)
        .with_harmonics(1)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);
    solver.add_capacitance(0, 0, 1e-12);

    let mut state = HbSolverState::new(2, 1);
    // Very poor initial guess - way off from solution
    state.x[0][0] = Complex64::new(10.0, 0.0); // Way too high
    state.x[1][0] = Complex64::new(-5.0, 0.0); // Negative voltage

    let result = solver.solve_newton(&mut state);
    // Source stepping should help recover
    assert!(result.is_ok(), "Source stepping should help: {:?}", result);
}

#[test]
fn test_newton_solver_multi_diode_circuit() {
    // Test circuit with multiple diodes (full-wave rectifier style)
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 4);

    // Nodes: 0=input, 1=output+, 2=output-, 3=load

    // Input source
    solver.set_dc_source(0, 2e-3);
    solver.add_conductance(0, 0, 1e-9);

    // Two diodes in parallel paths (simplified rectifier)
    solver.add_diode(0, 1, 1e-14, 1.0); // D1: input to output+
    solver.add_diode(2, 0, 1e-14, 1.0); // D2: output- to input

    // Load resistor
    solver.add_conductance(1, 1, 0.01);
    solver.add_conductance(2, 2, 0.01);
    solver.add_conductance(3, 3, 0.01);

    // Coupling between outputs
    solver.add_conductance(1, 3, 0.001);

    // Capacitors
    solver.add_capacitance(0, 0, 1e-12);
    solver.add_capacitance(1, 1, 1e-12);

    let mut state = HbSolverState::new(4, 3);
    for i in 0..4 {
        state.x[i][0] = Complex64::new(0.3, 0.0);
    }

    let result = solver.solve_newton(&mut state);
    assert!(
        result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })),
        "Multi-diode should converge or fail gracefully: {:?}",
        result
    );
}

#[test]
fn test_newton_solver_high_q_resonant() {
    // Test high-Q LC resonant circuit (challenging for convergence)
    let config = HbConfig::new(1e6)
        .with_harmonics(5)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    // High-Q LC tank
    solver.add_capacitance(0, 0, 100e-12); // 100pF
    solver.add_inductance(0, 0, 253.3e-9); // ~253nH for 1MHz resonance

    // Small loss resistor
    solver.add_conductance(0, 0, 1e-6);

    // Load
    solver.add_conductance(1, 1, 0.001);

    // Driving source at resonance
    solver.set_dc_source(0, 1e-6);
    solver.set_ac_source(0, 1e-6, 0.0);

    // Add diode for nonlinearity
    solver.add_diode(0, 1, 1e-14, 1.0);

    let mut state = HbSolverState::new(2, 5);
    state.x[0][0] = Complex64::new(0.5, 0.0);

    let result = solver.solve_newton(&mut state);
    // High-Q circuits are challenging - we mainly check it doesn't panic
    assert!(
        result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })),
        "High-Q should converge or fail gracefully: {:?}",
        result
    );
}

#[test]
fn test_numerical_jacobian_consistency() {
    // Verify Jacobian matches numerical derivative of residual
    // This is critical for Newton convergence
    let config = HbConfig::new(1e6).with_harmonics(1);
    let mut solver = HbSolver::new(config, 2);

    solver.add_conductance(0, 0, 0.001);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_capacitance(0, 0, 1e-12);
    solver.set_dc_source(0, 1e-3);

    let mut state = HbSolverState::new(2, 1);
    state.x[0][0] = Complex64::new(0.6, 0.0);
    state.x[1][0] = Complex64::new(0.1, 0.0);

    let gmin = 1e-9;

    // Compute analytical Jacobian
    let analytical_jac = solver.build_full_jacobian_with_gmin(&state, gmin);

    // Compute numerical Jacobian using finite differences
    let eps = 1e-6;
    let n = solver.num_nodes;
    let h = solver.num_harmonics + 1;
    let size = n * h;

    let mut numerical_jac = vec![vec![Complex64::new(0.0, 0.0); size]; size];

    // Compute residual at base point
    solver.compute_full_residual_with_gmin(&mut state, gmin);
    let base_residual: Vec<Vec<Complex64>> = state.residual.clone();

    for col in 0..size {
        let node_idx = col / h;
        let harm_idx = col % h;

        // Perturb real part
        let orig = state.x[node_idx][harm_idx];
        state.x[node_idx][harm_idx] = orig + Complex64::new(eps, 0.0);
        solver.compute_full_residual_with_gmin(&mut state, gmin);

        for row in 0..size {
            let r_node = row / h;
            let r_harm = row % h;
            let d_residual = state.residual[r_node][r_harm] - base_residual[r_node][r_harm];
            numerical_jac[row][col] = d_residual / eps;
        }

        state.x[node_idx][harm_idx] = orig;
    }

    // Compare analytical vs numerical Jacobian
    let mut max_diff = 0.0;
    let abs_tol = 1e-8; // Absolute tolerance for near-zero values

    for i in 0..size {
        for j in 0..size {
            let diff = (analytical_jac[i][j] - numerical_jac[i][j]).norm();
            let scale = analytical_jac[i][j].norm().max(numerical_jac[i][j].norm());

            // Use absolute tolerance for near-zero values, relative for larger
            let rel_diff = if scale < abs_tol {
                // Both values are near zero - check absolute difference
                if diff < abs_tol { 0.0 } else { diff / abs_tol }
            } else {
                diff / scale
            };

            if rel_diff > max_diff {
                max_diff = rel_diff;
            }
        }
    }

    // Jacobian should be reasonably accurate (within ~5% for finite differences)
    assert!(
        max_diff < 0.05,
        "Jacobian should match numerical: max relative diff = {}",
        max_diff
    );
}

#[test]
fn test_convergence_strategies_order() {
    // Verify convergence strategies are tried in correct order:
    // 1. Direct Newton
    // 2. GMIN stepping
    // 3. Source stepping
    // 4. Pseudo-transient
    //
    // We can verify this by checking a simple circuit converges quickly
    // (direct Newton) vs a hard circuit needing continuation

    // Easy circuit - should converge with direct Newton
    let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(50);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(0, 0, 0.001);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_capacitance(0, 0, 1e-12);

    let mut state = HbSolverState::new(2, 1);
    state.x[0][0] = Complex64::new(0.6, 0.0);
    state.x[1][0] = Complex64::new(0.1, 0.0);

    let result = solver.solve_newton(&mut state);
    assert!(result.is_ok(), "Easy circuit should converge");

    // Should converge in relatively few iterations for well-conditioned circuit
    assert!(
        state.iteration < 30,
        "Should converge quickly: {} iterations",
        state.iteration
    );
}

#[test]
fn test_residual_norm_decreases() {
    // Verify Newton iterations decrease residual norm (quadratic convergence)
    let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(20);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-6);
    solver.add_capacitance(0, 0, 1e-12);

    let mut state = HbSolverState::new(2, 1);
    state.x[0][0] = Complex64::new(0.6, 0.0);
    state.x[1][0] = Complex64::new(0.1, 0.0);

    // Compute initial residual
    let gmin = 1e-9;
    solver.compute_full_residual_with_gmin(&mut state, gmin);
    let initial_norm = state.residual_norm;

    // Run solver
    let result = solver.solve_newton(&mut state);
    assert!(result.is_ok());

    // Final residual should be much smaller
    solver.compute_full_residual_with_gmin(&mut state, gmin);
    let final_norm = state.residual_norm;

    assert!(
        final_norm < initial_norm * 1e-3,
        "Residual should decrease significantly: {} -> {}",
        initial_norm,
        final_norm
    );
}

// =========================================================================
// DC Operating Point Solver Tests
// =========================================================================

#[test]
fn test_dc_solve_resistor_divider() {
    // Simple 2-resistor voltage divider: R1=R2=1k
    let config = HbConfig::new(1e6).with_harmonics(3);
    let mut solver = HbSolver::new(config, 2);

    let g = 1.0 / 1000.0; // 1 mS
    solver.add_conductance(0, 0, g);
    solver.add_conductance(0, 1, -g);
    solver.add_conductance(1, 0, -g);
    solver.add_conductance(1, 1, g);
    solver.add_conductance(1, 1, g); // R2 to ground
    solver.set_dc_source(0, 1.0 * g);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "DC solve should succeed: {:?}", result);
    let dc_solution = result.unwrap();
    assert!(
        dc_solution[1].abs() < 1.0 && dc_solution[1] >= 0.0,
        "V1 should be between 0 and 1V, got {}",
        dc_solution[1]
    );
}

#[test]
fn test_dc_solve_diode_forward_bias() {
    // Forward-biased diode: 1mA -> V ~= 0.6-0.7V
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);
    solver.add_conductance(1, 1, 1e-9);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "DC solve should succeed: {:?}", result);
    let dc_solution = result.unwrap();
    let v_diode = dc_solution[0] - dc_solution[1];
    assert!(
        v_diode > 0.5 && v_diode < 1.0,
        "Diode forward voltage should be ~0.6-0.8V, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_diode_reverse_bias() {
    // Reverse-biased diode: negative current
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, -1e-5);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);
    solver.add_conductance(1, 1, 1e-9);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "DC solve for reverse bias: {:?}", result);
    let dc_solution = result.unwrap();
    let v_diode = dc_solution[0] - dc_solution[1];
    assert!(
        v_diode < 0.0,
        "Diode should be reverse-biased, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_bjt_common_emitter() {
    // NPN BJT in common-emitter
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 4);

    // Nodes: 0=C, 1=B, 2=E, 3=Vcc
    solver.set_dc_source(1, 10e-6); // Base current
    solver.add_conductance(0, 0, 0.001);
    solver.add_conductance(0, 3, -0.001);
    solver.add_conductance(3, 0, -0.001);
    solver.add_conductance(3, 3, 1.001);
    solver.set_dc_source(3, 5.0);
    solver.add_conductance(2, 2, 1.0);
    solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);
    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "BJT DC solve: {:?}", result);
    let dc = result.unwrap();
    let vbe = dc[1] - dc[2];
    assert!(
        vbe > 0.5 && vbe < 0.9,
        "V_BE should be ~0.6-0.7V, got {}",
        vbe
    );
}

#[test]
fn test_dc_solve_with_gmin_stepping() {
    // Series diodes - requires GMIN stepping
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 4);

    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_diode(1, 2, 1e-14, 1.0);
    solver.add_diode(2, 3, 1e-14, 1.0);
    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(3, 3, 1.0);
    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(
        result.is_ok(),
        "GMIN stepping should converge: {:?}",
        result
    );
    let dc = result.unwrap();
    let total_drop = dc[0] - dc[3];
    assert!(
        total_drop > 1.5 && total_drop < 2.5,
        "3 diodes ~1.8V, got {}",
        total_drop
    );
}

#[test]
fn test_dc_solve_linear_circuit() {
    // Pure linear circuit
    let config = HbConfig::new(1e6).with_harmonics(3);
    let mut solver = HbSolver::new(config, 3);

    let g = 0.001;
    solver.add_conductance(0, 0, g);
    solver.add_conductance(0, 1, -g);
    solver.add_conductance(1, 0, -g);
    solver.add_conductance(1, 1, 2.0 * g);
    solver.add_conductance(1, 2, -g);
    solver.add_conductance(2, 1, -g);
    solver.add_conductance(2, 2, g + 1.0);
    solver.set_dc_source(0, 1e-3);

    let mut state = HbSolverState::new(3, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Linear DC solve: {:?}", result);
    for (i, &v) in result.unwrap().iter().enumerate() {
        assert!(
            v.is_finite() && v.abs() < 100.0,
            "Node {} voltage: {}",
            i,
            v
        );
    }
}

#[test]
fn test_hb_with_dc_init_convergence() {
    // Verify DC init improves HB convergence
    let config = HbConfig::new(1e6)
        .with_harmonics(5)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1e-3);
    solver.set_ac_source(0, 0.1e-3, 0.0);
    solver.add_conductance(1, 1, 0.01);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);
    solver.add_capacitance(0, 0, 1e-12);

    let mut state = HbSolverState::new(2, 5);
    let result = solver.solve_newton(&mut state);

    assert!(result.is_ok(), "HB with DC init: {:?}", result);
    assert!(state.converged, "Should converge");
    let v_diode = state.x[0][0].re - state.x[1][0].re;
    assert!(
        v_diode > 0.5 && v_diode < 0.9,
        "V_diode ~0.6-0.7V, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_nmos_saturation() {
    // NMOS in saturation: Vgs > Vth, Vds > Vgs - Vth
    // Gate=1, Drain=2, Source=3 (grounded), Bulk=3
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // To set gate to 2V: Norton equivalent with G=1S to ground and I=2A
    // V = I/G = 2A/1S = 2V
    solver.set_dc_source(1, 2.0); // 2A current into gate node
    solver.set_dc_source(2, 1e-3); // Current into drain
    solver.add_conductance(1, 1, 1.0); // Gate conductance to ground (G=1S)
    solver.add_conductance(3, 3, 1.0); // Source grounded
    // NMOS: drain=2, gate=1, source=3, bulk=3, kp=200µA/V², vth=0.5V
    solver.add_nmos(2, 1, 3, 3, 2e-4, 0.5);
    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    // Set initial gate voltage
    state.x[1][0] = Complex64::new(2.0, 0.0);

    let result = solver.solve_dc_operating_point(&mut state);
    assert!(result.is_ok(), "NMOS DC solve: {:?}", result);

    let dc = result.unwrap();
    let vgs = dc[1] - dc[3];
    let vds = dc[2] - dc[3];

    // NMOS should be on with Vgs > Vth
    assert!(vgs > 0.5, "Vgs should exceed Vth=0.5V, got {}", vgs);
    // Drain should have positive voltage
    assert!(vds > 0.0, "Vds should be positive for NMOS, got {}", vds);
}

#[test]
fn test_dc_solve_pnp_bjt() {
    // PNP BJT: emitter positive, base lower, collector even lower
    // Nodes: Emitter=0, Base=1, Collector=2, Ground=3
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // PNP: current flows from emitter to base/collector
    solver.set_dc_source(0, 1e-3); // 1mA current into emitter

    // Use add_resistor for proper MNA stamp: 100 ohm base resistor to ground
    solver.add_resistor(1, 3, 100.0); // Base to ground: G = 0.01S
    // 100 ohm collector resistor to ground
    solver.add_resistor(2, 3, 100.0); // Collector to ground: G = 0.01S

    // Ground node (large conductance to clamp)
    solver.add_conductance(3, 3, 1.0);

    // PNP: collector=2, base=1, emitter=0, Is=1fA, Bf=100
    solver.add_pnp_bjt(2, 1, 0, 1e-15, 100.0);

    // Add small GMIN for numerical stability
    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "PNP BJT DC solve: {:?}", result);
    let dc = result.unwrap();
    let veb = dc[0] - dc[1]; // Emitter-Base voltage

    // PNP should have V_EB > 0 (emitter more positive than base)
    assert!(
        veb > 0.4 && veb < 1.0,
        "V_EB should be ~0.6-0.7V for PNP, got {}",
        veb
    );
}

#[test]
fn test_dc_solve_parallel_diodes() {
    // Two parallel diodes should share current
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    // 2mA total into two parallel diodes
    solver.set_dc_source(0, 2e-3);
    solver.add_conductance(1, 1, 1.0);
    // Two identical diodes in parallel
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Parallel diodes: {:?}", result);
    let dc = result.unwrap();
    let v_diode = dc[0] - dc[1];

    // Each diode carries 1mA, so voltage should be similar to single diode at 1mA
    // (slightly higher due to thermal voltage effects, but still ~0.6V)
    assert!(
        v_diode > 0.5 && v_diode < 0.9,
        "Parallel diodes ~0.6V, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_diode_high_current() {
    // High current (100mA) should give higher forward voltage
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 100e-3); // 100mA
    solver.add_conductance(1, 1, 1.0);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "High current diode: {:?}", result);
    let dc = result.unwrap();
    let v_diode = dc[0] - dc[1];

    // At 100mA, forward voltage should be higher (~0.7-0.8V)
    assert!(
        v_diode > 0.65 && v_diode < 1.0,
        "High current diode ~0.75V, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_diode_low_current() {
    // Low current (1µA) should give lower forward voltage
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 1e-6); // 1µA
    solver.add_conductance(1, 1, 1.0);
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_conductance(0, 0, 1e-9);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Low current diode: {:?}", result);
    let dc = result.unwrap();
    let v_diode = dc[0] - dc[1];

    // At 1µA, forward voltage should be lower (~0.35-0.5V)
    assert!(
        v_diode > 0.3 && v_diode < 0.6,
        "Low current diode ~0.4V, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_diode_chain_five() {
    // 5 series diodes should give ~3V drop
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 6);

    // 5 diodes in series: 0→1→2→3→4→5
    solver.add_diode(0, 1, 1e-14, 1.0);
    solver.add_diode(1, 2, 1e-14, 1.0);
    solver.add_diode(2, 3, 1e-14, 1.0);
    solver.add_diode(3, 4, 1e-14, 1.0);
    solver.add_diode(4, 5, 1e-14, 1.0);
    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(5, 5, 1.0);
    for n in 0..6 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(6, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "5 diode chain: {:?}", result);
    let dc = result.unwrap();
    let total_drop = dc[0] - dc[5];

    // 5 diodes × 0.6V ≈ 3.0V
    assert!(
        total_drop > 2.5 && total_drop < 4.0,
        "5 diodes ~3V, got {}",
        total_drop
    );
}

#[test]
fn test_dc_solve_mixed_npn_diode() {
    // NPN BJT with diode in emitter path
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 5);

    // Collector=0, Base=1, Emitter=2, Diode cathode=3, Ground=4
    solver.set_dc_source(1, 100e-6); // Base current (100µA)

    // Collector supply: 10mA into collector (simulates Vcc through load)
    // This provides the positive supply needed for NPN forward active operation
    solver.set_dc_source(0, 10e-3);

    // Collector load resistor to ground (1k ohm)
    solver.add_resistor(0, 4, 1000.0);

    // Ground node
    solver.add_conductance(4, 4, 1.0);

    // NPN: C=0, B=1, E=2
    solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);
    // Diode in emitter path: anode=2, cathode=3
    solver.add_diode(2, 3, 1e-14, 1.0);
    // Resistor from diode cathode to ground (100 ohm)
    solver.add_resistor(3, 4, 100.0);

    for n in 0..5 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(5, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Mixed NPN+diode: {:?}", result);
    let dc = result.unwrap();
    let vbe = dc[1] - dc[2];
    let v_diode = dc[2] - dc[3];

    // Both junctions should be forward biased
    assert!(vbe > 0.4 && vbe < 1.0, "V_BE should be ~0.6V, got {}", vbe);
    assert!(
        v_diode > 0.4 && v_diode < 1.0,
        "V_diode should be ~0.6V, got {}",
        v_diode
    );
}

#[test]
fn test_dc_solve_different_ideality_factors() {
    // Diode with n=2 (recombination-dominated) vs n=1
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver_n1 = HbSolver::new(config.clone(), 2);
    let mut solver_n2 = HbSolver::new(config, 2);

    // Diode with n=1
    solver_n1.set_dc_source(0, 1e-3);
    solver_n1.add_conductance(1, 1, 1.0);
    solver_n1.add_diode(0, 1, 1e-14, 1.0); // n=1
    solver_n1.add_conductance(0, 0, 1e-9);

    // Diode with n=2
    solver_n2.set_dc_source(0, 1e-3);
    solver_n2.add_conductance(1, 1, 1.0);
    solver_n2.add_diode(0, 1, 1e-14, 2.0); // n=2
    solver_n2.add_conductance(0, 0, 1e-9);

    let mut state_n1 = HbSolverState::new(2, 3);
    let mut state_n2 = HbSolverState::new(2, 3);

    let result_n1 = solver_n1.solve_dc_operating_point(&mut state_n1);
    let result_n2 = solver_n2.solve_dc_operating_point(&mut state_n2);

    assert!(result_n1.is_ok(), "n=1 diode: {:?}", result_n1);
    assert!(result_n2.is_ok(), "n=2 diode: {:?}", result_n2);

    let v_n1 = result_n1.unwrap()[0] - state_n1.x[1][0].re;
    let v_n2 = result_n2.unwrap()[0] - state_n2.x[1][0].re;

    // n=2 diode should have higher voltage for same current
    // (Vd = n * Vt * ln(I/Is + 1))
    assert!(
        v_n2 > v_n1,
        "n=2 diode ({}) should have higher Vf than n=1 ({})",
        v_n2,
        v_n1
    );
}

// =========================================================================
// PMOS Device Tests
// =========================================================================

#[test]
fn test_dc_solve_pmos_saturation() {
    // PMOS in saturation: Vsg > |Vth|, Vsd > Vsg - |Vth|
    // Gate=1, Drain=2, Source=0, Bulk=0
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // PMOS needs Vs > Vg (source higher than gate)
    // Source at 5V (node 0), gate at 3V (node 1), drain to ground through load
    solver.set_dc_source(0, 5.0); // 5A for 5V with 1S conductance
    solver.add_conductance(0, 0, 1.0); // Source voltage
    solver.set_dc_source(1, 3.0); // 3A for 3V gate (Vsg = 2V)
    solver.add_conductance(1, 1, 1.0);
    solver.add_resistor(2, 3, 1000.0); // Drain load to ground
    solver.add_conductance(3, 3, 1.0); // Ground

    // PMOS: drain=2, gate=1, source=0, bulk=0, vth=-0.5V, kp=100µA/V²
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::Pmos,
        terminals: vec![2, 1, 0, 0],
        params: NonlinearDeviceParams::mosfet(-0.5, 1e-4, 0.0),
    });

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "PMOS DC solve: {:?}", result);
    let dc = result.unwrap();
    let vsg = dc[0] - dc[1]; // Source-gate voltage

    // PMOS should be on with Vsg > |Vth| = 0.5V
    assert!(vsg > 0.5, "Vsg should exceed |Vth|=0.5V, got {}", vsg);
}

#[test]
fn test_dc_solve_pmos_cutoff() {
    // PMOS in cutoff: Vsg < |Vth|
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // Source and gate at same voltage -> Vsg = 0 < |Vth|
    solver.set_dc_source(0, 3.0); // Source at 3V
    solver.add_conductance(0, 0, 1.0);
    solver.set_dc_source(1, 3.0); // Gate at 3V (Vsg = 0)
    solver.add_conductance(1, 1, 1.0);
    solver.add_resistor(2, 3, 1000.0); // Drain load
    solver.add_conductance(3, 3, 1.0); // Ground

    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::Pmos,
        terminals: vec![2, 1, 0, 0],
        params: NonlinearDeviceParams::mosfet(-0.5, 1e-4, 0.0),
    });

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "PMOS cutoff DC solve: {:?}", result);
    let dc = result.unwrap();
    // In cutoff, drain should be near ground (no current through load)
    assert!(
        dc[2].abs() < 0.5,
        "PMOS in cutoff should have Vd near 0, got {}",
        dc[2]
    );
}

// =========================================================================
// BJT Region Tests
// =========================================================================

#[test]
fn test_dc_solve_npn_cutoff() {
    // NPN in cutoff: Vbe < 0.5V (well below turn-on)
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // Base and emitter at same potential -> Vbe = 0
    solver.set_dc_source(0, 5.0); // Collector supply
    solver.add_conductance(0, 0, 1.0);
    solver.add_resistor(0, 3, 1000.0); // Collector load
    solver.add_conductance(1, 1, 1.0); // Base to ground
    solver.add_conductance(2, 2, 1.0); // Emitter to ground
    solver.add_conductance(3, 3, 1.0); // Ground

    // NPN: C=0, B=1, E=2
    solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "NPN cutoff: {:?}", result);
    let dc = result.unwrap();
    let vbe = dc[1] - dc[2];

    // Base and emitter both grounded, Vbe should be ~0
    assert!(vbe.abs() < 0.3, "NPN cutoff Vbe should be ~0, got {}", vbe);
}

#[test]
fn test_dc_solve_npn_saturation() {
    // NPN in saturation: both junctions forward biased
    // Requires high base current and low collector load
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // High base current (1mA) with low collector load -> saturation
    solver.set_dc_source(1, 1e-3); // 1mA base current
    solver.set_dc_source(0, 1e-3); // Low collector supply
    solver.add_conductance(0, 0, 0.1); // Small collector conductance
    solver.add_conductance(2, 2, 1.0); // Emitter grounded
    solver.add_conductance(3, 3, 1.0); // Ground

    solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "NPN saturation: {:?}", result);
    let dc = result.unwrap();
    let vce = dc[0] - dc[2];

    // In saturation, Vce should be small (< 0.5V typically)
    assert!(vce < 1.0, "NPN saturation should have low Vce, got {}", vce);
}

// =========================================================================
// MOSFET Triode Region Tests
// =========================================================================

#[test]
fn test_dc_solve_nmos_triode() {
    // NMOS in triode: Vgs > Vth but Vds < Vgs - Vth
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // High gate voltage, low drain current -> triode
    solver.set_dc_source(1, 3.0); // Gate at 3V (Vgs = 3V)
    solver.add_conductance(1, 1, 1.0);
    solver.set_dc_source(2, 0.1e-3); // Small drain current
    solver.add_conductance(2, 2, 0.001); // Small drain conductance
    solver.add_conductance(3, 3, 1.0); // Source grounded

    solver.add_nmos(2, 1, 3, 3, 1e-3, 0.5); // High kp for low Vds

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "NMOS triode: {:?}", result);
    let dc = result.unwrap();
    let vgs = dc[1] - dc[3];
    let vds = dc[2] - dc[3];

    // In triode: Vds < Vgs - Vth
    let vdsat = vgs - 0.5;
    assert!(
        vds < vdsat || vds.abs() < 0.5,
        "NMOS should be in triode: Vds={} < Vdsat={}",
        vds,
        vdsat
    );
}

// =========================================================================
// Convergence Stress Tests
// =========================================================================

#[test]
fn test_dc_solve_high_impedance_node() {
    // Node with very high impedance (nearly floating)
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 3);

    // Very small current into high impedance node
    solver.set_dc_source(0, 1e-12); // pA current
    solver.add_conductance(0, 0, 1e-12); // 1TΩ to ground
    solver.add_conductance(1, 1, 1.0);
    solver.add_conductance(2, 2, 1.0);

    solver.add_diode(0, 1, 1e-14, 1.0);

    let mut state = HbSolverState::new(3, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    // Should converge even with high impedance
    assert!(result.is_ok(), "High impedance: {:?}", result);
}

#[test]
fn test_dc_solve_zero_bias_diode() {
    // Diode at zero bias should have zero current
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(100);
    let mut solver = HbSolver::new(config, 2);

    // Both nodes grounded -> zero bias
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(1, 1, 1.0);
    solver.add_diode(0, 1, 1e-14, 1.0);

    for n in 0..2 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Zero bias diode: {:?}", result);
    let dc = result.unwrap();
    let vd = dc[0] - dc[1];

    assert!(vd.abs() < 0.01, "Zero bias should give Vd~0, got {}", vd);
}

#[test]
fn test_dc_solve_very_large_current() {
    // Very large current (10A) stress test
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 2);

    solver.set_dc_source(0, 10.0); // 10A
    solver.add_conductance(1, 1, 1.0);
    solver.add_diode(0, 1, 1e-12, 1.0); // Larger Is for high current
    solver.add_conductance(0, 0, 1e-9);

    let mut state = HbSolverState::new(2, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Large current: {:?}", result);
    let dc = result.unwrap();
    let vd = dc[0] - dc[1];

    // Very high current should give ~0.8-1.0V (log relationship)
    assert!(
        vd > 0.6 && vd < 1.5,
        "High current diode Vf should be reasonable, got {}",
        vd
    );
}

// =========================================================================
// Multi-Device Complex Circuits
// =========================================================================

#[test]
fn test_dc_solve_cascode_nmos() {
    // Cascode: two NMOS in series (common in analog design)
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 5);

    // Vdd=0, Gate1=1, Mid=2, Gate2=3, Ground=4
    solver.set_dc_source(0, 5.0);
    solver.add_conductance(0, 0, 1.0);
    solver.set_dc_source(1, 1.5); // Lower gate at 1.5V
    solver.add_conductance(1, 1, 1.0);
    solver.set_dc_source(3, 3.0); // Upper gate at 3V
    solver.add_conductance(3, 3, 1.0);
    solver.add_conductance(4, 4, 1.0);

    // Lower NMOS: D=2, G=1, S=4
    solver.add_nmos(2, 1, 4, 4, 1e-4, 0.5);
    // Upper NMOS: D=0, G=3, S=2
    solver.add_nmos(0, 3, 2, 4, 1e-4, 0.5);

    solver.add_resistor(0, 4, 5000.0); // Load

    for n in 0..5 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(5, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Cascode: {:?}", result);
    let dc = result.unwrap();

    // Mid node should be between ground and Vdd
    assert!(
        dc[2] > 0.0 && dc[2] < dc[0],
        "Cascode mid node should be 0 < {} < {}",
        dc[2],
        dc[0]
    );
}

#[test]
fn test_dc_solve_differential_pair() {
    // NPN differential pair - fundamental analog building block
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 6);

    // Vcc=0, Out1=1, Out2=2, In1=3, In2=4, Tail=5
    solver.set_dc_source(0, 10.0); // Vcc supply
    solver.add_conductance(0, 0, 1.0);

    // Collector loads
    solver.add_resistor(1, 0, 1000.0);
    solver.add_resistor(2, 0, 1000.0);

    // Input bias (slight imbalance)
    solver.set_dc_source(3, 0.7);
    solver.add_conductance(3, 3, 1.0);
    solver.set_dc_source(4, 0.65);
    solver.add_conductance(4, 4, 1.0);

    // Tail current source (simplified as resistor to negative rail)
    solver.add_conductance(5, 5, 0.001);

    // Two NPNs: C1=1, B1=3, E1=5; C2=2, B2=4, E2=5
    solver.add_npn_bjt(1, 3, 5, 1e-15, 100.0);
    solver.add_npn_bjt(2, 4, 5, 1e-15, 100.0);

    for n in 0..6 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(6, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Diff pair: {:?}", result);
    let dc = result.unwrap();

    // Output difference should reflect input imbalance
    let vout_diff = dc[1] - dc[2];
    // With 50mV input difference and gain, expect some output difference
    assert!(
        vout_diff.abs() > 0.001,
        "Diff pair should have output difference, got {}",
        vout_diff
    );
}

#[test]
fn test_dc_solve_diode_bridge() {
    // Full-wave diode bridge rectifier (4 diodes)
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 5);

    // AC+ = 0, AC- = 1, DC+ = 2, DC- = 3, Load = 4
    solver.set_dc_source(0, 1e-3); // AC+ positive
    solver.add_conductance(1, 1, 1.0); // AC- grounded

    // Load resistor
    solver.add_resistor(2, 3, 1000.0);

    // Ground reference
    solver.add_conductance(3, 3, 1.0);

    // 4 diodes forming bridge
    solver.add_diode(0, 2, 1e-14, 1.0); // AC+ to DC+
    solver.add_diode(3, 0, 1e-14, 1.0); // DC- to AC+
    solver.add_diode(1, 2, 1e-14, 1.0); // AC- to DC+
    solver.add_diode(3, 1, 1e-14, 1.0); // DC- to AC-

    for n in 0..5 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(5, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Diode bridge: {:?}", result);
    let dc = result.unwrap();

    // DC output should be positive
    let vdc = dc[2] - dc[3];
    assert!(vdc >= 0.0, "Bridge DC output should be >= 0, got {}", vdc);
}

#[test]
fn test_dc_solve_ten_series_diodes() {
    // 10 series diodes - more stressful chain
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 11);

    // 10 diodes: 0->1->2->...->10
    for i in 0..10 {
        solver.add_diode(i, i + 1, 1e-14, 1.0);
    }
    solver.set_dc_source(0, 1e-3);
    solver.add_conductance(10, 10, 1.0);

    for n in 0..11 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(11, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "10 series diodes: {:?}", result);
    let dc = result.unwrap();
    let total_drop = dc[0] - dc[10];

    // 10 diodes × ~0.6V ≈ 6V
    assert!(
        total_drop > 5.0 && total_drop < 8.0,
        "10 diodes should drop ~6V, got {}",
        total_drop
    );
}

#[test]
fn test_dc_solve_cmos_inverter() {
    // CMOS inverter: PMOS and NMOS in series
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(150);
    let mut solver = HbSolver::new(config, 4);

    // Vdd=0, Out=1, Gnd=2, In=3
    solver.set_dc_source(0, 3.3);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(2, 2, 1.0);

    // Input at mid-rail (switching point)
    solver.set_dc_source(3, 1.65);
    solver.add_conductance(3, 3, 1.0);

    // PMOS: D=1, G=3, S=0, B=0
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::Pmos,
        terminals: vec![1, 3, 0, 0],
        params: NonlinearDeviceParams::mosfet(-0.7, 5e-5, 0.0),
    });

    // NMOS: D=1, G=3, S=2, B=2
    solver.add_nmos(1, 3, 2, 2, 5e-5, 0.7);

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-9);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "CMOS inverter: {:?}", result);
    let dc = result.unwrap();

    // At mid-rail input, output should be near mid-rail
    let vout = dc[1];
    assert!(
        vout > 0.5 && vout < 2.8,
        "CMOS inverter at midpoint should output ~Vdd/2, got {}",
        vout
    );
}

// ==================== COMPREHENSIVE TESTS ====================

#[test]
fn test_dc_solve_nmos_current_mirror() {
    // NMOS current mirror - basic analog building block
    // Two matched NMOS with diode-connected reference
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 5);

    // Vdd=0, Ref=1, Out=2, Gnd=3, Gate=4
    solver.set_dc_source(0, 5.0); // Vdd
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(3, 3, 1.0); // Ground

    // Reference current (1mA through resistor)
    solver.add_resistor(0, 1, 4000.0); // ~1mA at 5V-1V

    // Diode-connected NMOS (reference): D=1, G=1, S=3
    solver.add_nmos(1, 1, 3, 3, 1e-3, 0.7);

    // Mirror NMOS (output): D=2, G=1, S=3
    solver.add_nmos(2, 1, 3, 3, 1e-3, 0.7);

    // Load resistor on output (lower resistance for more current)
    solver.add_resistor(0, 2, 4000.0);

    for n in 0..5 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(5, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "NMOS current mirror: {:?}", result);
    let dc = result.unwrap();

    // Both drains should be at similar voltages (current matching)
    let vref = dc[1];
    let vout = dc[2];
    assert!(vref > 0.5, "Reference should be above Vth: {}", vref);
    assert!(vout > 0.5, "Output should be above Vth: {}", vout);
    // Current mirror outputs should be similar (within 20%)
    assert!(
        (vref - vout).abs() / vref.max(0.1) < 0.3,
        "Current mirror should match: Vref={}, Vout={}",
        vref,
        vout
    );
}

#[test]
fn test_dc_solve_wilson_current_mirror() {
    // Wilson current mirror - improved accuracy over simple mirror
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(300);
    let mut solver = HbSolver::new(config, 6);

    // Vdd=0, N1drain=1, N2drain=2, N3drain=3, Gnd=4, Gate=5
    solver.set_dc_source(0, 5.0);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(4, 4, 1.0);

    // Reference current input
    solver.add_resistor(0, 1, 3000.0);

    // N1: D=1, G=5, S=4 (input transistor)
    solver.add_nmos(1, 5, 4, 4, 1e-3, 0.7);

    // N2: D=2, G=5, S=4 (Wilson output)
    solver.add_nmos(2, 5, 4, 4, 1e-3, 0.7);

    // N3: D=3, G=1, S=2 (cascode, gate connected to N1 drain)
    solver.add_nmos(3, 1, 2, 4, 1e-3, 0.7);

    // Diode connection: gate (5) = N1 drain (1)
    solver.add_resistor(5, 1, 0.001); // Short circuit for diode connection

    // Output load
    solver.add_resistor(0, 3, 3000.0);

    for n in 0..6 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(6, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Wilson mirror: {:?}", result);
    let dc = result.unwrap();

    // Output should be in valid range
    assert!(dc[3] > 0.5 && dc[3] < 4.5, "Wilson output: {}", dc[3]);
}

#[test]
fn test_dc_solve_source_degeneration() {
    // NMOS with source degeneration resistor - tests linearity
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 4);

    // Vdd=0, Drain=1, Source=2, Gnd=3
    solver.set_dc_source(0, 5.0);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(3, 3, 1.0);

    // Gate bias at 2.5V
    solver.set_dc_source(1, 2.5);
    solver.add_conductance(1, 1, 0.01); // Weak gate bias

    // Load resistor
    solver.add_resistor(0, 1, 1000.0);

    // NMOS: D=1, G=gate via resistor, S=2
    solver.add_nmos(1, 1, 2, 3, 1e-3, 0.7);

    // Source degeneration resistor
    solver.add_resistor(2, 3, 100.0);

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Source degen: {:?}", result);
    let dc = result.unwrap();

    // Source should be above ground due to degeneration
    assert!(dc[2] > 0.01, "Source degeneration voltage: {}", dc[2]);
}

#[test]
fn test_dc_solve_large_circuit_8_devices() {
    // Large circuit with 8 nonlinear devices - stress test
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(500);
    let mut solver = HbSolver::new(config, 10);

    // Vdd=0, nodes 1-8 for devices, Gnd=9
    solver.set_dc_source(0, 3.3);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(9, 9, 1.0);

    // Add 8 diodes in parallel with resistors
    for i in 0..8 {
        let node = i + 1;
        // Resistor from Vdd to node
        solver.add_resistor(0, node, 10000.0 + (i as f64 * 1000.0));

        // Diode from node to ground
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Diode,
            terminals: vec![node, 9],
            params: NonlinearDeviceParams::diode(1e-14, 1.0),
        });
    }

    for n in 0..10 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(10, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Large circuit 8 devices: {:?}", result);
    let dc = result.unwrap();

    // All diode nodes should be at forward bias (~0.6-0.7V)
    for i in 1..9 {
        assert!(
            dc[i] > 0.5 && dc[i] < 0.8,
            "Node {} should be at diode drop: {}",
            i,
            dc[i]
        );
    }
}

#[test]
fn test_dc_solve_stiff_circuit() {
    // Stiff circuit with 1e12 conductance ratio - numerical stress test
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 4);

    // Vdd=0, Mid=1, Diode=2, Gnd=3
    solver.set_dc_source(0, 10.0);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(3, 3, 1.0);

    // Very large resistor (1 Mohm) from Vdd to mid
    solver.add_resistor(0, 1, 1e6);

    // Very small resistor (1 ohm) from mid to diode
    solver.add_resistor(1, 2, 1.0);

    // Diode to ground
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::Diode,
        terminals: vec![2, 3],
        params: NonlinearDeviceParams::diode(1e-14, 1.0),
    });

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Stiff circuit: {:?}", result);
    let dc = result.unwrap();

    // Mid and diode should be at nearly same voltage (small R between them)
    assert!(
        (dc[1] - dc[2]).abs() < 0.01,
        "Stiff nodes should be close: V1={}, V2={}",
        dc[1],
        dc[2]
    );
    // Diode should be forward biased
    assert!(dc[2] > 0.5 && dc[2] < 0.8, "Diode voltage: {}", dc[2]);
}

#[test]
fn test_dc_solve_level_shifter() {
    // Level shifter using source follower - analog staple
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 4);

    // Vdd=0, Gate=1, Source=2, Gnd=3
    solver.set_dc_source(0, 5.0);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(3, 3, 1.0);

    // Gate input at 3V
    solver.set_dc_source(1, 3.0);
    solver.add_conductance(1, 1, 1.0);

    // Source follower NMOS: D=0 (to Vdd), G=1, S=2
    solver.add_nmos(0, 1, 2, 3, 1e-3, 0.7);

    // Current source load (resistor to ground)
    solver.add_resistor(2, 3, 500.0);

    for n in 0..4 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(4, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Level shifter: {:?}", result);
    let dc = result.unwrap();

    // Source should follow gate minus Vth, but actual voltage depends on load current
    // With 500 ohm load, source voltage can be lower due to Id*Rs drop
    let vs = dc[2];
    assert!(
        vs > 0.1 && vs < 3.0,
        "Level shifter output should be in valid source follower range: {}",
        vs
    );
}

#[test]
fn test_dc_solve_pmos_current_mirror() {
    // PMOS current mirror - complementary to NMOS mirror
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 5);

    // Vdd=0, Ref=1, Out=2, Gnd=3, Gate=4
    solver.set_dc_source(0, 5.0);
    solver.add_conductance(0, 0, 1.0);
    solver.add_conductance(3, 3, 1.0);

    // Load resistor on reference leg
    solver.add_resistor(1, 3, 4000.0);

    // Diode-connected PMOS (reference): D=1, G=1, S=0
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::Pmos,
        terminals: vec![1, 1, 0, 0],
        params: NonlinearDeviceParams::mosfet(-0.7, 1e-3, 0.0),
    });

    // Mirror PMOS (output): D=2, G=1, S=0
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::Pmos,
        terminals: vec![2, 1, 0, 0],
        params: NonlinearDeviceParams::mosfet(-0.7, 1e-3, 0.0),
    });

    // Load resistor on output
    solver.add_resistor(2, 3, 4000.0);

    for n in 0..5 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(5, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "PMOS current mirror: {:?}", result);
    let dc = result.unwrap();

    // Both outputs should be in valid range
    assert!(dc[1] > 0.5 && dc[1] < 4.5, "PMOS ref: {}", dc[1]);
    assert!(dc[2] > 0.5 && dc[2] < 4.5, "PMOS out: {}", dc[2]);
}

#[test]
fn test_dc_solve_complementary_pair() {
    // NPN-PNP complementary pair - push-pull output stage
    let config = HbConfig::new(1e6)
        .with_harmonics(3)
        .with_max_iterations(200);
    let mut solver = HbSolver::new(config, 5);

    // Vcc=0, Out=1, Vee=2, BaseN=3, BaseP=4
    solver.set_dc_source(0, 5.0); // Vcc
    solver.add_conductance(0, 0, 1.0);
    solver.set_dc_source(2, -5.0); // Vee
    solver.add_conductance(2, 2, 1.0);

    // Base bias for both transistors
    solver.set_dc_source(3, 0.6); // NPN base
    solver.add_conductance(3, 3, 1.0);
    solver.set_dc_source(4, -0.6); // PNP base
    solver.add_conductance(4, 4, 1.0);

    // NPN: C=0 (Vcc), B=3, E=1 (output)
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::NpnBjt,
        terminals: vec![0, 3, 1],
        params: NonlinearDeviceParams::default(),
    });

    // PNP: C=2 (Vee), B=4, E=1 (output)
    solver.add_nonlinear_device(NonlinearDeviceInstance {
        device_type: NonlinearDeviceType::PnpBjt,
        terminals: vec![2, 4, 1],
        params: NonlinearDeviceParams::default(),
    });

    // Load resistor on output
    solver.add_resistor(1, 2, 1000.0);

    for n in 0..5 {
        solver.add_conductance(n, n, 1e-12);
    }

    let mut state = HbSolverState::new(5, 3);
    let result = solver.solve_dc_operating_point(&mut state);

    assert!(result.is_ok(), "Complementary pair: {:?}", result);
    let dc = result.unwrap();

    // Output should be near 0V (balanced biasing)
    assert!(
        dc[1].abs() < 2.0,
        "Complementary output should be near 0V: {}",
        dc[1]
    );
}
