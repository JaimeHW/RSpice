//! The compiled bytecode artifact and its instruction set.
//!
//! Defines [`CompiledModel`] — the serializable result of compilation — along
//! with [`Instruction`], the stack-machine opcodes, and the supporting stamp,
//! parameter, and noise-source descriptions.
//!
//! The whole artifact is `Serialize`/`Deserialize`, which is what lets
//! `rspice-core` cache compiled models on disk keyed by source digest instead
//! of recompiling a foundry model on every run.

use super::*;

/// Code generator
pub struct CodeGenerator {
    /// Collected Laplace filters
    pub(super) laplace_filters: std::cell::RefCell<Vec<StateSpaceFilter>>,
    /// Stable logical Laplace site to state-space filter slot.
    pub(super) laplace_sites:
        std::cell::RefCell<std::collections::HashMap<crate::ir::LaplaceSiteId, usize>>,
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
    /// Stable logical slew site to transactional filter slot.
    pub(super) slew_sites:
        std::cell::RefCell<std::collections::HashMap<crate::ir::SlewSiteId, usize>>,
    /// Stateful slot allocator for `cross`.
    pub(super) cross_detector_count: std::cell::Cell<usize>,
    /// Stateful slot allocator for `timer`.
    pub(super) timer_state_count: std::cell::Cell<usize>,
    /// Collected z-domain filters (`zi_*`).
    pub(super) zi_filters: std::cell::RefCell<Vec<crate::zfilter::ZiFilter>>,
    pub(super) zi_filter_definitions: std::cell::RefCell<Vec<CompiledZiFilterDefinition>>,
    pub(super) zi_sites: std::cell::RefCell<std::collections::HashMap<crate::ir::ZiSiteId, usize>>,
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
    /// Sorted, duplicate-free variable slots written by event-controlled
    /// procedural bodies. Runtime instances use this metadata to isolate
    /// speculative Newton evaluations from accepted-point state.
    #[serde(default)]
    pub event_state_variables: Vec<usize>,
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
    /// Per-site definition programs retained in the compiled artifact for
    /// validation and backend planning. Executable value/derivative programs
    /// also carry flattened operands so each site can freeze lazily at its
    /// first correctly ordered evaluation.
    #[serde(default)]
    pub zi_filter_definitions: Vec<CompiledZiFilterDefinition>,
    /// Small-signal noise sources extracted from contributions
    pub noise_sources: Vec<CompiledNoiseSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompiledZiPolynomial {
    Coefficients(Vec<BytecodeProgram>),
    Roots(Vec<(BytecodeProgram, BytecodeProgram)>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledZiFilterDefinition {
    pub numerator: CompiledZiPolynomial,
    pub denominator: CompiledZiPolynomial,
    pub period: BytecodeProgram,
    pub first_transition: BytecodeProgram,
}

/// Stack layout of one Zi polynomial's lazy-freeze operands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZiPolynomialLayout {
    Coefficients { len: usize },
    Roots { len: usize },
}

impl ZiPolynomialLayout {
    pub fn checked_value_count(self) -> Option<usize> {
        match self {
            Self::Coefficients { len } => Some(len),
            Self::Roots { len } => len.checked_mul(2),
        }
    }

    pub fn value_count(self) -> usize {
        self.checked_value_count().unwrap_or(usize::MAX)
    }

    pub fn definition_len(self) -> usize {
        match self {
            Self::Coefficients { len } | Self::Roots { len } => len,
        }
    }

    pub fn is_roots(self) -> bool {
        matches!(self, Self::Roots { .. })
    }
}

/// Runtime metadata shared by VM and native Zi helpers. Operand order is
/// numerator values, denominator values, period, first-transition time,
/// input/action, then dynamic transition time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZiRuntimeLayout {
    pub filter_id: usize,
    pub numerator: ZiPolynomialLayout,
    pub denominator: ZiPolynomialLayout,
    pub direct_assignment: bool,
}

impl ZiRuntimeLayout {
    pub const fn unit_coefficients(filter_id: usize) -> Self {
        Self {
            filter_id,
            numerator: ZiPolynomialLayout::Coefficients { len: 1 },
            denominator: ZiPolynomialLayout::Coefficients { len: 1 },
            direct_assignment: false,
        }
    }

    pub fn checked_operand_count(self) -> Option<usize> {
        self.numerator
            .checked_value_count()?
            .checked_add(self.denominator.checked_value_count()?)?
            .checked_add(crate::zfilter::ZI_FIXED_RUNTIME_OPERANDS)
    }

    pub fn validate_operand_budget(self) -> Result<usize, crate::zfilter::ZiFilterError> {
        let numerator = self.numerator.checked_value_count().ok_or_else(|| {
            crate::zfilter::ZiFilterError::InvalidDefinition(
                "Zi numerator root scalar count overflows usize".into(),
            )
        })?;
        let denominator = self.denominator.checked_value_count().ok_or_else(|| {
            crate::zfilter::ZiFilterError::InvalidDefinition(
                "Zi denominator root scalar count overflows usize".into(),
            )
        })?;
        crate::zfilter::validate_zi_runtime_operand_budget("Zi filter", numerator, denominator)
    }

    pub fn operand_count(self) -> usize {
        self.checked_operand_count().unwrap_or(usize::MAX)
    }

    /// Pack the layout into the native helper's third machine-word argument.
    /// Native execution is 64-bit; limits are checked instead of truncating.
    pub fn native_descriptor(self) -> Option<usize> {
        const FIELD_MASK: usize = (1 << 14) - 1;
        self.validate_operand_budget().ok()?;
        if usize::BITS < 64
            || self.filter_id > u32::MAX as usize
            || self.numerator.definition_len() > FIELD_MASK
            || self.denominator.definition_len() > FIELD_MASK
        {
            return None;
        }
        let mut packed = self.filter_id;
        packed |= self.numerator.definition_len() << 32;
        packed |= usize::from(self.numerator.is_roots()) << 46;
        packed |= self.denominator.definition_len() << 47;
        packed |= usize::from(self.denominator.is_roots()) << 61;
        packed |= usize::from(self.direct_assignment) << 62;
        Some(packed)
    }

    pub fn from_native_descriptor(packed: usize) -> Option<Self> {
        if usize::BITS < 64 || packed >> 63 != 0 {
            return None;
        }
        let numerator_len = (packed >> 32) & ((1 << 14) - 1);
        let denominator_len = (packed >> 47) & ((1 << 14) - 1);
        let layout = Self {
            filter_id: packed & u32::MAX as usize,
            numerator: if (packed >> 46) & 1 == 0 {
                ZiPolynomialLayout::Coefficients { len: numerator_len }
            } else {
                ZiPolynomialLayout::Roots { len: numerator_len }
            },
            denominator: if (packed >> 61) & 1 == 0 {
                ZiPolynomialLayout::Coefficients {
                    len: denominator_len,
                }
            } else {
                ZiPolynomialLayout::Roots {
                    len: denominator_len,
                }
            },
            direct_assignment: (packed >> 62) & 1 != 0,
        };
        layout.validate_operand_budget().ok()?;
        Some(layout)
    }

    pub fn freeze_filter(
        self,
        operands: &[f64],
    ) -> Result<crate::zfilter::ZiFilter, crate::zfilter::ZiFilterError> {
        let operand_count = self.validate_operand_budget()?;
        if operands.len() != operand_count {
            return Err(crate::zfilter::ZiFilterError::InvalidDefinition(format!(
                "lazy Zi definition expected {} operands, got {}",
                operand_count,
                operands.len()
            )));
        }
        let numerator_values = self.numerator.checked_value_count().ok_or_else(|| {
            crate::zfilter::ZiFilterError::InvalidDefinition(
                "Zi numerator root scalar count overflows usize".into(),
            )
        })?;
        let denominator_values = self.denominator.checked_value_count().ok_or_else(|| {
            crate::zfilter::ZiFilterError::InvalidDefinition(
                "Zi denominator root scalar count overflows usize".into(),
            )
        })?;
        let numerator = freeze_zi_polynomial(self.numerator, &operands[..numerator_values])?;
        let denominator = freeze_zi_polynomial(
            self.denominator,
            &operands[numerator_values..numerator_values + denominator_values],
        )?;
        let period_index = numerator_values + denominator_values;
        let period = operands[period_index];
        let first_transition = operands[period_index + 1];
        crate::zfilter::ZiFilter::new_with_timing(numerator, denominator, period, first_transition)
    }
}

fn freeze_zi_polynomial(
    layout: ZiPolynomialLayout,
    values: &[f64],
) -> Result<Vec<f64>, crate::zfilter::ZiFilterError> {
    match layout {
        ZiPolynomialLayout::Coefficients { len } => {
            if values.len() != len {
                return Err(crate::zfilter::ZiFilterError::InvalidDefinition(format!(
                    "Zi coefficient definition expected {len} values, got {}",
                    values.len()
                )));
            }
            Ok(values.to_vec())
        }
        ZiPolynomialLayout::Roots { len } => {
            let scalar_count = len.checked_mul(2).ok_or_else(|| {
                crate::zfilter::ZiFilterError::InvalidDefinition(
                    "Zi root scalar count overflows usize".into(),
                )
            })?;
            if values.len() != scalar_count {
                return Err(crate::zfilter::ZiFilterError::InvalidDefinition(format!(
                    "Zi root definition expected {} scalar values, got {}",
                    scalar_count,
                    values.len()
                )));
            }
            crate::zfilter::z_roots_to_coefficients(
                &values
                    .chunks_exact(2)
                    .map(|pair| (pair[0], pair[1]))
                    .collect::<Vec<_>>(),
            )
            .map_err(crate::zfilter::ZiFilterError::InvalidDefinition)
        }
    }
}

#[cfg(test)]
mod zi_runtime_layout_tests {
    use super::{ZiPolynomialLayout, ZiRuntimeLayout};

    #[test]
    fn shared_operand_budget_accepts_boundary_and_rejects_one_over() {
        let at_limit = ZiRuntimeLayout {
            filter_id: 0,
            numerator: ZiPolynomialLayout::Coefficients { len: 1019 },
            denominator: ZiPolynomialLayout::Coefficients { len: 1 },
            direct_assignment: false,
        };
        assert_eq!(
            at_limit.validate_operand_budget().unwrap(),
            crate::zfilter::MAX_ZI_RUNTIME_OPERANDS
        );
        assert!(at_limit.native_descriptor().is_some());

        let over_limit = ZiRuntimeLayout {
            numerator: ZiPolynomialLayout::Coefficients { len: 1020 },
            ..at_limit
        };
        let error = over_limit.validate_operand_budget().unwrap_err();
        assert!(error.to_string().contains("platform-uniform maximum 1024"));
        assert!(over_limit.native_descriptor().is_none());
        assert!(over_limit.freeze_filter(&[]).is_err());
    }

    #[test]
    fn mixed_roots_and_coefficients_use_scalar_slots_and_overflow_fails_closed() {
        let mixed = ZiRuntimeLayout {
            filter_id: 0,
            numerator: ZiPolynomialLayout::Roots { len: 509 },
            denominator: ZiPolynomialLayout::Coefficients { len: 2 },
            direct_assignment: false,
        };
        assert_eq!(mixed.validate_operand_budget().unwrap(), 1024);

        let overflow = ZiRuntimeLayout {
            numerator: ZiPolynomialLayout::Roots { len: usize::MAX },
            ..mixed
        };
        assert!(overflow.checked_operand_count().is_none());
        assert!(overflow.validate_operand_budget().is_err());
        assert!(overflow.native_descriptor().is_none());

        let packed_over_limit = (1020_usize << 32) | (1_usize << 47);
        assert!(ZiRuntimeLayout::from_native_descriptor(packed_over_limit).is_none());
    }
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
    /// Exposed through the selected module's instance-facing ABI. Hidden
    /// hierarchy slots still participate in default and range evaluation.
    #[serde(default = "default_true")]
    pub is_public: bool,
    /// Alternate instance-facing names (aliasparam); setting an alias
    /// writes this parameter
    pub aliases: Vec<SmolStr>,
    pub default: f64,
    /// Program computing the default from other parameters (evaluated in
    /// declaration order for parameters the instance did not set)
    pub default_program: Option<BytecodeProgram>,
    /// Whether the declared parameter type is `integer`.
    #[serde(default)]
    pub is_integer: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// Runtime parameter index supplying the lower bound.
    #[serde(default)]
    pub min_parameter: Option<usize>,
    /// Runtime parameter index supplying the upper bound.
    #[serde(default)]
    pub max_parameter: Option<usize>,
    /// Program computing a parameter-dependent lower bound.
    #[serde(default)]
    pub min_program: Option<BytecodeProgram>,
    /// Program computing a parameter-dependent upper bound.
    #[serde(default)]
    pub max_program: Option<BytecodeProgram>,
    /// Whether equality with the lower bound is forbidden.
    #[serde(default)]
    pub min_exclusive: bool,
    /// Whether equality with the upper bound is forbidden.
    #[serde(default)]
    pub max_exclusive: bool,
    /// Explicitly excluded values.
    #[serde(default)]
    pub exclude: Vec<f64>,
    /// Runtime parameter indices whose values are explicitly excluded.
    #[serde(default)]
    pub exclude_parameters: Vec<usize>,
    /// Programs computing parameter-dependent excluded values.
    #[serde(default)]
    pub exclude_programs: Vec<BytecodeProgram>,
}

const fn default_true() -> bool {
    true
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
    /// Z-domain filter: lazily freeze its leading definition operands, then
    /// pop transition time and input and push sampled output.
    ZiState(ZiRuntimeLayout),
    /// Read-only exact Jacobian view of a Zi site. It never writes the site's
    /// sample candidate or accepted state.
    ZiStateDerivative(ZiRuntimeLayout),
    /// Binary operations
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    /// Modulus (fmod semantics on reals, LRM 4.2.3)
    Mod,
    /// Signed-32-bit bitwise/logical-shift operations. Runtime conversion
    /// rounds to nearest with half cases away from zero.
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
    Asinh,
    Acosh,
    Atanh,
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
    /// Stack: `[expr] -> [d(expr)/dt]`
    DdtState(usize),
    /// State-based integration: idt(expr, ic) using state index
    /// Backward Euler: prev + expr * dt; returns ic at DC
    /// Stack: `[expr, ic] -> [integral]`
    IdtState(usize),
    /// Wrapped integration: idtmod(expr, ic, modulus, offset)
    /// The integral folds into [offset, offset + modulus)
    /// Stack: `[expr, ic, modulus, offset] -> [wrapped integral]`
    IdtModState(usize),
    /// Companion Jacobian factor for ddt: top-of-stack / dt (0 at DC)
    DdtJacobian,
    /// Companion Jacobian factor for idt: top-of-stack * dt (0 at DC)
    IdtJacobian,
    /// Slope of lookup table at the input point
    /// Stack: `[input] -> [dy/dx]`
    TableDerivative(usize),
    /// $limit function: bounds value change per iteration for convergence
    /// Uses state index to track previous value
    LimitState(usize),
    /// Named-limiter state-slot metadata for canonical native compilation.
    /// This instruction is deliberately non-executable.
    CanonicalLimitState(usize),
    /// Lookup table interpolation: uses table_id to reference stored table
    /// Pops input value from stack, pushes interpolated result
    TableLookup(usize),
    /// Absolute delay: absdelay(expr, delay_time)
    /// Uses a circular buffer indexed by delay_id
    /// Stack: `[expr, delay_time] -> [delayed_value]`
    AbsDelayState(usize),
    /// Transition filter: piecewise-linear smoothing
    /// Stack: `[expr, delay, rise_time, fall_time] -> [filtered_value]`
    TransitionState(usize),
    /// Slew rate limiter
    /// Stack: `[expr, max_pos_slew, max_neg_slew] -> [limited_value]`
    SlewState(usize),
    /// Cross (threshold crossing detection)
    /// Stack: `[expr, direction, time_tol, expr_tol, enable] -> [0 or 1]`
    CrossState(usize),
    /// Last zero-crossing time with linear interpolation.
    /// Stack: `[expr, direction] -> [time or -1]`
    LastCrossingState(usize),
    /// White noise source (returns 0 in time domain, contributes to noise analysis)
    /// Stack: `[power] -> [0]`
    WhiteNoise,
    /// Flicker noise source (1/f noise)
    /// Stack: `[power, exponent] -> [0]`
    FlickerNoise,
    /// Analysis check: returns 1 if analysis matches, else 0
    /// Parameter: analysis type ID (0=dc, 1=ac, 2=tran, etc.)
    Analysis(u8),
    /// Above event: initial-positive and rising zero-crossing detection
    /// Stack: `[expr, time_tol, expr_tol, enable] -> [0 or 1]`
    AboveState(usize),
    /// Timer event: one-shot or periodic time-based trigger
    /// Stack: `[start_time, period, time_tol, enable] -> [0 or 1]`
    TimerState(usize),
    /// Laplace filter with poles/zeros (state-space form)
    /// Stack: `[input] -> [filtered]`
    LaplaceState(usize),
    /// Conditional: if top is nonzero, use second, else third
    IfElse,
    /// Read-only exact Jacobian action of a Laplace site.
    /// Stack: `[input derivative] -> [filtered derivative]`
    ///
    /// Appended to preserve the serialized discriminants of all preceding
    /// bytecode instructions.
    LaplaceStateDerivative(usize),
    /// Read-only exact Jacobian action of one `slew` candidate.
    /// Stack: `[input, d_input, pos_rate, d_pos_rate, neg_rate, d_neg_rate]`
    /// -> `[d_output]`.
    ///
    /// Appended to preserve every preceding serialized discriminant.
    SlewStateDerivative(usize),
}
