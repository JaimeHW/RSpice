//! CFG: the canonical level that keeps control flow.
//!
//! Every level below this one describes a Verilog-A module as a flat list of
//! values with conditionals folded into per-value selects. That representation
//! is what forces the rest of the backend into its current shape, and this
//! module is the replacement: basic blocks, real terminators, and SSA values
//! whose merges are block parameters.
//!
//! ## Why blocks
//!
//! Dissolving `if` into `guard ? value : previous` costs three things at once.
//! The "previous" value has to be recovered by searching backwards through the
//! assignment history, which is bounded heuristic work where SSA construction is
//! exact and linear. Emitted source then scales as operations x guard depth x
//! derivative lanes, which is what makes the largest models too big to emit
//! directly. And operands shared between guarded consumers get hoisted above
//! their guard, so a disabled model option still costs — a hand-written model
//! skips it with one branch.
//!
//! Blocks fix all three by not throwing the information away in the first place.
//!
//! ## Block parameters instead of phi instructions
//!
//! A merge is expressed by giving the successor block a parameter and having
//! each predecessor pass an argument on its terminator, rather than by a `phi`
//! pseudo-instruction that reads "the value from predecessor N". The two are
//! equivalent, but parameters keep every value definition inside exactly one
//! block, which means a pass never has to special-case instructions that are
//! positional with respect to their predecessors. Differentiating a merge then
//! becomes what it should be: a block that takes one more parameter.
//!
//! ## Construction
//!
//! [`SsaBuilder`] implements Braun et al., *Simple and Efficient Construction of
//! Static Single Assignment Form* (CC 2013). It builds pruned, minimal SSA
//! directly from the structured statements as they are walked, with no dominance
//! frontier and no separate renaming pass. A block is *sealed* once every
//! predecessor it will ever have is known; reads before that point leave an
//! incomplete parameter behind, which is filled in on sealing. Loops are the
//! only construct that needs the delay, because a loop header is reachable from
//! its own body.
//!
//! ## The two filter coefficient contracts
//!
//! [`CfgValueKind::Laplace`] carries folded `f64` constants and
//! [`CfgValueKind::Zi`] carries SSA operands. That is deliberate asymmetry, not
//! an inconsistency waiting to be tidied, and each kind's documentation gives
//! the argument in full. In short: the front end *refuses* a non-constant
//! `laplace_*` coefficient by name, so an operand form there could hold nothing
//! a constant form cannot; while a `zi_*` coefficient is an ordinary expression
//! the runtime freezes per instance, which is exactly why a Zi checkpoint
//! serializes `num` and `den` where a Laplace checkpoint serializes only state.
//!
//! What both forms share is the property the state layout needs: the *length*
//! of each polynomial is syntactic either way, so the record's width is fixed at
//! compile time whether or not its contents are. The spelling of the
//! coefficients was never what made the state shape static, which is why the
//! two contracts can differ without the layout having to care.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::digital::{
    DigitalDriverId, DigitalSchedulingRegion, DigitalSensitivityTerm, DigitalWriteTarget,
};
use super::digital_value;
use super::ids::DigitalAnalogProbeId;
use super::state::CanonicalStateOperator;
use super::{
    BlockId, BranchId, BranchUnknownId, ContributionId, DigitalLocalId, DigitalSignalId, ExprId,
    NodeId, ParamId, ShapeId, ValueId, VariableId,
};

/// What SSA tracks a definition for.
///
/// Module variables are the obvious case. Contribution residuals are here for
/// the same reason a hand-written model accumulates into a local: `I(a,b) <+ x`
/// inside an `if` is `acc = acc + x` on the taken path and nothing on the other,
/// so the merge at the join is the ordinary one the block-parameter machinery
/// already builds. Modelling it any other way would mean a second, parallel
/// merge mechanism for effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgVariable {
    Local(VariableId),
    /// A variable declared inside a digital process, which lives in SSA rather
    /// than in the signal store.
    ///
    /// Separate from [`Self::Local`] because the two are drawn from different
    /// numbering spaces — one indexes the module's analog variables, the other
    /// the declarations of one process — and a merge that confused them would
    /// read an analog `real` where a four-state counter belongs.
    DigitalLocal(DigitalLocalId),
    Residual(ContributionId),
    /// Whether a potential contribution's leading instance-static guard prefix
    /// enables its topology. This is not ordinary statement reachability: once
    /// a bias-, time-, or state-dependent guard is encountered, topology stays
    /// active even when runtime control does not take the contribution's path.
    /// It must remain separate from the residual because an inactive/open
    /// contribution and an active ideal zero-volt source both have a numeric
    /// residual of zero, but only the latter closes the branch topology.
    Activation(ContributionId),
}

/// Arithmetic on one operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgUnaryOp {
    Neg,
    Not,
    Exp,
    LimExp,
    /// The runtime's own bounded exponential, distinct from `limexp`: it is
    /// clamped from below as well as above, so it has its own derivative.
    LimitedExp,
    /// `d/dx` of [`Self::LimitedExp`], which is not itself expressible as one.
    LimitedExpDerivative,
    Ln,
    Log10,
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Asinh,
    Acosh,
    Atanh,
    Floor,
    Ceil,
}

/// Arithmetic on two operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    /// Kept as an operation rather than lowered to a branch: a hand-written
    /// model writes `fmin`, and turning every `min` in a BSIM model into a
    /// diamond would cost more blocks than the whole rest of the module.
    Min,
    Max,
    /// `hypot(x, y)`, kept as one operation because it is the numerically
    /// careful form the library provides and squaring the operands to rebuild
    /// it would overflow exactly where it exists to not.
    Hypot,
    /// `atan2(y, x)`, left as its own operation for the same reason `hypot` is:
    /// the quadrant is part of the answer, and a quotient loses it.
    Atan2,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

/// Independent solver quantity named by a `ddx` probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CfgDdxAxis {
    /// Potential difference. A two-ended probe is the differential coordinate
    /// `(Vp - Vn)` with common mode held fixed.
    Potential {
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    },
    /// Flow of a branch whose potential/indirect contribution introduced a
    /// solver-owned branch-current unknown.
    BranchFlow {
        unknown: BranchUnknownId,
        reversed: bool,
    },
}

/// Bitwise and shift operators on the Verilog-AMS `integer` type.
///
/// Not folded into [`CfgBinaryOp`], and not into
/// [`digital_value::BitwiseOp`] either. `CfgBinaryOp` is arithmetic on the
/// reals and total; these are defined only where both operands round to a
/// representable signed 32-bit value, so an infinity or a magnitude past
/// `i32::MAX` is a runtime error rather than a number — which is a different
/// signature, not a different case of the same one. The four-state operators
/// are elementwise over `aval`/`bval` planes of a declared width and answer
/// `x` where these raise.
///
/// The conversion is the one `crate::integer_runtime` defines: round to
/// nearest with exact halves away from zero, then signed 32-bit two's
/// complement, with both shifts filling with zero per Verilog-AMS 2023 section
/// 4.2.11.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CfgIntegerBitwiseOp {
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

/// A Laplace transfer function, in whichever of the two realizable forms the
/// source's spelling reduces to.
///
/// See [`CfgValueKind::Laplace`] for why these are folded constants and why
/// there are two forms rather than the language's four.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CfgLaplaceTransfer {
    /// `laplace_zp`: complex zeros and poles as `(real, imaginary)` pairs. Kept
    /// unexpanded because the realization is built from the roots.
    ///
    /// No gain factor: the operator takes three arguments in Verilog-AMS, so a
    /// pole-zero filter's leading coefficient is always one, and a field that
    /// can only hold one is a field a consumer has to check.
    ZeroPole {
        zeros: Vec<(f64, f64)>,
        poles: Vec<(f64, f64)>,
    },
    /// `laplace_nd`, and `laplace_zd`/`laplace_np` after their root half is
    /// expanded: numerator and denominator coefficients in ascending powers of
    /// `s`, the convention the operator's argument lists use.
    Coefficients {
        numerator: Vec<f64>,
        denominator: Vec<f64>,
    },
}

/// One polynomial of a sampled-data filter, as the source wrote it.
///
/// The values are SSA operands rather than numbers; see [`CfgValueKind::Zi`]
/// for why, and for why the *count* is nonetheless a compile-time constant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CfgZiPolynomial {
    /// Coefficients ascending in `z^-1`.
    Coefficients(Vec<ValueId>),
    /// Roots as `(real, imaginary)` operand pairs, expanded by the runtime once
    /// they have values.
    Roots(Vec<(ValueId, ValueId)>),
}

impl CfgZiPolynomial {
    /// The operands, in the order the runtime reads them.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Coefficients(values) => values.clone(),
            Self::Roots(values) => values
                .iter()
                .flat_map(|(real, imaginary)| [*real, *imaginary])
                .collect(),
        }
    }

    fn map_operands(&mut self, map: &mut impl FnMut(ValueId) -> ValueId) {
        match self {
            Self::Coefficients(values) => {
                for value in values {
                    *value = map(*value);
                }
            }
            Self::Roots(values) => {
                for (real, imaginary) in values {
                    *real = map(*real);
                    *imaginary = map(*imaginary);
                }
            }
        }
    }
}

/// What an SSA value is.
///
/// Deliberately without a `Select`: a conditional is a [`CfgTerminator::Branch`]
/// and a merge is a block parameter. Deliberately without loop constructs:
/// a loop is a back edge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CfgValueKind {
    RealConstant(f64),
    BooleanConstant(bool),
    /// Defined by the enclosing block's parameter list, not by an instruction.
    BlockParameter,
    Parameter(ParamId),
    ParameterGiven(ParamId),
    /// Runtime connection state of one external terminal.
    PortConnected(u32),
    /// Accepted procedural state for a variable written by an event-controlled
    /// body. The dense slot is stable within one generated model.
    EventState(u32),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    Analysis(smol_str::SmolStr),
    /// `$simparam("name", fallback)`.
    SimParam {
        name: smol_str::SmolStr,
        fallback: ValueId,
    },
    NodePotential(NodeId),
    BranchFlow(BranchId),
    BranchUnknownFlow(BranchUnknownId),
    /// The current a model has already contributed to one branch, read as a
    /// *frozen* quantity rather than as the expression that produced it.
    ///
    /// A branch nothing solves for has no unknown; its current is the running
    /// sum of the flow contributions made to it, and a probe of that sum is
    /// how a compact model reads back its own terminal current. The sum is in
    /// hand at the read point, so a consumer that computes everything from this
    /// CFG inlines it — see
    /// [`CfgLowerer::contributed_flow`](super::cfg_lower) — and differentiates
    /// through it like any other expression.
    ///
    /// A consumer that does *not* compute everything from this CFG cannot. An
    /// executable plan keeps the shipped route's contribution order and its
    /// per-contribution current storage, and reads a completed contribution out
    /// of that storage: the value is whatever the runtime holds, so it has no
    /// derivative, and this leaf is how the CFG says so. Its own derivative is
    /// zero, which is exactly what "frozen" means, and no derivative rule is
    /// needed for it — [`super::ad`] seeds only the kinds it lists.
    ///
    /// `pos`/`neg` name the probe's endpoints as written, in canonical node
    /// numbering with `None` for ground, because a reversed probe is the
    /// negation of the forward one and the reader owns which orientation its
    /// storage keeps. `through` is the last contribution the walk had completed
    /// when the probe was taken: it is what makes `I(a, b)` read after the
    /// contributions see them and the same probe read before them see zero,
    /// without the leaf having to be pinned to a block.
    ///
    /// The translation to storage indices is deliberately *not* here.
    /// See `CfgRuntimeBindings` in `crate::native::cfg_program`.
    ContributedCurrent {
        pos: Option<NodeId>,
        neg: Option<NodeId>,
        through: ContributionId,
    },
    /// Unit-amplitude representative of one semantic noise process.
    /// Its large-signal value is identically zero; AD uses the identity to
    /// recover coherent routing gains without changing DC/transient behavior.
    NoiseProcess(u32),
    /// Time derivative of `input`, keyed by the source operator so its state
    /// slot survives any reordering the passes do.
    Ddt {
        operator: ExprId,
        input: ValueId,
    },
    /// The integration rule's `d/dt` coefficient for the current step.
    ///
    /// `ddt` differentiates to this times the derivative of its input, rather
    /// than to another `ddt`: a second `ddt` would claim a second state slot for
    /// a quantity that has no history of its own.
    DdtScale,
    /// `idt(x, ic)` — the time integral of `x`, in companion form.
    ///
    /// Carries its initial condition rather than defaulting it, because a step
    /// with no history to integrate from returns `ic` and not zero, and the
    /// operator id because the running total is a per-instance slot keyed by the
    /// call — the same reason [`Self::Ddt`] carries one.
    Idt {
        operator: ExprId,
        input: ValueId,
        ic: ValueId,
    },
    /// The integration rule's `dt` for the current step, and zero where there is
    /// no step to integrate over.
    ///
    /// `idt` differentiates to this times the derivative of its input, for the
    /// reason [`Self::DdtScale`] exists: a second `idt` would claim a second
    /// slot for a quantity with no history of its own.
    IdtScale,
    /// `idtmod(x, ic, modulus, offset)` — the time integral of `x` folded into
    /// the half-open interval `[offset, offset + modulus)`.
    ///
    /// A separate kind from [`Self::Idt`] rather than a flag on it. The wrap is
    /// not post-processing of an integral: the runtime translates the *history*
    /// onto the wrapped candidate's branch before integrating, which is why the
    /// VM keeps a distinct older-history lane for it (`state_older_candidate`
    /// documents exactly that). A consumer that read an `Idt` and applied a
    /// modulo afterwards would drift by a whole period every time the branch
    /// changed between steps.
    ///
    /// Keyed by the operator, like [`Self::Idt`]: the running total is a
    /// per-instance slot named by the call.
    ///
    /// ## State (W-B inventory)
    ///
    /// One integration slot per operator, in the same family `ddt`/`idt` draw
    /// from: `state_values_prev`, `state_values_older`,
    /// `state_derivatives_prev` and `state_initialized`, one `f64`/`bool` each,
    /// keyed by the dense slot the backend assigns to this `operator`.
    IdtMod {
        operator: ExprId,
        input: ValueId,
        ic: ValueId,
        modulus: ValueId,
        offset: ValueId,
    },
    /// `absdelay(x, delay, max_delay)` — transport delay.
    ///
    /// `max_delay` stays optional rather than defaulting to zero: absent, the
    /// LRM lets the buffer grow to whatever the delay asks for, and zero is a
    /// bound, not the absence of one. Collapsing the two would silently truncate
    /// a model that omitted the argument.
    ///
    /// ## State (W-B inventory)
    ///
    /// One transport buffer per operator: a `(time, value)` sample deque whose
    /// length is bounded by `max_delay` where one is given and by the observed
    /// delay where none is, plus the buffer's configuration and one speculative
    /// Newton candidate. Accepted samples are checkpointed; the candidate never
    /// is.
    AbsDelay {
        operator: ExprId,
        input: ValueId,
        delay: ValueId,
        max_delay: Option<ValueId>,
    },
    /// Exact local Jacobian action of one `absdelay` candidate.
    ///
    /// Both the input and the delay are differentiated, because a delay that
    /// depends on an unknown moves the sample point and the interpolation
    /// through it: this is the one dynamic operator whose *timing* operand is
    /// not primal-only.
    ///
    /// `order` is carried rather than inferred, mirroring the flat node's
    /// `derivative_order`. The runtime implements order one; a second
    /// differentiation produces order two here so that a consumer refuses it
    /// explicitly instead of emitting a Hessian that is silently zero.
    ///
    /// Shares [`Self::AbsDelay`]'s buffer — it reads the same history and
    /// allocates none of its own.
    AbsDelayDerivative {
        operator: ExprId,
        input: ValueId,
        input_derivative: ValueId,
        delay: ValueId,
        delay_derivative: ValueId,
        max_delay: Option<ValueId>,
        order: u8,
    },
    /// `slew(x, max_rise, max_fall)` — rate limiting.
    ///
    /// Only the rate-limited form reaches this kind. `slew(x)` with both rates
    /// omitted is an exact stateless passthrough by the LRM and stays lowered as
    /// its operand, which is what keeps a model that writes it free of a state
    /// slot it does not need.
    ///
    /// `max_rise` is therefore required and `max_fall` is not: the LRM's second
    /// argument is what makes this a limiter at all, and a falling rate without
    /// a rising one is not a form the operator has. An omitted `max_fall` is the
    /// rising rate's magnitude applied downwards.
    ///
    /// ## State (W-B inventory)
    ///
    /// One filter per operator holding the committed output and its time, plus
    /// one speculative candidate and its validity flag. Two `f64`s and a `bool`
    /// per lane; the committed lane is checkpointed, the candidate is not.
    Slew {
        operator: ExprId,
        input: ValueId,
        max_rise: ValueId,
        max_fall: Option<ValueId>,
    },
    /// Exact local Jacobian action of one `slew` candidate.
    ///
    /// The rates are differentiated as well as the input: a rate that depends on
    /// an unknown moves the clamp, and the runtime's local coefficient is a
    /// function of all three. An omitted `max_fall` is the negation of
    /// `max_rise` — materialised by the lowering rather than left implicit, so
    /// that the node carries the values the runtime multiplies.
    ///
    /// Shares [`Self::Slew`]'s filter and allocates none of its own.
    SlewDerivative {
        operator: ExprId,
        input: ValueId,
        input_derivative: ValueId,
        max_rise: ValueId,
        max_rise_derivative: ValueId,
        max_fall: ValueId,
        max_fall_derivative: ValueId,
    },
    /// `last_crossing(x, direction)` — the interpolated time of the most recent
    /// zero crossing, or a negative time before there has been one.
    ///
    /// `direction` is a value rather than a compile-time edge, matching
    /// [`Self::Cross`] and the JIT's own lowering: the same runtime encoding
    /// (`+1` rising, `-1` falling, `0` either) and the same freedom for a model
    /// to compute it. The operator spelling's edge keyword lowers to the
    /// corresponding constant, so the two source forms produce one node.
    ///
    /// Its value is a *time*, not an event level, which is why this is not a
    /// case of `Cross`: an omitted answer here is "no crossing yet" and reads
    /// as a negative time, where an omitted event level reads as false.
    ///
    /// ## State (W-B inventory)
    ///
    /// One detector per operator, drawn from the same family `cross` uses:
    /// committed `(value, time)` history, one speculative candidate, a validity
    /// flag, and a speculative refinement time the stepper reads. The committed
    /// lane is checkpointed; neither the candidate nor the refinement time is.
    LastCrossing {
        operator: ExprId,
        input: ValueId,
        direction: ValueId,
    },
    /// `laplace_zp`, `laplace_zd`, `laplace_np` or `laplace_nd` — a continuous
    /// linear filter applied to `input`.
    ///
    /// ## The coefficient contract
    ///
    /// The transfer function is carried as **folded constants**, not as SSA
    /// values, and the four spellings collapse to the two realizable forms in
    /// [`CfgLaplaceTransfer`].
    ///
    /// Constants because they cannot be anything else. The front end refuses a
    /// `laplace_*` whose coefficient list is not compile-time constant, by
    /// name, so a `Vec<ValueId>` form could hold nothing a `Vec<f64>` cannot —
    /// it would be a strictly wider representation with no wider meaning, and
    /// no backend to execute the extra width. It is also what makes the state
    /// *shape* static: the realization's order, and therefore the length of the
    /// state vector the checkpoint serializes, is a function of the polynomial
    /// degree. A runtime-valued coefficient list would make the size of a
    /// checkpoint record a runtime quantity.
    ///
    /// Two forms rather than four because the remaining two spellings are the
    /// same transfer function written differently: `laplace_zd` expands its
    /// zeros and `laplace_np` its poles into the corresponding polynomial, an
    /// exact and total compile-time step the executable IR already performs.
    /// Pole-zero form is *not* expanded, because the state-space realization is
    /// built from the roots directly and expanding first would lose that.
    ///
    /// ## State (W-B inventory)
    ///
    /// One state-space filter per operator: the realization's state vector, its
    /// older accepted copy, and its derivative lane, each of the realization's
    /// order. All three are checkpointed;
    /// [`crate::canonical_ir::state::CanonicalStateFamily::LaplaceFilter`]
    /// names the array.
    Laplace {
        operator: ExprId,
        input: ValueId,
        transfer: CfgLaplaceTransfer,
    },
    /// Exact local Jacobian action of one `laplace_*` candidate.
    ///
    /// Only the input is differentiated: the coefficients are constants, so the
    /// filter is linear in its operand and the action is the same filter driven
    /// by the operand's derivative. Shares [`Self::Laplace`]'s realization and
    /// allocates none of its own.
    LaplaceDerivative {
        operator: ExprId,
        input_derivative: ValueId,
        transfer: CfgLaplaceTransfer,
    },
    /// `zi_zp`, `zi_zd`, `zi_np` or `zi_nd` — a sampled-data filter whose input
    /// is read every `period` seconds and whose output holds between samples.
    ///
    /// ## The coefficient contract
    ///
    /// The opposite of [`Self::Laplace`]'s, and for the opposite reason: the
    /// coefficients are **SSA operands**, in the polynomial form the source
    /// wrote.
    ///
    /// The `zi_*` front end does not fold them. It accepts arbitrary
    /// expressions, the executable IR retains them as programs, and the runtime
    /// installs the evaluated values per instance the first time the filter is
    /// viewed — which is why the checkpoint serializes `num` and `den` at all,
    /// where a Laplace checkpoint serializes only state. Folding them here
    /// would refuse the parameterised filters the language admits and the
    /// shipped runtime already runs.
    ///
    /// What *is* compile-time is the polynomial's length, and that is what
    /// fixes the state shape: the sampled input and output histories are one
    /// element shorter than their coefficient lists, and both lists are
    /// syntactic. So the record's width is static even though its contents are
    /// not, which is the property the layout needs and the property the
    /// spelling of the coefficients was never the source of.
    ///
    /// The four spellings are kept as [`CfgZiPolynomial`]'s two forms per
    /// polynomial, matching the executable IR: a root list is expanded by the
    /// runtime, after the roots have values.
    ///
    /// ## State (W-B inventory)
    ///
    /// One sampled filter per operator: the frozen definition (coefficients,
    /// period, first-transition time), the committed input and output
    /// histories, the held output and its visible transition segment, the next
    /// sample index, and the accepted-time bookkeeping that lets the stepper
    /// land on the next edge. All of it is checkpointed; the speculative
    /// candidate for the current Newton pass is not.
    Zi {
        operator: ExprId,
        input: ValueId,
        numerator: CfgZiPolynomial,
        denominator: CfgZiPolynomial,
        period: ValueId,
        transition: ValueId,
        first_transition: ValueId,
        /// Whether this site is written directly into a contribution rather
        /// than into a variable. Verilog-AMS 2023 section 4.5.12 requires a
        /// strictly positive transition time of a filter in that position, and
        /// the runtime enforces it, so the distinction is semantic and belongs
        /// on the node rather than in whatever lowered it.
        direct_assignment: bool,
    },
    /// Exact local Jacobian action of one `zi_*` candidate.
    ///
    /// Shares [`Self::Zi`]'s filter. Like the Laplace derivative it
    /// differentiates only the input — but for a weaker reason, since a
    /// coefficient here *can* depend on a parameter. It cannot depend on an
    /// unknown: the definition is frozen once per instance, before the first
    /// Newton pass, so with respect to the solve it is a constant.
    ZiDerivative {
        operator: ExprId,
        input_derivative: ValueId,
        numerator: CfgZiPolynomial,
        denominator: CfgZiPolynomial,
        period: ValueId,
        transition: ValueId,
        first_transition: ValueId,
        direct_assignment: bool,
    },
    /// `cross(expr, direction, time_tol, expr_tol, enable)`, evaluated from
    /// accepted detector history into a speculative candidate lane.
    Cross {
        operator: ExprId,
        input: ValueId,
        direction: ValueId,
        time_tol: ValueId,
        expr_tol: ValueId,
        enable: ValueId,
    },
    /// `above(expr, time_tol, expr_tol, enable)`, including its initial-positive
    /// equilibrium behavior.
    Above {
        operator: ExprId,
        input: ValueId,
        time_tol: ValueId,
        expr_tol: ValueId,
        enable: ValueId,
    },
    /// `timer(start, period, time_tol, enable)`, which also requests the next
    /// exact transient breakpoint.
    Timer {
        operator: ExprId,
        start: ValueId,
        period: ValueId,
        time_tol: ValueId,
        enable: ValueId,
    },
    /// Newton limiting; carries an affine correction lane in the derivative
    /// pass whether or not `proposed` depends on an unknown.
    ///
    /// `candidate` is the inlined limiter body, not an alternative value:
    /// evaluating it *is* running the limiter. Its implicit arguments reach it
    /// as [`Self::LimitPrevious`] and as `proposed` itself.
    Limit {
        operator: ExprId,
        proposed: ValueId,
        candidate: ValueId,
        /// Which limiting function the source named, e.g. `pnjlim`.
        selector: smol_str::SmolStr,
    },
    /// `ddx(value, probe)` — one entry of the Jacobian, read back into the
    /// model.
    ///
    /// Left symbolic here because the lanes it names do not exist until the
    /// derivative pass runs; that pass replaces it with the lane itself. A
    /// model that reads its own transconductance is asking for a value the
    /// backend already computes, and this is how it gets the same one.
    Ddx {
        value: ValueId,
        axis: CfgDdxAxis,
    },
    /// The value this `$limit` returned on the previous Newton iteration.
    ///
    /// Equal to `proposed` whenever there is no previous iterate to speak of,
    /// which is what makes a static probe of a limited model well defined.
    LimitPrevious {
        operator: ExprId,
        proposed: ValueId,
    },
    Unary {
        op: CfgUnaryOp,
        input: ValueId,
    },
    Binary {
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    /// `&`, `|`, `^`, `<<` or `>>` on two analog `integer` operands.
    ///
    /// Its result is [`CfgValueType::Real`] because the analog half carries
    /// `integer` in an `f64` — that ABI is frozen and the discrete domain's
    /// [`CfgValueType::Integer`] is a different type for a different half of the
    /// language. What this node adds is the *operation*: the rounding, the
    /// 32-bit wrap and the zero fill all happen inside it rather than in
    /// whatever the consumer's host language does to a double.
    ///
    /// Piecewise constant, so its derivative is structurally zero — the same
    /// answer a comparison gets, and for the same reason.
    IntegerBitwise {
        op: CfgIntegerBitwiseOp,
        left: ValueId,
        right: ValueId,
    },
    /// `~x` on one analog `integer` operand.
    ///
    /// Its own kind rather than an [`CfgIntegerBitwiseOp`] with a dummy operand,
    /// for the reason [`Self::Unary`] is not [`Self::Binary`]: an arity that has
    /// to be checked is an arity that can be got wrong.
    IntegerBitwiseNot {
        input: ValueId,
    },

    // ---- Packed derivative lanes -------------------------------------------
    //
    // A derivative is a partial with respect to each unknown it can reach, and
    // giving each of those its own SSA value costs one emitted line per lane.
    // These kinds carry the whole set as one value instead. The set — the
    // value's *shape* — is exactly what the liveness analysis proved reachable,
    // so packing costs no arithmetic on lanes that are structurally zero: the
    // corpus averages 3-5 live lanes against 30-odd unknowns, and a uniform
    // width would multiply the work by that ratio.
    /// Every lane of this value's shape set to one constant. The seed for an
    /// unknown is a one-lane shape holding `1.0`; a merge that one arm cannot
    /// reach passes a zero.
    LaneSplat(f64),
    /// `input` re-laid-out into this value's shape, zero in lanes it lacks.
    ///
    /// Its shape must be a superset of the input's, which is what an operand
    /// reaching a merge or a wider consumer needs.
    LaneWiden {
        input: ValueId,
    },
    /// Elementwise over two packed values that share this value's shape.
    /// Only [`CfgBinaryOp::Add`] and [`CfgBinaryOp::Sub`] appear here.
    LaneBinary {
        op: CfgBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    /// Every lane of `input` combined with one scalar — the chain rule's
    /// multiplier. Only [`CfgBinaryOp::Mul`] and [`CfgBinaryOp::Div`] appear,
    /// and the scalar is always on the right so the division is the one the
    /// quotient rule wrote.
    LaneScalar {
        op: CfgBinaryOp,
        input: ValueId,
        scalar: ValueId,
    },
    /// One lane of a packed value, named by the unknown rather than by its
    /// position, so a reader does not have to know the shape's layout.
    LaneExtract {
        input: ValueId,
        lane: u32,
    },

    /// A value a coarser invalidation stage computed and cached.
    ///
    /// What one slice of a split body reads from another. It is a leaf here on
    /// purpose: within a stage the value has no derivation, which is exactly
    /// what makes the stage a function of its own inputs and cacheable
    /// independently.
    Staged {
        slot: u32,
    },

    // ---- Discrete domain ---------------------------------------------------
    //
    // These appear only in a digital process function. None of them is
    // differentiable, none is a function of a solver unknown, and none may
    // reach a residual — see `CfgValueType::is_digital`, which is what the
    // analog passes assert against.
    //
    // The operators are not folded into `Binary`. `CfgBinaryOp::Add` is
    // addition on the reals; four-state addition is a different function with
    // a different domain, whose answer to `1 + x` is `x` rather than a number.
    // One enum for both would put that distinction behind an operand type
    // check in every consumer.
    /// A four-state literal, already decoded to its planes.
    FourStateConstant(digital_value::FourStateValue),
    /// A signed 32-bit integer constant.
    IntegerConstant(i32),
    /// The current value of a declared net or variable.
    ///
    /// A leaf: within one process function a signal has no derivation. Two
    /// reads of the same signal on either side of a `Wait` are two of these
    /// and may differ, which is why they are not common-subexpressioned.
    DigitalSignalRead {
        signal: DigitalSignalId,
    },
    /// The current value of a real net (Verilog-AMS LRM 2.4 section 3.7).
    ///
    /// The real-valued twin of [`Self::DigitalSignalRead`], and a separate kind
    /// rather than the same one reinterpreted by the signal's declaration. A
    /// consumer that had to look the signal up to learn what type the node
    /// produces would be one lookup away from producing the wrong one, and this
    /// node's type — [`CfgValueType::Real`] — has to be readable from the node.
    ///
    /// A leaf in the same sense and not in the `is_leaf_kind` sense, for the
    /// same reason its four-state twin is not: two reads on either side of a
    /// `Wait` are meant to differ.
    DigitalRealSignalRead {
        signal: DigitalSignalId,
    },
    /// A continuous-domain potential, read from inside a process function.
    ///
    /// Verilog-AMS LRM 2.4 section 7.3.3's probe, and the *only* direction the
    /// section allows without qualification: section 7.3 permits reads from
    /// either context and writes from neither but the value's own, so there is
    /// no contributing twin of this node and there never will be.
    ///
    /// `is_digital`, and produces a [`CfgValueType::Real`], which is what makes
    /// it compose with the rest of the discrete-domain real machinery — a
    /// probe is an operand of [`Self::DigitalRealArithmetic`] and
    /// [`Self::DigitalRealCompare`] exactly as a `wreal` read is. It is
    /// deliberately *not* [`Self::NodePotential`] with a flag: that node is
    /// the analog body's, is differentiated by the AD pass, and is scheduled
    /// in the Newton class. This one is read by an interpreter that has no
    /// derivatives and no Newton loop.
    ///
    /// A leaf in the same sense [`Self::DigitalSignalRead`] is, and not in the
    /// `is_leaf_kind` sense, for the same reason: two probes on either side of
    /// a `Wait` are two samples of a moving quantity and are meant to differ,
    /// so the node stays pinned to its block.
    DigitalAnalogPotential {
        probe: DigitalAnalogProbeId,
    },
    /// Arithmetic over two real values, inside a process function.
    ///
    /// Distinct from [`Self::Binary`], which is the analog body's arithmetic on
    /// the same `f64`s. They compute the same numbers and belong to different
    /// halves of the language: this one is `is_digital`, is classified by
    /// `leaf_class`, and is refused by the analog emitter — none of which is
    /// true of `Binary`, and all of which is what keeps a real *net* from being
    /// mistaken for an analog quantity somewhere downstream.
    DigitalRealArithmetic {
        op: digital_value::RealArithmeticOp,
        left: ValueId,
        right: ValueId,
    },
    /// A comparison between two real values, yielding one unsigned bit.
    ///
    /// The result is [`CfgValueType::FourState`] of width one, not
    /// [`CfgValueType::Boolean`]: IEEE 1364-2005 section 5.4.2 rule (g) makes
    /// every comparison in the discrete domain a one-bit unsigned value, and
    /// the rest of the process machinery — `Branch`, `&&`, an assignment to a
    /// `reg` — reads exactly that. Producing a `Boolean` here would need a
    /// bridge node whose only job was to undo the mistake.
    DigitalRealCompare {
        op: digital_value::RealCompareOp,
        left: ValueId,
        right: ValueId,
    },
    /// `condition ? then_value : else_value` over two real arms.
    ///
    /// Verilog-AMS LRM 2.4 table 4-2 makes `?:` legal in a real expression, and
    /// it is the operator a real-number model is built out of — a rung of a
    /// ladder, a mux, a saturation.
    ///
    /// The condition is four-state, so it can be ambiguous, and the two
    /// standards answer different questions about that. IEEE 1364-2005 section
    /// 5.1.13 combines the arms bit by bit when the condition is `x` or `z`,
    /// which has no real-valued form: a real has no bits to combine. So the
    /// rule this node carries is section 9.4's — an ambiguous condition is not
    /// true, and the `else` arm is the value — which is the rule the
    /// interpreter already applies to a `Branch`, and therefore the one that
    /// makes `assign y = c ? a : b;` and the `if`/`else` it stands for agree.
    DigitalRealSelect {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    /// `$realtobits(x)` — the IEEE 754 bit pattern of a real, 64 bits wide.
    ///
    /// The one direction the two value domains are allowed to meet in.
    /// Verilog-AMS LRM 2.4 section 3.7 says a `wreal` "cannot be connected to
    /// any other wires, although connection to explicitly declared 64-bit wires
    /// can be done via system tasks `$realtobits` and `$bitstoreal`", and IEEE
    /// 1364-2005's conversion functions define the pattern as the real's own
    /// storage rather than as a rounding. So this is not a numeric conversion
    /// and loses nothing: every `f64`, including a NaN and both infinities, has
    /// a bit pattern, and it is exactly the one `f64::to_bits` returns.
    ///
    /// The width is fixed at 64 and is not context-determined. It is a property
    /// of the double-precision format the standard names, not of the expression
    /// the call sits in; a call sized to its context would produce a different
    /// pattern in a narrower one and still call itself the conversion.
    DigitalRealToBits {
        input: ValueId,
    },
    /// `$bitstoreal(b)` — the real whose IEEE 754 pattern `b` is.
    ///
    /// The exact inverse of [`Self::DigitalRealToBits`] over the values that
    /// have one. What has none is a four-state value holding `x` or `z`: the
    /// standard defines the conversion over a *bit pattern*, and an unknown bit
    /// is the absence of one. Neither standard rules on that case, so this one
    /// does, and it refuses at runtime rather than substituting a bit — the
    /// same reading [`digital_value::FourStateValue::to_u64`] already takes,
    /// where returning a number for an unknown would answer the question with a
    /// lie.
    ///
    /// The operand is 64 bits. A narrower one is a different pattern, not a
    /// shorter spelling of this one, and the lowering resizes to 64 before the
    /// node the way section 5.2.1 resizes to any other operand width.
    DigitalBitsToReal {
        input: ValueId,
    },
    /// Elementwise bitwise operator over four-state values.
    DigitalBitwise {
        op: digital_value::BitwiseOp,
        left: ValueId,
        right: ValueId,
    },
    /// Bitwise negation (`~`).
    DigitalBitwiseNot {
        input: ValueId,
    },
    /// Logical operator (`&&`, `||`), yielding one bit.
    DigitalLogical {
        op: digital_value::LogicalOp,
        left: ValueId,
        right: ValueId,
    },
    /// Logical negation (`!`), yielding one bit.
    DigitalLogicalNot {
        input: ValueId,
    },
    /// Logical equality (`==`) or inequality (`!=`), yielding one bit, unknown
    /// if either operand has an `x`/`z` bit.
    DigitalEquality {
        left: ValueId,
        right: ValueId,
        negate: bool,
        /// Whether the comparison's own context is signed, IEEE 1364-2005
        /// section 5.4.2: both operands signed, or neither. It decides how the
        /// narrower operand reaches the wider one, and nothing else — the
        /// result is an unsigned bit either way.
        signed: bool,
    },
    /// A `case` item match, yielding one bit and never an unknown one.
    ///
    /// A different operator from [`Self::DigitalEquality`], not a use of it.
    /// IEEE 1364-2005 section 9.5 compares a case item against the selector
    /// bit by bit *including* `x` and `z`, so a selector of `x0` matches the
    /// item `2'bx0` — while `==` yields `x` there and would send it to the
    /// default. Section 9.5.1's `casez` and `casex` are the same comparison
    /// with a set of positions ignored, which is why the three forms are one
    /// node with a kind rather than three lowerings.
    DigitalCaseMatch {
        selector: ValueId,
        label: ValueId,
        kind: digital_value::DigitalCaseMatch,
        /// Whether the two expressions being matched are both signed, which
        /// decides how the narrower one is extended to the wider.
        signed: bool,
    },
    /// Relational comparison, yielding one bit.
    DigitalRelational {
        op: digital_value::RelationalOp,
        left: ValueId,
        right: ValueId,
        /// Whether the comparison is signed, IEEE 1364-2005 section 5.4.2.
        ///
        /// The one flag on this node that changes an answer rather than a
        /// representation: `-1 < 0` holds between two signed operands and does
        /// not the moment either side is unsigned.
        signed: bool,
    },
    /// Arithmetic on four-state values, all-unknown if any operand bit is.
    DigitalArithmetic {
        op: digital_value::ArithmeticOp,
        left: ValueId,
        right: ValueId,
        /// Whether the operands stand for signed numbers.
        ///
        /// Inert for `+`, `-` and `*`, whose bits are the same either way once
        /// the operands are at a common width — the whole of signed arithmetic
        /// lives in the extension that happens before them. `/` and `%` are
        /// where it decides the answer.
        signed: bool,
    },
    /// Shift, keeping the shifted value's width. Whether it fills with zero or
    /// with the sign bit is the operator's own, [`digital_value::ShiftOp`].
    DigitalShift {
        op: digital_value::ShiftOp,
        value: ValueId,
        count: ValueId,
    },
    /// A constant bit or part select. Bounds are as written, so `[0:7]` and
    /// `[7:0]` select the same bits of differently declared signals.
    DigitalPartSelect {
        input: ValueId,
        msb: i64,
        lsb: i64,
    },
    /// Concatenation. The first part supplies the most significant bits.
    DigitalConcat {
        parts: Vec<ValueId>,
    },
    /// `condition ? then_value : else_value` as a value rather than a branch.
    ///
    /// A conditional *expression* does not suspend, so making it a branch
    /// would split a block for no reason and force the result through a block
    /// parameter. Conditional *statements* still lower to real branches.
    DigitalSelect {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    /// A blocking write (`=`), visible to the next instruction.
    DigitalBlockingWrite {
        target: DigitalWriteTarget,
        value: ValueId,
    },
    /// A nonblocking write (`<=`), deferred to a later scheduling region.
    ///
    /// Kept a separate kind from the blocking write rather than a flag on one.
    /// The two have different execution semantics and different visibility,
    /// and every consumer has to treat them differently; a shared kind would
    /// make "did you check the flag?" a review question on each one.
    DigitalNonblockingWrite {
        target: DigitalWriteTarget,
        value: ValueId,
        /// Where the update lands. Always
        /// [`DigitalSchedulingRegion::NonBlockingAssign`] for a `<=` written
        /// in source; carried explicitly so the region is read off the node
        /// rather than assumed from the kind.
        region: DigitalSchedulingRegion,
    },
    /// A continuous driver's contribution to a net (IEEE 1364-2005 section
    /// 6.1).
    ///
    /// A third write kind, and not a flag on either of the others, because it
    /// does not write the net: it publishes *this driver's* value for it. A net
    /// with two drivers has one value per driver and a resolution function
    /// between them (section 7.9), and a write node that stored into the net
    /// would have destroyed the other driver's contribution before the
    /// resolver ever saw it. The driver identity is what keeps them apart, and
    /// it is on the node so that no consumer has to recover it by asking which
    /// process it is standing in.
    ///
    /// Resolution itself is not the compiler's: which value wins when two
    /// drivers disagree is a simulation-kernel rule over the whole net, and
    /// the kernel owns the table.
    DigitalDriverWrite {
        driver: DigitalDriverId,
        /// Which bits this driver drives. `driver.signal` and `target.signal`
        /// name the same net — the id says *which driver of it*, the target
        /// says *which of its bits*.
        target: DigitalWriteTarget,
        value: ValueId,
    },
}

impl CfgValueKind {
    /// Whether this kind belongs to the discrete-domain half of the language.
    ///
    /// Deliberately an exhaustive match with no catch-all, and the only one
    /// over this enum that exists purely to be exhaustive. Every other place a
    /// new kind has to be considered — `operands`, `leaf_class`,
    /// `static_dependencies`, the emitter's inlining predicates — has a
    /// catch-all that will absorb it in silence. This one will not compile
    /// until somebody says which half of the language the new kind is in, and
    /// the answer is what the scheduling, derivative, and emitter refusals are
    /// all keyed on.
    pub fn is_digital(&self) -> bool {
        match self {
            Self::RealConstant(_)
            | Self::BooleanConstant(_)
            | Self::BlockParameter
            | Self::Parameter(_)
            | Self::ParameterGiven(_)
            | Self::PortConnected(_)
            | Self::EventState(_)
            | Self::Temperature
            | Self::ThermalVoltage
            | Self::Multiplicity
            | Self::Time
            | Self::Analysis(_)
            | Self::SimParam { .. }
            | Self::NodePotential(_)
            | Self::BranchFlow(_)
            | Self::BranchUnknownFlow(_)
            | Self::ContributedCurrent { .. }
            | Self::NoiseProcess(_)
            | Self::Ddt { .. }
            | Self::DdtScale
            | Self::Idt { .. }
            | Self::IdtScale
            | Self::IdtMod { .. }
            | Self::AbsDelay { .. }
            | Self::AbsDelayDerivative { .. }
            | Self::Slew { .. }
            | Self::SlewDerivative { .. }
            | Self::LastCrossing { .. }
            | Self::Laplace { .. }
            | Self::LaplaceDerivative { .. }
            | Self::Zi { .. }
            | Self::ZiDerivative { .. }
            | Self::Cross { .. }
            | Self::Above { .. }
            | Self::Timer { .. }
            | Self::Limit { .. }
            | Self::Ddx { .. }
            | Self::LimitPrevious { .. }
            | Self::Unary { .. }
            | Self::Binary { .. }
            | Self::IntegerBitwise { .. }
            | Self::IntegerBitwiseNot { .. }
            | Self::LaneSplat(_)
            | Self::LaneWiden { .. }
            | Self::LaneBinary { .. }
            | Self::LaneScalar { .. }
            | Self::LaneExtract { .. }
            | Self::Staged { .. } => false,

            Self::FourStateConstant(_)
            | Self::IntegerConstant(_)
            | Self::DigitalSignalRead { .. }
            | Self::DigitalRealSignalRead { .. }
            | Self::DigitalAnalogPotential { .. }
            | Self::DigitalRealArithmetic { .. }
            | Self::DigitalRealCompare { .. }
            | Self::DigitalRealSelect { .. }
            | Self::DigitalRealToBits { .. }
            | Self::DigitalBitsToReal { .. }
            | Self::DigitalBitwise { .. }
            | Self::DigitalBitwiseNot { .. }
            | Self::DigitalLogical { .. }
            | Self::DigitalLogicalNot { .. }
            | Self::DigitalEquality { .. }
            | Self::DigitalCaseMatch { .. }
            | Self::DigitalRelational { .. }
            | Self::DigitalArithmetic { .. }
            | Self::DigitalShift { .. }
            | Self::DigitalPartSelect { .. }
            | Self::DigitalConcat { .. }
            | Self::DigitalSelect { .. }
            | Self::DigitalBlockingWrite { .. }
            | Self::DigitalNonblockingWrite { .. }
            | Self::DigitalDriverWrite { .. } => true,
        }
    }

    /// Which state-bearing operator this value reads, and how it names the
    /// record.
    ///
    /// The one definition of "this kind owns runtime state", so a new stateful
    /// kind becomes visible to the allocator by being added here rather than by
    /// being remembered in a second list. It must agree with
    /// [`super::schedule`]'s Newton arm — a kind that owns accepted history and
    /// is cached at a coarser scope computes a waveform once and freezes it —
    /// and `cfg_state_sites_and_newton_leaves_agree` pins that they do.
    ///
    /// Every stateful kind is keyed by the canonical expression that owns it.
    /// That is a body-copy name, so it needs
    /// [`super::hir::HirExecutedCorrespondence`] to reach the executed copy the
    /// runtime allocates in.
    pub fn state_site(&self) -> Option<CfgStateSite> {
        let site = match self {
            Self::Ddt { operator, .. } => CfgStateSite(*operator, CanonicalStateOperator::Ddt),
            Self::Idt { operator, .. } => CfgStateSite(*operator, CanonicalStateOperator::Idt),
            Self::IdtMod { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::IdtMod)
            }
            Self::AbsDelay { operator, .. } | Self::AbsDelayDerivative { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::Absdelay)
            }
            Self::Slew { operator, .. } | Self::SlewDerivative { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::Slew)
            }
            // `cross` and `last_crossing` share a detector, which is why the
            // layout gives them one family; `above` draws from the same array.
            Self::LastCrossing { operator, .. } | Self::Cross { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::Cross)
            }
            Self::Above { operator, .. } => CfgStateSite(*operator, CanonicalStateOperator::Above),
            Self::Laplace { operator, .. } | Self::LaplaceDerivative { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::Laplace)
            }
            Self::Zi { operator, .. } | Self::ZiDerivative { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::Zi)
            }
            Self::Timer { operator, .. } => CfgStateSite(*operator, CanonicalStateOperator::Timer),
            // `LimitPrevious` reads the same anchor its `Limit` writes and
            // carries the same operator id, so it resolves to one record.
            Self::Limit { operator, .. } | Self::LimitPrevious { operator, .. } => {
                CfgStateSite(*operator, CanonicalStateOperator::Limit)
            }
            _ => return None,
        };
        Some(site)
    }

    /// Values this one reads.
    ///
    /// Every pass that rewrites or walks the graph needs this, and having one
    /// definition of it is what keeps a newly added kind from being silently
    /// skipped by half of them.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Unary { input, .. }
            | Self::Ddt { input, .. }
            | Self::Ddx { value: input, .. }
            | Self::LaneWiden { input }
            | Self::LaneExtract { input, .. }
            | Self::IntegerBitwiseNot { input }
            | Self::LimitPrevious {
                proposed: input, ..
            } => vec![*input],
            Self::LastCrossing {
                input, direction, ..
            } => vec![*input, *direction],
            Self::SimParam { fallback, .. } => vec![*fallback],
            Self::Binary { left, right, .. }
            | Self::IntegerBitwise { left, right, .. }
            | Self::LaneBinary { left, right, .. } => {
                vec![*left, *right]
            }
            Self::LaneScalar { input, scalar, .. } => vec![*input, *scalar],
            Self::Idt { input, ic, .. } => vec![*input, *ic],
            Self::IdtMod {
                input,
                ic,
                modulus,
                offset,
                ..
            } => vec![*input, *ic, *modulus, *offset],
            Self::AbsDelay {
                input,
                delay,
                max_delay,
                ..
            } => {
                let mut operands = vec![*input, *delay];
                operands.extend(*max_delay);
                operands
            }
            Self::AbsDelayDerivative {
                input,
                input_derivative,
                delay,
                delay_derivative,
                max_delay,
                ..
            } => {
                let mut operands = vec![*input, *input_derivative, *delay, *delay_derivative];
                operands.extend(*max_delay);
                operands
            }
            Self::Slew {
                input,
                max_rise,
                max_fall,
                ..
            } => {
                let mut operands = vec![*input, *max_rise];
                operands.extend(*max_fall);
                operands
            }
            Self::SlewDerivative {
                input,
                input_derivative,
                max_rise,
                max_rise_derivative,
                max_fall,
                max_fall_derivative,
                ..
            } => vec![
                *input,
                *input_derivative,
                *max_rise,
                *max_rise_derivative,
                *max_fall,
                *max_fall_derivative,
            ],
            // Only the input is an operand. The transfer function is folded
            // constants, so there is nothing else in the node for a pass to
            // rename, and nothing for the scheduler to depend on.
            Self::Laplace { input, .. } => vec![*input],
            Self::LaplaceDerivative {
                input_derivative, ..
            } => vec![*input_derivative],
            // Every coefficient *is* an operand here, and in the order the
            // runtime reads them: numerator, denominator, then the timing.
            Self::Zi {
                input,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                ..
            } => {
                let mut operands = numerator.operands();
                operands.extend(denominator.operands());
                operands.extend([*period, *transition, *first_transition, *input]);
                operands
            }
            Self::ZiDerivative {
                input_derivative,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                ..
            } => {
                let mut operands = numerator.operands();
                operands.extend(denominator.operands());
                operands.extend([*period, *transition, *first_transition, *input_derivative]);
                operands
            }
            Self::Cross {
                input,
                direction,
                time_tol,
                expr_tol,
                enable,
                ..
            } => vec![*input, *direction, *time_tol, *expr_tol, *enable],
            Self::Above {
                input,
                time_tol,
                expr_tol,
                enable,
                ..
            } => vec![*input, *time_tol, *expr_tol, *enable],
            Self::Timer {
                start,
                period,
                time_tol,
                enable,
                ..
            } => vec![*start, *period, *time_tol, *enable],
            Self::Limit {
                proposed,
                candidate,
                ..
            } => vec![*proposed, *candidate],

            Self::DigitalBitwiseNot { input }
            | Self::DigitalLogicalNot { input }
            | Self::DigitalRealToBits { input }
            | Self::DigitalBitsToReal { input }
            | Self::DigitalPartSelect { input, .. } => vec![*input],
            Self::DigitalBitwise { left, right, .. }
            | Self::DigitalLogical { left, right, .. }
            | Self::DigitalEquality { left, right, .. }
            | Self::DigitalRelational { left, right, .. }
            | Self::DigitalRealArithmetic { left, right, .. }
            | Self::DigitalRealCompare { left, right, .. }
            | Self::DigitalArithmetic { left, right, .. } => vec![*left, *right],
            Self::DigitalCaseMatch {
                selector, label, ..
            } => vec![*selector, *label],
            Self::DigitalShift { value, count, .. } => vec![*value, *count],
            Self::DigitalConcat { parts } => parts.clone(),
            Self::DigitalSelect {
                condition,
                then_value,
                else_value,
            }
            | Self::DigitalRealSelect {
                condition,
                then_value,
                else_value,
            } => vec![*condition, *then_value, *else_value],
            Self::DigitalBlockingWrite { value, .. }
            | Self::DigitalNonblockingWrite { value, .. }
            | Self::DigitalDriverWrite { value, .. } => vec![*value],

            _ => Vec::new(),
        }
    }

    pub(crate) fn map_operands(&mut self, mut map: impl FnMut(ValueId) -> ValueId) {
        match self {
            Self::Unary { input, .. }
            | Self::Ddt { input, .. }
            | Self::Ddx { value: input, .. }
            | Self::LaneWiden { input }
            | Self::LaneExtract { input, .. }
            | Self::IntegerBitwiseNot { input }
            | Self::LimitPrevious {
                proposed: input, ..
            } => *input = map(*input),
            Self::LastCrossing {
                input, direction, ..
            } => {
                *input = map(*input);
                *direction = map(*direction);
            }
            Self::SimParam { fallback, .. } => *fallback = map(*fallback),
            Self::Binary { left, right, .. }
            | Self::IntegerBitwise { left, right, .. }
            | Self::LaneBinary { left, right, .. } => {
                *left = map(*left);
                *right = map(*right);
            }
            Self::LaneScalar { input, scalar, .. } => {
                *input = map(*input);
                *scalar = map(*scalar);
            }
            Self::Idt { input, ic, .. } => {
                *input = map(*input);
                *ic = map(*ic);
            }
            Self::IdtMod {
                input,
                ic,
                modulus,
                offset,
                ..
            } => {
                *input = map(*input);
                *ic = map(*ic);
                *modulus = map(*modulus);
                *offset = map(*offset);
            }
            Self::AbsDelay {
                input,
                delay,
                max_delay,
                ..
            } => {
                *input = map(*input);
                *delay = map(*delay);
                if let Some(max_delay) = max_delay {
                    *max_delay = map(*max_delay);
                }
            }
            Self::AbsDelayDerivative {
                input,
                input_derivative,
                delay,
                delay_derivative,
                max_delay,
                ..
            } => {
                *input = map(*input);
                *input_derivative = map(*input_derivative);
                *delay = map(*delay);
                *delay_derivative = map(*delay_derivative);
                if let Some(max_delay) = max_delay {
                    *max_delay = map(*max_delay);
                }
            }
            Self::Slew {
                input,
                max_rise,
                max_fall,
                ..
            } => {
                *input = map(*input);
                *max_rise = map(*max_rise);
                if let Some(max_fall) = max_fall {
                    *max_fall = map(*max_fall);
                }
            }
            Self::SlewDerivative {
                input,
                input_derivative,
                max_rise,
                max_rise_derivative,
                max_fall,
                max_fall_derivative,
                ..
            } => {
                *input = map(*input);
                *input_derivative = map(*input_derivative);
                *max_rise = map(*max_rise);
                *max_rise_derivative = map(*max_rise_derivative);
                *max_fall = map(*max_fall);
                *max_fall_derivative = map(*max_fall_derivative);
            }
            Self::Laplace { input, .. } => *input = map(*input),
            Self::LaplaceDerivative {
                input_derivative, ..
            } => *input_derivative = map(*input_derivative),
            Self::Zi {
                input,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                ..
            } => {
                *input = map(*input);
                numerator.map_operands(&mut map);
                denominator.map_operands(&mut map);
                *period = map(*period);
                *transition = map(*transition);
                *first_transition = map(*first_transition);
            }
            Self::ZiDerivative {
                input_derivative,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                ..
            } => {
                *input_derivative = map(*input_derivative);
                numerator.map_operands(&mut map);
                denominator.map_operands(&mut map);
                *period = map(*period);
                *transition = map(*transition);
                *first_transition = map(*first_transition);
            }
            Self::Cross {
                input,
                direction,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                *input = map(*input);
                *direction = map(*direction);
                *time_tol = map(*time_tol);
                *expr_tol = map(*expr_tol);
                *enable = map(*enable);
            }
            Self::Above {
                input,
                time_tol,
                expr_tol,
                enable,
                ..
            } => {
                *input = map(*input);
                *time_tol = map(*time_tol);
                *expr_tol = map(*expr_tol);
                *enable = map(*enable);
            }
            Self::Timer {
                start,
                period,
                time_tol,
                enable,
                ..
            } => {
                *start = map(*start);
                *period = map(*period);
                *time_tol = map(*time_tol);
                *enable = map(*enable);
            }
            Self::Limit {
                proposed,
                candidate,
                ..
            } => {
                *proposed = map(*proposed);
                *candidate = map(*candidate);
            }

            Self::DigitalBitwiseNot { input }
            | Self::DigitalLogicalNot { input }
            | Self::DigitalRealToBits { input }
            | Self::DigitalBitsToReal { input }
            | Self::DigitalPartSelect { input, .. } => *input = map(*input),
            Self::DigitalBitwise { left, right, .. }
            | Self::DigitalLogical { left, right, .. }
            | Self::DigitalEquality { left, right, .. }
            | Self::DigitalRelational { left, right, .. }
            | Self::DigitalRealArithmetic { left, right, .. }
            | Self::DigitalRealCompare { left, right, .. }
            | Self::DigitalArithmetic { left, right, .. } => {
                *left = map(*left);
                *right = map(*right);
            }
            Self::DigitalCaseMatch {
                selector, label, ..
            } => {
                *selector = map(*selector);
                *label = map(*label);
            }
            Self::DigitalShift { value, count, .. } => {
                *value = map(*value);
                *count = map(*count);
            }
            Self::DigitalConcat { parts } => {
                for part in parts {
                    *part = map(*part);
                }
            }
            Self::DigitalSelect {
                condition,
                then_value,
                else_value,
            }
            | Self::DigitalRealSelect {
                condition,
                then_value,
                else_value,
            } => {
                *condition = map(*condition);
                *then_value = map(*then_value);
                *else_value = map(*else_value);
            }
            Self::DigitalBlockingWrite { value, .. }
            | Self::DigitalNonblockingWrite { value, .. }
            | Self::DigitalDriverWrite { value, .. } => *value = map(*value),

            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CfgValueType {
    Real,
    Boolean,
    /// A packed derivative, one `f64` per unknown in the named shape.
    Lanes(ShapeId),
    /// A signed 32-bit two's-complement integer.
    ///
    /// Distinct from [`Self::Real`] rather than tunnelled through it. The
    /// analog half does carry `integer` in an `f64` — that ABI is frozen and
    /// is not what this is — but a discrete-domain index, shift count, or
    /// delay is an integer whose wrapping and division behaviour is defined,
    /// and rounding it out of a float at each use is how those definitions get
    /// lost.
    Integer,
    /// A four-state value of a fixed width, held as `aval`/`bval` planes.
    ///
    /// The width is part of the type because almost every IEEE 1364-2005
    /// operator is width-sensitive: an assignment truncates or zero-extends to
    /// the target's width, and arithmetic that overflows the operand width
    /// wraps within it. A four-state value whose width is not known statically
    /// is not something this IR can represent, which is deliberate.
    FourState {
        width: u32,
    },
    /// Carries ordering, not data.
    ///
    /// The type of a signal write. A write has to sit in the instruction
    /// stream — its position is its execution order — but nothing may read
    /// what it "produces", and giving it the written value's type would invite
    /// exactly that. A nonblocking write in particular has no readable result
    /// at the point it appears: its update is not visible until the
    /// nonblocking region flushes.
    Effect,
}

impl CfgValueType {
    pub fn shape(self) -> Option<ShapeId> {
        match self {
            Self::Lanes(shape) => Some(shape),
            Self::Real | Self::Boolean | Self::Integer | Self::FourState { .. } | Self::Effect => {
                None
            }
        }
    }

    /// Whether this type belongs to the discrete-domain half of the language.
    ///
    /// The analog solver has no representation for any of these, so a value
    /// carrying one reaching a residual, a Jacobian lane, or a stage cache is
    /// a compiler bug rather than an unsupported model.
    pub const fn is_digital(self) -> bool {
        matches!(self, Self::Integer | Self::FourState { .. } | Self::Effect)
    }

    /// Declared width in bits, for the types that have one.
    pub const fn width(self) -> Option<u32> {
        match self {
            Self::FourState { width } => Some(width),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgValue {
    pub id: ValueId,
    pub value_type: CfgValueType,
    pub kind: CfgValueKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgInstruction {
    pub result: ValueId,
}

/// How a block ends. Every block has exactly one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CfgTerminator {
    Jump {
        target: BlockId,
        args: Vec<ValueId>,
    },
    Branch {
        condition: ValueId,
        then_target: BlockId,
        then_args: Vec<ValueId>,
        else_target: BlockId,
        else_args: Vec<ValueId>,
    },
    Return,
    /// Suspend a digital process here and resume into `resume` when the wait
    /// is satisfied.
    ///
    /// The suspension point the author wrote, in the position they wrote it.
    /// Resuming is a `Jump`: `resume_args` bind to `resume`'s block
    /// parameters, so whatever the process needs to survive the suspension
    /// travels through parameters like any other cross-block value, and no
    /// pass has to know that a process has state.
    ///
    /// Appears only in a digital process function. The analog body never
    /// suspends — a Newton iteration runs to completion or fails — so every
    /// analog pass treats this as unreachable rather than handling it.
    Wait {
        wait: DigitalWait,
        resume: BlockId,
        resume_args: Vec<ValueId>,
    },
    /// Placeholder while a block is under construction. Never present in a
    /// finished function; [`CfgFunction::validate`] rejects it.
    Unset,
}

/// How one CFG value names the runtime record it reads: the canonical
/// expression that owns the record, and the operator whose record it is.
///
/// See [`CfgValueKind::state_site`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CfgStateSite(pub ExprId, pub CanonicalStateOperator);

/// What a suspended process is waiting for.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalWait {
    /// `@(...)`: resume when any listed event occurs.
    ///
    /// An empty list would be a process that can never resume; the lowering
    /// refuses one rather than emitting it.
    Event(Vec<DigitalSensitivityTerm>),
    /// `#delay`: resume after this many time units have elapsed.
    ///
    /// The operand is an [`CfgValueType::Integer`] value, evaluated when the
    /// wait is reached rather than when the process starts — `#(n)` where `n`
    /// is a variable waits for its value at that moment.
    Delay(ValueId),
}

impl DigitalWait {
    /// Values this wait reads.
    pub fn operands(&self) -> Vec<ValueId> {
        match self {
            Self::Event(_) => Vec::new(),
            Self::Delay(delay) => vec![*delay],
        }
    }

    /// Rewrite the values this wait reads.
    ///
    /// The mutable twin of [`Self::operands`]. Every renumbering pass has to
    /// call it or a delay operand survives a compaction pointing at whatever
    /// value took its old index.
    pub fn map_operands(&mut self, mut map: impl FnMut(ValueId) -> ValueId) {
        match self {
            Self::Event(_) => {}
            Self::Delay(delay) => *delay = map(*delay),
        }
    }

    /// The signals whose events can resume the process.
    pub fn sensitivity(&self) -> &[DigitalSensitivityTerm] {
        match self {
            Self::Event(terms) => terms,
            Self::Delay(_) => &[],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgBlock {
    pub id: BlockId,
    /// Values this block defines by merging its predecessors' arguments.
    pub params: Vec<ValueId>,
    pub instructions: Vec<CfgInstruction>,
    pub terminator: CfgTerminator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CfgFunction {
    pub entry: BlockId,
    pub blocks: Vec<CfgBlock>,
    pub values: Vec<CfgValue>,
    /// The lane sets packed values are laid out over, each sorted and interned
    /// so a shape is compared by id. Empty until the derivative pass runs.
    #[serde(default)]
    pub shapes: Vec<Vec<u32>>,
}

impl CfgFunction {
    pub fn block(&self, id: BlockId) -> &CfgBlock {
        &self.blocks[usize::from(id)]
    }

    pub fn value(&self, id: ValueId) -> &CfgValue {
        &self.values[usize::from(id)]
    }

    /// The unknowns a shape carries, in layout order.
    pub fn shape(&self, id: ShapeId) -> &[u32] {
        &self.shapes[usize::from(id)]
    }

    /// The lane set of `value`, or `None` if it is an ordinary scalar.
    pub fn lanes_of(&self, value: ValueId) -> Option<&[u32]> {
        self.value(value)
            .value_type
            .shape()
            .map(|id| self.shape(id))
    }

    /// Where `lane` sits inside `value`'s shape.
    pub fn lane_position(&self, value: ValueId, lane: u32) -> Option<usize> {
        self.lanes_of(value)?
            .iter()
            .position(|candidate| *candidate == lane)
    }

    /// Blocks that jump or branch to `target`.
    pub fn predecessors(&self, target: BlockId) -> Vec<BlockId> {
        self.blocks
            .iter()
            .filter(|block| block.successors().contains(&target))
            .map(|block| block.id)
            .collect()
    }

    /// Structural checks a well-formed function must satisfy.
    ///
    /// Cheap enough to run after every construction, and worth it: an SSA bug
    /// that reaches the derivative pass presents as a wrong number rather than
    /// as a malformed graph, and is correspondingly harder to find.
    pub fn validate(&self) -> Result<(), CfgValidationError> {
        for block in &self.blocks {
            if matches!(block.terminator, CfgTerminator::Unset) {
                return Err(CfgValidationError::UnterminatedBlock(block.id));
            }
            for successor in block.successors() {
                let arg_count = block.arguments_to(successor).map_or(0, <[ValueId]>::len);
                let param_count = self.block(successor).params.len();
                if arg_count != param_count {
                    return Err(CfgValidationError::ArgumentCountMismatch {
                        from: block.id,
                        to: successor,
                        expected: param_count,
                        found: arg_count,
                    });
                }
            }
        }

        // Every value must be defined exactly once, either as a block parameter
        // or by an instruction.
        let mut defined = vec![0usize; self.values.len()];
        for block in &self.blocks {
            for param in &block.params {
                defined[usize::from(*param)] += 1;
            }
            for instruction in &block.instructions {
                defined[usize::from(instruction.result)] += 1;
            }
        }
        for (index, count) in defined.iter().enumerate() {
            let id = ValueId::from(index);
            match (count, &self.values[index].kind) {
                (1, _) => {}
                (0, CfgValueKind::BlockParameter) => {
                    return Err(CfgValidationError::UndefinedValue(id));
                }
                // Constants and leaves may be referenced without being placed in
                // a block; everything else must be scheduled somewhere.
                (0, kind) if is_leaf(kind) => {}
                (0, _) => return Err(CfgValidationError::UndefinedValue(id)),
                _ => return Err(CfgValidationError::MultiplyDefinedValue(id)),
            }
        }

        self.validate_shapes()
    }

    /// Packed values agree with their operands about which unknowns they carry.
    ///
    /// Worth checking because the failure it catches is otherwise silent: a
    /// derivative laid out over the wrong lane set still evaluates, and reports
    /// one unknown's partial in another's slot.
    fn validate_shapes(&self) -> Result<(), CfgValidationError> {
        for value in &self.values {
            let lanes = self.value_lanes(value.id);
            match &value.kind {
                CfgValueKind::LaneSplat(_) | CfgValueKind::BlockParameter => {}
                CfgValueKind::LaneWiden { input } => {
                    // A superset, not equality: widening is how a narrow operand
                    // reaches a merge that other arms made wider.
                    let source = self.value_lanes(*input);
                    if !source.iter().all(|lane| lanes.contains(lane)) {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                CfgValueKind::LaneBinary { left, right, .. } => {
                    if self.value_lanes(*left) != lanes || self.value_lanes(*right) != lanes {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                CfgValueKind::LaneScalar { input, scalar, .. } => {
                    if self.value_lanes(*input) != lanes
                        || self.value(*scalar).value_type.shape().is_some()
                    {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                CfgValueKind::LaneExtract { input, lane } => {
                    if !lanes.is_empty() || !self.value_lanes(*input).contains(lane) {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
                // The stateful operators' Jacobian actions, which are the one
                // family whose operands are deliberately of two kinds: the
                // primal ones the runtime evaluates its local coefficient from,
                // which are always scalar, and the derivative ones it
                // multiplies, which carry this value's own lanes.
                //
                // They cannot go through the catch-all below, which reads any
                // packed operand as a mistake — correctly, for arithmetic, and
                // wrongly for these.
                CfgValueKind::AbsDelayDerivative {
                    input,
                    input_derivative,
                    delay,
                    delay_derivative,
                    max_delay,
                    ..
                } => {
                    let mut primal = vec![*input, *delay];
                    primal.extend(*max_delay);
                    self.validate_stateful_derivative(
                        value.id,
                        lanes,
                        &primal,
                        &[*input_derivative, *delay_derivative],
                    )?;
                }
                CfgValueKind::SlewDerivative {
                    input,
                    input_derivative,
                    max_rise,
                    max_rise_derivative,
                    max_fall,
                    max_fall_derivative,
                    ..
                } => {
                    self.validate_stateful_derivative(
                        value.id,
                        lanes,
                        &[*input, *max_rise, *max_fall],
                        &[
                            *input_derivative,
                            *max_rise_derivative,
                            *max_fall_derivative,
                        ],
                    )?;
                }
                // The filters' actions have only the one derivative operand,
                // because they are linear and their coefficients are constants
                // of the solve. The `zi` coefficients and timing are primal
                // operands of the same node, so they go on the scalar side.
                CfgValueKind::LaplaceDerivative {
                    input_derivative, ..
                } => {
                    self.validate_stateful_derivative(value.id, lanes, &[], &[*input_derivative])?;
                }
                CfgValueKind::ZiDerivative {
                    input_derivative,
                    numerator,
                    denominator,
                    period,
                    transition,
                    first_transition,
                    ..
                } => {
                    let mut primal = numerator.operands();
                    primal.extend(denominator.operands());
                    primal.extend([*period, *transition, *first_transition]);
                    self.validate_stateful_derivative(
                        value.id,
                        lanes,
                        &primal,
                        &[*input_derivative],
                    )?;
                }
                // Every other kind is scalar arithmetic, and a packed operand
                // reaching one is the mistake this catches.
                kind => {
                    if !lanes.is_empty()
                        || kind
                            .operands()
                            .into_iter()
                            .any(|operand| self.value(operand).value_type.shape().is_some())
                    {
                        return Err(CfgValidationError::LaneShapeMismatch(value.id));
                    }
                }
            }
        }
        Ok(())
    }

    /// A stateful operator's Jacobian action agrees with its two operand
    /// classes.
    ///
    /// `primal` operands are what the runtime evaluates the local coefficient
    /// from and are always scalar; `derivative` operands are what it multiplies
    /// and carry exactly this value's lanes. The scalar `ddx` shadow form falls
    /// out of the same rule with an empty lane set on both sides.
    fn validate_stateful_derivative(
        &self,
        value: ValueId,
        lanes: &[u32],
        primal: &[ValueId],
        derivative: &[ValueId],
    ) -> Result<(), CfgValidationError> {
        if primal
            .iter()
            .any(|operand| self.value(*operand).value_type.shape().is_some())
        {
            return Err(CfgValidationError::LaneShapeMismatch(value));
        }
        if derivative
            .iter()
            .any(|operand| self.value_lanes(*operand) != lanes)
        {
            return Err(CfgValidationError::LaneShapeMismatch(value));
        }
        Ok(())
    }

    fn value_lanes(&self, value: ValueId) -> &[u32] {
        self.lanes_of(value).unwrap_or(&[])
    }
}

/// Whether a value may be referenced without any block defining it.
///
/// Also what decides, when a body is split by invalidation class, whether a
/// value crossing a stage boundary needs a cache slot or is simply rebuilt: a
/// leaf reads nothing, so rebuilding it is free and costs no slot.
pub(super) fn is_leaf_kind(kind: &CfgValueKind) -> bool {
    is_leaf(kind)
}

fn is_leaf(kind: &CfgValueKind) -> bool {
    matches!(
        kind,
        CfgValueKind::RealConstant(_)
            | CfgValueKind::BooleanConstant(_)
            | CfgValueKind::Parameter(_)
            | CfgValueKind::ParameterGiven(_)
            | CfgValueKind::PortConnected(_)
            | CfgValueKind::EventState(_)
            | CfgValueKind::Temperature
            | CfgValueKind::ThermalVoltage
            | CfgValueKind::Multiplicity
            | CfgValueKind::Time
            | CfgValueKind::Analysis(_)
            | CfgValueKind::DdtScale
            | CfgValueKind::IdtScale
            | CfgValueKind::NodePotential(_)
            | CfgValueKind::BranchFlow(_)
            | CfgValueKind::BranchUnknownFlow(_)
            | CfgValueKind::ContributedCurrent { .. }
            | CfgValueKind::NoiseProcess(_)
            | CfgValueKind::LaneSplat(_)
            | CfgValueKind::Staged { .. }
            // Digital constants are leaves for the same reason a real constant
            // is: they read nothing and rebuilding one is free.
            //
            // `DigitalSignalRead` is deliberately *not* here. It reads nothing
            // in the operand sense, but it is not a constant — its value is
            // whatever the signal holds when the node runs, and two reads on
            // either side of a `Wait` are meant to differ. It has to stay
            // pinned to a block so that ordering survives.
            | CfgValueKind::FourStateConstant(_)
            | CfgValueKind::IntegerConstant(_)
    )
}

impl CfgBlock {
    pub fn successors(&self) -> Vec<BlockId> {
        match &self.terminator {
            CfgTerminator::Jump { target, .. } => vec![*target],
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                if then_target == else_target {
                    vec![*then_target]
                } else {
                    vec![*then_target, *else_target]
                }
            }
            // A suspension is an edge like any other. The process resumes into
            // `resume`, and every pass that walks the graph has to see that or
            // it will call the resumed half unreachable and delete it.
            CfgTerminator::Wait { resume, .. } => vec![*resume],
            CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
        }
    }

    fn arguments_to(&self, successor: BlockId) -> Option<&[ValueId]> {
        match &self.terminator {
            CfgTerminator::Jump { target, args } if *target == successor => Some(args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                ..
            } if *then_target == successor => Some(then_args),
            CfgTerminator::Branch {
                else_target,
                else_args,
                ..
            } if *else_target == successor => Some(else_args),
            // Resuming passes arguments exactly as a jump does, so a block
            // parameter filled in on sealing finds them here.
            CfgTerminator::Wait {
                resume,
                resume_args,
                ..
            } if *resume == successor => Some(resume_args),
            _ => None,
        }
    }

    fn push_argument(&mut self, successor: BlockId, value: ValueId) {
        match &mut self.terminator {
            CfgTerminator::Jump { target, args } if *target == successor => args.push(value),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                if *then_target == successor {
                    then_args.push(value);
                }
                if *else_target == successor {
                    else_args.push(value);
                }
            }
            CfgTerminator::Wait {
                resume,
                resume_args,
                ..
            } if *resume == successor => resume_args.push(value),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgValidationError {
    UnterminatedBlock(BlockId),
    ArgumentCountMismatch {
        from: BlockId,
        to: BlockId,
        expected: usize,
        found: usize,
    },
    UndefinedValue(ValueId),
    MultiplyDefinedValue(ValueId),
    LaneShapeMismatch(ValueId),
    /// A discrete-domain value reached the derivative pass.
    ///
    /// Not an unsupported model — a compiler bug. Nothing in a four-state
    /// value is differentiable: it has no neighbourhood, and `x` is not a
    /// point you can perturb. The derivative pass runs on the analog body, so
    /// one of these arriving means a process function was handed to it, and
    /// the pass says so instead of quietly contributing a zero row to the
    /// Jacobian.
    DigitalValueInDerivative(ValueId),
    NestedDdx(ValueId),
}

impl std::fmt::Display for CfgValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedBlock(block) => write!(f, "{block} has no terminator"),
            Self::ArgumentCountMismatch {
                from,
                to,
                expected,
                found,
            } => write!(
                f,
                "{from} passes {found} arguments to {to}, which takes {expected}"
            ),
            Self::UndefinedValue(value) => write!(f, "{value} is never defined"),
            Self::MultiplyDefinedValue(value) => write!(f, "{value} is defined more than once"),
            Self::LaneShapeMismatch(value) => {
                write!(f, "{value} does not carry the lanes its operands do")
            }
            Self::DigitalValueInDerivative(value) => write!(
                f,
                "{value} is a discrete-domain value and cannot be differentiated"
            ),
            Self::NestedDdx(value) => write!(
                f,
                "{value} applies ddx to a value that already depends on ddx; nested ddx is not supported"
            ),
        }
    }
}

impl std::error::Error for CfgValidationError {}

/// Incremental SSA construction over a CFG being built.
///
/// The caller drives it while walking structured statements: create blocks,
/// [`SsaBuilder::write_variable`] on assignment, [`SsaBuilder::read_variable`]
/// on use, and [`SsaBuilder::seal_block`] once a block's predecessors are all
/// known. Straight-line and `if`/`else` blocks can be sealed as soon as they are
/// created; a loop header cannot be sealed until its back edge exists.
pub struct SsaBuilder {
    blocks: Vec<CfgBlock>,
    values: Vec<CfgValue>,
    /// Latest definition of a variable within a block.
    definitions: HashMap<(CfgVariable, BlockId), ValueId>,
    /// Parameters created for a variable in a block that was not yet sealed.
    incomplete: Vec<(BlockId, CfgVariable, ValueId)>,
    sealed: HashSet<BlockId>,
    /// Predecessors recorded as edges are added, because a block under
    /// construction cannot be scanned for them.
    predecessors: HashMap<BlockId, Vec<BlockId>>,
    /// Values the caller needs after construction.
    ///
    /// They have to be held here rather than by the caller because finishing
    /// rewrites value ids twice — collapsing trivial merges, then compacting —
    /// and an id read out beforehand names something else afterwards.
    outputs: Vec<ValueId>,
    /// The type a variable's merges take, when it is not the default.
    ///
    /// Empty for the analog body, whose every variable is a `real` — an
    /// `integer` included, because that ABI carries one in an `f64`. A digital
    /// process declares each of its locals, and the merge the builder creates
    /// for one then carries the declared type instead of guessing.
    variable_types: HashMap<CfgVariable, CfgValueType>,
    /// Blocks a [`CfgTerminator::Wait`] resumes into.
    ///
    /// Their parameters are exempt from trivial-merge collapse. A resumption
    /// does not preserve the value table — the process suspended, and whatever
    /// it computed before the suspension is gone — so a parameter that
    /// "trivially" equals a value defined before the `Wait` is not trivial at
    /// all: collapsing it makes the resumed half read something nothing
    /// defines. This is the one place the analog and digital construction
    /// rules genuinely differ, and it is inert for the analog body because
    /// nothing there suspends.
    resume_targets: HashSet<BlockId>,
    /// Where a collapsed merge sends its readers, as a union-find parent list.
    ///
    /// Recorded rather than applied. Rewriting every use at the moment a merge
    /// collapses is a scan of the whole value table per collapse, and a model
    /// with thousands of merges spends nearly all of construction doing it —
    /// measured at 27 of 28 seconds on `DIODE_CMC`. Reads resolve through this,
    /// and one pass at the end fixes the operands that were recorded before the
    /// redirection existed.
    redirect: Vec<Option<ValueId>>,
}

impl Default for SsaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SsaBuilder {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            values: Vec::new(),
            definitions: HashMap::new(),
            incomplete: Vec::new(),
            sealed: HashSet::new(),
            predecessors: HashMap::new(),
            variable_types: HashMap::new(),
            resume_targets: HashSet::new(),
            outputs: Vec::new(),
            redirect: Vec::new(),
        }
    }

    /// Declare the type a variable's values carry.
    ///
    /// Every merge the builder creates for `variable` takes this type, and so
    /// does the constant a merge with no reaching definition collapses to. A
    /// variable that is never declared is a `real`, which is what the analog
    /// body's every variable is and why that half needs no declarations.
    pub fn declare_variable(&mut self, variable: CfgVariable, value_type: CfgValueType) {
        self.variable_types.insert(variable, value_type);
    }

    fn variable_type(&self, variable: CfgVariable) -> CfgValueType {
        self.variable_types
            .get(&variable)
            .copied()
            .unwrap_or(CfgValueType::Real)
    }

    /// The value a variable of this type has where nothing defined it.
    ///
    /// Zero for a `real`, because that is the Verilog-AMS initial value of an
    /// analog variable. `x` for a four-state value, because IEEE 1364-2005
    /// section 4.2.2 gives an unwritten variable exactly that — the two
    /// languages disagree about what "no value yet" means, and the type says
    /// which of them is being asked.
    fn undefined_value(&mut self, value_type: CfgValueType) -> ValueId {
        let kind = match value_type {
            CfgValueType::FourState { width } => {
                CfgValueKind::FourStateConstant(digital_value::FourStateValue::splat(
                    width,
                    crate::four_state::FourStateBit::Unknown,
                ))
            }
            CfgValueType::Integer => CfgValueKind::IntegerConstant(0),
            CfgValueType::Boolean => CfgValueKind::BooleanConstant(false),
            CfgValueType::Real | CfgValueType::Lanes(_) | CfgValueType::Effect => {
                CfgValueKind::RealConstant(0.0)
            }
        };
        let value_type = match kind {
            CfgValueKind::RealConstant(_) => CfgValueType::Real,
            _ => value_type,
        };
        self.push_leaf(value_type, kind)
    }

    /// Follow a value through any merges that collapsed since it was recorded.
    fn resolve(&self, value: ValueId) -> ValueId {
        let mut current = value;
        // Bounded rather than recursive: a cycle would be a bug, and settling is
        // a better way to report one than a stack overflow.
        for _ in 0..self.redirect.len() {
            match self.redirect.get(usize::from(current)).copied().flatten() {
                Some(next) if next != current => current = next,
                _ => break,
            }
        }
        current
    }

    pub fn create_block(&mut self) -> BlockId {
        let id = BlockId::from(self.blocks.len());
        self.blocks.push(CfgBlock {
            id,
            params: Vec::new(),
            instructions: Vec::new(),
            terminator: CfgTerminator::Unset,
        });
        id
    }

    /// Append an instruction defining a fresh value.
    pub fn push(
        &mut self,
        block: BlockId,
        value_type: CfgValueType,
        kind: CfgValueKind,
    ) -> ValueId {
        let id = self.new_value(value_type, kind);
        self.blocks[usize::from(block)]
            .instructions
            .push(CfgInstruction { result: id });
        id
    }

    /// The type of a value defined so far.
    ///
    /// A lowering that computes a result width from its operands' widths needs
    /// to read them back while the function is still under construction.
    pub fn value_type_of(&self, value: ValueId) -> Option<CfgValueType> {
        self.values
            .get(usize::from(value))
            .map(|value| value.value_type)
    }

    /// What a value defined so far is.
    ///
    /// The counterpart of [`Self::value_type_of`] for a lowering that has to
    /// read an operand's *definition* back before the function is finished —
    /// folding a filter's coefficient list, which must be constants before the
    /// node carrying them can be built at all.
    pub fn kind_of(&self, value: ValueId) -> Option<&CfgValueKind> {
        self.values.get(usize::from(value)).map(|value| &value.kind)
    }

    /// Define a value without placing it in a block.
    ///
    /// For leaves — constants, parameters, node potentials — which every block
    /// may reference and which no block owns.
    pub fn push_leaf(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        debug_assert!(is_leaf(&kind), "non-leaf value pushed outside a block");
        self.new_value(value_type, kind)
    }

    fn new_value(&mut self, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        let id = ValueId::from(self.values.len());
        self.values.push(CfgValue {
            id,
            value_type,
            kind,
        });
        self.redirect.push(None);
        id
    }

    pub fn set_terminator(&mut self, block: BlockId, terminator: CfgTerminator) {
        let successors = match &terminator {
            CfgTerminator::Jump { target, .. } => vec![*target],
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => vec![*then_target, *else_target],
            CfgTerminator::Wait { resume, .. } => vec![*resume],
            CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
        };
        if let CfgTerminator::Wait { resume, .. } = &terminator {
            self.resume_targets.insert(*resume);
        }
        for successor in successors {
            self.predecessors.entry(successor).or_default().push(block);
        }
        self.blocks[usize::from(block)].terminator = terminator;
    }

    /// Route one value across an edge that cannot carry the value table.
    ///
    /// The mechanism a [`CfgTerminator::Wait`] needs and an ordinary jump does
    /// not: a suspension ends the run, so a value the resumed half reads has to
    /// arrive as a block parameter rather than be looked up. Adds the argument
    /// and the parameter together, which is what keeps the two lists aligned,
    /// and returns the parameter the resumed block reads instead.
    ///
    /// Call it immediately after the terminator that creates the edge and
    /// before anything reads a variable in `to`; a parameter added by the
    /// ordinary merge machinery in between would take its argument at sealing
    /// time and the positions would cross.
    pub fn carry_value(&mut self, value: ValueId, from: BlockId, to: BlockId) -> ValueId {
        debug_assert_eq!(
            self.blocks[usize::from(from)]
                .arguments_to(to)
                .map(<[ValueId]>::len),
            Some(self.blocks[usize::from(to)].params.len()),
            "carry_value must run before any other parameter is added to the target"
        );
        let value_type = self.values[usize::from(value)].value_type;
        let parameter = self.new_value(value_type, CfgValueKind::BlockParameter);
        self.blocks[usize::from(to)].params.push(parameter);
        self.blocks[usize::from(from)].push_argument(to, value);
        parameter
    }

    /// Route a variable's current value across such an edge.
    ///
    /// The variable reads as the new parameter in `to`, so everything after the
    /// suspension sees the value the process had when it suspended.
    pub fn carry_variable(
        &mut self,
        variable: CfgVariable,
        from: BlockId,
        to: BlockId,
    ) -> Option<ValueId> {
        let value = self.read_variable(variable, from)?;
        let parameter = self.carry_value(value, from, to);
        self.write_variable(variable, to, parameter);
        Some(parameter)
    }

    pub fn write_variable(&mut self, variable: CfgVariable, block: BlockId, value: ValueId) {
        self.definitions.insert((variable, block), value);
    }

    /// The value of `variable` on entry to `block`, creating merges as needed.
    pub fn read_variable(&mut self, variable: CfgVariable, block: BlockId) -> Option<ValueId> {
        if let Some(value) = self.definitions.get(&(variable, block)) {
            // Resolved on the way out rather than rewritten in place: the
            // definition table is large and a collapse would otherwise have to
            // sweep it.
            return Some(self.resolve(*value));
        }
        self.read_variable_recursive(variable, block)
    }

    fn read_variable_recursive(
        &mut self,
        variable: CfgVariable,
        block: BlockId,
    ) -> Option<ValueId> {
        if !self.sealed.contains(&block) {
            // Predecessors are still arriving. Reserve a parameter now and fill
            // its arguments in on sealing; this is the only reason construction
            // needs two passes over a loop.
            let value_type = self.variable_type(variable);
            let value = self.new_value(value_type, CfgValueKind::BlockParameter);
            self.blocks[usize::from(block)].params.push(value);
            self.incomplete.push((block, variable, value));
            self.write_variable(variable, block, value);
            return Some(value);
        }

        let preds = self.predecessors.get(&block).cloned().unwrap_or_default();
        let value = match preds.as_slice() {
            // Unreachable or the entry block: the variable has no definition
            // on this path, which the caller reports as a use before assignment.
            [] => return None,
            // One predecessor needs no merge — this is what keeps the form
            // minimal without a separate cleanup pass.
            [single] => self.read_variable(variable, *single)?,
            _ => {
                let value_type = self.variable_type(variable);
                let value = self.new_value(value_type, CfgValueKind::BlockParameter);
                self.blocks[usize::from(block)].params.push(value);
                // Written before recursing so a cycle terminates here rather
                // than spinning through the loop body forever.
                self.write_variable(variable, block, value);
                self.add_parameter_arguments(variable, block, value);
                return self.try_remove_trivial_parameter(block, value);
            }
        };
        self.write_variable(variable, block, value);
        Some(value)
    }

    fn add_parameter_arguments(
        &mut self,
        variable: CfgVariable,
        block: BlockId,
        parameter: ValueId,
    ) {
        let preds = self.predecessors.get(&block).cloned().unwrap_or_default();
        for pred in preds {
            // A predecessor with no definition contributes the parameter
            // itself, which keeps the edge well-formed while the rest of the
            // arguments arrive. If every predecessor does that, the merge has
            // no reaching definition at all, which is what
            // [`Self::try_remove_trivial_parameter`] then reports.
            let argument = self.read_variable(variable, pred).unwrap_or(parameter);
            self.blocks[usize::from(pred)].push_argument(block, argument);
        }
    }

    /// Drop a merge whose arguments are all the same value.
    ///
    /// Without this the form is correct but not minimal, and every redundant
    /// merge becomes a redundant derivative array downstream.
    ///
    /// The parameter may already have been handed out — a loop body reads its
    /// carried variable long before the header can be sealed — so removing it
    /// has to rewrite every use, not just the variable table. A reader that
    /// still names a removed parameter is a value defined nowhere, which
    /// [`CfgFunction::validate`] rejects but only after the fact.
    ///
    /// Returns `None` when *every* incoming argument is the parameter itself.
    /// A merge of nothing but itself is not a definition: the variable has no
    /// reaching definition on any path, and the caller reports a read before
    /// assignment. Handing the parameter back instead — which is what this did
    /// until a real model caught it — makes a block with two predecessors
    /// answer that question differently from one with a single predecessor,
    /// and the difference survives all the way to generated Rust that reads an
    /// uninitialised binding.
    ///
    /// The parameter is redirected to zero on the way out regardless, because
    /// by the time a loop header is sealed its body already holds the value and
    /// there is no longer anywhere to report it to.
    fn try_remove_trivial_parameter(
        &mut self,
        block: BlockId,
        parameter: ValueId,
    ) -> Option<ValueId> {
        // A resume block's parameters are its entire live-in state: the value
        // table does not survive the suspension that reaches it, so there is no
        // "the argument is already available here" to collapse into.
        if self.resume_targets.contains(&block) {
            return Some(parameter);
        }
        let mut unique: Option<ValueId> = None;
        let preds = self.predecessors.get(&block).cloned().unwrap_or_default();
        let position = match self.blocks[usize::from(block)]
            .params
            .iter()
            .position(|param| *param == parameter)
        {
            Some(position) => position,
            None => return Some(parameter),
        };

        // An edge that supplies no argument at this position is either still
        // under construction or malformed. Either way this pass must not touch
        // the parameter: mid-construction the fixed point at `finish` will
        // revisit it, and a genuinely malformed edge is
        // [`CfgFunction::validate`]'s to report — collapsing it here would
        // quietly repair the graph instead.
        let mut incomplete_edge = false;
        for pred in &preds {
            let Some(args) = self.blocks[usize::from(*pred)].arguments_to(block) else {
                incomplete_edge = true;
                continue;
            };
            let Some(argument) = args.get(position).copied() else {
                incomplete_edge = true;
                continue;
            };
            // Through the redirections, because an argument recorded earlier may
            // name a merge that has since collapsed.
            let argument = self.resolve(argument);
            // Self-references are what make a loop-carried merge non-trivial
            // only when something else also flows in; ignoring them here is
            // what lets a loop variable that never changes collapse.
            if argument == parameter {
                continue;
            }
            match unique {
                Some(existing) if existing != argument => return Some(parameter),
                _ => unique = Some(argument),
            }
        }

        if incomplete_edge {
            return Some(parameter);
        }

        // Removed either way: trivial when something unique flows in, undefined
        // when nothing does. Both leave the parameter with no role.
        self.blocks[usize::from(block)].params.remove(position);
        for pred in &preds {
            let block_index = usize::from(*pred);
            match &mut self.blocks[block_index].terminator {
                CfgTerminator::Jump { target, args } if *target == block => {
                    if position < args.len() {
                        args.remove(position);
                    }
                }
                CfgTerminator::Branch {
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                    ..
                } => {
                    if *then_target == block && position < then_args.len() {
                        then_args.remove(position);
                    }
                    if *else_target == block && position < else_args.len() {
                        else_args.remove(position);
                    }
                }
                _ => {}
            }
        }
        // Zero is the Verilog-AMS initial value of an analog variable, so an
        // undefined merge resolves to it for any reader that already holds the
        // parameter. The `None` is what tells the lowering to say so. The
        // parameter's own type chooses the constant, so a four-state merge
        // resolves to `x` rather than to a real zero nothing in a process can
        // read.
        let replacement = match unique {
            Some(value) => value,
            None => {
                let value_type = self.values[usize::from(parameter)].value_type;
                self.undefined_value(value_type)
            }
        };
        self.replace_all_uses(parameter, replacement);
        unique.map(|_| replacement)
    }

    /// Record that every reader of `from` means `to`.
    ///
    /// One entry, not a sweep. [`Self::apply_redirections`] settles the graph
    /// once construction is finished.
    fn replace_all_uses(&mut self, from: ValueId, to: ValueId) {
        if from == to {
            return;
        }
        self.redirect[usize::from(from)] = Some(to);
    }

    /// Rewrite every recorded operand through the redirections.
    fn apply_redirections(&mut self) {
        if self.redirect.iter().all(Option::is_none) {
            return;
        }
        let resolved: Vec<ValueId> = (0..self.values.len())
            .map(|index| self.resolve(ValueId::from(index)))
            .collect();
        let translate = |id: ValueId| resolved[usize::from(id)];

        for value in &mut self.values {
            value.kind.map_operands(translate);
        }
        for block in &mut self.blocks {
            match &mut block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    *condition = translate(*condition);
                    for arg in then_args.iter_mut().chain(else_args.iter_mut()) {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Wait {
                    wait, resume_args, ..
                } => {
                    wait.map_operands(translate);
                    for arg in resume_args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        for output in &mut self.outputs {
            *output = translate(*output);
        }
    }

    /// Collapse every merge that became trivial as a side effect of collapsing
    /// another, until none are left.
    ///
    /// Removing one parameter can make a second one's arguments agree, and that
    /// second one is not otherwise revisited — the incremental construction only
    /// examines a parameter when it is created or sealed.
    fn remove_trivial_parameters(&mut self) {
        loop {
            let mut removed = false;
            for block in 0..self.blocks.len() {
                let block = BlockId::from(block);
                let params = self.blocks[usize::from(block)].params.clone();
                for parameter in params {
                    if self.try_remove_trivial_parameter(block, parameter) != Some(parameter) {
                        removed = true;
                    }
                }
            }
            if !removed {
                return;
            }
        }
    }

    /// Declare that `block` will gain no further predecessors.
    pub fn seal_block(&mut self, block: BlockId) {
        if !self.sealed.insert(block) {
            return;
        }
        let pending: Vec<(BlockId, CfgVariable, ValueId)> = self
            .incomplete
            .iter()
            .filter(|(pending_block, _, _)| *pending_block == block)
            .copied()
            .collect();
        self.incomplete
            .retain(|(pending_block, _, _)| *pending_block != block);
        for (_, variable, parameter) in pending {
            self.add_parameter_arguments(variable, block, parameter);
            // Nothing to report to: the body that reads this variable was built
            // long before the header could be sealed, so an undefined result
            // has already been redirected to zero for it.
            let _ = self.try_remove_trivial_parameter(block, parameter);
        }
    }

    /// Seal every block that is still open.
    ///
    /// The backstop a lowering calls once it has finished creating edges. A
    /// construct that refused partway through leaves its blocks behind, and an
    /// unsealed block reaching [`Self::finish`] holds parameters whose
    /// arguments were never supplied.
    pub fn seal_all_blocks(&mut self) {
        for index in 0..self.blocks.len() {
            self.seal_block(BlockId::from(index));
        }
    }

    pub fn is_sealed(&self, block: BlockId) -> bool {
        self.sealed.contains(&block)
    }

    /// Finish construction.
    ///
    /// Every block must be sealed: an unsealed block has parameters whose
    /// arguments were never supplied, and shipping one would produce a function
    /// that validates today and reads uninitialised state later.
    ///
    /// Values are compacted on the way out. Collapsing a trivial merge leaves
    /// its parameter defined nowhere, and a dense value table that downstream
    /// passes can index without checking for holes is worth the one renumbering
    /// pass it costs.
    pub fn finish(self, entry: BlockId) -> Result<CfgFunction, CfgValidationError> {
        self.finish_with_outputs(entry, &[])
            .map(|(function, _)| function)
    }

    /// Finish, translating `outputs` into the ids they end up with.
    ///
    /// Anything the caller intends to read out of the finished function has to
    /// go through here. Ids taken before this point do not survive it: trivial
    /// merges collapse into their arguments, and the value table is renumbered.
    pub fn finish_with_outputs(
        mut self,
        entry: BlockId,
        outputs: &[ValueId],
    ) -> Result<(CfgFunction, Vec<ValueId>), CfgValidationError> {
        debug_assert!(
            self.incomplete.is_empty(),
            "finish() with {} unsealed block parameters",
            self.incomplete.len()
        );
        self.outputs = outputs.to_vec();
        self.remove_trivial_parameters();
        self.apply_redirections();
        self.compact();
        let outputs = std::mem::take(&mut self.outputs);
        let function = CfgFunction {
            entry,
            blocks: self.blocks,
            values: self.values,
            shapes: Vec::new(),
        };
        function.validate()?;
        Ok((function, outputs))
    }

    /// Drop values nothing defines or reads, and renumber what remains.
    fn compact(&mut self) {
        let mut live = vec![false; self.values.len()];

        // Anything a block defines is live, and so is anything a live value or
        // a terminator reads. Leaves are only live if something reads them.
        let mut worklist: Vec<ValueId> = Vec::new();
        let mark = |id: ValueId, live: &mut Vec<bool>, worklist: &mut Vec<ValueId>| {
            if !live[usize::from(id)] {
                live[usize::from(id)] = true;
                worklist.push(id);
            }
        };

        // An output may be a bare leaf — a residual is the zero constant when
        // nothing contributed — so it is seeded explicitly rather than relying
        // on some block naming it.
        for output in &self.outputs {
            mark(*output, &mut live, &mut worklist);
        }
        for block in &self.blocks {
            for param in &block.params {
                mark(*param, &mut live, &mut worklist);
            }
            for instruction in &block.instructions {
                mark(instruction.result, &mut live, &mut worklist);
            }
            match &block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        mark(*arg, &mut live, &mut worklist);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    mark(*condition, &mut live, &mut worklist);
                    for arg in then_args.iter().chain(else_args) {
                        mark(*arg, &mut live, &mut worklist);
                    }
                }
                CfgTerminator::Wait {
                    wait, resume_args, ..
                } => {
                    for operand in wait.operands() {
                        mark(operand, &mut live, &mut worklist);
                    }
                    for arg in resume_args {
                        mark(*arg, &mut live, &mut worklist);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        while let Some(id) = worklist.pop() {
            for operand in self.values[usize::from(id)].kind.operands() {
                mark(operand, &mut live, &mut worklist);
            }
        }

        let mut remap = vec![None; self.values.len()];
        let mut compacted = Vec::with_capacity(self.values.len());
        for (index, value) in self.values.iter().enumerate() {
            if !live[index] {
                continue;
            }
            let id = ValueId::from(compacted.len());
            remap[index] = Some(id);
            compacted.push(CfgValue {
                id,
                value_type: value.value_type,
                kind: value.kind.clone(),
            });
        }
        let translate = |id: ValueId| -> ValueId {
            remap[usize::from(id)].expect("live value must survive compaction")
        };

        for value in &mut compacted {
            value.kind.map_operands(translate);
        }
        for output in &mut self.outputs {
            *output = translate(*output);
        }
        for block in &mut self.blocks {
            for param in &mut block.params {
                *param = translate(*param);
            }
            for instruction in &mut block.instructions {
                instruction.result = translate(instruction.result);
            }
            match &mut block.terminator {
                CfgTerminator::Jump { args, .. } => {
                    for arg in args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Branch {
                    condition,
                    then_args,
                    else_args,
                    ..
                } => {
                    *condition = translate(*condition);
                    for arg in then_args.iter_mut().chain(else_args.iter_mut()) {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Wait {
                    wait, resume_args, ..
                } => {
                    wait.map_operands(translate);
                    for arg in resume_args {
                        *arg = translate(*arg);
                    }
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        self.values = compacted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn variable(index: usize) -> CfgVariable {
        CfgVariable::Local(VariableId::from(index))
    }

    fn constant(builder: &mut SsaBuilder, value: f64) -> ValueId {
        builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(value))
    }

    /// `x = 1; x = x + 1;` in one block needs no merges at all.
    #[test]
    fn straight_line_code_creates_no_block_parameters() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);

        let one = constant(&mut builder, 1.0);
        builder.write_variable(variable(0), entry, one);
        let read = builder.read_variable(variable(0), entry).expect("defined");
        let sum = builder.push(
            entry,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: read,
                right: one,
            },
        );
        builder.write_variable(variable(0), entry, sum);
        builder.set_terminator(entry, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert!(function.blocks.iter().all(|block| block.params.is_empty()));
    }

    /// A diamond where both arms assign needs exactly one merge — which is the
    /// whole point: one parameter, not one select per downstream value.
    #[test]
    fn a_diamond_merges_with_one_block_parameter() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let then_block = builder.create_block();
        let else_block = builder.create_block();
        let join = builder.create_block();
        builder.seal_block(entry);

        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(then_block);
        builder.seal_block(else_block);

        let two = constant(&mut builder, 2.0);
        let three = constant(&mut builder, 3.0);
        builder.write_variable(variable(0), then_block, two);
        builder.write_variable(variable(0), else_block, three);
        builder.set_terminator(
            then_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.set_terminator(
            else_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);

        let merged = builder.read_variable(variable(0), join).expect("merged");
        builder.set_terminator(join, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(function.block(join).params, vec![merged]);
        assert_eq!(
            function.value(merged).kind,
            CfgValueKind::BlockParameter,
            "a merge is a block parameter, not an instruction"
        );
        // Both arms must supply exactly one argument.
        for arm in [then_block, else_block] {
            let CfgTerminator::Jump { args, .. } = &function.block(arm).terminator else {
                panic!("arm must jump to the join");
            };
            assert_eq!(args.len(), 1);
        }
    }

    /// Both arms assigning the *same* value must not create a merge; a
    /// redundant parameter becomes a redundant derivative array downstream.
    #[test]
    fn a_diamond_assigning_one_value_needs_no_parameter() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let then_block = builder.create_block();
        let else_block = builder.create_block();
        let join = builder.create_block();
        builder.seal_block(entry);

        let shared = constant(&mut builder, 7.0);
        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(false));
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(then_block);
        builder.seal_block(else_block);
        builder.write_variable(variable(0), then_block, shared);
        builder.write_variable(variable(0), else_block, shared);
        builder.set_terminator(
            then_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.set_terminator(
            else_block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);

        let merged = builder.read_variable(variable(0), join).expect("defined");
        builder.set_terminator(join, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(merged, shared, "identical arms must collapse to the value");
        assert!(function.block(join).params.is_empty());
    }

    /// A value assigned only before the branch flows through untouched.
    #[test]
    fn a_value_defined_before_a_branch_needs_no_merge() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let body = builder.create_block();
        let join = builder.create_block();
        builder.seal_block(entry);

        let five = constant(&mut builder, 5.0);
        builder.write_variable(variable(0), entry, five);
        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: join,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);

        let read = builder.read_variable(variable(0), join).expect("defined");
        builder.set_terminator(join, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(read, five);
        assert!(function.block(join).params.is_empty());
    }

    /// The case sealing exists for: a loop header is reachable from its own
    /// body, so its merge cannot be built when the header is first visited.
    #[test]
    fn a_loop_header_merges_its_back_edge() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let header = builder.create_block();
        let body = builder.create_block();
        let exit = builder.create_block();
        builder.seal_block(entry);

        let zero = constant(&mut builder, 0.0);
        let one = constant(&mut builder, 1.0);
        builder.write_variable(variable(0), entry, zero);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        // Header is deliberately left unsealed: the back edge does not exist.
        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);

        let carried = builder.read_variable(variable(0), header).expect("defined");
        let next = builder.push(
            body,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: carried,
                right: one,
            },
        );
        builder.write_variable(variable(0), body, next);
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        // The back edge now exists, so the header can be sealed and its
        // reserved parameter filled in.
        builder.seal_block(header);
        builder.seal_block(exit);
        builder.set_terminator(exit, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(
            function.block(header).params,
            vec![carried],
            "the loop-carried variable must be a header parameter"
        );

        let CfgTerminator::Jump { args, .. } = &function.block(entry).terminator else {
            panic!("entry jumps to the header");
        };
        assert_eq!(args, &vec![zero], "the initial value enters on the edge in");

        let CfgTerminator::Jump { args, .. } = &function.block(body).terminator else {
            panic!("body jumps back to the header");
        };
        assert_eq!(
            args,
            &vec![next],
            "the updated value returns on the back edge"
        );
    }

    /// A loop-carried variable the body never writes: the header parameter is
    /// handed to an instruction *before* the back edge exists, and then turns
    /// out to be trivial. Removing it has to rewrite that instruction, or the
    /// function names a value nothing defines.
    #[test]
    fn collapsing_a_header_parameter_rewrites_the_uses_that_already_read_it() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let header = builder.create_block();
        let body = builder.create_block();
        let exit = builder.create_block();
        builder.seal_block(entry);

        let seven = constant(&mut builder, 7.0);
        let one = constant(&mut builder, 1.0);
        builder.write_variable(variable(0), entry, seven);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        let condition =
            builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
        builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(body);

        // Read inside the body while the header is unsealed: this reserves a
        // parameter and immediately consumes it.
        let carried = builder.read_variable(variable(0), header).expect("defined");
        let doubled = builder.push(
            body,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Add,
                left: carried,
                right: one,
            },
        );
        // Deliberately not written back: the variable is loop-invariant, so the
        // parameter it reserved is trivial once the back edge arrives.
        let _ = doubled;
        builder.set_terminator(
            body,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        builder.seal_block(header);
        builder.seal_block(exit);
        builder.set_terminator(exit, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert!(
            function.block(header).params.is_empty(),
            "a loop-invariant variable needs no header parameter"
        );
        let addition = function
            .values
            .iter()
            .find(|value| matches!(value.kind, CfgValueKind::Binary { .. }))
            .expect("the addition survives");
        let CfgValueKind::Binary { left, .. } = addition.kind else {
            unreachable!("just matched")
        };
        assert_eq!(
            function.value(left).kind,
            CfgValueKind::RealConstant(7.0),
            "the use must have been rewritten to the value that flowed in"
        );
    }

    #[test]
    fn reading_an_unassigned_variable_reports_rather_than_inventing() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);
        builder.set_terminator(entry, CfgTerminator::Return);
        assert_eq!(builder.read_variable(variable(3), entry), None);
    }

    #[test]
    fn an_unterminated_block_fails_validation() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);
        assert_eq!(
            builder.finish(entry),
            Err(CfgValidationError::UnterminatedBlock(BlockId::from(0usize)))
        );
    }

    #[test]
    fn an_argument_count_mismatch_fails_validation() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let target = builder.create_block();
        builder.seal_block(entry);
        builder.seal_block(target);

        let stray = builder.new_value(CfgValueType::Real, CfgValueKind::BlockParameter);
        builder.blocks[usize::from(target)].params.push(stray);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target,
                args: Vec::new(),
            },
        );
        builder.set_terminator(target, CfgTerminator::Return);

        assert_eq!(
            builder.finish(entry),
            Err(CfgValidationError::ArgumentCountMismatch {
                from: BlockId::from(0usize),
                to: BlockId::from(1usize),
                expected: 1,
                found: 0,
            })
        );
    }

    #[test]
    fn predecessors_are_reported_from_terminators() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        let target = builder.create_block();
        builder.seal_block(entry);
        builder.seal_block(target);
        builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target,
                args: Vec::new(),
            },
        );
        builder.set_terminator(target, CfgTerminator::Return);

        let function = builder.finish(entry).expect("valid function");
        assert_eq!(function.predecessors(target), vec![entry]);
        assert!(function.predecessors(entry).is_empty());
    }
}
