//! One frozen small-signal descriptor shared by transfer and natural-pole studies.
use super::xspice::RationalDescriptor;
use super::*;
use crate::analysis::pole_zero::PoleSpectrum;
use crate::engine::progress::StudyProgress;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use crate::solver::{ComplexMatrix, StaticMatrix};

pub(super) struct LinearizedDescriptor<'a> {
    pub g: ComplexMatrix,
    pub c: ComplexMatrix,
    circuit: &'a CircuitData,
    operating_point: &'a [Value],
    xspice: Vec<RationalDescriptor<'a>>,
}

impl LinearizedDescriptor<'_> {
    pub(super) fn has_external_states(&self) -> bool {
        self.circuit
            .bjts
            .devices
            .iter()
            .any(|bjt| bjt.uses_vbic_dynamic_charges())
            || Engine::pz_ac_nqs_state_count(self.circuit) != 0
            || !self.xspice.is_empty()
    }

    pub(super) fn into_analyzer(
        self,
        limits: ResourceLimits,
    ) -> Result<PoleZeroAnalyzer, SimulationError> {
        // Admit dense workspace only when the caller actually chooses this
        // path. Sparse transfer extraction must retain its own admission policy.
        let order = self
            .g
            .nrows
            .saturating_add(Engine::pz_ac_nqs_state_count(self.circuit))
            .saturating_add(
                self.xspice
                    .iter()
                    .map(RationalDescriptor::state_count)
                    .fold(0usize, usize::saturating_add),
            );
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            order.saturating_mul(order.saturating_mul(8).saturating_add(1)),
            limits.max_result_values,
        )?;
        let mut g = Matrix::from_dense(self.g.to_dense_real());
        let mut c = Matrix::from_dense(self.c.to_dense_imag());
        Engine::stamp_vbic_pz_descriptor_states(self.circuit, self.operating_point, &mut g, &mut c);
        Engine::stamp_ac_nqs_pz_descriptor_states(
            self.circuit,
            self.operating_point,
            &mut g,
            &mut c,
        )?;
        RationalDescriptor::stamp_all(&self.xspice, self.circuit, &mut g, &mut c);
        Ok(PoleZeroAnalyzer::new(g, c).with_resource_limits(limits))
    }
}

pub(super) fn extraction_error(error: PoleZeroAnalysisError) -> SimulationError {
    match error {
        PoleZeroAnalysisError::Aborted => SimulationError::Aborted,
        PoleZeroAnalysisError::ResourceLimit(error) => SimulationError::ResourceLimit(error),
        error => SimulationError::Solver(crate::solver::SolverError::InvalidCircuit(format!(
            "pole-zero extraction failed: {error}"
        ))),
    }
}

impl Engine {
    pub(super) fn ensure_pz_circuit(circuit: &CircuitData) -> Result<(), SimulationError> {
        Self::ensure_no_mixed_signal_analysis(circuit, "pole-zero analysis")?;
        Self::ensure_supported_dynamic_charges(circuit, "Pole-zero")?;
        Self::ensure_supported_pz_dynamic_state_descriptors(circuit)
    }

    /// The caller has prepared device caches at this accepted operating point.
    /// No circuit construction, bias solve or excitation selection occurs here.
    pub(super) fn linearized_pz_descriptor<'a>(
        circuit: &'a CircuitData,
        matrix: &StaticMatrix,
        operating_point: &'a [Value],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<LinearizedDescriptor<'a>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let xspice = RationalDescriptor::collect(circuit)?;
        let order = circuit
            .matrix_size()
            .saturating_add(Self::pz_ac_nqs_state_count(circuit))
            .saturating_add(
                xspice
                    .iter()
                    .map(RationalDescriptor::state_count)
                    .fold(0usize, usize::saturating_add),
            );
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            order,
            limits.max_matrix_unknowns,
        )?;
        let g = Self::try_build_small_signal_pz_matrix(circuit, matrix, operating_point, 0.0)?;
        let c = Self::try_build_small_signal_pz_matrix(circuit, matrix, operating_point, 1.0)?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(LinearizedDescriptor {
            g,
            c,
            circuit,
            operating_point,
            xspice,
        })
    }

    /// Extract natural poles from an existing accepted bias. In particular,
    /// STB can reuse its circuit and operating point without inventing a port
    /// or running another operating-point solve.
    pub(in crate::engine) fn pole_spectrum_at_bias(
        circuit: &mut CircuitData,
        matrix: &StaticMatrix,
        operating_point: &[Value],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<PoleSpectrum, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Self::ensure_pz_circuit(circuit)?;
        circuit.refresh_jiles_atherton_inductances(operating_point);
        Self::prepare_small_signal_state(circuit, operating_point)?;
        let spectrum =
            Self::linearized_pz_descriptor(circuit, matrix, operating_point, limits, abort)?
                .into_analyzer(limits)?
                .pole_spectrum_with_abort(abort)
                .map_err(extraction_error)?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            spectrum.poles.len().saturating_mul(2).saturating_add(4),
            limits.max_result_values,
        )?;
        Ok(spectrum)
    }

    /// Compute the complete natural pole spectrum of the admitted small-signal
    /// circuit model. No transfer ports or gain calculations are required.
    /// Frequencies are in rad/s; the certificate retains infinite multiplicity.
    pub fn run_pole_spectrum(&self, netlist: &Netlist) -> Result<PoleSpectrum, SimulationError> {
        self.run_pole_spectrum_with_abort(netlist, &NoAbort)
    }

    /// Cancellable natural-pole analysis with one construction and one bias solve.
    pub fn run_pole_spectrum_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<PoleSpectrum, SimulationError> {
        let progress = StudyProgress::new(abort)?;
        let setup = progress.stage(0.0, 0.2);
        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit_with_abort(netlist, &setup)?;
        Self::ensure_pz_circuit(&circuit)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        progress.report(0.2)?;
        let bias = progress.stage(0.2, 0.6);
        let operating_point = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            &bias,
        )?;
        progress.report(0.6)?;
        let extraction = progress.stage(0.6, 0.99);
        let spectrum = Self::pole_spectrum_at_bias(
            &mut circuit,
            &matrix,
            &operating_point,
            engine.config.resource_limits,
            &extraction,
        )?;
        progress.complete(spectrum)
    }
}
