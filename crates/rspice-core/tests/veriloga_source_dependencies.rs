#![cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]

#[path = "common/veriloga_source_fixture.rs"]
mod fixture;

use fixture::Fixture;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::{Engine, Netlist, ResourceKind, SimulationConfig, SimulationError};

#[test]
fn discovery_tracks_active_nested_sources_without_compiling_a_module() {
    let tree = Fixture::new("dependency_discovery");
    let root = tree.write("model.va", "`include \"disciplines.vams\"\n`define ACTIVE\n`include \"header.vh\"\nThis is not a valid module.\n");
    let header = tree.write(
        "header.vh",
        "`ifdef ACTIVE\n`include \"empty.vh\"\n`else\n`include \"does-not-exist.vh\"\n`endif\n",
    );
    let empty = tree.write("empty.vh", "");
    let deck = tree.write(
        "deck.cir",
        "sources\n.va model.va FIRST module=first\n.va model.va SECOND module=second\n.end\n",
    );
    let netlist = Netlist::parse_file(&deck).unwrap();
    let actual = Engine::default()
        .veriloga_source_dependencies_with_abort(&netlist, &NoAbort)
        .unwrap();
    let mut expected = vec![
        root.canonicalize().unwrap(),
        header.canonicalize().unwrap(),
        empty.canonicalize().unwrap(),
    ];
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn compiler_dependency_discovery_honors_macro_options_and_include_paths() {
    use rspice_veriloga::{CompilerOptions, FileSystemSourceProvider, VerilogACompiler};
    let tree = Fixture::new("configured_dependency_discovery");
    let root = tree.write(
        "model.va",
        "`ifdef CHOSEN\n`include \"selected.vh\"\n`else\n`include \"absent.vh\"\n`endif\n",
    );
    let header = tree.write("headers/selected.vh", "");
    let compiler = VerilogACompiler::new(CompilerOptions {
        include_paths: vec![tree.root().join("headers")],
        defines: vec![("CHOSEN".into(), None)],
        ..CompilerOptions::default()
    });
    let actual = compiler
        .provider_source_dependencies(&FileSystemSourceProvider, &root)
        .unwrap();
    let mut expected = vec![root.canonicalize().unwrap(), header.canonicalize().unwrap()];
    expected.sort();
    assert_eq!(actual, expected);
}

#[test]
fn dependency_discovery_preserves_cancellation_and_typed_resource_limits() {
    struct Cancel;
    impl AbortSignal for Cancel {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    let tree = Fixture::new("bounded_dependency_discovery");
    tree.write("model.va", "`include \"large.vh\"\n");
    tree.write("large.vh", &" ".repeat(1024));
    let deck = tree.write("deck.cir", "sources\n.va model.va\n.end\n");
    let netlist = Netlist::parse_file(&deck).unwrap();
    assert!(matches!(
        Engine::default().veriloga_source_dependencies_with_abort(&netlist, &Cancel),
        Err(SimulationError::Aborted)
    ));
    struct CancelDuringRead(std::sync::atomic::AtomicUsize);
    impl AbortSignal for CancelDuringRead {
        fn is_aborted(&self) -> bool {
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed) >= 7
        }
    }
    let cancel_during_read = CancelDuringRead(std::sync::atomic::AtomicUsize::new(0));
    assert!(matches!(
        Engine::default().veriloga_source_dependencies_with_abort(&netlist, &cancel_during_read),
        Err(SimulationError::Aborted)
    ));
    let mut config = SimulationConfig::default();
    config.resource_limits.max_dependency_source_bytes = 128;
    assert!(
        matches!(Engine::new(config).veriloga_source_dependencies_with_abort(&netlist, &NoAbort), Err(SimulationError::ResourceLimit(error)) if error.resource == ResourceKind::DependencySourceBytes)
    );
    for (depth, expanded, expected) in [
        (1, 4096, ResourceKind::IncludeDepth),
        (8, 32, ResourceKind::ExpandedSourceBytes),
    ] {
        let mut config = SimulationConfig::default();
        config.resource_limits.max_include_depth = depth;
        config.resource_limits.max_expanded_source_bytes = expanded;
        assert!(
            matches!(Engine::new(config).veriloga_source_dependencies_with_abort(&netlist, &NoAbort), Err(SimulationError::ResourceLimit(error)) if error.resource == expected)
        );
    }
}

#[test]
fn virtual_models_have_no_filesystem_dependency() {
    let netlist = Netlist::parse("virtual\n.va __rspice_project__/model.va\n.end\n").unwrap();
    assert!(
        Engine::default()
            .veriloga_source_dependencies_with_abort(&netlist, &NoAbort)
            .unwrap()
            .is_empty()
    );
}
