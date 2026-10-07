//! Dataset manifest facts projected from retained run authority and exact data inventory.
use crate::{
    analysis_payload::AnalysisResultPayload,
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    family_metadata::AnalysisResultFamilyMetadata,
    provenance::AnalysisResultSourceDomain,
    result_digest::ResultDigestEncoding,
    run::{SimulationRun, SimulationRunLifecycle},
    waveform::RetainedWaveform,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestRow {
    pub analysis: String,
    pub expansion: String,
    pub tasks: String,
    pub domain_axis: String,
    pub stored_values: String,
    pub precision: String,
    pub eligibility: String,
    pub task_identity: Option<String>,
    pub config_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestAuthority {
    pub source_domain: String,
    pub simulation_plan_id: Option<String>,
    pub project_revision: String,
    pub prepared_snapshot_digest: String,
    pub source_content_digest: String,
    pub source_check: String,
    pub source_check_digest: String,
    pub model_sources: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestViewModel {
    pub dataset_id: String,
    pub dataset_digest: String,
    pub run_id: String,
    pub run_sequence: String,
    pub run_label: String,
    pub lifecycle: String,
    pub execution_target: String,
    pub elapsed_time: String,
    pub inventory_title: String,
    pub inventory_status: String,
    pub integrity: String,
    pub qualification: String,
    pub task_count: usize,
    pub retained_result_count: usize,
    pub rows: Vec<ManifestRow>,
    pub authority: Option<ManifestAuthority>,
}

impl ManifestViewModel {
    #[must_use]
    /// Describe retained authority; the caller classifies its presentation-only live prefixes.
    pub fn from_run<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
        run: &SimulationRun<A>,
        is_live_partial: impl Fn(&A) -> bool,
    ) -> Self {
        let provenance_validation = run.validate_provenance();
        let provenance_is_valid = provenance_validation.is_ok();
        let prepared = run.prepared_receipt();
        let rows = match prepared {
            Some(receipt) if provenance_validation.is_ok() => receipt
                .tasks()
                .iter()
                .enumerate()
                .map(|(index, task)| {
                    let result = run.analyses.get(index);
                    ManifestRow {
                        analysis: task.result_analysis_type().display_name().to_owned(),
                        expansion: result.map_or_else(
                            || "not retained".to_owned(),
                            |analysis| expansion_label(analysis.as_ref()),
                        ),
                        tasks: "1".to_owned(),
                        domain_axis: domain_meta(task.result_analysis_type()).axis.to_owned(),
                        stored_values: result.map_or_else(
                            || "not retained".to_owned(),
                            |analysis| stored_values_label(analysis.as_ref()),
                        ),
                        precision: result.map_or_else(
                            || {
                                domain_meta(task.result_analysis_type())
                                    .precision
                                    .to_owned()
                            },
                            |analysis| precision_label(analysis.as_ref()),
                        ),
                        eligibility: result.map_or_else(
                            || format!("{} · non-sign-off", missing_result_status(run.lifecycle)),
                            |analysis| {
                                if is_live_partial(analysis) {
                                    "running · accepted samples · non-sign-off".to_owned()
                                } else if !analysis.as_ref().success {
                                    "failed · non-sign-off".to_owned()
                                } else if task.canonical_kind().availability().blocks_sign_off() {
                                    // The tier is a property of this task's
                                    // engine, so it is stated on this row
                                    // rather than only in the run-wide
                                    // qualification line above.
                                    "retained · preview engine · non-sign-off".to_owned()
                                } else {
                                    "retained · receipt matched · sign-off unavailable".to_owned()
                                }
                            },
                        ),
                        task_identity: Some(task.instance_id().to_string()),
                        config_digest: Some(task.config_digest().to_string()),
                    }
                })
                .collect(),
            Some(receipt) => receipt
                .tasks()
                .iter()
                .map(|task| ManifestRow {
                    analysis: task.result_analysis_type().display_name().to_owned(),
                    expansion: "association withheld".to_owned(),
                    tasks: "1".to_owned(),
                    domain_axis: domain_meta(task.result_analysis_type()).axis.to_owned(),
                    stored_values: "integrity mismatch".to_owned(),
                    precision: domain_meta(task.result_analysis_type())
                        .precision
                        .to_owned(),
                    eligibility: "blocked by receipt mismatch · non-sign-off".to_owned(),
                    task_identity: Some(task.instance_id().to_string()),
                    config_digest: Some(task.config_digest().to_string()),
                })
                .collect(),
            None => run
                .analyses
                .iter()
                .map(|analysis| ManifestRow {
                    analysis: analysis.as_ref().kind_display_name().to_owned(),
                    expansion: expansion_label(analysis.as_ref()),
                    tasks: "1".to_owned(),
                    domain_axis: domain_meta(analysis.as_ref().analysis_type).axis.to_owned(),
                    stored_values: stored_values_label(analysis.as_ref()),
                    precision: precision_label(analysis.as_ref()),
                    eligibility: if is_live_partial(analysis) {
                        "running · accepted samples · non-sign-off".to_owned()
                    } else if analysis.as_ref().success {
                        "legacy · no prepared receipt · sign-off unavailable".to_owned()
                    } else {
                        "failed · no prepared receipt · non-sign-off".to_owned()
                    },
                    task_identity: analysis
                        .as_ref()
                        .provenance()
                        .map(|provenance| provenance.source_instance_id().to_string()),
                    config_digest: None,
                })
                .collect(),
        };

        let integrity = match (&prepared, provenance_validation) {
            (Some(_), Ok(())) => "prepared receipt valid".to_owned(),
            (Some(_), Err(error)) => format!("blocked · {error}"),
            (None, Ok(())) => "legacy provenance valid".to_owned(),
            (None, Err(_)) if run.provenance().is_none() => {
                "unsealed · no authoritative provenance".to_owned()
            }
            (None, Err(error)) => format!("blocked · {error}"),
        };
        let authority = prepared.map(|receipt| {
            let source_check = if receipt.source_check_receipt().is_schematic_drc() {
                "schematic DRC"
            } else {
                "manual source check"
            };
            ManifestAuthority {
                source_domain: source_domain_label(receipt.source_domain()).to_owned(),
                simulation_plan_id: receipt.simulation_plan_id().map(|id| id.to_string()),
                project_revision: receipt.project_revision().get().to_string(),
                prepared_snapshot_digest: receipt.prepared_snapshot_digest().to_string(),
                source_content_digest: receipt.source_content_digest().to_string(),
                source_check: source_check.to_owned(),
                source_check_digest: receipt.source_check_receipt().digest().to_string(),
                model_sources: receipt
                    .project_model_sources()
                    .iter()
                    .map(|model| {
                        (
                            format!(
                                "{} · {} · revision {}",
                                model.model_name(),
                                model.source_id(),
                                model.revision().get()
                            ),
                            model.content_digest().to_string(),
                        )
                    })
                    .collect(),
            }
        });
        let task_count = prepared.map_or(run.analyses.len(), |receipt| receipt.tasks().len());

        Self {
            dataset_id: run.dataset_id.to_string(),
            dataset_digest: run
                .dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT)
                .to_string(),
            run_id: run.run_id.to_string(),
            run_sequence: run.id.to_string(),
            run_label: run.label.clone(),
            lifecycle: lifecycle_label(run.lifecycle).to_owned(),
            execution_target: run.execution_target.map_or_else(
                || "not retained".to_owned(),
                |target| target.label().to_owned(),
            ),
            elapsed_time: format!("{:.3} s", run.elapsed_time),
            inventory_title: inventory_title(run.lifecycle).to_owned(),
            inventory_status: inventory_status(run.lifecycle).to_owned(),
            integrity,
            qualification: qualification_label(run, provenance_is_valid),
            task_count,
            retained_result_count: run.analyses.len(),
            rows,
            authority,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DomainMeta {
    axis: &'static str,
    precision: &'static str,
}

const fn domain_meta(analysis: AnalysisType) -> DomainMeta {
    use AnalysisType as A;
    match analysis {
        A::DcOp => DomainMeta {
            axis: "scalar operating point",
            precision: "f64",
        },
        A::DcSweep | A::Parametric => DomainMeta {
            axis: "swept source or parameter",
            precision: "f64",
        },
        A::Ac | A::Stb => DomainMeta {
            axis: "log frequency",
            precision: "complex128",
        },
        A::Disto => DomainMeta {
            axis: "tone product",
            precision: "complex128",
        },
        A::Transient | A::TransientNoise => DomainMeta {
            axis: "adaptive time",
            precision: "f64",
        },
        A::Noise => DomainMeta {
            axis: "log frequency",
            precision: "f64",
        },
        A::PoleZero => DomainMeta {
            axis: "complex plane",
            precision: "complex128",
        },
        A::Tf => DomainMeta {
            axis: "frequency or operating point",
            precision: "complex128",
        },
        // `.DCMATCH` sweeps nothing: it solves one operating point and
        // reports a ranked list over the design's statistical variables,
        // which is the same abscissa a sensitivity report has.
        A::Sensitivity | A::DcMismatch => DomainMeta {
            axis: "parameter vector",
            precision: "f64",
        },
        // The abscissa is the offset from the carrier, which is what core
        // calls it and what the periodic noise members below already say.
        // Naming it after the translation instead named a different number --
        // `offset + n*f0`, which the run publishes as its own curve -- and it
        // also made .PXF contradict its own Studio caption.
        A::Qpxf | A::Qpnoise => DomainMeta {
            axis: "output frequency",
            precision: "complex128",
        },
        A::Pac | A::Pxf | A::Qpac => DomainMeta {
            axis: "offset frequency",
            precision: "complex128",
        },
        A::Pstb => DomainMeta {
            axis: "Floquet mode index",
            precision: "complex128",
        },
        A::Pnoise | A::Hbnoise => DomainMeta {
            axis: "offset frequency",
            precision: "f64",
        },
        A::MonteCarlo => DomainMeta {
            axis: "sample family",
            precision: "f64",
        },
        A::Corner => DomainMeta {
            axis: "PVT family",
            precision: "f64 / complex128",
        },
        A::Optimization => DomainMeta {
            axis: "iteration / candidate",
            precision: "f64",
        },
        A::Soa => DomainMeta {
            axis: "device / rule",
            precision: "f64",
        },
        A::SParameter | A::Hbsp | A::Psp => DomainMeta {
            axis: "frequency",
            precision: "complex128",
        },
        A::Envelope => DomainMeta {
            axis: "slow time",
            precision: "complex128",
        },
        A::Fourier => DomainMeta {
            axis: "harmonic index",
            precision: "complex128",
        },
        A::HarmonicBalance => DomainMeta {
            axis: "tone family",
            precision: "complex128",
        },
        A::Qpss => DomainMeta {
            axis: "signed tone lattice / frequency",
            precision: "complex128",
        },
        A::Pss => DomainMeta {
            axis: "periodic phase",
            precision: "f64",
        },
    }
}

fn precision_label<W: AsRef<RetainedWaveform>>(analysis: &AnalysisResult<W>) -> String {
    if analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .any(|waveform| waveform.complex.is_some())
        || matches!(
            analysis.result_payload.as_ref(),
            Some(
                AnalysisResultPayload::PoleZero { .. }
                    | AnalysisResultPayload::PssFloquet { .. }
                    | AnalysisResultPayload::Pstb { .. }
            )
        )
    {
        "complex128".to_owned()
    } else {
        domain_meta(analysis.analysis_type).precision.to_owned()
    }
}

fn stored_values_label<W: AsRef<RetainedWaveform>>(analysis: &AnalysisResult<W>) -> String {
    let mut parts = Vec::new();
    if !analysis.waveforms.is_empty() {
        let samples: usize = analysis
            .waveforms
            .iter()
            .map(AsRef::as_ref)
            .map(|waveform| waveform.x.len().min(waveform.y.len()))
            .sum();
        parts.push(format!(
            "{} waveform{} / {samples} samples",
            analysis.waveforms.len(),
            plural_suffix(analysis.waveforms.len())
        ));
    }
    if let Some(op) = &analysis.dc_op {
        parts.push(format!(
            "{} nodes / {} branches / {} power values",
            op.node_voltages.len(),
            op.branch_currents.len(),
            op.power_dissipation.len()
        ));
    }
    if analysis.device_op.is_some() {
        parts.push("device OP report".to_owned());
    }
    if let Some(noise) = &analysis.noise_summary {
        parts.push(format!(
            "{} noise contributor{}",
            noise.rows.len(),
            plural_suffix(noise.rows.len())
        ));
    }
    if let Some(family) = &analysis.family_metadata {
        parts.push(family_values_label(family));
    }
    if let Some(payload) = &analysis.result_payload {
        parts.push(payload_values_label(payload));
    }
    if !analysis.measurements.is_empty() {
        parts.push(format!(
            "{} measurement{}",
            analysis.measurements.len(),
            plural_suffix(analysis.measurements.len())
        ));
    }
    if !analysis.saved_output_receipts.is_empty() {
        parts.push(format!(
            "{} saved-output receipt{}",
            analysis.saved_output_receipts.len(),
            plural_suffix(analysis.saved_output_receipts.len())
        ));
    }
    if parts.is_empty() {
        if analysis.success {
            "no retained values".to_owned()
        } else {
            "failed · no retained values".to_owned()
        }
    } else {
        parts.join(" · ")
    }
}

fn family_values_label(family: &AnalysisResultFamilyMetadata) -> String {
    match family {
        AnalysisResultFamilyMetadata::Parametric { sweep_values, .. } => {
            format!("{} sweep points", sweep_values.len())
        }
        AnalysisResultFamilyMetadata::Corner { x_values, .. } => {
            format!("{} corner points", x_values.len())
        }
        AnalysisResultFamilyMetadata::MonteCarlo {
            runs_completed,
            variables,
            ..
        } => format!("{runs_completed} samples / {} variables", variables.len()),
        AnalysisResultFamilyMetadata::Optimization { iterations, .. } => {
            format!("{} iterations", iterations.len())
        }
        AnalysisResultFamilyMetadata::Soa { time } => {
            format!("{} SOA time points", time.len())
        }
        AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity,
            carrier_frequency_hz,
        } => {
            let quantity = match output_quantity {
                crate::family_metadata::PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity => {
                    "output-noise PSD"
                }
                crate::family_metadata::PeriodicNoiseOutputQuantity::TimingNoisePowerSpectralDensity => {
                    "timing-noise PSD in s²/Hz"
                }
                crate::family_metadata::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz => {
                    "phase noise in dBc/Hz"
                }
            };
            carrier_frequency_hz.map_or_else(
                || quantity.to_owned(),
                |carrier| format!("{quantity} / {} carrier", format_frequency(carrier)),
            )
        }
        AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm,
            noise_reference_temperature_kelvin,
        } => format!(
            "{}-port S-parameter references ({}){}",
            reference_impedances_ohm.len(),
            reference_impedances_ohm
                .iter()
                .map(|value| format!("{value} ohm"))
                .collect::<Vec<_>>()
                .join(", "),
            noise_reference_temperature_kelvin.map_or_else(String::new, |temperature| {
                format!(
                    " / noise source reference {temperature} K; noise factors linear, Rn in ohms"
                )
            }),
        ),
    }
}

fn format_frequency(value: f64) -> String {
    if value >= 1.0e9 {
        format!("{:.6} GHz", value / 1.0e9)
    } else if value >= 1.0e6 {
        format!("{:.6} MHz", value / 1.0e6)
    } else if value >= 1.0e3 {
        format!("{:.6} kHz", value / 1.0e3)
    } else {
        format!("{value:.6} Hz")
    }
}

fn payload_values_label(payload: &AnalysisResultPayload) -> String {
    match payload {
        AnalysisResultPayload::Stb { response } => format!(
            "{} loop-gain samples / {} measured unity crossings; {}; circuit stability {:?}",
            response.bode_points.len(),
            response.margins.num_crossovers,
            response.margin_assessment(),
            response.stability_verdict()
        ),
        AnalysisResultPayload::Qpnoise { response } => format!(
            "{} frequencies / {} outputs / {} physical mechanisms; full noise covariance and measurement statuses",
            response.points.len(),
            response.outputs.len(),
            response.sources.len()
        ),
        AnalysisResultPayload::Qpxf { response } => format!(
            "{} output frequencies / {} sources / {} input tuples; output {:?} at {:?}; full adjoint and unit transfers",
            response.metadata.output_frequencies_hz.len(),
            response.metadata.input_sources.len(),
            response.metadata.input_lattices.len(),
            response.metadata.request.output,
            response.metadata.request.output_lattice,
        ),
        AnalysisResultPayload::Qpac { response } => format!(
            "{} probe offsets / {} signed tuples / {} MNA coordinates; input {:?}, output {:?}",
            response.metadata.request.offsets_hz.len(),
            response.metadata.tuples.len(),
            response.metadata.node_names.len() + response.metadata.branch_names.len(),
            response.metadata.request.input_lattice,
            response.metadata.request.output_lattice,
        ),
        AnalysisResultPayload::Qpss { operating_point } => format!(
            "{} independent tones / {} MNA coordinates / {} signed spectral coefficients",
            operating_point.config().grid.frequencies_hz.len(),
            operating_point.spectra().len(),
            operating_point
                .spectra()
                .iter()
                .map(Vec::len)
                .sum::<usize>(),
        ),
        AnalysisResultPayload::DcSweep { evidence } => format!(
            "{} solved quantities / {} sweep members / {} primary traversal",
            evidence.quantities.len(),
            evidence.member_count(),
            evidence.direction.label(),
        ),
        AnalysisResultPayload::OperatingPoint {
            mna_node_names,
            mna_branch_names,
            ..
        } => format!(
            "{} MNA nodes / {} MNA branches",
            mna_node_names.len(),
            mna_branch_names.len()
        ),
        AnalysisResultPayload::PoleZero { poles, zeros, .. } => {
            format!("{} poles / {} zeros", poles.len(), zeros.len())
        }
        AnalysisResultPayload::PssFloquet {
            multipliers,
            floquet_evidence,
            stability_verdict,
            ..
        } => format!(
            "{} / {} / {}",
            floquet_manifest_count_label(multipliers.len(), floquet_evidence, "PSS multipliers"),
            floquet_manifest_evidence_label(floquet_evidence),
            floquet_manifest_verdict_label(*stability_verdict),
        ),
        AnalysisResultPayload::Pstb {
            modes,
            floquet_evidence,
            stability_verdict,
            stability_classification,
            num_unstable,
            ..
        } => format!(
            "{} / {} unstable / {} / {} ({})",
            floquet_manifest_count_label(modes.len(), floquet_evidence, "PSTB modes"),
            num_unstable.map_or_else(|| "legacy unknown".to_owned(), |count| count.to_string()),
            floquet_manifest_evidence_label(floquet_evidence),
            floquet_manifest_verdict_label(*stability_verdict),
            pstb_manifest_classification_label(*stability_classification),
        ),
        AnalysisResultPayload::Sensitivity { rows, .. } => {
            format!("{} sensitivities", rows.len())
        }
        AnalysisResultPayload::SensitivityStudy { evidence } => {
            let points = evidence.point_count();
            if points > 1 {
                format!(
                    "{} sensitivities at {points} frequencies",
                    evidence.rows.len()
                )
            } else {
                format!("{} sensitivities", evidence.rows.len())
            }
        }
        AnalysisResultPayload::DcMismatch { evidence } => format!(
            "{} of {} mismatch contributors",
            evidence.retained_contributors(),
            evidence.evaluated_contributors
        ),
        AnalysisResultPayload::ScalarMeasurements { values } => {
            format!("{} scalar values", values.len())
        }
        AnalysisResultPayload::TransferFunction { .. } => "transfer / impedance scalars".to_owned(),
        AnalysisResultPayload::Soa {
            source_history: _,
            evaluations,
            violations,
        } => format!(
            "{} SOA evaluations / {} violations",
            evaluations.len(),
            violations.len()
        ),
        AnalysisResultPayload::FftSpectrum { spectrum } => format!(
            "{} one-sided coefficients / {} window / {} points",
            spectrum.bin_count(),
            spectrum.window,
            spectrum.point_count
        ),
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            current_impulses,
            ..
        } => {
            let events: usize = digital_traces
                .iter()
                .map(|trace| trace.points.len())
                .chain(real_traces.iter().map(|trace| trace.points.len()))
                .sum();
            let impulses = current_impulses.as_ref().map_or(0, |history| {
                history
                    .traces
                    .iter()
                    .map(|trace| trace.points.len())
                    .sum::<usize>()
            });
            format!(
                "{} event nodes / {events} committed events / {impulses} current impulses",
                digital_traces.len() + real_traces.len()
            )
        }
    }
}

fn floquet_manifest_count_label(
    count: usize,
    evidence: &crate::floquet::FloquetSpectrumEvidence,
    noun: &str,
) -> String {
    if matches!(
        evidence,
        crate::floquet::FloquetSpectrumEvidence::Qualified { .. }
            | crate::floquet::FloquetSpectrumEvidence::NoDynamicModes
    ) {
        format!("{count} complete {noun}")
    } else {
        format!("{count} retained {noun}; completeness unavailable")
    }
}

const fn floquet_manifest_evidence_label(
    evidence: &crate::floquet::FloquetSpectrumEvidence,
) -> &'static str {
    match evidence {
        crate::floquet::FloquetSpectrumEvidence::NotComputed => "not computed",
        crate::floquet::FloquetSpectrumEvidence::NoDynamicModes => "no dynamic modes",
        crate::floquet::FloquetSpectrumEvidence::Qualified { .. } => "strictly qualified",
        crate::floquet::FloquetSpectrumEvidence::LegacyUnknown => "legacy evidence unknown",
    }
}

const fn floquet_manifest_verdict_label(
    verdict: crate::floquet::FloquetStabilityVerdictEvidence,
) -> &'static str {
    match verdict {
        crate::floquet::FloquetStabilityVerdictEvidence::Stable => "stable",
        crate::floquet::FloquetStabilityVerdictEvidence::Unstable => "unstable",
        crate::floquet::FloquetStabilityVerdictEvidence::Marginal => "marginal",
        crate::floquet::FloquetStabilityVerdictEvidence::Indeterminate => "indeterminate",
    }
}

const fn pstb_manifest_classification_label(
    classification: crate::floquet::PstbStabilityClassificationEvidence,
) -> &'static str {
    use crate::floquet::PstbStabilityClassificationEvidence as Classification;
    match classification {
        Classification::Stable => "stable",
        Classification::UnstableReal => "unstable real",
        Classification::UnstableComplex => "unstable complex",
        Classification::PeriodDoubling => "period doubling",
        Classification::NeimarkSacker => "Neimark-Sacker",
        Classification::SaddleNode => "saddle-node",
        Classification::Marginal => "marginal",
        Classification::Indeterminate => "indeterminate",
    }
}

fn expansion_label<W: AsRef<RetainedWaveform>>(analysis: &AnalysisResult<W>) -> String {
    analysis.provenance().map_or_else(
        || "legacy result".to_owned(),
        |provenance| {
            if provenance.authored_source_instance_id() == provenance.source_instance_id() {
                "single task".to_owned()
            } else {
                "materialized PVT point".to_owned()
            }
        },
    )
}

const fn missing_result_status(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "pending · not yet retained",
        SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => "not produced",
        SimulationRunLifecycle::Completed | SimulationRunLifecycle::LegacyUnknown => "not retained",
    }
}

/// The run-wide qualification line.
///
/// A valid receipt never *grants* sign-off — nothing here retains a sign-off
/// record — so the terminal case still reports it unavailable. What it also
/// does is name the blocker the receipt carries, read from the one owner
/// [`crate::run_receipt::PreparedRunReceipt::sign_off_blocker`].
///
/// Its doc said that before this. What the code did was rebuild the verdict out
/// of `unqualified_model_sources()` and `preview_engine_kinds()` — the two
/// halves that owner is a fold of — and restate them in a vocabulary of its
/// own, so a third disqualifying condition added to the receipt would leave
/// this line calling the run merely unqualified while Verify's tile refused it,
/// and the blocker named the objects while this named only their category.
///
/// The eligible case then stayed behind: it read "unavailable · no retained
/// sign-off qualification" for a receipt nothing disqualifies. Both cases come
/// from [`crate::run_receipt::SignOffStanding`] now, which is where Verify's tile
/// reads them too.
fn qualification_label<A>(run: &SimulationRun<A>, provenance_is_valid: bool) -> String {
    match run.lifecycle {
        SimulationRunLifecycle::LegacyUnknown => {
            "unavailable · legacy lifecycle unknown · non-sign-off".to_owned()
        }
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => {
            "unavailable · run is not terminal · non-sign-off".to_owned()
        }
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => {
            let Some(receipt) = run.prepared_receipt() else {
                return "unavailable · no retained qualification authority · non-sign-off"
                    .to_owned();
            };
            if !provenance_is_valid {
                return "blocked · receipt integrity mismatch · non-sign-off".to_owned();
            }
            // Both cases from the one owner. This cell wrote its own sentence
            // for the eligible one — "unavailable · no retained sign-off
            // qualification" — over the same receipt Verify's tile was
            // stamping `Eligible`, which is two surfaces of one workbench
            // disagreeing about whether the dataset in front of the reader may
            // be cited as evidence.
            receipt.sign_off_standing().qualification()
        }
    }
}

const fn inventory_title(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "Legacy analysis inventory",
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "Live analysis inventory",
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => "Retained analysis inventory",
    }
}

const fn inventory_status(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => {
            "legacy manifest · mutability authority unavailable"
        }
        SimulationRunLifecycle::Preparing
        | SimulationRunLifecycle::Running
        | SimulationRunLifecycle::Cancelling => "live manifest · digest changes until terminal",
        SimulationRunLifecycle::Completed
        | SimulationRunLifecycle::Failed
        | SimulationRunLifecycle::Aborted
        | SimulationRunLifecycle::Interrupted => "locked manifest",
    }
}

const fn lifecycle_label(lifecycle: SimulationRunLifecycle) -> &'static str {
    match lifecycle {
        SimulationRunLifecycle::LegacyUnknown => "legacy status unknown",
        SimulationRunLifecycle::Preparing => "preparing",
        SimulationRunLifecycle::Running => "running",
        SimulationRunLifecycle::Cancelling => "cancelling",
        SimulationRunLifecycle::Completed => "completed",
        SimulationRunLifecycle::Failed => "failed",
        SimulationRunLifecycle::Aborted => "aborted",
        SimulationRunLifecycle::Interrupted => "interrupted",
    }
}

const fn source_domain_label(domain: AnalysisResultSourceDomain) -> &'static str {
    match domain {
        AnalysisResultSourceDomain::SimulationPlan => "simulation plan",
        AnalysisResultSourceDomain::ManualDeck => "manual deck",
        AnalysisResultSourceDomain::LegacyUnclassified => "legacy unclassified",
    }
}

const fn plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waveform::RetainedWaveform as WaveformData;
    use crate::{
        analysis_tag::CanonicalAnalysisKind,
        provenance::{AnalysisResultProvenance, AnalysisResultSourceDomain},
        run::{ExecutionTarget, SimulationRunLifecycle},
        run_receipt::{
            PreparedRunReceipt, PreparedRunReceiptInput, PreparedRunTaskReceipt,
            PreparedSourceCheckReceipt,
        },
    };
    use rspice_app_types::product::{
        AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId,
    };
    type SimulationRun = crate::run::SimulationRun<AnalysisResult>;
    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }
    fn row_for_analysis(analysis: AnalysisResult) -> ManifestRow {
        let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
        run.add_analysis(analysis);
        ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial())
            .rows
            .remove(0)
    }
    #[test]
    fn legacy_manifest_is_digest_bound_and_fails_closed() {
        let mut run = SimulationRun::new(7, 0.0, ExecutionTarget::LocalDesktop);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0).with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0]),
            ]),
        );

        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());

        assert_eq!(manifest.dataset_id, run.dataset_id.to_string());
        assert_eq!(
            manifest.dataset_digest,
            run.dataset_content_digest_with_encoding(
                crate::result_digest::ResultDigestEncoding::CURRENT
            )
            .to_string()
        );
        assert_eq!(manifest.rows.len(), 1);
        assert_eq!(manifest.rows[0].domain_axis, "adaptive time");
        assert!(manifest.rows[0].stored_values.contains("2 samples"));
        assert_eq!(
            manifest.rows[0].eligibility,
            "legacy · no prepared receipt · sign-off unavailable"
        );
        assert_eq!(
            manifest.qualification,
            "unavailable · no retained qualification authority · non-sign-off"
        );
        assert_eq!(manifest.inventory_title, "Retained analysis inventory");
    }
    #[test]
    fn active_manifest_never_claims_to_be_frozen_or_qualified() {
        let run = SimulationRun::new(8, 0.0, ExecutionTarget::LocalDesktop);

        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());

        assert_eq!(manifest.inventory_title, "Live analysis inventory");
        assert!(manifest.inventory_status.starts_with("live manifest"));
        assert_eq!(
            manifest.qualification,
            "unavailable · run is not terminal · non-sign-off"
        );
        assert!(!manifest.inventory_title.contains("Frozen"));
    }
    #[test]
    fn legacy_unknown_manifest_does_not_claim_live_or_locked_authority() {
        let mut run = SimulationRun::new(9, 0.0, ExecutionTarget::LocalDesktop);
        run.lifecycle = SimulationRunLifecycle::LegacyUnknown;

        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());

        assert_eq!(manifest.inventory_title, "Legacy analysis inventory");
        assert!(manifest.inventory_status.starts_with("legacy manifest"));
        assert_eq!(
            manifest.qualification,
            "unavailable · legacy lifecycle unknown · non-sign-off"
        );
    }
    #[test]
    fn a_preview_engine_run_is_blocked_in_the_words_the_receipt_uses() {
        let instance_id = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0x51);
        let envelope_tag = CanonicalAnalysisKind::Envelope.tag();
        let receipt = PreparedRunReceipt::new(PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: revision,
            prepared_snapshot_digest: snapshot,
            source_content_digest: digest(0x52),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x53)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy: crate::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    instance_id,
                    revision,
                    Vec::new(),
                    envelope_tag,
                    digest(0x54),
                )
                .expect("valid task"),
            ],
        })
        .expect("valid receipt");
        let blocker = receipt
            .sign_off_blocker()
            .expect("a preview kind blocks sign-off");
        let provenance = AnalysisResultProvenance::new(instance_id, revision, snapshot, Vec::new())
            .expect("valid provenance");
        let mut run = SimulationRun::new_prepared(11, 0.0, ExecutionTarget::LocalDesktop, receipt);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Envelope, "Envelope", 0.0)
                .with_provenance(provenance),
        );

        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());

        assert_eq!(
            manifest.qualification,
            format!("blocked · {blocker} · non-sign-off")
        );
        assert!(
            manifest.qualification.contains("Envelope"),
            "the owner names the object, so this line does too: {}",
            manifest.qualification
        );
        assert_eq!(
            manifest.rows[0].eligibility,
            "retained · preview engine · non-sign-off"
        );
    }
    #[test]
    fn an_eligible_receipt_is_called_eligible_in_the_words_both_surfaces_use() {
        let instance_id = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0x41);
        let receipt = PreparedRunReceipt::new(PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(SimulationPlanId::new()),
            project_revision: revision,
            prepared_snapshot_digest: snapshot,
            source_content_digest: digest(0x42),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0x43)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy: crate::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(instance_id, revision, Vec::new(), 5, digest(0x44))
                    .expect("valid task"),
            ],
        })
        .expect("valid receipt");
        let provenance = AnalysisResultProvenance::new(instance_id, revision, snapshot, Vec::new())
            .expect("valid provenance");
        let mut run = SimulationRun::new_prepared(10, 0.0, ExecutionTarget::LocalDesktop, receipt);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0)
                .with_provenance(provenance),
        );

        let standing = run
            .prepared_receipt()
            .expect("the run carries its receipt")
            .sign_off_standing();
        let manifest = ManifestViewModel::from_run(&run, |analysis| analysis.is_live_partial());

        // What Verify's tile stamps, and what this cell prints, for the one
        // receipt.
        assert_eq!(standing.verdict(), "Eligible");
        assert_eq!(
            manifest.qualification,
            "eligible · every model released · every analysis production"
        );
        assert_eq!(manifest.qualification, standing.qualification());
        assert!(
            !manifest.qualification.contains("unavailable")
                && !manifest.qualification.contains("blocked"),
            "nothing disqualifies this run: {}",
            manifest.qualification
        );
        assert_eq!(
            manifest.rows[0].eligibility,
            "retained · receipt matched · sign-off unavailable"
        );
    }
    #[test]
    fn every_analysis_kind_has_a_truthful_domain_contract() {
        let kinds = [
            AnalysisType::DcOp,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
            AnalysisType::Disto,
            AnalysisType::Transient,
            AnalysisType::Noise,
            AnalysisType::PoleZero,
            AnalysisType::Tf,
            AnalysisType::Sensitivity,
            AnalysisType::Pac,
            AnalysisType::Pnoise,
            AnalysisType::Pxf,
            AnalysisType::Pstb,
            AnalysisType::Stb,
            AnalysisType::MonteCarlo,
            AnalysisType::Parametric,
            AnalysisType::Corner,
            AnalysisType::Optimization,
            AnalysisType::Soa,
            AnalysisType::SParameter,
            AnalysisType::Envelope,
            AnalysisType::Fourier,
            AnalysisType::HarmonicBalance,
            AnalysisType::Pss,
            AnalysisType::Qpss,
            AnalysisType::Hbsp,
            AnalysisType::Hbnoise,
            AnalysisType::Psp,
            AnalysisType::Qpac,
            AnalysisType::Qpnoise,
            AnalysisType::Qpxf,
            AnalysisType::TransientNoise,
            AnalysisType::DcMismatch,
        ];
        for kind in kinds {
            let meta = row_for_analysis(AnalysisResult::new(1, kind, "domain", 0.0));
            assert!(!meta.domain_axis.is_empty(), "{kind:?}");
            assert!(!meta.precision.is_empty(), "{kind:?}");
        }
    }
    #[test]
    fn periodic_payload_manifest_uses_complete_floquet_semantics() {
        let pss = AnalysisResult::new(1, AnalysisType::Pss, "PSS", 0.0).with_result_payload(
            AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pss).unwrap(),
        );
        let pstb = AnalysisResult::new(2, AnalysisType::Pstb, "PSTB", 0.0).with_result_payload(
            AnalysisResultPayload::legacy_periodic_marker(AnalysisType::Pstb).unwrap(),
        );

        let pss = row_for_analysis(pss);
        let pstb = row_for_analysis(pstb);
        assert_eq!(pstb.domain_axis, "Floquet mode index");
        assert_eq!(pss.precision, "complex128");
        assert_eq!(pstb.precision, "complex128");
        assert!(
            pss.stored_values.contains("retained PSS multipliers"),
            "{}",
            pss.stored_values
        );
        assert!(
            pstb.stored_values.contains("retained PSTB modes"),
            "{}",
            pstb.stored_values
        );
    }
}
