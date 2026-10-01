//! Retained periodic carrier engine state and admission checks.

pub use rspice_simulation_contract::periodic_carrier::PeriodicCarrier;

/// The retained large-signal solution a small-signal periodic run reads.
///
/// [`PeriodicCarrier`] is the *family* a request names; this is the converged
/// state of one instance of that family, as the plan's typed artifact handed
/// it over. The two are checked against each other exactly once, in
/// [`Self::accepted_by`], because that is the only place both are in hand: a
/// request linearized about a solution other than the one it named is a
/// different measurement reported under this one's name.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PeriodicCarrierState<'a> {
    /// A converged shooting `.PSS` orbit.
    Shooting(&'a rspice_core::engine::PssOperatingPoint),
    /// A converged `.HB` spectral state.
    HarmonicBalance(&'a rspice_core::engine::HbOperatingPoint),
}

impl PeriodicCarrierState<'_> {
    /// The family this state belongs to.
    pub(crate) fn family(self) -> PeriodicCarrier {
        match self {
            Self::Shooting(_) => PeriodicCarrier::Pss,
            Self::HarmonicBalance(_) => PeriodicCarrier::Hb,
        }
    }

    /// The fundamental the engine builds a conversion basis on for this state.
    ///
    /// Read off the carrier, never off the request: `prepare_periodic_ac` in
    /// `rspice-core/src/engine/hb/periodic_ac.rs` replaces the authored
    /// fundamental with this number before it solves anything, and the result
    /// is constructed from it. A shooting solve of an autonomous carrier moves
    /// its own period, and a harmonic-balance state carries the basis its
    /// producer froze.
    pub(crate) fn fundamental(self) -> rspice_core::Value {
        match self {
            Self::Shooting(point) => point.analysis().result.frequency,
            Self::HarmonicBalance(point) => point.config().fundamental_freq,
        }
    }

    /// The engine tolerance a run about this state must resolve with.
    ///
    /// A retained harmonic-balance state authenticates the *resolved engine
    /// configuration* it was produced under
    /// (`HbOperatingPointIdentity::resolved_simulation_identity`), and a
    /// consumer that resolved a different tolerance is refused before its
    /// numerical reuse — "retained HB resolved simulation configuration does
    /// not match the current engine configuration". So the tolerance comes off
    /// the carrier, exactly as
    /// `run_hbnoise_analysis_from_hb_with_source_path_and_abort` takes it.
    ///
    /// A shooting carrier keeps the authored value: its own artifact check
    /// compares the request's tolerance against the producer's, so reading it
    /// off the state would make that comparison tautological.
    pub(crate) fn engine_tolerance(self, authored: rspice_core::Value) -> rspice_core::Value {
        match self {
            Self::Shooting(_) => authored,
            Self::HarmonicBalance(point) => point.config().tolerance,
        }
    }

    /// Whether the carrier a request named admits this state, or the refusal
    /// that names both.
    ///
    /// `Preceding` writes no `FROM=` keyword and admits either family, which
    /// is what the engine's `resolve_periodic_source` does with it. The two
    /// named positions admit only their own.
    pub(crate) fn accepted_by(
        self,
        carrier: PeriodicCarrier,
        directive: &str,
    ) -> Result<(), String> {
        let family = self.family();
        if matches!(carrier, PeriodicCarrier::Preceding) || carrier == family {
            return Ok(());
        }
        Err(format!(
            "{directive} states {} but was handed a {} carrier; a small-signal run linearized \
             about a solution it did not name reports a different measurement",
            carrier
                .spice_name()
                .map_or_else(|| "no carrier".to_owned(), |name| format!("from={name}")),
            match family {
                PeriodicCarrier::Hb => "harmonic-balance",
                _ => "shooting-PSS",
            }
        ))
    }
}
