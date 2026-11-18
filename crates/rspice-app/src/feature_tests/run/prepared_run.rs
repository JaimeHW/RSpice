//! The prepared-run cases that read the code slot.

use rspice_simulation::netlist_preparation::reject_deferred_external_sources_with_project_runtimes;
use rspice_simulation::project_veriloga::preparation::project_veriloga_runtimes_referenced_by;

use crate::workbench::app_state::AppState;
use rspice_app_state::workbench::app_state::model::compile_project_bundle_receipt;
use rspice_simulation::execution::PreparationStage;

#[test]
fn case_altered_project_veriloga_key_is_rejected_before_dispatch() {
    let mut state = AppState::default();
    let owner = rspice_app_state::state::ProjectSourceOwner::cell_view(
        rspice_app_state::state::CellViewRef::new("behavioral", "owned", "veriloga"),
    );
    state
        .model.workspace
        .content
        .insert_project_source_bundle(
            rspice_app_state::state::ProjectSourceBundle::try_new(
                owner.clone(),
                "model.va",
                "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                [],
                [],
            )
            .expect("a valid cell-view bundle"),
        )
        .expect("install the cell view's Verilog-A source");
    let bundle = state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("installed project source bundle");
    let receipt =
        compile_project_bundle_receipt(state.model.workspace.content.project.id(), bundle, None)
            .expect("compile project Verilog-A source");
    state.code.code_workspace.veriloga.receipt = Some(receipt);

    let bundle = state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("installed project source bundle");
    let source_key = rspice_design::project_sources::project_veriloga_bundle_source_key(
        state.model.workspace.content.project.id(),
        bundle,
        "owned",
    )
    .expect("derive exact project source key");
    let exact_directive =
        rspice_simulation::netlist_preparation::project_veriloga_directive(&source_key, "owned");
    let exact_runtimes = project_veriloga_runtimes_referenced_by(
        state.model.workspace.content.project.id(),
        &state.model.workspace.content.project_sources,
        state
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| &receipt.compilation),
        &exact_directive,
    )
    .expect("inspect exact project directive");
    assert_eq!(exact_runtimes.len(), 1);
    reject_deferred_external_sources_with_project_runtimes(
        &exact_directive,
        &exact_runtimes,
        &Default::default(),
    )
    .expect("exact project identity is permitted");

    let altered_key = source_key.replacen("__rspice_project__", "__RSPICE_PROJECT__", 1);
    let altered_directive =
        rspice_simulation::netlist_preparation::project_veriloga_directive(&altered_key, "owned");
    let altered_runtimes = project_veriloga_runtimes_referenced_by(
        state.model.workspace.content.project.id(),
        &state.model.workspace.content.project_sources,
        state
            .code
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .map(|receipt| &receipt.compilation),
        &altered_directive,
    )
    .expect("inspect altered project directive");
    assert!(
        altered_runtimes.is_empty(),
        "case-altered virtual paths must not acquire the exact project runtime"
    );
    let error = reject_deferred_external_sources_with_project_runtimes(
        &altered_directive,
        &altered_runtimes,
        &Default::default(),
    )
    .expect_err("case-altered project key must remain an external dependency");
    assert_eq!(error.stage(), PreparationStage::SourceChecks);
    assert!(error.message().contains("unsealed external dependency"));
}
