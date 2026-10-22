//! The `.ASSERT` card as parsed from a deck.
//!
//! An assertion is a check a deck states about its own result: a real
//! expression held inside a band, a condition held true, or an operating-point
//! quantity of one or more devices held inside a band. It is RSpice's
//! SPICE-dialect card and uses Spectre's parameter names, so Spectre's
//! `assert` maps onto the same statement:
//!
//! ```text
//! .ASSERT <name> EXPR='<real expression>' [MIN=<v>] [MAX=<v>]
//! .ASSERT <name> COND='<condition>'
//! .ASSERT <name> DEV=<inst>|MOD=<model>|SUB=<subckt>|PRIMITIVE=<type> PARAM=<op quantity> [MIN=<v>] [MAX=<v>]
//!    common: [FROM=<t>] [TO=<t>] [DURATION=<t>] [LEVEL=NOTICE|WARNING|ERROR]
//!            [MESSAGE='<text>'] [ANALYSES=op,dc,tran] [XZ=FAIL|IGNORE]
//! ```
//!
//! These types are deck syntax, not results: the evaluator in
//! `analysis::assertions` reads them after an analysis completes. An
//! assertion observes the published result and never changes it, so it is
//! not part of a checkpoint's identity.

use crate::Value;
use crate::source_location::NetlistSourceLocation;

/// What an assertion checks.
#[derive(Debug, Clone, PartialEq)]
pub enum AssertCheck {
    /// `EXPR='<expr>' [MIN=] [MAX=]`: a real expression held inside the
    /// closed band. An absent bound does not constrain.
    Expression {
        expression: String,
        min: Option<Value>,
        max: Option<Value>,
    },
    /// `COND='<condition>'`: a condition held true.
    Condition { condition: String },
    /// `DEV=|MOD=|SUB=|PRIMITIVE= PARAM= [MIN=] [MAX=]`: an operating-point
    /// quantity of every device the target names, each checked on its own.
    Device {
        target: AssertTarget,
        /// The quantity's name as `@inst[param]` reads it, lower case.
        parameter: String,
        min: Option<Value>,
        max: Option<Value>,
    },
}

/// The devices a device-form assertion checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertTarget {
    /// `DEV=`: one instance.
    Instance(String),
    /// `MOD=`: every instance of a model card.
    Model(String),
    /// `SUB=`: every device inside every instance of a subcircuit.
    Subcircuit(String),
    /// `PRIMITIVE=`: every instance of a device family (`mosfet`, `bjt`, ...).
    Primitive(String),
}

/// How a violation is reported.
///
/// `FATAL`, which would stop the run at the violation, is refused: an
/// assertion is evaluated after the analysis, over its accepted result. An HDL
/// `$fatal` is what stops a run live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum AssertLevel {
    /// Reported only.
    Notice,
    /// Reported only. The opt-out from failing a run, as in the HDL.
    #[default]
    Warning,
    /// The run completes and its results are kept, but its verdict fails.
    Error,
}

impl std::fmt::Display for AssertLevel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Notice => "notice",
            Self::Warning => "warning",
            Self::Error => "error",
        })
    }
}

/// An analysis an assertion is evaluated after.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssertAnalysis {
    Op,
    Dc,
    Tran,
}

impl std::fmt::Display for AssertAnalysis {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Op => "op",
            Self::Dc => "dc",
            Self::Tran => "tran",
        })
    }
}

/// One `.ASSERT` card.
#[derive(Debug, Clone, PartialEq)]
pub struct AssertStatement {
    /// The card's name, as written.
    pub name: String,
    pub check: AssertCheck,
    /// `FROM=`: the start of the checked window (time, or the sweep value).
    pub from: Option<Value>,
    /// `TO=`: the end of the checked window.
    pub to: Option<Value>,
    /// `DURATION=`: a violation shorter than this is not reported; one
    /// exactly this long is.
    pub duration: Option<Value>,
    pub level: AssertLevel,
    /// `MESSAGE=`: the text a report carries.
    pub message: Option<String>,
    /// `ANALYSES=`: the analyses the card is evaluated after. Empty means
    /// every analysis that produces its operands.
    pub analyses: Vec<AssertAnalysis>,
    /// `XZ=IGNORE`: a stretch where an operand is X or Z neither passes nor
    /// fails, and is counted instead. By default (`XZ=FAIL`) it fails.
    pub ignore_unknown: bool,
    /// Where the card was written.
    pub origin: NetlistSourceLocation,
}
