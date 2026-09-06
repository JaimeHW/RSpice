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
