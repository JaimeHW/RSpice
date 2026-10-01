//! Retained source identity and reachability regressions for the import service.

use super::*;

fn import(
    display_name: &str,
    root_member: Option<&str>,
    files: Vec<(String, Vec<u8>)>,
    section: Option<&str>,
) -> Result<(String, ModelLibrary), String> {
    import_project_source_bundle(display_name, root_member, files, section)
}

#[test]
fn explicit_browser_root_ignores_unreachable_documents_binaries_and_sources() {
    let (_, library) = import(
        "selected-root.lib",
        Some("models/root.lib"),
        vec![
            (
                "models/root.lib".to_owned(),
                b".include \"device.inc\"\n".to_vec(),
            ),
            (
                "models/device.inc".to_owned(),
                b".model reachable_d D (IS=2e-14)\n".to_vec(),
            ),
            ("README.txt".to_owned(), b"Installation notes".to_vec()),
            ("datasheet.pdf".to_owned(), vec![0, 0xff, 0, 0xfe]),
            (
                "examples/unrelated.lib".to_owned(),
                b".model unrelated_d D (IS=9e-14)\n".to_vec(),
            ),
        ],
        None,
    )
    .expect("the selected executable closure imports");
    assert_eq!(library.source_contents.len(), 2);
    assert!(library.models.contains_key("reachable_d"));
    assert!(!library.models.contains_key("unrelated_d"));
    assert!(library.source_contents.iter().all(|source| {
        let path = source.path.to_string_lossy();
        !path.ends_with("README.txt") && !path.ends_with("datasheet.pdf")
    }));
}

#[test]
fn browser_bundle_resolves_sibling_names_case_insensitively_without_losing_identity() {
    let (_, library) = import(
        "case-bundle.lib",
        None,
        vec![
            ("ROOT.LIB".to_owned(), b".include \"device.inc\"\n".to_vec()),
            (
                "Device.INC".to_owned(),
                b".model nested_d D (IS=2e-14)\n".to_vec(),
            ),
        ],
        None,
    )
    .expect("browser bundles use portable case-insensitive sibling lookup");
    assert!(
        library
            .source_closure
            .iter()
            .any(|pin| pin.path.ends_with("Device.INC"))
    );
    assert!(library.models.contains_key("nested_d"));
}

#[test]
fn browser_bundle_discovers_native_spectre_include_edges_after_adaptation() {
    let (_, library) = import(
        "spectre-bundle.scs",
        None,
        vec![
            (
                "root.scs".to_owned(),
                b"simulator lang=spectre\ninclude \"device.scs\"\n".to_vec(),
            ),
            (
                "device.scs".to_owned(),
                b"simulator lang=spectre\nmodel native_d diode { is=2e-14 }\n".to_vec(),
            ),
        ],
        None,
    )
    .expect("adapted native Spectre includes retain authenticated edges");
    assert_eq!(library.source_edges.len(), 1);
    assert!(library.models.contains_key("native_d"));
}
