//! Exact DC mismatch evidence: one output's spread, and the ranked list of
//! statistical variables that own it.
//!
//! `.DCMATCH` produces no waveform. What it produces is five standard
//! deviations and a ranked table, and every layer that carries a DC mismatch
//! result — the run, the worker wire, the retained payload, the contribution
//! sheet, the CSV, the print — carries *this* value rather than re-spelling
//! the engine's answer in its own shape.
//!
//! The card's own trimming controls travel with the numbers. A list of ten
//! rows out of four hundred evaluated is not the same report as a list of ten
//! out of ten, and neither is a list cut by a share threshold; a reader who
//! cannot see which limits applied cannot tell one from the other, so the
//! limits are part of the evidence.
//!
//! Shares are the engine's Euler allocations and are **signed**. A variable
//! whose declared correlation partner cancels it removes variance from the
//! total, and its share is negative — a statement the design made rather than
//! an error. So nothing here treats a share as a fraction in `[0, 1]`: the
//! invariants are that the shares of an untrimmed list sum to one, that the
//! ranking is by magnitude, and that the trimming was by magnitude too.

/// Which statistical scope declared a contributor's variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DcMismatchScopeEvidence {
    /// A per-instance `mismatch` variable.
    Mismatch,
    /// A design-wide `process` variable, which every instance reads as one
    /// perfectly correlated draw.
    Process,
}

impl DcMismatchScopeEvidence {
    /// The engine's own lower-case tag, which is also the column's text.
    pub fn tag(self) -> &'static str {
        match self {
            Self::Mismatch => "mismatch",
            Self::Process => "process",
        }
    }
}

/// One statistical variable's contribution to the output's DC variance.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcMismatchContributorEvidence {
    /// Instance that drew the variable. A process variable belongs to no
    /// instance and the engine reports it against the design.
    pub instance: String,
    /// Canonical statistical parameter name.
    pub parameter: String,
    pub scope: DcMismatchScopeEvidence,
    /// Standard deviation of the parameter itself.
    pub sigma_parameter: f64,
    /// `d(output)/d(parameter)` at the nominal operating point.
    pub sensitivity: f64,
    /// `sensitivity * sigma_parameter`, in the output's unit. Signed.
    pub contribution: f64,
    /// This variable's Euler allocation of the total variance. Signed: a
    /// negative share reduces the spread through a declared correlation.
    pub share: f64,
}

/// What one `.DCMATCH` run answered.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcMismatchEvidence {
    /// The probe as the engine spelled it back, for example `V(OUT,IN)`.
    pub output: String,
    /// `V` or `A`, decided by the probe rather than authored.
    pub output_unit: String,
    /// The output with every statistical variable at its nominal value.
    pub nominal_value: f64,
    /// The multiple of sigma the card asked the report to quote.
    pub sigma_multiplier: f64,
    /// One-sigma total, before the multiplier.
    pub sigma_total: f64,
    /// The part of the total the per-instance mismatch variables own.
    pub sigma_mismatch: f64,
    /// The part of the total the design-wide process variables own.
    pub sigma_process: f64,
    pub include_mismatch: bool,
    pub include_process: bool,
    /// The card's `CONTRIBUTORS`. Zero is "every contributor".
    pub contributor_limit: u64,
    /// The card's `THRESHOLD`, as a variance-share magnitude.
    pub threshold: f64,
    /// The report basis the run was dispatched with: whether the sheet's bar
    /// column draws the share or the signed contribution. Not an engine
    /// input — the engine always computes both — but the run stated it, so
    /// the result carries it.
    pub normalized_contributions: bool,
    /// `correlate` statements the engine applied to the mismatch variance.
    pub applied_correlations_mismatch: u64,
    /// `correlate` statements the engine applied to the process variance.
    pub applied_correlations_process: u64,
    /// How many contributors the analysis evaluated, including those the
    /// card's limits dropped.
    pub evaluated_contributors: u64,
    /// Retained contributors, in the engine's rank order. Never re-sorted:
    /// the cumulative share below is only meaningful in this order.
    pub contributors: Vec<DcMismatchContributorEvidence>,
}

/// How far apart the variance identity may be and still be the same numbers.
///
/// `sigma_total` is `sqrt(mismatch^2 + process^2)` computed in the engine, so
/// the three fields agree to rounding rather than exactly.
const VARIANCE_IDENTITY_TOLERANCE: f64 = 1.0e-6;

/// How far the shares of an untrimmed list may sum from one.
const SHARE_SUM_TOLERANCE: f64 = 1.0e-9;

impl DcMismatchEvidence {
    /// The spread the card asked to be quoted: `sigma_multiplier *
    /// sigma_total`.
    pub fn quoted_sigma(&self) -> f64 {
        self.sigma_multiplier * self.sigma_total
    }

    /// Retained contributors.
    pub fn retained_contributors(&self) -> u64 {
        self.contributors.len() as u64
    }

    /// True when the card's limits dropped a contributor the analysis
    /// evaluated, which is when the cumulative share stops short of the whole
    /// and the remainder is worth stating.
    pub fn is_trimmed(&self) -> bool {
        self.retained_contributors() < self.evaluated_contributors
    }

    /// Running signed sum of the retained shares, in the engine's order.
    ///
    /// Signed and never renormalized: a correlated contributor can carry the
    /// running total past one and back, and rescaling a trimmed list to end
    /// at one would claim the unlisted contributors carry no variance.
    pub fn cumulative_shares(&self) -> Vec<f64> {
        let mut running = 0.0;
        self.contributors
            .iter()
            .map(|row| {
                running += row.share;
                running
            })
            .collect()
    }

    /// Variance the retained list does not account for, as a signed share.
    ///
    /// `None` for an untrimmed list, where it is zero by construction and
    /// stating it would only invite the reader to doubt the total.
    pub fn unlisted_share(&self) -> Option<f64> {
        if !self.is_trimmed() {
            return None;
        }
        Some(1.0 - self.contributors.iter().map(|row| row.share).sum::<f64>())
    }

    /// The largest contribution magnitude in the list, for the absolute bar's
    /// scale. Zero for an empty list.
    pub fn max_contribution_magnitude(&self) -> f64 {
        self.contributors
            .iter()
            .map(|row| row.contribution.abs())
            .fold(0.0, f64::max)
    }

    /// Refuse a ranking the engine could not have produced.
    ///
    /// Every rule here is a property of `.DCMATCH`'s own answer rather than a
    /// taste: the unit follows the probe, the three sigmas satisfy one
    /// identity, the retained list is ordered by share magnitude and trimmed
    /// by the card's two limits, a scope the card did not select contributes
    /// no row, and an untrimmed list's shares sum to one. A payload that
    /// breaks one of them is not a DC mismatch result, and a sheet drawing it
    /// would present a cumulative share that means nothing.
    pub fn validate(&self) -> Result<(), String> {
        if self.output.trim().is_empty() {
            return Err("DC mismatch evidence has no output probe".to_owned());
        }
        let expected_unit = if self.output.trim_start().starts_with("I(") {
            "A"
        } else {
            "V"
        };
        if self.output_unit != expected_unit {
            return Err(format!(
                "DC mismatch output {} is reported in {} rather than {expected_unit}",
                self.output, self.output_unit
            ));
        }
        if [
            self.nominal_value,
            self.sigma_multiplier,
            self.sigma_total,
            self.sigma_mismatch,
            self.sigma_process,
            self.threshold,
        ]
        .iter()
        .any(|value| !value.is_finite())
        {
            return Err("DC mismatch evidence has a non-finite quantity".to_owned());
        }
        if self.sigma_multiplier <= 0.0 {
            return Err(format!(
                "DC mismatch quotes {} sigma, which is not a positive multiple",
                self.sigma_multiplier
            ));
        }
        if self.sigma_total < 0.0 || self.sigma_mismatch < 0.0 || self.sigma_process < 0.0 {
            return Err("DC mismatch evidence has a negative standard deviation".to_owned());
        }
        if !(0.0..=1.0).contains(&self.threshold) {
            return Err(format!(
                "DC mismatch retained shares above {}, which is not a variance share in [0, 1]",
                self.threshold
            ));
        }
        if !self.include_mismatch && !self.include_process {
            return Err("DC mismatch evidence selects neither statistical scope".to_owned());
        }
        let parts = self.sigma_mismatch.powi(2) + self.sigma_process.powi(2);
        let total = self.sigma_total.powi(2);
        if (total - parts).abs() > VARIANCE_IDENTITY_TOLERANCE * total.max(parts).max(1.0e-300) {
            return Err(format!(
                "DC mismatch total sigma {} is not the mismatch and process parts {} and {}",
                self.sigma_total, self.sigma_mismatch, self.sigma_process
            ));
        }
        if self.retained_contributors() > self.evaluated_contributors {
            return Err(format!(
                "DC mismatch retained {} of {} evaluated contributors",
                self.retained_contributors(),
                self.evaluated_contributors
            ));
        }
        if self.contributor_limit > 0 && self.retained_contributors() > self.contributor_limit {
            return Err(format!(
                "DC mismatch retained {} contributors against a limit of {}",
                self.retained_contributors(),
                self.contributor_limit
            ));
        }

        let mut previous: Option<f64> = None;
        for row in &self.contributors {
            if row.instance.trim().is_empty() {
                return Err("DC mismatch contributor has no instance".to_owned());
            }
            if row.parameter.trim().is_empty() {
                return Err(format!(
                    "DC mismatch contributor {} has no parameter",
                    row.instance
                ));
            }
            if [
                row.sigma_parameter,
                row.sensitivity,
                row.contribution,
                row.share,
            ]
            .iter()
            .any(|value| !value.is_finite())
            {
                return Err(format!(
                    "DC mismatch contributor {} {} has a non-finite quantity",
                    row.instance, row.parameter
                ));
            }
            let selected = match row.scope {
                DcMismatchScopeEvidence::Mismatch => self.include_mismatch,
                DcMismatchScopeEvidence::Process => self.include_process,
            };
            if !selected {
                return Err(format!(
                    "DC mismatch lists a {} contributor the card did not ask for",
                    row.scope.tag()
                ));
            }
            if row.share.abs() < self.threshold {
                return Err(format!(
                    "DC mismatch kept {} {} at share {} under its own threshold {}",
                    row.instance, row.parameter, row.share, self.threshold
                ));
            }
            if previous.is_some_and(|previous| previous < row.share.abs()) {
                return Err(format!(
                    "DC mismatch contributor {} {} is out of the engine's rank order",
                    row.instance, row.parameter
                ));
            }
            previous = Some(row.share.abs());
        }

        // An untrimmed list accounts for the whole variance, and with signs
        // it accounts for it exactly. The one exception is the engine's own:
        // a design whose spreads are all zero has no variance to allocate,
        // and every share is zero.
        if !self.is_trimmed() && self.sigma_total > 0.0 {
            let sum: f64 = self.contributors.iter().map(|row| row.share).sum();
            if (sum - 1.0).abs() > SHARE_SUM_TOLERANCE {
                return Err(format!(
                    "DC mismatch lists every evaluated contributor, but their shares sum to {sum} \
                     rather than one"
                ));
            }
        }
        Ok(())
    }
}
