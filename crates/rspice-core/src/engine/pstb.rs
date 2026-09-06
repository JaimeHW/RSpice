//! Authored `.PSTB` runs: Floquet stability of a retained periodic orbit,
//! read at one loop probe.
//!
//! There is no harmonic-balance sibling to the entry below.
//! [`PssAnalysisResult::monodromy`](super::PssAnalysisResult) is the only
//! monodromy matrix the engine produces; [`HbOperatingPoint`](super::
//! HbOperatingPoint) carries a converged spectrum and no state-transition map
//! at all, so a `.PSTB` bound to a harmonic-balance carrier is refused where
//! the deck is planned rather than given a second entry point that cannot
//! answer it.
//!
//! The probe is resolved against the carrier's own
//! [`shooting_state_basis`](super::PssOperatingPoint::shooting_state_basis)
//! rather than against a circuit rebuilt beside it. The basis is what the
//! retained monodromy is indexed by, so resolving against it cannot name a
//! coordinate the matrix does not have; a circuit is built only on the failure
//! path, where naming the probes the deck does offer is worth the cost.

use crate::abort_signal::AbortSignal;
use crate::analysis::FloquetOrbitKind;
use crate::analysis::pstb::{PstbAnalyzer, PstbConfig, PstbResult};
use crate::circuit::CircuitData;
use crate::netlist::{Netlist, PstbCard};
use crate::{SimulationError, Value};

use super::{Engine, PssOperatingPoint};

/// One authored `.PSTB` run: the qualified Floquet spectrum plus the probe
/// identity it was measured at.
///
/// The probe travels beside the spectrum for the same reason
/// [`PeriodicNoiseResult`](super::PeriodicNoiseResult) carries its authored
/// output: the multipliers are a property of the orbit, but *which loop* they
/// were read through is the card's own statement, and the per-mode
/// participation below is meaningless without it.
#[derive(Debug, Clone)]
pub struct PeriodicStabilityResult {
    /// Canonical circuit spelling of the resolved loop probe.
    pub probe_instance: String,
    /// Index of the probe's current in the carrier's shooting-state basis,
    /// which is the coordinate the retained monodromy is indexed by.
    pub probe_state_index: usize,
    /// Normalized participation of the probe coordinate in each mode shape,
    /// `|v_i| / ||v||`, over the complete spectrum in its canonical order.
    pub probe_participation: Vec<Value>,
    /// The qualified spectrum itself.
    pub result: PstbResult,
}

impl Engine {
    /// Run one authored `.PSTB` card against a retained shooting-`.PSS`
    /// carrier.
    ///
    /// The carrier supplies the period and the monodromy matrix; the card
    /// supplies the loop probe, the classification boundary and the numerical
    /// contract the spectrum is judged under. Every invariant the result
    /// publishes is established by
    /// [`PstbAnalyzer`](crate::analysis::pstb::PstbAnalyzer) before it is
    /// returned, so a caller has nothing left to re-derive.
    ///
    /// There is deliberately no `..._from_hb_with_abort` sibling: a
    /// harmonic-balance operating point has no monodromy matrix, and the plan
    /// refuses a `.PSTB` that names one.
    pub fn run_pstb_card_from_pss_with_abort(
        &self,
        netlist: &Netlist,
        card: &PstbCard,
        operating_point: &PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PeriodicStabilityResult, SimulationError> {
        ensure_not_aborted(abort)?;
        validate_card(card)?;

        // The card states the harmonic resolution the study is about. It does
        // not solve its own carrier, so the only honest reading against a
        // retained one is a precondition: an orbit whose retained samples
        // cannot represent that many harmonics without aliasing is not the
        // orbit the card asked about.
        let capacity = operating_point.spectral_harmonic_capacity();
        if capacity < card.max_harmonics {
            return Err(SimulationError::Circuit(format!(
                "PSTB MAXHARM={} exceeds the retained carrier's spectral capacity of {capacity} \
                 harmonics; the orbit it would be read from is under-resolved",
                card.max_harmonics
            )));
        }

        let probe = self.resolve_pstb_probe(netlist, operating_point, &card.probe_instance)?;
        ensure_not_aborted(abort)?;

        let pss = operating_point.analysis();
        let order = pss.monodromy.len();
        for (index, row) in pss.monodromy.iter().enumerate() {
            poll_periodically(abort, index)?;
            if row.len() != order {
                return Err(SimulationError::Circuit(
                    "PSTB prerequisite PSS returned a non-square monodromy matrix".to_owned(),
                ));
            }
        }
        if order > 0 && probe.state_index >= order {
            return Err(SimulationError::Circuit(format!(
                "PSTB probe '{}' maps to shooting coordinate {} but the retained monodromy has \
                 order {order}",
                probe.canonical_name, probe.state_index
            )));
        }

        let orbit_kind = pss.result.floquet_orbit_kind;
        if !pss.result.has_consistent_floquet_contract()
            || pss.result.period_detected != (orbit_kind == FloquetOrbitKind::Autonomous)
        {
            return Err(SimulationError::Circuit(
                "PSTB prerequisite PSS Floquet orbit contract is inconsistent".to_owned(),
            ));
        }
        // An autonomous orbit has a free phase, so its spectrum must contain a
        // phase mode. A state-free periodic map has no modes at all, so the two
        // statements cannot both be true and the run refuses rather than
        // publishing an indeterminate verdict as if it were a measurement.
        if order == 0 && orbit_kind != FloquetOrbitKind::Driven {
            return Err(SimulationError::Circuit(
                "PSTB cannot judge an autonomous orbit whose periodic map has no dynamic state"
                    .to_owned(),
            ));
        }

        let config = PstbConfig::new()
            .with_num_eigenvalues(card.num_multipliers)
            .with_orbit_kind(orbit_kind)
            .with_eigenvectors(true)
            .with_tolerance(card.eigenvalue_tolerance)
            .with_stability_threshold(card.stability_threshold)
            .with_subharmonic_detection(card.detect_subharmonics);
        let result = PstbAnalyzer::new(config).analyze_monodromy_with_abort(
            &pss.monodromy,
            pss.period,
            abort,
        )?;

        let mut probe_participation = Vec::with_capacity(result.multipliers.len());
        for (index, multiplier) in result.multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            probe_participation.push(normalized_probe_participation(
                multiplier.eigenvector.as_deref(),
                probe.state_index,
                abort,
            )?);
        }
        ensure_not_aborted(abort)?;

        Ok(PeriodicStabilityResult {
            probe_instance: probe.canonical_name,
            probe_state_index: probe.state_index,
            probe_participation,
            result,
        })
    }

    /// Resolve an authored loop-probe name to its coordinate in the carrier's
    /// shooting-state basis.
    ///
    /// The basis is `["C:<capacitor>"..., "L:<inductor>"...]`, so the position
    /// of `L:<probe>` in it *is* `capacitors.len() + inductor_index`, which is
    /// bit-identically what
    /// [`CircuitData::inductor_probe_for_branch`](crate::circuit::CircuitData::
    /// inductor_probe_for_branch) computes from a built circuit. Resolving
    /// here needs no circuit at all; one is built only to say what the deck
    /// does offer when the name misses.
    fn resolve_pstb_probe(
        &self,
        netlist: &Netlist,
        operating_point: &PssOperatingPoint,
        probe_instance: &str,
    ) -> Result<ResolvedPstbProbe, SimulationError> {
        let probe_name = probe_instance.trim();
        let basis = operating_point.shooting_state_basis();
        // A legacy identityless artifact carries no basis at all. Resolving a
        // probe against it would silently name coordinate zero, so it is a
        // refusal: the retained state cannot say what its own coordinates are.
        if basis.is_empty() {
            return Err(SimulationError::Circuit(format!(
                "PSTB probe '{probe_name}' cannot be resolved: the retained PSS operating point \
                 carries no shooting-state basis, which is how an unauthenticated legacy artifact \
                 presents itself"
            )));
        }

        for (index, coordinate) in basis.iter().enumerate() {
            if let Some(name) = coordinate.strip_prefix("L:")
                && name.eq_ignore_ascii_case(probe_name)
            {
                return Ok(ResolvedPstbProbe {
                    canonical_name: name.to_owned(),
                    state_index: index,
                });
            }
        }
        Err(self.pstb_probe_diagnostic(netlist, basis, probe_name))
    }

    /// Name what the deck does offer when a loop probe misses.
    ///
    /// This is the only place a circuit is built for a `.PSTB` run, and it is
    /// built to produce a better message: the basis alone can list inductors,
    /// but it cannot say that the name resolves to a *branch* that is not an
    /// inductor, which is the mistake a deck author actually makes.
    fn pstb_probe_diagnostic(
        &self,
        netlist: &Netlist,
        basis: &[String],
        probe_name: &str,
    ) -> SimulationError {
        let inductor_probes = |circuit: Option<&CircuitData>| {
            format_probe_names(match circuit {
                Some(circuit) => circuit.inductor_probe_names(),
                None => basis
                    .iter()
                    .filter_map(|coordinate| coordinate.strip_prefix("L:"))
                    .map(str::to_owned)
                    .collect(),
            })
        };
        let Ok(circuit) = self.build_circuit(netlist) else {
            return SimulationError::Circuit(format!(
                "PSTB probe '{probe_name}' is not an inductor current in the retained carrier. \
                 Available inductor probes: {}",
                inductor_probes(None)
            ));
        };
        match circuit.get_branch_by_name(probe_name) {
            None => SimulationError::Circuit(format!(
                "PSTB probe '{probe_name}' was not found in branch-capable elements. Available \
                 branches: {}",
                format_probe_names(circuit.branch_probe_names())
            )),
            Some(branch_ordinal) if circuit.inductor_probe_for_branch(branch_ordinal).is_none() => {
                SimulationError::Circuit(format!(
                    "PSTB probe '{probe_name}' resolved to branch ordinal {branch_ordinal} but is \
                     not an inductor probe. PSTB supports dynamic inductor-current probes only. \
                     Available inductor probes: {}",
                    inductor_probes(Some(&circuit))
                ))
            }
            // The name is an inductor of *this* circuit but not a coordinate
            // of the retained carrier, so the two do not describe the same
            // deck. Saying so is more use than listing names that look right.
            Some(_) => SimulationError::Circuit(format!(
                "PSTB probe '{probe_name}' is an inductor of this circuit but not a coordinate of \
                 the retained PSS shooting-state basis, which spans: {}",
                inductor_probes(None)
            )),
        }
    }
}

/// Canonical probe identity and the shooting coordinate it occupies.
struct ResolvedPstbProbe {
    canonical_name: String,
    state_index: usize,
}

fn validate_card(card: &PstbCard) -> Result<(), SimulationError> {
    if card.probe_instance.trim().is_empty() {
        return Err(SimulationError::Circuit(
            "PSTB requires a loop-probe instance name".to_owned(),
        ));
    }
    if card.max_harmonics == 0 || card.num_multipliers == 0 {
        return Err(SimulationError::Circuit(
            "PSTB requires at least one carrier harmonic and one reported multiplier".to_owned(),
        ));
    }
    if !card.stability_threshold.is_finite() || card.stability_threshold < 1.0 {
        return Err(SimulationError::Circuit(format!(
            "PSTB requires a finite stability threshold of at least one, got {}",
            card.stability_threshold
        )));
    }
    if !card.eigenvalue_tolerance.is_finite() || card.eigenvalue_tolerance <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "PSTB requires a positive eigenvalue tolerance, got {}",
            card.eigenvalue_tolerance
        )));
    }
    Ok(())
}

/// Normalized participation of one shooting coordinate in one mode shape.
///
/// `|v_i| / ||v||` with the Euclidean norm accumulated by `hypot`, so a mode
/// whose components span many decades does not overflow on the way to a ratio
/// that is bounded by one.
fn normalized_probe_participation(
    eigenvector: Option<&[num_complex::Complex64]>,
    state_index: usize,
    abort: &dyn AbortSignal,
) -> Result<Value, SimulationError> {
    let vector = eigenvector.ok_or_else(|| {
        SimulationError::Circuit(
            "PSTB solver did not return a requested eigenvector; the probe's participation in \
             the mode cannot be stated"
                .to_owned(),
        )
    })?;
    let component = vector.get(state_index).ok_or_else(|| {
        SimulationError::Circuit(
            "PSTB eigenvector does not contain the configured probe coordinate".to_owned(),
        )
    })?;
    let mut norm = 0.0_f64;
    for (index, value) in vector.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(SimulationError::Circuit(
                "PSTB solver returned a non-finite eigenvector".to_owned(),
            ));
        }
        norm = norm.hypot(value.norm());
    }
    if !norm.is_finite() || norm == 0.0 {
        return Err(SimulationError::Circuit(
            "PSTB solver returned a zero-norm eigenvector".to_owned(),
        ));
    }
    let ratio = component.norm() / norm;
    if ratio.is_finite() {
        Ok(ratio.clamp(0.0, 1.0))
    } else {
        Err(SimulationError::Circuit(
            "PSTB probe participation is non-finite".to_owned(),
        ))
    }
}

/// Render a probe-name list for a diagnostic: canonically ordered, de-duplicated
/// case-insensitively, and truncated so one message cannot become a dump.
fn format_probe_names(mut names: Vec<String>) -> String {
    names.sort_by_cached_key(|name| name.to_ascii_uppercase());
    names.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    if names.is_empty() {
        return "<none>".to_owned();
    }
    const DISPLAY_LIMIT: usize = 12;
    if names.len() <= DISPLAY_LIMIT {
        return names.join(", ");
    }
    format!(
        "{}, ... (+{} more)",
        names[..DISPLAY_LIMIT].join(", "),
        names.len() - DISPLAY_LIMIT
    )
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
