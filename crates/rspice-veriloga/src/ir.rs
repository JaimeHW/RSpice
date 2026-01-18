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
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Variable definition  
#[derive(Debug, Clone)]
pub struct VarDef {
    pub name: SmolStr,
    pub is_state: bool,
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
    pub fn from_analyzed(module: &AnalyzedModule) -> Self {
        use crate::expr_converter::{ConversionContext, ExprConverter};

        let mut ir = DeviceIR {
            name: module.name.clone(),
            terminals: Vec::new(),
            internal_nodes: Vec::new(),
            parameters: Vec::new(),
            variables: Vec::new(),
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

        // Convert contributions to equations
        for contrib in &module.contributions {
            // Parse branch name (format: "pos,neg" or "pos")
            let branch = Self::parse_branch_name(&contrib.branch, &ctx);

            if let Some(branch_ref) = branch {
                // Convert the expression
                if let Ok(expr) = converter.convert(&contrib.expression) {
                    // Generate derivatives for Jacobian
                    let derivatives = Self::generate_derivatives(&expr, &ir.terminals);

                    ir.equations.push(BranchEquation {
                        branch: branch_ref,
                        is_current: contrib.is_current,
                        expr,
                        derivatives,
                    });
                }
            }
        }

        ir
    }

    /// Parse a branch name string like "p,n" or "p" to terminal indices
    fn parse_branch_name(
        branch_name: &str,
        ctx: &crate::expr_converter::ConversionContext,
    ) -> Option<BranchRef> {
        let parts: Vec<&str> = branch_name.split(',').collect();

        let pos_name = parts.first()?.trim();
        let pos_idx = ctx.terminal_index(pos_name)?;

        let neg_idx = if parts.len() > 1 {
            let neg_name = parts[1].trim();
            ctx.terminal_index(neg_name).unwrap_or(ctx.ground())
        } else {
            ctx.ground()
        };

        Some(BranchRef {
            pos_terminal: pos_idx,
            neg_terminal: neg_idx,
        })
    }

    /// Generate derivatives for Jacobian entries
    fn generate_derivatives(expr: &IrExpr, terminals: &[Terminal]) -> Vec<Derivative> {
        let mut derivatives = Vec::new();

        // Generate partial derivative with respect to each terminal voltage
        for (i, _) in terminals.iter().enumerate() {
            let wrt = DerivativeWrt::Voltage(i);
            let deriv_expr = autodiff::differentiate(expr, &wrt);
            let simplified = autodiff::simplify(deriv_expr);

            // Only add non-zero derivatives
            if !Self::is_zero(&simplified) {
                derivatives.push(Derivative {
                    wrt,
                    expr: simplified,
                });
            }
        }

        // Also generate time derivative if expression contains ddt
        if Self::contains_ddt(expr) {
            derivatives.push(Derivative {
                wrt: DerivativeWrt::Time,
                expr: IrExpr::Const(1.0), // Placeholder - actual derivative handled in transient
            });
        }

        derivatives
    }

    /// Check if an expression is zero (constant 0.0)
    fn is_zero(expr: &IrExpr) -> bool {
        matches!(expr, IrExpr::Const(v) if v.abs() < 1e-30)
    }

    /// Check if expression contains ddt operator
    fn contains_ddt(expr: &IrExpr) -> bool {
        match expr {
            IrExpr::Ddt(_) => true,
            IrExpr::Binary(_, l, r) => Self::contains_ddt(l) || Self::contains_ddt(r),
            IrExpr::Unary(_, e) => Self::contains_ddt(e),
            IrExpr::Call(_, args) => args.iter().any(Self::contains_ddt),
            IrExpr::Conditional(c, t, e) => {
                Self::contains_ddt(c) || Self::contains_ddt(t) || Self::contains_ddt(e)
            }
            IrExpr::Limexp(e) => Self::contains_ddt(e),
            IrExpr::Idt(e, _) => Self::contains_ddt(e),
            _ => false,
        }
    }
}

/// Automatic differentiation for Jacobian generation
pub mod autodiff {
    use super::*;

    /// Differentiate an expression with respect to a variable
    pub fn differentiate(expr: &IrExpr, wrt: &DerivativeWrt) -> IrExpr {
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

            IrExpr::Param(_) | IrExpr::Temperature | IrExpr::Vt | IrExpr::Time => {
                IrExpr::Const(0.0)
            }

            IrExpr::Binary(op, left, right) => {
                let dl = differentiate(left, wrt);
                let dr = differentiate(right, wrt);

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
                    _ => IrExpr::Const(0.0), // TODO: handle other ops
                }
            }

            IrExpr::Unary(UnaryOp::Neg, inner) => {
                IrExpr::Unary(UnaryOp::Neg, Box::new(differentiate(inner, wrt)))
            }

            IrExpr::Call(func, args) if args.len() == 1 => {
                let inner = &args[0];
                let di = differentiate(inner, wrt);

                // Chain rule: d(f(g)) = f'(g) * g'
                let outer_deriv = match func {
                    IrFunction::Exp => IrExpr::Call(IrFunction::Exp, vec![inner.clone()]),
                    IrFunction::Log => IrExpr::Binary(
                        BinaryOp::Div,
                        Box::new(IrExpr::Const(1.0)),
                        Box::new(inner.clone()),
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
                    _ => return IrExpr::Const(0.0),
                };

                IrExpr::Binary(BinaryOp::Mul, Box::new(outer_deriv), Box::new(di))
            }

            IrExpr::Limexp(inner) => {
                // d(limexp(x)) = limexp(x) * x' (same as exp, but clamped)
                let di = differentiate(inner, wrt);
                IrExpr::Binary(
                    BinaryOp::Mul,
                    Box::new(IrExpr::Limexp(inner.clone())),
                    Box::new(di),
                )
            }

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
                    _ => {}
                }

                IrExpr::Binary(op, Box::new(left), Box::new(right))
            }
            IrExpr::Unary(op, inner) => {
                let inner = simplify(*inner);
                if let (UnaryOp::Neg, IrExpr::Const(v)) = (op, &inner) {
                    return IrExpr::Const(-v);
                }
                IrExpr::Unary(op, Box::new(inner))
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::autodiff::*;
    use super::*;

    #[test]
    fn test_differentiate_voltage() {
        let expr = IrExpr::Voltage(0, 1);
        let wrt = DerivativeWrt::Voltage(0);
        let deriv = differentiate(&expr, &wrt);
        assert!(matches!(deriv, IrExpr::Const(1.0)));
    }

    #[test]
    fn test_simplify_identity() {
        let expr = IrExpr::Binary(
            BinaryOp::Add,
            Box::new(IrExpr::Const(0.0)),
            Box::new(IrExpr::Param("r".into())),
        );
        let simplified = simplify(expr);
        assert!(matches!(simplified, IrExpr::Param(_)));
    }

    #[test]
    fn test_simplify_multiply_zero() {
        let expr = IrExpr::Binary(
            BinaryOp::Mul,
            Box::new(IrExpr::Const(0.0)),
            Box::new(IrExpr::Param("r".into())),
        );
        let simplified = simplify(expr);
        assert!(matches!(simplified, IrExpr::Const(v) if v == 0.0));
    }

    #[test]
    fn test_simplify_multiply_one() {
        let expr = IrExpr::Binary(
            BinaryOp::Mul,
            Box::new(IrExpr::Const(1.0)),
            Box::new(IrExpr::Param("r".into())),
        );
        let simplified = simplify(expr);
        assert!(matches!(simplified, IrExpr::Param(_)));
    }

    #[test]
    fn test_differentiate_product_rule() {
        // d(V(0,1) * Param("g")) / dV(0)
        let expr = IrExpr::Binary(
            BinaryOp::Mul,
            Box::new(IrExpr::Voltage(0, 1)),
            Box::new(IrExpr::Param("g".into())),
        );
        let wrt = DerivativeWrt::Voltage(0);
        let deriv = differentiate(&expr, &wrt);
        // Should be: 1.0 * g + V * 0 = g
        let simplified = simplify(deriv);
        assert!(matches!(simplified, IrExpr::Param(name) if name == "g"));
    }

    #[test]
    fn test_differentiate_exp() {
        // d(exp(V(0,1))) / dV(0) = exp(V(0,1)) * 1 = exp(V(0,1))
        let expr = IrExpr::Call(IrFunction::Exp, vec![IrExpr::Voltage(0, 1)]);
        let wrt = DerivativeWrt::Voltage(0);
        let deriv = differentiate(&expr, &wrt);
        let simplified = simplify(deriv);
        // Result should contain exp
        assert!(matches!(simplified, IrExpr::Call(IrFunction::Exp, _)));
    }

    #[test]
    fn test_contains_ddt() {
        let expr_with_ddt = IrExpr::Ddt(Box::new(IrExpr::Voltage(0, 1)));
        assert!(DeviceIR::contains_ddt(&expr_with_ddt));

        let expr_without_ddt = IrExpr::Voltage(0, 1);
        assert!(!DeviceIR::contains_ddt(&expr_without_ddt));

        // Nested ddt
        let nested = IrExpr::Binary(
            BinaryOp::Mul,
            Box::new(IrExpr::Param("c".into())),
            Box::new(IrExpr::Ddt(Box::new(IrExpr::Voltage(0, 1)))),
        );
        assert!(DeviceIR::contains_ddt(&nested));
    }

    #[test]
    fn test_is_zero() {
        assert!(DeviceIR::is_zero(&IrExpr::Const(0.0)));
        assert!(DeviceIR::is_zero(&IrExpr::Const(1e-35)));
        assert!(!DeviceIR::is_zero(&IrExpr::Const(1.0)));
        assert!(!DeviceIR::is_zero(&IrExpr::Param("x".into())));
    }
}
