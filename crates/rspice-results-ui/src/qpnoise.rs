//! Retained QPNOISE integrals and contributor rankings alongside the plots.
use crate::presentation::{frequency_readout, panel_note};
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_core::engine::{
    QpnoiseAnalysisResult, QpnoiseIntegrationMethod, QpnoiseObservation, QpnoiseUnavailable,
    QpnoiseValue, QpxfQuantity,
};
use rspice_ui_kit::panels::section_header;
use rspice_ui_kit::plot::fmt_si_readout;
use rspice_ui_kit::widgets::measurement_table;

/// The contributors a reader acts on. Below this the shares are rounding.
const RANKED_CONTRIBUTORS: usize = 10;

fn value(v: QpnoiseValue, unit: &str) -> String {
    match v {
        QpnoiseValue::Finite(v) => fmt_si_readout(v, unit, 4),
        QpnoiseValue::Unavailable(why) => format!(
            "Unavailable: {}",
            match why {
                QpnoiseUnavailable::ZeroInputTransfer => "zero input transfer",
                QpnoiseUnavailable::OutsideNumericRange => "outside numerical range",
                QpnoiseUnavailable::NoFrequencyInterval => "a frequency interval is required",
                QpnoiseUnavailable::NegativeIntegrationFrequency =>
                    "integration band includes negative frequencies",
                QpnoiseUnavailable::IncompleteIntegrationBand =>
                    "sweep does not cover the integration band",
                QpnoiseUnavailable::UndefinedIntegrationSample =>
                    "integration contains undefined samples",
            }
        ),
    }
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
) {
    section_header(ui, "Noise", None);
    let request = &response.metadata.request;
    if let Some(integration) = &request.integration {
        let band = match integration.band_hz {
            Some([low, high]) => format!(
                "{} – {}",
                frequency_readout(quantity_policy, low, 4),
                frequency_readout(quantity_policy, high, 4)
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
            rows.push(("Output RMS".into(), value(integral.output_rms, unit)));
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
                rows.push(("Input RMS".into(), value(input, unit)));
            }
        }
        // Without a ranking there is no order to truncate to, and listing
        // every noise source in the circuit is not a contributor list; the
        // "ranking unavailable" line below says so instead.
        let indices: Vec<_> = spectrum.ranking.as_ref().map_or_else(Vec::new, |ranking| {
            ranking
                .iter()
                .take(RANKED_CONTRIBUTORS)
                .map(|v| v.source_index)
                .collect()
        });
        for (rank, source) in indices.into_iter().enumerate() {
            let share = spectrum
                .ranking
                .as_ref()
                .map(|r| format!("{:.1}%", r[rank].percentage));
            // "RMS" names the quantity, so it goes on the measurement and
            // never on the sentence that says why there is none.
            let rms = spectrum
                .integrated
                .as_ref()
                .map(|i| match i.contributor_rms[source] {
                    QpnoiseValue::Finite(_) => {
                        format!("{} rms", value(i.contributor_rms[source], unit))
                    }
                    QpnoiseValue::Unavailable(_) => value(i.contributor_rms[source], unit),
                });
            let reading = [share, rms].into_iter().flatten().collect::<Vec<_>>();
            if reading.is_empty() {
                continue;
            }
            rows.push((response.sources[source].name.clone(), reading.join(" · ")));
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
        if request.contributor_ranking && spectrum.ranking.is_none() {
            panel_note(
                ui,
                "Contributor ranking unavailable for this integration band.",
            );
        }
    }
}
