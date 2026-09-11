use rspice_veriloga::{CompilerOptions, NoPipelineControl, SourceProviderLimits, VerilogACompiler};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NONCE: AtomicU64 = AtomicU64::new(0);

struct Sources(PathBuf);
impl Sources {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "rspice-prepared-source-{}-{}",
            std::process::id(),
            NONCE.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, source: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, source).unwrap();
        path
    }
}
impl Drop for Sources {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn multiple_modules_compile_from_one_frozen_source_and_dependency_identity() {
    let files = Sources::new();
    let header = files.write("gain.vh", "`define GAIN 2\n");
    let root = files.write("models.va", "`include \"gain.vh\"\nmodule first(p,n); inout p,n; electrical p,n; parameter real gain=`GAIN; analog I(p,n)<+gain*V(p,n); endmodule\nmodule second(p,n); inout p,n; electrical p,n; parameter real gain=`GAIN; analog I(p,n)<+gain*V(p,n); endmodule\n");
    let compiler = VerilogACompiler::default();
    let prepared = compiler.prepare_file_runtime_source(&root).unwrap();
    assert_eq!(
        prepared.module_names().collect::<Vec<_>>(),
        ["first", "second"]
    );
    let original_header = prepared
        .dependencies()
        .iter()
        .find(|dependency| dependency.path.ends_with("gain.vh"))
        .unwrap()
        .clone();
    assert_eq!(
        original_header.content_identity,
        *blake3::hash(b"`define GAIN 2\n").as_bytes()
    );
    // A prepared module must never re-open an include or attach its new bytes
    // to the behavior compiled from the old contents.
    std::fs::remove_file(&header).unwrap();
    for module in ["first", "second"] {
        let built = prepared.compile_runtime(Some(module)).unwrap();
        assert_eq!(built.canonical_ir.hir.parameters[0].default, Some(2.0));
        assert!(built.source_dependencies.contains(&original_header));
    }
    files.write("gain.vh", "`define GAIN 7\n");
    let changed = compiler.prepare_file_runtime_source(&root).unwrap();
    assert_eq!(
        changed
            .compile_runtime(Some("second"))
            .unwrap()
            .canonical_ir
            .hir
            .parameters[0]
            .default,
        Some(7.0)
    );
    assert_ne!(
        changed.connect_specification().source_identity,
        prepared.connect_specification().source_identity
    );
    assert!(
        compiler
            .prepare_file_runtime_source_with_limits_and_control(
                &root,
                SourceProviderLimits {
                    max_total_source_bytes: 1,
                    ..SourceProviderLimits::UNBOUNDED
                },
                &NoPipelineControl,
            )
            .is_err()
    );
}

#[test]
fn standalone_connect_library_is_prepared_without_a_device_module() {
    let files = Sources::new();
    let root = files.write(
        "connections.vams",
        &rspice_veriloga::connect::library::builtin_connect_library_source(),
    );
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let prepared = compiler.prepare_file_runtime_source(&root).unwrap();
    assert_eq!(prepared.module_names().count(), 0);
    let specification = prepared.connect_specification();
    assert!(!specification.declares_module);
    assert_eq!(specification.rules.insertions().len(), 3);
    assert!(prepared.compile_runtime(None).is_err());
}

#[test]
fn named_connection_configurations_preserve_alternatives_and_reject_duplicate_declarations() {
    let files = Sources::new();
    let mut source = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, body)| *body)
        .collect::<String>();
    source.push_str("\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\nconnectrules High; connect d2a #(.vsup(5.0)); endconnectrules\nconnectrules Empty; endconnectrules\n");
    let root = files.write("alternatives.vams", &source);
    let compiler = VerilogACompiler::default();
    let specification = compiler
        .prepare_file_runtime_source(&root)
        .unwrap()
        .connect_specification();
    assert_eq!(
        specification
            .rules
            .blocks()
            .iter()
            .map(|block| block.name.as_str())
            .collect::<Vec<_>>(),
        ["Low", "High", "Empty"]
    );
    for (name, supply) in [("Low", 1.0), ("High", 5.0)] {
        let selected = specification.rules.select_block(name).unwrap();
        assert_eq!(selected.blocks().len(), 1);
        assert_eq!(selected.insertions().len(), 1);
        let rule = selected
            .select(
                "electrical",
                "logic",
                rspice_veriloga::connect::ConnectDirection::DiscreteToAnalog,
                &specification.disciplines,
            )
            .unwrap();
        assert_eq!(rule.numeric_parameters().unwrap()[0].1, supply);
    }
    assert!(
        specification
            .rules
            .select_block("Empty")
            .unwrap()
            .insertions()
            .is_empty()
    );
    assert!(specification.rules.select_block("low").is_err());
    for (duplicate, expected) in [
        ("connectrules Low; endconnectrules", "connectrules 'Low'"),
        (
            rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES[0].1,
            "connectmodule 'a2d'",
        ),
    ] {
        files.write("alternatives.vams", &format!("{source}\n{duplicate}\n"));
        let error = compiler
            .prepare_file_runtime_source(&root)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(expected) && error.contains("more than once"),
            "{error}"
        );
    }
}

#[derive(Default)]
struct PreparationObserver {
    cancelled: std::sync::atomic::AtomicBool,
    phases: std::sync::Mutex<Vec<rspice_veriloga::PipelinePhase>>,
    cancel_at: Option<rspice_veriloga::PipelinePhase>,
}
impl rspice_veriloga::PipelineControl for PreparationObserver {
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
    fn phase_completed(
        &self,
        timing: rspice_veriloga::PhaseTiming,
        _: &rspice_veriloga::PipelineMetrics,
    ) {
        self.phases.lock().unwrap().push(timing.phase);
        if self.cancel_at == Some(timing.phase) {
            self.cancelled.store(true, Ordering::SeqCst);
        }
    }
}

#[test]
fn prepared_virtual_source_shares_front_end_and_transports_standalone_connections() {
    use rspice_veriloga::{
        ConnectionLibraryArtifact, PipelinePhase, RuntimeQualificationOptions,
        VirtualCompileLimits, VirtualSourceBundle, VirtualSourceFile,
    };
    let files = Sources::new();
    let rules = rspice_veriloga::connect::library::builtin_connect_library_source();
    let root = files.write("rules.vams", &rules);
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let file_artifact = compiler
        .prepare_file_runtime_source(&root)
        .unwrap()
        .connection_artifact()
        .unwrap();
    let connection_bundle =
        VirtualSourceBundle::new("rules.vams", [VirtualSourceFile::new("rules.vams", &rules)])
            .unwrap();
    let standalone = compiler
        .prepare_virtual_runtime_source(&connection_bundle, VirtualCompileLimits::default())
        .unwrap();
    assert!(standalone.is_connect_library());
    assert_eq!(standalone.module_names().count(), 0);
    let artifact = standalone.connection_artifact().unwrap();
    assert_eq!(artifact, file_artifact);
    let packet = serde_json::to_value(&artifact).unwrap();
    let transported: ConnectionLibraryArtifact = serde_json::from_value(packet.clone()).unwrap();
    assert_eq!(
        transported
            .connect_specification()
            .unwrap()
            .rules
            .insertions()
            .len(),
        3
    );
    for field in ["source_package", "preprocessed_source", "schema_version"] {
        let mut altered = packet.clone();
        altered[field] = if field == "schema_version" {
            serde_json::json!(999)
        } else {
            serde_json::json!("altered")
        };
        let altered: ConnectionLibraryArtifact = serde_json::from_value(altered).unwrap();
        assert!(altered.validate_integrity().is_err(), "{field}");
    }
    let mut missing = packet;
    missing.as_object_mut().unwrap().remove("identity");
    assert!(serde_json::from_value::<ConnectionLibraryArtifact>(missing).is_err());

    let modules = "`include \"rules.vams\"\nmodule first(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule second(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n";
    let bundle = VirtualSourceBundle::new(
        "models.vams",
        [
            VirtualSourceFile::new("models.vams", modules),
            VirtualSourceFile::new("rules.vams", rules),
        ],
    )
    .unwrap();
    let observer = PreparationObserver::default();
    let prepared = compiler
        .prepare_virtual_runtime_source_with_control(
            &bundle,
            VirtualCompileLimits::default(),
            &observer,
        )
        .unwrap();
    assert_eq!(
        prepared.module_names().collect::<Vec<_>>(),
        ["first", "second"]
    );
    for name in ["first", "second"] {
        let compiled = prepared
            .compile_runtime_with_qualifications_and_control(
                name,
                RuntimeQualificationOptions::NONE,
                &observer,
            )
            .unwrap();
        compiled.validate_integrity().unwrap();
        assert_eq!(
            compiled
                .runtime
                .canonical_ir
                .metadata
                .source_identity
                .as_str(),
            prepared.connect_specification().source_identity
        );
        assert_eq!(compiled.dependency_closure, prepared.dependency_closure());
        assert_eq!(compiled.include_graph, prepared.include_graph());
    }
    // Compiler work is actually shared, rather than merely relabelled in metrics.
    let observed = observer.phases.lock().unwrap();
    for phase in [
        PipelinePhase::Preprocess,
        PipelinePhase::Lex,
        PipelinePhase::Parse,
        PipelinePhase::Semantic,
    ] {
        assert_eq!(
            observed.iter().filter(|&&seen| seen == phase).count(),
            1,
            "{phase}"
        );
    }
    drop(observed);
    observer.cancelled.store(true, Ordering::SeqCst);
    let error = prepared
        .compile_runtime_with_qualifications_and_control(
            "first",
            RuntimeQualificationOptions::NONE,
            &observer,
        )
        .unwrap_err();
    assert!(matches!(
        error.error,
        rspice_veriloga::CompileError::Cancelled(_)
    ));
    let error = compiler
        .prepare_virtual_runtime_source_with_control(
            &bundle,
            VirtualCompileLimits::default(),
            &observer,
        )
        .unwrap_err();
    assert!(matches!(
        error.error,
        rspice_veriloga::CompileError::Cancelled(_)
    ));
    let late_cancel = PreparationObserver {
        cancel_at: Some(PipelinePhase::Semantic),
        ..Default::default()
    };
    let error = compiler
        .prepare_virtual_runtime_source_with_control(
            &bundle,
            VirtualCompileLimits::default(),
            &late_cancel,
        )
        .unwrap_err();
    assert!(matches!(
        error.error,
        rspice_veriloga::CompileError::Cancelled(_)
    ));
}
