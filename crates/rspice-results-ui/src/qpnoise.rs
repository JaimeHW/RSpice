//! Retained QPNOISE integrals and contributor rankings alongside the plots.
use crate::noise_contrib::{ContributorLine, contributor_table};
use crate::presentation::{frequency_readout, panel_note};
use rspice_results::engine_evidence::{
    QpnoiseAnalysisResult, QpnoiseIntegrationMethod, QpnoiseObservation, QpnoiseUnavailable,
    QpnoiseValue, QpxfQuantity,
};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_si_significant;
use rspice_ui_kit::widgets::measurement_table;
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

fn unavailable_reason(why: QpnoiseUnavailable) -> &'static str {
    match why {
        QpnoiseUnavailable::ZeroInputTransfer => "zero input transfer",
        QpnoiseUnavailable::OutsideNumericRange => "outside numerical range",
        QpnoiseUnavailable::NoFrequencyInterval => "a frequency interval is required",
        QpnoiseUnavailable::NegativeIntegrationFrequency => {
            "integration band includes negative frequencies"
        }
        QpnoiseUnavailable::IncompleteIntegrationBand => {
            "sweep does not cover the integration band"
        }
        QpnoiseUnavailable::UndefinedIntegrationSample => "integration contains undefined samples",
    }
}

fn value(v: QpnoiseValue, unit: &str, digits: usize) -> String {
    match v {
        QpnoiseValue::Finite(v) => fmt_si_significant(v, unit, digits),
        QpnoiseValue::Unavailable(why) => format!("Unavailable: {}", unavailable_reason(why)),
    }
}

/// Every source the run ranked, in its order, each with its rms at the
/// output and its share; a source the run holds no rms for says why.
fn ranked_lines<'a>(
    names: &[&'a str],
    ranking: impl IntoIterator<Item = (usize, f64)>,
    rms: Option<&[QpnoiseValue]>,
) -> Vec<ContributorLine<'a>> {
    ranking
        .into_iter()
        .enumerate()
        .filter_map(|(rank, (source, share))| {
            let value = rms.and_then(|rms| rms.get(source));
            Some(ContributorLine {
                rank: rank + 1,
                device: names.get(source)?,
                mechanism: "",
                figure: match value {
                    Some(QpnoiseValue::Finite(value)) => Some(*value),
                    _ => None,
                },
                figure_absent: match value {
                    Some(QpnoiseValue::Unavailable(why)) => Some(unavailable_reason(*why)),
                    _ => None,
                },
                share_pct: Some(share),
            })
        })
        .collect()
}
/// Render measurements from source-qualified retained QPNOISE evidence.
///
/// Every value is a row: the integration band, each output's RMS figures and
/// each contributor's share are facts with names, and they sit on the same
/// label/value grid as the rest of the inspector rather than as sentences on
/// its edge.
pub fn summary(
    ui: &mut egui::Ui,
    response: &QpnoiseAnalysisResult,
    quantity_policy: &QuantityPresentationPolicy,
    digits: usize,
) {
    section_header(ui, "Noise", None);
    let request = &response.metadata.request;
    if let Some(integration) = &request.integration {
        let band = match integration.band_hz {
            Some([low, high]) => format!(
                "{} – {}",
                frequency_readout(quantity_policy, low, digits),
                frequency_readout(quantity_policy, high, digits)
            ),
            None => "full output band".into(),
        };
        let method = match integration.method {
            QpnoiseIntegrationMethod::Linear => "linear",
            QpnoiseIntegrationMethod::LogLog => "log-log",
        };
        let integration = format!("{band} · {method}");
        measurement_table(ui, &[("Integration", integration.as_str())]);
    }
    for (index, spectrum) in response.outputs.iter().enumerate() {
        let output = &request.outputs[index];
        let (name, unit) = match &output.observation {
            QpnoiseObservation::Voltage { positive, negative } => {
                (format!("V({positive},{negative})"), "V")
            }
            QpnoiseObservation::BranchCurrent { branch } => (format!("I({branch})"), "A"),
        };
        section_header(ui, &format!("Output {}: {name}", index + 1), None);
        let mut rows: Vec<(String, String)> = Vec::new();
        if let Some(integral) = &spectrum.integrated {
            rows.push((
                "Output RMS".into(),
                value(integral.output_rms, unit, digits),
            ));
            if let Some(input) = integral.input_rms {
                let unit = if response
                    .metadata
                    .input_source
                    .as_ref()
                    .is_some_and(|i| i.quantity == QpxfQuantity::Current)
                {
                    "A"
                } else {
                    "V"
                };
                rows.push(("Input RMS".into(), value(input, unit, digits)));
            }
        }
        let unavailable = spectrum
            .input_noise
            .iter()
            .flatten()
            .chain(spectrum.noise_figure_db.iter().flatten())
            .filter(|v| matches!(v, QpnoiseValue::Unavailable(_)))
            .count();
        if unavailable > 0 {
            rows.push((
                "Undefined points".into(),
                format!("{unavailable} (see CSV)"),
            ));
        }
        let refs: Vec<(&str, &str)> = rows
            .iter()
            .map(|(name, reading)| (name.as_str(), reading.as_str()))
            .collect();
        measurement_table(ui, &refs);
        // Without a ranking there is no order to list in, and listing every
        // noise source in the circuit is not a contributor list; the
        // "ranking unavailable" line below says so instead.
        if let Some(ranking) = &spectrum.ranking {
            let names: Vec<&str> = response
                .sources
                .iter()
                .map(|source| source.name.as_str())
                .collect();
            let lines = ranked_lines(
                &names,
                ranking
                    .iter()
                    .map(|rank| (rank.source_index, rank.percentage)),
                spectrum
                    .integrated
                    .as_ref()
                    .map(|integral| integral.contributor_rms.as_slice()),
            );
            contributor_table(
                ui,
                &format!("qpnoise-contributors-{index}"),
                &lines,
                ("RMS", unit),
                digits,
                None,
            );
        }
        if request.contributor_ranking && spectrum.ranking.is_none() {
            panel_note(
                ui,
                "Contributor ranking unavailable for this integration band.",
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_source_the_run_ranks_is_listed_in_its_order() {
        let names: Vec<String> = (0..25).map(|index| format!("R{index}")).collect();
        let names: Vec<&str> = names.iter().map(String::as_str).collect();
        let mut rms: Vec<QpnoiseValue> = (0..25)
            .map(|index| QpnoiseValue::Finite(1.0e-6 * f64::from(index + 1)))
            .collect();
        rms[3] = QpnoiseValue::Unavailable(QpnoiseUnavailable::ZeroInputTransfer);
        let ranking = (0..25).rev().map(|source| (source, 4.0));

        let lines = ranked_lines(&names, ranking, Some(&rms));
        assert_eq!(lines.len(), 25);
        assert_eq!((lines[0].rank, lines[0].device), (1, "R24"));
        assert_eq!(lines[0].figure, Some(1.0e-6 * 25.0));
        assert_eq!((lines[24].rank, lines[24].device), (25, "R0"));
        let refused = &lines[21];
        assert_eq!((refused.device, refused.figure), ("R3", None));
        assert_eq!(refused.figure_absent, Some("zero input transfer"));
    }
}
