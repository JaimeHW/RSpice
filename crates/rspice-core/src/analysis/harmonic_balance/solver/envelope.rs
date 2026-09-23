//! Slow-time charge/flux evolution using the registered physical F/Q model.
use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicError as Error, QuasiPeriodicSolution, QuasiPeriodicSolveConfig,
    SpectralEnvelopeMethod, SpectralEnvelopeState, SpectralEnvelopeEvent,
    SpectralEnvelopeEventConfig, SpectralEnvelopeEventEquation,
    solve::{JacobianEntry, envelope},
};

impl HbSolver {
    /// Resolve a sided source event in the retained Fourier basis. The
    /// structural event equations must be prepared from the physical circuit,
    /// not inferred from a Jacobian's numerical rank at one bias point.
    #[expect(
        clippy::too_many_arguments,
        reason = "event topology, sided forcing, solver policy and resources are independent"
    )]
    pub fn transition_spectral_envelope_with_abort(
        &mut self,
        previous: &SpectralEnvelopeState,
        rows: &[SpectralEnvelopeEventEquation],
        config: &SpectralEnvelopeEventConfig,
        sources: &[Vec<Complex64>],
        slow_source_rates: &[Vec<Complex64>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<SpectralEnvelopeEvent, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        let storage = self.envelope_storage()?;
        envelope::event::transition(
            self,
            previous,
            rows,
            config,
            sources,
            slow_source_rates,
            &storage,
            limits,
            abort,
        )
    }

    pub(crate) fn validate_spectral_envelope_circuit(&self) -> Result<(), Error> {
        self.envelope_storage().map(|_| ())
    }

    fn envelope_storage(&self) -> Result<Vec<JacobianEntry>, Error> {
        self.validate_quasi_periodic_circuit()?;
        // A distributed Y(jw) needs slow-time convolution/state equations.
        // dY/dw alone would approximate it and lose delayed physical history.
        if !self.exact_periodic_networks.is_empty() {
            return Err(Error::InvalidCircuit(
                "multirate Envelope requires explicit distributed-network memory states".into(),
            ));
        }
        if self.quasi_prescribed_integrals.is_some()
            || self.prescribed_integrals.iter().any(Option::is_some)
        {
            return Err(Error::InvalidCircuit(
                "multirate Envelope requires uneliminated physical integral equations".into(),
            ));
        }
        let mut storage = self.c_matrix.clone();
        for (index, branch) in self.periodic_mna_branches.iter().enumerate() {
            if let ExactMnaBranch::Inductor { inductance, .. } = branch {
                let coordinate = self.num_nodes + index;
                storage.push((coordinate, coordinate, -inductance));
            }
        }
        storage.extend(
            self.exact_mna_inductance_entries
                .iter()
                .map(|&(row, col, value)| (row, col, -value)),
        );
        Ok(storage)
    }

    /// Capture the physical charge and flux of an initial periodic solution.
    /// The caller must retain the same circuit coordinate order throughout the
    /// mission. This numerical state is not an authenticated engine artifact.
    pub fn initialize_spectral_envelope_with_abort(
        &mut self,
        time: Value,
        initial: &QuasiPeriodicSolution,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<SpectralEnvelopeState, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        let storage = self.envelope_storage()?;
        envelope::initialize(self, time, initial, &storage, limits, abort)
    }

    /// Advance carrier coefficients directly in slow time. `sources` is the
    /// full signed Fourier-series MNA right hand side at the requested time;
    /// no RF cycles are integrated. Physical charge history survives changes
    /// to source amplitudes or circuit coefficients between accepted steps.
    /// Failure leaves the caller's accepted state available for retry.
    #[expect(
        clippy::too_many_arguments,
        reason = "physical history, solver policy and resources are independent"
    )]
    pub fn step_spectral_envelope_with_abort(
        &mut self,
        previous: &SpectralEnvelopeState,
        time: Value,
        method: SpectralEnvelopeMethod,
        config: &QuasiPeriodicSolveConfig,
        sources: &[Vec<Complex64>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<SpectralEnvelopeState, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        let storage = self.envelope_storage()?;
        envelope::step(
            self, previous, time, method, config, sources, &storage, limits, abort,
        )
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod event_tests;
