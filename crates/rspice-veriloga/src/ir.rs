//! Intermediate Representation for Verilog-A
//!
//! The IR represents device equations in a form suitable for:
//! 1. Automatic differentiation (Jacobian generation)
//! 2. Code generation for MNA matrix stamping

use crate::ast::{BinaryOp, UnaryOp};
use crate::semantic::AnalyzedModule;
use smol_str::SmolStr;

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
    /// Variable assignments (in execution order)
    pub assignments: Vec<VarAssignment>,
    /// Branch equations
    pub equations: Vec<BranchEquation>,
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

/// Branch equation: represents I(p,n) <+ f(V, params)
#[derive(Debug, Clone)]
pub struct BranchEquation {
    /// Branch identifier
    pub branch: BranchRef,
    /// Whether this contributes current (true) or voltage (false)
    pub is_current: bool,
    /// The expression tree
    pub expr: IrExpr,
    /// Partial derivatives (Jacobian entries)
    pub derivatives: Vec<Derivative>,
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
    /// Voltage at terminal
    Voltage(usize),
    /// Current through branch
    Current(usize, usize),
    /// Time (for ddt)
    Time,
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

        // Convert assignments to IR (in order)
        for assign in &module.assignments {
            let expr = converter.convert(&assign.expression)?;
            ir.assignments.push(VarAssignment {
                var_index: assign.var_index,
                expr,
            });
        }

        // Forward-mode AD over the assignment sequence: build shadow
        // assignments holding each variable's partial derivative w.r.t.
        // every node voltage, so equation Jacobians chain through
        // intermediate variables.
        let shadows = autodiff::build_shadow_assignments(&mut ir, num_nodes);

        // Resolve ddx() operators now that the shadow context exists
        for assign in &mut ir.assignments {
            assign.expr = autodiff::resolve_ddx(&assign.expr, &shadows);
        }

        // Convert contributions to equations
        for contrib in &module.contributions {
            // Parse branch name (format: "pos,neg" or "pos")
            let branch_ref =
                Self::parse_branch_name(&contrib.branch, &ctx).ok_or_else(|| {
                    crate::error::CodeGenError::new(
                        crate::error::CodeGenErrorKind::InvalidExpression(format!(
                            "Unknown contribution branch '{}'",
                            contrib.branch
                        )),
                    )
                })?;

            // Convert the expression
            let expr = converter.convert(&contrib.expression)?;
            let expr = autodiff::resolve_ddx(&expr, &shadows);

            // Generate derivatives for Jacobian
            let derivatives = Self::generate_derivatives(&expr, num_nodes, &shadows);

            ir.equations.push(BranchEquation {
                branch: branch_ref,
                is_current: contrib.is_current,
                expr,
                derivatives,
            });
        }

        Ok(ir)
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
    /// space (terminals first, then internal nodes)
    fn generate_derivatives(
        expr: &IrExpr,
        num_nodes: usize,
        shadows: &autodiff::ShadowContext,
    ) -> Vec<Derivative> {
        let mut derivatives = Vec::new();

        // Generate partial derivative with respect to each node voltage
        for i in 0..num_nodes {
            let wrt = DerivativeWrt::Voltage(i);
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
        match expr {
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Temperature
            | IrExpr::Vt => true,
            IrExpr::Binary(_, l, r) => {
                Self::is_static_param_expr(l) && Self::is_static_param_expr(r)
            }
            IrExpr::Unary(_, e) | IrExpr::Limexp(e) => Self::is_static_param_expr(e),
            IrExpr::Call(_, args) => args.iter().all(Self::is_static_param_expr),
            IrExpr::Conditional(c, t, e) => {
                Self::is_static_param_expr(c)
                    && Self::is_static_param_expr(t)
                    && Self::is_static_param_expr(e)
            }
            _ => false,
        }
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

        /// Name of the shadow variable holding d(name)/dV(node)
        pub fn shadow_name(name: &str, node: usize) -> SmolStr {
            format!("{name}@d{node}").into()
        }

        pub fn is_shadowed(&self, name: &str) -> bool {
            self.shadowed.contains(name)
        }
    }

    /// Check whether an expression depends on node quantities, directly or
    /// through already-shadowed variables.
    fn depends_on_nodes(expr: &IrExpr, shadowed: &HashSet<SmolStr>) -> bool {
        match expr {
            IrExpr::Voltage(..) | IrExpr::Current(..) => true,
            IrExpr::Var(name) => shadowed.contains(name),
            IrExpr::Const(_)
            | IrExpr::Param(_)
            | IrExpr::ParamGiven(_)
            | IrExpr::Time
            | IrExpr::Temperature
            | IrExpr::Vt
            | IrExpr::Analysis(_) => false,
            IrExpr::Binary(_, l, r) => {
                depends_on_nodes(l, shadowed) || depends_on_nodes(r, shadowed)
            }
            IrExpr::Unary(_, e) | IrExpr::Limexp(e) | IrExpr::Ddt(e) => {
                depends_on_nodes(e, shadowed)
            }
            IrExpr::Idt(e, ic) => {
                depends_on_nodes(e, shadowed)
                    || ic.as_ref().is_some_and(|e| depends_on_nodes(e, shadowed))
            }
            IrExpr::Limit(e, step) => {
                depends_on_nodes(e, shadowed)
                    || step
                        .as_ref()
                        .is_some_and(|e| depends_on_nodes(e, shadowed))
            }
            IrExpr::Call(_, args) => args.iter().any(|a| depends_on_nodes(a, shadowed)),
            IrExpr::Conditional(c, t, e) => {
                depends_on_nodes(c, shadowed)
                    || depends_on_nodes(t, shadowed)
                    || depends_on_nodes(e, shadowed)
            }
            IrExpr::TableLookup { input, .. } => depends_on_nodes(input, shadowed),
            IrExpr::AbsDelay { expr, delay_time } => {
                depends_on_nodes(expr, shadowed) || depends_on_nodes(delay_time, shadowed)
            }
            IrExpr::Transition { expr, .. }
            | IrExpr::Slew { expr, .. }
            | IrExpr::Cross { expr, .. }
            | IrExpr::LaplaceZP { expr, .. }
            | IrExpr::LaplaceND { expr, .. }
            | IrExpr::Ddx { expr, .. } => depends_on_nodes(expr, shadowed),
            IrExpr::DdtCompanion(e) | IrExpr::IdtCompanion(e) => depends_on_nodes(e, shadowed),
            IrExpr::TableDerivative { input, .. } => depends_on_nodes(input, shadowed),
            IrExpr::WhiteNoise { power, .. } => depends_on_nodes(power, shadowed),
            IrExpr::FlickerNoise {
                power, exponent, ..
            } => depends_on_nodes(power, shadowed) || depends_on_nodes(exponent, shadowed),
            IrExpr::Above {
                expr, threshold, ..
            } => depends_on_nodes(expr, shadowed) || depends_on_nodes(threshold, shadowed),
            IrExpr::Timer { start_time, period } => {
                depends_on_nodes(start_time, shadowed)
                    || period
                        .as_ref()
                        .is_some_and(|e| depends_on_nodes(e, shadowed))
            }
        }
    }

    /// Build shadow derivative assignments for voltage-dependent variables.
    ///
    /// Rewrites `ir.assignments` so that each assignment to a
    /// voltage-dependent variable is preceded by assignments computing the
    /// variable's partial derivative w.r.t. every node voltage. Shadow
    /// variables are appended to `ir.variables`.
    pub fn build_shadow_assignments(ir: &mut DeviceIR, num_nodes: usize) -> ShadowContext {
        // Fixpoint: a variable is voltage-dependent if any assignment to it
        // depends on node quantities or on another shadowed variable.
        let mut shadowed: HashSet<SmolStr> = HashSet::new();
        loop {
            let mut changed = false;
            for assign in &ir.assignments {
                let name = ir.variables[assign.var_index].name.clone();
                if !shadowed.contains(&name) && depends_on_nodes(&assign.expr, &shadowed) {
                    shadowed.insert(name);
                    changed = true;
                }
            }
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
            for node in 0..num_nodes {
                let shadow = ShadowContext::shadow_name(name, node);
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
        let mut rewritten = Vec::with_capacity(originals.len() * (1 + num_nodes));
        for assign in originals {
            let target = ir.variables[assign.var_index].name.clone();
            if shadowed.contains(&target) {
                for node in 0..num_nodes {
                    let wrt = DerivativeWrt::Voltage(node);
                    let deriv = simplify(differentiate_with_shadows(&assign.expr, &wrt, &ctx));
                    let shadow = ShadowContext::shadow_name(&target, node);
                    rewritten.push(VarAssignment {
                        var_index: shadow_index[&shadow],
                        expr: deriv,
                    });
                }
            }
            rewritten.push(assign);
        }
        ir.assignments = rewritten;

        ctx
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
            // variable carries d(var)/dV(node)
            IrExpr::Var(name) => match wrt {
                DerivativeWrt::Voltage(node) if shadows.is_shadowed(name) => {
                    IrExpr::Var(ShadowContext::shadow_name(name, *node))
                }
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
