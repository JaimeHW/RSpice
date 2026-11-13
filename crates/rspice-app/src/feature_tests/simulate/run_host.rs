//! The run tier's host frozen for a campaign: what the frozen host records
//! stays off the live results and runs it was frozen from.

use crate::simulation::{RunHost, ViewerHost};
use crate::state::{RunEvent, RunEventSink};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::model::compile_project_bundle_receipt;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::documents::netlist_document::ActiveNetlistDocument;
use crate::workbench::framework::feature::LowerHandles;
use crate::workbench::framework::feature::{Cx, Peers};
use crate::workbench::simulate::run_host::{FrozenRun, run_host};
use std::sync::Arc;

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

/// What code shows reaches the run tier's host when the host is built, and
/// the host's frozen copy keeps it: the compilation receipt code holds and
/// the netlist document its editor shows.
#[test]
fn a_frozen_host_keeps_the_compilation_and_the_netlist_code_shows() {
    let mut app = RSpiceApp::test_instance();
    let owner = crate::state::ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
        "behavioral",
        "owned",
        "veriloga",
    ));
    app.state
        .model
        .workspace
        .content
        .insert_project_source_bundle(
            crate::state::ProjectSourceBundle::try_new(
                owner.clone(),
                crate::state::ProjectSourceLanguage::VerilogA,
                "model.va",
                "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                [],
                [],
            )
            .expect("a valid cell-view bundle"),
        )
        .expect("install the cell view's Verilog-A source");
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("installed project source bundle");
    let receipt = compile_project_bundle_receipt(
        app.state.model.workspace.content.project.id(),
        bundle,
        None,
    )
    .expect("compile project Verilog-A source");
    app.state.code.code_workspace.veriloga.receipt = Some(receipt);
    app.state.code.netlist.active_document = ActiveNetlistDocument::GeneratedDiff;
    app.state.code.netlist.active_document_initialized = true;

    let frozen = frozen_from(&mut app);

    let shown = frozen.netlist_shown();
    assert!(shown.settled && shown.comparison && !shown.owned_source);
    let held = app
        .state
        .code
        .veriloga_compilation()
        .expect("code holds the compilation");
    let kept = frozen
        .veriloga_compilation()
        .expect("the frozen host keeps the compilation");
    assert!(Arc::ptr_eq(kept.report(), held.report()));
}
