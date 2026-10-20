mod support;

use rspice_veriloga::{
    CodeGenerator, CompilerOptions, Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler,
};
use support::DeviceFixture;

#[test]
fn analog_function_argument_types_merge_in_either_declaration_order() {
    for declarations in [
        "input x; integer x;",
        "integer x; input x;",
        "input integer x; integer x;",
    ] {
        let fixture = DeviceFixture::compile(&format!(
            "module ftypes(p,n); inout p,n; electrical p,n; analog function real f; {declarations} f=x; endfunction analog I(p,n)<+f(V(p,n)); endmodule"
        ));
        let mut device = fixture.device("X", &[1, 0]);
        device.update_voltages(&[1.5]);
        assert_eq!(device.try_evaluate().unwrap()[0], 2.0, "{declarations}");
    }
    for declarations in [
        "input real x; integer x;",
        "input x; input x;",
        "input x; real x; integer x;",
    ] {
        assert!(
            VerilogACompiler::default()
                .compile(&format!(
                    "module bad; analog function real f; {declarations} f=x; endfunction endmodule"
                ))
                .is_err(),
            "{declarations}"
        );
    }
}

fn assert_unsupported(source: &str, expected_context: &str) {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect_err("unsupported syntax must not compile by being discarded");
    let message = error.to_string();
    assert!(
        message.contains(expected_context),
        "expected diagnostic containing {expected_context:?}, got {message:?}"
    );
}

#[test]
fn unsupported_module_instances_are_not_silently_discarded() {
    assert_unsupported(
        r#"
`include "disciplines.vams"
module unsupported_instance(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        "Undefined module: 'child'",
    );
}

#[test]
fn declared_module_hierarchy_executes_child_behavior() {
    let source = r#"
`include "disciplines.vams"
module child(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 2.0 * V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
endmodule
"#;
    let report = VerilogACompiler::new(CompilerOptions::default())
        .compile_runtime(source, Some("parent"))
        .expect("declared child hierarchy must elaborate");
    let fixture = DeviceFixture {
        model: report.model,
        canonical_ir: report.canonical_ir,
    };
    assert_eq!(fixture.stamp_programs.len(), 1);
    let mut device = fixture.device("X", &[1, 0]);
    for voltage in [-0.5, 0.0, 0.75] {
        let mut conductance = 0.0;
        let mut rhs = 0.0;
        device
            .try_stamp(
                &[voltage],
                |row, col, value| {
                    assert_eq!((row, col), (0, 0));
                    conductance += value;
                },
                |row, value| {
                    assert_eq!(row, 0);
                    rhs += value;
                },
            )
            .unwrap();
        assert_eq!((conductance, rhs), (2.0, 0.0));
    }
}

#[test]
fn direct_bytecode_generation_uses_the_same_hierarchy_elaborator() {
    let source = r#"
module child(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
endmodule
"#;
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source("<direct-codegen>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let parsed = Parser::new(&tokens).parse().unwrap();
    let analyzed = SemanticAnalyzer::new().analyze(&parsed).unwrap();

    let model = CodeGenerator::new()
        .generate_module(&analyzed, Some("parent"))
        .expect("public bytecode generation must elaborate hierarchy");
    assert_eq!(model.stamp_programs.len(), 1);
    assert_eq!(model.num_terminals, 2);
}

/// A digital declaration that now parses is still not silently discarded: the
/// refusal moved from the parser to the backend boundary, where the compiler
/// would have to execute it.
#[test]
fn unsupported_keyword_module_items_are_not_silently_discarded() {
    assert_unsupported(
        &module_with_item("wire hidden;"),
        "Verilog-AMS digital construct `wire` has no executable form in this compiler yet",
    );
}

fn module_with_item(item: &str) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         module digital_item(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   real x;\n\
         \x20   {item}\n\
         \x20   analog I(p, n) <+ V(p, n);\n\
         endmodule\n"
    )
}

fn module_with_statement(statement: &str) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         module digital_statement(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   real x;\n\
         \x20   analog begin\n\
         \x20       x = 1.0;\n\
         \x20       {statement}\n\
         \x20       I(p, n) <+ x * V(p, n);\n\
         \x20   end\n\
         endmodule\n"
    )
}

/// The digital half of Verilog-AMS that this compiler still has no grammar for
/// is refused by name, at the keyword that opens the construct.
///
/// Before this pin, `always @(posedge clk)` reached the parser as an ordinary
/// identifier and died as an unrecognized module item, blaming the wrong thing;
/// several other digital keywords lexed but had no production at all. Every one
/// of them stops on itself with a construct-specific diagnostic.
///
/// The list shrinks as the digital front end grows. `always`, `initial`, `reg`,
/// `wire`, and `assign` have module-item productions now and are pinned by
/// `tests/digital_grammar.rs` instead — they parse, resolve, and are refused at
/// the backend. They stay listed below in *statement* position, because a
/// digital construct still cannot appear inside an `analog` block.
#[test]
fn verilog_ams_digital_constructs_are_refused_by_name() {
    // Declarations, in module-item position.
    for (keyword, item) in [
        // `wreal` itself has a production now (Verilog-AMS LRM 2.4 section
        // 3.7) and is pinned by `tests/digital_grammar.rs`. `wreal4state` does
        // not: a real net that can also hold `x` and `z` is a different type
        // with no Accellera definition, so it stays refused by name.
        ("wreal4state", "wreal4state w;"),
        ("wand", "wand w;"),
        ("wor", "wor w;"),
        ("tri0", "tri0 t;"),
        ("tri1", "tri1 t;"),
        ("triand", "triand t;"),
        ("trior", "trior t;"),
        ("trireg", "trireg t;"),
        ("supply0", "supply0 vss;"),
        ("supply1", "supply1 vdd;"),
        // `time`, `realtime`, `event` and `task` have productions now (IEEE
        // 1364-2005 sections 4.8, 9.7.3 and 10.2), pinned by
        // `tests/digital_grammar.rs`.
        ("edge", "edge e;"),
        ("scalared", "scalared b;"),
        ("vectored", "vectored b;"),
        ("endtask", "endtask"),
        ("specify", "specify endspecify"),
        ("endspecify", "endspecify"),
    ] {
        assert_unsupported(
            &module_with_item(item),
            &format!("Verilog-AMS digital construct not yet supported: `{keyword}`"),
        );
    }

    // Procedural statements, inside the analog block. A digital construct has
    // no meaning in continuous-time code, so these do not retreat with the
    // module-item productions.
    for (keyword, statement) in [
        ("always", "always x = 1.0;"),
        ("reg", "reg r;"),
        ("wire", "wire w;"),
        ("wait", "wait (x) x = 1.0;"),
        ("fork", "fork x = 1.0; join"),
        ("join", "join"),
        ("assign", "assign x = 1.0;"),
        ("deassign", "deassign x;"),
        ("force", "force x = 1.0;"),
        ("release", "release x;"),
    ] {
        assert_unsupported(
            &module_with_statement(statement),
            &format!("Verilog-AMS digital construct not yet supported: `{keyword}`"),
        );
    }

    // `casez` and `casex` are analog statements in Verilog-AMS, so they are
    // refused as unimplemented there, not as digital constructs.
    for keyword in ["casex", "casez"] {
        assert_unsupported(
            &module_with_statement(&format!("{keyword} (1) default: x = 1.0; endcase")),
            &format!(
                "Unsupported analog statement: `{keyword}`: Verilog-AMS permits it in an \
                 analog block, but this compiler implements only `case` there"
            ),
        );
    }

    // File-scope items. `connectmodule` used to be here and is not any more:
    // Verilog-AMS LRM 2.4 Syntax 7-4 makes it a `module_keyword` and the
    // parser now reads one, so its refusal moved to the connect specification
    // machinery, where a module that does not bridge two domains is named for
    // what is wrong with it rather than for its keyword.
    for (keyword, source) in [
        (
            "primitive",
            "primitive latch(q, clk, d);\n    output q;\nendprimitive\n",
        ),
        ("endprimitive", "endprimitive\n"),
    ] {
        assert_unsupported(
            source,
            &format!("Verilog-AMS digital construct not yet supported: `{keyword}`"),
        );
    }
}

/// Reserving the digital keywords must not retract identifier space that
/// accepted sources already use.
///
/// The keywords that already had token kinds (`wire`, `force`, `assign`, ...)
/// have always doubled as ordinary names, because a Verilog-A source may
/// legitimately declare `real force;`. They are read as their construct only
/// where an assignment cannot start, so a variable named after one still
/// declares, assigns, and evaluates exactly as before.
#[test]
fn pre_existing_keywords_remain_usable_as_variable_names() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module keyword_named_variables(p, n);
    inout p, n;
    electrical p, n;
    real force;
    real wire;
    analog begin
        force = 2.0;
        wire = 3.0;
        I(p, n) <+ force * wire * V(p, n);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[1.0]);
    assert_eq!(
        device.try_evaluate().expect("keyword-named variables"),
        vec![6.0]
    );
}

/// `analog final` parses into its own block that nothing downstream reads.
///
/// Accepting the module would compile a device whose end-of-analysis behavior
/// the author wrote and the simulator never runs.
#[test]
fn analog_final_blocks_are_refused_instead_of_being_dropped() {
    assert_unsupported(
        r#"
`include "disciplines.vams"
module dropped_final(p, n);
    inout p, n;
    electrical p, n;
    real total;
    analog final begin
        total = 1.0;
    end
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        "`analog final` is parsed but never executed",
    );
}

/// The `analog final` refusal is recorded, not returned, so it cannot mask
/// the other diagnostics the analyzer accumulates for the same module.
#[test]
fn analog_final_refusal_reports_alongside_other_semantic_errors() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"
module dropped_final_and_bad_default(p, n);
    inout p, n;
    electrical p, n;
    parameter integer k = 3e9;
    real total;
    analog final begin
        total = 1.0;
    end
    analog I(p, n) <+ k * V(p, n);
endmodule
"#,
        )
        .expect_err("both defects must refuse the module");
    let message = error.to_string();
    for needle in [
        "`analog final` is parsed but never executed",
        "default of parameter 'k'",
    ] {
        assert!(
            message.contains(needle),
            "expected diagnostic containing {needle:?}, got {message:?}"
        );
    }
}

/// Refusing bare `initial` and `analog final` must not disturb the block that
/// shares their keywords and does have a consumer.
#[test]
fn analog_initial_still_runs_before_the_analog_block() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module seeded(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog initial gain = 4.0;
    analog I(p, n) <+ gain * V(p, n);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[1.5]);
    assert_eq!(
        device
            .try_evaluate()
            .expect("analog initial seeds the gain"),
        vec![6.0]
    );
}

#[test]
fn unsupported_nature_properties_are_not_silently_discarded() {
    assert_unsupported(
        r#"
nature custom_potential;
    access = CP;
    unsupported_property = 1;
endnature
module invalid_nature;
endmodule
"#,
        "Unsupported nature property: unsupported_property",
    );
}

#[test]
fn analog_functions_accept_direct_body_statements() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module direct_function_body(p, n);
    inout p, n;
    electrical p, n;

    analog function real square;
        input x;
        square = x * x;
    endfunction

    analog I(p, n) <+ square(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device.try_evaluate().expect("direct function evaluates"),
        vec![4.0]
    );
}

/// A module whose analog block holds `statement`, two statement levels deep
/// (the block is the first), with `x` holding the branch voltage.
fn nesting_module(statement: &str) -> String {
    format!(
        "module deep(p, n); inout p, n; electrical p, n; real x, y;\n\
         analog begin x = V(p, n); y = 0.0;\n{statement}\nI(p, n) <+ y * 1e-6; end\nendmodule\n"
    )
}

/// `levels` statement levels in all: `begin` blocks around one assignment.
fn statement_nesting(levels: usize) -> String {
    let blocks = levels - 2;
    nesting_module(&format!(
        "{}y = 2.0;{}",
        "begin ".repeat(blocks),
        " end".repeat(blocks)
    ))
}

/// `levels` expression levels in all: parentheses around one operand.
fn expression_nesting(levels: usize) -> String {
    let parentheses = levels - 1;
    nesting_module(&format!(
        "y = {}x{};",
        "(".repeat(parentheses),
        ")".repeat(parentheses)
    ))
}

/// An operator tree `depth` deep: a chain of `depth` operands.
fn operator_chain(depth: usize) -> String {
    nesting_module(&format!("y = {};", vec!["x"; depth].join(" + ")))
}

/// The nesting refusal for `source`: its message and byte offset.
fn nesting_refusal(source: &str) -> (String, usize) {
    match VerilogACompiler::default().compile(source).map(|_| ()) {
        Err(rspice_veriloga::CompileError::Parser(error)) => {
            assert_eq!(error.kind.code(), "VA-PARSE-NESTING-LIMIT", "{error}");
            (error.kind.to_string(), error.span.start as usize)
        }
        other => panic!("expected a nesting refusal, got {other:?}"),
    }
}

/// Nesting past the front end's bounds is refused by name, at the construct
/// that crosses the bound.
///
/// Every phase after the parser recurses over the tree, and before these
/// bounds a deep enough source overflowed the stack of one of them and aborted
/// the process: 1000 nested parentheses in the parser, a 3000-branch `?:`
/// chain and 1000 nested `if`s in semantic analysis, a 20000-operand sum in a
/// later phase. Each shape below aborted `rspice compile-va` that way.
#[test]
fn nesting_past_the_front_end_bounds_is_refused_by_name() {
    use rspice_veriloga::parser::{
        MAX_EXPRESSION_DEPTH, MAX_EXPRESSION_NESTING, MAX_STATEMENT_NESTING,
    };

    let source = statement_nesting(MAX_STATEMENT_NESTING + 1);
    let (message, offset) = nesting_refusal(&source);
    assert!(
        message.contains(&format!(
            "Statement nesting exceeds the compiler's limit of {MAX_STATEMENT_NESTING} levels"
        )),
        "{message}"
    );
    assert_eq!(offset, source.find("y = 2.0").unwrap(), "{message}");

    let source = expression_nesting(MAX_EXPRESSION_NESTING + 1);
    let (message, offset) = nesting_refusal(&source);
    assert!(
        message.contains(&format!(
            "Expression nesting exceeds the compiler's limit of {MAX_EXPRESSION_NESTING} levels"
        )),
        "{message}"
    );
    assert_eq!(offset, source.find("(x)").unwrap() + 1, "{message}");

    let source = operator_chain(MAX_EXPRESSION_DEPTH + 1);
    let (message, offset) = nesting_refusal(&source);
    assert!(
        message.contains(&format!(
            "Operator nesting exceeds the compiler's limit of {MAX_EXPRESSION_DEPTH} levels"
        )),
        "{message}"
    );
    assert_eq!(offset, source.find("x + x").unwrap(), "{message}");

    // The shapes that overflowed, each far past its bound.
    let nested_ifs = (0..1000)
        .map(|i| format!("if (x > {i}) begin "))
        .collect::<String>()
        + "y = 1.0;"
        + &" end".repeat(1000);
    let else_ifs = "if (x > 0) y = 0.0;".to_owned()
        + &(1..3000)
            .map(|i| format!(" else if (x > {i}) y = {i}.0;"))
            .collect::<String>();
    let ternaries = format!(
        "y = {}0.0;",
        (0..3000)
            .map(|i| format!("(x < {i}.0) ? {i}.0 : "))
            .collect::<String>()
    );
    let negations = format!("y = {}x;", "-".repeat(3000));
    let sum = format!("y = {};", vec!["x"; 20000].join(" + "));
    for statement in [nested_ifs, else_ifs, ternaries, negations, sum] {
        nesting_refusal(&nesting_module(&statement));
    }
}

/// A source at every bound compiles, through the full runtime pipeline, from a
/// thread with a small stack.
///
/// The bounds keep the tree small enough for the compiler's own stack, not the
/// caller's: the compile runs on a thread the compiler sizes, so a caller on a
/// spawned thread or a Windows main thread (1 MiB) still gets a diagnostic or
/// a model, never a stack overflow.
#[test]
fn sources_at_the_nesting_bounds_compile_from_a_small_stack() {
    use rspice_veriloga::parser::{
        MAX_EXPRESSION_DEPTH, MAX_EXPRESSION_NESTING, MAX_STATEMENT_NESTING,
    };

    let sources = [
        statement_nesting(MAX_STATEMENT_NESTING),
        expression_nesting(MAX_EXPRESSION_NESTING),
        operator_chain(MAX_EXPRESSION_DEPTH),
    ];
    std::thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(move || {
            for source in &sources {
                VerilogACompiler::default()
                    .compile_runtime(source, None)
                    .unwrap_or_else(|error| panic!("{error}"));
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

/// A misspelled directive stops the compile where it is written, and the IEEE
/// 1364 directives with no analog effect compile with a warning at each one.
///
/// Before, `` `inculde "opts.vams" `` compiled clean: the preprocessor left the
/// unknown name in the text and the parser dropped it with its line, so the
/// model silently lost whatever the include defined.
#[test]
fn directive_typos_are_refused_and_ignorable_directives_warn() {
    let typo = "`inculde \"opts.vams\"\nmodule t(p, n); inout p, n; electrical p, n;\n\
                analog I(p, n) <+ V(p, n);\nendmodule\n";
    let error = VerilogACompiler::default()
        .compile(typo)
        .map(|_| ())
        .expect_err("a misspelled `include must not compile");
    assert!(
        error
            .to_string()
            .contains("1:1: unknown compiler directive `inculde (did you mean `include?)"),
        "{error}"
    );

    let ignorable = "`celldefine\nmodule c(p, n); inout p, n; electrical p, n;\n\
                     `pragma vendor setting\nanalog begin\n`line 1 \"c.v\" 0\n\
                     I(p, n) <+ V(p, n);\nend\nendmodule\n`endcelldefine\n";
    let report = VerilogACompiler::default()
        .compile_runtime(ignorable, None)
        .unwrap_or_else(|error| panic!("{error}"));
    let warned: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "VA-PARSE-IGNORED-DIRECTIVE")
        .map(|diagnostic| {
            let start = diagnostic.span.as_ref().and_then(|span| span.start);
            (start.map(|start| start.line), diagnostic.message.as_str())
        })
        .collect();
    assert_eq!(
        warned,
        [
            (
                Some(1),
                "compiler directive `celldefine has no effect in this compiler and is ignored"
            ),
            (
                Some(3),
                "compiler directive `pragma has no effect in this compiler and is ignored"
            ),
            (
                Some(5),
                "compiler directive `line has no effect in this compiler and is ignored"
            ),
            (
                Some(9),
                "compiler directive `endcelldefine has no effect in this compiler and is ignored"
            ),
        ]
    );
}

/// Parse errors name tokens as they are written and name the mistakes a
/// Verilog-A author makes most, at the place to fix.
///
/// Before, each of these leaked the parser's own token names: a missing `end`
/// read "Expected Assign_, found Eof", `V(p,n) = 1;` read "Expected Assign_,
/// found LParen", `x <+ 1;` read "Expected Assign_, found Contribute", and an
/// extra `end` read "Unsupported module item: end".
#[test]
fn parse_errors_name_tokens_as_source_text() {
    let module = |body: &str| {
        format!("module m(p, n); inout p, n; electrical p, n; real x;\n{body}\nendmodule\n")
    };
    for (source, at, message) in [
        (
            module("analog begin\nI(p, n) <+ V(p, n);"),
            "begin",
            "'begin' has no matching 'end'; 'endmodule' comes first",
        ),
        (
            "module m(p, n); inout p, n; electrical p, n;\nanalog I(p, n) <+ V(p, n);\n".to_owned(),
            "module",
            "module 'm' has no matching 'endmodule'; end of file comes first",
        ),
        (
            module("analog V(p, n) = 1;"),
            "= 1",
            "'=' after the access function V(...): use '<+' to contribute to a branch",
        ),
        (
            module("analog begin x <+ 1; I(p, n) <+ x; end"),
            "<+ 1",
            "'<+' after 'x', which is not an access function",
        ),
        (
            module("analog begin I(p, n) <+ V(p, n); end\nend"),
            "end\nendmodule",
            "'end' with no 'begin' open",
        ),
        (
            module("analog begin x = 1\nI(p, n) <+ x; end"),
            "I(p, n) <+ x",
            "Expected ';', found 'I'",
        ),
        (
            module("analog begin if (V(p, n) > 0 begin I(p, n) <+ 1; end end"),
            "begin I(",
            "Expected ')', found 'begin'",
        ),
    ] {
        let error = match VerilogACompiler::default().compile(&source).map(|_| ()) {
            Err(rspice_veriloga::CompileError::Parser(error)) => error,
            other => panic!("{source}: expected a parse error, got {other:?}"),
        };
        let rendered = error.to_string();
        assert!(rendered.contains(message), "{source}: {rendered}");
        assert_eq!(
            error.span.start as usize,
            source.find(at).unwrap(),
            "{source}: {rendered}"
        );
    }
}

/// A preprocessing refusal from an in-memory compile is reported as the
/// preprocessor states it, under its own code; it used to read "I/O error:
/// Preprocessor error: ..." with the code of a file-system failure.
#[test]
fn an_in_memory_preprocessing_refusal_is_not_framed_as_an_io_error() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(&module_with_statement("x = `NOT_DEFINED;"))
        .expect_err("an undefined macro is refused");
    let message = error.to_string();
    assert!(
        message.contains("undefined macro `NOT_DEFINED"),
        "{message}"
    );
    assert!(!message.contains("I/O error"), "{message}");
    assert!(!message.contains("Preprocessor error"), "{message}");
    assert_eq!(error.diagnostic_code(), "VA-INPUT-PREPROCESS");
}
