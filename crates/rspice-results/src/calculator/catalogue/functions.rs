//! The functions that are not measurements: each maps a value, or a trace
//! sample by sample or along its x-axis, to another value or trace.
//!
//! This table and [`super::MEASUREMENTS`] together are every function the
//! evaluator runs: a name in neither is refused as unknown, so the reference
//! that lists them cannot offer a function the evaluator lacks, nor the
//! evaluator run one the reference hides.

/// Where a function belongs in the reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionGroup {
    /// Arithmetic on values: roots, logarithms, trigonometry, choices.
    Math,
    /// A signal along its x-axis, its complex projections, and conversions
    /// between analog and logic.
    Signal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionSpec {
    /// The name as the reference spells it; the evaluator ignores case.
    pub name: &'static str,
    /// Other spellings the evaluator accepts for the same function.
    pub aliases: &'static [&'static str],
    pub group: FunctionGroup,
    /// The arguments' names in a signature, in order.
    pub arguments: &'static [&'static str],
    /// How many leading arguments a call must give; the rest may be left off.
    pub required: usize,
    /// A few words for a reference row.
    pub summary: &'static str,
}

const fn function(
    name: &'static str,
    aliases: &'static [&'static str],
    group: FunctionGroup,
    arguments: &'static [&'static str],
    summary: &'static str,
) -> FunctionSpec {
    FunctionSpec {
        name,
        aliases,
        group,
        arguments,
        required: arguments.len(),
        summary,
    }
}

use FunctionGroup::{Math, Signal};

const X: &[&str] = &["x"];

pub const FUNCTIONS: &[FunctionSpec] = &[
    function(
        "complex",
        &[],
        Math,
        &["re", "im"],
        "rectangular complex value",
    ),
    function("abs", &[], Math, X, "absolute value"),
    function("sqrt", &[], Math, X, "square root"),
    function("exp", &[], Math, X, "exponential"),
    function("log", &["ln"], Math, X, "natural log"),
    function("log10", &[], Math, X, "log base 10"),
    function("sin", &[], Math, X, "sine of radians"),
    function("cos", &[], Math, X, "cosine of radians"),
    function("tan", &[], Math, X, "tangent of radians"),
    function("asin", &[], Math, X, "inverse sine, in radians"),
    function("acos", &[], Math, X, "inverse cosine, in radians"),
    function("atan", &[], Math, X, "inverse tangent, in radians"),
    function("sinh", &[], Math, X, "hyperbolic sine"),
    function("cosh", &[], Math, X, "hyperbolic cosine"),
    function("tanh", &[], Math, X, "hyperbolic tangent"),
    function("asinh", &[], Math, X, "inverse hyperbolic sine"),
    function("acosh", &[], Math, X, "inverse hyperbolic cosine"),
    function("atanh", &[], Math, X, "inverse hyperbolic tangent"),
    function(
        "limit",
        &[],
        Math,
        &["x", "lo", "hi"],
        "each value held between two bounds",
    ),
    function(
        "if",
        &[],
        Math,
        &["cond", "a", "b"],
        "a where cond is above 0.5, else b",
    ),
    function("mag", &["magnitude"], Signal, X, "complex magnitude"),
    function("real", &["re"], Signal, X, "real component"),
    function("imag", &["im"], Signal, X, "imaginary component"),
    function(
        "phase",
        &["phase_deg"],
        Signal,
        X,
        "phase in degrees; undefined at zero",
    ),
    function("phase_rad", &[], Signal, X, "phase in radians"),
    function("unwrap", &[], Signal, X, "continuous phase in degrees"),
    function("conj", &[], Signal, X, "complex conjugate"),
    function("dB", &["dB20"], Signal, X, "20·log₁₀|x|, an amplitude"),
    function("dB10", &[], Signal, X, "10·log₁₀|x|, a power"),
    function(
        "deriv",
        &["derivative"],
        Signal,
        X,
        "slope along the x-axis",
    ),
    function(
        "integ",
        &["integral"],
        Signal,
        X,
        "running integral along the x-axis",
    ),
    function(
        "clip",
        &[],
        Signal,
        &["x", "from", "to"],
        "the part between two x points",
    ),
    function("xval", &[], Signal, X, "the x-axis as a trace"),
    FunctionSpec {
        required: 2,
        ..function(
            "digital",
            &[],
            Signal,
            &["x", "vth", "vtl", "tx"],
            "logic levels read at thresholds",
        )
    },
    function(
        "analog",
        &[],
        Signal,
        X,
        "a logic signal or bus word as a value",
    ),
];

/// The table's entry the evaluator would run for `name`, by its own spelling
/// or an alias, ignoring case.
pub fn find_function(name: &str) -> Option<&'static FunctionSpec> {
    FUNCTIONS.iter().find(|spec| {
        spec.name.eq_ignore_ascii_case(name)
            || spec
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(name))
    })
}

impl FunctionSpec {
    /// The full signature: `digital(x, vth, vtl, tx)`.
    pub fn signature(&self) -> String {
        format!("{}({})", self.name, self.arguments.join(", "))
    }

    /// The signature with its optional arguments folded: `digital(x, vth, …)`.
    pub fn short_signature(&self) -> String {
        let folded = if self.required < self.arguments.len() {
            ", …"
        } else {
            ""
        };
        format!(
            "{}({}{folded})",
            self.name,
            self.arguments[..self.required].join(", ")
        )
    }

    /// The call a reference row inserts: the required argument slots, empty.
    pub fn template(&self) -> String {
        format!("{}({})", self.name, vec![""; self.required].join(", "))
    }
}
