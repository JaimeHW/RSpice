use crate::model_sources::SealedModelExecutionSources;
use crate::pdk::PdkTechnologyRegistry;
use crate::project_veriloga::preparation::prepared_pdk_veriloga_runtimes;
use crate::veriloga::thread_compiles;
use rspice_vocabulary::product::ProcessCorner;

/// The sealed sources of an installed PDK whose Verilog-A root carries `tag`,
/// so no other test's preparation compiles the same source.
fn sealed_pdk(tag: &str) -> SealedModelExecutionSources {
    let root = format!(
        "`include \"parts/resistance.vams\"\n// {tag}\nmodule pdk_resistor(p, n);\n    inout p, n;\n    electrical p, n;\n    parameter real r = `PDK_RESISTANCE;\n    analog I(p, n) <+ V(p, n) / r;\nendmodule\n"
    );
    let (bytes, authority) =
        crate::pdk::test_fixtures::fixture_archive_with_veriloga_source(root.as_bytes());
    let mut registry = PdkTechnologyRegistry::default();
    registry
        .install_archive_bytes(&bytes, &authority, "Install PDK Verilog-A fixture")
        .expect("PDK Verilog-A package installs");
    let package = registry.validated_packages()[0].clone();
    let sealed = registry
        .seal_model_sources_for_binding(&package.binding(), package.archive_digest())
        .expect("exact PDK Verilog-A closure seals");
    crate::model_sources::seal_catalog_execution_sources(
        &rspice_model_library::ModelCatalog::default(),
        &rspice_model_library::ModelResolutionRecords::default(),
    )
    .unwrap()
    .with_pdk_model_sources(sealed)
    .unwrap()
}

/// Every member of a campaign prepares the same PDK source; the first
/// compiles it and the rest read that runtime.
#[test]
fn a_pdk_source_compiles_once_per_process() {
    let sealed = sealed_pdk("a_pdk_source_compiles_once_per_process");
    thread_compiles::take();
    let first = prepared_pdk_veriloga_runtimes(&sealed).expect("the source compiles");
    let second = prepared_pdk_veriloga_runtimes(&sealed).expect("the source is read");
    assert_eq!(thread_compiles::take(), ["pdk_resistor"]);
    assert_eq!(first, second);
}

/// A frame that names a project's sources warms them: they compile on
/// another thread, and the preparation after that, in the frame that
/// validates a manual deck or queues a campaign, compiles nothing.
#[test]
fn a_warmed_pdk_source_compiles_off_the_calling_thread() {
    let sealed = sealed_pdk("a_warmed_pdk_source_compiles_off_the_calling_thread");
    thread_compiles::take();
    super::warm(&sealed, ProcessCorner::TT);
    let warmed = prepared_pdk_veriloga_runtimes(&sealed).expect("the warmed source is read");
    assert!(thread_compiles::take().is_empty());
    assert_eq!(warmed.len(), 1);
}

/// The bytes are checked on every preparation, so a compiled runtime is
/// never handed out for bytes other than those it was compiled from.
#[test]
fn a_compiled_pdk_source_still_refuses_changed_bytes() {
    let sealed = sealed_pdk("a_compiled_pdk_source_still_refuses_changed_bytes");
    prepared_pdk_veriloga_runtimes(&sealed).expect("the source compiles");
    let (package, archive_digest, artifacts, bindings) = sealed
        .pdk_veriloga_authority()
        .expect("the PDK's Verilog-A");
    let mut changed = artifacts.to_vec();
    changed[0].source.push_str("\n// changed\n");
    let error = super::pdk_runtime(package, archive_digest, &changed, &bindings[0])
        .expect_err("changed bytes are refused");
    assert!(
        error.to_string().contains("no longer matches digest"),
        "{error}"
    );
}

/// A preparation that must not wait stops at a source another thread is
/// compiling, and `compiling` holds on its thread until that compile ends.
/// The preparation after that compiles the source as before.
#[test]
fn a_preparation_that_must_not_wait_stops_at_a_source_still_compiling() {
    let sealed = sealed_pdk("a_preparation_that_must_not_wait_stops_at_a_source_still_compiling");
    let (package, archive_digest, _, bindings) = sealed
        .pdk_veriloga_authority()
        .expect("the PDK's Verilog-A");
    let compile = super::native::hold_compile(super::native::Key::pdk(
        package,
        archive_digest,
        &bindings[0],
    ));
    thread_compiles::take();
    assert!(super::unless_compiling(|| prepared_pdk_veriloga_runtimes(&sealed)).is_none());
    assert!(super::compiling());
    assert!(thread_compiles::take().is_empty());

    drop(compile);
    assert!(!super::compiling());
    let prepared = super::unless_compiling(|| prepared_pdk_veriloga_runtimes(&sealed))
        .expect("nothing is compiling");
    assert!(prepared.is_ok());
    assert_eq!(thread_compiles::take(), ["pdk_resistor"]);
}
