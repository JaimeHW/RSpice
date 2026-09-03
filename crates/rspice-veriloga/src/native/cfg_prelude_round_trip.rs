//! Does a value survive the prelude?
//!
//! The prelude's whole claim is that an entry which used to *be* its dependence
//! cone can become a one-instruction read of a slot without changing what it
//! evaluates to. That is a claim about bits, not about tolerance: the prelude
//! computes the same SSA values the entry's cone computes, in the same order,
//! through the same emitter, so the two must agree exactly. A tolerance here
//! would hide the one defect class that matters — a slot published from the
//! wrong value, or read from the wrong index, which lands within a tolerance
//! far more often than it lands outside one.
//!
//! So this compiles both forms of every entry of one small module, runs them
//! against the same evaluation context, and compares `to_bits`.
//!
//! It also pins the two things that make the slot ABI safe: the storage
//! validator refuses an array the plan would overrun, and the prelude
//! addresses exactly the analog state slots its entries do — no more, which
//! would mean the union prune dragged in a record no entry advances, and no
//! fewer, which would mean an entry lost one.

use std::collections::BTreeSet;

use super::cfg_prelude_census::prelude_inputs;
use super::model::NativeRequiredStorage;
use super::runtime::ExecutableMemory;
use super::{EvalContext, x64};
use crate::canonical_ir::prune_cfg_to_outputs;
use crate::jit::cfg_prelude::CfgPrelude;
use crate::jit::cfg_program::lower_cfg_function;
use crate::jit::expr::NativeOp;
use crate::jit::ssa::Program;
use crate::{CompilerOptions, VerilogACompiler};

/// A module with the three shapes the prelude has to survive: a conditional,
/// so a published value is a block parameter of a merge; a `ddt`, so the
/// program owns an analog state record whose slot the adoption check consults;
/// and two contributions, so entries come from more than one residual.
const SOURCE: &str = r#"
`include "disciplines.vams"
module prelude_round_trip(p, n, m);
  inout p, n, m;
  electrical p, n, m;
  parameter real r = 2.0;
  parameter real c = 1.0e-9;
  real vpn, vmn, g;
  analog begin
    vpn = V(p, n);
    vmn = V(m, n);
    if (vpn > 0.5)
      g = vpn * vpn / r;
    else
      g = vpn / r;
    I(p, n) <+ g + ddt(c * vpn);
    I(m, n) <+ vmn / r + g;
  end
endmodule
"#;

/// Every analog state slot a program addresses, in the vocabulary the plan's
/// adoption check uses.
fn state_slots(program: &Program) -> BTreeSet<(usize, usize)> {
    program
        .instructions()
        .iter()
        .filter_map(|instruction| match instruction.op() {
            NativeOp::DdtState(slot) => Some((0, slot)),
            NativeOp::IdtState(slot) | NativeOp::IdtModState(slot) => Some((1, slot)),
            NativeOp::LimitState(slot) => Some((2, slot)),
            NativeOp::TransitionState(slot) | NativeOp::TransitionStateDerivative(slot) => {
                Some((3, slot))
            }
            NativeOp::SlewState(slot) | NativeOp::SlewStateDerivative(slot) => Some((4, slot)),
            NativeOp::CrossState(slot) | NativeOp::LastCrossingState(slot) => Some((5, slot)),
            NativeOp::AboveState(slot) => Some((6, slot)),
            _ => None,
        })
        .collect()
}

/// The caller-owned buffers one evaluation of the fixture needs.
struct Storage {
    params: Vec<f64>,
    voltages: Vec<f64>,
    currents: Vec<f64>,
    branch_currents: Vec<f64>,
    variables: Vec<f64>,
    states: Vec<f64>,
    states_prev: Vec<f64>,
    states_older: Vec<f64>,
    states_older_candidate: Vec<f64>,
    derivatives: Vec<f64>,
    derivatives_prev: Vec<f64>,
    initialized: Vec<u8>,
    candidate_valid: Vec<u8>,
    prelude_slots: Vec<f64>,
}

impl Storage {
    fn new(model: &crate::codegen::CompiledModel, voltage: f64, slot_count: usize) -> Self {
        let required = NativeRequiredStorage::for_model(model);
        Self {
            params: model.parameters.iter().map(|_| 2.0).collect::<Vec<f64>>(),
            voltages: vec![voltage, 0.0, voltage * 0.5],
            currents: vec![0.0; model.stamp_programs.len()],
            branch_currents: vec![0.0; (model.num_terminals + 1) * (model.num_terminals + 1)],
            variables: vec![0.0; model.num_variables],
            states: vec![0.0; required.state_values],
            states_prev: vec![0.0; required.state_values_prev],
            states_older: vec![0.0; required.state_values],
            states_older_candidate: vec![0.0; required.state_older_candidate],
            derivatives: vec![0.0; required.state_values],
            derivatives_prev: vec![0.0; required.state_values],
            initialized: vec![0; required.state_initialized],
            candidate_valid: vec![0; required.state_candidate_valid],
            prelude_slots: vec![f64::NAN; slot_count],
        }
    }

    fn context(&mut self, terminals: usize) -> EvalContext {
        let mut context = EvalContext::empty_for_test();
        context.params = self.params.as_ptr();
        context.voltages = self.voltages.as_ptr();
        context.num_terminals = terminals;
        context.currents = self.currents.as_ptr();
        context.currents_len = self.currents.len();
        context.branch_currents = self.branch_currents.as_ptr();
        context.branch_currents_len = self.branch_currents.len();
        context.state_values = self.states.as_mut_ptr();
        context.state_values_len = self.states.len();
        context.state_prev = self.states_prev.as_ptr();
        context.state_prev_len = self.states_prev.len();
        context.state_older = self.states_older.as_ptr();
        context.state_older_len = self.states_older.len();
        context.state_older_candidate = self.states_older_candidate.as_mut_ptr();
        context.state_older_candidate_len = self.states_older_candidate.len();
        context.state_derivatives = self.derivatives.as_mut_ptr();
        context.state_derivatives_len = self.derivatives.len();
        context.state_derivatives_prev = self.derivatives_prev.as_ptr();
        context.state_derivatives_prev_len = self.derivatives_prev.len();
        context.state_initialized = self.initialized.as_mut_ptr();
        context.state_initialized_len = self.initialized.len();
        context.state_candidate_valid = self.candidate_valid.as_mut_ptr();
        context.state_candidate_valid_len = self.candidate_valid.len();
        context.prelude_slots = self.prelude_slots.as_mut_ptr();
        context.prelude_slots_len = self.prelude_slots.len();
        context
    }
}

/// Publish one block program and hand back its entry point.
fn publish(program: &Program, what: &str) -> (ExecutableMemory, usize) {
    let artifact = x64::codegen::compile_value_function_artifact_from_ssa(program)
        .unwrap_or_else(|error| panic!("{what}: x64 codegen: {error}"));
    let bytes = artifact.bytes().len();
    let memory = ExecutableMemory::allocate(artifact.bytes())
        .unwrap_or_else(|error| panic!("{what}: publish: {error}"));
    (memory, bytes)
}

fn call(memory: &ExecutableMemory, context: &EvalContext, variables: &[f64]) -> f64 {
    let pointer = memory.ptr_at(0).expect("entry inside published image");
    let entry: extern "C" fn(*const EvalContext, *const f64) -> f64 =
        unsafe { std::mem::transmute(pointer) };
    entry(context, variables.as_ptr())
}

#[test]
fn a_prelude_slot_read_agrees_with_the_entry_cone_bit_for_bit() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(SOURCE).expect("compile the fixture");
    let artifact = compiler
        .compile_canonical_ir(SOURCE)
        .expect("compile the fixture's canonical IR");
    let inputs = prelude_inputs("prelude_round_trip", &model, &artifact)
        .expect("the CFG route reaches the fixture's entries");
    assert!(
        inputs.entries.len() >= 4,
        "the fixture must have several entries or it proves nothing, found {}",
        inputs.entries.len()
    );

    let prelude = CfgPrelude::build(
        "prelude_round_trip",
        &inputs.function,
        &inputs.entries,
        &inputs.state,
        &inputs.bindings,
        &inputs.slots,
    )
    .expect("build the fixture's prelude");
    assert_eq!(
        prelude.slot_count(),
        inputs
            .entries
            .iter()
            .map(|(_, value)| *value)
            .collect::<BTreeSet<_>>()
            .len(),
        "one slot per distinct entry output, and no more",
    );

    // ---- what the shipped CFG route builds for each entry ----------------
    let mut cones = Vec::new();
    let mut cone_state_slots = BTreeSet::new();
    for (entry, value) in &inputs.entries {
        let (pruned, outputs) = prune_cfg_to_outputs(&inputs.function, &[*value]);
        let program = lower_cfg_function(&pruned, outputs[0], &inputs.state, &inputs.bindings)
            .unwrap_or_else(|error| panic!("{entry}: cone lowering: {error}"));
        cone_state_slots.extend(state_slots(&program));
        cones.push((*entry, program));
    }

    // ---- the prelude's state vocabulary is exactly the entries' ----------
    //
    // `BlockProgram::adopt` already refused a slot the module never allocated,
    // which is the other half of this: that check says every slot is claimed,
    // and this says the set is the same one the entries reach.
    assert!(
        !cone_state_slots.is_empty(),
        "the fixture must own at least one analog state record",
    );
    assert_eq!(
        state_slots(prelude.program().ssa()),
        cone_state_slots,
        "the prelude must advance exactly the state records its entries do",
    );

    // ---- run both forms against one context ------------------------------
    let (prelude_memory, prelude_bytes) = publish(prelude.program().ssa(), "prelude");
    let reads: Vec<(crate::jit::cfg_plan_builder::CfgPlanEntry, ExecutableMemory)> = inputs
        .entries
        .iter()
        .map(|(entry, _)| {
            let program = prelude
                .entry_program(*entry)
                .unwrap_or_else(|| panic!("{entry} has no prelude slot"))
                .unwrap_or_else(|error| panic!("{entry}: slot read lowering: {error}"));
            assert_eq!(
                program.instructions().len(),
                1,
                "{entry}'s program after the prelude must be one load",
            );
            let (memory, _) = publish(&program, "slot read");
            (*entry, memory)
        })
        .collect();
    let cone_images: Vec<(
        crate::jit::cfg_plan_builder::CfgPlanEntry,
        ExecutableMemory,
        usize,
    )> = cones
        .iter()
        .map(|(entry, program)| {
            let (memory, bytes) = publish(program, "entry cone");
            (*entry, memory, bytes)
        })
        .collect();
    let cone_image_bytes: usize = cone_images.iter().map(|(_, _, bytes)| bytes).sum();
    assert!(
        prelude_bytes < cone_image_bytes,
        "the prelude ({prelude_bytes} B) must be smaller than the cones it replaces \
         ({cone_image_bytes} B)",
    );

    for voltage in [0.25_f64, 0.75, -1.5] {
        let mut storage = Storage::new(&model, voltage, prelude.slot_count());
        let context = storage.context(model.num_terminals);
        context.clear_runtime_error();
        let sentinel = call(&prelude_memory, &context, &storage.variables);
        assert_eq!(sentinel, 0.0, "the prelude returns its constant");
        let prelude_error = context.take_runtime_error();
        assert!(
            prelude_error.is_none(),
            "the prelude must not fail at V={voltage}: {prelude_error:?}",
        );
        assert!(
            storage.prelude_slots.iter().all(|slot| !slot.is_nan()),
            "every slot must be published at V={voltage}",
        );

        for ((entry, cone), (read_entry, read)) in cone_images
            .iter()
            .map(|(entry, memory, _)| (entry, memory))
            .zip(&reads)
        {
            assert_eq!(entry, read_entry, "the two lists stay in step");
            context.clear_runtime_error();
            let expected = call(cone, &context, &storage.variables);
            let cone_error = context.take_runtime_error();
            context.clear_runtime_error();
            let actual = call(read, &context, &storage.variables);
            assert!(
                context.take_runtime_error().is_none(),
                "{entry}: a slot read cannot fail",
            );
            assert!(
                cone_error.is_none(),
                "{entry}: the cone failed at V={voltage}: {cone_error:?}",
            );
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{entry}: the slot read and the cone disagree at V={voltage} \
                 ({actual} against {expected})",
            );
        }
    }
}

#[test]
fn a_short_prelude_slot_array_is_refused() {
    let required = NativeRequiredStorage::default().with_prelude_slots(12);
    let error = required
        .validate_prelude_slot_storage(11)
        .expect_err("eleven slots cannot hold twelve publications");
    let message = error.to_string();
    assert!(
        message.contains("11") && message.contains("12"),
        "the refusal must name both counts, got {message}",
    );
    required
        .validate_prelude_slot_storage(12)
        .expect("exactly the required count is enough");
    required
        .validate_prelude_slot_storage(64)
        .expect("a larger array is a floor, not an equality");
    NativeRequiredStorage::default()
        .validate_prelude_slot_storage(0)
        .expect("a plan with no prelude requires no slots");
}
