use super::*;
use std::path::PathBuf;

fn write_periodic_include_fixture(
    source_file_name: &str,
    include_file_name: &str,
    include_contents: &str,
    netlist: &str,
) -> (tempfile::TempDir, PathBuf, String) {
    let temp_dir = tempfile::tempdir().expect("temp dir should be created");
    let source_path = temp_dir.path().join("project").join(source_file_name);
    std::fs::create_dir_all(
        source_path
            .parent()
            .expect("source path should have parent directory"),
    )
    .expect("project directory should be created");
    std::fs::create_dir_all(temp_dir.path().join("models"))
        .expect("models directory should be created");
    std::fs::write(
        temp_dir.path().join("models").join(include_file_name),
        include_contents,
    )
    .expect("include file should be written");

    (temp_dir, source_path, netlist.to_string())
}

#[test]
fn test_run_pac_analysis_executes_for_driven_rc() {
    let netlist = "* pac\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PacRunConfig {
        pss_fundamental_freq: 1e6,
        pss_num_harmonics: 8,
        pss_tolerance: 1e-4,
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: PacFrequencySweep::Decade,
        max_sideband: 2,
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        output_ref: None,
        pac_magnitude: 0.5,
        include_dc: true,
        reltol: 1e-3,
        abstol: 1e-12,
    };

    let result = run_pac_analysis(netlist, &cfg).expect("PAC analysis should execute");
    assert!(result.converged);
    assert!(!result.frequencies.is_empty());
    assert!(result.sidebands.contains(&0));
    assert_eq!(result.sidebands, vec![-2, -1, 0, 1, 2]);
    assert_eq!(result.spectra.len(), result.sidebands.len());
    assert!(
        result.spectra.iter().all(|(_, spectrum)| {
            spectrum.len() == result.frequencies.len()
                && spectrum
                    .iter()
                    .all(|(f, mag, phase)| f.is_finite() && mag.is_finite() && phase.is_finite())
        }),
        "expected finite PAC spectra at all sweep points"
    );
}

#[test]
fn test_run_pac_analysis_rejects_empty_sideband_configuration() {
    let netlist = "* pac invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PacRunConfig {
        max_sideband: 0,
        include_dc: false,
        output_node: "out".to_string(),
        input_source: "V1".to_string(),
        ..PacRunConfig::default()
    };

    let err = run_pac_analysis(netlist, &cfg)
        .expect_err("PAC without any enabled sidebands should be rejected");
    assert!(err.contains("at least one sideband"));
}

#[test]
fn test_run_pac_analysis_auto_infers_source_and_output() {
    let netlist = "* pac auto\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let result =
        run_pac_analysis_auto(netlist).expect("PAC auto mode should infer IO for simple RC");
    assert!(result.converged);
    assert!(!result.frequencies.is_empty());
    assert!(!result.spectra.is_empty());
}

#[test]
fn test_run_pac_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pac_top.rsch",
        "pac_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pac include fixture\n\
V1 in 0 1\n\
.include ../models/pac_stage.inc\n\
.end\n",
    );
    let cfg = PacRunConfig {
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        ..PacRunConfig::default()
    };

    let without_source = run_pac_analysis(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pac_analysis_with_source_path(&netlist, &cfg, Some(&source_path))
        .expect("source-aware PAC analysis should resolve include");
    assert!(with_source.converged);
    assert!(!with_source.frequencies.is_empty());
}

#[test]
fn test_run_pac_analysis_auto_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pac_auto_top.rsch",
        "pac_auto_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pac auto include fixture\n\
V1 in 0 1\n\
.include ../models/pac_auto_stage.inc\n\
.end\n",
    );

    let without_source = run_pac_analysis_auto(&netlist);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pac_analysis_auto_with_source_path(&netlist, Some(&source_path))
        .expect("source-aware PAC auto analysis should resolve include");
    assert!(with_source.converged);
    assert!(!with_source.spectra.is_empty());
}

#[test]
fn test_run_pxf_analysis_with_config_executes_for_driven_rc() {
    let netlist = "* pxf\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PxfRunConfig {
        pss_fundamental_freq: 1e6,
        pss_num_harmonics: 8,
        pss_tolerance: 1e-4,
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: PxfFrequencySweep::Decade,
        input_source: "V1".to_string(),
        input_sideband: 1,
        output_node: "out".to_string(),
        output_ref: None,
        output_sideband: 1,
        max_sideband: 3,
        reltol: 1e-3,
        abstol: 1e-12,
    };

    let result = run_pxf_analysis_with_config(netlist, &cfg).expect("PXF analysis should execute");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.frequencies.len(), result.transfer.len());
    assert_eq!(result.frequencies.len(), result.magnitude_db.len());
    assert_eq!(result.frequencies.len(), result.phase_deg.len());
    assert_eq!(result.frequencies.len(), result.output_frequencies.len());
    assert_eq!(result.input_sideband, 1);
    assert_eq!(result.output_sideband, 1);
    assert!(result.output_label.starts_with("V("));
    assert!(
        result
            .transfer
            .iter()
            .all(|value| value.re.is_finite() && value.im.is_finite())
    );
    assert!(result.magnitude_db.iter().all(|value| value.is_finite()));
    assert!(result.phase_deg.iter().all(|value| value.is_finite()));
}

#[test]
fn test_run_pxf_analysis_with_config_rejects_sideband_out_of_range() {
    let netlist = "* pxf invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PxfRunConfig {
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        max_sideband: 0,
        input_sideband: 1,
        output_sideband: 1,
        ..PxfRunConfig::default()
    };

    let err = run_pxf_analysis_with_config(netlist, &cfg)
        .expect_err("sideband outside configured range should fail");
    assert!(err.contains("sideband"));
}

#[test]
fn test_run_pxf_analysis_with_config_supports_differential_reference_node() {
    let netlist = "* pxf ref\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PxfRunConfig {
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        ..PxfRunConfig::default()
    };

    let result = run_pxf_analysis_with_config(netlist, &cfg)
        .expect("differential output reference should execute");
    assert_eq!(result.frequencies.len(), result.transfer.len());
    assert!(result.output_label.contains("out"));
    assert!(result.output_label.contains("in"));
}

#[test]
fn test_run_pxf_analysis_with_config_rejects_identical_output_and_reference_nodes() {
    let netlist = "* pxf invalid ref\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PxfRunConfig {
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        output_ref: Some("out".to_string()),
        ..PxfRunConfig::default()
    };

    let err = run_pxf_analysis_with_config(netlist, &cfg)
        .expect_err("matching output and reference nodes should fail");
    assert!(err.contains("cannot be the same"));
}

#[test]
fn test_run_pxf_analysis_auto_infers_output_node() {
    let netlist = "* pxf auto\nVDRIVE in 0 1\nR1 in out 2k\nC1 out 0 2n\n.end\n";
    let result = run_pxf_analysis(netlist).expect("PXF auto mode should infer IO");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.frequencies.len(), result.transfer.len());
    assert!(result.output_label.to_ascii_uppercase().contains("OUT"));
}

#[test]
fn test_run_pxf_analysis_with_config_and_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pxf_top.rsch",
        "pxf_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pxf include fixture\n\
V1 in 0 1\n\
.include ../models/pxf_stage.inc\n\
.end\n",
    );
    let cfg = PxfRunConfig {
        input_source: "V1".to_string(),
        output_node: "out".to_string(),
        ..PxfRunConfig::default()
    };

    let without_source = run_pxf_analysis_with_config(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source =
        run_pxf_analysis_with_config_and_source_path(&netlist, &cfg, Some(&source_path))
            .expect("source-aware PXF analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(with_source.frequencies.len(), with_source.transfer.len());
}

#[test]
fn test_run_pxf_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pxf_auto_top.rsch",
        "pxf_auto_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pxf auto include fixture\n\
V1 in 0 1\n\
.include ../models/pxf_auto_stage.inc\n\
.end\n",
    );

    let without_source = run_pxf_analysis(&netlist);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pxf_analysis_with_source_path(&netlist, Some(&source_path))
        .expect("source-aware PXF auto analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert!(
        with_source
            .output_label
            .to_ascii_uppercase()
            .contains("OUT")
    );
}

#[test]
fn test_run_pnoise_analysis_with_config_executes_output_referred() {
    let netlist = "* pnoise\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        pss_fundamental_freq: 1e6,
        pss_num_harmonics: 8,
        pss_tolerance: 1e-4,
        start_freq: 10.0,
        stop_freq: 1e6,
        points_per_unit: 6,
        sweep: PnoiseFrequencySweep::Decade,
        max_sideband: 3,
        output_node: "out".to_string(),
        output_ref: None,
        input_source: "V1".to_string(),
        noise_ref: PnoiseReference::Output,
        integrated_noise: true,
        noise_summary: true,
        reltol: 1e-3,
        abstol: 1e-18,
    };

    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("PNOISE output-referred analysis should execute");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.output_noise.len(), result.frequencies.len());
    assert_eq!(result.reference, PnoiseReference::Output);
    assert_eq!(result.sideband_factor, 7);
    assert!(
        result
            .total_output_noise
            .is_some_and(|value| value.is_finite() && value >= 0.0)
    );
}

#[test]
fn test_run_pnoise_analysis_with_phase_reference_produces_dbc() {
    let netlist = "* pnoise phase\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        noise_ref: PnoiseReference::Phase,
        max_sideband: 2,
        ..PnoiseRunConfig::default()
    };
    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("PNOISE phase-noise mode should execute");
    assert_eq!(result.reference, PnoiseReference::Phase);
    assert_eq!(result.output_noise.len(), result.frequencies.len());
    assert!(result.output_noise.iter().all(|value| value.is_finite()));
}

#[test]
fn test_run_pnoise_analysis_respects_netlist_temperature_options() {
    let netlist_cold = "* pnoise cold\n.OPTIONS TEMP=-40\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\nC1 out 0 1n\n.end\n";
    let netlist_hot = "* pnoise hot\n.OPTIONS TEMP=125\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        noise_ref: PnoiseReference::Output,
        start_freq: 1e3,
        stop_freq: 1e3,
        points_per_unit: 1,
        sweep: PnoiseFrequencySweep::Linear,
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };

    let cold = run_pnoise_analysis_with_config(netlist_cold, &cfg)
        .expect("cold-temperature PNOISE should execute");
    let hot = run_pnoise_analysis_with_config(netlist_hot, &cfg)
        .expect("hot-temperature PNOISE should execute");

    let cold_psd = *cold
        .output_noise
        .first()
        .expect("cold PNOISE run should contain one noise point");
    let hot_psd = *hot
        .output_noise
        .first()
        .expect("hot PNOISE run should contain one noise point");
    assert!(cold_psd.is_finite() && cold_psd > 0.0);
    assert!(hot_psd.is_finite() && hot_psd > cold_psd);
    let ratio = hot_psd / cold_psd;
    assert!(
        ratio > 1.4,
        "expected hot/cold output-noise ratio to reflect temperature scaling, got {}",
        ratio
    );
}

#[test]
fn test_run_pnoise_analysis_input_reference_matches_core_input_referred_density() {
    let netlist = "* pnoise input\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let output_cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        input_source: "V1".to_string(),
        noise_ref: PnoiseReference::Output,
        start_freq: 10.0,
        stop_freq: 1e7,
        points_per_unit: 8,
        sweep: PnoiseFrequencySweep::Decade,
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };
    let input_cfg = PnoiseRunConfig {
        noise_ref: PnoiseReference::Input,
        ..output_cfg.clone()
    };

    let output_result = run_pnoise_analysis_with_config(netlist, &output_cfg)
        .expect("output-referred PNOISE should execute");
    let input_result = run_pnoise_analysis_with_config(netlist, &input_cfg)
        .expect("input-referred PNOISE should execute");

    let input_curve = input_result
        .input_noise
        .as_ref()
        .expect("input-referred mode should return an input-noise vector");
    assert_eq!(input_curve.len(), output_result.output_noise.len());
    assert!(
        !input_result
            .warnings
            .iter()
            .any(|warning| warning.contains("TF fallback"))
    );
    assert!(
        !input_result
            .warnings
            .iter()
            .any(|warning| warning.contains("unity gain"))
    );

    let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
    let mut sim_config = build_engine_config(&parsed, None);
    sim_config.tolerance = input_cfg.pss_tolerance;
    let engine = Engine::new(sim_config);
    let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
    let out_idx = resolve_node_or_ground_index("out", &dc.node_names).expect("out must resolve");
    let ref_idx = resolve_node_or_ground_index("in", &dc.node_names).expect("in must resolve");
    let core_input = engine
        .run_noise_with_input_source(
            &parsed,
            out_idx,
            Some(ref_idx),
            "V1",
            &input_result.frequencies,
            300.0,
        )
        .expect("core input-referred noise run should execute");
    assert_eq!(core_input.len(), input_curve.len());
    for (idx, (core_point, ui_point)) in core_input.iter().zip(input_curve.iter()).enumerate() {
        let tol = 1e-24 + core_point.input_referred_density.abs() * 1e-9;
        assert!(
            (core_point.input_referred_density - *ui_point).abs() <= tol,
            "expected UI input-referred density to match core at idx {} (f={} Hz): core={}, ui={}",
            idx,
            core_point.frequency,
            core_point.input_referred_density,
            ui_point
        );
    }
}

#[test]
fn test_run_pnoise_analysis_sideband_output_matches_translated_core_sum() {
    let netlist = "* pnoise sideband output\nV1 in 0 1\nR1 in out 1k\nC1 out 0 2n\n.end\n";
    let cfg = PnoiseRunConfig {
        pss_fundamental_freq: 20e3,
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        noise_ref: PnoiseReference::Output,
        start_freq: 1e3,
        stop_freq: 9e3,
        points_per_unit: 7,
        sweep: PnoiseFrequencySweep::Linear,
        max_sideband: 2,
        ..PnoiseRunConfig::default()
    };
    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("sideband PNOISE output run should execute");

    let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
    let mut sim_config = build_engine_config(&parsed, None);
    sim_config.tolerance = cfg.pss_tolerance;
    let noise_temperature = sim_config.temperature;
    let engine = Engine::new(sim_config);
    let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
    let out_idx =
        resolve_node_or_ground_index("out", &dc.node_names).expect("out node should resolve");
    let ref_idx =
        resolve_node_or_ground_index("in", &dc.node_names).expect("ref node should resolve");
    let translated = build_pnoise_sideband_translated_frequencies(
        &result.frequencies,
        result.carrier_frequency,
        cfg.max_sideband,
    )
    .expect("translated sideband frequencies should be generated");
    let core = engine
        .run_noise_ports(
            &parsed,
            out_idx,
            Some(ref_idx),
            &translated,
            noise_temperature,
        )
        .expect("core sideband-translated noise run should execute");
    let expected = fold_sideband_samples(
        &core
            .iter()
            .map(|point| point.output_noise_density.max(0.0))
            .collect::<Vec<_>>(),
        result.frequencies.len(),
        result.sideband_factor,
        "output-referred",
    )
    .expect("folded output sideband sum should compute");

    assert_eq!(expected.len(), result.output_noise.len());
    for (idx, (expected_psd, actual_psd)) in
        expected.iter().zip(result.output_noise.iter()).enumerate()
    {
        let tol = 1e-24 + expected_psd.abs() * 1e-9;
        assert!(
            (expected_psd - actual_psd).abs() <= tol,
            "expected folded output PSD to match translated core sum at idx {}: expected={}, actual={}",
            idx,
            expected_psd,
            actual_psd
        );
    }
}

#[test]
fn test_run_pnoise_analysis_sideband_input_matches_translated_core_sum() {
    let netlist = "* pnoise sideband input\nV1 in 0 1\nR1 in out 1k\nC1 out 0 2n\n.end\n";
    let cfg = PnoiseRunConfig {
        pss_fundamental_freq: 20e3,
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        input_source: "V1".to_string(),
        noise_ref: PnoiseReference::Input,
        start_freq: 1e3,
        stop_freq: 9e3,
        points_per_unit: 7,
        sweep: PnoiseFrequencySweep::Linear,
        max_sideband: 2,
        ..PnoiseRunConfig::default()
    };
    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("sideband PNOISE input run should execute");
    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("TF fallback"))
    );

    let input_curve = result
        .input_noise
        .as_ref()
        .expect("input-referred run should produce input curve");
    let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
    let mut sim_config = build_engine_config(&parsed, None);
    sim_config.tolerance = cfg.pss_tolerance;
    let noise_temperature = sim_config.temperature;
    let engine = Engine::new(sim_config);
    let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
    let out_idx =
        resolve_node_or_ground_index("out", &dc.node_names).expect("out node should resolve");
    let ref_idx =
        resolve_node_or_ground_index("in", &dc.node_names).expect("ref node should resolve");
    let translated = build_pnoise_sideband_translated_frequencies(
        &result.frequencies,
        result.carrier_frequency,
        cfg.max_sideband,
    )
    .expect("translated sideband frequencies should be generated");
    let core = engine
        .run_noise_with_input_source(
            &parsed,
            out_idx,
            Some(ref_idx),
            "V1",
            &translated,
            noise_temperature,
        )
        .expect("core input-referred translated noise run should execute");
    let expected = fold_sideband_samples(
        &core
            .iter()
            .map(|point| point.input_referred_density.max(0.0))
            .collect::<Vec<_>>(),
        result.frequencies.len(),
        result.sideband_factor,
        "input-referred",
    )
    .expect("folded input sideband sum should compute");

    assert_eq!(expected.len(), input_curve.len());
    for (idx, (expected_psd, actual_psd)) in expected.iter().zip(input_curve.iter()).enumerate() {
        let tol = 1e-24 + expected_psd.abs() * 1e-9;
        assert!(
            (expected_psd - actual_psd).abs() <= tol,
            "expected folded input PSD to match translated core sum at idx {}: expected={}, actual={}",
            idx,
            expected_psd,
            actual_psd
        );
    }
}

#[test]
fn test_run_pnoise_analysis_sideband_contributor_percentages_are_normalized() {
    let netlist =
        "* pnoise sideband contributors\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        max_sideband: 2,
        noise_summary: true,
        ..PnoiseRunConfig::default()
    };
    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("sideband PNOISE contributor run should execute");
    assert!(!result.contributors.is_empty());
    let total_percentage: Value = result
        .contributors
        .iter()
        .map(|(_, percentage)| *percentage)
        .sum();
    assert!(
        (total_percentage - 100.0).abs() <= 1e-6,
        "expected sideband-folded contributor percentages to normalize to 100, got {}",
        total_percentage
    );
}

#[test]
fn test_run_pnoise_analysis_input_reference_rejects_unknown_input_source() {
    let netlist = "* pnoise input unknown source\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        noise_ref: PnoiseReference::Input,
        input_source: "V_NOT_PRESENT".to_string(),
        start_freq: 1e3,
        stop_freq: 1e5,
        points_per_unit: 3,
        sweep: PnoiseFrequencySweep::Decade,
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };

    let err = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect_err("input-referred PNOISE should reject unknown explicit input source");
    assert!(err.contains("V_NOT_PRESENT"));
    assert!(err.contains("independent"));
}

#[test]
fn test_run_pnoise_analysis_input_reference_requires_inferable_source_when_unspecified() {
    let netlist = "* pnoise missing source\nR1 out 0 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        noise_ref: PnoiseReference::Input,
        input_source: String::new(),
        start_freq: 1e3,
        stop_freq: 1e5,
        points_per_unit: 3,
        sweep: PnoiseFrequencySweep::Decade,
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };

    let err = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect_err("input-referred PNOISE should require explicit or inferable source");
    assert!(err.contains("requires an explicit input source"));
}

#[test]
fn test_run_pnoise_analysis_supports_differential_reference_node() {
    let netlist = "* pnoise differential\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };
    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("differential PNOISE output should execute");
    assert_eq!(result.output_noise.len(), result.frequencies.len());
    assert!(!result.contributors.is_empty());
    assert!(
        result
            .contributors
            .iter()
            .all(|(_, percentage)| percentage.is_finite() && *percentage >= 0.0)
    );
    assert!(
        !result
            .warnings
            .iter()
            .any(|warning| warning.contains("uncorrelated PSD summation"))
    );
}

#[test]
fn test_run_pnoise_analysis_differential_noise_is_not_less_than_single_ended() {
    let netlist = "* pnoise differential compare\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let single_cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        output_ref: None,
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };
    let differential_cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };

    let single = run_pnoise_analysis_with_config(netlist, &single_cfg)
        .expect("single-ended PNOISE should execute");
    let differential = run_pnoise_analysis_with_config(netlist, &differential_cfg)
        .expect("differential PNOISE should execute");
    assert_eq!(single.output_noise.len(), differential.output_noise.len());
    for (single_value, diff_value) in single
        .output_noise
        .iter()
        .zip(differential.output_noise.iter())
    {
        assert!(
            *diff_value + 1e-30 >= *single_value,
            "differential PSD {} should be >= single-ended PSD {}",
            diff_value,
            single_value
        );
    }
}

#[test]
fn test_run_pnoise_analysis_differential_output_matches_core_noise_port_density() {
    let netlist = "* pnoise differential parity\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        output_ref: Some("in".to_string()),
        start_freq: 10.0,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: PnoiseFrequencySweep::Decade,
        max_sideband: 0,
        ..PnoiseRunConfig::default()
    };
    let result = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect("differential PNOISE output should execute");

    let parsed = rspice_core::netlist::parse_netlist(netlist).expect("netlist should parse");
    let mut sim_config = build_engine_config(&parsed, None);
    sim_config.tolerance = cfg.pss_tolerance;
    let engine = Engine::new(sim_config);
    let dc = engine.run_dc_op(&parsed).expect("dc op should execute");
    let out_idx = resolve_node_or_ground_index("out", &dc.node_names).expect("out must resolve");
    let ref_idx = resolve_node_or_ground_index("in", &dc.node_names).expect("in must resolve");
    let core = engine
        .run_noise_ports(&parsed, out_idx, Some(ref_idx), &result.frequencies, 300.0)
        .expect("core differential noise port run should execute");

    assert_eq!(core.len(), result.output_noise.len());
    for (idx, (core_point, ui_point)) in core.iter().zip(result.output_noise.iter()).enumerate() {
        let tol = 1e-24 + core_point.output_noise_density.abs() * 1e-9;
        assert!(
            (core_point.output_noise_density - *ui_point).abs() <= tol,
            "expected UI differential output density to match core at idx {} (f={} Hz): core={}, ui={}",
            idx,
            core_point.frequency,
            core_point.output_noise_density,
            ui_point
        );
    }
}

#[test]
fn test_run_pnoise_analysis_rejects_identical_output_and_reference_nodes() {
    let netlist = "* pnoise invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        output_ref: Some("out".to_string()),
        ..PnoiseRunConfig::default()
    };
    let err = run_pnoise_analysis_with_config(netlist, &cfg)
        .expect_err("PNOISE output/reference node collision should fail");
    assert!(err.contains("cannot be the same"));
}

#[test]
fn test_run_pnoise_analysis_auto_infers_output_node() {
    let netlist = "* pnoise auto\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let result = run_pnoise_analysis(netlist).expect("PNOISE auto mode should infer output node");
    assert!(!result.frequencies.is_empty());
    assert_eq!(result.output_noise.len(), result.frequencies.len());
}

#[test]
fn test_run_pnoise_analysis_with_config_and_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pnoise_top.rsch",
        "pnoise_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pnoise include fixture\n\
V1 in 0 1\n\
.include ../models/pnoise_stage.inc\n\
.end\n",
    );
    let cfg = PnoiseRunConfig {
        output_node: "out".to_string(),
        input_source: "V1".to_string(),
        ..PnoiseRunConfig::default()
    };

    let without_source = run_pnoise_analysis_with_config(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source =
        run_pnoise_analysis_with_config_and_source_path(&netlist, &cfg, Some(&source_path))
            .expect("source-aware PNOISE analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(
        with_source.output_noise.len(),
        with_source.frequencies.len()
    );
}

#[test]
fn test_run_pnoise_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pnoise_auto_top.rsch",
        "pnoise_auto_stage.inc",
        "R1 in out 1k\nC1 out 0 1n\n",
        "* pnoise auto include fixture\n\
V1 in 0 1\n\
.include ../models/pnoise_auto_stage.inc\n\
.end\n",
    );

    let without_source = run_pnoise_analysis(&netlist);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pnoise_analysis_with_source_path(&netlist, Some(&source_path))
        .expect("source-aware PNOISE auto analysis should resolve include");
    assert!(!with_source.frequencies.is_empty());
    assert_eq!(
        with_source.output_noise.len(),
        with_source.frequencies.len()
    );
}

#[test]
fn test_run_pstb_analysis_with_config_executes_for_driven_rlc_probe() {
    let netlist = "* pstb\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
    let cfg = PstbRunConfig {
        pss_fundamental_freq: 1e6,
        pss_num_harmonics: 8,
        pss_tolerance: 1e-4,
        probe_instance: "LPROBE".to_string(),
        max_harmonics: 8,
        num_multipliers: 4,
        stability_threshold: 1.0 + 1e-6,
        detect_subharmonics: true,
        eigenvalue_tolerance: 1e-10,
    };

    let result = run_pstb_analysis_with_config(netlist, &cfg)
        .expect("PSTB analysis should execute with explicit config");
    assert!(result.period.is_finite() && result.period > 0.0);
    assert!(result.fundamental_frequency.is_finite() && result.fundamental_frequency > 0.0);
    assert_eq!(result.probe_instance, "LPROBE");
    assert!(result.probe_branch_ordinal > 0);
    assert!(result.probe_state_self_transition.is_finite());
    assert!(result.probe_state_column_norm.is_finite() && result.probe_state_column_norm >= 0.0);
    assert!(result.probe_state_row_norm.is_finite() && result.probe_state_row_norm >= 0.0);
    assert!(result.probe_state_persistence_db.is_finite());
    assert!(!result.mode_indices.is_empty());
    assert_eq!(
        result.mode_indices.len(),
        result.probe_mode_participation.len()
    );
    assert_eq!(result.mode_indices.len(), result.multiplier_magnitude.len());
    assert_eq!(result.mode_indices.len(), result.multiplier_phase_deg.len());
    assert_eq!(result.mode_indices.len(), result.mode_damping.len());
    assert_eq!(result.mode_indices.len(), result.mode_frequency_hz.len());
    assert_eq!(result.mode_indices.len(), result.stability_margin_db.len());
    assert!(
        result
            .probe_mode_participation
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0 && *value <= 1.0)
    );
    assert!(
        result
            .multiplier_magnitude
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0)
    );
    assert!(
        result
            .multiplier_phase_deg
            .iter()
            .all(|value| value.is_finite())
    );
    assert!(result.mode_damping.iter().all(|value| value.is_finite()));
    assert!(
        result
            .mode_frequency_hz
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0)
    );
    assert!(
        result
            .stability_margin_db
            .iter()
            .all(|value| value.is_finite())
    );
    assert!(result.dominant_multiplier_magnitude.is_finite());
    assert!(result.min_stability_margin_db.is_finite());
    assert!(
        result.dominant_probe_mode >= 1 && result.dominant_probe_mode <= result.mode_indices.len()
    );
    assert!(result.dominant_probe_mode_participation.is_finite());
    let max_probe_participation = result
        .probe_mode_participation
        .iter()
        .copied()
        .fold(0.0, Value::max);
    assert!((result.dominant_probe_mode_participation - max_probe_participation).abs() < 1e-12);
    assert!(result.stability_classification.len() >= 4);
    assert_eq!(
        result.num_unstable,
        result
            .multiplier_magnitude
            .iter()
            .filter(|value| **value > cfg.stability_threshold)
            .count()
    );
}

#[test]
fn test_run_pstb_analysis_with_config_rejects_unknown_probe() {
    let netlist =
        "* pstb missing probe\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
    let cfg = PstbRunConfig {
        probe_instance: "LDOES_NOT_EXIST".to_string(),
        ..PstbRunConfig::default()
    };

    let err = run_pstb_analysis_with_config(netlist, &cfg)
        .expect_err("PSTB should reject unknown probe instance names");
    assert!(err.contains("not found"));
    assert!(err.contains("Available branches"));
    assert!(err.contains("LPROBE"));
}

#[test]
fn test_run_pstb_analysis_with_config_rejects_non_dynamic_probe_branch() {
    let netlist =
        "* pstb non-dynamic probe\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
    let cfg = PstbRunConfig {
        probe_instance: "V1".to_string(),
        ..PstbRunConfig::default()
    };

    let err = run_pstb_analysis_with_config(netlist, &cfg)
        .expect_err("PSTB should reject voltage-source probes in probe-aware mode");
    assert!(err.contains("inductor"));
    assert!(err.contains("Available inductor probes"));
}

#[test]
fn test_run_pstb_analysis_with_config_maps_probe_to_expected_state_index() {
    let netlist =
        "* pstb state index\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
    let cfg = PstbRunConfig {
        probe_instance: "lprobe".to_string(),
        num_multipliers: 3,
        ..PstbRunConfig::default()
    };

    let result = run_pstb_analysis_with_config(netlist, &cfg)
        .expect("PSTB should resolve case-insensitive inductor probe names");

    let parsed = rspice_core::netlist::parse_netlist(netlist).expect("test netlist must parse");
    let engine = Engine::new(build_engine_config(&parsed, None));
    let circuit = engine
        .build_circuit(&parsed)
        .expect("test circuit should build");
    let expected_branch = circuit
        .get_branch_by_name("LPROBE")
        .expect("LPROBE branch must exist");
    let expected_probe = circuit
        .inductor_probe_for_branch(expected_branch)
        .expect("LPROBE should map to inductor branch");

    assert_eq!(result.probe_branch_ordinal, expected_branch);
    assert_eq!(result.probe_state_index, expected_probe.state_index);
}

#[test]
fn test_run_pstb_analysis_with_config_rejects_invalid_multiplier_count() {
    let netlist = "* pstb invalid\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n";
    let cfg = PstbRunConfig {
        num_multipliers: 0,
        ..PstbRunConfig::default()
    };

    let err = run_pstb_analysis_with_config(netlist, &cfg)
        .expect_err("PSTB should reject zero requested multipliers");
    assert!(err.contains("multipliers"));
}

#[test]
fn test_run_pstb_analysis_default_executes() {
    let netlist = "* pstb auto\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n";
    let result = run_pstb_analysis(netlist).expect("PSTB default mode should execute");
    assert!(!result.mode_indices.is_empty());
    assert_eq!(
        result.mode_indices.len(),
        result.probe_mode_participation.len()
    );
    assert_eq!(result.mode_indices.len(), result.multiplier_magnitude.len());
    assert!(result.stability_classification.len() >= 4);
    assert_eq!(result.probe_instance, "LPROBE");
}

#[test]
fn test_run_pstb_analysis_with_config_and_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pstb_top.rsch",
        "pstb_stage.inc",
        "R1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n",
        "* pstb include fixture\n\
V1 in 0 1\n\
.include ../models/pstb_stage.inc\n\
.end\n",
    );
    let cfg = PstbRunConfig {
        probe_instance: "LPROBE".to_string(),
        ..PstbRunConfig::default()
    };

    let without_source = run_pstb_analysis_with_config(&netlist, &cfg);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source =
        run_pstb_analysis_with_config_and_source_path(&netlist, &cfg, Some(&source_path))
            .expect("source-aware PSTB analysis should resolve include");
    assert!(!with_source.mode_indices.is_empty());
    assert_eq!(with_source.probe_instance, "LPROBE");
}

#[test]
fn test_run_pstb_analysis_with_source_path_resolves_relative_include() {
    let (_temp_dir, source_path, netlist) = write_periodic_include_fixture(
        "pstb_auto_top.rsch",
        "pstb_auto_stage.inc",
        "R1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n",
        "* pstb auto include fixture\n\
V1 in 0 1\n\
.include ../models/pstb_auto_stage.inc\n\
.end\n",
    );

    let without_source = run_pstb_analysis(&netlist);
    assert!(
        without_source.is_err(),
        "relative include should fail without source path"
    );

    let with_source = run_pstb_analysis_with_source_path(&netlist, Some(&source_path))
        .expect("source-aware PSTB default analysis should resolve include");
    assert!(!with_source.mode_indices.is_empty());
    assert_eq!(with_source.probe_instance, "LPROBE");
}
