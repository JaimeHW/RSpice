use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::rust_backend::{
    GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames,
    RustTranspileOptions, RustTranspiler, cleanup_stale_generated_device_folders,
    discover_veriloga_sources, write_generated_device, write_text_file_if_changed,
};
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

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
fn generated_device_writer_marks_device_folder_for_safe_cleanup() {
    let temp = temp_dir("rspice-rust-backend-marker");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: "pub fn marker() {}\n".to_string(),
        }],
    };

    write_generated_device(&temp, &device).expect("write generated device");

    assert!(
        temp.join("tiny_res__tiny_res__abc12345")
            .join(".rspice-veriloga-generated")
            .is_file(),
        "generated device folders need an explicit marker before source-tree cleanup may remove them"
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
fn generated_device_writer_rejects_empty_relative_path() {
    let temp = temp_dir("rspice-rust-backend-empty-path");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![GeneratedRustFile {
            relative_path: String::new(),
            contents: "pub fn marker() {}\n".to_string(),
        }],
    };

    let err = write_generated_device(&temp, &device).expect_err("reject empty path");
    assert!(err.to_string().contains("unsafe generated relative path"));

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_writer_rejects_unsafe_device_folder_name() {
    let temp = temp_dir("rspice-rust-backend-folder-paths");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "../escape".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: "pub fn marker() {}\n".to_string(),
        }],
    };

    let err = write_generated_device(&temp, &device).expect_err("reject unsafe folder");
    assert!(err.to_string().contains("unsafe generated device folder"));
    assert!(!temp.join("escape").exists());

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_writer_rejects_duplicate_relative_paths() {
    let temp = temp_dir("rspice-rust-backend-duplicate-paths");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: "pub fn first() {}\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: "pub fn second() {}\n".to_string(),
            },
        ],
    };

    let err = write_generated_device(&temp, &device).expect_err("reject duplicate file path");
    assert!(
        err.to_string()
            .contains("duplicate generated relative path")
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_writer_rejects_case_colliding_relative_paths() {
    let temp = temp_dir("rspice-rust-backend-case-paths");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: "pub fn first() {}\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "STAMP.rs".to_string(),
                contents: "pub fn second() {}\n".to_string(),
            },
        ],
    };

    let err = write_generated_device(&temp, &device).expect_err("reject case-colliding paths");
    assert!(
        err.to_string()
            .contains("case-colliding generated relative path")
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_writer_preserves_unchanged_files() {
    let temp = temp_dir("rspice-rust-backend-stable-write");
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: "pub const MARKER: usize = 1;\n".to_string(),
        }],
    };

    write_generated_device(&temp, &device).expect("initial write");
    let path = temp.join("tiny_res__tiny_res__abc12345").join("state.rs");
    let first_modified = std::fs::metadata(&path)
        .expect("metadata after first write")
        .modified()
        .expect("mtime after first write");
    std::thread::sleep(std::time::Duration::from_millis(25));
    write_generated_device(&temp, &device).expect("unchanged write");
    let second_modified = std::fs::metadata(&path)
        .expect("metadata after second write")
        .modified()
        .expect("mtime after second write");

    assert_eq!(first_modified, second_modified);

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_writer_removes_stale_generated_rust_files() {
    let temp = temp_dir("rspice-rust-backend-stale-files");
    let first = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: "pub mod stamp;\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: "#[path = \"stamp_blocks_0.rs\"] mod stamp_blocks_0;\n#[path = \"stamp_blocks_1.rs\"] mod stamp_blocks_1;\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "stamp_blocks_0.rs".to_string(),
                contents: "pub fn block_0() {}\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "stamp_blocks_1.rs".to_string(),
                contents: "pub fn block_1() {}\n".to_string(),
            },
        ],
    };
    let second = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: "pub mod stamp;\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: "#[path = \"stamp_blocks_0.rs\"] mod stamp_blocks_0;\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "stamp_blocks_0.rs".to_string(),
                contents: "pub fn block_0() {}\n".to_string(),
            },
        ],
    };

    write_generated_device(&temp, &first).expect("initial write");
    let stale_path = temp
        .join("tiny_res__tiny_res__abc12345")
        .join("stamp_blocks_1.rs");
    assert!(stale_path.exists());

    write_generated_device(&temp, &second).expect("rewrite with fewer files");

    assert!(
        !stale_path.exists(),
        "stale generated helper file must be removed"
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_text_writer_preserves_unchanged_files() {
    let temp = temp_dir("rspice-rust-backend-stable-text-write");
    let path = temp.join("registry.rs");

    let first_write =
        write_text_file_if_changed(&path, "pub const VALUE: usize = 1;\n").expect("write text");
    let first_modified = std::fs::metadata(&path)
        .expect("metadata after first write")
        .modified()
        .expect("mtime after first write");
    std::thread::sleep(std::time::Duration::from_millis(25));
    let second_write =
        write_text_file_if_changed(&path, "pub const VALUE: usize = 1;\n").expect("rewrite text");
    let second_modified = std::fs::metadata(&path)
        .expect("metadata after second write")
        .modified()
        .expect("mtime after second write");

    assert!(first_write, "initial write should touch the file");
    assert!(!second_write, "unchanged write should not touch the file");
    assert_eq!(first_modified, second_modified);

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_root_cleanup_removes_obsolete_digest_folders() {
    let temp = temp_dir("rspice-rust-backend-stale-device-folders");
    let devices = temp.join("devices");
    let keep = devices.join("tiny_res__tiny_res__abc12345");
    let stale = devices.join("tiny_res__tiny_res__deadbeef");
    std::fs::create_dir_all(&keep).expect("create current folder");
    std::fs::create_dir_all(&stale).expect("create stale folder");
    std::fs::write(stale.join("mod.rs"), "pub mod stale;\n").expect("write stale source");
    std::fs::write(stale.join(".rspice-veriloga-generated"), "generated\n")
        .expect("write stale marker");

    cleanup_stale_generated_device_folders(&devices, ["tiny_res__tiny_res__abc12345"])
        .expect("cleanup generated folders");

    assert!(keep.exists(), "current generated folder must be preserved");
    assert!(!stale.exists(), "obsolete digest folder must be removed");

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn generated_device_root_cleanup_preserves_unmarked_source_tree_directories() {
    let temp = temp_dir("rspice-rust-backend-source-tree-cleanup");
    let generated_root = temp.join("veriloga_generated");
    let keep = generated_root.join("tiny_res__tiny_res__abc12345");
    let stale = generated_root.join("tiny_res__tiny_res__deadbeef");
    let handwritten = generated_root.join("runtime_support");
    std::fs::create_dir_all(&keep).expect("create current folder");
    std::fs::create_dir_all(&stale).expect("create stale folder");
    std::fs::create_dir_all(&handwritten).expect("create handwritten folder");
    std::fs::write(stale.join(".rspice-veriloga-generated"), "generated\n")
        .expect("write stale marker");
    std::fs::write(handwritten.join("mod.rs"), "pub fn support() {}\n")
        .expect("write handwritten source");

    cleanup_stale_generated_device_folders(&generated_root, ["tiny_res__tiny_res__abc12345"])
        .expect("cleanup generated folders");

    assert!(keep.exists(), "current generated folder must be preserved");
    assert!(
        !stale.exists(),
        "marked stale generated folder must be removed"
    );
    assert!(
        handwritten.exists(),
        "unmarked source-tree directories must not be removed"
    );

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
fn discovery_accepts_case_insensitive_veriloga_extensions() {
    let dir = temp_dir("rspice-va-discovery-extension-case");
    std::fs::write(
        dir.join("upper.VA"),
        "module upper_case_source(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    )
    .expect("write uppercase extension source");

    let found = discover_veriloga_sources(&dir).expect("discover uppercase extension source");
    let names: Vec<_> = found
        .iter()
        .flat_map(|source| source.modules.iter().cloned())
        .collect();

    assert_eq!(names, vec!["upper_case_source".to_string()]);

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn discovery_tolerates_macros_comments_and_strings() {
    let dir = temp_dir("rspice-va-discovery-macros");
    std::fs::write(
        dir.join("macro_heavy.va"),
        r#"
// module comment_only(p,n);
`define DECLARE_FAKE module fake_from_macro(p,n)
module real_device(p,n);
    inout p,n;
    electrical p,n;
    analog begin
        $strobe("module not_a_device");
        I(p,n) <+ V(p,n);
    end
endmodule
"#,
    )
    .expect("write macro-heavy source");

    let found = discover_veriloga_sources(&dir).expect("discover macro-heavy source");
    let names: Vec<_> = found
        .iter()
        .flat_map(|source| source.modules.iter().cloned())
        .collect();

    assert_eq!(names, vec!["real_device".to_string()]);

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn discovery_ignores_modules_in_inactive_preprocessor_branches() {
    let dir = temp_dir("rspice-va-discovery-ifdef");
    std::fs::write(
        dir.join("conditional.va"),
        r#"
`ifdef __XYCE__
module inactive_name(p,n);
endmodule
`else
module active_name(p,n);
endmodule
`endif
"#,
    )
    .expect("write conditional source");

    let found = discover_veriloga_sources(&dir).expect("discover conditional source");
    let names: Vec<_> = found
        .iter()
        .flat_map(|source| source.modules.iter().cloned())
        .collect();

    assert_eq!(names, vec!["active_name".to_string()]);

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn discovery_uses_included_defines_for_conditional_modules() {
    let dir = temp_dir("rspice-va-discovery-include-defines");
    std::fs::write(dir.join("defs.include"), "`define ENABLE_INCLUDED_MODEL\n")
        .expect("write include");
    std::fs::write(
        dir.join("conditional_include.va"),
        r#"
`include "defs.include"
`ifdef ENABLE_INCLUDED_MODEL
module included_model(p,n);
    inout p,n;
    electrical p,n;
    analog I(p,n)<+V(p,n);
endmodule
`endif
"#,
    )
    .expect("write conditional include source");

    let found = discover_veriloga_sources(&dir).expect("discover include-guarded source");
    let names: Vec<_> = found
        .iter()
        .flat_map(|source| source.modules.iter().cloned())
        .collect();

    assert_eq!(names, vec!["included_model".to_string()]);

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
    assert!(stamp.contains("GeneratedDerivative::node(self.nodes[0]"));
    assert!(stamp.contains("GeneratedDerivative::node(self.nodes[1]"));
    assert!(stamp.contains("stamper.stamp_current"));
    assert!(!stamp.contains("Bytecode"));
    assert!(!stamp.contains("Interpreter"));
    assert!(!stamp.contains("HashMap"));
    assert!(!stamp.contains("fn store_into"), "{stamp}");
    assert!(!stamp.contains("fn select(condition"), "{stamp}");
    assert!(
        !stamp.contains("Self::add(left, Self::neg(right))"),
        "generated subtraction must emit one derivative pass instead of negating then adding:\n{stamp}"
    );
    assert!(
        !stamp.contains("self.params.r / (self.params.r * self.params.r)"),
        "division derivatives with constant denominator axis should simplify to one division:\n{stamp}"
    );
    assert!(
        !stamp.contains("let denominator = right.value * right.value;"),
        "compact AD division must hoist reciprocal work out of derivative loops:\n{stamp}"
    );
    assert!(
        !stamp.contains("let mut scratch = Scratch::new();"),
        "{stamp}"
    );
}

#[test]
fn rust_backend_omits_zero_derivative_locals_and_stamp_terms() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(constant_current_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile constant current source");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(!stamp.contains("_d_n0: f64 = 0.0"), "{stamp}");
    assert!(!stamp.contains("_d_n1: f64 = 0.0"), "{stamp}");
    assert!(!stamp.contains("_d_n0: f64 = if"), "{stamp}");
}

#[test]
fn rust_backend_collapses_zero_conditional_derivatives() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(parameter_conditional_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile parameter conditional assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(!stamp.contains("{ 0.0 } else { 0.0 }"), "{stamp}");
    assert!(!stamp.contains("_d_n0: f64 = if"), "{stamp}");
    assert!(!stamp.contains("_d_n1: f64 = if"), "{stamp}");
    assert!(stamp.contains("scratch.values[0] = (if"), "{stamp}");
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
}

#[test]
fn rust_backend_does_not_copy_scratch_derivatives_through_temporary_locals() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(same_branch_conditional_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile same branch conditional assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        !stamp.contains(": f64 = scratch.node_derivatives"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("if (self.params.use_alt > 0.0) { scratch.node_derivatives"),
        "{stamp}"
    );
    assert!(!stamp.contains("AdValue::select"), "{stamp}");
    assert!(
        !stamp.contains(
            "if (self.params.use_alt > 0.0) { scratch.values[0] } else { scratch.values[0] }"
        ),
        "{stamp}"
    );
    assert!(
        stamp.contains("GeneratedDerivative::node(self.nodes[0]"),
        "{stamp}"
    );
    assert!(stamp.contains("scratch.node_derivatives[1][0]"), "{stamp}");
}

#[test]
fn rust_backend_lowers_compact_conditional_noop_branch_as_guarded_store() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_conditional_noop_branch_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact conditional no-op branch assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("if (self.params.use_alt > 0.0) {"),
        "{stamp}"
    );
    assert!(
        stamp.contains(
            "scratch.store_ad(0, &AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)));"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::constant(scratch.values[0])"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_conditional_scalar_noop_branch_as_direct_store() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_conditional_scalar_noop_branch_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact conditional scalar no-op branch assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("if (self.params.use_alt > 0.0) {"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.values[1] = scratch.values[0];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.node_derivatives[1] = [0.0; Instance::NODE_COUNT];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.branch_derivatives[1] = [0.0; Instance::BRANCH_COUNT];"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::constant(scratch.values[0])"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_conditional_scalar_branch_as_direct_store() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_conditional_scalar_branch_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact conditional scalar branch assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "scratch.store_ad(1, &AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)));"
        ),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.values[1] = scratch.values[0];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.node_derivatives[1] = [0.0; Instance::NODE_COUNT];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.branch_derivatives[1] = [0.0; Instance::BRANCH_COUNT];"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::constant(scratch.values[0])"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_ad_identifier_assignment_as_direct_copy() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_ad_identifier_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact AD identifier assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("scratch.values[1] = scratch.values[0];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.node_derivatives[1] = scratch.node_derivatives[0];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.branch_derivatives[1] = scratch.branch_derivatives[0];"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("scratch.store_ad(1, &scratch.ad_value(0));"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_drops_compact_ad_self_assignment() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_ad_self_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact AD self assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        !stamp.contains("scratch.store_ad(0, &scratch.ad_value(0));"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("scratch.values[0] = scratch.values[0];"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_conditional_ad_noop_branch_as_direct_copy() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_conditional_ad_noop_branch_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact conditional AD no-op branch assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("if (self.params.use_alt > 0.0) {"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.values[1] = scratch.values[0];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.node_derivatives[1] = scratch.node_derivatives[0];"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.branch_derivatives[1] = scratch.branch_derivatives[0];"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("scratch.store_ad(1, &scratch.ad_value(0));"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_numeric_scale_without_full_ad_multiply() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_numeric_scale_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact numeric scale");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), 2.0)"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::mul(AdValue::constant(2.0), AdValue::voltage"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_additive_numeric_identities_directly() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_additive_identity_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact additive identity");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        !stamp.contains("AdValue::add(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), AdValue::constant(0.0))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::sub(AdValue::constant(0.0), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.store_ad(1, &AdValue::neg(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1))))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_duplicate_ad_operands_directly() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_duplicate_ad_operands_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact duplicate AD operands");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), 2.0)"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.store_ad(1, &AdValue::constant(0.0))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::add(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::sub(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_duplicate_ad_multiply_as_square() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_duplicate_ad_multiply_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact duplicate AD multiply");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("scratch.store_ad(1, &AdValue::square(scratch.ad_value(0)));"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::mul(scratch.ad_value(0), scratch.ad_value(0))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_add_with_negated_operand_as_subtract() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_add_with_negated_operand_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact add with negated operand");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("scratch.store_ad(1, &AdValue::sub(AdValue::scale(scratch.ad_value(0), 2.0), scratch.ad_value(0)))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::add(AdValue::neg(scratch.ad_value(0)), AdValue::scale(scratch.ad_value(0), 2.0))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_plus_negated_operand_without_constant_ad_wrapper() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_plus_negated_operand_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar plus negated operand");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("AdValue::sub_from_scalar(self.params.bias, scratch.ad_value(0))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::sub(AdValue::constant(self.params.bias), scratch.ad_value(0))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_parameter_scale_without_full_ad_multiply() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_parameter_scale_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact parameter scale");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), self.params.gain)"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::mul(AdValue::constant(self.params.gain), AdValue::voltage"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_expression_scale_without_full_ad_multiply() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_expression_scale_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar expression scale");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), (self.params.gain - 1.0))"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::mul(AdValue::constant((self.params.gain - 1.0)), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_numerator_division_without_constant_ad_wrapper() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_numerator_division_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar numerator division");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::div_from_scalar(2.0, AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::div(AdValue::constant(2.0), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_combines_nested_compact_scalar_scales() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_nested_scalar_scale_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact nested scalar scale");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), (3.0 * self.params.gain))"
        ),
        "{stamp}"
    );
    assert!(!stamp.contains("AdValue::scale(AdValue::scale"), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_combines_scaled_compact_duplicate_ad_operands() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scaled_duplicate_ad_operands_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile scaled compact duplicate AD operands");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::scale(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), (2.0 * self.params.gain))"
        ),
        "{stamp}"
    );
    assert!(!stamp.contains("AdValue::scale(AdValue::scale"), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_folds_compact_scalar_only_arithmetic_to_scratch_value() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_only_arithmetic_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar-only arithmetic");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("scratch.values[0] = (0.5 * self.params.gain);"),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(
        !stamp.contains("AdValue::scale(AdValue::constant(self.params.gain), 0.5)"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_folds_compact_scalar_comparisons_without_ad_wrappers() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_comparison_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar comparison");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("scratch.values[0] = if (self.params.gain <= 0.0) { 1.0 } else { 0.0 };"),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(
        !stamp
            .contains("AdValue::constant(self.params.gain).value <= AdValue::constant(0.0).value"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_stores_zero_derivative_compact_comparison_assignments_as_values() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_comparison_flag_current_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact comparison flag current");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "scratch.values[0] = if ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1])) > self.params.trip) { 1.0 } else { 0.0 };"
        ),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(
        stamp.contains("let eq0_e3_d_n0: f64 = scratch.values[0];"),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.node_derivatives[0]"), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_comparison_scale_factors_as_plain_scalars() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_comparison_scale_factor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact comparison scale factor");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("if (self.params.c1 > 0.0)"), "{stamp}");
    assert!(!stamp.contains("AdValue::constant(if"), "{stamp}");
    assert!(!stamp.contains("AdValue::add(AdValue::constant"), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_comparison_conditional_operands_as_values() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_comparison_conditional_operand_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact comparison conditional operand");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("scratch.values[0] = if"), "{stamp}");
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(!stamp.contains("AdValue::constant(if"), "{stamp}");
    assert!(!stamp.contains(".value) { 1.0 } else { 0.0 }"), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_mixed_scalar_comparisons_without_scalar_ad_wrappers() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_mixed_scalar_comparison_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact mixed scalar comparison");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "scratch.values[0] = if ((ctx.node_voltage(self.nodes[0]) - ctx.node_voltage(self.nodes[1])) > 0.0) { 1.0 } else { 0.0 };"
        ),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(
        !stamp.contains("AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)).value > 0.0"),
        "{stamp}"
    );
    assert!(!stamp.contains("AdValue::constant(0.0).value"), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_truth_conditions_without_ad_wrappers() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_truth_condition_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar truth condition");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("if (self.params.enabled != 0.0)"), "{stamp}");
    assert!(
        !stamp.contains("AdValue::constant(self.params.enabled).value != 0.0"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_compound_conditions_as_value_arithmetic() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_compound_condition_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact compound condition");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("scratch.values[0]"), "{stamp}");
    assert!(stamp.contains(" > self.params.trip"), "{stamp}");
    assert!(
        !stamp.contains("AdValue::add(") || !stamp.contains(").value > self.params.trip"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::mul(") || !stamp.contains(").value > self.params.trip"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_offsets_without_full_ad_add_or_subtract() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_offset_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar offset");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::offset(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), self.params.bias)"
        ),
        "{stamp}"
    );
    assert!(
        stamp.contains(
            "AdValue::sub_from_scalar(1.0, AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::add(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), AdValue::constant(self.params.bias))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::sub(AdValue::constant(1.0), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_folds_nested_compact_scalar_only_arithmetic_to_scratch_value() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_nested_scalar_only_arithmetic_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact nested scalar-only arithmetic");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("scratch.values[0] = ((4.0 * self.params.gain) * self.params.gain);"),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(
        !stamp.contains(
            "AdValue::scale(AdValue::constant((4.0 * self.params.gain)), self.params.gain)"
        ),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_omits_reactive_work_for_models_without_ddt() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(assignment_fed_resistor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile assignment-fed resistor");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();
    let reactive_body = stamp
        .split("pub fn stamp_reactive")
        .nth(1)
        .expect("reactive method");

    assert!(!reactive_body.contains("let mut g"), "{reactive_body}");
    assert!(!reactive_body.contains("g = "), "{reactive_body}");
    assert!(!reactive_body.contains("_q"), "{reactive_body}");
    assert!(
        !reactive_body.contains("stamp_current_reactive"),
        "{reactive_body}"
    );
}

#[test]
fn rust_backend_lowers_scalar_assignments_before_current_equations() {
    let src = r#"
module assigned_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    real g;
    analog begin
        g = 1.0 / r;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(src)
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile assignment-fed resistor");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("let mut scratch = Scratch::new();"),
        "{stamp}"
    );
    assert!(
        stamp.contains("scratch.values[0] = (1.0 / self.params.r);"),
        "{stamp}"
    );
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(stamp.contains("eq0_value"), "{stamp}");
    assert!(!stamp.contains("scratch.node_derivatives[0]"), "{stamp}");
    assert!(!stamp.contains("g_d_n0 ="), "{stamp}");
    assert!(!stamp.contains("g_d_n1 ="), "{stamp}");
    assert!(!stamp.contains("0.0 *"), "{stamp}");
    assert!(!stamp.contains("* 0.0"), "{stamp}");
    assert!(!stamp.contains(": f64 = g;"), "{stamp}");
    assert!(!stamp.contains("HashMap"), "{stamp}");
    assert!(!stamp.contains("Bytecode"), "{stamp}");
    assert!(!stamp.contains("g_q"), "{stamp}");
}

#[test]
fn rust_backend_keeps_reactive_shadow_code_out_of_transient_stamp() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(capacitor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile capacitor");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();
    let transient_body = stamp
        .split("pub fn stamp_reactive")
        .next()
        .expect("transient method");

    assert!(!transient_body.contains("_q"), "{transient_body}");
    assert!(stamp.contains("stamp_current_reactive"), "{stamp}");
}

#[test]
fn rust_backend_splits_transient_and_reactive_scratch_storage() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(mixed_dynamic_static_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile mixed dynamic/static device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();
    let transient_scratch = stamp
        .split("struct Scratch")
        .nth(1)
        .expect("transient scratch struct")
        .split("struct ReactiveScratch")
        .next()
        .expect("transient scratch section");
    let reactive_scratch = stamp
        .split("struct ReactiveScratch")
        .nth(1)
        .expect("reactive scratch struct")
        .split("#[derive(Clone)]")
        .next()
        .expect("reactive scratch section");

    assert!(!transient_scratch.contains("reactive_values"), "{stamp}");
    assert!(
        !transient_scratch.contains("reactive_node_derivatives"),
        "{stamp}"
    );
    assert!(reactive_scratch.contains("reactive_values"), "{stamp}");
    assert!(
        stamp.contains("let mut scratch = Scratch::new();"),
        "{stamp}"
    );
    assert!(
        stamp.contains("let mut scratch = ReactiveScratch::new();"),
        "{stamp}"
    );
}

#[test]
fn rust_backend_prunes_static_assignments_from_reactive_stamp() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(mixed_dynamic_static_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile mixed dynamic/static device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();
    let reactive_body = stamp
        .split("pub fn stamp_reactive")
        .nth(1)
        .expect("reactive method");

    assert!(!reactive_body.contains("static_g"), "{reactive_body}");
    assert!(
        reactive_body.contains("scratch.values[1]"),
        "{reactive_body}"
    );
    assert!(
        reactive_body.contains("stamp_current_reactive"),
        "{reactive_body}"
    );
}

#[test]
fn rust_backend_splits_large_stamp_bodies_into_helper_blocks() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&chunked_assignment_chain_source(320))
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile chunked assignment chain");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();
    let helper = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp_blocks_0.rs")
        .expect("stamp helper file")
        .contents
        .as_str();

    assert!(
        stamp.contains("let mut scratch = Scratch::new();"),
        "{stamp}"
    );
    assert!(stamp.contains("#[path = \"stamp_blocks_0.rs\"]"), "{stamp}");
    assert!(stamp.contains("mod stamp_blocks_0;"), "{stamp}");
    assert!(
        stamp.contains("self.stamp_transient_block_0(ctx, stamper, &mut scratch);"),
        "{stamp}"
    );
    assert!(
        helper.contains(
            "use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};"
        ),
        "{helper}"
    );
    assert!(helper.contains("fn stamp_transient_block_0"), "{helper}");
    assert!(helper.contains("fn stamp_transient_block_1"), "{helper}");
}

#[test]
fn rust_backend_lowers_runtime_loops_with_derivative_shadows() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(loop_accumulator_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile loop accumulator");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(state.contains("MAX_ANALOG_LOOP_ITERATIONS"), "{state}");
    assert!(stamp.contains("while"), "{stamp}");
    assert!(stamp.contains("loop_guard"), "{stamp}");
    assert!(
        stamp.contains("GeneratedDerivative::node(self.nodes[0]"),
        "{stamp}"
    );
    assert!(
        stamp.contains("GeneratedDerivative::node(self.nodes[1]"),
        "{stamp}"
    );
}

#[test]
fn generated_runtime_loop_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(loop_accumulator_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile loop accumulator");

    assert_generated_rust_compiles(&generated);
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

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_emits_terminal_and_internal_node_metadata() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(internal_node_resistor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile internal-node resistor");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();

    assert!(state.contains("TERMINAL_COUNT: usize = 2"), "{state}");
    assert!(state.contains("INTERNAL_NODE_COUNT: usize = 1"), "{state}");
    assert!(state.contains("NODE_COUNT: usize = 3"), "{state}");
    assert!(
        state.contains("INTERNAL_NODE_NAMES: [&str; 1] = [\"x\"]"),
        "{state}"
    );
}

#[test]
fn generated_internal_node_device_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(internal_node_resistor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile internal-node resistor");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn generated_assignment_fed_current_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(
            r#"
module assigned_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    real g;
    analog begin
        g = 1.0 / r;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
        )
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile assignment-fed resistor");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_ddt_current_into_stateful_stamp_and_reactive_stamp() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(capacitor_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile capacitor");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(state.contains("DDT_STATE_COUNT: usize = 1"), "{state}");
    assert!(state.contains("ddt_state_current"), "{state}");
    assert!(state.contains("set_timepoint"), "{state}");
    assert!(state.contains("accept_timestep"), "{state}");
    assert!(stamp.contains("eval_ddt"), "{stamp}");
    assert!(stamp.contains("ddt_jacobian"), "{stamp}");
    assert!(stamp.contains("pub fn stamp_reactive"), "{stamp}");
    assert!(stamp.contains("stamper.stamp_current_reactive"), "{stamp}");
    assert!(!stamp.contains("Bytecode"), "{stamp}");
    assert!(!stamp.contains("Interpreter"), "{stamp}");
}

#[test]
fn rust_backend_skips_dead_ddx_assignment() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(ddx_op_info_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile ddx operating-point assignment");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(!stamp.contains("ddx_projection(&"), "{stamp}");
    assert!(!stamp.contains("scratch.store_ad(0"), "{stamp}");
    assert!(!stamp.contains("_d_n0"), "{stamp}");
    assert!(!stamp.contains("_d_n1"), "{stamp}");
}

#[test]
fn generated_ddx_assignment_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(ddx_op_info_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile ddx operating-point assignment");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_rejects_ddx_feeding_current_until_second_derivatives_exist() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(ddx_current_source())
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("ddx contribution needs second derivatives");

    let rendered = err.to_string();
    assert!(rendered.contains("ddx"), "{rendered}");
    assert!(rendered.contains("second derivatives"), "{rendered}");
}

#[test]
fn generated_ddt_current_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(capacitor_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile capacitor");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_noise_terms_to_zero_for_large_signal_stamps() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(noisy_current_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile noisy current source");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("eq0_value"), "{stamp}");
    assert!(stamp.contains("0.0"), "{stamp}");
    assert!(!stamp.contains("white_noise"), "{stamp}");
    assert!(!stamp.contains("flicker_noise"), "{stamp}");
    assert!(!stamp.contains("Interpreter"), "{stamp}");
}

#[test]
fn generated_noise_term_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(noisy_current_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile noisy current source");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_intrinsic_math_with_analytic_derivatives() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(math_device_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile math device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains(".sqrt()"), "{stamp}");
    assert!(stamp.contains(".exp()"), "{stamp}");
    assert!(stamp.contains(".ln()"), "{stamp}");
    assert!(stamp.contains(".powf("), "{stamp}");
    assert!(stamp.contains(".floor()"), "{stamp}");
    assert!(
        !stamp.contains("arg.clone()"),
        "generated unary intrinsics must consume owned AdValue operands without cloning derivative arrays:\n{stamp}"
    );
    assert!(
        stamp.contains("GeneratedDerivative::node(self.nodes[0]"),
        "{stamp}"
    );
    assert!(stamp.contains("if "), "{stamp}");
    assert!(!stamp.contains("Interpreter"), "{stamp}");
}

#[test]
fn rust_backend_lowers_compact_scalar_exponent_pow_without_constant_ad_wrapper() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_exponent_pow_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar exponent pow");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::powf(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), self.params.exponent)"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::pow(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), AdValue::constant(self.params.exponent))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_base_pow_without_constant_ad_wrapper() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_base_pow_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar base pow");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::pow_from_scalar(scratch.values[0], AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::pow(AdValue::constant(scratch.values[0])"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_compact_scalar_min_max_without_constant_ad_wrapper() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_min_max_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar min max");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains(
            "AdValue::max_with_scalar(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), self.params.floor)"
        ),
        "{stamp}"
    );
    assert!(
        stamp.contains(
            "AdValue::min_from_scalar(self.params.ceiling, AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"
        ),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::max(AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)), AdValue::constant(self.params.floor))"),
        "{stamp}"
    );
    assert!(
        !stamp.contains("AdValue::min(AdValue::constant(self.params.ceiling), AdValue::voltage(ctx, &self.nodes, Some(0), Some(1)))"),
        "{stamp}"
    );
    assert_generated_rust_compiles(&generated);
}

#[test]
fn generated_compact_scalar_literal_pow_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_scalar_literal_pow_assignment_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile compact scalar literal pow");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains(".powf("), "{stamp}");
    assert_generated_rust_compiles(&generated);
}

#[test]
fn generated_intrinsic_math_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(math_device_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile math device");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn generated_pow_integer_derivative_stays_finite_for_negative_base() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(negative_base_pow_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile integer pow device");

    assert_generated_pow_square_runtime(&generated);
}

#[test]
fn rust_backend_lowers_conditional_expressions_with_selected_derivatives() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(conditional_device_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile conditional device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("if "), "{stamp}");
    assert!(stamp.contains(" > "), "{stamp}");
    assert!(
        stamp.contains("GeneratedDerivative::node(self.nodes[0]"),
        "{stamp}"
    );
    assert!(
        stamp.contains("GeneratedDerivative::node(self.nodes[1]"),
        "{stamp}"
    );
    assert!(!stamp.contains("Interpreter"), "{stamp}");
}

#[test]
fn rust_backend_keeps_noncompact_conditional_branches_lazy() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(noncompact_lazy_conditional_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile lazy conditional device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();
    let stamp_body = stamp
        .split("pub fn stamp(&mut self")
        .nth(1)
        .expect("stamp method")
        .split("pub fn stamp_reactive")
        .next()
        .expect("transient stamp body");
    let condition_index = stamp_body.find("if ").expect("conditional in stamp body");
    let sqrt_index = stamp_body
        .find(".sqrt()")
        .expect("sqrt branch in stamp body");

    assert!(
        !stamp_body[..condition_index].contains(".sqrt()"),
        "{stamp_body}"
    );
    assert!(
        sqrt_index > condition_index,
        "sqrt branch must be inside the generated conditional\n{stamp_body}"
    );
}

#[test]
fn generated_conditional_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(conditional_device_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile conditional device");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_numeric_comparisons_as_one_zero_values() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(comparison_value_device_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile comparison-valued device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("1.0"), "{stamp}");
    assert!(stamp.contains("0.0"), "{stamp}");
    assert!(stamp.contains("=="), "{stamp}");
    assert!(!stamp.contains("GeneratedDerivative::node"), "{stamp}");
}

#[test]
fn rust_backend_lowers_logical_comparisons_as_zero_reactive_values() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(logical_value_device_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile logical-valued device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("&&"), "{stamp}");
    assert!(stamp.contains("1.0"), "{stamp}");
    assert!(stamp.contains("0.0"), "{stamp}");
    assert!(stamp.contains("pub fn stamp_reactive"), "{stamp}");
}

#[test]
fn rust_backend_lowers_logical_not_as_one_zero_value() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(logical_not_device_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile logical-not-valued device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("!"), "{stamp}");
    assert!(stamp.contains("1.0"), "{stamp}");
    assert!(stamp.contains("0.0"), "{stamp}");
    assert!(!stamp.contains("GeneratedDerivative::node"), "{stamp}");
}

#[test]
fn rust_backend_lowers_simulator_system_functions_to_direct_runtime_access() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(system_function_device_source())
        .expect("canonical IR");

    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile system-function device");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(state.contains("param_given"), "{state}");
    assert!(state.contains("multiplicity"), "{state}");
    assert!(state.contains("mark_param_given"), "{state}");
    assert!(stamp.contains("ctx.temperature()"), "{stamp}");
    assert!(stamp.contains("ctx.thermal_voltage()"), "{stamp}");
    assert!(stamp.contains("self.time"), "{stamp}");
    assert!(stamp.contains("self.multiplicity"), "{stamp}");
    assert!(stamp.contains("self.param_given[0]"), "{stamp}");
    assert!(stamp.contains("1e-12"), "{stamp}");
    assert!(!stamp.contains("SystemFunction"), "{stamp}");
    assert!(!stamp.contains("Interpreter"), "{stamp}");
}

#[test]
fn rust_backend_rejects_compact_zero_arg_system_functions_with_arguments() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_temperature_with_argument_source())
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("compact $temperature with arguments must be rejected");
    let rendered = err.to_string();

    assert!(
        rendered.contains("$temperature expects 0 argument(s), found 1"),
        "{rendered}"
    );
}

#[test]
fn rust_backend_rejects_compact_port_connected_without_terminal_argument() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(compact_port_connected_without_argument_source())
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("compact $port_connected without a terminal must be rejected");
    let rendered = err.to_string();

    assert!(
        rendered.contains("$port_connected expects 1 argument(s), found 0"),
        "{rendered}"
    );
}

#[test]
fn generated_system_function_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(system_function_device_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile system-function device");

    assert_generated_rust_compiles(&generated);
}

fn assert_generated_rust_compiles(generated: &GeneratedRustDevice) {
    let temp = temp_dir("rspice-rust-backend-compile");

    write_generated_device(&temp, generated).expect("write generated device");
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

        pub fn temperature(&self) -> f64 {{
            300.15
        }}

        pub fn thermal_voltage(&self) -> f64 {{
            300.15 * 1.380649e-23 / 1.602176634e-19
        }}

        pub fn branch_current(&self, branch_ordinal: usize) -> f64 {{
            self.voltages.get(branch_ordinal).copied().unwrap_or(0.0)
        }}
    }}

    pub struct GeneratedStamper<'a> {{
        pub touched: &'a mut f64,
    }}

    pub struct GeneratedDerivative {{
        pub value: f64,
    }}

    impl GeneratedDerivative {{
        pub fn node(_node: usize, value: f64) -> Self {{
            Self {{ value }}
        }}

        pub fn branch(_branch_ordinal: usize, value: f64) -> Self {{
            Self {{ value }}
        }}
    }}

    impl<'a> GeneratedStamper<'a> {{
        pub fn stamp_current(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            value: f64,
            derivatives: &[GeneratedDerivative],
        ) {{
            *self.touched += value + derivatives.iter().map(|derivative| derivative.value).sum::<f64>();
        }}

        pub fn stamp_potential_branch(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _branch: usize,
            multiplicity: f64,
        ) {{
            *self.touched += multiplicity;
        }}

        pub fn stamp_potential(
            &mut self,
            _branch: usize,
            value: f64,
            derivatives: &[GeneratedDerivative],
        ) {{
            *self.touched += value + derivatives.iter().map(|derivative| derivative.value).sum::<f64>();
        }}
    }}

    pub struct GeneratedReactiveStamper<'a> {{
        pub touched: &'a mut f64,
    }}

    impl<'a> GeneratedReactiveStamper<'a> {{
        pub fn stamp_current_reactive(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            derivatives: &[GeneratedDerivative],
        ) {{
            *self.touched += derivatives.iter().map(|derivative| derivative.value).sum::<f64>();
        }}

        pub fn stamp_potential_reactive(
            &mut self,
            _branch: usize,
            derivatives: &[GeneratedDerivative],
        ) {{
            *self.touched += derivatives.iter().map(|derivative| derivative.value).sum::<f64>();
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

fn assert_generated_pow_square_runtime(generated: &GeneratedRustDevice) {
    let temp = temp_dir("rspice-rust-backend-run-pow");

    write_generated_device(&temp, generated).expect("write generated device");
    let executable = temp.join("run_pow_square.exe");
    let main = temp.join("run_pow_square.rs");
    std::fs::write(
        &main,
        format!(
            r#"
pub mod runtime {{
    pub const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

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

        pub fn temperature(&self) -> f64 {{
            300.15
        }}

        pub fn thermal_voltage(&self) -> f64 {{
            300.15 * THERMAL_VOLTAGE_PER_K
        }}

        pub fn branch_current(&self, branch_ordinal: usize) -> f64 {{
            self.voltages.get(branch_ordinal).copied().unwrap_or(0.0)
        }}
    }}

    pub struct GeneratedDerivative {{
        pub node: Option<usize>,
        pub value: f64,
    }}

    impl GeneratedDerivative {{
        pub fn node(node: usize, value: f64) -> Self {{
            Self {{ node: Some(node), value }}
        }}

        pub fn branch(_branch_ordinal: usize, value: f64) -> Self {{
            Self {{ node: None, value }}
        }}
    }}

    pub struct GeneratedStamper<'a> {{
        pub current: &'a mut f64,
        pub node_derivatives: &'a mut [f64; 2],
    }}

    impl<'a> GeneratedStamper<'a> {{
        pub fn stamp_current(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            value: f64,
            derivatives: &[GeneratedDerivative],
        ) {{
            *self.current += value;
            for derivative in derivatives {{
                if let Some(node) = derivative.node {{
                    self.node_derivatives[node] += derivative.value;
                }}
            }}
        }}

        pub fn stamp_potential_branch(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _branch: usize,
            _multiplicity: f64,
        ) {{
        }}

        pub fn stamp_potential(
            &mut self,
            _branch: usize,
            _value: f64,
            _derivatives: &[GeneratedDerivative],
        ) {{
        }}
    }}

    pub struct GeneratedReactiveStamper<'a> {{
        _marker: std::marker::PhantomData<&'a mut ()>,
    }}

    impl<'a> GeneratedReactiveStamper<'a> {{
        pub fn stamp_current_reactive(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _derivatives: &[GeneratedDerivative],
        ) {{
        }}

        pub fn stamp_potential_reactive(
            &mut self,
            _branch: usize,
            _derivatives: &[GeneratedDerivative],
        ) {{
        }}
    }}
}}

#[path = "{}"]
pub mod generated_device;

fn main() {{
    let mut instance = generated_device::Instance::new(&[0, 1]);
    let ctx = runtime::GeneratedEvalContext::new(&[-2.0, 0.0]);
    let mut current = 0.0;
    let mut node_derivatives = [0.0; 2];
    let mut stamper = runtime::GeneratedStamper {{
        current: &mut current,
        node_derivatives: &mut node_derivatives,
    }};
    instance.stamp(&ctx, &mut stamper);

    assert!(current.is_finite(), "current must be finite: {{current}}");
    assert!(node_derivatives[0].is_finite(), "d/dp must be finite: {{}}", node_derivatives[0]);
    assert!(node_derivatives[1].is_finite(), "d/dn must be finite: {{}}", node_derivatives[1]);
    assert!((current - 4.0).abs() < 1e-12, "current={{current}}");
    assert!((node_derivatives[0] + 4.0).abs() < 1e-12, "d/dp={{}}", node_derivatives[0]);
    assert!((node_derivatives[1] - 4.0).abs() < 1e-12, "d/dn={{}}", node_derivatives[1]);
}}
"#,
            temp.join(&generated.folder_name)
                .join("mod.rs")
                .display()
                .to_string()
                .replace('\\', "\\\\")
        ),
    )
    .expect("write generated runtime test");

    let output = std::process::Command::new(std::env::var("RUSTC").unwrap_or("rustc".to_string()))
        .arg("--edition=2024")
        .arg(&main)
        .arg("-o")
        .arg(&executable)
        .output()
        .expect("compile generated runtime test");

    assert!(
        output.status.success(),
        "generated runtime test did not compile\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let output = std::process::Command::new(&executable)
        .output()
        .expect("run generated runtime test");

    assert!(
        output.status.success(),
        "generated runtime test failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(temp);
}

#[test]
fn rust_backend_rejects_limexp_until_limited_runtime_exists() {
    let src = r#"
module diode_like(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ limexp(V(p, n));
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(src)
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("limexp must not lower to plain exp");

    let rendered = err.to_string();
    assert!(rendered.contains("diode_like"), "{rendered}");
    assert!(rendered.contains("limexp"), "{rendered}");
}

#[test]
fn rust_backend_lowers_potential_contribution_to_branch_unknown_stamp() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(zero_voltage_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile potential contribution");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(state.contains("BRANCH_COUNT: usize = 1"), "{state}");
    assert!(state.contains("set_branch_indices"), "{state}");
    assert!(stamp.contains("stamp_potential"), "{stamp}");
    assert!(stamp.contains("self.branches[0]"), "{stamp}");
}

#[test]
fn generated_potential_contribution_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(zero_voltage_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile potential contribution");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_keeps_distinct_named_potential_branches_with_shared_endpoints() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(shared_endpoint_voltage_branches())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile shared endpoint voltage branches");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(state.contains("BRANCH_COUNT: usize = 2"), "{state}");
    assert!(stamp.contains("self.branches[0]"), "{stamp}");
    assert!(stamp.contains("self.branches[1]"), "{stamp}");
}

#[test]
fn generated_shared_endpoint_voltage_branches_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(shared_endpoint_voltage_branches())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile shared endpoint voltage branches");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_named_branch_current_access_from_prior_contribution() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(named_branch_current_probe())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile named branch current access");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("eq0_value"), "{stamp}");
    assert!(stamp.contains("2.0"), "{stamp}");
}

#[test]
fn generated_named_branch_current_access_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(named_branch_current_probe())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile named branch current access");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_named_branch_custom_potential_access() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(thermal_branch_probe())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile custom potential branch access");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("ctx.node_voltage"), "{stamp}");
}

#[test]
fn generated_custom_potential_branch_access_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(thermal_branch_probe())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile custom potential branch access");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_simparam_parameter_default_to_explicit_fallback() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(simparam_default_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile simparam default");
    let state = generated
        .files
        .iter()
        .find(|file| file.relative_path == "state.rs")
        .expect("state file")
        .contents
        .as_str();

    assert!(state.contains("params.scale"), "{state}");
    assert!(state.contains("1.0"), "{state}");
}

#[test]
fn generated_simparam_parameter_default_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(simparam_default_source())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile simparam default");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_lowers_forward_potential_branch_current_probe() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(forward_potential_branch_current_probe())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile forward branch current probe");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(stamp.contains("AdValue::branch_current"), "{stamp}");
}

#[test]
fn rust_backend_lowers_potential_branch_current_axis_derivative() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(potential_branch_current_axis_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile branch-current axis device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("GeneratedDerivative::branch(self.branches[0]"),
        "{stamp}"
    );
    assert!(stamp.contains("self.params.r"), "{stamp}");
    assert!(!stamp.contains("let eq0_d_b0"), "{stamp}");
}

#[test]
fn rust_backend_propagates_branch_current_axis_through_assignment() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(assigned_branch_current_axis_source())
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile assigned branch-current axis device");
    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file")
        .contents
        .as_str();

    assert!(
        stamp.contains("GeneratedDerivative::branch(self.branches[0]"),
        "{stamp}"
    );
    assert!(stamp.contains("AdValue::branch_current"), "{stamp}");
    assert!(stamp.contains("self.params.r"), "{stamp}");
}

#[test]
fn generated_forward_potential_branch_current_probe_rust_compiles_with_runtime_stub() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(forward_potential_branch_current_probe())
        .expect("canonical IR");
    let generated = RustTranspiler::new(RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    })
    .transpile(&artifact)
    .expect("transpile forward branch current probe");

    assert_generated_rust_compiles(&generated);
}

#[test]
fn rust_backend_reports_real_module_for_expression_lowering_failures() {
    let src = r#"
module pow_res(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) ** 2.0;
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(src)
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("pow unsupported in first slice");
    let rendered = err.to_string();

    assert!(rendered.contains("<input>"));
    assert!(rendered.contains("pow_res"));
    assert!(rendered.contains("binary operator Pow"));
}

fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let nonce = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "{prefix}-{}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos(),
        nonce
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

fn constant_current_source() -> &'static str {
    r#"
module const_i(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 1.0;
endmodule
"#
}

fn parameter_conditional_assignment_source() -> &'static str {
    r#"
module parameter_conditional_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_alt = 0.0;
    real selected;
    analog begin
        selected = (use_alt > 0.0) ? 1.0 : 2.0;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn same_branch_conditional_assignment_source() -> &'static str {
    r#"
module same_branch_conditional_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_alt = 0.0;
    real v;
    real selected;
    analog begin
        v = V(p, n);
        selected = (use_alt > 0.0) ? v : v;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_conditional_noop_branch_assignment_source() -> &'static str {
    r#"
module compact_conditional_noop_branch_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_alt = 0.0;
    real selected;
    analog begin
        selected = 1.0;
        selected = (use_alt > 0.0) ? V(p, n) : selected;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_conditional_scalar_noop_branch_assignment_source() -> &'static str {
    r#"
module compact_conditional_scalar_noop_branch_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_alt = 0.0;
    parameter real gain = 2.0;
    real bias;
    real selected;
    analog begin
        bias = 1.0 + gain;
        selected = V(p, n);
        selected = (use_alt > 0.0) ? bias : selected;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_conditional_scalar_branch_assignment_source() -> &'static str {
    r#"
module compact_conditional_scalar_branch_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_alt = 0.0;
    parameter real gain = 2.0;
    real bias;
    real selected;
    analog begin
        bias = 1.0 + gain;
        selected = (use_alt > 0.0) ? V(p, n) : bias;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn assignment_fed_resistor_source() -> &'static str {
    r#"
module assigned_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    real g;
    analog begin
        g = 1.0 / r;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#
}

fn compact_numeric_scale_source() -> &'static str {
    r#"
module compact_numeric_scale(p, n);
    inout p, n;
    electrical p, n;
    real scaled;
    analog begin
        scaled = 2.0 * V(p, n);
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_additive_identity_source() -> &'static str {
    r#"
module compact_additive_identity(p, n);
    inout p, n;
    electrical p, n;
    real same;
    real negated;
    analog begin
        same = V(p, n) + 0.0;
        negated = 0.0 - V(p, n);
        I(p, n) <+ same + negated;
    end
endmodule
"#
}

fn compact_duplicate_ad_operands_source() -> &'static str {
    r#"
module compact_duplicate_ad_operands(p, n);
    inout p, n;
    electrical p, n;
    real doubled;
    real canceled;
    analog begin
        doubled = V(p, n) + V(p, n);
        canceled = V(p, n) - V(p, n);
        I(p, n) <+ doubled + canceled;
    end
endmodule
"#
}

fn compact_duplicate_ad_multiply_source() -> &'static str {
    r#"
module compact_duplicate_ad_multiply(p, n);
    inout p, n;
    electrical p, n;
    real v;
    real squared;
    analog begin
        v = V(p, n);
        squared = v * v;
        I(p, n) <+ squared;
    end
endmodule
"#
}

fn compact_add_with_negated_operand_source() -> &'static str {
    r#"
module compact_add_with_negated_operand(p, n);
    inout p, n;
    electrical p, n;
    real v;
    real combined;
    analog begin
        v = V(p, n);
        combined = (-v) + (2.0 * v);
        I(p, n) <+ combined;
    end
endmodule
"#
}

fn compact_scalar_plus_negated_operand_source() -> &'static str {
    r#"
module compact_scalar_plus_negated_operand(p, n);
    inout p, n;
    electrical p, n;
    parameter real bias = 1.0;
    real v;
    real shifted;
    analog begin
        v = V(p, n);
        shifted = (-v) + bias;
        I(p, n) <+ shifted;
    end
endmodule
"#
}

fn compact_ad_identifier_assignment_source() -> &'static str {
    r#"
module compact_ad_identifier_assignment(p, n);
    inout p, n;
    electrical p, n;
    real sensed;
    real copied;
    analog begin
        sensed = V(p, n);
        copied = sensed;
        I(p, n) <+ copied;
    end
endmodule
"#
}

fn compact_ad_self_assignment_source() -> &'static str {
    r#"
module compact_ad_self_assignment(p, n);
    inout p, n;
    electrical p, n;
    real sensed;
    analog begin
        sensed = V(p, n);
        sensed = sensed;
        I(p, n) <+ sensed;
    end
endmodule
"#
}

fn compact_conditional_ad_noop_branch_assignment_source() -> &'static str {
    r#"
module compact_conditional_ad_noop_branch_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_alt = 0.0;
    real candidate;
    real selected;
    analog begin
        candidate = V(p, n);
        selected = V(n, p);
        selected = (use_alt > 0.0) ? candidate : selected;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_parameter_scale_source() -> &'static str {
    r#"
module compact_parameter_scale(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real scaled;
    analog begin
        scaled = gain * V(p, n);
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_scalar_expression_scale_source() -> &'static str {
    r#"
module compact_scalar_expression_scale(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real scaled;
    analog begin
        scaled = (gain - 1.0) * V(p, n);
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_scalar_numerator_division_source() -> &'static str {
    r#"
module compact_scalar_numerator_division(p, n);
    inout p, n;
    electrical p, n;
    real scaled;
    analog begin
        scaled = 2.0 / V(p, n);
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_nested_scalar_scale_source() -> &'static str {
    r#"
module compact_nested_scalar_scale(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real scaled;
    analog begin
        scaled = (3.0 * V(p, n)) * gain;
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_scaled_duplicate_ad_operands_source() -> &'static str {
    r#"
module compact_scaled_duplicate_ad_operands(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real scaled;
    analog begin
        scaled = (V(p, n) + V(p, n)) * gain;
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_scalar_only_arithmetic_source() -> &'static str {
    r#"
module compact_scalar_only_arithmetic(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real factor;
    real scaled;
    analog begin
        factor = 0.5 * gain;
        scaled = factor * V(p, n);
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn compact_scalar_comparison_source() -> &'static str {
    r#"
module compact_scalar_comparison(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real selected;
    analog begin
        selected = gain <= 0.0;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_comparison_flag_current_source() -> &'static str {
    r#"
module compact_comparison_flag_current(p, n);
    inout p, n;
    electrical p, n;
    parameter real trip = 0.0;
    real flag;
    analog begin
        flag = V(p, n) > trip;
        I(p, n) <+ flag * V(p, n);
    end
endmodule
"#
}

fn compact_comparison_scale_factor_source() -> &'static str {
    r#"
module compact_comparison_scale_factor(p, n);
    inout p, n;
    electrical p, n;
    parameter real c1 = 1.0;
    parameter real c2 = 0.0;
    real shaped;
    analog begin
        shaped = ((c1 > 0.0) + (c2 > 0.0)) * V(p, n);
        I(p, n) <+ shaped;
    end
endmodule
"#
}

fn compact_comparison_conditional_operand_source() -> &'static str {
    r#"
module compact_comparison_conditional_operand(p, n);
    inout p, n;
    electrical p, n;
    real flag;
    analog begin
        flag = V(p, n) < ((V(p, n) > 0.0) ? V(p, n) : -V(p, n));
        I(p, n) <+ flag * V(p, n);
    end
endmodule
"#
}

fn compact_mixed_scalar_comparison_source() -> &'static str {
    r#"
module compact_mixed_scalar_comparison(p, n);
    inout p, n;
    electrical p, n;
    real selected;
    analog begin
        selected = V(p, n) > 0.0;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_scalar_truth_condition_source() -> &'static str {
    r#"
module compact_scalar_truth_condition(p, n);
    inout p, n;
    electrical p, n;
    parameter real enabled = 1.0;
    real selected;
    analog begin
        selected = enabled ? V(p, n) : 0.0;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_compound_condition_source() -> &'static str {
    r#"
module compact_compound_condition(p, n);
    inout p, n;
    electrical p, n;
    parameter real trip = 0.0;
    real v;
    real selected;
    analog begin
        v = V(p, n);
        selected = ((v + (2.0 * v)) > trip) ? v : -v;
        I(p, n) <+ selected;
    end
endmodule
"#
}

fn compact_scalar_offset_source() -> &'static str {
    r#"
module compact_scalar_offset(p, n);
    inout p, n;
    electrical p, n;
    parameter real bias = 0.1;
    real shifted;
    real reversed;
    analog begin
        shifted = V(p, n) + bias;
        reversed = 1.0 - V(p, n);
        I(p, n) <+ shifted + reversed;
    end
endmodule
"#
}

fn compact_nested_scalar_only_arithmetic_source() -> &'static str {
    r#"
module compact_nested_scalar_only_arithmetic(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real factor;
    real scaled;
    analog begin
        factor = (4.0 * gain) * gain;
        scaled = factor * V(p, n);
        I(p, n) <+ scaled;
    end
endmodule
"#
}

fn internal_node_resistor_source() -> &'static str {
    r#"
module internal_res(p, n);
    inout p, n;
    electrical p, n, x;
    analog begin
        I(p, x) <+ V(p, x);
        I(x, n) <+ V(x, n);
    end
endmodule
"#
}

fn capacitor_source() -> &'static str {
    r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-12 from (0:inf);
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#
}

fn mixed_dynamic_static_source() -> &'static str {
    r#"
module mixed_dyn_static(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    parameter real c = 1e-12 from (0:inf);
    real static_g;
    real cap_charge;
    analog begin
        static_g = 1.0 / r;
        cap_charge = c * V(p, n);
        I(p, n) <+ static_g * V(p, n);
        I(p, n) <+ ddt(cap_charge);
    end
endmodule
"#
}

fn chunked_assignment_chain_source(count: usize) -> String {
    assert!(count > 0);
    let mut source = String::from(
        r#"
module chunked_assignments(p, n);
    inout p, n;
    electrical p, n;
"#,
    );
    for index in 0..count {
        source.push_str(&format!("    real x{index};\n"));
    }
    source.push_str("    analog begin\n");
    source.push_str("        x0 = V(p, n);\n");
    for index in 1..count {
        source.push_str(&format!("        x{index} = x{} + 1.0;\n", index - 1));
    }
    source.push_str(&format!("        I(p, n) <+ x{};\n", count - 1));
    source.push_str(
        r#"    end
endmodule
"#,
    );
    source
}

fn zero_voltage_source() -> &'static str {
    r#"
module zero_vsource(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ 0.0;
endmodule
"#
}

fn named_branch_current_probe() -> &'static str {
    r#"
module named_branch_current_probe(p, n);
    inout p, n;
    electrical p, n, sense_node;
    branch (sense_node) sense;
    analog begin
        I(sense) <+ V(p, n);
        I(p, n) <+ 2.0 * I(sense);
    end
endmodule
"#
}

fn shared_endpoint_voltage_branches() -> &'static str {
    r#"
module shared_endpoint_voltage_branches(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) va;
    branch (p, n) vb;
    analog begin
        V(va) <+ 0.0;
        V(vb) <+ 1.0;
    end
endmodule
"#
}

fn thermal_branch_probe() -> &'static str {
    r#"
nature Temperature;
    access = Temp;
endnature

discipline thermal;
    potential Temperature;
enddiscipline

module thermal_branch_probe(p, n, t);
    inout p, n, t;
    electrical p, n;
    thermal t;
    branch (t) th;
    analog I(p, n) <+ Temp(th);
endmodule
"#
}

fn simparam_default_source() -> &'static str {
    r#"
module simparam_default(p, n);
    inout p, n;
    electrical p, n;
    parameter real scale = $simparam("scale", 1.0) from (0.0:2.0);
    analog I(p, n) <+ scale * V(p, n);
endmodule
"#
}

fn forward_potential_branch_current_probe() -> &'static str {
    r#"
module forward_branch_probe(p, n);
    inout p, n;
    electrical p, n;
    branch (p, n) probe;
    real seen;
    analog begin
        seen = I(probe);
        V(probe) <+ 0.0;
        I(p, n) <+ 0.0 * seen;
    end
endmodule
"#
}

fn potential_branch_current_axis_source() -> &'static str {
    r#"
module branch_current_axis(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 10.0 from [0:inf);
    branch (p, n) axis;
    analog V(axis) <+ r * I(axis);
endmodule
"#
}

fn assigned_branch_current_axis_source() -> &'static str {
    r#"
module assigned_branch_current_axis(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 10.0 from [0:inf);
    branch (p, n) axis;
    real seen;
    analog begin
        seen = I(axis);
        V(axis) <+ r * seen;
    end
endmodule
"#
}

fn loop_accumulator_source() -> &'static str {
    r#"
module loop_accum(p, n);
    inout p, n;
    electrical p, n;
    real loop;
    real acc;
    analog begin
        loop = 0.0;
        acc = 0.0;
        while (loop < 3.0) begin
            acc = acc + V(p, n);
            loop = loop + 1.0;
        end
        I(p, n) <+ acc;
    end
endmodule
"#
}

fn ddx_op_info_source() -> &'static str {
    r#"
module ddx_op_info(p, n);
    inout p, n;
    electrical p, n;
    real cap;
    analog begin
        cap = ddx(V(p, n) * V(p, n), V(p, n));
        I(p, n) <+ V(p, n);
    end
endmodule
"#
}

fn ddx_current_source() -> &'static str {
    r#"
module ddx_current(p, n);
    inout p, n;
    electrical p, n;
    real cap;
    analog begin
        cap = ddx(V(p, n) * V(p, n), V(p, n));
        I(p, n) <+ cap * V(p, n);
    end
endmodule
"#
}

fn noisy_current_source() -> &'static str {
    r#"
module noisy_source(p, n);
    inout p, n;
    electrical p, n;
    parameter real thermal = 1e-18 from [0:inf);
    parameter real flicker = 1e-20 from [0:inf);
    parameter real af = 1.0 from [0:inf);
    analog I(p, n) <+ white_noise(thermal, "thermal") + flicker_noise(flicker, af, "flicker");
endmodule
"#
}

fn math_device_source() -> &'static str {
    r#"
module math_device(p, n);
    inout p, n;
    electrical p, n;
    parameter real scale = 0.01 from (0:inf);
    analog I(p, n) <+ sqrt(abs(V(p, n)) + 1.0)
        + exp(scale * V(p, n))
        + ln(abs(V(p, n)) + 2.0)
        + pow(abs(V(p, n)) + 1.0, 2.0)
        + floor(V(p, n));
endmodule
"#
}

fn negative_base_pow_source() -> &'static str {
    r#"
module negative_pow(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ pow(V(p, n), 2.0);
endmodule
"#
}

fn compact_scalar_exponent_pow_source() -> &'static str {
    r#"
module compact_scalar_exponent_pow(p, n);
    inout p, n;
    electrical p, n;
    parameter real exponent = 2.0;
    real shaped;
    analog begin
        shaped = pow(V(p, n), exponent);
        I(p, n) <+ shaped;
    end
endmodule
"#
}

fn compact_scalar_base_pow_source() -> &'static str {
    r#"
module compact_scalar_base_pow(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    real base;
    real shaped;
    analog begin
        base = 1.0 + gain;
        shaped = pow(base, V(p, n));
        I(p, n) <+ shaped;
    end
endmodule
"#
}

fn compact_scalar_min_max_source() -> &'static str {
    r#"
module compact_scalar_min_max(p, n);
    inout p, n;
    electrical p, n;
    parameter real floor = 0.001;
    parameter real ceiling = 1.0;
    real lower;
    real upper;
    analog begin
        lower = max(V(p, n), floor);
        upper = min(ceiling, V(p, n));
        I(p, n) <+ lower + upper;
    end
endmodule
"#
}

fn compact_scalar_literal_pow_assignment_source() -> &'static str {
    r#"
module compact_scalar_literal_pow_assignment(p, n);
    inout p, n;
    electrical p, n;
    parameter real exponent = 2.0;
    real scale;
    analog begin
        scale = pow(2.0, exponent);
        I(p, n) <+ scale * V(p, n);
    end
endmodule
"#
}

fn conditional_device_source() -> &'static str {
    r#"
module conditional_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real rpos = 1000.0 from (0:inf);
    parameter real rneg = 2000.0 from (0:inf);
    analog I(p, n) <+ (V(p, n) > 0.0) ? V(p, n) / rpos : V(p, n) / rneg;
endmodule
"#
}

fn noncompact_lazy_conditional_source() -> &'static str {
    r#"
module noncompact_lazy_conditional(p, n);
    inout p, n;
    electrical p, n;
    parameter real use_bad = 0.0;
    analog I(p, n) <+ (use_bad > 0.0) ? sqrt(-1.0) : V(p, n);
endmodule
"#
}

fn comparison_value_device_source() -> &'static str {
    r#"
module comparison_value(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (V(p, n) == 0.0);
endmodule
"#
}

fn logical_value_device_source() -> &'static str {
    r#"
module logical_value(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ((V(p, n) > 0.0) && (V(p, n) < 1.0));
endmodule
"#
}

fn logical_not_device_source() -> &'static str {
    r#"
module logical_not_value(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ !(V(p, n) > 0.0);
endmodule
"#
}

fn system_function_device_source() -> &'static str {
    r#"
module system_query(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0 from [0:inf);
    analog I(p, n) <+ gain * (
        $temperature
        + $vt
        + $abstime
        + $realtime
        + $mfactor
        + $simparam("gmin")
        + $simparam("custom", 7.0)
        + $param_given(gain)
    );
endmodule
"#
}

fn compact_temperature_with_argument_source() -> &'static str {
    r#"
module bad_temperature_arity(p, n);
    inout p, n;
    electrical p, n;
    real temp;
    analog begin
        temp = $temperature(1.0) * V(p, n);
        I(p, n) <+ temp;
    end
endmodule
"#
}

fn compact_port_connected_without_argument_source() -> &'static str {
    r#"
module bad_port_connected_arity(p, n);
    inout p, n;
    electrical p, n;
    real connected;
    analog begin
        connected = $port_connected() * V(p, n);
        I(p, n) <+ connected;
    end
endmodule
"#
}
