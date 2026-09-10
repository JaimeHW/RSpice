//! Periodic AC (PAC) analysis: small-signal conversion analysis around a
//! harmonic-balance operating point.
//!
//! The large-signal periodic solution comes from the HB solver; the
//! small-signal stimulus is applied at `f = offset + m*f0` and the linearized
//! time-varying network maps it into responses at every sideband
//! `offset + k*f0`. See `harmonic_balance::solver::periodic_ac` for the
//! conversion-matrix formulation.

use super::periodic_ac::{PacOperatingPoint, PeriodicAcOutput, PreparedPeriodicAc};
use super::*;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::HbError as AnalysisHbError;
use crate::analysis::harmonic_balance::{PeriodicAcExcitation, PeriodicSidebandWindow};
use crate::analysis::pac::{PacConfig, PacResult};

/// PAC analysis result with convergence info
#[derive(Debug)]
pub struct PacAnalysisResult {
    /// The PAC solution: per-node sideband spectra and the conversion matrix
    /// for the configured output node.
    pub result: PacResult,
    /// Large-signal fundamental frequency (Hz)
    pub fundamental_freq: Value,
    /// Whether the operating-point solve converged
    pub converged: bool,
}

pub(super) struct PacInputPort {
    pub(super) node_injections: Vec<(usize, Complex64)>,
    pub(super) voltage_source_index: Option<usize>,
}

impl Engine {
    /// Run Periodic AC analysis.
    ///
    /// Solves the large-signal periodic operating point with harmonic
    /// balance, samples the periodically time-varying small-signal
    /// linearization, and solves the sideband-coupled system at every sweep
    /// offset. The per-node spectra answer "what does the circuit do to the
    /// input source's signal"; when an output node is configured, the full
    /// conversion matrix (every input sideband to every output sideband) is
    /// filled for that node.
    pub fn run_pac(
        &self,
        netlist: &Netlist,
        config: PacConfig,
    ) -> Result<PacAnalysisResult, SimulationError> {
        self.run_pac_with_abort(netlist, config, &NoAbort)
    }

    /// Run periodic AC with cooperative cancellation between operating-point
    /// iterations and frequency/sideband solves.
    pub fn run_pac_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pac_impl(netlist, config, None, abort)
    }

    /// Run periodic AC from an exact previously converged shooting-PSS
    /// operating point. The large-signal state is projected into the PAC
    /// spectral basis; it is never re-solved.
    pub fn run_pac_from_pss_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        operating_point: &super::super::PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pac_impl(
            netlist,
            config,
            Some(PacOperatingPoint::Shooting(operating_point)),
            abort,
        )
    }

    /// Run periodic AC from an exact previously converged harmonic-balance
    /// operating point. The large-signal state is consumed directly and is
    /// never re-solved.
    pub fn run_pac_from_hb_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        operating_point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pac_impl(
            netlist,
            config,
            Some(PacOperatingPoint::HarmonicBalance(operating_point)),
            abort,
        )
    }

    /// Run one authored `.PXF` card against a retained shooting-`.PSS`
    /// carrier.
    ///
    /// The card describes a periodic AC solve and one path through its
    /// conversion matrix; this runs that solve and reads that path. The
    /// element is taken straight from
    /// [`crate::analysis::pac::ConversionMatrix::get_transfer`],
    /// which is the only code that knows how a sideband pair maps onto the
    /// matrix's storage — reassembling the matrix into a dense cube first, as
    /// a caller once did, both costs `O(F*S^2)` and re-states that mapping in
    /// a second place where it can drift.
    pub fn run_pxf_card_from_pss_with_abort(
        &self,
        netlist: &Netlist,
        card: &crate::netlist::PxfCard,
        operating_point: &super::super::PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<crate::analysis::pxf::PxfResult, SimulationError> {
        let pac = self.run_pac_from_pss_with_abort(
            netlist,
            PacConfig::from(card),
            operating_point,
            abort,
        )?;
        pxf_transfer_from_pac(card, &pac, abort)
    }

    /// Run one authored `.PXF` card against a retained harmonic-balance
    /// carrier.
    ///
    /// `.PAC` and `.PNOISE` both accept either periodic carrier, and the
    /// conversion matrix a harmonic-balance operating point produces is the
    /// same object with the same axes, so `.PXF FROM=HB` reads out of it the
    /// same way.
    pub fn run_pxf_card_from_hb_with_abort(
        &self,
        netlist: &Netlist,
        card: &crate::netlist::PxfCard,
        operating_point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<crate::analysis::pxf::PxfResult, SimulationError> {
        let pac = self.run_pac_from_hb_with_abort(
            netlist,
            PacConfig::from(card),
            operating_point,
            abort,
        )?;
        pxf_transfer_from_pac(card, &pac, abort)
    }

    fn run_pac_impl(
        &self,
        netlist: &Netlist,
        mut config: PacConfig,
        operating_point: Option<PacOperatingPoint<'_>>,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        let input_name = config
            .input_source
            .clone()
            .ok_or_else(|| SimulationError::Circuit("PAC requires an input source".to_string()))?;

        let PreparedPeriodicAc {
            circuit,
            mut solver,
            state,
            node_names,
            branch_names,
            lifted_unknowns,
            ..
        } = self.prepare_periodic_ac(
            netlist,
            &mut config,
            operating_point,
            PeriodicAcOutput::NodeSpectra,
            abort,
        )?;
        let num_nodes = node_names.len();
        let branch_count = branch_names.len();
        let sideband_count = config.num_sidebands();

        // Resolve the named source to an exact unit small-signal excitation.
        let input_port = Self::pac_input_port(&circuit, &input_name, num_nodes)?;
        let branch_voltage = input_port
            .voltage_source_index
            .map(|source_index| {
                solver
                    .periodic_voltage_source_branch(source_index)
                    .map(|branch| vec![(branch, Complex64::new(1.0, 0.0))])
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "PAC input voltage source '{input_name}' has no periodic MNA branch"
                        ))
                    })
            })
            .transpose()?
            .unwrap_or_default();

        let mut sweep_config = config.clone();
        sweep_config.fundamental_freq = config.fundamental_freq;
        let frequencies = sweep_config
            .try_frequency_points_with_abort(abort)
            .map_err(|error| match error {
                crate::analysis::FrequencyGridError::Aborted => SimulationError::Aborted,
                _ => SimulationError::Circuit(format!("Invalid PAC frequency sweep: {error}")),
            })?;
        if frequencies.is_empty() {
            return Err(SimulationError::Circuit(
                "PAC frequency sweep produced no points".to_string(),
            ));
        }

        let output_idx = config
            .output_node
            .as_deref()
            .map(|name| {
                if netlist.ground_policy().is_ground(name) {
                    return Err(SimulationError::Circuit(
                        "PAC output node cannot be ground".to_owned(),
                    ));
                }
                node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "PAC output node '{name}' not found in circuit nodes"
                        ))
                    })
            })
            .transpose()?;
        let output_ref_idx = config
            .output_ref
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .map(|name| {
                if netlist.ground_policy().is_ground(name) {
                    return Ok(None);
                }
                node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .map(Some)
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "PAC output reference node '{name}' not found in circuit nodes"
                        ))
                    })
            })
            .transpose()?
            .flatten();
        if output_idx.is_some() && output_idx == output_ref_idx {
            return Err(SimulationError::Circuit(
                "PAC output node and reference node must be different".to_owned(),
            ));
        }

        let mut result = if output_idx.is_some() {
            PacResult::new(
                config.fundamental_freq,
                frequencies,
                config.sideband_min,
                config.sideband_max,
                node_names.clone(),
                branch_names,
            )
        } else {
            PacResult::new_without_conversion_matrix(
                config.fundamental_freq,
                frequencies,
                config.sideband_min,
                config.sideband_max,
                node_names.clone(),
                branch_names,
            )
        }
        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        result.set_input_source(&input_name);
        if let Some(ref out) = config.output_node {
            result.set_output_node(out);
        }
        // The solve below drives a unit excitation so the conversion matrix
        // stays a transfer function. The authored drive amplitude and the
        // sideband-zero selection travel on the result for whoever publishes
        // it; see `PacResult::pac_magnitude` and `PacResult::include_dc`.
        result.pac_magnitude = config.pac_magnitude;
        result.include_dc = config.include_dc;

        // Excitation columns: the input source's own frequency (m = 0)
        // always; every input sideband when a conversion matrix is wanted.
        let excitation_count = if output_idx.is_some() {
            sideband_count
        } else {
            1
        };
        let mut excitation_sidebands = Vec::new();
        excitation_sidebands
            .try_reserve_exact(excitation_count)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "PAC excitation-sideband allocation failed: {error}"
                ))
            })?;
        if output_idx.is_some() {
            excitation_sidebands.extend(config.sideband_min..=config.sideband_max);
        } else {
            excitation_sidebands.push(0);
        }

        for freq_idx in 0..result.frequencies.len() {
            let offset = result.frequencies[freq_idx];
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut excitations = Vec::new();
            let mut branch_excitations: Vec<&[(usize, Complex64)]> = Vec::new();
            excitations
                .try_reserve_exact(excitation_sidebands.len())
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "PAC excitation-column allocation failed: {error}"
                    ))
                })?;
            branch_excitations
                .try_reserve_exact(excitation_sidebands.len())
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "PAC branch-excitation-column allocation failed: {error}"
                    ))
                })?;
            for &m in &excitation_sidebands {
                let mut column_injections = Vec::new();
                column_injections
                    .try_reserve_exact(input_port.node_injections.len())
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "PAC input-port allocation failed: {error}"
                        ))
                    })?;
                column_injections.extend_from_slice(&input_port.node_injections);
                excitations.push(PeriodicAcExcitation {
                    sideband: m,
                    injections: column_injections,
                });
                branch_excitations.push(branch_voltage.as_slice());
            }

            let sideband_count = result.num_sidebands();
            solver
                .solve_periodic_ac_each_with_branch_voltages(
                    &state,
                    PeriodicSidebandWindow {
                        offset_hz: offset,
                        sideband_min: config.sideband_min,
                        sideband_max: config.sideband_max,
                    },
                    &excitations,
                    &branch_excitations,
                    |col, solution| {
                        if abort.is_aborted() {
                            return Err(AnalysisHbError::Aborted);
                        }
                        if solution.len() != lifted_unknowns {
                            return Err(AnalysisHbError::InvalidCircuit(format!(
                                "PAC solver returned {} values for a {lifted_unknowns}-value lifted MNA system",
                                solution.len()
                            )));
                        }
                        if let Some((index, value)) = solution
                            .iter()
                            .copied()
                            .enumerate()
                            .find(|(_, value)| !value.re.is_finite() || !value.im.is_finite())
                        {
                            return Err(AnalysisHbError::InvalidCircuit(format!(
                                "PAC solver returned a non-finite value at lifted MNA index {index} ({:+.6e}{:+.6e}j)",
                                value.re, value.im
                            )));
                        }
                        let m = *excitation_sidebands.get(col).ok_or_else(|| {
                            AnalysisHbError::InvalidCircuit(format!(
                                "PAC returned unexpected excitation column {col}"
                            ))
                        })?;
                        if m == 0 {
                            for k_idx in 0..sideband_count {
                                let k = i64::from(config.sideband_min)
                                    .checked_add(i64::try_from(k_idx).map_err(|_| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband offset exceeds i64".to_string(),
                                        )
                                    })?)
                                    .and_then(|value| i32::try_from(value).ok())
                                    .ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband index exceeds i32".to_string(),
                                        )
                                    })?;
                                let data = result
                                    .get_sideband_data_mut(freq_idx, k)
                                    .ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC result is missing frequency {freq_idx}, sideband {k}"
                                        ))
                                    })?;
                                if data.node_voltages.len() != num_nodes
                                    || data.branch_currents.len() != branch_count
                                {
                                    return Err(AnalysisHbError::InvalidCircuit(format!(
                                        "PAC sideband result cardinality differs at frequency {freq_idx}, sideband {k}: {} nodes/{} branches; expected {num_nodes}/{branch_count}",
                                        data.node_voltages.len(),
                                        data.branch_currents.len()
                                    )));
                                }
                                for node in 0..num_nodes {
                                    let index = node
                                        .checked_mul(sideband_count)
                                        .and_then(|row| row.checked_add(k_idx))
                                        .ok_or_else(|| {
                                            AnalysisHbError::InvalidCircuit(
                                                "PAC node-spectrum index overflows usize"
                                                    .to_string(),
                                            )
                                        })?;
                                    let value = *solution.get(index).ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC solution is missing node {node}, sideband {k}"
                                        ))
                                    })?;
                                    data.set_voltage(node, value).map_err(|error| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC node-spectrum publication failed: {error}"
                                        ))
                                    })?;
                                }
                                for branch in 0..branch_count {
                                    let row = num_nodes.checked_add(branch).ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC branch-row index overflows usize".to_string(),
                                        )
                                    })?;
                                    let index = row
                                        .checked_mul(sideband_count)
                                        .and_then(|row| row.checked_add(k_idx))
                                        .ok_or_else(|| {
                                            AnalysisHbError::InvalidCircuit(
                                                "PAC branch-spectrum index overflows usize"
                                                    .to_string(),
                                            )
                                        })?;
                                    let value = *solution.get(index).ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC solution is missing branch {branch}, sideband {k}"
                                        ))
                                    })?;
                                    data.set_current(branch, value).map_err(|error| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC branch-spectrum publication failed: {error}"
                                        ))
                                    })?;
                                }
                            }
                        }

                        if let Some(out) = output_idx {
                            for k_idx in 0..sideband_count {
                                let k = i64::from(config.sideband_min)
                                    .checked_add(i64::try_from(k_idx).map_err(|_| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband offset exceeds i64".to_string(),
                                        )
                                    })?)
                                    .and_then(|value| i32::try_from(value).ok())
                                    .ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband index exceeds i32".to_string(),
                                        )
                                    })?;
                                let output = solution[out * sideband_count + k_idx];
                                let output_voltage = if let Some(reference) = output_ref_idx {
                                    output - solution[reference * sideband_count + k_idx]
                                } else {
                                    output
                                };
                                if !output_voltage.re.is_finite()
                                    || !output_voltage.im.is_finite()
                                {
                                    return Err(AnalysisHbError::InvalidCircuit(format!(
                                        "PAC differential output is non-representable at offset {offset:.6e} Hz, input sideband {m}, output sideband {k}"
                                    )));
                                }
                                result
                                    .conversion_matrix
                                    .set(freq_idx, k, m, output_voltage)
                                    .map_err(|error| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC conversion publication failed: {error}"
                                        ))
                                    })?;
                            }
                        }
                        if abort.is_aborted() {
                            return Err(AnalysisHbError::Aborted);
                        }
                        Ok(())
                    },
                )
                .map_err(|error| match error {
                    AnalysisHbError::Aborted => SimulationError::Aborted,
                    error => SimulationError::Circuit(format!(
                        "PAC solve failed at offset {offset:.6e} Hz: {error}"
                    )),
                })?;

            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
        }

        result.set_convergence_info(state.iteration.max(1), state.residual_norm);

        Ok(PacAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            converged: state.converged,
        })
    }

    /// Map the named small-signal source to an exact MNA excitation.
    pub(in crate::engine::hb) fn pac_input_port(
        circuit: &CircuitData,
        input_name: &str,
        num_nodes: usize,
    ) -> Result<PacInputPort, SimulationError> {
        let trimmed = input_name.trim();

        let validate_terminals = |kind: &str, np: usize, nn: usize| {
            if np > num_nodes || nn > num_nodes {
                return Err(SimulationError::Circuit(format!(
                    "PAC input {kind} source '{trimmed}' references node pair ({np}, {nn}), outside the circuit's 0..={num_nodes} node range"
                )));
            }
            if np == nn {
                return Err(SimulationError::Circuit(format!(
                    "PAC input {kind} source '{trimmed}' has identical terminals and no effective port"
                )));
            }
            Ok(())
        };

        if let Some(idx) = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(trimmed))
        {
            let np = circuit.voltage_sources.node_pos[idx];
            let nn = circuit.voltage_sources.node_neg[idx];
            validate_terminals("voltage", np, nn)?;
            return Ok(PacInputPort {
                node_injections: Vec::new(),
                voltage_source_index: Some(idx),
            });
        }

        if let Some(idx) = circuit
            .current_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(trimmed))
        {
            let np = circuit.current_sources.node_pos[idx];
            let nn = circuit.current_sources.node_neg[idx];
            validate_terminals("current", np, nn)?;
            let mut injections = Vec::new();
            if np > 0 {
                injections.push((np - 1, Complex64::new(-1.0, 0.0)));
            }
            if nn > 0 {
                injections.push((nn - 1, Complex64::new(1.0, 0.0)));
            }
            return Ok(PacInputPort {
                node_injections: injections,
                voltage_source_index: None,
            });
        }

        Err(SimulationError::Circuit(format!(
            "PAC input source '{trimmed}' not found among independent sources"
        )))
    }
}

/// Read the one transfer an authored `.PXF` card names out of the periodic AC
/// solve it described.
///
/// The card's sweep is an *offset* grid, so `TransferPoint::freq_in` is the
/// offset the deck authored and `freq_out` is the absolute frequency the
/// converted response appears at, `output_sideband * f0 + offset`, which is
/// how `PacSidebandDescriptor::absolute_frequencies` already states an output
/// frequency for the same solve.
fn pxf_transfer_from_pac(
    card: &crate::netlist::PxfCard,
    pac: &PacAnalysisResult,
    abort: &dyn AbortSignal,
) -> Result<crate::analysis::pxf::PxfResult, SimulationError> {
    use crate::analysis::pxf::{PxfResult, TransferPoint};

    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let fundamental = pac.fundamental_freq;
    if !fundamental.is_finite() || fundamental <= 0.0 {
        return Err(SimulationError::Circuit(
            "PXF requires a positive fundamental frequency from its periodic carrier".to_string(),
        ));
    }
    // `get_transfer` is also the range check: a sideband the solve did not
    // span has no element, and it says so by coordinate. Asking the same
    // question twice is how the two answers start to differ.
    let transfers = pac
        .result
        .conversion_matrix
        .get_transfer(card.input_sideband, card.output_sideband)
        .map_err(|error| {
            SimulationError::Circuit(format!(
                "PXF transfer from sideband {} to sideband {} is unavailable: {error}",
                card.input_sideband, card.output_sideband
            ))
        })?;
    if transfers.is_empty() {
        return Err(SimulationError::Circuit(
            "PXF conversion matrix has no frequency points".to_string(),
        ));
    }

    let mut result = PxfResult::new(fundamental, card.input_sideband, card.output_sideband);
    result
        .points
        .try_reserve_exact(transfers.len())
        .map_err(|error| {
            SimulationError::Circuit(format!("PXF transfer-point allocation failed: {error}"))
        })?;
    let mut previous_offset: Option<Value> = None;
    for (index, transfer) in transfers.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let offset = transfer.frequency_offset;
        // A conversion element is indexed by its offset, so a grid that is not
        // finite, positive and strictly increasing would make the curve
        // metrics below — peak, bandwidth, unity-gain crossing, group delay —
        // read a shape the sweep does not have.
        if !offset.is_finite()
            || offset <= 0.0
            || previous_offset.is_some_and(|last| offset <= last)
        {
            return Err(SimulationError::Circuit(format!(
                "PXF input-frequency grid is not strictly increasing and positive at point {}",
                index + 1
            )));
        }
        previous_offset = Some(offset);
        if !transfer.transfer.re.is_finite() || !transfer.transfer.im.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "PXF transfer is not finite at point {} ({:+.6e}{:+.6e}j)",
                index + 1,
                transfer.transfer.re,
                transfer.transfer.im
            )));
        }
        let freq_out = transfer.output_frequency(fundamental).map_err(|error| {
            SimulationError::Circuit(format!(
                "PXF output frequency is unavailable at point {}: {error}",
                index + 1
            ))
        })?;
        result.points.push(TransferPoint {
            freq_in: offset,
            freq_out,
            transfer: transfer.transfer,
            sideband_in: card.input_sideband,
            sideband_out: card.output_sideband,
        });
    }

    result.compute_metrics();
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    Ok(result)
}
