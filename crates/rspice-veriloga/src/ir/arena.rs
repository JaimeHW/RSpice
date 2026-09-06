//! A compact arena for Verilog-A IR expressions.
//!
//! [`IrExpr`] is a 120-byte enum whose children are `Box`es, which the Windows
//! heap rounds to 128 bytes apiece. A shadow-expanded assignment forest is
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
//! Nothing here is on the production path yet. [`ExprArena::import`] and
//! [`ExprArena::export`] bridge losslessly to and from [`IrExpr`] so the
//! emitter, the AD core and the producers can each be ported against a type
//! that already exists.

use crate::ast::{BinaryOp, UnaryOp};
use smol_str::SmolStr;
use std::collections::HashMap;
use std::num::NonZeroU32;

use super::{
    AbsDelaySiteId, DdxAxis, IrExpr, IrFunction, LaplaceSiteId, NoiseSiteId, SlewSiteId,
    TransitionSiteId, ZiPolynomialDefinition, ZiSiteId,
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
    /// Whether an external terminal was connected on this instance.
    PortConnected(u32),
    /// Binary operation.
    Binary(BinaryOp, NodeId, NodeId),
    /// Unary operation.
    Unary(UnaryOp, NodeId),
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
    /// It exists so [`ExprArena::import`] is total over an [`IrExpr::Call`]
    /// whose argument list the front end never checked against an arity.
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
        /// `+1` rising, `-1` falling, `None` either.
        direction: Option<i32>,
    },
    /// One of the eighteen site-bearing, event, noise and filter operators,
    /// whose fields live in `ExprArena::heavy(_)`. The kind is repeated inline
    /// so a walk can classify a node without touching the side table.
    Heavy(HeavyKind, HeavyId),
}

/// Which [`Heavy`] payload a [`Node::Heavy`] carries.
///
/// The names are [`IrExpr`]'s, so a variant-name trace over [`Node`] reads the
/// same as one over [`IrExpr`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum HeavyKind {
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
/// These are [`IrExpr`]'s field shapes verbatim, with `Box<IrExpr>` children
/// replaced by [`NodeId`]. They live beside the nodes rather than inside them
/// because the widest of them is 113 bytes and would otherwise set the size of
/// every node in the forest, while none of them occurs in any shipped
/// assignment forest at all.
#[derive(Clone, PartialEq, Debug)]
pub enum Heavy {
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
}

/// The `usize` a terminal pair uses for ground (`expr_converter::GROUND_NODE`),
/// packed into a node's `u32` slots as `PACKED_GROUND`.
const GROUND_INDEX: usize = usize::MAX;
/// Ground's packed spelling. No real index may take this value.
const PACKED_GROUND: u32 = u32::MAX;

/// Pack a terminal or ordinal index into a node's `u32` slot.
fn pack_index(value: usize) -> u32 {
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
fn unpack_index(value: u32) -> usize {
    if value == PACKED_GROUND {
        GROUND_INDEX
    } else {
        value as usize
    }
}

impl ExprArena {
    /// Copy an [`IrExpr`] tree into the arena and return its root.
    ///
    /// Children are pushed before their parent, so an imported tree occupies a
    /// contiguous range of ids in post-order. Nothing is deduplicated: a
    /// subtree the source cloned twice becomes two node ranges, exactly as it
    /// is two subtrees today. Hash-consing is a separate, measured step and it
    /// may never apply to a site-bearing or state-allocating node.
    pub fn import(&mut self, expr: &IrExpr) -> NodeId {
        let node = match expr {
            IrExpr::Const(value) => Node::Const(*value),
            IrExpr::Param(name) => {
                let name = self.intern(name);
                Node::Param(name)
            }
            IrExpr::ParamGiven(name) => {
                let name = self.intern(name);
                Node::ParamGiven(name)
            }
            IrExpr::Var(name) => {
                let name = self.intern(name);
                Node::Var(name)
            }
            IrExpr::VarIndexed {
                array,
                base,
                len,
                lower,
                index,
            } => {
                let index = self.import(index);
                let array = self.intern(array);
                let payload = self.push_indexed(IndexedRead {
                    array,
                    base: *base,
                    len: *len,
                    lower: *lower,
                });
                Node::VarIndexed { payload, index }
            }
            IrExpr::Voltage(pos, neg) => Node::Voltage(pack_index(*pos), pack_index(*neg)),
            IrExpr::Current(pos, neg) => Node::Current(pack_index(*pos), pack_index(*neg)),
            IrExpr::BranchCurrent(ordinal) => Node::BranchCurrent(pack_index(*ordinal)),
            IrExpr::Time => Node::Time,
            IrExpr::Temperature => Node::Temperature,
            IrExpr::Vt => Node::Vt,
            IrExpr::Mfactor => Node::Mfactor,
            IrExpr::PortConnected(port) => Node::PortConnected(pack_index(*port)),
            IrExpr::Binary(op, left, right) => {
                let left = self.import(left);
                let right = self.import(right);
                Node::Binary(*op, left, right)
            }
            IrExpr::Unary(op, inner) => {
                let inner = self.import(inner);
                Node::Unary(*op, inner)
            }
            IrExpr::Call(func, args) => {
                let args = args.iter().map(|arg| self.import(arg)).collect::<Vec<_>>();
                return self.push_call(*func, &args);
            }
            IrExpr::Ddt(inner) => {
                let inner = self.import(inner);
                Node::Ddt(inner)
            }
            IrExpr::Idt(inner, ic) => {
                let inner = self.import(inner);
                let ic = self.import_optional(ic);
                Node::Idt(inner, ic)
            }
            IrExpr::IdtMod {
                expr,
                ic,
                modulus,
                offset,
            } => {
                let expr = self.import(expr);
                let ic = self.import_optional(ic);
                let modulus = self.import(modulus);
                let offset = self.import_optional(offset);
                let payload = self.push_optional_pair((ic, offset));
                Node::IdtMod {
                    expr,
                    modulus,
                    payload,
                }
            }
            IrExpr::Limexp(inner) => {
                let inner = self.import(inner);
                Node::Limexp(inner)
            }
            IrExpr::Limit(inner, step) => {
                let inner = self.import(inner);
                let step = self.import_optional(step);
                Node::Limit(inner, step)
            }
            IrExpr::CanonicalLimit(inner) => {
                let inner = self.import(inner);
                Node::CanonicalLimit(inner)
            }
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => {
                let input = self.import(input);
                let table = self.push_table(x_data.clone(), y_data.clone());
                Node::TableLookup { input, table }
            }
            IrExpr::TableDerivative {
                input,
                x_data,
                y_data,
            } => {
                let input = self.import(input);
                let table = self.push_table(x_data.clone(), y_data.clone());
                Node::TableDerivative { input, table }
            }
            IrExpr::Ddx { expr, axis } => {
                let expr = self.import(expr);
                let axis = self.push_ddx_axis(*axis);
                Node::Ddx { expr, axis }
            }
            IrExpr::DdtCompanion(inner) => {
                let inner = self.import(inner);
                Node::DdtCompanion(inner)
            }
            IrExpr::IdtCompanion(inner) => {
                let inner = self.import(inner);
                Node::IdtCompanion(inner)
            }
            IrExpr::Conditional(condition, then_expr, else_expr) => {
                let condition = self.import(condition);
                let then_expr = self.import(then_expr);
                let else_expr = self.import(else_expr);
                Node::Conditional(condition, then_expr, else_expr)
            }
            IrExpr::Analysis(name) => {
                let name = self.intern(name);
                Node::Analysis(name)
            }
            IrExpr::LastCrossing { expr, direction } => {
                let expr = self.import(expr);
                Node::LastCrossing {
                    expr,
                    direction: *direction,
                }
            }
            IrExpr::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => {
                let expr = self.import(expr);
                let delay_time = self.import(delay_time);
                let max_delay = self.import_optional(max_delay);
                return self.push_heavy(Heavy::AbsDelay {
                    site: *site,
                    expr,
                    delay_time,
                    max_delay,
                });
            }
            IrExpr::AbsDelayDerivative {
                site,
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                derivative_order,
            } => {
                let input = self.import(input);
                let input_derivative = self.import(input_derivative);
                let delay_time = self.import(delay_time);
                let delay_derivative = self.import(delay_derivative);
                let max_delay = self.import_optional(max_delay);
                return self.push_heavy(Heavy::AbsDelayDerivative {
                    site: *site,
                    input,
                    input_derivative,
                    delay_time,
                    delay_derivative,
                    max_delay,
                    derivative_order: *derivative_order,
                });
            }
            IrExpr::Transition {
                site,
                expr,
                delay,
                rise_time,
                fall_time,
            } => {
                let expr = self.import(expr);
                let delay = self.import_optional(delay);
                let rise_time = self.import_optional(rise_time);
                let fall_time = self.import_optional(fall_time);
                return self.push_heavy(Heavy::Transition {
                    site: *site,
                    expr,
                    delay,
                    rise_time,
                    fall_time,
                });
            }
            IrExpr::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
            } => {
                let input = self.import(input);
                let input_derivative = self.import(input_derivative);
                let delay = self.import_optional(delay);
                let rise_time = self.import_optional(rise_time);
                let fall_time = self.import_optional(fall_time);
                return self.push_heavy(Heavy::TransitionDerivative {
                    site: *site,
                    input,
                    input_derivative,
                    delay,
                    rise_time,
                    fall_time,
                });
            }
            IrExpr::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => {
                let expr = self.import(expr);
                let max_pos_slew = self.import_optional(max_pos_slew);
                let max_neg_slew = self.import_optional(max_neg_slew);
                return self.push_heavy(Heavy::Slew {
                    site: *site,
                    expr,
                    max_pos_slew,
                    max_neg_slew,
                });
            }
            IrExpr::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
            } => {
                let input = self.import(input);
                let input_derivative = self.import(input_derivative);
                let max_pos_slew = self.import_optional(max_pos_slew);
                let max_pos_slew_derivative = self.import_optional(max_pos_slew_derivative);
                let max_neg_slew = self.import_optional(max_neg_slew);
                let max_neg_slew_derivative = self.import_optional(max_neg_slew_derivative);
                return self.push_heavy(Heavy::SlewDerivative {
                    site: *site,
                    input,
                    input_derivative,
                    max_pos_slew,
                    max_pos_slew_derivative,
                    max_neg_slew,
                    max_neg_slew_derivative,
                });
            }
            IrExpr::Cross {
                expr,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => {
                let expr = self.import(expr);
                let direction = self.import_optional(direction);
                let time_tol = self.import_optional(time_tol);
                let expr_tol = self.import_optional(expr_tol);
                let enable = self.import_optional(enable);
                return self.push_heavy(Heavy::Cross {
                    expr,
                    direction,
                    time_tol,
                    expr_tol,
                    enable,
                });
            }
            IrExpr::Above {
                expr,
                time_tol,
                expr_tol,
                enable,
            } => {
                let expr = self.import(expr);
                let time_tol = self.import_optional(time_tol);
                let expr_tol = self.import_optional(expr_tol);
                let enable = self.import_optional(enable);
                return self.push_heavy(Heavy::Above {
                    expr,
                    time_tol,
                    expr_tol,
                    enable,
                });
            }
            IrExpr::Timer {
                start_time,
                period,
                time_tol,
                enable,
            } => {
                let start_time = self.import(start_time);
                let period = self.import_optional(period);
                let time_tol = self.import_optional(time_tol);
                let enable = self.import_optional(enable);
                return self.push_heavy(Heavy::Timer {
                    start_time,
                    period,
                    time_tol,
                    enable,
                });
            }
            IrExpr::WhiteNoise { site, power, name } => {
                let power = self.import(power);
                return self.push_heavy(Heavy::WhiteNoise {
                    site: *site,
                    power,
                    name: name.clone(),
                });
            }
            IrExpr::FlickerNoise {
                site,
                power,
                exponent,
                name,
            } => {
                let power = self.import(power);
                let exponent = self.import(exponent);
                return self.push_heavy(Heavy::FlickerNoise {
                    site: *site,
                    power,
                    exponent,
                    name: name.clone(),
                });
            }
            IrExpr::NoiseTable {
                site,
                points,
                log_interp,
                name,
            } => {
                return self.push_heavy(Heavy::NoiseTable {
                    site: *site,
                    points: points.clone(),
                    log_interp: *log_interp,
                    name: name.clone(),
                });
            }
            IrExpr::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                let expr = self.import(expr);
                return self.push_heavy(Heavy::LaplaceZP {
                    site: *site,
                    expr,
                    zeros: zeros.clone(),
                    poles: poles.clone(),
                    gain: *gain,
                });
            }
            IrExpr::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => {
                let expr = self.import(expr);
                return self.push_heavy(Heavy::LaplaceND {
                    site: *site,
                    expr,
                    numerator: numerator.clone(),
                    denominator: denominator.clone(),
                });
            }
            IrExpr::LaplaceZPDerivative {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => {
                let expr = self.import(expr);
                return self.push_heavy(Heavy::LaplaceZPDerivative {
                    site: *site,
                    expr,
                    zeros: zeros.clone(),
                    poles: poles.clone(),
                    gain: *gain,
                });
            }
            IrExpr::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            } => {
                let expr = self.import(expr);
                return self.push_heavy(Heavy::LaplaceNDDerivative {
                    site: *site,
                    expr,
                    numerator: numerator.clone(),
                    denominator: denominator.clone(),
                });
            }
            IrExpr::ZiFilter {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let expr = self.import(expr);
                let numerator = self.import_zi_polynomial(numerator);
                let denominator = self.import_zi_polynomial(denominator);
                let period = self.import(period);
                let transition = self.import(transition);
                let first_transition = self.import(first_transition);
                return self.push_heavy(Heavy::ZiFilter {
                    site: *site,
                    expr,
                    numerator,
                    denominator,
                    period,
                    transition,
                    first_transition,
                    direct_assignment: *direct_assignment,
                });
            }
            IrExpr::ZiFilterDerivative {
                site,
                expr,
                numerator,
                denominator,
                period,
                transition,
                first_transition,
                direct_assignment,
            } => {
                let expr = self.import(expr);
                let numerator = self.import_zi_polynomial(numerator);
                let denominator = self.import_zi_polynomial(denominator);
                let period = self.import(period);
                let transition = self.import(transition);
                let first_transition = self.import(first_transition);
                return self.push_heavy(Heavy::ZiFilterDerivative {
                    site: *site,
                    expr,
                    numerator,
                    denominator,
                    period,
                    transition,
                    first_transition,
                    direct_assignment: *direct_assignment,
                });
            }
        };
        self.push(node)
    }

    fn import_optional(&mut self, expr: &Option<Box<IrExpr>>) -> Option<NodeId> {
        expr.as_ref().map(|expr| self.import(expr))
    }

    fn import_zi_polynomial(&mut self, polynomial: &ZiPolynomialDefinition) -> ZiPolynomial {
        match polynomial {
            ZiPolynomialDefinition::Coefficients(terms) => {
                ZiPolynomial::Coefficients(terms.iter().map(|term| self.import(term)).collect())
            }
            ZiPolynomialDefinition::Roots(pairs) => ZiPolynomial::Roots(
                pairs
                    .iter()
                    .map(|(real, imaginary)| (self.import(real), self.import(imaginary)))
                    .collect(),
            ),
        }
    }

    /// Rebuild the [`IrExpr`] tree rooted at `id`.
    ///
    /// The inverse of [`ExprArena::import`] on every one of the 48 variants.
    /// A shared subtree is written out once per path, so exporting a forest
    /// the AD core shared is exactly as large as the same forest is today.
    pub fn export(&self, id: NodeId) -> IrExpr {
        match *self.node(id) {
            Node::Const(value) => IrExpr::Const(value),
            Node::Param(name) => IrExpr::Param(self.name(name).clone()),
            Node::ParamGiven(name) => IrExpr::ParamGiven(self.name(name).clone()),
            Node::Var(name) => IrExpr::Var(self.name(name).clone()),
            Node::VarIndexed { payload, index } => {
                let read = self.indexed(payload);
                IrExpr::VarIndexed {
                    array: self.name(read.array).clone(),
                    base: read.base,
                    len: read.len,
                    lower: read.lower,
                    index: self.export_boxed(index),
                }
            }
            Node::Voltage(pos, neg) => IrExpr::Voltage(unpack_index(pos), unpack_index(neg)),
            Node::Current(pos, neg) => IrExpr::Current(unpack_index(pos), unpack_index(neg)),
            Node::BranchCurrent(ordinal) => IrExpr::BranchCurrent(unpack_index(ordinal)),
            Node::Time => IrExpr::Time,
            Node::Temperature => IrExpr::Temperature,
            Node::Vt => IrExpr::Vt,
            Node::Mfactor => IrExpr::Mfactor,
            Node::PortConnected(port) => IrExpr::PortConnected(unpack_index(port)),
            Node::Binary(op, left, right) => {
                IrExpr::Binary(op, self.export_boxed(left), self.export_boxed(right))
            }
            Node::Unary(op, inner) => IrExpr::Unary(op, self.export_boxed(inner)),
            Node::Call { func, a, b, .. } => IrExpr::Call(
                func,
                a.into_iter().chain(b).map(|arg| self.export(arg)).collect(),
            ),
            Node::CallSpilled { func, args } => IrExpr::Call(
                func,
                self.call_args(args)
                    .iter()
                    .map(|arg| self.export(*arg))
                    .collect(),
            ),
            Node::Ddt(inner) => IrExpr::Ddt(self.export_boxed(inner)),
            Node::Idt(inner, ic) => IrExpr::Idt(self.export_boxed(inner), self.export_optional(ic)),
            Node::IdtMod {
                expr,
                modulus,
                payload,
            } => {
                let (ic, offset) = self.optional_pair(payload);
                IrExpr::IdtMod {
                    expr: self.export_boxed(expr),
                    ic: self.export_optional(ic),
                    modulus: self.export_boxed(modulus),
                    offset: self.export_optional(offset),
                }
            }
            Node::Limexp(inner) => IrExpr::Limexp(self.export_boxed(inner)),
            Node::Limit(inner, step) => {
                IrExpr::Limit(self.export_boxed(inner), self.export_optional(step))
            }
            Node::CanonicalLimit(inner) => IrExpr::CanonicalLimit(self.export_boxed(inner)),
            Node::TableLookup { input, table } => {
                let (x_data, y_data) = self.table(table);
                IrExpr::TableLookup {
                    input: self.export_boxed(input),
                    x_data: x_data.clone(),
                    y_data: y_data.clone(),
                }
            }
            Node::TableDerivative { input, table } => {
                let (x_data, y_data) = self.table(table);
                IrExpr::TableDerivative {
                    input: self.export_boxed(input),
                    x_data: x_data.clone(),
                    y_data: y_data.clone(),
                }
            }
            Node::Ddx { expr, axis } => IrExpr::Ddx {
                expr: self.export_boxed(expr),
                axis: self.ddx_axis(axis),
            },
            Node::DdtCompanion(inner) => IrExpr::DdtCompanion(self.export_boxed(inner)),
            Node::IdtCompanion(inner) => IrExpr::IdtCompanion(self.export_boxed(inner)),
            Node::Conditional(condition, then_expr, else_expr) => IrExpr::Conditional(
                self.export_boxed(condition),
                self.export_boxed(then_expr),
                self.export_boxed(else_expr),
            ),
            Node::Analysis(name) => IrExpr::Analysis(self.name(name).to_string()),
            Node::LastCrossing { expr, direction } => IrExpr::LastCrossing {
                expr: self.export_boxed(expr),
                direction,
            },
            Node::Heavy(_, id) => self.export_heavy(self.heavy(id)),
        }
    }

    fn export_heavy(&self, heavy: &Heavy) -> IrExpr {
        match heavy {
            Heavy::AbsDelay {
                site,
                expr,
                delay_time,
                max_delay,
            } => IrExpr::AbsDelay {
                site: *site,
                expr: self.export_boxed(*expr),
                delay_time: self.export_boxed(*delay_time),
                max_delay: self.export_optional(*max_delay),
            },
            Heavy::AbsDelayDerivative {
                site,
                input,
                input_derivative,
                delay_time,
                delay_derivative,
                max_delay,
                derivative_order,
            } => IrExpr::AbsDelayDerivative {
                site: *site,
                input: self.export_boxed(*input),
                input_derivative: self.export_boxed(*input_derivative),
                delay_time: self.export_boxed(*delay_time),
                delay_derivative: self.export_boxed(*delay_derivative),
                max_delay: self.export_optional(*max_delay),
                derivative_order: *derivative_order,
            },
            Heavy::Transition {
                site,
                expr,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::Transition {
                site: *site,
                expr: self.export_boxed(*expr),
                delay: self.export_optional(*delay),
                rise_time: self.export_optional(*rise_time),
                fall_time: self.export_optional(*fall_time),
            },
            Heavy::TransitionDerivative {
                site,
                input,
                input_derivative,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::TransitionDerivative {
                site: *site,
                input: self.export_boxed(*input),
                input_derivative: self.export_boxed(*input_derivative),
                delay: self.export_optional(*delay),
                rise_time: self.export_optional(*rise_time),
                fall_time: self.export_optional(*fall_time),
            },
            Heavy::Slew {
                site,
                expr,
                max_pos_slew,
                max_neg_slew,
            } => IrExpr::Slew {
                site: *site,
                expr: self.export_boxed(*expr),
                max_pos_slew: self.export_optional(*max_pos_slew),
                max_neg_slew: self.export_optional(*max_neg_slew),
            },
            Heavy::SlewDerivative {
                site,
                input,
                input_derivative,
                max_pos_slew,
                max_pos_slew_derivative,
                max_neg_slew,
                max_neg_slew_derivative,
            } => IrExpr::SlewDerivative {
                site: *site,
                input: self.export_boxed(*input),
                input_derivative: self.export_boxed(*input_derivative),
                max_pos_slew: self.export_optional(*max_pos_slew),
                max_pos_slew_derivative: self.export_optional(*max_pos_slew_derivative),
                max_neg_slew: self.export_optional(*max_neg_slew),
                max_neg_slew_derivative: self.export_optional(*max_neg_slew_derivative),
            },
            Heavy::Cross {
                expr,
                direction,
                time_tol,
                expr_tol,
                enable,
            } => IrExpr::Cross {
                expr: self.export_boxed(*expr),
                direction: self.export_optional(*direction),
                time_tol: self.export_optional(*time_tol),
                expr_tol: self.export_optional(*expr_tol),
                enable: self.export_optional(*enable),
            },
            Heavy::Above {
                expr,
                time_tol,
                expr_tol,
                enable,
            } => IrExpr::Above {
                expr: self.export_boxed(*expr),
                time_tol: self.export_optional(*time_tol),
                expr_tol: self.export_optional(*expr_tol),
                enable: self.export_optional(*enable),
            },
            Heavy::Timer {
                start_time,
                period,
                time_tol,
                enable,
            } => IrExpr::Timer {
                start_time: self.export_boxed(*start_time),
                period: self.export_optional(*period),
                time_tol: self.export_optional(*time_tol),
                enable: self.export_optional(*enable),
            },
            Heavy::WhiteNoise { site, power, name } => IrExpr::WhiteNoise {
                site: *site,
                power: self.export_boxed(*power),
                name: name.clone(),
            },
            Heavy::FlickerNoise {
                site,
                power,
                exponent,
                name,
            } => IrExpr::FlickerNoise {
                site: *site,
                power: self.export_boxed(*power),
                exponent: self.export_boxed(*exponent),
                name: name.clone(),
            },
            Heavy::NoiseTable {
                site,
                points,
                log_interp,
                name,
            } => IrExpr::NoiseTable {
                site: *site,
                points: points.clone(),
                log_interp: *log_interp,
                name: name.clone(),
            },
            Heavy::LaplaceZP {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => IrExpr::LaplaceZP {
                site: *site,
                expr: self.export_boxed(*expr),
                zeros: zeros.clone(),
                poles: poles.clone(),
                gain: *gain,
            },
            Heavy::LaplaceND {
                site,
                expr,
                numerator,
                denominator,
            } => IrExpr::LaplaceND {
                site: *site,
                expr: self.export_boxed(*expr),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
            },
            Heavy::LaplaceZPDerivative {
                site,
                expr,
                zeros,
                poles,
                gain,
            } => IrExpr::LaplaceZPDerivative {
                site: *site,
                expr: self.export_boxed(*expr),
                zeros: zeros.clone(),
                poles: poles.clone(),
                gain: *gain,
            },
            Heavy::LaplaceNDDerivative {
                site,
                expr,
                numerator,
                denominator,
            } => IrExpr::LaplaceNDDerivative {
                site: *site,
                expr: self.export_boxed(*expr),
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
            } => IrExpr::ZiFilter {
                site: *site,
                expr: self.export_boxed(*expr),
                numerator: self.export_zi_polynomial(numerator),
                denominator: self.export_zi_polynomial(denominator),
                period: self.export_boxed(*period),
                transition: self.export_boxed(*transition),
                first_transition: self.export_boxed(*first_transition),
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
            } => IrExpr::ZiFilterDerivative {
                site: *site,
                expr: self.export_boxed(*expr),
                numerator: self.export_zi_polynomial(numerator),
                denominator: self.export_zi_polynomial(denominator),
                period: self.export_boxed(*period),
                transition: self.export_boxed(*transition),
                first_transition: self.export_boxed(*first_transition),
                direct_assignment: *direct_assignment,
            },
        }
    }

    fn export_boxed(&self, id: NodeId) -> Box<IrExpr> {
        Box::new(self.export(id))
    }

    fn export_optional(&self, id: Option<NodeId>) -> Option<Box<IrExpr>> {
        id.map(|id| self.export_boxed(id))
    }

    fn export_zi_polynomial(&self, polynomial: &ZiPolynomial) -> ZiPolynomialDefinition {
        match polynomial {
            ZiPolynomial::Coefficients(terms) => ZiPolynomialDefinition::Coefficients(
                terms.iter().map(|term| self.export(*term)).collect(),
            ),
            ZiPolynomial::Roots(pairs) => ZiPolynomialDefinition::Roots(
                pairs
                    .iter()
                    .map(|(real, imaginary)| (self.export(*real), self.export(*imaginary)))
                    .collect(),
            ),
        }
    }
}
