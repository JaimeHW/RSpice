mod support;

use rspice_veriloga::{
    CodeGenerator, CompilerOptions, Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler,
};
use support::DeviceFixture;

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
    let model = VerilogACompiler::new(CompilerOptions::default())
        .compile_module(source, Some("parent"))
        .expect("declared child hierarchy must elaborate");
    assert_eq!(model.stamp_programs.len(), 1);
    assert_eq!(model.stamp_programs[0].value_program.instructions.len(), 3);
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
    let source_map = SourceMap::new();
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
        ("tri", "tri t;"),
        ("tri0", "tri0 t;"),
        ("tri1", "tri1 t;"),
        ("triand", "triand t;"),
        ("trior", "trior t;"),
        ("trireg", "trireg t;"),
        ("supply0", "supply0 vss;"),
        ("supply1", "supply1 vdd;"),
        ("time", "time t;"),
        ("realtime", "realtime t;"),
        ("event", "event e;"),
        ("edge", "edge e;"),
        ("scalared", "scalared b;"),
        ("vectored", "vectored b;"),
        ("task", "task report_it; endtask"),
        ("endtask", "endtask"),
        ("specify", "specify endspecify"),
        ("endspecify", "endspecify"),
    ] {
        assert_unsupported(
            &module_with_item(item),
            &format!("Verilog-AMS digital construct not yet supported: `{keyword}`"),
        );
    }

    // `defparam` is refused with its clause and its alternative rather than
    // with the bare keyword message. It is the one construct in this set an
    // author reaches for on purpose — it is the *other* spelling of a parameter
    // override — so "not supported" would leave nothing to do about it.
    assert_unsupported(
        &module_with_item("defparam u1.gain = 1;"),
        "`defparam`; IEEE 1364-2005 section 12.2.1",
    );

    // Procedural statements, inside the analog block. A digital construct has
    // no meaning in continuous-time code, so these do not retreat with the
    // module-item productions.
    for (keyword, statement) in [
        ("always", "always x = 1.0;"),
        ("reg", "reg r;"),
        ("wire", "wire w;"),
        ("wait", "wait (x) x = 1.0;"),
        ("casex", "casex (1) default: x = 1.0; endcase"),
        ("casez", "casez (1) default: x = 1.0; endcase"),
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
    parameter integer k = 1.5;
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
