use rspice_veriloga::{
    CompileError, Preprocessor, SourceDocumentOrigin, VerilogACompiler, VirtualCompileLimits,
    VirtualSourceBundle, VirtualSourceError, VirtualSourceFile,
};

const TOP: &str = r#"`include "disciplines.vams"
`include "support/gain.vams"
module helper(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
module selected(p, n);
    inout p, n;
    electrical p, n;
    parameter real resistance = 1000.0;
    analog I(p, n) <+ `GAIN * V(p, n) / resistance;
endmodule
"#;

const GAIN: &str = "`define GAIN 2.0\n";

const BUILTIN_LIMIT_SOURCE: &str = r#"`include "disciplines.vams"
`include "constants.vams"
module selected(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
"#;

fn bundle_with_unused(unused: &str) -> VirtualSourceBundle {
    VirtualSourceBundle::new(
        "models/top.va",
        [
            VirtualSourceFile::new("models/top.va", TOP),
            VirtualSourceFile::new("models/support/gain.vams", GAIN),
            VirtualSourceFile::new("unused.va", unused),
        ],
    )
    .expect("valid virtual source bundle")
}

#[test]
fn discovers_modules_and_resolves_parent_relative_includes_inside_the_seal() {
    let root = r#"`include "../shared/value.vh"
`include "disciplines.vams"
module retained_model(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) / `RETAINED_R;
endmodule
"#;
    let bundle = VirtualSourceBundle::from_sources(
        "models/device/root.va",
        [
            ("models/device/root.va", root),
            ("models/shared/value.vh", "`define RETAINED_R 1k\n"),
        ],
    )
    .expect("parent-relative virtual bundle is structurally valid");

    let discovery = VerilogACompiler::default()
        .discover_virtual_modules(&bundle, VirtualCompileLimits::default())
        .expect("discovery uses the sealed preprocessor graph");
    assert_eq!(discovery.module_names, ["retained_model"]);
    assert!(discovery.include_graph.iter().any(|edge| {
        edge.requested_path == "../shared/value.vh"
            && edge.included_path == "models/shared/value.vh"
    }));

    VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, "retained_model", VirtualCompileLimits::default())
        .expect("the discovered module compiles from the identical closure");
}

#[test]
fn compiles_nested_virtual_and_builtin_dependencies_with_exact_receipt() {
    let bundle = bundle_with_unused("module unused; endmodule\n");
    let compilation = VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, "selected", VirtualCompileLimits::default())
        .expect("sealed bundle compiles");

    assert_eq!(compilation.selected_module, "selected");
    assert_eq!(compilation.runtime.model.name.as_str(), "selected");
    assert_eq!(compilation.runtime.abi.module_name.as_str(), "selected");
    assert_eq!(compilation.runtime.abi.analog_port_count(), 2);
    assert_eq!(compilation.runtime.abi.parameters.len(), 1);
    assert_eq!(
        compilation
            .dependency_closure
            .iter()
            .map(|dependency| dependency.logical_path.as_str())
            .collect::<Vec<_>>(),
        [
            "models/top.va",
            "@rspice/stdlib/disciplines.vams",
            "models/support/gain.vams",
        ]
    );
    assert_eq!(compilation.dependency_closure[0].source, TOP);
    assert_eq!(compilation.dependency_closure[2].source, GAIN);
    assert_eq!(
        compilation.dependency_closure[1].origin,
        SourceDocumentOrigin::BuiltIn
    );
    assert_eq!(
        compilation.dependency_closure[2].origin,
        SourceDocumentOrigin::Provider
    );
    assert_eq!(compilation.dependency_closure[0].content_digest.len(), 64);
    assert_eq!(compilation.include_graph.len(), 2);
    assert_eq!(compilation.include_graph[0].including_path, "models/top.va");
    assert_eq!(
        compilation.include_graph[0].included_path,
        "@rspice/stdlib/disciplines.vams"
    );
    assert_eq!(compilation.include_graph[0].include_index, 0);
    assert_eq!(compilation.include_graph[1].include_index, 1);
    assert_eq!(compilation.source_bundle_identity.len(), 64);
    assert_eq!(compilation.dependency_closure_identity.len(), 64);
    assert_eq!(compilation.compiler_contract_identity.len(), 64);
    assert_eq!(compilation.runtime_contract_identity.len(), 64);
    compilation
        .validate_integrity()
        .expect("published contract revalidates");

    let mut corrupted = compilation.clone();
    corrupted.dependency_closure[0].content_digest = "0".repeat(64);
    let error = corrupted
        .validate_integrity()
        .expect_err("corrupted dependency digest must fail");
    assert!(error.to_string().contains("content digest mismatch"));

    let mut corrupted = compilation.clone();
    corrupted.source_bundle_identity = "0".repeat(64);
    let error = corrupted
        .validate_integrity()
        .expect_err("corrupted bundle identity must fail");
    assert!(
        error
            .to_string()
            .contains("source bundle identity mismatch")
    );

    let mut corrupted = compilation.clone();
    corrupted.include_graph[1].requested_path = "different.va".to_owned();
    let error = corrupted
        .validate_integrity()
        .expect_err("tampered include graph must fail");
    assert!(
        error
            .to_string()
            .contains("dependency closure identity mismatch")
    );

    let mut corrupted = compilation;
    corrupted.compiler_contract_identity = "0".repeat(64);
    let error = corrupted
        .validate_integrity()
        .expect_err("corrupted compiler identity must fail");
    assert!(
        error
            .to_string()
            .contains("compiler contract identity mismatch")
    );
}

#[test]
fn included_file_diagnostic_retains_its_exact_path_content_and_location() {
    let child = "module selected(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ @;\nendmodule\n";
    let bundle = VirtualSourceBundle::from_sources(
        "top.va",
        [("top.va", "`include \"child.va\"\n"), ("child.va", child)],
    )
    .expect("valid diagnostic fixture");

    let failure = VerilogACompiler::default()
        .compile_virtual_runtime_diagnosed(&bundle, "selected", VirtualCompileLimits::default())
        .expect_err("invalid included source must fail");
    let diagnostic = failure
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.logical_path.as_deref() == Some("child.va"))
        .expect("diagnostic maps to included source");

    assert_eq!(diagnostic.source.as_deref(), Some(child));
    assert_eq!(diagnostic.line, Some(4));
    let range = diagnostic.byte_start.expect("source byte start")
        ..diagnostic.byte_end.expect("source byte end");
    assert!(range.end <= child.len());
    assert!(child[range].contains('@'));
}

#[test]
fn include_graph_contains_only_conditionally_active_resolutions() {
    let bundle = VirtualSourceBundle::from_sources(
        "top.va",
        [
            (
                "top.va",
                "`ifdef NEVER\n`include \"inactive.va\"\n`endif\n`include \"active.va\"\n",
            ),
            ("inactive.va", "this source must remain unreachable\n"),
            (
                "active.va",
                "module selected(p, n); inout p, n; electrical p, n; analog I(p, n) <+ V(p, n); endmodule\n",
            ),
        ],
    )
    .expect("valid conditional fixture");

    let compilation = VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, "selected", VirtualCompileLimits::default())
        .expect("active include compiles");

    assert_eq!(compilation.include_graph.len(), 1);
    assert_eq!(compilation.include_graph[0].requested_path, "active.va");
    // The retained directive index is syntactic, so it points at the second
    // include even though the first branch was inactive.
    assert_eq!(compilation.include_graph[0].include_index, 1);
    assert!(
        compilation
            .dependency_closure
            .iter()
            .all(|dependency| dependency.logical_path != "inactive.va")
    );
}

#[test]
fn source_only_preprocessing_keeps_builtins_out_of_filesystem_dependency_paths() {
    let mut preprocessor = Preprocessor::new();
    preprocessor
        .preprocess_source("`include \"disciplines.vams\"\n")
        .expect("built-in header preprocesses");
    assert!(preprocessor.dependencies().is_empty());
    let documents = preprocessor.dependency_documents();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].origin, SourceDocumentOrigin::BuiltIn);
    assert_eq!(
        documents[0].logical_path.to_string_lossy(),
        "@rspice/stdlib/disciplines.vams"
    );
}

#[test]
fn compiler_owned_builtins_do_not_consume_project_file_budget() {
    let bundle = VirtualSourceBundle::from_sources("top.va", [("top.va", BUILTIN_LIMIT_SOURCE)])
        .expect("single-file source bundle");
    let limits = VirtualCompileLimits {
        max_files: 1,
        ..VirtualCompileLimits::default()
    };

    let compilation = VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, "selected", limits)
        .expect("compiler-owned headers fit outside the one-file project budget");

    assert_eq!(compilation.dependency_closure.len(), 3);
    assert_eq!(
        compilation
            .dependency_closure
            .iter()
            .filter(|dependency| dependency.origin == SourceDocumentOrigin::BuiltIn)
            .count(),
        2
    );
}

#[test]
fn compiler_owned_builtins_do_not_consume_project_source_byte_budget() {
    let bundle = VirtualSourceBundle::from_sources("top.va", [("top.va", BUILTIN_LIMIT_SOURCE)])
        .expect("single-file source bundle");
    let limits = VirtualCompileLimits {
        max_total_source_bytes: BUILTIN_LIMIT_SOURCE.len(),
        ..VirtualCompileLimits::default()
    };

    VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, "selected", limits)
        .expect("compiler-owned headers fit outside the exact project byte budget");
}

#[test]
fn builtin_reservation_cannot_be_spent_by_project_sources() {
    let root = "`include \"extra.va\"\nmodule selected; endmodule\n";
    let bundle = VirtualSourceBundle::from_sources(
        "top.va",
        [("top.va", root), ("extra.va", "// project dependency\n")],
    )
    .expect("two-file source bundle");
    let compiler = VerilogACompiler::default();

    let file_error = compiler
        .compile_virtual_runtime(
            &bundle,
            "selected",
            VirtualCompileLimits {
                max_files: 1,
                ..VirtualCompileLimits::default()
            },
        )
        .expect_err("project files must remain bounded by max_files");
    assert!(file_error.to_string().contains("contains 2 files"));

    let byte_error = compiler
        .compile_virtual_runtime(
            &bundle,
            "selected",
            VirtualCompileLimits {
                max_total_source_bytes: root.len() + "// project dependency\n".len() - 1,
                ..VirtualCompileLimits::default()
            },
        )
        .expect_err("project bytes must remain bounded by max_total_source_bytes");
    assert!(byte_error.to_string().contains("virtual bundle contains"));
}

#[test]
fn identities_are_stable_and_unused_documents_do_not_change_runtime_contract() {
    let compiler = VerilogACompiler::default();
    let first_bundle = bundle_with_unused("module unused; endmodule\n");
    let second_bundle = bundle_with_unused("module unused_changed; endmodule\n");
    let first = compiler
        .compile_virtual_runtime(&first_bundle, "selected", VirtualCompileLimits::default())
        .expect("first compile");
    let repeated = compiler
        .compile_virtual_runtime(&first_bundle, "selected", VirtualCompileLimits::default())
        .expect("repeated compile");
    let unused_changed = compiler
        .compile_virtual_runtime(&second_bundle, "selected", VirtualCompileLimits::default())
        .expect("compile with unused source changed");

    assert_eq!(
        first.source_bundle_identity,
        repeated.source_bundle_identity
    );
    assert_eq!(
        first.compiler_contract_identity,
        repeated.compiler_contract_identity
    );
    assert_eq!(
        first.runtime_contract_identity,
        repeated.runtime_contract_identity
    );
    assert_ne!(
        first.source_bundle_identity,
        unused_changed.source_bundle_identity
    );
    assert_eq!(
        first.dependency_closure_identity,
        unused_changed.dependency_closure_identity
    );
    assert_eq!(
        first.runtime_contract_identity,
        unused_changed.runtime_contract_identity
    );
}

#[test]
fn explicit_selection_resolves_multi_module_source_and_missing_selection_fails() {
    let bundle = bundle_with_unused("module unused; endmodule\n");
    let compiler = VerilogACompiler::default();
    compiler
        .compile_virtual_runtime(&bundle, "helper", VirtualCompileLimits::default())
        .expect("explicit helper selection");

    let empty = compiler
        .compile_virtual_runtime(&bundle, "", VirtualCompileLimits::default())
        .expect_err("empty selection must fail");
    assert!(empty.to_string().contains("must not be empty"), "{empty}");

    let missing = compiler
        .compile_virtual_runtime(&bundle, "missing", VirtualCompileLimits::default())
        .expect_err("unknown module must fail");
    assert!(
        missing.to_string().contains("module 'missing' not found"),
        "{missing}"
    );
}

#[test]
fn rejects_nonportable_and_case_colliding_logical_paths() {
    for path in [
        "",
        "/absolute.va",
        "C:\\absolute.va",
        "../escape.va",
        "models/../escape.va",
        "models//empty.va",
        "models/./dot.va",
        "nul\0path.va",
    ] {
        let error = VirtualSourceBundle::new(
            path,
            [VirtualSourceFile::new(path, "module x; endmodule\n")],
        )
        .expect_err("unsafe path must fail");
        assert!(
            matches!(
                error,
                VirtualSourceError::EmptyPath
                    | VirtualSourceError::NulPath(_)
                    | VirtualSourceError::AbsolutePath(_)
                    | VirtualSourceError::TraversalPath(_)
            ),
            "unexpected error for {path:?}: {error}"
        );
    }

    let duplicate = VirtualSourceBundle::new(
        "Models/Top.va",
        [
            VirtualSourceFile::new("Models/Top.va", "module x; endmodule\n"),
            VirtualSourceFile::new("models/top.VA", "module y; endmodule\n"),
        ],
    )
    .expect_err("case-insensitive duplicate must fail");
    assert!(matches!(
        duplicate,
        VirtualSourceError::CaseInsensitiveDuplicate { .. }
    ));
}

#[test]
fn rejects_missing_traversing_absolute_and_cyclic_includes() {
    let compiler = VerilogACompiler::default();
    for include in [
        "missing.va",
        "../escape.va",
        "/absolute.va",
        "C:\\absolute.va",
    ] {
        let source = format!(
            "`include \"{include}\"\nmodule selected(p, n); inout p,n; electrical p,n; analog I(p,n) <+ V(p,n); endmodule\n"
        );
        let bundle = VirtualSourceBundle::from_sources("top.va", [("top.va", source)])
            .expect("root path itself is valid");
        let error = compiler
            .compile_virtual_runtime(&bundle, "selected", VirtualCompileLimits::default())
            .expect_err("invalid or missing include must fail");
        let message = error.to_string();
        assert!(
            message.contains("Include file not found")
                || message.contains("virtual include")
                || message.contains("absolute"),
            "unexpected error for {include:?}: {message}"
        );
    }

    let cycle = VirtualSourceBundle::from_sources(
        "top.va",
        [
            (
                "top.va",
                "`include \"other.va\"\nmodule selected; endmodule\n",
            ),
            ("other.va", "`include \"top.va\"\n"),
        ],
    )
    .expect("structurally valid cycle bundle");
    let error = compiler
        .compile_virtual_runtime(&cycle, "selected", VirtualCompileLimits::default())
        .expect_err("cycle must fail");
    assert!(error.to_string().contains("Circular include"), "{error}");
}

#[test]
fn enforces_bundle_file_depth_and_expansion_bounds() {
    let bundle = bundle_with_unused("module unused; endmodule\n");
    let compiler = VerilogACompiler::default();

    let limits = VirtualCompileLimits {
        max_files: 2,
        ..VirtualCompileLimits::default()
    };
    let error = compiler
        .compile_virtual_runtime(&bundle, "selected", limits)
        .expect_err("file-count limit must fail");
    assert!(matches!(&error, CompileError::VirtualSource(_)));
    assert!(error.to_string().contains("contains 3 files"), "{error}");

    let limits = VirtualCompileLimits {
        max_file_bytes: 8,
        ..VirtualCompileLimits::default()
    };
    let error = compiler
        .compile_virtual_runtime(&bundle, "selected", limits)
        .expect_err("per-file limit must fail");
    assert!(error.to_string().contains("per-file limit"), "{error}");

    let depth_bundle = VirtualSourceBundle::from_sources(
        "top.va",
        [
            (
                "top.va",
                "`include \"one.va\"\nmodule selected; endmodule\n",
            ),
            ("one.va", "`include \"two.va\"\n"),
            ("two.va", "// leaf\n"),
        ],
    )
    .expect("depth fixture");
    let limits = VirtualCompileLimits {
        max_include_depth: 2,
        ..VirtualCompileLimits::default()
    };
    let error = compiler
        .compile_virtual_runtime(&depth_bundle, "selected", limits)
        .expect_err("depth limit must fail");
    assert!(error.to_string().contains("Include depth"), "{error}");

    let expansion_bundle = VirtualSourceBundle::from_sources(
        "top.va",
        [(
            "top.va",
            "`define MANY xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n`MANY\nmodule selected; endmodule\n",
        )],
    )
    .expect("expansion fixture");
    let limits = VirtualCompileLimits {
        max_expanded_bytes: 48,
        ..VirtualCompileLimits::default()
    };
    let error = compiler
        .compile_virtual_runtime(&expansion_bundle, "selected", limits)
        .expect_err("expanded-source limit must fail");
    assert!(error.to_string().contains("provider limit"), "{error}");
}
