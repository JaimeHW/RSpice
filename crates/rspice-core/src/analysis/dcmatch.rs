//! DC mismatch variance: what one output's spread is, and which devices own
//! it.
//!
//! `.DCMATCH` answers a question a Monte Carlo sweep answers only in the
//! limit: at the nominal DC operating point, how much does an output move
//! because of the design's own declared statistical variation, and which
//! instance and which variable is responsible for how much of it.
//!
//! It is a linearization, not a sampling. For every statistical variable in
//! scope the analysis takes the derivative of the output with respect to that
//! variable and multiplies it by the variable's standard deviation; the
//! variance of the output is the quadratic form `c^T R c` over those signed
//! products, where `R` is the population correlation between variables drawn
//! together. With bounds, the conditional covariance includes changes in both
//! spread and correlation, including unbounded partners in the same group. Mismatch
//! variables of different instances are independent by construction and a
//! process variable is one variable the whole design shares, so a design that
//! correlates nothing gets the plain sum of squares. Each contributor's share
//! is its Euler allocation `c_i * (R c)_i` over that total, and the shares are
//! what tells a designer which device to make bigger.
//!
//! Every standard deviation comes from the deck's own
//! `statistics { process { vary ... } mismatch { vary ... } }` block. There is
//! no default spread and no assumed distribution: a design that declares no
//! statistics is refused by name.

use crate::Value;

/// One statistical variable's contribution to an output's DC variance.
#[derive(Debug, Clone, PartialEq)]
pub struct DcMatchContributor {
    /// Instance the variable belongs to. A mismatch variable names the
    /// concrete instance that drew it; a process variable names the design,
    /// because every instance reads the same one.
    pub instance: String,
    /// Canonical (upper-case) statistical parameter name.
    pub parameter: String,
    /// Which statistical scope declared the variable.
    pub scope: DcMatchScope,
    /// Standard deviation of the parameter itself.
    pub sigma_parameter: Value,
    /// `d(output)/d(parameter)` at the nominal operating point.
    pub sensitivity: Value,
    /// `sensitivity * sigma_parameter`: the output displacement one standard
    /// deviation of this variable produces. Signed, so a reader can tell
    /// which way the output moves.
    pub contribution: Value,
    /// This contributor's Euler allocation of the total variance,
    /// `contribution * (R * contributions)_i / sigma_total^2`, which is
    /// `contribution^2 / sigma_total^2` when nothing is correlated. Zero when
    /// the total variance is zero, which is the only case where the shares do
    /// not sum to one.
    ///
    /// Signed. A negative share is a variable whose correlated partner cancels
    /// it — it removes variance from the total rather than adding to it — and
    /// is a statement the design made, not an error. Rank and threshold on the
    /// magnitude.
    pub share: Value,
}

/// Which statistical scope a `.DCMATCH` contributor came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DcMatchScope {
    /// A design-wide `process` variable, perfectly correlated across every
    /// instance that reads it.
    Process,
    /// A per-instance `mismatch` variable, independent between instances.
    Mismatch,
}

impl DcMatchScope {
    /// Stable lower-case tag used by result documents and diagnostics.
    pub const fn tag(self) -> &'static str {
        match self {
            Self::Process => "process",
            Self::Mismatch => "mismatch",
        }
    }
}

/// What one authored `.DCMATCH` card produced.
#[derive(Debug, Clone, PartialEq)]
pub struct DcMatchResult {
    /// The probe as the deck spelled it, for example `V(out,in)` or `I(V1)`.
    pub output: String,
    /// The output at the nominal operating point, every statistical variable
    /// at its nominal value.
    pub nominal_value: Value,
    /// The multiple of sigma the card asked the report to quote.
    pub sigma_multiplier: Value,
    /// Total output standard deviation, `sqrt(sigma_mismatch^2 +
    /// sigma_process^2)`, before the card's multiplier is applied.
    pub sigma_total: Value,
    /// The part of `sigma_total` the per-instance mismatch variables own.
    pub sigma_mismatch: Value,
    /// The part of `sigma_total` the design-wide process variables own.
    pub sigma_process: Value,
    /// Contributors retained by the card's `CONTRIBUTORS` and `THRESHOLD`
    /// limits, largest share first by magnitude — a correlated share carries a
    /// sign, and both limits read the magnitude.
    pub contributors: Vec<DcMatchContributor>,
    /// How many contributors the analysis evaluated, including any the
    /// card's limits dropped from `contributors`.
    pub evaluated_contributors: usize,
    /// How many `correlate` statements of the design's `statistics` block
    /// entered the mismatch variance. Zero means the mismatch variables were
    /// summed as independent — either because the block declares no mismatch
    /// correlation or because the card did not ask for the scope.
    pub applied_correlations_mismatch: usize,
    /// How many `correlate` statements entered the process variance, on the
    /// same terms.
    pub applied_correlations_process: usize,
}

impl DcMatchResult {
    /// The total standard deviation the card asked to be quoted:
    /// `sigma_multiplier * sigma_total`.
    pub fn quoted_sigma(&self) -> Value {
        self.sigma_multiplier * self.sigma_total
    }
}
