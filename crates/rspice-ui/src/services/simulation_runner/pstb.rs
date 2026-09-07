//! Periodic stability analysis.
//!
//! The Floquet multiplier spectrum of a periodic orbit, read through one loop
//! probe -- for feedback that is only meaningful over a cycle, such as
//! switched-mode regulators and sampled loops.
//!
//! The run itself belongs to the engine:
//! [`rspice_core::Engine::run_pstb_card_from_pss_with_abort`] resolves the
//! probe against the carrier's own shooting-state basis, judges the spectrum
//! under the card's boundary, and establishes every invariant the result
//! publishes before returning it. What is left here is the Studio's own share
//! -- turning a dialog or a deck line into that card, and projecting the
//! complete spectrum onto the modes the sheet is asked to display.

use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    error::{ensure_not_aborted, poll_periodically},
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use std::path::Path;

/// Explicit configuration for PSTB execution.
#[derive(Debug, Clone)]
pub struct PstbRunConfig {
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: Value,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: Value,
}

impl Default for PstbRunConfig {
    fn default() -> Self {
        Self {
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            probe_instance: "LPROBE".to_string(),
            max_harmonics: 10,
            num_multipliers: 10,
            stability_threshold: 1.0 + 1e-6,
            detect_subharmonics: true,
            eigenvalue_tolerance: 1e-10,
        }
    }
}

impl PstbRunConfig {
    /// Check what the card cannot carry.
    ///
    /// The probe name, the harmonic and multiplier counts, the stability
    /// boundary and the eigen-tolerance are all fields of [`PstbCard`](rspice_
    /// core::netlist::PstbCard), and the engine refuses each of them by name
    /// in `run_pstb_card_from_pss_with_abort`. Restating them here is how the
    /// Studio came to hold a threshold rule its own engine did not: only the
    /// prerequisite-PSS fields, which no card has a home for, are checked.
    fn validate(&self) -> Result<(), String> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err("PSTB requires a positive PSS fundamental frequency".to_string());
        }
        if self.pss_num_harmonics == 0 {
            return Err("PSTB requires at least one PSS harmonic".to_string());
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err("PSTB requires a positive PSS tolerance".to_string());
        }
        Ok(())
    }

    /// The authored `.PSTB` card this configuration states.
    fn to_card(&self) -> rspice_core::netlist::PstbCard {
        rspice_core::netlist::PstbCard {
            probe_instance: self.probe_instance.trim().to_owned(),
            max_harmonics: self.max_harmonics,
            num_multipliers: self.num_multipliers,
            stability_threshold: self.stability_threshold,
            detect_subharmonics: self.detect_subharmonics,
            eigenvalue_tolerance: self.eigenvalue_tolerance,
        }
    }
}

/// PSTB analysis data.
#[derive(Debug, Clone)]
pub struct PstbData {
    /// Period of the analyzed periodic orbit.
    pub period: Value,
    /// Fundamental frequency of the analyzed periodic orbit.
    pub fundamental_frequency: Value,
    /// Complete authenticated Floquet spectrum, sorted by magnitude.
    pub modes: Vec<PstbModeData>,
    /// Completeness and residual qualification for `modes`.
    pub floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
    /// Driven/autonomous policy copied from the prerequisite PSS result.
    pub orbit_kind: rspice_core::analysis::FloquetOrbitKind,
    /// Exact outer stability threshold used for classification.
    pub stability_threshold: Value,
    /// Canonical circuit identity of the resolved probe.
    pub probe_instance: String,
    /// Whether subharmonic classification was enabled.
    pub detect_subharmonics: bool,
    /// Explicitly selected autonomous phase-mode index.
    pub trivial_multiplier_index: Option<usize>,
    /// Shared four-state stability verdict.
    pub stability_verdict: rspice_core::analysis::FloquetStabilityVerdict,
    /// Rich PSTB classification refining the shared verdict.
    pub stability_classification: rspice_core::analysis::pstb::StabilityType,
    /// Finite signed global margin when an applicable mode exists.
    pub min_stability_margin_db: Option<Value>,
    /// Maximum multiplier magnitude over the complete spectrum.
    pub max_multiplier_magnitude: Value,
    /// Number of non-trivial modes outside the configured outer boundary.
    pub num_unstable: usize,
    /// Detected subharmonic orders over the complete spectrum.
    pub subharmonics: Vec<usize>,
    /// Whether the atomic qualified eigensolve completed.
    pub converged: bool,
    /// Iteration count reported by the eigensolver.
    pub iterations: usize,
    /// Mode indices (1-based) for plotting.
    pub mode_indices: Vec<Value>,
    /// Probe-local mode participation (normalized |v_i| contribution per mode).
    pub probe_mode_participation: Vec<Value>,
    /// Floquet multiplier magnitudes.
    pub multiplier_magnitude: Vec<Value>,
    /// Floquet multiplier phases in degrees.
    pub multiplier_phase_deg: Vec<Value>,
    /// Mode damping factors in 1/s.
    pub mode_damping: Vec<Value>,
    /// Natural mode frequencies in hertz.
    pub mode_frequency_hz: Vec<Value>,
    /// Per-mode stability margin in dB.
    pub stability_margin_db: Vec<Value>,
}

/// One complete retained mode. Display limits never truncate this vector.
#[derive(Debug, Clone, PartialEq)]
pub struct PstbModeData {
    pub multiplier: (Value, Value),
    pub exponent: (Value, Value),
    pub probe_participation: Value,
    pub is_unstable: bool,
    pub is_trivial: bool,
    pub subharmonic_order: Option<usize>,
}

/// Project one engine result onto what the Studio's sheet shows.
///
/// This is the whole of the Studio's remaining share. Every invariant the
/// spectrum carries -- Floquet currency, phase-mode selection, verdict and
/// classification agreement, canonical sort order, monodromy squareness, the
/// four aggregates and per-mode identity -- is established by
/// `PstbResult::validate_contract` before the analyzer returns, on both of its
/// return paths, so nothing here re-derives them. `modes` is always the
/// complete spectrum; only the six plotted curves honour the card's display
/// limit, because a truncated spectrum cannot prove stability.
fn build_pstb_data(
    stability: rspice_core::engine::PeriodicStabilityResult,
    max_display_modes: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    let rspice_core::engine::PeriodicStabilityResult {
        probe_instance,
        probe_state_index: _,
        probe_participation,
        result,
    } = stability;

    ensure_not_aborted(abort)?;
    let order = result.multipliers.len();
    let mut modes = Vec::with_capacity(order);
    for (index, multiplier) in result.multipliers.iter().enumerate() {
        poll_periodically(abort, index)?;
        let participation = probe_participation.get(index).copied().ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "PSTB mode {} carries no probe participation",
                index + 1
            ))
        })?;
        modes.push(PstbModeData {
            multiplier: (multiplier.value.re, multiplier.value.im),
            exponent: (multiplier.exponent.re, multiplier.exponent.im),
            probe_participation: participation,
            // The analyzer clears the per-mode order when detection is off, so
            // the flag is already spent: an order here means one was looked for
            // and found.
            subharmonic_order: multiplier.subharmonic_order,
            is_unstable: multiplier.is_unstable,
            is_trivial: multiplier.is_trivial,
        });
    }

    let display_count = order.min(max_display_modes);
    let mut mode_indices = Vec::with_capacity(display_count);
    let mut probe_mode_participation = Vec::with_capacity(display_count);
    let mut multiplier_magnitude = Vec::with_capacity(display_count);
    let mut multiplier_phase_deg = Vec::with_capacity(display_count);
    let mut mode_damping = Vec::with_capacity(display_count);
    let mut mode_frequency_hz = Vec::with_capacity(display_count);
    let mut stability_margin_db = Vec::with_capacity(display_count);
    for (index, multiplier) in result.multipliers.iter().take(display_count).enumerate() {
        poll_periodically(abort, index)?;
        mode_indices.push((index + 1) as Value);
        probe_mode_participation.push(modes[index].probe_participation);
        multiplier_magnitude.push(multiplier.magnitude());
        multiplier_phase_deg.push(multiplier.phase_degrees());
        mode_damping.push(multiplier.damping());
        mode_frequency_hz.push(multiplier.natural_frequency());
        stability_margin_db.push(multiplier.stability_margin_db());
    }
    ensure_not_aborted(abort)?;

    Ok(PstbData {
        period: result.period,
        fundamental_frequency: result.fundamental_frequency,
        modes,
        floquet_evidence: result.floquet_evidence,
        orbit_kind: result.orbit_kind,
        stability_threshold: result.stability_threshold,
        probe_instance,
        detect_subharmonics: result.detect_subharmonics,
        trivial_multiplier_index: result.trivial_multiplier_index,
        stability_verdict: result.stability_verdict,
        stability_classification: result.stability,
        min_stability_margin_db: result.min_stability_margin_db,
        max_multiplier_magnitude: result.max_multiplier_magnitude,
        num_unstable: result.num_unstable,
        subharmonics: result.subharmonics,
        converged: result.converged,
        iterations: result.iterations,
        mode_indices,
        probe_mode_participation,
        multiplier_magnitude,
        multiplier_phase_deg,
        mode_damping,
        mode_frequency_hz,
        stability_margin_db,
    })
}

/// Run PSTB standalone -- computing its own PSS carrier rather than receiving
/// one -- with cooperative cancellation.
///
/// Test-only. PSTB ships as a dependent task: the frequency spec runs PSS
/// first and hands the authenticated operating point to
/// [`run_pstb_analysis_from_pss_with_source_path_and_abort`], so nothing in the
/// product takes this path.
#[cfg(test)]
pub fn run_pstb_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PstbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    run_pstb_analysis_impl(netlist_text, config, None, None, abort)
}

/// Run PSTB from an exact retained PSS state with direct-call source-relative
/// include and model resolution.
pub fn run_pstb_analysis_from_pss_with_source_path_and_abort(
    netlist_text: &str,
    config: &PstbRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    run_pstb_analysis_impl(
        netlist_text,
        config,
        source_path,
        Some(operating_point),
        abort,
    )
}

fn run_pstb_analysis_impl(
    netlist_text: &str,
    config: &PstbRunConfig,
    source_path: Option<&Path>,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PstbData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = build_resolved_periodic_engine(
        &netlist,
        config.pss_tolerance,
        "PSTB resolved producer configuration is invalid",
    )?;
    let card = config.to_card();

    let owned_carrier;
    let carrier = match operating_point {
        Some(operating_point) => operating_point,
        None => {
            // A standalone run states its own carrier. The harmonic count is
            // the larger of the two the configuration names, because the card
            // refuses a carrier whose spectral capacity is below its MAXHARM.
            owned_carrier = engine
                .run_pss_operating_point_with_abort(
                    &netlist,
                    rspice_core::analysis::PssConfig::new(config.pss_fundamental_freq)
                        .with_harmonics(config.pss_num_harmonics.max(config.max_harmonics))
                        .with_tolerance(config.pss_tolerance)
                        .with_max_iterations(50)
                        .with_tstab_periods(10),
                    abort,
                )
                .map_err(|error| ServiceRunError::from_core("PSTB prerequisite PSS", error))?;
            &owned_carrier
        }
    };

    let stability = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &card, carrier, abort)
        .map_err(|error| ServiceRunError::from_core("PSTB error", error))?;
    build_pstb_data(stability, config.num_multipliers, abort)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};
    use rspice_core::analysis::pstb::{PstbAnalyzer, PstbConfig};
    use rspice_core::analysis::{
        FloquetOrbitKind, FloquetSpectrumEvidence, FloquetStabilityVerdict,
    };
    use rspice_core::engine::PeriodicStabilityResult;

    const STABILITY_THRESHOLD: Value = 1.0 + 1.0e-6;

    /// One engine result, with a participation vector whose values are chosen
    /// so a projection that reordered or recomputed them would be visible.
    fn stability(
        monodromy: &[Vec<Value>],
        orbit_kind: FloquetOrbitKind,
    ) -> PeriodicStabilityResult {
        let result = PstbAnalyzer::new(
            PstbConfig::new()
                .with_orbit_kind(orbit_kind)
                .with_eigenvectors(true)
                .with_stability_threshold(STABILITY_THRESHOLD),
        )
        .analyze_monodromy_with_abort(monodromy, 1.0, &NoAbort)
        .expect("the analyzer publishes a qualified spectrum");
        let probe_participation = (0..result.multipliers.len())
            .map(|index| (index + 1) as Value / 16.0)
            .collect();

        PeriodicStabilityResult {
            probe_instance: "LPROBE".to_owned(),
            probe_state_index: 0,
            probe_participation,
            result,
        }
    }

    #[test]
    fn pstb_service_preserves_typed_entry_abort() {
        let mut config = PstbRunConfig::default();
        config.pss_fundamental_freq = 0.0;

        let result =
            run_pstb_analysis_with_config_and_abort("not a netlist", &config, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    /// The display limit is the card's, and it governs the plotted curves
    /// only: `modes` is the complete authenticated spectrum whatever the sheet
    /// is asked to show, because a truncated spectrum cannot prove stability.
    #[test]
    fn presentation_limit_does_not_truncate_authenticated_spectrum() {
        let data = build_pstb_data(
            stability(
                &[
                    vec![0.5, 0.0, 0.0],
                    vec![0.0, 0.4, 0.0],
                    vec![0.0, 0.0, 0.3],
                ],
                FloquetOrbitKind::Driven,
            ),
            1,
            &NoAbort,
        )
        .expect("the projection publishes the spectrum it was handed");

        assert_eq!(data.modes.len(), 3);
        assert_eq!(data.mode_indices, vec![1.0]);
        assert_eq!(data.multiplier_magnitude, vec![0.5]);
        assert_eq!(data.stability_threshold, STABILITY_THRESHOLD);
        assert_eq!(data.probe_instance, "LPROBE");
        assert!(data.detect_subharmonics);
        let FloquetSpectrumEvidence::Qualified { certificate } = data.floquet_evidence else {
            panic!("expected a qualified complete spectrum");
        };
        assert_eq!(certificate.problem_order, 3);
        assert_eq!(data.stability_verdict, FloquetStabilityVerdict::Stable);
    }

    /// The engine resolves the probe and normalizes its share of every mode
    /// shape; the sheet publishes those numbers and computes none of its own.
    #[test]
    fn the_engines_probe_participation_reaches_the_sheet_unchanged() {
        let engine_result = stability(
            &[
                vec![0.5, 0.0, 0.0],
                vec![0.0, 0.4, 0.0],
                vec![0.0, 0.0, 0.3],
            ],
            FloquetOrbitKind::Driven,
        );
        let expected = engine_result.probe_participation.clone();
        let data = build_pstb_data(engine_result, 2, &NoAbort).expect("the projection succeeds");

        assert_eq!(expected, vec![1.0 / 16.0, 2.0 / 16.0, 3.0 / 16.0]);
        assert_eq!(
            data.modes
                .iter()
                .map(|mode| mode.probe_participation)
                .collect::<Vec<_>>(),
            expected,
            "every retained mode carries the engine's own participation"
        );
        assert_eq!(
            data.probe_mode_participation,
            expected[..2].to_vec(),
            "the plotted curve is the same numbers, cut to the display limit"
        );
    }

    /// A driven orbit whose periodic map carries no dynamic state is a real,
    /// authenticated answer -- and it has no curve to plot.
    #[test]
    fn driven_zero_order_spectrum_publishes_no_display_curves() {
        let data = build_pstb_data(stability(&[], FloquetOrbitKind::Driven), 1, &NoAbort)
            .expect("a state-free driven map is representable");

        assert!(data.modes.is_empty());
        assert!(data.mode_indices.is_empty());
        assert!(data.probe_mode_participation.is_empty());
        assert!(matches!(
            data.floquet_evidence,
            FloquetSpectrumEvidence::NoDynamicModes
        ));
        assert_eq!(data.stability_verdict, FloquetStabilityVerdict::Stable);
        assert_eq!(data.min_stability_margin_db, None);
        assert_eq!(data.num_unstable, 0);
    }
}
