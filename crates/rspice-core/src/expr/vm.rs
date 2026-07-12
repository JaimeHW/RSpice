//! Stack-based bytecode VM for expression evaluation
//!
//! Fast execution of compiled expressions without parsing overhead.

use super::ast::{LookupInterpolation, LookupTable};
use crate::{Value, netlist::ExpressionDialect};
use std::collections::HashMap;

const TWO_PI: Value = std::f64::consts::TAU;
const EXPR_ZERO_TOLERANCE: Value = 1.0e-12;
const XYCE_ATANH_EPSILON: Value = 1.0e-12;
const XYCE_TANH_SATURATION_THRESHOLD: Value = 20.0;

/// Bytecode instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Push constant onto stack
    PushConst(Value),
    /// Push time variable
    PushTime,
    /// Push frequency variable
    PushFreq,
    /// Push circuit temperature (degrees Celsius)
    PushTemperature,
    /// Load node voltage by index
    LoadVoltage(usize),
    /// Load branch current by index
    LoadCurrent(usize),

    // Binary ops (pop 2, push 1)
    Add,
    Sub,
    Mul,
    Div,
    Pow,

    // Comparison (pop 2, push 1)
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,

    // Logical (pop 2, push 1)
    And,
    Or,

    // Unary ops (pop 1, push 1)
    Neg,
    Not,

    // Built-in functions
    Abs,
    Sqrt,
    Exp,
    Log,
    Ln,
    Log10,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2, // atan2(y, x)
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Floor,
    Ceil,
    Round,
    Sqr,

    // Multi-arg functions
    Min,
    Max,
    Pwr,
    Pwrs,
    Limit,
    Sign,
    Uramp,
    Stp,
    Ustep,
    U2,
    Eq0,
    Ne0,
    Gt0,
    Lt0,
    Ge0,
    Le0,
    Table(usize),
    Pwl(usize),
    LookupTable(usize),
    Mod,
    SpicePulse(usize),
    SpiceSin(usize),
    SpiceExp(usize),
    SpiceSffm(usize),

    /// Conditional: if cond != 0, keep second, else keep third
    IfElse,
}

/// Compiled bytecode program
#[derive(Debug, Clone, Default)]
pub struct CompiledExpr {
    /// Bytecode instructions
    pub instructions: Vec<Instruction>,
    /// Node name to index mapping
    pub node_map: HashMap<String, usize>,
    /// Branch name to index mapping
    pub branch_map: HashMap<String, usize>,
    /// File-backed lookup tables resolved during circuit build.
    pub lookup_tables: Vec<LookupTable>,
}

impl CompiledExpr {
    /// Create empty program
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or assign a node index
    pub fn get_or_create_node(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.node_map.get(name) {
            idx
        } else {
            let idx = self.node_map.len();
            self.node_map.insert(name.to_string(), idx);
            idx
        }
    }

    /// Get or assign a branch index
    pub fn get_or_create_branch(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.branch_map.get(name) {
            idx
        } else {
            let idx = self.branch_map.len();
            self.branch_map.insert(name.to_string(), idx);
            idx
        }
    }

    /// Store a file-backed lookup table and return its bytecode index.
    pub fn add_lookup_table(&mut self, table: LookupTable) -> usize {
        let idx = self.lookup_tables.len();
        self.lookup_tables.push(table);
        idx
    }
}

/// Execution context for VM
pub struct Context<'a> {
    /// Node voltages (indexed by node_map)
    pub voltages: &'a [Value],
    /// Branch currents (indexed by branch_map)
    pub currents: &'a [Value],
    /// Current simulation time
    pub time: Value,
    /// Current frequency (for AC)
    pub frequency: Value,
    /// Circuit temperature in degrees Celsius (`temper`)
    pub temperature: Value,
    /// Dialect-specific expression-function semantics.
    pub expression_dialect: ExpressionDialect,
}

impl<'a> Context<'a> {
    /// Create a new context for DC analysis
    pub fn dc(voltages: &'a [Value], currents: &'a [Value]) -> Self {
        Self {
            voltages,
            currents,
            time: 0.0,
            frequency: 0.0,
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
            expression_dialect: ExpressionDialect::Ngspice,
        }
    }

    /// Create a new context for transient analysis
    pub fn transient(voltages: &'a [Value], currents: &'a [Value], time: Value) -> Self {
        Self {
            voltages,
            currents,
            time,
            frequency: 0.0,
            temperature: crate::analysis::temperature::kelvin_to_celsius(
                crate::constants::TEMP_REFERENCE,
            ),
            expression_dialect: ExpressionDialect::Ngspice,
        }
    }

    /// Set the circuit temperature (degrees Celsius) for `temper`.
    pub fn with_temperature(mut self, temperature: Value) -> Self {
        self.temperature = temperature;
        self
    }

    /// Set dialect-specific expression-function semantics.
    pub fn with_expression_dialect(mut self, dialect: ExpressionDialect) -> Self {
        self.expression_dialect = dialect;
        self
    }
}

/// Bytecode virtual machine
#[derive(Debug, Clone)]
pub struct Vm {
    stack: Vec<Value>,
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

impl Vm {
    /// Create a new VM with preallocated stack
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(32),
        }
    }

    /// Execute a compiled expression
    pub fn execute(&mut self, program: &CompiledExpr, ctx: &Context) -> Value {
        self.stack.clear();

        for instr in &program.instructions {
            match instr {
                Instruction::PushConst(v) => self.stack.push(*v),
                Instruction::PushTime => self.stack.push(ctx.time),
                Instruction::PushFreq => self.stack.push(ctx.frequency),
                Instruction::PushTemperature => self.stack.push(ctx.temperature),

                Instruction::LoadVoltage(idx) => {
                    let v = ctx.voltages.get(*idx).copied().unwrap_or(0.0);
                    self.stack.push(v);
                }
                Instruction::LoadCurrent(idx) => {
                    let i = ctx.currents.get(*idx).copied().unwrap_or(0.0);
                    self.stack.push(i);
                }

                // Binary operations
                Instruction::Add => self.binary_op(|a, b| a + b),
                Instruction::Sub => self.binary_op(|a, b| a - b),
                Instruction::Mul => self.binary_op(|a, b| a * b),
                Instruction::Div => self.binary_op(|a, b| if b != 0.0 { a / b } else { 0.0 }),
                Instruction::Pow => self.binary_op(|a, b| a.powf(b)),

                // Comparisons
                Instruction::Lt => self.binary_op(|a, b| if a < b { 1.0 } else { 0.0 }),
                Instruction::Le => self.binary_op(|a, b| if a <= b { 1.0 } else { 0.0 }),
                Instruction::Gt => self.binary_op(|a, b| if a > b { 1.0 } else { 0.0 }),
                Instruction::Ge => self.binary_op(|a, b| if a >= b { 1.0 } else { 0.0 }),
                Instruction::Eq => self.binary_op(|a, b| {
                    if (a - b).abs() < EXPR_ZERO_TOLERANCE {
                        1.0
                    } else {
                        0.0
                    }
                }),
                Instruction::Ne => self.binary_op(|a, b| {
                    if (a - b).abs() >= EXPR_ZERO_TOLERANCE {
                        1.0
                    } else {
                        0.0
                    }
                }),

                // Logical
                Instruction::And => {
                    self.binary_op(|a, b| if a != 0.0 && b != 0.0 { 1.0 } else { 0.0 })
                }
                Instruction::Or => {
                    self.binary_op(|a, b| if a != 0.0 || b != 0.0 { 1.0 } else { 0.0 })
                }

                // Unary operations
                Instruction::Neg => self.unary_op(|a| -a),
                Instruction::Not => self.unary_op(|a| if a == 0.0 { 1.0 } else { 0.0 }),

                // Math functions
                Instruction::Abs => self.unary_op(|a| a.abs()),
                Instruction::Sqrt => self.unary_op(|a| a.max(0.0).sqrt()),
                Instruction::Exp => self.unary_op(|a| a.exp()),
                Instruction::Log => {
                    let dialect = ctx.expression_dialect;
                    self.unary_op(|a| {
                        if dialect == ExpressionDialect::Xyce {
                            a.max(1e-38).log10()
                        } else {
                            a.max(1e-38).ln()
                        }
                    });
                }
                Instruction::Ln => self.unary_op(|a| a.max(1e-38).ln()),
                Instruction::Log10 => self.unary_op(|a| a.max(1e-38).log10()),
                Instruction::Sin => self.unary_op(|a| a.sin()),
                Instruction::Cos => self.unary_op(|a| a.cos()),
                Instruction::Tan => self.unary_op(|a| a.tan()),
                Instruction::Asin => self.unary_op(|a| a.clamp(-1.0, 1.0).asin()),
                Instruction::Acos => self.unary_op(|a| a.clamp(-1.0, 1.0).acos()),
                Instruction::Atan => self.unary_op(|a| a.atan()),
                Instruction::Atan2 => self.binary_op(|y, x| y.atan2(x)),
                Instruction::Sinh => self.unary_op(|a| a.sinh()),
                Instruction::Cosh => self.unary_op(|a| a.cosh()),
                Instruction::Tanh => {
                    let dialect = ctx.expression_dialect;
                    self.unary_op(|a| {
                        if dialect == ExpressionDialect::Xyce {
                            xyce_tanh(a)
                        } else {
                            a.tanh()
                        }
                    });
                }
                Instruction::Asinh => self.unary_op(|a| a.asinh()),
                Instruction::Acosh => self.unary_op(|a| a.acosh()),
                Instruction::Atanh => {
                    let dialect = ctx.expression_dialect;
                    self.unary_op(|a| {
                        if dialect == ExpressionDialect::Xyce {
                            xyce_atanh(a)
                        } else {
                            a.atanh()
                        }
                    });
                }
                Instruction::Floor => self.unary_op(|a| a.floor()),
                Instruction::Ceil => self.unary_op(|a| a.ceil()),
                Instruction::Round => self.unary_op(|a| a.round_ties_even()),
                Instruction::Sqr => self.unary_op(|a| a * a),
                Instruction::Sign => self.unary_op(|a| {
                    if a > EXPR_ZERO_TOLERANCE {
                        1.0
                    } else if a < -EXPR_ZERO_TOLERANCE {
                        -1.0
                    } else {
                        0.0
                    }
                }),
                Instruction::Uramp => self.unary_op(|a| a.max(0.0)),
                Instruction::Stp => {
                    self.unary_op(|a| if a > EXPR_ZERO_TOLERANCE { 1.0 } else { 0.0 })
                }
                Instruction::Ustep => self.unary_op(|a| {
                    if a > 0.0 {
                        1.0
                    } else if a < 0.0 {
                        0.0
                    } else {
                        0.5
                    }
                }),
                Instruction::U2 => self.unary_op(|a| a.clamp(0.0, 1.0)),
                Instruction::Eq0 => self.unary_op(|a| {
                    if a.abs() < EXPR_ZERO_TOLERANCE {
                        1.0
                    } else {
                        0.0
                    }
                }),
                Instruction::Ne0 => self.unary_op(|a| {
                    if a.abs() >= EXPR_ZERO_TOLERANCE {
                        1.0
                    } else {
                        0.0
                    }
                }),
                Instruction::Gt0 => self.unary_op(|a| if a > 0.0 { 1.0 } else { 0.0 }),
                Instruction::Lt0 => self.unary_op(|a| if a < 0.0 { 1.0 } else { 0.0 }),
                Instruction::Ge0 => self.unary_op(|a| if a >= 0.0 { 1.0 } else { 0.0 }),
                Instruction::Le0 => self.unary_op(|a| if a <= 0.0 { 1.0 } else { 0.0 }),

                // Multi-arg functions
                Instruction::Min => self.binary_op(|a, b| a.min(b)),
                Instruction::Max => self.binary_op(|a, b| a.max(b)),
                Instruction::Pwr => self.binary_op(|a, b| a.abs().powf(b)),
                Instruction::Pwrs => self.binary_op(|a, b| a.signum() * a.abs().powf(b)),
                Instruction::Mod => self.binary_op(|a, b| if b != 0.0 { a % b } else { 0.0 }),
                Instruction::Table(arg_count) => {
                    if *arg_count >= 3 && self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let x = args[0];
                        let result = table_interpolate_from_args(x, &args[1..]);
                        self.stack.truncate(start);
                        self.stack.push(result);
                    }
                }
                Instruction::Pwl(arg_count) => {
                    if *arg_count >= 3 && self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let x = args[0];
                        let result = pwl_interpolate_from_args(x, &args[1..]);
                        self.stack.truncate(start);
                        self.stack.push(result);
                    }
                }
                Instruction::LookupTable(index) => {
                    let result = program
                        .lookup_tables
                        .get(*index)
                        .map(|table| lookup_table_interpolate(ctx.time, table))
                        .unwrap_or(0.0);
                    self.stack.push(result);
                }
                Instruction::SpicePulse(arg_count) => {
                    if self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let result = if (1..=7).contains(arg_count) {
                            spice_pulse_from_args(ctx.time, args)
                        } else {
                            0.0
                        };
                        self.stack.truncate(start);
                        self.stack.push(result);
                    }
                }
                Instruction::SpiceSin(arg_count) => {
                    if self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let result = if (3..=6).contains(arg_count) {
                            spice_sin_from_args(ctx.time, args)
                        } else {
                            0.0
                        };
                        self.stack.truncate(start);
                        self.stack.push(result);
                    }
                }
                Instruction::SpiceExp(arg_count) => {
                    if self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let result = if (2..=6).contains(arg_count) {
                            spice_exp_from_args(ctx.time, args)
                        } else {
                            0.0
                        };
                        self.stack.truncate(start);
                        self.stack.push(result);
                    }
                }
                Instruction::SpiceSffm(arg_count) => {
                    if self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let result = if (2..=5).contains(arg_count) {
                            spice_sffm_from_args(ctx.time, args)
                        } else {
                            0.0
                        };
                        self.stack.truncate(start);
                        self.stack.push(result);
                    }
                }

                Instruction::Limit => {
                    // limit(x, lo, hi) - pop 3, push 1
                    if self.stack.len() >= 3 {
                        let hi = self.stack.pop().unwrap();
                        let lo = self.stack.pop().unwrap();
                        let x = self.stack.pop().unwrap();
                        self.stack.push(x.clamp(lo, hi));
                    }
                }

                Instruction::IfElse => {
                    // if(cond, then, else) - pop 3, push 1
                    if self.stack.len() >= 3 {
                        let else_val = self.stack.pop().unwrap();
                        let then_val = self.stack.pop().unwrap();
                        let cond = self.stack.pop().unwrap();
                        self.stack
                            .push(if cond != 0.0 { then_val } else { else_val });
                    }
                }
            }
        }

        self.stack.pop().unwrap_or(0.0)
    }

    #[inline]
    fn binary_op(&mut self, f: impl Fn(Value, Value) -> Value) {
        if self.stack.len() >= 2 {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            self.stack.push(f(a, b));
        }
    }

    #[inline]
    fn unary_op(&mut self, f: impl Fn(Value) -> Value) {
        if let Some(a) = self.stack.pop() {
            self.stack.push(f(a));
        }
    }
}

fn lookup_table_interpolate(x: Value, table: &LookupTable) -> Value {
    let points = table.points.as_ref();
    match points {
        [] => 0.0,
        [(_, y)] => *y,
        _ => {
            if x <= points[0].0 {
                return points[0].1;
            }
            let last = points.len() - 1;
            if x >= points[last].0 {
                return points[last].1;
            }

            let upper = points.partition_point(|(time, _)| *time < x);
            let lower = upper.saturating_sub(1);
            match &table.interpolation {
                LookupInterpolation::Linear => interpolate_segment(
                    x,
                    points[lower].0,
                    points[lower].1,
                    points[upper].0,
                    points[upper].1,
                    points[lower].1,
                ),
                LookupInterpolation::NaturalCubic { second_derivatives } => {
                    let span = points[upper].0 - points[lower].0;
                    let lower_weight = (points[upper].0 - x) / span;
                    let upper_weight = (x - points[lower].0) / span;
                    lower_weight * points[lower].1
                        + upper_weight * points[upper].1
                        + ((lower_weight.powi(3) - lower_weight) * second_derivatives[lower]
                            + (upper_weight.powi(3) - upper_weight) * second_derivatives[upper])
                            * span.powi(2)
                            / 6.0
                }
            }
        }
    }
}

fn table_interpolate_from_args(x: Value, args: &[Value]) -> Value {
    let pair_count = args.len() / 2;
    if pair_count == 0 {
        return 0.0;
    }

    let first_x = args[0];
    let first_y = args[1];
    if pair_count == 1 {
        return first_y;
    }

    if x <= first_x {
        return first_y;
    }

    let last_idx = 2 * (pair_count - 1);
    let last_x = args[last_idx];
    let last_y = args[last_idx + 1];
    if x >= last_x {
        return last_y;
    }

    for i in 0..(pair_count - 1) {
        let idx = 2 * i;
        let x1 = args[idx];
        let y1 = args[idx + 1];
        let x2 = args[idx + 2];
        let y2 = args[idx + 3];
        if x >= x1 && x <= x2 {
            return interpolate_segment(x, x1, y1, x2, y2, y1);
        }
    }

    last_y
}

fn pwl_interpolate_from_args(x: Value, args: &[Value]) -> Value {
    let pair_count = args.len() / 2;
    if pair_count == 0 {
        return 0.0;
    }

    let first_x = args[0];
    let first_y = args[1];
    if pair_count == 1 {
        return first_y;
    }

    let second_x = args[2];
    let second_y = args[3];
    let last_idx = 2 * (pair_count - 1);
    let prev_idx = last_idx - 2;
    let last_x = args[last_idx];
    let last_y = args[last_idx + 1];
    let prev_x = args[prev_idx];
    let prev_y = args[prev_idx + 1];
    let ascending = last_x >= first_x;

    if ascending {
        if x <= first_x {
            return interpolate_segment(x, first_x, first_y, second_x, second_y, first_y);
        }
        if x >= last_x {
            return interpolate_segment(x, prev_x, prev_y, last_x, last_y, last_y);
        }
    } else {
        if x >= first_x {
            return interpolate_segment(x, first_x, first_y, second_x, second_y, first_y);
        }
        if x <= last_x {
            return interpolate_segment(x, prev_x, prev_y, last_x, last_y, last_y);
        }
    }

    for i in 0..(pair_count - 1) {
        let idx = 2 * i;
        let x1 = args[idx];
        let y1 = args[idx + 1];
        let x2 = args[idx + 2];
        let y2 = args[idx + 3];
        let in_segment = if ascending {
            x >= x1 && x <= x2
        } else {
            x <= x1 && x >= x2
        };
        if in_segment {
            return interpolate_segment(x, x1, y1, x2, y2, y1);
        }
    }

    last_y
}

#[inline]
fn interpolate_segment(x: Value, x1: Value, y1: Value, x2: Value, y2: Value, flat: Value) -> Value {
    let dx = x2 - x1;
    if !dx.is_finite() || dx == 0.0 {
        return flat;
    }
    let t = (x - x1) / dx;
    y1 + t * (y2 - y1)
}

fn spice_pulse_from_args(time: Value, args: &[Value]) -> Value {
    let v1 = args[0];
    let v2 = args.get(1).copied().unwrap_or(0.0);
    let delay = args.get(2).copied().unwrap_or(0.0);
    let rise = args.get(3).copied().unwrap_or(0.0);
    let fall = args.get(4).copied().unwrap_or(0.0);
    let width = args.get(5).copied().unwrap_or(0.0);

    if time < delay {
        return v1;
    }

    let mut elapsed = time - delay;
    if let Some(period) = args
        .get(6)
        .copied()
        .filter(|period| period.is_finite() && *period > 0.0)
    {
        elapsed = elapsed.rem_euclid(period);
    }
    if rise > 0.0 && elapsed < rise {
        return v1 + (v2 - v1) * elapsed / rise;
    }

    let high_start = rise;
    let fall_start = high_start + width;
    if elapsed < fall_start {
        return v2;
    }

    let fall_end = fall_start + fall;
    if fall > 0.0 && elapsed < fall_end {
        return v2 + (v1 - v2) * (elapsed - fall_start) / fall;
    }

    v1
}

fn spice_sin_from_args(time: Value, args: &[Value]) -> Value {
    let offset = args[0];
    let amplitude = args[1];
    let frequency = args[2];
    let delay = args.get(3).copied().unwrap_or(0.0);
    let damping = args.get(4).copied().unwrap_or(0.0);
    let phase = args.get(5).copied().unwrap_or(0.0).to_radians();
    let elapsed = time - delay;

    if elapsed <= 0.0 {
        offset + amplitude * phase.sin()
    } else {
        offset
            + amplitude * (-damping * elapsed).exp() * (TWO_PI * frequency * elapsed + phase).sin()
    }
}

fn spice_exp_from_args(time: Value, args: &[Value]) -> Value {
    let v1 = args[0];
    let v2 = args[1];
    let td1 = args.get(2).copied().unwrap_or(0.0);
    let tau1 = args.get(3).copied().unwrap_or(0.0);
    let td2 = args.get(4).copied().unwrap_or(td1);
    let tau2 = args.get(5).copied().unwrap_or(0.0);

    if time <= td1 {
        v1
    } else if time <= td2 {
        v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
    } else {
        v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
            - (v2 - v1) * (1.0 - (-(time - td2) / tau2).exp())
    }
}

fn spice_sffm_from_args(time: Value, args: &[Value]) -> Value {
    let offset = args[0];
    let amplitude = args[1];
    let carrier_freq = args.get(2).copied().unwrap_or(0.0);
    let modulation_index = args.get(3).copied().unwrap_or(0.0);
    let signal_freq = args.get(4).copied().unwrap_or(0.0);

    offset
        + amplitude
            * (TWO_PI * carrier_freq * time
                + modulation_index * (TWO_PI * signal_freq * time).sin())
            .sin()
}

fn xyce_tanh(value: Value) -> Value {
    if value > XYCE_TANH_SATURATION_THRESHOLD {
        1.0
    } else if value < -XYCE_TANH_SATURATION_THRESHOLD {
        -1.0
    } else {
        value.tanh()
    }
}

fn xyce_atanh(value: Value) -> Value {
    value
        .clamp(XYCE_ATANH_EPSILON - 1.0, 1.0 - XYCE_ATANH_EPSILON)
        .atanh()
}
