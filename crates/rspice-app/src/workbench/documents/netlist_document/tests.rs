//! Owned includes are revision-bound, and edits to them are atomic.
//!
//! A dependency is read-only until copied; a rename preserves both consumer
//! and bytes; a workspace replacement commits root and owned include together
//! as one undo step, or refuses without a partial root edit. The outline is
//! parsed once per change, which is what keeps a large deck off the frame
//! budget.

use super::*;
use crate::product::ObjectRevision;
use crate::state::netlist_diagnostics::Diagnostic;
use crate::state::{
    DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
    NetlistDocumentId, NetlistValidationReceipt, SourceLocator,
};
use crate::workbench::code::tests::{CodeHarness, harness};

const ROOT: &str = "owned dependency fixture\n.include \"models/resistor.inc\"\nV1 out 0 1\nR1 out 0 rmodel\n.op\n.end\n";

const ORIGINAL_INCLUDE: &str = ".param rmodel=1k\n";

const EDITED_INCLUDE: &str = ".param rmodel=2.2k\n";

const INCLUDE_IDENTITY: &str = "models/resistor.inc";

const INCLUDE_NATIVE_ORIGIN: &str = "C:/project/models/resistor.inc";

/// Whether the netlist editor shows the project's own root source.
fn shows_owned_root(harness: &CodeHarness) -> bool {
    harness.state.netlist.active_document == ActiveNetlistDocument::OwnedSource
        && harness.state.netlist.active_dependency_identity.is_none()
}

/// The owned-dependency fixture seeded into a session's model and netlist
/// editor: a generated deck that includes one dependency, copied into the
/// project's own root source, which the editor shows, validated.
pub(crate) fn seed_owned_dependency(model: &mut Model, netlist: &mut NetlistDocumentState) {
    let locator = SourceLocator::try_new(INCLUDE_IDENTITY, "resistor.inc")
        .unwrap()
        .with_native_origin(INCLUDE_NATIVE_ORIGIN)
        .unwrap();
    let dependency = DependencyMetadata::unresolved_direct_to(0, INCLUDE_IDENTITY, locator)
        .unwrap()
        .resolve_utf8(ORIGINAL_INCLUDE.as_bytes().to_vec())
        .unwrap();
    let provenance = GeneratedProvenance::try_new(
        "rspice-owned-include-test",
        GenerationInput::new(
            ObjectRevision::INITIAL,
            crate::state::content_digest("owned-include-input"),
        ),
    )
    .unwrap();
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        ROOT.as_bytes().to_vec(),
        vec![dependency],
        Vec::new(),
    )
    .unwrap();
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).unwrap();
    let owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .unwrap();

    model.workspace.content.netlist_source = Some(ROOT.to_owned());
    model.workspace.content.netlist_document = Some(owned.clone());
    model.workspace.content.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        deck_id: uuid::Uuid::new_v4(),
        artifact_name: "owned.cir".to_owned(),
        strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::Lf,
        imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
        compatibility_reviewed: true,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });
    model.netlist_artifacts.generated_source = ROOT.to_owned();
    model.netlist_artifacts.generated_document = Some(generated);
    netlist.owned_document = Some(owned);
    netlist.active_document = ActiveNetlistDocument::OwnedSource;
    netlist.active_document_initialized = true;
    model.simulation.source.netlist_content = ROOT.to_owned();
    model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

/// The owned-dependency fixture over code's harness.
fn owned_dependency_state() -> CodeHarness {
    let mut harness = harness();
    seed_owned_dependency(&mut harness.model, &mut harness.state.netlist);
    harness
}

/// The navigator asks for the outline on every frame. Parsing costs the
/// deck, so the answer has to be the same object until the bytes change —
/// and a different one the moment they do, because a navigator listing
/// declarations the buffer no longer contains is worse than a slow one.
#[test]
fn the_visible_outline_is_parsed_once_per_change_and_never_reused_across_one() {
    let mut harness = owned_dependency_state();

    let first = visible_source_index_in(&harness.model, &mut harness.state.netlist);
    let again = visible_source_index_in(&harness.model, &mut harness.state.netlist);
    assert!(
        std::sync::Arc::ptr_eq(&first, &again),
        "an unchanged buffer must not be parsed twice"
    );
    assert!(first.describes(ROOT));
    assert_eq!(first.card(3), "V1 out 0 1");

    assert!(replace_owned_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.wb,
        format!("{ROOT}* one more card\n")
    ));
    let edited = visible_source_index_in(&harness.model, &mut harness.state.netlist);
    assert!(!std::sync::Arc::ptr_eq(&first, &edited));
    assert_eq!(edited.line_count(), first.line_count() + 1);

    // Switching documents changes the buffer without editing it.
    assert!(open_generated_primary_in(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert!(visible_source_index_in(&harness.model, &mut harness.state.netlist).describes(ROOT));
}

#[test]
fn reactivating_the_active_owned_root_preserves_validation_evidence() {
    let mut harness = owned_dependency_state();
    harness.state.netlist.revision = 9;
    let digest = crate::state::content_digest(ROOT);
    harness.model.netlist_artifacts.validation = Some(NetlistValidationReceipt {
        visible_content_digest: digest,
        executable_source_digest: crate::product::ContentDigest::from_bytes([0x51; 32]),
        prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
        project_revision: harness.model.workspace.content.project.revision().get(),
        task_count: 1,
        advisory_count: 0,
    });
    let validation = harness.model.netlist_artifacts.validation.clone();

    assert!(open_owned_primary_in(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert!(open_owned_primary_in(
        &mut harness.model,
        &mut harness.state.netlist
    ));

    assert_eq!(harness.state.netlist.revision, 9);
    assert_eq!(harness.model.netlist_artifacts.validation, validation);
    assert_eq!(harness.model.simulation.source.netlist_content, ROOT);
}

/// A participant's source applied to the model and adopted by the editor
/// leaves the editor on the root document the model holds, past the
/// revision it showed, and a second delivery of the same source changes
/// nothing.
#[test]
fn a_live_source_update_the_editor_adopts_leaves_it_on_the_models_document() {
    use rspice_app_state::workbench::app_state::model::LiveOwnedSource;

    let mut harness = owned_dependency_state();
    let revision = harness.state.netlist.revision;
    let source = format!("{ROOT}* a peer's card\n");
    let shows_owned_root = shows_owned_root(&harness);
    assert!(shows_owned_root, "the fixture shows the project's own root");

    let LiveOwnedSource::Applied(document) = harness
        .model
        .apply_live_owned_source(source.clone(), shows_owned_root)
    else {
        panic!("a participant's source applies");
    };
    adopt_live_owned_source(
        &mut harness.state.netlist,
        document.map(|document| *document),
    );

    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(source.as_str())
    );
    assert_eq!(harness.model.simulation.source.netlist_content, source);
    assert_eq!(
        harness.state.netlist.owned_document,
        harness.model.workspace.content.netlist_document
    );
    assert_eq!(
        harness
            .state
            .netlist
            .owned_document
            .as_ref()
            .map(|document| document.source()),
        Some(source.as_str())
    );
    assert_eq!(harness.state.netlist.revision, revision.wrapping_add(1));
    assert!(matches!(
        harness
            .model
            .apply_live_owned_source(source, shows_owned_root),
        LiveOwnedSource::Unchanged
    ));
}

#[test]
fn dependency_is_read_only_until_copy_then_edits_the_execution_closure() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        ORIGINAL_INCLUDE
    );
    assert!(!active_dependency_is_owned_in(
        &harness.model,
        &harness.state.netlist
    ));
    assert!(!replace_owned_dependency_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        EDITED_INCLUDE.to_owned()
    ));

    let document_id = copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert_eq!(
        copy_active_dependency_to_project(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap(),
        document_id,
        "copy-to-project is idempotent and preserves stable identity"
    );
    assert!(active_dependency_is_owned_in(
        &harness.model,
        &harness.state.netlist
    ));
    assert!(replace_owned_dependency_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        EDITED_INCLUDE.to_owned()
    ));

    let document = harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert_eq!(
        document.source(),
        ROOT,
        "include edits never rewrite the root"
    );
    assert_eq!(document.dependencies()[0].source(), Some(EDITED_INCLUDE));
    let descriptor = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap();
    let include = descriptor.owned_include(INCLUDE_IDENTITY).unwrap();
    assert_eq!(include.document_id, document_id);
    assert_eq!(include.revision, 2);
    assert_eq!(
        include.content_digest,
        crate::state::content_digest(EDITED_INCLUDE)
    );
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();

    let expanded = rspice_simulation::netlist_sources::expand_retained_netlist_dependencies(
        document.id(),
        document.source(),
        document.dependencies(),
    )
    .unwrap();
    assert!(expanded.source.contains(".param rmodel=2.2k"));
    assert!(!expanded.source.contains(".param rmodel=1k"));
}

#[test]
fn canonical_diagnostic_navigation_opens_the_exact_include_and_line() {
    let mut harness = owned_dependency_state();
    let mut diagnostic = Diagnostic::error("included model is malformed");
    diagnostic.source_path = Some(std::path::PathBuf::from(INCLUDE_NATIVE_ORIGIN));
    diagnostic.source_line = Some(17);
    diagnostic.line = None;
    let diagnostic_id = diagnostic.canonical.diagnostic_id;
    harness.state.netlist.diagnostics =
        std::sync::Arc::new(NetlistDiagnosticCollection::try_new(vec![diagnostic], ROOT).unwrap());

    open_diagnostic_location(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        &mut harness.wb,
        &mut harness.state.code_workspace.page,
        diagnostic_id,
    )
    .unwrap();

    assert_eq!(
        harness.state.netlist.active_dependency_identity.as_deref(),
        Some(INCLUDE_IDENTITY)
    );
    assert_eq!(harness.state.netlist.cursor_line, 17);
    assert_eq!(harness.state.netlist.requested_line, Some(18));
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        ORIGINAL_INCLUDE
    );
}

#[test]
fn owned_include_authority_can_be_released_and_reacquired_without_losing_bytes() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    let first_id = copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();

    release_active_dependency_from_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(!active_dependency_is_owned_in(
        &harness.model,
        &harness.state.netlist
    ));
    assert_eq!(
        active_dependency_in(&harness.model, &harness.state.netlist)
            .unwrap()
            .source(),
        Some(ORIGINAL_INCLUDE)
    );
    assert!(
        harness
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .owned_includes
            .is_empty()
    );

    let second_id = copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert_ne!(first_id, second_id);
    assert!(active_dependency_is_owned_in(
        &harness.model,
        &harness.state.netlist
    ));
    assert_eq!(
        active_dependency_in(&harness.model, &harness.state.netlist)
            .unwrap()
            .source(),
        Some(ORIGINAL_INCLUDE)
    );
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn owned_include_rename_is_revision_bound_and_preserves_consumer_and_bytes() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    let document_id = copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    begin_owned_include_lifecycle_action(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Rename,
    )
    .unwrap();
    harness
        .state
        .netlist
        .lifecycle_dialog
        .include_transaction
        .as_mut()
        .unwrap()
        .proposed_identity = "models/resistor-renamed.inc".to_owned();
    commit_owned_include_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();

    let identity = "models/resistor-renamed.inc";
    assert_eq!(
        harness.state.netlist.active_dependency_identity.as_deref(),
        Some(identity)
    );
    let dependency = active_dependency_in(&harness.model, &harness.state.netlist).unwrap();
    assert_eq!(dependency.requested_locator(), INCLUDE_IDENTITY);
    assert_eq!(dependency.locator().logical_identity(), identity);
    assert_eq!(dependency.source(), Some(ORIGINAL_INCLUDE));
    let include = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .owned_include(identity)
        .unwrap();
    assert_eq!(include.document_id, document_id);
    assert_eq!(include.revision, 2);
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn editing_root_include_cards_preserves_reachable_edges_and_prunes_removed_ownership() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(close_active_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.runs
    ));

    let without_include = ROOT
        .lines()
        .filter(|line| !line.starts_with(".include"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert!(replace_owned_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.wb,
        without_include
    ));
    let document = harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert!(document.dependencies().is_empty());
    assert!(
        harness
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .owned_includes
            .is_empty()
    );

    let with_new_include = harness
        .model
        .workspace
        .content
        .netlist_source
        .as_deref()
        .unwrap()
        .replacen(".op", ".include \"models/new.inc\"\n.op", 1);
    assert!(replace_owned_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.wb,
        with_new_include
    ));
    let dependency = &harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap()
        .dependencies()[0];
    assert_eq!(dependency.requested_locator(), "models/new.inc");
    assert!(dependency.source().is_none());
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn secondary_document_limit_refusal_leaves_the_active_document_unchanged() {
    let mut harness = owned_dependency_state();
    for index in 0..MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS {
        harness.state.netlist_open_documents.insert(
            crate::workbench::state::WorkspaceDocumentId::NetlistDependency {
                root: NetlistDocumentId::new(),
                logical_identity: format!("fixture-{index}.inc"),
            },
        );
    }
    let before_source = harness.model.simulation.source.netlist_content.clone();
    let before_document = harness.state.netlist.active_document;

    let error = open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap_err();
    assert!(error.contains("Close an include tab"));
    assert_eq!(harness.state.netlist.active_document, before_document);
    assert!(harness.state.netlist.active_dependency_root.is_none());
    assert!(harness.state.netlist.active_dependency_identity.is_none());
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        before_source
    );
}

#[test]
fn workspace_replacement_commits_root_and_owned_include_together() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    let edited_root = ROOT.replace("V1 out 0 1", "VINPUT out 0 1");
    let edits = vec![
        OwnedNetlistReplacement::root(ROOT, edited_root.clone(), 1),
        OwnedNetlistReplacement::dependency(
            INCLUDE_IDENTITY,
            ORIGINAL_INCLUDE,
            EDITED_INCLUDE.to_owned(),
            1,
        ),
    ];

    assert_eq!(
        replace_owned_sources_atomically(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents,
            edits
        )
        .unwrap(),
        2
    );
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(edited_root.as_str())
    );
    let document = harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert_eq!(document.source(), edited_root);
    assert_eq!(document.dependencies()[0].source(), Some(EDITED_INCLUDE));
    assert_eq!(
        harness.model.simulation.source.netlist_content, EDITED_INCLUDE,
        "the visible owned include follows the atomic commit"
    );
    let include = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .owned_include(INCLUDE_IDENTITY)
        .unwrap();
    assert_eq!(include.revision, 2);
    assert_eq!(
        include.content_digest,
        crate::state::content_digest(EDITED_INCLUDE)
    );
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn workspace_replacement_is_one_atomic_undo_and_redo_step() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    let edited_root = ROOT.replace("V1 out 0 1", "VINPUT out 0 1");
    let edits = vec![
        OwnedNetlistReplacement::root(ROOT, edited_root.clone(), 1),
        OwnedNetlistReplacement::dependency(
            INCLUDE_IDENTITY,
            ORIGINAL_INCLUDE,
            EDITED_INCLUDE.to_owned(),
            1,
        ),
    ];
    replace_owned_sources_atomically(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        edits,
    )
    .unwrap();
    assert!(can_undo_netlist_edit(&harness.state.netlist));

    assert!(
        undo_netlist_edit(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap()
        .is_some()
    );
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(ROOT)
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(ORIGINAL_INCLUDE)
    );
    assert!(can_redo_netlist_edit(&harness.state.netlist));

    assert!(
        redo_netlist_edit(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap()
        .is_some()
    );
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(edited_root.as_str())
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(EDITED_INCLUDE)
    );
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn workspace_replacement_rejects_stale_include_without_partial_root_edit() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    let stale_edits = vec![
        OwnedNetlistReplacement::root(ROOT, ROOT.replace("V1 out 0 1", "VINPUT out 0 1"), 1),
        OwnedNetlistReplacement::dependency(
            INCLUDE_IDENTITY,
            ORIGINAL_INCLUDE,
            ".param rmodel=3.3k\n".to_owned(),
            1,
        ),
    ];
    assert!(replace_owned_dependency_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        EDITED_INCLUDE.to_owned()
    ));
    let before = serde_json::to_vec(&harness.model.workspace).unwrap();

    let error = replace_owned_sources_atomically(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        stale_edits,
    )
    .unwrap_err();

    assert!(error.contains("changed after search results"), "{error}");
    assert_eq!(
        serde_json::to_vec(&harness.model.workspace).unwrap(),
        before
    );
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(ROOT)
    );
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        EDITED_INCLUDE
    );
}

#[test]
fn ordinary_root_edit_retains_authenticated_dependency_closure() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(close_active_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.runs
    ));
    let edited_root = ROOT.replace("V1 out 0 1", "VINPUT out 0 1");

    assert!(replace_owned_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.wb,
        edited_root.clone()
    ));

    let document = harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert_eq!(document.source(), edited_root);
    assert_eq!(document.dependencies()[0].source(), Some(ORIGINAL_INCLUDE));
    assert!(document.dependency_graph_is_sealed());
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn ordinary_root_edit_replaces_include_edge_atomically_and_requires_relink() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(close_active_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.runs
    ));
    let changed_include = ROOT.replace(INCLUDE_IDENTITY, "models/other.inc");

    assert!(replace_owned_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.wb,
        changed_include.clone()
    ));

    let document = harness
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert_eq!(document.source(), changed_include);
    assert_eq!(document.dependencies().len(), 1);
    assert_eq!(
        document.dependencies()[0].requested_locator(),
        "models/other.inc"
    );
    assert!(matches!(
        document.dependencies()[0].resolution(),
        crate::state::DependencyResolution::Unresolved
    ));
    assert!(
        harness
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .owned_includes
            .is_empty()
    );
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        changed_include
    );
}

#[test]
fn owned_dependency_identity_and_bytes_survive_project_round_trip() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    let document_id = copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(replace_owned_dependency_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        EDITED_INCLUDE.to_owned()
    ));

    let bytes = serde_json::to_vec(&harness.model.workspace).unwrap();
    let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&bytes).unwrap();
    restored
        .content
        .validate_simulation_configuration()
        .unwrap();
    let include = restored
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .owned_include(INCLUDE_IDENTITY)
        .unwrap();
    assert_eq!(include.document_id, document_id);
    assert_eq!(include.revision, 2);
    assert_eq!(
        restored
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(EDITED_INCLUDE)
    );
}

#[test]
fn revision_restore_restores_include_bytes_and_ownership_as_one_snapshot() {
    let mut harness = owned_dependency_state();
    let baseline_document = harness
        .state
        .netlist
        .owned_document
        .as_ref()
        .unwrap()
        .clone();
    harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap()
        .retain_revision(&baseline_document, "Before include ownership")
        .unwrap();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(replace_owned_dependency_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        EDITED_INCLUDE.to_owned()
    ));

    restore_owned_revision(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        0,
    )
    .unwrap();

    assert!(
        harness
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .owned_includes
            .is_empty()
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(ORIGINAL_INCLUDE)
    );
    assert!(harness.state.netlist.active_dependency_identity.is_none());
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn dependency_transitions_fail_closed_for_stale_or_generated_ownership() {
    let mut harness = owned_dependency_state();
    let before = harness.model.simulation.source.netlist_content.clone();
    assert!(
        open_netlist_dependency_in(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents,
            &mut harness.runs,
            "missing.inc"
        )
        .is_err()
    );
    assert_eq!(harness.model.simulation.source.netlist_content, before);
    assert!(harness.state.netlist.active_dependency_identity.is_none());

    open_generated_primary_in(&mut harness.model, &mut harness.state.netlist);
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    assert!(!active_dependency_is_owned_in(
        &harness.model,
        &harness.state.netlist
    ));
    assert!(
        copy_active_dependency_to_project(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .is_err()
    );
    assert!(!replace_owned_dependency_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        EDITED_INCLUDE.to_owned()
    ));
}

#[test]
fn project_validation_rejects_owned_include_digest_drift() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_mut()
        .unwrap()
        .owned_includes[0]
        .content_digest = crate::product::ContentDigest::from_bytes([0x7f; 32]);
    assert!(
        harness
            .model
            .workspace
            .content
            .validate_simulation_configuration()
            .is_err()
    );
}

#[test]
fn relink_reacquires_origin_and_replaces_exact_retained_bytes() {
    let mut harness = owned_dependency_state();
    begin_dependency_relink(&harness.model, &mut harness.state.netlist, INCLUDE_IDENTITY).unwrap();
    commit_dependency_relink(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        EDITED_INCLUDE.to_owned(),
        "resistor-qualified.lib".to_owned(),
        Some("C:/qualified/models/resistor-qualified.lib".to_owned()),
    )
    .unwrap();

    let dependency = active_dependency_in(&harness.model, &harness.state.netlist).unwrap();
    assert_eq!(dependency.source(), Some(EDITED_INCLUDE));
    assert_eq!(
        dependency.locator().display_name(),
        "resistor-qualified.lib"
    );
    assert_eq!(
        dependency.locator().native_origin(),
        Some("C:/qualified/models/resistor-qualified.lib")
    );
    assert_eq!(
        dependency.locator().logical_identity(),
        INCLUDE_IDENTITY,
        "relink must preserve the canonical include edge"
    );
    assert!(harness.model.workspace.content.netlist_source_dirty);
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn relink_rejects_late_picker_completion_after_document_revision_changes() {
    let mut harness = owned_dependency_state();
    begin_dependency_relink(&harness.model, &mut harness.state.netlist, INCLUDE_IDENTITY).unwrap();
    let document = harness.state.netlist.owned_document.as_mut().unwrap();
    document
        .replace_editable_source(
            document.content_digest(),
            ROOT.replace("V1 out 0 1", "V1 out 0 2").into_bytes(),
        )
        .unwrap();

    let error = commit_dependency_relink(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        EDITED_INCLUDE.to_owned(),
        "late.lib".to_owned(),
        None,
    )
    .unwrap_err();

    assert!(error.contains("changed while the picker was open"));
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(ORIGINAL_INCLUDE)
    );
}

#[test]
fn generated_dependency_relink_updates_only_the_generated_artifact() {
    let mut harness = owned_dependency_state();
    open_generated_primary_in(&mut harness.model, &mut harness.state.netlist);
    begin_dependency_relink(&harness.model, &mut harness.state.netlist, INCLUDE_IDENTITY).unwrap();
    commit_dependency_relink(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        EDITED_INCLUDE.to_owned(),
        "browser-picked.inc".to_owned(),
        None,
    )
    .unwrap();

    assert_eq!(
        active_dependency_in(&harness.model, &harness.state.netlist)
            .unwrap()
            .source(),
        Some(EDITED_INCLUDE)
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_document
            .as_ref()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some(ORIGINAL_INCLUDE),
        "relinking generated review authority must not mutate the owned root"
    );
}

#[test]
fn top_deck_lifecycle_is_atomic_and_preserves_inactive_decks() {
    let mut harness = owned_dependency_state();
    let original_id = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .deck_id;

    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Rename,
    )
    .unwrap();
    harness
        .state
        .netlist
        .lifecycle_dialog
        .transaction
        .as_mut()
        .unwrap()
        .proposed_path = "renamed.cir".to_owned();
    commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();

    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Move,
    )
    .unwrap();
    harness
        .state
        .netlist
        .lifecycle_dialog
        .transaction
        .as_mut()
        .unwrap()
        .proposed_path = "decks/renamed.cir".to_owned();
    commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();

    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Duplicate,
    )
    .unwrap();
    commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    let duplicate_id = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .deck_id;
    assert_ne!(duplicate_id, original_id);
    assert_eq!(
        harness.model.workspace.content.retained_netlist_decks.len(),
        1
    );
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(ROOT),
        "duplicating retains the exact source bytes"
    );
    assert_eq!(
        harness.model.workspace.content.retained_netlist_decks[0]
            .descriptor
            .artifact_name,
        "decks/renamed.cir"
    );

    select_retained_top_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        original_id,
    )
    .unwrap();
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .deck_id,
        original_id
    );
    assert_eq!(
        harness.model.workspace.content.retained_netlist_decks[0]
            .descriptor
            .deck_id,
        duplicate_id
    );

    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::New,
    )
    .unwrap();
    commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert_eq!(
        harness.model.workspace.content.retained_netlist_decks.len(),
        2
    );
    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some("* New RSpice top deck\n.end\n")
    );

    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Delete,
    )
    .unwrap();
    let message = commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(message.contains("native file was not deleted"));
    assert_eq!(
        harness.model.workspace.content.retained_netlist_decks.len(),
        1
    );
    assert!(harness.model.workspace.content.netlist_document.is_some());
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn project_replace_is_atomic_across_active_and_retained_top_decks() {
    let mut harness = owned_dependency_state();
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        INCLUDE_IDENTITY,
    )
    .unwrap();
    copy_active_dependency_to_project(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    assert!(open_owned_primary_in(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    let retained_id = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .deck_id;

    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Duplicate,
    )
    .unwrap();
    commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap();
    let active_id = harness
        .model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .deck_id;
    assert_ne!(active_id, retained_id);

    let edited_root = ROOT.replace("V1 out 0 1", "V1 out 0 2");
    let edits = vec![
        OwnedNetlistReplacement::retained_root(retained_id, ROOT, edited_root.clone(), 1),
        OwnedNetlistReplacement::retained_dependency(
            retained_id,
            INCLUDE_IDENTITY,
            ORIGINAL_INCLUDE,
            EDITED_INCLUDE.to_owned(),
            1,
        ),
    ];
    assert_eq!(
        replace_owned_sources_atomically(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents,
            edits
        )
        .unwrap(),
        2
    );

    assert_eq!(
        harness.model.workspace.content.netlist_source.as_deref(),
        Some(ROOT)
    );
    let retained = harness
        .model
        .workspace
        .content
        .retained_netlist_decks
        .iter()
        .find(|deck| deck.descriptor.deck_id == retained_id)
        .unwrap();
    assert_eq!(retained.document.source(), edited_root);
    assert_eq!(
        retained.document.dependencies()[0].source(),
        Some(EDITED_INCLUDE)
    );
    assert_eq!(
        retained.descriptor.owned_includes[0].content_digest,
        crate::state::content_digest(EDITED_INCLUDE)
    );
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();

    assert!(
        undo_netlist_edit(
            &mut harness.model,
            &mut harness.state.netlist,
            &mut harness.state.netlist_open_documents
        )
        .unwrap()
        .is_some()
    );
    let retained = harness
        .model
        .workspace
        .content
        .retained_netlist_decks
        .iter()
        .find(|deck| deck.descriptor.deck_id == retained_id)
        .unwrap();
    assert_eq!(retained.document.source(), ROOT);
    assert_eq!(
        retained.document.dependencies()[0].source(),
        Some(ORIGINAL_INCLUDE)
    );
    harness
        .model
        .workspace
        .content
        .validate_simulation_configuration()
        .unwrap();
}

#[test]
fn top_deck_lifecycle_rejects_a_stale_document_revision() {
    let mut harness = owned_dependency_state();
    begin_netlist_lifecycle_action_in(
        &harness.model,
        &mut harness.state.netlist,
        CodeSourceFileAction::Rename,
    )
    .unwrap();
    assert!(replace_owned_source(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.wb,
        ROOT.replace("V1 out 0 1", "V1 out 0 2")
    ));

    let error = commit_netlist_lifecycle_action(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
    )
    .unwrap_err();
    assert!(error.contains("changed while the lifecycle dialog was open"));
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .artifact_name,
        "owned.cir"
    );
}

#[test]
fn run_deck_snapshot_comparison_reports_the_edits_made_since_the_run() {
    const RAN: &str = "ran deck\nR1 out 0 1k\n.op\n.end\n";
    const EDITED: &str = "ran deck\nR1 out 0 2k\n.op\n.end\n";

    let mut harness = owned_dependency_state();
    harness.model.workspace.content.netlist_source = Some(EDITED.to_owned());
    harness.model.simulation.source.netlist_content = EDITED.to_owned();
    harness.state.netlist.active_document = ActiveNetlistDocument::OwnedSource;
    harness.state.netlist.active_document_initialized = true;
    harness.runs.manual_run.last_run_buffer = Some(RAN.to_owned());
    harness.runs.manual_run.last_run_id = Some(3);
    assert!(open_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs
    ));

    compare_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs,
    )
    .unwrap();

    assert_eq!(
        harness.state.netlist.active_document,
        ActiveNetlistDocument::GeneratedDiff
    );
    let diff = harness.state.netlist.generated_diff_source.as_str();
    assert!(diff.contains("--- run-3"), "{diff}");
    assert!(diff.contains("-R1 out 0 1k"), "{diff}");
    assert!(diff.contains("+R1 out 0 2k"), "{diff}");

    assert!(close_revision_comparison(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert_eq!(
        harness.state.netlist.active_document,
        ActiveNetlistDocument::OwnedSource
    );
}

#[test]
fn run_deck_snapshot_needs_both_the_run_and_its_deck() {
    let mut harness = owned_dependency_state();
    harness.runs.manual_run.last_run_buffer = Some("ran\n.end\n".to_owned());
    assert!(run_deck_snapshot_run_id_in(&harness.runs).is_none());
    assert!(!open_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs
    ));

    harness.runs.manual_run.last_run_id = Some(9);
    harness.runs.manual_run.last_run_buffer = None;
    assert!(run_deck_snapshot_run_id_in(&harness.runs).is_none());
    assert!(!open_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs
    ));
}

/// The executed-deck viewer: read-only, per point, and never substituting.
///
/// The retention half — that every run seals these at dispatch — is proved in
/// `simulation::controller::tests`. What is proved here is what the document
/// then is: the immutable run-snapshot document, showing the point that was
/// asked for, refusing a comparison it cannot honestly make, and refusing a
/// run whose deck this session does not hold rather than showing another one.
#[test]
fn an_executed_deck_opens_read_only_at_the_point_it_was_asked_for() {
    use crate::state::{ExecutedDeck, ExecutedDeckPoint, sealed_model_sources};

    const NOMINAL: &str =
        "run deck\n* RSpice sealed model source: pack rspice-opamps 2.1.0\nV1 out 0 1\n.op\n.end\n";
    const HOT: &str = "run deck\n* RSpice sealed model source: pack rspice-opamps 2.1.0\n.OPTIONS TEMP=125\nV1 out 0 1\n.op\n.end\n";

    let mut harness = owned_dependency_state();
    let point = |label: &str, text: &str| {
        let deck: std::sync::Arc<str> = std::sync::Arc::from(text);
        ExecutedDeckPoint {
            label: label.to_owned(),
            model_sources: sealed_model_sources(&deck),
            deck,
        }
    };
    harness
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: 4,
            points: vec![point("TT 27C", NOMINAL), point("TT 125C", HOT)],
        });

    assert!(reveal_executed_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs,
        &mut harness.wb,
        4,
        1
    ));
    assert_eq!(
        harness.state.netlist.active_document,
        ActiveNetlistDocument::RunSnapshot
    );
    assert_eq!(harness.model.simulation.source.netlist_content, HOT);
    assert!(
        !active_netlist_source_is_editable_in(&harness.model, &harness.state.netlist),
        "an executed deck is a viewer, never a buffer"
    );
    assert_eq!(
        harness
            .model
            .simulation
            .retained
            .executed_decks
            .get(4)
            .map(crate::state::ExecutedDeck::model_sources),
        Some(vec!["pack rspice-opamps 2.1.0".to_owned()]),
        "and its own comments name the release it was given"
    );
    assert!(
        compare_run_deck_snapshot_in(
            &mut harness.model,
            &mut harness.state.netlist,
            &harness.runs
        )
        .is_err(),
        "there is no working copy of a deck nobody typed"
    );

    // A point past the end resolves to one the run actually has rather than
    // to an empty document under a header naming the run.
    assert!(reveal_executed_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs,
        &mut harness.wb,
        4,
        40
    ));
    assert_eq!(harness.model.simulation.source.netlist_content, HOT);

    assert!(
        !reveal_executed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            &harness.runs,
            &mut harness.wb,
            5,
            0
        ),
        "a run whose deck is not held refuses rather than showing another one"
    );

    assert!(close_run_deck_snapshot(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert!(
        harness.state.netlist.executed_deck_view.is_none(),
        "leaving the viewer releases the run it was bound to"
    );
}

/// The badge over an executed deck is earned or it is not made.
///
/// `source_content_digest` is the one receipt field that authenticates deck
/// text, so it is the one thing a retained deck can be checked against, and it
/// covers the run-level source only. What is proved here is that a real
/// recomputation decides the verdict — a point that solved the sealed source
/// verbatim verifies, a corner's materialized variant of it is named as
/// unsealed rather than as tampered, a rewritten deck is named as matching
/// nothing, and a run with no receipt claims nothing at all.
#[test]
fn an_executed_deck_claims_verification_only_when_it_reproduces_the_sealed_digest() {
    use crate::state::{
        AnalysisResultSourceDomain, ExecutedDeck, ExecutedDeckPoint, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunProvenance,
        sealed_model_sources,
    };
    use crate::workbench::documents::netlist_document::ExecutedDeckVerification;

    const SEALED: &str = "authorized deck\nV1 out 0 1\n.op\n.end\n";
    const CORNER: &str = "authorized deck\n.lib cmos.lib ss\nV1 out 0 1\n.op\n.end\n";

    let digest = |value: u8| crate::product::ContentDigest::from_bytes([value; 32]);
    let point = |label: &str, text: &str| {
        let deck: std::sync::Arc<str> = std::sync::Arc::from(text);
        ExecutedDeckPoint {
            label: label.to_owned(),
            model_sources: sealed_model_sources(&deck),
            deck,
        }
    };
    let seal = |harness: &mut CodeHarness, source: &str| {
        let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0xa0),
            source_content_digest:
                rspice_simulation::sealed_source::sealed_executable_source_digest(
                    AnalysisResultSourceDomain::SimulationPlan,
                    source,
                )
                .expect("a plan run seals its executable source"),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0xa2)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    crate::product::AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    digest(0xa3),
                )
                .expect("task receipt"),
            ],
        })
        .expect("prepared run receipt");
        let run = harness
            .model
            .simulation
            .start_prepared_run(receipt)
            .unwrap();
        run.id
    };

    let mut harness = owned_dependency_state();
    let sequence = seal(&mut harness, SEALED);
    harness
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: sequence,
            points: vec![point("TT 27C", SEALED), point("SS 27C", CORNER)],
        });

    assert_eq!(
        ExecutedDeckVerification::of(&harness.model, sequence, 0),
        Some(ExecutedDeckVerification::Verified),
        "the point that solved the sealed source reproduces its digest"
    );
    assert_eq!(
        ExecutedDeckVerification::of(&harness.model, sequence, 1),
        Some(ExecutedDeckVerification::PointVariant),
        "a corner's materialized source is unsealed, which is not the same as wrong"
    );
    assert!(
        reveal_executed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            &harness.runs,
            &mut harness.wb,
            sequence,
            0
        ),
        "the deck opens at the point asked for"
    );
    assert_eq!(
        harness
            .state
            .netlist
            .executed_deck_view
            .map(|selection| selection.verification),
        Some(ExecutedDeckVerification::Verified),
        "and the verdict travels with the selection the strip reads"
    );

    // Every retained byte rewritten: nothing this run holds reproduces what
    // its receipt sealed, and the viewer must not soften that into "unsealed".
    harness
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: sequence,
            points: vec![point("TT 27C", "rewritten deck\n.end\n")],
        });
    assert_eq!(
        ExecutedDeckVerification::of(&harness.model, sequence, 0),
        Some(ExecutedDeckVerification::Unmatched),
        "a deck the run was never authorized over says exactly that"
    );

    let mut unsealed = owned_dependency_state();
    let mut legacy = crate::state::SimulationRun::new(3);
    legacy
        .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy history is explicitly classified");
    unsealed.model.simulation.retained.runs = vec![legacy].into();
    unsealed
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: 3,
            points: vec![point("TT 27C", SEALED)],
        });
    assert_eq!(
        ExecutedDeckVerification::of(&unsealed.model, 3, 0),
        Some(ExecutedDeckVerification::NotRecorded),
        "a run with no receipt digest has nothing to be checked against"
    );
    assert_eq!(
        ExecutedDeckVerification::of(&unsealed.model, 4, 0),
        None,
        "and a run whose decks are not held is not a verdict at all"
    );
}
