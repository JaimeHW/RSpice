//! Typed analysis results convert to and from validated transferable buffers.
use super::*;

impl WorkerSimulationResultTransport {
    pub(in crate::simulation::runner::worker_contract) fn from_result(
        result: WorkerSimulationResult,
        buffers: &mut Vec<Vec<f64>>,
    ) -> Result<Self, String> {
        Ok(match result {
            WorkerSimulationResult::DcOp {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                node_voltages,
                branch_currents,
                device_report,
            } => Self::DcOp {
                configuration: WorkerOpConfigTransport::from_config(configuration, buffers),
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution_digest: crate::simulation::execution::f64_sequence_digest(
                    "rspice.worker-dc-op-mna/v1",
                    &mna_solution,
                ),
                mna_solution: WorkerF64Series::from_vec(mna_solution, buffers),
                node_voltages,
                branch_currents,
                device_report,
            },
            WorkerSimulationResult::DcSweep {
                evidence,
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => Self::DcSweep {
                evidence: evidence
                    .map(|evidence| WorkerDcSweepEvidence::from_evidence(evidence, buffers)),
                sweep_var,
                sweep_values: WorkerF64Series::from_vec(sweep_values, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
            },
            WorkerSimulationResult::Transient {
                time,
                waveforms,
                measurements,
                convergence,
                events,
                spectra,
            } => Self::Transient {
                time: WorkerF64Series::from_vec(time, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
                convergence: convergence.as_ref().map(|quality| {
                    crate::simulation::results::ConvergenceTransport::from_evidence(
                        quality,
                        |values| WorkerF64Series::from_vec(values.into_owned(), buffers),
                    )
                }),
                events,
                spectra: WorkerRecordedFftSpectrumTransport::from_spectra(spectra, buffers),
            },
            WorkerSimulationResult::Fft {
                spectrum,
                convergence,
            } => Self::Fft {
                spectrum: WorkerRecordedFftSpectrumTransport::from_spectra(vec![spectrum], buffers)
                    .pop()
                    .expect("one spectrum in, one spectrum out"),
                convergence: convergence.as_ref().map(|quality| {
                    crate::simulation::results::ConvergenceTransport::from_evidence(
                        quality,
                        |values| WorkerF64Series::from_vec(values.into_owned(), buffers),
                    )
                }),
            },
            WorkerSimulationResult::Pss {
                measurements,
                operating_point,
            } => Self::Pss {
                measurements,
                operating_point: WorkerPssOperatingPointTransport::from_operating_point(
                    operating_point,
                    buffers,
                ),
            },
            WorkerSimulationResult::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => Self::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes: WorkerPstbModesTransport::from_modes(modes, buffers),
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices: WorkerF64Series::from_vec(mode_indices, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
            },
            WorkerSimulationResult::Qpnoise {
                frequencies,
                waveforms,
                response,
            } => Self::Qpnoise {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                response: WorkerQpnoiseResultTransport::from_response(response, buffers)?,
            },
            WorkerSimulationResult::Qpxf {
                frequencies,
                waveforms,
                response,
            } => Self::Qpxf {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                response: WorkerQpxfResultTransport::from_response(response, buffers),
            },
            WorkerSimulationResult::Qpac {
                frequencies,
                waveforms,
                response,
            } => Self::Qpac {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                response: WorkerQpacResultTransport::from_response(response, buffers),
            },
            WorkerSimulationResult::Qpss {
                frequencies,
                tuples,
                waveforms,
                operating_point,
            } => Self::Qpss {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                tuples,
                waveforms: transport_waveforms(waveforms, buffers),
                operating_point: WorkerQpssOperatingPointTransport::from_operating_point(
                    operating_point,
                    buffers,
                ),
            },
            WorkerSimulationResult::Hb {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => Self::Hb {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
                operating_point: WorkerHbOperatingPointTransport::from_operating_point(
                    operating_point,
                    buffers,
                ),
            },
            WorkerSimulationResult::Ac {
                convergence,
                frequencies,
                waveforms,
                measurements,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            } => Self::Ac {
                convergence: convergence.as_ref().map(|quality| {
                    crate::simulation::results::ConvergenceTransport::from_evidence(
                        quality,
                        |values| WorkerF64Series::from_vec(values.into_owned(), buffers),
                    )
                }),
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                measurements,
                noise_reference_temperature_kelvin,
                reference_impedances_ohm: reference_impedances_ohm
                    .map(|values| WorkerF64Series::from_vec(values, buffers)),
            },
            WorkerSimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => Self::Noise {
                frequencies: WorkerF64Series::from_vec(frequencies, buffers),
                output_noise: WorkerF64Series::from_vec(output_noise, buffers),
                input_noise: input_noise.map(|values| WorkerF64Series::from_vec(values, buffers)),
                contributors: contributors
                    .into_iter()
                    .map(|(name, values)| (name, WorkerF64Series::from_vec(values, buffers)))
                    .collect(),
                summary,
                measurements,
            },
            WorkerSimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => Self::Parametric {
                target,
                sweep_values: WorkerF64Series::from_vec(sweep_values, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                num_failures,
                member_measurements,
            },
            WorkerSimulationResult::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
                member_measurements,
            } => Self::Corner {
                x_values: WorkerF64Series::from_vec(x_values, buffers),
                x_label,
                x_unit,
                temperatures_c: WorkerF64Series::from_vec(temperatures_c, buffers),
                corner_labels,
                waveforms: transport_waveforms(waveforms, buffers),
                num_failures,
                member_measurements,
            },
            WorkerSimulationResult::ReliabilityMission {
                years,
                waveforms,
                response,
            } => Self::ReliabilityMission {
                years: WorkerF64Series::from_vec(years, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                response: WorkerReliabilityMissionTransport::from_response(response, buffers)?,
            },
            WorkerSimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => Self::Reliability {
                years: WorkerF64Series::from_vec(years, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                device_results,
            },
            WorkerSimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                best_objectives,
                converged,
            } => Self::Optimization {
                iterations: WorkerF64Series::from_vec(iterations, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                best_cost,
                best_variables,
                best_objectives,
                converged,
            },
            WorkerSimulationResult::Soa {
                convergence,
                time,
                waveforms,
                violations,
                evaluations,
            } => Self::Soa {
                convergence: convergence.as_ref().map(|quality| {
                    crate::simulation::results::ConvergenceTransport::from_evidence(
                        quality,
                        |values| WorkerF64Series::from_vec(values.into_owned(), buffers),
                    )
                }),
                time: WorkerF64Series::from_vec(time, buffers),
                waveforms: transport_waveforms(waveforms, buffers),
                violations,
                evaluations,
            },
            other => Self::Inline(other),
        })
    }

    pub(in crate::simulation::runner::worker_contract) fn into_result(
        self,
        buffers: &[Vec<f64>],
    ) -> Result<WorkerSimulationResult, String> {
        match self {
            Self::Inline(result) => {
                if matches!(
                    result,
                    WorkerSimulationResult::ReliabilityMission { .. }
                        | WorkerSimulationResult::Pstb { .. }
                        | WorkerSimulationResult::Qpnoise { .. }
                        | WorkerSimulationResult::Transient { .. }
                        | WorkerSimulationResult::Ac { .. }
                        | WorkerSimulationResult::Soa { .. }
                        | WorkerSimulationResult::Fft { .. }
                ) {
                    return Err(
                        "Waveform and convergence results must use the dedicated transfer-buffer transport"
                            .to_owned(),
                    );
                }
                Ok(result)
            }
            Self::DcOp {
                configuration,
                validated_startup_directives,
                mna_node_names,
                mna_branch_names,
                mna_solution,
                mna_solution_digest,
                node_voltages,
                branch_currents,
                device_report,
            } => {
                let configuration = configuration.into_config(buffers)?;
                let mna_solution = mna_solution.into_vec(buffers)?;
                let actual_digest = crate::simulation::execution::f64_sequence_digest(
                    "rspice.worker-dc-op-mna/v1",
                    &mna_solution,
                );
                if actual_digest != mna_solution_digest {
                    return Err(format!(
                        "worker DC operating-point MNA payload digest is {actual_digest}, expected {mna_solution_digest}"
                    ));
                }
                validate_worker_dc_op_state(
                    &configuration,
                    &mna_node_names,
                    &mna_branch_names,
                    &mna_solution,
                )?;
                Ok(WorkerSimulationResult::DcOp {
                    configuration,
                    validated_startup_directives,
                    mna_node_names,
                    mna_branch_names,
                    mna_solution,
                    node_voltages,
                    branch_currents,
                    device_report,
                })
            }
            Self::DcSweep {
                evidence,
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
            } => {
                dc_sweep::validate_dc_transport_size(&sweep_values, &waveforms, evidence.as_ref())?;
                Ok(WorkerSimulationResult::DcSweep {
                    evidence: evidence
                        .map(|evidence| evidence.into_evidence(buffers))
                        .transpose()?,
                    sweep_var,
                    sweep_values: sweep_values.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                    measurements,
                })
            }
            Self::Transient {
                time,
                waveforms,
                measurements,
                convergence,
                events,
                spectra,
            } => Ok(WorkerSimulationResult::Transient {
                time: time.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
                convergence: convergence
                    .map(|quality| {
                        quality.into_evidence(WorkerF64Series::len, |series| {
                            series.into_convergence_values(buffers)
                        })
                    })
                    .transpose()?,
                events,
                spectra: WorkerRecordedFftSpectrumTransport::into_spectra(spectra, buffers)?,
            }),
            Self::Fft {
                spectrum,
                convergence,
            } => Ok(WorkerSimulationResult::Fft {
                spectrum: WorkerRecordedFftSpectrumTransport::into_spectra(
                    vec![spectrum],
                    buffers,
                )?
                .pop()
                .expect("one spectrum in, one spectrum out"),
                convergence: convergence
                    .map(|quality| {
                        quality.into_evidence(WorkerF64Series::len, |series| {
                            series.into_convergence_values(buffers)
                        })
                    })
                    .transpose()?,
            }),
            Self::Pss {
                measurements,
                operating_point,
            } => Ok(WorkerSimulationResult::Pss {
                measurements,
                operating_point: operating_point.into_operating_point(buffers)?,
            }),
            Self::Pstb {
                period,
                fundamental_frequency,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                modes,
                floquet_evidence,
                orbit_kind,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                max_multiplier_magnitude,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                waveforms,
            } => {
                let result = WorkerSimulationResult::Pstb {
                    period,
                    fundamental_frequency,
                    stability_threshold,
                    probe_instance,
                    detect_subharmonics,
                    modes: modes.into_modes(buffers)?,
                    floquet_evidence,
                    orbit_kind,
                    trivial_multiplier_index,
                    stability_verdict,
                    stability_classification,
                    min_stability_margin_db,
                    max_multiplier_magnitude,
                    num_unstable,
                    subharmonics,
                    converged,
                    iterations,
                    mode_indices: mode_indices.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                };
                validate_worker_pstb_result(&result)?;
                Ok(result)
            }
            Self::Qpnoise {
                frequencies,
                waveforms,
                response,
            } => {
                // Bound and validate the complete result before display copies.
                let response = response.into_response(buffers)?;
                Ok(WorkerSimulationResult::Qpnoise {
                    frequencies: frequencies.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                    response,
                })
            }
            Self::Qpxf {
                frequencies,
                waveforms,
                response,
            } => {
                // Bound and validate the complete result before display copies.
                let response = response.into_response(buffers)?;
                Ok(WorkerSimulationResult::Qpxf {
                    frequencies: frequencies.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                    response,
                })
            }
            Self::Qpac {
                frequencies,
                waveforms,
                response,
            } => {
                // Bound and validate the complete result before display copies.
                let response = response.into_response(buffers)?;
                Ok(WorkerSimulationResult::Qpac {
                    frequencies: frequencies.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                    response,
                })
            }
            Self::Qpss {
                frequencies,
                tuples,
                waveforms,
                operating_point,
            } => Ok(WorkerSimulationResult::Qpss {
                frequencies: frequencies.into_vec(buffers)?,
                tuples,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                operating_point: operating_point.into_operating_point(buffers)?,
            }),
            Self::Hb {
                frequencies,
                waveforms,
                measurements,
                operating_point,
            } => Ok(WorkerSimulationResult::Hb {
                frequencies: frequencies.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
                operating_point: operating_point.into_operating_point(buffers)?,
            }),
            Self::Ac {
                convergence,
                frequencies,
                waveforms,
                measurements,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
            } => Ok(WorkerSimulationResult::Ac {
                convergence: convergence
                    .map(|quality| {
                        quality.into_evidence(WorkerF64Series::len, |series| {
                            series.into_convergence_values(buffers)
                        })
                    })
                    .transpose()?,
                frequencies: frequencies.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                measurements,
                noise_reference_temperature_kelvin,
                reference_impedances_ohm: reference_impedances_ohm
                    .map(|values| values.into_vec(buffers))
                    .transpose()?,
            }),
            Self::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
                measurements,
            } => Ok(WorkerSimulationResult::Noise {
                frequencies: frequencies.into_vec(buffers)?,
                output_noise: output_noise.into_vec(buffers)?,
                input_noise: input_noise
                    .map(|values| values.into_vec(buffers))
                    .transpose()?,
                contributors: contributors
                    .into_iter()
                    .map(|(name, values)| values.into_vec(buffers).map(|values| (name, values)))
                    .collect::<Result<_, _>>()?,
                summary,
                measurements,
            }),
            Self::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
                member_measurements,
            } => Ok(WorkerSimulationResult::Parametric {
                target,
                sweep_values: sweep_values.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                num_failures,
                member_measurements,
            }),
            Self::Corner {
                x_values,
                x_label,
                x_unit,
                temperatures_c,
                corner_labels,
                waveforms,
                num_failures,
                member_measurements,
            } => Ok(WorkerSimulationResult::Corner {
                x_values: x_values.into_vec(buffers)?,
                x_label,
                x_unit,
                temperatures_c: temperatures_c.into_vec(buffers)?,
                corner_labels,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                num_failures,
                member_measurements,
            }),
            Self::ReliabilityMission {
                years,
                waveforms,
                response,
            } => {
                let result = WorkerSimulationResult::ReliabilityMission {
                    years: years.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                    response: response.into_response(buffers)?,
                };
                validate_worker_reliability_result(&result)?;
                Ok(result)
            }
            Self::Reliability {
                years,
                waveforms,
                device_results,
            } => Ok(WorkerSimulationResult::Reliability {
                years: years.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                device_results,
            }),
            Self::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                best_objectives,
                converged,
            } => {
                crate::simulation::optimizer::validate_optimization_objectives(
                    &best_objectives,
                    best_cost,
                )?;
                Ok(WorkerSimulationResult::Optimization {
                    iterations: iterations.into_vec(buffers)?,
                    waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                    best_cost,
                    best_variables,
                    best_objectives,
                    converged,
                })
            }
            Self::Soa {
                convergence,
                time,
                waveforms,
                violations,
                evaluations,
            } => Ok(WorkerSimulationResult::Soa {
                convergence: convergence
                    .map(|quality| {
                        quality.into_evidence(WorkerF64Series::len, |series| {
                            series.into_convergence_values(buffers)
                        })
                    })
                    .transpose()?,
                time: time.into_vec(buffers)?,
                waveforms: worker_waveforms_from_transport(waveforms, buffers)?,
                violations,
                evaluations,
            }),
        }
    }
}
