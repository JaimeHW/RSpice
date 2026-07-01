//! Intermediate Representation for Verilog-A
//!
//! The IR represents device equations in a form suitable for:
//! 1. Automatic differentiation (Jacobian generation)
//! 2. Code generation for MNA matrix stamping

use crate::ast::{BinaryOp, UnaryOp};
use crate::semantic::AnalyzedModule;
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

/// Compiled device model in IR form
#[derive(Debug, Clone)]
pub struct DeviceIR {
    /// Module name
    pub name: SmolStr,
    /// Terminal/port definitions
    pub terminals: Vec<Terminal>,
    /// Internal node definitions (not in port list)
    pub internal_nodes: Vec<InternalNodeDef>,
    /// Parameter definitions
    pub parameters: Vec<ParamDef>,
    /// Internal variables (state)
    pub variables: Vec<VarDef>,
    /// Variable assignments and runtime loops (in execution order)
    pub assignments: Vec<IrAssignmentItem>,
    /// Array variables (elements are contiguous slots in `variables`)
    pub arrays: Vec<ArrayDef>,
    /// Branch equations
    pub equations: Vec<BranchEquation>,
    /// Branch-current unknowns introduced by potential contributions
    pub branch_unknowns: Vec<BranchUnknownDef>,
    /// Noise sources
    pub noise_sources: Vec<NoiseSourceDef>,
}

/// Terminal (port) definition
#[derive(Debug, Clone)]
pub struct Terminal {
    pub name: SmolStr,
    pub index: usize,
}

/// Internal node definition (not in port list)
#[derive(Debug, Clone)]
pub struct InternalNodeDef {
    pub name: SmolStr,
    pub index: usize,
}

/// Parameter definition
#[derive(Debug, Clone)]
pub struct ParamDef {
    pub name: SmolStr,
    /// Alternate instance-facing names (aliasparam); setting an alias
    /// writes this parameter
    pub aliases: Vec<SmolStr>,
    pub default: f64,
    /// Default expression when it does not fold to a constant (may
    /// reference previously declared parameters)
    pub default_expr: Option<IrExpr>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Variable definition  
#[derive(Debug, Clone)]
pub struct VarDef {
    pub name: SmolStr,
    pub is_state: bool,
}

/// Variable assignment in IR form
#[derive(Debug, Clone)]
pub struct VarAssignment {
    /// Index of variable being assigned (for indexed writes: the array's
    /// first element)
    pub var_index: usize,
    /// Runtime-indexed array element write (None for scalar targets)
    pub index: Option<IndexedTarget>,
    /// The expression to assign
    pub expr: IrExpr,
}

/// Runtime-indexed array write target: the element `index - lower` of the
/// contiguous run starting at the assignment's `var_index`
#[derive(Debug, Clone)]
pub struct IndexedTarget {
    /// Array name (for diagnostics and shadow naming)
    pub array: SmolStr,
    /// Number of elements
    pub len: usize,
    /// Declared lower bound
    pub lower: i64,
    /// Element index expression (evaluated against declared bounds)
    pub index: IrExpr,
}

/// Array variable layout: elements occupy contiguous variable slots
#[derive(Debug, Clone)]
pub struct ArrayDef {
    pub name: SmolStr,
    /// First element's variable index
    pub base: usize,
    /// Declared lower bound
    pub lower: i64,
    /// Number of elements
    pub len: usize,
}

/// An ordered evaluation step: a plain assignment or a runtime-bounded loop
#[derive(Debug, Clone)]
pub enum IrAssignmentItem {
    /// Single variable assignment
    Assign(VarAssignment),
    /// Loop executing its body while the condition evaluates nonzero
    Loop {
        condition: IrExpr,
        body: Vec<IrAssignmentItem>,
    },
}

/// Branch equation: represents I(p,n) <+ f(...) or V(p,n) <+ f(...)
#[derive(Debug, Clone)]
pub struct BranchEquation {
    /// Branch identifier
    pub branch: BranchRef,
    /// Whether this contributes current (true) or voltage (false)
    pub is_current: bool,
    /// Indirect contribution: `expr` is a constraint residual driven to
    /// zero by the branch unknown; the branch row carries f(x) = 0
    /// instead of the V(p)-V(n)-E source relation
    pub indirect: bool,
    /// Potential contributions reference a branch-current unknown
    pub branch_ordinal: Option<usize>,
    /// Instance-static activation condition (parameter-only guard peeled
    /// from the contribution). None = always active.
    pub static_condition: Option<IrExpr>,
    /// The expression tree
    pub expr: IrExpr,
    /// Partial derivatives (Jacobian entries)
    pub derivatives: Vec<Derivative>,
    /// Derivatives of the reactive operand Q (where expr ~ resistive +
    /// ddt(Q)): the small-signal capacitance/inductance entries stamped
    /// as jw * dQ/dx in AC analysis
    pub reactive_derivatives: Vec<Derivative>,
}

/// A branch-current unknown introduced by potential contributions
#[derive(Debug, Clone)]
pub struct BranchUnknownDef {
    /// Positive node (unified index)
    pub pos: usize,
    /// Negative node (unified index)
    pub neg: usize,
    /// Driven by an indirect contribution: the branch row holds the
    /// constraint equation, so the structural V(p)-V(n) row entries must
    /// not be stamped
    pub indirect: bool,
}

/// Branch reference
#[derive(Debug, Clone)]
pub struct BranchRef {
    pub pos_terminal: usize,
    pub neg_terminal: usize,
}

/// Derivative of an expression w.r.t. a variable
#[derive(Debug, Clone)]
pub struct Derivative {
    /// What we're differentiating with respect to
    pub wrt: DerivativeWrt,
    /// The derivative expression
    pub expr: IrExpr,
}

/// What a derivative is with respect to
#[derive(Debug, Clone)]
pub enum DerivativeWrt {
    /// Voltage at a unified node index
    Voltage(usize),
    /// Branch-current unknown (by ordinal)
    BranchCurrent(usize),
}

/// IR Expression tree
#[derive(Debug, Clone)]
pub enum IrExpr {
    /// Constant value
    Const(f64),
    /// Parameter reference
    Param(SmolStr),
    /// Whether a parameter was explicitly set on the instance
    /// ($param_given)
    ParamGiven(SmolStr),
    /// Variable reference
    Var(SmolStr),
    /// Runtime-indexed array element read: element `index - lower` of the
    /// contiguous variable run starting at `base`
    VarIndexed {
        /// Array name (for shadow naming)
        array: SmolStr,
        /// First element's variable index
        base: usize,
        /// Number of elements
        len: usize,
        /// Declared lower bound
        lower: i64,
        /// Element index expression
        index: Box<IrExpr>,
    },
    /// Voltage at terminal pair
    Voltage(usize, usize),
    /// Current through branch
    Current(usize, usize),
    /// Branch-current unknown of a potential contribution (by ordinal)
    BranchCurrent(usize),
    /// Time variable
    Time,
    /// Temperature ($temperature)
    Temperature,
    /// Thermal voltage ($vt)
    Vt,
    /// Instance multiplicity ($mfactor): the number of parallel copies
    /// this instance represents. The simulator scales flow contributions
    /// automatically; reading it supports models that need fine control.
    Mfactor,
    /// Whether an external terminal was connected on this instance.
    PortConnected(usize),
    /// Binary operation
    Binary(BinaryOp, Box<IrExpr>, Box<IrExpr>),
    /// Unary operation
    Unary(UnaryOp, Box<IrExpr>),
    /// Function call
    Call(IrFunction, Vec<IrExpr>),
    /// Time derivative (ddt)
    Ddt(Box<IrExpr>),
    /// Time integral (idt)
    Idt(Box<IrExpr>, Option<Box<IrExpr>>),
    /// Wrapped time integral (idtmod): the integral folds into
    /// [offset, offset + modulus)
    IdtMod {
        expr: Box<IrExpr>,
        ic: Option<Box<IrExpr>>,
        modulus: Box<IrExpr>,
        offset: Option<Box<IrExpr>>,
    },
    /// Limited exponential
    Limexp(Box<IrExpr>),
    /// $limit function for convergence control
    /// Bounds the expression change per Newton iteration
    /// Args: (expression, step_limit)
    Limit(Box<IrExpr>, Option<Box<IrExpr>>),
    /// $table_model lookup table interpolation
    /// Args: (input_expr, table_data) where table_data is (x_values, y_values)
    TableLookup {
        input: Box<IrExpr>,
        x_data: Vec<f64>,
        y_data: Vec<f64>,
    },
    /// absdelay - absolute transport delay
    /// Returns the value of expr delayed by delay_time seconds
    /// Uses a circular buffer for transient analysis
    AbsDelay {
        expr: Box<IrExpr>,
        delay_time: Box<IrExpr>,
    },
    /// transition - piecewise-linear signal smoothing
    /// Args: (expr, delay, rise_time, fall_time)
    /// Smoothly transitions between values over rise/fall times
    Transition {
        expr: Box<IrExpr>,
        delay: Option<Box<IrExpr>>,
        rise_time: Option<Box<IrExpr>>,
        fall_time: Option<Box<IrExpr>>,
    },
    /// slew - slew rate limiting
    /// Args: (expr, max_pos_slew, max_neg_slew)
    /// Limits the rate of change of the signal
    Slew {
        expr: Box<IrExpr>,
        max_pos_slew: Option<Box<IrExpr>>,
        max_neg_slew: Option<Box<IrExpr>>,
    },
    /// cross - threshold crossing detection
    /// Args: (expr, direction, time_tol, expr_tol)
    /// Returns 1 when expr crosses zero, else 0
    Cross {
        expr: Box<IrExpr>,
        direction: Option<i32>, // +1=rising, -1=falling, 0=both
        time_tol: Option<Box<IrExpr>>,
    },
    /// white_noise - white noise source for AC noise analysis
    /// Args: (power, name)
    WhiteNoise {
        power: Box<IrExpr>,
        name: Option<String>,
    },
    /// flicker_noise - 1/f flicker noise source
    /// Args: (power, exponent, name)
    FlickerNoise {
        power: Box<IrExpr>,
        exponent: Box<IrExpr>,
        name: Option<String>,
    },
    /// noise_table / noise_table_log - interpolated PSD over frequency.
    /// Points are (frequency, power) pairs sorted by frequency;
    /// `log_interp` selects log-log interpolation.
    NoiseTable {
        points: Vec<(f64, f64)>,
        log_interp: bool,
        name: Option<String>,
    },
    /// analysis(name) - check current analysis type
    /// Returns 1.0 if running specified analysis, else 0.0
    Analysis(String),
    /// above(expr, threshold, time_tol) - level crossing event
    /// Returns 1 when expr crosses above threshold, else 0
    Above {
        expr: Box<IrExpr>,
        threshold: Box<IrExpr>,
        time_tol: Option<Box<IrExpr>>,
    },
    /// timer(start, period) - periodic time event
    /// Returns 1 at time=start and every period thereafter
    Timer {
        start_time: Box<IrExpr>,
        period: Option<Box<IrExpr>>,
    },
    /// laplace_zp - s-domain filter with poles and zeros
    /// Args: (expr, zeros, poles, k_factor)
    LaplaceZP {
        expr: Box<IrExpr>,
        zeros: Vec<(f64, f64)>, // (real, imag) pairs
        poles: Vec<(f64, f64)>,
        gain: f64,
    },
    /// laplace_nd - s-domain filter with num/den coefficients
    /// Args: (expr, numerator_coeffs, denominator_coeffs)
    LaplaceND {
        expr: Box<IrExpr>,
        numerator: Vec<f64>, // ascending powers of s
        denominator: Vec<f64>,
    },
    /// zi_* - z-domain (sampled-data) filter: the input samples every
    /// `period` seconds and the difference equation output holds between
    /// samples. Coefficients ascend in z⁻¹.
    ZiFilter {
        expr: Box<IrExpr>,
        numerator: Vec<f64>,
        denominator: Vec<f64>,
        period: f64,
    },
    /// ddx(expr, V(node)) / ddx(expr, V(a,b)) - symbolic partial
    /// derivative w.r.t. a node potential or a branch potential
    /// difference. Resolved to an explicit derivative expression during
    /// device IR construction (where assignment chains are known).
    Ddx {
        expr: Box<IrExpr>,
        pos: usize,
        /// For V(a,b) probes the derivative antisymmetrizes over the
        /// pair: (d/dVa - d/dVb)/2
        neg: Option<usize>,
    },
    /// Companion-model Jacobian factor for ddt: operand / dt in transient,
    /// zero at DC (backward Euler)
    DdtCompanion(Box<IrExpr>),
    /// Companion-model Jacobian factor for idt: operand * dt in transient,
    /// zero at DC
    IdtCompanion(Box<IrExpr>),
    /// Slope of a lookup table evaluated at the input point
    TableDerivative {
        input: Box<IrExpr>,
        x_data: Vec<f64>,
        y_data: Vec<f64>,
    },
    /// Conditional
    Conditional(Box<IrExpr>, Box<IrExpr>, Box<IrExpr>),
}

/// Built-in functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrFunction {
    Abs,
    Sqrt,
    Exp,
    LimitedExp,
    Log,
    Log10,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Atan2,
    Floor,
    Ceil,
    Min,
    Max,
    Pow,
}

/// Frequency-interpolated PSD table (noise_table / noise_table_log)
#[derive(Debug, Clone)]
pub struct NoiseTableData {
    /// (frequency, power) points sorted by frequency
    pub points: Vec<(f64, f64)>,
    /// Interpolate in log-log coordinates
    pub log_interp: bool,
}

/// Noise source extracted from a contribution: the deterministic part of
/// the expression stamps as usual, and each `white_noise`/`flicker_noise`
/// term becomes one small-signal source injected at the contribution's
/// branch during noise analysis.
#[derive(Debug, Clone)]
pub struct NoiseSourceDef {
    /// Injection branch (the contribution's node pair)
    pub branch: BranchRef,
    /// Current contribution (true) injects across the nodes; a potential
    /// contribution injects at its branch-equation row (series EMF)
    pub is_current: bool,
    /// Branch ordinal for potential contributions
    pub branch_ordinal: Option<usize>,
    /// Index of the originating equation/stamp program (activation gates
    /// with the program's instance-static condition)
    pub equation_index: usize,
    /// Power spectral density at the operating point, including any
    /// multiplicative amplitude squared (A²/Hz, or V²/Hz for potential
    /// contributions)
    pub psd: IrExpr,
    /// Flicker frequency exponent (None = white): S(f) = psd / f^exp
    pub exponent: Option<IrExpr>,
    /// Frequency-interpolated PSD table; when present, `psd` carries only
    /// the amplitude-squared scale applied to the interpolated value
    pub table: Option<NoiseTableData>,
    /// Source label from the noise function's name argument
    pub name: Option<SmolStr>,
}

impl DeviceIR {
    /// Create a new device IR from an analyzed module
    ///
    /// Converts contributions to branch equations and generates
    /// Jacobian derivatives using automatic differentiation.
    /// Conversion failures are hard errors: silently dropping an equation
    /// would produce a wrong (but plausible-looking) device.
    pub fn from_analyzed(module: &AnalyzedModule) -> crate::error::CompileResult<Self> {
        use crate::expr_converter::{ConversionContext, ExprConverter};

        let mut ir = DeviceIR {
            name: module.name.clone(),
            terminals: Vec::new(),
            internal_nodes: Vec::new(),
            parameters: Vec::new(),
            variables: Vec::new(),
            assignments: Vec::new(),
            arrays: Vec::new(),
            equations: Vec::new(),
            branch_unknowns: Vec::new(),
            noise_sources: Vec::new(),
        };

        // Build terminals from ports
        for (idx, port) in module.ports.iter().enumerate() {
            ir.terminals.push(Terminal {
                name: port.name.clone(),
                index: idx,
            });
        }

        // Build internal nodes from analyzed module
        for node in &module.internal_nodes {
            ir.internal_nodes.push(InternalNodeDef {
                name: node.name.clone(),
                index: node.index,
            });
        }

        // Build parameters
        for param in &module.parameters {
            let (min, max) = param
                .range
                .as_ref()
                .map(|r| (r.min, r.max))
                .unwrap_or((None, None));

            ir.parameters.push(ParamDef {
                name: param.name.clone(),
                aliases: Vec::new(),
                default: param.default.unwrap_or(0.0),
                default_expr: None,
                min,
                max,
            });
        }

        // Attach aliasparam names to their target parameters
        for alias in &module.param_aliases {
            ir.parameters[alias.target]
                .aliases
                .push(alias.alias.clone());
        }

        // Build variables
        for var in &module.variables {
            ir.variables.push(VarDef {
                name: var.name.clone(),
                is_state: var.is_state,
            });
        }

        // Array layouts (element slots are already in `variables`)
        for (name, layout) in &module.arrays {
            ir.arrays.push(ArrayDef {
                name: name.clone(),
                base: layout.base,
                lower: layout.lower,
                len: layout.len,
            });
        }

        // Create conversion context
        let ctx = ConversionContext::from_module(module);
        let converter = ExprConverter::new(&ctx);
        let num_nodes = ctx.num_nodes();

        // Compile non-constant parameter defaults. They may reference
        // previously declared parameters and are evaluated per instance,
        // in declaration order, for parameters not explicitly given.
        for (idx, param) in module.parameters.iter().enumerate() {
            if param.default.is_none()
                && let Some(default_expr) = &param.default_expr
            {
                let converted = converter.convert(default_expr)?;
                if !Self::is_static_param_expr(&converted) {
                    return Err(crate::error::CodeGenError::new(
                        crate::error::CodeGenErrorKind::InvalidExpression(format!(
                            "default of parameter '{}' must depend only on parameters",
                            param.name
                        )),
                    )
                    .into());
                }
                ir.parameters[idx].default_expr = Some(converted);
            }
        }

        // Convert evaluation statements (assignments and runtime loops) to
        // IR, in order
        let mut items = Vec::with_capacity(module.statements.len());
        Self::convert_statements(&module.statements, &converter, &mut items)?;
        ir.assignments = items;

        // Pre-pass over contributions: parse branch refs and register a
        // branch-current unknown per node pair receiving a potential
        // contribution. Pairs are normalized so V(a,b) and V(b,a) share
        // one unknown (the reversed orientation flips the sign).
        let mut parsed_contribs: Vec<BranchRef> = Vec::with_capacity(module.contributions.len());
        // (min,max) node pair -> (ordinal, oriented positive node)
        let mut branch_table: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for contrib in &module.contributions {
            let branch_ref = Self::parse_branch_name(&contrib.branch, &ctx).ok_or_else(|| {
                crate::error::CodeGenError::new(crate::error::CodeGenErrorKind::InvalidExpression(
                    format!("Unknown contribution branch '{}'", contrib.branch),
                ))
            })?;

            // Potential contributions and indirect contributions (either
            // target kind) introduce a branch-current unknown
            if !contrib.is_current || contrib.indirect {
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                let ordinal = match branch_table.get(&key) {
                    Some(&(ordinal, _)) => ordinal,
                    None => {
                        let ordinal = ir.branch_unknowns.len();
                        ir.branch_unknowns.push(BranchUnknownDef {
                            pos: branch_ref.pos_terminal,
                            neg: branch_ref.neg_terminal,
                            indirect: contrib.indirect,
                        });
                        branch_table.insert(key, (ordinal, branch_ref.pos_terminal));
                        ordinal
                    }
                };
                // A branch is either constrained by one indirect equation
                // or driven by (summed) direct potential contributions;
                // mixing them would over-determine the unknown
                let registered_indirect = ir.branch_unknowns[ordinal].indirect;
                if registered_indirect != contrib.indirect
                    || (contrib.indirect && registered_indirect && {
                        // Second indirect contribution on the same pair
                        parsed_contribs.iter().zip(module.contributions.iter()).any(
                            |(prev_ref, prev)| {
                                prev.indirect
                                    && (
                                        prev_ref.pos_terminal.min(prev_ref.neg_terminal),
                                        prev_ref.pos_terminal.max(prev_ref.neg_terminal),
                                    ) == key
                            },
                        )
                    })
                {
                    return Err(crate::error::CodeGenError::new(
                        crate::error::CodeGenErrorKind::InvalidExpression(format!(
                            "branch '{}' is over-determined: a branch carries either one \
                             indirect constraint or direct potential contributions, not both",
                            contrib.branch
                        )),
                    )
                    .into());
                }
            }

            parsed_contribs.push(branch_ref);
        }
        let num_branches = ir.branch_unknowns.len();

        // Current probes I(a,b) of a branch that carries a potential
        // contribution read the branch unknown (exact), not the inferred
        // contribution cache.
        if !branch_table.is_empty() {
            autodiff::rewrite_branch_probes_in_items(&mut ir.assignments, &branch_table);
        }

        // Variables that are fixed per instance (computed purely from
        // parameters) may participate in topology guards
        let static_vars = Self::compute_instance_static_vars(&ir.assignments, &ir.variables);

        // Shadow liveness roots: only variables that contribution
        // expressions (the equation Jacobians chain through them) or
        // ddx() operands read need derivative shadows. Everything else —
        // operating-point reporting variables above all — keeps its
        // primal value but never costs shadow slots or updates.
        let mut shadow_roots: HashSet<SmolStr> = HashSet::new();
        for contrib in &module.contributions {
            let expr = converter.convert(&contrib.expression)?;
            autodiff::collect_var_names(&expr, &mut shadow_roots);
        }
        autodiff::collect_ddx_operand_names(&ir.assignments, &mut shadow_roots);

        // Forward-mode AD over the assignment sequence: build shadow
        // assignments holding each variable's partial derivative w.r.t.
        // every node voltage and branch-current unknown, so equation
        // Jacobians chain through intermediate variables. Shadow updates
        // recurse into loop bodies so loop-carried dependencies
        // differentiate correctly.
        let shadows =
            autodiff::build_shadow_assignments(&mut ir, num_nodes, num_branches, &shadow_roots);

        // Resolve ddx() operators now that the shadow context exists
        autodiff::resolve_ddx_in_items(&mut ir.assignments, &shadows);

        // Convert contributions to equations
        for (contrib, branch_ref) in module.contributions.iter().zip(parsed_contribs) {
            // Convert the expression
            let expr = converter.convert(&contrib.expression)?;
            let expr = autodiff::rewrite_branch_probes(&expr, &branch_table);
            let expr = autodiff::resolve_ddx(&expr, &shadows);

            // Peel instance-static guards (parameter expressions or
            // variables derived purely from parameters): a potential
            // contribution that is mode-disabled must leave the branch
            // open, not short it to zero volts.
            let (static_condition, expr) = Self::peel_static_condition(expr, &static_vars);

            let (branch_ref, expr, branch_ordinal) = if contrib.indirect {
                // Constraint equations are orientation-free (f == g holds
                // whichever way the target was written); the KCL couplings
                // use the unknown's registered orientation
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                let (ordinal, _) = branch_table[&key];
                let unknown = &ir.branch_unknowns[ordinal];
                (
                    BranchRef {
                        pos_terminal: unknown.pos,
                        neg_terminal: unknown.neg,
                    },
                    expr,
                    Some(ordinal),
                )
            } else if contrib.is_current {
                (branch_ref, expr, None)
            } else {
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                let (ordinal, oriented_pos) = branch_table[&key];
                if branch_ref.pos_terminal == oriented_pos {
                    (branch_ref, expr, Some(ordinal))
                } else {
                    // Reversed orientation: V(b,a) <+ E is V(a,b) <+ -E
                    let unknown = &ir.branch_unknowns[ordinal];
                    (
                        BranchRef {
                            pos_terminal: unknown.pos,
                            neg_terminal: unknown.neg,
                        },
                        IrExpr::Unary(UnaryOp::Neg, Box::new(expr)),
                        Some(ordinal),
                    )
                }
            };

            // Generate derivatives for Jacobian (over node voltages and
            // branch-current unknowns)
            let derivatives = Self::generate_derivatives(&expr, num_nodes, num_branches, &shadows);

            // Reactive (charge/flux) derivatives for AC analysis: extract
            // the ddt() operand and differentiate it
            let reactive_derivatives = match Self::extract_charge(&expr) {
                Some(charge) => {
                    Self::generate_derivatives(&charge, num_nodes, num_branches, &shadows)
                }
                None => Vec::new(),
            };

            // Extract small-signal noise sources (white_noise /
            // flicker_noise terms) for noise analysis; they evaluate to
            // zero in the large-signal programs
            let equation_index = ir.equations.len();
            Self::extract_noise_sources(
                &expr,
                &IrExpr::Const(1.0),
                &branch_ref,
                contrib.is_current,
                branch_ordinal,
                equation_index,
                &mut ir.noise_sources,
            )?;

            ir.equations.push(BranchEquation {
                branch: branch_ref,
                is_current: contrib.is_current,
                indirect: contrib.indirect,
                branch_ordinal,
                static_condition,
                expr,
                derivatives,
                reactive_derivatives,
            });
        }

        Ok(ir)
    }

    /// Whether an expression contains a noise function anywhere
    fn contains_noise(expr: &IrExpr) -> bool {
        match expr {
            IrExpr::WhiteNoise { .. } | IrExpr::FlickerNoise { .. } | IrExpr::NoiseTable { .. } => {
                true
            }
            IrExpr::Binary(_, l, r) => Self::contains_noise(l) || Self::contains_noise(r),
            IrExpr::Unary(_, e)
            | IrExpr::Limexp(e)
            | IrExpr::Ddt(e)
            | IrExpr::DdtCompanion(e)
            | IrExpr::IdtCompanion(e)
            | IrExpr::Limit(e, _) => Self::contains_noise(e),
            IrExpr::Idt(e, ic) => {
                Self::contains_noise(e) || ic.as_deref().is_some_and(Self::contains_noise)
            }
            IrExpr::Conditional(c, t, e) => {
                Self::contains_noise(c) || Self::contains_noise(t) || Self::contains_noise(e)
            }
            IrExpr::Call(_, args) => args.iter().any(Self::contains_noise),
            _ => false,
        }
    }

    /// Structurally extract noise sources from a contribution expression:
    /// `expr ~ deterministic + Σ amplitude_i · noise_i(...)`. Each source
    /// records its operating-point PSD as `amplitude² · power`, so scaled
    /// and guarded noise terms (`gain * white_noise(S)`, conditionals)
    /// keep exact small-signal semantics. Noise functions in nonlinear
    /// positions are hard errors — silently mis-shaping a noise spectrum
    /// would be worse than refusing the model.
    #[allow(clippy::too_many_arguments)]
    fn extract_noise_sources(
        expr: &IrExpr,
        amplitude: &IrExpr,
        branch: &BranchRef,
        is_current: bool,
        branch_ordinal: Option<usize>,
        equation_index: usize,
        out: &mut Vec<NoiseSourceDef>,
    ) -> crate::error::CompileResult<()> {
        if !Self::contains_noise(expr) {
            return Ok(());
        }
        let recurse = |e: &IrExpr, amp: &IrExpr, out: &mut Vec<NoiseSourceDef>| {
            Self::extract_noise_sources(
                e,
                amp,
                branch,
                is_current,
                branch_ordinal,
                equation_index,
                out,
            )
        };
        let unsupported = |what: &str| {
            crate::error::CompileError::from(crate::error::CodeGenError::new(
                crate::error::CodeGenErrorKind::UnsupportedFeature(format!(
                    "noise function in a {what} (noise terms must enter contributions \
                     additively, optionally scaled)"
                )),
            ))
        };
        let square = |amp: &IrExpr| {
            IrExpr::Binary(BinaryOp::Mul, Box::new(amp.clone()), Box::new(amp.clone()))
        };

        match expr {
            IrExpr::WhiteNoise { power, name } => {
                out.push(NoiseSourceDef {
                    branch: branch.clone(),
                    is_current,
                    branch_ordinal,
                    equation_index,
                    psd: IrExpr::Binary(BinaryOp::Mul, Box::new(square(amplitude)), power.clone()),
                    exponent: None,
                    table: None,
                    name: name.as_deref().map(SmolStr::from),
                });
                Ok(())
            }
            IrExpr::FlickerNoise {
                power,
                exponent,
                name,
            } => {
                out.push(NoiseSourceDef {
                    branch: branch.clone(),
                    is_current,
                    branch_ordinal,
                    equation_index,
                    psd: IrExpr::Binary(BinaryOp::Mul, Box::new(square(amplitude)), power.clone()),
                    exponent: Some((**exponent).clone()),
                    table: None,
                    name: name.as_deref().map(SmolStr::from),
                });
                Ok(())
            }
            IrExpr::NoiseTable {
                points,
                log_interp,
                name,
            } => {
                out.push(NoiseSourceDef {
                    branch: branch.clone(),
                    is_current,
                    branch_ordinal,
                    equation_index,
                    // The interpolated table value picks up the
                    // amplitude-squared scale at evaluation time
                    psd: square(amplitude),
                    exponent: None,
                    table: Some(NoiseTableData {
                        points: points.clone(),
                        log_interp: *log_interp,
                    }),
                    name: name.as_deref().map(SmolStr::from),
                });
                Ok(())
            }
            IrExpr::Binary(BinaryOp::Add | BinaryOp::Sub, l, r) => {
                // Sign flips square away
                recurse(l, amplitude, out)?;
                recurse(r, amplitude, out)
            }
            IrExpr::Binary(BinaryOp::Mul, l, r) => {
                match (Self::contains_noise(l), Self::contains_noise(r)) {
                    (true, true) => Err(unsupported("product of noise terms")),
                    (true, false) => {
                        let amp =
                            IrExpr::Binary(BinaryOp::Mul, Box::new(amplitude.clone()), r.clone());
                        recurse(l, &amp, out)
                    }
                    (false, true) => {
                        let amp =
                            IrExpr::Binary(BinaryOp::Mul, Box::new(amplitude.clone()), l.clone());
                        recurse(r, &amp, out)
                    }
                    (false, false) => Ok(()),
                }
            }
            IrExpr::Binary(BinaryOp::Div, l, r) => {
                if Self::contains_noise(r) {
                    return Err(unsupported("divisor"));
                }
                let amp = IrExpr::Binary(BinaryOp::Div, Box::new(amplitude.clone()), r.clone());
                recurse(l, &amp, out)
            }
            // Sign is irrelevant under the square
            IrExpr::Unary(UnaryOp::Neg | UnaryOp::Pos, e) => recurse(e, amplitude, out),
            // A guard gates the source: amplitude picks up cond ? 1 : 0,
            // which squares to the same 0/1 gate
            IrExpr::Conditional(c, t, e) => {
                if Self::contains_noise(c) {
                    return Err(unsupported("condition"));
                }
                if Self::contains_noise(t) {
                    let gate = IrExpr::Conditional(
                        c.clone(),
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Const(0.0)),
                    );
                    let amp =
                        IrExpr::Binary(BinaryOp::Mul, Box::new(amplitude.clone()), Box::new(gate));
                    recurse(t, &amp, out)?;
                }
                if Self::contains_noise(e) {
                    let gate = IrExpr::Conditional(
                        c.clone(),
                        Box::new(IrExpr::Const(0.0)),
                        Box::new(IrExpr::Const(1.0)),
                    );
                    let amp =
                        IrExpr::Binary(BinaryOp::Mul, Box::new(amplitude.clone()), Box::new(gate));
                    recurse(e, &amp, out)?;
                }
                Ok(())
            }
            // Anything else holding a noise term (inside ddt, functions,
            // comparisons, ...) has no defined small-signal meaning
            _ => Err(unsupported("nonlinear or dynamic position")),
        }
    }

    /// Extract the reactive operand of a contribution: for
    /// expr ~ resistive + ddt(Q), returns Q. Returns None when no ddt()
    /// is present.
    ///
    /// ddt() results must combine linearly per the LRM; sums, differences,
    /// negation, guards, and ddt-free multiplicative factors fold into Q
    /// (a bias-dependent factor f folds as f*Q, the quasi-static
    /// approximation: at the operating point dq/dt = 0, so the factor's
    /// own derivative carries no small-signal current).
    fn extract_charge(expr: &IrExpr) -> Option<IrExpr> {
        fn contains_ddt(e: &IrExpr) -> bool {
            match e {
                IrExpr::Ddt(_) => true,
                IrExpr::Binary(_, l, r) => contains_ddt(l) || contains_ddt(r),
                IrExpr::Unary(_, inner)
                | IrExpr::Limexp(inner)
                | IrExpr::DdtCompanion(inner)
                | IrExpr::IdtCompanion(inner) => contains_ddt(inner),
                IrExpr::Idt(inner, ic) => {
                    contains_ddt(inner) || ic.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::IdtMod {
                    expr,
                    ic,
                    modulus,
                    offset,
                } => {
                    contains_ddt(expr)
                        || ic.as_deref().is_some_and(contains_ddt)
                        || contains_ddt(modulus)
                        || offset.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Limit(inner, step) => {
                    contains_ddt(inner) || step.as_deref().is_some_and(contains_ddt)
                }
                IrExpr::Call(_, args) => args.iter().any(contains_ddt),
                IrExpr::Conditional(c, t, e) => {
                    contains_ddt(c) || contains_ddt(t) || contains_ddt(e)
                }
                IrExpr::TableLookup { input, .. } | IrExpr::TableDerivative { input, .. } => {
                    contains_ddt(input)
                }
                IrExpr::AbsDelay { expr, delay_time } => {
                    contains_ddt(expr) || contains_ddt(delay_time)
                }
                IrExpr::Transition { expr, .. }
                | IrExpr::Slew { expr, .. }
                | IrExpr::Cross { expr, .. }
                | IrExpr::LaplaceZP { expr, .. }
                | IrExpr::LaplaceND { expr, .. }
                | IrExpr::ZiFilter { expr, .. }
                | IrExpr::Ddx { expr, .. } => contains_ddt(expr),
                IrExpr::WhiteNoise { power, .. } => contains_ddt(power),
                IrExpr::FlickerNoise {
                    power, exponent, ..
                } => contains_ddt(power) || contains_ddt(exponent),
                IrExpr::NoiseTable { .. } => false,
                IrExpr::Above {
                    expr, threshold, ..
                } => contains_ddt(expr) || contains_ddt(threshold),
                IrExpr::Timer { start_time, period } => {
                    contains_ddt(start_time) || period.as_deref().is_some_and(contains_ddt)
                }
                // ddt() cannot appear in an element index (assignments
                // reject it upstream), so an indexed read is resistive
                IrExpr::VarIndexed { index, .. } => contains_ddt(index),
                IrExpr::Const(_)
                | IrExpr::Param(_)
                | IrExpr::ParamGiven(_)
                | IrExpr::Var(_)
                | IrExpr::Voltage(..)
                | IrExpr::Current(..)
                | IrExpr::BranchCurrent(_)
                | IrExpr::Time
                | IrExpr::Temperature
                | IrExpr::Vt
                | IrExpr::Mfactor
                | IrExpr::PortConnected(_)
                | IrExpr::Analysis(_) => false,
            }
        }

        match expr {
            IrExpr::Ddt(q) => Some((**q).clone()),
            IrExpr::Binary(op @ (BinaryOp::Add | BinaryOp::Sub), l, r) => {
                let ql = Self::extract_charge(l);
                let qr = Self::extract_charge(r);
                if ql.is_none() && qr.is_none() {
                    return None;
                }
                Some(IrExpr::Binary(
                    *op,
                    Box::new(ql.unwrap_or(IrExpr::Const(0.0))),
                    Box::new(qr.unwrap_or(IrExpr::Const(0.0))),
                ))
            }
            IrExpr::Binary(BinaryOp::Mul, l, r) => match (contains_ddt(l), contains_ddt(r)) {
                (false, false) => None,
                (false, true) => Self::extract_charge(r)
                    .map(|q| IrExpr::Binary(BinaryOp::Mul, l.clone(), Box::new(q))),
                (true, false) => Self::extract_charge(l)
                    .map(|q| IrExpr::Binary(BinaryOp::Mul, Box::new(q), r.clone())),
                (true, true) => {
                    log::warn!(
                        "ddt() on both sides of a product; reactive AC \
                             contribution omitted"
                    );
                    None
                }
            },
            IrExpr::Binary(BinaryOp::Div, l, r) if !contains_ddt(r) => Self::extract_charge(l)
                .map(|q| IrExpr::Binary(BinaryOp::Div, Box::new(q), r.clone())),
            IrExpr::Unary(op @ (UnaryOp::Neg | UnaryOp::Pos), e) => {
                Self::extract_charge(e).map(|q| IrExpr::Unary(*op, Box::new(q)))
            }
            IrExpr::Conditional(c, t, e) => {
                let qt = Self::extract_charge(t);
                let qe = Self::extract_charge(e);
                if qt.is_none() && qe.is_none() {
                    return None;
                }
                Some(IrExpr::Conditional(
                    c.clone(),
                    Box::new(qt.unwrap_or(IrExpr::Const(0.0))),
                    Box::new(qe.unwrap_or(IrExpr::Const(0.0))),
                ))
            }
            other => {
                if contains_ddt(other) {
                    log::warn!(
                        "ddt() inside an unsupported expression shape; its \
                         reactive contribution is omitted from AC analysis"
                    );
                }
                None
            }
        }
    }

    /// Peel leading instance-static guards (`cond ? inner : 0` where cond
    /// is fixed per instance) into a separate activation condition
    fn peel_static_condition(
        expr: IrExpr,
        static_vars: &HashSet<SmolStr>,
    ) -> (Option<IrExpr>, IrExpr) {
        let mut condition: Option<IrExpr> = None;
        let mut current = expr;
        loop {
            match current {
                IrExpr::Conditional(cond, then_expr, else_expr)
                    if Self::is_instance_static_expr(&cond, static_vars)
                        && matches!(*else_expr, IrExpr::Const(v) if v == 0.0) =>
                {
                    condition = Some(match condition {
                        Some(prev) => IrExpr::Binary(BinaryOp::And, Box::new(prev), cond),
                        None => *cond,
                    });
                    current = *then_expr;
                }
                other => return (condition, other),
            }
        }
    }

    /// Convert analyzed statements (assignments and runtime loops) to IR
    fn convert_statements(
        statements: &[crate::semantic::AnalyzedStatement],
        converter: &crate::expr_converter::ExprConverter,
        out: &mut Vec<IrAssignmentItem>,
    ) -> crate::error::CompileResult<()> {
        use crate::semantic::AnalyzedStatement;
        for stmt in statements {
            match stmt {
                AnalyzedStatement::Assignment(assign) => {
                    let expr = converter.convert(&assign.expression)?;
                    let index = match &assign.index {
                        Some(index_expr) => {
                            let (_base, lower, len) =
                                converter.array_layout(&assign.target).ok_or_else(|| {
                                    crate::error::CodeGenError::new(
                                        crate::error::CodeGenErrorKind::Internal(format!(
                                            "indexed assignment to unknown array '{}'",
                                            assign.target
                                        )),
                                    )
                                })?;
                            Some(IndexedTarget {
                                array: assign.target.clone(),
                                len,
                                lower,
                                index: converter.convert(index_expr)?,
                            })
                        }
                        None => None,
                    };
                    out.push(IrAssignmentItem::Assign(VarAssignment {
                        var_index: assign.var_index,
                        index,
                        expr,
                    }));
                }
                AnalyzedStatement::Loop(loop_stmt) => {
                    let condition = converter.convert(&loop_stmt.condition)?;
                    let mut body = Vec::with_capacity(loop_stmt.body.len());
                    Self::convert_statements(&loop_stmt.body, converter, &mut body)?;
                    out.push(IrAssignmentItem::Loop { condition, body });
                }
            }
        }
        Ok(())
    }

    /// Parse a branch name string like "p,n" or "p" to node indices
    fn parse_branch_name(
        branch_name: &str,
        ctx: &crate::expr_converter::ConversionContext,
    ) -> Option<BranchRef> {
        let parts: Vec<&str> = branch_name.split(',').collect();

        let pos_name = parts.first()?.trim();
        let pos_idx = ctx.node_index(pos_name)?;

        let neg_idx = if parts.len() > 1 {
            // An unknown negative node is an error, not silently ground
            ctx.node_index(parts[1].trim())?
        } else {
            ctx.ground()
        };

        Some(BranchRef {
            pos_terminal: pos_idx,
            neg_terminal: neg_idx,
        })
    }

    /// Generate derivatives for Jacobian entries over the unified node
    /// space (terminals, internal nodes) and the branch-current unknowns
    fn generate_derivatives(
        expr: &IrExpr,
        num_nodes: usize,
        num_branches: usize,
        shadows: &autodiff::ShadowContext,
    ) -> Vec<Derivative> {
        let mut derivatives = Vec::new();
        let active_axes = autodiff::expression_axes(expr, shadows, num_nodes);
        if active_axes == 0 {
            return derivatives;
        }

        for wrt in autodiff::axes(num_nodes, num_branches) {
            if !autodiff::mask_contains_axis(active_axes, &wrt, num_nodes) {
                continue;
            }
            let deriv_expr = autodiff::differentiate_with_shadows(expr, &wrt, shadows);
            let simplified = autodiff::simplify(deriv_expr);

            // Only add non-zero derivatives
            if !Self::is_zero(&simplified) {
                derivatives.push(Derivative {
                    wrt,
                    expr: simplified,
                });
            }
        }

        derivatives
    }

    /// Check if an expression is zero (constant 0.0)
    fn is_zero(expr: &IrExpr) -> bool {
        matches!(expr, IrExpr::Const(v) if v.abs() < 1e-30)
    }

    /// Check whether an expression depends only on parameters and constants
    /// (valid for instance-time parameter default evaluation)
    fn is_static_param_expr(expr: &IrExpr) -> bool {
        Self::is_instance_static_expr_with_options(expr, &HashSet::new(), false)
    }

    /// Check whether an expression is fixed per instance: it depends only
    /// on parameters, constants, temperature, analysis type, and variables
    /// proven instance-static. Such expressions may gate device topology.
    fn is_instance_static_expr(expr: &IrExpr, static_vars: &HashSet<SmolStr>) -> bool {
        Self::is_instance_static_expr_with_options(expr, static_vars, true)
    }

    fn is_instance_static_expr_with_options(
        expr: &IrExpr,
        static_vars: &HashSet<SmolStr>,
        allow_analysis: bool,
    ) -> bool {
        let recurse =
            |e: &IrExpr| Self::is_instance_static_expr_with_options(e, static_vars, allow_analysis);
        match expr {
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_) => true,
            IrExpr::Var(name) => static_vars.contains(name),
            // An indexed read is static when the index is static and every
            // element it could select is static
            IrExpr::VarIndexed {
                array,
                len,
                lower,
                index,
                ..
            } => {
                recurse(index)
                    && (*lower..*lower + *len as i64)
                        .all(|k| static_vars.contains(format!("{array}[{k}]").as_str()))
            }
            IrExpr::Binary(_, l, r) => recurse(l) && recurse(r),
            IrExpr::Unary(_, e) | IrExpr::Limexp(e) => recurse(e),
            IrExpr::Call(_, args) => args.iter().all(recurse),
            IrExpr::Conditional(c, t, e) => recurse(c) && recurse(t) && recurse(e),
            IrExpr::Analysis(_) => allow_analysis,
            _ => false,
        }
    }

    /// Fixpoint over the assignment tree: a variable is instance-static if
    /// every assignment to it uses only parameters, constants, and other
    /// instance-static variables. These variables hold the same value for
    /// every evaluation of a given instance (BSIM4's mode selectors like
    /// BSIM4rdsMod), so guards built from them may gate topology.
    fn compute_instance_static_vars(
        items: &[IrAssignmentItem],
        variables: &[VarDef],
    ) -> HashSet<SmolStr> {
        // Start from "all assigned variables are static" and remove any
        // with a non-static assignment until stable. Variables assigned
        // inside runtime loops stay eligible only if the loop condition is
        // also static (the iteration count must not vary per evaluation).
        let mut static_vars: HashSet<SmolStr> = HashSet::new();
        fn collect_targets(
            items: &[IrAssignmentItem],
            variables: &[VarDef],
            out: &mut HashSet<SmolStr>,
        ) {
            for item in items {
                match item {
                    IrAssignmentItem::Assign(a) => {
                        out.insert(variables[a.var_index].name.clone());
                    }
                    IrAssignmentItem::Loop { body, .. } => {
                        collect_targets(body, variables, out);
                    }
                }
            }
        }
        collect_targets(items, variables, &mut static_vars);

        loop {
            let mut changed = false;
            fn prune(
                items: &[IrAssignmentItem],
                variables: &[VarDef],
                static_vars: &mut HashSet<SmolStr>,
                changed: &mut bool,
                enclosing_static: bool,
            ) {
                for item in items {
                    match item {
                        IrAssignmentItem::Assign(a) => {
                            if let Some(target) = &a.index {
                                // A runtime-indexed write may land in any
                                // element; a non-static one evicts them all
                                let write_static = enclosing_static
                                    && DeviceIR::is_instance_static_expr(&a.expr, static_vars)
                                    && DeviceIR::is_instance_static_expr(
                                        &target.index,
                                        static_vars,
                                    );
                                if !write_static {
                                    for k in target.lower..target.lower + target.len as i64 {
                                        let elem: SmolStr = format!("{}[{k}]", target.array).into();
                                        if static_vars.remove(&elem) {
                                            *changed = true;
                                        }
                                    }
                                }
                                continue;
                            }
                            let name = &variables[a.var_index].name;
                            if static_vars.contains(name)
                                && (!enclosing_static
                                    || !DeviceIR::is_instance_static_expr(&a.expr, static_vars))
                            {
                                static_vars.remove(name);
                                *changed = true;
                            }
                        }
                        IrAssignmentItem::Loop { condition, body } => {
                            let loop_static = enclosing_static
                                && DeviceIR::is_instance_static_expr(condition, static_vars);
                            prune(body, variables, static_vars, changed, loop_static);
                        }
                    }
                }
            }
            prune(items, variables, &mut static_vars, &mut changed, true);
            if !changed {
                break;
            }
        }

        static_vars
    }
}

/// Automatic differentiation for Jacobian generation
pub mod autodiff {
    use super::*;
    use std::collections::{HashMap, HashSet};

    /// Bitmask over differentiation axes (node voltages first, then
    /// branch-current unknowns). Devices with more than 128 axes saturate
    /// to "all axes" — dense but always correct.
    pub(crate) type AxisMask = u128;

    /// All-axes mask (saturation value)
    const ALL_AXES: AxisMask = !0;

    /// Bit for one differentiation axis
    fn axis_bit(wrt: &DerivativeWrt, num_nodes: usize) -> AxisMask {
        let ordinal = match wrt {
            DerivativeWrt::Voltage(node) => *node,
            DerivativeWrt::BranchCurrent(k) => num_nodes + k,
        };
        if ordinal >= 128 {
            ALL_AXES
        } else {
            1 << ordinal
        }
    }

    pub(crate) fn mask_contains_axis(
        mask: AxisMask,
        wrt: &DerivativeWrt,
        num_nodes: usize,
    ) -> bool {
        mask & axis_bit(wrt, num_nodes) != 0
    }

    /// Bit for a unified node index appearing in a probe (the ground
    /// sentinel is not an axis)
    fn node_bit(node: usize) -> AxisMask {
        if node == usize::MAX {
            0
        } else if node >= 128 {
            ALL_AXES
        } else {
            1 << node
        }
    }

    /// Shadow-variable context for forward-mode AD through assignment
    /// sequences.
    ///
    /// For every variable whose value depends (transitively) on node
    /// voltages, a shadow variable holds d(var)/d(axis) — but only along
    /// the axes the variable can actually vary with (its dependency mask):
    /// a variable computed from V(g) and V(s) never carries shadows along
    /// the drain or any branch-current axis. The shadows are updated by
    /// generated assignments placed immediately before each original
    /// assignment.
    #[derive(Debug, Default)]
    pub struct ShadowContext {
        /// Dependency axes per voltage-dependent variable. For arrays, the
        /// array name and every element name share one mask: a runtime
        /// index may select any slot.
        shadowed: HashMap<SmolStr, AxisMask>,
        /// First slot of the contiguous shadow run per shadow-array name
        /// (`shadow_name(array, wrt)` -> variable index of element `lower`)
        array_shadow_base: HashMap<SmolStr, usize>,
        /// Node-axis count (axis ordinals of branch unknowns start here)
        num_nodes: usize,
    }

    impl ShadowContext {
        pub fn empty() -> Self {
            Self::default()
        }

        /// Name of the shadow variable holding the derivative of `name`
        /// along the given axis (node voltage or branch current)
        pub fn shadow_name(name: &str, wrt: &DerivativeWrt) -> SmolStr {
            match wrt {
                DerivativeWrt::Voltage(node) => format!("{name}@d{node}").into(),
                DerivativeWrt::BranchCurrent(k) => format!("{name}@dI{k}").into(),
            }
        }

        pub fn is_shadowed(&self, name: &str) -> bool {
            self.shadowed.get(name).is_some_and(|mask| *mask != 0)
        }

        /// Whether `name` carries a shadow along the given axis
        pub fn is_shadowed_on(&self, name: &str, wrt: &DerivativeWrt) -> bool {
            self.shadowed
                .get(name)
                .is_some_and(|mask| mask & axis_bit(wrt, self.num_nodes) != 0)
        }

        /// Dependency mask of a variable (0 when not shadowed)
        fn axes_of(&self, name: &str) -> AxisMask {
            self.shadowed.get(name).copied().unwrap_or(0)
        }

        /// First variable slot of an array's shadow run along an axis
        pub fn array_shadow_base(&self, array: &str, wrt: &DerivativeWrt) -> Option<usize> {
            self.array_shadow_base
                .get(&Self::shadow_name(array, wrt))
                .copied()
        }
    }

    /// All differentiation axes of a device: node voltages first, then
    /// branch-current unknowns
    pub(crate) fn axes(
        num_nodes: usize,
        num_branches: usize,
    ) -> impl Iterator<Item = DerivativeWrt> {
        (0..num_nodes)
            .map(DerivativeWrt::Voltage)
            .chain((0..num_branches).map(DerivativeWrt::BranchCurrent))
    }

    /// Collect every variable (and array) name an expression reads
    pub(crate) fn collect_var_names(expr: &IrExpr, out: &mut HashSet<SmolStr>) {
        map_expr(expr, &mut |e| {
            match e {
                IrExpr::Var(name) => {
                    out.insert(name.clone());
                }
                IrExpr::VarIndexed { array, .. } => {
                    out.insert(array.clone());
                }
                _ => {}
            }
            None
        });
    }

    /// Collect variable names appearing inside ddx() operands across an
    /// assignment tree (their derivative resolution reads shadows)
    pub(crate) fn collect_ddx_operand_names(
        items: &[IrAssignmentItem],
        out: &mut HashSet<SmolStr>,
    ) {
        fn scan(expr: &IrExpr, out: &mut HashSet<SmolStr>) {
            map_expr(expr, &mut |e| {
                if let IrExpr::Ddx { expr, .. } = e {
                    collect_var_names(expr, out);
                }
                None
            });
        }
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => scan(&assign.expr, out),
                IrAssignmentItem::Loop { condition, body } => {
                    scan(condition, out);
                    collect_ddx_operand_names(body, out);
                }
            }
        }
    }

    /// Check whether an expression can have a nonzero derivative along any
    /// node/branch axis, directly or through already-shadowed variables.
    ///
    /// Axes along which an expression can have a nonzero derivative,
    /// directly (probes) or through already-shadowed variables.
    ///
    /// Comparisons, logical operations, and event detectors differentiate
    /// to exactly zero regardless of their operands, so variables holding
    /// only such results (e.g. snapshotted branch guards) never need
    /// shadow slots; current probes are treated as constants in the DC
    /// Jacobian (matching [`differentiate_with_shadows`]).
    fn derivative_axes(
        expr: &IrExpr,
        deps: &HashMap<SmolStr, AxisMask>,
        num_nodes: usize,
    ) -> AxisMask {
        let recurse = |e: &IrExpr| derivative_axes(e, deps, num_nodes);
        match expr {
            IrExpr::Voltage(p, n) => node_bit(*p) | node_bit(*n),
            IrExpr::BranchCurrent(k) => axis_bit(&DerivativeWrt::BranchCurrent(*k), num_nodes),
            // Current probes differentiate to zero in the DC Jacobian
            IrExpr::Current(..) => 0,
            IrExpr::Var(name) => deps.get(name).copied().unwrap_or(0),
            // The index only selects; the elements carry the slope
            IrExpr::VarIndexed { array, .. } => deps.get(array).copied().unwrap_or(0),
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Time
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_)
            | IrExpr::Analysis(_) => 0,
            IrExpr::Binary(op, l, r) => match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
                    recurse(l) | recurse(r)
                }
                // Piecewise-constant results: derivative identically zero
                BinaryOp::Mod
                | BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::BitAnd
                | BinaryOp::BitOr
                | BinaryOp::BitXor
                | BinaryOp::Shl
                | BinaryOp::Shr => 0,
            },
            IrExpr::Unary(UnaryOp::Neg | UnaryOp::Pos, e) => recurse(e),
            IrExpr::Unary(UnaryOp::Not | UnaryOp::BitNot, _) => 0,
            IrExpr::Limexp(e) | IrExpr::Ddt(e) => recurse(e),
            IrExpr::Idt(e, _) => recurse(e),
            IrExpr::IdtMod { expr, .. } => recurse(expr),
            IrExpr::Limit(e, _) => recurse(e),
            IrExpr::Call(func, args) => match func {
                IrFunction::Floor | IrFunction::Ceil => 0,
                _ => args.iter().map(recurse).fold(0, |acc, m| acc | m),
            },
            // The condition only selects; the branches carry the slope
            IrExpr::Conditional(_, t, e) => recurse(t) | recurse(e),
            IrExpr::TableLookup { input, .. } => recurse(input),
            IrExpr::AbsDelay { expr, .. } => recurse(expr),
            IrExpr::Transition { expr, .. }
            | IrExpr::Slew { expr, .. }
            | IrExpr::LaplaceZP { expr, .. }
            | IrExpr::LaplaceND { expr, .. }
            | IrExpr::ZiFilter { expr, .. }
            | IrExpr::Ddx { expr, .. } => recurse(expr),
            IrExpr::DdtCompanion(e) | IrExpr::IdtCompanion(e) => recurse(e),
            IrExpr::TableDerivative { input, .. } => recurse(input),
            // Event detectors and noise sources are piecewise constant
            // (or zero) in the DC Jacobian
            IrExpr::Cross { .. }
            | IrExpr::Above { .. }
            | IrExpr::Timer { .. }
            | IrExpr::WhiteNoise { .. }
            | IrExpr::FlickerNoise { .. }
            | IrExpr::NoiseTable { .. } => 0,
        }
    }

    pub(crate) fn expression_axes(
        expr: &IrExpr,
        shadows: &ShadowContext,
        num_nodes: usize,
    ) -> AxisMask {
        derivative_axes(expr, &shadows.shadowed, num_nodes)
    }

    /// Accumulate per-variable dependency axes over an item tree
    /// (fixpoint helper for [`build_shadow_assignments`]).
    ///
    /// A voltage-dependent write into any array element shadows the whole
    /// array: a runtime index may route the value to any slot, so every
    /// element (and the array name itself, checked by indexed reads)
    /// shares one mask.
    fn scan_shadowed(
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        arrays: &[ArrayDef],
        num_nodes: usize,
        deps: &mut HashMap<SmolStr, AxisMask>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let mask = derivative_axes(&assign.expr, deps, num_nodes);
                    if mask == 0 {
                        continue;
                    }
                    let enclosing = arrays
                        .iter()
                        .find(|a| assign.var_index >= a.base && assign.var_index < a.base + a.len);
                    if let Some(array) = enclosing {
                        let current = deps.get(&array.name).copied().unwrap_or(0);
                        if current | mask != current {
                            let merged = current | mask;
                            deps.insert(array.name.clone(), merged);
                            for k in array.lower..array.lower + array.len as i64 {
                                deps.insert(format!("{}[{k}]", array.name).into(), merged);
                            }
                            *changed = true;
                        }
                    } else {
                        let name = &variables[assign.var_index].name;
                        let current = deps.get(name).copied().unwrap_or(0);
                        if current | mask != current {
                            deps.insert(name.clone(), current | mask);
                            *changed = true;
                        }
                    }
                }
                IrAssignmentItem::Loop { body, .. } => {
                    scan_shadowed(body, variables, arrays, num_nodes, deps, changed);
                }
            }
        }
    }

    /// Backward liveness step for shadow pruning: every variable read by
    /// an assignment to a live variable becomes live (indexed writes use
    /// the array name; the caller expands families afterwards)
    fn propagate_liveness(
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        live: &mut HashSet<SmolStr>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let target_live = match &assign.index {
                        Some(target) => live.contains(&target.array),
                        None => live.contains(&variables[assign.var_index].name),
                    };
                    if !target_live {
                        continue;
                    }
                    let mut reads = HashSet::new();
                    collect_var_names(&assign.expr, &mut reads);
                    if let Some(target) = &assign.index {
                        collect_var_names(&target.index, &mut reads);
                    }
                    for name in reads {
                        if live.insert(name) {
                            *changed = true;
                        }
                    }
                }
                IrAssignmentItem::Loop { body, .. } => {
                    propagate_liveness(body, variables, live, changed);
                }
            }
        }
    }

    /// Interleave shadow derivative updates before each original
    /// assignment, recursing into loop bodies so loop-carried voltage
    /// dependencies accumulate their derivatives per iteration
    fn interleave_shadows(
        items: Vec<IrAssignmentItem>,
        variables: &[VarDef],
        shadow_index: &HashMap<SmolStr, usize>,
        ctx: &ShadowContext,
        num_nodes: usize,
        num_branches: usize,
    ) -> Vec<IrAssignmentItem> {
        let mut rewritten = Vec::with_capacity(items.len() * 2);
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    if let Some(target) = &assign.index {
                        // Indexed write: the shadow run receives an indexed
                        // write of the value's derivative at the same slot,
                        // along the array's live axes only
                        let mask = ctx.axes_of(&target.array);
                        if mask != 0 {
                            for wrt in axes(num_nodes, num_branches) {
                                if mask & axis_bit(&wrt, num_nodes) == 0 {
                                    continue;
                                }
                                let deriv =
                                    simplify(differentiate_with_shadows(&assign.expr, &wrt, ctx));
                                let shadow_array = ShadowContext::shadow_name(&target.array, &wrt);
                                let shadow_base = ctx
                                    .array_shadow_base(&target.array, &wrt)
                                    .expect("shadowed array has a shadow run");
                                rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                    var_index: shadow_base,
                                    index: Some(IndexedTarget {
                                        array: shadow_array,
                                        len: target.len,
                                        lower: target.lower,
                                        index: target.index.clone(),
                                    }),
                                    expr: deriv,
                                }));
                            }
                        }
                        rewritten.push(IrAssignmentItem::Assign(assign));
                        continue;
                    }
                    let target = variables[assign.var_index].name.clone();
                    let mask = ctx.axes_of(&target);
                    if mask != 0 {
                        for wrt in axes(num_nodes, num_branches) {
                            if mask & axis_bit(&wrt, num_nodes) == 0 {
                                continue;
                            }
                            let deriv =
                                simplify(differentiate_with_shadows(&assign.expr, &wrt, ctx));
                            let shadow = ShadowContext::shadow_name(&target, &wrt);
                            rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                var_index: shadow_index[&shadow],
                                index: None,
                                expr: deriv,
                            }));
                        }
                    }
                    rewritten.push(IrAssignmentItem::Assign(assign));
                }
                IrAssignmentItem::Loop { condition, body } => {
                    let body = interleave_shadows(
                        body,
                        variables,
                        shadow_index,
                        ctx,
                        num_nodes,
                        num_branches,
                    );
                    rewritten.push(IrAssignmentItem::Loop { condition, body });
                }
            }
        }
        rewritten
    }

    /// Build shadow derivative assignments for voltage-dependent variables.
    ///
    /// Rewrites `ir.assignments` so that each assignment to a
    /// voltage-dependent variable is preceded by assignments computing the
    /// variable's partial derivative w.r.t. every node voltage and
    /// branch-current unknown. Shadow variables are appended to
    /// `ir.variables`.
    pub fn build_shadow_assignments(
        ir: &mut DeviceIR,
        num_nodes: usize,
        num_branches: usize,
        shadow_roots: &HashSet<SmolStr>,
    ) -> ShadowContext {
        // Fixpoint: a variable depends on an axis if any assignment to it
        // reads a probe of that axis or another variable depending on it.
        let mut deps: HashMap<SmolStr, AxisMask> = HashMap::new();
        loop {
            let mut changed = false;
            scan_shadowed(
                &ir.assignments,
                &ir.variables,
                &ir.arrays,
                num_nodes,
                &mut deps,
                &mut changed,
            );
            if !changed {
                break;
            }
        }

        // Backward liveness: a shadow matters only when the equation
        // Jacobians can reach it — the variable feeds a contribution (or
        // ddx operand) directly, or feeds an assignment to a live
        // variable. Dead shadows (operating-point reporting chains) are
        // dropped before any slot is allocated.
        let mut live: HashSet<SmolStr> = shadow_roots.clone();
        loop {
            let mut changed = false;
            propagate_liveness(&ir.assignments, &ir.variables, &mut live, &mut changed);
            // Array families share their mask; share liveness the same
            // way (one live element keeps the whole family)
            for array in &ir.arrays {
                let family_live = live.contains(&array.name)
                    || (array.lower..array.lower + array.len as i64)
                        .any(|k| live.contains(format!("{}[{k}]", array.name).as_str()));
                if family_live && live.insert(array.name.clone()) {
                    changed = true;
                }
                if family_live {
                    for k in array.lower..array.lower + array.len as i64 {
                        if live.insert(format!("{}[{k}]", array.name).into()) {
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        deps.retain(|name, _| live.contains(name));

        if deps.is_empty() {
            return ShadowContext::default();
        }

        // Register shadow variables along each variable's live axes only:
        // a value computed from V(g) and V(s) never varies with the drain
        // or any branch unknown, so those slots (and their update
        // assignments downstream) never exist. Array elements get their
        // slots in contiguous runs (allocated below) so runtime-indexed
        // reads and writes can address d(arr[i]) as
        // shadow_base + (i - lower); the scalar loop must skip them.
        let array_member: HashSet<SmolStr> = ir
            .arrays
            .iter()
            .filter(|a| deps.get(&a.name).copied().unwrap_or(0) != 0)
            .flat_map(|a| {
                std::iter::once(a.name.clone()).chain(
                    ir.variables[a.base..a.base + a.len]
                        .iter()
                        .map(|v| v.name.clone()),
                )
            })
            .collect();
        let mut shadow_index: HashMap<SmolStr, usize> = HashMap::new();
        for (name, mask) in &deps {
            if array_member.contains(name) {
                continue;
            }
            for wrt in axes(num_nodes, num_branches) {
                if mask & axis_bit(&wrt, num_nodes) == 0 {
                    continue;
                }
                let shadow = ShadowContext::shadow_name(name, &wrt);
                shadow_index.insert(shadow.clone(), ir.variables.len());
                ir.variables.push(VarDef {
                    name: shadow,
                    is_state: false,
                });
            }
        }

        // Contiguous shadow runs per (array, live axis)
        let mut array_shadow_base: HashMap<SmolStr, usize> = HashMap::new();
        let mut shadow_runs: Vec<VarDef> = Vec::new();
        for array in ir.arrays.iter() {
            let mask = deps.get(&array.name).copied().unwrap_or(0);
            if mask == 0 {
                continue;
            }
            for wrt in axes(num_nodes, num_branches) {
                if mask & axis_bit(&wrt, num_nodes) == 0 {
                    continue;
                }
                let run_base = ir.variables.len() + shadow_runs.len();
                array_shadow_base.insert(ShadowContext::shadow_name(&array.name, &wrt), run_base);
                for k in array.lower..array.lower + array.len as i64 {
                    let element = format!("{}[{k}]", array.name);
                    let shadow = ShadowContext::shadow_name(&element, &wrt);
                    shadow_index.insert(shadow.clone(), run_base + (k - array.lower) as usize);
                    shadow_runs.push(VarDef {
                        name: shadow,
                        is_state: false,
                    });
                }
            }
        }
        ir.variables.extend(shadow_runs);

        let ctx = ShadowContext {
            shadowed: deps,
            array_shadow_base,
            num_nodes,
        };

        // Interleave shadow updates before each original assignment.
        // Both the derivative and the original expression read the
        // pre-assignment values, so the shadows must be written first.
        let originals = std::mem::take(&mut ir.assignments);
        ir.assignments = interleave_shadows(
            originals,
            &ir.variables,
            &shadow_index,
            &ctx,
            num_nodes,
            num_branches,
        );

        ctx
    }

    /// Rewrite I(a,b) probes of branches carrying potential contributions
    /// into branch-current unknown references. The table maps a normalized
    /// (min,max) node pair to (ordinal, oriented positive node); a probe
    /// against the orientation negates.
    pub fn rewrite_branch_probes(
        expr: &IrExpr,
        table: &HashMap<(usize, usize), (usize, usize)>,
    ) -> IrExpr {
        map_expr(expr, &mut |e| {
            if let IrExpr::Current(p, n) = e {
                let key = (*p.min(n), *p.max(n));
                if let Some(&(ordinal, oriented_pos)) = table.get(&key) {
                    let unknown = IrExpr::BranchCurrent(ordinal);
                    return Some(if *p == oriented_pos {
                        unknown
                    } else {
                        IrExpr::Unary(UnaryOp::Neg, Box::new(unknown))
                    });
                }
            }
            None
        })
    }

    /// Apply [`rewrite_branch_probes`] across an assignment-item tree
    pub fn rewrite_branch_probes_in_items(
        items: &mut [IrAssignmentItem],
        table: &HashMap<(usize, usize), (usize, usize)>,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    assign.expr = rewrite_branch_probes(&assign.expr, table);
                    if let Some(target) = &mut assign.index {
                        target.index = rewrite_branch_probes(&target.index, table);
                    }
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = rewrite_branch_probes(condition, table);
                    rewrite_branch_probes_in_items(body, table);
                }
            }
        }
    }

    /// Resolve ddx() operators into explicit derivative expressions
    pub fn resolve_ddx(expr: &IrExpr, shadows: &ShadowContext) -> IrExpr {
        map_expr(expr, &mut |e| {
            if let IrExpr::Ddx { expr, pos, neg } = e {
                let inner = resolve_ddx(expr, shadows);
                let d_pos = simplify(differentiate_with_shadows(
                    &inner,
                    &DerivativeWrt::Voltage(*pos),
                    shadows,
                ));
                Some(match neg {
                    None => d_pos,
                    // ddx(f, V(a,b)): when f depends on the pair only
                    // through V(a)-V(b), (df/dVa - df/dVb)/2 is exactly
                    // df/d(Va-Vb)
                    Some(neg) => {
                        let d_neg = simplify(differentiate_with_shadows(
                            &inner,
                            &DerivativeWrt::Voltage(*neg),
                            shadows,
                        ));
                        simplify(IrExpr::Binary(
                            BinaryOp::Mul,
                            Box::new(IrExpr::Const(0.5)),
                            Box::new(IrExpr::Binary(
                                BinaryOp::Sub,
                                Box::new(d_pos),
                                Box::new(d_neg),
                            )),
                        ))
                    }
                })
            } else {
                None
            }
        })
    }

    /// Resolve ddx() operators across an assignment-item tree
    pub fn resolve_ddx_in_items(items: &mut [IrAssignmentItem], shadows: &ShadowContext) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    assign.expr = resolve_ddx(&assign.expr, shadows);
                    if let Some(target) = &mut assign.index {
                        target.index = resolve_ddx(&target.index, shadows);
                    }
                }
                IrAssignmentItem::Loop { condition, body } => {
                    *condition = resolve_ddx(condition, shadows);
                    resolve_ddx_in_items(body, shadows);
                }
            }
        }
    }

    /// Structurally map an IR expression bottom-up. The closure may replace
    /// a node entirely (returning Some) before its children are visited.
    fn map_expr(expr: &IrExpr, f: &mut impl FnMut(&IrExpr) -> Option<IrExpr>) -> IrExpr {
        if let Some(replacement) = f(expr) {
            return replacement;
        }
        match expr {
            IrExpr::Binary(op, l, r) => {
                IrExpr::Binary(*op, Box::new(map_expr(l, f)), Box::new(map_expr(r, f)))
            }
            IrExpr::Unary(op, e) => IrExpr::Unary(*op, Box::new(map_expr(e, f))),
            IrExpr::Call(func, args) => {
                IrExpr::Call(*func, args.iter().map(|a| map_expr(a, f)).collect())
            }
            IrExpr::Conditional(c, t, e) => IrExpr::Conditional(
                Box::new(map_expr(c, f)),
                Box::new(map_expr(t, f)),
                Box::new(map_expr(e, f)),
            ),
            IrExpr::Ddt(e) => IrExpr::Ddt(Box::new(map_expr(e, f))),
            IrExpr::Idt(e, ic) => IrExpr::Idt(
                Box::new(map_expr(e, f)),
                ic.as_ref().map(|e| Box::new(map_expr(e, f))),
            ),
            IrExpr::IdtMod {
                expr,
                ic,
                modulus,
                offset,
            } => IrExpr::IdtMod {
                expr: Box::new(map_expr(expr, f)),
                ic: ic.as_ref().map(|e| Box::new(map_expr(e, f))),
                modulus: Box::new(map_expr(modulus, f)),
                offset: offset.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::Limexp(e) => IrExpr::Limexp(Box::new(map_expr(e, f))),
            IrExpr::Limit(e, step) => IrExpr::Limit(
                Box::new(map_expr(e, f)),
                step.as_ref().map(|e| Box::new(map_expr(e, f))),
            ),
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => IrExpr::TableLookup {
                input: Box::new(map_expr(input, f)),
                x_data: x_data.clone(),
                y_data: y_data.clone(),
            },
            IrExpr::AbsDelay { expr, delay_time } => IrExpr::AbsDelay {
                expr: Box::new(map_expr(expr, f)),
                delay_time: Box::new(map_expr(delay_time, f)),
            },
            IrExpr::Transition {
                expr,
                delay,
                rise_time,
                fall_time,
            } => IrExpr::Transition {
                expr: Box::new(map_expr(expr, f)),
                delay: delay.as_ref().map(|e| Box::new(map_expr(e, f))),
                rise_time: rise_time.as_ref().map(|e| Box::new(map_expr(e, f))),
                fall_time: fall_time.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::Slew {
                expr,
                max_pos_slew,
                max_neg_slew,
            } => IrExpr::Slew {
                expr: Box::new(map_expr(expr, f)),
                max_pos_slew: max_pos_slew.as_ref().map(|e| Box::new(map_expr(e, f))),
                max_neg_slew: max_neg_slew.as_ref().map(|e| Box::new(map_expr(e, f))),
            },
            IrExpr::Ddx { expr, pos, neg } => IrExpr::Ddx {
                expr: Box::new(map_expr(expr, f)),
                pos: *pos,
                neg: *neg,
            },
            IrExpr::ZiFilter {
                expr,
                numerator,
                denominator,
                period,
            } => IrExpr::ZiFilter {
                expr: Box::new(map_expr(expr, f)),
                numerator: numerator.clone(),
                denominator: denominator.clone(),
                period: *period,
            },
            IrExpr::VarIndexed {
                array,
                base,
                len,
                lower,
                index,
            } => IrExpr::VarIndexed {
                array: array.clone(),
                base: *base,
                len: *len,
                lower: *lower,
                index: Box::new(map_expr(index, f)),
            },
            other => other.clone(),
        }
    }

    /// Differentiate an expression with respect to a variable
    /// (without assignment-chain shadows; prefer
    /// [`differentiate_with_shadows`] when a chain context exists)
    pub fn differentiate(expr: &IrExpr, wrt: &DerivativeWrt) -> IrExpr {
        differentiate_with_shadows(expr, wrt, &ShadowContext::default())
    }

    /// Differentiate an expression, chaining through shadowed variables
    pub fn differentiate_with_shadows(
        expr: &IrExpr,
        wrt: &DerivativeWrt,
        shadows: &ShadowContext,
    ) -> IrExpr {
        let differentiate = |e: &IrExpr| differentiate_with_shadows(e, wrt, shadows);
        match expr {
            IrExpr::Const(_) => IrExpr::Const(0.0),

            IrExpr::Voltage(p, n) => {
                if let DerivativeWrt::Voltage(v) = wrt {
                    if *v == *p {
                        IrExpr::Const(1.0)
                    } else if *v == *n {
                        IrExpr::Const(-1.0)
                    } else {
                        IrExpr::Const(0.0)
                    }
                } else {
                    IrExpr::Const(0.0)
                }
            }

            // Chain rule through intermediate variables: the shadow
            // variable carries the derivative along the active axis. A
            // variable that cannot vary along this axis differentiates to
            // zero without a shadow slot ever existing.
            IrExpr::Var(name) => {
                if shadows.is_shadowed_on(name, wrt) {
                    IrExpr::Var(ShadowContext::shadow_name(name, wrt))
                } else {
                    IrExpr::Const(0.0)
                }
            }

            // Runtime-indexed reads chain through the array's shadow run
            // at the same element; the index itself only selects
            IrExpr::VarIndexed {
                array,
                base: _,
                len,
                lower,
                index,
            } => match shadows.array_shadow_base(array, wrt) {
                Some(shadow_base) => IrExpr::VarIndexed {
                    array: ShadowContext::shadow_name(array, wrt),
                    base: shadow_base,
                    len: *len,
                    lower: *lower,
                    index: index.clone(),
                },
                None => IrExpr::Const(0.0),
            },

            // Branch-current unknowns differentiate to 1 along their own
            // axis and 0 along every other
            IrExpr::BranchCurrent(k) => match wrt {
                DerivativeWrt::BranchCurrent(j) if j == k => IrExpr::Const(1.0),
                _ => IrExpr::Const(0.0),
            },

            IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Time
            | IrExpr::Mfactor
            | IrExpr::PortConnected(_) => IrExpr::Const(0.0),

            IrExpr::Binary(op, left, right) => {
                let dl = differentiate(left);
                let dr = differentiate(right);

                match op {
                    BinaryOp::Add => IrExpr::Binary(BinaryOp::Add, Box::new(dl), Box::new(dr)),
                    BinaryOp::Sub => IrExpr::Binary(BinaryOp::Sub, Box::new(dl), Box::new(dr)),
                    BinaryOp::Mul => {
                        // Product rule: d(f*g) = f'*g + f*g'
                        IrExpr::Binary(
                            BinaryOp::Add,
                            Box::new(IrExpr::Binary(BinaryOp::Mul, Box::new(dl), right.clone())),
                            Box::new(IrExpr::Binary(BinaryOp::Mul, left.clone(), Box::new(dr))),
                        )
                    }
                    BinaryOp::Div => {
                        // Quotient rule: d(f/g) = (f'*g - f*g') / g^2
                        let num = IrExpr::Binary(
                            BinaryOp::Sub,
                            Box::new(IrExpr::Binary(BinaryOp::Mul, Box::new(dl), right.clone())),
                            Box::new(IrExpr::Binary(BinaryOp::Mul, left.clone(), Box::new(dr))),
                        );
                        let den = IrExpr::Binary(BinaryOp::Mul, right.clone(), right.clone());
                        IrExpr::Binary(BinaryOp::Div, Box::new(num), Box::new(den))
                    }
                    BinaryOp::Pow => {
                        // d(u^v) =
                        //   if v is const c: c*u^(c-1)*u'
                        //   else: u^v * (v' * ln(u) + v * u'/u)
                        match right.as_ref() {
                            IrExpr::Const(c) => {
                                let u_pow = IrExpr::Binary(
                                    BinaryOp::Pow,
                                    left.clone(),
                                    Box::new(IrExpr::Const(*c - 1.0)),
                                );
                                IrExpr::Binary(
                                    BinaryOp::Mul,
                                    Box::new(IrExpr::Const(*c)),
                                    Box::new(IrExpr::Binary(
                                        BinaryOp::Mul,
                                        Box::new(u_pow),
                                        Box::new(dl),
                                    )),
                                )
                            }
                            _ => {
                                let u_pow_v =
                                    IrExpr::Binary(BinaryOp::Pow, left.clone(), right.clone());
                                let vprime_ln_u = IrExpr::Binary(
                                    BinaryOp::Mul,
                                    Box::new(dr),
                                    Box::new(IrExpr::Call(
                                        IrFunction::Log,
                                        vec![left.as_ref().clone()],
                                    )),
                                );
                                let v_uprime_over_u = IrExpr::Binary(
                                    BinaryOp::Mul,
                                    right.clone(),
                                    Box::new(IrExpr::Binary(
                                        BinaryOp::Div,
                                        Box::new(dl),
                                        left.clone(),
                                    )),
                                );
                                let term = IrExpr::Binary(
                                    BinaryOp::Add,
                                    Box::new(vprime_ln_u),
                                    Box::new(v_uprime_over_u),
                                );
                                IrExpr::Binary(BinaryOp::Mul, Box::new(u_pow_v), Box::new(term))
                            }
                        }
                    }
                    // Piecewise-constant or discontinuous operators are treated
                    // as zero derivative in the DC Jacobian.
                    BinaryOp::Mod
                    | BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge
                    | BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::BitAnd
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::Shl
                    | BinaryOp::Shr => IrExpr::Const(0.0),
                }
            }

            IrExpr::Unary(UnaryOp::Neg, inner) => {
                IrExpr::Unary(UnaryOp::Neg, Box::new(differentiate(inner)))
            }
            // Unary plus is the identity
            IrExpr::Unary(UnaryOp::Pos, inner) => differentiate(inner),
            // Logical/bitwise negation is piecewise constant
            IrExpr::Unary(UnaryOp::Not | UnaryOp::BitNot, _) => IrExpr::Const(0.0),

            // d(c ? a : b) = c ? da : db
            IrExpr::Conditional(cond, then_expr, else_expr) => IrExpr::Conditional(
                cond.clone(),
                Box::new(differentiate(then_expr)),
                Box::new(differentiate(else_expr)),
            ),

            IrExpr::Call(func, args) if args.len() == 1 => {
                let inner = &args[0];
                let di = differentiate(inner);

                // Chain rule: d(f(g)) = f'(g) * g'
                let outer_deriv = match func {
                    IrFunction::Abs => IrExpr::Conditional(
                        Box::new(IrExpr::Binary(
                            BinaryOp::Ge,
                            Box::new(inner.clone()),
                            Box::new(IrExpr::Const(0.0)),
                        )),
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Const(-1.0)),
                    ),
                    IrFunction::Exp => IrExpr::Call(IrFunction::Exp, vec![inner.clone()]),
                    IrFunction::LimitedExp => limited_exp_derivative_scale(inner.clone()),
                    IrFunction::Log => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(inner.clone()),
                    ),
                    IrFunction::Log10 => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Mul,
                            Box::new(inner.clone()),
                            Box::new(IrExpr::Const(std::f64::consts::LN_10)),
                        )),
                    ),
                    IrFunction::Sqrt => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(0.5)),
                        Box::new(IrExpr::Call(IrFunction::Sqrt, vec![inner.clone()])),
                    ),
                    IrFunction::Sin => IrExpr::Call(IrFunction::Cos, vec![inner.clone()]),
                    IrFunction::Cos => IrExpr::Unary(
                        UnaryOp::Neg,
                        Box::new(IrExpr::Call(IrFunction::Sin, vec![inner.clone()])),
                    ),
                    IrFunction::Tan => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Pow,
                            Box::new(IrExpr::Call(IrFunction::Cos, vec![inner.clone()])),
                            Box::new(IrExpr::Const(2.0)),
                        )),
                    ),
                    IrFunction::Sinh => IrExpr::Call(IrFunction::Cosh, vec![inner.clone()]),
                    IrFunction::Cosh => IrExpr::Call(IrFunction::Sinh, vec![inner.clone()]),
                    IrFunction::Tanh => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Pow,
                            Box::new(IrExpr::Call(IrFunction::Cosh, vec![inner.clone()])),
                            Box::new(IrExpr::Const(2.0)),
                        )),
                    ),
                    IrFunction::Asin => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Call(
                            IrFunction::Sqrt,
                            vec![IrExpr::Binary(
                                BinaryOp::Sub,
                                Box::new(IrExpr::Const(1.0)),
                                Box::new(IrExpr::Binary(
                                    BinaryOp::Pow,
                                    Box::new(inner.clone()),
                                    Box::new(IrExpr::Const(2.0)),
                                )),
                            )],
                        )),
                    ),
                    IrFunction::Acos => IrExpr::Unary(
                        UnaryOp::Neg,
                        Box::new(IrExpr::Binary(
                            BinaryOp::Div,
                            Box::new(IrExpr::Const(1.0)),
                            Box::new(IrExpr::Call(
                                IrFunction::Sqrt,
                                vec![IrExpr::Binary(
                                    BinaryOp::Sub,
                                    Box::new(IrExpr::Const(1.0)),
                                    Box::new(IrExpr::Binary(
                                        BinaryOp::Pow,
                                        Box::new(inner.clone()),
                                        Box::new(IrExpr::Const(2.0)),
                                    )),
                                )],
                            )),
                        )),
                    ),
                    IrFunction::Atan => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(IrExpr::Binary(
                            BinaryOp::Add,
                            Box::new(IrExpr::Const(1.0)),
                            Box::new(IrExpr::Binary(
                                BinaryOp::Pow,
                                Box::new(inner.clone()),
                                Box::new(IrExpr::Const(2.0)),
                            )),
                        )),
                    ),
                    IrFunction::Floor | IrFunction::Ceil => IrExpr::Const(0.0),
                    _ => return IrExpr::Const(0.0),
                };

                IrExpr::Binary(BinaryOp::Mul, Box::new(outer_deriv), Box::new(di))
            }
            IrExpr::Call(IrFunction::Atan2, args) if args.len() == 2 => {
                // atan2(y, x): d = (x*dy - y*dx)/(x^2 + y^2)
                let y = args[0].clone();
                let x = args[1].clone();
                let dy = differentiate(&y);
                let dx = differentiate(&x);
                let num = IrExpr::Binary(
                    BinaryOp::Sub,
                    Box::new(IrExpr::Binary(
                        BinaryOp::Mul,
                        Box::new(x.clone()),
                        Box::new(dy),
                    )),
                    Box::new(IrExpr::Binary(
                        BinaryOp::Mul,
                        Box::new(y.clone()),
                        Box::new(dx),
                    )),
                );
                let den = IrExpr::Binary(
                    BinaryOp::Add,
                    Box::new(IrExpr::Binary(
                        BinaryOp::Pow,
                        Box::new(x),
                        Box::new(IrExpr::Const(2.0)),
                    )),
                    Box::new(IrExpr::Binary(
                        BinaryOp::Pow,
                        Box::new(y),
                        Box::new(IrExpr::Const(2.0)),
                    )),
                );
                IrExpr::Binary(BinaryOp::Div, Box::new(num), Box::new(den))
            }
            IrExpr::Call(IrFunction::Pow, args) if args.len() == 2 => {
                let as_binary = IrExpr::Binary(
                    BinaryOp::Pow,
                    Box::new(args[0].clone()),
                    Box::new(args[1].clone()),
                );
                differentiate(&as_binary)
            }
            IrExpr::Call(IrFunction::Min, args) if args.len() == 2 => {
                let left = args[0].clone();
                let right = args[1].clone();
                IrExpr::Conditional(
                    Box::new(IrExpr::Binary(
                        BinaryOp::Le,
                        Box::new(left.clone()),
                        Box::new(right.clone()),
                    )),
                    Box::new(differentiate(&left)),
                    Box::new(differentiate(&right)),
                )
            }
            IrExpr::Call(IrFunction::Max, args) if args.len() == 2 => {
                let left = args[0].clone();
                let right = args[1].clone();
                IrExpr::Conditional(
                    Box::new(IrExpr::Binary(
                        BinaryOp::Ge,
                        Box::new(left.clone()),
                        Box::new(right.clone()),
                    )),
                    Box::new(differentiate(&left)),
                    Box::new(differentiate(&right)),
                )
            }

            IrExpr::Limexp(inner) => {
                // d(limexp(x)) = limexp(x) * x' (same as exp, but clamped)
                let di = differentiate(inner);
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Limexp(inner.clone())),
                    Box::new(di),
                )
            }

            // ddt companion: d(ddt(q))/dV = (dq/dV) / dt under backward
            // Euler (zero at DC). The DdtCompanion wrapper multiplies its
            // operand by the integration coefficient at runtime.
            IrExpr::Ddt(inner) => IrExpr::DdtCompanion(Box::new(differentiate(inner))),

            // idt companion: d(idt(x))/dV = dt * dx/dV (zero at DC)
            IrExpr::Idt(inner, _) => IrExpr::IdtCompanion(Box::new(differentiate(inner))),

            // idtmod: the wrap is the identity almost everywhere, so the
            // small-signal derivative matches idt
            IrExpr::IdtMod { expr, .. } => IrExpr::IdtCompanion(Box::new(differentiate(expr))),

            // $limit passes its value through at convergence
            IrExpr::Limit(inner, _) => differentiate(inner),

            // Table lookup: slope of the active segment times the inner
            // derivative
            IrExpr::TableLookup {
                input,
                x_data,
                y_data,
            } => {
                let slope = IrExpr::TableDerivative {
                    input: input.clone(),
                    x_data: x_data.clone(),
                    y_data: y_data.clone(),
                };
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(slope),
                    Box::new(differentiate(input)),
                )
            }

            // Smoothing filters pass DC small-signal through; their
            // transient Jacobian approximation keeps the residual exact
            IrExpr::Transition { expr, .. }
            | IrExpr::Slew { expr, .. }
            | IrExpr::AbsDelay { expr, .. } => differentiate(expr),

            // Sampled-data filters: DC small-signal gain H(1) times the
            // inner derivative (the residual stays exact; the held-output
            // approximation only shapes convergence, like the laplace
            // filters below)
            IrExpr::ZiFilter {
                expr,
                numerator,
                denominator,
                ..
            } => {
                let num: f64 = numerator.iter().sum();
                let den: f64 = denominator.iter().sum();
                let gain = if den.abs() > 1e-300 { num / den } else { 0.0 };
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Const(gain)),
                    Box::new(differentiate(expr)),
                )
            }

            // Laplace filters: DC small-signal gain times the inner
            // derivative
            IrExpr::LaplaceND {
                expr,
                numerator,
                denominator,
            } => {
                let n0 = numerator.first().copied().unwrap_or(0.0);
                let d0 = denominator.first().copied().unwrap_or(1.0);
                let gain = if d0.abs() > 1e-300 { n0 / d0 } else { 0.0 };
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Const(gain)),
                    Box::new(differentiate(expr)),
                )
            }
            IrExpr::LaplaceZP {
                expr,
                zeros,
                poles,
                gain,
            } => {
                // H(0) = gain * prod(-z) / prod(-p) for real DC evaluation
                let num: f64 = zeros.iter().map(|(re, _)| -re).product();
                let den: f64 = poles.iter().map(|(re, _)| -re).product();
                let dc_gain = if den.abs() > 1e-300 {
                    gain * num / den
                } else {
                    0.0
                };
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Const(dc_gain)),
                    Box::new(differentiate(expr)),
                )
            }

            // Unresolved ddx: expand, then differentiate the expansion
            IrExpr::Ddx { .. } => {
                let resolved = resolve_ddx(expr, shadows);
                differentiate(&resolved)
            }

            // Event detectors, noise sources, analysis queries, and current
            // probes are treated as constants in the DC Jacobian
            _ => IrExpr::Const(0.0),
        }
    }

    fn limited_exp_derivative_scale(inner: IrExpr) -> IrExpr {
        const LIMIT: f64 = 80.0;
        let high = IrExpr::Binary(
            BinaryOp::Gt,
            Box::new(inner.clone()),
            Box::new(IrExpr::Const(LIMIT)),
        );
        let low = IrExpr::Binary(
            BinaryOp::Lt,
            Box::new(inner.clone()),
            Box::new(IrExpr::Const(-LIMIT)),
        );

        IrExpr::Conditional(
            Box::new(high),
            Box::new(IrExpr::Const(LIMIT.exp())),
            Box::new(IrExpr::Conditional(
                Box::new(low),
                Box::new(IrExpr::Const(0.0)),
                Box::new(IrExpr::Call(IrFunction::Exp, vec![inner])),
            )),
        )
    }

    /// Simplify an IR expression (constant folding, identity removal)
    pub fn simplify(expr: IrExpr) -> IrExpr {
        match expr {
            IrExpr::Binary(op, left, right) => {
                let left = simplify(*left);
                let right = simplify(*right);

                // Constant folding
                if let (IrExpr::Const(l), IrExpr::Const(r)) = (&left, &right) {
                    return IrExpr::Const(match op {
                        BinaryOp::Add => l + r,
                        BinaryOp::Sub => l - r,
                        BinaryOp::Mul => l * r,
                        BinaryOp::Div => l / r,
                        BinaryOp::Pow => l.powf(*r),
                        _ => return IrExpr::Binary(op, Box::new(left), Box::new(right)),
                    });
                }

                // Identity rules
                match op {
                    BinaryOp::Add => {
                        if let IrExpr::Const(0.0) = left {
                            return right;
                        }
                        if let IrExpr::Const(0.0) = right {
                            return left;
                        }
                    }
                    BinaryOp::Sub => {
                        if let IrExpr::Const(0.0) = right {
                            return left;
                        }
                    }
                    BinaryOp::Mul => {
                        if let IrExpr::Const(0.0) = left {
                            return IrExpr::Const(0.0);
                        }
                        if let IrExpr::Const(0.0) = right {
                            return IrExpr::Const(0.0);
                        }
                        if let IrExpr::Const(1.0) = left {
                            return right;
                        }
                        if let IrExpr::Const(1.0) = right {
                            return left;
                        }
                    }
                    BinaryOp::Div => {
                        if let IrExpr::Const(0.0) = left {
                            return IrExpr::Const(0.0);
                        }
                        if let IrExpr::Const(1.0) = right {
                            return left;
                        }
                    }
                    _ => {}
                }

                IrExpr::Binary(op, Box::new(left), Box::new(right))
            }
            IrExpr::Unary(op, inner) => {
                let inner = simplify(*inner);
                if let (UnaryOp::Neg, IrExpr::Const(v)) = (op, &inner) {
                    return IrExpr::Const(-v);
                }
                if let UnaryOp::Pos = op {
                    return inner;
                }
                IrExpr::Unary(op, Box::new(inner))
            }
            IrExpr::Conditional(cond, then_expr, else_expr) => {
                let cond = simplify(*cond);
                let then_expr = simplify(*then_expr);
                let else_expr = simplify(*else_expr);
                if let IrExpr::Const(c) = cond {
                    return if c != 0.0 { then_expr } else { else_expr };
                }
                IrExpr::Conditional(Box::new(cond), Box::new(then_expr), Box::new(else_expr))
            }
            IrExpr::Call(func, args) => {
                IrExpr::Call(func, args.into_iter().map(simplify).collect())
            }
            // Companion factors of a zero derivative vanish
            IrExpr::DdtCompanion(inner) => {
                let inner = simplify(*inner);
                if matches!(inner, IrExpr::Const(v) if v == 0.0) {
                    return IrExpr::Const(0.0);
                }
                IrExpr::DdtCompanion(Box::new(inner))
            }
            IrExpr::IdtCompanion(inner) => {
                let inner = simplify(*inner);
                if matches!(inner, IrExpr::Const(v) if v == 0.0) {
                    return IrExpr::Const(0.0);
                }
                IrExpr::IdtCompanion(Box::new(inner))
            }
            other => other,
        }
    }
}
