//! Shared authenticated circuit and spectral-state preparation for PAC and PSP.
//!
//! Consumers bind their observations after one elaboration. A retained state
//! is authenticated against the same resolved circuit configuration used to
//! register exact MNA devices and branches.

use super::*;
use crate::analysis::pac::PacConfig;
use crate::analysis::s_param::MaterializedRfPort;

pub(super) enum PacOperatingPoint<'a> {
    Shooting(&'a super::super::PssOperatingPoint),
    HarmonicBalance(&'a HbOperatingPoint),
}

pub(super) enum PeriodicAcOutput {
    NodeSpectra,
    PortScattering,
}

pub(super) struct PreparedPeriodicAc {
    pub circuit: CircuitData,
    pub solver: HbSolver,
    pub state: HbSolverState,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub rf_ports: Vec<MaterializedRfPort>,
    pub lifted_unknowns: usize,
}

impl Engine {
    pub(super) fn prepare_periodic_ac(
        &self,
        netlist: &Netlist,
        config: &mut PacConfig,
        operating_point: Option<PacOperatingPoint<'_>>,
        output: PeriodicAcOutput,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedPeriodicAc, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(operating_point) = &operating_point {
            config.fundamental_freq = match operating_point {
                PacOperatingPoint::Shooting(point) => point.analysis().result.frequency,
                PacOperatingPoint::HarmonicBalance(point) => point.config().fundamental_freq,
            };
        }
        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(SimulationError::Circuit(
                "PAC requires a positive fundamental frequency".to_string(),
            ));
        }
        config
            .validate()
            .map_err(|e| SimulationError::Circuit(format!("Invalid PAC config: {e}")))?;
        let frequency_count = config.frequency_point_count().map_err(|error| {
            SimulationError::Circuit(format!("Invalid PAC frequency sweep: {error}"))
        })?;
        self.ensure_analysis_points(frequency_count)?;
        let sideband_count = config.num_sidebands();
        self.ensure_analysis_points(sideband_count)?;
        let result_record_count = frequency_count.checked_mul(sideband_count).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "PAC result grid {frequency_count} frequencies x {sideband_count} sidebands overflows this platform"
            ))
        })?;
        self.ensure_analysis_points(result_record_count)?;

        // The operating point needs enough harmonics that every conversion
        // coupling G[k-m] over the sideband span exists, with headroom for
        // the drive itself. Compute in i64 so extreme public i32 bounds
        // cannot overflow before the resource policy rejects them.
        let span = usize::try_from(
            (i64::from(config.sideband_max) - i64::from(config.sideband_min)).unsigned_abs(),
        )
        .unwrap_or(usize::MAX);
        let extreme = usize::try_from(
            i64::from(config.sideband_min)
                .unsigned_abs()
                .max(i64::from(config.sideband_max).unsigned_abs()),
        )
        .unwrap_or(usize::MAX);
        let op_harmonics = span.max(extreme).max(8);
        self.ensure_analysis_points(op_harmonics.saturating_add(1))?;
        if let Some(operating_point) = &operating_point
            && op_harmonics
                > match operating_point {
                    PacOperatingPoint::Shooting(point) => point.spectral_harmonic_capacity(),
                    PacOperatingPoint::HarmonicBalance(point) => point.spectral_harmonic_capacity(),
                }
        {
            let capacity = match operating_point {
                PacOperatingPoint::Shooting(point) => point.spectral_harmonic_capacity(),
                PacOperatingPoint::HarmonicBalance(point) => point.spectral_harmonic_capacity(),
            };
            return Err(SimulationError::Circuit(format!(
                "PAC requires {op_harmonics} periodic harmonics for its sideband span, but the retained periodic state has capacity {}",
                capacity
            )));
        }

        let mut hb_config = match &operating_point {
            Some(PacOperatingPoint::HarmonicBalance(point)) => point.config().clone(),
            _ => HbConfig::new(config.fundamental_freq)
                .with_harmonics(op_harmonics)
                .with_oversample(4),
        };
        // PAC's tolerances govern an operating point solved here. Linearized
        // sideband systems use the periodic solver's own residual certification.
        if !matches!(operating_point, Some(PacOperatingPoint::HarmonicBalance(_))) {
            hb_config.tolerance = config.reltol;
            hb_config.abstol = config.abstol;
        }
        let hb_config = self.hb_config_for_netlist(netlist, hb_config)?;
        self.hb_validate_config(&hb_config)?;
        if let Some(PacOperatingPoint::HarmonicBalance(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, &hb_config)?;
        }
        if let Some(PacOperatingPoint::Shooting(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, point.config())?;
        }

        let (circuit, rf_ports) = self.build_circuit_with_rf_ports(netlist, &[], abort)?;
        // Every periodic small-signal analysis prepared here refuses a mixed
        // Verilog-AMS module for the same reason AC, noise, PSS and HB do: the
        // module's discrete half is executed by a transient event interleave,
        // which has no periodic steady state to linearize around. The refusal
        // has to happen here rather than in each caller, because this is the
        // only place the circuit exists before the periodic operating point is
        // either projected from a retained carrier or solved outright — and
        // without it the harmonic system is assembled with the host's
        // equations simply absent, which answers rather than refuses.
        Self::ensure_no_mixed_signal_analysis(
            &circuit,
            match &output {
                PeriodicAcOutput::NodeSpectra => "PAC analysis",
                PeriodicAcOutput::PortScattering => "PSP analysis",
            },
        )?;
        let hb_config = match &operating_point {
            Some(PacOperatingPoint::HarmonicBalance(_)) => hb_config,
            point => self.hb_config_for_dependent_sources(
                &circuit,
                hb_config,
                match point {
                    Some(PacOperatingPoint::Shooting(point)) => {
                        Some(point.spectral_harmonic_capacity())
                    }
                    _ => None,
                },
                abort,
            )?,
        };
        let op_harmonics = hb_config.num_harmonics;
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        let periodic_branches = circuit
            .num_branches()
            .checked_add(Self::hb_periodic_extra_branch_count(&circuit)?)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "PAC canonical and distributed-network branch count overflows this platform"
                        .to_string(),
                )
            })?;
        let periodic_unknowns = num_nodes.checked_add(periodic_branches).ok_or_else(|| {
            SimulationError::Circuit(
                "PAC periodic node and branch count overflows this platform".to_string(),
            )
        })?;
        let lifted_unknowns = periodic_unknowns.checked_mul(sideband_count).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "PAC lifted dimension {periodic_unknowns} MNA unknowns x {sideband_count} sidebands overflows this platform"
            ))
        })?;
        self.ensure_matrix_unknowns(lifted_unknowns)?;
        let retained_complex_values = match output {
            PeriodicAcOutput::NodeSpectra => result_record_count
                .checked_mul(periodic_unknowns)
                .and_then(|spectra| {
                    let conversion = if config.output_node.is_some() {
                        frequency_count
                            .checked_mul(sideband_count)?
                            .checked_mul(sideband_count)?
                    } else {
                        0
                    };
                    spectra.checked_add(conversion)
                }),
            PeriodicAcOutput::PortScattering => {
                if rf_ports.is_empty() {
                    return Err(crate::analysis::s_param::PortError::NoPortsDeclared.into());
                }
                let columns = rf_ports.len().checked_mul(sideband_count);
                columns
                    .and_then(|n| n.checked_mul(n))
                    .and_then(|n| n.checked_mul(frequency_count))
            }
        }
        .ok_or_else(|| {
            SimulationError::Circuit("periodic AC result dimensions overflow this platform".into())
        })?;
        self.ensure_result_values(retained_complex_values.checked_mul(2).ok_or_else(|| {
            SimulationError::Circuit(
                "periodic AC scalar-value count overflows this platform".into(),
            )
        })?)?;
        if let Some(summary) =
            periodic_capability::summarize(&periodic_capability::periodic_residual_gaps(&circuit))
        {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }
        if let Some(summary) =
            periodic_capability::summarize(&periodic_capability::periodic_descriptor_gaps(&circuit))
        {
            return Err(SimulationError::unsupported_capability(
                "analysis.pac.periodic_mna",
                format!(
                    "PAC exact periodic MNA is unavailable because the circuit contains {summary}"
                ),
            ));
        }

        let drive_tones = Self::hb_collect_drive_tones(&hb_config)?;

        let mut solver = self.new_hb_solver(hb_config.clone(), num_nodes)?;
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        // Use one canonical exact-MNA solver for both the large-signal
        // operating point and its periodic small-signal linearization. The
        // authored source spectra must be registered before the canonical
        // V/L/R branch map so its voltage-source descriptors retain the same
        // large-signal constraints Newton solves. Keeping one registry also
        // makes branch identity drift between the producer and consumer
        // structurally impossible.
        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_voltage_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;
        self.hb_stamp_periodic_mna_branches(&circuit, &mut solver)?;
        self.hb_stamp_current_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;

        let has_nonlinear = periodic_capability::has_exact_periodic_nonlinear_devices(&circuit);
        if has_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes)?;
        }
        let branch_names = solver.try_periodic_mna_branch_names().map_err(|error| {
            SimulationError::Circuit(format!(
                "PAC branch-result metadata construction failed: {error}"
            ))
        })?;

        if let Some(PacOperatingPoint::HarmonicBalance(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, &hb_config)?;
        }
        if let Some(PacOperatingPoint::Shooting(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, point.config())?;
        }

        let solve_operating_point = operating_point.is_none();
        let mut state = if let Some(operating_point) = operating_point {
            match operating_point {
                PacOperatingPoint::Shooting(point) => self.hb_state_from_pss_operating_point(
                    point,
                    &hb_config,
                    &node_names,
                    &branch_names,
                    abort,
                )?,
                PacOperatingPoint::HarmonicBalance(point) => {
                    point.to_solver_state(&node_names, &branch_names)?
                }
            }
        } else {
            HbSolverState::new(num_nodes, op_harmonics)
        };
        let branch_count = branch_names.len();
        state
            .try_prepare_mna_branches(branch_count, hb_config.num_harmonics)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "PAC operating-point MNA state construction failed: {error}"
                ))
            })?;
        if solve_operating_point {
            if has_nonlinear {
                solver
                    .solve_newton_with_abort(&mut state, abort)
                    .map_err(|e| match e {
                        crate::analysis::HbError::Aborted => SimulationError::Aborted,
                        _ => SimulationError::Circuit(format!(
                            "PAC operating-point solve failed: {e}"
                        )),
                    })?;
            } else {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                solver.solve_linear(&mut state).map_err(|e| {
                    SimulationError::Circuit(format!("PAC operating-point solve failed: {e}"))
                })?;
            }
        }
        if num_nodes.checked_add(branch_count) != Some(periodic_unknowns) {
            return Err(SimulationError::Circuit(format!(
                "PAC periodic solver exposes {num_nodes} nodes and {branch_count} branches, but resource qualification used {periodic_unknowns} MNA unknowns"
            )));
        }

        Ok(PreparedPeriodicAc {
            circuit,
            solver,
            state,
            node_names,
            branch_names,
            rf_ports,
            lifted_unknowns,
        })
    }
}
