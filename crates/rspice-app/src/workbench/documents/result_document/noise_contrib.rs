//! Noise contributor source binding and CSV export at the application boundary.

use super::bode;
use crate::state::SimulationState;
use crate::workbench::AppState;
use egui::Ui;
use rspice_results_ui::noise_contrib as view;

mod csv;
pub(crate) use csv::export_csv;

fn selected_evidence(
    simulation: &SimulationState,
    index: Option<usize>,
) -> view::ContributorEvidence<'_> {
    let analysis = index.and_then(|index| simulation.active_run()?.analyses.get(index));
    let Some(analysis) = analysis else {
        return view::ContributorEvidence::Unavailable;
    };
    match &analysis.noise_summary {
        Some(summary) => view::ContributorEvidence::Retained {
            summary,
            label: &analysis.label,
        },
        None => view::ContributorEvidence::NotRetained,
    }
}

/// Bind the contributor inspector to the same qualified analysis as the noise spectrum.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    bode::noise_spectrum_right_panel(ui, state);
    let index = bode::selected_noise_analysis_index(state);
    let evidence = selected_evidence(&state.simulation, index);
    view::right_panel(ui, evidence, &mut state.ui.results.cache);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, NoiseSummary, SimulationRun, WaveformData};

    #[test]
    fn hbnoise_csv_separates_decibels_from_density_and_retains_source_reference() {
        let figure = std::sync::Arc::new(crate::state::NoiseFigureEvidence {
            input_source: "V1".into(),
            source_resistor: "Rs".into(),
            source_resistance_ohm: 50.0,
            source_temperature_kelvin: 300.15,
            reference_temperature_kelvin: 290.0,
            frequencies: vec![1e3, 1e4],
            decibels: vec![2.0, 3.0],
        });
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE")
                .with_waveforms(vec![
                    WaveformData::new(
                        "onoise",
                        figure.frequencies.clone(),
                        vec![1e-18, 2e-18],
                        "#fff",
                    ),
                    WaveformData::new(
                        "Noise figure (SSB)",
                        figure.frequencies.clone(),
                        figure.decibels.clone(),
                        "#fff",
                    )
                    .with_unit("dB"),
                ])
                .with_noise_summary(NoiseSummary {
                    input_quantity: None,
                    conversion: Some(crate::state::PeriodicNoiseConversionEvidence {
                        sampling: None,
                        input_source: "V1".into(),
                        carrier_hz: 1e6,
                        input_sideband: 1,
                        output_sideband: -1,
                        max_sideband: 4,
                    }),
                    noise_figure: Some(figure),
                    band: (1e3, 1e4),
                    rows: vec![crate::state::NoiseContributorRow {
                        device: "Rs".into(),
                        mechanism: "thermal".into(),
                        power: 9e-15,
                        share_pct: 50.0,
                    }],
                    ..Default::default()
                }),
        );
        let export = export_csv(&run, &[0]).unwrap();
        let lines: Vec<Vec<&str>> = export
            .contents
            .lines()
            .map(|line| line.split(',').collect())
            .collect();
        assert_eq!(lines.len(), 7);
        assert!(lines.iter().all(|row| row.len() == 34), "{:?}", lines);
        let figure_rows: Vec<_> = lines
            .iter()
            .filter(|row| row[0] == "noise_figure")
            .collect();
        assert_eq!(figure_rows.len(), 2);
        assert_eq!(figure_rows[0][15].parse::<f64>().unwrap(), 2.0);
        assert_eq!(figure_rows[1][15].parse::<f64>().unwrap(), 3.0);
        for row in figure_rows {
            assert!(row[8].is_empty());
            assert_eq!(row[16], "V1");
            assert_eq!(row[17], "Rs");
            assert_eq!(row[18].parse::<f64>().unwrap(), 50.0);
            assert_eq!(row[19].parse::<f64>().unwrap(), 300.15);
            assert_eq!(row[20].parse::<f64>().unwrap(), 290.0);
        }
        for row in lines.iter().skip(1) {
            assert_eq!(row[21], "offset_hz");
            assert_eq!(row[22].parse::<f64>().unwrap(), 1e6);
            assert_eq!(row[23], "1");
            assert_eq!(row[24], "-1");
            assert_eq!(row[25], "4");
            if !row[7].is_empty() {
                let offset = row[7].parse::<f64>().unwrap();
                assert_eq!(row[26].parse::<f64>().unwrap(), offset + 1e6);
                assert_eq!(row[27].parse::<f64>().unwrap(), offset - 1e6);
            }
        }
        let densities: Vec<_> = lines.iter().filter(|row| row[0] == "spectrum").collect();
        assert_eq!(densities.len(), 2);
        assert!(
            densities
                .iter()
                .all(|row| row[3] == "onoise" && row[15].is_empty())
        );
    }

    #[test]
    fn selected_summary_is_bound_to_the_same_renderable_noise_analysis() {
        let mut first = AnalysisResult::new(1, AnalysisType::Noise, "first").with_waveforms(vec![
            WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#fff"),
        ]);
        first.noise_summary = Some(NoiseSummary {
            input_quantity: None,
            conversion: None,
            noise_figure: None,
            band: (1.0, 10.0),
            ..NoiseSummary::default()
        });
        let mut second =
            AnalysisResult::new(2, AnalysisType::Noise, "second").with_waveforms(vec![
                WaveformData::new("inoise", vec![1.0, 10.0], vec![3.0e-9, 4.0e-9], "#fff"),
            ]);
        second.noise_summary = Some(NoiseSummary {
            input_quantity: None,
            conversion: None,
            noise_figure: None,
            band: (2.0, 20.0),
            ..NoiseSummary::default()
        });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(first);
        run.add_analysis(second);
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(1));

        let view::ContributorEvidence::Retained { summary, label } = selected_evidence(
            &state.simulation,
            bode::selected_noise_analysis_index(&state),
        ) else {
            panic!("selected noise summary");
        };
        assert_eq!(label, "second");
        assert_eq!(summary.band, (2.0, 20.0));
    }
}
