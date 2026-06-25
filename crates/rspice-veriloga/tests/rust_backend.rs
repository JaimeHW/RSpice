use rspice_veriloga::rust_backend::{
    GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames, RustTranspiler,
    discover_veriloga_sources, write_generated_device,
};

#[test]
fn rust_backend_public_api_exists() {
    let _ = RustTranspiler::default();
    let diagnostic = RustBackendError::unsupported("fixture.va", "tiny_res", "arrays");

    assert!(diagnostic.to_string().contains("fixture.va"));
    assert!(diagnostic.to_string().contains("tiny_res"));
    assert!(diagnostic.to_string().contains("arrays"));
}

#[test]
fn generated_device_records_multiple_files() {
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__00000000".to_string(),
        files: Vec::new(),
        source_digest: "0000000000000000".to_string(),
    };

    assert_eq!(device.module_name, "tiny_res");
    assert!(device.files.is_empty());
}

#[test]
fn rust_backend_mangles_names_deterministically() {
    let names = RustDeviceNames::new("psp103.va", "PSP103_Module", "abcdef0123456789");

    assert_eq!(names.public_model_name, "PSP103_Module");
    assert_eq!(names.rust_module, "psp103_module__abcdef01");
    assert_eq!(names.folder, "psp103__psp103_module__abcdef01");
}

#[test]
fn generated_device_writer_splits_files_under_device_folder() {
    let temp = temp_dir("rspice-rust-backend-files");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: "pub mod eval;\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "eval.rs".to_string(),
                contents: "pub fn marker() -> f64 { 1.0 }\n".to_string(),
            },
        ],
    };

    write_generated_device(&temp, &device).expect("write generated device");

    assert!(
        temp.join("tiny_res__tiny_res__abc12345")
            .join("mod.rs")
            .exists()
    );
    assert!(
        temp.join("tiny_res__tiny_res__abc12345")
            .join("eval.rs")
            .exists()
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_writer_rejects_paths_outside_device_folder() {
    let temp = temp_dir("rspice-rust-backend-paths");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![GeneratedRustFile {
            relative_path: "../escape.rs".to_string(),
            contents: "pub fn marker() {}\n".to_string(),
        }],
    };

    let err = write_generated_device(&temp, &device).expect_err("reject unsafe path");
    assert!(err.to_string().contains("unsafe generated relative path"));

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn discovery_skips_include_only_files_and_sorts_modules() {
    let dir = temp_dir("rspice-va-discovery");
    std::fs::write(dir.join("defs.include"), "`define GAIN 1.0\n").expect("write include");
    std::fs::write(dir.join("disciplines.vams"), "nature Voltage; endnature\n")
        .expect("write vams");
    std::fs::write(
        dir.join("b.va"),
        "module beta(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    )
    .expect("write beta");
    std::fs::write(
        dir.join("a.va"),
        "module alpha(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    )
    .expect("write alpha");

    let found = discover_veriloga_sources(&dir).expect("discover sources");
    let names: Vec<_> = found
        .iter()
        .flat_map(|source| source.modules.iter().cloned())
        .collect();

    assert_eq!(names, vec!["alpha".to_string(), "beta".to_string()]);
    assert_eq!(found.len(), 2);

    let _ = std::fs::remove_dir_all(dir);
}

fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "{prefix}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}
