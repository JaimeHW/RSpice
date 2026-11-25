//! Noise contributor source binding and CSV export at the application boundary.

use super::bode;
use crate::results::visualization_document::{PinnedContributor, SheetSettingValue};
use crate::state::SimulationState;
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use crate::workbench::documents::result_document::{ResultViewer, ResultsParts, ResultsReads};
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

/// The contributors the reader pinned above the rest, as the view keeps
/// them for the analysis shown.
fn pinned(state: ResultsReads<'_>) -> Vec<PinnedContributor> {
    match super::sheet_settings::kept(state, ResultViewer::NoiseContrib) {
        Some(SheetSettingValue::NoiseContributors { pinned }) => pinned,
        _ => Vec::new(),
    }
}

/// Keep the reader's pins in the view, where undo steps them back and the
/// next run of the plan lists the same sources first.
fn keep_pinned(state: &mut ResultsParts<'_>, pinned: Vec<PinnedContributor>) {
    super::sheet_settings::keep(
        state,
        ResultViewer::NoiseContrib,
        (!pinned.is_empty()).then_some(SheetSettingValue::NoiseContributors { pinned }),
    );
}

/// Bind the contributor inspector to the same qualified analysis as the
/// noise spectrum. A contributor the reader opens is shown on the schematic.
pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    bode::noise_spectrum_right_panel(ui, state);
    let index = bode::selected_noise_analysis_index(state.results_reads());
    let evidence = selected_evidence(&state.model.simulation, index);
    let quantity_policy = state.model.preferences.quantity_presentation_policy();
    let digits = usize::from(
        state
            .results_reads()
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    );
    let mut pins = pinned(state.results_reads());
    let response = view::right_panel(ui, evidence, &quantity_policy, digits, &mut pins);
    if response.pins_changed {
        keep_pinned(state, pins);
    }
    if let Some(device) = response.shown {
        show_on_schematic(state, &device);
    }
}

/// Show the part a noise contributor names on the schematic, or say on the
/// status line why it cannot be.
pub(super) fn show_on_schematic(state: &mut ResultsParts<'_>, device: &str) {
    use crate::workbench::commands::schematic_cross_probe::show_noise_contributor_on_schematic;
    if let Err(reason) = show_noise_contributor_on_schematic(state, device) {
        state.workbench.prompt(reason);
    }
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

    /// A stored periodic-noise run over `frequency`: its output density in
    /// `unit`, each contributor's density after it, and the summary the
    /// engine keeps, on a 1 GHz carrier.
    fn stored_periodic_noise_run(
        frequency: Vec<f64>,
        unit: &str,
        densities: Vec<(String, Vec<f64>)>,
        output_quantity: crate::state::PeriodicNoiseOutputQuantity,
        summary: NoiseSummary,
    ) -> SimulationRun {
        stored_plan_run(
            (
                crate::product::SimulationPlanId::new(),
                crate::product::AnalysisInstanceId::new(),
            ),
            frequency,
            unit,
            densities,
            output_quantity,
            summary,
        )
    }

    /// The same, as a run of the authored analysis `ids` names.
    fn stored_plan_run(
        ids: (
            crate::product::SimulationPlanId,
            crate::product::AnalysisInstanceId,
        ),
        frequency: Vec<f64>,
        unit: &str,
        densities: Vec<(String, Vec<f64>)>,
        output_quantity: crate::state::PeriodicNoiseOutputQuantity,
        summary: NoiseSummary,
    ) -> SimulationRun {
        use super::super::test_fixtures::{completed_prepared_run, digest, stored_transient};
        let (mut stored, _) = stored_transient(frequency, densities, Some([0x5e; 32]));
        for waveform in &mut stored.waveforms {
            waveform.unit = Some(unit.to_owned());
        }
        stored.analysis_type = AnalysisType::Pnoise;
        stored.label = "PNOISE".to_owned();
        stored.family_metadata = Some(crate::state::AnalysisResultFamilyMetadata::PeriodicNoise {
            output_quantity,
            carrier_frequency_hz: Some(1.0e9),
        });
        let mut run =
            completed_prepared_run(ids.0, crate::product::ObjectRevision::INITIAL, digest(0x5e));
        run.add_analysis(
            stored.with_noise_summary(summary).with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    ids.1,
                    crate::product::ObjectRevision::INITIAL,
                    digest(0x5f),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            ),
        );
        run
    }

    /// The contributors the reader pins are the view's: the next run of the
    /// plan lists the same sources first, and undo steps a pin back.
    #[test]
    fn the_contributors_the_reader_pins_are_kept_by_the_view() {
        use super::super::view_edits;
        let ids = (
            crate::product::SimulationPlanId::new(),
            crate::product::AnalysisInstanceId::new(),
        );
        let row = |device: &str, mechanism: &str, power: f64| crate::state::NoiseContributorRow {
            device: device.into(),
            mechanism: mechanism.into(),
            power,
            share_pct: 0.0,
        };
        let run = || {
            stored_plan_run(
                ids,
                vec![1.0e3, 1.0e6],
                "V²/Hz",
                vec![("onoise".to_owned(), vec![1.0e-16, 1.0e-16])],
                crate::state::PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
                NoiseSummary {
                    rows: vec![
                        row("R1", "thermal", 3.0e-11),
                        row("M1", "flicker", 2.0e-11),
                        row("M1", "thermal", 1.0e-11),
                    ],
                    band: (1.0e3, 1.0e6),
                    ..NoiseSummary::default()
                },
            )
        };
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run()].into();
        assert!(state.model.simulation.select_run(0));
        state.results.results.session.viewer = super::super::ResultViewer::NoiseContrib;
        state.quick_frame();
        assert!(pinned(state.results_reads()).is_empty());

        let pin = |device: &str, mechanism: &str| PinnedContributor {
            device: device.into(),
            mechanism: mechanism.into(),
        };
        let chosen = vec![pin("M1", "thermal"), pin("R1", "thermal")];
        keep_pinned(&mut state.results_parts(), chosen.clone());
        state.quick_frame();

        // The session forgets them, as a restart does; then the plan runs
        // again.
        super::super::working_view::release(&mut state.results_parts());
        state.results.results.session.sheet_settings.clear();
        state.model.simulation.retained.runs.insert(0, run());
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();
        assert_eq!(pinned(state.results_reads()), chosen);

        keep_pinned(&mut state.results_parts(), chosen[..1].to_vec());
        state.quick_frame();
        let view = state.working_view().expect("the run's view").id();
        view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
            .expect("the unpin is undone");
        state.quick_frame();
        assert_eq!(pinned(state.results_reads()), chosen);
    }

    /// A periodic-noise run referred to the output opens on the Noise sheet:
    /// its V²/Hz density and every contributor's are the spectrum, and the
    /// contributor table lists the run's ranked rows.
    #[test]
    fn a_periodic_noise_run_opens_on_its_spectrum_with_its_ranked_contributors() {
        let frequency = vec![1.0e3, 1.0e4, 1.0e5, 1.0e6];
        let thermal = vec![4.0e-17; 4];
        let flicker: Vec<f64> = frequency.iter().map(|f| 1.0e-12 / f).collect();
        let output = thermal.iter().zip(&flicker).map(|(a, b)| a + b).collect();
        let rows = vec![
            crate::state::NoiseContributorRow {
                device: "R1".into(),
                mechanism: "thermal".into(),
                power: 4.0e-11,
                share_pct: 85.0,
            },
            crate::state::NoiseContributorRow {
                device: "M1".into(),
                mechanism: "flicker".into(),
                power: 6.9e-12,
                share_pct: 15.0,
            },
        ];
        let run = stored_periodic_noise_run(
            frequency,
            "V²/Hz",
            vec![
                ("onoise".to_owned(), output),
                ("noise(M1 flicker)".to_owned(), flicker),
                ("noise(R1 thermal)".to_owned(), thermal),
            ],
            crate::state::PeriodicNoiseOutputQuantity::OutputNoisePowerSpectralDensity,
            NoiseSummary {
                rows: rows.clone(),
                total_rms: Some(6.9e-6),
                band: (1.0e3, 1.0e6),
                ..NoiseSummary::default()
            },
        );
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();

        assert_eq!(
            super::super::opening_sheet(state.results_reads()),
            Some(super::super::ResultViewer::NoiseContrib)
        );
        let index = bode::selected_noise_analysis_index(state.results_reads());
        assert_eq!(index, Some(0));
        let view::ContributorEvidence::Retained { summary, .. } =
            selected_evidence(&state.model.simulation, index)
        else {
            panic!("the run's contributors are bound");
        };
        assert_eq!(summary.rows, rows);
        let model = bode::build_noise_model(state.results_reads()).expect("the spectrum is read");
        assert_eq!(&model.frequency[..], &[1.0e3, 1.0e4, 1.0e5, 1.0e6]);
        assert_eq!(model.total_rms_unit, "V rms");
        assert_eq!(model.jitter, None);
    }

    /// A periodic-noise run referred to the carrier's phase keeps decibels:
    /// it is the Phase Noise sheet's, and the Noise sheet beside it draws
    /// only the run's densities.
    #[test]
    fn a_phase_referenced_periodic_noise_run_stays_off_the_noise_sheet() {
        let offsets = vec![1.0e3, 1.0e4, 1.0e5];
        let mut run = stored_periodic_noise_run(
            offsets.clone(),
            "dBc/Hz",
            vec![
                ("phase_noise".to_owned(), vec![-80.0, -100.0, -120.0]),
                ("noise(R1 thermal)".to_owned(), vec![-90.0, -110.0, -130.0]),
            ],
            crate::state::PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz,
            NoiseSummary {
                band: (1.0e3, 1.0e5),
                ..NoiseSummary::default()
            },
        );
        run.add_analysis(
            AnalysisResult::new(2, AnalysisType::Noise, "NOISE").with_waveforms(vec![
                RetainedTrace::new("onoise", offsets, vec![1.0e-16, 1.0e-17, 1.0e-18], "#fff")
                    .with_unit("V²/Hz"),
            ]),
        );
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        // The data browser picks the analysis; the Noise sheet is the NOISE
        // analysis' sheet, and the phase-referenced run must not join it.
        assert!(state.model.simulation.select_analysis(1));
        state.results.results.session.viewer = super::super::ResultViewer::NoiseContrib;
        state.quick_frame();
        assert_eq!(
            state.results.results.session.viewer,
            super::super::ResultViewer::NoiseContrib
        );

        let tokens = crate::ui::tokens::Tokens::default();
        let presentation = state.model.preferences.result_presentation_policy();
        let models = super::super::waves::cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &tokens,
        );
        let drawn: Vec<_> = models.iter().map(|model| model.analysis_type).collect();
        assert_eq!(drawn, [AnalysisType::Noise]);
    }

    /// A periodic-noise run sampled at an edge's timing reads as that edge's
    /// jitter: a flat s²/Hz density S to half the carrier integrates to
    /// √(SΔf), and Jc and Jcc are the closed forms of its weighted integrals.
    #[test]
    fn a_timing_noise_run_reads_its_edge_jitter_where_the_closed_form_puts_it() {
        use rspice_core::analysis::pnoise::{
            PeriodicNoiseSamplePoint, PeriodicNoiseSampling, PeriodicNoiseSamplingEvidence,
        };
        let (carrier, density) = (1.0e9_f64, 1.0e-34_f64);
        let frequency = vec![1.0e3, 1.0e4, 1.0e5, 1.0e6, 1.0e7, 1.0e8, 5.0e8];
        let sampling = PeriodicNoiseSamplingEvidence {
            carrier_frequency_hz: carrier,
            request: PeriodicNoiseSampling::Edge {
                edge: Default::default(),
            },
            output: PeriodicNoiseSamplePoint {
                node: "clk".into(),
                reference: None,
                phase_degrees: 0.0,
                voltage: 0.5,
                slew_volts_per_second: 1.0e10,
            },
            reference: None,
            nominal_delay_seconds: None,
        };
        let run = stored_periodic_noise_run(
            frequency.clone(),
            "s²/Hz",
            vec![("onoise".to_owned(), vec![density; frequency.len()])],
            crate::state::PeriodicNoiseOutputQuantity::TimingNoisePowerSpectralDensity,
            NoiseSummary {
                conversion: Some(crate::state::PeriodicNoiseConversionEvidence {
                    sampling: Some(sampling),
                    input_source: String::new(),
                    carrier_hz: carrier,
                    input_sideband: 0,
                    output_sideband: 0,
                    max_sideband: 4,
                }),
                band: (1.0e3, 5.0e8),
                ..NoiseSummary::default()
            },
        );
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state.quick_frame();

        assert_eq!(
            super::super::opening_sheet(state.results_reads()),
            Some(super::super::ResultViewer::NoiseContrib)
        );
        let model = bode::build_noise_model(state.results_reads()).expect("the spectrum is read");
        let jitter = model.jitter.expect("a timing run reads as jitter");
        let (low, high) = (1.0e3, carrier / 2.0);
        let close = |value: f64, expected: f64| (value / expected - 1.0).abs() < 1.0e-6;
        let rms = jitter.rms_s.expect("the band is inside the sweep");
        assert!(close(rms, (density * (high - low)).sqrt()), "{rms}");

        // ∫ sin²(af) = f/2 − sin(2af)/4a; ∫ sin⁴(af) = 3f/8 − sin(2af)/4a + sin(4af)/32a.
        let a = std::f64::consts::PI / carrier;
        let sin2 = |f: f64| f / 2.0 - (2.0 * a * f).sin() / (4.0 * a);
        let sin4 = |f: f64| {
            3.0 * f / 8.0 - (2.0 * a * f).sin() / (4.0 * a) + (4.0 * a * f).sin() / (32.0 * a)
        };
        let cycle = jitter.cycle.expect("the sweep reaches half the carrier");
        let jc = (4.0 * density * (sin2(high) - sin2(low))).sqrt();
        let jcc = (16.0 * density * (sin4(high) - sin4(low))).sqrt();
        assert!(close(cycle.cycle_s, jc), "{} vs {jc}", cycle.cycle_s);
        assert!(
            close(cycle.cycle_to_cycle_s, jcc),
            "{} vs {jcc}",
            cycle.cycle_to_cycle_s
        );
    }
}
