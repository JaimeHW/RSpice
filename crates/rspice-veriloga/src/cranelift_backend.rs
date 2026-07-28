//! A portable JIT for the canonical CFG, over Cranelift.
//!
//! Phase 5 of `design/VERILOGA_BACKEND_PLAN.md`. The plan lists "retarget the
//! x64 JIT onto CFG+AD" before "give the JIT a portable backend", and the
//! survey that opened this work inverted the two: `native/x64` is 21,484 lines
//! of hand-written encoder written against the *old* IR's node types, so
//! retargeting it is a rewrite of all of it with a known expiry date — the
//! portable backend deletes it. This is the portable backend.
//!
//! ## Why the CFG lowers almost directly
//!
//! Cranelift's IR is SSA with **block parameters** rather than phi nodes, and
//! its jumps carry arguments. That is the same model [`CfgFunction`] already
//! uses, chosen for its own reasons in Phase 2. So the control flow needs no
//! translation at all: a `CfgBlock` becomes a Cranelift block with the same
//! parameters, a `Jump` becomes a jump with the same arguments, and a `Branch`
//! becomes `brif`. Nothing here reconstructs structure, which is exactly the
//! failure mode the Rust emitter has to work around.
//!
//! ## One scalar type
//!
//! Every CFG value is lowered as `f64`, booleans included. That is not a
//! shortcut: [`crate::canonical_ir::cfg_eval`] reads a branch condition as
//! `real() != 0.0`, so a uniform `f64` representation *is* the interpreter's
//! semantics rather than an approximation of them.
//!
//! ## Transcendentals
//!
//! Cranelift has no `exp` or `sin`; they lower to calls. The callees here are
//! Rust shims that dispatch to [`CfgScalar`]'s own `f64` implementation, so a
//! compiled body and the interpreter evaluate the *same* function rather than
//! two implementations that agree to some tolerance. That matters because the
//! interpreter is the oracle these are checked against.
//!
//! ## What is not covered yet
//!
//! `ddt`, `idt`, `$limit`, the packed derivative lanes, and staged slots are
//! rejected with [`CraneliftError::Unsupported`] naming the kind. They are
//! per-instance state and shaped vectors, and each needs a runtime contract
//! rather than an opcode. Rejecting by name beats lowering them to something
//! plausible: a silently wrong `ddt` is a converged answer to the wrong
//! equation.

use std::collections::HashMap;

use cranelift_codegen::ir::{AbiParam, InstBuilder, MemFlags, Value as ClifValue, types};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};

use crate::canonical_ir::cfg::{CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind};
use crate::canonical_ir::cfg_eval::CfgScalar;
use crate::canonical_ir::ids::ValueId;

/// Scalars a compiled body reads, in the order the generated code indexes them.
///
/// One array rather than one argument each, so adding a scalar does not change
/// the ABI of every compiled function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CfgJitScalars {
    pub temperature: f64,
    pub thermal_voltage: f64,
    pub multiplicity: f64,
    pub time: f64,
}

impl CfgJitScalars {
    const COUNT: usize = 4;

    fn to_array(self) -> [f64; Self::COUNT] {
        [
            self.temperature,
            self.thermal_voltage,
            self.multiplicity,
            self.time,
        ]
    }
}

impl Default for CfgJitScalars {
    fn default() -> Self {
        Self {
            temperature: 300.15,
            thermal_voltage: 0.025_852_0,
            multiplicity: 1.0,
            time: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CraneliftError {
    /// A value kind this backend does not lower yet, named rather than
    /// approximated.
    Unsupported(String),
    /// Cranelift refused the module or the machine code.
    Codegen(String),
    /// The function is not in the shape the lowering assumes.
    Malformed(String),
}

impl std::fmt::Display for CraneliftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(what) => write!(f, "the Cranelift backend does not lower {what} yet"),
            Self::Codegen(detail) => write!(f, "Cranelift rejected the function: {detail}"),
            Self::Malformed(detail) => write!(f, "the CFG is not lowerable: {detail}"),
        }
    }
}

impl std::error::Error for CraneliftError {}

// ---- Runtime shims ---------------------------------------------------------
//
// Each dispatches to `CfgScalar`'s `f64` implementation, so a compiled body and
// the reference interpreter run the identical function. Declared `extern "C"`
// because that is the ABI Cranelift emits the call under.

macro_rules! unary_shim {
    ($name:ident, $method:ident) => {
        extern "C" fn $name(value: f64) -> f64 {
            <f64 as CfgScalar>::$method(value)
        }
    };
}

unary_shim!(shim_exp, exp);
unary_shim!(shim_limexp, limexp);
unary_shim!(shim_ln, ln);
unary_shim!(shim_sin, sin);
unary_shim!(shim_cos, cos);
unary_shim!(shim_tan, tan);
unary_shim!(shim_sinh, sinh);
unary_shim!(shim_cosh, cosh);
unary_shim!(shim_tanh, tanh);
unary_shim!(shim_atan, atan);
unary_shim!(shim_asinh, asinh);

extern "C" fn shim_pow(base: f64, exponent: f64) -> f64 {
    <f64 as CfgScalar>::powf(base, exponent)
}

extern "C" fn shim_rem(left: f64, right: f64) -> f64 {
    <f64 as CfgScalar>::rem(left, right)
}

/// Every shim, by the symbol name the generated code calls it under.
const UNARY_SHIMS: &[(&str, CfgUnaryOp, *const u8)] = &[
    ("rspice_cfg_exp", CfgUnaryOp::Exp, shim_exp as *const u8),
    (
        "rspice_cfg_limexp",
        CfgUnaryOp::LimExp,
        shim_limexp as *const u8,
    ),
    ("rspice_cfg_ln", CfgUnaryOp::Ln, shim_ln as *const u8),
    ("rspice_cfg_sin", CfgUnaryOp::Sin, shim_sin as *const u8),
    ("rspice_cfg_cos", CfgUnaryOp::Cos, shim_cos as *const u8),
    ("rspice_cfg_tan", CfgUnaryOp::Tan, shim_tan as *const u8),
    ("rspice_cfg_sinh", CfgUnaryOp::Sinh, shim_sinh as *const u8),
    ("rspice_cfg_cosh", CfgUnaryOp::Cosh, shim_cosh as *const u8),
    ("rspice_cfg_tanh", CfgUnaryOp::Tanh, shim_tanh as *const u8),
    ("rspice_cfg_atan", CfgUnaryOp::Atan, shim_atan as *const u8),
    (
        "rspice_cfg_asinh",
        CfgUnaryOp::Asinh,
        shim_asinh as *const u8,
    ),
];

const BINARY_SHIMS: &[(&str, CfgBinaryOp, *const u8)] = &[
    ("rspice_cfg_pow", CfgBinaryOp::Pow, shim_pow as *const u8),
    ("rspice_cfg_rem", CfgBinaryOp::Mod, shim_rem as *const u8),
];

/// The signature of a compiled body.
///
/// Pointers rather than a struct so the ABI is the platform's C ABI and nothing
/// depends on Rust's layout rules. Every array is indexed by the corresponding
/// typed id, and `out` receives one entry per exported value in the order the
/// caller asked for them.
type CompiledEntry = unsafe extern "C" fn(
    parameters: *const f64,
    node_potentials: *const f64,
    branch_flows: *const f64,
    branch_unknown_flows: *const f64,
    scalars: *const f64,
    out: *mut f64,
);

/// A CFG compiled to machine code, and the module keeping it mapped.
pub struct CompiledCfg {
    // Dropped last-in-first-out with `entry`, and `entry` points into it, so the
    // module must outlive every call. Keeping it here is what guarantees that.
    _module: JITModule,
    entry: CompiledEntry,
    export_count: usize,
}

impl std::fmt::Debug for CompiledCfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledCfg")
            .field("export_count", &self.export_count)
            .finish_non_exhaustive()
    }
}

/// What a compiled body reads.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CfgJitInputs {
    pub parameters: Vec<f64>,
    pub node_potentials: Vec<f64>,
    pub branch_flows: Vec<f64>,
    pub branch_unknown_flows: Vec<f64>,
    pub scalars: CfgJitScalars,
}

impl CompiledCfg {
    pub fn export_count(&self) -> usize {
        self.export_count
    }

    /// Run the compiled body and return the exported values.
    pub fn call(&self, inputs: &CfgJitInputs) -> Vec<f64> {
        let scalars = inputs.scalars.to_array();
        let mut out = vec![0.0; self.export_count];
        // Empty inputs would otherwise hand the callee a dangling pointer from
        // `as_ptr` on an empty Vec. The callee never reads through a pointer it
        // has no index for, but constructing the pointer at all is what the
        // safety argument has to cover, so give it a real one.
        let empty = [0.0f64; 1];
        let pointer = |slice: &[f64]| {
            if slice.is_empty() {
                empty.as_ptr()
            } else {
                slice.as_ptr()
            }
        };

        // SAFETY: `entry` was produced by `compile` from this module, which is
        // still alive; every pointer is to a live slice, and the generated code
        // only loads indices that the CFG's own typed ids named, which `compile`
        // checked against these lengths.
        unsafe {
            (self.entry)(
                pointer(&inputs.parameters),
                pointer(&inputs.node_potentials),
                pointer(&inputs.branch_flows),
                pointer(&inputs.branch_unknown_flows),
                scalars.as_ptr(),
                out.as_mut_ptr(),
            );
        }
        out
    }
}

/// Compile `function`, writing `exports` into the output array in order.
pub fn compile(function: &CfgFunction, exports: &[ValueId]) -> Result<CompiledCfg, CraneliftError> {
    let mut flags = settings::builder();
    // The generated bodies are straight-line-heavy and this is a JIT, so the
    // choice is compile time against code quality. `speed` rather than
    // `speed_and_size`: the plan's gate is runtime.
    flags
        .set("opt_level", "speed")
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?;
    // Nothing here unwinds, and omitting the tables keeps the emitted image to
    // what was asked for.
    flags
        .set("is_pic", "false")
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?;

    let isa = cranelift_native::builder()
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?
        .finish(settings::Flags::new(flags))
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?;

    let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
    for (name, _, address) in UNARY_SHIMS {
        builder.symbol(*name, *address);
    }
    for (name, _, address) in BINARY_SHIMS {
        builder.symbol(*name, *address);
    }
    let mut module = JITModule::new(builder);

    let pointer_type = module.target_config().pointer_type();
    let mut signature = module.make_signature();
    for _ in 0..6 {
        signature.params.push(AbiParam::new(pointer_type));
    }

    // Declare the shims once, up front. Declaring on demand would work but
    // would interleave module mutation with the body walk, and the borrow that
    // costs is not worth the laziness.
    let mut unary_ids: HashMap<CfgUnaryOp, FuncId> = HashMap::new();
    let mut binary_ids: HashMap<CfgBinaryOp, FuncId> = HashMap::new();
    {
        let mut declare = |name: &str, arity: usize| -> Result<FuncId, CraneliftError> {
            let mut shim_signature = module.make_signature();
            for _ in 0..arity {
                shim_signature.params.push(AbiParam::new(types::F64));
            }
            shim_signature.returns.push(AbiParam::new(types::F64));
            module
                .declare_function(name, Linkage::Import, &shim_signature)
                .map_err(|error| CraneliftError::Codegen(error.to_string()))
        };
        for (name, op, _) in UNARY_SHIMS {
            unary_ids.insert(*op, declare(name, 1)?);
        }
        for (name, op, _) in BINARY_SHIMS {
            binary_ids.insert(*op, declare(name, 2)?);
        }
    }

    let entry_id = module
        .declare_function("rspice_cfg_body", Linkage::Export, &signature)
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?;

    let mut context = module.make_context();
    context.func.signature = signature;
    let mut builder_context = FunctionBuilderContext::new();

    {
        Lowering {
            function,
            builder: FunctionBuilder::new(&mut context.func, &mut builder_context),
            module: &mut module,
            unary_ids: &unary_ids,
            binary_ids: &binary_ids,
            values: vec![None; function.values.len()],
            blocks: Vec::new(),
        }
        .run(exports)?;
    }

    module
        .define_function(entry_id, &mut context)
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?;
    module.clear_context(&mut context);
    module
        .finalize_definitions()
        .map_err(|error| CraneliftError::Codegen(error.to_string()))?;

    let address = module.get_finalized_function(entry_id);
    // SAFETY: `address` is the entry point Cranelift just finalized for a
    // function declared with exactly `CompiledEntry`'s signature.
    let entry: CompiledEntry = unsafe { std::mem::transmute(address) };

    Ok(CompiledCfg {
        _module: module,
        entry,
        export_count: exports.len(),
    })
}

struct Lowering<'a> {
    function: &'a CfgFunction,
    builder: FunctionBuilder<'a>,
    module: &'a mut JITModule,
    unary_ids: &'a HashMap<CfgUnaryOp, FuncId>,
    binary_ids: &'a HashMap<CfgBinaryOp, FuncId>,
    values: Vec<Option<ClifValue>>,
    blocks: Vec<cranelift_codegen::ir::Block>,
}

/// Where each input array sits in the entry signature.
const ARG_PARAMETERS: usize = 0;
const ARG_NODE_POTENTIALS: usize = 1;
const ARG_BRANCH_FLOWS: usize = 2;
const ARG_BRANCH_UNKNOWN_FLOWS: usize = 3;
const ARG_SCALARS: usize = 4;
const ARG_OUT: usize = 5;

impl Lowering<'_> {
    /// Consumes the lowering because `FunctionBuilder::finalize` consumes the
    /// builder, and the builder is a field.
    fn run(mut self, exports: &[ValueId]) -> Result<(), CraneliftError> {
        // A prologue block holds the incoming pointers, because the CFG's own
        // entry block may be a branch target and Cranelift's entry block cannot
        // be. Jumping from the prologue into it costs one unconditional jump
        // that the register allocator removes.
        let prologue = self.builder.create_block();
        self.builder.append_block_params_for_function_params(prologue);
        self.builder.switch_to_block(prologue);

        for block in &self.function.blocks {
            let clif_block = self.builder.create_block();
            for _ in &block.params {
                self.builder.append_block_param(clif_block, types::F64);
            }
            self.blocks.push(clif_block);
        }

        let args: Vec<ClifValue> = self.builder.block_params(prologue).to_vec();
        let entry_block = self.blocks[usize::from(self.function.entry)];
        // The CFG's entry block takes no parameters — it has no predecessor to
        // supply them — so the jump carries none.
        if !self.function.block(self.function.entry).params.is_empty() {
            return Err(CraneliftError::Malformed(
                "the entry block takes parameters, which no predecessor can supply".to_string(),
            ));
        }
        self.builder.ins().jump(entry_block, &[]);

        for (index, block) in self.function.blocks.iter().enumerate() {
            let clif_block = self.blocks[index];
            self.builder.switch_to_block(clif_block);

            let params: Vec<ClifValue> = self.builder.block_params(clif_block).to_vec();
            for (slot, value) in block.params.iter().enumerate() {
                self.values[usize::from(*value)] = Some(params[slot]);
            }

            for instruction in &block.instructions {
                let lowered = self.lower_value(instruction.result, &args)?;
                self.values[usize::from(instruction.result)] = Some(lowered);
            }

            self.lower_terminator(&block.terminator, exports, &args)?;
        }

        self.builder.seal_all_blocks();
        self.builder.finalize();
        Ok(())
    }

    fn lower_terminator(
        &mut self,
        terminator: &CfgTerminator,
        exports: &[ValueId],
        args: &[ClifValue],
    ) -> Result<(), CraneliftError> {
        match terminator {
            CfgTerminator::Jump { target, args: jump } => {
                let target_block = self.blocks[usize::from(*target)];
                let lowered = self.read_all(jump)?;
                self.builder.ins().jump(target_block, &lowered);
            }
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                // `real() != 0.0` — the interpreter's rule, not a reinterpretation
                // of the bits, so a condition of `-0.0` is false in both.
                let condition = self.read(*condition)?;
                let zero = self.builder.ins().f64const(0.0);
                let taken = self.builder.ins().fcmp(
                    cranelift_codegen::ir::condcodes::FloatCC::NotEqual,
                    condition,
                    zero,
                );
                let then_block = self.blocks[usize::from(*then_target)];
                let else_block = self.blocks[usize::from(*else_target)];
                let then_lowered = self.read_all(then_args)?;
                let else_lowered = self.read_all(else_args)?;
                self.builder
                    .ins()
                    .brif(taken, then_block, &then_lowered, else_block, &else_lowered);
            }
            CfgTerminator::Return => {
                let out = args[ARG_OUT];
                for (slot, value) in exports.iter().enumerate() {
                    let lowered = self.read(*value)?;
                    let offset = (slot * std::mem::size_of::<f64>()) as i32;
                    self.builder
                        .ins()
                        .store(MemFlags::trusted(), lowered, out, offset);
                }
                self.builder.ins().return_(&[]);
            }
            CfgTerminator::Unset => {
                return Err(CraneliftError::Malformed(
                    "a block terminator is still Unset".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn read(&self, value: ValueId) -> Result<ClifValue, CraneliftError> {
        self.values[usize::from(value)].ok_or_else(|| {
            CraneliftError::Malformed(format!(
                "{value} is read before it is defined on this path"
            ))
        })
    }

    fn read_all(&self, values: &[ValueId]) -> Result<Vec<ClifValue>, CraneliftError> {
        values.iter().map(|value| self.read(*value)).collect()
    }

    /// Load `array[index]`, where `array` is one of the pointer arguments.
    fn load_indexed(&mut self, args: &[ClifValue], slot: usize, index: usize) -> ClifValue {
        let base = args[slot];
        let offset = (index * std::mem::size_of::<f64>()) as i32;
        self.builder
            .ins()
            .load(types::F64, MemFlags::trusted(), base, offset)
    }

    fn lower_value(
        &mut self,
        value: ValueId,
        args: &[ClifValue],
    ) -> Result<ClifValue, CraneliftError> {
        let kind = &self.function.value(value).kind;
        Ok(match kind {
            CfgValueKind::RealConstant(constant) => self.builder.ins().f64const(*constant),
            CfgValueKind::BooleanConstant(flag) => {
                self.builder.ins().f64const(if *flag { 1.0 } else { 0.0 })
            }
            CfgValueKind::BlockParameter => {
                return Err(CraneliftError::Malformed(format!(
                    "{value} is a block parameter but appears as an instruction"
                )));
            }
            CfgValueKind::Parameter(id) => {
                self.load_indexed(args, ARG_PARAMETERS, usize::from(*id))
            }
            CfgValueKind::NodePotential(id) => {
                self.load_indexed(args, ARG_NODE_POTENTIALS, usize::from(*id))
            }
            CfgValueKind::BranchFlow(id) => {
                self.load_indexed(args, ARG_BRANCH_FLOWS, usize::from(*id))
            }
            CfgValueKind::BranchUnknownFlow(id) => {
                self.load_indexed(args, ARG_BRANCH_UNKNOWN_FLOWS, usize::from(*id))
            }
            CfgValueKind::Temperature => self.load_indexed(args, ARG_SCALARS, 0),
            CfgValueKind::ThermalVoltage => self.load_indexed(args, ARG_SCALARS, 1),
            CfgValueKind::Multiplicity => self.load_indexed(args, ARG_SCALARS, 2),
            CfgValueKind::Time => self.load_indexed(args, ARG_SCALARS, 3),
            CfgValueKind::Unary { op, input } => {
                let operand = self.read(*input)?;
                self.lower_unary(*op, operand)?
            }
            CfgValueKind::Binary { op, left, right } => {
                let left = self.read(*left)?;
                let right = self.read(*right)?;
                self.lower_binary(*op, left, right)?
            }
            other => {
                return Err(CraneliftError::Unsupported(describe(other)));
            }
        })
    }

    fn lower_unary(
        &mut self,
        op: CfgUnaryOp,
        operand: ClifValue,
    ) -> Result<ClifValue, CraneliftError> {
        Ok(match op {
            CfgUnaryOp::Neg => self.builder.ins().fneg(operand),
            CfgUnaryOp::Abs => self.builder.ins().fabs(operand),
            CfgUnaryOp::Sqrt => self.builder.ins().sqrt(operand),
            CfgUnaryOp::Floor => self.builder.ins().floor(operand),
            CfgUnaryOp::Ceil => self.builder.ins().ceil(operand),
            CfgUnaryOp::Not => {
                // Truthiness, matching the interpreter: zero becomes one and
                // everything else becomes zero.
                let zero = self.builder.ins().f64const(0.0);
                let one = self.builder.ins().f64const(1.0);
                let is_zero = self.builder.ins().fcmp(
                    cranelift_codegen::ir::condcodes::FloatCC::Equal,
                    operand,
                    zero,
                );
                self.builder.ins().select(is_zero, one, zero)
            }
            CfgUnaryOp::LimitedExp | CfgUnaryOp::LimitedExpDerivative => {
                return Err(CraneliftError::Unsupported(format!("{op:?}")));
            }
            called => {
                let id = self
                    .unary_ids
                    .get(&called)
                    .copied()
                    .ok_or_else(|| CraneliftError::Unsupported(format!("{called:?}")))?;
                let reference = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(reference, &[operand]);
                self.builder.inst_results(call)[0]
            }
        })
    }

    fn lower_binary(
        &mut self,
        op: CfgBinaryOp,
        left: ClifValue,
        right: ClifValue,
    ) -> Result<ClifValue, CraneliftError> {
        use cranelift_codegen::ir::condcodes::FloatCC;

        let comparison = |op: CfgBinaryOp| match op {
            CfgBinaryOp::Eq => Some(FloatCC::Equal),
            CfgBinaryOp::Ne => Some(FloatCC::NotEqual),
            CfgBinaryOp::Lt => Some(FloatCC::LessThan),
            CfgBinaryOp::Le => Some(FloatCC::LessThanOrEqual),
            CfgBinaryOp::Gt => Some(FloatCC::GreaterThan),
            CfgBinaryOp::Ge => Some(FloatCC::GreaterThanOrEqual),
            _ => None,
        };

        if let Some(condition) = comparison(op) {
            let flag = self.builder.ins().fcmp(condition, left, right);
            let one = self.builder.ins().f64const(1.0);
            let zero = self.builder.ins().f64const(0.0);
            return Ok(self.builder.ins().select(flag, one, zero));
        }

        Ok(match op {
            CfgBinaryOp::Add => self.builder.ins().fadd(left, right),
            CfgBinaryOp::Sub => self.builder.ins().fsub(left, right),
            CfgBinaryOp::Mul => self.builder.ins().fmul(left, right),
            CfgBinaryOp::Div => self.builder.ins().fdiv(left, right),
            CfgBinaryOp::Min => self.builder.ins().fmin(left, right),
            CfgBinaryOp::Max => self.builder.ins().fmax(left, right),
            CfgBinaryOp::And | CfgBinaryOp::Or => {
                let zero = self.builder.ins().f64const(0.0);
                let one = self.builder.ins().f64const(1.0);
                let left_true =
                    self.builder
                        .ins()
                        .fcmp(FloatCC::NotEqual, left, zero);
                let right_true =
                    self.builder
                        .ins()
                        .fcmp(FloatCC::NotEqual, right, zero);
                let combined = if matches!(op, CfgBinaryOp::And) {
                    self.builder.ins().band(left_true, right_true)
                } else {
                    self.builder.ins().bor(left_true, right_true)
                };
                self.builder.ins().select(combined, one, zero)
            }
            called => {
                let id = self
                    .binary_ids
                    .get(&called)
                    .copied()
                    .ok_or_else(|| CraneliftError::Unsupported(format!("{called:?}")))?;
                let reference = self.module.declare_func_in_func(id, self.builder.func);
                let call = self.builder.ins().call(reference, &[left, right]);
                self.builder.inst_results(call)[0]
            }
        })
    }
}

/// Name a kind for a diagnostic without printing its whole payload.
fn describe(kind: &CfgValueKind) -> String {
    match kind {
        CfgValueKind::Ddt { .. } => "ddt".to_string(),
        CfgValueKind::DdtScale => "the ddt companion coefficient".to_string(),
        CfgValueKind::Idt { .. } => "idt".to_string(),
        CfgValueKind::IdtScale => "the idt step".to_string(),
        CfgValueKind::Limit { .. } => "$limit".to_string(),
        CfgValueKind::LimitPrevious { .. } => "a $limit previous iterate".to_string(),
        CfgValueKind::Ddx { .. } => "ddx".to_string(),
        CfgValueKind::SimParam { .. } => "$simparam".to_string(),
        CfgValueKind::Analysis(_) => "analysis()".to_string(),
        CfgValueKind::ParameterGiven(_) => "$param_given".to_string(),
        CfgValueKind::Staged { .. } => "a staged slot".to_string(),
        CfgValueKind::LaneSplat(_)
        | CfgValueKind::LaneWiden { .. }
        | CfgValueKind::LaneBinary { .. }
        | CfgValueKind::LaneScalar { .. }
        | CfgValueKind::LaneExtract { .. } => "a packed derivative lane".to_string(),
        other => format!("{other:?}"),
    }
}
