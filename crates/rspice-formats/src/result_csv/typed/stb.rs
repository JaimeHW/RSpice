//! Complete loop-gain samples, measured margins and diagnostics; undefined cells stay empty.
use super::{EncodedTypedCsv, TypedCsvSummary, csv_text};
use rspice_core::analysis::stb::StbResult;

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
}
