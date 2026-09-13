//! Execute shared math across helper calls and ordered publication boundaries
//! on the host's real native backend, including ARM64 CI runners.

#[cfg(target_arch = "aarch64")]
use super::aarch64::codegen::compile_assignment_pass_function;
use super::abi::EvalContext;
use super::assignment::NativeAssignment;
use super::expr::{BinaryMathOp, NativeOp, NativeProgram, UnaryMathOp, native_op_stack_effect};
use super::runtime::ExecutableMemory;
#[cfg(target_arch = "x86_64")]
use super::x64::codegen::compile_assignment_pass_function;

fn program(ops: Vec<NativeOp>) -> NativeProgram {
    let (mut depth, mut maximum) = (0_usize, 0);
    for op in &ops {
        let (pop, push) = native_op_stack_effect(op);
        depth = depth.checked_sub(pop).unwrap() + push;
        maximum = maximum.max(depth);
    }
    assert_eq!(depth, 1);
    NativeProgram::from_ops_for_test(ops, maximum, Vec::new(), Vec::new())
}

fn execute(assignments: &[NativeAssignment], variables: &mut [f64]) -> EvalContext {
    let context = EvalContext::empty_for_test();
    execute_with_context(assignments, variables, &context);
    context
}

fn execute_with_context(
    assignments: &[NativeAssignment],
    variables: &mut [f64],
    context: &EvalContext,
) {
    let bytes = compile_assignment_pass_function(assignments).unwrap();
    let memory = ExecutableMemory::allocate(&bytes).unwrap();
    let entry: extern "C" fn(*const EvalContext, *mut f64) =
        unsafe { std::mem::transmute(memory.ptr_at(0).unwrap()) };
    entry(context, variables.as_mut_ptr());
}

fn assignment(index: usize, ops: Vec<NativeOp>) -> NativeAssignment {
    NativeAssignment::Direct {
        var_index: index,
        program: program(ops),
    }
}

fn same_value(actual: f64, expected: f64) {
    if expected.is_nan() {
        assert!(actual.is_nan());
    } else {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
}

#[test]
fn dependent_publications_preserve_bits_and_each_reaching_write() {
    let assignments = [
        assignment(0, vec![NativeOp::LoadVariable(7)]),
        assignment(
            1,
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::UnaryMath(UnaryMathOp::Sin),
            ],
        ),
        assignment(7, vec![NativeOp::Const(-0.0)]),
        assignment(2, vec![NativeOp::LoadVariable(7)]),
        assignment(3, vec![NativeOp::LoadVariable(0)]),
        assignment(0, vec![NativeOp::Const(99.0)]),
        assignment(4, vec![NativeOp::LoadVariable(0)]),
    ];
    for input in [
        0.0,
        -0.0,
        0.75,
        -0.75,
        f64::from_bits(0x7ff8_0000_0000_0123),
    ] {
        let mut variables = [91.0; 8];
        variables[7] = input;
        let context = execute(&assignments, &mut variables);
        assert!(context.take_runtime_error().is_none());
        assert_eq!(variables[3].to_bits(), input.to_bits());
        same_value(variables[1], input.sin());
        assert_eq!(variables[2].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(variables[4], 99.0);
    }
}

#[test]
fn dependent_publications_survive_register_spills_and_math_calls() {
    let mut assignments: Vec<_> = (1..=64)
        .map(|index| {
            assignment(
                index,
                vec![NativeOp::LoadVariable(0), NativeOp::AddConst(index as f64)],
            )
        })
        .collect();
    assignments.push(assignment(
        65,
        vec![
            NativeOp::LoadVariable(0),
            NativeOp::UnaryMath(UnaryMathOp::Sin),
        ],
    ));
    let mut sum = vec![NativeOp::LoadVariable(1)];
    for index in 2..=64 {
        sum.extend([NativeOp::LoadVariable(index), NativeOp::Add]);
    }
    assignments.push(assignment(66, sum));
    assignments.push(assignment(67, vec![NativeOp::LoadVariable(66)]));
    for input in [-123.75, -0.0, 0.125, 1e16] {
        let mut variables = [91.0; 68];
        variables[0] = input;
        let context = execute(&assignments, &mut variables);
        assert!(context.take_runtime_error().is_none());
        let mut expected = input + 1.0;
        for (index, value) in variables.iter().enumerate().take(65).skip(1) {
            same_value(*value, input + index as f64);
            if index > 1 {
                expected += input + index as f64;
            }
        }
        same_value(variables[65], input.sin());
        same_value(variables[66], expected);
        same_value(variables[67], expected);
    }
}

#[test]
fn dynamic_reads_after_publication_observe_the_updated_array() {
    let read = vec![
        NativeOp::LoadVariable(0),
        NativeOp::LoadVariableDyn {
            base: 1,
            len: 2,
            lower: 0,
        },
    ];
    let assignments = [
        assignment(3, read.clone()),
        assignment(1, vec![NativeOp::Const(7.0)]),
        assignment(4, read),
        assignment(5, vec![NativeOp::LoadVariable(3)]),
        assignment(6, vec![NativeOp::LoadVariable(1)]),
    ];
    for index in [0.0, 1.0] {
        let mut variables = [index, 2.0, 3.0, 91.0, 92.0, 93.0, 94.0];
        let context = execute(&assignments, &mut variables);
        assert!(context.take_runtime_error().is_none());
        let previous = if index == 0.0 { 2.0 } else { 3.0 };
        assert_eq!(variables[3], previous);
        assert_eq!(variables[4], if index == 0.0 { 7.0 } else { 3.0 });
        assert_eq!(variables[5], previous);
        assert_eq!(variables[6], 7.0);
    }
}

#[test]
fn failed_check_of_published_value_keeps_prior_stores_and_stops_later_ones() {
    let assignments = [
        assignment(0, vec![NativeOp::LoadVariable(3)]),
        assignment(
            1,
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::Const(123.0),
                NativeOp::CheckedValue,
            ],
        ),
        assignment(2, vec![NativeOp::LoadVariable(0)]),
    ];
    for input in [0.75, f64::NAN] {
        let mut variables = [91.0, 92.0, 93.0, input];
        let context = execute(&assignments, &mut variables);
        assert_eq!(variables[0].to_bits(), input.to_bits());
        if input.is_nan() {
            assert!(
                context
                    .take_runtime_error()
                    .unwrap()
                    .contains("ddx operand is not finite")
            );
            assert_eq!(variables[1..3], [92.0, 93.0]);
        } else {
            assert!(context.take_runtime_error().is_none());
            assert_eq!(variables[1..3], [123.0, input]);
        }
    }
}

#[test]
fn pure_math_results_survive_intervening_calls_and_assignment_publication() {
    for (op, operands, expected) in [
        (
            NativeOp::UnaryMath(UnaryMathOp::Exp),
            vec![0.75],
            0.75_f64.exp(),
        ),
        (
            NativeOp::UnaryMath(UnaryMathOp::Exp),
            vec![1000.0],
            f64::INFINITY,
        ),
        (NativeOp::UnaryMath(UnaryMathOp::Log), vec![-1.0], f64::NAN),
        (
            NativeOp::BinaryMath(BinaryMathOp::Pow),
            vec![3.0, 1.25],
            3.0_f64.powf(1.25),
        ),
        (
            NativeOp::BinaryMath(BinaryMathOp::Hypot),
            vec![3e200, 4e200],
            3e200_f64.hypot(4e200),
        ),
        (
            NativeOp::BinaryMath(BinaryMathOp::Mod),
            vec![-0.0, 2.0],
            -0.0,
        ),
        (
            NativeOp::ProductRatio,
            vec![1e300, 1e300, 1e300, 1e300],
            1.0,
        ),
        (
            NativeOp::SumProductsDiv(2),
            vec![1e300, 1e300, -1e300, 1e300, 1.0],
            0.0,
        ),
    ] {
        let input_count = operands.len();
        let mut expression: Vec<_> = (0..input_count).map(NativeOp::LoadVariable).collect();
        expression.push(op);
        let mut repeated = expression.clone();
        repeated.push(NativeOp::AddConst(1.0));
        let assignments = [
            assignment(input_count, expression),
            assignment(
                input_count + 1,
                vec![
                    NativeOp::LoadVariable(0),
                    NativeOp::UnaryMath(UnaryMathOp::Sin),
                ],
            ),
            assignment(input_count + 2, repeated),
            assignment(
                input_count + 3,
                vec![
                    NativeOp::LoadVariable(input_count + 2),
                    NativeOp::LoadVariable(input_count),
                    NativeOp::Add,
                ],
            ),
        ];
        let mut variables = operands;
        variables.extend([99.0; 4]);
        let context = execute(&assignments, &mut variables);
        assert!(context.take_runtime_error().is_none(), "{op:?}");
        same_value(variables[input_count], expected);
        same_value(variables[input_count + 1], variables[0].sin());
        same_value(variables[input_count + 2], expected + 1.0);
        same_value(variables[input_count + 3], (expected + 1.0) + expected);
    }
}

#[test]
fn shared_math_keeps_guarded_checks_and_source_order_on_failure() {
    let exp = vec![
        NativeOp::LoadVariable(0),
        NativeOp::UnaryMath(UnaryMathOp::Exp),
    ];
    let guarded = vec![
        NativeOp::LoadVariable(1),
        NativeOp::LoadVariable(0),
        NativeOp::UnaryMath(UnaryMathOp::Log),
        NativeOp::Const(1.0),
        NativeOp::CheckedValue,
        NativeOp::LoadVariable(0),
        NativeOp::UnaryMath(UnaryMathOp::Exp),
        NativeOp::LoadVariable(0),
        NativeOp::UnaryMath(UnaryMathOp::Exp),
        NativeOp::Add,
        NativeOp::IfElse,
    ];
    let assignments = [
        assignment(2, exp.clone()),
        assignment(3, guarded),
        assignment(4, exp),
    ];
    for (bias, check) in [(0.5_f64, 1.0), (-0.5, 0.0), (-0.5, 1.0)] {
        let mut variables = [bias, check, 91.0, 92.0, 93.0];
        let context = execute(&assignments, &mut variables);
        same_value(variables[2], bias.exp());
        if bias < 0.0 && check != 0.0 {
            assert!(
                context
                    .take_runtime_error()
                    .unwrap()
                    .contains("ddx operand is not finite")
            );
            assert_eq!(variables[3..], [92.0, 93.0]);
        } else {
            assert!(context.take_runtime_error().is_none());
            same_value(
                variables[3],
                if check == 0.0 {
                    bias.exp() + bias.exp()
                } else {
                    1.0
                },
            );
            same_value(variables[4], bias.exp());
        }
    }
}

#[test]
fn helper_failures_abort_before_later_state_and_variable_writes() {
    for failed in [
        vec![
            NativeOp::Const(f64::NAN),
            NativeOp::Const(1.0),
            NativeOp::CheckedValue,
        ],
        vec![NativeOp::Const(f64::NAN), NativeOp::IntegerCast],
        vec![NativeOp::Const(1.0), NativeOp::TableLookup(0)],
        vec![NativeOp::Const(1.0), NativeOp::DdtState(0)],
    ] {
        let mut ops = failed;
        ops.extend([
            NativeOp::Const(3.0),
            NativeOp::StorePreludeSlot(0),
            NativeOp::Add,
        ]);
        let assignments = [
            assignment(0, vec![NativeOp::Const(5.0)]),
            assignment(1, ops),
            assignment(2, vec![NativeOp::Const(7.0)]),
        ];
        let mut variables = [91.0, 92.0, 93.0];
        let mut prelude = [94.0];
        let mut context = EvalContext::empty_for_test();
        context.prelude_slots = prelude.as_mut_ptr();
        context.prelude_slots_len = prelude.len();
        execute_with_context(&assignments, &mut variables, &context);
        assert!(context.take_runtime_error().is_some());
        assert_eq!(variables, [5.0, 92.0, 93.0]);
        assert_eq!(prelude, [94.0], "no state write may follow a failed helper");
    }
}
