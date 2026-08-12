//! Expression Abstract Syntax Tree
//!
//! Represents mathematical expressions for behavioral sources.

use crate::Value;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::sync::Arc;

/// Mathematical expression AST node
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Constant value
    Const(Value),
    /// Node voltage: V(node)
    NodeVoltage(String),
    /// Branch current: I(element)
    BranchCurrent(String),
    /// Binary operation
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary operation
    Unary { op: UnaryOp, operand: Box<Expr> },
    /// Function call
    Function { func: Function, args: Vec<Expr> },
    /// String literal used by build-time-resolved expression functions.
    StringLiteral(String),
    /// Build-time-resolved lookup table with an explicit runtime input.
    LookupTable {
        input: Box<Expr>,
        table: LookupTable,
    },
    /// Time variable (for transient)
    Time,
    /// Frequency variable (for AC)
    Frequency,
    /// Circuit temperature in degrees Celsius (`temper`)
    Temperature,
    /// Thermal voltage kT/q at the active circuit temperature.
    ThermalVoltage,
    /// Active minimum conductance used by the nonlinear solver.
    Gmin,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    // Comparison
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    // Logical
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// Built-in functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    // Basic math
    Abs,
    Sqrt,
    Exp,
    Log,
    Ln,
    Log10,
    // Trigonometric
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Atan2, // atan2(y, x) - two-argument arctangent
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    // Rounding
    Trunc,
    Floor,
    Ceil,
    Round,
    Sqr,
    // SPICE-specific
    Pwr,   // Dialect-specific named PWR function
    Pwrs,  // pwrs(x, y) = sign(x) * |x|^y
    Limit, // limit(nominal, variation) or limit(x, lo, hi)
    Min,
    Max,
    Sign,            // sgn(x) = -1, 0, or 1
    HspiceSign,      // sign(x, y) = sgn(y) * |x|
    Uramp,           // uramp(x) = max(0, x) - positive ramp
    Stp,             // stp(x) = 1 if x is positive outside expression-zero tolerance else 0
    Ustep,           // u(x) / ustep(x) = 0, 0.5, or 1 at the exact ngspice step boundary
    U2,              // u2(x) = clamp(x, 0, 1)
    Eq0,             // eq0(x) = 1 if x == 0 else 0
    Ne0,             // ne0(x) = 1 if x != 0 else 0
    Gt0,             // gt0(x) = 1 if x > 0 else 0
    Lt0,             // lt0(x) = 1 if x < 0 else 0
    Ge0,             // ge0(x) = 1 if x >= 0 else 0
    Le0,             // le0(x) = 1 if x <= 0 else 0
    Pow,             // Dialect-specific named POW function
    Table,           // table(x, x1,y1, x2,y2, ...)
    Pwl,             // pwl(x, x1,y1, x2,y2, ...) alias for table
    TableFile,       // tablefile("path") / tablefile(path) = file-backed table(time)
    FastTable, // fasttable("path") / fasttable(path) = file-backed table(time), no breakpoints
    FastTableFile, // fasttablefile("path") / fasttablefile(path)
    Cubic,     // cubic("path") = file-backed natural cubic spline of time
    CubicFile, // cubicfile("path") alias
    Akima,     // akima(x, x1,y1, ...) or akima("path"); spline alias
    AkimaFile, // akimafile("path") / splinefile("path") alias
    Wodicka,   // wodicka("path") = file-backed rounded-corner Akima variant
    WodickaFile, // wodickafile("path") alias
    Barycentric, // bli("path") = file-backed barycentric Lagrange interpolation
    BarycentricFile, // blifile("path") alias
    Sdt,       // sdt(x) = rollback-safe trapezoidal time integral
    Mod,       // mod(x, y) = x % y - modulo
    SpicePulse, // spice_pulse(v1, v2, td, tr, tf, pw[, per])
    SpiceSin,  // spice_sin(vo, va, freq, td, theta[, phase_degrees])
    SpiceExp,  // spice_exp(v1, v2, td1, tau1, td2, tau2)
    SpiceSffm, // spice_sffm(vo, va[, fc[, mdi[, fs]]])
    // Conditional
    If, // if(cond, then, else)
}

impl Function {
    /// Resolve a built-in function name using the canonical expression grammar
    /// and all accepted compatibility aliases.
    ///
    /// Keeping this table on the semantic enum gives behavioral parsing and
    /// output-expression validation one source of truth. User-defined
    /// functions intentionally return `None` here.
    pub(crate) fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_uppercase().as_str() {
            "ABS" | "M" | "MAG" => Some(Self::Abs),
            "SQRT" => Some(Self::Sqrt),
            "EXP" => Some(Self::Exp),
            "LOG" => Some(Self::Log),
            "LN" => Some(Self::Ln),
            "LOG10" => Some(Self::Log10),
            "SIN" => Some(Self::Sin),
            "COS" => Some(Self::Cos),
            "TAN" => Some(Self::Tan),
            "ASIN" | "ARCSIN" => Some(Self::Asin),
            "ACOS" | "ARCCOS" => Some(Self::Acos),
            "ATAN" | "ARCTAN" => Some(Self::Atan),
            "ATAN2" => Some(Self::Atan2),
            "SINH" => Some(Self::Sinh),
            "COSH" => Some(Self::Cosh),
            "TANH" => Some(Self::Tanh),
            "ASINH" => Some(Self::Asinh),
            "ACOSH" => Some(Self::Acosh),
            "ATANH" => Some(Self::Atanh),
            "INT" | "TRUNC" => Some(Self::Trunc),
            "FLOOR" => Some(Self::Floor),
            "CEIL" | "CEILING" => Some(Self::Ceil),
            "ROUND" | "NINT" => Some(Self::Round),
            "SQR" => Some(Self::Sqr),
            "MIN" => Some(Self::Min),
            "MAX" => Some(Self::Max),
            "POW" => Some(Self::Pow),
            "PWR" => Some(Self::Pwr),
            "PWRS" => Some(Self::Pwrs),
            "LIMIT" => Some(Self::Limit),
            "SGN" => Some(Self::Sign),
            "SIGN" => Some(Self::HspiceSign),
            "URAMP" => Some(Self::Uramp),
            "STP" | "STEP" => Some(Self::Stp),
            "U" | "USTEP" => Some(Self::Ustep),
            "U2" => Some(Self::U2),
            "EQ0" => Some(Self::Eq0),
            "NE0" => Some(Self::Ne0),
            "GT0" => Some(Self::Gt0),
            "LT0" => Some(Self::Lt0),
            "GE0" => Some(Self::Ge0),
            "LE0" => Some(Self::Le0),
            "TABLE" => Some(Self::Table),
            "TABLEFILE" => Some(Self::TableFile),
            "FASTTABLE" => Some(Self::FastTable),
            "FASTTABLEFILE" => Some(Self::FastTableFile),
            "CUBIC" => Some(Self::Cubic),
            "CUBICFILE" => Some(Self::CubicFile),
            "AKIMA" | "SPLINE" => Some(Self::Akima),
            "AKIMAFILE" | "SPLINEFILE" => Some(Self::AkimaFile),
            "WODICKA" => Some(Self::Wodicka),
            "WODICKAFILE" => Some(Self::WodickaFile),
            "BLI" => Some(Self::Barycentric),
            "BLIFILE" => Some(Self::BarycentricFile),
            "SDT" => Some(Self::Sdt),
            "PWL" => Some(Self::Pwl),
            "MOD" | "FMOD" => Some(Self::Mod),
            "SPICE_PULSE" => Some(Self::SpicePulse),
            "SPICE_SIN" => Some(Self::SpiceSin),
            "SPICE_EXP" => Some(Self::SpiceExp),
            "SPICE_SFFM" => Some(Self::SpiceSffm),
            "IF" | "TERNARY_FCN" => Some(Self::If),
            _ => None,
        }
    }
}

/// One-dimensional lookup data resolved and precomputed during circuit build.
#[derive(Debug, Clone, PartialEq)]
pub struct LookupTable {
    /// Sorted `(x, y)` pairs used for interpolation.
    pub points: Arc<[(Value, Value)]>,
    /// Interpolation algorithm and any build-time-precomputed coefficients.
    pub interpolation: LookupInterpolation,
    /// Whether the transient integrator should land on each `x` knot.
    pub transient_breakpoints: bool,
}

/// Interpolation algorithm for a build-time-resolved lookup table.
#[derive(Debug, Clone, PartialEq)]
pub enum LookupInterpolation {
    /// Piecewise-linear interpolation.
    Linear,
    /// Natural cubic spline, with one second derivative per knot.
    NaturalCubic { second_derivatives: Arc<[Value]> },
    /// Original Akima spline, with `(p1, p2, p3)` for each interval.
    Akima { coefficients: Arc<[[Value; 3]]> },
    /// Wodicka spline, with `(p1, p2, p3)` for each interval.
    Wodicka { coefficients: Arc<[[Value; 3]]> },
    /// First-form barycentric Lagrange interpolation weights.
    Barycentric { weights: Arc<[Value]> },
}

#[allow(clippy::should_implement_trait)]
impl Expr {
    /// Create a constant expression
    pub fn constant(value: Value) -> Self {
        Expr::Const(value)
    }

    /// Create a node voltage reference
    pub fn voltage(node: impl Into<String>) -> Self {
        Expr::NodeVoltage(node.into())
    }

    /// Create a branch current reference
    pub fn current(element: impl Into<String>) -> Self {
        Expr::BranchCurrent(element.into())
    }

    /// Create an addition
    pub fn add(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a subtraction
    pub fn sub(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Sub,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a multiplication
    pub fn mul(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a division
    pub fn div(left: Expr, right: Expr) -> Self {
        Expr::Binary {
            op: BinaryOp::Div,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a negation
    pub fn neg(operand: Expr) -> Self {
        Expr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(operand),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expr::Binary {
            op: BinaryOp::Sub,
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expr::Binary {
            op: BinaryOp::Div,
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expr::Unary {
            op: UnaryOp::Neg,
            operand: Box::new(self),
        }
    }
}
