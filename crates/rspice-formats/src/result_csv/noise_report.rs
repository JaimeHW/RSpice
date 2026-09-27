//! Exact noise report rows for caller-selected analyses and waveforms.
use crate::table::escape_csv_field as csv_field;
use rspice_results::{
    analysis_result::AnalysisResult, noise::PeriodicNoiseConversionEvidence,
    waveform::RetainedWaveform,
};

/// Accumulates each analysis in caller order without selecting or aligning waveforms.
#[derive(Debug)]
pub struct NoiseReportCsv {
    contents: String,
    rows: usize,
}
impl Default for NoiseReportCsv {
    fn default() -> Self {
        Self::new()
    }
}
impl NoiseReportCsv {
    pub fn new() -> Self {
        Self {
            contents: String::from(
                "record,analysis_sequence,analysis_label,trace,device,mechanism,sample_index,frequency_hz,spectral_density,power_v2,share_pct,total_rms_v,input_rms_v,band_start_hz,band_end_hz,noise_figure_db,source_generator,source_resistor,source_resistance_ohm,source_temperature_kelvin,reference_temperature_kelvin,frequency_axis,carrier_hz,input_sideband,output_sideband,max_sideband,input_frequency_hz,output_frequency_hz,input_rms_a,input_rms_unknown,spectral_density_unit,power_s2,timing_jitter_rms_s,sampling_evidence_json\n",
            ),
            rows: 0,
        }
    }
    /// Append the retained summary and the already-selected spectra, in their existing order.
    pub fn append_analysis<'a, W>(
        &mut self,
        analysis: &AnalysisResult<W>,
        waveforms: impl IntoIterator<Item = &'a RetainedWaveform>,
    ) {
        let conversion = analysis
            .noise_summary
            .as_ref()
            .and_then(|summary| summary.conversion.as_ref());
        if let Some(summary) = &analysis.noise_summary {
            let mut fields = record("summary", analysis);
            fields[32] = analysis
                .measurements
                .iter()
                .find(|measurement| measurement.name == "timing_jitter_rms_s")
                .and_then(|measurement| measurement.value)
                .map(|value| format!("{value:.17e}"))
                .unwrap_or_default();
            fields[11] = summary
                .total_rms
                .map(|value| format!("{value:.17e}"))
                .unwrap_or_default();
            let input_column = match summary.input_quantity {
                Some(rspice_core::analysis::noise::NoiseInputQuantity::Voltage) => 12,
                Some(rspice_core::analysis::noise::NoiseInputQuantity::Current) => 28,
                None => 29,
            };
            fields[input_column] = summary
                .input_rms
                .map(|value| format!("{value:.17e}"))
                .unwrap_or_default();
            fields[13] = format!("{:.17e}", summary.band.0);
            fields[14] = format!("{:.17e}", summary.band.1);
            append(&mut self.contents, fields, conversion, None);
            self.rows += 1;
            if let Some(figure) = &summary.noise_figure {
                for (sample_index, (&frequency, &decibels)) in
                    figure.frequencies.iter().zip(&figure.decibels).enumerate()
                {
                    let mut fields = record("noise_figure", analysis);
                    fields[3] = "Noise figure (SSB)".into();
                    fields[6] = sample_index.to_string();
                    fields[7] = format!("{frequency:.17e}");
                    fields[15] = format!("{decibels:.17e}");
                    fields[16] = csv_field(&figure.input_source);
                    fields[17] = csv_field(&figure.source_resistor);
                    fields[18] = format!("{:.17e}", figure.source_resistance_ohm);
                    fields[19] = format!("{:.17e}", figure.source_temperature_kelvin);
                    fields[20] = format!("{:.17e}", figure.reference_temperature_kelvin);
                    append(&mut self.contents, fields, conversion, Some(frequency));
                    self.rows += 1;
                }
            }
            for contributor in &summary.rows {
                let mut fields = record("contributor", analysis);
                fields[4] = csv_field(&contributor.device);
                fields[5] = csv_field(&contributor.mechanism);
                fields[if summary.is_timing() { 31 } else { 9 }] =
                    format!("{:.17e}", contributor.power);
                fields[10] = format!("{:.17e}", contributor.share_pct);
                fields[13] = format!("{:.17e}", summary.band.0);
                fields[14] = format!("{:.17e}", summary.band.1);
                append(&mut self.contents, fields, conversion, None);
                self.rows += 1;
            }
        }
        for waveform in waveforms {
            for (sample_index, (&frequency, &value)) in
                waveform.x.iter().zip(waveform.y.iter()).enumerate()
            {
                let mut fields = record("spectrum", analysis);
                fields[3] = csv_field(&waveform.name);
                fields[6] = sample_index.to_string();
                fields[7] = format!("{frequency:.17e}");
                fields[8] = format!("{value:.17e}");
                fields[30] = csv_field(waveform.unit.as_deref().unwrap_or(""));
                append(&mut self.contents, fields, conversion, Some(frequency));
                self.rows += 1;
            }
        }
    }
    pub fn row_count(&self) -> usize {
        self.rows
    }
    pub fn into_string(self) -> String {
        self.contents
    }
}

fn record<W>(kind: &str, analysis: &AnalysisResult<W>) -> Vec<String> {
    let mut fields = vec![String::new(); 34];
    fields[0] = kind.into();
    fields[1] = analysis.id.to_string();
    fields[2] = csv_field(&analysis.label);
    fields
}

fn append(
    contents: &mut String,
    mut fields: Vec<String>,
    conversion: Option<&PeriodicNoiseConversionEvidence>,
    offset: Option<f64>,
) {
    fields.resize(34, String::new());
    fields[21] = if conversion.is_some() {
        "offset_hz"
    } else {
        "frequency_hz"
    }
    .into();
    if let Some(channel) = conversion {
        fields[22] = format!("{:.17e}", channel.carrier_hz);
        if !channel.input_source.is_empty() {
            fields[23] = channel.input_sideband.to_string();
        }
        fields[24] = channel.output_sideband.to_string();
        fields[25] = channel.max_sideband.to_string();
        if let Some(offset) = offset {
            if !channel.input_source.is_empty() {
                fields[26] = format!("{:.17e}", channel.input_frequency(offset));
            }
            fields[27] = format!("{:.17e}", channel.output_frequency(offset));
        }
        fields[16] = csv_field(&channel.input_source);
        if let Some(sampling) = &channel.sampling
            && let Ok(json) = serde_json::to_string(sampling)
        {
            fields[33] = csv_field(&json);
        }
    }
    contents.push_str(&fields.join(","));
    contents.push('\n');
}
