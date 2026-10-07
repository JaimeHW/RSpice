//! Exact hardcopy tables resolved from validated retained analysis evidence.

use super::*;
use rspice_results::{
    analysis_payload::AnalysisResultPayload, analysis_result::AnalysisResult,
    family_metadata::AnalysisResultFamilyMetadata, waveform::RetainedWaveform,
};

pub const fn is_curve_viewer(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::Bode
            | ResultViewer::NoiseContrib
            | ResultViewer::Fft
            | ResultViewer::HarmonicBalance
            | ResultViewer::PhaseNoise
            | ResultViewer::Eye
            | ResultViewer::Hist
            | ResultViewer::Nyquist
            | ResultViewer::Smith
            | ResultViewer::Polar
    )
}

/// Resolve report tables without consulting application or viewer state.
pub fn semantic_result_summary<W: AsRef<RetainedWaveform>>(
    viewer: ResultViewer,
    analysis: &AnalysisResult<W>,
) -> Result<SemanticResultSummary, HardcopySourceError> {
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    let mut tables = Vec::new();
    match viewer {
        ResultViewer::Op => {
            if let Some(operating_point) = &analysis.dc_op {
                for (title, values) in [
                    ("Node voltages", &operating_point.node_voltages),
                    ("Branch currents", &operating_point.branch_currents),
                    ("Device power", &operating_point.power_dissipation),
                ] {
                    if !values.is_empty() {
                        tables.push(SemanticTable {
                            title: title.to_owned(),
                            columns: vec![
                                "Quantity".to_owned(),
                                "Value".to_owned(),
                                "Unit".to_owned(),
                            ],
                            rows: values
                                .iter()
                                .map(|value| {
                                    vec![
                                        value.name.clone(),
                                        exact_number(value.value),
                                        value.unit.clone(),
                                    ]
                                })
                                .collect(),
                        });
                    }
                }
            }
            if let Some(report) = analysis.device_op.as_ref()
                && !report.is_empty()
            {
                let mut rows = Vec::new();
                for entry in &report.entries {
                    if entry.params.is_empty() {
                        rows.push(vec![
                            entry.name.clone(),
                            entry.device_kind.to_owned(),
                            entry.region.unwrap_or_default().to_owned(),
                            String::new(),
                            String::new(),
                            String::new(),
                        ]);
                    } else {
                        rows.extend(entry.params.iter().map(|(name, value)| {
                            vec![
                                entry.name.clone(),
                                entry.device_kind.to_owned(),
                                entry.region.unwrap_or_default().to_owned(),
                                (*name).to_owned(),
                                exact_number(*value),
                                rspice_results::operating_point::report::device_param_unit(
                                    entry.device_kind,
                                    name,
                                )
                                .to_owned(),
                            ]
                        }));
                    }
                }
                tables.push(SemanticTable {
                    title: "Device operating point".to_owned(),
                    columns: vec![
                        "Device".to_owned(),
                        "Family".to_owned(),
                        "Region".to_owned(),
                        "Quantity".to_owned(),
                        "Value".to_owned(),
                        "Unit".to_owned(),
                    ],
                    rows,
                });
            }
            if tables.is_empty()
                && !matches!(
                    analysis.result_payload.as_ref(),
                    Some(AnalysisResultPayload::OperatingPoint { .. })
                )
            {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "operating point",
                ));
            }
        }
        ResultViewer::NoiseContrib => {
            let summary = analysis.noise_summary.as_ref().ok_or(
                HardcopySourceError::MissingViewerEvidence("noise contributor summary"),
            )?;
            tables.push(SemanticTable {
                title: format!(
                    "Noise contributors · {} Hz to {} Hz",
                    exact_number(summary.band.0),
                    exact_number(summary.band.1)
                ),
                columns: vec![
                    "Device".to_owned(),
                    "Mechanism".to_owned(),
                    format!("Power ({})", summary.power_unit()),
                    "Share (%)".to_owned(),
                ],
                rows: summary
                    .rows
                    .iter()
                    .map(|row| {
                        vec![
                            row.device.clone(),
                            row.mechanism.clone(),
                            exact_number(row.power),
                            exact_number(row.share_pct),
                        ]
                    })
                    .collect(),
            });
            tables.push(SemanticTable {
                title: "Integrated totals".to_owned(),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec![
                        "Output referred (V rms)".to_owned(),
                        summary
                            .total_rms
                            .map_or_else(|| "not retained".to_owned(), exact_number),
                    ],
                    vec![
                        format!("Input referred ({})", summary.input_rms_unit()),
                        summary
                            .input_rms
                            .map_or_else(|| "not retained".to_owned(), exact_number),
                    ],
                ],
            });
        }
        // The sheet serves two families, so the print does too — and the
        // refusal names the one that is actually missing rather than blaming
        // the sensitivity payload for a DC mismatch result's absence.
        ResultViewer::Contribution => match &analysis.result_payload {
            Some(AnalysisResultPayload::Sensitivity { output, rows, .. }) => {
                tables.push(SemanticTable {
                    title: format!("Sensitivity of {output}"),
                    columns: vec![
                        "Parameter".to_owned(),
                        "Raw".to_owned(),
                        "Normalized".to_owned(),
                    ],
                    rows: rows
                        .iter()
                        .map(|row| {
                            vec![
                                row.parameter.clone(),
                                sensitivity_number(row.raw),
                                sensitivity_number(row.normalized),
                            ]
                        })
                        .collect(),
                });
            }
            // Long form, frequency then variable: a printed table of a swept
            // study is read down the band for one variable and across it for
            // one frequency, and only one of those orders can be the rows'.
            // A DC study carries neither column, so it prints neither.
            Some(AnalysisResultPayload::SensitivityStudy { evidence }) => {
                let swept = evidence.frequency_at(0).is_some();
                let mut columns = vec!["Parameter".to_owned()];
                if swept {
                    columns.push("Frequency".to_owned());
                }
                columns.push("Raw".to_owned());
                columns.push("Normalized".to_owned());
                if swept {
                    columns.push("Phase".to_owned());
                }
                let mut printed = Vec::with_capacity(evidence.rows.len() * evidence.point_count());
                for point in 0..evidence.point_count() {
                    for row in &evidence.rows {
                        let mut cells = vec![row.parameter.clone()];
                        if let Some(frequency) = evidence.frequency_at(point) {
                            cells.push(exact_number(frequency));
                        }
                        cells.push(sensitivity_number(row.raw[point]));
                        cells.push(sensitivity_number(row.normalized[point]));
                        if let Some(phase) = row.phase.get(point) {
                            cells.push(sensitivity_number(*phase));
                        }
                        printed.push(cells);
                    }
                }
                tables.push(SemanticTable {
                    title: format!("Sensitivity of {}", evidence.output),
                    columns,
                    rows: printed,
                });
            }
            Some(AnalysisResultPayload::DcMismatch { evidence }) => {
                // The spread first, then the ranked list: a contributor table
                // read without the total it divides says nothing.
                tables.push(SemanticTable {
                    title: format!("DC mismatch of {}", evidence.output),
                    columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                    rows: vec![
                        vec!["Nominal".to_owned(), exact_number(evidence.nominal_value)],
                        vec!["Sigma total".to_owned(), exact_number(evidence.sigma_total)],
                        vec![
                            "Sigma mismatch".to_owned(),
                            exact_number(evidence.sigma_mismatch),
                        ],
                        vec![
                            "Sigma process".to_owned(),
                            exact_number(evidence.sigma_process),
                        ],
                        vec![
                            format!("{} sigma", evidence.sigma_multiplier),
                            exact_number(evidence.quoted_sigma()),
                        ],
                        vec![
                            "Retained".to_owned(),
                            format!(
                                "{} of {} evaluated",
                                evidence.retained_contributors(),
                                evidence.evaluated_contributors
                            ),
                        ],
                    ],
                });
                let cumulative = evidence.cumulative_shares();
                tables.push(SemanticTable {
                    title: "Contributors".to_owned(),
                    columns: vec![
                        "Rank".to_owned(),
                        "Instance".to_owned(),
                        "Parameter".to_owned(),
                        "Scope".to_owned(),
                        "Contribution".to_owned(),
                        "Share".to_owned(),
                        "Cumulative".to_owned(),
                    ],
                    rows: evidence
                        .contributors
                        .iter()
                        .zip(&cumulative)
                        .enumerate()
                        .map(|(rank, (row, running))| {
                            vec![
                                (rank + 1).to_string(),
                                row.instance.clone(),
                                row.parameter.clone(),
                                row.scope.tag().to_owned(),
                                exact_number(row.contribution),
                                exact_number(row.share),
                                exact_number(*running),
                            ]
                        })
                        .collect(),
                });
            }
            _ => {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "sensitivity or DC mismatch",
                ));
            }
        },
        ResultViewer::TransferFunction => {
            let Some(AnalysisResultPayload::TransferFunction {
                input_source,
                output_expression,
                gain,
                input_resistance,
                output_resistance,
                ..
            }) = &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "transfer function",
                ));
            };
            tables.push(SemanticTable {
                title: format!("{output_expression} / {input_source}"),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec!["Gain".to_owned(), format_optional_scalar(*gain)],
                    vec![
                        "Input resistance".to_owned(),
                        format_optional_scalar(*input_resistance),
                    ],
                    vec![
                        "Output resistance".to_owned(),
                        format_optional_scalar(*output_resistance),
                    ],
                ],
            });
        }
        ResultViewer::Specs => {
            if !analysis.measurements.is_empty() {
                tables.push(SemanticTable {
                    title: "Measurements and specifications".to_owned(),
                    columns: vec![
                        "Measurement".to_owned(),
                        "Value".to_owned(),
                        "Expected".to_owned(),
                        "Tolerance".to_owned(),
                        "Status".to_owned(),
                    ],
                    rows: analysis
                        .measurements
                        .iter()
                        .map(|measurement| {
                            vec![
                                measurement.name.clone(),
                                measurement
                                    .value
                                    .map_or_else(|| "not available".to_owned(), exact_number),
                                measurement
                                    .expected
                                    .map_or_else(|| "—".to_owned(), exact_number),
                                measurement
                                    .tolerance
                                    .map_or_else(|| "—".to_owned(), exact_number),
                                if measurement.passed { "pass" } else { "fail" }.to_owned(),
                            ]
                        })
                        .collect(),
                });
            } else if let Some(AnalysisResultPayload::ScalarMeasurements { values }) =
                &analysis.result_payload
            {
                tables.push(SemanticTable {
                    title: "Scalar measurements".to_owned(),
                    columns: vec!["Measurement".to_owned(), "Value".to_owned()],
                    rows: values
                        .iter()
                        .map(|(name, value)| vec![name.clone(), exact_number(*value)])
                        .collect(),
                });
            } else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "measurement/specification",
                ));
            }
        }
        ResultViewer::Table => {
            let Some(payload) = analysis.result_payload.as_ref() else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "typed periodic result",
                ));
            };
            tables.extend(periodic_result_tables(payload).ok_or(
                HardcopySourceError::MissingViewerEvidence("typed periodic result"),
            )?);
        }
        ResultViewer::NetworkMatrix => {
            let retained =
                rspice_results::network_matrix::report_tables(analysis).map_err(|reason| {
                    HardcopySourceError::InvalidVisualizationSource(reason.to_owned())
                })?;
            tables.extend(retained.into_iter().map(|table| SemanticTable {
                title: table.title,
                columns: table.columns,
                rows: table.rows,
            }));
        }
        ResultViewer::PoleZero => {
            let Some(AnalysisResultPayload::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            }) = &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("pole-zero"));
            };
            let mut rows = Vec::with_capacity(poles.len() + zeros.len());
            rows.extend(poles.iter().enumerate().map(|(index, value)| {
                vec![
                    format!("P{}", index + 1),
                    exact_number(value.real),
                    exact_number(value.imaginary),
                ]
            }));
            rows.extend(zeros.iter().enumerate().map(|(index, value)| {
                vec![
                    format!("Z{}", index + 1),
                    exact_number(value.real),
                    exact_number(value.imaginary),
                ]
            }));
            let gain = gain
                .map(exact_number)
                .unwrap_or_else(|| "unavailable".to_owned());
            tables.push(SemanticTable {
                title: format!("Pole-zero roots · gain {gain}"),
                columns: vec!["Root".to_owned(), "Real".to_owned(), "Imaginary".to_owned()],
                rows,
            });
            tables.push(SemanticTable {
                title: "Root-set qualification evidence".to_owned(),
                columns: vec![
                    "Set".to_owned(),
                    "Status".to_owned(),
                    "Order".to_owned(),
                    "Infinite".to_owned(),
                    "Max backward error".to_owned(),
                    "Qualification tolerance".to_owned(),
                ],
                rows: [("Poles", pole_evidence), ("Zeros", zero_evidence)]
                    .into_iter()
                    .map(|(name, evidence)| {
                        let certificate = evidence.certificate();
                        vec![
                            name.to_owned(),
                            evidence.label().to_owned(),
                            certificate
                                .map(|certificate| certificate.problem_order.to_string())
                                .unwrap_or_else(|| "—".to_owned()),
                            certificate
                                .map(|certificate| certificate.infinite_count.to_string())
                                .unwrap_or_else(|| "—".to_owned()),
                            certificate
                                .map(|certificate| exact_number(certificate.max_backward_error))
                                .unwrap_or_else(|| "—".to_owned()),
                            certificate
                                .map(|certificate| {
                                    exact_number(certificate.qualification_tolerance)
                                })
                                .unwrap_or_else(|| "—".to_owned()),
                        ]
                    })
                    .collect(),
            });
        }
        ResultViewer::Events => {
            let Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                ..
            }) = &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("event history"));
            };
            let mut rows = Vec::new();
            for trace in digital_traces {
                rows.extend(trace.points.iter().map(|point| {
                    vec![
                        exact_number(point.time_s),
                        trace.node_name.clone(),
                        "digital".to_owned(),
                        point.value_code.to_string(),
                    ]
                }));
            }
            for trace in real_traces {
                rows.extend(trace.points.iter().map(|point| {
                    vec![
                        exact_number(point.time_s),
                        trace.node_name.clone(),
                        "real".to_owned(),
                        exact_number(point.value),
                    ]
                }));
            }
            // One merged schedule in time order, which is how the sheet reads
            // it: a per-node listing would hide the ordering between nodes.
            rows.sort_by(|left, right| left[0].cmp(&right[0]).then_with(|| left[1].cmp(&right[1])));
            tables.push(SemanticTable {
                title: format!(
                    "Committed event history · {} nodes",
                    digital_traces.len() + real_traces.len()
                ),
                columns: vec![
                    "Time (s)".to_owned(),
                    "Node".to_owned(),
                    "Domain".to_owned(),
                    "Value".to_owned(),
                ],
                rows,
            });
        }
        ResultViewer::Soa => {
            let Some(AnalysisResultPayload::Soa {
                source_history: _,
                evaluations,
                violations,
            }) = &analysis.result_payload
            else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "safe operating area",
                ));
            };
            tables.push(SemanticTable {
                title: format!(
                    "Safe-operating-area rules · {} retained violation events",
                    violations.len()
                ),
                columns: vec![
                    "Device".to_owned(),
                    "Rule".to_owned(),
                    "Observed".to_owned(),
                    "Limit".to_owned(),
                    "Unit".to_owned(),
                    "Worst time (s)".to_owned(),
                    "Verdict".to_owned(),
                ],
                rows: evaluations
                    .iter()
                    .map(|evaluation| {
                        vec![
                            evaluation.device_id.clone(),
                            evaluation.description.clone(),
                            exact_number(evaluation.worst_actual_value),
                            exact_number(evaluation.limit_value),
                            evaluation.unit.clone(),
                            exact_number(evaluation.worst_time_s),
                            evaluation.verdict.label().to_owned(),
                        ]
                    })
                    .collect(),
            });
            for evaluation in evaluations {
                if let Some(envelope) = &evaluation.envelope {
                    let curve = &envelope.curve;
                    let mut rows = Vec::new();
                    if let Some(dc) = &curve.dc_currents_a {
                        for (v, i) in curve.voltages_v.iter().zip(dc) {
                            rows.push(vec!["DC".into(), exact_number(*v), exact_number(*i)]);
                        }
                    }
                    for pulse in &curve.pulses {
                        for (v, i) in curve.voltages_v.iter().zip(&pulse.currents_a) {
                            rows.push(vec![
                                exact_number(pulse.duration_s),
                                exact_number(*v),
                                exact_number(*i),
                            ]);
                        }
                    }
                    tables.push(SemanticTable {
                        title: format!(
                            "{} SOA curves · {} · {} · voltage {:?}, pulse {:?}",
                            evaluation.device_id,
                            curve.source,
                            curve.conditions,
                            curve.voltage_interpolation,
                            curve.pulse_interpolation
                        ),
                        columns: vec![
                            "Pulse (s) / DC".into(),
                            "Voltage (V)".into(),
                            "Allowed current (A)".into(),
                        ],
                        rows,
                    });
                }
            }
        }
        ResultViewer::Optimization => {
            let Some(AnalysisResultFamilyMetadata::Optimization {
                best_cost,
                best_variables,
                best_objectives,
                best_constraints,
                converged,
                ..
            }) = &analysis.family_metadata
            else {
                return Err(HardcopySourceError::MissingViewerEvidence("optimization"));
            };
            let mut rows = vec![vec![
                "Outcome".to_owned(),
                if *converged {
                    "converged".to_owned()
                } else {
                    "stopped without converging".to_owned()
                },
            ]];
            rows.push(vec!["Best cost".to_owned(), exact_number(*best_cost)]);
            rows.extend(
                best_variables
                    .iter()
                    .map(|(name, value)| vec![name.clone(), exact_number(*value)]),
            );
            for (index, observation) in best_objectives.iter().enumerate() {
                let term = &observation.objective;
                rows.push(vec![
                    format!(
                        "Objective {}: {} ({:?})",
                        index + 1,
                        term.measurement,
                        term.goal
                    ),
                    exact_number(observation.value),
                ]);
                rows.push(vec![
                    format!("Objective {} configuration", index + 1),
                    format!(
                        "unit {}; target {}; scale {}; weight {}",
                        if term.unit.is_empty() {
                            "native"
                        } else {
                            &term.unit
                        },
                        term.target.map(exact_number).unwrap_or_else(|| "—".into()),
                        exact_number(term.scale),
                        exact_number(term.weight)
                    ),
                ]);
                rows.push(vec![
                    format!("Objective {} cost", index + 1),
                    exact_number(observation.contribution),
                ]);
            }
            for (index, observation) in best_constraints.iter().enumerate() {
                let term = &observation.constraint;
                rows.push(vec![format!("Constraint {}: {}", index + 1, term.measurement), format!("unit {}; {}; value {}; lower {}; upper {}; tolerance {}; scale {}; normalized violation {}",
                    if term.unit.is_empty() { "native" } else { &term.unit },
                    if observation.violation == 0.0 { "satisfied" } else { "violated" }, exact_number(observation.value),
                    term.lower.map(exact_number).unwrap_or_else(|| "unbounded".into()), term.upper.map(exact_number).unwrap_or_else(|| "unbounded".into()),
                    exact_number(term.tolerance), exact_number(term.scale), exact_number(observation.violation))]);
            }
            tables.push(SemanticTable {
                title: "Optimizer outcome and best candidate".to_owned(),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows,
            });
        }
        // Both statistical sheets print the population itself rather than a
        // re-derivation of it: the correlation, the quartiles and the yield
        // the reader saw are functions of these rows, and a printed page that
        // carries the rows can be checked, while one that carries only the
        // summary can only be believed.
        ResultViewer::Scatter | ResultViewer::BoxViolin => {
            let Some(AnalysisResultFamilyMetadata::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                failures,
                variables,
                member_measurements,
                ..
            }) = &analysis.family_metadata
            else {
                return Err(HardcopySourceError::MissingViewerEvidence(
                    "Monte Carlo population",
                ));
            };
            tables.push(SemanticTable {
                title: "Monte Carlo execution".to_owned(),
                columns: vec!["Quantity".to_owned(), "Value".to_owned()],
                rows: vec![
                    vec!["Seed".to_owned(), format!("0x{seed:X}")],
                    vec!["Trials requested".to_owned(), runs_requested.to_string()],
                    vec!["Trials completed".to_owned(), runs_completed.to_string()],
                    vec!["Trials that failed".to_owned(), failures.to_string()],
                ],
            });
            if !member_measurements.is_empty() {
                let mut names: Vec<String> = Vec::new();
                for member in member_measurements {
                    for evidence in &member.measurements {
                        if !names
                            .iter()
                            .any(|name| name.eq_ignore_ascii_case(&evidence.name))
                        {
                            names.push(evidence.name.clone());
                        }
                    }
                }
                let mut columns = vec!["Trial".to_owned()];
                columns.extend(names.iter().cloned());
                tables.push(SemanticTable {
                    title: "Retained trial measurements".to_owned(),
                    columns,
                    rows: member_measurements
                        .iter()
                        .map(|member| {
                            let mut row = vec![member.member.label()];
                            row.extend(names.iter().map(|name| {
                                member
                                    .evidence_for(name)
                                    .map_or_else(String::new, |evidence| {
                                        evidence.value.map_or_else(
                                            || {
                                                evidence
                                                    .error
                                                    .clone()
                                                    .unwrap_or_else(|| "not measured".to_owned())
                                            },
                                            exact_number,
                                        )
                                    })
                            }));
                            row
                        })
                        .collect(),
                });
            }
            if !variables.is_empty() {
                let mut columns = vec!["Sample".to_owned()];
                columns.extend(variables.iter().map(|variable| variable.name.clone()));
                let depth = variables
                    .iter()
                    .map(|variable| variable.samples.len())
                    .max()
                    .unwrap_or(0);
                tables.push(SemanticTable {
                    title: "Sampled variables".to_owned(),
                    columns,
                    rows: (0..depth)
                        .map(|index| {
                            let mut row = vec![index.to_string()];
                            row.extend(variables.iter().map(|variable| {
                                variable
                                    .samples
                                    .get(index)
                                    .copied()
                                    .map_or_else(String::new, exact_number)
                            }));
                            row
                        })
                        .collect(),
                });
            }
        }
        ResultViewer::Manifest => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                "dataset-native Manifest cannot be derived from one analysis".to_owned(),
            ));
        }
        viewer => {
            return Err(HardcopySourceError::UnsupportedVisualizationViewer(
                viewer.label().to_owned(),
            ));
        }
    }
    Ok(SemanticResultSummary {
        viewer,
        title: analysis.label.clone(),
        tables,
        payload: analysis.result_payload.clone(),
    })
}

fn periodic_result_tables(payload: &AnalysisResultPayload) -> Option<Vec<SemanticTable>> {
    match payload {
        AnalysisResultPayload::PssFloquet {
            period_s,
            fundamental_frequency_hz,
            iterations,
            residual_norm,
            multipliers,
            floquet_evidence,
            orbit_kind,
            trivial_multiplier_index,
            stability_verdict,
        } => {
            let mut metadata = vec![
                periodic_metadata_row("Period", optional_exact_number(*period_s), "s"),
                periodic_metadata_row(
                    "Fundamental frequency",
                    optional_exact_number(*fundamental_frequency_hz),
                    "Hz",
                ),
                periodic_metadata_row("Iterations", optional_u64(*iterations), "count"),
                periodic_metadata_row("Residual norm", optional_exact_number(*residual_norm), ""),
                periodic_metadata_row(
                    "Authenticated complete multiplier count",
                    authenticated_floquet_count(multipliers.len(), floquet_evidence),
                    "count",
                ),
                periodic_metadata_row(
                    "Floquet evidence",
                    floquet_evidence_label(floquet_evidence).to_owned(),
                    "",
                ),
                periodic_metadata_row(
                    "Orbit policy",
                    floquet_orbit_label(*orbit_kind).to_owned(),
                    "",
                ),
                periodic_metadata_row(
                    "Trivial multiplier index",
                    optional_u64(*trivial_multiplier_index),
                    "zero-based",
                ),
                periodic_metadata_row(
                    "Stability verdict",
                    floquet_verdict_label(*stability_verdict).to_owned(),
                    "",
                ),
            ];
            append_floquet_certificate_rows(&mut metadata, floquet_evidence);
            Some(vec![
                SemanticTable {
                    title: "PSS Floquet evidence and global metrics".to_owned(),
                    columns: vec!["Field".to_owned(), "Value".to_owned(), "Unit".to_owned()],
                    rows: metadata,
                },
                SemanticTable {
                    title: floquet_spectrum_table_title(
                        "PSS",
                        multipliers.len(),
                        floquet_evidence,
                        "multipliers",
                    ),
                    columns: vec![
                        "Mode (zero-based)".to_owned(),
                        "Multiplier real".to_owned(),
                        "Multiplier imaginary".to_owned(),
                    ],
                    rows: multipliers
                        .iter()
                        .enumerate()
                        .map(|(index, multiplier)| {
                            vec![
                                index.to_string(),
                                exact_number(multiplier.multiplier.real),
                                exact_number(multiplier.multiplier.imaginary),
                            ]
                        })
                        .collect(),
                },
            ])
        }
        AnalysisResultPayload::Pstb {
            period_s,
            fundamental_frequency_hz,
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
        } => {
            let mut metadata = vec![
                periodic_metadata_row("Period", optional_exact_number(*period_s), "s"),
                periodic_metadata_row(
                    "Fundamental frequency",
                    optional_exact_number(*fundamental_frequency_hz),
                    "Hz",
                ),
                periodic_metadata_row(
                    "Stability threshold",
                    optional_exact_number(*stability_threshold),
                    "|lambda|",
                ),
                periodic_metadata_row(
                    "Probe instance",
                    probe_instance
                        .clone()
                        .unwrap_or_else(|| "legacy unknown".to_owned()),
                    "",
                ),
                periodic_metadata_row(
                    "Subharmonic detection",
                    optional_bool(*detect_subharmonics),
                    "",
                ),
                periodic_metadata_row(
                    "Authenticated complete mode count",
                    authenticated_floquet_count(modes.len(), floquet_evidence),
                    "count",
                ),
                periodic_metadata_row(
                    "Floquet evidence",
                    floquet_evidence_label(floquet_evidence).to_owned(),
                    "",
                ),
                periodic_metadata_row(
                    "Orbit policy",
                    floquet_orbit_label(*orbit_kind).to_owned(),
                    "",
                ),
                periodic_metadata_row(
                    "Trivial multiplier index",
                    optional_u64(*trivial_multiplier_index),
                    "zero-based",
                ),
                periodic_metadata_row(
                    "Stability verdict",
                    floquet_verdict_label(*stability_verdict).to_owned(),
                    "",
                ),
                periodic_metadata_row(
                    "Stability classification",
                    pstb_classification_label(*stability_classification).to_owned(),
                    "",
                ),
                periodic_metadata_row(
                    "Minimum stability margin",
                    optional_exact_number(*min_stability_margin_db),
                    "dB",
                ),
                periodic_metadata_row(
                    "Maximum multiplier magnitude",
                    optional_exact_number(*max_multiplier_magnitude),
                    "",
                ),
                periodic_metadata_row("Unstable modes", optional_u64(*num_unstable), "count"),
                periodic_metadata_row(
                    "Subharmonic orders",
                    if subharmonics.is_empty() {
                        "none".to_owned()
                    } else {
                        subharmonics
                            .iter()
                            .map(u64::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                    "",
                ),
                periodic_metadata_row("Converged", optional_bool(*converged), ""),
                periodic_metadata_row("Iterations", optional_u64(*iterations), "count"),
            ];
            append_floquet_certificate_rows(&mut metadata, floquet_evidence);
            Some(vec![
                SemanticTable {
                    title: "PSTB Floquet evidence and global metrics".to_owned(),
                    columns: vec!["Field".to_owned(), "Value".to_owned(), "Unit".to_owned()],
                    rows: metadata,
                },
                SemanticTable {
                    title: floquet_spectrum_table_title(
                        "PSTB",
                        modes.len(),
                        floquet_evidence,
                        "modes",
                    ),
                    columns: vec![
                        "Mode (zero-based)".to_owned(),
                        "Multiplier real".to_owned(),
                        "Multiplier imaginary".to_owned(),
                        "Exponent real (1/s)".to_owned(),
                        "Exponent imaginary (1/s)".to_owned(),
                        "Probe participation".to_owned(),
                        "Unstable".to_owned(),
                        "Trivial phase mode".to_owned(),
                        "Subharmonic order".to_owned(),
                    ],
                    rows: modes
                        .iter()
                        .enumerate()
                        .map(|(index, mode)| {
                            vec![
                                index.to_string(),
                                exact_number(mode.multiplier.real),
                                exact_number(mode.multiplier.imaginary),
                                exact_number(mode.exponent.real),
                                exact_number(mode.exponent.imaginary),
                                exact_number(mode.probe_participation),
                                mode.is_unstable.to_string(),
                                mode.is_trivial.to_string(),
                                mode.subharmonic_order
                                    .map_or_else(|| "—".to_owned(), |order| order.to_string()),
                            ]
                        })
                        .collect(),
                },
            ])
        }
        AnalysisResultPayload::DcSweep { .. } | AnalysisResultPayload::OperatingPoint { .. } | AnalysisResultPayload::PoleZero { .. } | AnalysisResultPayload::Sensitivity { .. } | AnalysisResultPayload::SensitivityStudy { .. } | AnalysisResultPayload::DcMismatch { .. } | AnalysisResultPayload::ScalarMeasurements { .. } | AnalysisResultPayload::TransferFunction { .. } | AnalysisResultPayload::Soa { .. } | AnalysisResultPayload::TransientEvents { .. }
        // A recorded spectrum exports through the ordinary complex waveform
        // path; its payload states the transform, not a table of its own.
        | AnalysisResultPayload::FftSpectrum { .. } | AnalysisResultPayload::Qpac { .. } | AnalysisResultPayload::Qpxf { .. } | AnalysisResultPayload::Qpnoise { .. }
            | AnalysisResultPayload::Qpss { .. } | AnalysisResultPayload::Stb { .. } => None,
    }
}

fn periodic_metadata_row(field: &str, value: String, unit: &str) -> Vec<String> {
    vec![field.to_owned(), value, unit.to_owned()]
}

fn authenticated_floquet_count(
    count: usize,
    evidence: &rspice_results::floquet::FloquetSpectrumEvidence,
) -> String {
    if matches!(
        evidence,
        rspice_results::floquet::FloquetSpectrumEvidence::Qualified { .. }
            | rspice_results::floquet::FloquetSpectrumEvidence::NoDynamicModes
    ) {
        count.to_string()
    } else {
        "not authenticated".to_owned()
    }
}

fn floquet_spectrum_table_title(
    analysis: &str,
    count: usize,
    evidence: &rspice_results::floquet::FloquetSpectrumEvidence,
    noun: &str,
) -> String {
    if matches!(
        evidence,
        rspice_results::floquet::FloquetSpectrumEvidence::Qualified { .. }
            | rspice_results::floquet::FloquetSpectrumEvidence::NoDynamicModes
    ) {
        format!("Complete {analysis} Floquet spectrum · {count} {noun}")
    } else {
        format!("Retained {analysis} Floquet data · {count} {noun} · completeness unavailable")
    }
}

fn append_floquet_certificate_rows(
    rows: &mut Vec<Vec<String>>,
    evidence: &rspice_results::floquet::FloquetSpectrumEvidence,
) {
    if let Some(certificate) = evidence.certificate() {
        rows.extend([
            periodic_metadata_row(
                "Certificate problem order",
                certificate.problem_order.to_string(),
                "count",
            ),
            periodic_metadata_row(
                "Certificate maximum backward error",
                exact_number(certificate.max_backward_error),
                "",
            ),
            periodic_metadata_row(
                "Certificate qualification tolerance",
                exact_number(certificate.qualification_tolerance),
                "",
            ),
        ]);
    }
}

fn optional_exact_number(value: Option<f64>) -> String {
    value.map_or_else(|| "legacy unknown".to_owned(), exact_number)
}

fn optional_u64(value: Option<u64>) -> String {
    value.map_or_else(|| "legacy unknown".to_owned(), |value| value.to_string())
}

fn optional_bool(value: Option<bool>) -> String {
    value.map_or_else(|| "legacy unknown".to_owned(), |value| value.to_string())
}

const fn floquet_evidence_label(
    evidence: &rspice_results::floquet::FloquetSpectrumEvidence,
) -> &'static str {
    match evidence {
        rspice_results::floquet::FloquetSpectrumEvidence::NotComputed => "not computed",
        rspice_results::floquet::FloquetSpectrumEvidence::NoDynamicModes => "no dynamic modes",
        rspice_results::floquet::FloquetSpectrumEvidence::Qualified { .. } => "strictly qualified",
        rspice_results::floquet::FloquetSpectrumEvidence::LegacyUnknown => {
            "legacy evidence unknown"
        }
    }
}

const fn floquet_orbit_label(
    orbit: rspice_results::floquet::FloquetOrbitKindEvidence,
) -> &'static str {
    match orbit {
        rspice_results::floquet::FloquetOrbitKindEvidence::Driven => "driven",
        rspice_results::floquet::FloquetOrbitKindEvidence::Autonomous => "autonomous",
        rspice_results::floquet::FloquetOrbitKindEvidence::LegacyUnknown => "legacy unknown",
    }
}

const fn floquet_verdict_label(
    verdict: rspice_results::floquet::FloquetStabilityVerdictEvidence,
) -> &'static str {
    match verdict {
        rspice_results::floquet::FloquetStabilityVerdictEvidence::Stable => "stable",
        rspice_results::floquet::FloquetStabilityVerdictEvidence::Unstable => "unstable",
        rspice_results::floquet::FloquetStabilityVerdictEvidence::Marginal => "marginal",
        rspice_results::floquet::FloquetStabilityVerdictEvidence::Indeterminate => "indeterminate",
    }
}

const fn pstb_classification_label(
    classification: rspice_results::floquet::PstbStabilityClassificationEvidence,
) -> &'static str {
    use rspice_results::floquet::PstbStabilityClassificationEvidence as Classification;
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

fn sensitivity_number(value: rspice_results::sensitivity::SensitivityValue<f64>) -> String {
    match value {
        rspice_results::sensitivity::SensitivityValue::Available(value) => exact_number(value),
        rspice_results::sensitivity::SensitivityValue::Unavailable { unavailable } => {
            format!("Unavailable ({})", unavailable.as_str())
        }
    }
}

fn exact_number(value: f64) -> String {
    format!("{value:.17e}")
}

fn format_optional_scalar(
    scalar: Option<rspice_results::transfer_function::TransferFunctionScalarEvidence>,
) -> String {
    scalar.map_or_else(
        || "not requested".to_owned(),
        |value| match value {
            rspice_results::transfer_function::TransferFunctionScalarEvidence::Finite(value) => {
                exact_number(value)
            }
            rspice_results::transfer_function::TransferFunctionScalarEvidence::PositiveInfinity => {
                "+infinity".to_owned()
            }
            rspice_results::transfer_function::TransferFunctionScalarEvidence::NegativeInfinity => {
                "-infinity".to_owned()
            }
        },
    )
}

#[cfg(test)]
mod tests;
