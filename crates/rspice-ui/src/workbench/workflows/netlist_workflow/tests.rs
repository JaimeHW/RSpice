//! Netlist workflow tests.

use super::bundle::*;
use super::compose::*;
use super::external_change::*;
use super::import::*;
use super::platform::*;
use super::save::*;
use super::staging::*;
use super::*;

const GENERATED_BASE: &str = "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.end\n";

#[test]
fn started_netlist_import_routes_every_calling_page_to_its_completion_owner() {
    let mut state = AppState::default();
    state.workbench.workspace = crate::workbench::state::Workspace::Models;
    state.ui.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;

    assert!(!route_started_netlist_import(&mut state, false));
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Models
    );
    assert_eq!(
        state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation
    );

    assert!(route_started_netlist_import(&mut state, true));
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Netlist
    );
    assert_eq!(
        state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    );
}

#[test]
fn import_decoder_and_encoder_preserve_supported_file_boundaries() {
    let source = "* caf\u{00e9}\r\nV1 out 0 1\r\n.op\r\n.end\r\n";
    let utf16 = crate::state::NetlistTextEncoding::Utf16LeBom
        .encode(source)
        .expect("UTF-16 encoding succeeds");
    let (decoded, encoding) = decode_import_bytes(&utf16).expect("UTF-16 import decodes");

    assert_eq!(decoded, source);
    assert_eq!(encoding, crate::state::NetlistTextEncoding::Utf16LeBom);
    assert_eq!(
        crate::state::NetlistLineEnding::detect(&decoded),
        crate::state::NetlistLineEnding::Crlf
    );
    assert_eq!(encoding.encode(&decoded).unwrap(), utf16);

    let latin1 = crate::state::NetlistTextEncoding::Latin1
        .encode(source)
        .expect("Latin-1 fixture is representable");
    let (decoded, encoding) = decode_import_bytes(&latin1).expect("Latin-1 import decodes");
    assert_eq!(encoding, crate::state::NetlistTextEncoding::Latin1);
    assert_eq!(decoded, source);
    assert_eq!(encoding.encode(&decoded).unwrap(), latin1);
    assert!(encoding.encode("\u{20ac}").is_err());
}

fn generated_bundle_fixture() -> Vec<u8> {
    use crate::product::{ContentDigest, ObjectRevision};
    use crate::state::{
        DependencyMetadata, GeneratedArtifact, GeneratedProvenance, GenerationInput, SourceLocator,
    };

    let root = "bundle fixture\n.include \"models/base.lib\"\nV1 out 0 1\n.op\n.end\n";
    let base = SourceLocator::try_new("models/base.lib", "base.lib").unwrap();
    let child = SourceLocator::try_new("models/devices/core.lib", "core.lib").unwrap();
    let dependencies = vec![
        DependencyMetadata::unresolved_direct_to(0, "models/base.lib", base.clone())
            .unwrap()
            .with_authority(crate::state::DependencySourceAuthority::Vendor)
            .resolve_utf8(b".include \"devices/core.lib\"\n.model base nmos level=1\n".to_vec())
            .unwrap(),
        DependencyMetadata::unresolved_transitive_to(base, 0, "devices/core.lib", child)
            .unwrap()
            .with_authority(crate::state::DependencySourceAuthority::TechnologyPackage)
            .resolve_utf8(b".model core nmos level=1 vto=0.45\n".to_vec())
            .unwrap(),
    ];
    let artifact = GeneratedArtifact::try_from_utf8(
        GeneratedProvenance::try_new(
            "netlist-bundle-import-test",
            GenerationInput::new(
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5a; 32]),
            ),
        )
        .unwrap(),
        root.as_bytes().to_vec(),
        dependencies,
        Vec::new(),
    )
    .unwrap();
    // SPICE is already the canonical dialect, so the fixture renders each
    // document unchanged and the bundle under test is exactly what the
    // generator produced.
    crate::io::build_generated_bundle(&artifact, crate::io::NetlistFormat::Spice, true, str::to_owned)
        .unwrap()
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
    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        bytes,
        None,
        "portable-run.zip".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_ref().unwrap();
    assert!(review.archive_import);
    assert_eq!(review.display_name, "portable-run.spice");
    assert!(review.source_path.is_none());
    assert_eq!(review.dependencies.len(), 2);

    assert!(commit_staged_netlist_import(&mut state));
    assert!(state.workspace.netlist_source_path.is_none());
    assert_eq!(
        state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.as_str()),
        Some("portable-run.spice")
    );
    let document = state.workspace.netlist_document.as_ref().unwrap();
    assert!(document.dependency_graph_is_sealed());
    assert_eq!(document.dependencies().len(), 2);
}

#[test]
fn generated_bundle_import_rejects_tampered_member_bytes() {
    let mut bytes = generated_bundle_fixture();
    let needle = b"vto=0.45";
    let offset = bytes
        .windows(needle.len())
        .position(|window| window == needle)
        .expect("fixture contains retained model bytes");
    bytes[offset] ^= 0x01;

    let error = parse_generated_netlist_bundle(&bytes).expect_err("tamper must fail closed");
    assert!(error.contains("CRC-32 verification"), "{error}");
}

#[test]
fn three_way_merge_combines_independent_lines_and_marks_overlaps() {
    let base = ".param a=1\n.param b=2\n.op\n";
    let local = ".param a=10\n.param b=2\n.op\n";
    let external = ".param a=1\n.param b=20\n.op\n";
    let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
    assert_eq!(conflicts, 0);
    assert_eq!(merged, ".param a=10\n.param b=20\n.op\n");

    let external = ".param a=100\n.param b=2\n.op\n";
    let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
    assert_eq!(conflicts, 1);
    assert!(merged.contains("<<<<<<< RSPICE LOCAL"));
    assert!(merged.contains(".param a=10"));
    assert!(merged.contains(".param a=100"));

    let base = "a\nb\nc\n";
    let local = "a\ninserted locally\nb\nc\n";
    let external = "a\nb\nchanged externally\n";
    let (merged, conflicts) = three_way_merge_source(Some(base), local, external);
    assert_eq!(conflicts, 0);
    assert_eq!(merged, "a\ninserted locally\nb\nchanged externally\n");
}

#[test]
fn staged_import_is_cancel_safe_and_commits_only_the_reviewed_snapshot() {
    let mut state = AppState::default();
    let original_project = state.workspace.project.id();
    let original_source = state.workspace.netlist_source.clone();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* staged\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();

    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source.clone(),
        None,
        "staged.cir".to_owned(),
    ));
    assert_eq!(state.workspace.project.id(), original_project);
    assert_eq!(state.workspace.netlist_source, original_source);
    assert!(state.ui.netlist.import_review.is_some());

    assert!(commit_staged_netlist_import(&mut state));
    assert_eq!(
        state.workspace.netlist_source.as_deref(),
        std::str::from_utf8(&source).ok()
    );
    assert!(state.ui.netlist.import_review.is_none());
    assert_eq!(
        state
            .workspace
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
    assert!(apply_imported_netlist(
        &mut state,
        original.to_owned(),
        None,
        "history.cir",
    ));
    let baseline_revision = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .revision_history[0]
        .document_revision;
    assert!(
        crate::workbench::documents::netlist_document::replace_owned_source(
            &mut state,
            modified.to_owned(),
        )
    );
    let modified_revision = state
        .workspace
        .netlist_document
        .as_ref()
        .unwrap()
        .revision()
        .get();
    assert!(modified_revision > baseline_revision);

    crate::workbench::documents::netlist_document::compare_owned_revision(&mut state, 0)
        .expect("comparison opens");
    assert!(state.simulation.netlist_content.contains("-V1 out 0 1"));
    crate::workbench::documents::netlist_document::close_revision_comparison(&mut state);
    assert_eq!(state.simulation.netlist_content, modified);

    crate::workbench::documents::netlist_document::restore_owned_revision(&mut state, 0)
        .expect("history restore commits");
    assert_eq!(state.workspace.netlist_source.as_deref(), Some(original));
    let restored_revision = state
        .workspace
        .netlist_document
        .as_ref()
        .unwrap()
        .revision()
        .get();
    assert!(restored_revision > modified_revision);
    let history = &state
        .workspace
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .revision_history;
    assert_eq!(history.len(), 3);
    assert_eq!(history[1].source, modified);
    assert_eq!(history[2].source, original);
    state
        .workspace
        .validate_simulation_configuration()
        .expect("restored workspace validates");

    let persisted = serde_json::to_vec(&state.workspace).expect("serialize workspace");
    let restored: crate::state::ProjectWorkspace =
        serde_json::from_slice(&persisted).expect("deserialize workspace");
    restored
        .validate_simulation_configuration()
        .expect("persisted history validates");
    assert_eq!(
        restored
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history,
        *history
    );
}

#[test]
fn detected_foreign_dialect_requires_explicit_acceptance() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* hspice deck\n.option post=2\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    let retained_source = String::from_utf8(source.clone()).unwrap();

    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "foreign.sp".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_ref().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Hspice
    );
    assert!(!review.can_commit());
    assert_eq!(review.blocking_issue_count(), 0);

    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_ok());
    assert!(review.can_commit());
    review.selected_dialect = crate::state::NetlistSourceDialect::RSpice;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());
    review.selected_dialect = crate::state::NetlistSourceDialect::Hspice;
    assert!(commit_staged_netlist_import(&mut state));
    assert_eq!(
        state.workspace.netlist_source.as_deref(),
        Some(retained_source.as_str())
    );
    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
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
    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "standard-subset.cir".to_owned(),
    ));

    {
        let review = state.ui.netlist.import_review.as_mut().unwrap();
        review.selected_dialect = NetlistSourceDialect::Spice3Ngspice;
        review.compatibility_accepted = true;
        assert!(review.dialect_qualification().is_ok());
        assert!(review.can_commit());
    }

    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Hspice;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Spectre;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Ads;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let dialect = NetlistSourceDialect::Unknown;
    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = dialect;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err(), "{dialect:?}");
    assert!(!review.can_commit(), "{dialect:?}");

    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::Pspice;
    review.compatibility_accepted = true;
    assert!(review.dialect_qualification().is_err());
    assert!(!review.can_commit());

    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = NetlistSourceDialect::RSpice;
    review.compatibility_accepted = false;
    assert!(review.dialect_qualification().is_ok());
    assert!(review.can_commit());
    cancel_staged_netlist_import(&mut state);
}

#[test]
fn spectre_spice_interoperability_import_preserves_source_and_profile() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"simulator lang=spice\n* SPICE interoperability deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    let retained_source = String::from_utf8(source.clone()).unwrap();

    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "spectre-spice.scs".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Spectre
    );
    assert_eq!(review.blocking_issue_count(), 0);
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(commit_staged_netlist_import(&mut state));
    assert_eq!(
        state.workspace.netlist_source.as_deref(),
        Some(retained_source.as_str())
    );
    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
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

    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "ads-spice-export.net".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Ads
    );
    assert_eq!(review.blocking_issue_count(), 0);
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(commit_staged_netlist_import(&mut state));
    assert_eq!(
        state.workspace.netlist_source.as_deref(),
        Some(retained_source.as_str())
    );
    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
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
    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "declarative-spice3.cir".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.detected_dialect,
        crate::state::NetlistSourceDialect::Spice3Ngspice
    );
    assert_eq!(review.blocking_issue_count(), 0);
    review.selected_dialect = crate::state::NetlistSourceDialect::Spice3Ngspice;
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(commit_staged_netlist_import(&mut state));

    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Spice3Ngspice)
    );
    assert!(descriptor.compatibility_reviewed);
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
    );
    let persisted = serde_json::to_vec(&state.workspace).unwrap();
    let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&persisted).unwrap();
    assert_eq!(
        restored
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
    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "declarative-pspice.cir".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_mut().unwrap();
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
    assert!(commit_staged_netlist_import(&mut state));

    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
    assert_eq!(
        descriptor.imported_dialect,
        Some(crate::state::NetlistSourceDialect::Pspice)
    );
    assert!(descriptor.compatibility_reviewed);
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::PspiceDeclarativeV2)
    );
    let persisted = serde_json::to_vec(&state.workspace).unwrap();
    let restored: crate::state::ProjectWorkspace = serde_json::from_slice(&persisted).unwrap();
    assert_eq!(
        restored
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.execution_profile),
        Some(crate::state::NetlistExecutionProfile::PspiceDeclarativeV2)
    );
}

fn quarantined_owned_ngspice_state() -> AppState {
    let source = "* retained declarative source\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    assert!(apply_imported_netlist(
        &mut state,
        source.to_owned(),
        None,
        "legacy-ngspice.cir",
    ));
    let descriptor = state.workspace.netlist_descriptor.as_mut().unwrap();
    descriptor.imported_dialect = Some(crate::state::NetlistSourceDialect::Spice3Ngspice);
    descriptor.compatibility_reviewed = false;
    descriptor.execution_profile = None;
    state.workspace.netlist_source_dirty = false;
    state.workspace.project_metadata_dirty = false;
    state
        .workspace
        .validate_simulation_configuration()
        .expect("quarantined source remains a valid, non-executable project");
    state
}

#[test]
fn quarantined_owned_source_requalifies_without_external_file_or_source_replacement() {
    let mut state = quarantined_owned_ngspice_state();
    let source = state.workspace.netlist_source.clone();
    let history = state
        .workspace
        .netlist_descriptor
        .as_ref()
        .unwrap()
        .revision_history
        .clone();

    assert!(begin_owned_netlist_profile_review(&mut state));
    let review = state.ui.netlist.import_review.as_mut().unwrap();
    assert_eq!(
        review.operation,
        crate::workbench::documents::netlist_document::NetlistImportOperation::RequalifyOwnedSource
    );
    assert!(review.selected_file_path.is_none());
    assert_eq!(review.source, source.as_deref().unwrap());
    review.compatibility_accepted = true;
    assert!(review.can_commit());
    assert!(commit_staged_netlist_import(&mut state));

    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
    assert_eq!(
        descriptor.execution_profile,
        Some(crate::state::NetlistExecutionProfile::Spice3NgspiceV2)
    );
    assert!(descriptor.compatibility_reviewed);
    assert!(!descriptor.execution_profile_review_required());
    assert_eq!(descriptor.revision_history, history);
    assert_eq!(state.workspace.netlist_source, source);
    assert!(state.workspace.project_metadata_dirty);
    assert!(state.ui.netlist.import_review.is_none());
}

#[test]
fn owned_profile_review_rejects_source_revision_change_transactionally() {
    let mut state = quarantined_owned_ngspice_state();
    assert!(begin_owned_netlist_profile_review(&mut state));
    state
        .ui
        .netlist
        .import_review
        .as_mut()
        .unwrap()
        .compatibility_accepted = true;
    assert!(
        crate::workbench::documents::netlist_document::replace_owned_source(
            &mut state,
            "* changed after review\nV1 out 0 2\nR1 out 0 1k\n.op\n.end\n".to_owned(),
        )
    );

    assert!(!commit_staged_netlist_import(&mut state));
    let descriptor = state.workspace.netlist_descriptor.as_ref().unwrap();
    assert!(descriptor.execution_profile.is_none());
    assert!(!descriptor.compatibility_reviewed);
    assert!(
        state
            .ui
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
    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "imperative-ngspice.cir".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_mut().unwrap();
    review.selected_dialect = crate::state::NetlistSourceDialect::Spice3Ngspice;
    review.compatibility_accepted = true;
    let error = review.dialect_qualification().unwrap_err();
    assert!(error.contains("spice3-ngspice/2"));
    assert!(error.contains("wrdata"));
    assert!(!review.can_commit());
    cancel_staged_netlist_import(&mut state);
}

#[test]
fn semantic_loss_parser_diagnostics_block_import() {
    let mut state = AppState::default();
    let transaction =
        crate::workbench::lifecycle::project_lifecycle::begin_project_replacement(&mut state)
            .expect("replacement transaction starts");
    let source = b"* ignored option is unsafe\n.options definitely_unknown=7\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_vec();
    assert!(stage_netlist_import(
        &mut state,
        transaction,
        NetlistImportMode::ImportIntoProject,
        source,
        None,
        "unknown-option.cir".to_owned(),
    ));
    let review = state.ui.netlist.import_review.as_ref().unwrap();
    assert!(review.blocking_issue_count() > 0);
    assert!(review.issues.iter().any(|issue| {
        issue.severity
            == crate::workbench::documents::netlist_document::NetlistImportIssueSeverity::Blocking
            && issue.message.contains("unknown-option")
    }));
    assert!(!review.can_commit());
    cancel_staged_netlist_import(&mut state);
}

fn state_with_owned_strategy(
    generated_source: &str,
    authored_source: &str,
    strategy: crate::state::OwnedNetlistEditStrategy,
) -> AppState {
    use crate::product::ObjectRevision;
    use crate::state::{
        GeneratedArtifact, GeneratedProvenance, GenerationInput, NetlistDocument,
        NetlistDocumentId, content_digest,
    };

    let provenance = GeneratedProvenance::try_new(
        "rspice-netlist-workflow-test",
        GenerationInput::new(ObjectRevision::INITIAL, content_digest("test-inputs")),
    )
    .expect("valid generated provenance");
    let artifact = GeneratedArtifact::try_from_utf8(
        provenance,
        generated_source.as_bytes().to_vec(),
        Vec::new(),
        Vec::new(),
    )
    .expect("valid generated artifact");
    let generated = NetlistDocument::from_generated(NetlistDocumentId::new(), artifact)
        .expect("valid generated document");
    let mut owned = generated
        .create_editable_copy(NetlistDocumentId::new(), generated.content_digest())
        .expect("editable copy");
    owned
        .replace_editable_source(owned.content_digest(), authored_source.as_bytes().to_vec())
        .expect("authored source");

    let mut state = AppState::default();
    state.workspace.netlist_source = Some(authored_source.to_owned());
    state.workspace.netlist_document = Some(owned);
    state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        artifact_name: "owned.cir".to_owned(),
        strategy,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::detect(authored_source),
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });
    state
}

#[test]
fn owned_source_strategy_executes_exact_authored_bytes_without_generated_composition() {
    let authored = "* independently owned\r\nV9 out 0 9\r\n.tran 1n 10n\r\n.end\r\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::OwnedSource,
    );

    let composed = compose_owned_netlist_execution_source(&state, authored)
        .expect("owned source is executable");

    assert_eq!(composed.as_bytes(), authored.as_bytes());
}

#[test]
fn parameter_option_override_retains_base_and_appends_override_before_end() {
    let authored = "* project corner\n.param gain=22\n+ trim=0.5\n.options method=gear\n.temp 85\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );

    let composed = compose_owned_netlist_execution_source(&state, authored)
        .expect("parameter override is executable");

    assert_eq!(
        composed,
        "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n* project corner\n.param gain=22\n+ trim=0.5\n.options method=gear\n.temp 85\n.end"
    );
}

#[test]
fn include_order_override_replaces_all_base_include_cards_and_continuations() {
    let authored =
        ".lib \"models/b.lib\" SS\n.include \"models/a.lib\"\n.veriloga \"models/device.va\"\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
    );

    let composed = compose_owned_netlist_execution_source(&state, authored)
        .expect("include-order override is executable");

    assert_eq!(
        composed,
        "* generated\n.option reltol=1e-3\n.param gain=10\nV1 out 0 1\nR1 out 0 1k\n.op\n.measure op vout FIND V(out)\n.save V(out)\n.lib \"models/b.lib\" SS\n.include \"models/a.lib\"\n.veriloga \"models/device.va\"\n.end"
    );
    assert!(!composed.contains("section=fast"));
}

#[test]
fn analysis_only_deck_replaces_base_analysis_measurement_and_output_cards() {
    let authored = ".tran 1n 10n\n.measure tran vmax MAX V(out)\n.probe V(out) I(V1)\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
    );

    let composed = compose_owned_netlist_execution_source(&state, authored)
        .expect("analysis-only deck is executable");

    assert_eq!(
        composed,
        "* generated\n.option reltol=1e-3\n.param gain=10\n.include \"models/a.lib\"\n.lib \"models/b.lib\" TT\n+ section=fast\nV1 out 0 1\nR1 out 0 1k\n.tran 1n 10n\n.measure tran vmax MAX V(out)\n.probe V(out) I(V1)\n.end"
    );
    assert!(!composed.contains(".op\n"));
    assert!(!composed.contains(".save V(out)"));
}

#[test]
fn narrow_override_rejects_device_cards_and_cross_strategy_directives() {
    let device = "Roverride out 0 2k\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        device,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );
    let error = compose_owned_netlist_execution_source(&state, device)
        .expect_err("device card must fail closed");
    assert!(error.contains("'roverride'"));
    assert!(error.contains("line 1"));

    let wrong_strategy = ".include \"models/other.lib\"\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        wrong_strategy,
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
    );
    let error = compose_owned_netlist_execution_source(&state, wrong_strategy)
        .expect_err("cross-strategy directive must fail closed");
    assert!(error.contains("'.include'"));
    assert!(error.contains("line 1"));
}

#[test]
fn narrow_override_rejects_orphan_continuation() {
    let authored = "+ sweep=fast\n.param gain=22\n";
    let state = state_with_owned_strategy(
        GENERATED_BASE,
        authored,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );

    let error = compose_owned_netlist_execution_source(&state, authored)
        .expect_err("orphan continuation must fail closed");

    assert_eq!(
        error,
        "Override line 1 is a continuation without an allowed owning card."
    );
}

#[test]
fn narrow_override_requires_retained_generated_base() {
    let authored = ".tran 1n 10n\n";
    let mut state = AppState::default();
    state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
        artifact_name: "analysis.cir".to_owned(),
        strategy: crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck,
        source_encoding: crate::state::NetlistTextEncoding::Utf8,
        source_line_ending: crate::state::NetlistLineEnding::Lf,
        imported_dialect: None,
        compatibility_reviewed: false,
        execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
        external_file_sha256: None,
        save_history: Vec::new(),
        revision_history: Vec::new(),
        owned_includes: Vec::new(),
    });

    let error = compose_owned_netlist_execution_source(&state, authored)
        .expect_err("missing generated base must fail closed");

    assert_eq!(
        error,
        "Narrow override has no retained generated base artifact."
    );
}

#[test]
fn narrow_override_rejects_generated_base_without_end_terminator() {
    let authored = ".param gain=22\n";
    let state = state_with_owned_strategy(
        "* malformed generated base\nR1 out 0 1k\n.op\n",
        authored,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );

    let error = compose_owned_netlist_execution_source(&state, authored)
        .expect_err("missing end terminator must fail closed");

    assert_eq!(error, "Retained generated base has no .end terminator.");
}

#[test]
fn imported_netlist_becomes_dirty_manual_source_without_deleting_retained_runs() {
    let mut state = AppState::default();
    state.simulation.start_run();
    assert!(state.simulation.has_results());

    let imported = apply_imported_netlist(
        &mut state,
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
        state.workspace.netlist_source.as_deref(),
        Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n")
    );
    assert_eq!(
        state.workspace.netlist_source_path.as_deref(),
        Some(std::path::Path::new("bias.cir"))
    );
    assert!(state.workspace.netlist_source_dirty);
    assert!(state.workspace.any_dirty());
    assert!(state.workspace.netlist_document.is_some());
    assert_eq!(
        state
            .workspace
            .netlist_descriptor
            .as_ref()
            .map(|descriptor| descriptor.artifact_name.as_str()),
        Some("bias.cir")
    );
    state
        .workspace
        .validate_simulation_configuration()
        .expect("imported canonical source must satisfy project persistence invariants");
    assert!(state.simulation.has_results());
    assert_eq!(state.simulation.runs.len(), 1);
    assert!(state.recent_files.is_empty());
}

#[test]
fn opening_a_netlist_commits_an_independent_netlist_first_project() {
    let mut state = AppState::default();
    let original_project_id = state.workspace.project.id();
    state.simulation.start_run();
    state.workspace.netlist_source = Some("old\n.op\n.end\n".to_owned());

    assert!(apply_opened_netlist_project(
        &mut state,
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
        Vec::new(),
    ));

    assert_ne!(state.workspace.project.id(), original_project_id);
    assert_eq!(state.workspace.project.name(), "bias");
    assert!(!state.simulation.has_results());
    assert_eq!(
        state.workspace.netlist_source_path.as_deref(),
        Some(std::path::Path::new("bias.cir"))
    );
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Netlist
    );
}

#[test]
fn importing_a_deck_refuses_read_only_projects_without_mutation() {
    let mut state = AppState::default();
    state.workbench.safe_mode.active = true;
    state.workbench.safe_mode.applied = crate::workbench::state::LocalSafeModeOptions {
        open_project_read_only: true,
        ..Default::default()
    };
    state.workspace.netlist_source = Some("old\n.op\n.end\n".to_owned());

    assert!(!apply_imported_netlist(
        &mut state,
        "new\n.op\n.end\n".to_owned(),
        None,
        "new.cir",
    ));
    assert_eq!(
        state.workspace.netlist_source.as_deref(),
        Some("old\n.op\n.end\n")
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn text_first_import_validates_and_retains_explicit_run_authorization_without_schematic() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.schematic.components.clear();
    let source = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

    assert!(apply_imported_netlist(
        &mut app.state,
        source.to_owned(),
        None,
        "standalone.cir",
    ));
    assert!(validate_visible_netlist_source(&mut app));
    assert!(app.state.ui.netlist.validation.is_some());
    assert_eq!(
        app.state.ui.netlist.externally_saved_content_digest,
        Some(crate::workbench::documents::netlist_document::source_content_digest(source))
    );
    assert_eq!(app.manual_deck_run_block_reason(), None);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn validation_publishes_exact_direct_and_transitive_dependency_closure() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-dependency-closure-{}",
        uuid::Uuid::new_v4()
    ));
    let models = dir.join("models");
    std::fs::create_dir_all(&models).expect("create dependency fixture");
    let root = dir.join("root.cir");
    let first_source = ".include \"nested.inc\"\nRmodel out 0 2k\n";
    let nested_source = "Cmodel out 0 1p\n";
    std::fs::write(models.join("first.inc"), first_source).expect("write direct include");
    std::fs::write(models.join("nested.inc"), nested_source).expect("write transitive include");
    let source = "standalone\n.include \"models/first.inc\"\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(apply_imported_netlist(
        &mut app.state,
        source.to_owned(),
        Some(root.clone()),
        "root.cir",
    ));
    assert!(validate_visible_netlist_source(&mut app));

    let document = app
        .state
        .workspace
        .netlist_document
        .as_ref()
        .expect("validated canonical document");
    assert!(document.dependency_graph_is_sealed());
    assert_eq!(document.dependencies().len(), 2);
    let direct = document
        .dependencies()
        .iter()
        .find(|dependency| dependency.direct_include_index() == Some(0))
        .expect("direct dependency");
    assert_eq!(direct.requested_locator(), "models/first.inc");
    assert_eq!(direct.source(), Some(first_source));
    assert_eq!(direct.locator().logical_identity(), "models/first.inc");
    assert!(
        !direct
            .locator()
            .logical_identity()
            .contains(&dir.to_string_lossy().to_string()),
        "portable logical identities must not expose the native project root"
    );
    let transitive = document
        .dependencies()
        .iter()
        .find(|dependency| dependency.parent().is_some())
        .expect("transitive dependency");
    assert_eq!(transitive.requested_locator(), "nested.inc");
    assert_eq!(transitive.source(), Some(nested_source));
    assert_eq!(transitive.parent_include_index(), Some(0));

    std::fs::remove_dir_all(dir).expect("remove dependency fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn validation_distinguishes_inline_lib_sections_from_external_lib_paths() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-library-closure-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&dir).expect("create library fixture");
    let root = dir.join("root.cir");
    let library_source =
        ".lib TT\n.model DMOD D IS=1e-14\n.endl TT\n.lib SS\n.model DMOD D IS=2e-14\n.endl SS\n";
    std::fs::write(dir.join("corners.lib"), library_source).expect("write library");
    let source = "standalone\n.lib LOCAL\n.model LOCALMOD D IS=3e-14\n.endl LOCAL\n.lib \"corners.lib\" TT\nD1 out 0 DMOD\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(apply_imported_netlist(
        &mut app.state,
        source.to_owned(),
        Some(root),
        "root.cir",
    ));
    assert!(validate_visible_netlist_source(&mut app));

    let document = app.state.workspace.netlist_document.as_ref().unwrap();
    assert_eq!(document.include_directives().len(), 1);
    assert_eq!(document.dependencies().len(), 1);
    assert_eq!(
        document.dependencies()[0].requested_locator(),
        "corners.lib"
    );
    assert_eq!(document.dependencies()[0].source(), Some(library_source));

    std::fs::remove_dir_all(dir).expect("remove library fixture");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn narrow_strategy_dependencies_attach_to_the_source_that_owns_the_directives() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-narrow-dependencies-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&dir).expect("create narrow dependency fixture");
    let root = dir.join("owned.inc");
    std::fs::write(dir.join("base.inc"), "Rbase out 0 1k\n").expect("write base include");
    std::fs::write(dir.join("override.inc"), "Roverride out 0 2k\n")
        .expect("write override include");
    let generated = "* generated\n.include \"base.inc\"\nV1 out 0 1\n.op\n.end\n";

    let parameter_source = ".param gain=22\n";
    std::fs::write(&root, parameter_source).expect("write parameter source");
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state = state_with_owned_strategy(
        generated,
        parameter_source,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );
    app.state.workspace.netlist_source_path = Some(root.clone());
    app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
    app.state.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    app.state.ui.netlist.active_document_initialized = true;
    app.state.simulation.netlist_content = parameter_source.to_owned();

    assert!(validate_visible_netlist_source(&mut app));
    let parameter_document = app.state.workspace.netlist_document.as_ref().unwrap();
    assert!(parameter_document.dependencies().is_empty());
    assert!(
        parameter_document
            .generated_artifact()
            .dependency_graph_is_sealed()
    );
    assert_eq!(
        parameter_document.generated_artifact().dependencies()[0].source(),
        Some("Rbase out 0 1k\n")
    );

    let include_source = ".include \"override.inc\"\n";
    std::fs::write(&root, include_source).expect("write include-order source");
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state = state_with_owned_strategy(
        generated,
        include_source,
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
    );
    app.state.workspace.netlist_source_path = Some(root);
    app.state.ui.netlist.owned_document = app.state.workspace.netlist_document.clone();
    app.state.ui.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    app.state.ui.netlist.active_document_initialized = true;
    app.state.simulation.netlist_content = include_source.to_owned();

    assert!(validate_visible_netlist_source(&mut app));
    let include_document = app.state.workspace.netlist_document.as_ref().unwrap();
    assert!(include_document.dependency_graph_is_sealed());
    assert_eq!(include_document.dependencies().len(), 1);
    assert_eq!(
        include_document.dependencies()[0].requested_locator(),
        "override.inc"
    );
    assert_eq!(
        include_document.dependencies()[0].source(),
        Some("Roverride out 0 2k\n")
    );

    std::fs::remove_dir_all(dir).expect("remove narrow dependency fixture");
}

#[test]
fn empty_netlist_import_is_rejected_without_clearing_existing_state() {
    let mut state = AppState::default();
    state.workspace.netlist_source = Some("existing\n.op\n.end\n".to_owned());
    state.workspace.netlist_source_path = Some(std::path::PathBuf::from("existing.cir"));
    state.simulation.netlist_content = "existing\n.op\n.end\n".to_owned();

    let imported = apply_imported_netlist(&mut state, " \n\t".to_owned(), None, "empty.cir");

    assert!(!imported);
    assert_eq!(
        state.workspace.netlist_source.as_deref(),
        Some("existing\n.op\n.end\n")
    );
    assert_eq!(
        state.workspace.netlist_source_path.as_deref(),
        Some(std::path::Path::new("existing.cir"))
    );
    assert_eq!(state.simulation.netlist_content, "existing\n.op\n.end\n");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn ordinary_source_save_refuses_to_overwrite_external_changes() {
    let path = std::env::temp_dir().join(format!(
        "rspice-netlist-external-conflict-{}.cir",
        uuid::Uuid::new_v4()
    ));
    let authored = "* owned\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let external = "* changed by another editor\nV1 out 0 2\n.op\n.end\n";
    std::fs::write(&path, authored).expect("write imported source");
    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(apply_imported_netlist(
        &mut app.state,
        authored.to_owned(),
        Some(path.clone()),
        "owned.cir",
    ));
    assert!(validate_visible_netlist_source(&mut app));
    std::fs::write(&path, external).expect("publish external edit");

    assert!(!save_owned_netlist_source(
        &mut app.state,
        &app.simulation_controller,
        &crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
        false,
        "Attempt conflicting save",
    ));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), external);
    assert!(app.state.ui.netlist.external_change.is_some());
    app.state
            .ui
            .netlist
            .external_change
            .as_mut()
            .unwrap()
            .resolution = crate::workbench::documents::netlist_document::NetlistExternalChangeResolution::ReloadExternal;
    apply_staged_external_netlist_change(&mut app.state)
        .expect("explicit external reload succeeds");
    assert_eq!(
        app.state.workspace.netlist_source.as_deref(),
        Some(external)
    );
    assert!(app.state.ui.netlist.external_change.is_none());
    assert_eq!(
        app.state.ui.netlist.externally_saved_content_digest,
        Some(crate::state::content_digest(external))
    );
    assert!(
        app.state
            .workspace
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history
            .len()
            >= 2
    );
    std::fs::remove_file(path).expect("remove conflict fixture");
}
