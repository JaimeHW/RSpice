use rspice_veriloga::{
    CompileError, CompilerOptions, NoPipelineControl, PipelineControl, VerilogACompiler,
    VirtualCompileLimits, VirtualSourceBundle, VirtualSourceError,
};

const DEVICE: &str = r#"
module source(q);
 output q; logic q; reg q;
 initial q=1;
endmodule
module top(p);
 parameter integer N=1;
 inout p; electrical p;
 source first(p);
 generate if (N==2) begin : extra
  source second(p);
 end endgenerate
endmodule
"#;
const LIBRARY: &str = r#"
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1;
 analog I(a)<+(V(a)-(d ? level : 0.0))/1000;
endmodule
connectrules low; connect drive split #(.level(1.0)); endconnectrules
connectrules high; connect drive split #(.level(3.0)); endconnectrules
"#;

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
}

fn bundle(path: &str, source: &str) -> VirtualSourceBundle {
    VirtualSourceBundle::from_sources(path, [(path, source)]).unwrap()
}

#[test]
fn selected_virtual_configuration_survives_transport_and_root_replay() {
    let compiler = compiler();
    let device = compiler
        .prepare_virtual_runtime_source(
            &bundle("device.vams", DEVICE),
            VirtualCompileLimits::default(),
        )
        .unwrap();
    let library = compiler
        .prepare_virtual_runtime_source(
            &bundle("rules.vams", LIBRARY),
            VirtualCompileLimits::default(),
        )
        .unwrap();
    let low = library.connection_configuration("low").unwrap();
    let high = library.connection_configuration("high").unwrap();
    let mut first = device
        .compile_runtime_with_connections("top", &low, &NoPipelineControl)
        .unwrap();
    let second = device
        .compile_runtime_with_connections("top", &high, &NoPipelineControl)
        .unwrap();
    assert_eq!(first.dependency_closure.len(), 1);
    assert_eq!(first.dependency_closure[0].source, DEVICE);
    assert_eq!(first.source_bundle_identity, second.source_bundle_identity);
    assert_eq!(
        first.compiler_contract_identity,
        second.compiler_contract_identity
    );
    assert_ne!(
        first.runtime_contract_identity,
        second.runtime_contract_identity
    );
    assert_eq!(
        first.runtime.canonical_ir.connections.configuration(),
        Some(&low)
    );
    first.runtime = serde_json::from_slice(&serde_json::to_vec(&first.runtime).unwrap()).unwrap();
    first.validate_integrity().unwrap();

    let assigned = compiler
        .specialize_mixed_runtime(
            &first.runtime.canonical_ir,
            &[("N", 2.0)],
            &NoPipelineControl,
        )
        .unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap();
    assert_eq!(
        replay.runtime_source_identity(None).unwrap(),
        assigned.canonical_ir.runtime_source_identity()
    );
    let rebuilt = replay.compile_runtime(None).unwrap();
    assert_eq!(
        rebuilt.canonical_ir.connection_identity,
        assigned.canonical_ir.connection_identity
    );
    assert_eq!(
        rebuilt.canonical_ir.source_specialization,
        assigned.canonical_ir.source_specialization
    );
    assert_eq!(
        rebuilt
            .canonical_ir
            .hir
            .parameters
            .iter()
            .find(|p| p.name == "N")
            .unwrap()
            .default,
        Some(2.0)
    );
    let rebound = replay
        .compile_runtime_with_connections(Some("top"), &high, &NoPipelineControl)
        .unwrap();
    assert_eq!(
        rebound.canonical_ir.connections.configuration(),
        Some(&high)
    );
    assert_eq!(
        rebound.canonical_ir.source_specialization,
        assigned.canonical_ir.source_specialization
    );
    for error in [
        replay.runtime_source_identity(Some("source")).unwrap_err(),
        replay.compile_runtime(Some("source")).unwrap_err(),
        replay
            .compile_runtime_with_connections(Some("source"), &high, &NoPipelineControl)
            .unwrap_err(),
    ] {
        assert!(
            error.to_string().contains("belongs to module 'top'"),
            "{error}"
        );
    }
    // Swapping in another valid report must not preserve the old envelope.
    first.runtime = second.runtime;
    assert!(
        first
            .validate_integrity()
            .unwrap_err()
            .to_string()
            .contains("runtime contract identity mismatch")
    );
}

#[test]
fn configured_virtual_inputs_obey_expansion_limits_and_cancellation() {
    struct Cancelled;
    impl PipelineControl for Cancelled {
        fn is_cancelled(&self) -> bool {
            true
        }
    }
    let compiler = compiler();
    let library = compiler
        .prepare_virtual_runtime_source(
            &bundle("rules.vams", LIBRARY),
            VirtualCompileLimits::default(),
        )
        .unwrap();
    let low = library.connection_configuration("low").unwrap();
    let device = compiler
        .prepare_virtual_runtime_source(
            &bundle("device.vams", DEVICE),
            VirtualCompileLimits {
                max_expanded_bytes: DEVICE.len() + 64,
                ..Default::default()
            },
        )
        .unwrap();
    let failure = device
        .compile_runtime_with_connections("top", &low, &NoPipelineControl)
        .unwrap_err();
    assert!(matches!(
        failure.error,
        CompileError::VirtualSource(VirtualSourceError::ExpandedSourceTooLarge { .. })
    ));
    let failure = device
        .compile_runtime_with_connections("top", &low, &Cancelled)
        .unwrap_err();
    assert!(matches!(failure.error, CompileError::Cancelled(_)));

    // Selecting rules already in the device closure must count its bytes once.
    let combined = format!("{DEVICE}\n{LIBRARY}");
    let same = compiler
        .prepare_virtual_runtime_source(
            &bundle("combined.vams", &combined),
            VirtualCompileLimits {
                max_expanded_bytes: combined.len() + 64,
                ..Default::default()
            },
        )
        .unwrap();
    let low = same.connection_configuration("low").unwrap();
    same.compile_runtime_with_connections("top", &low, &NoPipelineControl)
        .unwrap()
        .validate_integrity()
        .unwrap();
}

#[test]
fn configured_virtual_conflicts_identify_the_external_library_document() {
    let compiler = compiler();
    let device = compiler
        .prepare_virtual_runtime_source(
            &bundle("top.vams", DEVICE),
            VirtualCompileLimits::default(),
        )
        .unwrap();
    let text =
        format!("module top(p); inout p; electrical p; analog I(p)<+V(p); endmodule\n{LIBRARY}");
    // Both separately sealed roots can have the same logical name.
    let library = compiler
        .prepare_virtual_runtime_source(&bundle("top.vams", &text), VirtualCompileLimits::default())
        .unwrap();
    let low = library.connection_configuration("low").unwrap();
    let failure = device
        .compile_runtime_with_connections("top", &low, &NoPipelineControl)
        .unwrap_err();
    assert!(failure.to_string().contains("ordinary module 'top'"));
    let [diagnostic] = failure.diagnostics.as_slice() else {
        panic!("{failure:?}")
    };
    assert_eq!(
        diagnostic.logical_path.as_deref(),
        Some("top.vams (preprocessed)")
    );
    let source = diagnostic.source.as_deref().unwrap();
    assert_eq!(source, low.library().preprocessed_source());
    assert!(
        source[diagnostic.byte_start.unwrap()..diagnostic.byte_end.unwrap()].contains("module top")
    );
    assert_eq!(diagnostic.line, Some(1));
}

#[test]
fn authored_connect_root_has_the_same_identity_after_artifact_replay() {
    let compiler = compiler();
    let library = compiler
        .prepare_virtual_runtime_source(
            &bundle("rules.vams", LIBRARY),
            VirtualCompileLimits::default(),
        )
        .unwrap();
    let low = library.connection_configuration("low").unwrap();
    let runtime = compiler
        .compile_connect_runtime_with_configuration(
            "rules.vams",
            low.library().preprocessed_source(),
            "drive",
            &[("level", 2.0)],
            &low,
            &NoPipelineControl,
        )
        .unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&runtime.canonical_ir, &NoPipelineControl)
        .unwrap();
    assert_eq!(
        replay.runtime_source_identity(None).unwrap(),
        runtime.canonical_ir.runtime_source_identity()
    );
    assert_eq!(
        replay.runtime_source_identity(Some("drive")).unwrap(),
        runtime.canonical_ir.runtime_source_identity()
    );
    let rebuilt = replay.compile_runtime(None).unwrap();
    assert_eq!(
        rebuilt.canonical_ir.connection_identity,
        runtime.canonical_ir.connection_identity
    );
    assert_eq!(
        rebuilt.canonical_ir.source_specialization,
        runtime.canonical_ir.source_specialization
    );
    assert_eq!(
        rebuilt
            .canonical_ir
            .hir
            .parameters
            .iter()
            .find(|p| p.name == "level")
            .unwrap()
            .default,
        Some(2.0)
    );
}
