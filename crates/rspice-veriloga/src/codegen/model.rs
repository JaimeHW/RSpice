use super::*;

/// Code generator
pub struct CodeGenerator<'a> {
    #[allow(dead_code)]
    pub(super) options: &'a CompilerOptions,
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
}

/// Compiled device model ready for simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledModel {
    /// Model name
    pub name: SmolStr,
    /// Number of terminals
    pub num_terminals: usize,
    /// Terminal names
    pub terminal_names: Vec<SmolStr>,
    /// Parameter definitions
    pub parameters: Vec<CompiledParameter>,
    /// Number of variables
    pub num_variables: usize,
    /// Variable assignment programs (executed in order before contributions)
    pub assignment_programs: Vec<AssignmentProgram>,
    /// Compiled stamp programs for each contribution
    pub stamp_programs: Vec<StampProgram>,
    /// Lookup tables for $table_model (x_data, y_data pairs)
    pub lookup_tables: Vec<LookupTable>,
    /// Number of internal nodes (if any)
    pub internal_nodes: usize,
    /// Number of branch currents to track
    pub branch_currents: usize,
    /// Laplace state-space filters
    pub laplace_filters: Vec<StateSpaceFilter>,
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
    pub default: f64,
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
}

/// Assignment program for a variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentProgram {
    /// Index of variable being assigned
    pub var_index: usize,
    /// The bytecode program to compute the value
    pub program: BytecodeProgram,
}

/// Location to stamp in matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampLocation {
    pub row: StampIndex,
    pub col: StampIndex,
    pub sign: f64,
}

/// Index for stamping (terminal or internal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StampIndex {
    Terminal(usize),
    Internal(usize),
    Ground,
}

/// Jacobian entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JacobianEntry {
    pub row: StampIndex,
    pub col: StampIndex,
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
    /// Push voltage V(i, j)
    PushVoltage(usize, usize),
    /// Push current I(i, j)
    PushCurrent(usize, usize),
    /// Push internal node voltage (for internal nodes not in port list)
    PushInternalVoltage(usize),
    /// Push variable value
    PushVariable(usize),
    /// Push temperature
    PushTemperature,
    /// Push thermal voltage
    PushVt,
    /// Push time
    PushTime,
    /// Binary operations
    Add,
    Sub,
    Mul,
    Div,
    Pow,
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
    /// Uses backward Euler: (current - prev) / dt
    DdtState(usize),
    /// State-based integration: idt(expr, ic) using state index
    /// Uses forward Euler: prev + expr * dt
    IdtState(usize),
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
