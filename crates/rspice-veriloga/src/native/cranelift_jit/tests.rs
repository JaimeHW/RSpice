use super::*;
use crate::codegen::{CompiledParameter, StampIndex, StampLocation};

#[test]
fn test_jit_compiler_creation() {
    let compiler = JitCompiler::new();
    assert!(compiler.is_ok(), "JIT compiler should initialize");
}

#[test]
fn test_simple_model_compilation() {
    let model = CompiledModel {
        name: "test".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![CompiledParameter {
            name: "g".into(),
            default: 0.001,
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
                    Instruction::PushParam(0),
                    Instruction::PushVoltage(0, 1),
                    Instruction::Mul,
                ],
            },
            jacobian_programs: vec![],
        }],
        lookup_tables: vec![],
        laplace_filters: vec![],
        internal_nodes: 0,
        branch_currents: 0,
    };

    let compiler = JitCompiler::new().unwrap();
    let result = compiler.compile(&model);
    assert!(
        result.is_ok(),
        "Simple model should compile: {:?}",
        result.err()
    );
}

#[test]
fn test_compiled_stamp_evaluates_with_typed_function_pointer_bridge() {
    let model = CompiledModel {
        name: "gm".into(),
        num_terminals: 2,
        terminal_names: vec!["p".into(), "n".into()],
        parameters: vec![CompiledParameter {
            name: "g".into(),
            default: 0.002,
            min: Some(0.0),
            max: None,
        }],
        num_variables: 0,
        assignment_programs: vec![],
        stamp_programs: vec![StampProgram {
            stamp_locations: vec![],
            value_program: BytecodeProgram {
                instructions: vec![
                    Instruction::PushParam(0),
                    Instruction::PushVoltage(0, 1),
                    Instruction::Mul,
                ],
            },
            jacobian_programs: vec![],
        }],
        lookup_tables: vec![],
        laplace_filters: vec![],
        internal_nodes: 0,
        branch_currents: 0,
    };

    let native = JitCompiler::new()
        .expect("jit compiler")
        .compile(&model)
        .expect("compiled model");

    let voltages = [1.5, 0.5];
    let params = [0.002];
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
        timestep: 0.0,
        state_prev: std::ptr::null(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
    };

    let value = native.evaluate_stamp(0, &ctx, &[]);
    assert!((value - 0.002).abs() < 1e-12);
}

// ============================================
// Laplace JIT Tests - Comprehensive Coverage
// ============================================

/// Safe wrapper for rspice_laplace_step for use in tests.
/// Encapsulates the unsafe FFI call in a safe abstraction.
fn laplace_step_test(
    filters_ptr: *mut crate::laplace::StateSpaceFilter,
    filters_len: usize,
    filter_id: usize,
    input: f64,
    timestep: f64,
) -> f64 {
    // This wrapper provides a test-friendly interface to the JIT helper function
    unsafe { rspice_laplace_step(filters_ptr, filters_len, filter_id, input, timestep) }
}

/// Safe wrapper for rspice_current_lookup for use in tests.
fn current_lookup_test(
    branch_currents: &[f64],
    currents: &[f64],
    num_terminals: usize,
    pos: usize,
    neg: usize,
) -> f64 {
    unsafe {
        rspice_current_lookup(
            branch_currents.as_ptr(),
            branch_currents.len(),
            currents.as_ptr(),
            currents.len(),
            num_terminals,
            pos,
            neg,
        )
    }
}

#[test]
fn test_current_lookup_prefers_terminal_pair_table() {
    let branch_currents = vec![
        f64::NAN,
        2.0e-3, //
        -2.0e-3,
        f64::NAN,
    ];
    let currents = vec![9.0e-3];

    let value = current_lookup_test(&branch_currents, &currents, 2, 0, 1);
    assert!((value - 2.0e-3).abs() < 1e-15);
}

#[test]
fn test_current_lookup_falls_back_to_first_current_for_nan_entry() {
    let branch_currents = vec![
        f64::NAN,
        f64::NAN, //
        f64::NAN,
        f64::NAN,
    ];
    let currents = vec![1.25e-3, 7.0e-3];

    let value = current_lookup_test(&branch_currents, &currents, 2, 1, 0);
    assert!((value - 1.25e-3).abs() < 1e-15);
}

#[test]
fn test_current_lookup_falls_back_to_first_current_for_out_of_bounds_pair() {
    let branch_currents = vec![f64::NAN; 4];
    let currents = vec![3.0e-3];

    let value = current_lookup_test(&branch_currents, &currents, 2, 4, 1);
    assert!((value - 3.0e-3).abs() < 1e-15);
}

#[test]
fn test_current_lookup_returns_zero_without_currents_or_pair() {
    let branch_currents = vec![f64::NAN; 4];
    let currents = Vec::new();

    let value = current_lookup_test(&branch_currents, &currents, 2, 1, 1);
    assert_eq!(value, 0.0);
}

#[test]
fn test_laplace_step_test_null_pointer() {
    // Should return input unchanged when filters_ptr is null
    let result = laplace_step_test(std::ptr::null_mut(), 0, 0, 5.0, 1e-4);
    assert!(
        (result - 5.0).abs() < 1e-10,
        "Null pointer should return input unchanged"
    );
}

#[test]
fn test_laplace_step_test_out_of_bounds() {
    // Should return input unchanged when filter_id >= filters_len
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let result = laplace_step_test(&mut filter, 1, 5, 3.0, 1e-4); // filter_id 5 >= len 1
    assert!(
        (result - 3.0).abs() < 1e-10,
        "Out of bounds should return input unchanged"
    );
}

#[test]
fn test_laplace_step_test_dc_passthrough() {
    // Zero timestep should return DC gain * input
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    // For lowpass, DC gain is 1.0
    let result = laplace_step_test(&mut filter, 1, 0, 2.5, 0.0);
    assert!(
        (result - 2.5).abs() < 1e-10,
        "DC passthrough should return dc_gain * input"
    );
}

#[test]
fn test_laplace_step_test_negative_timestep() {
    // Negative timestep should also return DC passthrough
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let result = laplace_step_test(&mut filter, 1, 0, 1.5, -1e-4);
    assert!(
        (result - 1.5).abs() < 1e-10,
        "Negative timestep should return DC passthrough"
    );
}

#[test]
fn test_laplace_step_test_first_order_single_step() {
    // Single step of first-order lowpass
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let result = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);
    // First step from zero should produce a non-zero result (but less than 1.0)
    assert!(result > 0.0, "First step should produce positive output");
    assert!(result < 1.0, "First step should not reach steady state");
}

#[test]
fn test_laplace_step_test_first_order_convergence() {
    // Multiple steps should converge to steady state
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let timestep = 1e-4;
    let mut output = 0.0;

    // Step 1000 times (100ms with 100us steps)
    for _ in 0..1000 {
        output = laplace_step_test(&mut filter, 1, 0, 1.0, timestep);
    }

    // Should be very close to 1.0 (steady state for unity DC gain)
    assert!(
        (output - 1.0).abs() < 0.01,
        "First-order should converge to 1.0, got {}",
        output
    );
}

#[test]
fn test_laplace_step_test_second_order_convergence() {
    // Second-order lowpass should also converge
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_second_order(100.0, 0.707);
    let timestep = 1e-4;
    let mut output = 0.0;

    for _ in 0..1000 {
        output = laplace_step_test(&mut filter, 1, 0, 1.0, timestep);
    }

    assert!(
        (output - 1.0).abs() < 0.01,
        "Second-order should converge to 1.0, got {}",
        output
    );
}

#[test]
fn test_laplace_step_test_state_persistence() {
    // State should persist between calls
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);

    // First step
    let out1 = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);
    // Second step
    let out2 = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);

    // Output should increase as filter charges
    assert!(
        out2 > out1,
        "Output should increase with state persistence: {} > {}",
        out2,
        out1
    );
}

#[test]
fn test_laplace_step_test_multiple_filters() {
    // Multiple filters in array
    let mut filters = vec![
        crate::laplace::StateSpaceFilter::lowpass_first_order(100.0),
        crate::laplace::StateSpaceFilter::lowpass_first_order(1000.0), // Faster filter
    ];

    let timestep = 1e-4;

    // Step each filter 100 times
    let mut out0 = 0.0;
    let mut out1 = 0.0;
    for _ in 0..100 {
        out0 = laplace_step_test(filters.as_mut_ptr(), 2, 0, 1.0, timestep);
        out1 = laplace_step_test(filters.as_mut_ptr(), 2, 1, 1.0, timestep);
    }

    // 1000Hz filter should settle faster than 100Hz filter
    assert!(
        out1 > out0,
        "1000Hz filter should be faster: {} > {}",
        out1,
        out0
    );
}

#[test]
fn test_laplace_step_test_differentiator() {
    // Differentiator should block DC
    let mut filter = crate::laplace::StateSpaceFilter::differentiator(0.001);

    // Step with constant input - should decay to zero
    let mut output = 0.0;
    for _ in 0..1000 {
        output = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);
    }

    // Differentiator DC gain is 0
    assert!(
        output.abs() < 0.1,
        "Differentiator should approach 0 for constant input, got {}",
        output
    );
}

#[test]
fn test_laplace_step_test_integrator() {
    // Integrator (leaky) should settle
    let mut filter = crate::laplace::StateSpaceFilter::integrator(0.001);

    let mut output = 0.0;
    for _ in 0..1000 {
        output = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);
    }

    // Should approach 1.0 (DC gain = 1 for this leaky integrator)
    assert!(
        (output - 1.0).abs() < 0.01,
        "Integrator should converge, got {}",
        output
    );
}

#[test]
fn test_laplace_step_test_zero_input() {
    // Zero input should keep output at zero
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);

    let mut output = 0.0;
    for _ in 0..100 {
        output = laplace_step_test(&mut filter, 1, 0, 0.0, 1e-4);
    }

    assert!(
        output.abs() < 1e-10,
        "Zero input should produce zero output, got {}",
        output
    );
}

#[test]
fn test_laplace_step_test_varying_input() {
    // Filter should track varying input
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(1000.0); // Fast filter

    // Step up to 1.0
    for _ in 0..500 {
        laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);
    }
    let high = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-4);

    // Step down to 0.0
    for _ in 0..500 {
        laplace_step_test(&mut filter, 1, 0, 0.0, 1e-4);
    }
    let low = laplace_step_test(&mut filter, 1, 0, 0.0, 1e-4);

    assert!(high > 0.9, "Should track high input");
    assert!(low < 0.1, "Should track low input");
}

#[test]
fn test_laplace_step_test_extreme_timestep() {
    // Very large timestep should still work
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let result = laplace_step_test(&mut filter, 1, 0, 1.0, 1.0); // 1 second timestep

    // Should be close to steady state in one step
    assert!(
        (result - 1.0).abs() < 0.01,
        "Large timestep should approach steady state, got {}",
        result
    );
}

#[test]
fn test_laplace_step_test_tiny_timestep() {
    // Very small timestep should make small change
    let mut filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let result = laplace_step_test(&mut filter, 1, 0, 1.0, 1e-10); // 0.1ns timestep

    // Should be very close to zero (almost no time passed)
    assert!(
        result < 0.01,
        "Tiny timestep should produce tiny output, got {}",
        result
    );
}

#[test]
fn test_eval_context_laplace_fields() {
    // Verify EvalContext has laplace fields at correct offsets
    use std::mem;

    let ctx = EvalContext {
        voltages: std::ptr::null(),
        internal_voltages: std::ptr::null(),
        params: std::ptr::null(),
        branch_currents: std::ptr::null(),
        branch_currents_len: 0,
        currents: std::ptr::null(),
        currents_len: 0,
        num_terminals: 0,
        temperature: 300.0,
        time: 0.0,
        timestep: 1e-6,
        state_prev: std::ptr::null(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
    };

    // Verify struct has the laplace fields
    assert!(ctx.laplace_filters.is_null());
    assert_eq!(ctx.laplace_filters_len, 0);

    // Verify struct size is reasonable
    let size = mem::size_of::<EvalContext>();
    assert!(
        size > 80,
        "EvalContext should include laplace fields, size = {}",
        size
    );
}

#[test]
fn test_jit_laplace_consistency_with_interpreter() {
    // Verify JIT helper produces same results as direct StateSpaceFilter::step
    let mut jit_filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);
    let mut interp_filter = crate::laplace::StateSpaceFilter::lowpass_first_order(100.0);

    let timestep = 1e-4;

    for i in 0..100 {
        let input = if i < 50 { 1.0 } else { 0.5 };
        let jit_out = laplace_step_test(&mut jit_filter, 1, 0, input, timestep);
        let interp_out = interp_filter.step(input, timestep);

        assert!(
            (jit_out - interp_out).abs() < 1e-10,
            "JIT and interpreter should match at step {}: {} vs {}",
            i,
            jit_out,
            interp_out
        );
    }
}
