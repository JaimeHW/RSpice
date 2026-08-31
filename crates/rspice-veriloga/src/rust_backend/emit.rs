//! Emitting Rust from a [`CfgFunction`].
//!
//! The blocks go back to being `if`/`else` and `loop`. That is not a
//! concession — it is the point. A `match` on a block index would be correct
//! and would defeat the whole exercise: the reason control flow survived to
//! here is so the generated code can skip work, and a dispatch loop asks the
//! optimiser to rediscover a shape this pass already knows.
//!
//! ## Only the shapes the pipeline produces
//!
//! Every graph reaching this emitter came from [`super::super::canonical_ir::cfg_lower`],
//! which builds diamonds for conditionals and a header/body/exit triple for
//! loops; the derivative and simplification passes add values but never edges.
//! So the emitter recognises exactly those two shapes and reports anything else
//! rather than falling back to a dispatch loop. An unstructured graph here would
//! mean an earlier pass had rewritten control flow, which is worth a loud
//! failure rather than slow code.
//!
//! ## Merges become bindings
//!
//! A one-value straight-line diamond is an `if` expression. Multi-value or
//! nested diamonds retain declaration plus edge assignment, and loop-carried
//! parameters remain mutable bindings. Rust's definite-assignment and type
//! analyses therefore continue to check the properties SSA maintained by
//! construction without making the common one-value merge three statements.
//!
//! ## Predicates stay predicates
//!
//! Boolean CFG values emit as Rust `bool`. Numeric `0.0`/`1.0` conversions are
//! written only where a predicate enters arithmetic or a scalar cache; real
//! values entering control flow are tested against zero. Keeping that boundary
//! explicit preserves the semantics used by
//! [`super::super::canonical_ir::cfg_eval`] without making every comparison an
//! `if` expression that Rust immediately has to turn back into a predicate.
//!
//! ## Derivatives are scalar-or-packed, and that is where the size went
//!
//! A one-lane derivative emits as plain `f64`; widths two through thirty-two
//! use fixed loop-free runtime newtypes (`L2` through `L32`). Larger future
//! shapes retain the shared runtime's const-generic `Lanes<N>` fallback.
//! The elementwise rules emit as `a + b` and `a * s` — one line each, whatever
//! `N` is. That is the whole reason the IR packs: fully scalarised derivatives
//! cost a line per lane, and the wide MOSFETs carry a hundred thousand of them.
//! The one-lane exception removes an otherwise pointless array wrapper without
//! multiplying any expression. The newtype exists so wider lines are operators
//! rather than calls, which is worth another third of the bytes; `rustc`
//! promotes the small fixed array inside it to registers, so none of this is a
//! loop at run time.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write;

use crate::canonical_ir::cfg::{
    CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind, CfgValueType,
};
use crate::canonical_ir::{BlockId, ValueId};

/// What the emitted body expects to find in scope.
///
/// Held as names rather than baked in so the same emitter serves the generated
/// device, a standalone test program, and whatever the runtime path ends up
/// wanting, without a second copy of the value rules.
#[derive(Debug, Clone)]
pub struct EmitBindings {
    pub parameters: String,
    pub parameter_given: String,
    pub event_state: String,
    pub node_potentials: String,
    pub branch_flows: String,
    pub branch_unknown_flows: String,
    pub temperature: String,
    pub thermal_voltage: String,
    pub multiplicity: String,
    pub time: String,
    pub ddt: String,
    /// Dense generated state slots keyed by source operator id.
    pub ddt_slots: HashMap<crate::canonical_ir::ExprId, usize>,
    pub ddt_scale: String,
    pub idt: String,
    pub idt_slots: HashMap<crate::canonical_ir::ExprId, usize>,
    pub idt_scale: String,
    /// Stateful generated event-control evaluators.
    pub cross: String,
    pub cross_slots: HashMap<crate::canonical_ir::ExprId, usize>,
    pub above: String,
    pub timer: String,
    pub timer_slots: HashMap<crate::canonical_ir::ExprId, usize>,
    /// Called as `analysis("dc")`.
    pub analysis: String,
    /// Called as `simparam("gmin", fallback)`.
    pub simparam: String,
    /// Called as `limit(operator, proposed, candidate)`.
    pub limit: String,
    pub limit_slots: HashMap<crate::canonical_ir::ExprId, usize>,
    /// Called as `limit_previous(operator, proposed)`.
    pub limit_previous: String,
    /// Indexed as `staged[slot]` — what coarser invalidation stages cached.
    pub staged: String,
}

impl Default for EmitBindings {
    fn default() -> Self {
        Self {
            parameters: "parameters".into(),
            parameter_given: "parameter_given".into(),
            event_state: "event_state".into(),
            node_potentials: "node_potentials".into(),
            branch_flows: "branch_flows".into(),
            branch_unknown_flows: "branch_unknown_flows".into(),
            temperature: "temperature".into(),
            thermal_voltage: "thermal_voltage".into(),
            multiplicity: "multiplicity".into(),
            time: "time".into(),
            ddt: "ddt".into(),
            ddt_slots: HashMap::new(),
            ddt_scale: "ddt_scale".into(),
            idt: "idt".into(),
            idt_slots: HashMap::new(),
            idt_scale: "idt_scale".into(),
            cross: "cross".into(),
            cross_slots: HashMap::new(),
            above: "above".into(),
            timer: "timer".into(),
            timer_slots: HashMap::new(),
            analysis: "analysis".into(),
            simparam: "simparam".into(),
            limit: "limit".into(),
            limit_slots: HashMap::new(),
            limit_previous: "limit_previous".into(),
            staged: "staged".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmitError {
    /// A shape the structured emitter does not recognise. Always an earlier
    /// pass's doing, never a model's.
    UnstructuredControlFlow(BlockId),
    /// A `ddx` that the derivative pass should have resolved.
    UnresolvedDdx(ValueId),
    /// A discrete-domain value reached the analog emitter.
    ///
    /// This emitter writes `f64` arithmetic against the solver's ABI. A
    /// four-state value has no `f64` that means the same thing, so there is
    /// nothing to emit and emitting anything would be a wrong number rather
    /// than a missing feature. `ModelPlan::build` refuses a module carrying a
    /// process before it reaches here; this is the backstop.
    DigitalValueInAnalogEmitter(ValueId),
    /// Stateful transition runtime is provided by the VM/native/WASM paths;
    /// direct generated Rust must reject it before emission.
    UnsupportedTransition(ValueId),
}

impl std::fmt::Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnstructuredControlFlow(block) => {
                write!(
                    f,
                    "{block} is not a diamond or a loop the emitter can reform"
                )
            }
            Self::UnresolvedDdx(value) => {
                write!(f, "{value} is a ddx the derivative pass did not resolve")
            }
            Self::DigitalValueInAnalogEmitter(value) => write!(
                f,
                "{value} is a discrete-domain value and has no analog form to emit"
            ),
            Self::UnsupportedTransition(value) => write!(
                f,
                "{value} is a stateful transition unsupported by the direct generated-Rust runtime"
            ),
        }
    }
}

impl std::error::Error for EmitError {}

/// Emit the body of a function computing `outputs`.
///
/// The returned source is statements, not a complete item: the caller supplies
/// the signature and whatever the bindings name. `outputs` come back as the
/// expressions that read them.
pub fn emit_body(
    function: &CfgFunction,
    outputs: &[ValueId],
    bindings: &EmitBindings,
) -> Result<(String, Vec<String>), EmitError> {
    let mut emitter = Emitter {
        function,
        bindings,
        source: String::new(),
        declared: HashSet::new(),
        loop_headers: back_edge_targets(function),
        post_dominators: immediate_post_dominators(function),
        predecessor_counts: predecessor_counts(function),
        emitted: vec![false; function.values.len()],
        effectful_branches: HashSet::new(),
        inlined: HashMap::new(),
        names: Vec::new(),
        wanted: outputs.iter().copied().collect(),
        captured: BTreeMap::new(),
    };
    emitter.plan_emission_liveness(outputs);
    emitter.plan_inlining(outputs);
    emitter.plan_names();
    emitter.leaves()?;
    // Captures are declared between the leaves and the body: they have to be in
    // scope for the whole body, and which of them are needed is only known once
    // the body has been walked, so the text is spliced in afterwards.
    let splice = emitter.source.len();
    emitter.block(function.entry, None, 1)?;
    let declarations = emitter.capture_declarations();
    emitter.source.insert_str(splice, &declarations);
    let names = outputs
        .iter()
        .map(|value| emitter.output_name(*value))
        .collect();
    Ok((emitter.source, names))
}

pub const MAX_FIXED_LANE_WIDTH: usize = 32;

/// Runtime constructor/type name for a packed derivative width.
pub(super) fn lane_type_name(width: usize) -> String {
    if (2..=MAX_FIXED_LANE_WIDTH).contains(&width) {
        format!("L{width}")
    } else {
        "Lanes".to_string()
    }
}

/// Exact packed-lane runtime imports needed by one emitted CFG function.
pub(super) fn lane_runtime_types(function: &CfgFunction) -> BTreeSet<String> {
    function
        .values
        .iter()
        .filter_map(|value| function.lanes_of(value.id))
        .map(<[u32]>::len)
        .filter(|width| *width > 1)
        .map(lane_type_name)
        .collect()
}

/// Standalone copy of the helpers emitted arithmetic calls.
///
/// Generated model crates import the identical definitions from
/// `rspice-veriloga-runtime`; this copy keeps direct-rustc emitter and benchmark
/// programs self-contained.
pub const RUNTIME_PRELUDE: &str = r#"
#[inline(always)]
fn rspice_limexp(x: f64) -> f64 {
    if x < 80.0 { x.exp() } else { (80.0f64).exp() * (x - 80.0 + 1.0) }
}

#[inline(always)]
fn rspice_limited_exp(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34 * (x - 80.0 + 1.0)
    } else if x < -80.0 {
        1.804851387e-35
    } else {
        x.exp()
    }
}

#[inline(always)]
fn rspice_limited_exp_derivative(x: f64) -> f64 {
    if x > 80.0 {
        5.54062238439351e34
    } else if x < -80.0 {
        0.0
    } else {
        x.exp()
    }
}

/// A packed derivative: one partial per unknown the value can reach.
///
/// A newtype rather than a bare `[f64; N]` so the elementwise rules emit as
/// `a + b` and `a * s` instead of named calls. That is not cosmetic — these
/// operations are most of a large model's generated source, and an operator is
/// a dozen characters shorter than a call at every one of them.
#[repr(transparent)]
#[derive(Clone, Copy)]
struct Lanes<const N: usize>([f64; N]);

macro_rules! generic_lane_operator {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl<const N: usize> core::ops::$trait for Lanes<N> {
            type Output = Self;
            #[inline(always)]
            fn $method(self, rhs: Self) -> Self {
                let mut out = self.0;
                let mut i = 0;
                while i < N {
                    out[i] = self.0[i] $operator rhs.0[i];
                    i += 1;
                }
                Self(out)
            }
        }
    };
}
generic_lane_operator!(Add, add, +);
generic_lane_operator!(Sub, sub, -);

impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] * rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] / rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: usize) -> &f64 { &self.0[index] }
}

macro_rules! define_fixed_lanes {
    ($name:ident, $width:literal, [$($index:tt),+ $(,)?]) => {
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        struct $name([f64; $width]);

        impl core::ops::Add for $name {
            type Output = Self;
            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self([$((self.0[$index] + rhs.0[$index])),+])
            }
        }
        impl core::ops::Sub for $name {
            type Output = Self;
            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self([$((self.0[$index] - rhs.0[$index])),+])
            }
        }
        impl core::ops::Mul<f64> for $name {
            type Output = Self;
            #[inline(always)]
            fn mul(self, rhs: f64) -> Self {
                Self([$((self.0[$index] * rhs)),+])
            }
        }
        impl core::ops::Div<f64> for $name {
            type Output = Self;
            #[inline(always)]
            fn div(self, rhs: f64) -> Self {
                Self([$((self.0[$index] / rhs)),+])
            }
        }
        impl core::ops::Index<usize> for $name {
            type Output = f64;
            #[inline(always)]
            fn index(&self, index: usize) -> &f64 { &self.0[index] }
        }
    };
}

define_fixed_lanes!(L2, 2, [0, 1]);
define_fixed_lanes!(L3, 3, [0, 1, 2]);
define_fixed_lanes!(L4, 4, [0, 1, 2, 3]);
define_fixed_lanes!(L5, 5, [0, 1, 2, 3, 4]);
define_fixed_lanes!(L6, 6, [0, 1, 2, 3, 4, 5]);
define_fixed_lanes!(L7, 7, [0, 1, 2, 3, 4, 5, 6]);
define_fixed_lanes!(L8, 8, [0, 1, 2, 3, 4, 5, 6, 7]);
define_fixed_lanes!(L9, 9, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
define_fixed_lanes!(L10, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
define_fixed_lanes!(L11, 11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
define_fixed_lanes!(L12, 12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
define_fixed_lanes!(L13, 13, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
define_fixed_lanes!(L14, 14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
define_fixed_lanes!(L15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
define_fixed_lanes!(L16, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
define_fixed_lanes!(L17, 17, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
define_fixed_lanes!(L18, 18, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
define_fixed_lanes!(L19, 19, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
define_fixed_lanes!(L20, 20, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
define_fixed_lanes!(L21, 21, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
define_fixed_lanes!(L22, 22, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]);
define_fixed_lanes!(L23, 23, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]);
define_fixed_lanes!(L24, 24, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
define_fixed_lanes!(L25, 25, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]);
define_fixed_lanes!(L26, 26, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]);
define_fixed_lanes!(L27, 27, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]);
define_fixed_lanes!(L28, 28, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]);
define_fixed_lanes!(L29, 29, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]);
define_fixed_lanes!(L30, 30, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);
define_fixed_lanes!(L31, 31, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
define_fixed_lanes!(L32, 32, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
"#;

struct Emitter<'a> {
    function: &'a CfgFunction,
    bindings: &'a EmitBindings,
    source: String,
    declared: HashSet<ValueId>,
    loop_headers: HashSet<BlockId>,
    /// Immediate post-dominators, which is what a diamond's join is.
    post_dominators: Vec<Option<BlockId>>,
    /// Incoming edge count for each block. Expression-form arms must be owned
    /// by exactly one branch so emitting them lexically cannot duplicate work.
    predecessor_counts: Vec<usize>,
    /// Values that can affect a requested output or necessary control flow.
    ///
    /// Ordinary CFG DCE keeps every branch condition because the CFG still
    /// names it. Slicing a model (especially stamp preprocessing shared with
    /// noise) can leave whole nested diamonds that compute no requested value.
    /// Keeping a second, emission-specific liveness set lets the Rust backend
    /// omit those pure conditions and their operands without mutating the
    /// validated canonical CFG.
    emitted: Vec<bool>,
    /// Branches whose choice can affect an emitted value, plus loop control.
    effectful_branches: HashSet<BlockId>,
    /// Values emitted at their use rather than bound to a name.
    ///
    /// A compact model is mostly values read exactly once, and binding each of
    /// them costs a line of generated Rust that says nothing. Substituting them
    /// into their one consumer is what the tier this replaces was doing to stay
    /// small, and without it the emitted source is several times larger for no
    /// difference in the compiled code.
    inlined: HashMap<ValueId, String>,
    /// Dense local names for values that survive expression inlining.
    ///
    /// Scheduling keeps stable CFG ids while cutting a function into stages,
    /// leaving large gaps in the ids one emitted Rust body actually names.
    /// Local ordinals preserve every scope and dependency while avoiding those
    /// gaps in source text and in rustc's identifier table.
    names: Vec<String>,
    /// The values the caller asked for.
    wanted: HashSet<ValueId>,
    /// Wanted values that a guarded or looping region defines, and the name
    /// each is copied out into.
    ///
    /// Rust's scoping is lexical and the emitted control flow is real, so a
    /// value bound inside an `if` arm has no name after it. Every other value
    /// in the graph is fine — an operand is read where it is defined or arrives
    /// through a block parameter, which is declared outside the construct —
    /// but a value the *caller* reads has no such path. A stage export is
    /// exactly that: a coarse stage computes something inside a guard and a
    /// finer stage reads it back through a slot.
    ///
    /// The copy starts at zero, which is what a reader sees if the region never
    /// runs. That is unobservable rather than merely convenient: the reader is
    /// control-dependent on the same guard, so its own stage carries a copy of
    /// that guard and only reads the slot on the path that wrote it. A reader
    /// that were *not* control-dependent on it would have been raised into this
    /// value's class by the scheduler and would not be reading a slot at all.
    captured: BTreeMap<ValueId, String>,
}

/// Keep emitted expressions shallow enough for rustc's parser, type checker,
/// and MIR builder. A temporary is SSA, so stopping here changes source shape
/// rather than generated runtime storage.
const MAX_INLINED_EXPRESSION_NODES: usize = 32;

impl Emitter<'_> {
    /// Find values and control decisions that can affect `outputs`.
    ///
    /// This deliberately starts without branch conditions. A branch becomes
    /// live only when one of its arms carries or computes an already-live
    /// value before the join. Newly-live conditions can themselves flow
    /// through outer diamonds, so the scan iterates to a fixed point. Loops
    /// stay conservative: even an output-free loop controls termination.
    fn plan_emission_liveness(&mut self, outputs: &[ValueId]) {
        let incoming = incoming_block_arguments(self.function);
        let mut worklist = Vec::new();
        for output in outputs {
            mark_live(*output, &mut self.emitted, &mut worklist);
        }
        propagate_value_liveness(self.function, &incoming, &mut self.emitted, &mut worklist);

        loop {
            let mut changed = false;
            for block in &self.function.blocks {
                let CfgTerminator::Branch {
                    condition,
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                } = &block.terminator
                else {
                    continue;
                };
                if self.effectful_branches.contains(&block.id) {
                    continue;
                }
                let effectful = if self.loop_headers.contains(&block.id) {
                    true
                } else if let Some(join) = self.join_of(block.id) {
                    self.region_has_live_effect(*then_target, then_args, join)
                        || self.region_has_live_effect(*else_target, else_args, join)
                } else {
                    // A branch without a post-dominating join may return or
                    // loop on only one path. Preserve that control behavior.
                    true
                };
                if effectful {
                    self.effectful_branches.insert(block.id);
                    mark_live(*condition, &mut self.emitted, &mut worklist);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
            propagate_value_liveness(self.function, &incoming, &mut self.emitted, &mut worklist);
        }
    }

    /// Whether `start..join` carries or computes an emitted value.
    fn region_has_live_effect(&self, start: BlockId, edge_args: &[ValueId], join: BlockId) -> bool {
        if self.edge_carries_live_value(start, edge_args) {
            return true;
        }
        let mut seen = HashSet::new();
        let mut stack = vec![start];
        while let Some(current) = stack.pop() {
            if current == join || !seen.insert(current) {
                continue;
            }
            if self.loop_headers.contains(&current) {
                return true;
            }
            let block = self.function.block(current);
            if block
                .instructions
                .iter()
                .any(|instruction| self.emitted[usize::from(instruction.result)])
            {
                return true;
            }
            match &block.terminator {
                CfgTerminator::Jump { target, args } => {
                    if self.edge_carries_live_value(*target, args) {
                        return true;
                    }
                    stack.push(*target);
                }
                CfgTerminator::Branch {
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                    ..
                } => {
                    if self.edge_carries_live_value(*then_target, then_args)
                        || self.edge_carries_live_value(*else_target, else_args)
                    {
                        return true;
                    }
                    stack.push(*then_target);
                    stack.push(*else_target);
                }
                // Reaching a function exit before the proposed join changes
                // control behavior and is never an empty region.
                CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {
                    return true;
                }
            }
        }
        false
    }

    fn edge_carries_live_value(&self, target: BlockId, args: &[ValueId]) -> bool {
        self.function
            .block(target)
            .params
            .iter()
            .zip(args)
            .any(|(parameter, _)| self.emitted[usize::from(*parameter)])
    }

    /// Decide which values are substituted into their consumer.
    ///
    /// Read exactly once, defined by an instruction in the same block as that
    /// read, and not itself an output. The same-block rule is what keeps this
    /// from moving work across a branch: a value defined before an `if` and read
    /// inside it would otherwise be evaluated only on one path, which is a
    /// different program when the expression can trap or when the branch is the
    /// hot one.
    fn plan_inlining(&mut self, outputs: &[ValueId]) {
        let mut uses = vec![0usize; self.function.values.len()];
        let mut block_of: Vec<Option<BlockId>> = vec![None; self.function.values.len()];
        // Terminators and the caller name their operands directly rather than
        // through `operand`, so those values must retain bindings.
        let mut must_bind = vec![false; self.function.values.len()];
        // A widen can spell one input once per held lane. Its deliberately
        // coarse use count below is enough to prevent one-use inlining, but not
        // exact enough for the source-cost calculation used for constants.
        let mut expanded_use = vec![false; self.function.values.len()];
        // The one reader, recorded as it is counted: searching for it per
        // candidate would be quadratic, and these graphs run to hundreds of
        // thousands of values.
        let mut reader_of: Vec<Option<ValueId>> = vec![None; self.function.values.len()];
        for value in &self.function.values {
            if !self.emitted[usize::from(value.id)] {
                continue;
            }
            // A widen names each held source lane once. Substituting a
            // multi-lane expression into it would therefore emit that
            // expression repeatedly; a scalar source still appears only once
            // and remains safe to inline.
            let repeated_widen = match value.kind {
                CfgValueKind::LaneWiden { input } => self
                    .function
                    .lanes_of(input)
                    .is_some_and(|lanes| lanes.len() > 1),
                _ => false,
            };
            if repeated_widen {
                if let CfgValueKind::LaneWiden { input } = value.kind {
                    expanded_use[usize::from(input)] = true;
                }
            }
            let weight = usize::from(repeated_widen) + 1;
            for operand in value.kind.operands() {
                uses[usize::from(operand)] += weight;
                reader_of[usize::from(operand)] = Some(value.id);
            }
        }
        for block in &self.function.blocks {
            for instruction in &block.instructions {
                if !self.emitted[usize::from(instruction.result)] {
                    continue;
                }
                block_of[usize::from(instruction.result)] = Some(block.id);
            }
            match &block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        if !self.emitted[usize::from(*arg)] {
                            continue;
                        }
                        uses[usize::from(*arg)] += 1;
                        must_bind[usize::from(*arg)] = true;
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    if self.effectful_branches.contains(&block.id) {
                        uses[usize::from(*condition)] += 1;
                        must_bind[usize::from(*condition)] = true;
                    }
                    for arg in then_args.iter().chain(else_args) {
                        if !self.emitted[usize::from(*arg)] {
                            continue;
                        }
                        uses[usize::from(*arg)] += 1;
                        must_bind[usize::from(*arg)] = true;
                    }
                }
                CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
            }
        }
        // An output is read by the caller, which no operand list records.
        for output in outputs {
            uses[usize::from(*output)] += 2;
            must_bind[usize::from(*output)] = true;
        }

        // Pure leaves can be substituted when their sole reader is an ordinary
        // value. Unlike arithmetic, this moves no potentially trapping work
        // across a branch. Calls such as `$simparam` and `$analysis` remain
        // bindings because caller-provided implementations need not be pure.
        for value in &self.function.values {
            let index = usize::from(value.id);
            if !self.emitted[index] {
                continue;
            }
            let single_use_leaf = uses[index] == 1
                && reader_of[index].is_some()
                && Self::is_inlineable_leaf(&value.kind);
            let cheaper_constant = !must_bind[index]
                && !expanded_use[index]
                && self
                    .constant_leaf_expression_len(value.id)
                    .is_some_and(|length| constant_inlining_saves_source(length, uses[index]));
            if block_of[index].is_none() && (single_use_leaf || cheaper_constant) {
                // Marked now, filled in by `leaves` in value-id order.
                self.inlined.insert(value.id, String::new());
            }
        }

        for block in &self.function.blocks {
            for instruction in &block.instructions {
                let result = instruction.result;
                if !self.emitted[usize::from(result)] {
                    continue;
                }
                if uses[usize::from(result)] != 1 {
                    continue;
                }
                let same_block = reader_of[usize::from(result)]
                    .and_then(|reader| block_of[usize::from(reader)])
                    .is_some_and(|defined_in| defined_in == block.id);
                if same_block {
                    // Marked now, filled in when the instruction is reached.
                    self.inlined.insert(result, String::new());
                }
            }
        }

        // One-use substitution is source-efficient, but left unbounded it can
        // turn a long arithmetic chain into a single 16-kilobyte Rust
        // expression. Keep the expansion under a fixed node budget and let an
        // ordinary SSA binding cut the tree when it would cross that boundary.
        // CFG values are created after their operands; a forward reference is
        // treated conservatively as over budget if a future transformation ever
        // changes that invariant.
        let mut expanded_nodes = vec![1usize; self.function.values.len()];
        for value in &self.function.values {
            if !self.inlined.contains_key(&value.id) {
                continue;
            }
            let index = usize::from(value.id);
            let mut nodes = 1usize;
            for operand in value.kind.operands() {
                let operand_index = usize::from(operand);
                if operand_index >= index {
                    nodes = MAX_INLINED_EXPRESSION_NODES + 1;
                    break;
                }
                nodes = nodes.saturating_add(if self.inlined.contains_key(&operand) {
                    expanded_nodes[operand_index]
                } else {
                    1
                });
            }
            if nodes > MAX_INLINED_EXPRESSION_NODES {
                self.inlined.remove(&value.id);
            } else {
                expanded_nodes[index] = nodes;
            }
        }
    }

    fn plan_names(&mut self) {
        self.names = vec![String::new(); self.function.values.len()];
        let mut ordinal = 0usize;
        for value in &self.function.values {
            if !self.emitted[usize::from(value.id)] || self.inlined.contains_key(&value.id) {
                continue;
            }
            self.names[usize::from(value.id)] = compact_local_name(ordinal);
            ordinal += 1;
        }
    }

    fn value_name(&self, value: ValueId) -> &str {
        let name = &self.names[usize::from(value)];
        debug_assert!(
            !name.is_empty(),
            "inlined value {value} has no local binding"
        );
        name
    }

    /// Bind every value no block defines.
    ///
    /// Constants, parameters, and node potentials belong to no block by design
    /// — every block may read them and none owns them — so nothing in the walk
    /// over blocks would ever emit them. In id order, which is a valid order
    /// because a leaf that reads another (a `$simparam` fallback) was created
    /// after it.
    fn leaves(&mut self) -> Result<(), EmitError> {
        let mut defined: HashSet<ValueId> = HashSet::new();
        for block in &self.function.blocks {
            defined.extend(block.params.iter().copied());
            defined.extend(block.instructions.iter().map(|entry| entry.result));
        }
        for value in &self.function.values {
            if !self.emitted[usize::from(value.id)] {
                continue;
            }
            if defined.contains(&value.id) {
                continue;
            }
            let expression = self.expression(value.id)?;
            let expression_is_atomic = self.inlined_expression_is_atomic(value.id);
            if let Some(slot) = self.inlined.get_mut(&value.id) {
                *slot = if expression_is_atomic {
                    expression
                } else {
                    format!("({expression})")
                };
                continue;
            }
            self.line(
                1,
                &format!("let {}={expression};", self.value_name(value.id)),
            );
        }
        Ok(())
    }

    /// Emit `block` and everything it flows into, stopping before `stop`.
    fn block(
        &mut self,
        block: BlockId,
        stop: Option<BlockId>,
        depth: usize,
    ) -> Result<(), EmitError> {
        let mut current = block;
        loop {
            if Some(current) == stop {
                return Ok(());
            }
            if self.loop_headers.contains(&current) {
                self.emit_loop(current, depth)?;
                match self.loop_exit(current) {
                    Some(exit) => {
                        current = exit;
                        continue;
                    }
                    None => return Ok(()),
                }
            }

            self.instructions(current, depth)?;
            match &self.function.block(current).terminator {
                CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {
                    return Ok(());
                }
                CfgTerminator::Jump { target, args } => {
                    let (target, args) = (*target, args.clone());
                    if self.loop_headers.contains(&target) {
                        self.initialize_or_pass_loop_arguments(target, &args, depth);
                    } else {
                        self.pass_arguments(target, &args, depth);
                    }
                    current = target;
                }
                CfgTerminator::Branch { .. } => {
                    // A conditional edge cannot initialize a carried value in
                    // both arms with one Rust declaration. Keep those loop
                    // targets declared outside the arms; each selected edge
                    // assigns them through the ordinary parallel-copy path.
                    self.declare_loop_targets(current, depth);
                    let join = self.emit_diamond(current, depth)?;
                    match join {
                        Some(join) => current = join,
                        None => return Ok(()),
                    }
                }
            }
        }
    }

    /// `if c { .. } else { .. }`, returning the block both arms reach.
    fn emit_diamond(&mut self, block: BlockId, depth: usize) -> Result<Option<BlockId>, EmitError> {
        let CfgTerminator::Branch {
            condition,
            then_target,
            then_args,
            else_target,
            else_args,
        } = self.function.block(block).terminator.clone()
        else {
            return Err(EmitError::UnstructuredControlFlow(block));
        };

        let join = self.join_of(block);
        if let Some(join) = join
            && self.emit_single_value_diamond(
                condition,
                then_target,
                &then_args,
                else_target,
                &else_args,
                join,
                depth,
            )?
        {
            return Ok(Some(join));
        }
        // Declared before the arms so both may assign it, which is also how
        // Rust's definite-assignment check ends up doing the SSA verification.
        if let Some(join) = join {
            self.declare_all(&self.function.block(join).params.clone(), depth);
        }

        if let Some(join) = join {
            let then_empty = self.empty_diamond_arm(then_target, &then_args, join);
            let else_empty = self.empty_diamond_arm(else_target, &else_args, join);
            if then_empty && else_empty {
                return Ok(Some(join));
            }
            if else_empty {
                let condition = self.truth_operand(condition);
                self.line(depth, &format!("if {condition}{{"));
                self.pass_arguments(then_target, &then_args, depth + 1);
                self.block(then_target, Some(join), depth + 1)?;
                self.line(depth, "}");
                return Ok(Some(join));
            }
            if then_empty {
                let condition = self.false_operand(condition);
                self.line(depth, &format!("if {condition}{{"));
                self.pass_arguments(else_target, &else_args, depth + 1);
                self.block(else_target, Some(join), depth + 1)?;
                self.line(depth, "}");
                return Ok(Some(join));
            }
        }

        let condition = self.truth_operand(condition);
        self.line(depth, &format!("if {condition}{{"));
        self.pass_arguments(then_target, &then_args, depth + 1);
        self.block(then_target, join, depth + 1)?;
        self.line(depth, "}else{");
        self.pass_arguments(else_target, &else_args, depth + 1);
        self.block(else_target, join, depth + 1)?;
        self.line(depth, "}");
        Ok(join)
    }

    /// Whether emitting one arm would produce no live statements or edge writes.
    ///
    /// Shared preprocessing can remove every data value from a deeply nested
    /// region while canonical CFG DCE conservatively retains its conditions.
    /// Walk the whole arm so a chain of routing blocks and no-op diamonds is as
    /// empty as a direct edge to the join.
    fn empty_diamond_arm(&self, target: BlockId, edge_args: &[ValueId], join: BlockId) -> bool {
        !self.region_has_live_effect(target, edge_args, join)
    }

    /// Emit a one-value, straight-line diamond as one typed `if` expression.
    ///
    /// Multi-value joins deliberately retain ordinary edge assignments. Tuple
    /// expressions save source, but paired seven-sample BSIM-BULK measurements
    /// showed a repeatable 0.78% leaf-compile regression. Each non-empty arm
    /// must be a unique-predecessor block that jumps directly to the join, so
    /// this preserves instruction order and never moves work across a condition
    /// or loop boundary.
    #[allow(clippy::too_many_arguments)]
    fn emit_single_value_diamond(
        &mut self,
        condition: ValueId,
        then_target: BlockId,
        then_args: &[ValueId],
        else_target: BlockId,
        else_args: &[ValueId],
        join: BlockId,
        depth: usize,
    ) -> Result<bool, EmitError> {
        let params = self.function.block(join).params.clone();
        if params.len() != 1
            || !self.emitted[usize::from(params[0])]
            || self.declared.contains(&params[0])
        {
            return Ok(false);
        }
        let Some((then_block, then_result)) = self.single_value_arm(then_target, then_args, join)
        else {
            return Ok(false);
        };
        let Some((else_block, else_result)) = self.single_value_arm(else_target, else_args, join)
        else {
            return Ok(false);
        };

        let param = params[0];
        self.declared.insert(param);
        let condition = self.truth_operand(condition);
        self.line(
            depth,
            &format!("let {}=if {condition}{{", self.value_name(param)),
        );
        self.emit_single_value_arm(then_block, then_result, param, depth + 1)?;
        self.line(depth, "}else{");
        self.emit_single_value_arm(else_block, else_result, param, depth + 1)?;
        self.line(depth, "};");
        Ok(true)
    }

    /// Return the optional straight-line arm block and its one join argument.
    fn single_value_arm(
        &self,
        target: BlockId,
        edge_args: &[ValueId],
        join: BlockId,
    ) -> Option<(Option<BlockId>, ValueId)> {
        if target == join {
            return (edge_args.len() == 1).then(|| (None, edge_args[0]));
        }
        let block = self.function.block(target);
        if self.loop_headers.contains(&target)
            || self.predecessor_counts[usize::from(target)] != 1
            || !block.params.is_empty()
            || !edge_args.is_empty()
        {
            return None;
        }
        let CfgTerminator::Jump { target, args } = &block.terminator else {
            return None;
        };
        (*target == join && args.len() == 1).then(|| (Some(block.id), args[0]))
    }

    fn emit_single_value_arm(
        &mut self,
        block: Option<BlockId>,
        result: ValueId,
        target: ValueId,
        depth: usize,
    ) -> Result<(), EmitError> {
        if let Some(block) = block {
            self.instructions(block, depth)?;
        }
        let result = self.coerce_operand(result, self.function.value(target).value_type);
        self.line(depth, &result);
        Ok(())
    }

    /// `loop { .. if !c { break } .. }` for the header/body/exit triple the
    /// lowering builds.
    fn emit_loop(&mut self, header: BlockId, depth: usize) -> Result<(), EmitError> {
        // The carried variables enter with the values the edge into the loop
        // supplied, and are reassigned on the back edge.
        for param in &self.function.block(header).params.clone() {
            self.declare_mutable(*param, depth);
        }
        self.line(depth, "loop{");
        self.instructions(header, depth + 1)?;

        let CfgTerminator::Branch {
            condition,
            then_target,
            then_args,
            else_target,
            else_args,
        } = self.function.block(header).terminator.clone()
        else {
            return Err(EmitError::UnstructuredControlFlow(header));
        };
        let (body, body_args, exit, exit_args) =
            if self.reaches_back_to(then_target, header, else_target) {
                (then_target, then_args, else_target, else_args)
            } else {
                (else_target, else_args, then_target, then_args)
            };

        let condition = self.false_operand(condition);
        self.line(depth + 1, &format!("if {condition}{{"));
        self.pass_arguments(exit, &exit_args, depth + 2);
        self.line(depth + 2, "break;");
        self.line(depth + 1, "}");

        self.pass_arguments(body, &body_args, depth + 1);
        self.block(body, Some(header), depth + 1)?;
        // Falling out of the body means reaching the back edge, whose arguments
        // were already assigned by `pass_arguments` on the jump.
        self.line(depth, "}");
        Ok(())
    }

    fn loop_exit(&self, header: BlockId) -> Option<BlockId> {
        let CfgTerminator::Branch {
            then_target,
            else_target,
            ..
        } = self.function.block(header).terminator
        else {
            return None;
        };
        if self.reaches_back_to(then_target, header, else_target) {
            Some(else_target)
        } else {
            Some(then_target)
        }
    }

    /// Whether `candidate` leads back to `header` without passing `other`.
    fn reaches_back_to(&self, candidate: BlockId, header: BlockId, other: BlockId) -> bool {
        let mut seen: HashSet<BlockId> = HashSet::new();
        let mut stack = vec![candidate];
        while let Some(block) = stack.pop() {
            if block == header {
                return true;
            }
            if block == other || !seen.insert(block) {
                continue;
            }
            stack.extend(self.function.block(block).successors());
        }
        false
    }

    /// The block both arms of a branch reconverge at.
    ///
    /// It is the branching block's immediate post-dominator, computed once for
    /// the whole function. Searching for it per branch instead — walking
    /// forward from each arm until the frontiers meet — is quadratic in the
    /// block count, which on a CMC-class model with a few thousand blocks
    /// dominated emission time.
    fn join_of(&self, block: BlockId) -> Option<BlockId> {
        match self.post_dominators[usize::from(block)] {
            Some(join) if join != block => Some(join),
            _ => None,
        }
    }

    fn instructions(&mut self, block: BlockId, depth: usize) -> Result<(), EmitError> {
        // A parameter is assigned by the edge that arrived here, so its value is
        // already the one this block sees.
        for param in &self.function.block(block).params.clone() {
            if !self.emitted[usize::from(*param)] {
                continue;
            }
            self.capture(*param, depth);
        }
        for instruction in &self.function.block(block).instructions.clone() {
            if !self.emitted[usize::from(instruction.result)] {
                continue;
            }
            let expression = self.expression(instruction.result)?;
            // Composite expressions are parenthesised because they are about to
            // be dropped into the middle of another expression. Constructor,
            // call, literal, and indexing forms are already atomic, so wrapping
            // those only adds tokens for rustc to parse.
            let expression_is_atomic = self.inlined_expression_is_atomic(instruction.result);
            if let Some(slot) = self.inlined.get_mut(&instruction.result) {
                *slot = if expression_is_atomic {
                    expression
                } else {
                    format!("({expression})")
                };
                continue;
            }
            self.line(
                depth,
                &format!("let {}={expression};", self.value_name(instruction.result)),
            );
            self.capture(instruction.result, depth);
        }
        Ok(())
    }

    /// Copy a wanted value out of the region that defines it.
    ///
    /// `depth` is the lexical nesting of the statement being emitted: one is the
    /// body's own level, where a binding outlives the walk and needs no copy.
    /// Anything deeper is inside an `if` arm or a `loop`, and the copy is the
    /// only way its value reaches the caller. Inside a loop the assignment runs
    /// every iteration and what survives is the last one, which is what "the
    /// value after the loop" means.
    fn capture(&mut self, value: ValueId, depth: usize) {
        if depth <= 1 || !self.wanted.contains(&value) || self.captured.contains_key(&value) {
            return;
        }
        let name = format!("o{}", self.value_name(value));
        self.line(depth, &format!("{name}={};", self.value_name(value)));
        self.captured.insert(value, name);
    }

    fn capture_declarations(&self) -> String {
        let mut out = String::new();
        for (value, name) in &self.captured {
            let initial = self.zero(*value);
            let _ = writeln!(out, "let mut {name}={initial};");
        }
        out
    }

    fn output_name(&self, value: ValueId) -> String {
        self.captured
            .get(&value)
            .cloned()
            .unwrap_or_else(|| self.value_name(value).to_owned())
    }

    /// Assign a successor's parameters from the arguments on this edge.
    ///
    /// Through temporaries only when an argument is itself one of the
    /// parameters being written — a loop that swaps two carried variables would
    /// otherwise feed the first assignment into the second read. A merge at the
    /// end of a conditional never has that shape, and on a model with a few
    /// thousand blocks the temporaries are otherwise two emitted lines per
    /// parameter per edge for nothing.
    fn pass_arguments(&mut self, target: BlockId, args: &[ValueId], depth: usize) {
        let pairs = self
            .function
            .block(target)
            .params
            .iter()
            .copied()
            .zip(args.iter().copied())
            .filter(|(parameter, _)| self.emitted[usize::from(*parameter)])
            .collect::<Vec<_>>();
        if pairs.is_empty() {
            return;
        }
        let clobbers = pairs
            .iter()
            .any(|(_, argument)| pairs.iter().any(|(parameter, _)| parameter == argument));
        if !clobbers {
            for (param, argument) in pairs {
                let argument = self.coerce_operand(argument, self.function.value(param).value_type);
                self.line(depth, &format!("{}={argument};", self.value_name(param)));
            }
            return;
        }
        for (index, (param, argument)) in pairs.iter().copied().enumerate() {
            let argument = self.coerce_operand(argument, self.function.value(param).value_type);
            self.line(depth, &format!("let edge{index}={argument};"));
        }
        for (index, (param, _)) in pairs.into_iter().enumerate() {
            self.line(depth, &format!("{} = edge{index};", self.value_name(param)));
        }
    }

    /// Initialize loop-carried values directly on their first unconditional
    /// edge, then use ordinary parallel assignments on every later edge.
    ///
    /// The old path emitted `let mut value = 0.0; value = argument;` for every
    /// loop parameter even when a unique preheader supplied its real initial
    /// value immediately. Direct initialization is valid only when every live
    /// parameter is new and no argument refers to a parameter being declared.
    /// Conditional entries, back edges, and swaps retain `pass_arguments`.
    fn initialize_or_pass_loop_arguments(
        &mut self,
        target: BlockId,
        args: &[ValueId],
        depth: usize,
    ) {
        let pairs = self
            .function
            .block(target)
            .params
            .iter()
            .copied()
            .zip(args.iter().copied())
            .filter(|(parameter, _)| self.emitted[usize::from(*parameter)])
            .collect::<Vec<_>>();
        if pairs.is_empty() {
            return;
        }
        let can_initialize = pairs
            .iter()
            .all(|(parameter, _)| !self.declared.contains(parameter))
            && !pairs
                .iter()
                .any(|(_, argument)| pairs.iter().any(|(parameter, _)| parameter == argument));
        if !can_initialize {
            for (parameter, _) in &pairs {
                self.declare_mutable(*parameter, depth);
            }
            self.pass_arguments(target, args, depth);
            return;
        }
        for (parameter, argument) in pairs {
            self.declared.insert(parameter);
            let argument = self.coerce_operand(argument, self.function.value(parameter).value_type);
            self.line(
                depth,
                &format!("let mut {}={argument};", self.value_name(parameter)),
            );
        }
    }

    fn declare_loop_targets(&mut self, block: BlockId, depth: usize) {
        for successor in self.function.block(block).successors() {
            if !self.loop_headers.contains(&successor) {
                continue;
            }
            for param in &self.function.block(successor).params.clone() {
                self.declare_mutable(*param, depth);
            }
        }
    }

    fn declare_all(&mut self, values: &[ValueId], depth: usize) {
        for value in values {
            if !self.emitted[usize::from(*value)] {
                continue;
            }
            if self.declared.insert(*value) {
                self.line(depth, &format!("let {};", self.value_name(*value)));
            }
        }
    }

    fn declare_mutable(&mut self, value: ValueId, depth: usize) {
        if !self.emitted[usize::from(value)] {
            return;
        }
        if self.declared.insert(value) {
            let initial = self.zero(value);
            self.line(
                depth,
                &format!("let mut {}={initial};", self.value_name(value)),
            );
        }
    }

    fn zero(&self, value: ValueId) -> String {
        if self.function.value(value).value_type == CfgValueType::Boolean {
            return "false".to_string();
        }
        match self.function.lanes_of(value) {
            Some(lanes) if lanes.len() > 1 => {
                format!("{}([0.0;{}])", lane_type_name(lanes.len()), lanes.len())
            }
            Some(_) | None => "0.0".to_string(),
        }
    }

    fn line(&mut self, _depth: usize, text: &str) {
        self.source.push_str(text);
        self.source.push('\n');
    }

    /// How a reader names `value`: its binding, or its whole expression when it
    /// was chosen for substitution.
    fn operand(&self, value: ValueId) -> String {
        match self.inlined.get(&value) {
            Some(expression) if !expression.is_empty() => expression.clone(),
            _ => self.value_name(value).to_owned(),
        }
    }

    /// A value in numeric context, preserving Verilog-A's `0.0`/`1.0`
    /// representation only at the boundary where arithmetic requires it.
    fn numeric_operand(&self, value: ValueId) -> String {
        let operand = self.operand(value);
        if self.function.value(value).value_type == CfgValueType::Boolean {
            format!("({operand} as u8 as f64)")
        } else {
            operand
        }
    }

    /// A value in control-flow context. IEEE NaN remains true because the
    /// numeric rule is exactly `value != 0.0`, matching the evaluator.
    fn truth_operand(&self, value: ValueId) -> String {
        if let CfgValueKind::ParameterGiven(parameter) = self.function.value(value).kind {
            return format!(
                "{}[{}]",
                self.bindings.parameter_given,
                usize::from(parameter)
            );
        }
        let operand = self.operand(value);
        if self.function.value(value).value_type == CfgValueType::Boolean {
            operand
        } else {
            format!("{operand}!=0.0")
        }
    }

    fn false_operand(&self, value: ValueId) -> String {
        if let CfgValueKind::ParameterGiven(parameter) = self.function.value(value).kind {
            return format!(
                "!{}[{}]",
                self.bindings.parameter_given,
                usize::from(parameter)
            );
        }
        let operand = self.operand(value);
        if self.function.value(value).value_type == CfgValueType::Boolean {
            format!("!{operand}")
        } else {
            format!("{operand}==0.0")
        }
    }

    /// Coerce an edge or output to the representation its destination expects.
    fn coerce_operand(&self, value: ValueId, target: CfgValueType) -> String {
        match target {
            CfgValueType::Boolean => self.truth_operand(value),
            CfgValueType::Real
                if self.function.value(value).value_type == CfgValueType::Boolean =>
            {
                self.numeric_operand(value)
            }
            CfgValueType::Real | CfgValueType::Lanes(_) => self.operand(value),
            // No coercion exists. This emitter produces `f64` arithmetic, and
            // a four-state value has no `f64` that means the same thing;
            // `ModelPlan::build` refuses a module carrying one before any of
            // it is emitted, so reaching here is a routing bug.
            CfgValueType::Integer | CfgValueType::FourState { .. } | CfgValueType::Effect => {
                self.operand(value)
            }
        }
    }

    fn inlined_expression_is_atomic(&self, value: ValueId) -> bool {
        if matches!(
            self.function.value(value).kind,
            CfgValueKind::ParameterGiven(_)
        ) && self.function.value(value).value_type != CfgValueType::Boolean
        {
            return false;
        }
        if matches!(self.function.value(value).kind, CfgValueKind::Staged { .. })
            && self.function.value(value).value_type == CfgValueType::Boolean
        {
            return false;
        }
        matches!(
            self.function.value(value).kind,
            CfgValueKind::RealConstant(_)
                | CfgValueKind::BooleanConstant(_)
                | CfgValueKind::Parameter(_)
                | CfgValueKind::ParameterGiven(_)
                | CfgValueKind::EventState(_)
                | CfgValueKind::Temperature
                | CfgValueKind::ThermalVoltage
                | CfgValueKind::Multiplicity
                | CfgValueKind::Time
                | CfgValueKind::NodePotential(_)
                | CfgValueKind::BranchFlow(_)
                | CfgValueKind::BranchUnknownFlow(_)
                | CfgValueKind::Staged { .. }
                | CfgValueKind::Ddt { .. }
                | CfgValueKind::DdtScale
                | CfgValueKind::Idt { .. }
                | CfgValueKind::IdtScale
                | CfgValueKind::Limit { .. }
                | CfgValueKind::LimitPrevious { .. }
                | CfgValueKind::LaneSplat(_)
                | CfgValueKind::LaneWiden { .. }
                | CfgValueKind::LaneExtract { .. }
        )
    }

    fn is_inlineable_leaf(kind: &CfgValueKind) -> bool {
        matches!(
            kind,
            CfgValueKind::RealConstant(_)
                | CfgValueKind::BooleanConstant(_)
                | CfgValueKind::Parameter(_)
                | CfgValueKind::ParameterGiven(_)
                | CfgValueKind::EventState(_)
                | CfgValueKind::Temperature
                | CfgValueKind::ThermalVoltage
                | CfgValueKind::Multiplicity
                | CfgValueKind::Time
                | CfgValueKind::NodePotential(_)
                | CfgValueKind::BranchFlow(_)
                | CfgValueKind::BranchUnknownFlow(_)
                | CfgValueKind::Staged { .. }
        )
    }

    /// Exact emitted length of an immutable constant leaf, when it has one.
    fn constant_leaf_expression_len(&self, value: ValueId) -> Option<usize> {
        match self.function.value(value).kind {
            CfgValueKind::RealConstant(constant) => Some(real_literal(constant).len()),
            CfgValueKind::BooleanConstant(constant) => Some(if constant {
                "true".len()
            } else {
                "false".len()
            }),
            CfgValueKind::LaneSplat(constant) => {
                let width = self.function.lanes_of(value).map_or(0, <[u32]>::len);
                let literal = real_literal(constant);
                Some(if width == 1 {
                    literal.len()
                } else {
                    format!("{}([{literal};{width}])", lane_type_name(width)).len()
                })
            }
            _ => None,
        }
    }

    fn unary_expression(&self, op: CfgUnaryOp, input: ValueId) -> String {
        if op == CfgUnaryOp::Not {
            unary(op, &self.truth_operand(input))
        } else {
            unary(op, &self.numeric_operand(input))
        }
    }

    fn binary_expression(&self, op: CfgBinaryOp, left: ValueId, right: ValueId) -> String {
        if matches!(op, CfgBinaryOp::And | CfgBinaryOp::Or) {
            binary(op, &self.truth_operand(left), &self.truth_operand(right))
        } else {
            binary(
                op,
                &self.numeric_operand(left),
                &self.numeric_operand(right),
            )
        }
    }

    fn expression(&self, value: ValueId) -> Result<String, EmitError> {
        let bindings = self.bindings;
        Ok(match &self.function.value(value).kind {
            CfgValueKind::RealConstant(constant) => real_literal(*constant),
            CfgValueKind::BooleanConstant(constant) => {
                if *constant {
                    "true".into()
                } else {
                    "false".into()
                }
            }
            CfgValueKind::BlockParameter => self.value_name(value).to_owned(),
            CfgValueKind::Parameter(parameter) => {
                format!("{}[{}]", bindings.parameters, usize::from(*parameter))
            }
            CfgValueKind::ParameterGiven(parameter) => {
                let given = format!("{}[{}]", bindings.parameter_given, usize::from(*parameter));
                if self.function.value(value).value_type == CfgValueType::Boolean {
                    given
                } else {
                    format!("{given} as u8 as f64")
                }
            }
            CfgValueKind::EventState(slot) => {
                format!("{}[{slot}]", bindings.event_state)
            }
            CfgValueKind::Temperature => bindings.temperature.clone(),
            CfgValueKind::ThermalVoltage => bindings.thermal_voltage.clone(),
            CfgValueKind::Multiplicity => bindings.multiplicity.clone(),
            CfgValueKind::Time => bindings.time.clone(),
            CfgValueKind::Analysis(name) => match name.as_str() {
                "__rspice_initial_step" => "ctx.analysis_initial_step()".to_string(),
                "__rspice_final_step" => "ctx.analysis_final_step()".to_string(),
                _ => format!("{}(\"{name}\")", bindings.analysis),
            },
            CfgValueKind::SimParam { name, fallback } => format!(
                "{}(\"{name}\", {})",
                bindings.simparam,
                self.numeric_operand(*fallback)
            ),
            CfgValueKind::NodePotential(node) => {
                format!("{}[{}]", bindings.node_potentials, usize::from(*node))
            }
            CfgValueKind::BranchFlow(branch) => {
                format!("{}[{}]", bindings.branch_flows, usize::from(*branch))
            }
            CfgValueKind::BranchUnknownFlow(unknown) => format!(
                "{}[{}]",
                bindings.branch_unknown_flows,
                usize::from(*unknown)
            ),
            CfgValueKind::Ddt { operator, input } => format!(
                "{}({}, {})",
                bindings.ddt,
                bindings
                    .ddt_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*input)
            ),
            CfgValueKind::DdtScale => format!("{}()", bindings.ddt_scale),
            CfgValueKind::Idt {
                operator,
                input,
                ic,
            } => format!(
                "{}({}, {}, {})",
                bindings.idt,
                bindings
                    .idt_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*input),
                self.numeric_operand(*ic)
            ),
            CfgValueKind::IdtScale => format!("{}()", bindings.idt_scale),
            CfgValueKind::Cross {
                operator,
                input,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => format!(
                "{}({}, {}, {}, {}, {}, {})",
                bindings.cross,
                bindings
                    .cross_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*input),
                self.numeric_operand(*direction),
                self.numeric_operand(*time_tol),
                self.numeric_operand(*expr_tol),
                self.numeric_operand(*enable),
            ),
            CfgValueKind::Above {
                operator,
                input,
                time_tol,
                expr_tol,
                enable,
            } => format!(
                "{}({}, {}, {}, {}, {})",
                bindings.above,
                bindings
                    .cross_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*input),
                self.numeric_operand(*time_tol),
                self.numeric_operand(*expr_tol),
                self.numeric_operand(*enable),
            ),
            CfgValueKind::Timer {
                operator,
                start,
                period,
                time_tol,
                enable,
            } => format!(
                "{}({}, {}, {}, {}, {})",
                bindings.timer,
                bindings
                    .timer_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*start),
                self.numeric_operand(*period),
                self.numeric_operand(*time_tol),
                self.numeric_operand(*enable),
            ),
            CfgValueKind::Limit {
                operator,
                proposed,
                candidate,
                ..
            } => format!(
                "{}({}, {}, {})",
                bindings.limit,
                bindings
                    .limit_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*proposed),
                self.numeric_operand(*candidate)
            ),
            CfgValueKind::LimitPrevious { operator, proposed } => format!(
                "{}({}, {})",
                bindings.limit_previous,
                bindings
                    .limit_slots
                    .get(operator)
                    .copied()
                    .unwrap_or_else(|| usize::from(*operator)),
                self.numeric_operand(*proposed)
            ),
            CfgValueKind::Ddx { .. } => return Err(EmitError::UnresolvedDdx(value)),
            CfgValueKind::Transition { .. } | CfgValueKind::TransitionDerivative { .. } => {
                return Err(EmitError::UnsupportedTransition(value));
            }
            CfgValueKind::Unary { op, input } => self.unary_expression(*op, *input),
            CfgValueKind::Binary { op, left, right } => self.binary_expression(*op, *left, *right),
            CfgValueKind::LaneSplat(constant) => {
                let width = self.function.lanes_of(value).map_or(0, <[u32]>::len);
                if width == 1 {
                    real_literal(*constant)
                } else {
                    format!(
                        "{}([{};{width}])",
                        lane_type_name(width),
                        real_literal(*constant)
                    )
                }
            }
            // The one packed form that is written out rather than called: which
            // lane lands where is a per-value permutation, not an operation.
            CfgValueKind::LaneWiden { input } => {
                let source = self.function.lanes_of(*input).unwrap_or(&[]);
                let elements: Vec<String> = self
                    .function
                    .lanes_of(value)
                    .unwrap_or(&[])
                    .iter()
                    .map(|lane| match source.iter().position(|held| held == lane) {
                        Some(position) => self.lane_element(*input, position),
                        None => "0.0".to_string(),
                    })
                    .collect();
                if elements.len() == 1 {
                    elements
                        .into_iter()
                        .next()
                        .unwrap_or_else(|| "0.0".to_string())
                } else {
                    format!(
                        "{}([{}])",
                        lane_type_name(elements.len()),
                        elements.join(",")
                    )
                }
            }
            CfgValueKind::LaneBinary { op, left, right } => {
                let symbol = if matches!(op, CfgBinaryOp::Sub) {
                    "-"
                } else {
                    "+"
                };
                format!("{}{symbol} {}", self.operand(*left), self.operand(*right))
            }
            CfgValueKind::LaneScalar { op, input, scalar } => {
                let symbol = if matches!(op, CfgBinaryOp::Div) {
                    "/"
                } else {
                    "*"
                };
                format!(
                    "{}{symbol} {}",
                    self.operand(*input),
                    self.numeric_operand(*scalar)
                )
            }
            CfgValueKind::LaneExtract { input, lane } => {
                let position = self.function.lane_position(*input, *lane).unwrap_or(0);
                self.lane_element(*input, position)
            }
            CfgValueKind::Staged { slot } => {
                let staged = format!("{}[{slot}]", bindings.staged);
                if self.function.value(value).value_type == CfgValueType::Boolean {
                    format!("{staged}!=0.0")
                } else {
                    staged
                }
            }

            CfgValueKind::FourStateConstant(_)
            | CfgValueKind::IntegerConstant(_)
            | CfgValueKind::DigitalSignalRead { .. }
            | CfgValueKind::DigitalBitwise { .. }
            | CfgValueKind::DigitalBitwiseNot { .. }
            | CfgValueKind::DigitalLogical { .. }
            | CfgValueKind::DigitalLogicalNot { .. }
            | CfgValueKind::DigitalEquality { .. }
            | CfgValueKind::DigitalCaseMatch { .. }
            | CfgValueKind::DigitalRelational { .. }
            | CfgValueKind::DigitalArithmetic { .. }
            | CfgValueKind::DigitalShift { .. }
            | CfgValueKind::DigitalPartSelect { .. }
            | CfgValueKind::DigitalConcat { .. }
            | CfgValueKind::DigitalSelect { .. }
            | CfgValueKind::DigitalBlockingWrite { .. }
            | CfgValueKind::DigitalNonblockingWrite { .. }
            | CfgValueKind::DigitalDriverWrite { .. } => {
                return Err(EmitError::DigitalValueInAnalogEmitter(value));
            }
        })
    }

    fn lane_element(&self, value: ValueId, position: usize) -> String {
        let operand = self.operand(value);
        if self
            .function
            .lanes_of(value)
            .is_some_and(|lanes| lanes.len() == 1)
        {
            operand
        } else {
            format!("{operand}[{position}]")
        }
    }
}

fn predecessor_counts(function: &CfgFunction) -> Vec<usize> {
    let mut counts = vec![0usize; function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            counts[usize::from(successor)] += 1;
        }
    }
    counts
}

/// Immediate post-dominators, by the same iteration a compiler uses for
/// dominators, run on the reversed graph.
///
/// The lowering leaves exactly one `Return`, so the reversed graph has a single
/// entry and needs no synthetic exit.
fn immediate_post_dominators(function: &CfgFunction) -> Vec<Option<BlockId>> {
    let Some(exit) = function
        .blocks
        .iter()
        .find(|block| matches!(block.terminator, CfgTerminator::Return))
        .map(|block| block.id)
    else {
        return vec![None; function.blocks.len()];
    };

    let mut successors: Vec<Vec<BlockId>> = vec![Vec::new(); function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            // Reversed: a successor's predecessor list is its successor list here.
            successors[usize::from(successor)].push(block.id);
        }
    }

    let order = reverse_postorder(&successors, exit, function.blocks.len());
    let mut position = vec![usize::MAX; function.blocks.len()];
    for (index, block) in order.iter().enumerate() {
        position[usize::from(*block)] = index;
    }

    let mut predecessors: Vec<Vec<BlockId>> = vec![Vec::new(); function.blocks.len()];
    for (block, targets) in successors.iter().enumerate() {
        for target in targets {
            predecessors[usize::from(*target)].push(BlockId::from(block));
        }
    }

    let mut ipdom: Vec<Option<BlockId>> = vec![None; function.blocks.len()];
    ipdom[usize::from(exit)] = Some(exit);
    loop {
        let mut changed = false;
        for block in &order {
            if *block == exit {
                continue;
            }
            let mut candidate: Option<BlockId> = None;
            for predecessor in &predecessors[usize::from(*block)] {
                if ipdom[usize::from(*predecessor)].is_none() {
                    continue;
                }
                candidate = Some(match candidate {
                    Some(current) => intersect(&ipdom, &position, *predecessor, current, exit),
                    None => *predecessor,
                });
            }
            if candidate.is_some() && ipdom[usize::from(*block)] != candidate {
                ipdom[usize::from(*block)] = candidate;
                changed = true;
            }
        }
        if !changed {
            return ipdom;
        }
    }
}

fn reverse_postorder(successors: &[Vec<BlockId>], entry: BlockId, count: usize) -> Vec<BlockId> {
    let mut visited = vec![false; count];
    let mut postorder = Vec::with_capacity(count);
    let mut stack = vec![(entry, 0usize)];
    visited[usize::from(entry)] = true;
    while let Some((block, index)) = stack.pop() {
        let edges = &successors[usize::from(block)];
        if index < edges.len() {
            stack.push((block, index + 1));
            let next = edges[index];
            if !visited[usize::from(next)] {
                visited[usize::from(next)] = true;
                stack.push((next, 0));
            }
        } else {
            postorder.push(block);
        }
    }
    postorder.reverse();
    postorder
}

/// The nearest block that post-dominates both, walking up the partial tree.
fn intersect(
    ipdom: &[Option<BlockId>],
    position: &[usize],
    mut left: BlockId,
    mut right: BlockId,
    exit: BlockId,
) -> BlockId {
    while left != right {
        while position[usize::from(left)] > position[usize::from(right)] {
            match ipdom[usize::from(left)] {
                Some(next) if next != left => left = next,
                _ => return exit,
            }
        }
        while position[usize::from(right)] > position[usize::from(left)] {
            match ipdom[usize::from(right)] {
                Some(next) if next != right => right = next,
                _ => return exit,
            }
        }
    }
    left
}

/// Blocks that some edge jumps backwards into.
fn back_edge_targets(function: &CfgFunction) -> HashSet<BlockId> {
    // Depth-first, tracking the stack: a successor already on the stack is
    // reached by a back edge and is therefore a loop header.
    let mut headers = HashSet::new();
    let mut state: HashMap<BlockId, u8> = HashMap::new();
    let mut stack = vec![(function.entry, 0usize)];
    state.insert(function.entry, 1);
    while let Some((block, index)) = stack.pop() {
        let successors = function.block(block).successors();
        if index < successors.len() {
            stack.push((block, index + 1));
            let successor = successors[index];
            match state.get(&successor) {
                Some(1) => {
                    headers.insert(successor);
                }
                Some(_) => {}
                None => {
                    state.insert(successor, 1);
                    stack.push((successor, 0));
                }
            }
        } else {
            state.insert(block, 2);
        }
    }
    headers
}

/// Arguments reaching each block parameter, indexed by value id.
fn incoming_block_arguments(function: &CfgFunction) -> Vec<Vec<ValueId>> {
    let mut incoming = vec![Vec::new(); function.values.len()];
    let mut record = |target: BlockId, args: &[ValueId]| {
        for (parameter, argument) in function.block(target).params.iter().zip(args) {
            incoming[usize::from(*parameter)].push(*argument);
        }
    };
    for block in &function.blocks {
        match &block.terminator {
            CfgTerminator::Jump { target, args } => record(*target, args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                record(*then_target, then_args);
                record(*else_target, else_args);
            }
            CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
        }
    }
    incoming
}

fn mark_live(value: ValueId, live: &mut [bool], worklist: &mut Vec<ValueId>) {
    let slot = &mut live[usize::from(value)];
    if !*slot {
        *slot = true;
        worklist.push(value);
    }
}

/// Close liveness over ordinary operands and SSA merge inputs.
fn propagate_value_liveness(
    function: &CfgFunction,
    incoming: &[Vec<ValueId>],
    live: &mut [bool],
    worklist: &mut Vec<ValueId>,
) {
    while let Some(value) = worklist.pop() {
        for operand in function.value(value).kind.operands() {
            mark_live(operand, live, worklist);
        }
        if matches!(function.value(value).kind, CfgValueKind::BlockParameter) {
            for argument in &incoming[usize::from(value)] {
                mark_live(*argument, live, worklist);
            }
        }
    }
}

/// A literal that reads back as exactly this value.
fn real_literal(value: f64) -> String {
    if value.is_nan() {
        return "f64::NAN".into();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "f64::NEG_INFINITY".into()
        } else {
            "f64::INFINITY".into()
        };
    }
    // Both `Display` forms use Rust's shortest round-tripping conversion. Plain
    // decimal wins for small integers and ordinary fractions, while scientific
    // notation wins for large magnitudes. The explicit type is required when a
    // generated binding is only used as a float-method receiver; Rust otherwise
    // leaves that binding ambiguous between f32 and f64.
    let decimal = value.to_string();
    let scientific = format!("{value:e}");
    let shortest = if decimal.len() <= scientific.len() {
        decimal
    } else {
        scientific
    };
    format!("{shortest}f64")
}

/// A short, deterministic Rust identifier for an otherwise meaningless local.
///
/// Dense numbering already keeps IR ids that disappeared during optimization
/// out of the source. Bijective base 26 cuts the remaining identifier bytes
/// substantially on large models without changing the token stream's structure
/// or relying on formatting that `rustfmt` would undo. Uppercase-only names are
/// valid Rust identifiers, cannot be keywords, and cannot collide with the
/// lowercase runtime bindings used by emitted expressions.
fn compact_local_name(mut ordinal: usize) -> String {
    // Fourteen base-26 digits cover every 64-bit usize value.
    let mut encoded = [0u8; 14];
    let mut cursor = encoded.len();
    loop {
        cursor -= 1;
        encoded[cursor] = b'A' + (ordinal % 26) as u8;
        ordinal /= 26;
        if ordinal == 0 {
            break;
        }
        // Make the representation bijective: there is no zero digit before
        // the first character, so Z is followed by AA rather than BA.
        ordinal -= 1;
    }
    std::str::from_utf8(&encoded[cursor..])
        .expect("the compact local alphabet contains only ASCII")
        .to_owned()
}

fn unary(op: CfgUnaryOp, input: &str) -> String {
    match op {
        CfgUnaryOp::Neg => format!("-{input}"),
        CfgUnaryOp::Not => format!("!({input})"),
        CfgUnaryOp::Exp => format!("{input}.exp()"),
        CfgUnaryOp::LimExp => format!("rspice_limexp({input})"),
        CfgUnaryOp::LimitedExp => format!("rspice_limited_exp({input})"),
        CfgUnaryOp::LimitedExpDerivative => format!("rspice_limited_exp_derivative({input})"),
        CfgUnaryOp::Ln => format!("{input}.ln()"),
        CfgUnaryOp::Log10 => format!("{input}.log10()"),
        CfgUnaryOp::Sqrt => format!("{input}.sqrt()"),
        CfgUnaryOp::Abs => format!("{input}.abs()"),
        CfgUnaryOp::Sin => format!("{input}.sin()"),
        CfgUnaryOp::Cos => format!("{input}.cos()"),
        CfgUnaryOp::Tan => format!("{input}.tan()"),
        CfgUnaryOp::Sinh => format!("{input}.sinh()"),
        CfgUnaryOp::Cosh => format!("{input}.cosh()"),
        CfgUnaryOp::Tanh => format!("{input}.tanh()"),
        CfgUnaryOp::Asin => format!("{input}.asin()"),
        CfgUnaryOp::Acos => format!("{input}.acos()"),
        CfgUnaryOp::Atan => format!("{input}.atan()"),
        CfgUnaryOp::Asinh => format!("{input}.asinh()"),
        CfgUnaryOp::Acosh => format!("{input}.acosh()"),
        CfgUnaryOp::Atanh => format!("{input}.atanh()"),
        CfgUnaryOp::Floor => format!("{input}.floor()"),
        CfgUnaryOp::Ceil => format!("{input}.ceil()"),
    }
}

fn binary(op: CfgBinaryOp, left: &str, right: &str) -> String {
    match op {
        CfgBinaryOp::Add => format!("{left}+ {right}"),
        CfgBinaryOp::Sub => format!("{left}- {right}"),
        CfgBinaryOp::Mul => format!("{left}* {right}"),
        CfgBinaryOp::Div => format!("{left}/ {right}"),
        CfgBinaryOp::Mod => format!("{left}% {right}"),
        CfgBinaryOp::Pow => format!("{left}.powf({right})"),
        CfgBinaryOp::Hypot => format!("{left}.hypot({right})"),
        CfgBinaryOp::Atan2 => format!("{left}.atan2({right})"),
        // Written out rather than `f64::min`, which disagrees with the
        // interpreter when one operand is NaN: it returns the other, this
        // returns whichever the comparison selects. Two backends that differ on
        // NaN differ exactly where a model has already gone wrong and where the
        // difference is hardest to trace.
        CfgBinaryOp::Min => format!("if {left}<= {right}{{{left}}}else{{{right}}}"),
        CfgBinaryOp::Max => format!("if {left}>= {right}{{{left}}}else{{{right}}}"),
        CfgBinaryOp::Eq => format!("{left}== {right}"),
        CfgBinaryOp::Ne => format!("{left}!= {right}"),
        CfgBinaryOp::Lt => format!("{left}< {right}"),
        CfgBinaryOp::Le => format!("{left}<= {right}"),
        CfgBinaryOp::Gt => format!("{left}> {right}"),
        CfgBinaryOp::Ge => format!("{left}>= {right}"),
        CfgBinaryOp::And => format!("{left}&& {right}"),
        CfgBinaryOp::Or => format!("{left}|| {right}"),
    }
}

/// Whether substituting a constant is no larger than binding it once.
///
/// A compact binding costs `let `, `=`, `;\n`, the expression, and its name.
/// Every use also costs the name. Assuming the
/// shortest possible one-byte name makes this conservative: when it returns
/// true, substitution cannot increase generated source after dense naming.
fn constant_inlining_saves_source(expression_len: usize, uses: usize) -> bool {
    if uses == 0 {
        return false;
    }
    uses.saturating_mul(expression_len)
        <= 8usize.saturating_add(expression_len).saturating_add(uses)
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_FIXED_LANE_WIDTH, RUNTIME_PRELUDE, compact_local_name, constant_inlining_saves_source,
        lane_type_name, real_literal,
    };

    #[test]
    fn compact_local_names_are_stable_across_base_26_boundaries() {
        assert_eq!(compact_local_name(0), "A");
        assert_eq!(compact_local_name(25), "Z");
        assert_eq!(compact_local_name(26), "AA");
        assert_eq!(compact_local_name(51), "AZ");
        assert_eq!(compact_local_name(52), "BA");
        assert_eq!(compact_local_name(701), "ZZ");
        assert_eq!(compact_local_name(702), "AAA");
    }

    #[test]
    fn constant_inlining_uses_a_conservative_source_cost() {
        // A six-byte `0.0f64` is smaller through two uses, even if the binding
        // would have had the shortest possible local name.
        assert!(constant_inlining_saves_source(6, 1));
        assert!(constant_inlining_saves_source(6, 2));
        assert!(!constant_inlining_saves_source(6, 3));
        assert!(!constant_inlining_saves_source(6, 4));

        // A longer literal is no larger through two uses.
        assert!(constant_inlining_saves_source(10, 1));
        assert!(constant_inlining_saves_source(10, 2));
        assert!(!constant_inlining_saves_source(10, 3));
        assert!(!constant_inlining_saves_source(6, 0));
    }

    #[test]
    fn real_literals_choose_the_shortest_exact_spelling() {
        assert_eq!(real_literal(0.0), "0f64");
        assert_eq!(real_literal(1.0), "1f64");
        assert_eq!(real_literal(1e20), "1e20f64");

        for value in [
            -0.0,
            0.1,
            -1.234_567_890_123_456_7,
            f64::MIN_POSITIVE,
            f64::MAX,
        ] {
            let literal = real_literal(value);
            let parsed: f64 = literal
                .strip_suffix("f64")
                .expect("generated literal carries its type")
                .parse()
                .expect("generated literal parses as f64");
            assert_eq!(parsed.to_bits(), value.to_bits(), "{literal}");
        }
    }

    #[test]
    fn packed_lane_widths_select_fixed_types_and_retain_a_bounded_fallback() {
        assert_eq!(lane_type_name(2), "L2");
        assert_eq!(lane_type_name(3), "L3");
        assert_eq!(lane_type_name(MAX_FIXED_LANE_WIDTH), "L32");
        assert_eq!(lane_type_name(MAX_FIXED_LANE_WIDTH + 1), "Lanes");

        let fixed = RUNTIME_PRELUDE
            .split("macro_rules! define_fixed_lanes")
            .nth(1)
            .and_then(|source| source.split("define_fixed_lanes!(L2").next())
            .expect("standalone prelude contains fixed lane definitions");
        assert!(!fixed.contains("while "));
        assert!(
            !fixed
                .lines()
                .any(|line| line.trim_start().starts_with("for "))
        );
    }
}
