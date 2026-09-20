//! Noise-figure settings and physical result references survive project storage.

use super::*;

#[test]
fn hbnoise_reference_results_survive_project_load_and_reject_tampering_and_old_schema() {
    for conversion in [
        None,
        Some(crate::state::PeriodicNoiseConversionEvidence {
            input_source: "V1".into(),
            carrier_hz: 1e6,
            input_sideband: 1,
            output_sideband: -1,
            max_sideband: 4,
        }),
    ] {
        let figure = std::sync::Arc::new(crate::state::NoiseFigureEvidence {
            input_source: "V1".into(),
            source_resistor: "Rs".into(),
            source_resistance_ohm: 75.0,
            source_temperature_kelvin: 310.0,
            reference_temperature_kelvin: 290.0,
            frequencies: vec![1e3, 1e4],
            decibels: vec![2.0, 3.0],
        });
        let summary = NoiseSummary {
            conversion: conversion.clone(),
            noise_figure: Some(figure.clone()),
            band: (1e3, 1e4),
            ..Default::default()
        };
        let mut run = SimulationRun::new(1);
        run.mark_running().unwrap();
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE")
                .with_waveforms(vec![
                    crate::state::WaveformData::new(
                        "Noise figure (SSB)",
                        figure.frequencies.clone(),
                        figure.decibels.clone(),
                        "#fff",
                    )
                    .with_unit("dB"),
                ])
                .with_noise_summary(summary),
        );
        seal_legacy_unattributed(&mut run);
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run].into();
        simulation.next_run_id = 1;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let json = serialize_project_file(&project).unwrap();
        let restored = load_project_text(&json, None)
            .unwrap()
            .simulation_results
            .into_simulation_state()
            .unwrap();
        assert_eq!(
            restored
                .active_analysis()
                .unwrap()
                .noise_summary
                .as_ref()
                .unwrap()
                .conversion,
            conversion
        );
        assert_eq!(
            restored
                .active_analysis()
                .unwrap()
                .noise_summary
                .as_ref()
                .unwrap()
                .noise_figure
                .as_ref(),
            Some(&figure)
        );
        for field in ["source_resistance_ohm", "reference_temperature_kelvin"] {
            let mut tampered: serde_json::Value = serde_json::from_str(&json).unwrap();
            tampered["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]["noise_figure"]
                [field] = serde_json::json!(500.0);
            let rejected = load_project_text(&tampered.to_string(), None).unwrap();
            assert!(rejected.simulation_results.runs.is_empty());
            assert!(rejected.simulation_results_warning.is_some());
        }
        if conversion.is_some() {
            for (field, value) in [
                ("carrier_hz", serde_json::json!(2e6)),
                ("input_source", serde_json::json!("V2")),
                ("input_sideband", serde_json::json!(0)),
                ("output_sideband", serde_json::json!(0)),
                ("max_sideband", serde_json::json!(5)),
            ] {
                let mut tampered: serde_json::Value = serde_json::from_str(&json).unwrap();
                tampered["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]["conversion"]
                    [field] = value;
                let rejected = load_project_text(&tampered.to_string(), None).unwrap();
                assert!(rejected.simulation_results.runs.is_empty(), "{field}");
                assert!(rejected.simulation_results_warning.is_some());
            }
        }
        let mut v32: serde_json::Value = serde_json::from_str(&json).unwrap();
        v32["simulation_results"]["schema_version"] = serde_json::json!(32);
        let migrated = load_project_text(&v32.to_string(), None).unwrap();
        assert_eq!(
            migrated.simulation_results.runs.is_empty(),
            conversion.is_some()
        );
        let mut old: serde_json::Value = serde_json::from_str(&json).unwrap();
        old["simulation_results"]["schema_version"] = serde_json::json!(31);
        let rejected = load_project_text(&old.to_string(), None).unwrap();
        assert!(rejected.simulation_results.runs.is_empty());
        assert!(
            rejected
                .simulation_results_warning
                .unwrap()
                .contains(if conversion.is_some() {
                    "before v33"
                } else {
                    "before v32"
                })
        );
    }
}
