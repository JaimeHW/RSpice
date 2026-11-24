//! What a signal's menu takes from it: the calculator holding it, its
//! spectrum, and a plot of it against another signal of its analysis.

use rspice_results::result_presentation::{ResultViewer, SourceWaveformPresentationKey};
use rspice_results::visualization_document::Abscissa;
use rspice_results_ui::selection::ResultBrowserSelectionKey;
use rspice_results_ui::session::XyStrip;

use super::super::results_actions::{plot_against, select_result_signal_by_key};
use super::browser::finished_hierarchical_run;
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace};
use crate::workbench::documents::result_document::browser_plot::result_quantity_ticked;
use crate::workbench::documents::result_document::{AnalysisPresentationKey, ResultsTestHost};
use crate::workbench::results::commands::ResultsCommand;

/// `name` in the first analysis of the run the host holds.
fn signal(host: &ResultsTestHost, name: &str) -> SourceWaveformPresentationKey {
    let run = host.model.simulation.active_run().expect("a run");
    SourceWaveformPresentationKey::new(
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
        name,
    )
}

#[test]
fn a_signal_sent_to_the_calculator_opens_it_holding_the_signal_and_its_spectrum_reads_it() {
    let mut host = finished_hierarchical_run();
    host.workbench
        .activate(crate::workbench::state::Workspace::Results);
    let input = signal(&host, "V(in)");
    assert!(select_result_signal_by_key(
        &mut host.results_parts(),
        &input
    ));

    host.run_command(ResultsCommand::SendTraceToCalculator);
    assert!(host.results.waveform_calculator_dialog);
    assert_eq!(host.results.calculator_panel.expression, "V(in)");

    host.run_command(ResultsCommand::TraceSpectrum);
    assert_eq!(host.results.results.session.viewer, ResultViewer::Fft);
    assert_eq!(
        host.results.analysis.fft_state.selected_source.as_deref(),
        Some("V(in)"),
        "the spectrum is taken of the signal, not of the analysis' first"
    );
}

#[test]
fn a_signal_plotted_against_another_is_drawn_against_it_in_a_strip_of_its_own() {
    let mut host = finished_hierarchical_run();
    let out = signal(&host, "V(out)");
    let analysis = out.analysis();

    plot_against(&mut host.results_parts(), &out, "V(in)").unwrap();
    plot_against(&mut host.results_parts(), &out, "V(in)").unwrap();
    assert_eq!(
        *host.results.results.session.xy_strips,
        [XyStrip::new(
            analysis,
            "V",
            Abscissa::Signal {
                name: "V(in)".to_owned()
            }
        )],
        "the volts pane against V(in), made once however often it is asked for"
    );
    assert!(
        result_quantity_ticked(
            &host.results_parts(),
            &ResultBrowserSelectionKey::Waveform(out.clone())
        ),
        "the signal is plotted, so the strip has its curve to draw"
    );
    assert_eq!(
        plot_against(&mut host.results_parts(), &out, "V(nowhere)"),
        Err("V(nowhere) is not a signal of this analysis.".to_owned())
    );

    let mut ac = ResultsTestHost::default();
    let trace = |name: &str| RetainedTrace::new(name, vec![1.0, 10.0], vec![0.0, 1.0], "#fff");
    ac.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_waveforms(vec![trace("V(out)"), trace("V(in)")]),
    );
    let out = signal(&ac, "V(out)");
    assert_eq!(
        plot_against(&mut ac.results_parts(), &out, "V(in)"),
        Err("A signal is plotted against another in a transient or DC sweep.".to_owned())
    );
}
