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
/// A two-port's four scattering coefficients.
const TWO_PORT: &[&str] = &["s11", "s12", "s21", "s22"];

/// A function of a two-port's coefficients, with `optional` trailing
/// arguments after them.
const fn network(
    name: &'static str,
    arguments: &'static [&'static str],
    required: usize,
    summary: &'static str,
) -> FunctionSpec {
    FunctionSpec {
        required,
        ..function(name, &[], Signal, arguments, summary)
    }
}

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
        "clamped between bounds",
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
        "groupDelay",
        &[],
        Signal,
        X,
        "-dφ/dω, the delay of a response",
    ),
    network(
        "kf",
        TWO_PORT,
        4,
        "Rollett stability factor K of a two-port",
    ),
    network(
        "b1f",
        TWO_PORT,
        4,
        "stability measure B1, positive when stable",
    ),
    network(
        "mu",
        TWO_PORT,
        4,
        "load-plane stability factor μ, above 1 when stable",
    ),
    network("mu_prime", TWO_PORT, 4, "source-plane stability factor μ′"),
    network(
        "gt",
        &["s11", "s12", "s21", "s22", "gs", "gl"],
        4,
        "transducer gain at source and load reflections",
    ),
    network(
        "ga",
        &["s11", "s12", "s21", "s22", "gs"],
        4,
        "available gain from a source reflection",
    ),
    network(
        "gp",
        &["s11", "s12", "s21", "s22", "gl"],
        4,
        "operating power gain into a load reflection",
    ),
    network("gmax", TWO_PORT, 4, "MAG when stable, otherwise MSG"),
    network("gmsg", TWO_PORT, 4, "maximum stable gain |S21|/|S12|"),
    network(
        "vswr",
        &["s"],
        1,
        "voltage standing-wave ratio of a reflection",
    ),
    network(
        "zp",
        &["i", "j", "s11", "s12", "s21", "s22", "r1", "r2"],
        6,
        "impedance parameter Zij on port references",
    ),
    network(
        "yp",
        &["i", "j", "s11", "s12", "s21", "s22", "r1", "r2"],
        6,
        "admittance parameter Yij on port references",
    ),
    network(
        "hp",
        &["i", "j", "s11", "s12", "s21", "s22", "r1", "r2"],
        6,
        "hybrid parameter Hij on port references",
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
