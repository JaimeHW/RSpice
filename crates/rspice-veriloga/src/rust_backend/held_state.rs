//! Which variables a model reads back from an earlier evaluation hold no
//! history a periodic steady state could miss.
//!
//! A variable kept between evaluations (see `super::harmonic_balance`) is
//! read back on a path that reaches the read without assigning it. Most such
//! paths in the shipped compact models cannot run: the variable is assigned
//! under `if (CTAT != 0)` and read under the same parameter test, or set once
//! in `@(initial_step)` from parameters and read by every evaluation after.
//! The SSA form cannot see either, because it does not correlate one branch
//! with another. This module does, over the conditions an instance's
//! parameters fix.
//!
//! # Conditions an instance fixes
//!
//! A value is *instance-static* when it is computed from parameters,
//! `$param_given`, `$port_connected`, the ambient temperature and
//! `$simparam` alone, through pure operators and through merges whose
//! incoming edges no two evaluations of one instance can choose differently.
//! An instance fixes every such branch condition, so one assignment σ of its
//! atoms (canonical comparisons and other static booleans) decides every
//! instance-static branch, at every evaluation. `@(initial_step)` and
//! `@(final_step)` are two atoms of their own, true at one evaluation each.
//! A branch on anything else may go either way at each evaluation, except
//! that in a body without loops, where one evaluation computes each value
//! once, two branches on one test of the same values go the same way: each
//! such repeated test is an atom of its own for that evaluation, and every
//! formula that compares evaluations, or asks what σ fixes, has those atoms
//! quantified out.
//!
//! The formula `P(B)` over those atoms holds for every σ under which block
//! `B` may run. It is exact in σ, with every other branch free, and is held
//! as a reduced ordered binary decision diagram, so formulas are compared by
//! identity.
//!
//! # A variable that keeps no history
//!
//! For one variable, `R` is the formula under which the kept value, read at
//! some evaluation in the steady state (neither initial nor final step), can
//! reach a residual, an activation or a branch condition: found backwards
//! from those roots through every use, and through a merge only along the
//! edge that passes it. `A` is the formula under which some evaluation
//! before the final step assigns the variable. Both are over-approximations,
//! so a proof from them is a proof.
//!
//! * **Assigned before every read the parameters allow** when `R ∧ A` is
//!   false: for each instance, either no steady-state evaluation reads the
//!   kept value, or no evaluation ever assigns it and the kept value is the
//!   one every analysis starts from.
//! * **Set once at the initial step** when no steady-state evaluation
//!   assigns it, every value the initial step may assign is
//!   instance-static, and σ with the step's own atom decides which: the kept
//!   value is then a function of the parameters from the first evaluation
//!   on, which harmonic balance reproduces by evaluating its own initial
//!   step once before it samples.
//!
//! Everything else keeps a value that depends on the waveform's past.

use crate::canonical_ir::ValueId;
use crate::canonical_ir::cfg::{
    CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind, CfgValueType,
};
use rspice_veriloga_runtime::SimulationParameter;
use std::collections::{HashMap, HashSet};

/// How a variable the body reads back and assigns behaves across
/// evaluations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HeldClass {
    /// Every read the instance's parameters allow sees this evaluation's
    /// assignment, or the variable is never assigned.
    AssignedBeforeRead,
    /// Assigned only at the initial step, from parameters.
    SetAtInitialStep,
    /// Its kept value depends on the waveform's past.
    History,
}

/// The atom for `@(initial_step)`.
const INITIAL_STEP: u32 = 0;
/// The atom for `@(final_step)`.
const FINAL_STEP: u32 = 1;
/// Atoms reserved for the comparisons of one subject.
const SUBJECT_RUN: u32 = 32;
/// Diagram nodes a model may build before the proof gives up and every
/// variable it was asked about keeps history.
const NODE_BUDGET: usize = 8_000_000;
const FALSE: u32 = 0;
const TRUE: u32 = 1;

#[derive(Debug)]
struct Exhausted;

type Proved<T> = Result<T, Exhausted>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Apply {
    And,
    Or,
    Xor,
}

/// A reduced ordered binary decision diagram over atoms `0..`, with node 0
/// false and node 1 true.
struct Diagram {
    nodes: Vec<(u32, u32, u32)>,
    unique: HashMap<(u32, u32, u32), u32>,
    applied: HashMap<(Apply, u32, u32), u32>,
}

impl Diagram {
    fn new() -> Self {
        Self {
            nodes: vec![(u32::MAX, FALSE, FALSE), (u32::MAX, TRUE, TRUE)],
            unique: HashMap::new(),
            applied: HashMap::new(),
        }
    }

    fn node(&mut self, atom: u32, low: u32, high: u32) -> Proved<u32> {
        if low == high {
            return Ok(low);
        }
        if let Some(&node) = self.unique.get(&(atom, low, high)) {
            return Ok(node);
        }
        if self.nodes.len() >= NODE_BUDGET {
            return Err(Exhausted);
        }
        let node = u32::try_from(self.nodes.len()).map_err(|_| Exhausted)?;
        self.nodes.push((atom, low, high));
        self.unique.insert((atom, low, high), node);
        Ok(node)
    }

    fn atom(&mut self, atom: u32) -> Proved<u32> {
        self.node(atom, FALSE, TRUE)
    }

    fn not(&mut self, f: u32) -> Proved<u32> {
        self.apply(Apply::Xor, f, TRUE)
    }

    fn and(&mut self, a: u32, b: u32) -> Proved<u32> {
        self.apply(Apply::And, a, b)
    }

    fn or(&mut self, a: u32, b: u32) -> Proved<u32> {
        self.apply(Apply::Or, a, b)
    }

    fn apply(&mut self, op: Apply, a: u32, b: u32) -> Proved<u32> {
        let terminal = match op {
            Apply::And if a == FALSE || b == FALSE => Some(FALSE),
            Apply::And if a == TRUE => Some(b),
            Apply::And if b == TRUE || a == b => Some(a),
            Apply::Or if a == TRUE || b == TRUE => Some(TRUE),
            Apply::Or if a == FALSE => Some(b),
            Apply::Or if b == FALSE || a == b => Some(a),
            Apply::Xor if a == b => Some(FALSE),
            Apply::Xor if a == FALSE => Some(b),
            Apply::Xor if b == FALSE => Some(a),
            _ => None,
        };
        if let Some(result) = terminal {
            return Ok(result);
        }
        let (a, b) = if a <= b { (a, b) } else { (b, a) };
        if let Some(&result) = self.applied.get(&(op, a, b)) {
            return Ok(result);
        }
        let (atom_a, low_a, high_a) = self.nodes[a as usize];
        let (atom_b, low_b, high_b) = self.nodes[b as usize];
        let atom = atom_a.min(atom_b);
        let (low_a, high_a) = if atom_a == atom {
            (low_a, high_a)
        } else {
            (a, a)
        };
        let (low_b, high_b) = if atom_b == atom {
            (low_b, high_b)
        } else {
            (b, b)
        };
        let low = self.apply(op, low_a, low_b)?;
        let high = self.apply(op, high_a, high_b)?;
        let result = self.node(atom, low, high)?;
        self.applied.insert((op, a, b), result);
        Ok(result)
    }

    /// `f` with `atom` fixed at `value`.
    fn restrict(&mut self, f: u32, atom: u32, value: bool) -> Proved<u32> {
        let mut memo = HashMap::new();
        self.restrict_memo(f, atom, value, &mut memo)
    }

    fn restrict_memo(
        &mut self,
        f: u32,
        atom: u32,
        value: bool,
        memo: &mut HashMap<u32, u32>,
    ) -> Proved<u32> {
        let (node_atom, low, high) = self.nodes[f as usize];
        if f <= TRUE || node_atom > atom {
            return Ok(f);
        }
        if node_atom == atom {
            return Ok(if value { high } else { low });
        }
        if let Some(&result) = memo.get(&f) {
            return Ok(result);
        }
        let low = self.restrict_memo(low, atom, value, memo)?;
        let high = self.restrict_memo(high, atom, value, memo)?;
        let result = self.node(node_atom, low, high)?;
        memo.insert(f, result);
        Ok(result)
    }

    /// `f` with `atom` free: true for some value of it.
    fn exists(&mut self, f: u32, atom: u32) -> Proved<u32> {
        let low = self.restrict(f, atom, false)?;
        let high = self.restrict(f, atom, true)?;
        self.or(low, high)
    }

    /// `f` at an evaluation that is neither the initial nor the final step.
    fn steady(&mut self, f: u32) -> Proved<u32> {
        let f = self.restrict(f, INITIAL_STEP, false)?;
        self.restrict(f, FINAL_STEP, false)
    }

    /// `f` with both step atoms free.
    fn any_step(&mut self, f: u32) -> Proved<u32> {
        let f = self.exists(f, INITIAL_STEP)?;
        self.exists(f, FINAL_STEP)
    }
}

/// The structural identity of an instance-static value, so two computations
/// of `AB == 0` at different places are one atom.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Key {
    Real(u64),
    Boolean(bool),
    Integer(i32),
    Parameter(crate::canonical_ir::ParamId),
    ParameterGiven(crate::canonical_ir::ParamId),
    PortConnected(u32),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    SimParam(SimulationParameter),
    SimParamPresent(SimulationParameter),
    Unary(CfgUnaryOp, u32),
    Binary(CfgBinaryOp, u32, u32),
    Select(u32, u32, u32),
    /// Any other pure operator, by its whole description: two of them are
    /// one only when they are written alike.
    Other(String),
    /// Any other operator an evaluation applies, over its operands' keys.
    Evaluated(String, Vec<u32>),
    /// A merge, which is its own identity.
    Merge(ValueId),
    /// A real used as a condition.
    Truthy(u32),
}

/// What a comparison atom is about: one value against constants, or two
/// values against each other (as their difference against zero).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Subject {
    Value(u32),
    Pair(u32, u32),
}

/// A comparison of a subject with a constant, the subject on the left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relation {
    Below,
    AtMost,
    Equal,
    AtLeast,
    Above,
}

impl Relation {
    fn flipped(self) -> Self {
        match self {
            Self::Below => Self::Above,
            Self::AtMost => Self::AtLeast,
            Self::Equal => Self::Equal,
            Self::AtLeast => Self::AtMost,
            Self::Above => Self::Below,
        }
    }

    fn holds(self, subject: f64, constant: f64) -> bool {
        match self {
            Self::Below => subject < constant,
            Self::AtMost => subject <= constant,
            Self::Equal => subject == constant,
            Self::AtLeast => subject >= constant,
            Self::Above => subject > constant,
        }
    }
}

/// The relation a comparison operator states of its left operand. `Ne` is
/// the negation of the `Equal` atom.
fn relation(op: CfgBinaryOp) -> Relation {
    match op {
        CfgBinaryOp::Lt => Relation::Below,
        CfgBinaryOp::Le => Relation::AtMost,
        CfgBinaryOp::Gt => Relation::Above,
        CfgBinaryOp::Ge => Relation::AtLeast,
        _ => Relation::Equal,
    }
}

/// A value strictly between `low` and `high`, if a double lies there.
fn strictly_between(low: f64, high: f64) -> Option<f64> {
    let candidate = if low == f64::NEG_INFINITY && high == f64::INFINITY {
        0.0
    } else if low == f64::NEG_INFINITY {
        high - 1.0 - high.abs()
    } else if high == f64::INFINITY {
        low + 1.0 + low.abs()
    } else {
        low + (high - low) / 2.0
    };
    (candidate > low && candidate < high).then_some(candidate)
}

/// One CFG edge: its source and target blocks, the arguments it passes and
/// the branch condition, with the polarity, that selects it.
struct Edge<'a> {
    from: usize,
    to: usize,
    args: &'a [ValueId],
    condition: Option<(ValueId, bool)>,
}

/// The proof over one model's lowered body.
pub(super) struct HeldStateProof<'a> {
    function: &'a CfgFunction,
    diagram: Diagram,
    edges: Vec<Edge<'a>>,
    incoming: Vec<Vec<usize>>,
    /// Each block parameter's merge: the edges into its block with the
    /// argument each passes.
    merges: HashMap<ValueId, Vec<(usize, ValueId)>>,
    /// The merges each value is an argument of.
    merged_into: HashMap<ValueId, Vec<ValueId>>,
    instance_static: Vec<bool>,
    static_merges: HashSet<ValueId>,
    keys: HashMap<Key, u32>,
    key_of: Vec<Option<u32>>,
    atoms: HashMap<u32, u32>,
    truth: HashMap<(ValueId, bool), u32>,
    compared: HashMap<(CfgBinaryOp, ValueId, ValueId), u32>,
    /// Each static merge's incoming edges, with the reach that selects each
    /// as the round that proved it computed them.
    selections: HashMap<ValueId, Vec<(u32, ValueId)>>,
    /// What each comparison atom says about its subject.
    bounds: HashMap<u32, (Subject, Relation, f64)>,
    /// Each subject's run of atoms: its first atom and how many it holds.
    subject_runs: HashMap<Subject, (u32, u32)>,
    next_atom: u32,
    /// The assignments of the comparison atoms some real (or NaN) value of
    /// each subject allows: `x < 1` and `x == 1` never hold together.
    consistent: u32,
    reach: Vec<u32>,
    edge_reach: Vec<u32>,
    /// Per value, the σ under which it can reach a residual (see
    /// [`Self::compute_influence`]); empty until the first classification.
    influence: Vec<u32>,
    /// The atoms of conditions one evaluation decides, each tested by more
    /// than one branch ([`Self::correlate_repeated_conditions`]).
    evaluation_atoms: Vec<u32>,
    /// Each repeated condition's structural key and its atom.
    evaluation_atom_of: HashMap<u32, u32>,
    /// The structural key of every value an evaluation computes, from its
    /// operands' keys down to the values it reads.
    evaluation_key_of: Vec<Option<u32>>,
}

impl<'a> HeldStateProof<'a> {
    /// Build the reach formulas, or `None` when the diagram outgrows its
    /// budget.
    pub(super) fn new(function: &'a CfgFunction) -> Option<Self> {
        let mut proof = Self::skeleton(function);
        proof.solve().ok()?;
        proof.correlate_repeated_conditions().ok()?;
        Some(proof)
    }

    /// Give each condition an evaluation decides, and more than one branch
    /// tests, an atom of its own, so the branches agree within one
    /// evaluation.
    ///
    /// BSIM-CMG 107 assigns `T3 = pow(T2, 2.0)` under `if ((F0-z2) >
    /// -0.3/T14)` and reads `T3` further down under the same test of the
    /// same values; with each test free, the read seemed to reach the kept
    /// `T3`. Two tests that compute one structure from the same values agree
    /// at every evaluation in a body without loops, where each value is
    /// computed once. A body with a loop keeps every such test free, and a
    /// test made once is free either way. The atoms describe one evaluation,
    /// so every formula compared across evaluations, and every selection
    /// tested for being fixed, has them quantified out
    /// ([`Self::across_evaluations`]).
    fn correlate_repeated_conditions(&mut self) -> Proved<()> {
        let order = self.reverse_postorder();
        let mut position = vec![usize::MAX; self.function.blocks.len()];
        for (rank, &block) in order.iter().enumerate() {
            position[block] = rank;
        }
        if self
            .edges
            .iter()
            .any(|edge| position[edge.to] <= position[edge.from])
        {
            return Ok(());
        }
        let mut tested: HashMap<u32, usize> = HashMap::new();
        let mut leaves = Vec::new();
        for block in &self.function.blocks {
            if let CfgTerminator::Branch { condition, .. } = &block.terminator {
                leaves.clear();
                self.evaluation_conditions(*condition, &mut leaves);
                for &leaf in &leaves {
                    let key = self.evaluation_key(leaf);
                    *tested.entry(key).or_default() += 1;
                }
            }
        }
        let mut repeated: Vec<u32> = tested
            .into_iter()
            .filter(|&(_, count)| count > 1)
            .map(|(key, _)| key)
            .collect();
        if repeated.is_empty() {
            return Ok(());
        }
        repeated.sort_unstable();
        for key in repeated {
            let atom = self.next_atom;
            self.next_atom = self.next_atom.checked_add(1).ok_or(Exhausted)?;
            self.evaluation_atom_of.insert(key, atom);
            self.evaluation_atoms.push(atom);
        }
        self.truth.clear();
        self.compute_reach()
    }

    /// The tests a branch on `condition` makes that an evaluation decides:
    /// the condition itself, or the operands of its `!`, `&&` and `||`.
    fn evaluation_conditions(&self, condition: ValueId, leaves: &mut Vec<ValueId>) {
        if self.instance_static[usize::from(condition)] {
            return;
        }
        match &self.function.value(condition).kind {
            CfgValueKind::Unary {
                op: CfgUnaryOp::Not,
                input,
            } => self.evaluation_conditions(*input, leaves),
            CfgValueKind::Binary {
                op: CfgBinaryOp::And | CfgBinaryOp::Or,
                left,
                right,
            } => {
                self.evaluation_conditions(*left, leaves);
                self.evaluation_conditions(*right, leaves);
            }
            CfgValueKind::Analysis(name)
                if is_step(name, "__rspice_initial_step")
                    || is_step(name, "__rspice_final_step") => {}
            _ => leaves.push(condition),
        }
    }

    /// The structural identity of a value one evaluation computes: an
    /// instance-static value's own key, a pure operator over its operands'
    /// identities, and anything else (a probe, a merge, a call) itself.
    fn evaluation_key(&mut self, root: ValueId) -> u32 {
        if let Some(key) = self.evaluation_key_of[usize::from(root)] {
            return key;
        }
        let function = self.function;
        let mut stack = vec![(root, false)];
        while let Some((value, expanded)) = stack.pop() {
            if self.evaluation_key_of[usize::from(value)].is_some() {
                continue;
            }
            let kind = &function.value(value).kind;
            if self.instance_static[usize::from(value)] {
                let key = self.key(value);
                self.evaluation_key_of[usize::from(value)] = Some(key);
                continue;
            }
            let pure = static_kind(kind) && !matches!(kind, CfgValueKind::BlockParameter);
            if pure && !expanded {
                stack.push((value, true));
                for operand in kind.operands() {
                    if self.evaluation_key_of[usize::from(operand)].is_none() {
                        stack.push((operand, false));
                    }
                }
                continue;
            }
            let operand =
                |proof: &Self, id: &ValueId| proof.evaluation_key_of[usize::from(*id)].unwrap_or(0);
            let key = if !pure {
                Key::Merge(value)
            } else {
                match kind {
                    CfgValueKind::Unary { op, input } => Key::Unary(*op, operand(self, input)),
                    CfgValueKind::Binary { op, left, right } => {
                        let (left, right) = (operand(self, left), operand(self, right));
                        match op {
                            CfgBinaryOp::Gt => Key::Binary(CfgBinaryOp::Lt, right, left),
                            CfgBinaryOp::Ge => Key::Binary(CfgBinaryOp::Le, right, left),
                            CfgBinaryOp::Add
                            | CfgBinaryOp::Mul
                            | CfgBinaryOp::Min
                            | CfgBinaryOp::Max
                            | CfgBinaryOp::Hypot
                            | CfgBinaryOp::Eq
                            | CfgBinaryOp::Ne
                            | CfgBinaryOp::And
                            | CfgBinaryOp::Or => Key::Binary(*op, left.min(right), left.max(right)),
                            _ => Key::Binary(*op, left, right),
                        }
                    }
                    CfgValueKind::Select {
                        condition,
                        then_value,
                        else_value,
                    } => Key::Select(
                        operand(self, condition),
                        operand(self, then_value),
                        operand(self, else_value),
                    ),
                    other => Key::Evaluated(
                        match other {
                            CfgValueKind::SumProductsDiv { .. } => "SumProductsDiv".to_owned(),
                            CfgValueKind::IntegerArithmetic { op, .. } => {
                                format!("IntegerArithmetic {op:?}")
                            }
                            CfgValueKind::IntegerBitwise { op, .. } => {
                                format!("IntegerBitwise {op:?}")
                            }
                            CfgValueKind::IntegerBitwiseNot { .. } => {
                                "IntegerBitwiseNot".to_owned()
                            }
                            // Written alike, operands included, or not one.
                            _ => format!("{other:?}"),
                        },
                        other
                            .operands()
                            .iter()
                            .map(|id| operand(self, id))
                            .collect(),
                    ),
                }
            };
            let key = self.intern(key);
            self.evaluation_key_of[usize::from(value)] = Some(key);
        }
        self.evaluation_key_of[usize::from(root)].unwrap_or(0)
    }

    /// `f` with every evaluation atom free: what holds across evaluations.
    fn across_evaluations(&mut self, mut f: u32) -> Proved<u32> {
        for position in 0..self.evaluation_atoms.len() {
            f = self.diagram.exists(f, self.evaluation_atoms[position])?;
        }
        Ok(f)
    }

    fn skeleton(function: &'a CfgFunction) -> Self {
        let blocks = function.blocks.len();
        let mut edges = Vec::new();
        let mut incoming = vec![Vec::new(); blocks];
        for (index, block) in function.blocks.iter().enumerate() {
            let outgoing: Vec<(usize, &[ValueId], Option<(ValueId, bool)>)> =
                match &block.terminator {
                    CfgTerminator::Jump { target, args } => {
                        vec![(usize::from(*target), args.as_slice(), None)]
                    }
                    CfgTerminator::Branch {
                        condition,
                        then_target,
                        then_args,
                        else_target,
                        else_args,
                    } => vec![
                        (
                            usize::from(*then_target),
                            then_args.as_slice(),
                            Some((*condition, true)),
                        ),
                        (
                            usize::from(*else_target),
                            else_args.as_slice(),
                            Some((*condition, false)),
                        ),
                    ],
                    CfgTerminator::Wait {
                        resume,
                        resume_args,
                        ..
                    } => vec![(usize::from(*resume), resume_args.as_slice(), None)],
                    CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
                };
            for (to, args, condition) in outgoing {
                incoming[to].push(edges.len());
                edges.push(Edge {
                    from: index,
                    to,
                    args,
                    condition,
                });
            }
        }
        let mut merges: HashMap<ValueId, Vec<(usize, ValueId)>> = HashMap::new();
        let mut merged_into: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
        for (index, edge) in edges.iter().enumerate() {
            for (&param, &arg) in function.blocks[edge.to].params.iter().zip(edge.args) {
                merges.entry(param).or_default().push((index, arg));
                merged_into.entry(arg).or_default().push(param);
            }
        }
        Self {
            function,
            diagram: Diagram::new(),
            incoming,
            merges,
            merged_into,
            instance_static: vec![false; function.values.len()],
            static_merges: HashSet::new(),
            keys: HashMap::new(),
            key_of: vec![None; function.values.len()],
            atoms: HashMap::new(),
            truth: HashMap::new(),
            compared: HashMap::new(),
            selections: HashMap::new(),
            bounds: HashMap::new(),
            subject_runs: HashMap::new(),
            next_atom: FINAL_STEP + 1,
            consistent: TRUE,
            reach: vec![FALSE; blocks],
            edge_reach: vec![FALSE; edges.len()],
            influence: Vec::new(),
            evaluation_atoms: Vec::new(),
            evaluation_atom_of: HashMap::new(),
            evaluation_key_of: vec![None; function.values.len()],
            edges,
        }
    }

    /// Alternate between the instance-static values and the reach formulas
    /// their conditions give, until no further merge is proved static.
    ///
    /// Each merge proved static can make the next one's arguments static, so
    /// a chain of them (PSP 103's `LGSOURCE_i` is `CLIP_LOW` of `LGS_i`, a
    /// merge of `jww`, itself a merge of `WE` and `JW_i`) is proved one link
    /// a pass. A merge is proved on reach formulas that over-approximate the
    /// paths, which is sound, so a pass proves every link it can on the
    /// reach it starts with before the newly static conditions refine it.
    /// Every pass that goes on proves a merge, so the passes end.
    fn solve(&mut self) -> Proved<()> {
        loop {
            self.mark_instance_static();
            self.truth.clear();
            self.compared.clear();
            self.compute_reach()?;
            self.consistent = self.consistency()?;
            let mut grew = false;
            while self.prove_static_merges()? {
                grew = true;
                self.mark_instance_static();
            }
            if !grew {
                return Ok(());
            }
        }
    }

    /// Prove static, on the present reach formulas, each merge whose
    /// arguments are all instance-static and whose selection σ fixes;
    /// whether any was.
    fn prove_static_merges(&mut self) -> Proved<bool> {
        let mut proved = Vec::new();
        for (&merge, incoming) in &self.merges {
            if !self.static_merges.contains(&merge)
                && incoming
                    .iter()
                    .all(|&(_, arg)| self.instance_static[usize::from(arg)])
            {
                proved.push(merge);
            }
        }
        proved.sort_unstable();
        let mut grew = false;
        for merge in proved {
            if self.selection_is_fixed(merge, true)? {
                let selection = self.merges[&merge]
                    .iter()
                    .map(|&(edge, arg)| (self.edge_reach[edge], arg))
                    .filter(|&(reach, _)| reach != FALSE)
                    .collect();
                self.selections.insert(merge, selection);
                self.static_merges.insert(merge);
                grew = true;
            }
        }
        Ok(grew)
    }

    /// Whether every pair of edges into `merge` that pass different values
    /// is never both possible for one σ: with the step atoms free on each
    /// side when `across_evaluations`, so the choice is one an instance
    /// fixes for every evaluation, or shared otherwise, so it is fixed
    /// within each evaluation.
    fn selection_is_fixed(&mut self, merge: ValueId, across_evaluations: bool) -> Proved<bool> {
        let incoming = self.merges[&merge].clone();
        let mut live = Vec::new();
        for (edge, arg) in incoming {
            let mut reach = self.edge_reach[edge];
            if reach == FALSE {
                continue;
            }
            // A choice one evaluation's own tests make is fixed by no σ.
            reach = self.across_evaluations(reach)?;
            if across_evaluations {
                reach = self.diagram.any_step(reach)?;
            }
            let identity = if self.instance_static[usize::from(arg)] {
                Some(self.key(arg))
            } else {
                None
            };
            live.push((reach, arg, identity));
        }
        for (first, &(reach_a, arg_a, key_a)) in live.iter().enumerate() {
            for &(reach_b, arg_b, key_b) in &live[first + 1..] {
                let same = arg_a == arg_b || (key_a.is_some() && key_a == key_b);
                if !same && self.possible(reach_a, reach_b)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn mark_instance_static(&mut self) {
        let values = &self.function.values;
        loop {
            let mut changed = false;
            for value in values {
                let index = usize::from(value.id);
                if self.instance_static[index] {
                    continue;
                }
                let proved = match &value.kind {
                    CfgValueKind::BlockParameter => self.static_merges.contains(&value.id),
                    kind if static_kind(kind) => kind
                        .operands()
                        .iter()
                        .all(|operand| self.instance_static[usize::from(*operand)]),
                    _ => false,
                };
                if proved {
                    self.instance_static[index] = true;
                    changed = true;
                }
            }
            if !changed {
                return;
            }
        }
    }

    /// `P` for every block and edge, by passes in reverse postorder until no
    /// block's formula widens: one pass for an acyclic body, a few more for
    /// each loop.
    fn compute_reach(&mut self) -> Proved<()> {
        let blocks = self.function.blocks.len();
        self.reach = vec![FALSE; blocks];
        self.edge_reach = vec![FALSE; self.edges.len()];
        let entry = usize::from(self.function.entry);
        let order = self.reverse_postorder();
        loop {
            let mut changed = false;
            for &block in &order {
                let mut reach = if block == entry { TRUE } else { FALSE };
                for position in 0..self.incoming[block].len() {
                    let edge = self.incoming[block][position];
                    let condition = match self.edges[edge].condition {
                        Some((value, polarity)) => self.truth(value, polarity)?,
                        None => TRUE,
                    };
                    let from = self
                        .diagram
                        .and(self.reach[self.edges[edge].from], condition)?;
                    self.edge_reach[edge] = from;
                    reach = self.diagram.or(reach, from)?;
                }
                if reach != self.reach[block] {
                    self.reach[block] = reach;
                    changed = true;
                }
            }
            if !changed {
                return Ok(());
            }
        }
    }

    fn reverse_postorder(&self) -> Vec<usize> {
        let blocks = self.function.blocks.len();
        let mut outgoing = vec![Vec::new(); blocks];
        for edge in &self.edges {
            outgoing[edge.from].push(edge.to);
        }
        let entry = usize::from(self.function.entry);
        let mut visited = vec![false; blocks];
        let mut postorder = Vec::with_capacity(blocks);
        let mut stack = vec![(entry, 0usize)];
        visited[entry] = true;
        while let Some((block, next)) = stack.pop() {
            if let Some(&successor) = outgoing[block].get(next) {
                stack.push((block, next + 1));
                if !visited[successor] {
                    visited[successor] = true;
                    stack.push((successor, 0));
                }
            } else {
                postorder.push(block);
            }
        }
        postorder.reverse();
        postorder
    }

    /// Whether some σ the comparisons' subjects allow satisfies both.
    fn possible(&mut self, a: u32, b: u32) -> Proved<bool> {
        let both = self.diagram.and(a, b)?;
        Ok(self.diagram.and(both, self.consistent)? != FALSE)
    }

    /// The conjunction, over every subject compared more than once, of the
    /// atom assignments one of its values gives: each point and open
    /// interval between the constants it is compared with, and NaN, which
    /// fails every comparison.
    fn consistency(&mut self) -> Proved<u32> {
        let mut subjects: HashMap<Subject, Vec<(u32, Relation, f64)>> = HashMap::new();
        for (&atom, &(subject, relation, constant)) in &self.bounds {
            subjects
                .entry(subject)
                .or_default()
                .push((atom, relation, constant));
        }
        let mut subjects: Vec<_> = subjects
            .into_iter()
            .filter(|(_, bounds)| bounds.len() > 1)
            .collect();
        subjects.sort_unstable_by_key(|(subject, _)| *subject);
        let mut consistent = TRUE;
        for (_, mut bounds) in subjects {
            bounds.sort_unstable_by_key(|&(atom, _, _)| atom);
            let mut constants: Vec<f64> = bounds
                .iter()
                .map(|&(_, _, constant)| constant)
                .filter(|constant| !constant.is_nan())
                .collect();
            constants.sort_unstable_by(f64::total_cmp);
            constants.dedup_by(|a, b| a == b);
            let mut witnesses = vec![f64::NAN];
            let mut below = f64::NEG_INFINITY;
            for &constant in &constants {
                if let Some(between) = strictly_between(below, constant) {
                    witnesses.push(between);
                }
                witnesses.push(constant);
                below = constant;
            }
            if let Some(between) = strictly_between(below, f64::INFINITY) {
                witnesses.push(between);
            }
            let mut allowed = FALSE;
            for witness in witnesses {
                let mut region = TRUE;
                for &(atom, relation, constant) in &bounds {
                    let literal = self.diagram.atom(atom)?;
                    let literal = if relation.holds(witness, constant) {
                        literal
                    } else {
                        self.diagram.not(literal)?
                    };
                    region = self.diagram.and(region, literal)?;
                }
                allowed = self.diagram.or(allowed, region)?;
            }
            consistent = self.diagram.and(consistent, allowed)?;
        }
        Ok(consistent)
    }

    /// An over-approximation of the σ under which `value` is `polarity`:
    /// exact for an instance-static condition, each step atom for its step,
    /// its evaluation atom for a test one evaluation repeats
    /// ([`Self::correlate_repeated_conditions`]), and true for anything else
    /// an evaluation decides.
    fn truth(&mut self, value: ValueId, polarity: bool) -> Proved<u32> {
        if let Some(&known) = self.truth.get(&(value, polarity)) {
            return Ok(known);
        }
        let kind = &self.function.value(value).kind;
        let result = if self.instance_static[usize::from(value)] {
            let exact = self.exact(value)?;
            if polarity {
                exact
            } else {
                self.diagram.not(exact)?
            }
        } else {
            match kind {
                CfgValueKind::Analysis(name) if is_step(name, "__rspice_initial_step") => {
                    let atom = self.diagram.atom(INITIAL_STEP)?;
                    if polarity {
                        atom
                    } else {
                        self.diagram.not(atom)?
                    }
                }
                CfgValueKind::Analysis(name) if is_step(name, "__rspice_final_step") => {
                    let atom = self.diagram.atom(FINAL_STEP)?;
                    if polarity {
                        atom
                    } else {
                        self.diagram.not(atom)?
                    }
                }
                CfgValueKind::Unary {
                    op: CfgUnaryOp::Not,
                    input,
                } => {
                    let input = *input;
                    self.truth(input, !polarity)?
                }
                CfgValueKind::Binary {
                    op: op @ (CfgBinaryOp::And | CfgBinaryOp::Or),
                    left,
                    right,
                } => {
                    let (op, left, right) = (*op, *left, *right);
                    let left = self.truth(left, polarity)?;
                    let right = self.truth(right, polarity)?;
                    if (op == CfgBinaryOp::And) == polarity {
                        self.diagram.and(left, right)?
                    } else {
                        self.diagram.or(left, right)?
                    }
                }
                _ if !self.evaluation_atoms.is_empty() => {
                    let key = self.evaluation_key(value);
                    match self.evaluation_atom_of.get(&key).copied() {
                        Some(atom) => {
                            let atom = self.diagram.atom(atom)?;
                            if polarity {
                                atom
                            } else {
                                self.diagram.not(atom)?
                            }
                        }
                        None => TRUE,
                    }
                }
                _ => TRUE,
            }
        };
        self.truth.insert((value, polarity), result);
        Ok(result)
    }

    /// The exact formula of an instance-static value used as a condition.
    fn exact(&mut self, value: ValueId) -> Proved<u32> {
        let function = self.function;
        let condition = function.value(value);
        if condition.value_type != CfgValueType::Boolean {
            let key = self.key(value);
            let key = self.intern(Key::Truthy(key));
            return self.atom_for(key);
        }
        match &condition.kind {
            CfgValueKind::BooleanConstant(constant) => Ok(if *constant { TRUE } else { FALSE }),
            CfgValueKind::Unary {
                op: CfgUnaryOp::Not,
                input,
            } => {
                let input = self.exact(*input)?;
                self.diagram.not(input)
            }
            CfgValueKind::Binary {
                op: op @ (CfgBinaryOp::And | CfgBinaryOp::Or),
                left,
                right,
            } => {
                let left = self.exact(*left)?;
                let right = self.exact(*right)?;
                if *op == CfgBinaryOp::And {
                    self.diagram.and(left, right)
                } else {
                    self.diagram.or(left, right)
                }
            }
            CfgValueKind::Binary {
                op:
                    op @ (CfgBinaryOp::Eq
                    | CfgBinaryOp::Ne
                    | CfgBinaryOp::Lt
                    | CfgBinaryOp::Le
                    | CfgBinaryOp::Gt
                    | CfgBinaryOp::Ge),
                left,
                right,
            } => self.compare(*op, *left, *right),
            _ => {
                let key = self.key(value);
                self.atom_for(key)
            }
        }
    }

    /// The exact formula of a comparison of two instance-static values. A
    /// static merge is expanded over its incoming edges, which σ selects one
    /// of, so `SWNQS_i == 4` is false when no edge passes 4 and the tests of
    /// one chain of `if`s exclude each other.
    fn compare(&mut self, op: CfgBinaryOp, left: ValueId, right: ValueId) -> Proved<u32> {
        if let Some(&known) = self.compared.get(&(op, left, right)) {
            return Ok(known);
        }
        let expanded = if let Some(selection) = self.selections.get(&left).cloned() {
            Some((selection, true))
        } else {
            self.selections
                .get(&right)
                .cloned()
                .map(|selection| (selection, false))
        };
        let result = if let Some((selection, on_left)) = expanded {
            let mut any = FALSE;
            for (reach, arg) in selection {
                let compared = if on_left {
                    self.compare(op, arg, right)?
                } else {
                    self.compare(op, left, arg)?
                };
                let both = self.diagram.and(reach, compared)?;
                any = self.diagram.or(any, both)?;
            }
            any
        } else if let (Some(a), Some(b)) = (self.constant(left), self.constant(right)) {
            let holds = match op {
                CfgBinaryOp::Eq => a == b,
                CfgBinaryOp::Ne => a != b,
                CfgBinaryOp::Lt => a < b,
                CfgBinaryOp::Le => a <= b,
                CfgBinaryOp::Gt => a > b,
                _ => a >= b,
            };
            if holds { TRUE } else { FALSE }
        } else {
            let (left_value, right_value) = (left, right);
            let (left, right) = (self.key(left), self.key(right));
            let key = match op {
                CfgBinaryOp::Eq | CfgBinaryOp::Ne => {
                    Key::Binary(CfgBinaryOp::Eq, left.min(right), left.max(right))
                }
                CfgBinaryOp::Gt => Key::Binary(CfgBinaryOp::Lt, right, left),
                CfgBinaryOp::Ge => Key::Binary(CfgBinaryOp::Le, right, left),
                _ => Key::Binary(op, left, right),
            };
            let key = self.intern(key);
            let bound = match (self.constant(left_value), self.constant(right_value)) {
                (None, Some(constant)) => Some((Subject::Value(left), relation(op), constant)),
                (Some(constant), None) => {
                    Some((Subject::Value(right), relation(op).flipped(), constant))
                }
                (None, None) if left <= right => {
                    Some((Subject::Pair(left, right), relation(op), 0.0))
                }
                (None, None) => Some((Subject::Pair(right, left), relation(op).flipped(), 0.0)),
                (Some(_), Some(_)) => None,
            };
            let atom = self.atom_about(key, bound)?;
            if op == CfgBinaryOp::Ne {
                self.diagram.not(atom)?
            } else {
                atom
            }
        };
        self.compared.insert((op, left, right), result);
        Ok(result)
    }

    fn constant(&self, value: ValueId) -> Option<f64> {
        match self.function.value(value).kind {
            CfgValueKind::RealConstant(constant) => Some(constant),
            CfgValueKind::IntegerConstant(constant) => Some(f64::from(constant)),
            CfgValueKind::BooleanConstant(constant) => Some(f64::from(u8::from(constant))),
            _ => None,
        }
    }

    fn atom_for(&mut self, key: u32) -> Proved<u32> {
        self.atom_about(key, None)
    }

    /// The atom for `key`. A comparison's atom is numbered in a run its
    /// subject reserves on first use, so the subject's consistency is a
    /// small diagram over adjacent atoms; one past the run's end is left
    /// unconstrained, which loses nothing but precision.
    fn atom_about(&mut self, key: u32, bound: Option<(Subject, Relation, f64)>) -> Proved<u32> {
        if let Some(&atom) = self.atoms.get(&key) {
            return self.diagram.atom(atom);
        }
        let reserve = |next: &mut u32, count: u32| -> Proved<u32> {
            let start = *next;
            *next = next.checked_add(count).ok_or(Exhausted)?;
            Ok(start)
        };
        let atom = match bound {
            Some(bound @ (subject, _, _)) => {
                let run = match self.subject_runs.get(&subject).copied() {
                    Some(run) => run,
                    None => {
                        let start = reserve(&mut self.next_atom, SUBJECT_RUN)?;
                        self.subject_runs.insert(subject, (start, 0));
                        (start, 0)
                    }
                };
                if run.1 < SUBJECT_RUN {
                    self.subject_runs.insert(subject, (run.0, run.1 + 1));
                    self.bounds.insert(run.0 + run.1, bound);
                    run.0 + run.1
                } else {
                    reserve(&mut self.next_atom, 1)?
                }
            }
            None => reserve(&mut self.next_atom, 1)?,
        };
        self.atoms.insert(key, atom);
        self.diagram.atom(atom)
    }

    fn intern(&mut self, key: Key) -> u32 {
        let next = u32::try_from(self.keys.len()).unwrap_or(u32::MAX);
        *self.keys.entry(key).or_insert(next)
    }

    /// The structural identity of an instance-static value, operands first.
    fn key(&mut self, root: ValueId) -> u32 {
        if let Some(key) = self.key_of[usize::from(root)] {
            return key;
        }
        let function = self.function;
        let mut stack = vec![(root, false)];
        while let Some((value, expanded)) = stack.pop() {
            if self.key_of[usize::from(value)].is_some() {
                continue;
            }
            let kind = &function.value(value).kind;
            let structural = !matches!(kind, CfgValueKind::BlockParameter)
                && self.instance_static[usize::from(value)];
            if structural && !expanded {
                stack.push((value, true));
                for operand in kind.operands() {
                    if self.key_of[usize::from(operand)].is_none() {
                        stack.push((operand, false));
                    }
                }
                continue;
            }
            let operand = |proof: &Self, id: &ValueId| proof.key_of[usize::from(*id)].unwrap_or(0);
            let key = if !structural {
                Key::Merge(value)
            } else {
                match kind {
                    CfgValueKind::RealConstant(constant) => Key::Real(constant.to_bits()),
                    CfgValueKind::BooleanConstant(constant) => Key::Boolean(*constant),
                    CfgValueKind::IntegerConstant(constant) => Key::Integer(*constant),
                    CfgValueKind::Parameter(id) => Key::Parameter(*id),
                    CfgValueKind::ParameterGiven(id) => Key::ParameterGiven(*id),
                    CfgValueKind::PortConnected(port) => Key::PortConnected(*port),
                    CfgValueKind::Temperature => Key::Temperature,
                    CfgValueKind::ThermalVoltage => Key::ThermalVoltage,
                    CfgValueKind::Multiplicity => Key::Multiplicity,
                    CfgValueKind::SimParamValue(parameter) => Key::SimParam(*parameter),
                    CfgValueKind::SimParamPresent(parameter) => Key::SimParamPresent(*parameter),
                    CfgValueKind::Unary { op, input } => Key::Unary(*op, operand(self, input)),
                    CfgValueKind::Binary { op, left, right } => {
                        let (left, right) = (operand(self, left), operand(self, right));
                        match op {
                            CfgBinaryOp::Gt => Key::Binary(CfgBinaryOp::Lt, right, left),
                            CfgBinaryOp::Ge => Key::Binary(CfgBinaryOp::Le, right, left),
                            CfgBinaryOp::Add
                            | CfgBinaryOp::Mul
                            | CfgBinaryOp::Min
                            | CfgBinaryOp::Max
                            | CfgBinaryOp::Hypot
                            | CfgBinaryOp::Eq
                            | CfgBinaryOp::Ne
                            | CfgBinaryOp::And
                            | CfgBinaryOp::Or => Key::Binary(*op, left.min(right), left.max(right)),
                            _ => Key::Binary(*op, left, right),
                        }
                    }
                    CfgValueKind::Select {
                        condition,
                        then_value,
                        else_value,
                    } => Key::Select(
                        operand(self, condition),
                        operand(self, then_value),
                        operand(self, else_value),
                    ),
                    other => Key::Other(format!("{other:?}")),
                }
            };
            let key = self.intern(key);
            self.key_of[usize::from(value)] = Some(key);
        }
        self.key_of[usize::from(root)].unwrap_or(0)
    }

    /// Classify each variable by its kept-value leaves and its value at
    /// function exit. `roots` are the residuals and activations.
    pub(super) fn classify(
        &mut self,
        leaves: &[ValueId],
        roots: &[ValueId],
        candidate: ValueId,
    ) -> HeldClass {
        self.try_classify(leaves, roots, candidate)
            .unwrap_or(HeldClass::History)
    }

    fn try_classify(
        &mut self,
        leaves: &[ValueId],
        roots: &[ValueId],
        candidate: ValueId,
    ) -> Proved<HeldClass> {
        let leaf_set: HashSet<ValueId> = leaves.iter().copied().collect();
        // Every value that may carry the kept value: a leaf, or a merge one
        // of whose arguments may.
        let mut carriers = leaf_set.clone();
        let mut pending: Vec<ValueId> = leaves.to_vec();
        while let Some(value) = pending.pop() {
            for &merge in self.merged_into.get(&value).into_iter().flatten() {
                if carriers.insert(merge) {
                    pending.push(merge);
                }
            }
        }
        if self.influence.is_empty() {
            self.compute_influence(leaves, roots)?;
        }
        let mut reads = FALSE;
        for &leaf in leaves {
            reads = self.diagram.or(reads, self.influence[usize::from(leaf)])?;
        }
        let reads = self.across_evaluations(reads)?;
        let steady_reads = self.diagram.steady(reads)?;
        let assigned = self.assigned_formula(&carriers, &leaf_set, candidate)?;
        let assigned = self.across_evaluations(assigned)?;
        let before_final = self.diagram.restrict(assigned, FINAL_STEP, false)?;
        let ever_assigned = self.diagram.exists(before_final, INITIAL_STEP)?;
        if !self.possible(steady_reads, ever_assigned)? {
            return Ok(HeldClass::AssignedBeforeRead);
        }
        let steady_assigned = self.diagram.steady(assigned)?;
        if !self.possible(steady_assigned, TRUE)?
            && self.initial_value_is_static(&carriers, &leaf_set, candidate)?
        {
            return Ok(HeldClass::SetAtInitialStep);
        }
        Ok(HeldClass::History)
    }

    /// Prepare [`Self::classify`] for variables whose kept-value leaves are
    /// among `leaves`, or `None` when the diagram outgrows its budget.
    pub(super) fn prepare(&mut self, leaves: &[ValueId], roots: &[ValueId]) -> Option<()> {
        self.compute_influence(leaves, roots).ok()
    }

    /// For every value downstream of `sources`, the σ under which the value
    /// it takes at its definition can reach a residual, an activation or a
    /// branch: backwards from those roots, through each use, and through a
    /// merge only along the edge that passes it.
    fn compute_influence(&mut self, sources: &[ValueId], roots: &[ValueId]) -> Proved<()> {
        let function = self.function;
        let count = function.values.len();
        let mut uses: Vec<Vec<(ValueId, Option<usize>)>> = vec![Vec::new(); count];
        for value in &function.values {
            if !matches!(value.kind, CfgValueKind::BlockParameter) {
                for operand in value.kind.operands() {
                    uses[usize::from(operand)].push((value.id, None));
                }
            }
        }
        for (&merge, incoming) in &self.merges {
            for &(edge, arg) in incoming {
                uses[usize::from(arg)].push((merge, Some(edge)));
            }
        }
        let mut downstream = vec![false; count];
        let mut pending: Vec<ValueId> = sources.to_vec();
        while let Some(value) = pending.pop() {
            if std::mem::replace(&mut downstream[usize::from(value)], true) {
                continue;
            }
            pending.extend(uses[usize::from(value)].iter().map(|&(user, _)| user));
        }
        let order: Vec<usize> = (0..count)
            .rev()
            .filter(|&index| downstream[index])
            .collect();
        let mut influence = vec![FALSE; count];
        let mut seeds = vec![FALSE; count];
        for &root in roots {
            seeds[usize::from(root)] = TRUE;
        }
        for (index, block) in function.blocks.iter().enumerate() {
            if let CfgTerminator::Branch { condition, .. } = &block.terminator
                && downstream[usize::from(*condition)]
            {
                let seed = &mut seeds[usize::from(*condition)];
                *seed = self.diagram.or(*seed, self.reach[index])?;
            }
        }
        loop {
            let mut changed = false;
            for &index in &order {
                let mut reaches = seeds[index];
                for &(user, edge) in &uses[index] {
                    let through = influence[usize::from(user)];
                    let through = match edge {
                        Some(edge) => self.diagram.and(through, self.edge_reach[edge])?,
                        None => through,
                    };
                    reaches = self.diagram.or(reaches, through)?;
                }
                if reaches != influence[index] {
                    influence[index] = reaches;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        self.influence = influence;
        Ok(())
    }

    /// The σ under which the value at function exit is not the kept value.
    fn assigned_formula(
        &mut self,
        carriers: &HashSet<ValueId>,
        leaves: &HashSet<ValueId>,
        candidate: ValueId,
    ) -> Proved<u32> {
        if leaves.contains(&candidate) {
            return Ok(FALSE);
        }
        if !carriers.contains(&candidate) {
            return Ok(TRUE);
        }
        // The merges the exit value is built from, and a fixed point over
        // them from false.
        let mut tree = Vec::new();
        let mut seen = HashSet::new();
        let mut pending = vec![candidate];
        while let Some(value) = pending.pop() {
            if !carriers.contains(&value) || leaves.contains(&value) || !seen.insert(value) {
                continue;
            }
            tree.push(value);
            pending.extend(self.merges[&value].iter().map(|&(_, arg)| arg));
        }
        let mut formula: HashMap<ValueId, u32> = tree.iter().map(|&value| (value, FALSE)).collect();
        loop {
            let mut changed = false;
            for &merge in &tree {
                let mut assigned = FALSE;
                for &(edge, arg) in &self.merges[&merge].clone() {
                    let from = if leaves.contains(&arg) {
                        FALSE
                    } else {
                        formula.get(&arg).copied().unwrap_or(TRUE)
                    };
                    let both = self.diagram.and(self.edge_reach[edge], from)?;
                    assigned = self.diagram.or(assigned, both)?;
                }
                if formula[&merge] != assigned {
                    formula.insert(merge, assigned);
                    changed = true;
                }
            }
            if !changed {
                return Ok(formula[&candidate]);
            }
        }
    }

    /// Whether the exit value is built only from instance-static values and
    /// the kept value, through merges σ and the step atoms decide.
    fn initial_value_is_static(
        &mut self,
        carriers: &HashSet<ValueId>,
        leaves: &HashSet<ValueId>,
        candidate: ValueId,
    ) -> Proved<bool> {
        let mut seen = HashSet::new();
        let mut pending = vec![candidate];
        while let Some(value) = pending.pop() {
            if leaves.contains(&value) || !seen.insert(value) {
                continue;
            }
            if !carriers.contains(&value) {
                if !self.instance_static[usize::from(value)] {
                    return Ok(false);
                }
                continue;
            }
            if !self.selection_is_fixed(value, false)? {
                return Ok(false);
            }
            pending.extend(self.merges[&value].iter().map(|&(_, arg)| arg));
        }
        Ok(true)
    }
}

fn is_step(name: &str, step: &str) -> bool {
    name.eq_ignore_ascii_case(step)
}

/// Whether a value of this kind is instance-static when its operands are.
fn static_kind(kind: &CfgValueKind) -> bool {
    match kind {
        CfgValueKind::RealConstant(_)
        | CfgValueKind::BooleanConstant(_)
        | CfgValueKind::IntegerConstant(_)
        | CfgValueKind::Parameter(_)
        | CfgValueKind::ParameterGiven(_)
        | CfgValueKind::PortConnected(_)
        | CfgValueKind::Temperature
        | CfgValueKind::ThermalVoltage
        | CfgValueKind::Multiplicity
        | CfgValueKind::SimParamValue(_)
        | CfgValueKind::SimParamPresent(_)
        | CfgValueKind::Unary { .. }
        | CfgValueKind::SumProductsDiv { .. }
        | CfgValueKind::Select { .. }
        | CfgValueKind::IntegerArithmetic { .. }
        | CfgValueKind::IntegerBitwise { .. }
        | CfgValueKind::IntegerBitwiseNot { .. } => true,
        CfgValueKind::Binary { op, .. } => {
            !matches!(op, CfgBinaryOp::CheckedValue | CfgBinaryOp::DiscreteValue)
        }
        _ => false,
    }
}
