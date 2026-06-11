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
    /// Index of variable being assigned
    pub var_index: usize,
    /// The expression to assign
    pub expr: IrExpr,
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
    /// Potential contributions reference a branch-current unknown
    pub branch_ordinal: Option<usize>,
    /// Instance-static activation condition (parameter-only guard peeled
    /// from the contribution). None = always active.
    pub static_condition: Option<IrExpr>,
    /// The expression tree
    pub expr: IrExpr,
    /// Partial derivatives (Jacobian entries)
    pub derivatives: Vec<Derivative>,
}

/// A branch-current unknown introduced by potential contributions
#[derive(Debug, Clone)]
pub struct BranchUnknownDef {
    /// Positive node (unified index)
    pub pos: usize,
    /// Negative node (unified index)
    pub neg: usize,
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
    /// ddx(expr, V(node)) - symbolic partial derivative w.r.t. a node
    /// potential. Resolved to an explicit derivative expression during
    /// device IR construction (where assignment chains are known).
    Ddx { expr: Box<IrExpr>, node: usize },
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

/// Noise source definition
#[derive(Debug, Clone)]
pub struct NoiseSourceDef {
    pub branch: BranchRef,
    pub kind: NoiseKind,
    pub power_expr: IrExpr,
    pub name: Option<SmolStr>,
}

/// Noise source kind
#[derive(Debug, Clone)]
pub enum NoiseKind {
    White,
    Flicker { exponent: f64 },
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
                default: param.default.unwrap_or(0.0),
                default_expr: None,
                min,
                max,
            });
        }

        // Build variables
        for var in &module.variables {
            ir.variables.push(VarDef {
                name: var.name.clone(),
                is_state: var.is_state,
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
        let mut parsed_contribs = Vec::with_capacity(module.contributions.len());
        // (min,max) node pair -> (ordinal, oriented positive node)
        let mut branch_table: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for contrib in &module.contributions {
            let branch_ref =
                Self::parse_branch_name(&contrib.branch, &ctx).ok_or_else(|| {
                    crate::error::CodeGenError::new(
                        crate::error::CodeGenErrorKind::InvalidExpression(format!(
                            "Unknown contribution branch '{}'",
                            contrib.branch
                        )),
                    )
                })?;

            if !contrib.is_current {
                let key = (
                    branch_ref.pos_terminal.min(branch_ref.neg_terminal),
                    branch_ref.pos_terminal.max(branch_ref.neg_terminal),
                );
                branch_table.entry(key).or_insert_with(|| {
                    let ordinal = ir.branch_unknowns.len();
                    ir.branch_unknowns.push(BranchUnknownDef {
                        pos: branch_ref.pos_terminal,
                        neg: branch_ref.neg_terminal,
                    });
                    (ordinal, branch_ref.pos_terminal)
                });
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

        // Forward-mode AD over the assignment sequence: build shadow
        // assignments holding each variable's partial derivative w.r.t.
        // every node voltage and branch-current unknown, so equation
        // Jacobians chain through intermediate variables. Shadow updates
        // recurse into loop bodies so loop-carried dependencies
        // differentiate correctly.
        let shadows = autodiff::build_shadow_assignments(&mut ir, num_nodes, num_branches);

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

            let (branch_ref, expr, branch_ordinal) = if contrib.is_current {
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
            let derivatives =
                Self::generate_derivatives(&expr, num_nodes, num_branches, &shadows);

            ir.equations.push(BranchEquation {
                branch: branch_ref,
                is_current: contrib.is_current,
                branch_ordinal,
                static_condition,
                expr,
                derivatives,
            });
        }

        Ok(ir)
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
                        Some(prev) => {
                            IrExpr::Binary(BinaryOp::And, Box::new(prev), cond)
                        }
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
                    out.push(IrAssignmentItem::Assign(VarAssignment {
                        var_index: assign.var_index,
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

        for wrt in autodiff::axes(num_nodes, num_branches) {
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
        Self::is_instance_static_expr(expr, &HashSet::new())
    }

    /// Check whether an expression is fixed per instance: it depends only
    /// on parameters, constants, temperature, and variables proven
    /// instance-static. Such expressions may gate device topology.
    fn is_instance_static_expr(expr: &IrExpr, static_vars: &HashSet<SmolStr>) -> bool {
        let recurse = |e: &IrExpr| Self::is_instance_static_expr(e, static_vars);
        match expr {
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Temperature
            | IrExpr::Vt => true,
            IrExpr::Var(name) => static_vars.contains(name),
            IrExpr::Binary(_, l, r) => recurse(l) && recurse(r),
            IrExpr::Unary(_, e) | IrExpr::Limexp(e) => recurse(e),
            IrExpr::Call(_, args) => args.iter().all(recurse),
            IrExpr::Conditional(c, t, e) => recurse(c) && recurse(t) && recurse(e),
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
                            let name = &variables[a.var_index].name;
                            if static_vars.contains(name)
                                && (!enclosing_static
                                    || !DeviceIR::is_instance_static_expr(
                                        &a.expr,
                                        static_vars,
                                    ))
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

    /// Shadow-variable context for forward-mode AD through assignment
    /// sequences.
    ///
    /// For every variable whose value depends (transitively) on node
    /// voltages, a shadow variable per node holds d(var)/dV(node). The
    /// shadows are updated by generated assignments placed immediately
    /// before each original assignment.
    #[derive(Debug, Default)]
    pub struct ShadowContext {
        /// Variables with voltage-dependent values
        shadowed: HashSet<SmolStr>,
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
            self.shadowed.contains(name)
        }
    }

    /// All differentiation axes of a device: node voltages first, then
    /// branch-current unknowns
    pub(crate) fn axes(num_nodes: usize, num_branches: usize) -> impl Iterator<Item = DerivativeWrt> {
        (0..num_nodes)
            .map(DerivativeWrt::Voltage)
            .chain((0..num_branches).map(DerivativeWrt::BranchCurrent))
    }

    /// Check whether an expression can have a nonzero derivative along any
    /// node/branch axis, directly or through already-shadowed variables.
    ///
    /// Comparisons, logical operations, and event detectors differentiate
    /// to exactly zero regardless of their operands, so variables holding
    /// only such results (e.g. snapshotted branch guards) never need
    /// shadow slots.
    fn has_nonzero_derivative(expr: &IrExpr, shadowed: &HashSet<SmolStr>) -> bool {
        let recurse = |e: &IrExpr| has_nonzero_derivative(e, shadowed);
        match expr {
            IrExpr::Voltage(..) | IrExpr::Current(..) | IrExpr::BranchCurrent(_) => true,
            IrExpr::Var(name) => shadowed.contains(name),
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Time
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Analysis(_) => false,
            IrExpr::Binary(op, l, r) => match op {
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Pow => recurse(l) || recurse(r),
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
                | BinaryOp::Shr => false,
            },
            IrExpr::Unary(UnaryOp::Neg | UnaryOp::Pos, e) => recurse(e),
            IrExpr::Unary(UnaryOp::Not | UnaryOp::BitNot, _) => false,
            IrExpr::Limexp(e) | IrExpr::Ddt(e) => recurse(e),
            IrExpr::Idt(e, _) => recurse(e),
            IrExpr::IdtMod { expr, .. } => recurse(expr),
            IrExpr::Limit(e, _) => recurse(e),
            IrExpr::Call(func, args) => match func {
                IrFunction::Floor | IrFunction::Ceil => false,
                _ => args.iter().any(recurse),
            },
            // The condition only selects; the branches carry the slope
            IrExpr::Conditional(_, t, e) => recurse(t) || recurse(e),
            IrExpr::TableLookup { input, .. } => recurse(input),
            IrExpr::AbsDelay { expr, .. } => recurse(expr),
            IrExpr::Transition { expr, .. }
            | IrExpr::Slew { expr, .. }
            | IrExpr::LaplaceZP { expr, .. }
            | IrExpr::LaplaceND { expr, .. }
            | IrExpr::Ddx { expr, .. } => recurse(expr),
            IrExpr::DdtCompanion(e) | IrExpr::IdtCompanion(e) => recurse(e),
            IrExpr::TableDerivative { input, .. } => recurse(input),
            // Event detectors and noise sources are piecewise constant
            // (or zero) in the DC Jacobian
            IrExpr::Cross { .. }
            | IrExpr::Above { .. }
            | IrExpr::Timer { .. }
            | IrExpr::WhiteNoise { .. }
            | IrExpr::FlickerNoise { .. } => false,
        }
    }

    /// Collect voltage-dependent variable names over an item tree
    /// (fixpoint helper for [`build_shadow_assignments`])
    fn scan_shadowed(
        items: &[IrAssignmentItem],
        variables: &[VarDef],
        shadowed: &mut HashSet<SmolStr>,
        changed: &mut bool,
    ) {
        for item in items {
            match item {
                IrAssignmentItem::Assign(assign) => {
                    let name = &variables[assign.var_index].name;
                    if !shadowed.contains(name) && has_nonzero_derivative(&assign.expr, shadowed) {
                        shadowed.insert(name.clone());
                        *changed = true;
                    }
                }
                IrAssignmentItem::Loop { body, .. } => {
                    scan_shadowed(body, variables, shadowed, changed);
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
                    let target = variables[assign.var_index].name.clone();
                    if ctx.is_shadowed(&target) {
                        for wrt in axes(num_nodes, num_branches) {
                            let deriv =
                                simplify(differentiate_with_shadows(&assign.expr, &wrt, ctx));
                            let shadow = ShadowContext::shadow_name(&target, &wrt);
                            rewritten.push(IrAssignmentItem::Assign(VarAssignment {
                                var_index: shadow_index[&shadow],
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
    ) -> ShadowContext {
        // Fixpoint: a variable is voltage-dependent if any assignment to it
        // depends on node quantities or on another shadowed variable.
        let mut shadowed: HashSet<SmolStr> = HashSet::new();
        loop {
            let mut changed = false;
            scan_shadowed(&ir.assignments, &ir.variables, &mut shadowed, &mut changed);
            if !changed {
                break;
            }
        }

        if shadowed.is_empty() {
            return ShadowContext::default();
        }

        let ctx = ShadowContext {
            shadowed: shadowed.clone(),
        };

        // Register shadow variables
        let mut shadow_index: HashMap<SmolStr, usize> = HashMap::new();
        for name in &shadowed {
            for wrt in axes(num_nodes, num_branches) {
                let shadow = ShadowContext::shadow_name(name, &wrt);
                shadow_index.insert(shadow.clone(), ir.variables.len());
                ir.variables.push(VarDef {
                    name: shadow,
                    is_state: false,
                });
            }
        }

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
            if let IrExpr::Ddx { expr, node } = e {
                let inner = resolve_ddx(expr, shadows);
                let wrt = DerivativeWrt::Voltage(*node);
                Some(simplify(differentiate_with_shadows(&inner, &wrt, shadows)))
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
            IrExpr::Ddx { expr, node } => IrExpr::Ddx {
                expr: Box::new(map_expr(expr, f)),
                node: *node,
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
            // variable carries the derivative along the active axis
            IrExpr::Var(name) => {
                if shadows.is_shadowed(name) {
                    IrExpr::Var(ShadowContext::shadow_name(name, wrt))
                } else {
                    IrExpr::Const(0.0)
                }
            }

            // Branch-current unknowns differentiate to 1 along their own
            // axis and 0 along every other
            IrExpr::BranchCurrent(k) => match wrt {
                DerivativeWrt::BranchCurrent(j) if j == k => IrExpr::Const(1.0),
                _ => IrExpr::Const(0.0),
            },

            IrExpr::Param(_) | IrExpr::Temperature | IrExpr::Vt | IrExpr::Time => {
                IrExpr::Const(0.0)
            }

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
            IrExpr::IdtMod { expr, .. } => {
                IrExpr::IdtCompanion(Box::new(differentiate(expr)))
            }

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
                IrExpr::Binary(BinaryOp::Mul, Box::new(slope), Box::new(differentiate(input)))
            }

            // Smoothing filters pass DC small-signal through; their
            // transient Jacobian approximation keeps the residual exact
            IrExpr::Transition { expr, .. }
            | IrExpr::Slew { expr, .. }
            | IrExpr::AbsDelay { expr, .. } => differentiate(expr),

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
