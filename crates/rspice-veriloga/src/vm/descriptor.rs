//! Exact `G + sC` descriptors of a Verilog-A small-signal linearization.
//!
//! The complex evaluator answers "what is the device's admittance at this
//! frequency". Pole-zero extraction and shaped transient noise need the
//! question before it: a finite set of states whose descriptor pencil *is*
//! that admittance at every frequency. This module evaluates the same
//! differentiated bytecode, through the same dispatch, on linear forms
//!
//! ```text
//! g + s·c + Σ (gᵢ + s·cᵢ)·zᵢ
//! ```
//!
//! where the scalar part is the coefficient of the column being
//! differentiated and each `zᵢ` is an explicit operator state. Each dynamic
//! operator is *cut* into states with their own equations:
//!
//! * `ddt` multiplies by `s`, first cutting through an auxiliary state
//!   `u − X = 0` when its input already carries `s`;
//! * `idt` cuts a state `s·y − X = 0` and returns `y`;
//! * `laplace_*` cuts its realization `s·z − A·z − B·X = 0` and returns
//!   `C·z + D·X`, once per filter;
//! * `absdelay`, `zi_*` and `idtmod` have no finite rational realization and
//!   refuse with [`VmError::NonRationalOperator`].
//!
//! A cut input `X` is assembled from every program that reaches the cut:
//! each contributes its own column's coefficient, and the input's terms on
//! inner states, which do not depend on the column, are entered once.
//! Repeated contributions must agree, or the build refuses with
//! [`VmError::InconsistentDescriptor`].

use super::small_signal::{DynamicOperators, SmallSignalEngine, SmallSignalScalar};
use super::{VmContext, VmError};
use crate::codegen::{AssignmentStep, BytecodeProgram, ZiRuntimeLayout};
use std::collections::{BTreeMap, BTreeSet};

/// A row or column of a [`SmallSignalDescriptor`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DescriptorIndex {
    /// A circuit unknown, as its 0-based matrix row. In a
    /// [`NoiseProcessRealization`] it is instead the injection's position in
    /// [`NoiseProcessRealization::injections`].
    Circuit(usize),
    /// An explicit operator state, indexing [`SmallSignalDescriptor::states`].
    State(usize),
    /// The unit noise input of a [`NoiseProcessRealization`].
    NoiseInput,
}

/// One coefficient `g + s·c` of a descriptor pencil.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DescriptorEntry {
    /// The equation the coefficient belongs to.
    pub row: DescriptorIndex,
    /// The unknown it multiplies.
    pub col: DescriptorIndex,
    /// Conductance-like coefficient (the `G` matrix).
    pub g: f64,
    /// Capacitance-like coefficient of `s` (the `C` matrix).
    pub c: f64,
}

/// The analog operator an explicit state realizes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DescriptorStateKind {
    /// One state of the state-space realization of a Laplace filter.
    Laplace {
        /// The instance's Laplace filter, in compiled filter order.
        filter: usize,
    },
    /// The integral of one `idt` site.
    Idt {
        /// The site's integration slot.
        slot: usize,
    },
    /// An algebraic copy `u = X` that lets `ddt` act on an input already
    /// carrying `s`.
    Auxiliary,
}

/// An exact `G + sC` realization of a Verilog-A instance's small-signal
/// response at one operating point.
///
/// The unknowns are the circuit unknowns followed by [`Self::states`]. Every
/// state has its own equation row. Reducing the states out,
///
/// ```text
/// Y(s) = (G + sC)_cc − (G + sC)_cs · ((G + sC)_ss)⁻¹ · (G + sC)_sc,
/// ```
///
/// gives the admittance the complex small-signal stamp produces at `s = jω`.
/// Entries are summed: a (row, column) pair appears at most once, sorted, and
/// no entry is exactly zero.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SmallSignalDescriptor {
    /// The operator each explicit state realizes; `State(i)` is entry `i`.
    pub states: Vec<DescriptorStateKind>,
    /// The nonzero coefficients, sorted by row and then column.
    pub entries: Vec<DescriptorEntry>,
}

/// The shaping of one Verilog-A noise process, as a descriptor driven by a
/// unit noise input.
///
/// Row `Circuit(i)` is the gain of injection `i`; the column `NoiseInput` is
/// the process's unit source. Reducing the states out gives, at `s = jω`, the
/// injection gains the per-frequency noise evaluation reports, including the
/// multiplicity scaling and the RHS sign. A process with no shaping operator
/// has no states.
#[derive(Clone, Debug, PartialEq)]
pub struct NoiseProcessRealization {
    /// The compiled process this realizes.
    pub process_id: usize,
    /// The shaping states and gains, with rows `Circuit(i)` for injection
    /// `i` and the single input column `NoiseInput`.
    pub descriptor: SmallSignalDescriptor,
    /// `(node_pos, node_neg)` of each injection, as circuit node numbers with
    /// 0 for ground, in the order the per-frequency noise evaluation reports
    /// them.
    pub injections: Vec<(usize, usize)>,
}

/// A derivative direction of the compiled model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Axis {
    /// A node voltage, as the unified node index (terminals, then internal
    /// nodes).
    Node(usize),
    /// A branch-current unknown.
    Branch(usize),
    /// The tangent of noise process `process_id`.
    Noise(usize),
}

/// What derivative the program being evaluated computes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Column {
    /// An operating-point value: no derivative at all.
    #[default]
    Primal,
    /// The derivative along one axis.
    Axis(Axis),
    /// A derivative the descriptor does not export (limiter correction).
    Ignored,
}

/// The column each variable slot's assignment computes, read from the
/// derivative-shadow names the compiler gives them (`x@d3`, `x@dI0`,
/// `x@dN2`, `x@dL`, `a@d1[0]`). A higher-order shadow is a derivative along
/// its last axis.
pub(crate) fn shadow_columns(names: &[smol_str::SmolStr], slots: usize) -> Vec<Column> {
    (0..slots)
        .map(|slot| {
            names
                .get(slot)
                .map_or(Column::Primal, |name| shadow_column(name))
        })
        .collect()
}

fn shadow_column(name: &str) -> Column {
    let Some(at) = name.rfind("@d") else {
        return Column::Primal;
    };
    let suffix = &name[at + 2..];
    let suffix = suffix.split_once('[').map_or(suffix, |(head, _)| head);
    if suffix == "L" {
        return Column::Ignored;
    }
    let (axis, digits): (fn(usize) -> Axis, &str) = match suffix.strip_prefix('I') {
        Some(digits) => (Axis::Branch, digits),
        None => match suffix.strip_prefix('N') {
            Some(digits) => (Axis::Noise, digits),
            None => (Axis::Node, suffix),
        },
    };
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Column::Primal;
    }
    digits
        .parse()
        .map_or(Column::Primal, |index| Column::Axis(axis(index)))
}

/// `g + s·c`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Coefficient {
    g: f64,
    c: f64,
}

impl Coefficient {
    const ZERO: Self = Self { g: 0.0, c: 0.0 };

    fn is_zero(self) -> bool {
        self.g == 0.0 && self.c == 0.0
    }

    fn scaled(self, scale: f64) -> Self {
        Self {
            g: self.g * scale,
            c: self.c * scale,
        }
    }

    fn add(&mut self, other: Self) {
        self.g += other.g;
        self.c += other.c;
    }

    /// Two evaluations of one coefficient agree to 1e-12 relative.
    fn agrees_with(self, other: Self) -> bool {
        fn close(a: f64, b: f64) -> bool {
            a == b || (a - b).abs() <= 1e-12 * a.abs().max(b.abs())
        }
        close(self.g, other.g) && close(self.c, other.c)
    }
}

/// A linear form `g + s·c + Σ (gᵢ + s·cᵢ)·zᵢ`, the value one descriptor
/// evaluation computes.
///
/// For an operating-point value the form is the constant `g`. For a
/// derivative, `g + s·c` is the coefficient of the column being
/// differentiated and the state terms are the value's dependence on explicit
/// operator states. State terms are sorted by state and none is exactly zero.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct DescriptorValue {
    scalar: Coefficient,
    states: Vec<(usize, Coefficient)>,
}

impl DescriptorValue {
    fn state(state: usize, coefficient: Coefficient) -> Self {
        Self {
            scalar: Coefficient::ZERO,
            states: vec![(state, coefficient)],
        }
    }

    fn is_constant(&self) -> bool {
        self.scalar.c == 0.0 && self.states.is_empty()
    }

    fn carries_s(&self) -> bool {
        self.scalar.c != 0.0 || self.states.iter().any(|(_, term)| term.c != 0.0)
    }

    fn map(mut self, f: impl Fn(Coefficient) -> Coefficient) -> Self {
        self.scalar = f(self.scalar);
        for (_, term) in &mut self.states {
            *term = f(*term);
        }
        self.states.retain(|(_, term)| !term.is_zero());
        self
    }

    /// `s` times a form that carries no `s`.
    fn times_s(self) -> Self {
        self.map(|term| Coefficient { g: 0.0, c: term.g })
    }

    fn scale_by(self, scale: f64) -> Self {
        self.map(|term| term.scaled(scale))
    }

    fn divide_by(self, divisor: f64) -> Self {
        self.map(|term| Coefficient {
            g: term.g / divisor,
            c: term.c / divisor,
        })
    }

    fn plus(self, other: Self) -> Self {
        let mut states = Vec::with_capacity(self.states.len() + other.states.len());
        let mut left = self.states.into_iter().peekable();
        let mut right = other.states.into_iter().peekable();
        loop {
            let left_state = left.peek().map(|&(state, _)| state);
            let right_state = right.peek().map(|&(state, _)| state);
            let next = match (left_state, right_state) {
                (Some(a), Some(b)) if a == b => {
                    let (state, mut term) = left.next().expect("peeked");
                    term.add(right.next().expect("peeked").1);
                    (state, term)
                }
                (Some(a), Some(b)) if a < b => left.next().expect("peeked"),
                (_, Some(_)) => right.next().expect("peeked"),
                (Some(_), None) => left.next().expect("peeked"),
                (None, None) => break,
            };
            if !next.1.is_zero() {
                states.push(next);
            }
        }
        let mut scalar = self.scalar;
        scalar.add(other.scalar);
        Self { scalar, states }
    }
}

impl std::ops::Neg for DescriptorValue {
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|term| Coefficient {
            g: -term.g,
            c: -term.c,
        })
    }
}

impl SmallSignalScalar for DescriptorValue {
    // A product of two dynamic forms is not affine in `s`; the sticky flag
    // carries that refusal to the next program boundary.
    const TRACK_RANGE: bool = true;

    fn constant(value: f64) -> Self {
        Self {
            scalar: Coefficient { g: value, c: 0.0 },
            states: Vec::new(),
        }
    }
    fn real_component(&self) -> f64 {
        self.scalar.g
    }
    fn is_real(&self) -> bool {
        self.is_constant()
    }
    fn is_finite(&self) -> bool {
        self.scalar.g.is_finite()
            && self.scalar.c.is_finite()
            && self
                .states
                .iter()
                .all(|(_, term)| term.g.is_finite() && term.c.is_finite())
    }
    fn add(self, other: Self, _not_affine: &mut bool) -> Self {
        self.plus(other)
    }
    fn subtract(self, other: Self, _not_affine: &mut bool) -> Self {
        self.plus(-other)
    }
    fn multiply(self, other: Self, not_affine: &mut bool) -> Self {
        if other.is_constant() {
            self.scale_by(other.scalar.g)
        } else if self.is_constant() {
            other.scale_by(self.scalar.g)
        } else {
            *not_affine = true;
            Self::default()
        }
    }
    fn divide(self, other: Self, not_affine: &mut bool) -> Self {
        if other.is_constant() {
            self.divide_by(other.scalar.g)
        } else {
            *not_affine = true;
            Self::default()
        }
    }
    fn scale(self, scale: f64, _not_affine: &mut bool) -> Self {
        self.scale_by(scale)
    }
    fn sum_products_div(
        pairs: &[[Self; 2]],
        divisor: Self,
        not_affine: &mut bool,
    ) -> Result<Self, VmError> {
        let mut sum = Self::default();
        for [left, right] in pairs {
            sum = sum.plus(left.clone().multiply(right.clone(), not_affine));
        }
        Ok(sum.divide(divisor, not_affine))
    }
    fn sticky_error() -> VmError {
        VmError::InconsistentDescriptor(
            "a product or quotient of two dynamic small-signal values is not affine in s".into(),
        )
    }
}

/// The operator cuts of one descriptor evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CutKey {
    Laplace(usize),
    Idt(usize),
    Ddt(usize),
}

/// A cut operator: its states, and the input assembled so far.
#[derive(Debug, Default)]
struct Cut {
    /// Each state of the cut, with the weight its equation gives the input.
    rows: Vec<(usize, f64)>,
    /// The input's coefficient on each axis.
    axis_inputs: BTreeMap<Axis, Coefficient>,
    /// The input's terms on inner states.
    state_inputs: BTreeMap<usize, Coefficient>,
}

/// Operator state of a descriptor evaluation, kept beside the shared engine.
#[derive(Debug, Default)]
pub(super) struct DescriptorCuts {
    /// What the program being evaluated differentiates.
    column: Column,
    /// The column each variable slot's assignment computes.
    columns: Vec<Column>,
    states: Vec<DescriptorStateKind>,
    cuts: BTreeMap<CutKey, Cut>,
    /// Each state's own equation terms: `(row state, column state, g + s·c)`.
    dynamics: Vec<(usize, usize, Coefficient)>,
    /// `ddt` slots that cut. Fixed for an evaluation pass, so every column
    /// takes the same decision; a slot first found to need a cut mid-pass
    /// joins the set and asks for another pass.
    ddt_cuts: BTreeSet<usize>,
    restart: bool,
}

impl DescriptorCuts {
    fn allocate(&mut self, kind: DescriptorStateKind) -> usize {
        self.states.push(kind);
        self.states.len() - 1
    }

    /// The cut for `key`, created by `create` on first use.
    fn cut(
        &mut self,
        key: CutKey,
        create: impl FnOnce(&mut Self) -> Vec<(usize, f64)>,
    ) -> &mut Cut {
        if !self.cuts.contains_key(&key) {
            let rows = create(self);
            self.cuts.insert(
                key,
                Cut {
                    rows,
                    ..Cut::default()
                },
            );
        }
        self.cuts.get_mut(&key).expect("cut was just created")
    }

    /// Enter `input` as the input of the cut `key`.
    fn connect(&mut self, key: CutKey, input: &DescriptorValue) -> Result<(), VmError> {
        let column = self.column;
        let cut = self
            .cuts
            .get_mut(&key)
            .expect("cut exists before its input");
        if !input.scalar.is_zero() {
            match column {
                Column::Axis(axis) => agree(&mut cut.axis_inputs, axis, input.scalar, || {
                    format!("{key:?} input along {axis:?}")
                })?,
                Column::Ignored => {}
                Column::Primal => {
                    return Err(VmError::InvalidRuntimeOperation(format!(
                        "{key:?} was reached by a program that differentiates no column"
                    )));
                }
            }
        }
        for &(state, term) in &input.states {
            agree(&mut cut.state_inputs, state, term, || {
                format!("{key:?} input on state {state}")
            })?;
        }
        Ok(())
    }

    fn ddt(&mut self, slot: usize, input: DescriptorValue) -> Result<DescriptorValue, VmError> {
        if !self.ddt_cuts.contains(&slot) {
            if !input.carries_s() {
                return Ok(input.times_s());
            }
            self.ddt_cuts.insert(slot);
            self.restart = true;
        }
        let key = CutKey::Ddt(slot);
        let state = self.cut(key, |cuts| {
            let state = cuts.allocate(DescriptorStateKind::Auxiliary);
            cuts.dynamics
                .push((state, state, Coefficient { g: 1.0, c: 0.0 }));
            vec![(state, -1.0)]
        });
        let state = state.rows[0].0;
        self.connect(key, &input)?;
        Ok(DescriptorValue::state(
            state,
            Coefficient { g: 0.0, c: 1.0 },
        ))
    }

    fn integral(
        &mut self,
        slot: usize,
        input: DescriptorValue,
    ) -> Result<DescriptorValue, VmError> {
        let key = CutKey::Idt(slot);
        let state = self.cut(key, |cuts| {
            let state = cuts.allocate(DescriptorStateKind::Idt { slot });
            cuts.dynamics
                .push((state, state, Coefficient { g: 0.0, c: 1.0 }));
            vec![(state, -1.0)]
        });
        let state = state.rows[0].0;
        self.connect(key, &input)?;
        Ok(DescriptorValue::state(
            state,
            Coefficient { g: 1.0, c: 0.0 },
        ))
    }

    fn laplace(
        &mut self,
        filter: usize,
        realization: &crate::laplace::StateSpaceRealization<'_>,
        input: DescriptorValue,
    ) -> Result<DescriptorValue, VmError> {
        let feedthrough = input.clone().scale_by(realization.d);
        if realization.b.is_empty() {
            return Ok(feedthrough);
        }
        let key = CutKey::Laplace(filter);
        let rows = &self
            .cut(key, |cuts| {
                let first = cuts.states.len();
                for _ in realization.b {
                    cuts.allocate(DescriptorStateKind::Laplace { filter });
                }
                for (row, a_row) in realization.a.iter().enumerate() {
                    for (col, &a) in a_row.iter().enumerate() {
                        let term = Coefficient {
                            g: -a,
                            c: if row == col { 1.0 } else { 0.0 },
                        };
                        if !term.is_zero() {
                            cuts.dynamics.push((first + row, first + col, term));
                        }
                    }
                }
                realization
                    .b
                    .iter()
                    .enumerate()
                    .map(|(row, &b)| (first + row, -b))
                    .collect()
            })
            .rows;
        let output = rows
            .iter()
            .zip(realization.c)
            .filter(|(_, c)| **c != 0.0)
            .map(|(&(state, _), &c)| (state, Coefficient { g: c, c: 0.0 }))
            .collect();
        self.connect(key, &input)?;
        Ok(DescriptorValue {
            scalar: Coefficient::ZERO,
            states: output,
        }
        .plus(feedthrough))
    }
}

/// Insert `term` at `key`, or check it against the value already there.
fn agree<K: Ord + Copy>(
    map: &mut BTreeMap<K, Coefficient>,
    key: K,
    term: Coefficient,
    what: impl FnOnce() -> String,
) -> Result<(), VmError> {
    if term.is_zero() {
        return Ok(());
    }
    match map.get(&key) {
        None => {
            map.insert(key, term);
            Ok(())
        }
        Some(existing) if existing.agrees_with(term) => Ok(()),
        Some(existing) => Err(VmError::InconsistentDescriptor(format!(
            "{} is {} + s*{} in one program and {} + s*{} in another",
            what(),
            existing.g,
            existing.c,
            term.g,
            term.c
        ))),
    }
}

impl DynamicOperators for DescriptorValue {
    type Realization = DescriptorCuts;

    fn begin_assignment(engine: &mut SmallSignalEngine<'_, Self>, target: Option<usize>) {
        let cuts = &mut engine.realization;
        cuts.column = target.map_or(Column::Primal, |slot| {
            cuts.columns.get(slot).copied().unwrap_or(Column::Primal)
        });
    }

    fn ddt_derivative(
        engine: &mut SmallSignalEngine<'_, Self>,
        slot: usize,
    ) -> Result<(), VmError> {
        let input = engine.pop("ddt input derivative")?;
        let _primal = engine.pop_real("ddt primal")?;
        let result = engine.realization.ddt(slot, input)?;
        engine.push(result);
        Ok(())
    }

    fn idt_jacobian(_engine: &mut SmallSignalEngine<'_, Self>) -> Result<(), VmError> {
        // The legacy companion factor names no integration slot, so the
        // programs that reach it cannot share one state.
        Err(VmError::InvalidInstruction(
            "IdtJacobian names no integration slot to realize as a descriptor state",
        ))
    }

    fn integral_derivative(
        engine: &mut SmallSignalEngine<'_, Self>,
        slot: usize,
        wrapped: bool,
    ) -> Result<(), VmError> {
        if wrapped {
            return Err(VmError::NonRationalOperator(IDTMOD));
        }
        // The initial condition and an `assert` reset are large-signal only.
        let _ic_derivative = engine.pop("integral initial-condition derivative")?;
        let input = engine.pop("integral input derivative")?;
        let _primal = engine.pop_real("integral primal")?;
        let result = engine.realization.integral(slot, input)?;
        engine.push(result);
        Ok(())
    }

    fn idtmod_state(
        _engine: &mut SmallSignalEngine<'_, Self>,
        _slot: usize,
    ) -> Result<(), VmError> {
        Err(VmError::NonRationalOperator(IDTMOD))
    }

    fn laplace_derivative(
        engine: &mut SmallSignalEngine<'_, Self>,
        filter_id: usize,
    ) -> Result<(), VmError> {
        let input = engine.pop("LaplaceStateDerivative")?;
        let context: &VmContext = engine.context;
        let realization = context
            .laplace_filters
            .get(filter_id)
            .ok_or(VmError::InvalidInstruction("missing laplace filter"))?
            .realization()
            .map_err(|error| {
                VmError::classified(
                    error.is_iterate_dependent(),
                    format!("Laplace filter {filter_id}: {error}"),
                )
            })?;
        let result = engine.realization.laplace(filter_id, &realization, input)?;
        engine.push(result);
        Ok(())
    }

    fn zi(
        _engine: &mut SmallSignalEngine<'_, Self>,
        _layout: ZiRuntimeLayout,
        _derivative: bool,
    ) -> Result<(), VmError> {
        Err(VmError::NonRationalOperator(ZI))
    }

    fn absdelay(
        _engine: &mut SmallSignalEngine<'_, Self>,
        _buffer_id: usize,
        _with_maximum: bool,
        _derivative: bool,
        _mixed: bool,
    ) -> Result<(), VmError> {
        Err(VmError::NonRationalOperator(ABSDELAY))
    }
}

/// The spelling of each non-rational operator in a refusal and in
/// [`crate::device::VerilogADevice::small_signal_descriptor_blockers`].
pub(crate) const ABSDELAY: &str = "absdelay";
pub(crate) const ZI: &str = "zi_*";
pub(crate) const IDTMOD: &str = "idtmod";

/// Where one evaluated program's value is stamped.
#[derive(Clone, Copy, Debug)]
pub(crate) struct OutputTarget {
    /// Identifies the equation value the program differentiates. Programs
    /// of one target differ only in their column, so their state terms are
    /// the same and are entered once.
    pub(crate) key: (usize, usize, usize, u64),
    pub(crate) row: Option<DescriptorIndex>,
    /// Sign and multiplicity applied to the value.
    pub(crate) scale: f64,
}

#[derive(Debug)]
struct OutputTerms {
    row: DescriptorIndex,
    scale: f64,
    states: BTreeMap<usize, Coefficient>,
}

/// One pass of a descriptor build: the shared engine evaluating on linear
/// forms, plus the entries the caller has stamped.
pub(crate) struct DescriptorBuild<'a> {
    engine: SmallSignalEngine<'a, DescriptorValue>,
    entries: BTreeMap<(DescriptorIndex, DescriptorIndex), Coefficient>,
    outputs: BTreeMap<(usize, usize, usize, u64), OutputTerms>,
}

impl<'a> DescriptorBuild<'a> {
    /// Start a pass from the operating-point variable image `seed`. `ddt_cuts`
    /// are the `ddt` slots an earlier pass found must cut.
    pub(crate) fn new(
        context: &'a VmContext,
        seed: &[f64],
        columns: Vec<Column>,
        ddt_cuts: BTreeSet<usize>,
    ) -> Result<Self, VmError> {
        let cuts = DescriptorCuts {
            columns,
            ddt_cuts,
            ..DescriptorCuts::default()
        };
        Ok(Self {
            engine: SmallSignalEngine::with_realization(context, seed, cuts)?,
            entries: BTreeMap::new(),
            outputs: BTreeMap::new(),
        })
    }

    /// Replay the assignment stream, as the complex evaluator does.
    pub(crate) fn replay(
        &mut self,
        steps: &[AssignmentStep],
        live: Option<&[bool]>,
    ) -> Result<(), VmError> {
        self.engine.execute_assignment_steps(steps, live)
    }

    /// Evaluate one derivative program along `column`.
    pub(crate) fn evaluate(
        &mut self,
        program: &BytecodeProgram,
        column: Column,
    ) -> Result<DescriptorValue, VmError> {
        self.engine.realization.column = column;
        self.engine.execute_value(program)
    }

    /// Add a fixed coefficient.
    pub(crate) fn add_entry(&mut self, row: DescriptorIndex, col: DescriptorIndex, g: f64, c: f64) {
        self.entries
            .entry((row, col))
            .or_default()
            .add(Coefficient { g, c });
    }

    /// Stamp a program's value: its column coefficient at `(target.row,
    /// column)`, and its state terms once per target.
    pub(crate) fn add_output(
        &mut self,
        target: OutputTarget,
        column: Option<DescriptorIndex>,
        value: DescriptorValue,
    ) -> Result<(), VmError> {
        let Some(row) = target.row else {
            return Ok(());
        };
        if let Some(column) = column {
            let scalar = value.scalar.scaled(target.scale);
            self.add_entry(row, column, scalar.g, scalar.c);
        }
        let terms = self.outputs.entry(target.key).or_insert(OutputTerms {
            row,
            scale: target.scale,
            states: BTreeMap::new(),
        });
        for (state, term) in value.states {
            agree(&mut terms.states, state, term, || {
                format!("the output {:?} on state {state}", target.key)
            })?;
        }
        Ok(())
    }

    /// The `ddt` cuts to rerun with, when this pass found a new one.
    pub(crate) fn restart(&self) -> Option<BTreeSet<usize>> {
        let cuts = &self.engine.realization;
        cuts.restart.then(|| cuts.ddt_cuts.clone())
    }

    /// Assemble the descriptor. `axis_index` places each cut-input axis, or
    /// drops it.
    ///
    /// A state no input reaches, directly or through other states, is
    /// identically zero in every small-signal solution, as the complex
    /// evaluator's zero transfer of a zero input says; it is removed with
    /// every entry that names it. That is an operator whose input does not
    /// depend on any unknown, or one reached only along a derivative the
    /// descriptor does not export.
    pub(crate) fn finish(
        mut self,
        axis_index: impl Fn(Axis) -> Option<DescriptorIndex>,
    ) -> Result<SmallSignalDescriptor, VmError> {
        let cuts = std::mem::take(&mut self.engine.realization);
        let mut entries = self.entries;
        let mut add = |row, col, term: Coefficient| {
            entries.entry((row, col)).or_default().add(term);
        };
        for terms in self.outputs.values() {
            for (&state, &term) in &terms.states {
                add(
                    terms.row,
                    DescriptorIndex::State(state),
                    term.scaled(terms.scale),
                );
            }
        }
        for &(row, col, term) in &cuts.dynamics {
            add(
                DescriptorIndex::State(row),
                DescriptorIndex::State(col),
                term,
            );
        }
        for cut in cuts.cuts.values() {
            for &(state, weight) in &cut.rows {
                let row = DescriptorIndex::State(state);
                for (&axis, &term) in &cut.axis_inputs {
                    if let Some(col) = axis_index(axis) {
                        add(row, col, term.scaled(weight));
                    }
                }
                for (&inner, &term) in &cut.state_inputs {
                    add(row, DescriptorIndex::State(inner), term.scaled(weight));
                }
            }
        }
        entries.retain(|_, term| !term.is_zero());
        if let Some(((row, col), term)) = entries
            .iter()
            .find(|(_, term)| !term.g.is_finite() || !term.c.is_finite())
        {
            return Err(VmError::InvalidNumericResult(format!(
                "small-signal descriptor entry ({row:?}, {col:?}) is {} + s*{}",
                term.g, term.c
            )));
        }

        let renumbered = controllable_states(&entries, cuts.states.len());
        let states = cuts
            .states
            .into_iter()
            .zip(&renumbered)
            .filter_map(|(kind, index)| index.map(|_| kind))
            .collect();
        let place = |index: DescriptorIndex| match index {
            DescriptorIndex::State(state) => renumbered[state].map(DescriptorIndex::State),
            other => Some(other),
        };
        let entries = entries
            .into_iter()
            .filter_map(|((row, col), term)| Some(((place(row)?, place(col)?), term)));
        Ok(SmallSignalDescriptor {
            states,
            entries: entries
                .map(|((row, col), term)| DescriptorEntry {
                    row,
                    col,
                    g: term.g,
                    c: term.c,
                })
                .collect(),
        })
    }
}

/// The new number of each state an input column reaches, directly or
/// through other states' equations; `None` for the rest.
fn controllable_states(
    entries: &BTreeMap<(DescriptorIndex, DescriptorIndex), Coefficient>,
    count: usize,
) -> Vec<Option<usize>> {
    let mut reached = vec![false; count];
    let mut pending: Vec<usize> = entries
        .keys()
        .filter_map(|&(row, col)| match (row, col) {
            (DescriptorIndex::State(state), DescriptorIndex::Circuit(_))
            | (DescriptorIndex::State(state), DescriptorIndex::NoiseInput) => Some(state),
            _ => None,
        })
        .collect();
    while let Some(state) = pending.pop() {
        if std::mem::replace(&mut reached[state], true) {
            continue;
        }
        pending.extend(entries.keys().filter_map(|&(row, col)| match (row, col) {
            (DescriptorIndex::State(to), DescriptorIndex::State(from)) if from == state => Some(to),
            _ => None,
        }));
    }
    let mut next = 0;
    reached
        .into_iter()
        .map(|kept| {
            kept.then(|| {
                next += 1;
                next - 1
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_names_name_their_last_axis() {
        assert_eq!(shadow_column("x"), Column::Primal);
        assert_eq!(shadow_column("x@d3"), Column::Axis(Axis::Node(3)));
        assert_eq!(shadow_column("x@dI0"), Column::Axis(Axis::Branch(0)));
        assert_eq!(shadow_column("x@dN128"), Column::Axis(Axis::Noise(128)));
        assert_eq!(shadow_column("x@dL"), Column::Ignored);
        assert_eq!(shadow_column("a@d1[0]"), Column::Axis(Axis::Node(1)));
        assert_eq!(shadow_column("q@d1@d2"), Column::Axis(Axis::Node(2)));
        assert_eq!(shadow_column("@ddx_update3"), Column::Primal);
        assert_eq!(
            shadow_column("@ddx_update3@d0"),
            Column::Axis(Axis::Node(0))
        );
    }

    #[test]
    fn forms_add_by_state_and_drop_cancelled_terms() {
        let z = |state, g| DescriptorValue::state(state, Coefficient { g, c: 0.0 });
        let sum = z(2, 1.0).plus(z(0, 3.0)).plus(z(2, -1.0));
        assert_eq!(sum, z(0, 3.0));
        let mut flag = false;
        assert_eq!(
            z(1, 2.0).multiply(DescriptorValue::constant(4.0), &mut flag),
            z(1, 8.0)
        );
        assert!(!flag);
        let _ = z(1, 2.0).multiply(z(0, 1.0), &mut flag);
        assert!(flag, "a product of two dynamic forms is not affine in s");
    }
}
