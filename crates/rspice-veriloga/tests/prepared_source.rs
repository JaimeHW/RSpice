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
fn runtime_preparation_retains_source_limits_and_original_compiler_locations() {
    use rspice_veriloga::ProviderCompileError;
    use rspice_veriloga::preprocessor::SourceResource;

    let files = Sources::new();
    let root = files.write("root.va", "`include \"child.va\"\n");
    let child = files.write(
        "child.va",
        &format!("// header\nmodule broken({};\n", "port".repeat(1024)),
    );
    let compiler = VerilogACompiler::default();
    for (limits, resource, limit) in [
        (
            SourceProviderLimits {
                max_include_depth: 1,
                ..SourceProviderLimits::UNBOUNDED
            },
            SourceResource::IncludeDepth,
            1,
        ),
        (
            SourceProviderLimits {
                max_total_source_bytes: 256,
                ..SourceProviderLimits::UNBOUNDED
            },
            SourceResource::TotalSourceBytes,
            256,
        ),
        (
            SourceProviderLimits {
                max_expanded_bytes: 256,
                ..SourceProviderLimits::UNBOUNDED
            },
            SourceResource::ExpandedBytes,
            256,
        ),
    ] {
        let error = compiler
            .prepare_file_runtime_source_with_diagnostics(&root, limits, &NoPipelineControl)
            .unwrap_err();
        let ProviderCompileError::Source(error) = error else {
            panic!("{error:?}")
        };
        let failure = error.resource_limit.unwrap();
        assert_eq!(failure.resource, resource);
        assert_eq!(failure.limit, limit);
        assert!(failure.requested > limit);
    }
    let error = compiler
        .prepare_file_runtime_source_with_diagnostics(
            &root,
            SourceProviderLimits::UNBOUNDED,
            &NoPipelineControl,
        )
        .unwrap_err();
    let ProviderCompileError::Compile { diagnostics, .. } = error else {
        panic!("{error:?}")
    };
    assert!(!diagnostics.is_empty());
    assert_eq!(diagnostics[0].line, Some(2));
    assert_eq!(
        PathBuf::from(diagnostics[0].path.as_ref().unwrap()),
        child.canonicalize().unwrap()
    );

    let missing = files.0.join("missing.va");
    let error = compiler
        .prepare_file_runtime_source_with_diagnostics(
            &missing,
            SourceProviderLimits::UNBOUNDED,
            &NoPipelineControl,
        )
        .unwrap_err();
    let ProviderCompileError::Source(error) = error else {
        panic!("{error:?}")
    };
    assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
    assert_eq!(error.file, Some(missing));
}

#[test]
fn runtime_preparation_cancels_during_source_reads_in_both_apis() {
    struct CancelDuringRead(std::sync::atomic::AtomicUsize);
    impl rspice_veriloga::PipelineControl for CancelDuringRead {
        fn is_cancelled(&self) -> bool {
            self.0.fetch_add(1, Ordering::SeqCst) >= 4
        }
    }
    let files = Sources::new();
    let root = files.write(
        "large.va",
        &format!("// {}\nmodule m; endmodule\n", "x".repeat(65536)),
    );
    let compiler = VerilogACompiler::default();
    let error = compiler
        .prepare_file_runtime_source_with_diagnostics(
            &root,
            SourceProviderLimits::UNBOUNDED,
            &CancelDuringRead(std::sync::atomic::AtomicUsize::new(0)),
        )
        .unwrap_err();
    assert!(
        matches!(error, rspice_veriloga::ProviderCompileError::Source(error) if error.cancelled)
    );
    let error = compiler
        .prepare_file_runtime_source_with_limits_and_control(
            &root,
            SourceProviderLimits::UNBOUNDED,
            &CancelDuringRead(std::sync::atomic::AtomicUsize::new(0)),
        )
        .unwrap_err();
    assert!(
        matches!(error, rspice_veriloga::CompileError::Cancelled(cancelled)
        if cancelled.phase == rspice_veriloga::PipelinePhase::Preprocess)
    );
}

#[test]
fn file_and_provider_compilation_retain_warnings_in_original_includes() {
    use rspice_veriloga::CompileDiagnosticSeverity;
    use rspice_veriloga::preprocessor::FileSystemSourceProvider;

    let files = Sources::new();
    let root = files.write("root.va", "`include \"child.va\"\n");
    let source = "module chatty(p,n);\ninout p,n; electrical p,n;\nanalog begin\n $display(\"hello\");\n I(p,n) <+ V(p,n);\nend\nendmodule\n";
    let child = files.write("child.va", source);
    let compiler = VerilogACompiler::default();
    let from_file = compiler.compile_file_with_metadata(&root).unwrap();
    let from_provider = compiler
        .compile_provider_module_with_metadata_and_control(
            &FileSystemSourceProvider,
            &root,
            None,
            &NoPipelineControl,
        )
        .unwrap();
    assert_eq!(from_file.diagnostics, from_provider.diagnostics);
    let prepared = compiler.prepare_file_runtime_source(&root).unwrap();
    assert_eq!(prepared.diagnostics(), from_file.diagnostics);
    let runtime = prepared.compile_runtime(None).unwrap();
    assert_eq!(runtime.diagnostics, from_file.diagnostics);
    assert_eq!(from_file.diagnostics.len(), 1);
    let warning = &from_file.diagnostics[0];
    assert_eq!(warning.severity, CompileDiagnosticSeverity::Warning);
    assert_eq!(warning.code, "VA-SEM-NO-EFFECT-SYSTEM-TASK");
    assert_eq!(
        warning.path.as_deref(),
        Some(child.canonicalize().unwrap().to_str().unwrap())
    );
    assert_eq!(warning.line, Some(4));
    assert_eq!(warning.column, Some(2));
    assert!(source[warning.byte_start.unwrap()..warning.byte_end.unwrap()].contains("$display"));
}

#[test]
fn filesystem_loader_errors_retain_paths_kinds_and_error_sources() {
    use rspice_veriloga::preprocessor::{
        BoundedFileSystemSourceProvider, FileSystemSourceProvider,
    };
    use rspice_veriloga::{Preprocessor, SourceProvider};
    use std::error::Error as _;
    use std::io::ErrorKind;

    let mut files = Sources::new();
    files.0 = files.0.canonicalize().unwrap();
    let invalid = files.0.join("invalid.va");
    std::fs::write(&invalid, [0xff, 0xfe]).unwrap();
    let missing = files.0.join("missing.va");
    let bounded = BoundedFileSystemSourceProvider::new(
        SourceProviderLimits::UNBOUNDED,
        usize::MAX,
        usize::MAX,
        &NoPipelineControl,
    );
    for provider in [&FileSystemSourceProvider as &dyn SourceProvider, &bounded] {
        for (path, kind) in [
            (&invalid, ErrorKind::InvalidData),
            (&missing, ErrorKind::NotFound),
        ] {
            let error = provider.load_root(path).unwrap_err();
            assert_eq!(error.file.as_ref(), Some(path));
            assert_eq!(error.io_error.as_ref().unwrap().kind(), kind);
            let cloned = error.clone();
            assert_eq!(cloned.io_error.as_ref().unwrap().kind(), kind);
            assert!(cloned.source().unwrap().is::<std::io::Error>());
            assert!(!error.cancelled);
            assert!(error.resource_limit.is_none());
        }
        let root = files.write("root.va", "`include \"invalid.va\"\n");
        let error = Preprocessor::new()
            .preprocess_provider_root(provider, &root)
            .unwrap_err();
        assert_eq!(error.file.as_ref(), Some(&invalid));
        assert_eq!(
            error.io_error.as_ref().unwrap().kind(),
            ErrorKind::InvalidData
        );
    }
}

#[cfg(unix)]
#[test]
fn dangling_includes_cannot_fall_through_to_another_file_or_builtin() {
    use rspice_veriloga::preprocessor::{
        BoundedFileSystemSourceProvider, FileSystemSourceProvider,
    };
    use rspice_veriloga::{Preprocessor, SourceProvider};

    let mut files = Sources::new();
    files.0 = files.0.canonicalize().unwrap();
    let root = files.write("root.va", "`include \"disciplines.vams\"\n");
    let header = files.0.join("disciplines.vams");
    std::os::unix::fs::symlink(files.0.join("absent.vams"), &header).unwrap();
    let fallback = files.0.join("fallback");
    std::fs::create_dir(&fallback).unwrap();
    std::fs::write(fallback.join("disciplines.vams"), "// different model\n").unwrap();
    let bounded = BoundedFileSystemSourceProvider::new(
        SourceProviderLimits::UNBOUNDED,
        usize::MAX,
        usize::MAX,
        &NoPipelineControl,
    );
    for provider in [&FileSystemSourceProvider as &dyn SourceProvider, &bounded] {
        for paths in [vec![], vec![fallback.clone()]] {
            let mut preprocessor = Preprocessor::new();
            for path in paths {
                preprocessor.add_include_path(path);
            }
            let error = preprocessor
                .preprocess_provider_root(provider, &root)
                .unwrap_err();
            assert_eq!(error.file.as_ref(), Some(&header));
            assert_eq!(
                error.io_error.as_ref().unwrap().kind(),
                std::io::ErrorKind::NotFound
            );
        }
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
