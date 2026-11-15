//! Authored `.PSTB` runs: Floquet stability of a retained periodic orbit,
//! read at one loop probe.
//!
//! An orbit event shooting solved carries the whole solution as its
//! coordinates, and a generated model's coordinates on native shooting may
//! include a node its own branches tie to another; the spectrum is read
//! over the coordinates the period remembers, the probe's first.
//!
//! There is no harmonic-balance sibling to the entry below.
//! [`PssAnalysisResult::monodromy`](super::PssAnalysisResult) is the only
//! monodromy matrix the engine produces; [`HbOperatingPoint`](super::
//! HbOperatingPoint) carries a converged spectrum and no state-transition map
//! at all, so a `.PSTB` bound to a harmonic-balance carrier is refused where
//! the deck is planned rather than given a second entry point that cannot
//! answer it.
//!
//! The retained circuit and configuration are authenticated before resolving
//! a probe against the carrier's own
//! [`shooting_state_basis`](super::PssOperatingPoint::shooting_state_basis).
//! A dependent winding current is projected through the same graph reduction
//! that defined that basis, so its participation uses the physical current.

use crate::abort_signal::AbortSignal;
use crate::analysis::FloquetOrbitKind;
use crate::analysis::pstb::{PstbAnalyzer, PstbConfig, PstbResult};
use crate::circuit::CircuitData;
use crate::netlist::{ElementKind, Netlist, PstbCard};
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
    /// Single coordinate proportional to the probe current, or `None` when
    /// the current is a combination of independent shooting coordinates.
    pub probe_state_index: Option<usize>,
    /// Sparse physical-current projection `(state index, signed weight)` in
    /// ascending index order. The monodromy uses these same coordinates.
    pub probe_state_projection: Vec<(usize, Value)>,
    /// Normalized projection in each mode shape, `|w.v| / (||w|| ||v||)`,
    /// over the complete spectrum in its canonical order.
    pub probe_participation: Vec<Value>,
    /// The qualified spectrum itself.
    pub result: PstbResult,
}

impl Engine {
    /// Run one authored `.PSTB` card against a retained shooting-`.PSS`
    /// carrier.
    ///
    /// The carrier supplies the period and the monodromy matrix; the card
    /// supplies the loop probe and the classification boundary the spectrum
    /// is judged under. Every invariant the result
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

        let engine = self.resolved_for_netlist(netlist);
        let order = operating_point.analysis().monodromy.len();
        // A cached carrier can bypass circuit construction. Admit the actual
        // eigenproblem and its complete retained spectrum before that fast path
        // allocates the matrix, eigenvectors, or probe-participation result.
        engine.ensure_matrix_unknowns(order)?;
        let result_values = order
            .saturating_mul(order.saturating_mul(3).saturating_add(7))
            .saturating_add(16);
        // Real monodromy + complex eigenvectors, multiplier/exponent pairs,
        // participation and sparse probe coordinates, and scalar evidence.
        engine.ensure_result_values(result_values)?;
        let probe =
            engine.resolve_pstb_probe(netlist, operating_point, &card.probe_instance, abort)?;
        ensure_not_aborted(abort)?;

        let pss = operating_point.analysis();
        for (index, row) in pss.monodromy.iter().enumerate() {
            poll_periodically(abort, index)?;
            if row.len() != order {
                return Err(SimulationError::Circuit(
                    "PSTB prerequisite PSS returned a non-square monodromy matrix".to_owned(),
                ));
            }
        }
        // A resolved probe indexes a nonempty independent basis. A fully
        // prescribed reactive circuit can have an order-zero carrier, but
        // probe resolution rejects it before any modal participation is read.
        if probe.projection.is_empty() || probe.projection.iter().any(|&(index, _)| index >= order)
        {
            return Err(SimulationError::Circuit(format!(
                "PSTB probe '{}' has no valid dynamic-current projection into the retained monodromy of order {order}",
                probe.canonical_name,
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
        let config = PstbConfig::new()
            .with_orbit_kind(orbit_kind)
            .with_eigenvectors(true)
            .with_stability_threshold(card.stability_threshold)
            .with_subharmonic_detection(card.detect_subharmonics);
        // An event orbit's coordinates are the whole solution, and a
        // generated model's are the nodes its charges read, one of which its
        // own branches may tie to another: the monodromy maps such a
        // coordinate to zero or to a combination of the others, and the
        // spectrum is read over the coordinates the period remembers.
        let reduced;
        let (monodromy, probe) = match remembered_coordinates(
            &pss.monodromy,
            operating_point.shooting_state_basis(),
            &probe,
            operating_point.is_event_orbit(),
        )? {
            Some((matrix, projection)) => {
                reduced = matrix;
                (
                    reduced.as_slice(),
                    ResolvedPstbProbe {
                        canonical_name: probe.canonical_name,
                        projection,
                    },
                )
            }
            None => (pss.monodromy.as_slice(), probe),
        };
        let result =
            PstbAnalyzer::new(config).analyze_monodromy_with_abort(monodromy, pss.period, abort)?;

        let mut probe_participation = Vec::with_capacity(result.multipliers.len());
        for (index, multiplier) in result.multipliers.iter().enumerate() {
            poll_periodically(abort, index)?;
            probe_participation.push(normalized_probe_participation(
                multiplier.eigenvector.as_deref(),
                &probe.projection,
                abort,
            )?);
        }
        ensure_not_aborted(abort)?;

        Ok(PeriodicStabilityResult {
            probe_instance: probe.canonical_name,
            probe_state_index: match probe.projection.as_slice() {
                &[(index, _)] => Some(index),
                _ => None,
            },
            probe_state_projection: probe.projection,
            probe_participation,
            result,
        })
    }

    /// Resolve an authored loop-probe name to its coordinate in the carrier's
    /// shooting-state basis.
    ///
    /// The retained basis contains independent charge-voltage coordinates
    /// followed by inductor currents. Resolving `L:<probe>` by name remains
    /// correct when charge branches share a voltage or add diode coordinates.
    /// Dependent-current projections are resolved from an authenticated circuit when
    /// the authored winding is not the representative named by the basis.
    fn resolve_pstb_probe(
        &self,
        netlist: &Netlist,
        operating_point: &PssOperatingPoint,
        probe_instance: &str,
        abort: &dyn AbortSignal,
    ) -> Result<ResolvedPstbProbe, SimulationError> {
        let probe_name = probe_instance.trim();
        let basis = operating_point.shooting_state_basis();
        // Fully prescribed storage has no free coordinates; transported
        // legacy artifacts also lack a basis. Neither can name a dynamic
        // probe, but only the latter is an authentication failure.
        if basis.is_empty() {
            let reason = if operating_point.producer_identity().is_some() {
                "its storage is fully prescribed and it has no independent dynamic coordinate"
            } else {
                "it is an unauthenticated legacy artifact"
            };
            return Err(SimulationError::Circuit(format!(
                "PSTB probe '{probe_name}' cannot be resolved: the retained PSS operating point \
                 carries no shooting-state basis: {reason}"
            )));
        }

        let engine = self.resolved_for_netlist(netlist);
        operating_point.authenticate_for_reuse(
            netlist,
            &engine.config,
            operating_point.config(),
            crate::identity::AnalysisKind::Pstb,
        )?;

        if operating_point.is_event_orbit() {
            // Every inductor current is a coordinate of an event orbit, named
            // by its branch. An authored inductor the basis names is the
            // probe, and the netlist says so without a circuit being built.
            let prefix = super::pss_event::EVENT_BASIS_PREFIX;
            let coordinate = format!("{prefix}I({probe_name})");
            let index = basis
                .iter()
                .position(|name| name.eq_ignore_ascii_case(&coordinate));
            if let Some(index) = index
                && netlist.elements.iter().any(|element| {
                    element.name.eq_ignore_ascii_case(probe_name)
                        && matches!(element.kind, ElementKind::Inductor { model: None, .. })
                })
                && let Some(canonical_name) = basis[index]
                    .get(prefix.len()..)
                    .and_then(|name| name.strip_prefix("I("))
                    .and_then(|name| name.strip_suffix(')'))
            {
                return Ok(ResolvedPstbProbe {
                    canonical_name: canonical_name.to_owned(),
                    projection: vec![(index, 1.0)],
                });
            }
            // A probe inside a subcircuit or bound to a model, and every
            // refusal, read the circuit.
            let circuit = engine.build_circuit_with_abort(netlist, abort)?;
            let inductor = circuit
                .inductor_probe_names()
                .into_iter()
                .find(|name| name.eq_ignore_ascii_case(probe_name));
            if let Some(canonical_name) = inductor
                && let Some(index) = index
            {
                return Ok(ResolvedPstbProbe {
                    canonical_name,
                    projection: vec![(index, 1.0)],
                });
            }
            return Err(Self::pstb_probe_diagnostic(&circuit, basis, probe_name));
        }
        for (index, coordinate) in basis.iter().enumerate() {
            poll_periodically(abort, index)?;
            if let Some(name) = coordinate.strip_prefix("L:")
                && name.eq_ignore_ascii_case(probe_name)
            {
                return Ok(ResolvedPstbProbe {
                    canonical_name: name.to_owned(),
                    projection: vec![(index, 1.0)],
                });
            }
        }
        let mut circuit = super::pss::PssCircuit::new_with_abort(
            engine.build_circuit_with_abort(netlist, abort)?,
            engine.config.resource_limits,
            abort,
        )?;
        circuit.restore_delay_basis(
            operating_point.shooting_state_basis(),
            engine.config.resource_limits,
        )?;
        if circuit
            .inductor_probe_names()
            .iter()
            .any(|name| name.eq_ignore_ascii_case(probe_name))
        {
            operating_point.validate_shooting_basis_for_circuit(&circuit)?;
            if let Some((canonical_name, projection)) =
                circuit.inductor_probe_projection(probe_name)
            {
                return Ok(ResolvedPstbProbe {
                    canonical_name,
                    projection,
                });
            }
        }
        Err(Self::pstb_probe_diagnostic(&circuit, basis, probe_name))
    }

    /// Name what the deck does offer when a loop probe misses.
    ///
    /// Reuse the circuit inspected for dependent-current projections to distinguish
    /// an absent name from an existing branch that is not an inductor.
    fn pstb_probe_diagnostic(
        circuit: &CircuitData,
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
                    inductor_probes(Some(circuit))
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

/// A coordinate whose column the period maps below this fraction of the
/// largest, each in its unit's scale, is one the orbit does not remember;
/// a column within it of the span of those before it is not independent.
/// The fraction is the accuracy of a differenced monodromy, whose columns
/// for an algebraic coordinate are its traversals' Newton tolerance.
const REMEMBERED: Value = 1.0e-6;

/// A reduced monodromy and the probe's projection onto its coordinates.
type Reduction = (Vec<Vec<Value>>, Vec<(usize, Value)>);

/// The monodromy over the coordinates the period remembers, the probe's
/// first, with the probe's projection there. An event orbit always takes the
/// reduction; a native orbit takes it only where its map is singular by
/// structure (a coordinate's row or column exactly zero, which the analyzer
/// refuses), and with a probe that is one coordinate the period remembers,
/// and keeps its own map otherwise (`None`).
///
/// A coordinate whose row of `Φ` is exactly zero (the period fixes it
/// whatever it starts at) or whose column is (nothing the period remembers
/// reads it) puts `Φ` in block-triangular form with a zero diagonal block,
/// so deleting its row and column removes one zero multiplier and keeps the
/// rest; that is repeated until no such coordinate is left. Of those left,
/// the columns are those of the independent coordinates `S` and
/// combinations `Φe_d = Σ a_ds·Φe_s` of them. Then `Φ = Φ_{:,S}·L` with
/// `(Lz)_s = z_s + Σ_d a_ds·z_d`, and the nonzero spectrum of `Φ` is that of
/// `L·Φ_{:,S} = Φ_{S,S} + Aᵀ·Φ_{D,S}`, an operator on `S` whose eigenvectors
/// carry the probe's own coordinate.
fn remembered_coordinates(
    monodromy: &[Vec<Value>],
    basis: &[String],
    probe: &ResolvedPstbProbe,
    event_orbit: bool,
) -> Result<Option<Reduction>, SimulationError> {
    let n = monodromy.len();
    let mut active: Vec<usize> = (0..n).collect();
    while let Some(position) = active.iter().position(|&k| {
        active.iter().all(|&col| monodromy[k][col] == 0.0)
            || active.iter().all(|&row| monodromy[row][k] == 0.0)
    }) {
        active.remove(position);
    }
    if !event_orbit && active.len() == n {
        return Ok(None);
    }
    let &[(probe_index, _)] = probe.projection.as_slice() else {
        if !event_orbit {
            return Ok(None);
        }
        return Err(SimulationError::Circuit(format!(
            "PSTB probe '{}' is not one coordinate of the event orbit",
            probe.canonical_name
        )));
    };
    // Each coordinate's unit, so columns of volts and amperes compare.
    let scale: Vec<Value> = basis
        .iter()
        .map(|name| {
            let name = name.trim_start_matches(super::pss_event::EVENT_BASIS_PREFIX);
            if name.starts_with("I(") || name.starts_with("L:") {
                1.0e-3
            } else {
                1.0
            }
        })
        .collect();
    // Formed over the coordinates left, in the units' scale.
    let scaled = |row: usize, col: usize| monodromy[row][col] * scale[col] / scale[row];
    let column =
        |col: usize| -> Vec<Value> { active.iter().map(|&row| scaled(row, col)).collect() };
    let norm = |vector: &[Value]| {
        vector
            .iter()
            .map(|value| value * value)
            .sum::<Value>()
            .sqrt()
    };
    let largest = active
        .iter()
        .map(|&col| norm(&column(col)))
        .fold(0.0, Value::max);
    let remembered: Vec<usize> = active
        .iter()
        .copied()
        .filter(|&col| norm(&column(col)) > REMEMBERED * largest)
        .collect();
    if !remembered.contains(&probe_index) {
        if !event_orbit {
            return Ok(None);
        }
        return Err(SimulationError::Circuit(format!(
            "PSTB probe '{}' carries no state the period remembers: its current is fixed by \
             the circuit around it",
            probe.canonical_name
        )));
    }
    let order = std::iter::once(probe_index)
        .chain(remembered.iter().copied().filter(|&col| col != probe_index));
    // Modified Gram-Schmidt, twice, with each kept column's `R`.
    let mut basis_vectors: Vec<Vec<Value>> = Vec::new();
    let mut r_columns: Vec<Vec<Value>> = Vec::new();
    let mut kept = Vec::new();
    let mut dropped = Vec::new();
    for col in order {
        let original = column(col);
        let size = norm(&original);
        let mut residual = original;
        let mut coefficients = vec![0.0; basis_vectors.len()];
        for _ in 0..2 {
            for (q, coefficient) in basis_vectors.iter().zip(&mut coefficients) {
                let dot: Value = q.iter().zip(&residual).map(|(a, b)| a * b).sum();
                *coefficient += dot;
                residual.iter_mut().zip(q).for_each(|(r, q)| *r -= dot * q);
            }
        }
        let left = norm(&residual);
        if left > REMEMBERED * size {
            residual.iter_mut().for_each(|r| *r /= left);
            coefficients.push(left);
            basis_vectors.push(residual);
            r_columns.push(coefficients);
            kept.push(col);
        } else {
            dropped.push((col, coefficients));
        }
    }
    let r = kept.len();
    // `a_d = R⁻¹·Qᵀ·Φe_d`, by back substitution on the kept columns' `R`.
    let combinations: Vec<(usize, Vec<Value>)> = dropped
        .into_iter()
        .map(|(col, mut q_coefficients)| {
            q_coefficients.resize(r, 0.0);
            for i in (0..r).rev() {
                let mut value = q_coefficients[i];
                for j in i + 1..r {
                    value -= r_columns[j][i] * q_coefficients[j];
                }
                q_coefficients[i] = value / r_columns[i][i];
            }
            (col, q_coefficients)
        })
        .collect();
    // Returned in the coordinates' own units.
    let reduced = (0..r)
        .map(|i| {
            (0..r)
                .map(|j| {
                    let entry = scaled(kept[i], kept[j])
                        + combinations
                            .iter()
                            .map(|(d, a)| a[i] * scaled(*d, kept[j]))
                            .sum::<Value>();
                    entry * scale[kept[i]] / scale[kept[j]]
                })
                .collect()
        })
        .collect();
    Ok(Some((reduced, vec![(0, 1.0)])))
}

/// Canonical probe identity and its physical-current projection.
struct ResolvedPstbProbe {
    canonical_name: String,
    projection: Vec<(usize, Value)>,
}

fn validate_card(card: &PstbCard) -> Result<(), SimulationError> {
    if card.probe_instance.trim().is_empty() {
        return Err(SimulationError::Circuit(
            "PSTB requires a loop-probe instance name".to_owned(),
        ));
    }
    if card.max_harmonics == 0 {
        return Err(SimulationError::Circuit(
            "PSTB requires at least one carrier harmonic".to_owned(),
        ));
    }
    if !card.stability_threshold.is_finite() || card.stability_threshold < 1.0 {
        return Err(SimulationError::Circuit(format!(
            "PSTB requires a finite stability threshold of at least one, got {}",
            card.stability_threshold
        )));
    }
    Ok(())
}

/// A normalized physical-current observable, including junction sums. Scale
/// both operands before their norms/dot product so finite extreme components
/// cannot overflow an otherwise bounded, scale-invariant participation.
fn normalized_probe_participation(
    eigenvector: Option<&[num_complex::Complex64]>,
    projection: &[(usize, Value)],
    abort: &dyn AbortSignal,
) -> Result<Value, SimulationError> {
    let vector = eigenvector.ok_or_else(|| SimulationError::Circuit(
        "PSTB solver did not return a requested eigenvector; the probe's participation in the mode cannot be stated".to_owned(),
    ))?;
    let mut scale = 0.0_f64;
    let mut direct_norm = 0.0_f64;
    for (index, value) in vector.iter().enumerate() {
        poll_periodically(abort, index)?;
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(SimulationError::Circuit(
                "PSTB solver returned a non-finite eigenvector".to_owned(),
            ));
        }
        scale = scale.max(value.re.abs()).max(value.im.abs());
        direct_norm = direct_norm.hypot(value.norm());
    }
    if scale == 0.0 {
        return Err(SimulationError::Circuit(
            "PSTB solver returned a zero-norm eigenvector".to_owned(),
        ));
    }
    if let &[(index, weight)] = projection
        && index < vector.len()
        && weight.is_finite()
        && weight != 0.0
        && scale >= Value::MIN_POSITIVE
        && direct_norm.is_finite()
        && direct_norm > 0.0
    {
        return Ok((vector[index].norm() / direct_norm).clamp(0.0, 1.0));
    }
    let mut norm = 0.0_f64;
    for (index, value) in vector.iter().enumerate() {
        poll_periodically(abort, index)?;
        norm = norm.hypot(value.re / scale).hypot(value.im / scale);
    }
    let mut weight_scale = 0.0_f64;
    for (term, &(index, weight)) in projection.iter().enumerate() {
        poll_periodically(abort, term)?;
        if index >= vector.len() || !weight.is_finite() {
            return Err(SimulationError::Circuit(
                "PSTB probe projection has an invalid coordinate or weight".to_owned(),
            ));
        }
        weight_scale = weight_scale.max(weight.abs());
    }
    if weight_scale == 0.0 {
        return Err(SimulationError::Circuit(
            "PSTB probe current has no independent dynamic coordinate".to_owned(),
        ));
    }
    let mut dot = num_complex::Complex64::new(0.0, 0.0);
    let mut correction = num_complex::Complex64::new(0.0, 0.0);
    let mut weight_norm = 0.0_f64;
    for (term, &(index, weight)) in projection.iter().enumerate() {
        poll_periodically(abort, term)?;
        let weight = weight / weight_scale;
        weight_norm = weight_norm.hypot(weight);
        crate::numerics::compensated_add(
            &mut dot.re,
            &mut correction.re,
            (vector[index].re / scale) * weight,
        );
        crate::numerics::compensated_add(
            &mut dot.im,
            &mut correction.im,
            (vector[index].im / scale) * weight,
        );
    }
    let ratio = (dot + correction).norm() / norm / weight_norm;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;
    use num_complex::Complex64;

    #[test]
    fn physical_probe_participation_is_scale_invariant_even_at_float_extremes() {
        for scale in [1.0, 1e308, 1e-308, f64::from_bits(1)] {
            let vector = [Complex64::new(scale, scale), Complex64::new(scale, -scale)];
            for projection in [vec![(0, 1.0)], vec![(0, 1.0), (1, -1.0)]] {
                let actual =
                    normalized_probe_participation(Some(&vector), &projection, &NoAbort).unwrap();
                assert!(
                    (actual - std::f64::consts::FRAC_1_SQRT_2).abs() < 4.0 * f64::EPSILON,
                    "scale={scale:e}, participation={actual}"
                );
            }
        }
    }

    #[test]
    fn a_junction_probe_retains_the_small_current_left_after_modal_cancellation() {
        let vector = [
            Complex64::new(1.0, 0.0),
            Complex64::new(1e-20, 0.0),
            Complex64::new(-1.0, 0.0),
        ];
        let actual = normalized_probe_participation(
            Some(&vector),
            &[(0, 1.0), (1, 1.0), (2, 1.0)],
            &NoAbort,
        )
        .unwrap();
        let expected = 1e-20 / 6.0_f64.sqrt();
        assert!((actual / expected - 1.0).abs() < 4.0 * f64::EPSILON);
    }
}
