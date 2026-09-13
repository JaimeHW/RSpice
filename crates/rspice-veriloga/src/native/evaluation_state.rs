//! Execute procedural input reads on each host's actual native backend.

#[cfg(target_arch = "aarch64")]
use super::aarch64::codegen::compile_assignment_pass_function;
use super::abi::EvalContext;
use super::assignment::NativeAssignment;
use super::expr::{NativeOp, NativeProgram};
use super::runtime::ExecutableMemory;
#[cfg(target_arch = "x86_64")]
use super::x64::codegen::compile_assignment_pass_function;

fn assignment(index: usize, op: NativeOp) -> NativeAssignment {
    NativeAssignment::Direct {
        var_index: index,
        program: NativeProgram::from_ops_for_test(vec![op], 1, Vec::new(), Vec::new()),
    }
}

#[test]
fn evaluation_state_reads_survive_candidate_publication_and_preserve_bits() {
    let assignments = [
        assignment(0, NativeOp::LoadEvaluationState(0)),
        assignment(1, NativeOp::Const(999.0)),
        assignment(2, NativeOp::LoadEvaluationState(0)),
    ];
    let bytes = compile_assignment_pass_function(&assignments).unwrap();
    let memory = ExecutableMemory::allocate(&bytes).unwrap();
    let entry: extern "C" fn(*const EvalContext, *mut f64) =
        unsafe { std::mem::transmute(memory.ptr_at(0).unwrap()) };

    for input in [0.0, -0.0, 1.25, f64::from_bits(0x7ff8_0000_0000_0123)] {
        let saved = [input];
        let mut variables = [91.0, 92.0, 93.0];
        let context = EvalContext {
            evaluation_state_inputs: saved.as_ptr(),
            evaluation_state_inputs_len: saved.len(),
            ..EvalContext::empty_for_test()
        };
        for _ in 0..2 {
            entry(&context, variables.as_mut_ptr());
            assert!(context.take_runtime_error().is_none());
            assert_eq!(variables[0].to_bits(), input.to_bits());
            assert_eq!(variables[1], 999.0);
            assert_eq!(variables[2].to_bits(), input.to_bits());
            assert_eq!(saved[0].to_bits(), input.to_bits());
        }
    }
}

#[test]
fn evaluation_state_unavailable_aborts_before_subsequent_publication() {
    let saved = [4.0];
    for (pointer, length, index) in [
        (std::ptr::null(), 0, 0),
        (std::ptr::null(), 1, 0),
        (saved.as_ptr(), 0, 0),
        (saved.as_ptr(), 1, 1),
    ] {
        let assignments = [
            assignment(0, NativeOp::Const(5.0)),
            assignment(1, NativeOp::LoadEvaluationState(index)),
            assignment(2, NativeOp::Const(7.0)),
        ];
        let bytes = compile_assignment_pass_function(&assignments).unwrap();
        let memory = ExecutableMemory::allocate(&bytes).unwrap();
        let entry: extern "C" fn(*const EvalContext, *mut f64) =
            unsafe { std::mem::transmute(memory.ptr_at(0).unwrap()) };
        let context = EvalContext {
            evaluation_state_inputs: pointer,
            evaluation_state_inputs_len: length,
            ..EvalContext::empty_for_test()
        };
        let mut variables = [91.0, 92.0, 93.0];
        entry(&context, variables.as_mut_ptr());
        assert_eq!(
            context.take_runtime_error().as_deref(),
            Some("procedural evaluation-state input is unavailable")
        );
        assert_eq!(variables, [5.0, 92.0, 93.0]);
    }
}
