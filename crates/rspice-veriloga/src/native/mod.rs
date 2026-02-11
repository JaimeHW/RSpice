//! Native Code Generation for Verilog-A
//!
//! Provides JIT compilation of Verilog-A models to native machine code
//! using Cranelift. This gives near-C performance without requiring
//! external compilers.

#[cfg(feature = "native")]
pub mod cranelift_jit;

#[cfg(feature = "native")]
pub use cranelift_jit::{EvalContext, JitCompiler, JitError, JitResult, NativeModel};

use crate::codegen::CompiledModel;
#[cfg(feature = "native")]
use crate::codegen::Instruction;

#[cfg(feature = "native")]
fn first_unsupported_instruction(model: &CompiledModel) -> Option<&'static str> {
    for program in &model.assignment_programs {
        for instruction in &program.program.instructions {
            if matches!(instruction, Instruction::PushCurrent(_, _)) {
                return Some("PushCurrent");
            }
        }
    }

    for stamp in &model.stamp_programs {
        for instruction in &stamp.value_program.instructions {
            if matches!(instruction, Instruction::PushCurrent(_, _)) {
                return Some("PushCurrent");
            }
        }

        for jacobian in &stamp.jacobian_programs {
            for instruction in &jacobian.program.instructions {
                if matches!(instruction, Instruction::PushCurrent(_, _)) {
                    return Some("PushCurrent");
                }
            }
        }
    }

    None
}

/// Try to compile a model to native code
///
/// Returns `Some(NativeModel)` if compilation succeeds, `None` otherwise.
/// The caller should fall back to bytecode interpretation if this returns `None`.
#[cfg(feature = "native")]
pub fn try_compile_native(model: &CompiledModel) -> Option<NativeModel> {
    if let Some(opcode) = first_unsupported_instruction(model) {
        log::debug!(
            "[JIT] Skipping native compilation for '{}': unsupported instruction {}",
            model.name,
            opcode
        );
        return None;
    }

    log::debug!("[JIT] Compiling model '{}' with Cranelift...", model.name);

    match JitCompiler::new() {
        Ok(compiler) => match compiler.compile(model) {
            Ok(native_model) => {
                log::info!(
                    "[JIT] Successfully compiled '{}': {} assignments, {} stamps",
                    model.name,
                    model.assignment_programs.len(),
                    model.stamp_programs.len()
                );
                Some(native_model)
            }
            Err(e) => {
                log::warn!("[JIT] Compilation failed for '{}': {}", model.name, e);
                None
            }
        },
        Err(e) => {
            log::warn!("[JIT] Failed to create compiler: {}", e);
            None
        }
    }
}

/// Stub for when native feature is disabled
#[cfg(not(feature = "native"))]
pub fn try_compile_native(_model: &CompiledModel) -> Option<()> {
    None
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "native")]
    use super::*;

    #[cfg(feature = "native")]
    use crate::codegen::{
        AssignmentProgram, BytecodeProgram, CompiledParameter, Instruction, StampIndex,
        StampLocation, StampProgram,
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
                default: 0.001, // 1kΩ
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
    fn test_jit_falls_back_for_push_current() {
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
            native.is_none(),
            "PushCurrent must fall back to interpreter for correctness"
        );
    }
}
