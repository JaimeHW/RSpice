use super::*;

/// Code generator
pub struct CodeGenerator {
    /// Collected Laplace filters
    pub(super) laplace_filters: std::cell::RefCell<Vec<StateSpaceFilter>>,
    /// Collected lookup tables used by $table_model expressions.
    pub(super) lookup_tables: std::cell::RefCell<Vec<LookupTable>>,
    /// Stateful slot allocator for `$limit`.
    pub(super) limit_state_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `absdelay`.
    pub(super) delay_buffer_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `transition`.
    pub(super) transition_filter_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `slew`.
    pub(super) slew_filter_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `cross`.
    pub(super) cross_detector_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `above`.
    pub(super) above_detector_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `timer`.
    pub(super) timer_state_count: std::cell::Cell<usize>,
    /// Collected z-domain filters (`zi_*`).
    pub(super) zi_filters: std::cell::RefCell<Vec<crate::zfilter::ZiFilter>>,
}

/// Compiled device model ready for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledModel {
    /// Model name
    pub name: SmolStr,
    /// Stable digest of the preprocessed source text used to produce this
    /// model. Canonical-native compilation requires this to match the
    /// canonical IR artifact so bytecode-era lowering cannot be paired with
    /// unrelated MIR/HIR.
    #[serde(default)]
    pub source_digest: SmolStr,
    /// Number of terminals
    pub num_terminals: usize,
    /// Terminal names
    pub terminal_names: Vec<SmolStr>,
    /// Parameter definitions
    pub parameters: Vec<CompiledParameter>,
    /// Number of variables
    pub num_variables: usize,
    /// Variable names (index-aligned with the runtime variable storage);
    /// used for operating-point reporting and debugging
    pub variable_names: Vec<SmolStr>,
    /// Evaluation steps (assignments and runtime loops), executed in order
    /// before the contributions
    pub assignment_steps: Vec<AssignmentStep>,
    /// Compiled stamp programs for each contribution
    pub stamp_programs: Vec<StampProgram>,
    /// Lookup tables for $table_model (x_data, y_data pairs)
    pub lookup_tables: Vec<LookupTable>,
    /// Number of internal nodes (if any)
    pub internal_nodes: usize,
    /// Branch-current unknowns required by potential contributions; the
    /// engine must allocate one extra system unknown per entry
    pub branch_sources: Vec<CompiledBranchSource>,
    /// Laplace state-space filters
    pub laplace_filters: Vec<StateSpaceFilter>,
    /// Z-domain (sampled-data) filters
    pub zi_filters: Vec<crate::zfilter::ZiFilter>,
    /// Small-signal noise sources extracted from contributions
    pub noise_sources: Vec<CompiledNoiseSource>,
}

/// Compiled noise source: PSD evaluated at the operating point, injected
/// at the originating contribution's branch during noise analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledNoiseSource {
    /// Positive injection node
    pub pos: StampIndex,
    /// Negative injection node
    pub neg: StampIndex,
    /// Current contribution (true) injects across the nodes; a potential
    /// contribution injects at its branch-equation row as a series EMF
    pub is_current: bool,
    /// Branch ordinal for potential contributions
    pub branch_ordinal: Option<usize>,
    /// Originating stamp program (activation gates with it)
    pub program_idx: usize,
    /// Power spectral density at the operating point (A²/Hz for current
    /// contributions, V²/Hz for potential contributions). For table
    /// sources this is the amplitude-squared scale on the interpolated
    /// value.
    pub psd_program: BytecodeProgram,
    /// Flicker frequency exponent program (None = white)
    pub exponent_program: Option<BytecodeProgram>,
    /// Frequency-interpolated PSD table: sorted (f, p) points and whether
    /// interpolation runs in log-log coordinates
    pub table: Option<(Vec<(f64, f64)>, bool)>,
    /// Source label from the noise function's name argument
    pub name: Option<SmolStr>,
}

/// Lookup table for $table_model interpolation
///
/// Provides commercial-grade linear interpolation with:
/// - Binary search for O(log n) lookup performance
/// - Linear extrapolation beyond table bounds
/// - Proper handling of edge cases (empty, single point, duplicate x)
/// - Optional derivative computation for Jacobian calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupTable {
    /// X (input) values - must be sorted in ascending order
    pub x_data: Vec<f64>,
    /// Y (output) values - same length as x_data
    pub y_data: Vec<f64>,
    /// Optional table name for debugging
    pub name: Option<SmolStr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledParameter {
    pub name: SmolStr,
    /// Alternate instance-facing names (aliasparam); setting an alias
    /// writes this parameter
    pub aliases: Vec<SmolStr>,
    pub default: f64,
    /// Program computing the default from other parameters (evaluated in
    /// declaration order for parameters the instance did not set)
    pub default_program: Option<BytecodeProgram>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Stamp program for a contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampProgram {
    /// Which row/col this stamps to
    pub stamp_locations: Vec<StampLocation>,
    /// The bytecode program to evaluate the value
    pub value_program: BytecodeProgram,
    /// Jacobian programs (one per derivative)
    pub jacobian_programs: Vec<JacobianEntry>,
    /// Reactive Jacobian programs: dQ/dx of the contribution's ddt()
    /// operand. AC analysis stamps these as jw * dQ/dx (capacitances for
    /// current contributions, inductances for potential contributions).
    pub reactive_jacobians: Vec<JacobianEntry>,
    /// For potential contributions: the branch-current unknown this
    /// equation defines. None for current contributions.
    pub branch_ordinal: Option<usize>,
    /// Indirect contribution: the value program computes a constraint
    /// residual stamped current-style onto the branch row (the device
    /// accumulates its companion RHS like a KCL row, not a source row)
    pub indirect: bool,
    /// Instance-static activation condition (parameter-only). When it
    /// evaluates to zero the program is skipped entirely - for potential
    /// contributions this leaves the branch open instead of shorting it.
    pub static_condition: Option<BytecodeProgram>,
}

/// Assignment program for a variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentProgram {
    /// Index of variable being assigned
    pub var_index: usize,
    /// The bytecode program to compute the value
    pub program: BytecodeProgram,
}

/// One evaluation step: a variable assignment or a runtime-bounded loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentStep {
    /// Compute a value and store it in a variable
    Assign(AssignmentProgram),
    /// Compute an element index and a value, then store the value in
    /// element `index - lower` of the contiguous variable run at `base`.
    /// Out-of-range indexes are a runtime error (never a silent skip).
    AssignIndexed {
        /// First element's variable slot
        base: usize,
        /// Number of elements
        len: usize,
        /// Declared lower bound
        lower: i64,
        /// Element index program
        index: BytecodeProgram,
        /// Value program
        value: BytecodeProgram,
    },
    /// Execute the body steps while the condition program evaluates
    /// nonzero (re-checked before every iteration)
    Loop {
        condition: BytecodeProgram,
        body: Vec<AssignmentStep>,
    },
}

/// Location to stamp in matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampLocation {
    pub row: StampIndex,
    pub col: StampIndex,
    pub sign: f64,
}

/// Index for stamping (terminal, internal node, branch unknown, or ground)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StampIndex {
    Terminal(usize),
    Internal(usize),
    /// Branch-current unknown introduced by a potential contribution
    Branch(usize),
    Ground,
}

/// Differentiation column axis of a Jacobian entry
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColumnAxis {
    /// Unified node index (terminals first, then internal nodes)
    Node(usize),
    /// Branch-current unknown ordinal
    Branch(usize),
}

/// A branch-current unknown of a potential contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledBranchSource {
    /// Positive node of the source branch
    pub pos: StampIndex,
    /// Negative node of the source branch
    pub neg: StampIndex,
    /// Constrained by an indirect contribution: the branch row holds the
    /// constraint equation, so the structural V(p)-V(n) row entries are
    /// not stamped (the KCL column couplings remain)
    pub indirect: bool,
}

/// Jacobian entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JacobianEntry {
    pub row: StampIndex,
    pub col: StampIndex,
    /// Differentiation axis of this column (node voltage or branch
    /// current). Used to compute the companion RHS term G*x.
    pub col_axis: ColumnAxis,
    /// Sign applied to the derivative value when stamping
    pub sign: f64,
    pub program: BytecodeProgram,
}

/// Bytecode program for expression evaluation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BytecodeProgram {
    pub instructions: Vec<Instruction>,
}

/// VM Instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    /// Push constant
    PushConst(f64),
    /// Push parameter value
    PushParam(usize),
    /// Push 1.0 if the parameter was explicitly set on the instance
    PushParamGiven(usize),
    /// Push the value of a branch-current unknown (potential contribution)
    PushBranchCurrent(usize),
    /// Push voltage V(i, j)
    PushVoltage(usize, usize),
    /// Push current I(i, j)
    PushCurrent(usize, usize),
    /// Push internal node voltage (for internal nodes not in port list)
    PushInternalVoltage(usize),
    /// Push variable value
    PushVariable(usize),
    /// Pop an element index, then push the value of element
    /// `index - lower` from the contiguous variable run at `base`.
    /// Out-of-range indexes are a runtime error.
    PushVariableDyn {
        base: usize,
        len: usize,
        lower: i64,
    },
    /// Push temperature
    PushTemperature,
    /// Push thermal voltage
    PushVt,
    /// Push time
    PushTime,
    /// Push the instance multiplicity ($mfactor)
    PushMfactor,
    /// Push whether an external terminal is connected on this instance.
    PushPortConnected(usize),
    /// Z-domain filter: pop the input, push the sampled-data output
    ZiState(usize),
    /// Binary operations
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    /// Modulus (fmod semantics on reals, LRM 4.2.3)
    Mod,
    /// Bitwise/shift operations (operands truncate to integers)
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    /// Unary operations
    Neg,
    /// Functions
    Abs,
    Sqrt,
    Exp,
    Log,
    Log10,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Min,
    Max,
    /// Limited exponential (for convergence)
    Limexp,
    /// CMC-style limited exponential with low-side clamp.
    LimitedExp,
    /// Inverse trigonometric functions
    Asin,
    Acos,
    Atan,
    Atan2, // 2-argument arctangent(y, x)
    /// Rounding functions
    Floor,
    Ceil,
    /// Power function (2-argument)
    FnPow,
    /// Comparison operations (return 1.0 for true, 0.0 for false)
    Gt, // Greater than
    Lt, // Less than
    Ge, // Greater than or equal
    Le, // Less than or equal
    Eq, // Equal
    Ne, // Not equal
    /// Logical operations
    And, // Logical and
    Or, // Logical or
    Not, // Logical not
    /// State-based time derivative: ddt(expr) using state index
    /// Backward Euler: (current - prev) / dt; records current into state
    /// Stack: [expr] -> [d(expr)/dt]
    DdtState(usize),
    /// State-based integration: idt(expr, ic) using state index
    /// Backward Euler: prev + expr * dt; returns ic at DC
    /// Stack: [expr, ic] -> [integral]
    IdtState(usize),
    /// Wrapped integration: idtmod(expr, ic, modulus, offset)
    /// The integral folds into [offset, offset + modulus)
    /// Stack: [expr, ic, modulus, offset] -> [wrapped integral]
    IdtModState(usize),
    /// Companion Jacobian factor for ddt: top-of-stack / dt (0 at DC)
    DdtJacobian,
    /// Companion Jacobian factor for idt: top-of-stack * dt (0 at DC)
    IdtJacobian,
    /// Slope of lookup table at the input point
    /// Stack: [input] -> [dy/dx]
    TableDerivative(usize),
    /// $limit function: bounds value change per iteration for convergence
    /// Uses state index to track previous value
    LimitState(usize),
    /// Lookup table interpolation: uses table_id to reference stored table
    /// Pops input value from stack, pushes interpolated result
    TableLookup(usize),
    /// Absolute delay: absdelay(expr, delay_time)
    /// Uses a circular buffer indexed by delay_id
    /// Stack: [expr, delay_time] -> [delayed_value]
    AbsDelayState(usize),
    /// Transition filter: piecewise-linear smoothing
    /// Stack: [expr, delay, rise_time, fall_time] -> [filtered_value]
    TransitionState(usize),
    /// Slew rate limiter
    /// Stack: [expr, max_pos_slew, max_neg_slew] -> [limited_value]  
    SlewState(usize),
    /// Cross (threshold crossing detection)
    /// Stack: [expr] -> [0 or 1]
    CrossState(usize),
    /// White noise source (returns 0 in time domain, contributes to noise analysis)
    /// Stack: [power] -> [0]
    WhiteNoise,
    /// Flicker noise source (1/f noise)
    /// Stack: [power, exponent] -> [0]
    FlickerNoise,
    /// Analysis check: returns 1 if analysis matches, else 0
    /// Parameter: analysis type ID (0=dc, 1=ac, 2=tran, etc.)
    Analysis(u8),
    /// Above event: level crossing detection above threshold
    /// Stack: [expr, threshold] -> [0 or 1]
    AboveState(usize),
    /// Timer event: periodic time-based trigger
    /// Stack: [start_time, period] -> [0 or 1]
    TimerState(usize),
    /// Laplace filter with poles/zeros (state-space form)
    /// Stack: [input] -> [filtered]
    LaplaceState(usize),
    /// Conditional: if top is nonzero, use second, else third
    IfElse,
}
