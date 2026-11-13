//! The run tier's host frozen for a campaign: what the frozen host records
//! stays off the live results and runs it was frozen from.

use crate::simulation::{RunHost, ViewerHost};
use crate::state::{RunEvent, RunEventSink};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::feature::LowerHandles;
use crate::workbench::framework::feature::{Cx, Peers};
use crate::workbench::simulate::run_host::{FrozenRun, run_host};

/// What the run tier's host over simulate's context over `app` freezes,
/// lending it the application's parts as the frame does.
fn frozen_from(app: &mut RSpiceApp) -> FrozenRun {
    let handles = LowerHandles {
        export_io: app.export_workflow_io.as_ref(),
        run_controller: &mut app.simulation_controller,
        model_hub: &mut app.model_hub,
        symbols: app.symbol_library.as_ref(),
    };
    let state = &mut app.state;
    let mut cx = Cx::new(
        &mut *state.simulate,
        &mut [],
        &mut state.model,
        &mut *state.workbench,
        &mut state.platform,
        &mut state.runs,
        handles,
        Peers::new(
            Some(&mut *state.design),
            Some(&*state.cloud),
            Some(&*state.hardcopy),
            Some(&mut *state.results),
            None,
            Some(&*state.code),
            Some(&*state.models),
        ),
        command_registry(),
    );
    let (host, _) = run_host(&mut cx);
    host.freeze()
}

#[test]
fn a_campaigns_frozen_host_leaves_the_live_results_and_runs_untouched() {
    let mut app = RSpiceApp::test_instance();
    let live_source = app.state.results.analysis.fft_state.selected_source.clone();
    let live_rerun = app.state.runs.manual_run.rerun_queued;
    let mut frozen = frozen_from(&mut app);

    frozen.apply_run_event(RunEvent::FftSourceChosen {
        source: "V(out)".to_owned(),
    });
    frozen.runs_mut().manual_run.rerun_queued = !live_rerun;

    assert_eq!(
        frozen.viewer_reading().fft_source.as_deref(),
        Some("V(out)")
    );
    assert_eq!(frozen.runs().manual_run.rerun_queued, !live_rerun);
    assert_eq!(
        app.state.results.analysis.fft_state.selected_source,
        live_source
    );
    assert_eq!(app.state.runs.manual_run.rerun_queued, live_rerun);
}
