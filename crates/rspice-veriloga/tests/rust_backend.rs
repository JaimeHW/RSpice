use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::rust_backend::{
    GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames, RustTranspiler,
    RustTranspileOptions, discover_veriloga_sources, write_generated_device,
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

#[test]
fn rust_backend_generates_direct_rust_for_algebraic_current() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(tiny_resistor_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile simple resistor");

    assert_eq!(generated.module_name, "tiny_res");
    assert_eq!(generated.public_model_name, "tiny_res");
    assert_eq!(generated.files.len(), 3);
    assert!(
        generated
            .files
            .iter()
            .any(|file| file.relative_path == "mod.rs")
    );
    assert!(
        generated
            .files
            .iter()
            .any(|file| file.relative_path == "state.rs")
    );
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("ctx.node_voltage(self.nodes[0])"));
    assert!(stamp.contains("ctx.node_voltage(self.nodes[1])"));
    assert!(stamp.contains("self.params.r"));
    assert!(stamp.contains("eq0_value"));
    assert!(stamp.contains("eq0_d_n0"));
    assert!(stamp.contains("eq0_d_n1"));
    assert!(stamp.contains("stamper.stamp_current"));
    assert!(!stamp.contains("Bytecode"));
    assert!(!stamp.contains("Interpreter"));
    assert!(!stamp.contains("HashMap"));
}

#[test]
fn generated_algebraic_current_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(tiny_resistor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
        .transpile(&artifact)
        .expect("transpile simple resistor");
    let temp = temp_dir("rspice-rust-backend-compile");

    write_generated_device(&temp, &generated).expect("write generated device");
    let lib = temp.join("compile_smoke.rs");
    std::fs::write(
        &lib,
        format!(
            r#"
pub mod runtime {{
    pub struct GeneratedEvalContext<'a> {{
        voltages: &'a [f64],
    }}

    impl<'a> GeneratedEvalContext<'a> {{
        pub fn new(voltages: &'a [f64]) -> Self {{
            Self {{ voltages }}
        }}

        pub fn node_voltage(&self, node: usize) -> f64 {{
            self.voltages[node]
        }}
    }}

    pub struct GeneratedStamper<'a> {{
        pub touched: &'a mut f64,
    }}

    impl<'a> GeneratedStamper<'a> {{
        pub fn stamp_current(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            value: f64,
            derivatives: &[(usize, f64)],
        ) {{
            *self.touched += value + derivatives.iter().map(|(_, value)| *value).sum::<f64>();
        }}
    }}
}}

#[path = "{}"]
pub mod generated_device;
"#,
            temp.join(&generated.folder_name)
                .join("mod.rs")
                .display()
                .to_string()
                .replace('\\', "\\\\")
        ),
    )
    .expect("write compile smoke file");

    let output = std::process::Command::new(std::env::var("RUSTC").unwrap_or("rustc".to_string()))
        .arg("--edition=2024")
        .arg("--crate-type=lib")
        .arg(&lib)
        .arg("-o")
        .arg(temp.join("compile_smoke.rlib"))
        .output()
        .expect("run rustc");

    assert!(
        output.status.success(),
        "generated rust did not compile\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(temp);
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

fn tiny_resistor_source() -> &'static str {
    r#"
module tiny_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#
}
