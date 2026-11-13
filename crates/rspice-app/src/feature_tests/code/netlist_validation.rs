//! Validating the visible deck, in whole applications: what an import, an
//! edit, a narrow override or a library folder leaves validated, which
//! dependencies the validation seals, and how a save after it meets an
//! external change.

use crate::workbench::FolderListPreference;
use crate::workbench::commands::netlist_validation::validate_visible_netlist_source;
use crate::workbench::workflows::netlist_workflow::tests::seed_owned_strategy;
use crate::workbench::workflows::netlist_workflow::tests::vendor_profiles::{cases, source};
use crate::workbench::workflows::netlist_workflow::{
    apply_staged_external_netlist_change, save_owned_netlist_source,
};
use crate::workbench::{AppState, RSpiceApp};

#[test]
fn text_first_import_validates_and_runs_without_claiming_a_native_save_binding() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let source = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

    assert!(
        app.state
            .apply_imported_netlist(source.to_owned(), None, "standalone.cir")
    );
    assert!(validate_visible_netlist_source(&mut app));
    assert!(app.state.model.netlist_artifacts.validation.is_some());
    assert!(
        app.state
            .code
            .netlist
            .externally_saved_content_digest
            .is_none()
    );
    assert_eq!(app.manual_deck_run_block_reason(), None);
}

#[test]
fn validated_owned_source_can_run_before_external_source_synchronization() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let imported = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let edited = "standalone edited\nV1 out 0 2\nR1 out 0 2k\n.op\n.end\n";

    assert!(
        app.state
            .apply_imported_netlist(imported.to_owned(), None, "standalone.cir")
    );
    assert!(
        crate::workbench::documents::netlist_document::replace_owned_source(
            &mut app.state.model,
            &mut app.state.code.netlist,
            &mut app.state.code.netlist_open_documents,
            &mut app.state.workbench,
            edited.to_owned()
        )
    );
    let edited_digest = crate::state::content_digest(edited);
    assert_ne!(
        app.state.code.netlist.externally_saved_content_digest,
        Some(edited_digest)
    );

    assert!(validate_visible_netlist_source(&mut app));
    assert_eq!(app.manual_deck_run_block_reason(), None);
}

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
    assert!(
        app.state
            .apply_imported_netlist(source.to_owned(), Some(root.clone()), "root.cir")
    );
    assert!(validate_visible_netlist_source(&mut app));

    let document = app
        .state
        .model
        .workspace
        .content
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
    assert!(
        app.state
            .apply_imported_netlist(source.to_owned(), Some(root), "root.cir")
    );
    assert!(validate_visible_netlist_source(&mut app));

    let document = app
        .state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert_eq!(document.include_directives().len(), 1);
    assert_eq!(document.dependencies().len(), 1);
    assert_eq!(
        document.dependencies()[0].requested_locator(),
        "corners.lib"
    );
    assert_eq!(document.dependencies()[0].source(), Some(library_source));

    std::fs::remove_dir_all(dir).expect("remove library fixture");
}

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
    app.state = AppState::default();
    seed_owned_strategy(
        &mut app.state.model,
        generated,
        parameter_source,
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride,
    );
    app.state.model.workspace.content.netlist_source_path = Some(root.clone());
    app.state.code.netlist.owned_document =
        app.state.model.workspace.content.netlist_document.clone();
    app.state.code.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    app.state.code.netlist.active_document_initialized = true;
    app.state.model.simulation.source.netlist_content = parameter_source.to_owned();

    assert!(validate_visible_netlist_source(&mut app));
    let parameter_document = app
        .state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
    assert!(parameter_document.dependencies().is_empty());
    assert!(
        parameter_document
            .generated_artifact()
            .unwrap()
            .dependency_graph_is_sealed()
    );
    assert_eq!(
        parameter_document
            .generated_artifact()
            .unwrap()
            .dependencies()[0]
            .source(),
        Some("Rbase out 0 1k\n")
    );

    let include_source = ".include \"override.inc\"\n";
    std::fs::write(&root, include_source).expect("write include-order source");
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state = AppState::default();
    seed_owned_strategy(
        &mut app.state.model,
        generated,
        include_source,
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride,
    );
    app.state.model.workspace.content.netlist_source_path = Some(root);
    app.state.code.netlist.owned_document =
        app.state.model.workspace.content.netlist_document.clone();
    app.state.code.netlist.active_document =
        crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource;
    app.state.code.netlist.active_document_initialized = true;
    app.state.model.simulation.source.netlist_content = include_source.to_owned();

    assert!(validate_visible_netlist_source(&mut app));
    let include_document = app
        .state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .unwrap();
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
fn ordinary_source_save_refuses_to_overwrite_external_changes() {
    let path = std::env::temp_dir().join(format!(
        "rspice-netlist-external-conflict-{}.cir",
        uuid::Uuid::new_v4()
    ));
    let authored = "* owned\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let external = "* changed by another editor\nV1 out 0 2\n.op\n.end\n";
    std::fs::write(&path, authored).expect("write imported source");
    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(
        app.state
            .apply_imported_netlist(authored.to_owned(), Some(path.clone()), "owned.cir")
    );
    assert!(validate_visible_netlist_source(&mut app));
    std::fs::write(&path, external).expect("publish external edit");

    assert!(!save_owned_netlist_source(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.workbench,
        &crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
        false,
        crate::state::NetlistTextEncoding::Utf8,
        "Attempt conflicting save",
    ));
    assert_eq!(std::fs::read_to_string(&path).unwrap(), external);
    assert!(app.state.code.netlist.external_change.is_some());
    app.state
            .code
            .netlist
            .external_change
            .as_mut()
            .unwrap()
            .resolution = crate::workbench::documents::netlist_document::NetlistExternalChangeResolution::ReloadExternal;
    apply_staged_external_netlist_change(
        &mut app.state.model,
        &mut app.state.code.netlist,
        &mut app.state.code.netlist_open_documents,
        &app.state.workbench,
    )
    .expect("explicit external reload succeeds");
    assert_eq!(
        app.state.model.workspace.content.netlist_source.as_deref(),
        Some(external)
    );
    assert!(app.state.code.netlist.external_change.is_none());
    assert_eq!(
        app.state.code.netlist.externally_saved_content_digest,
        Some(crate::state::content_digest(external))
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .unwrap()
            .revision_history
            .len()
            >= 2
    );
    std::fs::remove_file(path).expect("remove conflict fixture");
}

#[test]
fn validation_resolves_an_include_only_a_library_folder_holds() {
    let directory = std::env::temp_dir().join(format!(
        "rspice-netlist-library-folder-{}",
        uuid::Uuid::new_v4()
    ));
    let deck = directory.join("deck");
    let shared = directory.join("shared");
    std::fs::create_dir_all(&deck).expect("create deck folder");
    std::fs::create_dir_all(&shared).expect("create library folder");
    let body = "Rmodel out 0 2k\n";
    std::fs::write(shared.join("device.inc"), body).expect("write library include");
    let root = deck.join("root.cir");
    let source = "standalone\n.include \"device.inc\"\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(
        app.state
            .apply_imported_netlist(source.to_owned(), Some(root), "root.cir")
    );
    assert!(
        !validate_visible_netlist_source(&mut app),
        "nothing the deck is searched from holds device.inc"
    );

    app.state
        .model
        .preferences
        .set_folders(FolderListPreference::LibraryFolders, vec![shared.clone()])
        .expect("a full path");
    assert!(validate_visible_netlist_source(&mut app));
    let document = app
        .state
        .model
        .workspace
        .content
        .netlist_document
        .as_ref()
        .expect("validated canonical document");
    assert!(document.dependency_graph_is_sealed());
    let [dependency] = document.dependencies() else {
        panic!("one include: {:?}", document.dependencies());
    };
    assert_eq!(dependency.source(), Some(body));
    let resolution = app
        .state
        .code
        .code_workspace
        .include_resolutions
        .get("device.inc")
        .expect("validation retains where the include resolved");
    assert_eq!(
        resolution.stage(),
        rspice_core::netlist::IncludeSearchStage::LibraryPath(0)
    );
    let canonical = |path: &std::path::Path| std::fs::canonicalize(path).expect("fixture path");
    assert_eq!(
        resolution.resolved().parent().map(canonical),
        Some(canonical(&shared))
    );

    std::fs::remove_dir_all(directory).expect("remove library fixture");
}

#[test]
fn vendor_admitted_decks_have_explicit_preparation_outcomes() {
    let mut failures = Vec::new();
    for (relative, accepted, expected, profile) in cases() {
        if !accepted {
            continue;
        }
        let (source, path) = source(relative);
        let mut app = RSpiceApp::test_instance();
        assert!(
            app.state
                .apply_imported_netlist(source.clone(), Some(path), relative)
        );
        let descriptor = app
            .state
            .model
            .workspace
            .content
            .netlist_descriptor
            .as_mut()
            .unwrap();
        descriptor.imported_dialect = Some(profile.source_dialect());
        descriptor.execution_profile = Some(profile);
        descriptor.compatibility_reviewed = true;
        let ready = validate_visible_netlist_source(&mut app);
        let error = app
            .state
            .model
            .netlist_artifacts
            .validation_error
            .as_deref()
            .unwrap_or_default();
        if (expected == "ready" && !ready)
            || (expected != "ready" && (ready || !error.contains(expected)))
        {
            failures.push(format!(
                "{relative}: expected preparation {expected:?}, got ready={ready}, {error}"
            ));
        }
        assert_eq!(
            app.state.model.workspace.content.netlist_source.as_deref(),
            Some(source.as_str())
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
