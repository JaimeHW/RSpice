//! Periodic port scattering from one authenticated circuit and linearization.

mod noise;

pub use noise::PspNoiseCorrelation;

use super::periodic_ac::{PacOperatingPoint, PeriodicAcOutput, PreparedPeriodicAc};
use super::*;
use crate::analysis::HbError;
use crate::analysis::harmonic_balance::{
    PeriodicAcExcitation, PeriodicNoiseSource, PeriodicSidebandWindow,
};
use crate::analysis::pac::PacConfig;
use crate::analysis::s_param::{
    NetworkError, PortRealization, SMatrix, SParameterPort, s_column_from_port_voltages,
};

/// Periodic scattering at the authored, real power-wave reference impedances.
#[derive(Debug)]
pub struct PspAnalysisResult {
    /// Physical ports in ascending port-number order, after elaboration.
    pub ports: Vec<SParameterPort>,
    /// Actual frequency of the retained periodic operating point, in hertz.
    pub fundamental_freq: Value,
    /// First retained sideband.
    pub sideband_min: i32,
    /// Last retained sideband (inclusive).
    pub sideband_max: i32,
    /// One lifted scattering matrix per offset frequency. Rows and columns
    /// are physical-port-major, then ascending sideband, both one-based in
    /// `SMatrix`. Thus `(port_index * sideband_count + sideband_offset + 1)`
    /// identifies a wave channel; it does not identify an additional circuit port.
    pub data: Vec<SMatrix>,
    /// Intrinsic device noise waves at the same authored references and
    /// channel order as `data`. External port-termination noise is excluded.
    pub noise: Option<Vec<PspNoiseCorrelation>>,
}

/// Authenticated periodic setup, ready to sweep all physical ports together.
/// Port selection can be validated through [`Self::ports`] before solving.
pub struct PreparedPsp {
    noise_limits: crate::ResourceLimits,
    solver: HbSolver,
    state: HbSolverState,
    ports: Vec<SParameterPort>,
    nodes: Vec<(Option<usize>, Option<usize>)>,
    branch_drives: Vec<[(usize, Complex64); 1]>,
    physical_references: Vec<Value>,
    authored_references: Vec<Value>,
    frequencies: Vec<Value>,
    config: PacConfig,
    lifted_unknowns: usize,
    noise_sources: Option<Vec<PeriodicNoiseSource>>,
}

impl Engine {
    /// Prepare PSP from a retained shooting orbit. The PAC configuration
    /// supplies sweep/sideband settings; input/output selectors and amplitude
    /// do not select or scale the port scattering matrix.
    pub fn prepare_psp_from_pss_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        point: &super::super::PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPsp, SimulationError> {
        self.resolved_for_netlist(netlist).prepare_psp(
            netlist,
            config,
            PacOperatingPoint::Shooting(point),
            false,
            abort,
        )
    }

    /// Prepare HBSP directly from a retained harmonic-balance state.
    /// See [`Self::prepare_psp_from_pss_with_abort`] for the sweep contract.
    pub fn prepare_psp_from_hb_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPsp, SimulationError> {
        self.resolved_for_netlist(netlist).prepare_psp(
            netlist,
            config,
            PacOperatingPoint::HarmonicBalance(point),
            false,
            abort,
        )
    }

    /// Prepare shooting PSP with optional intrinsic correlated noise waves.
    pub fn prepare_psp_from_pss_with_noise_and_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        point: &super::super::PssOperatingPoint,
        noise: bool,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPsp, SimulationError> {
        self.resolved_for_netlist(netlist).prepare_psp(
            netlist,
            config,
            PacOperatingPoint::Shooting(point),
            noise,
            abort,
        )
    }

    /// Prepare HBSP with optional intrinsic correlated noise waves.
    pub fn prepare_psp_from_hb_with_noise_and_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        point: &HbOperatingPoint,
        noise: bool,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPsp, SimulationError> {
        self.resolved_for_netlist(netlist).prepare_psp(
            netlist,
            config,
            PacOperatingPoint::HarmonicBalance(point),
            noise,
            abort,
        )
    }

    fn prepare_psp(
        &self,
        netlist: &Netlist,
        mut config: PacConfig,
        point: PacOperatingPoint<'_>,
        noise: bool,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPsp, SimulationError> {
        let PreparedPeriodicAc {
            circuit,
            mut solver,
            state,
            node_names,
            rf_ports,
            lifted_unknowns,
            ..
        } = self.prepare_periodic_ac(
            netlist,
            &mut config,
            Some(point),
            PeriodicAcOutput::PortScattering,
            abort,
        )?;
        let sidebands = config.num_sidebands();
        let mut ports = Vec::with_capacity(rf_ports.len());
        let mut nodes = Vec::with_capacity(rf_ports.len());
        let mut branch_drives = Vec::with_capacity(rf_ports.len());
        let mut physical_references = Vec::new();
        let mut authored_references = Vec::new();
        let node_row = |name: &str| -> Result<Option<usize>, SimulationError> {
            if netlist.ground_policy().is_ground(name) {
                return Ok(None);
            }
            node_names
                .iter()
                .position(|node| node.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "PSP port node '{name}' is missing from the periodic circuit"
                    ))
                })
        };
        let excluded_resistors = rf_ports
            .iter()
            .map(|port| port.termination.clone())
            .collect::<Vec<_>>();
        for materialized in rf_ports {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let port = &materialized.port;
            if port.realization != PortRealization::Thevenin {
                return Err(SimulationError::Circuit(format!(
                    "PSP port '{}' requires a physical Thevenin termination in its producer circuit",
                    port.source_name
                )));
            }
            let resistance =
                super::super::sp::termination_impedance(&circuit, &materialized, abort)?;
            let input = Self::pac_input_port(&circuit, &port.source_name, node_names.len())?;
            let branch = input
                .voltage_source_index
                .and_then(|index| solver.periodic_voltage_source_branch(index))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "PSP port '{}' has no periodic voltage-source branch",
                        port.source_name
                    ))
                })?;
            nodes.push((node_row(&port.node_pos)?, node_row(&port.node_neg)?));
            branch_drives.push([(branch, Complex64::new(1.0, 0.0))]);
            physical_references.extend(std::iter::repeat_n(resistance, sidebands));
            authored_references.extend(std::iter::repeat_n(port.z0, sidebands));
            ports.push(materialized.port);
        }
        let frequencies =
            config
                .try_frequency_points_with_abort(abort)
                .map_err(|error| match error {
                    crate::analysis::FrequencyGridError::Aborted => SimulationError::Aborted,
                    error => {
                        SimulationError::Circuit(format!("Invalid PSP frequency sweep: {error}"))
                    }
                })?;
        let noise_sources = if noise {
            let channels = ports.len().checked_mul(sidebands).ok_or_else(|| {
                SimulationError::Circuit(
                    "PSP noise channel dimensions overflow this platform".into(),
                )
            })?;
            // Account for scattering plus covariance (two complex sweeps),
            // and bound the adjoints retained by the joint noise calculation.
            self.ensure_result_values(
                channels
                    .checked_mul(channels)
                    .and_then(|n| n.checked_mul(frequencies.len()))
                    .and_then(|n| n.checked_mul(4))
                    .ok_or_else(|| {
                        SimulationError::Circuit(
                            "PSP noise result dimensions overflow this platform".into(),
                        )
                    })?,
            )?;
            self.ensure_result_values(
                channels
                    .checked_mul(lifted_unknowns)
                    .and_then(|n| n.checked_mul(2))
                    .ok_or_else(|| {
                        SimulationError::Circuit(
                            "PSP noise adjoint dimensions overflow this platform".into(),
                        )
                    })?,
            )?;
            Some(self.prepare_periodic_noise_sources(
                &circuit,
                &mut solver,
                &state,
                &excluded_resistors,
                abort,
            )?)
        } else {
            None
        };
        Ok(PreparedPsp {
            noise_limits: self.config.resource_limits,
            solver,
            state,
            ports,
            nodes,
            branch_drives,
            physical_references,
            authored_references,
            frequencies,
            config,
            lifted_unknowns,
            noise_sources,
        })
    }
}

impl PreparedPsp {
    /// Physical port planes and authored wave references from the circuit
    /// that was authenticated against the retained operating point.
    pub fn ports(&self) -> &[SParameterPort] {
        &self.ports
    }

    /// Exact offset-frequency grid that will be solved, in hertz.
    pub fn frequencies(&self) -> &[Value] {
        &self.frequencies
    }

    /// Solve every input port/sideband using a shared periodic linearization
    /// and factorization at each frequency. Only port waves are retained.
    pub fn run_with_abort(
        mut self,
        abort: &dyn AbortSignal,
    ) -> Result<PspAnalysisResult, SimulationError> {
        let sidebands = self.config.num_sidebands();
        let channels = self.physical_references.len(); // checked by preparation's result budget
        let mut excitations = Vec::with_capacity(channels);
        let mut branch_voltages: Vec<&[(usize, Complex64)]> = Vec::with_capacity(channels);
        for drive in &self.branch_drives {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            for sideband in self.config.sideband_min..=self.config.sideband_max {
                excitations.push(PeriodicAcExcitation {
                    sideband,
                    injections: Vec::new(),
                });
                branch_voltages.push(drive.as_slice());
            }
        }
        let mut data = Vec::with_capacity(self.frequencies.len());
        let mut noise = self
            .noise_sources
            .as_ref()
            .map(|_| Vec::with_capacity(self.frequencies.len()));
        let mut voltages = Vec::with_capacity(channels);
        for frequency in self.frequencies {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut matrix = SMatrix::new(frequency, channels);
            self.solver
                .solve_periodic_ac_each_with_branch_voltages(
                    &self.state,
                    PeriodicSidebandWindow {
                        offset_hz: frequency,
                        sideband_min: self.config.sideband_min,
                        sideband_max: self.config.sideband_max,
                    },
                    &excitations,
                    &branch_voltages,
                    |column, solution| {
                        if abort.is_aborted() {
                            return Err(HbError::Aborted);
                        }
                        if column >= channels || solution.len() != self.lifted_unknowns {
                            return Err(HbError::InvalidCircuit(
                                "PSP periodic solution dimensions differ from the prepared circuit"
                                    .into(),
                            ));
                        }
                        if solution
                            .iter()
                            .any(|v| !v.re.is_finite() || !v.im.is_finite())
                        {
                            return Err(HbError::InvalidCircuit(
                                "PSP periodic solution contains a non-finite value".into(),
                            ));
                        }
                        voltages.clear();
                        for &(positive, negative) in &self.nodes {
                            if abort.is_aborted() {
                                return Err(HbError::Aborted);
                            }
                            for band in 0..sidebands {
                                let voltage = positive
                                    .map_or(Complex64::ZERO, |n| solution[n * sidebands + band])
                                    - negative.map_or(Complex64::ZERO, |n| {
                                        solution[n * sidebands + band]
                                    });
                                voltages.push(voltage);
                            }
                        }
                        // Treat each sideband as a wave channel, using the
                        // same range-safe extraction as ordinary SP. Only the
                        // excited port AND sideband subtracts the incident wave.
                        let waves = s_column_from_port_voltages(
                            &voltages,
                            column,
                            &self.physical_references,
                        )
                        .map_err(|error| {
                            HbError::InvalidCircuit(format!("PSP wave extraction failed: {error}"))
                        })?;
                        for (row, wave) in waves.into_iter().enumerate() {
                            if row % 256 == 0 && abort.is_aborted() {
                                return Err(HbError::Aborted);
                            }
                            matrix.set(row + 1, column + 1, wave);
                        }
                        Ok(())
                    },
                )
                .map_err(|error| match error {
                    HbError::Aborted => SimulationError::Aborted,
                    error => SimulationError::Circuit(format!("PSP solve failed: {error}")),
                })?;
            matrix
                .renormalize_with_abort(&self.physical_references, &self.authored_references, abort)
                .map_err(|error| match error {
                    NetworkError::Aborted => SimulationError::Aborted,
                    error => {
                        SimulationError::Circuit(format!("PSP wave conversion failed: {error}"))
                    }
                })?;
            if let (Some(sources), Some(points)) = (&self.noise_sources, &mut noise) {
                points.push(noise::solve_noise_waves(
                    &mut self.solver,
                    &self.state,
                    &self.config,
                    &matrix,
                    &self.nodes,
                    &self.physical_references,
                    sources,
                    &self.noise_limits,
                    abort,
                    &self.authored_references,
                )?);
            }
            data.push(matrix);
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(PspAnalysisResult {
            ports: self.ports,
            fundamental_freq: self.config.fundamental_freq,
            sideband_min: self.config.sideband_min,
            sideband_max: self.config.sideband_max,
            data,
            noise,
        })
    }
}
