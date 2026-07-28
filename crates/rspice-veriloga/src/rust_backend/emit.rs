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
//! A block parameter is declared before the construct that merges into it and
//! assigned on each incoming edge — `let p; if c { p = a } else { p = b }` for a
//! diamond, `let mut p = init;` reassigned at the back edge for a loop. Rust's
//! definite-assignment analysis then checks, for free, the property SSA was
//! maintaining by construction.
//!
//! ## Everything is `f64`
//!
//! Including predicates, as `0.0` and `1.0`. The alternative — `bool` for
//! comparisons — needs a conversion wherever a derivative multiplies by one,
//! which is every `min` and every `max`. Uniform typing keeps the emitted
//! arithmetic identical to what [`super::super::canonical_ir::cfg_eval`]
//! computes, which is what makes the interpreter a usable oracle for this
//! output.
//!
//! ## Derivatives are arrays, and that is where the size went
//!
//! A packed derivative emits as the `Lanes<N>` newtype from [`RUNTIME_PRELUDE`]
//! over the lanes its shape names, and the elementwise rules emit as `a + b` and
//! `a * s` — one line each, whatever `N` is. That is the whole reason the IR
//! packs: a scalarised derivative costs a line per lane, and the wide MOSFETs
//! carry a hundred thousand of them. The newtype exists so those lines are
//! operators rather than calls, which is worth another third of the bytes;
//! `rustc` promotes the small fixed array inside it to registers, so none of
//! this is a loop at run time.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write;

use crate::canonical_ir::cfg::{CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind};
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
    pub node_potentials: String,
    pub branch_flows: String,
    pub branch_unknown_flows: String,
    pub temperature: String,
    pub thermal_voltage: String,
    pub multiplicity: String,
    pub time: String,
    pub ddt: String,
    pub ddt_scale: String,
    pub idt: String,
    pub idt_scale: String,
    /// Called as `analysis("dc")`.
    pub analysis: String,
    /// Called as `simparam("gmin", fallback)`.
    pub simparam: String,
    /// Called as `limit(operator, proposed, candidate)`.
    pub limit: String,
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
            node_potentials: "node_potentials".into(),
            branch_flows: "branch_flows".into(),
            branch_unknown_flows: "branch_unknown_flows".into(),
            temperature: "temperature".into(),
            thermal_voltage: "thermal_voltage".into(),
            multiplicity: "multiplicity".into(),
            time: "time".into(),
            ddt: "ddt".into(),
            ddt_scale: "ddt_scale".into(),
            idt: "idt".into(),
            idt_scale: "idt_scale".into(),
            analysis: "analysis".into(),
            simparam: "simparam".into(),
            limit: "limit".into(),
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
        inlined: HashMap::new(),
        wanted: outputs.iter().copied().collect(),
        captured: BTreeMap::new(),
    };
    emitter.plan_inlining(outputs);
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

/// Helpers the emitted arithmetic calls. Include once per generated module.
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
#[derive(Clone, Copy)]
struct Lanes<const N: usize>([f64; N]);

impl<const N: usize> core::ops::Add for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] + rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Sub for Lanes<N> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] - rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

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
    fn index(&self, index: usize) -> &f64 {
        &self.0[index]
    }
}
"#;

struct Emitter<'a> {
    function: &'a CfgFunction,
    bindings: &'a EmitBindings,
    source: String,
    declared: HashSet<ValueId>,
    loop_headers: HashSet<BlockId>,
    /// Immediate post-dominators, which is what a diamond's join is.
    post_dominators: Vec<Option<BlockId>>,
    /// Values emitted at their use rather than bound to a name.
    ///
    /// A compact model is mostly values read exactly once, and binding each of
    /// them costs a line of generated Rust that says nothing. Substituting them
    /// into their one consumer is what the tier this replaces was doing to stay
    /// small, and without it the emitted source is several times larger for no
    /// difference in the compiled code.
    inlined: HashMap<ValueId, String>,
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

impl Emitter<'_> {
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
        // The one reader, recorded as it is counted: searching for it per
        // candidate would be quadratic, and these graphs run to hundreds of
        // thousands of values.
        let mut reader_of: Vec<Option<ValueId>> = vec![None; self.function.values.len()];
        for value in &self.function.values {
            // A widen names its operand once per lane of the result, so
            // substituting an expression into it would emit that expression N
            // times. Counting it twice keeps it bound.
            let weight = usize::from(matches!(value.kind, CfgValueKind::LaneWiden { .. })) + 1;
            for operand in value.kind.operands() {
                uses[usize::from(operand)] += weight;
                reader_of[usize::from(operand)] = Some(value.id);
            }
        }
        for block in &self.function.blocks {
            for instruction in &block.instructions {
                block_of[usize::from(instruction.result)] = Some(block.id);
            }
            match &block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        uses[usize::from(*arg)] += 1;
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    uses[usize::from(*condition)] += 1;
                    for arg in then_args.iter().chain(else_args) {
                        uses[usize::from(*arg)] += 1;
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        // An output is read by the caller, which no operand list records.
        for output in outputs {
            uses[usize::from(*output)] += 2;
        }

        for block in &self.function.blocks {
            for instruction in &block.instructions {
                let result = instruction.result;
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
            if defined.contains(&value.id) {
                continue;
            }
            let expression = self.expression(value.id)?;
            self.line(1, &format!("let {} = {expression};", value_name(value.id)));
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
            // A loop header's carried variables are assigned by the edge into
            // the loop, which is emitted here — before `emit_loop` gets a
            // chance to declare them, and outside any arm that would scope them
            // away.
            self.declare_loop_targets(current, depth);
            match &self.function.block(current).terminator {
                CfgTerminator::Return | CfgTerminator::Unset => return Ok(()),
                CfgTerminator::Jump { target, args } => {
                    let (target, args) = (*target, args.clone());
                    self.pass_arguments(target, &args, depth);
                    current = target;
                }
                CfgTerminator::Branch { .. } => {
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
        // Declared before the arms so both may assign it, which is also how
        // Rust's definite-assignment check ends up doing the SSA verification.
        if let Some(join) = join {
            for param in &self.function.block(join).params {
                self.declare(*param, depth);
            }
        }

        self.line(depth, &format!("if {} != 0.0 {{", value_name(condition)));
        self.pass_arguments(then_target, &then_args, depth + 1);
        self.block(then_target, join, depth + 1)?;
        self.line(depth, "} else {");
        self.pass_arguments(else_target, &else_args, depth + 1);
        self.block(else_target, join, depth + 1)?;
        self.line(depth, "}");
        Ok(join)
    }

    /// `loop { .. if !c { break } .. }` for the header/body/exit triple the
    /// lowering builds.
    fn emit_loop(&mut self, header: BlockId, depth: usize) -> Result<(), EmitError> {
        // The carried variables enter with the values the edge into the loop
        // supplied, and are reassigned on the back edge.
        for param in &self.function.block(header).params.clone() {
            self.declare_mutable(*param, depth);
        }
        self.line(depth, "loop {");
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

        self.line(
            depth + 1,
            &format!("if {} == 0.0 {{", value_name(condition)),
        );
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
            self.capture(*param, depth);
        }
        for instruction in &self.function.block(block).instructions.clone() {
            let expression = self.expression(instruction.result)?;
            // Parenthesised because it is about to be dropped into the middle of
            // another expression, where precedence is not ours to reason about.
            if let Some(slot) = self.inlined.get_mut(&instruction.result) {
                *slot = format!("({expression})");
                continue;
            }
            self.line(
                depth,
                &format!("let {} = {expression};", value_name(instruction.result)),
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
        let name = format!("out{}", usize::from(value));
        self.line(depth, &format!("{name} = {};", value_name(value)));
        self.captured.insert(value, name);
    }

    fn capture_declarations(&self) -> String {
        let mut out = String::new();
        for value in self.captured.keys() {
            let (declared_type, initial) = match self.function.lanes_of(*value) {
                Some(lanes) => (
                    format!("Lanes<{}>", lanes.len()),
                    format!("Lanes([0.0; {}])", lanes.len()),
                ),
                None => ("f64".to_string(), "0.0".to_string()),
            };
            let _ = writeln!(
                out,
                "    let mut out{}: {declared_type} = {initial};",
                usize::from(*value)
            );
        }
        out
    }

    fn output_name(&self, value: ValueId) -> String {
        self.captured
            .get(&value)
            .cloned()
            .unwrap_or_else(|| value_name(value))
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
        let params = self.function.block(target).params.clone();
        if params.is_empty() {
            return;
        }
        let clobbers = args.iter().any(|argument| params.contains(argument));
        if !clobbers {
            for (param, argument) in params.iter().zip(args) {
                self.line(
                    depth,
                    &format!("{} = {};", value_name(*param), value_name(*argument)),
                );
            }
            return;
        }
        for (index, argument) in args.iter().enumerate() {
            self.line(
                depth,
                &format!("let edge{index} = {};", value_name(*argument)),
            );
        }
        for (index, param) in params.iter().enumerate() {
            self.line(depth, &format!("{} = edge{index};", value_name(*param)));
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

    fn declare(&mut self, value: ValueId, depth: usize) {
        if self.declared.insert(value) {
            let declared_type = self.type_name(value);
            self.line(
                depth,
                &format!("let {}: {declared_type};", value_name(value)),
            );
        }
    }

    fn declare_mutable(&mut self, value: ValueId, depth: usize) {
        if self.declared.insert(value) {
            let (declared_type, initial) = match self.function.lanes_of(value) {
                Some(lanes) => (
                    format!("Lanes<{}>", lanes.len()),
                    format!("Lanes([0.0; {}])", lanes.len()),
                ),
                None => ("f64".to_string(), "0.0".to_string()),
            };
            self.line(
                depth,
                &format!(
                    "let mut {}: {declared_type} = {initial};",
                    value_name(value)
                ),
            );
        }
    }

    fn type_name(&self, value: ValueId) -> String {
        match self.function.lanes_of(value) {
            Some(lanes) => format!("Lanes<{}>", lanes.len()),
            None => "f64".to_string(),
        }
    }

    fn line(&mut self, depth: usize, text: &str) {
        for _ in 0..depth {
            self.source.push_str("    ");
        }
        self.source.push_str(text);
        self.source.push('\n');
    }

    /// How a reader names `value`: its binding, or its whole expression when it
    /// was chosen for substitution.
    fn operand(&self, value: ValueId) -> String {
        match self.inlined.get(&value) {
            Some(expression) if !expression.is_empty() => expression.clone(),
            _ => value_name(value),
        }
    }

    fn expression(&self, value: ValueId) -> Result<String, EmitError> {
        let bindings = self.bindings;
        Ok(match &self.function.value(value).kind {
            CfgValueKind::RealConstant(constant) => real_literal(*constant),
            CfgValueKind::BooleanConstant(constant) => {
                if *constant {
                    "1.0f64".into()
                } else {
                    "0.0f64".into()
                }
            }
            CfgValueKind::BlockParameter => value_name(value),
            CfgValueKind::Parameter(parameter) => {
                format!("{}[{}]", bindings.parameters, usize::from(*parameter))
            }
            CfgValueKind::ParameterGiven(parameter) => format!(
                "if {}[{}] {{ 1.0 }} else {{ 0.0 }}",
                bindings.parameter_given,
                usize::from(*parameter)
            ),
            CfgValueKind::Temperature => bindings.temperature.clone(),
            CfgValueKind::ThermalVoltage => bindings.thermal_voltage.clone(),
            CfgValueKind::Multiplicity => bindings.multiplicity.clone(),
            CfgValueKind::Time => bindings.time.clone(),
            CfgValueKind::Analysis(name) => format!("{}(\"{name}\")", bindings.analysis),
            CfgValueKind::SimParam { name, fallback } => format!(
                "{}(\"{name}\", {})",
                bindings.simparam,
                self.operand(*fallback)
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
                usize::from(*operator),
                self.operand(*input)
            ),
            CfgValueKind::DdtScale => format!("{}()", bindings.ddt_scale),
            CfgValueKind::Idt { operator, input, ic } => format!(
                "{}({}, {}, {})",
                bindings.idt,
                usize::from(*operator),
                self.operand(*input),
                self.operand(*ic)
            ),
            CfgValueKind::IdtScale => format!("{}()", bindings.idt_scale),
            CfgValueKind::Limit {
                operator,
                proposed,
                candidate,
                ..
            } => format!(
                "{}({}, {}, {})",
                bindings.limit,
                usize::from(*operator),
                self.operand(*proposed),
                self.operand(*candidate)
            ),
            CfgValueKind::LimitPrevious { operator, proposed } => format!(
                "{}({}, {})",
                bindings.limit_previous,
                usize::from(*operator),
                self.operand(*proposed)
            ),
            CfgValueKind::Ddx { .. } => return Err(EmitError::UnresolvedDdx(value)),
            CfgValueKind::Unary { op, input } => unary(*op, &self.operand(*input)),
            CfgValueKind::Binary { op, left, right } => {
                binary(*op, &self.operand(*left), &self.operand(*right))
            }
            CfgValueKind::LaneSplat(constant) => {
                let width = self.function.lanes_of(value).map_or(0, <[u32]>::len);
                format!("Lanes([{}; {width}])", real_literal(*constant))
            }
            // The one packed form that is written out rather than called: which
            // lane lands where is a per-value permutation, not an operation.
            CfgValueKind::LaneWiden { input } => {
                let source = self.function.lanes_of(*input).unwrap_or(&[]);
                let name = self.operand(*input);
                let elements: Vec<String> = self
                    .function
                    .lanes_of(value)
                    .unwrap_or(&[])
                    .iter()
                    .map(|lane| match source.iter().position(|held| held == lane) {
                        Some(position) => format!("{name}[{position}]"),
                        None => "0.0".to_string(),
                    })
                    .collect();
                format!("Lanes([{}])", elements.join(", "))
            }
            CfgValueKind::LaneBinary { op, left, right } => {
                let symbol = if matches!(op, CfgBinaryOp::Sub) { "-" } else { "+" };
                format!(
                    "{} {symbol} {}",
                    self.operand(*left),
                    self.operand(*right)
                )
            }
            CfgValueKind::LaneScalar { op, input, scalar } => {
                let symbol = if matches!(op, CfgBinaryOp::Div) { "/" } else { "*" };
                format!(
                    "{} {symbol} {}",
                    self.operand(*input),
                    self.operand(*scalar)
                )
            }
            CfgValueKind::LaneExtract { input, lane } => {
                let position = self.function.lane_position(*input, *lane).unwrap_or(0);
                format!("{}[{position}]", self.operand(*input))
            }
            CfgValueKind::Staged { slot } => format!("{}[{slot}]", bindings.staged),
        })
    }
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

fn value_name(value: ValueId) -> String {
    format!("v{}", usize::from(value))
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
    let mut text = String::new();
    // `{:e}` is the shortest form that round-trips, which matters: a truncated
    // constant is a wrong model, not a smaller one.
    let _ = write!(text, "{value:e}");
    format!("{text}f64")
}

fn unary(op: CfgUnaryOp, input: &str) -> String {
    match op {
        CfgUnaryOp::Neg => format!("-{input}"),
        CfgUnaryOp::Not => format!("if {input} == 0.0 {{ 1.0 }} else {{ 0.0 }}"),
        CfgUnaryOp::Exp => format!("{input}.exp()"),
        CfgUnaryOp::LimExp => format!("rspice_limexp({input})"),
        CfgUnaryOp::LimitedExp => format!("rspice_limited_exp({input})"),
        CfgUnaryOp::LimitedExpDerivative => format!("rspice_limited_exp_derivative({input})"),
        CfgUnaryOp::Ln => format!("{input}.ln()"),
        CfgUnaryOp::Sqrt => format!("{input}.sqrt()"),
        CfgUnaryOp::Abs => format!("{input}.abs()"),
        CfgUnaryOp::Sin => format!("{input}.sin()"),
        CfgUnaryOp::Cos => format!("{input}.cos()"),
        CfgUnaryOp::Tan => format!("{input}.tan()"),
        CfgUnaryOp::Sinh => format!("{input}.sinh()"),
        CfgUnaryOp::Cosh => format!("{input}.cosh()"),
        CfgUnaryOp::Tanh => format!("{input}.tanh()"),
        CfgUnaryOp::Atan => format!("{input}.atan()"),
        CfgUnaryOp::Asinh => format!("{input}.asinh()"),
        CfgUnaryOp::Floor => format!("{input}.floor()"),
        CfgUnaryOp::Ceil => format!("{input}.ceil()"),
    }
}

fn binary(op: CfgBinaryOp, left: &str, right: &str) -> String {
    match op {
        CfgBinaryOp::Add => format!("{left} + {right}"),
        CfgBinaryOp::Sub => format!("{left} - {right}"),
        CfgBinaryOp::Mul => format!("{left} * {right}"),
        CfgBinaryOp::Div => format!("{left} / {right}"),
        CfgBinaryOp::Mod => format!("{left} % {right}"),
        CfgBinaryOp::Pow => format!("{left}.powf({right})"),
        // Written out rather than `f64::min`, which disagrees with the
        // interpreter when one operand is NaN: it returns the other, this
        // returns whichever the comparison selects. Two backends that differ on
        // NaN differ exactly where a model has already gone wrong and where the
        // difference is hardest to trace.
        CfgBinaryOp::Min => format!("if {left} <= {right} {{ {left} }} else {{ {right} }}"),
        CfgBinaryOp::Max => format!("if {left} >= {right} {{ {left} }} else {{ {right} }}"),
        CfgBinaryOp::Eq => predicate(&format!("{left} == {right}")),
        CfgBinaryOp::Ne => predicate(&format!("{left} != {right}")),
        CfgBinaryOp::Lt => predicate(&format!("{left} < {right}")),
        CfgBinaryOp::Le => predicate(&format!("{left} <= {right}")),
        CfgBinaryOp::Gt => predicate(&format!("{left} > {right}")),
        CfgBinaryOp::Ge => predicate(&format!("{left} >= {right}")),
        CfgBinaryOp::And => predicate(&format!("{left} != 0.0 && {right} != 0.0")),
        CfgBinaryOp::Or => predicate(&format!("{left} != 0.0 || {right} != 0.0")),
    }
}

fn predicate(test: &str) -> String {
    format!("if {test} {{ 1.0 }} else {{ 0.0 }}")
}
