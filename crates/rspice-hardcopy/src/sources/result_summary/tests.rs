use super::*;
use rspice_results::{analysis_type::AnalysisType, simulation_values::ComplexResultValue};

#[test]
fn viewer_partition_covers_every_results_family() {
    let curve_viewers = [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::NoiseContrib,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::Nyquist,
        ResultViewer::Smith,
    ];
    let summary_viewers = [
        ResultViewer::Op,
        ResultViewer::Contribution,
        ResultViewer::TransferFunction,
        ResultViewer::Specs,
        ResultViewer::PoleZero,
    ];
    assert!(curve_viewers.into_iter().all(is_curve_viewer));
    let analysis: AnalysisResult =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient", 0.0);
    // Noise supports both a spectrum and a ranked summary.
    for viewer in curve_viewers
        .into_iter()
        .chain([ResultViewer::Polar])
        .filter(|viewer| *viewer != ResultViewer::NoiseContrib)
    {
        assert!(matches!(
            semantic_result_summary(viewer, &analysis),
            Err(HardcopySourceError::UnsupportedVisualizationViewer(_))
        ));
    }
    assert!(
        summary_viewers
            .into_iter()
            .all(|viewer| !is_curve_viewer(viewer))
    );
}

#[test]
fn typed_sensitivity_summary_preserves_unavailable_quantities_and_exact_zero() {
    use rspice_results::sensitivity::{SensitivityUnavailability, SensitivityValue};
    let payload = AnalysisResultPayload::Sensitivity {
        output: "V(out)".to_owned(),
        result_mode: rspice_results::sensitivity::SensitivityResultMode::Dc,
        rows: vec![rspice_results::sensitivity::SensitivityResultRow {
            parameter: "gain".to_owned(),
            raw: 0.0.into(),
            normalized: SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput),
        }],
    };
    let analysis: AnalysisResult = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS", 0.0)
        .with_result_payload(payload.clone());
    let summary = semantic_result_summary(ResultViewer::Contribution, &analysis).unwrap();
    assert_eq!(summary.payload, Some(payload));
    assert_eq!(
        summary.tables[0].rows,
        vec![vec![
            "gain".to_owned(),
            exact_number(0.0),
            "Unavailable (zero-output)".to_owned()
        ]]
    );
}

#[test]
fn typed_pole_zero_summary_preserves_native_payload_and_exact_values() {
    let payload = AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 2.0,
        }],
        zeros: vec![ComplexResultValue {
            real: -3.0,
            imaginary: 0.0,
        }],
        pole_evidence: rspice_results::pole_zero::PoleZeroRootSetEvidence::LegacyUnknown,
        zero_evidence: rspice_results::pole_zero::PoleZeroRootSetEvidence::LegacyUnknown,
        gain: Some(4.0),
    };
    let analysis: AnalysisResult = AnalysisResult::new(3, AnalysisType::PoleZero, "PZ", 0.0)
        .with_result_payload(payload.clone());
    let summary = semantic_result_summary(ResultViewer::PoleZero, &analysis).unwrap();
    assert_eq!(summary.viewer, ResultViewer::PoleZero);
    assert_eq!(summary.payload, Some(payload));
    assert_eq!(summary.tables[0].rows.len(), 2);
    assert_eq!(summary.tables[0].rows[0][1], exact_number(-1.0));
    assert_eq!(summary.tables[0].rows[0][2], exact_number(2.0));
}

#[test]
fn typed_pstb_table_summary_preserves_complete_modes_and_global_evidence() {
    let certificate = rspice_results::floquet::FloquetSpectrumCertificateEvidence {
        problem_order: 1,
        max_backward_error: 0.0,
        qualification_tolerance:
            rspice_results::floquet::FloquetSpectrumCertificateEvidence::canonical_qualification_tolerance(1)
                .unwrap(),
    };
    let payload = AnalysisResultPayload::Pstb {
        period_s: Some(1.0),
        fundamental_frequency_hz: Some(1.0),
        stability_threshold: Some(1.0),
        probe_instance: Some("LPROBE".to_owned()),
        detect_subharmonics: Some(false),
        modes: vec![rspice_results::floquet::PstbFloquetModeEvidence {
            multiplier: ComplexResultValue {
                real: 0.5,
                imaginary: 0.0,
            },
            exponent: ComplexResultValue {
                real: 0.5_f64.ln(),
                imaginary: 0.0,
            },
            probe_participation: 0.25,
            is_unstable: false,
            is_trivial: false,
            subharmonic_order: None,
        }],
        floquet_evidence: rspice_results::floquet::FloquetSpectrumEvidence::Qualified {
            certificate,
        },
        orbit_kind: rspice_results::floquet::FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: rspice_results::floquet::FloquetStabilityVerdictEvidence::Stable,
        stability_classification:
            rspice_results::floquet::PstbStabilityClassificationEvidence::Stable,
        min_stability_margin_db: Some(-20.0 * 0.5_f64.log10()),
        max_multiplier_magnitude: Some(0.5),
        num_unstable: Some(0),
        subharmonics: Vec::new(),
        converged: Some(true),
        iterations: Some(0),
    };
    let analysis: AnalysisResult = AnalysisResult::new(4, AnalysisType::Pstb, "PSTB", 0.0)
        .with_result_payload(payload.clone());

    let summary = semantic_result_summary(ResultViewer::Table, &analysis).unwrap();

    assert_eq!(summary.viewer, ResultViewer::Table);
    assert_eq!(summary.payload, Some(payload));
    assert_eq!(summary.tables.len(), 2);
    assert!(summary.tables[0].title.contains("global metrics"));
    assert!(
        summary.tables[0]
            .rows
            .iter()
            .any(|row| { row[0] == "Certificate problem order" && row[1] == "1" })
    );
    assert_eq!(summary.tables[1].rows.len(), 1);
    assert_eq!(summary.tables[1].rows[0][1], exact_number(0.5));
    assert_eq!(summary.tables[1].rows[0][5], exact_number(0.25));
}

#[test]
fn a_dc_mismatch_sheet_prints_its_contributor_table() {
    use rspice_results::dc_mismatch::{
        DcMismatchContributorEvidence, DcMismatchEvidence, DcMismatchScopeEvidence,
    };

    let payload = AnalysisResultPayload::DcMismatch {
        evidence: std::sync::Arc::new(DcMismatchEvidence {
            output: "V(OUT)".to_owned(),
            output_unit: "V".to_owned(),
            nominal_value: 0.5,
            sigma_multiplier: 3.0,
            sigma_total: 2.0e-3,
            sigma_mismatch: 2.0e-3,
            sigma_process: 0.0,
            include_mismatch: true,
            include_process: false,
            contributor_limit: 2,
            threshold: 0.0,
            normalized_contributions: true,
            applied_correlations_mismatch: 0,
            applied_correlations_process: 0,
            evaluated_contributors: 6,
            contributors: vec![
                DcMismatchContributorEvidence {
                    instance: "R1".to_owned(),
                    parameter: "R1V".to_owned(),
                    scope: DcMismatchScopeEvidence::Mismatch,
                    sigma_parameter: 10.0,
                    sensitivity: 2.0e-4,
                    contribution: 2.0e-3,
                    share: 0.75,
                },
                DcMismatchContributorEvidence {
                    instance: "R2".to_owned(),
                    parameter: "R2V".to_owned(),
                    scope: DcMismatchScopeEvidence::Mismatch,
                    sigma_parameter: 10.0,
                    sensitivity: -1.0e-4,
                    contribution: -1.0e-3,
                    share: -0.25,
                },
            ],
        }),
    };
    let analysis: AnalysisResult = AnalysisResult::new(1, AnalysisType::DcMismatch, "DCMATCH", 0.0)
        .with_result_payload(payload.clone());
    let summary = semantic_result_summary(ResultViewer::Contribution, &analysis)
        .expect("a DC mismatch payload has a semantic table");
    assert_eq!(summary.payload, Some(payload));
    assert_eq!(summary.tables.len(), 2);
    assert_eq!(summary.tables[0].title, "DC mismatch of V(OUT)");
    assert_eq!(
        summary.tables[0].rows[1],
        vec!["Sigma total".to_owned(), exact_number(2.0e-3)]
    );
    assert_eq!(
        summary.tables[0].rows[5],
        vec!["Retained".to_owned(), "2 of 6 evaluated".to_owned()]
    );
    assert_eq!(summary.tables[1].title, "Contributors");
    assert_eq!(
        summary.tables[1].rows[0],
        vec![
            "1".to_owned(),
            "R1".to_owned(),
            "R1V".to_owned(),
            "mismatch".to_owned(),
            exact_number(2.0e-3),
            exact_number(0.75),
            exact_number(0.75),
        ]
    );
    // The signed cumulative sum, never renormalized.
    assert_eq!(summary.tables[1].rows[1][5], exact_number(-0.25));
    assert_eq!(summary.tables[1].rows[1][6], exact_number(0.5));

    // And the refusal names the family that is actually missing.
    let bare: AnalysisResult = AnalysisResult::new(2, AnalysisType::DcMismatch, "DCMATCH", 0.0);
    assert!(semantic_result_summary(ResultViewer::Contribution, &bare).is_err());
}
