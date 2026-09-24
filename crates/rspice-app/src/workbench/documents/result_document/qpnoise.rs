//! Retained QPNOISE integrals and contributor rankings alongside the plots.
use rspice_core::engine::{
    QpnoiseAnalysisResult, QpnoiseObservation, QpnoiseUnavailable, QpnoiseValue, QpxfQuantity,
};
pub(super) fn is_renderable(analysis: &crate::state::AnalysisResult) -> bool {
    analysis.analysis_type == crate::state::AnalysisType::Qpnoise
        && matches!(
            analysis.result_payload,
            Some(crate::state::AnalysisResultPayload::Qpnoise { .. })
        )
        && !analysis.waveforms.is_empty()
}

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
pub(super) fn summary(ui: &mut egui::Ui, response: &QpnoiseAnalysisResult) {
    ui.collapsing("QPNOISE measurements", |ui| {
        ui.small("Each spectrum uses its output's physical frequency. Cross spectra retain complex correlation. Gaps mark unavailable referred measurements.");
        let request = &response.metadata.request;
        if let Some(integration) = &request.integration {
            let band = match integration.band_hz {
                Some([low, high]) => format!("{low:.6e}–{high:.6e} Hz"),
                None => "full physical output band".into(),
            };
            ui.label(format!("Integration: {band} ({:?})", integration.method));
        }
        for (index, spectrum) in response.outputs.iter().enumerate() {
            let output = &request.outputs[index];
            let (name, unit) = match &output.observation {
                QpnoiseObservation::Voltage { positive, negative } => (format!("V({positive},{negative})"), "V"),
                QpnoiseObservation::BranchCurrent { branch } => (format!("I({branch})"), "A"),
            };
            let heading = format!("Output {}: {name} {:?}", index + 1, output.lattice);
            ui.collapsing(heading, |ui| {
                if let Some(integral) = &spectrum.integrated {
                    ui.label(format!("Output RMS: {}", value(integral.output_rms, unit)));
                    if let Some(input) = integral.input_rms {
                        let unit = if response.metadata.input_source.as_ref().is_some_and(|i| i.quantity == QpxfQuantity::Current) { "A" } else { "V" };
                        ui.label(format!("Input RMS: {}", value(input, unit)));
                    }
                }
                if request.contributor_ranking && spectrum.ranking.is_none() {
                    ui.label("Contributor ranking unavailable for this integration band.");
                }
                let indices: Vec<_> = spectrum.ranking.as_ref()
                    .map(|r| r.iter().map(|v| v.source_index).collect())
                    .unwrap_or_else(|| (0..response.sources.len()).collect());
                for (rank, source) in indices.into_iter().enumerate() {
                    let share = spectrum.ranking.as_ref()
                        .map(|r| format!(" — {:.4}%", r[rank].percentage))
                        .unwrap_or_default();
                    let rms = spectrum.integrated.as_ref()
                        .map(|i| format!(" — {} RMS", value(i.contributor_rms[source], unit)))
                        .unwrap_or_default();
                    ui.label(format!("{}{}{rms}", response.sources[source].name, share));
                }
                let unavailable = spectrum.input_noise.iter().flatten()
                    .chain(spectrum.noise_figure_db.iter().flatten())
                    .filter(|v| matches!(v, QpnoiseValue::Unavailable(_)))
                    .count();
                if unavailable > 0 {
                    ui.label(format!("{unavailable} unavailable measurement samples; exact reasons are retained in CSV."));
                }
            });
        }
    });
}
