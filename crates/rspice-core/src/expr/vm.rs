//! Stack-based bytecode VM for expression evaluation
//!
//! Fast execution of compiled expressions without parsing overhead.

use crate::Value;
use std::collections::HashMap;

/// Bytecode instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Push constant onto stack
    PushConst(Value),
    /// Push time variable
    PushTime,
    /// Push frequency variable
    PushFreq,
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
    U2,
    Eq0,
    Ne0,
    Gt0,
    Lt0,
    Ge0,
    Le0,
    Table(usize),
    Pwl(usize),
    Mod,

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
}

impl<'a> Context<'a> {
    /// Create a new context for DC analysis
    pub fn dc(voltages: &'a [Value], currents: &'a [Value]) -> Self {
        Self {
            voltages,
            currents,
            time: 0.0,
            frequency: 0.0,
        }
    }

    /// Create a new context for transient analysis
    pub fn transient(voltages: &'a [Value], currents: &'a [Value], time: Value) -> Self {
        Self {
            voltages,
            currents,
            time,
            frequency: 0.0,
        }
    }
}

/// Bytecode virtual machine
#[derive(Debug)]
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
                Instruction::Eq => {
                    self.binary_op(|a, b| if (a - b).abs() < 1e-12 { 1.0 } else { 0.0 })
                }
                Instruction::Ne => {
                    self.binary_op(|a, b| if (a - b).abs() >= 1e-12 { 1.0 } else { 0.0 })
                }

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
                Instruction::Log => self.unary_op(|a| a.max(1e-38).ln()),
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
                Instruction::Tanh => self.unary_op(|a| a.tanh()),
                Instruction::Asinh => self.unary_op(|a| a.asinh()),
                Instruction::Acosh => self.unary_op(|a| a.acosh()),
                Instruction::Atanh => self.unary_op(|a| a.atanh()),
                Instruction::Floor => self.unary_op(|a| a.floor()),
                Instruction::Ceil => self.unary_op(|a| a.ceil()),
                Instruction::Round => self.unary_op(|a| a.round_ties_even()),
                Instruction::Sqr => self.unary_op(|a| a * a),
                Instruction::Sign => self.unary_op(|a| {
                    if a > 0.0 {
                        1.0
                    } else if a < 0.0 {
                        -1.0
                    } else {
                        0.0
                    }
                }),
                Instruction::Uramp => self.unary_op(|a| a.max(0.0)),
                Instruction::Stp => self.unary_op(|a| {
                    if a > 0.0 {
                        1.0
                    } else if a == 0.0 {
                        0.5
                    } else {
                        0.0
                    }
                }),
                Instruction::U2 => self.unary_op(|a| a.clamp(0.0, 1.0)),
                Instruction::Eq0 => self.unary_op(|a| if a.abs() < 1e-12 { 1.0 } else { 0.0 }),
                Instruction::Ne0 => self.unary_op(|a| if a.abs() >= 1e-12 { 1.0 } else { 0.0 }),
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
                Instruction::Table(arg_count) | Instruction::Pwl(arg_count) => {
                    if *arg_count >= 3 && self.stack.len() >= *arg_count {
                        let start = self.stack.len() - *arg_count;
                        let args = &self.stack[start..];
                        let x = args[0];
                        let result = table_interpolate_from_args(x, &args[1..]);
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
        return interpolate_segment(x, first_x, first_y, args[2], args[3], first_y);
    }

    let last_idx = 2 * (pair_count - 1);
    let last_x = args[last_idx];
    let last_y = args[last_idx + 1];
    if x >= last_x {
        let prev_idx = 2 * (pair_count - 2);
        return interpolate_segment(
            x,
            args[prev_idx],
            args[prev_idx + 1],
            last_x,
            last_y,
            last_y,
        );
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

#[inline]
fn interpolate_segment(x: Value, x1: Value, y1: Value, x2: Value, y2: Value, flat: Value) -> Value {
    if (x2 - x1).abs() < 1e-12 {
        return flat;
    }
    let t = (x - x1) / (x2 - x1);
    y1 + t * (y2 - y1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_arithmetic() {
        let mut program = CompiledExpr::new();
        // 2 + 3 * 4 = 14
        program.instructions = vec![
            Instruction::PushConst(2.0),
            Instruction::PushConst(3.0),
            Instruction::PushConst(4.0),
            Instruction::Mul,
            Instruction::Add,
        ];

        let mut vm = Vm::new();
        let ctx = Context::dc(&[], &[]);
        let result = vm.execute(&program, &ctx);
        assert!((result - 14.0).abs() < 1e-10);
    }

    #[test]
    fn test_vm_voltage_reference() {
        let mut program = CompiledExpr::new();
        let node_idx = program.get_or_create_node("2");

        // V(2) * 2
        program.instructions = vec![
            Instruction::LoadVoltage(node_idx),
            Instruction::PushConst(2.0),
            Instruction::Mul,
        ];

        let voltages_unused = [0.0, 5.0]; // Node 2 is at index 1 (assuming mapping)
        let mut vm = Vm::new();
        let _ctx = Context::dc(&voltages_unused, &[]);

        // Since our mapping puts "2" -> 0, we need voltage at index 0
        let voltages = [5.0];
        let ctx = Context::dc(&voltages, &[]);
        let result = vm.execute(&program, &ctx);
        assert!((result - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_vm_round_uses_ties_even() {
        let mut program = CompiledExpr::new();
        program.instructions = vec![Instruction::PushConst(2.5), Instruction::Round];
        let mut vm = Vm::new();
        let result = vm.execute(&program, &Context::dc(&[], &[]));
        assert!((result - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_vm_stp_and_u2_semantics() {
        let mut stp_program = CompiledExpr::new();
        stp_program.instructions = vec![Instruction::PushConst(0.0), Instruction::Stp];
        let mut vm = Vm::new();
        let stp_result = vm.execute(&stp_program, &Context::dc(&[], &[]));
        assert!((stp_result - 0.5).abs() < 1e-12);

        let mut u2_program = CompiledExpr::new();
        u2_program.instructions = vec![Instruction::PushConst(1.25), Instruction::U2];
        let u2_result = vm.execute(&u2_program, &Context::dc(&[], &[]));
        assert!((u2_result - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_vm_table_interpolation() {
        let mut program = CompiledExpr::new();
        // TABLE(0.5, 0,0, 1,10, 2,20) -> 5
        program.instructions = vec![
            Instruction::PushConst(0.5),
            Instruction::PushConst(0.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(10.0),
            Instruction::PushConst(2.0),
            Instruction::PushConst(20.0),
            Instruction::Table(7),
        ];

        let mut vm = Vm::new();
        let result = vm.execute(&program, &Context::dc(&[], &[]));
        assert!((result - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_vm_table_extrapolation_uses_end_segments() {
        let mut low_program = CompiledExpr::new();
        // TABLE(-1, 0,0, 1,10) -> -10
        low_program.instructions = vec![
            Instruction::PushConst(-1.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(10.0),
            Instruction::Table(5),
        ];
        let mut vm = Vm::new();
        let low_result = vm.execute(&low_program, &Context::dc(&[], &[]));
        assert!((low_result - (-10.0)).abs() < 1e-12);

        let mut high_program = CompiledExpr::new();
        // TABLE(3, 0,0, 1,10, 2,20) -> 30
        high_program.instructions = vec![
            Instruction::PushConst(3.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(1.0),
            Instruction::PushConst(10.0),
            Instruction::PushConst(2.0),
            Instruction::PushConst(20.0),
            Instruction::Table(7),
        ];
        let high_result = vm.execute(&high_program, &Context::dc(&[], &[]));
        assert!((high_result - 30.0).abs() < 1e-12);
    }

    #[test]
    fn test_vm_table_duplicate_x_segment_is_flat() {
        let mut program = CompiledExpr::new();
        // duplicate x points should avoid divide-by-zero and return first y
        program.instructions = vec![
            Instruction::PushConst(0.5),
            Instruction::PushConst(0.0),
            Instruction::PushConst(2.0),
            Instruction::PushConst(0.0),
            Instruction::PushConst(8.0),
            Instruction::Table(5),
        ];
        let mut vm = Vm::new();
        let result = vm.execute(&program, &Context::dc(&[], &[]));
        assert!((result - 2.0).abs() < 1e-12);
    }
}
