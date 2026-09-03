use rspice_core::analysis::{StbConfig, StbSweepType};
use rspice_core::engine::{
    TransientFftHarmonic, TransientFftResult, TransientResult, TransientResultCompressed,
};
use rspice_core::netlist::{FftFormat, FftOutput};
use rspice_core::{AbortSignal, Engine, Netlist, NoAbort, ResourceLimits, SimulationConfig};

use crate::abort::ConfiguredAbort;
use crate::deck_result_document::*;
use crate::dto::*;
use crate::errors::*;
use crate::handles::*;
use crate::options::*;
use crate::result_document::*;
use crate::runners::deck::*;
use crate::runners::direct::*;
use crate::stb_result_document::*;

#[test]
fn output_directive_names_cover_the_browser_diagnostic_contract() {
    use rspice_core::netlist::OutputDirectiveKind;

    for (kind, expected) in [
        (OutputDirectiveKind::Save, "save"),
        (OutputDirectiveKind::Probe, "probe"),
        (OutputDirectiveKind::Print, "print"),
        (OutputDirectiveKind::Plot, "plot"),
        (OutputDirectiveKind::Measure, "measure"),
        (OutputDirectiveKind::Four, "four"),
        (OutputDirectiveKind::Fft, "fft"),
    ] {
        assert_eq!(output_directive_name(kind), expected);
    }
}

#[test]
fn summary_includes_nonfatal_parser_diagnostics() {
    let summary = summarize_netlist(
        "diagnostic deck\n\
         V1 in 0 1\n\
         R1 in 0 1k\n\
         .options vendorcompat=1\n\
         .end\n",
    )
    .expect("deck parses with warning");

    assert_eq!(summary.diagnostics.len(), 1);
    assert_eq!(summary.diagnostics[0].line, 4);
    assert!(
        summary.diagnostics[0]
            .message
            .to_ascii_lowercase()
            .contains("vendorcompat")
    );
}

#[test]
fn summary_exposes_structured_startup_diagnostics_additively() {
    let summary =
        summarize_netlist_detailed("startup diagnostic\nV1 in 0 1\n.IC V(MISSING)=1\n.END\n")
            .expect("an unknown startup node is a non-fatal semantic warning");

    assert!(
        summary
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "startup-undefined-node")
    );
    assert_eq!(summary.startup_diagnostics.len(), 1);
    let diagnostic = &summary.startup_diagnostics[0];
    assert_eq!(diagnostic.code, "startup-undefined-node");
    assert_eq!(diagnostic.stage, "startup_topology");
    assert_eq!(diagnostic.directive, "ic");
    assert_eq!(diagnostic.canonical_nodes, ["MISSING"]);
    assert_eq!(diagnostic.origins[0].line, 3);
    assert_eq!(diagnostic.scopes[0].kind, "top_level");
}

#[test]
fn startup_conflict_error_preserves_both_modes_and_origins() {
    let error =
        WasmError::from_parse_error(rspice_core::netlist::ParseError::StartupDirectiveConflict(
            Box::new(rspice_core::netlist::StartupDirectiveConflictError {
                first_kind: rspice_core::netlist::StartupDirectiveKind::Ic,
                first: rspice_core::netlist::NetlistSourceLocation::in_file("deck.cir", 3),
                conflicting_kind: rspice_core::netlist::StartupDirectiveKind::NodeSet,
                conflicting: rspice_core::netlist::NetlistSourceLocation::in_file(
                    "included.cir",
                    4,
                ),
            }),
        ));

    assert_eq!(error.kind, "conflicting_startup_directives");
    assert_eq!(error.category, "startup_directive_validation");
    assert_eq!(error.primary_source.as_deref(), Some("deck.cir"));
    assert_eq!(error.primary_line, Some(3));
    assert_eq!(error.related_source.as_deref(), Some("included.cir"));
    assert_eq!(error.related_line, Some(4));
    assert_eq!(error.first_startup_kind.as_deref(), Some("ic"));
    assert_eq!(error.conflicting_startup_kind.as_deref(), Some("nodeset"));
    assert!(error.unresolved_output_symbols.is_empty());
}

#[test]
fn unresolved_subcircuit_parameter_error_preserves_typed_hierarchy_identity() {
    let error = WasmError::from_parse_error(
        rspice_core::netlist::ParseError::UnresolvedSubcircuitParameter(Box::new(
            rspice_core::netlist::UnresolvedSubcircuitParameterError {
                subcircuit_name: "cell".into(),
                canonical_subcircuit_name: "CELL".into(),
                instance_name: "x1".into(),
                canonical_instance_name: "X1".into(),
                qualified_instance_name: "TOP.X1".into(),
                parameter_name: "foo".into(),
                canonical_parameter_name: "FOO".into(),
                expression: "TIME + meh".into(),
                missing_dependency: Some("MEH".into()),
                reason: "Undefined parameter: MEH".into(),
            },
        )),
    );

    assert_eq!(error.kind, "unresolved_subcircuit_parameter");
    assert_eq!(error.category, "subcircuit_parameter_resolution");
    assert_eq!(error.subcircuit_name.as_deref(), Some("cell"));
    assert_eq!(error.qualified_instance_name.as_deref(), Some("TOP.X1"));
    assert_eq!(error.parameter_name.as_deref(), Some("foo"));
    assert_eq!(error.canonical_parameter_name.as_deref(), Some("FOO"));
    assert_eq!(error.expression.as_deref(), Some("TIME + meh"));
    assert_eq!(error.missing_dependency.as_deref(), Some("MEH"));
    assert_eq!(error.reason.as_deref(), Some("Undefined parameter: MEH"));
}

#[test]
fn undefined_subcircuit_error_preserves_typed_hierarchy_identity() {
    let error = WasmError::from_parse_error(rspice_core::netlist::ParseError::UndefinedSubcircuit(
        Box::new(rspice_core::netlist::UndefinedSubcircuitError {
            subcircuit_name: "missing".into(),
            canonical_subcircuit_name: "MISSING".into(),
            instance_name: "x1".into(),
            canonical_instance_name: "X1".into(),
            qualified_instance_name: "TOP.X1".into(),
        }),
    ));

    assert_eq!(error.kind, "undefined_subcircuit");
    assert_eq!(error.category, "subcircuit_resolution");
    assert_eq!(error.subcircuit_name.as_deref(), Some("missing"));
    assert_eq!(error.canonical_subcircuit_name.as_deref(), Some("MISSING"));
    assert_eq!(error.instance_name.as_deref(), Some("x1"));
    assert_eq!(error.canonical_instance_name.as_deref(), Some("X1"));
    assert_eq!(error.qualified_instance_name.as_deref(), Some("TOP.X1"));
}

#[test]
fn missing_device_model_error_preserves_typed_device_identity() {
    let error = WasmError::from_parse_error(rspice_core::netlist::ParseError::MissingDeviceModel(
        Box::new(rspice_core::netlist::MissingDeviceModelError {
            line: 4,
            device_name: "d1".into(),
            canonical_device_name: "D1".into(),
            device_type: "DIODE".into(),
        }),
    ));

    assert_eq!(error.kind, "missing_device_model");
    assert_eq!(error.category, "device_model_resolution");
    assert_eq!(error.primary_line, Some(4));
    assert_eq!(error.instance_name.as_deref(), Some("d1"));
    assert_eq!(error.canonical_instance_name.as_deref(), Some("D1"));
    assert_eq!(error.reason.as_deref(), Some("DIODE"));
}

#[test]
fn browser_resource_defaults_are_stricter_than_desktop_defaults() {
    let browser = WasmResourceLimits::default();
    let desktop = ResourceLimits::default();

    assert_eq!(browser.max_netlist_bytes, 8 * MEBIBYTE);
    assert_eq!(browser.max_analysis_points, 200_000);
    assert_eq!(browser.max_result_values, 2_000_000);
    assert_eq!(browser.max_parallel_workers, 1);
    assert!(browser.max_netlist_bytes < desktop.max_netlist_bytes);
    assert!(browser.max_analysis_points < desktop.max_analysis_points);
}

#[test]
fn browser_health_probe_exercises_parser_and_solver() {
    let report = health_check_with_options_detailed(&WasmExecutionOptions::default())
        .expect("browser backend is ready");
    assert_eq!(report.status, "ready");
    assert!(report.ready);
    assert_eq!(report.element_count, 2);
    assert_eq!(report.node_count, 1);
    assert_eq!(report.branch_count, 1);
    assert!((report.output_voltage - 1.0).abs() <= 1.0e-12);
}

#[test]
fn partial_options_inherit_defaults_and_reject_unknown_controls() {
    let options: WasmExecutionOptions = serde_json::from_value(serde_json::json!({
        "resourceLimits": {"maxAnalysisPoints": 17}
    }))
    .expect("partial browser policy deserializes");
    assert_eq!(options.resource_limits.max_analysis_points, 17);
    assert_eq!(
        options.resource_limits.max_netlist_bytes,
        WasmResourceLimits::default().max_netlist_bytes
    );

    assert!(
        serde_json::from_value::<WasmExecutionOptions>(serde_json::json!({
            "resourceLimits": {"maxAnalaysisPoints": 17}
        }))
        .is_err(),
        "misspelled resource controls must fail closed"
    );
}

#[test]
fn parse_and_analysis_limits_publish_typed_resource_details() {
    let mut parse_options = WasmExecutionOptions::default();
    parse_options.resource_limits.max_netlist_bytes = 8;
    let parse_error = summarize_netlist_with_options_detailed(
        "bounded browser deck\nV1 1 0 1\n.END\n",
        &parse_options,
    )
    .expect_err("source must exceed the explicit browser byte ceiling");
    assert_eq!(parse_error.kind, "resource_limit");
    assert_eq!(parse_error.code, "resource_limit");
    assert_eq!(parse_error.category, "resource_limit");
    assert!(!parse_error.retryable);
    assert_eq!(parse_error.resource.as_deref(), Some("netlist_bytes"));
    assert_eq!(parse_error.limit, Some(8));

    let mut analysis_options = WasmExecutionOptions::default();
    analysis_options.resource_limits.max_analysis_points = 2;
    let analysis_error = run_ac_analysis_with_options_detailed(
        "valid\nV1 1 0 1\nR1 1 0 1k\n.END\n",
        &[1.0, 10.0, 100.0],
        &analysis_options,
    )
    .expect_err("frequency vector must exceed the explicit point ceiling");
    assert_eq!(analysis_error.kind, "resource_limit");
    assert_eq!(analysis_error.resource.as_deref(), Some("analysis_points"));
    assert_eq!(analysis_error.requested, Some(3));
    assert_eq!(analysis_error.limit, Some(2));
}

#[test]
fn simulation_errors_share_core_codes_and_retry_policy() {
    let cancelled = WasmError::from_simulation_error(rspice_core::engine::SimulationError::Aborted);
    assert_eq!(cancelled.kind, "aborted");
    assert_eq!(cancelled.code, "aborted");
    assert_eq!(cancelled.category, "cancellation");
    assert!(cancelled.retryable);

    let convergence = WasmError::from_simulation_error(
        rspice_core::engine::SimulationError::ConvergenceFailed(37),
    );
    assert_eq!(convergence.code, "convergence_error");
    assert_eq!(convergence.iterations, Some(37));
    assert!(!convergence.retryable);

    let behavioral = WasmError::from_simulation_error(
        rspice_core::engine::SimulationError::BehavioralReference(Box::new(
            rspice_core::device::BehavioralReferenceError {
                owner_name: "b2".to_string(),
                canonical_owner_name: "B2".to_string(),
                dependency_name: "b1".to_string(),
                canonical_dependency_name: "B1".to_string(),
                reason:
                    rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable,
            },
        )),
    );
    assert_eq!(behavioral.code, "behavioral_reference_error");
    assert_eq!(behavioral.instance_name.as_deref(), Some("b2"));
    assert_eq!(behavioral.canonical_instance_name.as_deref(), Some("B2"));
    assert_eq!(behavioral.missing_dependency.as_deref(), Some("B1"));
    assert_eq!(
        behavioral.reason.as_deref(),
        Some("lead_current_not_solution_variable")
    );
}

#[test]
fn ac_input_validation_rejects_non_finite_and_negative_frequencies() {
    let source = "valid\nV1 1 0 1\nR1 1 0 1k\n.END\n";
    for frequencies in [[1.0, f64::NAN], [1.0, -1.0]] {
        let error = run_ac_analysis_detailed(source, &frequencies)
            .expect_err("invalid explicit frequency must fail at the boundary");
        assert_eq!(error.kind, "invalid_argument");
        assert_eq!(error.category, "input_validation");
    }
}

const CANCELLATION_DECK: &str = "browser cancellation\n\
    V1 out 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
    R1 out 0 1k\n\
    .end\n";

fn assert_cancelled(error: Box<WasmError>) {
    assert_eq!(error.code, "aborted");
    assert_eq!(error.kind, "aborted");
    assert_eq!(error.category, "cancellation");
    assert!(error.retryable);
}

#[test]
fn every_browser_analysis_path_propagates_the_explicit_abort_source() {
    let options = WasmExecutionOptions::default();
    let abort = rspice_core::abort_signal::ImmediateAbort;

    assert_cancelled(
        run_dc_operating_point_with_options_and_abort_detailed(CANCELLATION_DECK, &options, &abort)
            .expect_err("OP must observe the frontend abort source"),
    );
    assert_cancelled(
        run_ac_analysis_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            &[1.0, 10.0],
            &options,
            &abort,
        )
        .expect_err("AC must observe the frontend abort source"),
    );
    assert_cancelled(
        run_transient_analysis_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            10.0e-6,
            1.0e-9,
            &options,
            &abort,
        )
        .expect_err("TRAN must observe the frontend abort source"),
    );
    assert_cancelled(
        run_transient_analysis_compressed_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            10.0e-6,
            1.0e-9,
            &WasmCompressionOptions::default(),
            &options,
            &abort,
        )
        .expect_err("compressed TRAN must observe the frontend abort source"),
    );
    assert_cancelled(
        run_operating_point_document_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            1,
            &options,
            &abort,
        )
        .expect_err("typed OP must observe the frontend abort source"),
    );
    assert_cancelled(
        run_dc_sweep_document_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            "V1",
            0.0,
            1.0,
            0.5,
            1,
            &options,
            &abort,
        )
        .expect_err("typed DC must observe the frontend abort source"),
    );
    assert_cancelled(
        run_ac_document_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            &[1.0, 10.0],
            1,
            &options,
            &abort,
        )
        .expect_err("typed AC must observe the frontend abort source"),
    );
    assert_cancelled(
        run_transient_document_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            10.0e-6,
            1.0e-9,
            1,
            &options,
            &abort,
        )
        .expect_err("typed TRAN must observe the frontend abort source"),
    );
    assert_cancelled(
        run_noise_document_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            "out",
            None,
            "V1",
            &[1.0, 10.0],
            1,
            &options,
            &abort,
        )
        .expect_err("typed noise must observe the frontend abort source"),
    );
    assert_cancelled(
        run_stb_document_with_options_and_abort_detailed(
            CANCELLATION_DECK,
            "V1",
            WasmStbSweep::Linear,
            2,
            1.0,
            10.0,
            true,
            1,
            &options,
            &abort,
        )
        .expect_err("typed STB must observe the frontend abort source"),
    );
    assert_cancelled(
        run_authored_deck_document_with_options_and_abort_detailed(
            "authored cancellation\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
            &options,
            &abort,
        )
        .expect_err("authored deck execution must observe the frontend abort source"),
    );
}

const AUTHORED_STEP_TRAN_DECK: &str = "authored STEP transient\n\
    .param load=1k\n\
    V1 out 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
    R1 out 0 {load}\n\
    .step param load list 1k 2k\n\
    .tran 200n 1u\n\
    .end\n";

const AUTHORED_TEMP_AC_DECK: &str = "authored TEMP AC\n\
    V1 out 0 AC 1\n\
    R1 out 0 1k\n\
    .temp 0 27\n\
    .ac lin 2 1 10\n\
    .end\n";

#[test]
fn authored_step_and_temp_wrap_only_the_authored_physical_analysis() {
    let step = run_authored_deck_document_detailed(AUTHORED_STEP_TRAN_DECK)
        .expect("STEP/TRAN authored deck executes");
    assert_eq!(step.coordinates.len(), 2);
    assert_eq!(step.planned_analyses.len(), 1);
    assert_eq!(step.planned_analyses[0].analysis_instance_id, "tran-001");
    assert_eq!(step.results.len(), 2);
    assert!(
        step.results
            .iter()
            .all(|result| result.analysis_instance_id == "tran-001")
    );
    assert!(
        step.results
            .iter()
            .all(|result| result.document.analysis.kind == AnalogAnalysisKind::Transient)
    );

    let temperature = run_authored_deck_document_detailed(AUTHORED_TEMP_AC_DECK)
        .expect("TEMP/AC authored deck executes");
    assert_eq!(temperature.coordinates.len(), 2);
    assert_eq!(temperature.results.len(), 2);
    assert!(
        temperature
            .results
            .iter()
            .all(|result| result.analysis_instance_id == "ac-001")
    );
    assert!(
        temperature
            .results
            .iter()
            .all(|result| result.document.analysis.kind == AnalogAnalysisKind::AcSmallSignal)
    );
}

#[test]
fn authored_data_backed_step_preserves_row_bindings_and_coordinates() {
    let deck = "authored DATA STEP\n\
        .param load=1k bias=1\n\
        V1 out 0 {bias}\n\
        R1 out 0 {load}\n\
        .data corners load bias\n\
        1k 1\n\
        2k 2\n\
        .enddata\n\
        .step data=corners\n\
        .op\n\
        .end\n";
    let document = run_authored_deck_document_detailed(deck)
        .expect("DATA-backed STEP executes through the canonical materializer");
    assert_eq!(document.axes.len(), 1);
    assert_eq!(document.axes[0].kind, "data");
    assert_eq!(document.axes[0].data_bindings, ["bias", "load"]);
    assert_eq!(document.coordinates.len(), 2);
    for (index, coordinate) in document.coordinates.iter().enumerate() {
        assert_eq!(coordinate.index, index);
        let DeckAxisValue::DataRow { bindings } = &coordinate.assignments[0].value else {
            panic!("DATA coordinate must retain named row bindings")
        };
        assert_eq!(
            bindings
                .iter()
                .map(|binding| binding.name.as_str())
                .collect::<Vec<_>>(),
            ["bias", "load"]
        );
    }
}

#[test]
fn authored_repeated_analyses_preserve_order_and_unique_instance_ids() {
    let deck = "authored repeated analyses\n\
        V1 out 0 DC 1 AC 1\n\
        R1 out 0 1k\n\
        .op\n\
        .ac lin 3 1 10\n\
        .ac lin 4 10 100\n\
        .end\n";
    let document =
        run_authored_deck_document_detailed(deck).expect("repeated authored analyses execute");
    assert_eq!(document.coordinates.len(), 1);
    assert_eq!(
        document
            .planned_analyses
            .iter()
            .map(|analysis| analysis.analysis_instance_id.as_str())
            .collect::<Vec<_>>(),
        ["op-001", "ac-001", "ac-002"]
    );
    assert_eq!(
        document
            .results
            .iter()
            .map(|result| result.analysis_instance_id.as_str())
            .collect::<Vec<_>>(),
        ["op-001", "ac-001", "ac-002"]
    );
    assert_eq!(document.results[1].document.point_count, 3);
    assert_eq!(document.results[2].document.point_count, 4);
    assert_ne!(
        document.results[1].output_namespace,
        document.results[2].output_namespace
    );
}

#[test]
fn authored_multi_coordinate_multi_analysis_budget_is_cumulative() {
    let deck = "authored cumulative result budget\n\
        .param load=1k\n\
        V1 out 0 DC 1 AC 1\n\
        R1 out 0 {load}\n\
        .step param load list 1k 2k\n\
        .op\n\
        .ac lin 3 1 10\n\
        .end\n";
    let accepted = run_authored_deck_document_detailed(deck)
        .expect("cumulative-budget fixture executes under defaults");
    assert_eq!(accepted.coordinates.len(), 2);
    assert_eq!(accepted.results.len(), 4);
    let contributions = accepted
        .results
        .iter()
        .map(|result| result.document.retained_numeric_value_count())
        .collect::<Vec<_>>();
    let total = contributions.iter().sum::<usize>();
    assert!(contributions.iter().all(|value| *value < total - 1));

    let mut options = WasmExecutionOptions::default();
    options.resource_limits.max_result_values = total - 1;
    let error =
        run_authored_deck_document_with_options_and_abort_detailed(deck, &options, &NoAbort)
            .expect_err("aggregate result values above the shared ceiling must fail");
    assert_eq!(error.code, "resource_limit");
    assert_eq!(error.resource.as_deref(), Some("result_values"));
    assert_eq!(error.requested, Some(total));
    assert_eq!(error.limit, Some(total - 1));
}

#[test]
fn conditional_coordinate_local_schemas_are_stable_by_coordinate_identity() {
    fn execute(values: &str) -> DeckResultDocument {
        run_authored_deck_document_detailed(&format!(
            "conditional topology\n\
             .param sel=0\n\
             V1 in 0 AC 1\n\
             .step param sel list {values}\n\
             .if (sel==0)\n\
             R1 in 0 1k\n\
             .else\n\
             R1 in mid 1k\n\
             R2 mid 0 1k\n\
             .endif\n\
             .ac lin 2 1 10\n\
             .end\n"
        ))
        .expect("conditional authored deck executes")
    }

    let forward = execute("0 1");
    let reverse = execute("1 0");
    let schema = |document: &DeckResultDocument| {
        document
            .results
            .iter()
            .map(|result| {
                (
                    result.document.coordinate_id.clone().unwrap(),
                    result
                        .document
                        .signals
                        .iter()
                        .map(|signal| signal.canonical_name.clone())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    assert_eq!(schema(&forward), schema(&reverse));
    assert_ne!(
        forward.results[0].document.signals.len(),
        forward.results[1].document.signals.len(),
        "conditional topology must retain coordinate-local schemas"
    );
}

#[test]
fn authored_deck_rejects_unsupported_analysis_shapes() {
    let unsupported = run_authored_deck_document_detailed(
        "unsupported deck\nV1 out 0 1\nR1 out 0 1k\n.tf V(out) V1\n.end\n",
    )
    .expect_err("unmapped TF must fail closed");
    assert_eq!(unsupported.code, "unsupported_deck_analysis");

    let nested_dc = run_authored_deck_document_detailed(
        "nested DC\nV1 out 0 0\nV2 x 0 0\nR1 out 0 1k\n.dc V1 0 1 1 V2 0 1 1\n.end\n",
    )
    .expect_err("nested DC schema must fail closed");
    assert_eq!(nested_dc.code, "unsupported_deck_analysis");

    let alter = run_authored_deck_document_detailed(
        "ALTER deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.alter second\nR1 out 0 2k\n.end\n",
    )
    .expect_err("textual ALTER must fail before materialization");
    assert_eq!(alter.code, "unsupported_deck_axis");
}

#[test]
fn authored_tran_default_and_explicit_max_step_contract_is_exact() {
    assert_eq!(
        resolved_authored_tran_step(1.0, 100.0, None, None).unwrap(),
        1.0
    );
    assert_eq!(
        resolved_authored_tran_step(10.0, 100.0, None, None).unwrap(),
        2.0
    );
    assert_eq!(
        resolved_authored_tran_step(10.0, 100.0, Some(90.0), None).unwrap(),
        0.2
    );
    assert_eq!(
        resolved_authored_tran_step(10.0, 100.0, Some(90.0), Some(0.25)).unwrap(),
        0.25
    );
    for error in [
        resolved_authored_tran_step(0.0, 100.0, None, None),
        resolved_authored_tran_step(1.0, 0.0, None, None),
        resolved_authored_tran_step(1.0, 100.0, Some(-1.0), None),
        resolved_authored_tran_step(1.0, 100.0, None, Some(0.0)),
        resolved_authored_tran_step(1.0, 100.0, None, Some(f64::NAN)),
    ] {
        assert_eq!(error.unwrap_err().code, "invalid_argument");
    }
}

const TYPED_DOCUMENT_DECK: &str = "browser typed analog document\n\
    V1 in 0 DC 0 AC 1 PULSE(0 1 0 1n 1n 1u 2u)\n\
    R1 in out 1k\n\
    R2 out 0 1k\n\
    .save V(out) I(V1)\n\
    .end\n";

const STB_DOCUMENT_DECK: &str = "browser typed STB document\n\
    EAMP out 0 in 0 10\n\
    VPROBE out fb 0\n\
    RF fb in 10k\n\
    RIN in 0 1k\n\
    .end\n";

#[test]
fn scalar_stb_document_retains_primary_bode_nyquist_margins_and_units() {
    let document = run_stb_document_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Linear,
        4,
        10.0,
        1.0e3,
        true,
    )
    .expect("typed STB document executes");

    assert_eq!(document.schema, STB_RESULT_SCHEMA);
    assert_eq!(document.schema_version, STB_RESULT_VERSION);
    assert_eq!(document.analysis.id, "stb-001");
    assert_eq!(document.coordinate_id, None);
    assert_eq!(document.point_count, 4);
    assert_eq!(document.primary.frequencies.len(), 4);
    assert_eq!(document.primary.loop_gains.len(), 4);
    assert_eq!(document.bode.frequencies.len(), 4);
    assert_eq!(document.bode.loop_gains.len(), 4);
    assert_eq!(document.nyquist.as_ref().unwrap().real.len(), 4);
    assert_eq!(document.retained_numeric_value_count().unwrap(), 4 * 12 + 6);
    assert_eq!(document.margins.units.gain_margin_db, StbUnit::Decibel);
    assert_eq!(
        document.margins.units.phase_margin_frequency,
        StbUnit::Hertz
    );

    let core_netlist = Netlist::parse(STB_DOCUMENT_DECK).expect("parse core STB deck");
    let core_result = Engine::new(SimulationConfig::default())
        .run_stb_with_abort(
            &core_netlist,
            StbConfig::new()
                .with_sweep(10.0, 1.0e3, 4)
                .with_sweep_type(StbSweepType::Linear)
                .with_probe("VPROBE")
                .with_nyquist(true),
            &NoAbort,
        )
        .expect("core STB reference executes");
    assert_eq!(document.primary.frequencies, core_result.frequencies);
    for (mapped, core) in document
        .primary
        .loop_gains
        .iter()
        .zip(&core_result.loop_gains)
    {
        assert_eq!(mapped.real.to_bits(), core.re.to_bits());
        assert_eq!(mapped.imaginary.to_bits(), core.im.to_bits());
    }
    for (index, core) in core_result.result.bode_points.iter().enumerate() {
        assert_eq!(
            document.bode.frequencies[index].to_bits(),
            core.frequency.to_bits()
        );
        assert_eq!(
            document.bode.magnitudes[index].to_bits(),
            core.magnitude.to_bits()
        );
        assert_eq!(
            document.bode.magnitudes_db[index].to_bits(),
            core.magnitude_db.to_bits()
        );
        assert_eq!(
            document.bode.phase_degrees[index].to_bits(),
            core.phase_deg.to_bits()
        );
        assert_eq!(
            document.bode.loop_gains[index].real.to_bits(),
            core.loop_gain.re.to_bits()
        );
        assert_eq!(
            document.bode.loop_gains[index].imaginary.to_bits(),
            core.loop_gain.im.to_bits()
        );
    }
    let mapped_nyquist = document.nyquist.as_ref().expect("mapped Nyquist data");
    for (index, core) in core_result.result.nyquist_points.iter().enumerate() {
        assert_eq!(mapped_nyquist.real[index].to_bits(), core.real.to_bits());
        assert_eq!(
            mapped_nyquist.imaginary[index].to_bits(),
            core.imag.to_bits()
        );
        assert_eq!(
            mapped_nyquist.frequencies[index].to_bits(),
            core.frequency.to_bits()
        );
    }
    let core_margins = &core_result.result.margins;
    assert_eq!(
        document.margins.gain_margin_db.to_bits(),
        core_margins.gain_margin_db.to_bits()
    );
    assert_eq!(
        document.margins.gain_margin_frequency.to_bits(),
        core_margins.gain_margin_freq.to_bits()
    );
    assert_eq!(
        document.margins.phase_margin_degrees.to_bits(),
        core_margins.phase_margin_deg.to_bits()
    );
    assert_eq!(
        document.margins.phase_margin_frequency.to_bits(),
        core_margins.phase_margin_freq.to_bits()
    );
    assert_eq!(
        document.margins.dc_gain_db.to_bits(),
        core_margins.dc_gain_db.to_bits()
    );
    assert_eq!(
        document.margins.unity_gain_bandwidth.to_bits(),
        core_margins.unity_gain_bandwidth.to_bits()
    );
    assert_eq!(
        document.margins.conditionally_stable,
        core_margins.conditionally_stable
    );
    assert_eq!(document.margins.num_crossovers, core_margins.num_crossovers);
    assert_eq!(document.margins.is_stable, core_margins.is_stable());

    let metadata = document.metadata(128).expect("STB metadata projects");
    assert!(metadata.has_nyquist);
    assert_eq!(metadata.series.len(), 10);
    assert!(metadata.series.iter().any(|descriptor| {
        descriptor.group == "bode"
            && descriptor.name == "phase_degrees"
            && descriptor.unit == StbUnit::Degree
    }));

    let json = serde_json::to_string(&document).expect("STB document JSON serializes");
    let decoded: StbResultDocument =
        serde_json::from_str(&json).expect("STB document JSON deserializes");
    decoded.validate().expect("STB JSON round trip validates");
    assert_eq!(decoded.primary.frequencies, document.primary.frequencies);
    for (decoded, original) in decoded
        .primary
        .loop_gains
        .iter()
        .zip(&document.primary.loop_gains)
    {
        assert!((decoded.real - original.real).abs() <= f64::EPSILON);
        assert!((decoded.imaginary - original.imaginary).abs() <= f64::EPSILON);
    }
    assert_eq!(
        decoded.nyquist.as_ref().map(|series| series.real.len()),
        document.nyquist.as_ref().map(|series| series.real.len())
    );
}

#[test]
fn scalar_stb_optional_nyquist_and_exact_resource_accounting_are_fail_closed() {
    let without_nyquist = run_stb_document_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Linear,
        4,
        10.0,
        1.0e3,
        false,
    )
    .expect("STB executes without Nyquist projection");
    assert!(without_nyquist.nyquist.is_none());
    assert_eq!(
        without_nyquist.retained_numeric_value_count().unwrap(),
        4 * 9 + 6
    );

    let mut options = WasmExecutionOptions::default();
    options.resource_limits.max_result_values = 4 * 12 + 6 - 1;
    let error = run_stb_document_with_options_and_abort_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Linear,
        4,
        10.0,
        1.0e3,
        true,
        1,
        &options,
        &NoAbort,
    )
    .expect_err("one value below exact retained STB accounting must fail");
    assert_eq!(error.code, "resource_limit");
    assert_eq!(error.category, "resource_limit");
    assert_eq!(error.resource.as_deref(), Some("result_values"));
    assert_eq!(error.requested, Some(4 * 12 + 6));
    assert_eq!(error.limit, Some(4 * 12 + 5));
}

#[test]
fn retained_stb_handle_enforces_exact_bounded_window_columns() {
    let document = run_stb_document_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Linear,
        4,
        10.0,
        1.0e3,
        true,
    )
    .expect("typed STB document executes");
    let cancelled_document = document.clone();
    let mut handle =
        WasmStbResultHandle::new_with_abort(document, ResourceLimits::default(), &NoAbort)
            .expect("valid handle");
    handle.maximum_window_values = 24;
    assert_eq!(
        handle
            .metadata_snapshot()
            .expect("STB metadata projects")
            .maximum_window_values,
        24
    );
    assert!(handle.window_snapshot(0, 2).is_ok());
    let error = handle
        .window_snapshot(0, 3)
        .expect_err("36-value STB window exceeds a 24-value ceiling");
    assert_eq!(error.code, "invalid_result_window");
    assert_eq!(error.category, "result_transfer");

    let abort = rspice_core::abort_signal::CountingAbort::new(3);
    let error =
        WasmStbResultHandle::new_with_abort(cancelled_document, ResourceLimits::default(), &abort)
            .expect_err("retained-handle validation must remain cancellable");
    assert_eq!(error.code, "aborted");
    assert_eq!(error.category, "cancellation");
}

#[test]
fn stb_boundary_validation_uses_typed_argument_errors() {
    let error = run_stb_document_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Decade,
        0,
        10.0,
        1.0e3,
        true,
    )
    .expect_err("zero STB density must fail at the browser boundary");
    assert_eq!(error.code, "invalid_argument");
    assert_eq!(error.category, "input_validation");

    let error = WasmStbSweep::parse("logarithmic")
        .expect_err("unknown STB sweep spelling must fail closed");
    assert_eq!(error.code, "invalid_argument");

    for error in [
        stb_metadata_error(StbDocumentError::Allocation(
            "synthetic metadata allocation failure".to_owned(),
        )),
        stb_window_error(StbDocumentError::Allocation(
            "synthetic window allocation failure".to_owned(),
        )),
    ] {
        assert_eq!(error.code, "result_allocation_failed");
        assert_eq!(error.category, "result_transfer");
    }
}

#[test]
fn typed_documents_cover_scalar_op_dc_ac_tran_and_noise_without_schema_loss() {
    let op = run_operating_point_document_detailed(TYPED_DOCUMENT_DECK)
        .expect("typed OP document executes");
    assert_eq!(op.analysis.id, "op-001");
    assert_eq!(op.coordinate_id, None);
    assert!(
        op.signals
            .iter()
            .any(|signal| signal.kind == AnalogSignalKind::BranchCurrent)
    );

    let dc = run_dc_sweep_document_detailed(TYPED_DOCUMENT_DECK, "V1", -1.0, 1.0, 1.0)
        .expect("typed DC document executes");
    assert_eq!(dc.point_count, 3);
    assert_eq!(dc.axes[0].values, [-1.0, 0.0, 1.0]);
    assert!(
        dc.signals
            .iter()
            .any(|signal| signal.kind == AnalogSignalKind::DeviceObservable)
    );

    let ac = run_ac_document_detailed(TYPED_DOCUMENT_DECK, &[1.0, 10.0])
        .expect("typed AC document executes");
    assert_eq!(ac.analysis.id, "ac-001");
    assert!(ac.signals.iter().any(|signal| {
        signal.kind == AnalogSignalKind::BranchCurrent
            && matches!(signal.values, SignalValues::Complex { .. })
    }));

    let tran = run_transient_document_detailed(TYPED_DOCUMENT_DECK, 2.0e-6, 20.0e-9)
        .expect("typed transient document executes");
    assert_eq!(tran.axes[0].unit, Some(SignalUnit::Second));
    assert!(tran.signals.iter().any(|signal| {
        signal.canonical_name == "i(v1)" && matches!(signal.values, SignalValues::Real { .. })
    }));

    let noise = run_noise_document_detailed(TYPED_DOCUMENT_DECK, "out", None, "V1", &[1.0, 10.0])
        .expect("typed noise document executes");
    assert_eq!(noise.analysis.id, "noise-001");
    assert!(noise.signals.iter().any(|signal| {
        signal.canonical_name == "output_noise_density"
            && signal.unit == Some(SignalUnit::VoltSquaredPerHertz)
    }));
    assert!(noise.signals.iter().any(|signal| {
        signal.kind == AnalogSignalKind::BranchCurrent
            && matches!(signal.values, SignalValues::Complex { .. })
    }));
}

#[test]
fn retained_result_handle_enforces_bounded_windows_and_exposes_descriptors_only() {
    let document = run_ac_document_detailed(TYPED_DOCUMENT_DECK, &[1.0, 10.0, 100.0])
        .expect("typed AC document executes");
    let mut handle =
        WasmAnalogResultHandle::new(document, ResourceLimits::default()).expect("valid handle");
    handle.maximum_window_values = 20;
    let metadata = handle.metadata_snapshot();
    assert_eq!(metadata.point_count, 3);
    assert_eq!(metadata.coordinate_id, None);
    assert!(metadata.maximum_window_values <= 20);
    assert!(handle.window_snapshot(0, 1).is_ok());
    let error = handle
        .window_snapshot(0, 3)
        .expect_err("oversized transfer must fail closed");
    assert_eq!(error.code, "invalid_result_window");
    assert_eq!(error.category, "result_transfer");
}

#[test]
fn zero_timeout_cancels_and_oversized_timeout_fails_before_work() {
    let abort = ConfiguredAbort::new(Some(0), &NoAbort)
        .expect("a zero deadline is a valid immediate-cancellation policy");
    assert!(abort.is_aborted());

    let error = match ConfiguredAbort::new(Some(MAX_TIMEOUT_MILLISECONDS + 1), &NoAbort) {
        Ok(_) => panic!("an implausibly large browser deadline must fail closed"),
        Err(error) => error,
    };
    assert_eq!(error.code, "invalid_argument");
    assert!(error.message.contains("timeoutMilliseconds"));
}

const FFT_PARITY_DECK: &str = "browser transient FFT parity\n\
    V1 out 0 SIN(0 1 1k)\n\
    R1 out 0 1k\n\
    .options fft fft_mode=1 fft_accurate=0 fftout=1\n\
    .tran 1u 1m\n\
    .fft v(out) np=128 format=unorm window=hann freq=1k fmin=1k fmax=10k\n\
    .fft {2*v(out)} np=64 format=norm window=rect\n\
    .end\n";

#[test]
fn authored_deck_attaches_complete_fft_results_to_the_exact_transient_parent() {
    let document =
        run_authored_deck_document_detailed(FFT_PARITY_DECK).expect("authored FFT deck executes");
    assert_eq!(document.results.len(), 1);
    assert_eq!(document.results[0].analysis_instance_id, "tran-001");
    assert_eq!(document.fft_results.len(), 2);
    for (index, fft) in document.fft_results.iter().enumerate() {
        assert_eq!(fft.coordinate_index, 0);
        assert_eq!(fft.parent_result_index, 0);
        assert_eq!(fft.snapshot.parent_analysis_id, "tran-001");
        assert_eq!(fft.snapshot.analysis_id, format!("fft-{:03}", index + 1));
        assert!(
            fft.output_namespace
                .ends_with(&format!("/tran-001/fft-{:03}", index + 1))
        );
    }

    let analog_values = document.results[0].document.retained_numeric_value_count();
    let fft_values = document
        .fft_results
        .iter()
        .map(|fft| {
            let bin_values = fft.snapshot.bins.indices.len() * 6;
            let (metric_values, harmonic_values) =
                fft.snapshot.metrics.as_ref().map_or((0, 0), |metrics| {
                    (
                        7 + usize::from(metrics.sfdr_spur_bin.is_some())
                            + usize::from(metrics.sfdr_spur_frequency.is_some()),
                        metrics.largest_harmonics.ranks.len() * 6,
                    )
                });
            bin_values + metric_values + harmonic_values
        })
        .sum::<usize>();
    assert_eq!(
        document.retained_numeric_value_count().unwrap(),
        analog_values + fft_values
    );
}

#[test]
fn authored_deck_attaches_each_global_fft_to_each_repeated_transient_parent() {
    let document = run_authored_deck_document_detailed(
        "repeated transient FFT parents\n\
         V1 out 0 SIN(0 1 1k)\n\
         R1 out 0 1k\n\
         .tran 10u 1m\n\
         .tran 20u 1m\n\
         .fft V(out) NP=16 FORMAT=UNORM WINDOW=HANN\n\
         .end\n",
    )
    .expect("each authored transient receives the global FFT request");

    assert_eq!(document.results.len(), 2);
    assert_eq!(document.fft_results.len(), 2);
    for (parent_index, expected_parent) in ["tran-001", "tran-002"].iter().enumerate() {
        let result = &document.results[parent_index];
        let fft = &document.fft_results[parent_index];
        assert_eq!(result.analysis_instance_id, *expected_parent);
        assert_eq!(fft.parent_result_index, parent_index);
        assert_eq!(fft.snapshot.parent_analysis_id, *expected_parent);
        assert_eq!(fft.snapshot.analysis_id, "fft-001");
        assert!(
            fft.output_namespace
                .ends_with(&format!("/{expected_parent}/fft-001"))
        );
    }
}

const ANALOG_PARITY_DECK: &str = "browser complete analog transient parity\n\
    VDD d 0 5\n\
    VG g 0 PULSE(0 3 100n 20n 20n 500n 1u)\n\
    M1 d g 0 0 NM W=10u L=1u\n\
    .model NM NMOS (LEVEL=1 VTO=1 KP=100u)\n\
    VMEM memory 0 0.2\n\
    .model MRM MEMRISTOR LEVEL=2 RON=50 ROFF=1k\n\
    YMEMRISTOR MR1 memory 0 MRM IVRELATION=1\n\
    .save V(d) I(VDD) @M1[gm] @M1[id]\n\
    .tran 20n 2u\n\
    .end\n";

fn synthetic_analog_result() -> TransientResult {
    TransientResult {
        time: vec![0.0, 1.0, 2.0],
        step_sizes: vec![0.0, 1.0, 1.0],
        voltages: vec![vec![1.0, 2.0, 3.0], Vec::new()],
        branch_currents: vec![vec![4.0, 5.0, 6.0], Vec::new()],
        num_nodes: 2,
        node_names: vec!["first".into(), "projected-node".into()],
        branch_names: vec!["VFIRST".into(), "VPROJECTED".into()],
        digital_traces: Vec::new(),
        real_traces: Vec::new(),
        device_op_traces: vec![
            rspice_core::engine::TransientDeviceOpTrace {
                device_name: "M2".into(),
                parameter: "gm".into(),
                values: vec![7.0, 8.0, 9.0],
            },
            rspice_core::engine::TransientDeviceOpTrace {
                device_name: "M1".into(),
                parameter: "id".into(),
                values: vec![10.0, 11.0, 12.0],
            },
        ],
        store_traces: vec![
            rspice_core::engine::TransientStoreTrace {
                name: "YMEMRISTOR!SECOND:R".into(),
                values: vec![13.0, 14.0, 15.0],
            },
            rspice_core::engine::TransientStoreTrace {
                name: "YMEMRISTOR!FIRST:R".into(),
                values: vec![16.0, 17.0, 18.0],
            },
        ],
        fft_results: Vec::new(),
    }
}

fn synthetic_compressed_analog_result() -> TransientResultCompressed {
    let result = synthetic_analog_result();
    let compression_config = rspice_core::engine::CompressionConfig::default();
    TransientResultCompressed {
        time: result.time,
        step_sizes: result.step_sizes,
        voltages: result.voltages,
        branch_currents: result.branch_currents,
        num_nodes: result.num_nodes,
        node_names: result.node_names,
        branch_names: result.branch_names,
        device_op_traces: result.device_op_traces,
        store_traces: result.store_traces,
        fft_results: result.fft_results,
        compression_ratio: 2.0,
        input_points: 6,
        compression_report: rspice_core::engine::TransientCompressionReport {
            schema_version: rspice_core::engine::TRANSIENT_COMPRESSION_REPORT_VERSION,
            algorithm: rspice_core::engine::TransientCompressionAlgorithm::MultiChannelRdpLinearV1,
            sample_domain:
                rspice_core::engine::TransientCompressionSampleDomain::AcceptedInputSamples,
            applied_policy: (&compression_config).into(),
            input_points: 6,
            retained_points: 3,
            worst_observed: Some(rspice_core::engine::TransientCompressionErrorObservation {
                signal: rspice_core::engine::TransientCompressionSignal::new(
                    rspice_core::engine::TransientCompressionSignalKind::Voltage,
                    "v(first)",
                )
                .expect("synthetic compression signal is valid"),
                input_sample_index: 1,
                time: 0.5,
                actual_value: 1.5,
                absolute_error: 0.0,
                relative_error: Some(0.0),
                allowed_tolerance: compression_config.abs_tol + compression_config.rel_tol * 1.5,
                tolerance_utilization: 0.0,
            }),
        },
    }
}

fn fft_parity_fixture() -> (TransientResult, TransientSnapshot) {
    let netlist = Netlist::parse(FFT_PARITY_DECK).expect("FFT parity deck parses in core");
    let core = Engine::new(SimulationConfig::default())
        .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &NoAbort)
        .expect("FFT parity deck executes in core");
    let wasm = run_transient_analysis_detailed(FFT_PARITY_DECK, 1.0e-3, 1.0e-6)
        .expect("FFT parity deck executes through browser adapter");
    (core, wasm)
}

fn assert_harmonic_parity(core: &[TransientFftHarmonic], wasm: &TransientFftHarmonicsSnapshot) {
    assert_eq!(
        wasm.ranks,
        core.iter()
            .map(|harmonic| harmonic.rank)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.bins,
        core.iter().map(|harmonic| harmonic.bin).collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.frequencies,
        core.iter()
            .map(|harmonic| harmonic.frequency)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.magnitudes,
        core.iter()
            .map(|harmonic| harmonic.magnitude)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.magnitudes_db,
        core.iter()
            .map(|harmonic| harmonic.magnitude_db)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.phase_degrees,
        core.iter()
            .map(|harmonic| harmonic.phase_degrees)
            .collect::<Vec<_>>()
    );
}

fn assert_fft_parity(core: &TransientFftResult, wasm: &TransientFftSnapshot) {
    match &core.output {
        FftOutput::Probe(probe) => {
            assert_eq!(wasm.source_kind, "probe");
            assert_eq!(&wasm.source_text, probe);
            assert_eq!(&wasm.authored_output, probe);
        }
        FftOutput::Expression(expression) => {
            assert_eq!(wasm.source_kind, "expression");
            assert_eq!(&wasm.source_text, expression);
            assert_eq!(wasm.authored_output, format!("{{{expression}}}"));
        }
    }
    assert!(wasm.analysis_id.starts_with("fft-"));
    assert_eq!(wasm.parent_analysis_id, "tran-001");
    assert!(wasm.ordinal > 0);
    assert_eq!(wasm.output_name, core.output_name);
    assert_eq!(wasm.physical_type, core.physical_type);
    assert_eq!(
        wasm.value_unit.as_deref(),
        fft_value_unit(core.physical_type, core.format).unwrap()
    );
    assert_eq!(wasm.start_time, core.start_time);
    assert_eq!(wasm.stop_time, core.stop_time);
    assert_eq!(wasm.sample_interval, core.sample_interval);
    assert_eq!(wasm.point_count, core.point_count);
    assert_eq!(wasm.accurate_sampling, core.accurate_sampling);
    assert_eq!(wasm.format, fft_format_name(core.format));
    assert_eq!(wasm.mode, fft_mode_name(core.mode));
    assert_eq!(wasm.window, fft_window_name(core.window));
    assert_eq!(wasm.window_name, core.window_name);
    assert_eq!(wasm.alpha, core.alpha);
    assert_eq!(wasm.coherent_gain, core.coherent_gain);
    assert_eq!(wasm.frequency_resolution, core.frequency_resolution);
    assert_eq!(wasm.fundamental_bin, core.fundamental_bin);
    assert_eq!(wasm.minimum_metric_bin, core.minimum_metric_bin);
    assert_eq!(wasm.maximum_metric_bin, core.maximum_metric_bin);
    assert_eq!(
        wasm.bins.indices,
        core.bins.iter().map(|bin| bin.index).collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.bins.frequencies,
        core.bins
            .iter()
            .map(|bin| bin.frequency)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.bins.real,
        core.bins.iter().map(|bin| bin.real).collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.bins.imaginary,
        core.bins
            .iter()
            .map(|bin| bin.imaginary)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.bins.magnitudes,
        core.bins
            .iter()
            .map(|bin| bin.magnitude)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.bins.phase_degrees,
        core.bins
            .iter()
            .map(|bin| bin.phase_degrees)
            .collect::<Vec<_>>()
    );

    match (&core.metrics, &wasm.metrics) {
        (Some(core), Some(wasm)) => {
            assert_eq!(wasm.fundamental_magnitude, core.fundamental_magnitude);
            assert_eq!(wasm.thd_ratio, core.thd_ratio);
            assert_eq!(wasm.thd_db, core.thd_db);
            assert_eq!(wasm.sndr_db, core.sndr_db);
            assert_eq!(wasm.enob_bits, core.enob_bits);
            assert_eq!(wasm.snr_db, core.snr_db);
            assert_eq!(wasm.sfdr_db, core.sfdr_db);
            assert_eq!(wasm.sfdr_spur_bin, core.sfdr_spur_bin);
            assert_eq!(wasm.sfdr_spur_frequency, core.sfdr_spur_frequency);
            assert_harmonic_parity(&core.largest_harmonics, &wasm.largest_harmonics);
        }
        (None, None) => {}
        _ => panic!("browser FFT metrics optionality differs from core"),
    }
}

fn assert_object_fields(value: &serde_json::Value, expected: &[&str]) {
    let object = value.as_object().expect("contract value must be an object");
    let mut actual = object.keys().map(String::as_str).collect::<Vec<_>>();
    actual.sort_unstable();
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    assert_eq!(actual, expected);
}

#[test]
fn transient_fft_adapter_preserves_core_values_and_source_order() {
    let (core, wasm) = fft_parity_fixture();
    assert_eq!(wasm.fft_results.len(), core.fft_results.len());
    for (core, wasm) in core.fft_results.iter().zip(&wasm.fft_results) {
        assert_fft_parity(core, wasm);
    }
    assert_eq!(wasm.fft_results[0].analysis_id, "fft-001");
    assert_eq!(wasm.fft_results[0].ordinal, 1);
    assert_eq!(wasm.fft_results[1].analysis_id, "fft-002");
    assert_eq!(wasm.fft_results[1].ordinal, 2);
    assert_eq!(wasm.fft_results[0].output_name, "V(OUT)");
    assert_eq!(wasm.fft_results[1].output_name, "{2*v(out)}");
}

#[test]
fn transient_fft_value_units_cover_every_supported_quantity_and_format() {
    use FftFormat::{Normalized, Unnormalized};

    assert_eq!(fft_value_unit("voltage", Normalized).unwrap(), Some("1"));
    assert_eq!(fft_value_unit("current", Normalized).unwrap(), Some("1"));
    assert_eq!(fft_value_unit("parameter", Normalized).unwrap(), Some("1"));
    assert_eq!(fft_value_unit("voltage", Unnormalized).unwrap(), Some("V"));
    assert_eq!(fft_value_unit("current", Unnormalized).unwrap(), Some("A"));
    assert_eq!(fft_value_unit("parameter", Unnormalized).unwrap(), None);
    assert!(fft_value_unit("unsupported", Normalized).is_err());
}

#[test]
fn transient_fft_snapshot_conversion_rejects_unsupported_physical_type() {
    let netlist = Netlist::parse(FFT_PARITY_DECK).expect("FFT parity deck parses in core");
    let mut core = Engine::new(SimulationConfig::default())
        .run_tran_with_abort(&netlist, 1.0e-3, 1.0e-6, &NoAbort)
        .expect("FFT parity deck executes in core");
    core.fft_results[0].physical_type = "unsupported";

    let error = transient_snapshot_from_result(core)
        .expect_err("unknown FFT physical types must fail snapshot conversion");
    assert!(error.contains("unsupported transient FFT physical type 'unsupported'"));
}

#[test]
fn transient_analog_adapter_preserves_complete_inventory_order_and_missingness() {
    let full = transient_snapshot_from_result(synthetic_analog_result())
        .expect("valid full analog result adapts");
    assert_eq!(full.time, [0.0, 1.0, 2.0]);
    assert_eq!(full.step_sizes, [0.0, 1.0, 1.0]);
    assert_eq!(full.num_nodes, 2);
    assert_eq!(full.node_names, ["first", "projected-node"]);
    assert_eq!(full.voltages[0].as_deref(), Some(&[1.0, 2.0, 3.0][..]));
    assert_eq!(full.voltages[1], None);
    assert_eq!(full.branch_names, ["VFIRST", "VPROJECTED"]);
    assert_eq!(
        full.branch_currents[0].as_deref(),
        Some(&[4.0, 5.0, 6.0][..])
    );
    assert_eq!(full.branch_currents[1], None);
    assert_eq!(
        full.device_op_traces
            .iter()
            .map(|trace| (trace.device_name.as_str(), trace.parameter.as_str()))
            .collect::<Vec<_>>(),
        [("M2", "gm"), ("M1", "id")]
    );
    assert_eq!(
        full.store_traces
            .iter()
            .map(|trace| trace.name.as_str())
            .collect::<Vec<_>>(),
        ["YMEMRISTOR!SECOND:R", "YMEMRISTOR!FIRST:R"]
    );
    assert_eq!(full.compression, None);

    let compressed =
        transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
            .expect("valid compressed analog result adapts");
    assert_eq!(compressed.time, full.time);
    assert_eq!(compressed.step_sizes, full.step_sizes);
    assert_eq!(compressed.node_names, full.node_names);
    assert_eq!(compressed.voltages, full.voltages);
    assert_eq!(compressed.branch_names, full.branch_names);
    assert_eq!(compressed.branch_currents, full.branch_currents);
    assert_eq!(compressed.device_op_traces, full.device_op_traces);
    assert_eq!(compressed.store_traces, full.store_traces);
    assert_eq!(
        compressed.compression,
        Some(TransientCompressionSnapshot {
            schema_version: rspice_core::engine::TRANSIENT_COMPRESSION_REPORT_VERSION,
            algorithm: "multi-channel-rdp-linear-v1".to_string(),
            sample_domain: "accepted-input-samples".to_string(),
            enabled: true,
            absolute_tolerance: 1.0e-6,
            relative_tolerance: 1.0e-3,
            maximum_retained_interval: 0.0,
            input_points: 6,
            retained_points: 3,
            compression_ratio: 2.0,
            worst_observed: Some(TransientCompressionErrorSnapshot {
                signal_kind: "voltage".to_string(),
                canonical_name: "v(first)".to_string(),
                input_sample_index: 1,
                time: 0.5,
                actual_value: 1.5,
                absolute_error: 0.0,
                relative_error: Some(0.0),
                allowed_tolerance: 1.501e-3,
                tolerance_utilization: 0.0,
            }),
        })
    );
}

#[test]
fn transient_analog_adapter_matches_actual_core_execution_inventory() {
    let netlist = Netlist::parse(ANALOG_PARITY_DECK).expect("analog parity deck parses");
    let core = Engine::new(SimulationConfig::default())
        .run_tran_with_abort(&netlist, 2.0e-6, 20.0e-9, &NoAbort)
        .expect("analog parity deck executes in core");
    let wasm = run_transient_analysis_detailed(ANALOG_PARITY_DECK, 2.0e-6, 20.0e-9)
        .expect("analog parity deck executes through browser adapter");

    assert_eq!(wasm.time, core.time);
    assert_eq!(wasm.step_sizes, core.step_sizes);
    assert_eq!(wasm.num_nodes, core.num_nodes);
    assert_eq!(wasm.node_names, core.node_names);
    assert_eq!(wasm.branch_names, core.branch_names);
    for (adapted, source) in wasm.voltages.iter().zip(&core.voltages) {
        assert_eq!(
            adapted.as_deref(),
            (!source.is_empty()).then_some(source.as_slice())
        );
    }
    for (adapted, source) in wasm.branch_currents.iter().zip(&core.branch_currents) {
        assert_eq!(
            adapted.as_deref(),
            (!source.is_empty()).then_some(source.as_slice())
        );
    }
    assert_eq!(
        wasm.device_op_traces
            .iter()
            .map(|trace| (
                trace.device_name.as_str(),
                trace.parameter.as_str(),
                &trace.values
            ))
            .collect::<Vec<_>>(),
        core.device_op_traces
            .iter()
            .map(|trace| (
                trace.device_name.as_str(),
                trace.parameter.as_str(),
                &trace.values
            ))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        wasm.store_traces
            .iter()
            .map(|trace| (trace.name.as_str(), &trace.values))
            .collect::<Vec<_>>(),
        core.store_traces
            .iter()
            .map(|trace| (trace.name.as_str(), &trace.values))
            .collect::<Vec<_>>()
    );
    assert!(
        wasm.voltages.iter().any(Option::is_none),
        "authored .SAVE projection must remain explicit"
    );
    assert!(
        wasm.branch_currents.iter().any(Option::is_none),
        "projected-out branch currents must remain explicit"
    );
    assert!(
        wasm.device_op_traces
            .iter()
            .any(|trace| trace.device_name.eq_ignore_ascii_case("M1")
                && trace.parameter.eq_ignore_ascii_case("gm")),
        "requested device operating-point trace is missing"
    );
    assert_eq!(
        wasm.store_traces
            .iter()
            .map(|trace| trace.name.as_str())
            .collect::<Vec<_>>(),
        ["YMEMRISTOR!MR1:R"]
    );
    assert_eq!(wasm.compression, None);

    let compression_options = WasmCompressionOptions {
        absolute_tolerance: 1.0e-8,
        relative_tolerance: 1.0e-4,
        enabled: true,
        maximum_interval: 100.0e-9,
    };
    let compressed_core = Engine::new(SimulationConfig::default())
        .run_tran_compressed_with_abort(
            &netlist,
            2.0e-6,
            20.0e-9,
            compression_options
                .to_core()
                .expect("compression options are valid"),
            &NoAbort,
        )
        .expect("analog parity deck executes through core compression");
    let compressed = transient_snapshot_from_compressed_result(compressed_core.clone())
        .expect("actual compressed core result adapts");
    let public_compressed = run_transient_analysis_compressed_detailed(
        ANALOG_PARITY_DECK,
        2.0e-6,
        20.0e-9,
        &compression_options,
    )
    .expect("compressed browser API executes");
    assert_eq!(public_compressed, compressed);
    assert_eq!(compressed.time, compressed_core.time);
    assert_eq!(compressed.step_sizes, compressed_core.step_sizes);
    assert_eq!(compressed.node_names, compressed_core.node_names);
    assert_eq!(compressed.branch_names, compressed_core.branch_names);
    assert_eq!(
        compressed.device_op_traces.len(),
        compressed_core.device_op_traces.len()
    );
    assert_eq!(
        compressed.store_traces.len(),
        compressed_core.store_traces.len()
    );
    assert_eq!(
        compressed.compression,
        Some(TransientCompressionSnapshot {
            schema_version: compressed_core.compression_report.schema_version,
            algorithm: compressed_core
                .compression_report
                .algorithm
                .as_str()
                .to_string(),
            sample_domain: compressed_core
                .compression_report
                .sample_domain
                .as_str()
                .to_string(),
            enabled: compressed_core.compression_report.applied_policy.enabled,
            absolute_tolerance: compressed_core
                .compression_report
                .applied_policy
                .absolute_tolerance,
            relative_tolerance: compressed_core
                .compression_report
                .applied_policy
                .relative_tolerance,
            maximum_retained_interval: compressed_core
                .compression_report
                .applied_policy
                .maximum_retained_interval,
            input_points: compressed_core.input_points,
            retained_points: compressed_core.time.len(),
            compression_ratio: compressed_core.compression_ratio,
            worst_observed: compressed_core
                .compression_report
                .worst_observed
                .map(|observation| TransientCompressionErrorSnapshot {
                    signal_kind: observation.signal.kind.as_str().to_string(),
                    canonical_name: observation.signal.canonical_name,
                    input_sample_index: observation.input_sample_index,
                    time: observation.time,
                    actual_value: observation.actual_value,
                    absolute_error: observation.absolute_error,
                    relative_error: observation.relative_error,
                    allowed_tolerance: observation.allowed_tolerance,
                    tolerance_utilization: observation.tolerance_utilization,
                }),
        })
    );
}

#[test]
fn transient_compression_options_fail_closed() {
    for options in [
        WasmCompressionOptions {
            absolute_tolerance: -1.0,
            ..WasmCompressionOptions::default()
        },
        WasmCompressionOptions {
            relative_tolerance: f64::NAN,
            ..WasmCompressionOptions::default()
        },
        WasmCompressionOptions {
            maximum_interval: f64::INFINITY,
            ..WasmCompressionOptions::default()
        },
    ] {
        let error = options
            .to_core()
            .expect_err("invalid compression policy must be rejected");
        assert_eq!(error.kind, "invalid_argument");
        assert_eq!(error.category, "input_validation");
    }

    let unknown = serde_json::from_value::<WasmCompressionOptions>(serde_json::json!({
        "absoluteTolerance": 1.0e-6,
        "misspelledTolerance": 1.0e-3,
    }));
    assert!(
        unknown.is_err(),
        "unknown compression fields must fail closed"
    );
}

#[test]
fn transient_fft_dto_round_trips_and_inventory_covers_every_field() {
    const TRANSIENT_FIELDS: &[&str] = &[
        "time",
        "step_sizes",
        "num_nodes",
        "node_names",
        "voltages",
        "branch_names",
        "branch_currents",
        "device_op_traces",
        "store_traces",
        "fft_results",
        "compression",
    ];
    const FFT_FIELDS: &[&str] = &[
        "analysis_id",
        "parent_analysis_id",
        "ordinal",
        "source_kind",
        "source_text",
        "authored_output",
        "output_name",
        "physical_type",
        "value_unit",
        "start_time",
        "stop_time",
        "sample_interval",
        "point_count",
        "accurate_sampling",
        "format",
        "mode",
        "window",
        "window_name",
        "alpha",
        "coherent_gain",
        "frequency_resolution",
        "fundamental_bin",
        "minimum_metric_bin",
        "maximum_metric_bin",
        "bins",
        "metrics",
    ];
    const BIN_FIELDS: &[&str] = &[
        "indices",
        "frequencies",
        "real",
        "imaginary",
        "magnitudes",
        "phase_degrees",
    ];
    const METRIC_FIELDS: &[&str] = &[
        "fundamental_magnitude",
        "thd_ratio",
        "thd_db",
        "sndr_db",
        "enob_bits",
        "snr_db",
        "sfdr_db",
        "sfdr_spur_bin",
        "sfdr_spur_frequency",
        "largest_harmonics",
    ];
    const HARMONIC_FIELDS: &[&str] = &[
        "ranks",
        "bins",
        "frequencies",
        "magnitudes",
        "magnitudes_db",
        "phase_degrees",
    ];
    const DEVICE_OP_FIELDS: &[&str] = &["device_name", "parameter", "values"];
    const STORE_FIELDS: &[&str] = &["name", "values"];
    const COMPRESSION_FIELDS: &[&str] = &[
        "schema_version",
        "algorithm",
        "sample_domain",
        "enabled",
        "absolute_tolerance",
        "relative_tolerance",
        "maximum_retained_interval",
        "input_points",
        "retained_points",
        "compression_ratio",
        "worst_observed",
    ];
    const COMPRESSION_ERROR_FIELDS: &[&str] = &[
        "signal_kind",
        "canonical_name",
        "input_sample_index",
        "time",
        "actual_value",
        "absolute_error",
        "relative_error",
        "allowed_tolerance",
        "tolerance_utilization",
    ];

    let (_, snapshot) = fft_parity_fixture();
    let encoded = serde_json::to_value(&snapshot).expect("serialize transient FFT DTO");
    assert_object_fields(&encoded, TRANSIENT_FIELDS);
    let first = &encoded["fft_results"][0];
    assert_object_fields(first, FFT_FIELDS);
    assert_eq!(first["physical_type"], "voltage");
    assert_eq!(first["value_unit"], "V");
    assert_eq!(encoded["fft_results"][1]["physical_type"], "parameter");
    assert_eq!(encoded["fft_results"][1]["value_unit"], "1");
    assert_object_fields(&first["bins"], BIN_FIELDS);
    assert_object_fields(&first["metrics"], METRIC_FIELDS);
    assert_object_fields(&first["metrics"]["largest_harmonics"], HARMONIC_FIELDS);

    let decoded: TransientSnapshot =
        serde_json::from_value(encoded).expect("deserialize transient FFT DTO");
    assert_eq!(decoded, snapshot);

    let mut without_metrics = snapshot.clone();
    without_metrics.fft_results[0].metrics = None;
    let encoded = serde_json::to_value(without_metrics).expect("serialize absent metrics");
    assert!(encoded["fft_results"][0]["metrics"].is_null());

    let mut unnormalized_parameter = snapshot;
    unnormalized_parameter.fft_results[1].format = "unnormalized".to_string();
    unnormalized_parameter.fft_results[1].value_unit = None;
    let encoded = serde_json::to_value(unnormalized_parameter)
        .expect("serialize unnormalized parameter FFT unit");
    assert!(encoded["fft_results"][1]["value_unit"].is_null());

    let analog = transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
        .expect("compressed analog DTO adapts");
    let encoded = serde_json::to_value(&analog).expect("serialize complete analog DTO");
    assert_object_fields(&encoded["device_op_traces"][0], DEVICE_OP_FIELDS);
    assert_object_fields(&encoded["store_traces"][0], STORE_FIELDS);
    assert_object_fields(&encoded["compression"], COMPRESSION_FIELDS);
    assert_object_fields(
        &encoded["compression"]["worst_observed"],
        COMPRESSION_ERROR_FIELDS,
    );
    assert!(encoded["voltages"][1].is_null());
    assert!(encoded["branch_currents"][1].is_null());
    let decoded: TransientSnapshot =
        serde_json::from_value(encoded).expect("deserialize complete analog DTO");
    assert_eq!(decoded, analog);
}

#[cfg(target_arch = "wasm32")]
fn js_shared_cancellation_options(cancelled: bool) -> JsValue {
    let buffer = js_sys::SharedArrayBuffer::new(4);
    let view = js_sys::Int32Array::new(buffer.as_ref());
    js_sys::Atomics::store(&view, 0, i32::from(cancelled))
        .expect("Node supports Atomics.store on SharedArrayBuffer");

    let cancellation = js_sys::Object::new();
    js_sys::Reflect::set(
        &cancellation,
        &JsValue::from_str("mechanism"),
        &JsValue::from_str("sharedInt32"),
    )
    .expect("set cancellation mechanism");
    js_sys::Reflect::set(&cancellation, &JsValue::from_str("view"), &view)
        .expect("set cancellation view");

    let options = js_sys::Object::new();
    js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &cancellation)
        .expect("set cancellation policy");
    options.into()
}

#[cfg(target_arch = "wasm32")]
fn assert_js_error_code(error: JsValue, expected: &str) {
    assert_eq!(
        js_property(&error, "code")
            .expect("RSpiceError has a code")
            .as_string()
            .as_deref(),
        Some(expected)
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn node_shared_control_word_cancels_every_analysis_export() {
    let options = || js_shared_cancellation_options(true);

    assert_js_error_code(
        run_dc_operating_point_js(CANCELLATION_DECK, options())
            .expect_err("pre-set shared flag cancels OP"),
        "aborted",
    );
    assert_js_error_code(
        run_ac_analysis_js(CANCELLATION_DECK, vec![1.0, 10.0], options())
            .expect_err("pre-set shared flag cancels AC"),
        "aborted",
    );
    assert_js_error_code(
        run_transient_analysis_js(CANCELLATION_DECK, 10.0e-6, 1.0e-9, options())
            .expect_err("pre-set shared flag cancels TRAN"),
        "aborted",
    );
    assert_js_error_code(
        run_transient_analysis_compressed_js(
            CANCELLATION_DECK,
            10.0e-6,
            1.0e-9,
            JsValue::NULL,
            options(),
        )
        .expect_err("pre-set shared flag cancels compressed TRAN"),
        "aborted",
    );
    assert_js_error_code(
        run_operating_point_document_js(CANCELLATION_DECK, 1, options())
            .expect_err("pre-set shared flag cancels typed OP"),
        "aborted",
    );
    assert_js_error_code(
        run_dc_sweep_document_js(CANCELLATION_DECK, "V1", 0.0, 1.0, 0.5, 1, options())
            .expect_err("pre-set shared flag cancels typed DC"),
        "aborted",
    );
    assert_js_error_code(
        run_ac_document_js(CANCELLATION_DECK, vec![1.0, 10.0], 1, options())
            .expect_err("pre-set shared flag cancels typed AC"),
        "aborted",
    );
    assert_js_error_code(
        run_transient_document_js(CANCELLATION_DECK, 10.0e-6, 1.0e-9, 1, options())
            .expect_err("pre-set shared flag cancels typed TRAN"),
        "aborted",
    );
    assert_js_error_code(
        run_noise_document_js(
            CANCELLATION_DECK,
            "out",
            None,
            "V1",
            vec![1.0, 10.0],
            1,
            options(),
        )
        .expect_err("pre-set shared flag cancels typed noise"),
        "aborted",
    );
    assert_js_error_code(
        run_stb_document_js(
            CANCELLATION_DECK,
            "V1",
            "linear",
            2,
            1.0,
            10.0,
            true,
            1,
            options(),
        )
        .expect_err("pre-set shared flag cancels typed STB"),
        "aborted",
    );
    assert_js_error_code(
        run_authored_deck_document_js(
            "authored JS cancellation\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
            options(),
        )
        .expect_err("pre-set shared flag cancels authored deck execution"),
        "aborted",
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn authored_deck_public_js_handle_preserves_axes_ids_and_typed_windows() {
    let handle = run_authored_deck_document_js(AUTHORED_TEMP_AC_DECK, JsValue::NULL)
        .expect("public authored deck export executes under Node");
    assert_eq!(handle.coordinate_count(), 2);
    assert_eq!(handle.result_count(), 2);
    assert_eq!(handle.fft_result_count(), 0);

    let metadata = handle.metadata_js().expect("deck metadata serializes");
    assert_eq!(
        js_property(&metadata, "schema")
            .expect("schema exists")
            .as_string()
            .as_deref(),
        Some(DECK_RESULT_SCHEMA)
    );
    let coordinates = js_array_property(&metadata, "coordinates")
        .expect("canonical coordinate descriptors exist");
    assert_eq!(coordinates.length(), 2);
    let results =
        js_array_property(&metadata, "results").expect("canonical result summaries exist");
    assert_eq!(results.length(), 2);
    assert_eq!(
        js_property(&results.get(0), "analysisInstanceId")
            .expect("stable analysis instance id exists")
            .as_string()
            .as_deref(),
        Some("ac-001")
    );
    assert_ne!(
        js_property(&coordinates.get(0), "id")
            .expect("first coordinate id exists")
            .as_string(),
        js_property(&coordinates.get(1), "id")
            .expect("second coordinate id exists")
            .as_string()
    );

    let result_metadata = handle
        .result_metadata_js(0)
        .expect("coordinate-local schema serializes");
    assert_eq!(
        js_property(&result_metadata, "coordinateId")
            .expect("result coordinate id exists")
            .as_string(),
        js_property(&coordinates.get(0), "id")
            .expect("coordinate id exists")
            .as_string()
    );
    let window = handle
        .read_window_js(0, 0, 1)
        .expect("bounded coordinate-local window transfers");
    let axes = js_array_property(&window, "axes").expect("result axes exist");
    assert!(
        js_property(&axes.get(0), "values")
            .expect("frequency values exist")
            .is_instance_of::<js_sys::Float64Array>()
    );
    let signals = js_array_property(&window, "signals").expect("result signals exist");
    let values = js_property(&signals.get(0), "values").expect("signal values exist");
    assert!(
        js_property(&values, "real")
            .expect("complex real values exist")
            .is_instance_of::<js_sys::Float64Array>()
    );
    assert!(
        js_property(&values, "validity")
            .expect("signal validity exists")
            .is_instance_of::<js_sys::Uint8Array>()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn authored_deck_node_regressions_cover_step_repetition_conditionals_and_fail_closed() {
    let step = run_authored_deck_document_js(AUTHORED_STEP_TRAN_DECK, JsValue::NULL)
        .expect("public STEP/TRAN deck executes");
    assert_eq!(step.coordinate_count(), 2);
    assert_eq!(step.result_count(), 2);
    assert!(
        step.document()
            .results
            .iter()
            .all(|result| result.analysis_instance_id == "tran-001")
    );

    let repeated = run_authored_deck_document_js(
        "Node repeated analyses\n\
         V1 out 0 DC 1 AC 1\n\
         R1 out 0 1k\n\
         .op\n\
         .ac lin 3 1 10\n\
         .ac lin 4 10 100\n\
         .end\n",
        JsValue::NULL,
    )
    .expect("public repeated-analysis deck executes");
    assert_eq!(
        repeated
            .document()
            .results
            .iter()
            .map(|result| result.analysis_instance_id.as_str())
            .collect::<Vec<_>>(),
        ["op-001", "ac-001", "ac-002"]
    );

    let conditional = |values: &str| {
        run_authored_deck_document_js(
            &format!(
                "Node conditional topology\n\
                 .param sel=0\n\
                 V1 in 0 AC 1\n\
                 .step param sel list {values}\n\
                 .if (sel==0)\n\
                 R1 in 0 1k\n\
                 .else\n\
                 R1 in mid 1k\n\
                 R2 mid 0 1k\n\
                 .endif\n\
                 .ac lin 3 1 10\n\
                 .end\n"
            ),
            JsValue::NULL,
        )
        .expect("public conditional deck executes")
    };
    let forward = conditional("0 1");
    let reverse = conditional("1 0");
    let schemas = |handle: &WasmDeckResultHandle| {
        handle
            .document()
            .results
            .iter()
            .map(|result| {
                (
                    result.document.coordinate_id.clone().unwrap(),
                    result
                        .document
                        .signals
                        .iter()
                        .map(|signal| signal.canonical_name.clone())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>()
    };
    assert_eq!(schemas(&forward), schemas(&reverse));

    assert_js_error_code(
        run_authored_deck_document_js(
            "Node unsupported\nV1 out 0 1\nR1 out 0 1k\n.tf V(out) V1\n.end\n",
            JsValue::NULL,
        )
        .expect_err("unmapped authored analysis must fail through the JS export"),
        "unsupported_deck_analysis",
    );
    assert_js_error_code(
        run_authored_deck_document_js(
            "Node malformed\nV1 out 0 1\nR1 out 0 1k!\n.end\n",
            JsValue::NULL,
        )
        .expect_err("malformed authored deck must fail through the JS export"),
        "parse_error",
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn node_cancellation_options_fail_closed() {
    let unsupported = js_sys::Object::new();
    let unsupported_cancellation = js_sys::Object::new();
    js_sys::Reflect::set(
        &unsupported_cancellation,
        &JsValue::from_str("mechanism"),
        &JsValue::from_str("abortSignal"),
    )
    .expect("set unsupported mechanism");
    js_sys::Reflect::set(
        &unsupported,
        &JsValue::from_str("cancellation"),
        &unsupported_cancellation,
    )
    .expect("set unsupported cancellation object");
    assert_js_error_code(
        run_dc_operating_point_js(CANCELLATION_DECK, unsupported.into())
            .expect_err("DOM AbortSignal must not appear supported"),
        "unsupported_cancellation",
    );

    let ordinary = js_sys::Object::new();
    let ordinary_cancellation = js_sys::Object::new();
    js_sys::Reflect::set(
        &ordinary_cancellation,
        &JsValue::from_str("mechanism"),
        &JsValue::from_str("sharedInt32"),
    )
    .expect("set shared mechanism");
    let ordinary_view = js_sys::Int32Array::new_with_length(1);
    js_sys::Reflect::set(
        &ordinary_cancellation,
        &JsValue::from_str("view"),
        &ordinary_view,
    )
    .expect("set ordinary view");
    js_sys::Reflect::set(
        &ordinary,
        &JsValue::from_str("cancellation"),
        &ordinary_cancellation,
    )
    .expect("set ordinary cancellation object");
    assert_js_error_code(
        run_dc_operating_point_js(CANCELLATION_DECK, ordinary.into())
            .expect_err("ordinary ArrayBuffer must not masquerade as shared cancellation"),
        "invalid_argument",
    );

    for field in ["resourceLimits", "timeoutMilliseconds", "cancellation"] {
        let malformed = js_sys::Object::new();
        js_sys::Reflect::set(&malformed, &JsValue::from_str(field), &JsValue::NULL)
            .expect("set explicit null execution option");
        assert_js_error_code(
            run_dc_operating_point_js(CANCELLATION_DECK, malformed.into())
                .expect_err("explicit null execution fields must not become defaults"),
            "invalid_argument",
        );
    }

    let timeout = js_sys::Object::new();
    js_sys::Reflect::set(
        &timeout,
        &JsValue::from_str("timeoutMilliseconds"),
        &JsValue::from_f64(0.0),
    )
    .expect("set zero timeout");
    assert_js_error_code(
        run_dc_operating_point_js(CANCELLATION_DECK, timeout.into())
            .expect_err("zero timeout requests immediate cancellation"),
        "aborted",
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn transient_analog_js_contract_uses_typed_arrays_and_explicit_missingness() {
    let snapshot = transient_snapshot_from_result(synthetic_analog_result())
        .expect("full analog fixture adapts");
    let serialized = serialize_transient_to_js(&snapshot).expect("serialize complete analog DTO");

    for field in ["time", "step_sizes"] {
        assert!(
            js_property(&serialized, field)
                .expect("top-level numeric column exists")
                .is_instance_of::<js_sys::Float64Array>(),
            "{field} is not a Float64Array"
        );
    }

    let voltages =
        js_array_property(&serialized, "voltages").expect("voltage waveform collection exists");
    assert!(voltages.get(0).is_instance_of::<js_sys::Float64Array>());
    assert!(voltages.get(1).is_null());
    let currents = js_array_property(&serialized, "branch_currents")
        .expect("branch-current waveform collection exists");
    assert!(currents.get(0).is_instance_of::<js_sys::Float64Array>());
    assert!(currents.get(1).is_null());

    for collection in ["device_op_traces", "store_traces"] {
        let traces = js_array_property(&serialized, collection).expect("trace collection exists");
        assert!(
            js_property(&traces.get(0), "values")
                .expect("trace values exist")
                .is_instance_of::<js_sys::Float64Array>(),
            "{collection} values are not a Float64Array"
        );
    }
    assert!(
        js_property(&serialized, "compression")
            .expect("compression property exists")
            .is_null()
    );

    let decoded: TransientSnapshot = serde_wasm_bindgen::from_value(serialized)
        .expect("typed-array analog contract round-trips to its Rust DTO");
    assert_eq!(decoded, snapshot);

    let compressed =
        transient_snapshot_from_compressed_result(synthetic_compressed_analog_result())
            .expect("compressed analog fixture adapts");
    let serialized =
        serialize_transient_to_js(&compressed).expect("serialize compressed analog DTO");
    assert!(
        !js_property(&serialized, "compression")
            .expect("compression property exists")
            .is_null()
    );
    let decoded: TransientSnapshot = serde_wasm_bindgen::from_value(serialized)
        .expect("compressed analog contract round-trips to its Rust DTO");
    assert_eq!(decoded, compressed);

    let compression_options = serde_wasm_bindgen::to_value(&WasmCompressionOptions {
        absolute_tolerance: 1.0e-8,
        relative_tolerance: 1.0e-4,
        maximum_interval: 100.0e-9,
        enabled: true,
    })
    .expect("compression options serialize");
    let executed = run_transient_analysis_compressed_js(
        ANALOG_PARITY_DECK,
        2.0e-6,
        20.0e-9,
        compression_options,
        JsValue::NULL,
    )
    .expect("compressed analog API executes under wasm32");
    assert!(
        js_property(&executed, "time")
            .expect("executed time exists")
            .is_instance_of::<js_sys::Float64Array>()
    );
    assert!(
        !js_property(&executed, "compression")
            .expect("executed compression provenance exists")
            .is_null()
    );
    assert!(
        js_array_property(&executed, "device_op_traces")
            .expect("executed device operating-point traces exist")
            .length()
            >= 2
    );
    assert_eq!(
        js_array_property(&executed, "store_traces")
            .expect("executed typed store traces exist")
            .length(),
        1
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn transient_fft_js_contract_uses_typed_numeric_columns_and_explicit_null() {
    let (_, mut snapshot) = fft_parity_fixture();
    snapshot.fft_results[1].metrics = None;
    let serialized = serialize_transient_to_js(&snapshot).expect("serialize browser FFT DTO");
    let fft_results = js_property(&serialized, "fft_results")
        .expect("FFT result collection exists")
        .dyn_into::<js_sys::Array>()
        .expect("FFT result collection is an array");
    let first = fft_results.get(0);
    let bins = js_property(&first, "bins").expect("FFT bin object exists");
    assert!(
        js_property(&bins, "indices")
            .expect("FFT indices exist")
            .is_instance_of::<js_sys::Uint32Array>()
    );
    for field in [
        "frequencies",
        "real",
        "imaginary",
        "magnitudes",
        "phase_degrees",
    ] {
        assert!(
            js_property(&bins, field)
                .expect("FFT numeric column exists")
                .is_instance_of::<js_sys::Float64Array>()
        );
    }

    let metrics = js_property(&first, "metrics").expect("FFT metrics property exists");
    let harmonics =
        js_property(&metrics, "largest_harmonics").expect("FFT ranked harmonic object exists");
    assert!(
        js_property(&harmonics, "ranks")
            .expect("FFT harmonic ranks exist")
            .is_instance_of::<js_sys::Uint32Array>()
    );
    assert!(
        js_property(&harmonics, "magnitudes")
            .expect("FFT harmonic magnitudes exist")
            .is_instance_of::<js_sys::Float64Array>()
    );

    let second = fft_results.get(1);
    assert!(
        js_property(&second, "metrics")
            .expect("FFT metrics property exists")
            .is_null()
    );

    let decoded: TransientSnapshot = serde_wasm_bindgen::from_value(serialized)
        .expect("typed-array FFT contract round-trips to its Rust DTO");
    assert_eq!(decoded, snapshot);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn result_document_windows_use_typed_numeric_and_validity_columns() {
    let document = run_ac_document_detailed(TYPED_DOCUMENT_DECK, &[1.0, 10.0])
        .expect("typed AC document executes under wasm32");
    let window = document.window(0, 2, 128).expect("bounded window exists");
    let serialized = serialize_result_window_to_js(&window).expect("serialize typed result window");

    let axes = js_array_property(&serialized, "axes").expect("axis collection exists");
    assert!(
        js_property(&axes.get(0), "values")
            .expect("axis values exist")
            .is_instance_of::<js_sys::Float64Array>()
    );
    let signals = js_array_property(&serialized, "signals").expect("signal collection exists");
    let values = js_property(&signals.get(0), "values").expect("signal values exist");
    assert!(
        js_property(&values, "real")
            .expect("complex real values exist")
            .is_instance_of::<js_sys::Float64Array>()
    );
    assert!(
        js_property(&values, "imaginary")
            .expect("complex imaginary values exist")
            .is_instance_of::<js_sys::Float64Array>()
    );
    assert!(
        js_property(&values, "validity")
            .expect("validity values exist")
            .is_instance_of::<js_sys::Uint8Array>()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn stb_windows_use_typed_columns_and_explicit_optional_nyquist() {
    let document = run_stb_document_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Linear,
        4,
        10.0,
        1.0e3,
        true,
    )
    .expect("typed STB document executes under wasm32");
    let window = document
        .window(0, 2, 128)
        .expect("bounded STB window exists");
    let serialized =
        serialize_stb_result_window_to_js(&window).expect("serialize typed STB window");

    let primary = js_property(&serialized, "primary").expect("primary STB group exists");
    let primary_gain = js_property(&primary, "loopGain").expect("primary loop gain exists");
    for (object, fields) in [
        (&primary, &["frequencies"][..]),
        (&primary_gain, &["real", "imaginary"][..]),
    ] {
        for field in fields {
            assert!(
                js_property(object, field)
                    .expect("primary STB numeric field exists")
                    .is_instance_of::<js_sys::Float64Array>()
            );
        }
    }
    let bode = js_property(&serialized, "bode").expect("Bode STB group exists");
    for field in ["frequencies", "magnitudes", "magnitudesDb", "phaseDegrees"] {
        assert!(
            js_property(&bode, field)
                .expect("Bode numeric field exists")
                .is_instance_of::<js_sys::Float64Array>()
        );
    }
    let nyquist = js_property(&serialized, "nyquist").expect("Nyquist group exists");
    for field in ["real", "imaginary", "frequencies"] {
        assert!(
            js_property(&nyquist, field)
                .expect("Nyquist numeric field exists")
                .is_instance_of::<js_sys::Float64Array>()
        );
    }

    let without_nyquist = run_stb_document_detailed(
        STB_DOCUMENT_DECK,
        "VPROBE",
        WasmStbSweep::Linear,
        2,
        10.0,
        100.0,
        false,
    )
    .expect("STB without Nyquist executes under wasm32");
    let serialized = serialize_stb_result_window_to_js(
        &without_nyquist
            .window(0, 2, 128)
            .expect("bounded non-Nyquist STB window exists"),
    )
    .expect("serialize non-Nyquist STB window");
    assert!(
        js_property(&serialized, "nyquist")
            .expect("Nyquist optionality is explicit")
            .is_null()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn public_stb_export_returns_structured_metadata_and_typed_windows() {
    let handle = run_stb_document_js(
        STB_DOCUMENT_DECK,
        "VPROBE",
        "linear",
        4,
        10.0,
        1.0e3,
        true,
        1,
        JsValue::UNDEFINED,
    )
    .expect("public STB export executes");

    assert_eq!(handle.analysis_id(), "stb-001");
    assert_eq!(handle.point_count(), 4);
    let metadata = handle.metadata_js().expect("public metadata serializes");
    assert_eq!(
        js_property(&metadata, "schema")
            .expect("metadata schema")
            .as_string()
            .as_deref(),
        Some(STB_RESULT_SCHEMA)
    );
    assert!(
        js_property(&metadata, "margins")
            .expect("metadata margins")
            .is_object()
    );

    let window = handle
        .read_window_js(0, 2)
        .expect("public bounded window serializes");
    let primary = js_property(&window, "primary").expect("public primary series");
    assert!(
        js_property(&primary, "frequencies")
            .expect("public primary frequencies")
            .is_instance_of::<js_sys::Float64Array>()
    );
    let nyquist = js_property(&window, "nyquist").expect("public Nyquist series");
    assert!(
        js_property(&nyquist, "real")
            .expect("public Nyquist real values")
            .is_instance_of::<js_sys::Float64Array>()
    );
}
