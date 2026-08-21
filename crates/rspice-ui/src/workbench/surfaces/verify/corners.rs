//! Corners, optimization, and reliability: verification beyond the nominal run.
//!
//! Each of these panes reports evidence from runs other than the nominal one,
//! and each states which run it came from. A corner without a completed run,
//! an optimization without a converged result, or an SoA rule with no
//! evaluation is shown as missing evidence rather than folded into the
//! nominal verdict — the nominal case passing says nothing about the corners.

use super::*;

use crate::workbench::AppState;

pub(super) fn corners(ui: &mut Ui, app: &mut RSpiceApp) {
    let result = latest_analysis(app, crate::state::AnalysisType::Corner);
    let point_count = result
        .and_then(|analysis| analysis.waveforms.first())
        .map_or(0, |waveform| waveform.x.len());
    let signal_count = result.map_or(0, |analysis| analysis.waveforms.len());
    // `AnalysisResult::measurements` has no corner-point identity. It must not
    // be presented as a pointwise specification matrix until that provenance
    // is retained by the result schema.
    let configured_specifications = app.state.workspace.specs.len();
    let (elapsed, run_detail) = verification_run(app).map_or_else(
        || {
            (
                "No dataset".to_owned(),
                "No active immutable run".to_owned(),
            )
        },
        |run| {
            (
                format!("{:.3} s", run.elapsed_time),
                format!("Run {} · dataset {}", run.id, run.dataset_id),
            )
        },
    );
    let t = Tokens::get(ui.ctx());
    let items = [
        (
            "PVT points retained".to_owned(),
            point_count.to_string(),
            if point_count == 0 {
                "No process-corner evidence".to_owned()
            } else {
                run_detail
            },
            if point_count == 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "Signals retained".to_owned(),
            signal_count.to_string(),
            "source-attributed corner traces".to_owned(),
            if signal_count == 0 {
                t.color.warn
            } else {
                t.color.ok
            },
        ),
        (
            "Specifications configured".to_owned(),
            configured_specifications.to_string(),
            "No pointwise verdict payload retained".to_owned(),
            t.color.warn,
        ),
        (
            "Runtime".to_owned(),
            elapsed,
            "active immutable run".to_owned(),
            t.color.text,
        ),
    ];
    verification_kpi_strip(ui, &items);

    if let Some(result) = result {
        let axis = result
            .waveforms
            .first()
            .map(|waveform| waveform.x.as_slice())
            .unwrap_or(&[]);
        if axis.is_empty() {
            card(ui, "Corner result integrity", |ui| {
                ui.label("The retained corner result has no point axis. It is not presented as verification evidence.");
            });
        } else {
            let mut headers = vec![("Signal".to_owned(), 0.24)];
            let point_fraction = 0.64 / axis.len() as f32;
            let point_labels = corner_point_labels(app, result);
            headers.extend(axis.iter().enumerate().map(|(index, value)| {
                let label = point_labels
                    .as_ref()
                    .and_then(|labels| labels.get(index))
                    .cloned()
                    .unwrap_or_else(|| format!("P{:02} · {}", index + 1, format_scalar(*value)));
                (label, point_fraction)
            }));
            headers.push(("State".to_owned(), 0.12));
            let t = Tokens::get(ui.ctx());
            let nominal_index = app
                .state
                .workbench
                .verification
                .corner_compare_nominal
                .then(|| corner_nominal_index(app))
                .flatten();
            let rows = result
                .waveforms
                .iter()
                .map(|waveform| {
                    let aligned = waveform.x.as_slice() == axis && waveform.y.len() == axis.len();
                    let mut row = vec![TableCell::text(&waveform.name)];
                    if aligned {
                        let nominal = nominal_index
                            .and_then(|index| waveform.y.get(index))
                            .copied();
                        row.extend(waveform.y.iter().map(|value| {
                            TableCell::mono(nominal.map_or_else(
                                || format_scalar(*value),
                                |nominal| format!("{:+.6e}", *value - nominal),
                            ))
                        }));
                    } else {
                        row.extend((0..axis.len()).map(|_| TableCell::mono("—")));
                    }
                    row.push(TableCell::tone(
                        if aligned { "ALIGNED" } else { "INVALID" },
                        if aligned { t.color.ok } else { t.color.err },
                    ));
                    row
                })
                .collect::<Vec<_>>();
            table_section_header(
                ui,
                "Full PVT retained-value matrix",
                Some(if nominal_index.is_some() {
                    "exact delta from retained nominal point"
                } else {
                    "absolute source-attributed values"
                }),
                None,
            );
            render_data_table(
                ui,
                "verify-corner-matrix",
                &headers,
                &rows,
                Some((220.0 + axis.len() as f32 * 88.0).max(1_450.0)),
            );
            corner_evidence_details(ui, app, result, point_count);
            if let Some(run_id) = verification_run(app).map(|run| run.id) {
                executed_deck_section(ui, &mut app.state, run_id);
            }
        }
    } else {
        card(ui, "Corner evidence", |ui| {
            ui.label("No retained process-corner analysis is available for the active dataset. Run the configured corner plan to create evidence.");
        });
    }
}

/// What this run's models were, and the source one of its points solved.
///
/// Both halves are read out of the sealed decks the run itself executed rather
/// than re-derived from the project. A project that has moved on since the run
/// would answer a different question with exactly the same confidence, which
/// is the failure mode attribution exists to prevent: every phrase here is a
/// comment the engine was actually handed.
///
/// A run whose decks this session no longer holds states that. It does not
/// fall back to the working deck, which is a different document.
fn executed_deck_section(ui: &mut Ui, state: &mut AppState, run_id: u64) {
    table_section_header(
        ui,
        "Executed deck and model sources",
        Some("as this run's own sealed sources state them"),
        None,
    );
    let held: Option<(Vec<String>, Vec<String>)> =
        state.simulation.executed_decks.get(run_id).map(|deck| {
            (
                deck.points
                    .iter()
                    .map(|point| point.label.clone())
                    .collect(),
                deck.model_sources(),
            )
        });
    let Some((labels, sources)) = held else {
        property_row(
            ui,
            "Executed deck",
            "not retained — this session did not run it",
        );
        return;
    };
    if sources.is_empty() {
        property_row(
            ui,
            "Model sources",
            "this deck seals no model source of its own",
        );
    } else {
        for (index, source) in sources.iter().enumerate() {
            model_source_row(ui, state, index, source);
        }
    }
    let selected = state
        .workbench
        .verification
        .executed_deck_point
        .min(labels.len().saturating_sub(1));
    ui.horizontal(|ui| {
        egui::ComboBox::from_id_salt("verify-executed-deck-point")
            .selected_text(labels.get(selected).cloned().unwrap_or_default())
            .show_ui(ui, |ui| {
                for (index, label) in labels.iter().enumerate() {
                    ui.selectable_value(
                        &mut state.workbench.verification.executed_deck_point,
                        index,
                        label,
                    );
                }
            });
        if ui
            .button("Open executed deck")
            .on_hover_text(
                "Opens the exact source this point's engine read, as a read-only document \
                 sealed with the run.",
            )
            .clicked()
        {
            crate::workbench::documents::netlist_document::reveal_executed_deck(
                state, run_id, selected,
            );
        }
    });
}

/// One model source this run was given, and the route back to it.
///
/// A source sealed under a pack release is the one kind this workspace can
/// route to, because the release is a thing the Model Hub ledger lists. The
/// pack identity is read back out of the label the deck carries rather than
/// looked up in the project, so the row cannot offer a route to a release the
/// run did not actually use.
fn model_source_row(ui: &mut Ui, state: &mut AppState, index: usize, source: &str) {
    let label = format!("Model source {}", index + 1);
    let Some((pack_id, _)) = crate::state::model_library::labelled_pack(source) else {
        property_row(ui, &label, source);
        return;
    };
    let pack_id = pack_id.to_owned();
    ui.horizontal(|ui| {
        property_row(ui, &label, source);
        if ui
            .button("Open in Models")
            .on_hover_text("Selects this release in the Model Hub ledger.")
            .clicked()
        {
            state.workbench.models_view.catalog_scope =
                crate::workbench::state::ModelsCatalogScope::InstalledPacks;
            state.workbench.models_view.selected_pack = Some(pack_id.clone());
            state.workbench.models_page = crate::workbench::state::ModelsPage::Models;
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Models);
        }
    });
}

pub(super) fn export_active_corner_matrix(app: &mut RSpiceApp) {
    let result_id =
        latest_analysis(app, crate::state::AnalysisType::Corner).map(|result| result.id);
    let analysis_index = result_id.and_then(|result_id| {
        app.state.simulation.active_run().and_then(|run| {
            run.analyses.iter().position(|analysis| {
                analysis.id == result_id
                    && analysis.analysis_type == crate::state::AnalysisType::Corner
                    && verified_analysis(analysis)
            })
        })
    });
    if let Some(index) = analysis_index {
        app.state.simulation.select_analysis(index);
        Command::ExportWaveformsCsv.execute(app);
        app.state.workbench.verification.action_receipt =
            "The retained process-corner matrix was selected for CSV export.".to_owned();
    } else {
        app.state.workbench.verification.action_receipt =
            "Corner export blocked: no source-attributed successful corner result is active."
                .to_owned();
    }
}

pub(super) fn corner_point_labels(
    app: &RSpiceApp,
    result: &crate::state::AnalysisResult,
) -> Option<Vec<String>> {
    use crate::simulation::plan::AnalysisDraft;

    let provenance = result.provenance.as_ref()?;
    let plan = app.state.sim_setup.stable_analysis_plan().ok()?;
    let instance = plan
        .instances()
        .iter()
        .find(|instance| instance.id() == provenance.authored_source_instance_id())?;
    if instance.modified_revision() != provenance.source_revision() {
        return None;
    }
    let AnalysisDraft::Corner(state) = instance.draft() else {
        return None;
    };
    let labels = state
        .to_config(
            &app.state.sim_setup.run_set,
            app.state.sim_setup.reference_pvt,
        )
        .ok()?
        .corner_names();
    let points = result.waveforms.first()?.x.len();
    (labels.len() == points).then_some(labels)
}

pub(super) fn corner_nominal_index(app: &RSpiceApp) -> Option<usize> {
    let result = latest_analysis(app, crate::state::AnalysisType::Corner)?;
    let labels = corner_point_labels(app, result)?;
    let exact_nominal = format!(
        "TT_1.00V_{:.0}C",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    labels.iter().position(|label| label == &exact_nominal)
}

pub(super) fn corner_evidence_details(
    ui: &mut Ui,
    app: &RSpiceApp,
    result: &crate::state::AnalysisResult,
    point_count: usize,
) {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_width();
    let split = available > 1_020.0;
    let render_worst = |ui: &mut Ui| {
        table_section_header(ui, "Worst PVT points", None, None);
        let headers = vec![
            ("Rank".to_owned(), 0.16),
            ("Point".to_owned(), 0.28),
            ("Limiting spec".to_owned(), 0.36),
            ("Margin".to_owned(), 0.20),
        ];
        let rows = vec![vec![
            TableCell::mono("—"),
            TableCell::text("No pointwise evidence"),
            TableCell::text("Point-attributed measurements not retained"),
            TableCell::tone("NO VERDICT", t.color.warn),
        ]];
        render_data_table(ui, "verify-corner-worst-points", &headers, &rows, None);
    };
    let render_reproducibility = |ui: &mut Ui| {
        table_section_header(ui, "Run reproducibility", None, None);
        let run = verification_run(app);
        property_row(
            ui,
            "Dataset",
            &run.map_or_else(
                || "unavailable".to_owned(),
                |run| run.dataset_id.to_string(),
            ),
        );
        property_row(
            ui,
            "Run provenance",
            if run.and_then(|run| run.prepared_receipt()).is_some() {
                "prepared snapshot retained"
            } else {
                "prepared snapshot unavailable"
            },
        );
        property_row(
            ui,
            "Analysis provenance",
            if result.provenance.is_some() {
                "source identity retained"
            } else {
                "legacy / unattributed"
            },
        );
        property_row(ui, "PVT points", &point_count.to_string());
    };
    ui.add_space(1.0);
    if split {
        let left = (available - 1.0) * 0.45;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            ui.allocate_ui_with_layout(
                egui::vec2(left, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                render_worst,
            );
            ui.allocate_ui_with_layout(
                egui::vec2(available - left - 1.0, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                render_reproducibility,
            );
        });
    } else {
        render_worst(ui);
        ui.add_space(1.0);
        render_reproducibility(ui);
    }
}

pub(super) fn optimization(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Optimization execution", |ui| {
        ui.horizontal_wrapped(|ui| {
            if Button::new("Run optimization").accent().show(ui).clicked() {
                let result = request_analysis_run(
                    app,
                    &[crate::simulation::plan::AnalysisKind::Optimization],
                );
                record_verification_action(
                    app,
                    result,
                    "Optimization was dispatched through the active production analysis plan.",
                );
            }
            if Button::new("Configure optimization").show(ui).clicked() {
                let result = open_analysis_configuration(
                    app,
                    crate::simulation::plan::AnalysisKind::Optimization,
                );
                record_verification_action(
                    app,
                    result,
                    "The typed optimization configuration was opened.",
                );
            }
        });
        if let Some(result) = latest_analysis(app, crate::state::AnalysisType::Optimization) {
            let iterations = result
                .waveforms
                .first()
                .map_or(0, |waveform| waveform.x.len());
            property_row(ui, "Retained result", &format!("{iterations} iterations"));
            property_row(ui, "Dataset", "immutable active result");
        } else {
            execution_card_note(
                ui,
                "No optimization result is retained. Configure a bounded variable set and execute the production optimization analysis to create evidence.",
            );
        }
    });
    if let Some(result) = latest_analysis(app, crate::state::AnalysisType::Optimization) {
        let t = Tokens::get(ui.ctx());
        let headers = vec![
            ("Trace".to_owned(), 0.38),
            ("Samples".to_owned(), 0.18),
            ("Final value".to_owned(), 0.26),
            ("State".to_owned(), 0.18),
        ];
        let rows = result
            .waveforms
            .iter()
            .map(|waveform| {
                vec![
                    TableCell::text(&waveform.name),
                    TableCell::mono(waveform.y.len().to_string()),
                    TableCell::mono(
                        waveform
                            .y
                            .last()
                            .map_or_else(|| "—".to_owned(), |value| format_scalar(*value)),
                    ),
                    TableCell::tone(
                        if result.success { "RETAINED" } else { "FAILED" },
                        if result.success {
                            t.color.ok
                        } else {
                            t.color.err
                        },
                    ),
                ]
            })
            .collect::<Vec<_>>();
        table_section_header(
            ui,
            "Candidate traces",
            Some("source-owned result evidence"),
            None,
        );
        render_data_table(ui, "verify-optimization-results", &headers, &rows, None);
    }
    action_receipt(ui, app);
}

pub(super) fn reliability(ui: &mut Ui, app: &mut RSpiceApp) {
    card(ui, "Reliability execution", |ui| {
        ui.horizontal_wrapped(|ui| {
            if Button::new("Run preview plan").accent().show(ui).clicked() {
                let result = request_analysis_run(
                    app,
                    &[
                        crate::simulation::plan::AnalysisKind::Soa,
                        crate::simulation::plan::AnalysisKind::Reliability,
                    ],
                );
                record_verification_action(
                    app,
                    result,
                    "Electrical SOA and reliability preview analyses were dispatched through the active plan.",
                );
            }
            if Button::new("Edit mission profile").show(ui).clicked() {
                let result = open_analysis_configuration(
                    app,
                    crate::simulation::plan::AnalysisKind::Reliability,
                );
                record_verification_action(
                    app,
                    result,
                    "The typed reliability mission-profile configuration was opened.",
                );
            }
        });
        execution_card_note(
            ui,
            "Electrical reliability evaluates operating stress and mission aging. Physical geometry checks remain owned by the Physical DRC flow.",
        );
    });

    let soa_payload = latest_analysis(app, crate::state::AnalysisType::Soa)
        .filter(|analysis| analysis.validate_retained_evidence().is_ok())
        .and_then(|analysis| match &analysis.result_payload {
            Some(crate::state::AnalysisResultPayload::Soa {
                evaluations,
                violations,
            }) => Some((evaluations.as_slice(), violations.as_slice())),
            _ => None,
        });
    let t = Tokens::get(ui.ctx());
    let soa_headers = vec![
        ("Rule".to_owned(), 0.27),
        ("Device / net".to_owned(), 0.16),
        ("Observed".to_owned(), 0.15),
        ("Limit".to_owned(), 0.14),
        ("Margin".to_owned(), 0.13),
        ("Cross-probe".to_owned(), 0.15),
    ];
    let mut cross_probe_device = None;
    if let Some((evaluations, _violations)) = soa_payload {
        let pass_count = evaluations
            .iter()
            .filter(|evaluation| evaluation.verdict == crate::state::SoaRuleVerdictEvidence::Pass)
            .count();
        let warning_count = evaluations
            .iter()
            .filter(|evaluation| {
                evaluation.verdict == crate::state::SoaRuleVerdictEvidence::Warning
            })
            .count();
        let blocking_count = evaluations
            .len()
            .saturating_sub(pass_count)
            .saturating_sub(warning_count);
        let status = if blocking_count > 0 {
            format!("{blocking_count} blocking · {warning_count} warning")
        } else if warning_count > 0 {
            format!("{pass_count} pass · {warning_count} near limit")
        } else {
            format!("{pass_count} / {} pass", evaluations.len())
        };
        table_section_header(ui, "Electrical-stress rule results", Some(&status), None);
        let clicked_rule = render_virtual_data_table(
            ui,
            "verify-soa-rule-results",
            &soa_headers,
            evaluations.len(),
            true,
            |index| {
                let evaluation = &evaluations[index];
                let margin = (evaluation.limit_value - evaluation.worst_actual_value)
                    / evaluation.limit_value.abs();
                let (_, color) = soa_rule_verdict_display(evaluation.verdict, &t);
                vec![
                    TableCell::mono(format!(
                        "{} · {}",
                        soa_parameter_display(evaluation.parameter),
                        evaluation.description
                    )),
                    TableCell::mono(&evaluation.device_id),
                    TableCell::mono(format_value(
                        evaluation.worst_actual_value,
                        &evaluation.unit,
                    )),
                    TableCell::mono(format_value(evaluation.limit_value, &evaluation.unit)),
                    TableCell::tone(format!("{:+.2}%", margin * 100.0), color),
                    TableCell::tone(format!("Open {}", evaluation.device_id), t.color.accent),
                ]
            },
        );
        cross_probe_device = clicked_rule.map(|index| evaluations[index].device_id.clone());
    } else {
        table_section_header(
            ui,
            "Electrical-stress rule results",
            Some("no validated active-dataset evidence"),
            None,
        );
        let _ = render_virtual_data_table(
            ui,
            "verify-soa-rule-results",
            &soa_headers,
            0,
            false,
            |_| Vec::new(),
        );
    }
    if let Some(device_id) = cross_probe_device {
        let result = cross_probe_reliability_device(app, &device_id);
        record_verification_action(
            app,
            result,
            &format!("Cross-probed SOA evidence to schematic device '{device_id}'."),
        );
    }

    let reliability_payload = latest_analysis(app, crate::state::AnalysisType::Reliability)
        .filter(|analysis| analysis.validate_retained_evidence().is_ok())
        .and_then(
            |analysis| match (&analysis.family_metadata, &analysis.result_payload) {
                (
                    Some(crate::state::AnalysisResultFamilyMetadata::Reliability { years }),
                    Some(crate::state::AnalysisResultPayload::Reliability { devices }),
                ) => Some((years.as_slice(), devices.as_slice())),
                _ => None,
            },
        );
    if let Some((years, devices)) = reliability_payload {
        let mut headers = vec![
            ("Device / metric".to_owned(), 0.24),
            ("Run stress".to_owned(), 0.18),
        ];
        let checkpoint_width = 0.58 / years.len() as f32;
        headers.extend(
            years
                .iter()
                .map(|years| (format!("{} years", format_scalar(*years)), checkpoint_width)),
        );
        table_section_header(
            ui,
            "Aging projection",
            Some("engineering preview · not sign-off eligible"),
            None,
        );
        let _ = render_virtual_data_table(
            ui,
            "verify-reliability-device-results",
            &headers,
            devices.len().saturating_mul(7),
            false,
            |row_index| reliability_projection_row(&devices[row_index / 7], row_index % 7),
        );
    } else {
        let headers = vec![
            ("Device / metric".to_owned(), 0.45),
            ("Run stress".to_owned(), 0.25),
            ("Lifetime projection".to_owned(), 0.30),
        ];
        table_section_header(
            ui,
            "Aging projection",
            Some("no validated active-dataset evidence"),
            None,
        );
        let _ = render_virtual_data_table(
            ui,
            "verify-reliability-device-results",
            &headers,
            0,
            false,
            |_| Vec::new(),
        );
    }
    action_receipt(ui, app);
}

pub(super) fn execution_card_note(ui: &mut Ui, note: &str) {
    ui.add_space(8.0);
    ui.add(egui::Label::new(note).wrap());
}

pub(super) fn reliability_projection_row(
    device: &crate::state::ReliabilityDeviceEvidence,
    metric: usize,
) -> Vec<TableCell> {
    let dash = || TableCell::mono("\u{2014}");
    let mut row = match metric {
        0 => vec![
            TableCell::mono(format!("{} · VGS stress", device.device_id)),
            TableCell::mono(format_value(device.stress.average_gate_stress_v, "V")),
        ],
        1 => vec![
            TableCell::mono(format!("{} · VDS stress", device.device_id)),
            TableCell::mono(format_value(device.stress.average_drain_stress_v, "V")),
        ],
        2 => vec![
            TableCell::mono(format!("{} · temperature", device.device_id)),
            TableCell::mono(format_value(device.stress.average_temperature_k, "K")),
        ],
        3 => vec![
            TableCell::mono(format!("{} · duration", device.device_id)),
            TableCell::mono(format_value(device.stress.duration_s, "s")),
        ],
        4 => vec![
            TableCell::mono(format!("{} · \u{0394}Vth", device.device_id)),
            dash(),
        ],
        5 => vec![
            TableCell::mono(format!("{} · \u{0394} mobility", device.device_id)),
            dash(),
        ],
        _ => vec![
            TableCell::mono(format!("{} · \u{0394}Rds", device.device_id)),
            dash(),
        ],
    };
    for checkpoint in &device.checkpoints {
        row.push(match metric {
            0..=3 => dash(),
            4 => TableCell::mono(format_value(
                checkpoint.shift.threshold_voltage_shift_v,
                "V",
            )),
            5 => TableCell::mono(format!("{:+.4}%", checkpoint.shift.mobility_shift * 100.0)),
            _ => TableCell::mono(format!(
                "{:+.4}%",
                checkpoint.shift.drain_source_resistance_shift * 100.0
            )),
        });
    }
    row
}

pub(super) fn cross_probe_reliability_device(
    app: &mut RSpiceApp,
    device_id: &str,
) -> Result<(), String> {
    let component_id = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.name.eq_ignore_ascii_case(device_id))
        .map(|component| component.id)
        .ok_or_else(|| {
            format!("SOA device '{device_id}' is not present in the active schematic revision")
        })?;
    app.state
        .schematic
        .selection
        .select_only_component(component_id);
    app.state
        .workbench
        .activate(super::super::super::state::Workspace::Design);
    Ok(())
}

pub(super) fn soa_parameter_display(parameter: crate::state::SoaParameterEvidence) -> &'static str {
    use crate::state::SoaParameterEvidence;
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => "VGS max",
        SoaParameterEvidence::DrainSourceVoltage => "VDS max",
        SoaParameterEvidence::GateDrainVoltage => "VGD max",
        SoaParameterEvidence::BaseEmitterVoltage => "VBE max",
        SoaParameterEvidence::CollectorEmitterVoltage => "VCE max",
        SoaParameterEvidence::BaseCollectorVoltage => "VBC max",
        SoaParameterEvidence::DrainCurrent => "ID max",
        SoaParameterEvidence::CollectorCurrent => "IC max",
        SoaParameterEvidence::PowerDissipation => "Power max",
        SoaParameterEvidence::Temperature => "Temperature max",
    }
}

pub(super) fn soa_rule_verdict_display(
    verdict: crate::state::SoaRuleVerdictEvidence,
    tokens: &Tokens,
) -> (&'static str, egui::Color32) {
    use crate::state::SoaRuleVerdictEvidence;
    match verdict {
        SoaRuleVerdictEvidence::Pass => ("PASS", tokens.color.ok),
        SoaRuleVerdictEvidence::Warning => ("WARNING", tokens.color.warn),
        SoaRuleVerdictEvidence::Violation => ("VIOLATION", tokens.color.err),
        SoaRuleVerdictEvidence::Critical => ("CRITICAL", tokens.color.err),
    }
}
