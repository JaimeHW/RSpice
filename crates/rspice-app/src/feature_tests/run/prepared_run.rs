//! The prepared-run cases that read the code slot or import a deck through
//! the netlist workflow.

#[cfg(not(target_arch = "wasm32"))]
use std::fs;

use rspice_simulation::netlist_preparation::reject_deferred_external_sources_with_project_runtimes;
use rspice_simulation::project_veriloga::preparation::project_veriloga_runtimes_referenced_by;

use super::session_of;
use crate::simulation::controller::SimulationController;
#[cfg(not(target_arch = "wasm32"))]
use crate::simulation::controller::prepared_run::tests::fixture_dir;
use crate::simulation::controller::prepared_run::tests::manual_deck_state;
use crate::simulation::execution::PreparationStage;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::compile_project_bundle_receipt;
use crate::workbench::workflows::netlist_workflow::apply_imported_netlist;

#[test]
fn case_altered_project_veriloga_key_is_rejected_before_dispatch() {
    let mut state = AppState::default();
    let owner = crate::state::ProjectSourceOwner::cell_view(crate::state::CellViewRef::new(
        "behavioral",
        "owned",
        "veriloga",
    ));
    state
        .model.workspace
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

#[test]
fn reviewed_foreign_profile_is_applied_when_the_owned_deck_is_prepared() {
    let source = "reviewed export\nsimulator lang=spice\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut state = session_of(manual_deck_state(source));
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "reviewed.cir",
    ));
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spectre);
    descriptor.execution_profile = Some(crate::state::NetlistExecutionProfile::SpectreSpiceV1);
    descriptor.compatibility_reviewed = true;
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("the accepted adapter must also reach run preparation");
    let dispatch = controller
        .consume_snapshot_for_dispatch(&state)
        .expect("validated source dispatches");
    assert_eq!(dispatch.manual_source(), Some(source));
    let parsed =
        rspice_core::Netlist::parse(dispatch.executable_netlist()).expect("prepared source parses");
    assert_eq!(parsed.elements.len(), 2);
    assert_eq!(parsed.analyses.len(), 1);
    assert!(
        dispatch
            .executable_netlist()
            .contains("presentation directive: simulator lang=spice")
    );
}

#[test]
fn an_unreviewed_owned_profile_cannot_acquire_a_manual_execution_permit() {
    let source = "quarantined\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut state = session_of(manual_deck_state(source));
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "quarantined.cir",
    ));
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
    descriptor.execution_profile = None;
    descriptor.compatibility_reviewed = false;
    let mut controller = SimulationController::new();
    assert!(controller.validate_manual_deck_document(&state).is_err());
    assert!(controller.run_authorization.retained_snapshot().is_none());
}

#[test]
fn reviewed_ngspice_profile_binds_engine_defaults_and_revalidates_edits() {
    let source = "reviewed control\nV1 out 0 1\nR1 out 0 1k\n.control; analyses\nop; harmless note\n.endc\n.end\n";
    let mut state = session_of(manual_deck_state(source));
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "reviewed.cir"
    ));
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
    descriptor.execution_profile = Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2);
    descriptor.compatibility_reviewed = true;
    let mut controller = SimulationController::new();
    controller.validate_manual_deck_document(&state).unwrap();
    let dispatch = controller.consume_snapshot_for_dispatch(&state).unwrap();
    assert_eq!(dispatch.manual_source(), Some(source));
    assert!(
        dispatch
            .executable_netlist()
            .contains("RSpice execution profile: spice3-ngspice/2")
    );
    let parsed = rspice_core::Netlist::parse(dispatch.executable_netlist()).unwrap();
    let config = rspice_simulation_contract::options::SimulationOptions::default()
        .resolve_simulation_config(Some(&parsed.options));
    assert_eq!(config.spice_dialect, rspice_core::SpiceDialect::Ngspice);
    assert_eq!(
        config.resolved_jfet_level2_model(),
        rspice_core::SpiceDialect::Ngspice.default_jfet_level2_model()
    );
    for edit in [
        source.replace("op; harmless note", "wrdata out.txt v(out)"),
        source.replace(
            ".control; analyses",
            ".OPTIONS RSPICE_DIALECT=XYCE\n.control",
        ),
        source.replace(".control; analyses", ".unsupported_card\n.control"),
    ] {
        state.model.workspace.content.netlist_source = Some(edit);
        assert!(controller.validate_manual_deck_document(&state).is_err());
        assert!(controller.run_authorization.retained_snapshot().is_none());
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn reviewed_spectre_header_survives_path_based_dependency_expansion() {
    let directory = fixture_dir("reviewed-spectre-header");
    let source = "simulator lang=spice\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let profile = crate::state::NetlistExecutionProfile::SpectreSpiceV1;
    for extension in ["cir", "scs"] {
        let path = directory.join(format!("export.{extension}"));
        fs::write(&path, source).unwrap();
        let adapted = profile.adapt_source(source).unwrap();
        let parsed = rspice_core::Netlist::parse_with_path(&adapted, &path).unwrap();
        profile.validate_parsed_netlist(&parsed).unwrap();
        let mut state = session_of(manual_deck_state(source));
        assert!(apply_imported_netlist(
            &mut state,
            source.to_owned(),
            Some(path),
            "export"
        ));
        let descriptor = state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_mut()
            .unwrap();
        descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spectre);
        descriptor.execution_profile = Some(profile);
        descriptor.compatibility_reviewed = true;
        let mut controller = SimulationController::new();
        controller.validate_manual_deck_document(&state).unwrap();
        let dispatch = controller.consume_snapshot_for_dispatch(&state).unwrap();
        assert_eq!(dispatch.manual_source(), Some(source));
        let parsed = rspice_core::Netlist::parse(dispatch.executable_netlist()).unwrap();
        assert_eq!(parsed.elements.len(), 2);
        assert_eq!(parsed.analyses.len(), 1);
        assert_eq!(parsed.options.spice_dialect, Some(profile.spice_dialect()));
    }
    fs::remove_dir_all(directory).unwrap();
}
