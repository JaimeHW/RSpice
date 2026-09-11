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
