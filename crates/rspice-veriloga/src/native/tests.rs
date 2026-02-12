#[cfg(feature = "native")]
use super::*;

#[cfg(feature = "native")]
use crate::codegen::{
    AssignmentProgram, BytecodeProgram, CompiledParameter, Instruction, StampIndex, StampLocation,
    StampProgram,
};

#[test]
#[cfg(feature = "native")]
fn test_jit_simple_resistor() {
    // Create a simple resistor model: I = G * V
    let model = CompiledModel {
        name: "resistor".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![CompiledParameter {
            name: "g".into(),
            default: 0.001, // 1kO
            min: Some(0.0),
            max: None,
        }],
        num_variables: 0,
        assignment_programs: vec![],
        stamp_programs: vec![StampProgram {
            stamp_locations: vec![StampLocation {
                row: StampIndex::Terminal(0),
                col: StampIndex::Ground,
                sign: 1.0,
            }],
            value_program: BytecodeProgram {
                instructions: vec![
                    Instruction::PushParam(0),      // G
                    Instruction::PushVoltage(0, 1), // V(p,n)
                    Instruction::Mul,               // G * V
                ],
            },
            jacobian_programs: vec![],
        }],
        lookup_tables: vec![],
        laplace_filters: vec![],
        internal_nodes: 0,
        branch_currents: 0,
    };

    let native = try_compile_native(&model);
    assert!(native.is_some(), "Resistor model should compile");

    let native = native.unwrap();

    // Test evaluation
    let voltages = [1.0, 0.0]; // 1V across resistor
    let params = [0.001]; // 1mS conductance
    let vars = [];

    let ctx = EvalContext {
        voltages: voltages.as_ptr(),
        internal_voltages: std::ptr::null(),
        params: params.as_ptr(),
        branch_currents: std::ptr::null(),
        branch_currents_len: 0,
        currents: std::ptr::null(),
        currents_len: 0,
        num_terminals: 2,
        temperature: 300.0,
        time: 0.0,
        timestep: 1e-9,
        state_prev: std::ptr::null(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
    };

    let current = native.evaluate_stamp(0, &ctx, &vars);
    let expected = 0.001 * 1.0; // G * V = 1mA

    assert!(
        (current - expected).abs() < 1e-12,
        "Current should be {} but got {}",
        expected,
        current
    );
}

#[test]
#[cfg(feature = "native")]
fn test_jit_with_assignments() {
    // Model with intermediate variable: T0 = V * 2, I = G * T0
    let model = CompiledModel {
        name: "with_vars".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![CompiledParameter {
            name: "g".into(),
            default: 0.001,
            min: None,
            max: None,
        }],
        num_variables: 1,
        assignment_programs: vec![AssignmentProgram {
            var_index: 0,
            program: BytecodeProgram {
                instructions: vec![
                    Instruction::PushVoltage(0, 1),
                    Instruction::PushConst(2.0),
                    Instruction::Mul, // T0 = V * 2
                ],
            },
        }],
        stamp_programs: vec![StampProgram {
            stamp_locations: vec![StampLocation {
                row: StampIndex::Terminal(0),
                col: StampIndex::Ground,
                sign: 1.0,
            }],
            value_program: BytecodeProgram {
                instructions: vec![
                    Instruction::PushParam(0),    // G
                    Instruction::PushVariable(0), // T0
                    Instruction::Mul,             // G * T0
                ],
            },
            jacobian_programs: vec![],
        }],
        lookup_tables: vec![],
        laplace_filters: vec![],
        internal_nodes: 0,
        branch_currents: 0,
    };

    let native = try_compile_native(&model);
    assert!(native.is_some(), "Model with variables should compile");

    let native = native.unwrap();

    let voltages = [1.0, 0.0];
    let params = [0.001];
    let mut vars = [0.0];

    let ctx = EvalContext {
        voltages: voltages.as_ptr(),
        internal_voltages: std::ptr::null(),
        params: params.as_ptr(),
        branch_currents: std::ptr::null(),
        branch_currents_len: 0,
        currents: std::ptr::null(),
        currents_len: 0,
        num_terminals: 2,
        temperature: 300.0,
        time: 0.0,
        timestep: 1e-9,
        state_prev: std::ptr::null(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
    };

    // First compute assignments
    native.evaluate_assignments(&ctx, &mut vars);
    assert!((vars[0] - 2.0).abs() < 1e-12, "T0 should be 2.0");

    // Then evaluate stamp
    let current = native.evaluate_stamp(0, &ctx, &vars);
    let expected = 0.001 * 2.0; // G * T0 = 2mA

    assert!(
        (current - expected).abs() < 1e-12,
        "Current should be {} but got {}",
        expected,
        current
    );
}

#[test]
#[cfg(feature = "native")]
fn test_jit_compiles_push_current_program() {
    let model = CompiledModel {
        name: "requires_push_current".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![],
        num_variables: 0,
        assignment_programs: vec![],
        stamp_programs: vec![StampProgram {
            stamp_locations: vec![],
            value_program: BytecodeProgram {
                instructions: vec![Instruction::PushCurrent(0, 1)],
            },
            jacobian_programs: vec![],
        }],
        lookup_tables: vec![],
        laplace_filters: vec![],
        internal_nodes: 0,
        branch_currents: 0,
    };

    let native = try_compile_native(&model);
    assert!(
        native.is_some(),
        "PushCurrent should compile in native mode"
    );
    let native = native.unwrap();

    let voltages = [3.0, 1.0];
    let params = [];
    let vars = [];

    let mut branch_currents = vec![f64::NAN; 4];
    branch_currents[1] = 2.0e-3;
    branch_currents[2] = -2.0e-3;
    let fallback_currents = vec![9.0e-3];

    let ctx_from_pair = EvalContext {
        voltages: voltages.as_ptr(),
        internal_voltages: std::ptr::null(),
        params: params.as_ptr(),
        branch_currents: branch_currents.as_ptr(),
        branch_currents_len: branch_currents.len(),
        currents: fallback_currents.as_ptr(),
        currents_len: fallback_currents.len(),
        num_terminals: 2,
        temperature: 300.0,
        time: 0.0,
        timestep: 1e-9,
        state_prev: std::ptr::null(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
    };
    let pair_value = native.evaluate_stamp(0, &ctx_from_pair, &vars);
    assert!((pair_value - 2.0e-3).abs() < 1e-12);

    let nan_branch_currents = vec![f64::NAN; 4];
    let ctx_from_fallback = EvalContext {
        voltages: voltages.as_ptr(),
        internal_voltages: std::ptr::null(),
        params: params.as_ptr(),
        branch_currents: nan_branch_currents.as_ptr(),
        branch_currents_len: nan_branch_currents.len(),
        currents: fallback_currents.as_ptr(),
        currents_len: fallback_currents.len(),
        num_terminals: 2,
        temperature: 300.0,
        time: 0.0,
        timestep: 1e-9,
        state_prev: std::ptr::null(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
    };
    let fallback_value = native.evaluate_stamp(0, &ctx_from_fallback, &vars);
    assert!((fallback_value - 9.0e-3).abs() < 1e-12);
}
