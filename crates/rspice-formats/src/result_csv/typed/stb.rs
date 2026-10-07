//! Complete loop-gain samples, measured margins and diagnostics; undefined cells stay empty.
use super::{EncodedTypedCsv, TypedCsvSummary, csv_text};
use rspice_core::analysis::pole_zero::{RootSetEvidence, StabilityVerdict};
use rspice_core::analysis::stb::{CircuitPoleEvidence, CircuitPoleFailure, StbResult};

pub(super) fn prepare(response: &StbResult) -> EncodedTypedCsv {
    let mut contents = String::from(
        "kind,frequency_hz,real,imaginary,magnitude,magnitude_db,phase_degrees,name,value,unit,detail\n",
    );
    let number = |value: Option<f64>| value.map(|v| format!("{v:.17e}")).unwrap_or_default();
    let mut row = |kind: &str, fields: &[(usize, String)]| {
        let mut cells = vec![String::new(); 11];
        cells[0] = kind.to_owned();
        for (index, value) in fields {
            cells[*index] = csv_text(value);
        }
        contents.push_str(&cells.join(","));
        contents.push('\n');
    };
    for p in &response.bode_points {
        row(
            "ac",
            &[
                (1, number(Some(p.frequency))),
                (2, number(Some(p.loop_gain.re))),
                (3, number(Some(p.loop_gain.im))),
                (4, number(p.magnitude)),
                (5, number(p.magnitude_db)),
                (6, number(p.phase_deg)),
            ],
        );
    }
    let dc = response.margins.dc_loop_gain;
    row(
        "dc",
        &[
            (1, "0".into()),
            (2, number(dc.map(|v| v.re))),
            (3, number(dc.map(|v| v.im))),
            (
                10,
                if dc.is_some() {
                    "measured"
                } else {
                    "unavailable"
                }
                .into(),
            ),
        ],
    );
    for (name, unit, margin) in [
        ("gain_margin_db", "dB", response.margins.gain_margin),
        ("phase_margin_degrees", "deg", response.margins.phase_margin),
    ] {
        row(
            "margin",
            &[
                (1, number(margin.map(|m| m.frequency))),
                (7, name.into()),
                (8, number(margin.map(|m| m.value))),
                (9, unit.into()),
                (
                    10,
                    if margin.is_some() {
                        "measured"
                    } else {
                        "no_crossover"
                    }
                    .into(),
                ),
            ],
        );
    }
    row(
        "scalar",
        &[
            (7, "num_crossovers".into()),
            (8, response.margins.num_crossovers.to_string()),
            (9, "count".into()),
        ],
    );
    row(
        "scalar",
        &[
            (7, "nyquist_retained".into()),
            (8, u8::from(!response.nyquist_points.is_empty()).to_string()),
            (9, "1".into()),
        ],
    );
    for warning in &response.warnings {
        row("warning", &[(10, warning.clone())]);
    }
    let mut evidence = |name: &str, value: String| {
        row("circuit_evidence", &[(7, name.into()), (8, value)]);
    };
    evidence(
        "stability",
        match response.stability_verdict() {
            StabilityVerdict::Stable => "stable",
            StabilityVerdict::Unstable => "unstable",
            StabilityVerdict::Indeterminate => "indeterminate",
        }
        .into(),
    );
    match &response.circuit_poles {
        CircuitPoleEvidence::NotComputed => evidence("status", "not_computed".into()),
        CircuitPoleEvidence::Unavailable { cause } => {
            evidence("status", "unavailable".into());
            match cause {
                CircuitPoleFailure::Unsupported { capability, detail } => {
                    evidence("failure_kind", "unsupported".into());
                    evidence("capability", capability.clone());
                    evidence("detail", detail.clone());
                }
                CircuitPoleFailure::Numerical { detail } => {
                    evidence("failure_kind", "numerical".into());
                    evidence("detail", detail.clone());
                }
                CircuitPoleFailure::ResourceLimit {
                    resource,
                    requested,
                    limit,
                } => {
                    evidence("failure_kind", "resource_limit".into());
                    evidence("resource", resource.clone());
                    evidence("requested", requested.to_string());
                    evidence("limit", limit.to_string());
                }
            }
        }
        CircuitPoleEvidence::Available { spectrum } => {
            evidence("status", "available".into());
            evidence(
                "qualification",
                match &spectrum.evidence {
                    RootSetEvidence::QualifiedEmpty { .. } => "qualified_empty",
                    RootSetEvidence::Qualified { .. } => "qualified",
                    RootSetEvidence::Approximate { .. } => "approximate",
                    RootSetEvidence::NotRequested => "not_requested",
                    RootSetEvidence::LegacyUnknown => "legacy_unknown",
                    _ => "unknown",
                }
                .into(),
            );
            if let Some(certificate) = spectrum.evidence.certificate() {
                evidence("problem_order", certificate.problem_order.to_string());
                evidence("infinite_count", certificate.infinite_count.to_string());
                evidence(
                    "max_backward_error",
                    number(Some(certificate.max_backward_error)),
                );
                evidence(
                    "qualification_tolerance",
                    number(Some(certificate.qualification_tolerance)),
                );
            }
            for (index, pole) in spectrum.poles.iter().enumerate() {
                row(
                    "circuit_pole",
                    &[
                        (2, number(Some(pole.re))),
                        (3, number(Some(pole.im))),
                        (7, format!("pole_{}", index + 1)),
                        (9, "rad/s".into()),
                    ],
                );
            }
        }
    }
    EncodedTypedCsv {
        contents,
        summary: TypedCsvSummary::Stb {
            frequency_count: response.bode_points.len(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stb_csv_keeps_measured_zero_separate_from_undefined_quantities() {
        let mut response = rspice_core::analysis::stb::StbAnalyzer::new(Default::default())
            .analyze(&[10.0], &[rspice_core::Complex64::new(0.0, 0.0)])
            .unwrap();
        response.margins.dc_loop_gain = Some(rspice_core::Complex64::new(0.0, 0.0));
        response
            .warnings
            .push("diagnostic, with punctuation".into());
        let csv = prepare(&response);
        let ac: Vec<_> = csv.contents.lines().nth(1).unwrap().split(',').collect();
        assert_eq!(ac.len(), 11);
        assert_eq!(ac[2].parse::<f64>().unwrap(), 0.0);
        assert_eq!(ac[4].parse::<f64>().unwrap(), 0.0);
        assert_eq!(&ac[5..7], &["", ""]);
        assert!(csv.contents.contains("no_crossover"));
        assert!(csv.contents.contains("\"diagnostic, with punctuation\""));
        assert!(!csv.contents.contains("NaN"));
    }

    #[test]
    fn stb_csv_preserves_natural_modes_and_typed_missing_evidence() {
        use rspice_core::analysis::pole_zero::{PoleSpectrum, SpectrumCertificate};
        let mut response = rspice_core::analysis::stb::StbAnalyzer::new(Default::default())
            .analyze(&[10.0], &[rspice_core::Complex64::new(1.0, 0.0)])
            .unwrap();
        assert!(prepare(&response).contents.contains("status,not_computed"));
        response.circuit_poles = CircuitPoleEvidence::Available {
            spectrum: PoleSpectrum {
                poles: vec![rspice_core::Complex64::new(2.0, 0.0)],
                evidence: RootSetEvidence::Qualified {
                    certificate: SpectrumCertificate::exact(3, 2).unwrap(),
                },
            },
        };
        let csv = prepare(&response).contents;
        let pole: Vec<_> = csv
            .lines()
            .find(|r| r.starts_with("circuit_pole,"))
            .unwrap()
            .split(',')
            .collect();
        assert_eq!(pole.len(), 11);
        assert_eq!(pole[1], "");
        assert_eq!(pole[2].parse::<f64>().unwrap(), 2.0);
        assert_eq!(pole[9], "rad/s");
        for field in [
            "stability,unstable",
            "status,available",
            "qualification,qualified",
            "problem_order,3",
            "infinite_count,2",
            "max_backward_error,",
            "qualification_tolerance,",
        ] {
            assert!(csv.contains(field), "{field}");
        }
        response.circuit_poles = CircuitPoleEvidence::Unavailable {
            cause: CircuitPoleFailure::ResourceLimit {
                resource: "result_values".into(),
                requested: 100,
                limit: 90,
            },
        };
        let csv = prepare(&response).contents;
        for field in [
            "stability,indeterminate",
            "status,unavailable",
            "failure_kind,resource_limit",
            "resource,result_values",
            "requested,100",
            "limit,90",
        ] {
            assert!(csv.contains(field), "{field}");
        }
        assert!(!csv.contains("circuit_pole,"));
    }
}
