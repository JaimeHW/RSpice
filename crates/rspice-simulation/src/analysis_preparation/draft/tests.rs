//! Portable authored controls reach the exact engine request and retained result.

use crate::analysis_preparation::{AnalysisInputs, analysis_draft_spec};
use crate::preparation::QueuedAnalysis;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::run::SimulationRun;
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

fn spec_for_draft(draft: &AnalysisDraft) -> Result<AnalysisSpec, String> {
    let inputs: AnalysisInputs<'_, SimulationRun, AnalysisResult> = AnalysisInputs {
        sim_setup: &Default::default(),
        schematic: &Default::default(),
        selected_components: &Default::default(),
        runs: &[],
        active_run: None,
        project_revision: rspice_app_types::product::ObjectRevision::INITIAL,
        plan_payloads: &[],
    };
    analysis_draft_spec(&inputs, draft)
}

#[test]
fn studio_ac_data_authored_columns_and_netlist_tables_reach_results() {
    use crate::results::SimulationResult;
    use rspice_simulation_contract::config::AcDataTableOptions;
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    use rspice_simulation_contract::{analysis_draft::AnalysisDraft, drafts::AcDataDraft};

    let mut draft = AcDataDraft {
        table_name: "points:1".into(),
        frequencies: "1k, 0, 1k".into(),
        ..Default::default()
    };
    for (name, values) in [("load", "1k, 2k, 500"), ("R1:R", "1k, 1k, 2k")] {
        draft.parameter_columns.push(Default::default());
        let column = draft.parameter_columns.last_mut().unwrap();
        column.name = name.into();
        column.values = values.into();
    }
    let spec_for = |draft: &AcDataDraft| {
        let json = serde_json::to_value(draft).unwrap();
        let restored: AcDataDraft = serde_json::from_value(json.clone()).unwrap();
        assert_eq!(serde_json::to_value(&restored).unwrap(), json);
        let spec = spec_for_draft(&AnalysisDraft::AcData(restored)).unwrap();
        spec.validate().unwrap();
        let worker = WorkerAnalysisSpec::from(&spec);
        let worker: WorkerAnalysisSpec =
            serde_json::from_value(serde_json::to_value(worker).unwrap()).unwrap();
        assert_eq!(AnalysisSpec::from(worker), spec);
        spec
    };
    for name in ["2026-study", "bad name", "500", "pts+tail", "$pts"] {
        let mut invalid = draft.clone();
        invalid.table_name = name.into();
        assert!(invalid.to_config().is_err(), "accepted table name {name}");
    }
    let authored = spec_for(&draft);
    let cards = crate::analysis_preparation::build_ac_data_command(&authored).unwrap();
    let source = format!(
        "AC row controls\n.param load=900\nV1 in 0 AC 1\nR1 in out 900\nR2 out 0 {{load}}\n{cards}\n.end\n"
    );
    for from_netlist in [false, true] {
        let mut selected = draft.clone();
        selected.from_netlist = from_netlist;
        if from_netlist {
            selected.frequencies = "unfinished frequency".into();
            selected.parameter_columns[0].values = "unfinished value".into();
        }
        let spec = spec_for(&selected);
        let command = crate::analysis_preparation::build_ac_data_command(&spec).unwrap();
        if from_netlist {
            assert_eq!(command, ".ac DATA=points:1");
        }
        let run = crate::runner::pvt_point_evidence::run_declaration(
            &source,
            "AC table",
            QueuedAnalysis {
                spec,
                config: None,
                spec_options: Default::default(),
                analysis_line: command,
                numeric_override: None,
            },
            27.0,
            crate::execution::SavePolicy::RetainEngineProducedResults,
            &[],
        )
        .unwrap();
        assert_eq!(run.analyses.len(), 1);
        let analysis = &run.analyses[0];
        assert!(analysis.success, "{:?}", analysis.error_message);
        let output = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name.eq_ignore_ascii_case("|V(out)|"))
            .unwrap();
        assert_eq!(
            output.x.iter().copied().collect::<Vec<_>>(),
            [1000.0, 0.0, 1000.0]
        );
        for (actual, expected) in output.y.iter().zip([0.5, 2.0 / 3.0, 0.2]) {
            assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
        }
    }

    // Direct worker requests carry complete columns even without generated cards.
    let AnalysisSpec::AcData {
        table_name,
        frequencies,
        table_options,
    } = authored
    else {
        unreachable!()
    };
    let bare = "AC direct\n.param load=900\nV1 in 0 AC 1\nR1 in out 900\nR2 out 0 {load}\n.end\n";
    let bridge = crate::engine_bridge::EngineBridge::new();
    let run = |source: &str, frequencies: Vec<f64>, options: &AcDataTableOptions| {
        bridge.run_ac_data_with_source_path(
            source,
            None,
            &table_name,
            frequencies,
            options,
            &rspice_core::NoAbort,
        )
    };
    let SimulationResult::Ac {
        waveforms,
        frequencies: actual_axis,
        ..
    } = run(bare, frequencies.clone(), &table_options).unwrap()
    else {
        panic!("AC result")
    };
    assert_eq!(actual_axis, frequencies);
    for (actual, expected) in waveforms["V(OUT)"]
        .y_values
        .iter()
        .zip([0.5, 2.0 / 3.0, 0.2])
    {
        assert!((actual - expected).abs() < 1e-10);
    }
    let mut mismatch = table_options.clone();
    mismatch.parameter_columns[0].values[0] = 2000.0;
    assert!(
        run(&source, frequencies.clone(), &mismatch)
            .unwrap_err()
            .to_string()
            .contains("configured parameter values")
    );
    let reference = AcDataTableOptions {
        from_netlist: true,
        ..Default::default()
    };
    assert!(
        run(bare, Vec::new(), &reference)
            .unwrap_err()
            .to_string()
            .contains("unknown .DATA table")
    );
    for (name, values) in [
        ("HERTZ", "1 2 3"),
        ("LOAD", "1 2 3"),
        ("bad name", "1 2 3"),
        ("extra", "1 2"),
    ] {
        let mut invalid = draft.clone();
        invalid.parameter_columns.push(Default::default());
        let column = invalid.parameter_columns.last_mut().unwrap();
        column.name = name.into();
        column.values = values.into();
        assert!(invalid.to_config().is_err(), "{name}: {values}");
    }
}

#[test]
fn transient_noise_seed_inheritance_and_zero_scale_reach_the_solver() {
    use rspice_simulation_contract::worker_spec::WorkerAnalysisSpec;
    use rspice_simulation_contract::{analysis_draft::AnalysisDraft, drafts::TransientNoiseDraft};

    let draft = |seed: &str, scale: &str| TransientNoiseDraft {
        stop_time: "64n".into(),
        step_time: "1n".into(),
        start_time: "0".into(),
        max_step: "1n".into(),
        seed: seed.into(),
        noise_fmax: "1G".into(),
        noise_fmin: "1M".into(),
        scale: scale.into(),
        use_initial_conditions: false,
    };
    let run = |seed: &str, scale: &str| {
        let json = serde_json::to_value(draft(seed, scale)).unwrap();
        let restored: TransientNoiseDraft = serde_json::from_value(json.clone()).unwrap();
        let mut restored = AnalysisDraft::TransientNoise(restored);
        restored.prepare_after_restore();
        assert!(restored.manifest_configuration_error().is_none());
        let AnalysisDraft::TransientNoise(settings) = &restored else {
            unreachable!()
        };
        assert_eq!(serde_json::to_value(settings).unwrap(), json);
        let spec = spec_for_draft(&restored).unwrap();
        spec.validate().unwrap();
        let wire = WorkerAnalysisSpec::from(&spec);
        let wire: WorkerAnalysisSpec =
            serde_json::from_value(serde_json::to_value(wire).unwrap()).unwrap();
        let restored = AnalysisSpec::from(wire);
        assert_eq!(spec, restored);
        let card = crate::analysis_preparation::build_transient_noise_command(&restored).unwrap();
        assert_eq!(card.contains("NOISESEED="), !seed.trim().is_empty());
        let source = format!(
            "transient noise controls\n.options seed=0\nV1 in 0 1\nR1 in out 10k\nR2 out 0 10k\n{card}\n.end\n"
        );
        let parsed = rspice_core::Netlist::parse(&source).unwrap();
        let noise = parsed.options.transient_noise.unwrap();
        assert_eq!(noise.seed, settings.parsed_seed().unwrap());
        assert_eq!(noise.scale, scale.parse::<f64>().unwrap());
        let result = crate::runner::pvt_point_evidence::run_declaration(
            &source,
            "Transient noise",
            QueuedAnalysis {
                spec: restored,
                config: None,
                spec_options: Default::default(),
                analysis_line: card,
                numeric_override: None,
            },
            27.0,
            crate::execution::SavePolicy::RetainEngineProducedResults,
            &[],
        )
        .unwrap();
        assert_eq!(result.analyses.len(), 1);
        let analysis = &result.analyses[0];
        assert!(analysis.success, "{:?}", analysis.error_message);
        assert_eq!(analysis.analysis_type, AnalysisType::TransientNoise);
        let output = analysis
            .waveforms
            .iter()
            .find(|waveform| {
                waveform.name.eq_ignore_ascii_case("out")
                    || waveform.name.eq_ignore_ascii_case("V(out)")
            })
            .unwrap();
        let time = output.x.iter().copied().collect::<Vec<_>>();
        let values = output.y.iter().copied().collect::<Vec<_>>();
        assert!(!time.is_empty());
        assert_eq!(time.len(), values.len());
        assert!(values.iter().all(|value| value.is_finite()));
        (time, values)
    };

    let inherited = run("", "1");
    let explicit_zero = run("0", "1");
    let another_seed = run("1", "1");
    let deterministic = run("0", "0");
    let doubled = run("0", "2");
    assert_eq!(
        inherited, explicit_zero,
        "blank seed inherits .OPTIONS SEED=0"
    );
    assert_ne!(
        explicit_zero.1, another_seed.1,
        "zero is a real random seed"
    );
    assert_eq!(explicit_zero.0, doubled.0);
    assert_eq!(explicit_zero.0, deterministic.0);
    assert!(
        explicit_zero
            .1
            .iter()
            .any(|value| (value - 0.5).abs() > 1e-8)
    );
    for ((single, double), silent) in explicit_zero.1.iter().zip(&doubled.1).zip(&deterministic.1) {
        assert!(
            (silent - 0.5).abs() < 1e-10,
            "zero scale preserves the DC divider solution"
        );
        assert!(((double - silent) - 2.0 * (single - silent)).abs() < 1e-10);
    }

    for seed in ["", "0", "18446744073709551615"] {
        assert!(spec_for_draft(&AnalysisDraft::TransientNoise(draft(seed, "0"))).is_ok());
    }
    for seed in ["-1", "1.5", "18446744073709551616", "unfinished"] {
        assert!(
            spec_for_draft(&AnalysisDraft::TransientNoise(draft(seed, "1"))).is_err(),
            "{seed}"
        );
    }
    for scale in ["-1", "NaN", "inf"] {
        assert!(
            spec_for_draft(&AnalysisDraft::TransientNoise(draft("0", scale))).is_err(),
            "{scale}"
        );
    }
}
