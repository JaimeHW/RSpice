//! Source document dialog tests: include search roots, role labels, the lifecycle
//! transactions, and the creation form a project with no deck opens.

use super::fixtures::english;
use super::*;
use crate::state::ProjectDescriptor;
use rspice_simulation::netlist_preparation::IncludeSearchChain;
use std::path::{Path, PathBuf};

#[test]
fn a_project_places_its_relative_entries_against_its_own_folder() {
    let mut project = ProjectDescriptor::default();
    project.path = Some(PathBuf::from("/projects/mixer/mixer.rspiceproj"));
    project
        .set_include_search_paths(vec![PathBuf::from("models"), PathBuf::from("/opt/pdk/lib")])
        .expect("chain is accepted");

    let directories = rspice_simulation::netlist_preparation::IncludeSearchChain::resolve(
        project.include_search_paths(),
        project.data_root(),
    )
    .directories();
    assert_eq!(directories.len(), 2);
    assert_eq!(directories[0], Path::new("/projects/mixer").join("models"));
    assert_eq!(directories[1], PathBuf::from("/opt/pdk/lib"));
}

#[test]
fn relative_entries_resolve_against_the_project_data_root() {
    let root = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-include-chain-relative-{}",
        std::process::id()
    ));
    let models = root.join("models");
    std::fs::create_dir_all(&models).expect("create chain fixture");

    let chain = IncludeSearchChain::resolve(
        &[PathBuf::from("models"), PathBuf::from("absent")],
        Some(root.as_path()),
    );

    assert_eq!(chain.entries().len(), 2);
    assert_eq!(chain.entries()[0].resolved(), models.as_path());
    assert!(chain.entries()[0].exists());
    assert_eq!(chain.entries()[1].resolved(), root.join("absent"));
    assert!(!chain.entries()[1].exists());
    std::fs::remove_dir_all(&root).ok();
}

/// The choice of engine entry point lives here and nowhere else, so it is
/// asserted by its effect: the same deck refuses to resolve without a
/// chain and resolves through it with one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_chain_decides_which_engine_entry_point_a_host_parse_takes() {
    let root = crate::fixture_root::canonical_temp_dir().join(format!(
        "rspice-include-chain-entry-point-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    let deck_dir = root.join("deck");
    let library_dir = root.join("pdk");
    std::fs::create_dir_all(&deck_dir).expect("create deck directory");
    std::fs::create_dir_all(&library_dir).expect("create library directory");
    std::fs::write(
        library_dir.join("device.lib"),
        ".model DCHAIN D(Is=1e-14)\n",
    )
    .expect("write library");
    let deck_path = deck_dir.join("top.cir");
    let source = "chain entry point\n.include device.lib\nD1 out 0 DCHAIN\n.end\n";
    let options = rspice_core::netlist::NetlistParseOptions::default();

    let without = IncludeSearchChain::default().parse_with_abort(
        source,
        Some(&deck_path),
        options,
        &rspice_core::abort_signal::NoAbort,
    );
    assert!(
        without.is_err(),
        "without a chain the include is nowhere the resolver looks"
    );

    let chain = IncludeSearchChain::resolve(&[library_dir.clone()], Some(root.as_path()));
    let with = chain
        .parse_with_abort(
            source,
            Some(&deck_path),
            options,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the chain resolves the include");
    assert_eq!(with.source_path.as_deref(), Some(deck_path.as_path()));

    std::fs::remove_dir_all(&root).ok();
}

/// Line endings are reported from the retained bytes. A whole-file rewrite
/// normalizes a mixed document, so the dialog has to be able to say so.
#[test]
fn line_endings_are_measured_rather_than_assumed() {
    let english = english();
    let summary = |content| english.text(line_ending_summary(content));
    assert_eq!(summary("a\nb\n"), "LF");
    assert_eq!(summary("a\r\nb\r\n"), "CRLF");
    assert_eq!(summary("a\r\nb\n"), "mixed LF and CRLF");
    assert_eq!(summary("single"), "none \u{00b7} single line");
}
