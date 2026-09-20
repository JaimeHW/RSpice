//! Lossless noise spectra, source references and periodic conversion channels.

use super::super::{ResultSheetCsv, csv_field};
use crate::state::{AnalysisResult, PeriodicNoiseConversionEvidence, SimulationRun};

fn record(kind: &str, analysis: &AnalysisResult) -> Vec<String> {
    let mut fields = vec![String::new(); 31];
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
    fields.resize(31, String::new());
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
    }
    contents.push_str(&fields.join(","));
    contents.push('\n');
}

pub(crate) fn export_csv(
    run: &SimulationRun,
    analysis_indices: &[usize],
) -> Option<ResultSheetCsv> {
    let mut contents = String::from(
        "record,analysis_sequence,analysis_label,trace,device,mechanism,sample_index,frequency_hz,spectral_density,power_v2,share_pct,total_rms_v,input_rms_v,band_start_hz,band_end_hz,noise_figure_db,source_generator,source_resistor,source_resistance_ohm,source_temperature_kelvin,reference_temperature_kelvin,frequency_axis,carrier_hz,input_sideband,output_sideband,max_sideband,input_frequency_hz,output_frequency_hz,input_rms_a,input_rms_unknown,spectral_density_unit\n",
    );
    let mut rows = 0;
    for &index in analysis_indices {
        let analysis = run.analyses.get(index)?;
        let conversion = analysis
            .noise_summary
            .as_ref()
            .and_then(|summary| summary.conversion.as_ref());
        if let Some(summary) = &analysis.noise_summary {
            let mut fields = record("summary", analysis);
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
            append(&mut contents, fields, conversion, None);
            rows += 1;
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
                    append(&mut contents, fields, conversion, Some(frequency));
                    rows += 1;
                }
            }
            for contributor in &summary.rows {
                let mut fields = record("contributor", analysis);
                fields[4] = csv_field(&contributor.device);
                fields[5] = csv_field(&contributor.mechanism);
                fields[9] = format!("{:.17e}", contributor.power);
                fields[10] = format!("{:.17e}", contributor.share_pct);
                fields[13] = format!("{:.17e}", summary.band.0);
                fields[14] = format!("{:.17e}", summary.band.1);
                append(&mut contents, fields, conversion, None);
                rows += 1;
            }
        }
        for waveform in analysis
            .waveforms
            .iter()
            .filter(|waveform| waveform.visible && waveform.name != "Noise figure (SSB)")
        {
            for (sample_index, (&frequency, &value)) in
                waveform.x.iter().zip(waveform.y.iter()).enumerate()
            {
                let mut fields = record("spectrum", analysis);
                fields[3] = csv_field(&waveform.name);
                fields[6] = sample_index.to_string();
                fields[7] = format!("{frequency:.17e}");
                fields[8] = format!("{value:.17e}");
                fields[30] = csv_field(waveform.unit.as_deref().unwrap_or(""));
                append(&mut contents, fields, conversion, Some(frequency));
                rows += 1;
            }
        }
    }
    (rows != 0).then(|| ResultSheetCsv {
        default_name: "rspice-noise-contributions.csv",
        detail: format!("{rows} noise spectrum and contribution rows"),
        contents,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::analysis::noise::NoiseInputQuantity;

    #[test]
    fn noise_input_units_keep_current_rms_out_of_the_voltage_csv_column() {
        for (quantity, column, unit) in [
            (Some(NoiseInputQuantity::Voltage), 12, "V²/Hz"),
            (Some(NoiseInputQuantity::Current), 28, "A²/Hz"),
            (None, 29, ""),
        ] {
            let mut run = SimulationRun::new(1);
            let mut wave = crate::state::WaveformData::new(
                "inoise",
                vec![1e3, 1e4],
                vec![1e-21, 1e-21],
                "#fff",
            );
            if !unit.is_empty() {
                wave = wave.with_unit(unit);
            }
            run.add_analysis(
                AnalysisResult::new(1, crate::state::AnalysisType::Noise, "NOISE")
                    .with_waveforms(vec![wave])
                    .with_noise_summary(crate::state::NoiseSummary {
                        input_quantity: quantity,
                        input_rms: Some(3e-9),
                        band: (1e3, 1e4),
                        ..Default::default()
                    }),
            );
            let export = export_csv(&run, &[0]).unwrap();
            let rows: Vec<Vec<_>> = export
                .contents
                .lines()
                .map(|line| line.split(',').collect())
                .collect();
            assert!(rows.iter().all(|row| row.len() == 31));
            assert_eq!(rows[1][column].parse::<f64>().unwrap(), 3e-9);
            for other in [12, 28, 29].into_iter().filter(|index| *index != column) {
                assert!(rows[1][other].is_empty());
            }
            assert_eq!(rows[2][30], unit);
        }
    }
}
