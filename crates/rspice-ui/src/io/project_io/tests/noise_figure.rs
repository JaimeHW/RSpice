//! Noise-figure settings and physical result references survive project storage.

use super::*;

#[test]
fn sampled_pnoise_studio_results_round_trip_and_reject_changed_geometry() {
    use rspice_core::analysis::pnoise::{
        PeriodicNoiseSamplePoint, PeriodicNoiseSampling, PeriodicNoiseSamplingEvidence,
    };
    let sampling = PeriodicNoiseSamplingEvidence {
        carrier_frequency_hz: 1000.0,
        request: PeriodicNoiseSampling::Edge {
            edge: Default::default(),
        },
        output: PeriodicNoiseSamplePoint {
            node: "out".into(),
            reference: None,
            phase_degrees: 0.0,
            voltage: 0.0,
            slew_volts_per_second: 1000.0,
        },
        reference: None,
        nominal_delay_seconds: None,
    };
    let mut run = SimulationRun::new(1);
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Pnoise, "Sampled noise")
            .with_waveforms(vec![
                crate::state::WaveformData::new(
                    "onoise",
                    vec![10.0, 100.0],
                    vec![1e-20, 1e-20],
                    "#fff",
                )
                .with_unit("s²/Hz"),
            ])
            .with_noise_summary(NoiseSummary {
                conversion: Some(crate::state::PeriodicNoiseConversionEvidence {
                    sampling: Some(sampling.clone()),
                    input_source: String::new(),
                    carrier_hz: 1000.0,
                    input_sideband: 0,
                    output_sideband: 0,
                    max_sideband: 1,
                }),
                band: (10.0, 100.0),
                ..Default::default()
            }),
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
            .conversion
            .as_ref()
            .unwrap()
            .sampling,
        Some(sampling)
    );
    for path in ["phase_degrees", "voltage", "slew_volts_per_second"] {
        let mut changed: serde_json::Value = serde_json::from_str(&json).unwrap();
        changed["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]["conversion"]["sampling"]
            ["output"][path] = serde_json::json!(123.0);
        assert!(
            load_project_text(&changed.to_string(), None)
                .unwrap()
                .simulation_results
                .runs
                .is_empty(),
            "{path}"
        );
    }
    let mut old: serde_json::Value = serde_json::from_str(&json).unwrap();
    old["simulation_results"]["schema_version"] = serde_json::json!(39);
    assert!(
        load_project_text(&old.to_string(), None)
            .unwrap()
            .simulation_results
            .runs
            .is_empty()
    );
}

#[test]
fn hbnoise_reference_results_survive_project_load_and_reject_tampering_and_old_schema() {
    for conversion in [
        None,
        Some(crate::state::PeriodicNoiseConversionEvidence {
            sampling: None,
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
            input_quantity: None,
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

#[test]
fn noise_input_units_survive_projects_and_reject_unit_tampering() {
    use rspice_core::analysis::noise::NoiseInputQuantity;
    for quantity in [
        None,
        Some(NoiseInputQuantity::Voltage),
        Some(NoiseInputQuantity::Current),
    ] {
        let summary = NoiseSummary {
            input_quantity: quantity,
            input_rms: Some(3e-9),
            band: (1e3, 1e4),
            ..Default::default()
        };
        let mut run = SimulationRun::new(1);
        run.mark_running().unwrap();
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        let mut wave =
            crate::state::WaveformData::new("inoise", vec![1e3, 1e4], vec![1e-21, 1e-21], "#fff");
        if let Some(quantity) = quantity {
            wave = wave.with_unit(quantity.density_unit());
        }
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Noise, "NOISE")
                .with_waveforms(vec![wave])
                .with_noise_summary(summary.clone()),
        );
        seal_legacy_unattributed(&mut run);
        let digest = run.analyses[0].result_data_digest();
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
            restored.active_analysis().unwrap().noise_summary.as_ref(),
            Some(&summary)
        );
        assert_eq!(
            restored.active_analysis().unwrap().result_data_digest(),
            digest
        );
        let mut old: serde_json::Value = serde_json::from_str(&json).unwrap();
        old["simulation_results"]["schema_version"] = 33.into();
        let migrated = load_project_text(&old.to_string(), None).unwrap();
        assert_eq!(
            migrated.simulation_results.runs.is_empty(),
            quantity.is_some()
        );
        if quantity.is_none() {
            assert_eq!(
                migrated
                    .simulation_results
                    .into_simulation_state()
                    .unwrap()
                    .active_analysis()
                    .unwrap()
                    .result_data_digest(),
                digest
            );
        }
        if quantity.is_some() {
            let mut changed: serde_json::Value = serde_json::from_str(&json).unwrap();
            changed["simulation_results"]["runs"][0]["analyses"][0]["noise_summary"]["input_quantity"] =
                serde_json::to_value(if quantity == Some(NoiseInputQuantity::Current) {
                    NoiseInputQuantity::Voltage
                } else {
                    NoiseInputQuantity::Current
                })
                .unwrap();
            assert!(
                load_project_text(&changed.to_string(), None)
                    .unwrap()
                    .simulation_results
                    .runs
                    .is_empty()
            );
        }
    }
}
