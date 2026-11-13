//! Noise contributor source binding and CSV export at the application boundary.

use super::bode;
use crate::state::SimulationState;
use crate::workbench::documents::result_document::ResultsParts;
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
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
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    bode::noise_spectrum_right_panel(ui, state);
    let index = bode::selected_noise_analysis_index(state.results_reads());
    let evidence = selected_evidence(&state.model.simulation, index);
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    view::right_panel(ui, evidence, &quantity_policy);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, NoiseSummary, RetainedTrace, SimulationRun};

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
                    RetainedTrace::new(
                        "onoise",
                        figure.frequencies.clone(),
                        vec![1e-18, 2e-18],
                        "#fff",
                    ),
                    RetainedTrace::new(
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
            RetainedTrace::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#fff"),
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
                RetainedTrace::new("inoise", vec![1.0, 10.0], vec![3.0e-9, 4.0e-9], "#fff"),
            ]);
        second.noise_summary = Some(NoiseSummary {
            input_quantity: None,
            conversion: None,
            noise_figure: None,
            band: (2.0, 20.0),
            ..NoiseSummary::default()
        });
        let mut state = ResultsTestHost::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(first);
        run.add_analysis(second);
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        assert!(state.model.simulation.select_analysis(1));

        let view::ContributorEvidence::Retained { summary, label } = selected_evidence(
            &state.model.simulation,
            bode::selected_noise_analysis_index(state.results_reads()),
        ) else {
            panic!("selected noise summary");
        };
        assert_eq!(label, "second");
        assert_eq!(summary.band, (2.0, 20.0));
    }

    fn contributor_summary() -> NoiseSummary {
        let sources = [
            ("M1", "flicker", 4.1e-12, 41.0),
            ("M1", "thermal", 2.2e-12, 22.0),
            ("XAMP.XBIAS.M14", "flicker", 1.4e-12, 14.0),
            ("R7", "thermal", 9.0e-13, 9.0),
            ("Q3", "shot", 6.0e-13, 6.0),
            ("M2", "thermal", 4.0e-13, 4.0),
            ("RLOAD", "thermal", 2.0e-13, 2.0),
            ("Q3", "burst", 1.0e-13, 1.0),
        ];
        NoiseSummary {
            rows: sources
                .iter()
                .map(|(device, mechanism, power, share)| {
                    rspice_results::noise::NoiseContributorRow {
                        device: (*device).to_owned(),
                        mechanism: (*mechanism).to_owned(),
                        power: *power,
                        share_pct: *share,
                    }
                })
                .collect(),
            total_rms: Some(3.2e-6),
            band: (10.0, 1.0e6),
            ..NoiseSummary::default()
        }
    }

    fn contributor_panel(ui: &mut Ui, summary: &NoiseSummary) {
        view::right_panel(
            ui,
            view::ContributorEvidence::Retained {
                summary,
                label: "NOISE",
            },
            &rspice_vocabulary::quantity::QuantityPresentationPolicy::default(),
        );
    }

    /// The contributor table's rows are the dense rows of the sheet's other
    /// tables at either density: one line each, device and mechanism side by
    /// side, the header one of them, and the rows abutting.
    #[test]
    fn the_contributor_rows_are_dense_at_either_density() {
        let summary = contributor_summary();
        for density in [
            rspice_ui_kit::tokens::Density::Compact,
            rspice_ui_kit::tokens::Density::Relaxed,
        ] {
            let ctx = egui::Context::default();
            crate::ui::Theme {
                density,
                ..crate::ui::Theme::default()
            }
            .apply(&ctx);
            ctx.enable_accesskit();
            let mut output = None;
            for _ in 0..2 {
                output = Some(ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(340.0, 640.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default()
                            .show(ctx, |ui| contributor_panel(ui, &summary));
                    },
                ));
            }
            let output = output.expect("two frames ran");
            let metrics = crate::ui::tokens::Tokens::get(&ctx).metrics;
            let row = f64::from(metrics.row_h - 4.0);
            let tree = output
                .platform_output
                .accesskit_update
                .expect("the panel publishes an accessibility tree");
            let header = tree
                .nodes
                .iter()
                .find(|(_, node)| {
                    node.role() == egui::accesskit::Role::ColumnHeader
                        && node.label() == Some("Source")
                })
                .and_then(|(_, node)| node.bounds())
                .expect("the table names its columns");
            let mut tops: Vec<f64> = tree
                .nodes
                .iter()
                .filter(|(_, node)| {
                    node.value()
                        .or(node.label())
                        .is_some_and(|text| text.starts_with("Contributor rank"))
                })
                .filter_map(|(_, node)| node.bounds())
                .map(|bounds| bounds.y0)
                .collect();
            tops.sort_by(f64::total_cmp);
            assert_eq!(tops.len(), summary.rows.len(), "{density:?}");
            assert!(
                (header.y1 - header.y0 - row).abs() <= 0.5,
                "{density:?}: the header is {:.1} px tall",
                header.y1 - header.y0
            );
            assert!(
                (tops[0] - header.y1).abs() <= 0.5,
                "{density:?}: the first row starts {:.1} px below the header",
                tops[0] - header.y1
            );
            for pair in tops.windows(2) {
                assert!(
                    (pair[1] - pair[0] - row).abs() <= 0.5,
                    "{density:?}: rows {:.1} px apart, wanted {row} px rows that abut",
                    pair[1] - pair[0]
                );
            }
            let mut painted = Vec::new();
            for clipped in &output.shapes {
                if let egui::Shape::Text(text) = &clipped.shape {
                    painted.push(text.galley.job.text.clone());
                }
            }
            assert!(
                painted.iter().any(|text| text == "M1 · flicker"),
                "{density:?}: device and mechanism share one line: {painted:?}"
            );
            // A long device elides in the middle and keeps its leaf and its
            // mechanism.
            assert!(
                painted
                    .iter()
                    .any(|text| text.ends_with("M14 · flicker") && text.contains('…')),
                "{density:?}: the long source lost its leaf or its mechanism: {painted:?}"
            );
            assert!(
                painted.iter().all(|text| !text.contains('\n')),
                "{density:?}: a cell wraps onto a second line"
            );
        }
    }

    /// The contributor panel for review, written under `RSPICE_PICTURES` when
    /// it is set.
    #[test]
    #[ignore = "writes the review pictures"]
    fn write_the_noise_contributor_picture() {
        let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
            return;
        };
        let summary = contributor_summary();
        let canvas = rspice_ui_kit::raster::render_themed(
            crate::ui::Theme::default(),
            egui::vec2(340.0, 420.0),
            |ui, _| contributor_panel(ui, &summary),
        );
        std::fs::write(
            format!("{directory}/noise-contributors.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    }
}
