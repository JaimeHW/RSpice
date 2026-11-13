//! The netlist workflow through the whole application: imports staged,
//! reviewed and committed over a project replacement, decks opened as
//! netlist-first projects, and imported decks applied to the open one.

use crate::workbench::app_state::AppState;
use crate::workbench::workflows::netlist_workflow::bundle::parse_generated_netlist_bundle;
use crate::workbench::workflows::netlist_workflow::tests::generated_bundle_fixture;
use crate::workbench::workflows::netlist_workflow::{
    NetlistImportMetadata, NetlistImportMode, cancel_staged_netlist_import,
    save_owned_netlist_source, sha256,
};

#[test]
fn launcher_drop_stages_a_new_netlist_project_review() {
    let mut state = AppState::default();
    let source = b"dropped deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

    assert!(state.stage_dropped_netlist_project(source.to_vec(), None, "dropped.cir".to_owned()));

    let review = state
        .code
        .netlist
        .import_review
        .as_ref()
        .expect("launcher drop owns a staged review");
    assert_eq!(
        review.operation,
        crate::workbench::documents::netlist_document::NetlistImportOperation::OpenProject
    );
    assert_eq!(review.display_name, "dropped.cir");
    assert_eq!(review.source.as_bytes(), source);
    assert_eq!(review.blocking_issue_count(), 0);
}

#[test]
fn save_as_preserves_unvalidated_work_and_can_convert_legacy_source_to_utf8() {
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    #[derive(Debug)]
    struct CaptureIo {
        bytes: Rc<RefCell<Vec<u8>>>,
    }

    impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for CaptureIo {
        fn show_save_dialog(
            &self,
            _config: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(Some(PathBuf::from("converted.cir")))
        }

        fn write_text_file(&self, _path: &Path, contents: &str) -> Result<(), String> {
            self.bytes.replace(contents.as_bytes().to_vec());
            Ok(())
        }

        fn write_bytes_file_observed(
            &self,
            _destination: &crate::workbench::workflows::export_workflow::ObservedExportDestination,
            contents: &[u8],
            _mime_type: &str,
        ) -> Result<(), String> {
            self.bytes.replace(contents.to_vec());
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            false
        }
    }

    let mut app = crate::workbench::RSpiceApp::test_instance();
    let latin1_source = "* caf\u{00e9}\nV1 out 0 1\n.op\n.end\n";
    assert!(
        app.state
            .apply_imported_netlist(latin1_source.to_owned(), None, "legacy.cir")
    );
    app.state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap()
        .source_encoding = crate::state::NetlistTextEncoding::Latin1;
    let utf8_source = "* caf\u{00e9} and \u{03a9}\nV1 out 0 1\n.op\n.end\n";
    assert!(
        crate::workbench::documents::netlist_document::replace_owned_source(
            &mut app.state.model,
            &mut app.state.code.netlist,
            &mut app.state.code.netlist_open_documents,
            &mut app.state.workbench,
            utf8_source.to_owned()
        )
    );
    assert!(app.state.model.netlist_artifacts.validation.is_none());
    let imported_digest = app.state.code.netlist.externally_saved_content_digest;
    let bytes = Rc::new(RefCell::new(Vec::new()));
    let io = CaptureIo {
        bytes: Rc::clone(&bytes),
    };

    assert!(save_owned_netlist_source(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.workbench,
        &io,
        true,
        crate::state::NetlistTextEncoding::Utf8,
        "",
    ));

    assert_eq!(bytes.borrow().as_slice(), utf8_source.as_bytes());
    assert!(app.state.model.netlist_artifacts.validation.is_none());
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .source_encoding,
        crate::state::NetlistTextEncoding::Latin1,
    );
    assert_eq!(
        app.state.code.netlist.externally_saved_content_digest, imported_digest,
        "a browser download is an independent copy, not a canonical save acknowledgement"
    );
}

#[test]
fn authenticated_generated_bundle_stages_and_commits_retained_closure() {
    let bytes = generated_bundle_fixture();
    let parsed = parse_generated_netlist_bundle(&bytes).expect("authenticated bundle");
    assert_eq!(parsed.dependencies.len(), 2);
    assert_eq!(
        parsed.dependencies[0].authority(),
        crate::state::DependencySourceAuthority::Vendor
    );
    assert_eq!(
        parsed.dependencies[1].authority(),
        crate::state::DependencySourceAuthority::TechnologyPackage
    );
    assert!(parsed.expanded_source.contains(".model base nmos level=1"));
    assert!(
        parsed
            .expanded_source
            .contains(".model core nmos level=1 vto=0.45")
    );
    assert!(
        !parsed
            .expanded_source
            .to_ascii_lowercase()
            .contains(".include")
    );

    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        bytes,
        None,
        "portable-run.zip".to_owned()
    ));
    let review = state.code.netlist.import_review.as_ref().unwrap();
    assert!(review.archive_import);
    assert_eq!(review.display_name, "portable-run.spice");
    assert!(review.source_path.is_none());
    assert_eq!(review.dependencies.len(), 2);

    assert!(state.commit_staged_netlist_import());
    assert!(state.model.workspace.content.netlist_source_path.is_none());
    assert_eq!(
        state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.as_str()),
        Some("portable-run.spice")
    );
    let document = state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert!(document.dependency_graph_is_sealed());
    assert_eq!(document.dependencies().len(), 2);
}

#[test]
fn staged_import_is_cancel_safe_and_commits_only_the_reviewed_snapshot() {
    let mut state = AppState::default();
    let original_project = state.model.workspace.content.project.id();
    let original_source = state.model.workspace.content.netlist_source.clone();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* staged\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();

    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source.clone(),
        None,
        "staged.cir".to_owned()
    ));
    assert_eq!(state.model.workspace.content.project.id(), original_project);
    assert_eq!(
        state.model.workspace.content.netlist_source,
        original_source
    );
    assert!(state.code.netlist.import_review.is_some());

    assert!(state.commit_staged_netlist_import());
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        std::str::from_utf8(&source).ok()
    );
    assert!(state.code.netlist.import_review.is_none());
    assert_eq!(
        state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.imported_dialect),
        Some(crate::state::NetlistSourceDialect::RSpice)
    );
}

#[test]
fn owned_netlist_history_compare_and_restore_are_persisted_and_monotonic() {
    let original = "* baseline\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let modified = "* modified\nV1 out 0 2\nR1 out 0 2k\n.op\n.end\n";
    let mut state = AppState::default();
    assert!(state.apply_imported_netlist(original.to_owned(), None, "history.cir"));
    let baseline_revision = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .revision_history[0]
        .document_revision;
    assert!(
        crate::workbench::documents::netlist_document::replace_owned_source(
            &mut state.model,
            &mut state.code.netlist,
            &mut state.code.netlist_open_documents,
            &mut state.workbench,
            modified.to_owned()
        )
    );
    let modified_revision = state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap()
        .revision()
        .get();
    assert!(modified_revision > baseline_revision);

    crate::workbench::documents::netlist_document::compare_owned_revision(
        &mut state.model,
        &mut state.code.netlist,
        0,
    )
    .expect("comparison opens");
    assert!(
        state
            .model
            .simulation
            .source
            .netlist_content
            .contains("-V1 out 0 1")
    );
    crate::workbench::documents::netlist_document::close_revision_comparison(
        &mut state.model,
        &mut state.code.netlist,
    );
    assert_eq!(state.model.simulation.source.netlist_content, modified);

    crate::workbench::documents::netlist_document::restore_owned_revision(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        0,
    )
    .expect("history restore commits");
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some(original)
    );
    let restored_revision = state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap()
        .revision()
        .get();
    assert!(restored_revision > modified_revision);
    let history = &state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .revision_history;
    assert_eq!(history.len(), 3);
    assert_eq!(history[1].source, modified);
    assert_eq!(history[2].source, original);
    state
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .expect("restored workspace validates");

    let persisted = serde_json::to_vec(&state.model.workspace).expect("serialize workspace");
    let restored: crate::state::ProjectWorkspace =
        serde_json::from_slice(&persisted).expect("deserialize workspace");
    restored
        .content
        .validate_simulation_configuration()
        .expect("persisted history validates");
    assert_eq!(
        restored
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history,
        *history
    );
}

/// The import review starts from the dialect Preferences sets only when the
/// deck names none; markers in the deck win over the preference.
#[test]
fn a_deck_naming_no_dialect_starts_from_the_preferred_one() {
    for (source, expected) in [
        (
            &b"* plain\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n"[..],
            crate::state::NetlistSourceDialect::Spice3Ngspice,
        ),
        (
            &b"* hspice deck\n.option post=2\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n"[..],
            crate::state::NetlistSourceDialect::Hspice,
        ),
    ] {
        let mut state = AppState::default();
        state
            .model
            .preferences
            .set_choice(
                crate::workbench::ChoicePreference::ImportedNetlistDialect,
                1,
            )
            .expect("SPICE3 / ngspice is an importable dialect");
        let transaction =
            crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
                .expect("replacement transaction starts");
        assert!(state.stage_netlist_import(
            transaction,
            NetlistImportMode::ImportIntoProject,
            source.to_vec(),
            None,
            "deck.cir".to_owned()
        ));
        let review = state.code.netlist.import_review.as_ref().unwrap();
        assert_eq!(review.selected_dialect, expected);
    }
}

#[test]
fn detected_foreign_dialect_requires_explicit_acceptance() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* hspice deck\n.option post=2\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    let retained_source = String::from_utf8(source.clone()).unwrap();

    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "foreign.sp".to_owned()
    ));
    let review = state.code.netlist.import_review.as_ref().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Hspice
    );
    assert!(!review.can_commit());
    assert_eq!(review.blocking_issue_count(), 0);

    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_ok());
    assert!(review.can_commit());
    review.selected_dialect = crate::state::NetlistSourceDialect::RSpice;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());
    review.selected_dialect = crate::state::NetlistSourceDialect::Hspice;
    assert!(state.commit_staged_netlist_import());
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some(retained_source.as_str())
    );
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Hspice)
    );
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::HspiceDeclarativeV1)
    );
}

#[test]
fn only_source_authenticated_import_dialects_can_commit_after_acceptance() {
    use crate::state::NetlistSourceDialect;

    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* standard subset\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "standard-subset.cir".to_owned()
    ));

    {
        let review = state.code.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::Spice3Ngspice;
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_ok());
        assert!(review.can_commit());
    }

    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Hspice;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Spectre;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Ads;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let dialect = NetlistSourceDialect::Unknown;
    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = dialect;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err(), "{dialect:?}");
    assert!(!review.can_commit(), "{dialect:?}");

    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Pspice;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::RSpice;
    review.compatibility_accepted = false;
    assert!(review.dialect_qualification().is_ok());
    assert!(review.can_commit());
    cancel_staged_netlist_import(&mut state.code.netlist, &mut state.platform);
}

#[test]
fn spectre_spice_interoperability_import_preserves_source_and_profile() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"simulator lang=spice\n* SPICE interoperability deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    let retained_source = String::from_utf8(source.clone()).unwrap();

    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "spectre-spice.scs".to_owned()
    ));
    let review = state.code.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Spectre
    );
    assert_eq!(review.blocking_issue_count(), 0);
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(state.commit_staged_netlist_import());
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some(retained_source.as_str())
    );
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Spectre)
    );
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::SpectreSpiceV1)
    );
}

#[test]
fn ads_spice_export_import_preserves_source_and_profile() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"Options ResourceUsage=yes UseNutmegFormat=no TopDesignName=\"divider\"\n* ADS SPICE export\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    let retained_source = String::from_utf8(source.clone()).unwrap();

    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "ads-spice-export.net".to_owned()
    ));
    let review = state.code.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Ads
    );
    assert_eq!(review.blocking_issue_count(), 0);
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(state.commit_staged_netlist_import());
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some(retained_source.as_str())
    );
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Ads)
    );
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::AdsSpiceExportV1)
    );
}

#[test]
fn qualified_spice3_ngspice_import_persists_exact_versioned_profile() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* declarative SPICE3 subset\n.param gain=2\nV1 in 0 1\nB1 out 0 V=log(exp({gain}))\nR1 out 0 1k\n.control\nop\nsave v(out)\n.endc\n.end\n".to_vec();
    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "declarative-spice3.cir".to_owned()
    ));
    let review = state.code.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Spice3Ngspice
    );
    assert_eq!(review.blocking_issue_count(), 0);
    review.selected_dialect = crate::state::NetlistSourceDialect::Spice3Ngspice;
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(state.commit_staged_netlist_import());

    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Spice3Ngspice)
    );
    assert!(descriptor.compatibility_reviewed);
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
    );
    let persisted = serde_json::to_vec(&state.model.workspace).unwrap();
    let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&persisted).unwrap();
    assert_eq!(
        restored
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.execution_profile),
        Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
    );
}

#[test]
fn qualified_pspice_import_persists_exact_versioned_profile() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* PSpice declarative subset\nV1 in 0 AC 1\nE1 out 0 CHEBYSHEV {V(in)} = BP (800Hz 1.2kHz 2kHz 3kHz) .1dB 50dB\nR1 out 0 1k\n.probe V(out)\n.ac dec 10 100 10k\n.end\n"
            .to_vec();
    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "declarative-pspice.cir".to_owned()
    ));
    let review = state.code.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Pspice
    );
    assert!(
        review
            .transformations
            .iter()
            .any(|entry| entry.contains("CHEBYSHEV"))
    );
    review.compatibility_accepted = true;
    assert!(
        review.can_commit(),
        "issues={:?}; qualification={:?}",
        review.issues,
        review.dialect_qualification()
    );
    assert!(state.commit_staged_netlist_import());

    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Pspice)
    );
    assert!(descriptor.compatibility_reviewed);
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::PspiceDeclarativeV2)
    );
    let persisted = serde_json::to_vec(&state.model.workspace).unwrap();
    let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&persisted).unwrap();
    assert_eq!(
        restored
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.execution_profile),
        Some(crate::state::NetlistExecutionProfile::PspiceDeclarativeV2)
    );
}

fn quarantined_owned_ngspice_state() -> AppState {
    let source = "* retained declarative source\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut state = AppState::default();
    state.open_project_session();
    assert!(state.apply_imported_netlist(source.to_owned(), None, "legacy-ngspice.cir"));
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
    descriptor.compatibility_reviewed = false;
    descriptor.execution_profile = None;
    state.model.workspace.content.netlist_source_dirty = false;
    state.model.workspace.content.project_metadata_dirty = false;
    state
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .expect("quarantined source remains a valid, non-executable project");
    state
}

#[test]
fn quarantined_owned_source_requalifies_without_external_file_or_source_replacement() {
    let mut state = quarantined_owned_ngspice_state();
    let source = state.model.workspace.content.netlist_source.clone();
    let history = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .revision_history
        .clone();

    assert!(state.begin_owned_netlist_profile_review());
    let review = state.code.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.operation,
        crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource
    );
    assert!(review.selected_file_path.is_none());
    assert_eq!(review.source, source.as_deref().unwrap());
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(state.commit_staged_netlist_import());

    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
    );
    assert!(descriptor.compatibility_reviewed);
    assert!(!descriptor.execution_profile_review_required());
    assert_eq!(descriptor.revision_history, history);
    assert_eq!(state.model.workspace.content.netlist_source, source);
    assert!(state.model.workspace.content.project_metadata_dirty);
    assert!(state.code.netlist.import_review.is_none());
}

#[test]
fn owned_profile_review_rejects_source_revision_change_transactionally() {
    let mut state = quarantined_owned_ngspice_state();
    assert!(state.begin_owned_netlist_profile_review());
    state
        .code
        .netlist
        .import_review
        .as_mut()
        .unwrap()
        .compatibility_accepted = true;
    assert!(
        crate::workbench::documents::netlist_document::replace_owned_source(
            &mut state.model,
            &mut state.code.netlist,
            &mut state.code.netlist_open_documents,
            &mut state.workbench,
            "* changed after review\nV1 out 0 2\nR1 out 0 1k\n.op\n.end\n".to_owned()
        )
    );

    assert!(!state.commit_staged_netlist_import());
    let descriptor = state
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    assert!(descriptor.execution_profile.is_none());
    assert!(!descriptor.compatibility_reviewed);
    assert!(
        state
            .code
            .netlist
            .import_review
            .as_ref()
            .and_then(|review| review.error.as_deref())
            .is_some_and(|error| error.contains("project changed"))
    );
}

#[test]
fn imperative_ngspice_control_command_cannot_use_declarative_profile() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source =
        b"* imperative ngspice\nV1 out 0 1\n.control\nop\nwrdata out.txt v(out)\n.endc\n.end\n"
            .to_vec();
    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "imperative-ngspice.cir".to_owned()
    ));
    let review = state.code.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = crate::state::NetlistSourceDialect::Spice3Ngspice;
    review.compatibility_accepted = true;
    let error = review.dialect_qualification().unwrap_err();
    assert!(error.contains("spice3-ngspice/2"));
    assert!(error.contains("wrdata"));
    assert!(!review.can_commit());
    cancel_staged_netlist_import(&mut state.code.netlist, &mut state.platform);
}

#[test]
fn semantic_loss_parser_diagnostics_block_import() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* ignored option is unsafe\n.options definitely_unknown=7\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    assert!(state.stage_netlist_import(
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "unknown-option.cir".to_owned()
    ));
    let review = state.code.netlist.import_review.as_ref().unwrap();
    assert!(review.blocking_issue_count() > 0);
    assert!(review.issues.iter().any(|issue| {
        issue.severity
            == crate::workbench::documents::netlist_document::NetlistImportIssueSeverity::Blocking
            && issue.message.contains("unknown-option")
    }));
    assert!(!review.can_commit());
    cancel_staged_netlist_import(&mut state.code.netlist, &mut state.platform);
}

#[test]
fn imported_netlist_becomes_dirty_manual_source_without_deleting_retained_runs() {
    let mut state = AppState::default();
    state.model.simulation.start_run();
    assert!(state.model.simulation.retained.has_results());

    let imported = state.apply_imported_netlist(
        "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
        Some(std::path::PathBuf::from("bias.cir")),
        "bias.cir",
    );

    assert!(imported);
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Netlist
    );
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n")
    );
    assert_eq!(
        state.model.workspace.content.netlist_source_path.as_deref(),
        Some(std::path::Path::new("bias.cir"))
    );
    assert!(state.model.workspace.content.netlist_source_dirty);
    assert!(state.model.workspace.any_dirty());
    assert!(state.model.workspace.content.netlist_document.is_some());
    assert_eq!(
        state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.as_str()),
        Some("bias.cir")
    );
    state
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .expect("imported canonical source must satisfy project persistence invariants");
    assert!(state.model.simulation.retained.has_results());
    assert_eq!(state.model.simulation.retained.runs.len(), 1);
    assert!(state.platform.recent_files.is_empty());
}

#[test]
fn opening_a_netlist_commits_an_independent_netlist_first_project() {
    let mut state = AppState::default();
    let original_project_id = state.model.workspace.content.project.id();
    state.model.simulation.start_run();
    state.model.workspace.content.netlist_source = Some("old\n.op\n.end\n".to_owned());

    assert!(state.apply_opened_netlist_project(
        "new\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
        Some(std::path::PathBuf::from("bias.cir")),
        "bias.cir",
        NetlistImportMetadata {
            encoding: crate::state::NetlistTextEncoding::Utf8,
            line_ending: crate::state::NetlistLineEnding::Lf,
            dialect: crate::state::NetlistSourceDialect::RSpice,
            compatibility_reviewed: false,
            raw_sha256: sha256(b"new\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n"),
        },
        Vec::new()
    ));

    assert_ne!(
        state.model.workspace.content.project.id(),
        original_project_id
    );
    assert_eq!(state.model.workspace.content.project.name(), "bias");
    assert!(!state.model.simulation.retained.has_results());
    assert_eq!(
        state.model.workspace.content.netlist_source_path.as_deref(),
        Some(std::path::Path::new("bias.cir"))
    );
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Netlist
    );
}

/// The deck is installed into a candidate project that a failure discards,
/// so the reason has to be carried out of it to reach the reader at all.
#[test]
fn a_netlist_project_that_cannot_open_states_why() {
    let mut state = AppState::default();
    let original_project_id = state.model.workspace.content.project.id();
    let source = "deck\nR1 out 0 1k\n.op\n.end\n";

    assert!(!state.apply_opened_netlist_project(
        source.to_owned(), // A path with no name gives the deck no identity to be filed under.
        Some(std::path::PathBuf::new()),
        "bias.cir",
        NetlistImportMetadata {
            encoding: crate::state::NetlistTextEncoding::Utf8,
            line_ending: crate::state::NetlistLineEnding::Lf,
            dialect: crate::state::NetlistSourceDialect::RSpice,
            compatibility_reviewed: false,
            raw_sha256: sha256(source.as_bytes()),
        },
        Vec::new()
    ));

    assert_eq!(
        state.model.workspace.content.project.id(),
        original_project_id
    );
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the failure is recorded");
    assert_eq!(line.severity, crate::diagnostics::LogSeverity::Error);
    let reason = line
        .message
        .strip_prefix("Netlist project open failed: ")
        .and_then(|rest| rest.strip_suffix(". The current project was left unchanged."))
        .unwrap_or_else(|| panic!("the line states the failure: {}", line.message));
    assert!(
        reason.contains("logical identity"),
        "the line states why: {}",
        line.message
    );
}

#[test]
fn importing_a_deck_refuses_read_only_projects_without_mutation() {
    let mut state = AppState::default();
    state.model.safe_mode.active = true;
    state.model.safe_mode.applied = crate::state::write_policy::LocalSafeModeOptions {
        open_project_read_only: true,
        ..Default::default()
    };
    state.model.workspace.content.netlist_source = Some("old\n.op\n.end\n".to_owned());

    assert!(!state.apply_imported_netlist("new\n.op\n.end\n".to_owned(), None, "new.cir"));
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some("old\n.op\n.end\n")
    );
}

#[test]
fn empty_netlist_import_is_rejected_without_clearing_existing_state() {
    let mut state = AppState::default();
    state.model.workspace.content.netlist_source = Some("existing\n.op\n.end\n".to_owned());
    state.model.workspace.content.netlist_source_path =
        Some(std::path::PathBuf::from("existing.cir"));
    state.model.simulation.source.netlist_content = "existing\n.op\n.end\n".to_owned();

    let imported = state.apply_imported_netlist(" \n\t".to_owned(), None, "empty.cir");

    assert!(!imported);
    assert_eq!(
        state.model.workspace.content.netlist_source.as_deref(),
        Some("existing\n.op\n.end\n")
    );
    assert_eq!(
        state.model.workspace.content.netlist_source_path.as_deref(),
        Some(std::path::Path::new("existing.cir"))
    );
    assert_eq!(
        state.model.simulation.source.netlist_content,
        "existing\n.op\n.end\n"
    );
}
