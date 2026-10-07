//! Publish the shared control host's completed runs through core documents.

use super::*;
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind,
};
use rspice_core::execution::control::{
    ControlError, ControlErrorKind, ControlLimits, ControlProgram,
};
use rspice_core::execution::topology_fingerprint_with_abort;
use rspice_core::netlist::ControlScriptSource;

pub(super) fn run(
    netlist: Netlist,
    options: &WasmExecutionOptions,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<DeckExecution> {
    let limits = options.resource_limits.to_core();
    let initial_plan = DeckPlan::from_netlist_with_abort(&netlist, &limits, abort)
        .map_err(deck_plan_wasm_error)?;
    if !initial_plan.axes().is_empty()
        || !netlist.fft_analyses.is_empty()
        || netlist
            .analyses
            .iter()
            .any(|command| matches!(command, AnalysisCommand::Four { .. }))
        || netlist.options.restart.is_some()
        || options.transient_compression.is_some()
    {
        return Err(unsupported_deck_analysis(
            "control-script execution with declarative run axes, Fourier/FFT, restart or compression is not yet implemented".into(),
        ));
    }
    let script = netlist
        .control_script
        .clone()
        .ok_or_else(|| document_error("control execution requires retained source".into()))?;
    let program = ControlProgram::parse_deck_with_abort(
        script.text(),
        ControlLimits {
            max_source_bytes: limits.max_expanded_source_bytes,
            max_source_lines: limits.max_netlist_lines,
            max_loop_values: limits.max_batch_runs,
            ..ControlLimits::default()
        },
        abort,
    )
    .map_err(|error| command_error(error, &script))?;
    let engine = engine_with_resource_limits(limits)?;
    let mut session = program.start(netlist.params.clone());
    let mut circuit =
        ControlCircuit::new(netlist).map_err(|error| execution_error(error, &script))?;
    let mut presentations = Vec::new();
    let mut presentation_values = 0usize;
    let mut topologies = Vec::new();

    while let Some(command) = session
        .next_command(&mut circuit, abort)
        .map_err(|error| command_error(error, &script))?
    {
        // The core accounts for all retained raw datasets. Charge previously
        // retained presentation samples as well before the next allocation.
        let mut config = engine.config().clone();
        config.resource_limits.max_result_values =
            limits.max_result_values.saturating_sub(presentation_values);
        let bounded = Engine::try_new(config).map_err(|error| {
            simulation_error(rspice_core::SimulationError::Configuration(error))
        })?;
        let previous = circuit.datasets().len();
        match circuit
            .execute(&bounded, &command, session.variables(), abort)
            .map_err(|error| execution_error(error, &script))?
        {
            ControlCommandEffect::CircuitChanged => {}
            ControlCommandEffect::Analyses(_) => {
                let topology = topology_fingerprint_with_abort(&bounded, circuit.netlist(), abort)
                    .map_err(simulation_error)?;
                for _ in previous..circuit.datasets().len() {
                    topologies.push(topology);
                }
            }
            ControlCommandEffect::Presentation(presentation) => {
                // Even metadata-only scripts cannot retain unbounded output.
                let requested = presentations.len().saturating_add(1);
                if requested > limits.max_batch_runs {
                    return Err(resource_limit_error(
                        ResourceKind::BatchRuns,
                        requested,
                        limits.max_batch_runs,
                    ));
                }
                let traces = match &presentation.kind {
                    ControlPresentationKind::Plot { traces, .. }
                    | ControlPresentationKind::Print(traces) => traces.as_slice(),
                    ControlPresentationKind::UnitsChanged { .. } => &[],
                };
                let count = traces.iter().fold(
                    presentation.scalars.len().saturating_mul(2),
                    |count, trace| {
                        count.saturating_add(
                            trace
                                .x
                                .samples
                                .len()
                                .saturating_add(trace.y.samples.len())
                                .saturating_mul(2),
                        )
                    },
                );
                presentation_values = presentation_values.saturating_add(count);
                if presentation_values > limits.max_result_values {
                    return Err(resource_limit_error(
                        ResourceKind::ResultValues,
                        presentation_values,
                        limits.max_result_values,
                    ));
                }
                presentations.push(presentation);
            }
        }
    }
    ensure_not_aborted(abort)?;
    if circuit.datasets().is_empty() {
        return Err(unsupported_deck_analysis(
            "control script completed without an analysis result".into(),
        ));
    }

    // The ordered script determines its analysis list at runtime. Describe
    // that realized list; do not publish the declarative implicit-OP placeholder.
    let mut realized = circuit.netlist().clone();
    realized.analyses = circuit
        .datasets()
        .iter()
        .map(|dataset| dataset.command.clone())
        .collect();
    realized.control_script = None;
    let plan = DeckPlan::from_netlist_with_abort(&realized, &limits, abort)
        .map_err(deck_plan_wasm_error)?;
    let coordinates = plan
        .coordinates_with_abort(&limits, abort)
        .map_err(deck_plan_wasm_error)?;
    let coordinate = coordinates
        .first()
        .ok_or_else(|| document_error("control run has no coordinate".into()))?;
    let result_coordinate = ResultCoordinate::from_run_coordinate(coordinate);
    let mut results = Vec::new();
    let mut dataset_names = Vec::new();
    let mut retained = presentation_values;
    for (dataset, topology) in circuit.into_datasets().into_iter().zip(topologies) {
        ensure_not_aborted(abort)?;
        let mut remaining = limits;
        remaining.max_result_values = limits.max_result_values.saturating_sub(retained);
        let projection_error = |error| {
            let error = match error {
                rspice_core::execution::ResultDocumentError::ResourceLimit(mut error)
                    if error.resource == ResourceKind::ResultValues =>
                {
                    error.requested = error.requested.saturating_add(retained);
                    error.limit = limits.max_result_values;
                    rspice_core::execution::ResultDocumentError::ResourceLimit(error)
                }
                other => other,
            };
            document_projection_error(error)
        };
        let builder = match &dataset.result {
            ControlAnalysisResult::OperatingPoint(result) => {
                AnalysisResultDocument::from_operating_point(
                    dataset.analysis_id,
                    result,
                    dataset.device_op_report.as_deref(),
                )
            }
            ControlAnalysisResult::DcSweep(result) => {
                AnalysisResultDocument::from_dc_analysis(dataset.analysis_id, result)
            }
            ControlAnalysisResult::TransferFunction(result) => {
                AnalysisResultDocument::from_transfer_function(dataset.analysis_id, result)
            }
            ControlAnalysisResult::PoleZero(result) => {
                AnalysisResultDocument::from_pole_zero(dataset.analysis_id, result)
            }
            ControlAnalysisResult::Noise(points) => {
                AnalysisResultDocument::from_noise(dataset.analysis_id, points)
            }
            ControlAnalysisResult::Ac(points) => {
                AnalysisResultDocument::from_ac(dataset.analysis_id, points)
            }
            ControlAnalysisResult::AcTable(table) => {
                AnalysisResultDocument::from_ac_table_with_limits_and_abort(
                    dataset.analysis_id,
                    table,
                    &remaining,
                    abort,
                )
            }
            ControlAnalysisResult::NoiseTable(table) => {
                AnalysisResultDocument::from_noise_table_with_limits_and_abort(
                    dataset.analysis_id,
                    table,
                    &remaining,
                    abort,
                )
            }
            ControlAnalysisResult::Transient(result) => AnalysisResultDocument::from_transient(
                dataset.analysis_id,
                result,
                None,
                Vec::new(),
            ),
        }
        .map_err(projection_error)?;
        let namespace = format!("control/{}", dataset.analysis_id);
        let document = builder
            .coordinate(result_coordinate.clone())
            .topology_fingerprint(topology)
            .namespaces(ResultNamespaces {
                output: namespace.clone(),
                checkpoint: namespace,
            })
            .build_with_limits_and_abort(&remaining, abort)
            .map_err(projection_error)?;
        retained = retained.saturating_add(document.total_value_count());
        if retained > limits.max_result_values {
            return Err(resource_limit_error(
                ResourceKind::ResultValues,
                retained,
                limits.max_result_values,
            ));
        }
        dataset_names.push(dataset.name);
        results.push(document);
    }
    Ok(DeckExecution {
        plan,
        coordinates,
        results,
        control_presentations: presentations,
        control_datasets: dataset_names,
    })
}

fn command_error(error: ControlError, script: &ControlScriptSource) -> Box<WasmError> {
    let (code, category) = match error.kind {
        ControlErrorKind::Syntax => ("control.syntax", "netlist"),
        ControlErrorKind::Expression => ("control.expression", "netlist"),
        ControlErrorKind::Host => ("control.command", "netlist"),
        ControlErrorKind::ResourceLimit => ("control.resource_limit", "resource_limit"),
        ControlErrorKind::Aborted => ("control.aborted", "cancellation"),
    };
    source_error(
        WasmError::new(error.to_string(), code, category),
        error.line,
        script,
    )
}

fn execution_error(error: ControlExecutionError, script: &ControlScriptSource) -> Box<WasmError> {
    match error {
        ControlExecutionError::Command(error) => command_error(error, script),
        ControlExecutionError::Simulation { line, source } => {
            source_error(WasmError::from_simulation_error(source), line, script)
        }
        ControlExecutionError::Configuration { line, source } => source_error(
            WasmError::from_simulation_error(rspice_core::SimulationError::Configuration(source)),
            line,
            script,
        ),
    }
}

fn source_error(mut error: WasmError, line: usize, script: &ControlScriptSource) -> Box<WasmError> {
    if let Some(origin) = script.origin(line) {
        error.primary_source = origin
            .path
            .as_ref()
            .map(|source| source.display().to_string());
        error.primary_line = Some(origin.line);
    } else {
        error.primary_line = Some(line);
    }
    Box::new(error)
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn pole_zero_control_matches_direct_documents_and_retains_root_presentations() {
        let source = "PZ\nV1 in 0 0\nR1 in mid 1\nL1 mid out 1\nC1 out 0 1\n";
        let direct =
            run_authored_deck_document_detailed(&format!("{source}.pz in 0 out 0 vol pz\n.end\n"))
                .unwrap();
        for cards in [
            ".control\npz in 0 out 0 vol pz\nlet saved = pz1.pole(1)\nprint pole(1) saved\n.endc",
            ".pz in 0 out 0 vol pz\n.control\nrun\nprint pole(1)\n.endc",
        ] {
            let control =
                run_authored_deck_document_detailed(&format!("{source}{cards}\n.end\n")).unwrap();
            assert_eq!(control.control_datasets, ["pz1"]);
            assert_eq!(control.results[0].payload(), direct.results[0].payload());
            let rspice_core::engine::ControlPresentationKind::Print(traces) =
                &control.control_presentations[0].kind
            else {
                panic!("print")
            };
            assert_eq!(traces[0].y.unit, SignalUnit::RadianPerSecond);
            assert!((traces[0].y.samples[0].im - 3.0_f64.sqrt() / 2.0).abs() < 1e-9);
            let invalid = format!(
                "{source}{}\n.end\n",
                cards.replace("print pole(1)", "print pole(9)")
            );
            assert!(run_authored_deck_document_detailed(&invalid).is_err());
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn transfer_function_control_matches_direct_and_mixed_declarative_results() {
        let source = "Transfer\nI1 0 out 0 AC 1\nR1 out 0 3k\n";
        let direct =
            run_authored_deck_document_detailed(&format!("{source}.tf V(out) I1\n.end\n")).unwrap();
        for cards in [
            ".control\ntf V(out) I1\n.endc",
            ".tf V(out) I1\n.control\nrun\n.endc",
        ] {
            let control =
                run_authored_deck_document_detailed(&format!("{source}{cards}\n.end\n")).unwrap();
            assert_eq!(control.control_datasets, ["tf1"]);
            assert_eq!(control.results[0].scalars(), direct.results[0].scalars());
            assert_eq!(control.results[0].payload(), direct.results[0].payload());
            assert_eq!(
                control.results[0].scalars()[0].unit(),
                Some(&SignalUnit::Ohm)
            );
        }
        let mixed = run_authored_deck_document_detailed(&format!("{source}.op\n.tf V(out) I1\n.ac lin 2 10 100\n.control\nrun\nprint tf1.transfer_function\n.endc\n.end\n")).unwrap();
        assert_eq!(mixed.control_datasets, ["op1", "tf1", "ac1"]);
        assert_eq!(mixed.results[1].scalars(), direct.results[0].scalars());
        let rspice_core::engine::ControlPresentationKind::Print(traces) =
            &mixed.control_presentations[0].kind
        else {
            panic!("print");
        };
        assert_eq!(traces[0].y.unit, SignalUnit::Ohm);
        assert!((traces[0].y.samples[0].re - 3000.0).abs() < 1e-7);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn transfer_scalar_assignments_drive_subsequent_analyses() {
        let deck = run_authored_deck_document_detailed("TF\nI1 0 out 0\nR1 out 0 3k\n.control\ntf V(out) I1\nlet resistance = tf1.transfer_function*2\nif resistance > 5000\nalter R1 $resistance\ntf V(out) I1\nend\nprint tf1.transfer_function tf2.transfer_function\n.endc\n.end\n").unwrap();
        assert_eq!(deck.control_datasets, ["tf1", "tf2"]);
        let rspice_core::engine::ControlPresentationKind::Print(traces) =
            &deck.control_presentations[0].kind
        else {
            panic!("print");
        };
        assert_eq!(traces.len(), 2);
        for (trace, expected) in traces.iter().zip([3000.0, 6000.0]) {
            assert!((trace.y.samples[0].re - expected).abs() < 1e-7);
            assert_eq!(trace.y.unit, SignalUnit::Ohm);
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn transfer_function_cancellation_and_failed_probes_publish_no_deck() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct CancelAfter {
            polls: AtomicUsize,
            limit: usize,
        }
        impl AbortSignal for CancelAfter {
            fn is_aborted(&self) -> bool {
                self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
            }
        }
        let source = "Transfer\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.control\nop\ntf V(out) V1\nprint transfer_function\n.endc\n.end\n";
        let options = WasmExecutionOptions::default();
        let counted = CancelAfter {
            polls: AtomicUsize::new(0),
            limit: usize::MAX,
        };
        run_authored_deck_document_with_options_and_abort_detailed(source, &options, &counted)
            .unwrap();
        let polls = counted.polls.load(Ordering::Relaxed);
        for limit in [0, polls / 2, polls - 1] {
            let abort = CancelAfter {
                polls: AtomicUsize::new(0),
                limit,
            };
            let error = run_authored_deck_document_with_options_and_abort_detailed(
                source, &options, &abort,
            )
            .unwrap_err();
            assert_eq!(error.category, "cancellation");
        }
        let error = run_authored_deck_document_detailed(
            &source.replace("tf V(out) V1", "tf V(missing) V1"),
        )
        .unwrap_err();
        assert_eq!(error.primary_line, Some(7));
        let mut limited = options;
        limited.resource_limits.max_result_values = 5;
        let error =
            run_authored_deck_document_with_options_and_abort_detailed(source, &limited, &NoAbort)
                .unwrap_err();
        assert_eq!(error.category, "resource_limit");
    }

    const TABLE_CANCELLATION_SOURCE: &str = "WASM table\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {resistance} tc1=.01 tnom=27\nC1 out 0 1u\n.data points HERTZ resistance TEMP\n100 1k 27\n10 2k 127\n100 3k 77\n.enddata\n";

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn table_failure_and_cancellation_publish_no_partial_deck() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct CancelAfter {
            calls: AtomicUsize,
            limit: usize,
        }
        impl AbortSignal for CancelAfter {
            fn is_aborted(&self) -> bool {
                self.calls.fetch_add(1, Ordering::Relaxed) >= self.limit
            }
        }
        let source = format!(
            "{TABLE_CANCELLATION_SOURCE}.control\nop\nac data=points\nnoise V(out) V1 data=points\n.endc\n.end\n"
        );
        let options = WasmExecutionOptions::default();
        let counted = CancelAfter {
            calls: AtomicUsize::new(0),
            limit: usize::MAX,
        };
        run_authored_deck_document_with_options_and_abort_detailed(&source, &options, &counted)
            .unwrap();
        let polls = counted.calls.load(Ordering::Relaxed);
        for limit in [0, polls / 2, polls - 1] {
            let abort = CancelAfter {
                calls: AtomicUsize::new(0),
                limit,
            };
            let error = run_authored_deck_document_with_options_and_abort_detailed(
                &source, &options, &abort,
            )
            .unwrap_err();
            assert_eq!(error.category, "cancellation");
        }
        let mut limited = options;
        limited.resource_limits.max_result_values = 20;
        let error =
            run_authored_deck_document_with_options_and_abort_detailed(&source, &limited, &NoAbort)
                .unwrap_err();
        assert_eq!(error.category, "resource_limit");
        let invalid = source.replace("100 3k 77", "-100 3k 77");
        assert!(run_authored_deck_document_detailed(&invalid).is_err());
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn control_frequency_tables_match_direct_documents_and_keep_coordinates() {
        let source = "Tables\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {resistance}\nC1 out 0 1u\n.data points FREQ resistance\n100 1k\n10 2k\n100 3k\n.enddata\n";
        for command in ["ac data=points", "noise V(out) V1 data=points"] {
            let direct =
                run_authored_deck_document_detailed(&format!("{source}.{command}\n.end\n"))
                    .unwrap();
            assert!(direct.results[0].frequency_table().is_some());
            assert_eq!(direct.results[0].axes().len(), 2);
            let metadata = crate::document::result_metadata(&direct.results[0], 1024).unwrap();
            let wire = serde_json::to_value(metadata).unwrap();
            assert_eq!(wire["frequencyTable"]["requestedRows"], 3);
            assert_eq!(
                wire["frequencyTable"]["columns"][1]["axis"],
                "data(resistance)"
            );
            assert_eq!(
                wire["frequencyTable"]["columns"][1]["target"]["target"],
                "RESISTANCE"
            );
            for cards in [
                format!(".control\n{command}\n.endc"),
                format!(".{command}\n.control\nrun\n.endc"),
            ] {
                let control =
                    run_authored_deck_document_detailed(&format!("{source}{cards}\n.end\n"))
                        .unwrap();
                assert_eq!(control.results[0].axes(), direct.results[0].axes());
                assert_eq!(control.results[0].signals(), direct.results[0].signals());
                assert_eq!(
                    control.results[0].frequency_table(),
                    direct.results[0].frequency_table()
                );
            }
        }
    }

    #[test]
    fn control_noise_document_matches_direct_current_referred_noise() {
        let source = "Noise\nI1 0 out DC 0 AC 1\nR1 out 0 1k\n";
        let direct = run_authored_deck_document_detailed(&format!(
            "{source}.noise V(out) I1 lin 3 10 100\n.end\n"
        ))
        .unwrap();
        let control = run_authored_deck_document_detailed(&format!(
            "{source}.control\nnoise V(out) I1 lin 3 10 100\nprint inoise_spectrum dni(R1)\n.endc\n.end\n"
        )).unwrap();
        assert_eq!(control.control_datasets, ["noise1"]);
        assert_eq!(control.results[0].signals(), direct.results[0].signals());
        assert_eq!(control.results[0].payload(), direct.results[0].payload());
        assert_eq!(control.control_presentations.len(), 1);
    }

    #[test]
    fn control_dc_document_matches_direct_nested_sweep() {
        let source = "DC\nI1 0 out 0\nR1 out bias 1k\nV2 bias 0 0\n";
        let direct = run_authored_deck_document_detailed(&format!(
            "{source}.dc I1 list 1m 0 2m V2 list 1 3\n.end\n"
        ))
        .unwrap();
        for cards in [
            ".control\ndc I1 list 1m 0 2m V2 list 1 3\nprint v(out) vs v2\n.endc",
            ".dc I1 list 1m 0 2m V2 list 1 3\n.control\nrun\nprint v(out) vs v2\n.endc",
        ] {
            let control =
                run_authored_deck_document_detailed(&format!("{source}{cards}\n.end\n")).unwrap();
            assert_eq!(control.control_datasets, ["dc1"]);
            assert_eq!(control.control_presentations.len(), 1);
            assert_eq!(control.results[0].axes(), direct.results[0].axes());
            assert_eq!(control.results[0].signals(), direct.results[0].signals());
            assert_eq!(
                control.results[0].device_states(),
                direct.results[0].device_states()
            );
            assert_eq!(control.results[0].axes().len(), 2);
        }
    }
}
