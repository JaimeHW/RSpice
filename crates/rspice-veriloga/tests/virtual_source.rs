use rspice_veriloga::{
    CompileError, CompilerOptions, Preprocessor, SourceDocumentOrigin, VerilogACompiler,
    VirtualCompileLimits, VirtualSourceBundle, VirtualSourceError, VirtualSourceFile,
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
fn resolves_parent_relative_includes_inside_the_seal() {
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

    let compilation = VerilogACompiler::default()
        .compile_virtual_runtime(&bundle, "retained_model", VirtualCompileLimits::default())
        .expect("the module compiles from the sealed preprocessor graph");
    assert_eq!(compilation.selected_module, "retained_model");
    assert!(compilation.include_graph.iter().any(|edge| {
        edge.requested_path == "../shared/value.vh"
            && edge.included_path == "models/shared/value.vh"
    }));
    assert!(
        compilation
            .dependency_closure
            .iter()
            .any(|dependency| dependency.logical_path == "models/shared/value.vh")
    );
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
fn prepared_virtual_warnings_identify_original_includes() {
    let child = "module chatty(p,n);\ninout p,n; electrical p,n;\nanalog begin\n $display(\"ignored\");\n I(p,n) <+ V(p,n);\nend\nendmodule\n";
    let bundle = VirtualSourceBundle::from_sources(
        "root.va",
        [("root.va", "`include \"child.va\"\n"), ("child.va", child)],
    )
    .unwrap();
    // Discarding output tasks makes the included `$display` a warning.
    let prepared = VerilogACompiler::new(CompilerOptions {
        discard_output_tasks: true,
        ..CompilerOptions::default()
    })
    .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
    .unwrap();
    let [warning] = prepared.diagnostics() else {
        panic!("expected one compiler warning")
    };
    assert_eq!(warning.path.as_deref(), Some("child.va"));
    assert_eq!(warning.line, Some(4));
    assert_eq!(warning.column, Some(2));
    assert!(child[warning.byte_start.unwrap()..warning.byte_end.unwrap()].contains("$display"));
}

/// A module's variables read before they are assigned are one finding that
/// lists each at the line of its first such read in the file the author
/// wrote, on every path that reports it: the prepared source, the virtual
/// runtime report the workbench publishes, and an in-memory compile whose
/// header include moves every expanded line away from its authored one.
#[test]
fn retained_variables_are_listed_at_their_authored_lines() {
    const CODE: &str = "VA-SEM-RETAINED-VARIABLE";
    let root = "`include \"disciplines.vams\"\nmodule counter(p, n);\n  inout p, n; electrical p, n;\n  real count, held;\n  analog begin\n    count = count + 1;\n`include \"body.vams\"\n    I(p, n) <+ count * V(p, n) + held;\n  end\nendmodule\n";
    let body = "// hold the input while it is high\nif (V(p, n) > 0.5) held = held + V(p, n);\n";
    let listed = "Module `counter` has 2 variables read before they are assigned, so they keep \
                  their values from the last accepted time point: `count` (line 6), `held` \
                  (line 2 of body.vams)";
    let bundle =
        VirtualSourceBundle::from_sources("root.va", [("root.va", root), ("body.vams", body)])
            .unwrap();
    let compiler = VerilogACompiler::default();

    let prepared = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap();
    let [warning] = prepared.diagnostics() else {
        panic!("expected one compiler warning")
    };
    assert_eq!(warning.code, CODE);
    assert_eq!(warning.message, listed);
    assert_eq!(warning.path.as_deref(), Some("root.va"));
    assert_eq!(warning.line, Some(6));

    let compilation = compiler
        .compile_virtual_runtime(&bundle, "counter", VirtualCompileLimits::default())
        .unwrap();
    let reported: Vec<_> = compilation
        .runtime
        .diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.message.as_str()))
        .collect();
    assert_eq!(reported, [(CODE, listed)]);

    let single = root.replace(
        "`include \"body.vams\"\n",
        "    if (V(p, n) > 0.5) held = held + V(p, n);\n",
    );
    let report = compiler.compile_runtime(&single, Some("counter")).unwrap();
    let reported: Vec<_> = report
        .diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.message.as_str()))
        .collect();
    assert_eq!(
        reported,
        [(
            CODE,
            "Module `counter` has 2 variables read before they are assigned, so they keep \
             their values from the last accepted time point: `count` (line 6), `held` (line 7)"
        )]
    );
}

/// A virtual runtime compile places a module's processes, system calls and
/// analog tasks in the included file and on the line they were written, not
/// on a line of the expanded text under the bundle's root.
#[test]
fn virtual_runtime_places_processes_and_tasks_where_they_were_written() {
    let root = "`include \"disciplines.vams\"\n`include \"body.vams\"\n";
    let body = "module ticker(p);\n\
                inout p; electrical p;\n\
                reg q;\n\
                initial begin\n\
                #1 $display(\"tick\"); q = 1;\n\
                end\n\
                analog begin\n\
                $strobe(\"level\");\n\
                V(p) <+ 0;\n\
                end\n\
                endmodule\n";
    let bundle =
        VirtualSourceBundle::from_sources("root.vams", [("root.vams", root), ("body.vams", body)])
            .unwrap();
    let compilation = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
    .compile_virtual_runtime(&bundle, "ticker", VirtualCompileLimits::default())
    .unwrap();
    let runtime = &compilation.runtime;
    let digital = &runtime.canonical_ir.digital;
    let placed = |file: &str, line: u32| (file.ends_with("body.vams"), line);
    let processes: Vec<_> = digital
        .processes
        .iter()
        .map(|process| {
            let origin = process.origin.as_ref().expect("the process is placed");
            placed(&origin.file, origin.line)
        })
        .collect();
    let calls: Vec<_> = digital
        .system_calls
        .iter()
        .map(|call| {
            let origin = call.origin.as_ref().expect("the system call is placed");
            placed(&origin.file, origin.line)
        })
        .collect();
    let tasks: Vec<_> = runtime
        .model
        .analog_task_origins
        .iter()
        .map(|origin| placed(&origin.file, origin.line))
        .collect();
    assert_eq!(processes, [(true, 4)], "the initial is on body.vams:4");
    assert_eq!(calls, [(true, 5)], "the $display is on body.vams:5");
    assert_eq!(tasks, [(true, 8)], "the $strobe is on body.vams:8");
}

/// An included module with a parameter: a process on line 5, a digital
/// `$display` on line 6 and an analog `$strobe` on line 9 of the included
/// file.
const PLACED_BODY: &str = "module ticker(p);\n\
                           inout p; electrical p;\n\
                           parameter real k = 1.0;\n\
                           reg q;\n\
                           initial begin\n\
                           #1 $display(\"tick\"); q = 1;\n\
                           end\n\
                           analog begin\n\
                           $strobe(\"level\");\n\
                           V(p) <+ k * 0;\n\
                           end\n\
                           endmodule\n";

/// Where a report placed its processes, system calls and analog tasks:
/// whether each is in the file named `file`, and on which line.
fn placements_in(
    report: &rspice_veriloga::RuntimeCompileReport,
    file: &str,
) -> [Vec<(bool, u32)>; 3] {
    let placed = |written: &str, line: u32| (written.ends_with(file), line);
    let digital = &report.canonical_ir.digital;
    [
        digital
            .processes
            .iter()
            .map(|process| {
                let origin = process.origin.as_ref().expect("the process is placed");
                placed(&origin.file, origin.line)
            })
            .collect(),
        digital
            .system_calls
            .iter()
            .map(|call| {
                let origin = call.origin.as_ref().expect("the system call is placed");
                placed(&origin.file, origin.line)
            })
            .collect(),
        report
            .model
            .analog_task_origins
            .iter()
            .map(|origin| placed(&origin.file, origin.line))
            .collect(),
    ]
}

fn placements(report: &rspice_veriloga::RuntimeCompileReport) -> [Vec<(bool, u32)>; 3] {
    placements_in(report, "body.vams")
}

fn placed_in_body() -> [Vec<(bool, u32)>; 3] {
    [vec![(true, 5)], vec![(true, 6)], vec![(true, 9)]]
}

fn ams() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
}

/// Text compiled in memory places what it declares on the line it was
/// written, not on a line of the text the included standard header expanded
/// it into, as a prepared compile does. In-memory text includes no file of
/// its own, so the header is the include it can have.
#[test]
fn in_memory_runtime_places_processes_and_tasks_where_they_were_written() {
    let root = format!("`include \"disciplines.vams\"\n{PLACED_BODY}");
    let report = ams().compile_runtime(&root, Some("ticker")).unwrap();
    assert_eq!(
        placements_in(&report, "<input>"),
        [vec![(true, 6)], vec![(true, 7)], vec![(true, 10)]],
        "the module's lines follow the include on line 1"
    );
}

/// A specialization compiles its artifact's expanded text, and still places
/// every process, system call and analog task where the included file has it.
#[test]
fn a_specialized_module_keeps_the_places_of_its_included_sites() {
    let bundle = VirtualSourceBundle::from_sources(
        "root.vams",
        [
            (
                "root.vams",
                "`include \"disciplines.vams\"\n`include \"body.vams\"\n",
            ),
            ("body.vams", PLACED_BODY),
        ],
    )
    .unwrap();
    let compiler = ams();
    let original = compiler
        .compile_virtual_runtime(&bundle, "ticker", VirtualCompileLimits::default())
        .unwrap()
        .runtime;
    assert_eq!(placements(&original), placed_in_body());
    let mut specialized = compiler
        .specialize_mixed_runtime(
            &original.canonical_ir,
            &[("k", 2.0)],
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    println!(
        "ATTRIBUTE analog tasks as specialized: {:?}",
        placements(&specialized)[2]
    );
    specialized.model.locate_analog_tasks_as(&original.model);
    assert_eq!(placements(&specialized), placed_in_body());
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
