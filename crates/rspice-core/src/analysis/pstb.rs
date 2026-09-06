//! Periodic Stability (PSTB) Analysis Module
//!
//! PSTB analysis evaluates the stability of periodic steady-state solutions
//! using Floquet theory. This is critical for:
//!
//! - **Oscillator design**: Verify stable oscillation and startup conditions
//! - **PLL stability**: Check loop stability around limit cycles
//! - **Bifurcation detection**: Find parameter values where stability changes
//! - **Parametric oscillation**: Detect unintended oscillation modes
//!
//! # Theory
//!
//! For a periodic orbit x*(t) with period T, small perturbations δx(t) evolve as:
//!
//! δx(t) ≈ Φ(t,0) · δx(0)
//!
//! where Φ(t,0) is the state transition (Monodromy) matrix. The eigenvalues of
//! Φ(T,0) are called **Floquet multipliers** (λ). Stability is determined by:
//!
//! - |λ| < 1 for all λ (except one λ = 1 for autonomous systems): Stable
//! - |λ| > 1 for any λ: Unstable
//! - |λ| = 1 (beyond the trivial one): Marginally stable / bifurcation point
//!
//! # Floquet Exponents
//!
//! Floquet exponents μ relate to multipliers as: λ = exp(μ·T)
//!
//! For oscillators, one μ = 0 always (corresponding to phase shifts along orbit).
//! The other exponents determine amplitude stability.

use crate::abort_signal::AbortSignal;
use crate::analysis::{
    FloquetOrbitKind, FloquetSpectrumEvidence, FloquetStabilityVerdict, classify_floquet_stability,
    select_autonomous_phase_mode,
};
use crate::numerics::eigenspectrum::{OrdinarySpectrumError, qualified_real_eigenspectrum};
use crate::{SimulationError, Value};
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// PSTB Configuration
//=============================================================================

/// Configuration for Periodic Stability (PSTB) analysis
#[derive(Debug, Clone)]
pub struct PstbConfig {
    /// Explicit policy for driven versus autonomous periodic orbits.
    pub orbit_kind: FloquetOrbitKind,

    /// Compatibility field. PSTB always retains the complete spectrum because
    /// a truncated spectrum cannot prove stability.
    pub num_eigenvalues: usize,

    /// Compatibility field. Strict qualification always uses the canonical
    /// `128 * n * EPSILON` backward-error bound.
    pub eigenvalue_tolerance: Value,

    /// Compatibility field. The faer eigensolve is atomic and does not expose
    /// an iteration limit.
    pub max_iterations: usize,

    /// Compatibility field. Exponents remain populated to preserve the public
    /// result schema; callers may ignore them when this is false.
    pub compute_exponents: bool,

    /// Whether to compute eigenvectors (mode shapes)
    pub compute_eigenvectors: bool,

    /// Outer numerical band around the physical `|λ| = 1` boundary.
    /// Values must be finite and at least one. The symmetric inner bound is
    /// `max(0, 2 - stability_threshold)`; only roots below it are stable.
    pub stability_threshold: Value,

    /// Enable subharmonic detection (multipliers at roots of unity).
    pub detect_subharmonics: bool,

    /// Numerical perturbation for finite difference Monodromy computation
    pub fd_perturbation: Value,
}

impl Default for PstbConfig {
    fn default() -> Self {
        Self {
            orbit_kind: FloquetOrbitKind::Driven,
            num_eigenvalues: 0, // Compute all
            eigenvalue_tolerance: 1e-10,
            max_iterations: 1000,
            compute_exponents: true,
            compute_eigenvectors: false,
            stability_threshold: 1.0 + 1e-6, // Allow small margin for numerical error
            detect_subharmonics: true,
            fd_perturbation: 1e-8,
        }
    }
}

impl PstbConfig {
    /// Create new PSTB configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set number of eigenvalues to compute
    pub fn with_num_eigenvalues(mut self, n: usize) -> Self {
        self.num_eigenvalues = n;
        self
    }

    /// Set the periodic-orbit policy used for unity-mode classification.
    pub fn with_orbit_kind(mut self, orbit_kind: FloquetOrbitKind) -> Self {
        self.orbit_kind = orbit_kind;
        self
    }

    /// Enable/disable eigenvector computation
    pub fn with_eigenvectors(mut self, compute: bool) -> Self {
        self.compute_eigenvectors = compute;
        self
    }

    /// Set stability threshold
    pub fn with_stability_threshold(mut self, threshold: Value) -> Self {
        self.stability_threshold = threshold;
        self
    }

    /// Enable/disable subharmonic detection
    pub fn with_subharmonic_detection(mut self, detect: bool) -> Self {
        self.detect_subharmonics = detect;
        self
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, tol: Value) -> Self {
        self.eigenvalue_tolerance = tol;
        self
    }
}

//=============================================================================
// Floquet Multiplier
//=============================================================================

/// A Floquet multiplier with associated metadata
#[derive(Debug, Clone)]
pub struct FloquetMultiplier {
    /// Complex eigenvalue (Floquet multiplier)
    pub value: Complex64,

    /// Floquet exponent: μ = ln(λ)/T.
    ///
    /// Exact-zero multipliers are rejected because a finite-dimensional
    /// continuous-time state-transition map must be nonsingular.
    pub exponent: Complex64,

    /// Index in sorted order (0 = most unstable)
    pub index: usize,

    /// Whether this multiplier indicates instability
    pub is_unstable: bool,

    /// Whether this is the one explicitly selected autonomous phase mode.
    pub is_trivial: bool,

    /// Subharmonic order if |λ| ≈ 1 and λ is near a root of unity
    pub subharmonic_order: Option<usize>,

    /// Associated eigenvector (if computed)
    pub eigenvector: Option<Vec<Complex64>>,
}

impl FloquetMultiplier {
    /// Create a new Floquet multiplier
    pub fn new(value: Complex64, period: Value, stability_threshold: Value) -> Self {
        let exponent = value.ln() / period;
        let magnitude = value.norm();

        // Check for subharmonic (λ near n-th root of unity)
        let subharmonic_order = Self::detect_subharmonic(&value);

        Self {
            value,
            exponent,
            index: 0,
            is_unstable: magnitude > stability_threshold,
            is_trivial: false,
            subharmonic_order,
            eigenvector: None,
        }
    }

    /// Detect if multiplier is near a root of unity (subharmonic)
    fn detect_subharmonic(value: &Complex64) -> Option<usize> {
        let mag = value.norm();
        if (mag - 1.0).abs() > 0.01 {
            return None; // Not on unit circle
        }

        let angle = value.arg().abs();

        // Check for n-th roots of unity (n = 2, 3, 4, ...)
        for n in 2..=8 {
            let expected_angle = 2.0 * PI / (n as f64);
            if (angle - expected_angle).abs() < 0.01 {
                return Some(n);
            }
        }

        None
    }

    /// Get magnitude of multiplier
    pub fn magnitude(&self) -> Value {
        self.value.norm()
    }

    /// Get phase angle in radians
    pub fn phase(&self) -> Value {
        self.value.arg()
    }

    /// Get phase angle in degrees
    pub fn phase_degrees(&self) -> Value {
        self.value.arg() * 180.0 / PI
    }

    /// Get damping factor (negative real part of exponent)
    pub fn damping(&self) -> Value {
        -self.exponent.re
    }

    /// Get natural frequency from exponent
    pub fn natural_frequency(&self) -> Value {
        self.exponent.im.abs() / (2.0 * PI)
    }

    /// Stability margin in dB (20·log10(1/|λ|)).
    pub fn stability_margin_db(&self) -> Value {
        -20.0 * self.magnitude().log10()
    }
}

//=============================================================================
// Stability Classification
//=============================================================================

/// Classification of periodic orbit stability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StabilityType {
    /// All multipliers are strictly inside the configured inner band.
    Stable,

    /// At least one multiplier outside unit circle with positive real part
    UnstableReal,

    /// Pair of complex multipliers outside unit circle
    UnstableComplex,

    /// Multiplier at λ = -1 (period-doubling bifurcation)
    PeriodDoubling,

    /// Complex pair on unit circle (Neimark-Sacker/torus bifurcation)
    NeimarkSacker,

    /// Multiplier at λ = +1 (saddle-node bifurcation)
    SaddleNode,

    /// Marginal stability (numerical uncertainty)
    Marginal,

    /// Stability could not be established from consistent qualified evidence.
    Indeterminate,
}

impl StabilityType {
    /// Check if this is a stable classification
    pub fn is_stable(&self) -> bool {
        matches!(self, StabilityType::Stable)
    }
}

//=============================================================================
// PSTB Result
//=============================================================================

/// Result of Periodic Stability (PSTB) analysis
#[derive(Debug, Clone)]
pub struct PstbResult {
    /// Period of the analyzed orbit
    pub period: Value,

    /// Fundamental frequency (1/period)
    pub fundamental_frequency: Value,

    /// All computed Floquet multipliers (sorted by magnitude, descending)
    pub multipliers: Vec<FloquetMultiplier>,

    /// Strict completeness and residual evidence for the multiplier vector.
    pub floquet_evidence: FloquetSpectrumEvidence,

    /// Driven/autonomous orbit policy applied during classification.
    pub orbit_kind: FloquetOrbitKind,

    /// One explicitly selected autonomous phase-mode index in sorted order.
    pub trivial_multiplier_index: Option<usize>,

    /// Shared evidence-aware stability verdict.
    pub stability_verdict: FloquetStabilityVerdict,

    /// Overall stability classification
    pub stability: StabilityType,

    /// Monodromy matrix (state transition over one period)
    pub monodromy: Vec<Vec<Value>>,

    /// Number of unstable multipliers
    pub num_unstable: usize,

    /// Minimum signed stability margin among applicable non-phase modes.
    /// `None` means there is no applicable mode (for example, a zero-order
    /// driven map or an autonomous spectrum containing only its phase mode).
    pub min_stability_margin_db: Option<Value>,

    /// Maximum multiplier magnitude
    pub max_multiplier_magnitude: Value,

    /// Detected subharmonics (if any)
    pub subharmonics: Vec<usize>,

    /// Exact outer magnitude boundary this spectrum was classified under.
    ///
    /// Without it the result cannot say what it means: [`FloquetMultiplier::
    /// is_unstable`] is defined relative to a boundary the result did not
    /// carry, so every consumer had to be told the number separately and
    /// trusted to have been told the same one the analyzer used.
    pub stability_threshold: Value,

    /// Whether subharmonic orders were retained.
    ///
    /// An empty [`Self::subharmonics`] is otherwise ambiguous between "no mode
    /// sits near a root of unity" and "nobody looked".
    pub detect_subharmonics: bool,

    /// Whether the analysis converged
    pub converged: bool,

    /// Number of iterations used. Zero means the atomic eigensolver did not
    /// expose an iteration count.
    pub iterations: usize,
}

impl PstbResult {
    /// Check if the periodic orbit is stable
    pub fn is_stable(&self) -> bool {
        self.stability_verdict == FloquetStabilityVerdict::Stable
    }

    /// Re-derive every quantity this result publishes and refuse the result if
    /// any of them disagrees with the spectrum it was derived from.
    ///
    /// These are the result's own invariants, not a consumer's. Before this
    /// existed, one caller — the Studio's PSTB service — re-derived all of
    /// them and refused the result when they disagreed, which meant the
    /// command line, python and the browser shipped a *weaker* guarantee than
    /// the GUI for the same numbers. The analyzer establishes them here
    /// instead, so every route inherits the same refusal.
    ///
    /// What is checked: the period/frequency pair, the classification boundary
    /// and the convergence flag; Floquet evidence currency and consistency
    /// with the retained multipliers; autonomous phase-mode selection; the
    /// verdict and the rich classification it refines; canonical sort order;
    /// monodromy squareness and finiteness against the spectrum's order; the
    /// four aggregates; and per-mode identity, finiteness, flags, derived
    /// display quantities and eigenvector cardinality.
    pub(crate) fn validate_contract(&self, abort: &dyn AbortSignal) -> Result<(), SimulationError> {
        ensure_not_aborted(abort)?;
        if !self.period.is_finite()
            || self.period <= 0.0
            || !self.fundamental_frequency.is_finite()
            || self.fundamental_frequency <= 0.0
            || self.fundamental_frequency != 1.0 / self.period
        {
            return Err(contract_violation(
                "period and fundamental frequency are not a finite reciprocal pair",
            ));
        }
        if !self.stability_threshold.is_finite() || self.stability_threshold < 1.0 {
            return Err(contract_violation(
                "the retained stability boundary is not a finite magnitude of at least one",
            ));
        }
        if !self.converged {
            return Err(contract_violation(
                "the spectrum is published as converged only when the qualified eigensolve completed",
            ));
        }

        let values = self
            .multipliers
            .iter()
            .map(|multiplier| multiplier.value)
            .collect::<Vec<_>>();
        if !matches!(
            &self.floquet_evidence,
            FloquetSpectrumEvidence::NoDynamicModes | FloquetSpectrumEvidence::Qualified { .. }
        ) || !self.floquet_evidence.is_consistent_with(&values)
        {
            return Err(contract_violation(
                "Floquet evidence is absent, non-current, or inconsistent with the spectrum",
            ));
        }

        let expected_trivial_index = if self.orbit_kind == FloquetOrbitKind::Autonomous
            && matches!(
                &self.floquet_evidence,
                FloquetSpectrumEvidence::Qualified { .. }
            ) {
            select_autonomous_phase_mode(&values)
        } else {
            None
        };
        if self.trivial_multiplier_index != expected_trivial_index {
            return Err(contract_violation(
                "autonomous phase-mode selection is inconsistent with the spectrum",
            ));
        }

        let expected_verdict = classify_floquet_stability(
            &values,
            &self.floquet_evidence,
            self.orbit_kind,
            self.trivial_multiplier_index,
            self.stability_threshold - 1.0,
        );
        if self.stability_verdict != expected_verdict
            || !classification_refines_verdict(self.stability_verdict, self.stability)
        {
            return Err(contract_violation(
                "the stability verdict or the classification refining it is inconsistent with the spectrum",
            ));
        }
        if !multipliers_are_sorted(&self.multipliers) {
            return Err(contract_violation(
                "Floquet modes are not in canonical sorted order",
            ));
        }

        let order = self.multipliers.len();
        if self.monodromy.len() != order {
            return Err(contract_violation(
                "the retained monodromy order does not match the spectrum it produced",
            ));
        }
        for (row_index, row) in self.monodromy.iter().enumerate() {
            poll_periodically(abort, row_index)?;
            if row.len() != order || row.iter().any(|value| !value.is_finite()) {
                return Err(contract_violation(
                    "the retained monodromy is not a finite square matrix of the spectrum's order",
                ));
            }
        }

        for (index, multiplier) in self.multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            let magnitude = multiplier.magnitude();
            let trivial = self.trivial_multiplier_index == Some(index);
            let natural_frequency = multiplier.natural_frequency();
            if multiplier.index != index
                || !multiplier.value.re.is_finite()
                || !multiplier.value.im.is_finite()
                || !multiplier.exponent.re.is_finite()
                || !multiplier.exponent.im.is_finite()
                || !magnitude.is_finite()
                || magnitude <= 0.0
                || multiplier.is_trivial != trivial
                || multiplier.is_unstable != (!trivial && magnitude > self.stability_threshold)
                || (!self.detect_subharmonics && multiplier.subharmonic_order.is_some())
                // A caller that did not ask for eigenvectors gets none; one
                // that did gets a full set. A vector of the wrong length is
                // neither, and would index a mode shape onto the wrong state.
                || multiplier
                    .eigenvector
                    .as_ref()
                    .is_some_and(|vector| vector.len() != order)
                || !multiplier.phase_degrees().is_finite()
                || !multiplier.damping().is_finite()
                || !natural_frequency.is_finite()
                || natural_frequency < 0.0
                || !multiplier.stability_margin_db().is_finite()
            {
                return Err(SimulationError::Circuit(format!(
                    "PSTB mode {} violates its own contract: identity, finite values, flags, \
                     derived quantities, or eigenvector cardinality is invalid",
                    index + 1
                )));
            }
        }

        let expected_num_unstable = self
            .multipliers
            .iter()
            .filter(|multiplier| multiplier.is_unstable)
            .count();
        let expected_max_magnitude = self
            .multipliers
            .first()
            .map_or(0.0, FloquetMultiplier::magnitude);
        let expected_min_margin = self
            .multipliers
            .iter()
            .enumerate()
            .filter(|(index, _)| self.trivial_multiplier_index != Some(*index))
            .map(|(_, multiplier)| multiplier.stability_margin_db())
            .min_by(Value::total_cmp);
        let expected_subharmonics = self
            .multipliers
            .iter()
            .filter_map(|multiplier| multiplier.subharmonic_order)
            .collect::<Vec<_>>();
        if self.num_unstable != expected_num_unstable
            || !self.max_multiplier_magnitude.is_finite()
            || self.max_multiplier_magnitude != expected_max_magnitude
            || self
                .min_stability_margin_db
                .is_some_and(|margin| !margin.is_finite())
            || self.min_stability_margin_db != expected_min_margin
            || self.subharmonics != expected_subharmonics
        {
            return Err(contract_violation(
                "aggregate counts, margins, or subharmonics do not match the complete spectrum",
            ));
        }
        Ok(())
    }
}

fn contract_violation(detail: &str) -> SimulationError {
    SimulationError::Circuit(format!("PSTB result violates its own contract: {detail}"))
}

/// Whether the rich PSTB classification is one of the labels the shared
/// four-state verdict admits.
///
/// The two are not independent: every [`StabilityType`] belongs to exactly one
/// [`FloquetStabilityVerdict`], so publishing the classification publishes the
/// verdict. This is the partition, stated once.
fn classification_refines_verdict(
    verdict: FloquetStabilityVerdict,
    classification: StabilityType,
) -> bool {
    match verdict {
        FloquetStabilityVerdict::Stable => classification == StabilityType::Stable,
        FloquetStabilityVerdict::Unstable => matches!(
            classification,
            StabilityType::UnstableReal | StabilityType::UnstableComplex
        ),
        FloquetStabilityVerdict::Marginal => matches!(
            classification,
            StabilityType::PeriodDoubling
                | StabilityType::NeimarkSacker
                | StabilityType::SaddleNode
                | StabilityType::Marginal
        ),
        FloquetStabilityVerdict::Indeterminate => classification == StabilityType::Indeterminate,
    }
}

/// Whether the retained modes are in the analyzer's canonical order:
/// magnitude descending, then real part, then imaginary part.
fn multipliers_are_sorted(multipliers: &[FloquetMultiplier]) -> bool {
    multipliers.windows(2).all(|pair| {
        pair[1]
            .magnitude()
            .total_cmp(&pair[0].magnitude())
            .then_with(|| pair[0].value.re.total_cmp(&pair[1].value.re))
            .then_with(|| pair[0].value.im.total_cmp(&pair[1].value.im))
            .is_le()
    })
}

//=============================================================================
// PSTB Analyzer
//=============================================================================

/// Periodic Stability (PSTB) Analyzer
///
/// Analyzes stability of periodic orbits using Floquet theory.
#[derive(Debug)]
pub struct PstbAnalyzer {
    /// Configuration
    config: PstbConfig,

    /// State dimension
    dimension: usize,
}

impl PstbAnalyzer {
    /// Create a new PSTB analyzer
    pub fn new(config: PstbConfig) -> Self {
        Self {
            config,
            dimension: 0,
        }
    }

    /// Analyze stability from a pre-computed Monodromy matrix, cooperatively
    /// returning [`SimulationError::Aborted`] when cancellation is requested.
    ///
    /// Cancellation is polled throughout matrix preparation, conversion,
    /// residual qualification, and result conversion, and immediately before
    /// and after the atomic eigensolve.
    pub fn analyze_monodromy_with_abort(
        &mut self,
        monodromy: &[Vec<Value>],
        period: Value,
        abort: &dyn AbortSignal,
    ) -> Result<PstbResult, SimulationError> {
        ensure_not_aborted(abort)?;
        if !period.is_finite() || period <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "PSTB requires a finite positive period, got {period}"
            )));
        }
        let fundamental_frequency = 1.0 / period;
        if !fundamental_frequency.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "PSTB period is too small to form a finite fundamental frequency: {period}"
            )));
        }
        if !self.config.stability_threshold.is_finite() || self.config.stability_threshold < 1.0 {
            return Err(SimulationError::Circuit(format!(
                "PSTB requires a finite stability threshold greater than or equal to one, got {}",
                self.config.stability_threshold
            )));
        }

        if monodromy.is_empty() {
            self.dimension = 0;
            let floquet_evidence = FloquetSpectrumEvidence::NoDynamicModes;
            let stability_verdict = classify_floquet_stability(
                &[],
                &floquet_evidence,
                self.config.orbit_kind,
                None,
                self.config.stability_threshold - 1.0,
            );
            let stability = if stability_verdict == FloquetStabilityVerdict::Stable {
                StabilityType::Stable
            } else {
                StabilityType::Indeterminate
            };
            let result = PstbResult {
                period,
                fundamental_frequency,
                multipliers: Vec::new(),
                floquet_evidence,
                orbit_kind: self.config.orbit_kind,
                trivial_multiplier_index: None,
                stability_verdict,
                stability,
                monodromy: Vec::new(),
                num_unstable: 0,
                min_stability_margin_db: None,
                max_multiplier_magnitude: 0.0,
                subharmonics: Vec::new(),
                stability_threshold: self.config.stability_threshold,
                detect_subharmonics: self.config.detect_subharmonics,
                converged: true,
                iterations: 0,
            };
            result.validate_contract(abort)?;
            return Ok(result);
        }

        let spectrum = qualified_real_eigenspectrum(monodromy, abort)
            .map_err(map_eigenspectrum_error_for_pstb)?;
        let n = spectrum.certificate.problem_order;
        debug_assert!(
            spectrum.certificate.max_backward_error <= spectrum.certificate.qualification_tolerance
        );
        self.dimension = n;
        let floquet_evidence = FloquetSpectrumEvidence::qualified(spectrum.certificate)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "PSTB eigensolver returned an invalid Floquet certificate".to_owned(),
                )
            })?;

        // Create FloquetMultiplier objects
        let mut multipliers = Vec::with_capacity(spectrum.eigenvalues.len());
        for (i, &ev) in spectrum.eigenvalues.iter().enumerate() {
            poll_periodically(abort, i)?;
            if ev.re == 0.0 && ev.im == 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "PSTB Floquet multiplier {i} is exactly zero; a continuous-time state-transition map must be nonsingular"
                )));
            }
            let mut fm = FloquetMultiplier::new(ev, period, self.config.stability_threshold);
            // A run that was told not to look for subharmonics does not report
            // finding one. Masking here rather than at each display keeps the
            // per-mode order and the aggregate from disagreeing about whether
            // anybody looked.
            if !self.config.detect_subharmonics {
                fm.subharmonic_order = None;
            }
            if !fm.exponent.re.is_finite() || !fm.exponent.im.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "PSTB Floquet exponent {i} is non-finite"
                )));
            }
            fm.index = i;
            if self.config.compute_eigenvectors {
                fm.eigenvector = Some(spectrum.right_eigenvectors[i].clone());
            }
            multipliers.push(fm);
        }

        // Sort by magnitude (most unstable first)
        ensure_not_aborted(abort)?;
        multipliers.sort_by(|a, b| {
            b.magnitude()
                .total_cmp(&a.magnitude())
                .then_with(|| a.value.re.total_cmp(&b.value.re))
                .then_with(|| a.value.im.total_cmp(&b.value.im))
        });
        ensure_not_aborted(abort)?;

        // Re-index after sorting
        for (i, m) in multipliers.iter_mut().enumerate() {
            poll_periodically(abort, i)?;
            m.index = i;
        }

        let multiplier_values = multipliers
            .iter()
            .map(|multiplier| multiplier.value)
            .collect::<Vec<_>>();
        let trivial_multiplier_index = if self.config.orbit_kind == FloquetOrbitKind::Autonomous {
            select_autonomous_phase_mode(&multiplier_values)
        } else {
            None
        };
        if let Some(index) = trivial_multiplier_index {
            multipliers[index].is_trivial = true;
            multipliers[index].is_unstable = false;
        }

        let classification_band = self.config.stability_threshold - 1.0;
        let stability_verdict = classify_floquet_stability(
            &multiplier_values,
            &floquet_evidence,
            self.config.orbit_kind,
            trivial_multiplier_index,
            classification_band,
        );

        // Classify stability
        let stability = self.classify_stability_with_abort(
            &multipliers,
            stability_verdict,
            trivial_multiplier_index,
            abort,
        )?;

        // Count unstable and compute margins
        let mut num_unstable = 0;
        for (index, multiplier) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if multiplier.is_unstable {
                num_unstable += 1;
            }
        }
        let max_magnitude = multipliers.first().map(|m| m.magnitude()).unwrap_or(0.0);

        let mut min_margin_db: Option<Value> = None;
        for (index, multiplier) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if trivial_multiplier_index == Some(index) {
                continue;
            }
            let margin = multiplier.stability_margin_db();
            min_margin_db = Some(match min_margin_db {
                Some(current) if current.total_cmp(&margin).is_le() => current,
                _ => margin,
            });
        }

        // Collect subharmonics. The per-mode order was already cleared above
        // when detection is off, so the aggregate follows the modes rather
        // than re-reading the configuration and risking the two disagreeing.
        let mut subharmonics = Vec::new();
        for (index, multiplier) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if let Some(order) = multiplier.subharmonic_order {
                subharmonics.push(order);
            }
        }

        let mut monodromy_copy = Vec::with_capacity(monodromy.len());
        let mut flat_index = 0;
        for row in monodromy {
            let mut copied_row = Vec::with_capacity(row.len());
            for &value in row {
                poll_periodically(abort, flat_index)?;
                flat_index += 1;
                copied_row.push(value);
            }
            monodromy_copy.push(copied_row);
        }
        ensure_not_aborted(abort)?;

        let result = PstbResult {
            period,
            fundamental_frequency,
            multipliers,
            floquet_evidence,
            orbit_kind: self.config.orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
            stability,
            monodromy: monodromy_copy,
            num_unstable,
            min_stability_margin_db: min_margin_db,
            max_multiplier_magnitude: max_magnitude,
            subharmonics,
            stability_threshold: self.config.stability_threshold,
            detect_subharmonics: self.config.detect_subharmonics,
            // The qualified eigenspectrum helper returns only after every
            // eigenpair has passed the canonical residual criterion.
            converged: true,
            iterations: 0,
        };
        result.validate_contract(abort)?;
        Ok(result)
    }

    /// Classify stability based on multipliers
    fn classify_stability_with_abort(
        &self,
        multipliers: &[FloquetMultiplier],
        verdict: FloquetStabilityVerdict,
        trivial_multiplier_index: Option<usize>,
        abort: &dyn AbortSignal,
    ) -> Result<StabilityType, SimulationError> {
        match verdict {
            FloquetStabilityVerdict::Stable => return Ok(StabilityType::Stable),
            FloquetStabilityVerdict::Indeterminate => {
                return Ok(StabilityType::Indeterminate);
            }
            FloquetStabilityVerdict::Unstable => {
                let dominant = multipliers
                    .iter()
                    .find(|multiplier| multiplier.is_unstable)
                    .ok_or_else(|| {
                        SimulationError::Circuit(
                            "PSTB unstable verdict has no multiplier outside the outer boundary"
                                .to_owned(),
                        )
                    })?;
                return if dominant.value.im.abs() > 0.01 {
                    Ok(StabilityType::UnstableComplex)
                } else {
                    Ok(StabilityType::UnstableReal)
                };
            }
            FloquetStabilityVerdict::Marginal => {}
        }

        // Refine a marginal verdict into a recognized bifurcation when the
        // retained values support that label.
        for (index, m) in multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            if trivial_multiplier_index == Some(index) {
                continue;
            }
            let mag = m.magnitude();
            let imag = m.value.im.abs();

            // Near λ = -1: period doubling
            if (m.value + Complex64::new(1.0, 0.0)).norm() < 0.01 {
                return Ok(StabilityType::PeriodDoubling);
            }

            // Near λ = +1: saddle-node. The one explicitly selected
            // autonomous phase mode was skipped above.
            if (m.value - Complex64::new(1.0, 0.0)).norm() < 0.01 {
                return Ok(StabilityType::SaddleNode);
            }

            // Complex pair near unit circle: Neimark-Sacker
            if (mag - 1.0).abs() < 0.01 && imag > 0.01 {
                return Ok(StabilityType::NeimarkSacker);
            }
        }
        Ok(StabilityType::Marginal)
    }
}

fn map_eigenspectrum_error_for_pstb(error: OrdinarySpectrumError) -> SimulationError {
    match error {
        OrdinarySpectrumError::Aborted => SimulationError::Aborted,
        error => SimulationError::Circuit(format!(
            "PSTB Floquet spectrum qualification failed: {error}"
        )),
    }
}

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_periodically(abort: &dyn AbortSignal, index: usize) -> Result<(), SimulationError> {
    const ABORT_POLL_STRIDE: usize = 64;
    if index.is_multiple_of(ABORT_POLL_STRIDE) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, ImmediateAbort, NoAbort};

    #[test]
    fn pstb_eigenanalysis_honors_entry_abort() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let result = analyzer.analyze_monodromy_with_abort(
            &[vec![1.0, 0.0], vec![0.0, 0.5]],
            1.0,
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(SimulationError::Aborted)));
    }

    #[test]
    fn pstb_eigenanalysis_honors_abort_during_validation() {
        const DIMENSION: usize = 16;
        let mut monodromy = vec![vec![0.0; DIMENSION]; DIMENSION];
        for (row, values) in monodromy.iter_mut().enumerate() {
            values[row] = 0.9 - row as Value * 0.001;
            values[(row + 1) % DIMENSION] = 0.01;
        }

        let abort = CountingAbort::new(2);
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let result = analyzer.analyze_monodromy_with_abort(&monodromy, 1e-6, &abort);

        assert!(matches!(result, Err(SimulationError::Aborted)));
        assert!(abort.count() > 2);
    }

    #[test]
    fn pstb_rejects_invalid_period_and_threshold() {
        for period in [0.0, -1.0, Value::NAN, Value::INFINITY, Value::from_bits(1)] {
            let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
            assert!(
                analyzer
                    .analyze_monodromy_with_abort(&[vec![0.5]], period, &NoAbort)
                    .is_err(),
                "period {period:?} must be rejected"
            );
        }

        for threshold in [0.999, 0.0, -1.0, Value::NAN, Value::INFINITY] {
            let config = PstbConfig {
                stability_threshold: threshold,
                ..Default::default()
            };
            let mut analyzer = PstbAnalyzer::new(config);
            assert!(
                analyzer
                    .analyze_monodromy_with_abort(&[vec![0.5]], 1.0, &NoAbort)
                    .is_err(),
                "threshold {threshold:?} must be rejected"
            );
        }
    }

    #[test]
    fn pstb_accepts_authenticated_empty_monodromy_and_rejects_other_invalid_inputs() {
        let mut empty_analyzer = PstbAnalyzer::new(PstbConfig::default());
        let empty = empty_analyzer
            .analyze_monodromy_with_abort(&[], 1.0, &NoAbort)
            .unwrap();
        assert_eq!(
            empty.floquet_evidence,
            FloquetSpectrumEvidence::NoDynamicModes
        );
        assert_eq!(empty.stability_verdict, FloquetStabilityVerdict::Stable);
        assert!(empty.is_stable());

        let invalid = [vec![vec![1.0, 0.0], vec![0.0]], vec![vec![Value::NAN]]];
        for monodromy in invalid {
            let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
            assert!(
                analyzer
                    .analyze_monodromy_with_abort(&monodromy, 1.0, &NoAbort)
                    .is_err()
            );
        }
    }

    #[test]
    fn pstb_rejects_an_exact_zero_multiplier() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let error = analyzer
            .analyze_monodromy_with_abort(&[vec![0.0]], 1.0, &NoAbort)
            .unwrap_err();

        assert!(
            matches!(error, SimulationError::Circuit(message) if message.contains("exactly zero"))
        );
    }

    #[test]
    fn near_unity_mode_is_not_exempt_without_autonomous_evidence() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default());
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![1.0]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability, StabilityType::SaddleNode);
        assert!(!result.is_stable());
        assert_eq!(result.min_stability_margin_db, Some(-0.0));
    }

    #[test]
    fn wide_outer_threshold_cannot_manufacture_stability() {
        let config = PstbConfig {
            stability_threshold: 2.0,
            ..Default::default()
        };
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![1.5]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability, StabilityType::Marginal);
        assert!(!result.is_stable());
    }

    #[test]
    fn autonomous_phase_selection_uses_only_the_canonical_band() {
        let mut config = PstbConfig::default()
            .with_orbit_kind(FloquetOrbitKind::Autonomous)
            .with_stability_threshold(2.0);
        let mut analyzer = PstbAnalyzer::new(config.clone());
        let invalid_phase = analyzer
            .analyze_monodromy_with_abort(&[vec![1.5, 0.0], vec![0.0, 0.5]], 1.0, &NoAbort)
            .unwrap();
        assert_eq!(invalid_phase.trivial_multiplier_index, None);
        assert_eq!(
            invalid_phase.stability_verdict,
            FloquetStabilityVerdict::Indeterminate
        );
        assert!(!invalid_phase.is_stable());

        config.stability_threshold = 1.0 + 1e-6;
        let mut analyzer = PstbAnalyzer::new(config);
        let stable = analyzer
            .analyze_monodromy_with_abort(&[vec![1.0, 0.0], vec![0.0, 0.5]], 1.0, &NoAbort)
            .unwrap();
        assert_eq!(stable.trivial_multiplier_index, Some(0));
        assert_eq!(stable.stability_verdict, FloquetStabilityVerdict::Stable);
        assert!(stable.is_stable());

        let phase_only = analyzer
            .analyze_monodromy_with_abort(&[vec![1.0]], 1.0, &NoAbort)
            .unwrap();
        assert_eq!(phase_only.trivial_multiplier_index, Some(0));
        assert_eq!(phase_only.min_stability_margin_db, None);
    }

    #[test]
    fn selected_autonomous_phase_mode_is_excluded_from_unstable_metadata() {
        let config = PstbConfig::default()
            .with_orbit_kind(FloquetOrbitKind::Autonomous)
            .with_stability_threshold(1.0);
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![1.0000005, 0.0], vec![0.0, 0.5]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability_verdict, FloquetStabilityVerdict::Stable);
        assert_eq!(result.trivial_multiplier_index, Some(0));
        assert!(result.multipliers[0].is_trivial);
        assert!(!result.multipliers[0].is_unstable);
        assert_eq!(result.num_unstable, 0);
        assert!(result.is_stable());
    }

    #[test]
    fn pstb_returns_full_sorted_spectrum_with_paired_eigenvectors() {
        let matrix = vec![
            vec![0.0, -1.0, 0.0],
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.5],
        ];
        let mut config = PstbConfig::default().with_eigenvectors(true);
        config.num_eigenvalues = 1;
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&matrix, 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.multipliers.len(), 3);
        assert!(result.converged);
        assert!(matches!(
            result.floquet_evidence,
            FloquetSpectrumEvidence::Qualified { .. }
        ));
        assert!(
            result
                .multipliers
                .windows(2)
                .all(|pair| pair[0].magnitude() >= pair[1].magnitude())
        );

        for multiplier in &result.multipliers {
            let vector = multiplier.eigenvector.as_ref().unwrap();
            let mut residual_norm = 0.0_f64;
            for row in 0..matrix.len() {
                let product: Complex64 = matrix[row]
                    .iter()
                    .zip(vector)
                    .map(|(&coefficient, &component)| coefficient * component)
                    .sum();
                residual_norm =
                    residual_norm.hypot((product - multiplier.value * vector[row]).norm());
            }
            assert!(residual_norm < 1e-12, "residual={residual_norm:.3e}");
        }
    }

    #[test]
    fn subharmonic_reporting_honors_configuration() {
        let config = PstbConfig {
            detect_subharmonics: false,
            ..Default::default()
        };
        let mut analyzer = PstbAnalyzer::new(config);
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![-1.0]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability, StabilityType::PeriodDoubling);
        assert!(result.subharmonics.is_empty());
        // The per-mode order is cleared too. A run told not to look does not
        // report a finding on one mode while its aggregate says nobody looked.
        assert_eq!(result.multipliers[0].subharmonic_order, None);
        assert!(!result.detect_subharmonics);
    }

    #[test]
    fn the_result_retains_the_boundary_it_was_classified_under() {
        let mut analyzer = PstbAnalyzer::new(PstbConfig::default().with_stability_threshold(1.25));
        let result = analyzer
            .analyze_monodromy_with_abort(&[vec![1.5, 0.0], vec![0.0, 0.5]], 1.0, &NoAbort)
            .unwrap();

        assert_eq!(result.stability_threshold, 1.25);
        assert!(result.detect_subharmonics);
        // Which is what makes `is_unstable` checkable at all: 1.5 is outside
        // 1.25 and 0.5 is not, and a reader can now verify that without being
        // handed the boundary separately.
        assert!(result.multipliers[0].is_unstable);
        assert!(!result.multipliers[1].is_unstable);
        result.validate_contract(&NoAbort).unwrap();
    }

    /// Every aggregate the analyzer publishes is re-derived before the result
    /// leaves it, so no consumer has to re-derive them to be safe.
    #[test]
    fn a_result_whose_published_quantities_drifted_is_refused_by_its_own_contract() {
        let sound = || {
            PstbAnalyzer::new(PstbConfig::default().with_eigenvectors(true))
                .analyze_monodromy_with_abort(&[vec![0.5, 0.0], vec![0.0, 0.25]], 1.0, &NoAbort)
                .unwrap()
        };
        sound().validate_contract(&NoAbort).unwrap();

        type Mutation = (&'static str, fn(&mut PstbResult));
        let mutations: [Mutation; 7] = [
            ("aggregate counts", |result| result.num_unstable = 1),
            ("aggregate counts", |result| {
                result.max_multiplier_magnitude = 2.0
            }),
            ("aggregate counts", |result| {
                result.min_stability_margin_db = None
            }),
            ("canonical sorted order", |result| {
                result.multipliers.swap(0, 1)
            }),
            ("monodromy", |result| {
                result.monodromy.pop();
            }),
            ("mode 1", |result| result.multipliers[0].is_unstable = true),
            ("mode 1", |result| {
                result.multipliers[0].eigenvector = Some(Vec::new())
            }),
        ];
        for (expected, mutate) in mutations {
            let mut result = sound();
            mutate(&mut result);
            let error = result
                .validate_contract(&NoAbort)
                .expect_err("a drifted quantity must be refused");
            assert!(
                error.to_string().contains(expected),
                "the refusal must name what drifted, got: {error}"
            );
        }
    }
}
