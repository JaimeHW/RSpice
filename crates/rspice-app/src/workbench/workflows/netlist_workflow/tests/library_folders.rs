//! Validating a deck whose include only a library folder holds.

use super::*;
use crate::workbench::FolderListPreference;

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
    assert!(apply_imported_netlist(
        &mut app.state,
        source.to_owned(),
        Some(root),
        "root.cir",
    ));
    assert!(
        !validate_visible_netlist_source(&mut app),
        "nothing the deck is searched from holds device.inc"
    );

    app.state
        .ui
        .preferences
        .set_folders(FolderListPreference::LibraryFolders, vec![shared.clone()])
        .expect("a full path");
    assert!(validate_visible_netlist_source(&mut app));
    let document = app
        .state
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
        .ui
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
