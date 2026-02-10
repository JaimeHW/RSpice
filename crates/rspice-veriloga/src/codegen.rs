//! Code Generator for Verilog-A
//!
//! Generates simulator-ready device models from IR.
//! Produces bytecode programs for efficient simulation.

use crate::CompilerOptions;
use crate::ast::BinaryOp;
use crate::error::{CodeGenError, CodeGenErrorKind, CompileError, CompileResult};
use crate::ir::{BranchEquation, DerivativeWrt, DeviceIR, IrExpr, IrFunction};
use crate::laplace::{Complex, StateSpaceFilter};
use crate::semantic::AnalyzedFile;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// Code generator
pub struct CodeGenerator<'a> {
    #[allow(dead_code)]
    options: &'a CompilerOptions,
    /// Collected Laplace filters
    laplace_filters: std::cell::RefCell<Vec<StateSpaceFilter>>,
    /// Collected lookup tables used by $table_model expressions.
    lookup_tables: std::cell::RefCell<Vec<LookupTable>>,
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

impl Default for LookupTable {
    fn default() -> Self {
        Self::new()
    }
}

impl LookupTable {
    /// Create a new empty lookup table
    pub fn new() -> Self {
        Self {
            x_data: Vec::new(),
            y_data: Vec::new(),
            name: None,
        }
    }

    /// Create a lookup table from data vectors
    ///
    /// # Panics
    /// Panics if x_data and y_data have different lengths
    pub fn from_data(x_data: Vec<f64>, y_data: Vec<f64>) -> Self {
        assert_eq!(
            x_data.len(),
            y_data.len(),
            "LookupTable: x_data and y_data must have the same length"
        );
        Self {
            x_data,
            y_data,
            name: None,
        }
    }

    /// Create a lookup table with a name for debugging
    pub fn from_data_named(x_data: Vec<f64>, y_data: Vec<f64>, name: impl Into<SmolStr>) -> Self {
        assert_eq!(
            x_data.len(),
            y_data.len(),
            "LookupTable: x_data and y_data must have the same length"
        );
        Self {
            x_data,
            y_data,
            name: Some(name.into()),
        }
    }

    /// Check if the table is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.x_data.is_empty()
    }

    /// Get the number of data points
    #[inline]
    pub fn len(&self) -> usize {
        self.x_data.len()
    }

    /// Perform linear interpolation at the given x value
    ///
    /// Behavior:
    /// - Empty table: returns 0.0
    /// - Single point: returns that y value (constant)
    /// - x below range: linear extrapolation from first two points
    /// - x above range: linear extrapolation from last two points
    /// - x within range: linear interpolation between bracketing points
    pub fn interpolate(&self, x: f64) -> f64 {
        let n = self.x_data.len();

        // Handle edge cases
        if n == 0 {
            return 0.0;
        }
        if n == 1 {
            return self.y_data[0];
        }

        // Handle out-of-bounds with linear extrapolation
        if x <= self.x_data[0] {
            // Extrapolate below range
            return self.linear_extrapolate(0, 1, x);
        }
        if x >= self.x_data[n - 1] {
            // Extrapolate above range
            return self.linear_extrapolate(n - 2, n - 1, x);
        }

        // Binary search for the bracketing interval
        let idx = self.find_interval(x);
        self.linear_interpolate(idx, idx + 1, x)
    }

    /// Compute the derivative (slope) at the given x value
    ///
    /// This is useful for Jacobian computation. Uses the local slope
    /// of the linear interpolation segment.
    pub fn derivative(&self, x: f64) -> f64 {
        let n = self.x_data.len();

        if n < 2 {
            return 0.0;
        }

        // Find the interval and return its slope
        if x <= self.x_data[0] || x >= self.x_data[n - 1] {
            // Use endpoint slopes for extrapolation
            if x <= self.x_data[0] {
                return self.slope(0, 1);
            } else {
                return self.slope(n - 2, n - 1);
            }
        }

        let idx = self.find_interval(x);
        self.slope(idx, idx + 1)
    }

    /// Binary search to find the interval containing x
    /// Returns index i such that x_data[i] <= x < x_data[i+1]
    #[inline]
    fn find_interval(&self, x: f64) -> usize {
        // Binary search for the insertion point
        match self
            .x_data
            .binary_search_by(|probe| probe.partial_cmp(&x).unwrap_or(std::cmp::Ordering::Equal))
        {
            Ok(idx) => {
                // Exact match - use this as the lower bound
                // But clamp to ensure idx+1 is valid
                if idx >= self.x_data.len() - 1 {
                    self.x_data.len() - 2
                } else {
                    idx
                }
            }
            Err(idx) => {
                // Not found - idx is where it would be inserted
                // So the interval is [idx-1, idx]
                if idx == 0 { 0 } else { idx - 1 }
            }
        }
    }

    /// Linear interpolation between points at indices i and j
    #[inline]
    fn linear_interpolate(&self, i: usize, j: usize, x: f64) -> f64 {
        let x0 = self.x_data[i];
        let x1 = self.x_data[j];
        let y0 = self.y_data[i];
        let y1 = self.y_data[j];

        // Guard against division by zero
        if (x1 - x0).abs() < 1e-30 {
            return y0;
        }

        let t = (x - x0) / (x1 - x0);
        y0 + t * (y1 - y0)
    }

    /// Linear extrapolation using points at indices i and j
    #[inline]
    fn linear_extrapolate(&self, i: usize, j: usize, x: f64) -> f64 {
        // Same as interpolation, just allows x outside [x_i, x_j]
        self.linear_interpolate(i, j, x)
    }

    /// Compute the slope between points at indices i and j
    #[inline]
    fn slope(&self, i: usize, j: usize) -> f64 {
        let x0 = self.x_data[i];
        let x1 = self.x_data[j];
        let y0 = self.y_data[i];
        let y1 = self.y_data[j];

        if (x1 - x0).abs() < 1e-30 {
            return 0.0;
        }

        (y1 - y0) / (x1 - x0)
    }

    /// Validate the table data (sorted, no NaN, etc.)
    pub fn validate(&self) -> Result<(), String> {
        if self.x_data.len() != self.y_data.len() {
            return Err("x_data and y_data must have the same length".to_string());
        }

        // Check for NaN/Inf
        for (i, (&x, &y)) in self.x_data.iter().zip(self.y_data.iter()).enumerate() {
            if !x.is_finite() {
                return Err(format!("x_data[{}] = {} is not finite", i, x));
            }
            if !y.is_finite() {
                return Err(format!("y_data[{}] = {} is not finite", i, y));
            }
        }

        // Check sorted order
        for i in 1..self.x_data.len() {
            if self.x_data[i] < self.x_data[i - 1] {
                return Err(format!(
                    "x_data is not sorted: x[{}] = {} < x[{}] = {}",
                    i,
                    self.x_data[i],
                    i - 1,
                    self.x_data[i - 1]
                ));
            }
        }

        Ok(())
    }
}

/// Compiled parameter
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

impl<'a> CodeGenerator<'a> {
    /// Create a new code generator
    pub fn new(options: &'a CompilerOptions) -> Self {
        Self {
            options,
            laplace_filters: std::cell::RefCell::new(Vec::new()),
            lookup_tables: std::cell::RefCell::new(Vec::new()),
        }
    }

    /// Generate compiled model from analyzed file
    pub fn generate(&self, analyzed: &AnalyzedFile) -> CompileResult<CompiledModel> {
        // Get the first module (for now, single module per file)
        let module = analyzed.modules.values().next().ok_or_else(|| {
            CodeGenError::new(CodeGenErrorKind::Internal("No modules found".into()))
        })?;

        // Build IR
        let ir = DeviceIR::from_analyzed(module);

        // Generate code from IR
        self.generate_from_ir(&ir)
    }

    /// Generate from IR
    fn generate_from_ir(&self, ir: &DeviceIR) -> CompileResult<CompiledModel> {
        self.lookup_tables.borrow_mut().clear();
        self.laplace_filters.borrow_mut().clear();

        let mut model = CompiledModel {
            name: ir.name.clone(),
            num_terminals: ir.terminals.len(),
            terminal_names: ir.terminals.iter().map(|t| t.name.clone()).collect(),
            parameters: ir
                .parameters
                .iter()
                .map(|p| CompiledParameter {
                    name: p.name.clone(),
                    default: p.default,
                    min: p.min,
                    max: p.max,
                })
                .collect(),
            num_variables: ir.variables.len(),
            assignment_programs: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: ir.internal_nodes.len(),
            branch_currents: 0,
            laplace_filters: Vec::new(),
        };

        // Generate assignment programs (executed in order before contributions)
        for assign in &ir.assignments {
            let program = self.compile_expr(&assign.expr, ir)?;
            model.assignment_programs.push(AssignmentProgram {
                var_index: assign.var_index,
                program,
            });
        }

        // Generate stamp programs for each equation
        for eq in &ir.equations {
            let program = self.compile_equation(eq, ir)?;
            model.stamp_programs.push(program);
        }

        model.laplace_filters = self.laplace_filters.take();
        model.lookup_tables = self.lookup_tables.take();

        Ok(model)
    }

    /// Compile a branch equation to a stamp program
    fn compile_equation(&self, eq: &BranchEquation, ir: &DeviceIR) -> CompileResult<StampProgram> {
        let value_program = self.compile_expr(&eq.expr, ir)?;

        let mut jacobian_programs = Vec::new();
        for deriv in &eq.derivatives {
            let program = self.compile_expr(&deriv.expr, ir)?;
            let (row, col) = self.derivative_indices(&eq.branch, &deriv.wrt, eq.is_current);
            jacobian_programs.push(JacobianEntry { row, col, program });
        }

        // Build stamp locations for the contribution
        let pos = eq.branch.pos_terminal;
        let neg = eq.branch.neg_terminal;

        let stamp_locations = if eq.is_current {
            // Current contribution: stamps to RHS at pos and neg
            vec![
                StampLocation {
                    row: StampIndex::Terminal(pos),
                    col: StampIndex::Ground,
                    sign: -1.0,
                },
                StampLocation {
                    row: StampIndex::Terminal(neg),
                    col: StampIndex::Ground,
                    sign: 1.0,
                },
            ]
        } else {
            // Voltage contribution would need branch equation
            vec![]
        };

        Ok(StampProgram {
            stamp_locations,
            value_program,
            jacobian_programs,
        })
    }

    /// Get row/col for a derivative
    fn derivative_indices(
        &self,
        branch: &crate::ir::BranchRef,
        wrt: &DerivativeWrt,
        is_current: bool,
    ) -> (StampIndex, StampIndex) {
        match wrt {
            DerivativeWrt::Voltage(node) => {
                if is_current {
                    (
                        StampIndex::Terminal(branch.pos_terminal),
                        StampIndex::Terminal(*node),
                    )
                } else {
                    (StampIndex::Internal(0), StampIndex::Terminal(*node))
                }
            }
            DerivativeWrt::Current(p, _n) => (StampIndex::Terminal(*p), StampIndex::Internal(0)),
            DerivativeWrt::Time => (StampIndex::Ground, StampIndex::Ground),
        }
    }

    /// Compile an IR expression to bytecode
    fn compile_expr(&self, expr: &IrExpr, ir: &DeviceIR) -> CompileResult<BytecodeProgram> {
        let mut program = BytecodeProgram::default();
        self.emit_expr(expr, ir, &mut program)?;
        Ok(program)
    }

    /// Emit bytecode for an expression
    fn emit_expr(
        &self,
        expr: &IrExpr,
        ir: &DeviceIR,
        program: &mut BytecodeProgram,
    ) -> CompileResult<()> {
        match expr {
            IrExpr::Const(v) => {
                program.instructions.push(Instruction::PushConst(*v));
            }
            IrExpr::Param(name) => {
                let idx = ir
                    .parameters
                    .iter()
                    .position(|p| &p.name == name)
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown parameter: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushParam(idx));
            }
            IrExpr::Var(name) => {
                let idx = ir
                    .variables
                    .iter()
                    .position(|v| &v.name == name)
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown variable: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushVariable(idx));
            }
            IrExpr::Voltage(p, n) => {
                program.instructions.push(Instruction::PushVoltage(*p, *n));
            }
            IrExpr::Current(p, n) => {
                program.instructions.push(Instruction::PushCurrent(*p, *n));
            }
            IrExpr::Temperature => {
                program.instructions.push(Instruction::PushTemperature);
            }
            IrExpr::Vt => {
                program.instructions.push(Instruction::PushVt);
            }
            IrExpr::Time => {
                program.instructions.push(Instruction::PushTime);
            }
            IrExpr::Binary(op, left, right) => {
                self.emit_expr(left, ir, program)?;
                self.emit_expr(right, ir, program)?;
                program.instructions.push(match op {
                    // Arithmetic
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Sub => Instruction::Sub,
                    BinaryOp::Mul => Instruction::Mul,
                    BinaryOp::Div => Instruction::Div,
                    BinaryOp::Pow => Instruction::Pow,
                    // Comparisons
                    BinaryOp::Gt => Instruction::Gt,
                    BinaryOp::Lt => Instruction::Lt,
                    BinaryOp::Ge => Instruction::Ge,
                    BinaryOp::Le => Instruction::Le,
                    BinaryOp::Eq => Instruction::Eq,
                    BinaryOp::Ne => Instruction::Ne,
                    // Logical
                    BinaryOp::And => Instruction::And,
                    BinaryOp::Or => Instruction::Or,
                    _ => {
                        return Err(CompileError::CodeGen(CodeGenError::new(
                            CodeGenErrorKind::UnsupportedFeature(format!("Binary op {:?}", op)),
                        )));
                    }
                });
            }
            IrExpr::Unary(crate::ast::UnaryOp::Neg, inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Neg);
            }
            IrExpr::Call(func, args) => {
                for arg in args {
                    self.emit_expr(arg, ir, program)?;
                }
                program.instructions.push(match func {
                    IrFunction::Abs => Instruction::Abs,
                    IrFunction::Sqrt => Instruction::Sqrt,
                    IrFunction::Exp => Instruction::Exp,
                    IrFunction::Log => Instruction::Log,
                    IrFunction::Log10 => Instruction::Log10,
                    IrFunction::Sin => Instruction::Sin,
                    IrFunction::Cos => Instruction::Cos,
                    IrFunction::Tan => Instruction::Tan,
                    IrFunction::Sinh => Instruction::Sinh,
                    IrFunction::Cosh => Instruction::Cosh,
                    IrFunction::Tanh => Instruction::Tanh,
                    IrFunction::Min => Instruction::Min,
                    IrFunction::Max => Instruction::Max,
                    // Inverse trig
                    IrFunction::Asin => Instruction::Asin,
                    IrFunction::Acos => Instruction::Acos,
                    IrFunction::Atan => Instruction::Atan,
                    IrFunction::Atan2 => Instruction::Atan2,
                    // Rounding
                    IrFunction::Floor => Instruction::Floor,
                    IrFunction::Ceil => Instruction::Ceil,
                    // Power
                    IrFunction::Pow => Instruction::FnPow,
                });
            }
            IrExpr::Limexp(inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Limexp);
            }
            IrExpr::Conditional(cond, then_expr, else_expr) => {
                self.emit_expr(cond, ir, program)?;
                self.emit_expr(then_expr, ir, program)?;
                self.emit_expr(else_expr, ir, program)?;
                program.instructions.push(Instruction::IfElse);
            }
            IrExpr::Unary(crate::ast::UnaryOp::Not, inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Not);
            }
            IrExpr::Ddt(inner) => {
                // For DC analysis, ddt = 0. For transient, would need state tracking.
                // For now, emit 0 for DC compatibility
                let _ = inner; // Mark as intentionally unused for now
                program.instructions.push(Instruction::PushConst(0.0));
            }
            IrExpr::Idt(inner, ic) => {
                // For DC analysis, idt behavior depends on context
                // For now, use initial condition if provided, else 0
                if let Some(ic_expr) = ic {
                    self.emit_expr(ic_expr, ir, program)?;
                } else {
                    let _ = inner; // Mark as intentionally unused for now
                    program.instructions.push(Instruction::PushConst(0.0));
                }
            }
            IrExpr::Limit(inner, step) => {
                // $limit(expr, step) - bounds value change per Newton iteration
                // For DC, we track previous value and limit the step
                // State index 0 is used for limit tracking
                self.emit_expr(inner, ir, program)?;
                if let Some(step_expr) = step {
                    self.emit_expr(step_expr, ir, program)?;
                } else {
                    // Default step limit for pn-junction type limiting
                    program.instructions.push(Instruction::PushConst(0.7)); // ~2*Vt
                }
                program.instructions.push(Instruction::LimitState(0));
            }
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => {
                // $table_model lookup with linear interpolation
                // Emit input expression, then TableLookup instruction referencing the table
                self.emit_expr(input, ir, program)?;
                let table_id = self.register_lookup_table(x_data, y_data)?;
                program
                    .instructions
                    .push(Instruction::TableLookup(table_id));
            }
            IrExpr::AbsDelay { expr, delay_time } => {
                // absdelay(expr, delay_time) - transport delay
                // Emit expression value, then delay time, then AbsDelayState instruction
                self.emit_expr(expr, ir, program)?;
                self.emit_expr(delay_time, ir, program)?;
                // Use delay buffer index 0 for now
                // In production, each absdelay call would have its own buffer
                program.instructions.push(Instruction::AbsDelayState(0));
            }
            IrExpr::Transition {
                expr,
                delay,
                rise_time,
                fall_time,
            } => {
                // transition(expr, delay, rise_time, fall_time)
                self.emit_expr(expr, ir, program)?;
                // Emit delay (default 0)
                if let Some(d) = delay {
                    self.emit_expr(d, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Emit rise_time (default 0 = instantaneous)
                if let Some(r) = rise_time {
                    self.emit_expr(r, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                // Emit fall_time (default to rise_time)
                if let Some(f) = fall_time {
                    self.emit_expr(f, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                program.instructions.push(Instruction::TransitionState(0));
            }
            IrExpr::Slew {
                expr,
                max_pos_slew,
                max_neg_slew,
            } => {
                // slew(expr, max_pos_slew, max_neg_slew)
                self.emit_expr(expr, ir, program)?;
                // Emit max_pos_slew (default infinity = no limit)
                if let Some(p) = max_pos_slew {
                    self.emit_expr(p, ir, program)?;
                } else {
                    program
                        .instructions
                        .push(Instruction::PushConst(f64::INFINITY));
                }
                // Emit max_neg_slew (default to max_pos_slew)
                if let Some(n) = max_neg_slew {
                    self.emit_expr(n, ir, program)?;
                } else {
                    program
                        .instructions
                        .push(Instruction::PushConst(f64::INFINITY));
                }
                program.instructions.push(Instruction::SlewState(0));
            }
            IrExpr::Cross {
                expr,
                direction,
                time_tol: _,
            } => {
                // cross(expr, direction, time_tol)
                self.emit_expr(expr, ir, program)?;
                // Push direction constant (-1, 0, or +1)
                let dir = direction.unwrap_or(0);
                program
                    .instructions
                    .push(Instruction::PushConst(dir as f64));
                program.instructions.push(Instruction::CrossState(0));
            }
            IrExpr::WhiteNoise { power, name: _ } => {
                // $white_noise(power, name)
                // In time domain, noise returns 0
                // Contributes to AC noise analysis
                self.emit_expr(power, ir, program)?;
                program.instructions.push(Instruction::WhiteNoise);
            }
            IrExpr::FlickerNoise {
                power,
                exponent,
                name: _,
            } => {
                // $flicker_noise(power, exponent, name)
                self.emit_expr(power, ir, program)?;
                self.emit_expr(exponent, ir, program)?;
                program.instructions.push(Instruction::FlickerNoise);
            }
            IrExpr::Analysis(name) => {
                // analysis(name) - check current analysis type
                let analysis_id = match name.to_lowercase().as_str() {
                    "dc" => 0,
                    "ac" => 1,
                    "tran" | "transient" => 2,
                    "noise" => 3,
                    "ic" => 4,
                    _ => 255, // Unknown = always false
                };
                program
                    .instructions
                    .push(Instruction::Analysis(analysis_id));
            }
            IrExpr::Above {
                expr,
                threshold,
                time_tol: _,
            } => {
                // above(expr, threshold) - level crossing
                self.emit_expr(expr, ir, program)?;
                self.emit_expr(threshold, ir, program)?;
                program.instructions.push(Instruction::AboveState(0));
            }
            IrExpr::Timer { start_time, period } => {
                // timer(start, period) - periodic trigger
                self.emit_expr(start_time, ir, program)?;
                if let Some(p) = period {
                    self.emit_expr(p, ir, program)?;
                } else {
                    program.instructions.push(Instruction::PushConst(0.0));
                }
                program.instructions.push(Instruction::TimerState(0));
            }
            IrExpr::Unary(op, _) => {
                return Err(CompileError::CodeGen(CodeGenError::new(
                    CodeGenErrorKind::UnsupportedFeature(format!("Unary op {:?}", op)),
                )));
            }
            IrExpr::LaplaceZP {
                expr,
                zeros,
                poles,
                gain,
            } => {
                self.emit_expr(expr, ir, program)?;

                let p_complex: Vec<Complex> = poles
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect();
                let z_complex: Vec<Complex> = zeros
                    .iter()
                    .map(|(re, im)| Complex::new(*re, *im))
                    .collect();

                let filter = StateSpaceFilter::from_poles_zeros(&p_complex, &z_complex, *gain);
                let filter_id = self.laplace_filters.borrow().len();
                self.laplace_filters.borrow_mut().push(filter);

                program
                    .instructions
                    .push(Instruction::LaplaceState(filter_id));
            }
            IrExpr::LaplaceND {
                expr,
                numerator,
                denominator,
            } => {
                self.emit_expr(expr, ir, program)?;

                // IR has ascending powers: n0 + n1*s + ...
                // StateSpaceFilter expects descending: n_k*s^k + ... + n0
                let mut num_desc = numerator.clone();
                num_desc.reverse();
                let mut den_desc = denominator.clone();
                den_desc.reverse();

                let filter = StateSpaceFilter::from_transfer_function(&num_desc, &den_desc);
                let filter_id = self.laplace_filters.borrow().len();
                self.laplace_filters.borrow_mut().push(filter);

                program
                    .instructions
                    .push(Instruction::LaplaceState(filter_id));
            }
        }
        Ok(())
    }

    fn register_lookup_table(&self, x_data: &[f64], y_data: &[f64]) -> CompileResult<usize> {
        if x_data.len() != y_data.len() {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model x/y table length mismatch".into(),
            ))
            .into());
        }
        if x_data.len() < 2 {
            return Err(CodeGenError::new(CodeGenErrorKind::InvalidExpression(
                "$table_model requires at least two table points".into(),
            ))
            .into());
        }

        let mut tables = self.lookup_tables.borrow_mut();
        if let Some((existing_idx, _)) = tables
            .iter()
            .enumerate()
            .find(|(_, table)| table.x_data == x_data && table.y_data == y_data)
        {
            return Ok(existing_idx);
        }

        let table = LookupTable::from_data(x_data.to_vec(), y_data.to_vec());
        tables.push(table);
        Ok(tables.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IrExpr;

    // ========================================================================
    // Bytecode Program Tests
    // ========================================================================

    #[test]
    fn test_bytecode_program_creation() {
        let mut program = BytecodeProgram::default();
        program.instructions.push(Instruction::PushConst(1.0));
        program.instructions.push(Instruction::PushConst(2.0));
        program.instructions.push(Instruction::Add);
        assert_eq!(program.instructions.len(), 3);
    }

    #[test]
    fn test_instruction_variants() {
        // Test all instruction variants can be created
        let instructions = vec![
            Instruction::PushConst(std::f64::consts::PI),
            Instruction::PushParam(0),
            Instruction::PushVoltage(0, 1),
            Instruction::PushCurrent(0, 1),
            Instruction::PushTime,
            Instruction::PushTemperature,
            Instruction::PushVt,
            Instruction::Add,
            Instruction::Sub,
            Instruction::Mul,
            Instruction::Div,
            Instruction::Neg,
            Instruction::Pow,
            Instruction::Abs,
            Instruction::Sqrt,
            Instruction::Exp,
            Instruction::Log,
            Instruction::Log10,
            Instruction::Sin,
            Instruction::Cos,
            Instruction::Tan,
            Instruction::Sinh,
            Instruction::Cosh,
            Instruction::Tanh,
            Instruction::Min,
            Instruction::Max,
            Instruction::Limexp,
            Instruction::IfElse,
        ];
        assert_eq!(instructions.len(), 28);
    }

    // ========================================================================
    // Code Generation Tests
    // ========================================================================

    #[test]
    fn test_compile_const_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Const(42.0);
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 1);
        assert!(
            matches!(program.instructions[0], Instruction::PushConst(v) if (v - 42.0).abs() < 1e-10)
        );
    }

    #[test]
    fn test_compile_voltage_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Voltage(0, 1);
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 1);
        assert!(matches!(
            program.instructions[0],
            Instruction::PushVoltage(0, 1)
        ));
    }

    #[test]
    fn test_compile_param_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Param("g".into());
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 1);
        assert!(matches!(program.instructions[0], Instruction::PushParam(0)));
    }

    #[test]
    fn test_compile_binary_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // g * V(0,1)
        let expr = IrExpr::Binary(
            BinaryOp::Mul,
            Box::new(IrExpr::Param("g".into())),
            Box::new(IrExpr::Voltage(0, 1)),
        );
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 3);
        assert!(matches!(program.instructions[0], Instruction::PushParam(0)));
        assert!(matches!(
            program.instructions[1],
            Instruction::PushVoltage(0, 1)
        ));
        assert!(matches!(program.instructions[2], Instruction::Mul));
    }

    #[test]
    fn test_compile_function_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // exp(V(0,1))
        let expr = IrExpr::Call(crate::ir::IrFunction::Exp, vec![IrExpr::Voltage(0, 1)]);
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(
            program.instructions[0],
            Instruction::PushVoltage(0, 1)
        ));
        assert!(matches!(program.instructions[1], Instruction::Exp));
    }

    #[test]
    fn test_compile_conditional_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // cond ? 1.0 : 0.0
        let expr = IrExpr::Conditional(
            Box::new(IrExpr::Voltage(0, 1)),
            Box::new(IrExpr::Const(1.0)),
            Box::new(IrExpr::Const(0.0)),
        );
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 4);
        assert!(matches!(program.instructions[3], Instruction::IfElse));
    }

    #[test]
    fn test_compile_limexp() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Limexp(Box::new(IrExpr::Const(20.0)));
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(program.instructions[1], Instruction::Limexp));
    }

    #[test]
    fn test_compile_negation() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Unary(crate::ast::UnaryOp::Neg, Box::new(IrExpr::Const(5.0)));
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(program.instructions[0], Instruction::PushConst(_)));
        assert!(matches!(program.instructions[1], Instruction::Neg));
    }

    #[test]
    fn test_compile_system_vars() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();

        // Temperature
        let temp = codegen.compile_expr(&IrExpr::Temperature, &ir).unwrap();
        assert!(matches!(temp.instructions[0], Instruction::PushTemperature));

        // Vt
        let vt = codegen.compile_expr(&IrExpr::Vt, &ir).unwrap();
        assert!(matches!(vt.instructions[0], Instruction::PushVt));

        // Time
        let time = codegen.compile_expr(&IrExpr::Time, &ir).unwrap();
        assert!(matches!(time.instructions[0], Instruction::PushTime));
    }

    #[test]
    fn test_compile_complex_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // is * (exp(V(0,1) / vt) - 1)
        // Simplified: exp(V) - 1
        let expr = IrExpr::Binary(
            BinaryOp::Sub,
            Box::new(IrExpr::Call(
                crate::ir::IrFunction::Exp,
                vec![IrExpr::Voltage(0, 1)],
            )),
            Box::new(IrExpr::Const(1.0)),
        );
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        // PushVoltage, Exp, PushConst(1), Sub
        assert_eq!(program.instructions.len(), 4);
    }

    #[test]
    fn test_compile_table_lookup_registers_table_and_instruction() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);
        let ir = create_test_ir();

        let expr = IrExpr::TableLookup {
            input: Box::new(IrExpr::Voltage(0, 1)),
            x_data: vec![0.0, 1.0, 2.0],
            y_data: vec![0.0, 1.0, 4.0],
        };
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(
            program.instructions[0],
            Instruction::PushVoltage(0, 1)
        ));
        assert!(matches!(
            program.instructions[1],
            Instruction::TableLookup(0)
        ));
        let tables = codegen.lookup_tables.borrow();
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].x_data, vec![0.0, 1.0, 2.0]);
        assert_eq!(tables[0].y_data, vec![0.0, 1.0, 4.0]);
    }

    #[test]
    fn test_compile_table_lookup_reuses_identical_table() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);
        let ir = create_test_ir();

        let expr_a = IrExpr::TableLookup {
            input: Box::new(IrExpr::Const(0.25)),
            x_data: vec![0.0, 1.0],
            y_data: vec![0.0, 2.0],
        };
        let expr_b = IrExpr::TableLookup {
            input: Box::new(IrExpr::Const(0.75)),
            x_data: vec![0.0, 1.0],
            y_data: vec![0.0, 2.0],
        };
        let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
        let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

        assert!(matches!(
            prog_a.instructions[1],
            Instruction::TableLookup(0)
        ));
        assert!(matches!(
            prog_b.instructions[1],
            Instruction::TableLookup(0)
        ));
        assert_eq!(codegen.lookup_tables.borrow().len(), 1);
    }

    #[test]
    fn test_compile_table_lookup_assigns_distinct_ids_for_distinct_tables() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);
        let ir = create_test_ir();

        let expr_a = IrExpr::TableLookup {
            input: Box::new(IrExpr::Const(0.25)),
            x_data: vec![0.0, 1.0],
            y_data: vec![0.0, 1.0],
        };
        let expr_b = IrExpr::TableLookup {
            input: Box::new(IrExpr::Const(0.25)),
            x_data: vec![0.0, 2.0],
            y_data: vec![0.0, 1.0],
        };
        let prog_a = codegen.compile_expr(&expr_a, &ir).unwrap();
        let prog_b = codegen.compile_expr(&expr_b, &ir).unwrap();

        assert!(matches!(
            prog_a.instructions[1],
            Instruction::TableLookup(0)
        ));
        assert!(matches!(
            prog_b.instructions[1],
            Instruction::TableLookup(1)
        ));
        assert_eq!(codegen.lookup_tables.borrow().len(), 2);
    }

    #[test]
    fn test_stamp_location() {
        let loc = StampLocation {
            row: StampIndex::Terminal(0),
            col: StampIndex::Terminal(1),
            sign: 1.0,
        };
        assert!(matches!(loc.row, StampIndex::Terminal(0)));
        assert!(matches!(loc.col, StampIndex::Terminal(1)));
        assert!((loc.sign - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_stamp_index_variants() {
        let term = StampIndex::Terminal(0);
        let internal = StampIndex::Internal(1);
        let ground = StampIndex::Ground;

        assert!(matches!(term, StampIndex::Terminal(0)));
        assert!(matches!(internal, StampIndex::Internal(1)));
        assert!(matches!(ground, StampIndex::Ground));
    }

    // ========================================================================
    // Helper Functions
    // ========================================================================

    fn create_test_ir() -> crate::ir::DeviceIR {
        crate::ir::DeviceIR {
            name: "test".into(),
            terminals: vec![
                crate::ir::Terminal {
                    name: "p".into(),
                    index: 0,
                },
                crate::ir::Terminal {
                    name: "n".into(),
                    index: 1,
                },
            ],
            internal_nodes: vec![],
            parameters: vec![crate::ir::ParamDef {
                name: "g".into(),
                default: 0.001,
                min: Some(0.0),
                max: None,
            }],
            variables: vec![],
            assignments: vec![],
            equations: vec![],
            noise_sources: vec![],
        }
    }

    // ========================================================================
    // LookupTable Tests - Comprehensive Commercial-Grade Coverage
    // ========================================================================

    #[test]
    fn test_lookup_table_empty() {
        let table = LookupTable::new();
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
        // Empty table returns 0 for any input
        assert_eq!(table.interpolate(0.0), 0.0);
        assert_eq!(table.interpolate(1.0), 0.0);
        assert_eq!(table.interpolate(-100.0), 0.0);
        assert_eq!(table.derivative(0.0), 0.0);
    }

    #[test]
    fn test_lookup_table_single_point() {
        let table = LookupTable::from_data(vec![1.0], vec![5.0]);
        assert!(!table.is_empty());
        assert_eq!(table.len(), 1);
        // Single point table returns constant for any input
        assert_eq!(table.interpolate(0.0), 5.0);
        assert_eq!(table.interpolate(1.0), 5.0);
        assert_eq!(table.interpolate(100.0), 5.0);
        assert_eq!(table.derivative(0.0), 0.0);
    }

    #[test]
    fn test_lookup_table_two_points() {
        // Simple line from (0, 0) to (1, 1)
        let table = LookupTable::from_data(vec![0.0, 1.0], vec![0.0, 1.0]);

        // Exact points
        assert!((table.interpolate(0.0) - 0.0).abs() < 1e-12);
        assert!((table.interpolate(1.0) - 1.0).abs() < 1e-12);

        // Midpoint
        assert!((table.interpolate(0.5) - 0.5).abs() < 1e-12);

        // Other interpolation
        assert!((table.interpolate(0.25) - 0.25).abs() < 1e-12);
        assert!((table.interpolate(0.75) - 0.75).abs() < 1e-12);

        // Derivative is constant 1.0
        assert!((table.derivative(0.5) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_lookup_table_linear_extrapolation_below() {
        // Line from (1, 10) to (2, 20)
        let table = LookupTable::from_data(vec![1.0, 2.0], vec![10.0, 20.0]);

        // Below range - linear extrapolation with slope 10
        assert!((table.interpolate(0.0) - 0.0).abs() < 1e-12); // 10 - 10*1 = 0
        assert!((table.interpolate(-1.0) - (-10.0)).abs() < 1e-12); // 10 - 10*2 = -10
    }

    #[test]
    fn test_lookup_table_linear_extrapolation_above() {
        // Line from (1, 10) to (2, 20)
        let table = LookupTable::from_data(vec![1.0, 2.0], vec![10.0, 20.0]);

        // Above range - linear extrapolation with slope 10
        assert!((table.interpolate(3.0) - 30.0).abs() < 1e-12); // 20 + 10*1 = 30
        assert!((table.interpolate(4.0) - 40.0).abs() < 1e-12); // 20 + 10*2 = 40
    }

    #[test]
    fn test_lookup_table_multiple_segments() {
        // Piecewise linear: (0,0), (1,2), (2,1), (3,3)
        let table = LookupTable::from_data(vec![0.0, 1.0, 2.0, 3.0], vec![0.0, 2.0, 1.0, 3.0]);

        // Exact points
        assert!((table.interpolate(0.0) - 0.0).abs() < 1e-12);
        assert!((table.interpolate(1.0) - 2.0).abs() < 1e-12);
        assert!((table.interpolate(2.0) - 1.0).abs() < 1e-12);
        assert!((table.interpolate(3.0) - 3.0).abs() < 1e-12);

        // Interpolation in first segment (slope = 2)
        assert!((table.interpolate(0.5) - 1.0).abs() < 1e-12);

        // Interpolation in second segment (slope = -1)
        assert!((table.interpolate(1.5) - 1.5).abs() < 1e-12);

        // Interpolation in third segment (slope = 2)
        assert!((table.interpolate(2.5) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_lookup_table_derivative() {
        // Piecewise linear with different slopes
        let table = LookupTable::from_data(vec![0.0, 1.0, 3.0], vec![0.0, 2.0, 6.0]);

        // First segment: slope = 2
        assert!((table.derivative(0.5) - 2.0).abs() < 1e-12);

        // Second segment: slope = (6-2)/(3-1) = 2
        assert!((table.derivative(2.0) - 2.0).abs() < 1e-12);

        // Extrapolation uses endpoint slopes
        assert!((table.derivative(-1.0) - 2.0).abs() < 1e-12);
        assert!((table.derivative(5.0) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_lookup_table_negative_values() {
        // Table with negative x and y
        let table = LookupTable::from_data(vec![-2.0, -1.0, 0.0, 1.0], vec![-4.0, -1.0, 0.0, 1.0]);

        assert!((table.interpolate(-1.5) - (-2.5)).abs() < 1e-12);
        assert!((table.interpolate(-0.5) - (-0.5)).abs() < 1e-12);
        assert!((table.interpolate(0.5) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_lookup_table_validate_success() {
        let table = LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]);
        assert!(table.validate().is_ok());
    }

    #[test]
    fn test_lookup_table_validate_unsorted() {
        let mut table = LookupTable::new();
        table.x_data = vec![0.0, 2.0, 1.0]; // Not sorted!
        table.y_data = vec![0.0, 1.0, 2.0];
        assert!(table.validate().is_err());
    }

    #[test]
    fn test_lookup_table_validate_nan() {
        let mut table = LookupTable::new();
        table.x_data = vec![0.0, f64::NAN, 2.0];
        table.y_data = vec![0.0, 1.0, 2.0];
        assert!(table.validate().is_err());
    }

    #[test]
    fn test_lookup_table_validate_inf() {
        let mut table = LookupTable::new();
        table.x_data = vec![0.0, 1.0, 2.0];
        table.y_data = vec![0.0, f64::INFINITY, 2.0];
        assert!(table.validate().is_err());
    }

    #[test]
    fn test_lookup_table_very_close_x_values() {
        // Test numerical stability with very close x values
        let table = LookupTable::from_data(vec![1.0, 1.0 + 1e-15, 2.0], vec![10.0, 10.0, 20.0]);
        // Should not crash, may return first value for near-duplicate x
        let result = table.interpolate(1.0 + 1e-16);
        assert!(result.is_finite());
    }

    #[test]
    fn test_lookup_table_large_scale() {
        // Large table for binary search performance
        let n = 1000;
        let x_data: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let y_data: Vec<f64> = (0..n).map(|i| (i as f64).powi(2)).collect();
        let table = LookupTable::from_data(x_data, y_data);

        // Test interpolation at various points
        assert!((table.interpolate(500.0) - 250000.0).abs() < 1e-12);
        // Linear interp between 500²=250000 and 501²=251001 at 500.5
        // = 250000 + (251001-250000)*0.5 = 250500.5
        assert!((table.interpolate(500.5) - 250500.5).abs() < 1e-9);

        // Verify it performs well (no timeout issues with binary search)
        for i in 0..100 {
            let x = i as f64 * 10.0;
            let _ = table.interpolate(x);
        }
    }

    #[test]
    fn test_lookup_table_named() {
        let table = LookupTable::from_data_named(vec![0.0, 1.0], vec![0.0, 1.0], "diode_iv");
        assert!(table.name.is_some());
        assert_eq!(table.name.as_ref().unwrap().as_str(), "diode_iv");
    }

    #[test]
    fn test_lookup_table_realistic_diode() {
        // Realistic diode I-V characteristic approximation
        // V:   -1.0, -0.5,  0.0,  0.2,  0.4,  0.5,  0.6,  0.7
        // I:    0.0,  0.0,  0.0, 1e-8, 1e-5, 1e-3, 0.01, 0.1
        let table = LookupTable::from_data(
            vec![-1.0, -0.5, 0.0, 0.2, 0.4, 0.5, 0.6, 0.7],
            vec![0.0, 0.0, 0.0, 1e-8, 1e-5, 1e-3, 0.01, 0.1],
        );

        // Reverse bias - zero current
        assert!(table.interpolate(-0.7).abs() < 1e-12);

        // Forward bias - increasing current
        assert!(table.interpolate(0.65) > 0.001);
        assert!(table.interpolate(0.7) > 0.01);
    }
}
