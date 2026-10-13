//! Retained QPNOISE integrals and contributor rankings alongside the plots.
use crate::presentation::panel_note;
use rspice_core::engine::{
    QpnoiseAnalysisResult, QpnoiseObservation, QpnoiseUnavailable, QpnoiseValue, QpxfQuantity,
};
use rspice_ui_kit::widgets::{measurement_table, section_header};

fn value(v: QpnoiseValue, unit: &str) -> String {
    match v {
        QpnoiseValue::Finite(v) => format!("{v:.6e} {unit}"),
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
pub fn summary(ui: &mut egui::Ui, response: &QpnoiseAnalysisResult) {
    section_header(ui, "QPNOISE measurements", None);
    panel_note(
        ui,
        "Each spectrum uses its output's physical frequency. Cross spectra retain complex \
         correlation. Gaps mark unavailable referred measurements.",
    );
    let request = &response.metadata.request;
    if let Some(integration) = &request.integration {
        let band = match integration.band_hz {
            Some([low, high]) => format!("{low:.6e}–{high:.6e} Hz"),
            None => "full physical output band".into(),
        };
        let integration = format!("{band} ({:?})", integration.method);
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
        let mut rows: Vec<(String, String)> =
            vec![("Lattice".into(), format!("{:?}", output.lattice))];
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
        let indices: Vec<_> = spectrum
            .ranking
            .as_ref()
            .map(|r| r.iter().map(|v| v.source_index).collect())
            .unwrap_or_else(|| (0..response.sources.len()).collect());
        for (rank, source) in indices.into_iter().enumerate() {
            let share = spectrum
                .ranking
                .as_ref()
                .map(|r| format!("{:.4}%", r[rank].percentage));
            let rms = spectrum
                .integrated
                .as_ref()
                .map(|i| format!("{} RMS", value(i.contributor_rms[source], unit)));
            let reading = [share, rms].into_iter().flatten().collect::<Vec<_>>();
            let reading = if reading.is_empty() {
                "retained".to_owned()
            } else {
                reading.join(" · ")
            };
            rows.push((response.sources[source].name.clone(), reading));
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
                "Unavailable samples".into(),
                format!("{unavailable} · reasons retained in CSV"),
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
