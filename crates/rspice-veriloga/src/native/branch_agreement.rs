//! Agreement census between the shipped select lowering of a conditional and
//! the branch lowering the block model makes possible.
//!
//! Every value program of every shipped Verilog-A module is compiled twice:
//! once from the postfix lift the backends ship, and once after
//! [`Program::with_branching_conditionals`] re-expresses each `IfElse` as a
//! real diamond. Both go through the same register allocator, emitter and
//! independent x64 decoder, so the census answers two questions at corpus
//! scale: does the branch form always compile and verify, and does it produce
//! the same number when both arms are pure.
//!
//! Programs whose operands read storage this harness cannot size — analog
//! operator state, filter banks, noise — are compiled and verified but not
//! executed; the census reports how many it ran. The two lowerings are
//! *expected* to diverge in exactly one way: an operand that fails only
//! because the select form evaluates the arm it does not choose. Those are
//! counted rather than treated as disagreements, because avoiding them is the
//! point of the branch form.
//!
//! `#[ignore]`d: this is release-qualification work. Run it with
//! `--release --features native -- --ignored --nocapture`.

use super::census_models::shipped_census_models;
use crate::jit::expr::{NativeOp, NativeProgram};
use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
use crate::jit::ssa::Program;
use crate::native::abi::EvalContext;
use crate::native::assignment::NativeAssignment;
use crate::native::runtime::ExecutableMemory;
use crate::native::x64::codegen::{
    compile_value_function_artifact, compile_value_function_artifact_from_ssa,
};

/// Whether every operand of a program reads storage this harness sizes.
///
/// Analog operator state, filter banks, delay buffers and noise all index
/// runtime structures a bare context cannot stand up, so those programs are
/// compiled and verified but not run.
fn is_executable_here(program: &NativeProgram) -> bool {
    program.ops().iter().all(|op| {
        matches!(
            op,
            NativeOp::Const(_)
                | NativeOp::LoadParam(_)
                | NativeOp::LoadParamGiven(_)
                | NativeOp::LoadPortConnected(_)
                | NativeOp::LoadVoltage { .. }
                | NativeOp::LoadCurrent(_)
                | NativeOp::LoadPriorCurrent(_)
                | NativeOp::LoadInternalVoltage(_)
                | NativeOp::LoadVariable(_)
                | NativeOp::LoadVariableDyn { .. }
                | NativeOp::LoadBranchUnknown(_)
                | NativeOp::LoadTemperature
                | NativeOp::LoadThermalVoltage
                | NativeOp::LoadTime
                | NativeOp::LoadMfactor
                | NativeOp::Analysis(_)
                | NativeOp::Add
                | NativeOp::Sub
                | NativeOp::Mul
                | NativeOp::Div
                | NativeOp::AddConst(_)
                | NativeOp::SubConst(_)
                | NativeOp::MulConst(_)
                | NativeOp::DivConst(_)
                | NativeOp::SubFromConst(_)
                | NativeOp::DivFromConst(_)
                | NativeOp::Neg
                | NativeOp::Abs
                | NativeOp::Square
                | NativeOp::Sqrt
                | NativeOp::Compare(_)
                | NativeOp::CompareConst(..)
                | NativeOp::Logical(_)
                | NativeOp::LogicalConst(..)
                | NativeOp::IfElse
                | NativeOp::Extremum(_)
                | NativeOp::ExtremumConst(..)
                | NativeOp::ExtremumConstLhs(..)
                | NativeOp::UnaryMath(_)
                | NativeOp::BinaryMath(_)
                | NativeOp::IntegerCast
                | NativeOp::IntegerBinary(_)
                | NativeOp::IntegerShiftConst(..)
                | NativeOp::IntegerBinaryConst(..)
        )
    })
}

/// Storage a value entry may index, sized from the model it came from.
struct HostStorage {
    params: Vec<f64>,
    param_given: Vec<u8>,
    port_connected: Vec<u8>,
    voltages: Vec<f64>,
    internal_voltages: Vec<f64>,
    branch_unknowns: Vec<f64>,
    branch_currents: Vec<f64>,
    currents: Vec<f64>,
    variables: Vec<f64>,
    num_terminals: usize,
    fill: f64,
}

impl HostStorage {
    /// Build once per model. Building it per program is what turned this
    /// census into a memset benchmark: a large compact model has tens of
    /// thousands of variables, and the census walks hundreds of thousands of
    /// programs.
    fn for_model(model: &crate::codegen::CompiledModel, fill: f64) -> Self {
        // Slack past every declared count keeps a stray index inside the
        // allocation: an unchecked indexed load must not decide whether this
        // census process survives.
        const SLACK: usize = 64;
        let terminals = model.num_terminals + SLACK;
        Self {
            params: vec![fill; model.parameters.len() + SLACK],
            param_given: vec![1; model.parameters.len() + SLACK],
            port_connected: vec![1; terminals],
            voltages: vec![fill; terminals],
            internal_voltages: vec![fill; model.internal_nodes + SLACK],
            branch_unknowns: vec![fill; model.branch_sources.len() + SLACK],
            branch_currents: vec![fill; terminals * terminals],
            currents: vec![fill; model.stamp_programs.len() + SLACK],
            variables: vec![fill; model.num_variables + SLACK],
            num_terminals: model.num_terminals,
            fill,
        }
    }

    fn context(&self, analysis_type: u8) -> EvalContext {
        let mut context = EvalContext::empty_for_test();
        context.params = self.params.as_ptr();
        context.param_given = self.param_given.as_ptr();
        context.param_given_len = self.param_given.len();
        context.port_connected = self.port_connected.as_ptr();
        context.port_connected_len = self.port_connected.len();
        context.voltages = self.voltages.as_ptr();
        context.internal_voltages = self.internal_voltages.as_ptr();
        context.branch_unknowns = self.branch_unknowns.as_ptr();
        context.branch_currents = self.branch_currents.as_ptr();
        context.branch_currents_len = self.branch_currents.len();
        context.currents = self.currents.as_ptr();
        context.currents_len = self.currents.len();
        context.num_terminals = self.num_terminals;
        context.temperature = 300.15;
        context.time = 1.0e-9;
        context.timestep = 1.0e-12;
        context.multiplicity = 1.0;
        context.analysis_type = analysis_type;
        context
    }
}

#[derive(Default)]
struct Tally {
    models: usize,
    programs: usize,
    conditional_programs: usize,
    split_conditionals: usize,
    sunk_failing_operands: usize,
    executed: usize,
    executions: usize,
    select_only_failures: usize,
}

fn value_programs(plan: &crate::jit::model_plan::NativeModelPlan) -> Vec<&NativeProgram> {
    fn push_assignments<'a>(assignments: &'a [NativeAssignment], out: &mut Vec<&'a NativeProgram>) {
        for assignment in assignments {
            match assignment {
                NativeAssignment::Direct { program, .. } => out.push(program),
                NativeAssignment::Indexed { index, value, .. } => {
                    out.push(index);
                    out.push(value);
                }
                NativeAssignment::Loop { condition, body } => {
                    out.push(condition);
                    push_assignments(body, out);
                }
            }
        }
    }

    let mut programs = Vec::new();
    push_assignments(&plan.assignments, &mut programs);
    push_assignments(&plan.post_assignments, &mut programs);
    programs.extend(plan.parameter_defaults.iter().flatten());
    programs.extend(plan.static_conditions.iter().flatten());
    programs.extend(plan.stamp_values.iter());
    programs.extend(plan.jacobians.iter().flatten());
    programs.extend(plan.reactive_jacobians.iter().flatten());
    programs.extend(plan.noise_psd.iter());
    programs.extend(plan.noise_exponents.iter().flatten());
    programs
}

fn compare_program(
    name: &str,
    index: usize,
    program: &NativeProgram,
    storages: &[(HostStorage, u8)],
    tally: &mut Tally,
) {
    let select_ssa = Program::lower(program)
        .unwrap_or_else(|error| panic!("{name} program {index}: postfix lift: {error}"));
    let conditionals = select_ssa
        .instructions()
        .iter()
        .filter(|instruction| matches!(instruction.op(), NativeOp::IfElse))
        .count();
    let branch_ssa = select_ssa
        .with_branching_conditionals()
        .unwrap_or_else(|error| panic!("{name} program {index}: conditional split: {error}"));
    tally.programs += 1;
    if conditionals == 0 {
        assert!(
            branch_ssa.is_single_block(),
            "{name} program {index}: a program with no conditional gains no blocks"
        );
        return;
    }
    tally.conditional_programs += 1;
    tally.split_conditionals += conditionals;
    // The budget that rules out the classic blowup: re-expressing a
    // conditional as a diamond must not duplicate the continuation after it.
    // Every instruction is emitted exactly once and each conditional adds
    // exactly its three blocks, so both counts stay linear in the postfix
    // program however deeply the conditionals nest.
    assert_eq!(
        branch_ssa.blocks().len(),
        1 + 3 * conditionals,
        "{name} program {index}: every conditional lays out exactly one diamond"
    );
    assert_eq!(
        branch_ssa.instructions().len(),
        select_ssa.instructions().len() - conditionals,
        "{name} program {index}: the branch form moves instructions, it does not copy them"
    );
    assert!(
        branch_ssa.blocks().len() <= 1 + 3 * program.ops().len()
            && branch_ssa.instructions().len() <= program.ops().len(),
        "{name} program {index}: block program exceeds its postfix size budget"
    );
    // Operands that only the taken arm reads, and that can fail: these are the
    // evaluations the select form cannot avoid.
    for block in branch_ssa.blocks() {
        if matches!(block.terminator(), crate::jit::ssa::Terminator::Return(_)) {
            continue;
        }
        if block.id().index() == 0 {
            continue;
        }
        tally.sunk_failing_operands += branch_ssa.instructions()
            [block.instruction_start()..block.instruction_end()]
            .iter()
            .filter(|instruction| instruction.effects().may_fail())
            .count();
    }

    let select = compile_value_function_artifact(program)
        .unwrap_or_else(|error| panic!("{name} program {index}: select codegen: {error}"));
    let branch = compile_value_function_artifact_from_ssa(&branch_ssa)
        .unwrap_or_else(|error| panic!("{name} program {index}: branch codegen: {error}"));

    if !is_executable_here(program) {
        return;
    }
    tally.executed += 1;
    let select_memory = ExecutableMemory::allocate(select.bytes())
        .unwrap_or_else(|error| panic!("{name} program {index}: publish select: {error}"));
    let branch_memory = ExecutableMemory::allocate(branch.bytes())
        .unwrap_or_else(|error| panic!("{name} program {index}: publish branch: {error}"));
    let entry =
        |memory: &ExecutableMemory| -> extern "C" fn(*const EvalContext, *const f64) -> f64 {
            let pointer = memory.ptr_at(0).expect("entry inside published image");
            unsafe { std::mem::transmute(pointer) }
        };
    let select_entry = entry(&select_memory);
    let branch_entry = entry(&branch_memory);

    for (storage, analysis) in storages {
        let fill = storage.fill;
        let context = storage.context(*analysis);
        context.clear_runtime_error();
        let expected = select_entry(&context, storage.variables.as_ptr());
        let select_error = context.take_runtime_error();
        context.clear_runtime_error();
        let actual = branch_entry(&context, storage.variables.as_ptr());
        let branch_error = context.take_runtime_error();
        tally.executions += 1;
        match (select_error, branch_error) {
            (None, None) => assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{name} program {index}: branch and select disagree at fill={fill}"
            ),
            (Some(_), None) => tally.select_only_failures += 1,
            (Some(select_error), Some(branch_error)) => assert_eq!(
                select_error, branch_error,
                "{name} program {index}: the two lowerings fail differently at fill={fill}"
            ),
            (None, Some(branch_error)) => panic!(
                "{name} program {index}: the branch form fails where the select form does not at fill={fill}: {branch_error}"
            ),
        }
    }
}

#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn branch_lowering_agrees_with_the_select_form_across_the_shipped_census() {
    let mut tally = Tally::default();
    let mut total_compile_seconds = 0.0_f64;
    let mut total_census_seconds = 0.0_f64;
    for shipped in shipped_census_models() {
        let module = &shipped.name;
        let census_started = std::time::Instant::now();
        let plan = build_model_plan_with_canonical_ir(&shipped.model, &shipped.canonical_ir)
            .unwrap_or_else(|error| panic!("{module}: native plan: {error}"));
        tally.models += 1;
        let before = tally.programs;
        let started = std::time::Instant::now();
        let storages = [
            (HostStorage::for_model(&shipped.model, 0.0), 0_u8),
            (HostStorage::for_model(&shipped.model, 0.7), 0),
            (HostStorage::for_model(&shipped.model, -0.4), 2),
        ];
        let programs = value_programs(&plan);
        let ops = programs
            .iter()
            .map(|program| program.ops().len())
            .sum::<usize>();
        for (index, program) in programs.into_iter().enumerate() {
            compare_program(module, index, program, &storages, &mut tally);
        }
        eprintln!(
            "branch-agreement model={module} programs={} ops={ops} seconds={:.1}",
            tally.programs - before,
            started.elapsed().as_secs_f64()
        );
        let census_seconds = census_started.elapsed().as_secs_f64();
        eprintln!(
            "branch-agreement model={module} compile_seconds={:.1} census_seconds={census_seconds:.1} cached={}",
            shipped.compile_seconds, shipped.from_cache
        );
        total_compile_seconds += shipped.compile_seconds;
        total_census_seconds += census_seconds;
    }
    eprintln!(
        "branch-agreement total_compile_seconds={total_compile_seconds:.1} total_census_seconds={total_census_seconds:.1}"
    );

    eprintln!(
        "branch-agreement models={} programs={} conditional_programs={} split_conditionals={} sunk_failing_operands={} executed_programs={} executions={} select_only_failures={}",
        tally.models,
        tally.programs,
        tally.conditional_programs,
        tally.split_conditionals,
        tally.sunk_failing_operands,
        tally.executed,
        tally.executions,
        tally.select_only_failures,
    );
    assert_eq!(tally.models, 43, "the shipped census is 43 modules");
    assert!(
        tally.split_conditionals > 0 && tally.executions > 0,
        "the census must actually exercise the branch form"
    );
}
