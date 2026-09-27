//! Noise report selection, filename and publication detail.
use super::super::ResultSheetCsv;
use crate::state::SimulationRun;
use rspice_formats::result_csv::NoiseReportCsv;

pub(crate) fn export_csv(
    run: &SimulationRun,
    analysis_indices: &[usize],
) -> Option<ResultSheetCsv> {
    let mut csv = NoiseReportCsv::new();
    for &index in analysis_indices {
        let analysis = run.analyses.get(index)?;
        let waveforms = analysis
            .waveforms
            .iter()
            .filter(|waveform| waveform.visible && waveform.name != "Noise figure (SSB)")
            .map(AsRef::as_ref);
        csv.append_analysis(analysis, waveforms);
    }
    let rows = csv.row_count();
    (rows != 0).then(|| ResultSheetCsv {
        default_name: "rspice-noise-contributions.csv",
        detail: format!("{rows} noise spectrum and contribution rows"),
        contents: csv.into_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AnalysisResult;
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
            assert!(rows.iter().all(|row| row.len() == 34));
            assert_eq!(rows[1][column].parse::<f64>().unwrap(), 3e-9);
            for other in [12, 28, 29].into_iter().filter(|index| *index != column) {
                assert!(rows[1][other].is_empty());
            }
            assert_eq!(rows[2][30], unit);
        }
    }
}
