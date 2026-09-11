//! A compact arena for Verilog-A IR expressions.
//!
//! The front end used to build a 120-byte enum whose children were `Box`es,
//! which the Windows heap rounds to 128 bytes apiece. A shadow-expanded assignment forest is
//! tens of millions of those, so representation — not the derivative's node
//! count — is what decides whether a large compact model compiles at all.
//!
//! This module holds the replacement: a 16-byte [`Node`] addressed by a
//! 4-byte [`NodeId`], stored in an [`ExprArena`] of fixed 1 Mi-node chunks so
//! a growing forest never reallocates and dropping it is a few hundred frees
//! rather than a hundred million. Names are interned, and the eighteen
//! site-bearing, event, noise and filter operators — none of which occurs in
//! any shipped assignment forest — keep today's field shapes in a side
//! [`Heavy`] table so their bulk never widens the node.
//!
//! This is now the only expression representation the front end has: the
//! converter writes nodes here, the differentiation core rewrites them, and
//! the emitter reads them. The `IrExpr`/`ExprArena::import`/`ExprArena::export`
//! bridge the port was staged across is gone with the last of its callers.
//!
//! # What a consumer of the arena owes
//!
//! Under `Box` trees a subtree has exactly one parent. In the arena it can
//! have several: the eighty-six `.clone()` sites in the shadow builder become
//! id copies, which is where the memory goes. Sharing is identity-neutral for
//! a walk that unfolds the tree (it visits a shared node once per path, as the
//! copies are visited today) and for a pure function of the subtree (the same
//! answer however often it is computed). It is *not* neutral for anything
//! else, and no digest in this crate hashes the forest, so these four rules
//! are the whole of what keeps the compiler's output byte-identical:
//!
//! 1. **The emitter reproduces `emit_expr`'s post-order.** Children in field
//!    order, then the node's own instruction — never the reverse, never a
//!    child skipped because its id was seen before. The order is not merely
//!    the instruction stream: the per-emission state slots (`limit_state_count`,
//!    `cross_detector_count`, `timer_state_count`) and a Zi site's sub-programs
//!    are allocated *at the visit*, so a visit that happens once instead of
//!    twice renumbers them.
//! 2. **The five `assign_*_site_ordinals` walks stay tree-unfolding path
//!    walks.** They carry a preorder counter, so a site reached by two paths
//!    must take two ordinals, exactly as its two copies do today. Never
//!    memoize them by [`NodeId`] — that is the one rewrite whose result is not
//!    a function of the subtree alone.
//! 3. **Pure rewrites may memoize by [`NodeId`].** `simplify`, `resolve_ddx`,
//!    `rename_variable_reads`, `simplified_constant`
//!    and the noise-axis collector all answer a question about a subtree, so
//!    computing the answer once and reusing it is the same answer. The two
//!    memos keyed by node address today become [`NodeId`]-keyed.
//! 4. **Hash-consing, if it comes, applies to pure kinds only.** Never to a
//!    site-bearing node (`AbsDelay`, `Transition`, `Slew`, the Laplace and Zi
//!    filters, the noise processes) and never to a state-allocating one
//!    (`Ddt`, `Idt`, `IdtMod`, `Limit`, `CanonicalLimit`, `Cross`, `Above`,
//!    `LastCrossing`, `Timer`): merging two of those merges two slots, two
//!    candidates or two ordinals into one. That is why [`ExprArena::push`]
//!    deduplicates nothing.

use crate::ast::{BinaryOp, UnaryOp};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::num::NonZeroU32;

use super::{
    AbsDelaySiteId, DdxAxis, IrFunction, LaplaceSiteId, NoiseSiteId, SlewSiteId, TransitionSiteId,
    ZiSiteId,
};

/// Nodes per chunk, as a shift. One chunk is 1 Mi nodes = 16 MiB.
const CHUNK_SHIFT: u32 = 20;
/// Nodes per chunk.
const CHUNK_LEN: usize = 1 << CHUNK_SHIFT;
/// Index mask within a chunk.
const CHUNK_MASK: usize = CHUNK_LEN - 1;

/// Index of a node in one [`ExprArena`].
///
/// Stored as the index plus one so `Option<NodeId>` is four bytes, which is
/// what lets a node with two optional children stay inside the 16-byte
/// budget. An id is only meaningful in the arena that issued it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct NodeId(NonZeroU32);

impl NodeId {
    /// The id for a node at `index`.
    #[inline]
    fn from_index(index: u32) -> Self {
        Self(NonZeroU32::new(index.wrapping_add(1)).expect("node index is below u32::MAX"))
    }

    /// The node's index in its arena, suitable as a memo key.
    #[inline]
    pub fn index(self) -> u32 {
        self.0.get() - 1
    }
}

/// An interned name.
///
/// The interner is a lookup and nothing else: the value of a `NameId` records
/// only the order names happened to be first seen in, which depends on the
/// order expressions were built. **Nothing may ever derive an output order —
/// a slot number, a program order, a serialized sequence — from a `NameId`.**
/// Order by the name text, or by the structure that holds it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct NameId(u32);

impl NameId {
    /// The name's index in its arena's interner. A lookup key, never an order.
    #[inline]
    pub fn index(self) -> u32 {
        self.0
    }
}

/// Index of a [`Heavy`] payload in one [`ExprArena`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct HeavyId(u32);

impl HeavyId {
    /// The payload's index in its arena.
    #[inline]
    pub fn index(self) -> u32 {
        self.0
    }
}

/// Index of a `$table_model` data pair in one [`ExprArena`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct TableId(u32);

impl TableId {
    /// The table's index in its arena.
    #[inline]
    pub fn index(self) -> u32 {
        self.0
    }
}

/// The array read a [`Node::VarIndexed`] performs, less its index expression.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexedRead {
    /// Array name, for shadow naming.
    pub array: NameId,
    /// First element's variable index.
    pub base: usize,
    /// Number of elements.
    pub len: usize,
    /// Declared lower bound.
    pub lower: i64,
}

/// One expression node.
///
/// Every payload is at most 12 bytes with alignment at most 4, or 8 bytes
/// with alignment 8 ([`Node::Const`]), so the enum is 16 bytes;
/// `node_is_sixteen_bytes` pins it. Children are [`NodeId`]s into the same
/// arena, and a node is never mutated after it is pushed — a rewrite appends.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Node {
    /// Constant value.
    Const(f64),
    /// Parameter reference.
    Param(NameId),
    /// Whether a parameter was explicitly set on the instance (`$param_given`).
    ParamGiven(NameId),
    /// Variable reference.
    Var(NameId),
    /// Runtime-indexed array element read. The array, base, length and lower
    /// bound are `ExprArena::indexed(payload)`.
    VarIndexed {
        /// Index into the arena's indexed-read table.
        payload: u32,
        /// Element index expression.
        index: NodeId,
    },
    /// Voltage at a terminal pair, ground spelled `u32::MAX`.
    Voltage(u32, u32),
    /// Current through a branch, ground spelled `u32::MAX`.
    Current(u32, u32),
    /// Branch-current unknown of a potential contribution, by ordinal.
    BranchCurrent(u32),
    /// Time variable.
    Time,
    /// Temperature (`$temperature`).
    Temperature,
    /// Thermal voltage (`$vt`).
    Vt,
    /// Instance multiplicity (`$mfactor`).
    Mfactor,
    /// Required simulator-owned value; unavailable values are errors.
    SimParamValue(rspice_veriloga_runtime::SimulationParameter),
    /// Whether the simulator currently provides a numeric query.
    SimParamPresent(rspice_veriloga_runtime::SimulationParameter),
    /// Whether an external terminal was connected on this instance.
    PortConnected(u32),
    /// Binary operation.
    Binary(BinaryOp, NodeId, NodeId),
    /// Unary operation.
    Unary(UnaryOp, NodeId),
    /// Identity value with zero derivative, used for coefficients outside ddt
    /// while constructing the reactive linearization.
    FreezeDerivative(NodeId),
    /// Function call of at most two arguments, which is every arity
    /// [`IrFunction`] has. `argc` says how many of `a` and `b` are live and is
    /// always `a.is_some() + b.is_some()`; construct through
    /// [`ExprArena::push_call`] so it cannot disagree.
    Call {
        /// The built-in being called.
        func: IrFunction,
        /// Number of live argument slots: 0, 1 or 2.
        argc: u8,
        /// First argument.
        a: Option<NodeId>,
        /// Second argument.
        b: Option<NodeId>,
    },
    /// A call with more than two arguments, which no [`IrFunction`] has and no
    /// shipped model produces. Its arguments are `ExprArena::call_args(args)`.
    /// It exists because `ExprConverter::convert_call` passes an authored
    /// argument list through without checking it against an arity, so
    /// `sqrt(a, b, c)` is constructible and has to have a representation.
    CallSpilled {
        /// The built-in being called.
        func: IrFunction,
        /// Index into the arena's spilled-argument table.
        args: u32,
    },
    /// Time derivative (`ddt`).
    Ddt(NodeId),
    /// Time integral (`idt`) and its optional initial condition.
    Idt(NodeId, Option<NodeId>),
    /// Wrapped time integral (`idtmod`). The optional initial condition and
    /// offset are `ExprArena::optional_pair(payload)`.
    IdtMod {
        /// Integrand.
        expr: NodeId,
        /// Modulus.
        modulus: NodeId,
        /// Index into the arena's optional-pair table, holding `(ic, offset)`.
        payload: u32,
    },
    /// Limited exponential.
    Limexp(NodeId),
    /// `$limit` and its optional step limit.
    Limit(NodeId, Option<NodeId>),
    /// Non-executable carrier that allocates a named limiter's state slot.
    CanonicalLimit(NodeId),
    /// `$table_model` lookup.
    TableLookup {
        /// Input expression.
        input: NodeId,
        /// Index of the `(x, y)` data in the arena's table list.
        table: TableId,
    },
    /// Slope of a lookup table at the input point.
    TableDerivative {
        /// Input expression.
        input: NodeId,
        /// Index of the `(x, y)` data in the arena's table list.
        table: TableId,
    },
    /// Symbolic partial derivative. The axis is `ExprArena::ddx_axis(axis)`.
    Ddx {
        /// Differentiated expression.
        expr: NodeId,
        /// Index into the arena's `ddx` axis table.
        axis: u32,
    },
    /// Companion-model Jacobian factor for `ddt`.
    DdtCompanion(NodeId),
    /// Tangent of a DDT candidate, retaining its accepted-history site.
    DdtDerivative {
        primal: NodeId,
        input_derivative: NodeId,
    },
    /// Companion-model Jacobian factor for `idt`.
    IdtCompanion(NodeId),
    /// Conditional: condition, then, else.
    Conditional(NodeId, NodeId, NodeId),
    /// `analysis(name)`.
    Analysis(NameId),
    /// Time of the most recent zero crossing.
    LastCrossing {
        /// Monitored expression.
        expr: NodeId,
        /// Runtime integer expression: `+1` rising, `-1` falling, `0`/`None` either.
        direction: Option<NodeId>,
    },
    /// One of the eighteen site-bearing, event, noise and filter operators,
    /// whose fields live in `ExprArena::heavy(_)`. The kind is repeated inline
    /// so a walk can classify a node without touching the side table.
    Heavy(HeavyKind, HeavyId),
}

/// Which [`Heavy`] payload a [`Node::Heavy`] carries.
///
/// One kind per site-bearing, event, noise or filter operator, repeated inline
/// in the node so a walk can classify one without touching the side table.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum HeavyKind {
    /// [`Heavy::IntegralDerivative`].
    IntegralDerivative,
    /// [`Heavy::AbsDelay`].
    AbsDelay,
    /// [`Heavy::AbsDelayDerivative`].
    AbsDelayDerivative,
    /// [`Heavy::Transition`].
    Transition,
    /// [`Heavy::TransitionDerivative`].
    TransitionDerivative,
    /// [`Heavy::Slew`].
    Slew,
    /// [`Heavy::SlewDerivative`].
    SlewDerivative,
    /// [`Heavy::Cross`].
    Cross,
    /// [`Heavy::Above`].
    Above,
    /// [`Heavy::Timer`].
    Timer,
    /// [`Heavy::WhiteNoise`].
    WhiteNoise,
    /// [`Heavy::FlickerNoise`].
    FlickerNoise,
    /// [`Heavy::NoiseTable`].
    NoiseTable,
    /// [`Heavy::LaplaceZP`].
    LaplaceZP,
    /// [`Heavy::LaplaceND`].
    LaplaceND,
    /// [`Heavy::LaplaceZPDerivative`].
    LaplaceZPDerivative,
    /// [`Heavy::LaplaceNDDerivative`].
    LaplaceNDDerivative,
    /// [`Heavy::ZiFilter`].
    ZiFilter,
    /// [`Heavy::ZiFilterDerivative`].
    ZiFilterDerivative,
}

/// A z-domain filter polynomial, as coefficients or as root pairs.
#[derive(Clone, PartialEq, Debug)]
pub enum ZiPolynomial {
    /// Coefficients ascending in `z^-1`.
    Coefficients(Vec<NodeId>),
    /// `(real, imaginary)` root pairs.
    Roots(Vec<(NodeId, NodeId)>),
}

/// The fields of one site-bearing, event, noise or filter operator.
///
/// These are the operators' authored field shapes, with every child a
/// [`NodeId`]. They live beside the nodes rather than inside them
/// because the widest of them is 113 bytes and would otherwise set the size of
/// every node in the forest, while none of them occurs in any shipped
/// assignment forest at all.
#[derive(Clone, PartialEq, Debug)]
pub enum Heavy {
    /// Local derivative action of the original integral candidate.
    IntegralDerivative {
        /// Original `Idt` or `IdtMod` node, preserving its state slot.
        primal: NodeId,
        /// Derivative of the integrand.
        input_derivative: NodeId,
        /// Derivative of the initial condition.
        ic_derivative: NodeId,
        /// Derivative of the modulus, for a circular integral.
        modulus_derivative: Option<NodeId>,
    },
    /// `absdelay` — absolute transport delay.
    AbsDelay {
        /// Site identity shared with the derivative.
        site: AbsDelaySiteId,
        /// Delayed expression.
        expr: NodeId,
        /// Delay in seconds.
        delay_time: NodeId,
        /// Optional maximum delay.
        max_delay: Option<NodeId>,
    },
    /// Local first-derivative action of one `absdelay` candidate.
    AbsDelayDerivative {
        /// Site identity shared with the primal.
        site: AbsDelaySiteId,
        /// Delayed expression.
        input: NodeId,
        /// Derivative of the delayed expression.
        input_derivative: NodeId,
        /// Delay in seconds.
        delay_time: NodeId,
        /// Derivative of the delay.
        delay_derivative: NodeId,
        /// Optional maximum delay.
        max_delay: Option<NodeId>,
        /// Derivative order, retained so lowering can refuse orders above one.
        derivative_order: u8,
    },
    /// `transition` — piecewise-linear smoothing.
    Transition {
        /// Site identity shared with the derivative.
        site: TransitionSiteId,
        /// Smoothed expression.
        expr: NodeId,
        /// Optional delay.
        delay: Option<NodeId>,
        /// Optional rise time.
        rise_time: Option<NodeId>,
        /// Optional fall time.
        fall_time: Option<NodeId>,
    },
    /// Local derivative action of one `transition` candidate.
    TransitionDerivative {
        /// Site identity shared with the primal.
        site: TransitionSiteId,
        /// Smoothed expression.
        input: NodeId,
        /// Derivative of the smoothed expression.
        input_derivative: NodeId,
        /// Optional delay.
        delay: Option<NodeId>,
        /// Optional rise time.
        rise_time: Option<NodeId>,
        /// Optional fall time.
        fall_time: Option<NodeId>,
    },
    /// `slew` — slew-rate limiting.
    Slew {
        /// Site identity shared with the derivative.
        site: SlewSiteId,
        /// Limited expression.
        expr: NodeId,
        /// Optional maximum positive slew.
        max_pos_slew: Option<NodeId>,
        /// Optional maximum negative slew.
        max_neg_slew: Option<NodeId>,
    },
    /// Local derivative action of one `slew` candidate.
    SlewDerivative {
        /// Site identity shared with the primal.
        site: SlewSiteId,
        /// Limited expression.
        input: NodeId,
        /// Derivative of the limited expression.
        input_derivative: NodeId,
        /// Optional maximum positive slew.
        max_pos_slew: Option<NodeId>,
        /// Derivative of the maximum positive slew.
        max_pos_slew_derivative: Option<NodeId>,
        /// Optional maximum negative slew.
        max_neg_slew: Option<NodeId>,
        /// Derivative of the maximum negative slew.
        max_neg_slew_derivative: Option<NodeId>,
    },
    /// `cross` — threshold-crossing event.
    Cross {
        /// Monitored expression.
        expr: NodeId,
        /// Runtime direction: `+1` rising, `-1` falling, `0` both.
        direction: Option<NodeId>,
        /// Optional time tolerance.
        time_tol: Option<NodeId>,
        /// Optional expression tolerance.
        expr_tol: Option<NodeId>,
        /// Optional enable.
        enable: Option<NodeId>,
    },
    /// `above` — rising zero-crossing event.
    Above {
        /// Monitored expression.
        expr: NodeId,
        /// Optional time tolerance.
        time_tol: Option<NodeId>,
        /// Optional expression tolerance.
        expr_tol: Option<NodeId>,
        /// Optional enable.
        enable: Option<NodeId>,
    },
    /// `timer` — time event.
    Timer {
        /// First event time.
        start_time: NodeId,
        /// Optional repeat period.
        period: Option<NodeId>,
        /// Optional time tolerance.
        time_tol: Option<NodeId>,
        /// Optional enable.
        enable: Option<NodeId>,
    },
    /// `white_noise`.
    WhiteNoise {
        /// Process identity.
        site: NoiseSiteId,
        /// Power spectral density.
        power: NodeId,
        /// Optional process name.
        name: Option<String>,
    },
    /// `flicker_noise`.
    FlickerNoise {
        /// Process identity.
        site: NoiseSiteId,
        /// Power spectral density.
        power: NodeId,
        /// Frequency exponent.
        exponent: NodeId,
        /// Optional process name.
        name: Option<String>,
    },
    /// `noise_table` / `noise_table_log`.
    NoiseTable {
        /// Process identity.
        site: NoiseSiteId,
        /// `(frequency, power)` points sorted by frequency.
        points: Vec<(f64, f64)>,
        /// Interpolate in log-log coordinates.
        log_interp: bool,
        /// Optional process name.
        name: Option<String>,
    },
    /// `laplace_zp` — pole-zero s-domain filter.
    LaplaceZP {
        /// Site identity shared with the derivative.
        site: LaplaceSiteId,
        /// Filtered expression.
        expr: NodeId,
        /// `(real, imaginary)` zeros.
        zeros: Vec<(f64, f64)>,
        /// `(real, imaginary)` poles.
        poles: Vec<(f64, f64)>,
        /// Gain.
        gain: f64,
    },
    /// `laplace_nd` — coefficient-form s-domain filter.
    LaplaceND {
        /// Site identity shared with the derivative.
        site: LaplaceSiteId,
        /// Filtered expression.
        expr: NodeId,
        /// Numerator, ascending powers of `s`.
        numerator: Vec<f64>,
        /// Denominator, ascending powers of `s`.
        denominator: Vec<f64>,
    },
    /// Jacobian action of a pole-zero Laplace filter.
    LaplaceZPDerivative {
        /// Site identity shared with the primal.
        site: LaplaceSiteId,
        /// Filtered expression.
        expr: NodeId,
        /// `(real, imaginary)` zeros.
        zeros: Vec<(f64, f64)>,
        /// `(real, imaginary)` poles.
        poles: Vec<(f64, f64)>,
        /// Gain.
        gain: f64,
    },
    /// Jacobian action of a coefficient-form Laplace filter.
    LaplaceNDDerivative {
        /// Site identity shared with the primal.
        site: LaplaceSiteId,
        /// Filtered expression.
        expr: NodeId,
        /// Numerator, ascending powers of `s`.
        numerator: Vec<f64>,
        /// Denominator, ascending powers of `s`.
        denominator: Vec<f64>,
    },
    /// `zi_*` — z-domain sampled-data filter.
    ZiFilter {
        /// Site identity shared with the derivative.
        site: ZiSiteId,
        /// Filtered expression.
        expr: NodeId,
        /// Numerator polynomial.
        numerator: ZiPolynomial,
        /// Denominator polynomial.
        denominator: ZiPolynomial,
        /// Sampling period.
        period: NodeId,
        /// Transition time.
        transition: NodeId,
        /// First transition time.
        first_transition: NodeId,
        /// Whether the output is assigned directly.
        direct_assignment: bool,
    },
    /// Jacobian action of a `zi_*` filter.
    ZiFilterDerivative {
        /// Site identity shared with the primal.
        site: ZiSiteId,
        /// Filtered expression.
        expr: NodeId,
        /// Numerator polynomial.
        numerator: ZiPolynomial,
        /// Denominator polynomial.
        denominator: ZiPolynomial,
        /// Sampling period.
        period: NodeId,
        /// Transition time.
        transition: NodeId,
        /// First transition time.
        first_transition: NodeId,
        /// Whether the output is assigned directly.
        direct_assignment: bool,
    },
}

impl Heavy {
    /// Which kind this payload is, for the inline tag on [`Node::Heavy`].
    pub fn kind(&self) -> HeavyKind {
        match self {
            Self::IntegralDerivative { .. } => HeavyKind::IntegralDerivative,
            Self::AbsDelay { .. } => HeavyKind::AbsDelay,
            Self::AbsDelayDerivative { .. } => HeavyKind::AbsDelayDerivative,
            Self::Transition { .. } => HeavyKind::Transition,
            Self::TransitionDerivative { .. } => HeavyKind::TransitionDerivative,
            Self::Slew { .. } => HeavyKind::Slew,
            Self::SlewDerivative { .. } => HeavyKind::SlewDerivative,
            Self::Cross { .. } => HeavyKind::Cross,
            Self::Above { .. } => HeavyKind::Above,
            Self::Timer { .. } => HeavyKind::Timer,
            Self::WhiteNoise { .. } => HeavyKind::WhiteNoise,
            Self::FlickerNoise { .. } => HeavyKind::FlickerNoise,
            Self::NoiseTable { .. } => HeavyKind::NoiseTable,
            Self::LaplaceZP { .. } => HeavyKind::LaplaceZP,
            Self::LaplaceND { .. } => HeavyKind::LaplaceND,
            Self::LaplaceZPDerivative { .. } => HeavyKind::LaplaceZPDerivative,
            Self::LaplaceNDDerivative { .. } => HeavyKind::LaplaceNDDerivative,
            Self::ZiFilter { .. } => HeavyKind::ZiFilter,
            Self::ZiFilterDerivative { .. } => HeavyKind::ZiFilterDerivative,
        }
    }
}

/// Name lookup for [`Node::Param`], [`Node::Var`] and their neighbours.
///
/// A lookup only. See [`NameId`] for the rule the whole crate depends on.
#[derive(Clone, Debug, Default)]
struct Interner {
    names: Vec<SmolStr>,
    index: HashMap<SmolStr, NameId>,
}

impl Interner {
    fn intern(&mut self, name: &str) -> NameId {
        if let Some(id) = self.index.get(name) {
            return *id;
        }
        let name = SmolStr::new(name);
        let id = NameId(u32::try_from(self.names.len()).expect("name count is below u32::MAX"));
        self.names.push(name.clone());
        self.index.insert(name, id);
        id
    }

    fn name(&self, id: NameId) -> &SmolStr {
        &self.names[id.0 as usize]
    }
}

/// One module's expression forest.
///
/// Nodes are appended and never moved or mutated, so a [`NodeId`] stays valid
/// for the arena's lifetime and a subtree may be reachable from more than one
/// parent. Sharing is identity-neutral for every consumer that is a
/// tree-unfolding walk or a pure function of the subtree; see the module
/// documentation for the two obligations that are not.
#[derive(Clone, Debug, Default)]
pub struct ExprArena {
    /// Fixed-capacity chunks, so growth never reallocates a live forest.
    chunks: Vec<Vec<Node>>,
    /// Number of nodes pushed.
    len: u32,
    /// Interned names.
    names: Interner,
    /// Payloads of the eighteen heavy operators.
    heavy: Vec<Heavy>,
    /// `$table_model` `(x, y)` data.
    tables: Vec<(Vec<f64>, Vec<f64>)>,
    /// Array reads of [`Node::VarIndexed`].
    indexed: Vec<IndexedRead>,
    /// `(ic, offset)` of [`Node::IdtMod`].
    optional_pairs: Vec<(Option<NodeId>, Option<NodeId>)>,
    /// Axes of [`Node::Ddx`].
    ddx_axes: Vec<DdxAxis>,
    /// Argument lists of [`Node::CallSpilled`].
    call_args: Vec<Vec<NodeId>>,
}

impl ExprArena {
    /// An empty arena.
    pub fn new() -> Self {
        Self::default()
    }

    /// The node `id` addresses.
    #[inline]
    pub fn node(&self, id: NodeId) -> &Node {
        let index = id.index() as usize;
        &self.chunks[index >> CHUNK_SHIFT][index & CHUNK_MASK]
    }

    /// Append a node and return its id.
    pub fn push(&mut self, node: Node) -> NodeId {
        let index = self.len;
        assert!(
            index < u32::MAX,
            "expression arena holds the most nodes a NodeId can address"
        );
        let chunk = (index as usize) >> CHUNK_SHIFT;
        if chunk == self.chunks.len() {
            self.chunks.push(Vec::with_capacity(CHUNK_LEN));
        }
        self.chunks[chunk].push(node);
        self.len = index + 1;
        NodeId::from_index(index)
    }

    /// Append a call, choosing the inline or the spilled encoding by arity.
    pub fn push_call(&mut self, func: IrFunction, args: &[NodeId]) -> NodeId {
        let node = match *args {
            [] => Node::Call {
                func,
                argc: 0,
                a: None,
                b: None,
            },
            [a] => Node::Call {
                func,
                argc: 1,
                a: Some(a),
                b: None,
            },
            [a, b] => Node::Call {
                func,
                argc: 2,
                a: Some(a),
                b: Some(b),
            },
            _ => {
                let index = u32::try_from(self.call_args.len())
                    .expect("spilled call count is below u32::MAX");
                self.call_args.push(args.to_vec());
                Node::CallSpilled { func, args: index }
            }
        };
        self.push(node)
    }

    /// Append a heavy payload and the node that addresses it.
    pub fn push_heavy(&mut self, heavy: Heavy) -> NodeId {
        let kind = heavy.kind();
        let id = HeavyId(u32::try_from(self.heavy.len()).expect("heavy count is below u32::MAX"));
        self.heavy.push(heavy);
        self.push(Node::Heavy(kind, id))
    }

    /// Intern a name.
    ///
    /// The returned id is a lookup key. Deriving any output order from its
    /// value would make the compiler's output depend on construction order.
    pub fn intern(&mut self, name: &str) -> NameId {
        self.names.intern(name)
    }

    /// The name `id` addresses.
    #[inline]
    pub fn name(&self, id: NameId) -> &SmolStr {
        self.names.name(id)
    }

    /// The heavy payload `id` addresses.
    #[inline]
    pub fn heavy(&self, id: HeavyId) -> &Heavy {
        &self.heavy[id.0 as usize]
    }

    /// Append `$table_model` data.
    pub fn push_table(&mut self, x_data: Vec<f64>, y_data: Vec<f64>) -> TableId {
        let id = TableId(u32::try_from(self.tables.len()).expect("table count is below u32::MAX"));
        self.tables.push((x_data, y_data));
        id
    }

    /// The `(x, y)` data `id` addresses.
    #[inline]
    pub fn table(&self, id: TableId) -> &(Vec<f64>, Vec<f64>) {
        &self.tables[id.0 as usize]
    }

    /// Append an array read, returning its [`Node::VarIndexed`] payload index.
    pub fn push_indexed(&mut self, read: IndexedRead) -> u32 {
        let index =
            u32::try_from(self.indexed.len()).expect("indexed-read count is below u32::MAX");
        self.indexed.push(read);
        index
    }

    /// The array read `payload` addresses.
    #[inline]
    pub fn indexed(&self, payload: u32) -> &IndexedRead {
        &self.indexed[payload as usize]
    }

    /// Append an optional child pair, returning its payload index.
    pub fn push_optional_pair(&mut self, pair: (Option<NodeId>, Option<NodeId>)) -> u32 {
        let index = u32::try_from(self.optional_pairs.len())
            .expect("optional-pair count is below u32::MAX");
        self.optional_pairs.push(pair);
        index
    }

    /// The optional child pair `payload` addresses.
    #[inline]
    pub fn optional_pair(&self, payload: u32) -> (Option<NodeId>, Option<NodeId>) {
        self.optional_pairs[payload as usize]
    }

    /// Append a `ddx` axis, returning its payload index.
    pub fn push_ddx_axis(&mut self, axis: DdxAxis) -> u32 {
        let index = u32::try_from(self.ddx_axes.len()).expect("ddx axis count is below u32::MAX");
        self.ddx_axes.push(axis);
        index
    }

    /// The `ddx` axis `payload` addresses.
    #[inline]
    pub fn ddx_axis(&self, payload: u32) -> DdxAxis {
        self.ddx_axes[payload as usize]
    }

    /// The spilled argument list `args` addresses.
    #[inline]
    pub fn call_args(&self, args: u32) -> &[NodeId] {
        &self.call_args[args as usize]
    }

    /// The arguments of a call node, whichever encoding it uses.
    pub fn arguments(&self, node: &Node) -> Vec<NodeId> {
        match node {
            Node::Call { a, b, .. } => a.iter().chain(b.iter()).copied().collect(),
            Node::CallSpilled { args, .. } => self.call_args(*args).to_vec(),
            _ => Vec::new(),
        }
    }

    /// How many nodes the arena holds.
    #[inline]
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Whether the arena holds no nodes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Whether the primal forest needs a limiter displacement direction.
    pub(crate) fn has_limiters(&self) -> bool {
        self.chunks
            .iter()
            .flatten()
            .any(|node| matches!(node, Node::Limit(..) | Node::CanonicalLimit(..)))
    }
}

/// The `usize` a terminal pair uses for ground (`expr_converter::GROUND_NODE`),
/// packed into a node's `u32` slots as `PACKED_GROUND`.
const GROUND_INDEX: usize = usize::MAX;
/// Ground's packed spelling. No real index may take this value.
const PACKED_GROUND: u32 = u32::MAX;

/// Pack a terminal or ordinal index into a node's `u32` slot.
///
/// The converter speaks `usize` because that is what
/// `expr_converter::ConversionContext` resolves a node name to, and
/// `GROUND_NODE` is `usize::MAX`; the four packed payloads hold a `u32`. This
/// is the only way to write one, so a producer cannot cast a ground sentinel
/// into a real terminal index by accident.
pub fn pack_index(value: usize) -> u32 {
    if value == GROUND_INDEX {
        return PACKED_GROUND;
    }
    let packed = u32::try_from(value).expect("terminal index is below u32::MAX");
    assert_ne!(
        packed, PACKED_GROUND,
        "terminal index collides with the packed ground sentinel"
    );
    packed
}

/// Unpack a terminal or ordinal index from a node's `u32` slot.
///
/// The four payloads [`pack_index`] writes — [`Node::Voltage`],
/// [`Node::Current`], [`Node::BranchCurrent`] and [`Node::PortConnected`] —
/// hold a `u32` where the source holds a `usize`, with `u32::MAX` standing for
/// `expr_converter::GROUND_NODE`. Every consumer that reads one of them owes
/// the bytecode and the canonical route the `usize` spelling, so it reads them
/// through here rather than casting.
pub fn unpack_index(value: u32) -> usize {
    if value == PACKED_GROUND {
        GROUND_INDEX
    } else {
        value as usize
    }
}

/// Hand every child slot of `node` that the generic walks descend into to `f`,
/// in field order.
///
/// The child set every generic walk shares, including the slots they
/// deliberately stop at: an event, noise or companion operand is compiled into
/// a program of its own, so no generic walk enters it. [`operator_operands`] is where those operands are reached.
pub fn for_each_child<F: FnMut(NodeId)>(arena: &ExprArena, node: &Node, f: &mut F) {
    let optional = |slot: Option<NodeId>, f: &mut F| {
        if let Some(child) = slot {
            f(child);
        }
    };
    match node {
        Node::Binary(_, left, right)
        | Node::DdtDerivative {
            primal: left,
            input_derivative: right,
        } => {
            f(*left);
            f(*right);
        }
        Node::Unary(_, inner) => f(*inner),
        Node::Call { a, b, .. } => {
            optional(*a, f);
            optional(*b, f);
        }
        Node::CallSpilled { args, .. } => {
            for arg in arena.call_args(*args) {
                f(*arg);
            }
        }
        Node::Conditional(condition, then_expr, else_expr) => {
            f(*condition);
            f(*then_expr);
            f(*else_expr);
        }
        Node::Ddt(inner)
        | Node::FreezeDerivative(inner)
        | Node::Limexp(inner)
        | Node::CanonicalLimit(inner)
        | Node::Ddx { expr: inner, .. }
        | Node::TableLookup { input: inner, .. }
        | Node::VarIndexed { index: inner, .. } => f(*inner),
        Node::Idt(inner, second) | Node::Limit(inner, second) => {
            f(*inner);
            optional(*second, f);
        }
        Node::IdtMod {
            expr,
            modulus,
            payload,
        } => {
            let (ic, offset) = arena.optional_pair(*payload);
            f(*expr);
            optional(ic, f);
            f(*modulus);
            optional(offset, f);
        }
        Node::Heavy(_, id) => for_each_heavy_child(arena.heavy(*id), f),
        // Leaves for the generic walks. The event, noise and companion nodes
        // carry operands they never descend into; see `operator_operands`.
        Node::Const(_)
        | Node::Param(_)
        | Node::ParamGiven(_)
        | Node::Var(_)
        | Node::Voltage(..)
        | Node::Current(..)
        | Node::BranchCurrent(_)
        | Node::Time
        | Node::Temperature
        | Node::Vt
        | Node::SimParamValue(_)
        | Node::SimParamPresent(_)
        | Node::Mfactor
        | Node::PortConnected(_)
        | Node::Analysis(_)
        | Node::LastCrossing { .. }
        | Node::DdtCompanion(_)
        | Node::IdtCompanion(_)
        | Node::TableDerivative { .. } => {}
    }
}

fn for_each_heavy_child<F: FnMut(NodeId)>(heavy: &Heavy, f: &mut F) {
    let optional = |slot: Option<NodeId>, f: &mut F| {
        if let Some(child) = slot {
            f(child);
        }
    };
    match heavy {
        Heavy::IntegralDerivative {
            primal,
            input_derivative,
            ic_derivative,
            modulus_derivative,
        } => {
            f(*primal);
            f(*input_derivative);
            f(*ic_derivative);
            optional(*modulus_derivative, f);
        }
        Heavy::AbsDelay {
            expr,
            delay_time,
            max_delay,
            ..
        } => {
            f(*expr);
            f(*delay_time);
            optional(*max_delay, f);
        }
        Heavy::AbsDelayDerivative {
            input,
            input_derivative,
            delay_time,
            delay_derivative,
            max_delay,
            ..
        } => {
            f(*input);
            f(*input_derivative);
            f(*delay_time);
            f(*delay_derivative);
            optional(*max_delay, f);
        }
        Heavy::Transition {
            expr,
            delay,
            rise_time,
            fall_time,
            ..
        } => {
            f(*expr);
            optional(*delay, f);
            optional(*rise_time, f);
            optional(*fall_time, f);
        }
        Heavy::TransitionDerivative {
            input,
            input_derivative,
            delay,
            rise_time,
            fall_time,
            ..
        } => {
            f(*input);
            f(*input_derivative);
            optional(*delay, f);
            optional(*rise_time, f);
            optional(*fall_time, f);
        }
        Heavy::Slew {
            expr,
            max_pos_slew,
            max_neg_slew,
            ..
        } => {
            f(*expr);
            optional(*max_pos_slew, f);
            optional(*max_neg_slew, f);
        }
        Heavy::SlewDerivative {
            input,
            input_derivative,
            max_pos_slew,
            max_pos_slew_derivative,
            max_neg_slew,
            max_neg_slew_derivative,
            ..
        } => {
            f(*input);
            f(*input_derivative);
            optional(*max_pos_slew, f);
            optional(*max_pos_slew_derivative, f);
            optional(*max_neg_slew, f);
            optional(*max_neg_slew_derivative, f);
        }
        Heavy::LaplaceZP { expr, .. }
        | Heavy::LaplaceND { expr, .. }
        | Heavy::LaplaceZPDerivative { expr, .. }
        | Heavy::LaplaceNDDerivative { expr, .. } => f(*expr),
        Heavy::ZiFilter {
            expr,
            period,
            transition,
            first_transition,
            ..
        }
        | Heavy::ZiFilterDerivative {
            expr,
            period,
            transition,
            first_transition,
            ..
        } => {
            f(*expr);
            f(*period);
            f(*transition);
            f(*first_transition);
        }
        // The event and noise operators are leaves for the generic walks; the
        // Zi polynomials are a slot the walks do not enter either.
        Heavy::Cross { .. }
        | Heavy::Above { .. }
        | Heavy::Timer { .. }
        | Heavy::WhiteNoise { .. }
        | Heavy::FlickerNoise { .. }
        | Heavy::NoiseTable { .. } => {}
    }
}

/// The operand programs an operator owns, which [`for_each_child`] stops at.
///
/// The arena's spelling of `reaching_definition`'s walk of the same
/// name: an operator's operands are compiled into programs of their own rather
/// than into the expression holding it, but they are evaluated with that
/// expression and read the same definitions, so a pass that resolves reads has
/// to reach them. Nothing else the generic walks stop at owns a
/// sub-expression — a Laplace or Zi coefficient list is numbers, a companion
/// is a slot ordinal.
pub fn operator_operands(arena: &ExprArena, node: &Node) -> Vec<NodeId> {
    let mut operands = Vec::new();
    let optional = |slot: Option<NodeId>, operands: &mut Vec<NodeId>| {
        if let Some(operand) = slot {
            operands.push(operand);
        }
    };
    match node {
        Node::LastCrossing { expr, direction } => {
            operands.push(*expr);
            optional(*direction, &mut operands);
        }
        Node::Heavy(_, id) => match arena.heavy(*id) {
            Heavy::WhiteNoise { power, .. } => operands.push(*power),
            Heavy::FlickerNoise {
                power, exponent, ..
            } => {
                operands.push(*power);
                operands.push(*exponent);
            }
            Heavy::Cross {
                expr,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => {
                operands.push(*expr);
                optional(*direction, &mut operands);
                optional(*time_tol, &mut operands);
                optional(*expr_tol, &mut operands);
                optional(*enable, &mut operands);
            }
            Heavy::Above {
                expr,
                time_tol,
                expr_tol,
                enable,
            } => {
                operands.push(*expr);
                optional(*time_tol, &mut operands);
                optional(*expr_tol, &mut operands);
                optional(*enable, &mut operands);
            }
            Heavy::Timer {
                start_time,
                period,
                time_tol,
                enable,
            } => {
                operands.push(*start_time);
                optional(*period, &mut operands);
                optional(*time_tol, &mut operands);
                optional(*enable, &mut operands);
            }
            _ => {}
        },
        _ => {}
    }
    operands
}

/// Walk the tree rooted at `id` in preorder, node before children.
///
/// A shared subtree is visited once per path, which is what makes the
/// site-ordinal walks correct over an arena and is the whole of why nothing
/// here memoizes by [`NodeId`].
pub fn visit(arena: &ExprArena, id: NodeId, f: &mut impl FnMut(&Node)) {
    let node = *arena.node(id);
    f(&node);
    for_each_child(arena, &node, &mut |child| visit(arena, child, f));
}

/// Rewrite the tree rooted at `id`, appending only what changed.
///
/// `f` sees each node before its children and may replace it outright, in which case the replacement is
/// pushed as written and its children are not walked. Otherwise the children
/// are rewritten in the same slots the boxed walk rebuilt, and the node is pushed
/// again only if one of them moved — an unchanged subtree keeps its id, so a
/// rewrite that changes nothing allocates nothing.
///
/// Nothing is mutated in place. Every id handed out before the call still
/// addresses the node it did.
pub fn rewrite(
    arena: &mut ExprArena,
    id: NodeId,
    f: &mut impl FnMut(&mut ExprArena, Node) -> Option<Node>,
) -> NodeId {
    let node = *arena.node(id);
    if let Some(replacement) = f(arena, node) {
        return arena.push(replacement);
    }
    rebuild_children(arena, id, node, &mut |arena, child| {
        rewrite(arena, child, f)
    })
}

/// Rebuild one node's children through `descend`, keeping its id if none moved.
///
/// This is [`rewrite`] without the replacement test: the child slots
/// the boxed walk rebuilt, in the order it rebuilt them, with the recursion left
/// to the caller. [`rewrite`] is this plus a closure that calls itself, and a
/// walk whose *matched* node needs to recurse — the Laplace site-ordinal walk,
/// which numbers an outer transfer function and then descends into its operand
/// — needs this directly, because a closure handed to [`rewrite`] cannot call
/// [`rewrite`] again while it is borrowed.
///
/// The descent rules are the ones the boxed walks had, and they are not
/// uniform: a companion factor, a table derivative, `last_crossing` and the
/// event and noise operators are leaves here, and a Laplace or Zi coefficient
/// list is a slot this never enters. Anything that needs those must reach them
/// itself, as the emitter does.
pub fn rebuild_children(
    arena: &mut ExprArena,
    id: NodeId,
    node: Node,
    descend: &mut impl FnMut(&mut ExprArena, NodeId) -> NodeId,
) -> NodeId {
    match node {
        Node::Binary(op, left, right) => {
            let new_left = descend(arena, left);
            let new_right = descend(arena, right);
            if new_left == left && new_right == right {
                return id;
            }
            arena.push(Node::Binary(op, new_left, new_right))
        }
        Node::DdtDerivative {
            primal,
            input_derivative,
        } => {
            let new_primal = descend(arena, primal);
            let new_input = descend(arena, input_derivative);
            if new_primal == primal && new_input == input_derivative {
                return id;
            }
            arena.push(Node::DdtDerivative {
                primal: new_primal,
                input_derivative: new_input,
            })
        }
        Node::Unary(op, inner) => {
            let new_inner = descend(arena, inner);
            if new_inner == inner {
                return id;
            }
            arena.push(Node::Unary(op, new_inner))
        }
        Node::Call { func, argc, a, b } => {
            let new_a = rebuild_optional(arena, a, descend);
            let new_b = rebuild_optional(arena, b, descend);
            if new_a == a && new_b == b {
                return id;
            }
            arena.push(Node::Call {
                func,
                argc,
                a: new_a,
                b: new_b,
            })
        }
        Node::CallSpilled { func, args } => {
            let old = arena.call_args(args).to_vec();
            let new = old
                .iter()
                .map(|arg| descend(arena, *arg))
                .collect::<Vec<_>>();
            if new == old {
                return id;
            }
            arena.push_call(func, &new)
        }
        Node::Conditional(condition, then_expr, else_expr) => {
            let new_condition = descend(arena, condition);
            let new_then = descend(arena, then_expr);
            let new_else = descend(arena, else_expr);
            if new_condition == condition && new_then == then_expr && new_else == else_expr {
                return id;
            }
            arena.push(Node::Conditional(new_condition, new_then, new_else))
        }
        Node::Ddt(inner) => rebuild_unary(arena, id, inner, Node::Ddt, descend),
        Node::FreezeDerivative(inner) => {
            rebuild_unary(arena, id, inner, Node::FreezeDerivative, descend)
        }
        Node::Limexp(inner) => rebuild_unary(arena, id, inner, Node::Limexp, descend),
        Node::CanonicalLimit(inner) => {
            rebuild_unary(arena, id, inner, Node::CanonicalLimit, descend)
        }
        Node::Ddx { expr, axis } => {
            rebuild_unary(arena, id, expr, |expr| Node::Ddx { expr, axis }, descend)
        }
        Node::TableLookup { input, table } => rebuild_unary(
            arena,
            id,
            input,
            |input| Node::TableLookup { input, table },
            descend,
        ),
        Node::VarIndexed { payload, index } => rebuild_unary(
            arena,
            id,
            index,
            |index| Node::VarIndexed { payload, index },
            descend,
        ),
        Node::Idt(inner, second) => {
            let new_inner = descend(arena, inner);
            let new_second = rebuild_optional(arena, second, descend);
            if new_inner == inner && new_second == second {
                return id;
            }
            arena.push(Node::Idt(new_inner, new_second))
        }
        Node::Limit(inner, second) => {
            let new_inner = descend(arena, inner);
            let new_second = rebuild_optional(arena, second, descend);
            if new_inner == inner && new_second == second {
                return id;
            }
            arena.push(Node::Limit(new_inner, new_second))
        }
        Node::IdtMod {
            expr,
            modulus,
            payload,
        } => {
            let (ic, offset) = arena.optional_pair(payload);
            let new_expr = descend(arena, expr);
            let new_ic = rebuild_optional(arena, ic, descend);
            let new_modulus = descend(arena, modulus);
            let new_offset = rebuild_optional(arena, offset, descend);
            if new_expr == expr && new_ic == ic && new_modulus == modulus && new_offset == offset {
                return id;
            }
            let payload = arena.push_optional_pair((new_ic, new_offset));
            arena.push(Node::IdtMod {
                expr: new_expr,
                modulus: new_modulus,
                payload,
            })
        }
        Node::Heavy(_, heavy) => {
            let old = arena.heavy(heavy).clone();
            let new = rebuild_heavy(arena, &old, descend);
            if new == old {
                return id;
            }
            arena.push_heavy(new)
        }
        // Leaves for the generic walks: an unchanged node keeps its id, which
        // is the arena's spelling of the boxed walk's `other => other.clone()`.
        Node::Const(_)
        | Node::Param(_)
        | Node::ParamGiven(_)
        | Node::Var(_)
        | Node::Voltage(..)
        | Node::Current(..)
        | Node::BranchCurrent(_)
        | Node::Time
        | Node::Temperature
        | Node::Vt
        | Node::SimParamValue(_)
        | Node::SimParamPresent(_)
        | Node::Mfactor
        | Node::PortConnected(_)
        | Node::Analysis(_)
        | Node::LastCrossing { .. }
        | Node::DdtCompanion(_)
        | Node::IdtCompanion(_)
        | Node::TableDerivative { .. } => id,
    }
}

fn rebuild_unary(
    arena: &mut ExprArena,
    id: NodeId,
    child: NodeId,
    build: impl Fn(NodeId) -> Node,
    descend: &mut impl FnMut(&mut ExprArena, NodeId) -> NodeId,
) -> NodeId {
    let new_child = descend(arena, child);
    if new_child == child {
        return id;
    }
    arena.push(build(new_child))
}

fn rebuild_optional(
    arena: &mut ExprArena,
    child: Option<NodeId>,
    descend: &mut impl FnMut(&mut ExprArena, NodeId) -> NodeId,
) -> Option<NodeId> {
    child.map(|child| descend(arena, child))
}

fn rebuild_heavy(
    arena: &mut ExprArena,
    heavy: &Heavy,
    descend: &mut impl FnMut(&mut ExprArena, NodeId) -> NodeId,
) -> Heavy {
    match heavy {
        Heavy::IntegralDerivative {
            primal,
            input_derivative,
            ic_derivative,
            modulus_derivative,
        } => Heavy::IntegralDerivative {
            primal: descend(arena, *primal),
            input_derivative: descend(arena, *input_derivative),
            ic_derivative: descend(arena, *ic_derivative),
            modulus_derivative: rebuild_optional(arena, *modulus_derivative, descend),
        },
        Heavy::AbsDelay {
            site,
            expr,
            delay_time,
            max_delay,
        } => Heavy::AbsDelay {
            site: *site,
            expr: descend(arena, *expr),
            delay_time: descend(arena, *delay_time),
            max_delay: rebuild_optional(arena, *max_delay, descend),
        },
        Heavy::AbsDelayDerivative {
            site,
            input,
            input_derivative,
            delay_time,
            delay_derivative,
            max_delay,
            derivative_order,
        } => Heavy::AbsDelayDerivative {
            site: *site,
            input: descend(arena, *input),
            input_derivative: descend(arena, *input_derivative),
            delay_time: descend(arena, *delay_time),
            delay_derivative: descend(arena, *delay_derivative),
            max_delay: rebuild_optional(arena, *max_delay, descend),
            derivative_order: *derivative_order,
        },
        Heavy::Transition {
            site,
            expr,
            delay,
            rise_time,
            fall_time,
        } => Heavy::Transition {
            site: *site,
            expr: descend(arena, *expr),
            delay: rebuild_optional(arena, *delay, descend),
            rise_time: rebuild_optional(arena, *rise_time, descend),
            fall_time: rebuild_optional(arena, *fall_time, descend),
        },
        Heavy::TransitionDerivative {
            site,
            input,
            input_derivative,
            delay,
            rise_time,
            fall_time,
        } => Heavy::TransitionDerivative {
            site: *site,
            input: descend(arena, *input),
            input_derivative: descend(arena, *input_derivative),
            delay: rebuild_optional(arena, *delay, descend),
            rise_time: rebuild_optional(arena, *rise_time, descend),
            fall_time: rebuild_optional(arena, *fall_time, descend),
        },
        Heavy::Slew {
            site,
            expr,
            max_pos_slew,
            max_neg_slew,
        } => Heavy::Slew {
            site: *site,
            expr: descend(arena, *expr),
            max_pos_slew: rebuild_optional(arena, *max_pos_slew, descend),
            max_neg_slew: rebuild_optional(arena, *max_neg_slew, descend),
        },
        Heavy::SlewDerivative {
            site,
            input,
            input_derivative,
            max_pos_slew,
            max_pos_slew_derivative,
            max_neg_slew,
            max_neg_slew_derivative,
        } => Heavy::SlewDerivative {
            site: *site,
            input: descend(arena, *input),
            input_derivative: descend(arena, *input_derivative),
            max_pos_slew: rebuild_optional(arena, *max_pos_slew, descend),
            max_pos_slew_derivative: rebuild_optional(arena, *max_pos_slew_derivative, descend),
            max_neg_slew: rebuild_optional(arena, *max_neg_slew, descend),
            max_neg_slew_derivative: rebuild_optional(arena, *max_neg_slew_derivative, descend),
        },
        Heavy::LaplaceZP {
            site,
            expr,
            zeros,
            poles,
            gain,
        } => Heavy::LaplaceZP {
            site: *site,
            expr: descend(arena, *expr),
            zeros: zeros.clone(),
            poles: poles.clone(),
            gain: *gain,
        },
        Heavy::LaplaceND {
            site,
            expr,
            numerator,
            denominator,
        } => Heavy::LaplaceND {
            site: *site,
            expr: descend(arena, *expr),
            numerator: numerator.clone(),
            denominator: denominator.clone(),
        },
        Heavy::LaplaceZPDerivative {
            site,
            expr,
            zeros,
            poles,
            gain,
        } => Heavy::LaplaceZPDerivative {
            site: *site,
            expr: descend(arena, *expr),
            zeros: zeros.clone(),
            poles: poles.clone(),
            gain: *gain,
        },
        Heavy::LaplaceNDDerivative {
            site,
            expr,
            numerator,
            denominator,
        } => Heavy::LaplaceNDDerivative {
            site: *site,
            expr: descend(arena, *expr),
            numerator: numerator.clone(),
            denominator: denominator.clone(),
        },
        Heavy::ZiFilter {
            site,
            expr,
            numerator,
            denominator,
            period,
            transition,
            first_transition,
            direct_assignment,
        } => Heavy::ZiFilter {
            site: *site,
            expr: descend(arena, *expr),
            numerator: numerator.clone(),
            denominator: denominator.clone(),
            period: descend(arena, *period),
            transition: descend(arena, *transition),
            first_transition: descend(arena, *first_transition),
            direct_assignment: *direct_assignment,
        },
        Heavy::ZiFilterDerivative {
            site,
            expr,
            numerator,
            denominator,
            period,
            transition,
            first_transition,
            direct_assignment,
        } => Heavy::ZiFilterDerivative {
            site: *site,
            expr: descend(arena, *expr),
            numerator: numerator.clone(),
            denominator: denominator.clone(),
            period: descend(arena, *period),
            transition: descend(arena, *transition),
            first_transition: descend(arena, *first_transition),
            direct_assignment: *direct_assignment,
        },
        // Leaves for the generic walks, cloned unchanged the way the boxed walk
        // cloned them.
        Heavy::Cross { .. }
        | Heavy::Above { .. }
        | Heavy::Timer { .. }
        | Heavy::WhiteNoise { .. }
        | Heavy::FlickerNoise { .. }
        | Heavy::NoiseTable { .. } => heavy.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The leading identifier of a `Debug` rendering, which for a derived
    /// `Debug` is the variant's name.
    fn variant_name(rendered: &str) -> String {
        rendered
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }

    /// The operator name a [`Node`] stands for.
    fn node_variant_name(node: &Node) -> String {
        match node {
            Node::Heavy(kind, _) => format!("{kind:?}"),
            Node::CallSpilled { .. } => "Call".to_string(),
            other => variant_name(&format!("{other:?}")),
        }
    }

    fn marker(arena: &mut ExprArena, name: &str) -> NodeId {
        let name = arena.intern(name);
        arena.push(Node::Var(name))
    }

    #[test]
    fn node_is_sixteen_bytes() {
        assert_eq!(std::mem::size_of::<Node>(), 16);
        assert_eq!(std::mem::size_of::<NodeId>(), 4);
        assert_eq!(std::mem::size_of::<Option<NodeId>>(), 4);
    }

    #[test]
    fn a_call_of_more_than_two_arguments_spills() {
        let mut arena = ExprArena::new();
        let args = [1.0, 2.0, 3.0].map(|value| arena.push(Node::Const(value)));
        let id = arena.push_call(IrFunction::Max, &args);
        assert!(matches!(arena.node(id), Node::CallSpilled { .. }));
        assert_eq!(arena.arguments(arena.node(id)).len(), 3);
    }

    /// The slots the generic walks stop at, and the walk that reaches them.
    #[test]
    fn operator_operands_reaches_what_visit_does_not() {
        let mut arena = ExprArena::new();
        let monitored = marker(&mut arena, "monitored");
        let direction = arena.push(Node::Const(1.0));
        let enable = marker(&mut arena, "enabled");
        let id = arena.push_heavy(Heavy::Cross {
            expr: monitored,
            direction: Some(direction),
            time_tol: None,
            expr_tol: None,
            enable: Some(enable),
        });

        let mut visited = 0;
        visit(&arena, id, &mut |_| visited += 1);
        assert_eq!(visited, 1, "the generic walk must stop at a cross");

        let operands = operator_operands(&arena, arena.node(id));
        assert_eq!(operands.len(), 3);
        let names = operands
            .iter()
            .map(|operand| node_variant_name(arena.node(*operand)))
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["Var", "Const", "Var"]);
    }

    /// One node of every kind, each with markers in the slots the walks enter.
    ///
    /// The bridge fixtures this replaces built one boxed tree of every variant
    /// and compared a round trip; with the boxed tree gone the property worth
    /// keeping is that [`rewrite`] is an identity on a node it does not change,
    /// for every shape it can meet.
    fn one_of_every_node(arena: &mut ExprArena) -> Vec<NodeId> {
        let m = |arena: &mut ExprArena| marker(arena, "m");
        let table = arena.push_table(vec![0.0, 1.0], vec![0.0, 1.0]);
        let axis = arena.push_ddx_axis(DdxAxis::Potential {
            pos: Some(0),
            neg: None,
        });
        let indexed = {
            let array = arena.intern("a");
            arena.push_indexed(IndexedRead {
                array,
                base: 0,
                len: 2,
                lower: 0,
            })
        };
        let site = TransitionSiteId {
            source: 0,
            start: 0,
            end: 1,
            ordinal: 0,
        };
        let mut out = Vec::new();
        let plain = [
            Node::Const(1.0),
            Node::Time,
            Node::Temperature,
            Node::Vt,
            Node::Mfactor,
            Node::Voltage(0, u32::MAX),
            Node::Current(u32::MAX, 1),
            Node::BranchCurrent(0),
            Node::PortConnected(0),
        ];
        for node in plain {
            out.push(arena.push(node));
        }
        for name in ["p", "g", "v", "an"] {
            let interned = arena.intern(name);
            out.push(arena.push(Node::Param(interned)));
            out.push(arena.push(Node::ParamGiven(interned)));
            out.push(arena.push(Node::Var(interned)));
            out.push(arena.push(Node::Analysis(interned)));
        }
        let one = m(arena);
        let two = m(arena);
        out.push(arena.push(Node::Binary(BinaryOp::Add, one, two)));
        out.push(arena.push(Node::Unary(UnaryOp::Neg, one)));
        out.push(arena.push(Node::Conditional(one, two, one)));
        out.push(arena.push(Node::Ddt(one)));
        out.push(arena.push(Node::FreezeDerivative(one)));
        out.push(arena.push(Node::Idt(one, Some(two))));
        out.push(arena.push(Node::Idt(one, None)));
        out.push(arena.push(Node::Limexp(one)));
        out.push(arena.push(Node::Limit(one, Some(two))));
        out.push(arena.push(Node::CanonicalLimit(one)));
        out.push(arena.push(Node::TableLookup { input: one, table }));
        out.push(arena.push(Node::TableDerivative { input: one, table }));
        out.push(arena.push(Node::Ddx { expr: one, axis }));
        out.push(arena.push(Node::DdtCompanion(one)));
        out.push(arena.push(Node::DdtDerivative {
            primal: one,
            input_derivative: two,
        }));
        out.push(arena.push(Node::IdtCompanion(one)));
        out.push(arena.push(Node::LastCrossing {
            expr: one,
            direction: Some(one),
        }));
        out.push(arena.push(Node::VarIndexed {
            payload: indexed,
            index: one,
        }));
        let payload = arena.push_optional_pair((Some(one), Some(two)));
        out.push(arena.push(Node::IdtMod {
            expr: one,
            modulus: two,
            payload,
        }));
        out.push(arena.push_call(IrFunction::Min, &[]));
        out.push(arena.push_call(IrFunction::Min, &[one]));
        out.push(arena.push_call(IrFunction::Min, &[one, two]));
        out.push(arena.push_call(IrFunction::Min, &[one, two, one]));
        out.push(arena.push_heavy(Heavy::Transition {
            site,
            expr: one,
            delay: Some(two),
            rise_time: None,
            fall_time: None,
        }));
        out.push(arena.push_heavy(Heavy::Cross {
            expr: one,
            direction: Some(two),
            time_tol: None,
            expr_tol: None,
            enable: None,
        }));
        out
    }

    #[test]
    fn rewrite_that_changes_nothing_returns_the_input_and_pushes_nothing() {
        let mut arena = ExprArena::new();
        for id in one_of_every_node(&mut arena) {
            let before = arena.len();
            let rewritten = rewrite(&mut arena, id, &mut |_, _| None);
            let node = *arena.node(id);
            assert_eq!(rewritten, id, "rewrite moved an unchanged {node:?}");
            assert_eq!(
                arena.len(),
                before,
                "rewrite allocated on an unchanged {node:?}"
            );
        }
    }

    #[test]
    fn rewrite_appends_only_the_path_that_changed() {
        let mut arena = ExprArena::new();
        let left = marker(&mut arena, "left");
        let right = marker(&mut arena, "right");
        let root = arena.push(Node::Binary(BinaryOp::Add, left, right));
        let before = arena.len();

        let replacement = arena.intern("replacement");
        let rewritten = rewrite(&mut arena, root, &mut |arena, node| match node {
            Node::Var(name) if arena.name(name).as_str() == "right" => Some(Node::Var(replacement)),
            _ => None,
        });

        assert_ne!(rewritten, root);
        // One replacement node and one rebuilt parent; the untouched left
        // operand keeps its id.
        assert_eq!(arena.len(), before + 2);
        let Node::Binary(_, new_left, new_right) = *arena.node(rewritten) else {
            panic!("the rewritten root is still a binary node");
        };
        assert_eq!(new_left, left);
        assert_ne!(new_right, right);
    }

    /// A shared subtree carrying a `transition`, walked twice.
    ///
    /// This is the property the producer stage's move onto the arena put at
    /// risk and the reason the five site-ordinal walks recurse rather than
    /// memoize: `x + x` over one shared node has **two** `transition` sites in
    /// the emitted program, not one, so a walk over it must hand out two
    /// ordinals and must not collapse them.
    #[test]
    fn a_walk_over_a_shared_subtree_unfolds_it() {
        let mut arena = ExprArena::new();
        let inner = marker(&mut arena, "x");
        let shared = arena.push_heavy(Heavy::Transition {
            site: TransitionSiteId {
                source: 0,
                start: 0,
                end: 1,
                ordinal: 0,
            },
            expr: inner,
            delay: None,
            rise_time: None,
            fall_time: None,
        });
        let root = arena.push(Node::Binary(BinaryOp::Add, shared, shared));

        let mut seen = 0;
        visit(&arena, root, &mut |node| {
            if matches!(node, Node::Heavy(HeavyKind::Transition, _)) {
                seen += 1;
            }
        });
        assert_eq!(
            seen, 2,
            "a shared subtree is visited once per path, not once per node"
        );

        // And a rewrite gives each path its own node, which is what lets the
        // two paths take different site ordinals.
        let mut next = 0_u32;
        let rewritten = rewrite(&mut arena, root, &mut |arena, node| {
            let Node::Heavy(HeavyKind::Transition, heavy) = node else {
                return None;
            };
            let mut updated = arena.heavy(heavy).clone();
            if let Heavy::Transition { site, .. } = &mut updated {
                site.ordinal = next;
                next += 1;
            }
            let pushed = arena.push_heavy(updated);
            Some(*arena.node(pushed))
        });
        assert_eq!(next, 2, "each path was numbered");
        let Node::Binary(_, left, right) = *arena.node(rewritten) else {
            panic!("the rewritten root is still a binary node");
        };
        assert_ne!(left, right, "the two paths were given separate nodes");
        let ordinal = |id| match *arena.node(id) {
            Node::Heavy(HeavyKind::Transition, heavy) => match arena.heavy(heavy) {
                Heavy::Transition { site, .. } => site.ordinal,
                other => panic!("expected a transition, found {other:?}"),
            },
            other => panic!("expected a transition, found {other:?}"),
        };
        assert_eq!((ordinal(left), ordinal(right)), (0, 1));
    }

    #[test]
    fn the_interner_answers_by_name_and_only_by_name() {
        let mut arena = ExprArena::new();
        let first = arena.intern("gm");
        let again = arena.intern("gm");
        let other = arena.intern("gds");
        assert_eq!(first, again);
        assert_ne!(first, other);
        assert_eq!(arena.name(first).as_str(), "gm");
        assert_eq!(arena.name(other).as_str(), "gds");
    }

    #[test]
    fn nodes_read_back_across_a_chunk_boundary() {
        let mut arena = ExprArena::new();
        let count = CHUNK_LEN + 16;
        let ids = (0..count)
            .map(|index| arena.push(Node::Const(index as f64)))
            .collect::<Vec<_>>();
        assert_eq!(arena.len() as usize, count);
        for (index, id) in ids.iter().enumerate() {
            assert_eq!(*arena.node(*id), Node::Const(index as f64));
        }
    }
}
