//! The IEEE 1364-2005 digital subset that Verilog-AMS embeds.
//!
//! Frontend acceptance, elaboration, and diagnostics for unsupported constructs.
//! Process execution is covered separately in `digital_process_execution.rs`.

use rspice_veriloga::ast::*;
use rspice_veriloga::four_state::{FourStateBit, LiteralBase};
use rspice_veriloga::semantic::AnalyzedFile;
use rspice_veriloga::{
    CompileError, CompilerOptions, Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler,
};

fn parse(source: &str) -> SourceFile {
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source("<digital>", source);
    let tokens = Lexer::new(source, source_id)
        .collect_tokens()
        .unwrap_or_else(|error| panic!("lexing failed: {error}"));
    Parser::new(&tokens)
        .parse()
        .unwrap_or_else(|error| panic!("parsing failed: {error}"))
}

fn parse_error(source: &str) -> String {
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source("<digital>", source);
    let tokens = match Lexer::new(source, source_id).collect_tokens() {
        Ok(tokens) => tokens,
        Err(error) => return error.to_string(),
    };
    Parser::new(&tokens)
        .parse()
        .expect_err("source must not parse")
        .to_string()
}

/// Parse and analyze, which together are what a digital module must pass.
fn analyze(source: &str) -> AnalyzedFile {
    SemanticAnalyzer::new()
        .analyze(&parse(source))
        .unwrap_or_else(|error| panic!("semantic analysis failed: {error}"))
}

fn analyze_error(source: &str) -> String {
    SemanticAnalyzer::new()
        .analyze(&parse(source))
        .expect_err("semantic analysis must refuse this module")
        .to_string()
}

/// The message a full compile produces, which is where the backend refusal
/// lives.
fn compile_error(source: &str, module: Option<&str>) -> CompileError {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_module(source, module)
        .expect_err("a module with digital content must not compile")
}

/// The one module a single-module fixture parsed to.
fn parsed_module(file: &SourceFile) -> &Module {
    let Item::Module(module) = &file.items[0] else {
        panic!("expected a module as the first item");
    };
    module
}

fn only_module(analyzed: &AnalyzedFile) -> &rspice_veriloga::semantic::AnalyzedModule {
    assert_eq!(analyzed.modules.len(), 1, "expected exactly one module");
    analyzed.modules.values().next().expect("one module")
}

/// A module whose ports and analog body are fixed, so a test can vary only the
/// digital section it is about.
fn digital_module(section: &str) -> String {
    format!(
        "module dut(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   real gain;\n\
         {section}\n\
         \x20   analog I(p, n) <+ gain * V(p, n);\n\
         endmodule\n"
    )
}

// ===========================================================================
// Declarations
// ===========================================================================

/// IEEE 1364-2005 sections 3.9 and 4.2.1: `reg` and net declarations take an
/// optional signedness qualifier and an optional packed range, in that order.
#[test]
fn digital_declarations_resolve_their_shape() {
    let analyzed = analyze(&digital_module(
        "    wire w;\n\
         \x20   wire [7:0] bus;\n\
         \x20   wire signed [15:0] sbus;\n\
         \x20   reg q;\n\
         \x20   reg [3:0] nibble;\n\
         \x20   reg signed [31:0] counter;\n\
         \x20   reg [0:7] reversed;\n\
         \x20   integer plain;",
    ));
    let module = only_module(&analyzed);
    let shapes: Vec<String> = module
        .digital
        .signals
        .iter()
        .map(|signal| {
            format!(
                "{}{} {} {} width={}",
                signal.class.keyword(),
                if signal.signedness.is_signed() {
                    " signed"
                } else {
                    ""
                },
                signal
                    .range
                    .map_or_else(|| "scalar".to_string(), |range| range.spelling()),
                signal.name,
                signal.width,
            )
        })
        .collect();

    assert_eq!(
        shapes,
        vec![
            "wire scalar w width=1",
            "wire [7:0] bus width=8",
            "wire signed [15:0] sbus width=16",
            "reg scalar q width=1",
            "reg [3:0] nibble width=4",
            "reg signed [31:0] counter width=32",
            // A descending range is a different declaration, not a
            // normalization of the ascending one.
            "reg [0:7] reversed width=8",
        ]
    );
    // `integer` keeps its continuous-domain declaration: IEEE 1364-2005
    // section 3.9 gives it neither a range nor a signedness qualifier, so the
    // digital grammar needed nothing new for it.
    assert!(
        module
            .variables
            .iter()
            .any(|variable| variable.name == "plain"),
        "`integer` must still declare an ordinary module variable"
    );
}

/// IEEE 1364-2005 section 12.3.3: a port declaration carries the same
/// signedness and range as a net declaration.
#[test]
fn port_declarations_carry_vector_shape() {
    let source = "module bus_port(a, b, c);\n\
                  \x20   input [7:0] a;\n\
                  \x20   input signed [15:0] b;\n\
                  \x20   output c;\n\
                  \x20   electrical a, b, c;\n\
                  endmodule\n";
    let file = parse(source);
    let Item::Module(module) = &file.items[0] else {
        panic!("expected a module");
    };
    let shapes: Vec<(&str, bool, bool)> = module
        .port_declarations
        .iter()
        .map(|declaration| {
            (
                declaration.names[0].as_str(),
                declaration.signedness.is_signed(),
                declaration.range.is_some(),
            )
        })
        .collect();
    assert_eq!(
        shapes,
        vec![("a", false, true), ("b", true, true), ("c", false, false)]
    );
}

/// IEEE 1364-2005 section 12.3.4: `output q; reg q;` is how a port becomes
/// procedurally assignable, so the second declaration re-types the port
/// instead of colliding with it.
#[test]
fn an_output_port_may_be_redeclared_as_a_register() {
    let analyzed = analyze(
        "module port_reg(clk, q);\n\
         \x20   input clk;\n\
         \x20   output q;\n\
         \x20   wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= 1'b1;\n\
         endmodule\n",
    );
    let module = only_module(&analyzed);
    let signal = module
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == "q")
        .unwrap();
    assert_eq!(signal.name, "q");
    assert!(signal.redeclares_port);
}

/// IEEE 1364-2005 section 12.3.3 declares a port as a list of plain
/// identifiers, so a port is never an unpacked array: not through its port
/// declaration, and not through the net or variable declaration that completes
/// it, at a root or in a child. An RSpice-authored probe has Icarus `-g2005`
/// reject the ANSI and non-ANSI forms, at a root and in a child, with "Ports
/// cannot be unpacked arrays".
#[test]
fn an_unpacked_port_array_is_a_language_error() {
    for (form, source) in [
        ("ANSI net", "module top(input wire a [0:1]); endmodule\n"),
        (
            "ANSI variable",
            "module top(output reg q [0:1]); endmodule\n",
        ),
        (
            "ANSI inherited name",
            "module top(input wire b, a [0:1]); endmodule\n",
        ),
        (
            "ANSI integer",
            "module top(output integer q [0:1]); endmodule\n",
        ),
        (
            "port declaration",
            "module top(a); input a [0:1]; endmodule\n",
        ),
        (
            "net redeclaration",
            "module top(a); input a; wire a [0:1]; endmodule\n",
        ),
        (
            "variable redeclaration",
            "module top(q); output q; reg q [0:1]; endmodule\n",
        ),
        (
            "integer redeclaration",
            "module top(q); output q; integer q [0:1]; initial q[0] = 1; endmodule\n",
        ),
        (
            "child",
            "module leaf(a, y); input a; output y; wire a [0:1]; endmodule\n\
             module top; wire w [0:1]; wire y; leaf u(w, y); endmodule\n",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(source, Some("top"))
            .expect_err(form);
        let message = error.to_string();
        assert!(
            message.contains("unpacked array") && message.contains("12.3.3"),
            "{form}: {message}"
        );
        assert!(
            !message.contains("Unsupported"),
            "{form} is a language error, not a refusal: {message}"
        );
        assert!(
            error.diagnostic_code().ends_with("-UNPACKED-PORT-ARRAY"),
            "{form}: {}",
            error.diagnostic_code()
        );
    }
}

/// IEEE 1364-2005 section 12.3.3 lets only an output port be a variable, and
/// the rule is the module's own: a root answers to it as a child does. An
/// RSpice-authored probe has Icarus `-g2005` reject the non-ANSI forms at a
/// root ("declared as input and as a reg type"). Icarus accepts the ANSI
/// `input reg` spelling and reads the port as a net, a form the standard's
/// grammar (IEEE 1364-2005 A.2.1.2, `input [net_type] ...`) does not have.
#[test]
fn a_root_input_port_may_not_be_a_variable() {
    for (form, source) in [
        ("input reg", "module top(a); input a; reg a; endmodule\n"),
        ("inout reg", "module top(a); inout a; reg a; endmodule\n"),
        ("ANSI input reg", "module top(input reg a); endmodule\n"),
        ("ANSI inout reg", "module top(inout reg a); endmodule\n"),
        ("ANSI input real", "module top(input real a); endmodule\n"),
        (
            "input integer",
            "module top(a); input a; integer a; initial a = 1; endmodule\n",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(source, Some("top"))
            .expect_err(form);
        let message = error.to_string();
        assert!(
            message.contains("only an output port be a variable") && message.contains("12.3.3"),
            "{form}: {message}"
        );
        assert_eq!(
            error.diagnostic_code(),
            "VA-SEM-VARIABLE-INPUT-PORT",
            "{form}: {message}"
        );
    }
    // An output port is the one a variable may complete.
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            "module top(q); output q; reg q; initial q = 1'b1; endmodule\n",
            Some("top"),
        )
        .expect("an output port may be a reg");
}

/// IEEE 1364-2005 section 12.3.3 declares every port of the list of ports
/// `input`, `output` or `inout`. A variable declaration does not give a port
/// a direction: an RSpice-authored probe has Icarus `-g2005` reject
/// `module m(q); integer q;` with "Port q (1) of module m has no direction
/// declaration".
#[test]
fn a_port_an_integer_completes_still_needs_a_direction() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            "module m(q); integer q; initial q = 5; endmodule\n",
            Some("m"),
        )
        .expect_err("a port with no direction");
    let message = error.to_string();
    assert!(
        message.contains("Port 'q' has no direction declaration") && message.contains("12.3.3"),
        "{message}"
    );
    assert_eq!(
        error.diagnostic_code(),
        "VA-SEM-PORT-WITHOUT-DIRECTION",
        "{message}"
    );
}

#[test]
fn a_digital_name_may_not_collide_with_an_analog_one() {
    let message = analyze_error(&digital_module("    reg gain;"));
    assert!(
        message.contains("Duplicate symbol: 'gain'"),
        "expected a duplicate-symbol diagnostic, got {message:?}"
    );
}

/// Bounds must be resolvable at compile time, because a declared width is the
/// storage a later wave allocates.
#[test]
fn vector_bounds_must_be_constant_and_bounded() {
    let message = analyze_error(&digital_module(
        "    reg [gain:0] bad;\n\
         \x20   analog gain = 3.0;",
    ));
    assert!(
        message.contains("`reg` vector bounds must be compile-time constants"),
        "expected a constant-bounds diagnostic, got {message:?}"
    );

    for range in [
        "100000:0",
        "4294967296:0",
        "0:4294967296",
        "-9.223372036854776e18:0",
    ] {
        let message = analyze_error(&digital_module(&format!("reg [{range}] enormous;")));
        assert!(
            message.contains("this compiler supports at most 65536 bits per signal"),
            "expected a width-limit diagnostic for {range}, got {message:?}"
        );
    }
}

/// An unpacked array's limit is on entries, whatever each entry's width; it
/// was refused as a "vector" of "65537 bits", and every later use of the
/// array added "Undeclared symbol: 'm'".
#[test]
fn an_unpacked_array_past_the_entry_limit_is_refused_once_in_entries() {
    let message = analyze_error(&digital_module(
        "    reg [7:0] m [0:65536];\n\
         \x20   initial begin m[0] = 8'h5a; m[1] = m[0]; end",
    ));
    assert!(
        message.contains("unpacked array `m` [0:65536] has 65537 entries")
            && message.contains("at most 65536 entries per array")
            && !message.contains("bits")
            && !message.contains("Undeclared"),
        "{message}"
    );
}

#[test]
fn digital_bounds_outside_the_index_range_are_refused_before_conversion() {
    for range in [
        "1e30:1e30",
        "-1e30:-1e30",
        "9.223372036854776e18:9.223372036854776e18",
        "-1e30:1e30",
    ] {
        let message = analyze_error(&digital_module(&format!("reg [{range}] invalid;")));
        assert!(
            message.contains("signed 64-bit index range"),
            "{range}: {message}"
        );
    }
}

#[test]
fn integer_range_expressions_keep_bits_beyond_f64_precision() {
    let source = digital_module("reg [9007199254740992+1:9007199254740992] exact;");
    let analyzed = analyze(&source);
    let signal = &only_module(&analyzed).digital.signals[0];
    let bounds = signal.range.expect("packed range");
    assert_eq!(
        (bounds.msb, bounds.lsb),
        (9_007_199_254_740_993, 9_007_199_254_740_992)
    );
    assert_eq!(bounds.width(), 2);
}

#[test]
fn vector_range_arithmetic_never_wraps() {
    use rspice_veriloga::canonical_ir::VectorBounds;
    for bounds in [
        VectorBounds {
            msb: 4_294_967_296,
            lsb: 0,
        },
        VectorBounds {
            msb: i64::MIN,
            lsb: i64::MAX,
        },
        VectorBounds {
            msb: i64::MAX,
            lsb: i64::MIN,
        },
    ] {
        assert_eq!(bounds.width(), u32::MAX);
    }
    for bounds in [
        VectorBounds {
            msb: i64::MAX,
            lsb: i64::MAX - 3,
        },
        VectorBounds {
            msb: i64::MIN + 3,
            lsb: i64::MIN,
        },
        VectorBounds {
            msb: i64::MIN,
            lsb: i64::MIN + 3,
        },
        VectorBounds {
            msb: i64::MAX - 3,
            lsb: i64::MAX,
        },
    ] {
        assert_eq!(bounds.width(), 4);
        for position in 0..4 {
            let index = bounds.index_at(position);
            assert!(bounds.contains(index));
            assert_eq!(bounds.position_of(index), position);
        }
        let indices = bounds.indices_msb_first().collect::<Vec<_>>();
        assert_eq!(indices.first(), Some(&bounds.msb));
        assert_eq!(indices.last(), Some(&bounds.lsb));
        for index in [i64::MIN, i64::MAX] {
            let position = bounds.position_of(index);
            assert_eq!(bounds.contains(index), (0..4).contains(&position));
        }
    }
}

// ===========================================================================
// Processes and event controls
// ===========================================================================

/// IEEE 1364-2005 sections 9.7.4 and 9.9: an `always` process opens with its
/// event control, and the tree keeps it there rather than hoisting a
/// sensitivity list out of the body.
#[test]
fn processes_keep_their_event_control_as_the_body_opener() {
    let analyzed = analyze(&digital_module(
        "    wire clk, rst, d;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk or negedge rst) q <= d;",
    ));
    let module = only_module(&analyzed);
    let process = &module.digital.processes[0];
    assert_eq!(process.kind, DigitalProcessKind::Always);
    assert!(!process.implicit_sensitivity);

    let sensitivity = process
        .sensitivity
        .as_ref()
        .expect("an explicit event control has a static sensitivity list");
    let terms: Vec<(&str, Option<EdgeKind>)> = sensitivity
        .iter()
        .map(|term| (term.signal.as_str(), term.edge))
        .collect();
    assert_eq!(
        terms,
        vec![
            ("clk", Some(EdgeKind::Posedge)),
            ("rst", Some(EdgeKind::Negedge)),
        ]
    );

    // The body still *is* the timing control, so a process IR reads the
    // suspension point from the shape the author wrote.
    let DigitalStatement::Timing(timing) = &process.body else {
        panic!("process body must be the timing control it opens with");
    };
    assert!(matches!(timing.control, TimingControl::Event(_)));
    assert!(timing.statement.is_some());
}

/// IEEE 1364-2005 section 9.7.4: `,` and `or` are synonyms in an event
/// expression, and a term with no edge is level-sensitive.
#[test]
fn sensitivity_lists_accept_commas_or_and_level_terms() {
    for separator in [",", " or "] {
        let analyzed = analyze(&digital_module(&format!(
            "    wire a, b;\n\
             \x20   reg y;\n\
             \x20   always @(a{separator}b) y = a;"
        )));
        let module = only_module(&analyzed);
        let terms: Vec<(&str, Option<EdgeKind>)> = module.digital.processes[0]
            .sensitivity
            .as_ref()
            .expect("explicit list")
            .iter()
            .map(|term| (term.signal.as_str(), term.edge))
            .collect();
        assert_eq!(terms, vec![("a", None), ("b", None)], "for {separator:?}");
    }
}

/// IEEE 1364-2005 section 9.7.5: `@*` and `@(*)` are the implicit sensitivity
/// list. The list is not materialized here — computing it means reading the
/// guarded statement, and a stale copy would be worse than none.
#[test]
fn implicit_sensitivity_lists_parse_in_both_spellings() {
    for spelling in ["@*", "@(*)"] {
        let analyzed = analyze(&digital_module(&format!(
            "    wire a, b;\n\
             \x20   reg y;\n\
             \x20   always {spelling} y = a & b;"
        )));
        let process = &only_module(&analyzed).digital.processes[0];
        assert!(
            process.implicit_sensitivity,
            "{spelling} must record an implicit sensitivity list"
        );
        assert!(
            process.sensitivity.is_none(),
            "{spelling} has no static term list"
        );
    }
}

#[test]
fn delay_controls_and_initial_processes_parse() {
    let analyzed = analyze(&digital_module(
        "    reg q;\n\
         \x20   initial begin\n\
         \x20       q = 1'b0;\n\
         \x20       #5 q = 1'b1;\n\
         \x20   end",
    ));
    let process = &only_module(&analyzed).digital.processes[0];
    assert_eq!(process.kind, DigitalProcessKind::Initial);
    // An `initial` process runs once, so it needs no timing control; the delay
    // is inside the block rather than at the opening.
    assert!(process.sensitivity.is_none());
    assert!(!process.implicit_sensitivity);
}

/// Processes are identified in declaration order, so a later pass can name one
/// without owning the container it came from.
#[test]
fn process_identity_follows_declaration_order_and_restarts_per_module() {
    let file = parse(
        "module first(clk);\n\
         \x20   input clk;\n\
         \x20   electrical clk;\n\
         \x20   reg a, b;\n\
         \x20   always @(clk) a = 1'b1;\n\
         \x20   initial b = 1'b0;\n\
         \x20   always @(clk) b = a;\n\
         endmodule\n\
         module second(clk);\n\
         \x20   input clk;\n\
         \x20   electrical clk;\n\
         \x20   reg c;\n\
         \x20   always @(clk) c = 1'b1;\n\
         endmodule\n",
    );
    let ids: Vec<Vec<u32>> = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Module(module) => Some(
                module
                    .digital_processes
                    .iter()
                    .map(|process| process.id.0)
                    .collect(),
            ),
            _ => None,
        })
        .collect();
    assert_eq!(ids, vec![vec![0, 1, 2], vec![0]]);
}

/// IEEE 1364-2005 section 9.9.2: an `always` process restarts as soon as it
/// finishes, so one with no timing control never suspends and simulation time
/// cannot advance past it.
#[test]
fn an_always_process_without_timing_control_is_refused() {
    let message = analyze_error(&digital_module(
        "    reg q;\n\
         \x20   always q = 1'b1;",
    ));
    assert!(
        message.contains("never suspends") && message.contains("@(posedge clk)"),
        "expected a suspension diagnostic naming the fix, got {message:?}"
    );
}

#[test]
fn an_empty_event_control_is_refused_with_the_implicit_alternative() {
    let message = parse_error(&digital_module(
        "    reg q;\n\
         \x20   always @() q = 1'b1;",
    ));
    assert!(
        message.contains("needs at least one term") && message.contains("`@*`"),
        "expected an empty-event-control diagnostic, got {message:?}"
    );
}

#[test]
fn computed_event_expression_with_no_inputs_is_legal() {
    let analyzed = analyze(&digital_module("reg q; always @(1 + 2) q = 1'b1;"));
    assert!(
        only_module(&analyzed).digital.processes[0]
            .sensitivity
            .is_none()
    );
}

// ===========================================================================
// Statements
// ===========================================================================

/// Behavioral statements include runtime-selected procedural bit targets.
#[test]
fn every_wave_one_behavioral_statement_parses() {
    let analyzed = analyze(&digital_module(
        "    wire clk, sel;\n\
         \x20   reg [3:0] q;\n\
         \x20   integer i;\n\
         \x20   always @(posedge clk) begin : body\n\
         \x20       if (sel) q = 4'b0001; else q = 4'b0010;\n\
         \x20       case (q)\n\
         \x20           4'b0001, 4'b0010: q = 4'b0100;\n\
         \x20           default: q = 4'b0000;\n\
         \x20       endcase\n\
         \x20       casez (q)\n\
         \x20           4'b1???: q = 4'b0001;\n\
         \x20           default: q = 4'b0010;\n\
         \x20       endcase\n\
         \x20       casex (q)\n\
         \x20           4'b1xx0: q = 4'b0011;\n\
         \x20           default: q = 4'b0100;\n\
         \x20       endcase\n\
         \x20       for (i = 0; i < 4; i = i + 1) q[i] = 1'b0;\n\
         \x20       while (i > 0) i = i - 1;\n\
         \x20       repeat (3) i = i + 1;\n\
         \x20       ;\n\
         \x20   end",
    ));
    let process = &only_module(&analyzed).digital.processes[0];
    let DigitalStatement::Timing(timing) = &process.body else {
        panic!("process opens with its event control");
    };
    let Some(statement) = &timing.statement else {
        panic!("the event control guards the block");
    };
    let DigitalStatement::Block(block) = statement.as_ref() else {
        panic!("expected a named block");
    };
    assert_eq!(block.name.as_deref(), Some("body"));

    let kinds: Vec<&str> = block
        .statements
        .iter()
        .map(|statement| match statement {
            DigitalStatement::Conditional(_) => "if",
            DigitalStatement::Case(case) => case.kind.keyword(),
            DigitalStatement::For(_) => "for",
            DigitalStatement::While(_) => "while",
            DigitalStatement::Repeat(_) => "repeat",
            DigitalStatement::Forever(_) => "forever",
            DigitalStatement::Null(_) => "null",
            DigitalStatement::BlockingAssign(_) => "=",
            DigitalStatement::NonblockingAssign(_) => "<=",
            DigitalStatement::Timing(_) => "timing",
            DigitalStatement::Block(_) => "block",
            DigitalStatement::SystemTask(_) => "system task",
            DigitalStatement::TaskEnable(_) => "task",
            DigitalStatement::Fork(_) => "fork",
            DigitalStatement::Disable(_) => "disable",
            DigitalStatement::Wait(_) => "wait",
            DigitalStatement::EventTrigger(_) => "->",
            DigitalStatement::ProceduralContinuous(statement) => statement.kind.keyword(),
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            "if", "case", "casez", "casex", "for", "while", "repeat", "null"
        ]
    );
}

#[test]
fn forever_loops_parse_around_a_suspension_point() {
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always forever @(posedge clk) q <= ~q;",
    ));
    let process = &only_module(&analyzed).digital.processes[0];
    assert!(matches!(process.body, DigitalStatement::Forever(_)));
    // The suspension point is inside the loop, which is what keeps the process
    // from spinning.
    assert!(process.body.contains_timing_control());
}

/// IEEE 1364-2005 section 9.2: `=` and `<=` are different statements, and the
/// tree keeps them apart rather than normalizing one into the other.
#[test]
fn blocking_and_nonblocking_assignments_stay_distinct() {
    let analyzed = analyze(&digital_module(
        "    wire clk, d;\n\
         \x20   reg q, r;\n\
         \x20   always @(posedge clk) begin\n\
         \x20       r = d;\n\
         \x20       q <= r;\n\
         \x20       q <= #5 r;\n\
         \x20   end",
    ));
    let process = &only_module(&analyzed).digital.processes[0];
    let DigitalStatement::Timing(timing) = &process.body else {
        panic!("process opens with its event control");
    };
    let Some(DigitalStatement::Block(block)) = timing.statement.as_deref() else {
        panic!("expected a block");
    };
    assert!(matches!(
        block.statements[0],
        DigitalStatement::BlockingAssign(_)
    ));
    assert!(matches!(
        block.statements[1],
        DigitalStatement::NonblockingAssign(_)
    ));
    // Intra-assignment timing control (IEEE 1364-2005 section 9.2.2) belongs to
    // the assignment, not to a wrapper statement.
    let DigitalStatement::NonblockingAssign(delayed) = &block.statements[2] else {
        panic!("expected a nonblocking assignment");
    };
    assert!(matches!(delayed.timing, Some(TimingControl::Delay(_))));
}

#[test]
fn continuous_assignments_parse_with_targets_and_delays() {
    let analyzed = analyze(&digital_module(
        "    wire a, b;\n\
         \x20   wire y, z;\n\
         \x20   assign y = a & b;\n\
         \x20   assign #3 z = a | b;",
    ));
    let module = only_module(&analyzed);
    let targets: Vec<&str> = module
        .digital
        .continuous_assigns
        .iter()
        .map(|assignment| assignment.target.as_str())
        .collect();
    assert_eq!(targets, vec!["y", "z"]);
    assert!(
        module.digital.continuous_assigns[1]
            .assignment
            .delay
            .is_some()
    );
}

/// IEEE 1364-2005 section 9.3: `force`, `release`, `assign` and `deassign`
/// are procedural statements; the two that install an override carry a
/// right-hand side, and a target may be a concatenation.
#[test]
fn procedural_continuous_assignments_parse() {
    let file = parse(&digital_module(
        "    wire clk;\n\
         \x20   wire [3:0] w;\n\
         \x20   reg q, r;\n\
         \x20   initial begin\n\
         \x20       force {w[1], q} = {clk, 1'b0};\n\
         \x20       release {w[1], q};\n\
         \x20       assign r = q;\n\
         \x20       deassign r;\n\
         \x20   end",
    ));
    let module = parsed_module(&file);
    let DigitalStatement::Block(block) = &module.digital_processes[0].body else {
        panic!("expected a block");
    };
    let statements: Vec<(&str, bool, usize)> = block
        .statements
        .iter()
        .map(|statement| match statement {
            DigitalStatement::ProceduralContinuous(statement) => (
                statement.kind.keyword(),
                statement.value.is_some(),
                statement.target.written_names().len(),
            ),
            other => panic!("unexpected statement {other:?}"),
        })
        .collect();
    assert_eq!(
        statements,
        [
            ("force", true, 2),
            ("release", false, 2),
            ("assign", true, 1),
            ("deassign", false, 1)
        ]
    );
    let message = parse_error(&digital_module("    reg q;\n    initial release q = 1;"));
    assert!(!message.contains("not yet supported"), "{message}");
}

/// The message lowering a module to canonical IR refuses it with.
fn lowering_error(source: &str) -> String {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(source, None)
        .expect_err("lowering must refuse this module")
        .to_string()
}

/// Section 9.3.1 limits a procedural `assign` to variables; section 9.3.2
/// lets `force` take a net, a constant select of a net, or a variable, but
/// never a select of a variable or a memory word.
#[test]
fn procedural_continuous_targets_follow_section_9_3() {
    let module = |body: &str| {
        format!(
            "module t;\n  wire [3:0] w;\n  reg [3:0] v;\n  reg [3:0] m [0:1];\n  \
             initial begin {body} end\nendmodule\n"
        )
    };
    let message = analyze_error(&module("assign w = 4'd1;"));
    assert!(message.contains("`w`, which is a `wire`"), "{message}");
    for (body, expected) in [
        (
            "force v[1] = 1'b1;",
            "a bit- or part-select of the variable `v`",
        ),
        ("force m[0] = 4'd1;", "a memory or one of its words"),
        ("release m[1];", "a memory or one of its words"),
    ] {
        let message = lowering_error(&module(body));
        assert!(message.contains(expected), "{body}: {message}");
    }
    for body in [
        "force w = 4'd1;",
        "force w[2] = 1'b1;",
        "force w[3:2] = 2'b10;",
        "force {w[0], v} = 5'h1f;",
        "assign v = w;",
        "release w[2]; deassign v;",
    ] {
        VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(&module(body), None)
            .unwrap_or_else(|error| panic!("{body}: {error}"));
    }
    let message = lowering_error(&module("force w = $random;"));
    assert!(message.contains("calls `$random`"), "{message}");
}

/// Section 10.2.1: a variable of an automatic task may be neither the target
/// nor an operand of a procedural continuous assignment, and section 10.4.4
/// keeps them out of functions altogether.
#[test]
fn procedural_continuous_assignments_refuse_automatic_variables_and_functions() {
    let message = analyze_error(
        "module t;\n  reg q;\n  task automatic f(input a);\n    force q = a;\n  endtask\n\
         initial f(1'b1);\nendmodule\n",
    );
    assert!(
        message.contains("a `force` or `release` cannot use `a`")
            && message.contains("automatic task `f`"),
        "{message}"
    );
    let message = analyze_error(
        "module t;\n  reg q;\n  task automatic f;\n    reg l;\n    assign l = q;\n  endtask\n\
         initial f;\nendmodule\n",
    );
    assert!(
        message.contains("a procedural `assign` or `deassign` cannot use `l`"),
        "{message}"
    );
    let message = analyze_error(
        "module t;\n  reg q;\n  function g(input a);\n    begin force q = a; g = a; end\n  \
         endfunction\n  initial q = g(1'b0);\nendmodule\n",
    );
    assert!(
        message.contains("a procedural continuous assignment"),
        "{message}"
    );
}

// ===========================================================================
// Assignment legality
// ===========================================================================

/// IEEE 1364-2005 section 6.2: a procedural assignment drives a variable and a
/// continuous assignment drives a net. Each violation names the declaration
/// that has to change.
#[test]
fn assignment_targets_follow_the_net_versus_variable_rule() {
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   wire y;\n\
         \x20   always @(posedge clk) y = 1'b1;",
    ));
    assert!(
        message.contains("`y`, which is a `wire`")
            && message.contains("declare it `reg`")
            && message.contains("continuous `assign`"),
        "expected a net-target diagnostic naming both fixes, got {message:?}"
    );

    let message = analyze_error(&digital_module(
        "    wire a;\n\
         \x20   reg y;\n\
         \x20   assign y = a;",
    ));
    assert!(
        message.contains("`y`, which is a `reg`")
            && message.contains("declare it `wire`")
            && message.contains("`always` process"),
        "expected a variable-target diagnostic naming both fixes, got {message:?}"
    );
}

/// A nonblocking assignment exists only inside a process. Writing one where a
/// continuous assignment belongs says so, instead of reporting a token
/// mismatch on `<=`.
#[test]
fn a_nonblocking_continuous_assignment_is_refused_with_the_reason() {
    let message = parse_error(&digital_module(
        "    wire a, y;\n\
         \x20   assign y <= a;",
    ));
    assert!(
        message.contains("nonblocking assignment is only legal inside")
            && message.contains("continuous `assign` uses `=`"),
        "expected a continuous-assignment diagnostic, got {message:?}"
    );
}

#[test]
fn undeclared_identifiers_inside_a_process_are_refused() {
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= missing;",
    ));
    assert!(
        message.contains("Undeclared symbol: 'missing'"),
        "expected an undeclared-symbol diagnostic, got {message:?}"
    );
}

/// Constant vector selects have the same clipped semantics as runtime selects.
/// Declaration direction and width are still validated separately.
#[test]
fn constant_vector_selects_allow_clipped_reads_and_writes() {
    for (declaration, target) in [
        ("reg [3:0] nibble;", "nibble[7]"),
        ("reg [7:4] high;", "high[3]"),
        ("reg [4:7] high;", "high[8]"),
        ("reg [7:4] high;", "high[5:2]"),
        ("reg [4:7] high;", "high[2:5]"),
        ("integer count;", "count[33:30]"),
    ] {
        let analyzed = analyze(&digital_module(&format!(
            "{declaration} reg [3:0] observed; initial begin observed={target}; {target}=4'b1010; end"
        )));
        assert_eq!(only_module(&analyzed).digital.processes.len(), 1);
    }

    let message = analyze_error(&digital_module("reg scalar; initial scalar[1]=1'b1;"));
    assert!(
        message.contains("scalar has no selectable bits"),
        "expected a scalar-select diagnostic, got {message:?}"
    );
}

/// IEEE 1364-2005 section 4.2.1: a part select runs in the direction the
/// vector was declared in.
///
/// Both indices of `bus[0:3]` are inside a `reg [7:0]`, so the bounds check
/// passes it; what it is not is a part select of that vector. It was silently
/// read as `bus[3:0]` before this refusal, which is a wrong answer rather than
/// a missing one.
#[test]
fn a_part_select_must_run_in_the_declarations_direction() {
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg [7:0] bus;\n\
         \x20   reg [3:0] low;\n\
         \x20   always @(posedge clk) low <= bus[0:3];",
    ));
    assert!(
        message.contains(
            "part select `bus[0:3]` runs against the direction `bus` was declared \
                          in, `[7:0]`"
        ),
        "expected a part-select direction diagnostic, got {message:?}"
    );

    // The mirror image: an ascending declaration refuses the descending form.
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg [0:7] bus;\n\
         \x20   reg [3:0] low;\n\
         \x20   always @(posedge clk) bus[3:0] <= low;",
    ));
    assert!(
        message.contains(
            "part select `bus[3:0]` runs against the direction `bus` was declared \
                          in, `[0:7]`"
        ),
        "expected a part-select direction diagnostic, got {message:?}"
    );

    // A range that is not anchored at zero is judged by position and not by
    // which index is the larger number: `[4:7] q; q[5:6]` is in the declared
    // direction, and `q[6:5]` is not.
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg [4:7] q;\n\
         \x20   reg [1:0] middle;\n\
         \x20   always @(posedge clk) middle <= q[5:6];",
    ));
    assert_eq!(only_module(&analyzed).digital.signals.len(), 3);

    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg [4:7] q;\n\
         \x20   reg [1:0] middle;\n\
         \x20   always @(posedge clk) middle <= q[6:5];",
    ));
    assert!(
        message.contains(
            "part select `q[6:5]` runs against the direction `q` was declared in, \
                          `[4:7]`"
        ),
        "expected a part-select direction diagnostic, got {message:?}"
    );

    // One bit selected twice is not reversed, in either direction.
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg [4:7] q;\n\
         \x20   reg one;\n\
         \x20   always @(posedge clk) one <= q[6:6];",
    ));
    assert_eq!(only_module(&analyzed).digital.signals.len(), 3);
}

#[test]
fn part_selects_resolve_with_clipped_bounds() {
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg [7:0] bus;\n\
         \x20   reg [3:0] high;\n\
         \x20   always @(posedge clk) high <= bus[7:4];",
    ));
    assert_eq!(only_module(&analyzed).digital.signals.len(), 3);

    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg [7:0] bus;\n\
         \x20   reg [3:0] high;\n\
         \x20   always @(posedge clk) high <= bus[9:4];",
    ));
    assert_eq!(only_module(&analyzed).digital.signals.len(), 3);
}

/// The two halves of the language share one expression grammar, so a
/// continuous-domain form reaches a process's parser. Most of them have no
/// meaning there and stop by name.
///
/// A branch access is the exception and is tested separately below: Verilog-AMS
/// LRM 2.4 section 7.3.3 makes probing a continuous net from a discrete context
/// legal, and it is the one continuous-domain form a process is *supposed* to
/// contain.
#[test]
fn continuous_domain_expressions_are_refused_inside_a_process() {
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= (ddt(V(p, n)) > 0.0);",
    ));
    assert!(
        message.contains("call to `ddt` inside a discrete-domain expression"),
        "expected an analog-operator diagnostic, got {message:?}"
    );
}

/// Verilog-AMS LRM 2.4 section 7.3.3: "All continuous nets can be probed from
/// a discrete context using access functions."
///
/// The clause's own example is `always @(posedge clk) out = V(in);`, so this is
/// not a tolerated oddity — it is the construct that makes a module's two
/// halves one design rather than two that happen to share a file. Section 7.3
/// fixes the direction: reads cross, writes do not.
#[test]
fn a_process_may_probe_a_continuous_net() {
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg hi;\n\
         \x20   always @(posedge clk) hi <= (V(p, n) > 0.5);",
    ));
    assert_eq!(only_module(&analyzed).digital.processes.len(), 1);
}

/// Cross-domain reads include flow; a discrete signal is still not a
/// continuous net and cannot be passed to an analog access function.
#[test]
fn process_flow_probes_are_accepted_and_discrete_operands_are_refused() {
    let analyzed = analyze(&digital_module(
        "wire clk; reg hi; always @(posedge clk) hi <= (I(p,n)>0.5);",
    ));
    assert_eq!(only_module(&analyzed).digital.processes.len(), 1);

    // A discrete net has no potential to probe at all.
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg hi;\n\
         \x20   always @(posedge clk) hi <= (V(clk) > 0.5);",
    ));
    assert!(
        message.contains("which is a discrete-domain net and has no potential"),
        "expected a discrete-net diagnostic, got {message:?}"
    );
}

// ===========================================================================
// Four-state literals
// ===========================================================================

/// IEEE 1364-2005 section 3.5.1: a based literal may carry `x`, `z`, and `?`
/// digits, and a leading unknown extends leftward with itself.
#[test]
fn four_state_literals_decode_bit_by_bit() {
    let file = parse(&digital_module(
        "    wire clk;\n\
         \x20   reg [7:0] bus;\n\
         \x20   always @(posedge clk) bus <= 8'b1z0x_0011;",
    ));
    let Item::Module(module) = &file.items[0] else {
        panic!("expected a module");
    };
    let DigitalStatement::Timing(timing) = &module.digital_processes[0].body else {
        panic!("process opens with its event control");
    };
    let Some(DigitalStatement::NonblockingAssign(assign)) = timing.statement.as_deref() else {
        panic!("expected a nonblocking assignment");
    };
    let Expression::Digital(DigitalExpr::FourState(literal)) = &assign.value else {
        panic!("expected a four-state literal");
    };
    assert_eq!(literal.value.base, LiteralBase::Binary);
    assert_eq!(literal.value.declared_width, Some(8));
    assert!(!literal.value.signed);
    let spelled: String = literal
        .value
        .bits
        .iter()
        .map(|bit| bit.as_char())
        .collect::<String>();
    assert_eq!(spelled, "1z0x0011");
    assert!(literal.value.has_unknown_bits());
    assert_eq!(literal.value.bits[1], FourStateBit::HighImpedance);
    assert_eq!(literal.value.bits[3], FourStateBit::Unknown);
}

/// The refusal that used to live in the lexer now lives where it can name the
/// domain: the continuous half has no representation for `x` or `z`, so a
/// four-state literal in an analog expression stops with that reason.
#[test]
fn four_state_literals_are_refused_in_the_continuous_domain() {
    let message = analyze_error(
        "module analog_four_state(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   real gain;\n\
         \x20   analog begin\n\
         \x20       gain = 4'b10x1;\n\
         \x20       I(p, n) <+ gain * V(p, n);\n\
         \x20   end\n\
         endmodule\n",
    );
    assert!(
        message.contains("four-state literal")
            && message.contains("no value in the continuous (analog) domain")
            && message.contains("supported in discrete expressions"),
        "expected a domain diagnostic naming the construct and where it is legal, got {message:?}"
    );
}

#[test]
fn part_selects_of_analog_owned_arrays_are_refused() {
    let message = analyze_error(
        "module analog_part_select(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   real gain;\n\
         \x20   real bus[0:7];\n\
         \x20   analog begin\n\
         \x20       gain = bus[7:4];\n\
         \x20       I(p, n) <+ gain * V(p, n);\n\
         \x20   end\n\
         endmodule\n",
    );
    assert!(
        message.contains("packed analog read of `bus`")
            && message.contains("requires discrete storage"),
        "expected a part-select domain diagnostic, got {message:?}"
    );
}

/// The three operators this wave added are discrete-domain forms, so the
/// continuous domain refuses each by name rather than by token.
///
/// Before they lexed at all, `a ~^ b` died as an unexpected `~` and `a === b`
/// as an invalid expression — diagnostics that blamed a character rather than
/// the construct. What the analog half must say now is which operator it is and
/// that the discrete half is where it belongs.
#[test]
fn the_new_discrete_operators_are_refused_in_the_continuous_domain() {
    for (expression, construct) in [
        ("(a ~^ b)", "bitwise XNOR operator"),
        ("(a ^~ b)", "bitwise XNOR operator"),
        ("(a === b)", "case equality operator"),
        ("(a !== b)", "case equality operator"),
        ("(&a)", "reduction operator"),
        ("(~&a)", "reduction operator"),
        ("(|a)", "reduction operator"),
        ("(~|a)", "reduction operator"),
        ("(^a)", "reduction operator"),
        ("(~^a)", "reduction operator"),
        // `>>>` joined them for the same reason: its fill rule is IEEE
        // 1364-2005 section 4.1.12's, and the continuous domain shifts machine
        // integers through a bytecode instruction, a native encoding and a
        // WebAssembly opcode, none of which has one. `<<<` is absent because
        // the standard makes it `<<`, which the analog half already has.
        ("(a >>> b)", "arithmetic right shift operator"),
    ] {
        let message = analyze_error(&format!(
            "module analog_operator(p, n);\n\
             \x20   inout p, n;\n\
             \x20   electrical p, n;\n\
             \x20   integer a, b;\n\
             \x20   real gain;\n\
             \x20   analog begin\n\
             \x20       a = 1; b = 2;\n\
             \x20       gain = {expression};\n\
             \x20       I(p, n) <+ gain * V(p, n);\n\
             \x20   end\n\
             endmodule\n"
        ));
        assert!(
            message.contains(construct) && message.contains("continuous (analog) domain"),
            "expected `{expression}` to be refused as a {construct}, got {message:?}"
        );
    }
}

/// `~^` and `^~` are one operator with two spellings, sitting on XOR's tier of
/// IEEE 1364-2005 table 4-2: tighter than `|`, looser than `&`.
///
/// Pinned on the tree rather than on a value, because a precedence defect that
/// happens to produce the right answer for one operand pair is still a
/// precedence defect.
#[test]
fn xnor_parses_at_the_xor_precedence_tier() {
    for spelling in ["~^", "^~"] {
        let file = parse(&digital_module(&format!(
            "    wire a, b, c;\n\
         \x20   wire y;\n\
         \x20   assign y = a & b {spelling} c | a;"
        )));
        let module = parsed_module(&file);
        let value = &module.continuous_assigns[0].value;

        // The outermost operator is `|`, the loosest of the three.
        let Expression::Binary(or) = value else {
            panic!("expected `|` outermost, got {value:?}");
        };
        assert_eq!(or.op, BinaryOp::BitOr);

        // Its left operand is the XNOR, whose own left operand is the `&`.
        let Expression::Digital(DigitalExpr::Xnor(xnor)) = &*or.left else {
            panic!("expected XNOR under `|`, got {:?}", or.left);
        };
        let Expression::Binary(and) = &*xnor.left else {
            panic!("expected `&` under XNOR, got {:?}", xnor.left);
        };
        assert_eq!(and.op, BinaryOp::BitAnd);
    }
}

/// `===` and `!==` share the tier of `==` and `!=`, so a chain of them groups
/// left exactly as a chain of the logical forms does.
#[test]
fn case_equality_parses_on_the_equality_tier() {
    let file = parse(&digital_module(
        "    wire a, b, c;\n\
     \x20   wire y, z;\n\
     \x20   assign y = a === b == c;\n\
     \x20   assign z = a !== b;",
    ));
    let module = parsed_module(&file);

    // `a === b == c` is `(a === b) == c`: same tier, left associative.
    let Expression::Binary(outer) = &module.continuous_assigns[0].value else {
        panic!("expected `==` outermost");
    };
    assert_eq!(outer.op, BinaryOp::Eq);
    let Expression::Digital(DigitalExpr::CaseEquality(inner)) = &*outer.left else {
        panic!("expected `===` under `==`, got {:?}", outer.left);
    };
    assert!(!inner.negate);

    let Expression::Digital(DigitalExpr::CaseEquality(negated)) =
        &module.continuous_assigns[1].value
    else {
        panic!("expected `!==`");
    };
    assert!(negated.negate, "`!==` is `===` with the sense inverted");
}

/// Every reduction spelling parses to the operator it names, including over a
/// concatenation — the form that has no signal to bit-select out of.
#[test]
fn reduction_operators_parse_in_every_spelling() {
    for (source, expected) in [
        ("&a", ReductionOp::And),
        ("~&a", ReductionOp::Nand),
        ("|a", ReductionOp::Or),
        ("~|a", ReductionOp::Nor),
        ("^a", ReductionOp::Xor),
        ("~^a", ReductionOp::Xnor),
        ("^~a", ReductionOp::Xnor),
        ("^{a, b}", ReductionOp::Xor),
    ] {
        let file = parse(&digital_module(&format!(
            "    wire a, b;\n\
         \x20   wire y;\n\
         \x20   assign y = {source};"
        )));
        let value = &parsed_module(&file).continuous_assigns[0].value;
        let Expression::Digital(DigitalExpr::Reduction(reduction)) = value else {
            panic!("expected a reduction for `{source}`, got {value:?}");
        };
        assert_eq!(reduction.op, expected, "{source}");
    }
}

/// A `~` that is not followed by `&` or `|` is still the bitwise negation of
/// section 4.1.9, and a `&` that follows an operand is still the binary form.
///
/// The reduction operators reuse every one of their spellings from the binary
/// operators, so the only thing keeping them apart is position. This is the
/// pin on that: if the parser started reading `a & b` as `a` followed by a
/// reduction, or `~a` as a malformed reduction, the whole language would shift
/// underneath every existing model.
#[test]
fn the_binary_and_unary_readings_of_the_shared_spellings_survive() {
    let file = parse(&digital_module(
        "    wire [1:0] a, b;\n\
     \x20   wire [1:0] w, x;\n\
     \x20   wire y;\n\
     \x20   assign w = ~a;\n\
     \x20   assign x = a & b;\n\
     \x20   assign y = a & &b;",
    ));
    let assigns = &parsed_module(&file).continuous_assigns;

    let Expression::Unary(negation) = &assigns[0].value else {
        panic!("`~a` is a bitwise negation, got {:?}", assigns[0].value);
    };
    assert_eq!(negation.op, UnaryOp::BitNot);

    let Expression::Binary(and) = &assigns[1].value else {
        panic!("`a & b` is a binary AND, got {:?}", assigns[1].value);
    };
    assert_eq!(and.op, BinaryOp::BitAnd);

    // `a & &b` is a binary AND whose right operand is a reduction: the second
    // `&` is in a unary position and the first is not.
    let Expression::Binary(mixed) = &assigns[2].value else {
        panic!("`a & &b` is a binary AND, got {:?}", assigns[2].value);
    };
    assert_eq!(mixed.op, BinaryOp::BitAnd);
    let Expression::Digital(DigitalExpr::Reduction(reduction)) = &*mixed.right else {
        panic!("expected a reduction on the right, got {:?}", mixed.right);
    };
    assert_eq!(reduction.op, ReductionOp::And);
}

/// A malformed four-state literal still stops at the lexer, because there is
/// no value to carry forward.
#[test]
fn malformed_four_state_literals_fail_closed_at_the_lexer() {
    for (source, needle) in [
        ("4'd1x", "must be exactly one x or z digit"),
        ("0'bx", "has a zero width"),
        ("70000'bx", "materializes at most"),
    ] {
        let message = parse_error(&digital_module(&format!(
            "    wire clk;\n\
             \x20   reg q;\n\
             \x20   always @(posedge clk) q <= {source};"
        )));
        assert!(
            message.contains(needle),
            "expected {needle:?} for {source}, got {message:?}"
        );
    }
}

// ===========================================================================
// The backend boundary
// ===========================================================================

/// A module with digital content parses, resolves, and is then refused by name
/// wherever the compiler would have to execute it.
#[test]
fn digital_modules_are_refused_at_the_executable_boundary() {
    let source = digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= 1'b1;",
    );
    // Parse and semantic analysis accept it, which is the whole point: the
    // author gets real diagnostics for the digital source.
    let analyzed = analyze(&source);
    assert_eq!(only_module(&analyzed).digital.processes.len(), 1);

    let error = compile_error(&source, None);
    assert_eq!(
        error.diagnostic_code(),
        "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
    );
    let message = error.to_string();
    assert!(
        message.contains("Verilog-AMS digital construct `wire`")
            && message.contains("has no executable form in this compiler yet")
            && message.contains("Remove the digital section"),
        "expected a backend refusal naming the construct and the fix, got {message:?}"
    );
}

/// The refusal names whichever construct comes first in the source, so the
/// diagnostic points at the top of the digital section.
#[test]
fn the_backend_refusal_names_the_first_digital_construct() {
    for (section, keyword) in [
        ("    reg q;\n     always @(p) q <= 1'b1;", "reg"),
        ("    wire y;\n     assign y = 1'b1;", "wire"),
        (
            "    always @(p) begin : b reg r; r = 1'b1; end\n     reg q;",
            "always",
        ),
    ] {
        let error = compile_error(&digital_module(section), None);
        let message = error.to_string();
        assert!(
            message.contains(&format!("Verilog-AMS digital construct `{keyword}`")),
            "expected `{keyword}` to be named first, got {message:?}"
        );
    }
}

/// A child instance's digital content is refused too. Hierarchy elaboration
/// flattens a child into the parent, so without this the child's processes
/// would vanish into a device that compiles.
#[test]
fn digital_content_in_an_instantiated_child_is_refused() {
    let source = "module child(p, n);\n\
                  \x20   inout p, n;\n\
                  \x20   electrical p, n;\n\
                  \x20   reg q;\n\
                  \x20   always @(p) q <= 1'b1;\n\
                  \x20   analog I(p, n) <+ V(p, n);\n\
                  endmodule\n\
                  module parent(p, n);\n\
                  \x20   inout p, n;\n\
                  \x20   electrical p, n;\n\
                  \x20   child u1(p, n);\n\
                  endmodule\n";
    let error = compile_error(source, Some("parent"));
    let message = error.to_string();
    assert!(
        message.contains("digital instance `u1` of module `child`"),
        "expected the child's digital content to be named, got {message:?}"
    );
}

/// A module with no digital content is unaffected: the refusal is reachable
/// only from content the digital grammar produced.
#[test]
fn continuous_domain_modules_still_compile() {
    let model = VerilogACompiler::new(CompilerOptions::default())
        .compile(&digital_module("    // no digital section"))
        .expect("a continuous-domain module must still compile");
    assert_eq!(model.num_terminals, 2);
}

#[cfg(feature = "native")]
mod canonical_ir_boundary {
    use super::*;

    /// The canonical-IR path no longer refuses a process: it lowers one.
    ///
    /// This is where the boundary used to be. It moved outward once processes
    /// had a canonical form, and the artifact now carries the lowered process
    /// rather than the compiler declining to build one.
    #[test]
    fn canonical_ir_construction_lowers_digital_modules() {
        let source = digital_module(
            "    wire clk;\n\
             \x20   reg q;\n\
             \x20   always @(posedge clk) q <= 1'b1;",
        );
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect("canonical IR must lower a digital module");
        assert_eq!(artifact.digital.processes.len(), 1);
        assert_eq!(artifact.digital.signals.len(), 2);
    }

    /// A continuous assignment lowers to a driver process and a driver on the
    /// net, rather than being refused or -- worse -- dropped.
    #[test]
    fn canonical_ir_construction_lowers_a_continuous_assignment() {
        let source = digital_module(
            "    wire a, b;\n\
             \x20   wire y;\n\
             \x20   assign y = a & b;",
        );
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect("a continuous assignment must lower");
        assert_eq!(artifact.digital.processes.len(), 1);
        assert_eq!(artifact.digital.drivers.len(), 1);
        let driver = &artifact.digital.drivers[0];
        let y = artifact
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == "y")
            .expect("declared");
        assert_eq!(driver.id.signal, y.id);
        assert_eq!(driver.id.index, 0);
        assert_eq!(driver.process, artifact.digital.processes[0].id);
    }

    /// A delay on a driver (IEEE 1364-2005 sections 6.1.3 and 7.14) lowers to
    /// the plan's delay table, one delay per transition in design ticks: one
    /// value is every transition's, and with two, turning off takes the
    /// lesser. A net's own delay lowers beside it. A delay that reads a
    /// variable is refused, naming section 6.1.3.
    #[test]
    fn canonical_ir_construction_lowers_delayed_drivers_and_nets() {
        use rspice_veriloga::canonical_ir::digital::DigitalTransitionDelay;
        let source = digital_module(
            "    wire a;\n\
             \x20   wire y, z;\n\
             \x20   wire #(1, 2, 3) w;\n\
             \x20   assign #5 y = a;\n\
             \x20   assign #(4, 2) z = a;\n\
             \x20   assign w = a;",
        );
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect("delayed drivers and nets lower");
        let plan = &artifact.digital;
        let delay = |rise, fall, turn_off| DigitalTransitionDelay {
            rise,
            fall,
            turn_off,
            exponent: plan.timing.precision_exponent,
        };
        let delays: Vec<_> = plan
            .delays
            .drivers
            .iter()
            .map(|delayed| {
                (
                    plan.signals[usize::from(delayed.driver.signal)]
                        .name
                        .as_str(),
                    delayed.delay,
                )
            })
            .collect();
        assert_eq!(delays, [("y", delay(5, 5, 5)), ("z", delay(4, 2, 2))]);
        let nets: Vec<_> = plan
            .delays
            .nets
            .iter()
            .map(|delayed| {
                (
                    plan.signals[usize::from(delayed.signal)].name.as_str(),
                    delayed.delay,
                )
            })
            .collect();
        assert_eq!(nets, [("w", delay(1, 2, 3))]);

        let source = digital_module(
            "    wire a;\n\
             \x20   reg [3:0] d;\n\
             \x20   wire y;\n\
             \x20   assign #(d) y = a;",
        );
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect_err("a delay read from a variable is refused")
            .to_string();
        assert!(
            error.contains("not a constant expression") && error.contains("6.1.3"),
            "{error}"
        );
    }

    /// VAMS-2023 8.2 initializes module variables before process startup.
    /// Assignment sizing retains all packed bits, including X/Z and signed fills.
    #[test]
    fn packed_declaration_initializers_preserve_width_bits_and_parameter_values() {
        use rspice_veriloga::canonical_ir::digital::DigitalInitialValue;
        let source = digital_module(
            r#"
parameter INIT=8'h96;
reg q=1'b0;
reg signed [7:0] signed_value=4'shf;
reg [0:7] ascending=INIT;
reg [3:0] truncated=8'hab;
reg [11:0] unknown=4'bx10z;
reg [7:0] rounded=2.5;
reg [63:0] exact=64'h0020000000000001;
reg [95:0] wide={32'h01234567,32'h89abcdef,32'hfedcba98};
"#,
        );
        let artifact = VerilogACompiler::default()
            .compile_canonical_ir(&source)
            .unwrap();
        let plan = &artifact.digital;
        assert!(plan.processes.is_empty() && plan.drivers.is_empty());
        for (name, expected) in [
            ("q", "0".to_string()),
            ("signed_value", "11111111".to_string()),
            ("ascending", "10010110".to_string()),
            ("truncated", "1011".to_string()),
            ("unknown", "00000000x10z".to_string()),
            ("rounded", "00000011".to_string()),
            ("exact", format!("{:064b}", 9_007_199_254_740_993_u64)),
            (
                "wide",
                format!(
                    "{:032b}{:032b}{:032b}",
                    0x01234567_u32, 0x89abcdef_u32, 0xfedcba98_u32
                ),
            ),
        ] {
            let signal = plan
                .signals
                .iter()
                .find(|signal| signal.name == name)
                .unwrap();
            let Some(DigitalInitialValue::FourState(value)) = &signal.initial_value else {
                panic!("{name}")
            };
            assert_eq!(value.spelling(), expected, "{name}");
            assert_eq!(value.width(), signal.width, "{name}");
        }
        let decoded: rspice_veriloga::canonical_ir::CanonicalDigitalPlan =
            serde_json::from_str(&serde_json::to_string(plan).unwrap()).unwrap();
        assert_eq!(&decoded, plan);
        decoded.validate().unwrap();
        let analog_scalar =
            "module scalar(p); inout p; electrical p; analog I(p)<+64'h0020000000000001; endmodule";
        assert!(
            VerilogACompiler::default()
                .compile_canonical_ir(analog_scalar)
                .is_err(),
            "an analog scalar consumer must not silently round an exact packed literal"
        );
        for expression in ["other", "$time", "V(p)"] {
            let source = digital_module(&format!("reg other; reg invalid={expression};"));
            assert!(
                VerilogACompiler::default()
                    .compile_canonical_ir(&source)
                    .is_err(),
                "{expression}"
            );
        }
    }

    #[test]
    fn analog_array_reads_preserve_element_types_and_validate_probe_ranges() {
        use rspice_veriloga::canonical_ir::{CfgValueKind, CfgValueType};
        let source = r#"
module arrays(p);
  inout p; electrical p;
  real values[-2:0]; integer codes[2:0];
  integer index; real sample; integer code;
  analog begin
    values[-2]=V(p); values[-1]=2*V(p); values[0]=3*V(p);
    codes[0]=-3; codes[1]=-4; codes[2]=-5;
  end
  initial begin
    index=-2; sample=values[index]; code=codes[index+2];
    #1; index=0; sample=values[index]; code=codes[2];
  end
endmodule
"#;
        let artifact = VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap();
        let plan = &artifact.digital;
        assert_eq!(plan.analog_probes.len(), 6);
        assert_eq!(artifact.hir.digital_observations.len(), 6);
        let function = &plan.processes[0].function;
        let reads: Vec<_> = function
            .values
            .iter()
            .filter_map(|value| {
                if let CfgValueKind::DigitalAnalogVariable {
                    array_index: Some(selection),
                    ..
                } = &value.kind
                {
                    assert_eq!(selection.len, 3);
                    assert_eq!(value.kind.operands(), vec![selection.index]);
                    Some(value.value_type)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(
            reads,
            [
                CfgValueType::Real,
                CfgValueType::FourState { width: 32 },
                CfgValueType::Real,
                CfgValueType::FourState { width: 32 }
            ]
        );
        let decoded: rspice_veriloga::canonical_ir::CanonicalDigitalPlan =
            serde_json::from_str(&serde_json::to_string(plan).unwrap()).unwrap();
        decoded.validate().unwrap();
        for (lower, len) in [(-2, 0), (-2, 7), (i64::MAX, 3), (-1, 3)] {
            let mut invalid = decoded.clone();
            let read = invalid.processes[0]
                .function
                .values
                .iter_mut()
                .find(|value| {
                    matches!(
                        value.kind,
                        CfgValueKind::DigitalAnalogVariable {
                            array_index: Some(_),
                            ..
                        }
                    )
                })
                .unwrap();
            if let CfgValueKind::DigitalAnalogVariable {
                array_index: Some(selection),
                ..
            } = &mut read.kind
            {
                selection.lower = lower;
                selection.len = len;
            }
            let errors = invalid.validate().unwrap_err();
            assert!(format!("{errors:?}").contains("array"), "{errors:?}");
        }
        assert!(
            VerilogACompiler::default()
                .compile_canonical_ir(&source.replace("sample=values[index]", "sample=values"))
                .is_err()
        );
    }

    /// The runtime path, which is what feeds the JIT and generated-Rust
    /// backends, refuses it as well.
    #[test]
    fn runtime_compilation_refuses_digital_modules() {
        let source = digital_module(
            "    wire clk;\n\
             \x20   reg q;\n\
             \x20   always @(posedge clk) q <= 1'b1;",
        );
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(&source, None)
            .expect_err("runtime compilation must refuse a digital module");
        assert_eq!(
            error.diagnostic_code(),
            "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
        );
    }
}

// ===========================================================================
// Module instantiation (IEEE 1364-2005 sections 12.1.2 and 12.3)
// ===========================================================================

/// A source with one gate module and a top that instantiates it.
///
/// The top keeps the analog body every other fixture here has, because the
/// module a hierarchy is compiled *for* is still a device the solver calls.
fn hierarchy(child: &str, top_section: &str) -> String {
    format!(
        "{child}\n\
         module top(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         {top_section}\n\
         \x20   analog I(p, n) <+ V(p, n);\n\
         endmodule\n"
    )
}

/// A two-input NAND with an internal net, so a test can see what hoisting does
/// to a name that has nowhere else to go.
const NAND2: &str = "module nand2(y, a, b);\n\
                     \x20   output y;\n\
                     \x20   input a, b;\n\
                     \x20   wire y, a, b, t;\n\
                     \x20   assign t = a & b;\n\
                     \x20   assign y = ~t;\n\
                     endmodule\n";

fn hierarchy_error(source: &str) -> String {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_module(source, Some("top"))
        .expect_err("this hierarchy must be refused")
        .to_string()
}

fn top_module(parsed: &SourceFile) -> &Module {
    parsed
        .items
        .iter()
        .find_map(|item| match item {
            Item::Module(module) if module.name == "top" => Some(module),
            _ => None,
        })
        .expect("the fixture declares a module named `top`")
}

/// The ordered form: connections match ports left to right.
#[test]
fn an_ordered_port_connection_list_parses_positionally() {
    let source = hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(y, a, b);");
    let parsed = parse(&source);
    let top = top_module(&parsed);
    assert_eq!(top.instances.len(), 1);
    let instance = &top.instances[0];
    assert_eq!(instance.module, "nand2");
    assert_eq!(instance.name, "g1");
    let names: Vec<String> = instance
        .connections
        .iter()
        .map(|connection| match connection {
            Connection::Ordered {
                signal: Some(Expression::Identifier(identifier)),
                ..
            } => identifier.name.to_string(),
            other => panic!("expected an ordered identifier connection, got {other:?}"),
        })
        .collect();
    assert_eq!(names, vec!["y", "a", "b"]);
}

/// The named form: each connection carries the port it binds.
#[test]
fn a_named_port_connection_list_parses_with_its_port_names() {
    let source = hierarchy(
        NAND2,
        "    wire a, b, y;\n     nand2 g1(.b(b), .y(y), .a(a));",
    );
    let parsed = parse(&source);
    let pairs: Vec<(String, String)> = top_module(&parsed).instances[0]
        .connections
        .iter()
        .map(|connection| match connection {
            Connection::Named {
                port,
                signal: Some(Expression::Identifier(identifier)),
                ..
            } => (port.to_string(), identifier.name.to_string()),
            other => panic!("expected a named connection, got {other:?}"),
        })
        .collect();
    assert_eq!(
        pairs,
        vec![
            ("b".to_string(), "b".to_string()),
            ("y".to_string(), "y".to_string()),
            ("a".to_string(), "a".to_string()),
        ]
    );
}

/// IEEE 1364-2005 section 12.1.2 lets a range after the instance name declare
/// an array of instances: one instance per element, named after its index
/// from the left bound, with bounds this module's constants fold.
#[test]
fn an_instance_array_becomes_one_instance_per_element() {
    let parsed = parse(&hierarchy(
        NAND2,
        "    parameter N = 2;\n    wire a, b, y;\n     nand2 g[N-1:-1](y, a, b);",
    ));
    let names: Vec<&str> = top_module(&parsed)
        .instances
        .iter()
        .map(|instance| instance.name.as_str())
        .collect();
    assert_eq!(names, ["g[1]", "g[0]", "g[-1]"]);
}

/// The bounds of an array of instances are constant expressions.
#[test]
fn an_instance_array_with_a_variable_bound_is_refused_by_name() {
    let message = parse_error(&hierarchy(
        NAND2,
        "    wire a, b, y; reg [1:0] n;\n     nand2 g[n:0](y, a, b);",
    ));
    assert!(
        message.contains("instance array range") && message.contains("12.1.2"),
        "{message}"
    );
}

/// A child with two parameters and a localparam, for `defparam` targets.
const PARAMETERIZED: &str = "module leaf(y);\n\
                             \x20   output y;\n\
                             \x20   parameter P = 1, Q = 2;\n\
                             \x20   localparam L = P + 1;\n\
                             \x20   wire y;\n\
                             \x20   assign y = P[0];\n\
                             endmodule\n\
                             module mid(y);\n\
                             \x20   output y;\n\
                             \x20   wire y;\n\
                             \x20   leaf l(y);\n\
                             endmodule\n";

/// A comma-separated `defparam` assigns each of its targets: one of the
/// writer's own instances, and one below an instance of a module
/// instantiated once.
#[test]
fn defparam_assigns_each_of_its_targets() {
    let compiled = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(
        &hierarchy(
            PARAMETERIZED,
            "    wire y, z;\n     leaf u1(y);\n     mid m(z);\n     defparam u1.P = 3, m.l.Q = 4;",
        ),
        Some("top"),
    )
    .expect("the hierarchy compiles");
    // u1.P and u1.Q, in elaboration order. `m.l.Q` keeps its value as an
    // expression of `mid`'s scope; the HDL host tests run it.
    assert_eq!(child_parameters(&compiled)[..2], [3.0, 2.0]);
}

/// The elaborated `P` and `Q` of each `leaf` instance, in elaboration order,
/// up to the first one kept as an expression. A flattened child's parameter
/// keeps a runtime slot under an internal name that ends with its own.
fn child_parameters(compiled: &rspice_veriloga::RuntimeCompileReport) -> Vec<f64> {
    compiled
        .canonical_ir
        .hir
        .parameters
        .iter()
        .filter(|parameter| parameter.name.ends_with("_P") || parameter.name.ends_with("_Q"))
        .map_while(|parameter| parameter.elaboration_value.or(parameter.default))
        .collect()
}

/// The value a compiled design elaborated one parameter to.
fn elaborated_parameter(compiled: &rspice_veriloga::RuntimeCompileReport, name: &str) -> f64 {
    let parameters = &compiled.canonical_ir.hir.parameters;
    parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .and_then(|parameter| parameter.elaboration_value)
        .unwrap_or_else(|| {
            panic!(
                "no elaborated `{name}` among {:?}",
                parameters
                    .iter()
                    .map(|parameter| &parameter.name)
                    .collect::<Vec<_>>()
            )
        })
}

/// Two `defparam`s on one parameter: the last in source order wins (section
/// 12.2.1), with a warning naming both.
#[test]
fn the_last_of_two_defparams_wins_with_a_warning_naming_both() {
    let compiled = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(
        &hierarchy(
            PARAMETERIZED,
            "    wire y, z;\n     leaf #(.P(2)) u1(y);\n     mid m(z);\n     \
             defparam u1.P = 5;\n     defparam u1.P = 6;",
        ),
        Some("top"),
    )
    .expect("the hierarchy compiles");
    assert_eq!(child_parameters(&compiled), [6.0, 2.0, 1.0, 2.0]);
    let warnings: Vec<_> = compiled
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "VA-SEM-DEFPARAM-REPLACED")
        .collect();
    assert_eq!(warnings.len(), 1, "{:?}", compiled.diagnostics);
    let message = &warnings[0].message;
    assert!(
        message.contains("`defparam u1.P` in module `top` replaces the earlier `defparam u1.P`")
            && message.contains("top.u1.P")
            && message.contains("12.2.1"),
        "{message}"
    );
}

/// What a `defparam` cannot reach is refused by name.
#[test]
fn defparam_targets_outside_its_reach_are_refused_by_name() {
    let refused = |section: &str| {
        hierarchy_error(&hierarchy(
            PARAMETERIZED,
            &format!("    wire y, z, w;\n     leaf u1(y);\n     mid m(z);\n{section}"),
        ))
    };
    for (section, expected) in [
        ("     defparam u1.L = 3;", "a `localparam` of module `leaf`"),
        ("     defparam u1.R = 3;", "declares no parameter `R`"),
        (
            "     defparam nope.P = 3;",
            "names `nope`, which is neither",
        ),
        (
            "     defparam m.x.P = 3;",
            "`x`, which is not an instance of module `mid`",
        ),
        ("     defparam m.l.P = w;", "names a declaration"),
        (
            "     mid m2(w);\n     defparam m.l.P = 3;",
            "passes through module `mid`, which is instantiated 2 times",
        ),
    ] {
        let message = refused(section);
        assert!(message.contains(expected), "{section}: {message}");
    }
    let message = parse_error(&hierarchy(
        PARAMETERIZED,
        "    wire y;\n     generate if (1) begin : g\n       leaf u(y);\n       \
         defparam u.P = 3;\n     end endgenerate",
    ));
    assert!(
        message.contains("`defparam`") && message.contains("generate block"),
        "{message}"
    );
    let message = parse_error(&hierarchy(
        PARAMETERIZED,
        "    wire y;\n     leaf u1(y);\n     defparam g[0].u.P = 3;",
    ));
    assert!(message.contains("an indexed scope"), "{message}");
    // `top` has an analog block: its own parameters take instance values
    // at run time, where the `defparam` could not win.
    for section in [
        "     parameter T = 1;\n     defparam top.T = 3;",
        "     parameter T = 1;\n     defparam T = 3;",
    ] {
        let message = refused(section);
        assert!(message.contains("has an analog block"), "{message}");
    }
    // A simple name in an instantiated module would reach every instance.
    let message = hierarchy_error(&hierarchy(
        "module leaf(y);\n    output y;\n    parameter P = 1;\n    wire y;\n    \
         assign y = P[0];\n    defparam P = 2;\nendmodule\n",
        "    wire y;\n     leaf u1(y);",
    ));
    assert!(message.contains("which is instantiated"), "{message}");
}

/// An instance value names an operand its scope never declares, or one it
/// declares as a variable: both are the author's error (IEEE 1364-2005
/// section 12.2 makes the value a constant expression), not a missing form.
#[test]
fn an_override_naming_an_undeclared_or_variable_operand_is_a_language_error() {
    for (section, expected) in [
        (
            "    wire y;\n     leaf #(NOPE) u1(y);",
            "Undeclared symbol: 'NOPE'",
        ),
        (
            "    wire y;\n     integer v;\n     leaf #(v + 1) u1(y);",
            "`v` is not a constant",
        ),
        (
            "    wire y;\n     integer v;\n     leaf u1(y);\n     defparam u1.P = v;",
            "`v` is not a constant",
        ),
    ] {
        let message = hierarchy_error(&hierarchy(PARAMETERIZED, section));
        assert!(
            message.contains(expected) && !message.contains("Unsupported feature"),
            "{section}: {message}"
        );
    }
}

/// A `defparam` on a digital root's own parameter takes precedence over the
/// instance value an X card gives it, with a warning (design P2 section 4.2).
#[test]
fn a_defparam_on_a_root_parameter_takes_precedence_over_its_instance_value() {
    let source = "module top(y);\n\
                  \x20   output y;\n\
                  \x20   parameter integer T = 1;\n\
                  \x20   wire y;\n\
                  \x20   assign y = T[1];\n\
                  \x20   defparam top.T = 3;\n\
                  endmodule\n";
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let initial = compiler
        .compile_runtime(source, Some("top"))
        .expect("the module compiles");
    let specialized = compiler
        .specialize_mixed_runtime(
            &initial.canonical_ir,
            &[("T", 2.0)],
            &rspice_veriloga::NoPipelineControl,
        )
        .expect("the instance value is accepted");
    for report in [&initial, &specialized] {
        assert_eq!(elaborated_parameter(report, "T"), 3.0);
    }
    let warning = specialized
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "VA-SEM-DEFPARAM-REPLACED")
        .unwrap_or_else(|| panic!("{:?}", specialized.diagnostics));
    assert!(
        warning
            .message
            .contains("takes precedence over the instance value"),
        "{}",
        warning.message
    );
}

/// A hierarchy that elaborates is still refused by the bytecode backend, which
/// has no representation for a process at all. The refusal reaches an instance
/// too: without that the whole hierarchy would vanish into a device that
/// compiled.
#[test]
fn the_backend_refusal_names_a_digital_instance() {
    let source = hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(y, a, b);");
    let error = compile_error(&source, Some("top"));
    assert_eq!(
        error.diagnostic_code(),
        "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
    );
    assert!(
        error.to_string().contains("Verilog-AMS digital construct"),
        "{error}"
    );
}

/// A top module whose only discrete-domain content is an instance is the case
/// the instance arm of the refusal exists for: nothing else in the module is
/// digital, so nothing else could name it, and without the arm the whole
/// hierarchy would compile into a device silently missing it.
#[test]
fn a_module_whose_only_digital_content_is_an_instance_still_refuses() {
    let source = format!(
        "{NAND2}\n\
         module top(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   nand2 g1();\n\
         \x20   analog I(p, n) <+ V(p, n);\n\
         endmodule\n"
    );
    let message = compile_error(&source, Some("top")).to_string();
    assert!(
        message.contains("digital instance `g1` of module `nand2`"),
        "{message}"
    );
}

/// An `input` port its own module drives is coerced to `inout`, with a
/// warning, whether or not anybody instantiates the module (IEEE 1364-2005
/// section 12.3.8: a port "declared as input ... but used as an output" may be
/// coerced to inout).
///
/// The "whether or not" is the point: the coercion is read off the module's
/// own text rather than off a connection, so the same source is not accepted
/// one way or the other depending on where it sits in a hierarchy.
#[test]
fn a_module_driving_its_own_input_port_coerces_it_to_inout() {
    let coerced = |source: &str| {
        let analyzed = analyze(source);
        let module = only_module(&analyzed);
        let port = module
            .ports
            .iter()
            .find(|port| port.name == "a")
            .expect("port a");
        assert_eq!(port.direction, PortDirection::Inout, "{source}");
        assert!(port.coerced_from_input, "{source}");
        let untouched = module
            .ports
            .iter()
            .find(|port| port.name == "y")
            .expect("port y");
        assert_eq!(untouched.direction, PortDirection::Output);
        assert!(!untouched.coerced_from_input);
        let warnings: Vec<_> = analyzed
            .warnings
            .iter()
            .filter(|warning| warning.code == "VA-SEM-COERCED-INPUT-PORT")
            .collect();
        assert_eq!(warnings.len(), 1, "{:?}", analyzed.warnings);
        assert!(
            warnings[0].message.contains("`a`"),
            "{}",
            warnings[0].message
        );
        assert!(
            warnings[0].message.contains("12.3.8"),
            "{}",
            warnings[0].message
        );
    };
    // Two drivers of one port warn once.
    coerced(
        "module driver(y, a);\n\
         \x20   output y;\n     input a;\n     wire y, a;\n\
         \x20   assign y = 1'b0;\n\
         \x20   assign a = y;\n     assign a = 1'b1;\n\
         endmodule\n",
    );
    // A gate primitive is a continuous assignment by the time this runs, so the
    // same rule reaches it.
    coerced(
        "module gated(y, a);\n\
         \x20   output y;\n     input a;\n     wire y, a;\n\
         \x20   buf u (a, y);\n\
         endmodule\n",
    );

    // An input port nothing inside drives keeps its direction, unwarned.
    let analyzed = analyze(
        "module reader(y, a);\n\
         \x20   output y;\n     input a;\n     wire y, a;\n\
         \x20   assign y = a;\n\
         endmodule\n",
    );
    let port = &only_module(&analyzed).ports[1];
    assert_eq!(
        (port.direction, port.coerced_from_input),
        (PortDirection::Input, false)
    );
    assert!(analyzed.warnings.is_empty(), "{:?}", analyzed.warnings);

    // An `inout` port may be driven from inside.
    let analyzed = analyze(
        "module bidirectional(io, a);\n\
         \x20   inout io;\n     input a;\n     wire io, a;\n\
         \x20   assign io = a;\n\
         endmodule\n",
    );
    assert_eq!(only_module(&analyzed).digital.continuous_assigns.len(), 1);
}

/// A port connected to *some of* a net's bits does not collapse; it becomes an
/// implicit continuous assignment, in whichever direction the port's own
/// direction gives it (IEEE 1364-2005 section 12.3.9).
///
/// Two nets cannot be joined when one of them is four bits and the other is one
/// of those four, so the collapse reading has nothing to say here. The
/// assignment reading does, and it is a *real* driver: four instances driving
/// four bits of one net are four drivers of that net, each covering its own
/// bit, so the resolver sees four contributions rather than four whole-net
/// writes that overwrite each other.
#[test]
fn a_bit_select_port_connection_becomes_a_driver_on_that_bit() {
    let source = "module inv(y, a);\n\
         \x20   output y;\n     input a;\n     wire y, a;\n\
         \x20   assign y = ~a;\n\
         endmodule\n\
         module top(bus, out);\n\
         \x20   input [3:0] bus;\n\
         \x20   output [3:0] out;\n\
         \x20   wire [3:0] bus, out;\n\
         \x20   inv u0 (out[0], bus[0]);\n\
         \x20   inv u1 (out[1], bus[1]);\n\
         \x20   inv u2 (out[2], bus[2]);\n\
         \x20   inv u3 (out[3], bus[3]);\n\
         endmodule\n";
    let plan = plan(source, "top");

    // The ports kept their own signals rather than collapsing onto `out`/`bus`.
    let names: Vec<&str> = plan
        .signals
        .iter()
        .map(|signal| signal.name.as_str())
        .collect();
    for instance in ["u0", "u1", "u2", "u3"] {
        for port in ["y", "a"] {
            assert!(
                names.contains(&format!("{instance}.{port}").as_str()),
                "no elaborated signal `{instance}.{port}`; got {names:?}"
            );
        }
    }

    // `out` has four drivers, one per bit, with distinct indices.
    let out = plan
        .signals
        .iter()
        .find(|signal| signal.name == "out")
        .expect("`out` is declared");
    let mut driven: Vec<(u32, i64)> = plan
        .drivers_of(out.id)
        .map(|driver| {
            let bit = match driver.target.select {
                rspice_veriloga::canonical_ir::digital::DigitalWriteSelect::Bit(position) => {
                    position
                }
                ref other => panic!("expected a bit select, got {other:?}"),
            };
            (driver.id.index, bit)
        })
        .collect();
    driven.sort_unstable();
    assert_eq!(driven, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);

    // Each instance's own input port is driven from the parent's bit, so it has
    // a driver of its own rather than sharing the parent's net.
    for instance in ["u0", "u1", "u2", "u3"] {
        let port = plan
            .signals
            .iter()
            .find(|signal| signal.name == format!("{instance}.a"))
            .expect("the input port kept its own signal");
        assert_eq!(
            plan.drivers_of(port.id).count(),
            1,
            "`{instance}.a` must be driven from the parent's bit"
        );
    }
}

/// A part-select connection carries its whole width, and the driver covers
/// exactly those bits.
#[test]
fn a_part_select_port_connection_drives_exactly_its_bits() {
    let source = "module half(y, a);\n\
         \x20   output [1:0] y;\n     input [1:0] a;\n\
         \x20   wire [1:0] y, a;\n\
         \x20   assign y = ~a;\n\
         endmodule\n\
         module top(bus, out);\n\
         \x20   input [3:0] bus;\n\
         \x20   output [3:0] out;\n\
         \x20   wire [3:0] bus, out;\n\
         \x20   half lo (out[1:0], bus[1:0]);\n\
         \x20   half hi (out[3:2], bus[3:2]);\n\
         endmodule\n";
    let plan = plan(source, "top");

    let out = plan
        .signals
        .iter()
        .find(|signal| signal.name == "out")
        .expect("`out` is declared");
    let mut parts: Vec<(i64, i64)> = plan
        .drivers_of(out.id)
        .map(|driver| match driver.target.select {
            rspice_veriloga::canonical_ir::digital::DigitalWriteSelect::Part { msb, lsb } => {
                (msb, lsb)
            }
            ref other => panic!("expected a part select, got {other:?}"),
        })
        .collect();
    parts.sort_unstable();
    assert_eq!(parts, vec![(1, 0), (3, 2)]);
}

/// A child's own parameter is fixed at its declared default (IEEE 1364-2005
/// section 12.2), and a parameter-sized expression inside the child folds
/// against the *child's* table.
///
/// The two `WIDTH`s in this fixture are deliberately different numbers. A pass
/// that folded the child's `{WIDTH{1'b0}}` with the parent's would produce a
/// two-bit constant where the child asked for four, and the resulting device
/// would be silently the wrong width rather than refused.
#[test]
fn a_child_folds_its_own_parameters_and_never_the_parents() {
    let source = "module sink(y, a);\n\
         \x20   parameter WIDTH = 4;\n\
         \x20   output [WIDTH-1:0] y;\n\
         \x20   input [WIDTH-1:0] a;\n\
         \x20   wire [WIDTH-1:0] y, a;\n\
         \x20   assign y = a ^ {WIDTH{1'b1}};\n\
         endmodule\n\
         module top(bus, out);\n\
         \x20   parameter WIDTH = 2;\n\
         \x20   input [3:0] bus;\n\
         \x20   output [3:0] out;\n\
         \x20   wire [3:0] bus, out;\n\
         \x20   sink u (out, bus);\n\
         endmodule\n";
    let plan = plan(source, "top");

    let port = plan
        .signals
        .iter()
        .find(|signal| signal.name == "out")
        .expect("`out` is declared");
    assert_eq!(port.width, 4, "the child elaborated at its own WIDTH");
    // One driver, from the child's `assign`, over the whole four-bit net.
    assert_eq!(plan.drivers_of(port.id).count(), 1);
}

/// Every construct this wave does not elaborate refuses by name, with the
/// clause that governs it. Nothing is dropped, and nothing is guessed at.
#[test]
fn unelaborated_instance_constructs_refuse_by_name() {
    let cases: Vec<(&str, String, Vec<&str>)> = vec![
        (
            "an excess ordered parameter override",
            hierarchy(
                "module gate(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y, a;\n\
                 \x20   assign y = a;\n\
                 endmodule\n",
                "    wire x, z;\n     gate #(2) g1(z, x);",
            ),
            vec!["at most 0 parameter overrides"],
        ),
        (
            "a runtime bit-select connected to an inout port",
            hierarchy(
                "module pass(io, a);\n\
                 \x20   inout io;\n     input a;\n     wire io, a;\n\
                 \x20   assign io = a;\n\
                 endmodule\n",
                "    wire [1:0] bus;\n     wire x; integer index;\n     pass g1(bus[index], x);",
            ),
            vec!["bidirectional connection requires integer coordinates at elaboration"],
        ),
        (
            "an undeclared connection name",
            hierarchy(
                NAND2,
                "`default_nettype none\n    wire a, b;\n     nand2 g1(y, a, b);",
            ),
            vec!["implicit net", "default_nettype none"],
        ),
        (
            "an input port declared a variable",
            hierarchy(
                "module gate(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y;\n     reg a;\n\
                 \x20   assign y = a;\n\
                 endmodule\n",
                "    wire x, z;\n     gate g1(z, x);",
            ),
            vec!["only an output port be a variable", "12.3.3"],
        ),
        (
            "an output port connected to a variable",
            hierarchy(
                "module gate(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y, a;\n\
                 \x20   assign y = a;\n\
                 endmodule\n",
                "    wire x;\n     reg z;\n     gate g1(z, x);",
            ),
            vec!["connects an output or inout port to a net", "12.3.9.2"],
        ),
        (
            "an input port coerced to inout on a continuous net",
            hierarchy(DRIVEN_INPUT, "    wire z;\n     gate g1(z, p);"),
            vec![
                "`input` port `a` of instance `g1`",
                "12.3.8",
                "bidirectional converter",
            ],
        ),
        (
            "an output port driving the connecting module's input port",
            format!(
                "{DRIVEN_INPUT}\n\
                 module mid(q, d);\n\
                 \x20   output q;\n     input d;\n     wire q, d;\n\
                 \x20   gate g(q, d);\n\
                 endmodule\n\
                 module top(p, n);\n\
                 \x20   inout p, n;\n     electrical p, n;\n\
                 \x20   wire x, z;\n     mid m1(z, x);\n\
                 \x20   analog I(p, n) <+ V(p, n);\n\
                 endmodule\n"
            ),
            vec!["receives through an input port", "12.3.8"],
        ),
        (
            "an unknown named port",
            hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(.q(y));"),
            vec!["Undeclared symbol: 'q'"],
        ),
        (
            "more ordered connections than ports",
            hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(y, a, b, a);"),
            vec!["at most 3 port connections"],
        ),
        (
            "an instantiation cycle",
            hierarchy(
                "module loop(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y, a, t;\n\
                 \x20   assign t = a;\n     loop u(y, t);\n\
                 endmodule\n",
                "    wire x, z;\n     loop g1(z, x);",
            ),
            vec!["Circular dependency", "specialization repeats ancestor"],
        ),
        (
            "a continuous port connected directly to a discrete net",
            hierarchy(
                "module res(p, n);\n\
                 \x20   inout p, n;\n     electrical p, n;\n\
                 \x20   analog I(p, n) <+ V(p, n);\n\
                 endmodule\n\
                 module gate(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y, a;\n\
                 \x20   assign y = a;\n     res r1(y, a);\n\
                 endmodule\n",
                "    wire x, z;\n     gate g1(z, x);",
            ),
            vec![
                "discrete net",
                "without a connect module",
                "deck node",
                "converter module",
            ],
        ),
    ];

    for (label, source, fragments) in cases {
        let message = hierarchy_error(&source);
        for fragment in fragments {
            assert!(
                message.contains(fragment),
                "refusing {label} must mention {fragment:?}, got {message:?}"
            );
        }
    }
}

/// A module that drives its own `input` port `a`, which IEEE 1364-2005
/// section 12.3.8 coerces to `inout`.
const DRIVEN_INPUT: &str = "module gate(y, a);\n\
                            \x20   output y;\n     input a;\n     wire y, a;\n\
                            \x20   assign y = 1'b0;\n     assign a = y;\n\
                            endmodule\n";

/// The coerced port joins its connection as an `inout` port does (IEEE
/// 1364-2005 section 12.3.8): the driver inside the instance is a driver of
/// the parent's net, and the parent's other driver resolves with it there.
#[test]
fn a_coerced_input_port_drives_its_connection_in_the_parent() {
    let source = format!(
        "{DRIVEN_INPUT}\n\
         module top(z);\n\
         \x20   output z;\n     wire x, z;\n\
         \x20   gate g1(z, x);\n     assign x = 1'b1;\n\
         endmodule\n"
    );
    let linked = plan(&source, "top");
    let x = linked
        .signals
        .iter()
        .find(|signal| signal.name == "x")
        .expect("the parent's net");
    assert!(
        linked.signals.iter().all(|signal| signal.name != "g1.a"),
        "the coerced port collapsed onto `x`"
    );
    // `assign a = y;` inside the instance and `assign x = 1'b1;` beside it.
    assert_eq!(linked.drivers_of(x.id).count(), 2);

    // Connected to a constant, the port has nothing outside to drive and
    // takes the constant as the input it was declared.
    let source = format!(
        "{DRIVEN_INPUT}\n\
         module top(z);\n\
         \x20   output z;\n     wire z;\n\
         \x20   gate g1(z, 1'b0);\n\
         endmodule\n"
    );
    plan(&source, "top");
}

const BOUNDARY_CHILDREN: &str = "module dig(q);\n\
                                 \x20   output q; reg q;\n\
                                 \x20   initial q = 0;\n\
                                 endmodule\n\
                                 module res(a, b);\n\
                                 \x20   inout a, b; electrical a, b;\n\
                                 \x20   analog I(a, b) <+ V(a, b);\n\
                                 endmodule\n";

/// A declared net that joins a discrete child output and an analog child port
/// is a discipline boundary inside the design (LRM 2.4 clause 7). The output
/// keeps its own signal, and the compiled design links it on the net's
/// circuit node, where the host converts it as it converts a deck-level port.
#[test]
fn a_discipline_boundary_inside_the_design_links_the_discrete_port() {
    for declaration in ["wire m;", "electrical m;"] {
        let artifact = VerilogACompiler::new(CompilerOptions {
            enable_ams: true,
            ..CompilerOptions::default()
        })
        .compile_canonical_ir_module(
            &hierarchy(
                BOUNDARY_CHILDREN,
                &format!("    {declaration}\n     dig u1(m);\n     res u2(m, p);"),
            ),
            Some("top"),
        )
        .unwrap_or_else(|error| panic!("`{declaration}` must compile: {error}"));
        let [port] = artifact.hir.hoisted_ports.as_slice() else {
            panic!("`{declaration}`: {:?}", artifact.hir.hoisted_ports);
        };
        assert_eq!(port.signal, "u1.q", "{declaration}");
        assert_eq!(port.bit, Some(0), "{declaration}");
        assert_eq!(port.direction, PortDirection::Output, "{declaration}");
        let rspice_veriloga::canonical_ir::HirExternalNet::Net(net) = &port.net else {
            panic!("`{declaration}`: {:?}", port.net);
        };
        assert!(
            artifact
                .hir
                .internal_nodes
                .iter()
                .any(|node| &node.name == net),
            "`{declaration}`: the port lands on the analog half's node for `m`, not {net}"
        );
    }
}

/// One bit of a continuous vector is a scalar net of its own after
/// flattening, so a discrete port on it links on that bit's node.
#[test]
fn a_discrete_port_on_a_bit_of_a_continuous_vector_links_on_that_bit() {
    let artifact = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
    .compile_canonical_ir_module(
        &hierarchy(
            BOUNDARY_CHILDREN,
            "    electrical [1:0] m;\n     dig u1(m[0]);\n     res u2(m[0], p);",
        ),
        Some("top"),
    )
    .unwrap_or_else(|error| panic!("a bit of a continuous vector links: {error}"));
    let [port] = artifact.hir.hoisted_ports.as_slice() else {
        panic!("{:?}", artifact.hir.hoisted_ports);
    };
    assert_eq!(port.signal, "u1.q");
    assert_eq!(
        port.net,
        rspice_veriloga::canonical_ir::HirExternalNet::Net("m[0]".into())
    );
    assert!(
        artifact
            .hir
            .internal_nodes
            .iter()
            .any(|node| node.name == "m[0]"),
        "the port lands on the analog half's node for `m[0]`"
    );
}

/// Both connection forms describe the same design, so they must elaborate to
/// the same plan — the same nets, the same drivers, the same identities. A
/// test that only checked one form would not notice a binder that read the
/// named form's ports in connection order.
#[cfg(feature = "native")]
#[test]
fn the_two_connection_forms_elaborate_to_the_same_plan() {
    let ordered = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            &hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(y, a, b);"),
            Some("top"),
        )
        .expect("the ordered form must elaborate");
    let named = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            &hierarchy(
                NAND2,
                "    wire a, b, y;\n     nand2 g1(.b(b), .y(y), .a(a));",
            ),
            Some("top"),
        )
        .expect("the named form must elaborate");
    assert_eq!(ordered.digital.signals, named.digital.signals);
    assert_eq!(ordered.digital.drivers, named.digital.drivers);
    assert_eq!(ordered.digital.processes, named.digital.processes);
}

// ===========================================================================
// Section 12.3 port declarations, and section 7.2 gate primitives
// ===========================================================================

/// The digital plan a source compiles to, by module name.
fn plan(source: &str, module: &str) -> rspice_veriloga::canonical_ir::CanonicalDigitalPlan {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(source, Some(module))
        .unwrap_or_else(|error| panic!("this module must compile: {error}"))
        .digital
}

/// One signal's identity and shape, without the span: name, width, declared
/// bounds, and whether a procedural assignment may write it.
#[cfg(feature = "native")]
type SignalShape = (String, u32, Option<(i64, i64)>, bool);

/// Every signal's shape, in declaration order.
///
/// Two spellings of one declaration are different text and therefore different
/// offsets, so a comparison that included the span would only ever say the two
/// sources are not the same source.
#[cfg(feature = "native")]
fn shapes(plan: &rspice_veriloga::canonical_ir::CanonicalDigitalPlan) -> Vec<SignalShape> {
    plan.signals
        .iter()
        .map(|signal| {
            (
                signal.name.to_string(),
                signal.width,
                signal.bounds,
                signal.procedurally_assignable,
            )
        })
        .collect()
}

/// One driver's identity, target and owning process, without the span.
#[cfg(feature = "native")]
type DriverShape = (
    rspice_veriloga::canonical_ir::DigitalDriverId,
    rspice_veriloga::canonical_ir::DigitalWriteTarget,
    rspice_veriloga::canonical_ir::DigitalProcessId,
);

/// Every driver's shape, in declaration order.
#[cfg(feature = "native")]
fn driver_shapes(plan: &rspice_veriloga::canonical_ir::CanonicalDigitalPlan) -> Vec<DriverShape> {
    plan.drivers
        .iter()
        .map(|driver| (driver.id, driver.target.clone(), driver.process))
        .collect()
}

/// A signal's declared width, or `None` if the plan has no such signal.
#[cfg(feature = "native")]
fn width_of(plan: &rspice_veriloga::canonical_ir::CanonicalDigitalPlan, name: &str) -> Option<u32> {
    plan.signals
        .iter()
        .find(|signal| signal.name == name)
        .map(|signal| signal.width)
}

/// IEEE 1364-2005 section 12.3.3: a port with no net or variable declaration of
/// its own is implicitly a net of the port's declared range.
///
/// A purely structural design declares nothing else at all, so without the
/// implicit net its ports would be absent from the plan and every reference to
/// one would be an undeclared name.
#[cfg(feature = "native")]
#[test]
fn a_port_with_no_declaration_of_its_own_is_an_implicit_net() {
    let plan = plan(
        "module implicit(a, b, y);\n\
         \x20   input a;\n\
         \x20   input [3:0] b;\n\
         \x20   output y;\n\
         \x20   assign y = a & b[0];\n\
         endmodule\n",
        "implicit",
    );
    assert_eq!(width_of(&plan, "a"), Some(1));
    // The implicit net takes the port's declared range, not a scalar default.
    assert_eq!(width_of(&plan, "b"), Some(4));
    assert_eq!(width_of(&plan, "y"), Some(1));
    assert_eq!(plan.drivers.len(), 1, "one `assign` is one driver");
}

/// Section 12.3.4's compact form. `output reg [3:0] q;` and the two-declaration
/// spelling it stands for must produce the same plan, or a design would mean
/// something different depending on which way its author wrote it.
#[cfg(feature = "native")]
#[test]
fn a_typed_port_declaration_means_the_same_as_the_two_declaration_form() {
    const BODY: &str = "\x20   always @(posedge clk) q <= d;\nendmodule\n";
    let compact = plan(
        &format!(
            "module compact(clk, d, q);\n\
             \x20   input clk;\n     input [3:0] d;\n     output reg [3:0] q;\n{BODY}"
        ),
        "compact",
    );
    let separate = plan(
        &format!(
            "module compact(clk, d, q);\n\
             \x20   input clk;\n     input [3:0] d;\n     output [3:0] q;\n\
             \x20   reg [3:0] q;\n{BODY}"
        ),
        "compact",
    );
    // Compared on everything but the span: the two spellings are different
    // text, so a `reg` declared on the port and one declared beside it point at
    // different offsets while declaring the same signal.
    assert_eq!(shapes(&compact), shapes(&separate));
    assert_eq!(compact.processes.len(), separate.processes.len());
    assert_eq!(width_of(&compact, "q"), Some(4));
    assert!(
        compact
            .signals
            .iter()
            .any(|signal| signal.name == "q" && signal.procedurally_assignable),
        "a `reg` port must be procedurally assignable"
    );
}

/// A packed range and a replication count may name a parameter, because IEEE
/// 1364-2005 section 12.2 fixes a parameter at elaboration. The analog half
/// deliberately treats a parameter as a per-instance runtime value, so this is
/// the one place the two halves read the same declaration differently.
#[cfg(feature = "native")]
#[test]
fn a_parameter_may_size_a_packed_range_and_a_replication() {
    let plan = plan(
        "module sized(clk, q);\n\
         \x20   parameter WIDTH = 6;\n\
         \x20   input clk;\n\
         \x20   output reg [WIDTH-1:0] q;\n\
         \x20   always @(posedge clk) q <= {WIDTH{1'b1}};\n\
         endmodule\n",
        "sized",
    );
    assert_eq!(width_of(&plan, "q"), Some(6));
}

/// Section 7.2's eight gate primitives are defined by the same truth tables
/// section 4.1 gives the operators, so `nand g (y, a, b)` and
/// `assign y = ~(a & b)` must lower to the same driver. Two paths to one
/// meaning is two chances to disagree; this is the pin that says there is one
/// path.
#[cfg(feature = "native")]
#[test]
fn a_gate_primitive_lowers_to_the_operator_form_of_itself() {
    for (gate, operator) in [
        ("and  u (y, a, b);", "assign y = a & b;"),
        ("nand u (y, a, b);", "assign y = ~(a & b);"),
        ("or   u (y, a, b);", "assign y = a | b;"),
        ("nor  u (y, a, b);", "assign y = ~(a | b);"),
        ("xor  u (y, a, b);", "assign y = a ^ b;"),
        ("xnor u (y, a, b);", "assign y = ~(a ^ b);"),
        // Section 7.4: `buf` drives `x` for `z`, as `a ^ 1'b0` does.
        ("buf  u (y, a);", "assign y = a ^ 1'b0;"),
        ("not  u (y, a);", "assign y = ~a;"),
    ] {
        let source = |body: &str| {
            format!(
                "module g(a, b, y);\n     input a, b;\n     output y;\n\x20   {body}\nendmodule\n"
            )
        };
        let from_gate = plan(&source(gate), "g");
        let from_operator = plan(&source(operator), "g");
        // The whole control-flow graph, value for value — everything except
        // where in the text it came from, which is the one thing the two
        // spellings genuinely differ about.
        let functions = |plan: &rspice_veriloga::canonical_ir::CanonicalDigitalPlan| {
            plan.processes
                .iter()
                .map(|process| {
                    (
                        process.id,
                        process.kind,
                        process.function.clone(),
                        process.static_sensitivity.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(
            functions(&from_gate),
            functions(&from_operator),
            "`{gate}` and `{operator}` must lower to the same process function"
        );
        assert_eq!(driver_shapes(&from_gate), driver_shapes(&from_operator));
    }
}

/// Section 7.4 puts a buffer's input last and lets every earlier terminal be an
/// output, which is the opposite of every other gate and the one shape a reader
/// has to check rather than assume.
#[cfg(feature = "native")]
#[test]
fn a_buffer_drives_every_terminal_but_its_last() {
    let plan = plan(
        "module b(a, y, z);\n\
         \x20   input a;\n     output y, z;\n\
         \x20   buf u (y, z, a);\n\
         endmodule\n",
        "b",
    );
    assert_eq!(plan.drivers.len(), 2, "two outputs are two drivers");
    let driven: Vec<&str> = plan
        .drivers
        .iter()
        .map(|driver| {
            plan.signal(driver.id.signal)
                .expect("declared")
                .name
                .as_str()
        })
        .collect();
    assert_eq!(driven, vec!["y", "z"]);
}

/// An unnamed gate instance is legal (section 7.1) and one declaration may
/// carry several instances.
#[cfg(feature = "native")]
#[test]
fn a_gate_declaration_may_be_unnamed_and_may_carry_several_instances() {
    let plan = plan(
        "module g(a, b, y, z);\n\
         \x20   input a, b;\n     output y, z;\n\
         \x20   nand (y, a, b), g2 (z, a, b);\n\
         endmodule\n",
        "g",
    );
    assert_eq!(plan.drivers.len(), 2);
}

/// A gate delay is a delay on the gate's driver, as an `assign #2`'s is
/// (IEEE 1364-2005 section 7.14), and a gate takes no more delays than A.3.1
/// gives it.
#[cfg(feature = "native")]
#[test]
fn a_gate_delay_is_its_drivers() {
    let plan = plan(
        "module g(a, b, y, z);\n\
         \x20   input a, b;\n     output y, z;\n\
         \x20   nand #(2, 3) u (y, a, b);\n\
         \x20   bufif1 #(1, 2, 3) v (z, a, b);\n\
         endmodule\n",
        "g",
    );
    let delays: Vec<_> = plan
        .delays
        .drivers
        .iter()
        .map(|delayed| {
            let delay = delayed.delay;
            (delay.rise, delay.fall, delay.turn_off)
        })
        .collect();
    assert_eq!(delays, [(2, 3, 2), (1, 2, 3)]);
    for (gate, found) in [
        ("nand #(1, 2, 3) u (y, a, b);", "at most 2"),
        ("pullup #1 (y);", "gives this primitive none"),
        ("tran #1 (y, z);", "gives this primitive none"),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(
                &format!(
                    "module g(a, b, y, z);\n    input a, b;\n    output y, z;\n    {gate}\nendmodule\n"
                ),
                Some("g"),
            )
            .expect_err("too many delays")
            .to_string();
        assert!(error.contains(found), "{gate}: {error}");
    }
}

/// A drive strength (IEEE 1364-2005 sections 6.1.4 and 7.8) on a gate, a
/// continuous assignment and a net declaration assignment, in either order;
/// a net type keyword names its net type.
#[test]
fn drive_strengths_and_net_types_parse() {
    use rspice_veriloga::canonical_ir::digital::{DigitalDriveStrength, DigitalStrengthLevel};
    let file = parse(
        "module g(a, b, y, z);\n\
         \x20   input a, b;\n     output y, z;\n     tri1 t;\n     supply0 s;\n\
         \x20   nand (strong1, weak0) u (y, a, b);\n\
         \x20   assign (pull0, highz1) z = a;\n\
         \x20   wire (weak1, supply0) w = b;\n\
         endmodule\n",
    );
    let module = parsed_module(&file);
    let strengths: Vec<_> = module
        .continuous_assigns
        .iter()
        .map(|assign| assign.strength)
        .collect();
    use DigitalStrengthLevel::*;
    let pair = |zero, one| Some(DigitalDriveStrength { zero, one });
    assert_eq!(
        strengths,
        [pair(Weak, Strong), pair(Pull, HighZ), pair(Supply, Weak)]
    );
    let kinds: Vec<_> = module.digital_nets.iter().map(|net| net.kind).collect();
    assert!(kinds.contains(&DigitalNetKind::Tri1) && kinds.contains(&DigitalNetKind::Supply0));
}

/// Section 6.1.4: `(highz0, highz1)` drives nothing and is illegal, and a pair
/// must give one strength to each value.
#[test]
fn an_illegal_drive_strength_pair_is_refused_by_name() {
    for pair in ["(highz1, highz0)", "(strong0, weak0)"] {
        let error = parse_error(&format!(
            "module g(a, y);\n\
             \x20   input a;\n     output y;\n\
             \x20   assign {pair} y = a;\n\
             endmodule\n"
        ));
        assert!(error.contains("section 6.1.4"), "{error}");
    }
}

/// `trireg` (IEEE 1364-2005 sections 4.4.1, 4.6.3 and 7.14.2): a charge
/// strength, `medium` by default, and the charge decay time as the third
/// delay. A decay time that is not a positive number is refused by name.
#[test]
fn trireg_declarations_parse_their_charge_strength_and_decay_time() {
    use rspice_veriloga::canonical_ir::digital::DigitalStrengthLevel::{Large, Medium, Small};
    let file = parse(
        "module g;\n\
         \x20   trireg t0;\n     trireg (large) #(0, 0, 50) t1;\n\
         \x20   trireg (small) [3:0] t2;\n     trireg #(0, 0, 2.5) t3;\n\
         endmodule\n",
    );
    let kinds: Vec<_> = parsed_module(&file)
        .digital_nets
        .iter()
        .map(|net| net.kind)
        .collect();
    let trireg = |charge, decay: Option<f64>| DigitalNetKind::Trireg {
        charge,
        decay: decay.and_then(ChargeDecay::new),
    };
    assert_eq!(
        kinds,
        [
            trireg(Medium, None),
            trireg(Large, Some(50.0)),
            trireg(Small, None),
            trireg(Medium, Some(2.5)),
        ]
    );
    // The rise and fall delays are the net's delay (section 6.1.3), kept
    // unless both are zero.
    let file = parse("module g;\n    trireg #(1, 0, 5) t;\n    trireg #(0, 0, 5) u;\nendmodule\n");
    let delayed: Vec<_> = parsed_module(&file)
        .net_delays
        .iter()
        .map(|delay| (delay.name.as_str(), delay.delay.values.len()))
        .collect();
    assert_eq!(delayed, [("t", 2)]);
    for (declaration, found) in [
        ("trireg #(0, 0, 0) t;", "not a positive decimal number"),
        ("trireg #(0, 0, d) t;", "not a positive decimal number"),
        ("trireg #(0, 0, 5, 1) t;", "at most 3"),
    ] {
        let error = parse_error(&format!(
            "module g;\n    parameter d = 5;\n    {declaration}\nendmodule\n"
        ));
        assert!(error.contains(found), "{declaration}: {error}");
    }
}

/// Section 7.14.2: the decay time is a delay in module units, rounded at the
/// module's precision; one that rounds to nothing is refused by name.
#[test]
fn a_trireg_lowers_its_decay_time_to_design_ticks() {
    use rspice_veriloga::canonical_ir::digital::{
        DigitalChargeDecay, DigitalNetType, DigitalStrengthLevel,
    };
    let plan = VerilogACompiler::default()
        .compile_canonical_ir(
            "`timescale 1ns/100ps\nmodule g;\n  trireg (large) #(0, 0, 2.5) t;\nendmodule\n",
        )
        .unwrap()
        .digital;
    assert_eq!(
        plan.strength.nets[0].net_type,
        DigitalNetType::Trireg {
            charge: DigitalStrengthLevel::Large,
            decay: Some(DigitalChargeDecay {
                ticks: 25,
                exponent: -10
            }),
        }
    );
    let error =
        lowering_error("`timescale 1ns/1ns\nmodule g;\n  trireg #(0, 0, 0.25) t;\nendmodule\n");
    assert!(error.contains("rounds to no time"), "{error}");
}

/// Two plans that differ only in a net's charge strength (section 7.14.1)
/// are different designs, so they carry different content identities.
#[test]
fn a_nets_strength_is_part_of_the_plan_identity() {
    let plan = |charge: &str| {
        VerilogACompiler::default()
            .compile_canonical_ir(&format!("module g;\n  trireg ({charge}) t;\nendmodule\n"))
            .unwrap()
            .digital
    };
    let (large, small) = (plan("large"), plan("small"));
    assert_ne!(large.content_identity, small.content_identity);
    // Nothing else differs: given the large net's strength table, the small
    // net's plan validates under the large one's identity.
    let mut swapped = small;
    swapped.content_identity = large.content_identity;
    swapped.strength = large.strength;
    swapped
        .validate()
        .expect("the plans differ only in strength");
}

/// Section 5.3: elaboration keeps the selected value of each `min : typ :
/// max` expression, whether it is one of a list of delays or a parameter's
/// value.
#[test]
fn a_min_typ_max_expression_keeps_the_selected_value() {
    use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
    for (delay_select, expected) in [
        (DigitalDelaySelect::Min, (4, 7, 1)),
        (DigitalDelaySelect::Typ, (5, 8, 2)),
        (DigitalDelaySelect::Max, (6, 9, 3)),
    ] {
        let plan = VerilogACompiler::new(CompilerOptions {
            delay_select,
            ..CompilerOptions::default()
        })
        .compile_canonical_ir(
            "`timescale 1ns/1ns\nmodule g;\n  parameter p = 1:2:3;\n  reg a;\n  wire y;\n\
             \x20 assign #(4:5:6, 7:8:9, p) y = a;\nendmodule\n",
        )
        .unwrap()
        .digital;
        let delay = plan.delays.drivers[0].delay;
        assert_eq!(
            (delay.rise, delay.fall, delay.turn_off),
            expected,
            "{delay_select:?}"
        );
    }
}

/// An array of gate instances (section 7.1.5) becomes one driver per element,
/// each taking its bit of a terminal as wide as the array and the whole of a
/// one-bit terminal (section 7.1.6).
#[test]
fn a_gate_instance_array_expands_one_driver_per_element() {
    let file = parse(
        "module g(v, e, y);\n\
         \x20   input [3:0] v; input e;\n     output [3:0] y;\n\
         \x20   bufif1 n[3:0] (y, v, e);\n\
         endmodule\n",
    );
    let module = parsed_module(&file);
    let targets: Vec<String> = module
        .continuous_assigns
        .iter()
        .map(|assignment| format!("{:?}", assignment.target))
        .collect();
    assert_eq!(targets.len(), 4, "{targets:?}");
    assert!(
        module
            .continuous_assigns
            .iter()
            .all(|assignment| assignment.gate.is_some()),
        "every element is a gated driver"
    );
    assert!(
        targets[0].contains("Number") && targets[0].contains("\"3\""),
        "{targets:?}"
    );
    assert!(targets[3].contains("\"0\""), "{targets:?}");
}

/// A terminal neither one bit nor one bit per instance wide is refused by the
/// clause, and so are a switch with a drive strength (A.3.1) and a primitive
/// with the wrong number of terminals.
#[test]
fn gate_and_switch_misuse_is_refused_by_name() {
    let error = parse_error(
        "module g(v, y);\n\
         \x20   input [2:0] v;\n     output [3:0] y;\n\
         \x20   not n[3:0] (y, v);\n\
         endmodule\n",
    );
    assert!(error.contains("section 7.1.6"), "{error}");
    let error = parse_error(
        "module g(a, c, y);\n\
         \x20   input a, c;\n     output y;\n\
         \x20   nmos (strong0, strong1) (y, a, c);\n\
         endmodule\n",
    );
    assert!(error.contains("A.3.1"), "{error}");
    let error = parse_error(
        "module g(a, y);\n\
         \x20   input a;\n     output y;\n\
         \x20   bufif1 (y, a);\n\
         endmodule\n",
    );
    assert!(error.contains("exactly 3"), "{error}");
    let error = parse_error(
        "module g(y);\n\
         \x20   output y;\n\
         \x20   pullup (strong0) (y);\n\
         endmodule\n",
    );
    assert!(error.contains("A.3.1"), "{error}");
    let error = parse_error(
        "module g(a, y);\n\
         \x20   inout a, y;\n\
         \x20   tran (strong0, strong1) (y, a);\n\
         endmodule\n",
    );
    assert!(error.contains("A.3.1"), "{error}");
    for (switch, terminals, count) in [
        ("tran", "(y, a, c)", "exactly 2"),
        ("rtranif1", "(y, a)", "exactly 3"),
    ] {
        let error = parse_error(&format!(
            "module g(a, c, y);\n\
             \x20   inout a, y; input c;\n\
             \x20   {switch} {terminals};\n\
             endmodule\n"
        ));
        assert!(error.contains(count), "{switch}: {error}");
    }
}

/// A user-defined primitive's table and instances are checked against
/// IEEE 1364-2005 section 8, each misuse refused by name.
#[test]
fn primitive_misuse_is_refused_by_name() {
    let udp = |header: &str, rows: &str| {
        format!(
            "primitive p {header}\n  table\n{rows}\n  endtable\nendprimitive\n\
             module top;\n  reg a, b; wire y;\n  p u (y, a, b);\nendmodule\n"
        )
    };
    for (source, expected) in [
        (
            udp("(o, a, b);\n  output o; input a, b;", "    r 0 : 1;"),
            "a table row of the wrong form",
        ),
        (
            udp(
                "(o, a, b);\n  output o; reg o; input a, b;",
                "    r f : ? : 1;",
            ),
            "at most one edge",
        ),
        (
            udp("(o, a, b);\n  output o; input a, b;", "    0 : 1;"),
            "1 input symbols for 2 inputs",
        ),
        (
            udp(
                "(o, a, b);\n  output o; input a, b;\n  initial o = 1'b0;",
                "    0 0 : 1;",
            ),
            "section 8.5",
        ),
        (
            "primitive p (o, a);\n  output o; input a;\n  table\n    0 : 1;\n  endtable\n\
             endprimitive\nmodule top;\n  reg a; wire y;\n  p u[1:0] (y, a);\nendmodule\n"
                .to_string(),
            "an array of primitive instances",
        ),
        (
            "primitive p (o, a);\n  output o; input a;\n  table\n    0 : 1;\n  endtable\n\
             endprimitive\nmodule top;\n  reg a; wire y;\n  p u (.o(y), .z(a));\nendmodule\n"
                .to_string(),
            "a connection to no port `z`",
        ),
        (
            "primitive p (o, a);\n  output o; input a;\n  table\n    0 : 1;\n  endtable\n\
             endprimitive\nmodule top;\n  reg a; wire y;\n  p (strong0, weak1) u (y, a);\n\
             endmodule\n"
                .to_string(),
            "a drive strength on a primitive instance",
        ),
        (
            "module m(input a);\nendmodule\nmodule top;\n  reg a;\n  m (a);\nendmodule\n"
                .to_string(),
            "no instance name",
        ),
    ] {
        let error = parse_error(&source);
        assert!(error.contains(expected), "{source}\n{error}");
    }
}

/// A table row of the wrong shape and a second definition of a primitive's
/// name are language errors (IEEE 1364-2005 sections 8.1.4 and 4.11), not
/// constructs refused by name.
#[test]
fn a_malformed_primitive_is_a_language_error() {
    let table = |header: &str, row: &str| {
        format!("primitive p({header});\n  table\n    {row};\n  endtable\nendprimitive\n")
    };
    let defined = table("output o, input a", "0 : 1");
    for (source, expected) in [
        (
            table("output o, input a", "? 1 ? 0 0 0 : 0"),
            "a row with 6 input symbols for 1 inputs",
        ),
        (
            table("output o, input a", "0 : 0 0"),
            "a row with an output that is not one symbol",
        ),
        (
            table("output reg o, input a", "0 : 0 0 : 0"),
            "a row with a state that is not one symbol",
        ),
        (
            format!("{defined}{defined}"),
            "Duplicate primitive declaration: 'p'",
        ),
    ] {
        let error = parse_error(&source);
        assert!(
            error.contains(expected) && !error.contains("Unsupported"),
            "{source}\n{error}"
        );
    }
}

/// A bidirectional switch joins two net bits, so a terminal that is an
/// expression or a whole vector is refused by the clause.
#[test]
fn a_bidirectional_switch_terminal_must_be_a_net_bit() {
    for (terminals, expected) in [
        (
            "v, w",
            "a scalar net or a bit select of a vector net (IEEE 1364-2005 section 7.6)",
        ),
        ("w[0], v[0] & w[1]", "a gate drives a net"),
    ] {
        let source = format!("module top;\n  wire [1:0] v, w;\n  tran ({terminals});\nendmodule\n");
        let message = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(&source, Some("top"))
            .expect_err(terminals)
            .to_string();
        assert!(message.contains(expected), "{terminals}: {message}");
    }
}

// ===========================================================================
// Who owns a module-level `real`
// ===========================================================================
//
// The rule is stated in full in the `digital_lower` module documentation.
// These pin its three answers, because the interesting one — the refusal — is
// the only thing standing between a mixed-signal module and two owners for one
// variable.

/// A `real` a process writes, in a module with no analog block, moves.
///
/// This is the whole point of the rule: `state` becomes a discrete-domain
/// variable of the plan, which is what lets the canonical filter idiom hold its
/// state across a clock edge.
#[test]
fn a_module_level_real_a_process_writes_becomes_a_digital_variable() {
    let plan = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(
            "module f(clk, vin, vout);\n\
             \x20   input clk;\n\
             \x20   input wreal vin;\n\
             \x20   output wreal vout;\n\
             \x20   real state;\n\
             \x20   always @(posedge clk) state <= state + (vin - state) * 0.25;\n\
             \x20   assign vout = state;\n\
             endmodule\n",
        )
        .expect("a module with no analog block may own a real in the discrete domain")
        .digital;
    let state = plan
        .signals
        .iter()
        .find(|signal| signal.name == "state")
        .expect("`state` is a discrete-domain signal now");
    assert!(state.kind.is_real(), "it carries a real");
    assert_eq!(state.width, 0, "and therefore no bits");
    assert!(
        state.procedurally_assignable,
        "it is a variable, so section 6.2 lets a process write it"
    );
    assert_eq!(
        plan.drivers_of(state.id).count(),
        0,
        "a variable has no drivers; the write goes straight into the store"
    );
}

/// A discrete-owned real read by analog equations has a transactional input
/// slot shared by every analog backend (Verilog-AMS section 7.3.6.5).
#[test]
fn a_module_level_real_the_analog_body_reads_uses_a_state_input() {
    // `digital_module`'s analog block is `I(p, n) <+ gain * V(p, n)`, so this
    // fixture is precisely the read case.
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
     \x20   always @(posedge clk) gain = gain + 1.0;",
    ));
    let module = only_module(&analyzed);
    let slot = module
        .variables
        .iter()
        .position(|variable| variable.name == "gain")
        .unwrap();
    assert!(module.variables[slot].is_state);
    assert!(module.event_state_variables.contains(&slot));
    assert!(
        module
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "gain")
    );
}

/// Port direction and a discrete variable declaration describe one symbol;
/// its analog read uses the same state-input bank as an internal variable.
#[test]
fn discrete_ports_read_by_analog_equations_use_typed_state_inputs() {
    for (declaration, process, real) in [
        ("output q; reg q;", "initial q=1;", false),
        ("output reg q;", "initial q=1;", false),
        ("input q; wire q;", "", false),
        ("output real q;", "initial q=1.5;", true),
    ] {
        let source = format!(
            "module dut(p,q); inout p; electrical p; {declaration}
            {process} analog I(p)<+q*V(p); endmodule"
        );
        let analyzed = analyze(&source);
        let module = only_module(&analyzed);
        let slot = module
            .variables
            .iter()
            .position(|variable| variable.name == "q")
            .unwrap();
        assert_eq!(
            module
                .variables
                .iter()
                .filter(|variable| variable.name == "q")
                .count(),
            1
        );
        assert!(module.event_state_variables.contains(&slot));
        let symbol = module.symbol_table.lookup("q").unwrap();
        assert_eq!(symbol.kind, rspice_veriloga::semantic::SymbolKind::Variable);
        assert_eq!(
            symbol.value_type,
            if real {
                rspice_veriloga::types::ValueType::Real
            } else {
                rspice_veriloga::types::ValueType::Integer
            }
        );
        assert!(symbol.attrs.is_state);
    }
    let error = analyze_error(
        "module dut(p,q); inout p; electrical p; output q; reg q;
        initial q=1; analog begin q=0; I(p)<+V(p); end endmodule",
    );
    assert!(
        error.contains("cannot be written by the analog body"),
        "{error}"
    );
}

/// A `real` **both** halves write is a different refusal, and a permanent one.
///
/// Section 7.3: "Write operations of nets and variables are only allowed from
/// the context of their domain." No scheduling rule makes two writers legal, so
/// the message cites the sentence rather than promising a later wave.
#[test]
fn a_module_level_real_written_by_both_domains_is_refused_by_name() {
    let error = analyze_error(
        "module dut(p, n);\n\
     \x20   inout p, n;\n\
     \x20   electrical p, n;\n\
     \x20   real gain;\n\
     \x20   wire clk;\n\
     \x20   always @(posedge clk) gain = gain + 1.0;\n\
     \x20   analog begin\n\
     \x20       gain = 2.0;\n\
     \x20       I(p, n) <+ V(p, n);\n\
     \x20   end\n\
     endmodule\n",
    );
    assert!(
        error.contains("`gain` is written by both the analog body and a discrete process")
            && error.contains("section 7.3"),
        "the refusal must name the variable and the clause, got {error:?}"
    );
}

/// And an analog body that never mentions the variable does not refuse at all.
///
/// This is the refusal that went away. The rule used to be about the *module* —
/// any analog block disqualified every module-level `real` a process wrote —
/// so a module whose two halves are simply independent was refused, with a
/// message about clocks that did not describe it.
#[test]
fn an_analog_body_that_ignores_the_variable_leaves_the_promotion_alone() {
    let analyzed = analyze(
        "module dut(p, n);\n\
     \x20   inout p, n;\n\
     \x20   electrical p, n;\n\
     \x20   real acc;\n\
     \x20   wire clk;\n\
     \x20   always @(posedge clk) acc = acc + 1.0;\n\
     \x20   analog I(p, n) <+ V(p, n) / 1000.0;\n\
     endmodule\n",
    );
    assert!(
        only_module(&analyzed)
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "acc"),
        "the process's `real` moved into the discrete domain"
    );
}

/// Reading an analog-owned real does not transfer its ownership.
#[test]
fn a_module_level_real_no_process_writes_stays_in_the_analog_domain() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&digital_module(
            "wire clk; reg q; always @(posedge clk) q=(gain>1.0);",
        ))
        .unwrap();
    assert!(
        !artifact
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "gain")
    );
    assert_eq!(
        artifact
            .hir
            .digital_observations
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<_>>(),
        vec!["gain"]
    );
}

/// `input real` is refused: only an output port may be a variable.
#[test]
fn an_input_real_port_is_refused() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            "module child(vin);\n\
             \x20   input real vin;\n\
             endmodule\n\
             module top(vin);\n\
             \x20   input wreal vin;\n\
             \x20   child u1(vin);\n\
             endmodule\n",
            Some("top"),
        )
        .expect_err("an input port cannot be a variable")
        .to_string();
    assert!(
        error.contains("only an output port be a variable"),
        "expected section 12.3.3's rule, got {error:?}"
    );
}

/// A packed range on a `real` is refused rather than read as a width.
#[test]
fn a_range_on_a_real_port_is_refused() {
    let error = analyze_error(
        "module f(vout);\n\
         \x20   output real [3:0] vout;\n\
         endmodule\n",
    );
    assert!(
        error.contains("no bit width"),
        "expected section 3.9's rule, got {error:?}"
    );
}

#[test]
fn module_integer_ownership_retains_scope_types_and_event_inputs() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(
            r#"
module owners(p); inout p; electrical p;
integer count,shadow; real hidden; reg signed [7:0] bits; reg done;
analog begin shadow=7; hidden=1; I(p)<+(count+bits)*V(p); end
initial begin
  begin : local_scope
    integer shadow; real hidden;
    shadow=2; hidden=3;
    begin : nested integer count; count=9; end
  end
  count=-3; bits=-1;
  #1 count=count+1;
  done=(shadow==7);
end
endmodule
"#,
        )
        .unwrap();
    let plan = &artifact.digital;
    let count = plan
        .signals
        .iter()
        .find(|signal| signal.name == "count")
        .unwrap();
    assert!(count.integer && count.signed && count.procedurally_assignable);
    assert_eq!((count.width, count.bounds), (32, Some((31, 0))));
    assert!(
        plan.signals
            .iter()
            .all(|signal| signal.name != "shadow" && signal.name != "hidden")
    );
    assert!(
        !plan
            .signals
            .iter()
            .find(|signal| signal.name == "bits")
            .unwrap()
            .integer
    );
    assert!(
        artifact
            .hir
            .variables
            .iter()
            .find(|variable| variable.name == "count")
            .unwrap()
            .is_state
    );
    let encoded = serde_json::to_string(plan).unwrap();
    let decoded: rspice_veriloga::canonical_ir::CanonicalDigitalPlan =
        serde_json::from_str(&encoded).unwrap();
    assert_eq!(&decoded, plan);
    decoded.validate().unwrap();
    let mut invalid = decoded;
    invalid.signals[usize::from(count.id)].signed = false;
    assert!(
        invalid.validate().is_err(),
        "an integer tag cannot certify unsigned storage"
    );

    let mut invalid_input = artifact.clone();
    invalid_input.digital.signals[usize::from(count.id)].integer = false;
    assert!(
        invalid_input
            .validate()
            .unwrap_err()
            .iter()
            .any(|error| error
                .message
                .contains("analog input 'count' has an incompatible digital type")),
        "a packed 32-bit tag cannot replace a signed integer input"
    );

    let analyzed = analyze(
        r#"
module event_inputs(p); inout p; electrical p;
integer enabled, held; real period;
initial begin enabled=1; period=2e-9; end
analog begin @(timer(0,period,0,enabled)) held=1; I(p)<+held; end
endmodule
"#,
    );
    let module = only_module(&analyzed);
    for name in ["enabled", "period"] {
        let slot = module
            .variables
            .iter()
            .position(|variable| variable.name == name)
            .unwrap();
        assert!(
            module.variables[slot].is_state,
            "{name} is read only in an event operand"
        );
        assert!(module.event_state_variables.contains(&slot));
    }
}

#[test]
fn module_integer_ownership_checks_domains_widths_and_array_storage() {
    for kind in ["real", "integer"] {
        for analog in ["analog value=2;", "analog initial value=2;"] {
            let error = analyze_error(&format!(
                "module bad; {kind} value; initial value=1; {analog} endmodule"
            ));
            assert!(
                error.contains("written by both the analog body and a discrete process"),
                "{error}"
            );
        }
    }
    // A 32-bit grouping is past Verilog-AMS 2023 Table 7-1's 31 bits, which
    // RSpice extends: the analog read is a real holding the integer value.
    for kind in ["reg", "reg signed"] {
        let analyzed = analyze(&format!(
            "module wide(p); inout p; electrical p; {kind} [31:0] bits; initial bits=1; analog I(p)<+bits; endmodule"
        ));
        let module = only_module(&analyzed);
        let slot = module
            .variables
            .iter()
            .position(|variable| variable.name == "bits")
            .unwrap();
        assert_eq!(
            module.variables[slot].value_type,
            rspice_veriloga::types::ValueType::Real
        );
    }
    let analyzed = analyze("module owned; integer values[0:1]; initial values[0]=1; endmodule");
    let array = &only_module(&analyzed).digital.signals[0];
    assert_eq!(array.name, "values");
    assert_eq!(
        array.class,
        rspice_veriloga::semantic::DigitalSignalClass::Variable(DigitalVariableKind::Integer)
    );
    assert_eq!(array.width, 32);
    assert_eq!(
        array.dimensions,
        [rspice_veriloga::semantic::VectorBounds { msb: 0, lsb: 1 }]
    );
}

#[test]
fn analog_block_ownership_respects_sequential_initializers_and_scope_exit() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(
            r#"
module analog_scopes(p); inout p; electrical p;
integer gain, later, restored; reg [63:0] hidden;
real measured;
initial begin gain=3; later=5; restored=11; hidden=1; end
analog begin
  begin : outer
    real sample=gain+later;
    integer gain=2, later=7;
    integer hidden=1;
    for (gain=0; gain<2; gain=gain+1) later=later+1;
    begin : nested
      integer restored=13;
      restored=restored+1;
      measured=sample+restored+hidden;
    end
    measured=measured+restored;
  end
  I(p)<+measured*V(p);
end
endmodule
"#,
        )
        .unwrap();
    for name in ["gain", "later", "restored"] {
        assert!(
            artifact
                .hir
                .variables
                .iter()
                .find(|variable| variable.name == name)
                .unwrap()
                .is_state,
            "{name}"
        );
    }
    assert!(
        artifact
            .hir
            .variables
            .iter()
            .all(|variable| variable.name != "hidden"),
        "the local hidden variable must not bind the 64-bit digital signal as an analog input"
    );
    assert!(
        artifact
            .hir
            .variables
            .iter()
            .any(|variable| variable.name.starts_with("hidden__blk") && !variable.is_state)
    );
    // Scope must also be restored for writes after a nested shadow has ended.
    let error = analyze_error(
        r#"
module bad; integer value; initial value=1;
analog begin
  begin : local_scope integer value; value=3; end
  value=2;
end
endmodule
"#,
    );
    assert!(
        error.contains("written by both the analog body and a discrete process"),
        "{error}"
    );
}

#[test]
fn numeric_declaration_initializers_retain_types_parameter_scope_and_identity() {
    use rspice_veriloga::canonical_ir::digital::DigitalInitialValue;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(
            r#"
module initialized(p); inout p; electrical p;
parameter real HALF=-1.5; localparam BASE=5;
integer rounded=HALF, signed_value=8'shfe, unknown='hx;
real gain=BASE/2.0, analog_only=3;
initial begin #1; rounded=rounded+1; signed_value=0; unknown=0; gain=gain+1; end
analog I(p)<+(rounded+signed_value+gain+analog_only)*V(p);
endmodule
"#,
        )
        .unwrap();
    let plan = &artifact.digital;
    let initial = |name: &str| {
        plan.signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap()
            .initial_value
            .as_ref()
            .unwrap()
    };
    for name in ["rounded", "signed_value"] {
        let DigitalInitialValue::FourState(value) = initial(name) else {
            panic!("{name}")
        };
        assert_eq!(value.width(), 32);
        assert_eq!(value.to_integer(true), Some(-2));
    }
    let DigitalInitialValue::FourState(value) = initial("unknown") else {
        panic!("unknown")
    };
    assert_eq!(value.to_integer(true), None);
    assert_eq!(initial("gain"), &DigitalInitialValue::Real(2.5));
    assert!(
        plan.signals
            .iter()
            .all(|signal| signal.name != "analog_only")
    );
    assert_eq!(
        plan.processes.len(),
        1,
        "initialization must not synthesize another process"
    );
    let encoded = serde_json::to_string(plan).unwrap();
    let decoded: rspice_veriloga::canonical_ir::CanonicalDigitalPlan =
        serde_json::from_str(&encoded).unwrap();
    assert_eq!(&decoded, plan);
    decoded.validate().unwrap();
    let mut bad = decoded;
    bad.signals
        .iter_mut()
        .find(|signal| signal.name == "gain")
        .unwrap()
        .initial_value = Some(DigitalInitialValue::FourState(
        rspice_veriloga::canonical_ir::digital_value::FourStateValue::from_integer(32, 1),
    ));
    assert!(bad.validate().is_err());
    let hierarchical = VerilogACompiler::default()
        .compile_canonical_ir_module(
            r#"
module child;
parameter BASE=5; integer state=BASE+1;
initial #1 state=state+1;
endmodule
module top;
parameter BASE=99;
child a(); child b();
endmodule
"#,
            Some("top"),
        )
        .unwrap();
    for name in ["a.state", "b.state"] {
        let signal = hierarchical
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap();
        let Some(DigitalInitialValue::FourState(value)) = &signal.initial_value else {
            panic!("{name}")
        };
        assert_eq!(
            value.to_integer(true),
            Some(6),
            "{name} must use its own parameter scope"
        );
    }
    for expression in ["other", "$time", "V(p)"] {
        let source = format!(
            "module invalid(p); inout p; electrical p; integer other; integer state={expression}; initial begin other=2; state=3; end endmodule"
        );
        assert!(
            VerilogACompiler::default()
                .compile_canonical_ir(&source)
                .is_err(),
            "nonconstant initializer {expression}"
        );
    }
}

#[test]
fn computed_and_resized_input_connections_elaborate() {
    let computed = hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(y, a & b, b);");
    plan(&computed, "top").validate().unwrap();
    let resized = hierarchy(
        "module bus1(y,a); output y; input [3:0] a; wire y; wire [3:0] a; assign y=a[0]; endmodule",
        "    wire x,z; bus1 g1(z,x);",
    );
    plan(&resized, "top").validate().unwrap();
}

#[test]
fn real_net_dimensions_do_not_consume_the_net_name_as_a_discipline() {
    for declaration in [
        "wreal cells[0:1];",
        "wreal ddiscrete cells[0:1][-1:0];",
        "wreal ddiscrete[0:1];",
        "wreal cells[INDEX[0]:1];",
        "wreal ddiscrete [3:0] cells;",
        "wrealsum cells[-1:0], more[2:3];",
    ] {
        parse(&format!("module top; {declaration} endmodule"));
    }
    for declaration in ["wreal custom cells;", "wreal custom [3:0] cells;"] {
        let error = analyze_error(&format!("module top; {declaration} endmodule"));
        assert!(
            error.contains("discipline") && error.contains("custom"),
            "{error}"
        );
    }
}

#[test]
fn inline_discrete_disciplines_and_port_prefixes_retain_the_declared_domain() {
    let definition = "discipline sense; domain discrete; enddiscipline";
    for ports in [
        "module top(r); input sense wreal r;",
        "module top(input sense wreal r);",
        "module top(r); input wreal sense r;",
        "module top(input wreal sense r);",
        "module top(r); input r; wreal sense r;",
    ] {
        let analyzed = analyze(&format!("{definition} {ports} endmodule"));
        let module = only_module(&analyzed);
        assert_eq!(module.ports[0].discipline, "sense");
        assert!(module.digital.signals[0].class.is_real());
    }
    let analyzed = analyze(&format!(
        "{definition} module top(p); output p; electrical p;
        wire sense signed [3:0] word;
        reg sense signed [3:0] data;
        wreal ddiscrete sample;
        assign word=data; assign sample=1.5;
        initial data=3;
        analog V(p)<+word+sample;
        endmodule"
    ));
    assert!(only_module(&analyzed).internal_nodes.is_empty());
    for body in [
        "wreal electrical value;",
        "wreal value; electrical value;",
        "module child(input electrical wreal value); endmodule",
    ] {
        let source = if body.starts_with("module") {
            body.to_string()
        } else {
            format!("module top; {body} endmodule")
        };
        let error = analyze_error(&source);
        assert!(error.contains("requires a discrete discipline"), "{error}");
    }
    let error = analyze_error(&format!(
        "{definition} module top; wreal sense value; logic value; endmodule"
    ));
    assert!(error.contains("Incompatible disciplines"), "{error}");
}

// ===========================================================================
// System tasks and the IEEE 1800 statement forms (design P2 sections 1.2, 1.3)
// ===========================================================================

/// The statements of one `initial` block, parsed.
fn initial_statements(section: &str) -> Vec<DigitalStatement> {
    let file = parse(&digital_module(&format!(
        "    reg [7:0] q; integer i;\n    initial begin {section} end"
    )));
    let module = parsed_module(&file);
    let DigitalStatement::Block(block) = &module.digital_processes[0].body else {
        panic!("expected a block body");
    };
    block.statements.clone()
}

/// IEEE 1364-2005 section 17: a system task enable is a statement, with or
/// without arguments, and an empty argument is kept in its position.
#[test]
fn system_task_enables_parse_with_their_arguments() {
    let statements =
        initial_statements("$display(\"q=%d\", q,, 3); $finish; $finish(2); $monitoron;");
    let tasks: Vec<(&str, usize)> = statements
        .iter()
        .map(|statement| match statement {
            DigitalStatement::SystemTask(task) => (task.name.as_str(), task.args.len()),
            other => panic!("expected a system task, got {other:?}"),
        })
        .collect();
    assert_eq!(
        tasks,
        [
            ("$display", 4),
            ("$finish", 0),
            ("$finish", 1),
            ("$monitoron", 0)
        ]
    );
    let DigitalStatement::SystemTask(display) = &statements[0] else {
        unreachable!()
    };
    assert!(matches!(display.args[0], Expression::StringLit(_)));
    assert!(matches!(display.args[2], Expression::NullArgument(_)));
}

/// IEEE 1800-2017 section 11.4.1-2's statement forms, which both reference
/// simulators accept in a `.v` source (design P2 G2): each is the blocking
/// assignment it abbreviates.
#[test]
fn compound_and_step_assignments_desugar_to_blocking_assignments() {
    let statements =
        initial_statements("q <<= 1; q += 2; q >>>= 1; ++i; i++; --i; i--; q[3:0] |= 4'h5;");
    let forms: Vec<String> = statements
        .iter()
        .map(|statement| {
            let DigitalStatement::BlockingAssign(assign) = statement else {
                panic!("expected a blocking assignment, got {statement:?}");
            };
            assert!(assign.timing.is_none());
            match &assign.value {
                Expression::Binary(binary) => format!("{:?}", binary.op),
                Expression::Digital(DigitalExpr::ArithmeticShiftRight(_)) => ">>>".into(),
                other => panic!("unexpected right-hand side {other:?}"),
            }
        })
        .collect();
    assert_eq!(
        forms,
        ["Shl", "Add", ">>>", "Add", "Add", "Sub", "Sub", "BitOr"]
    );
    // The target is read back as the left operand.
    let DigitalStatement::BlockingAssign(assign) = &statements[0] else {
        unreachable!()
    };
    let Expression::Binary(binary) = &assign.value else {
        unreachable!()
    };
    assert!(matches!(&*binary.left, Expression::Identifier(identifier) if identifier.name == "q"));
}

#[test]
fn a_for_header_accepts_the_step_forms() {
    for header in [
        "i = 0; i < 4; i++",
        "i = 0; i < 4; ++i",
        "i = 0; i < 4; i += 1",
    ] {
        let statements = initial_statements(&format!("for ({header}) q = q + 1;"));
        let DigitalStatement::For(statement) = &statements[0] else {
            panic!("expected a for loop for `{header}`");
        };
        assert!(
            matches!(statement.update.value, Expression::Binary(_)),
            "{header}"
        );
    }
}

/// A compound assignment has no intra-assignment timing form, and two
/// separated tokens are not an operator.
#[test]
fn malformed_compound_assignments_are_refused() {
    let message = parse_error(&digital_module("    reg q;\n    initial q += #1 2;"));
    assert!(
        message.contains("intra-assignment timing control"),
        "{message}"
    );
    let message = parse_error(&digital_module("    reg q;\n    initial q + = 2;"));
    assert!(!message.is_empty());
}

/// A stray `;` after `endmodule` is IEEE 1800's empty description item, which
/// both reference simulators accept (C3 `delay.v`).
#[test]
fn a_semicolon_after_endmodule_is_an_empty_item() {
    let file =
        parse("module m; reg q; initial q = 0; endmodule; // delay\nmodule n; endmodule;;\n");
    assert_eq!(file.items.len(), 2);
}

/// Design P2 section 1.5: a name outside IEEE 1364-2005 section 17 and the
/// named 1800 extensions is refused by name, and a task and a function cannot
/// stand in for each other.
#[test]
fn system_calls_outside_the_subset_are_refused_by_name() {
    for (call, expected) in [
        ("$foo(q);", "`$foo` is not a system task or function"),
        ("$countdrivers(q);", "annex C informative task"),
        (
            "$dumpports(q, \"p.evcd\");",
            "`$dumpports` is an extended value change dump task (IEEE 1364-2005 section 18.3)",
        ),
        (
            "$dumpportsoff;",
            "`$dumpportsoff` is an extended value change dump",
        ),
        (
            "$sdf_annotate(\"x.sdf\");",
            "`$sdf_annotate` is not supported yet: SDF back-annotation",
        ),
        ("q = $display;", "is a system task and returns no value"),
        ("$clog2(q);", "is a system function and returns a value"),
        (
            "q = $dist_uniform(i, 1);",
            "takes 3 argument(s), and was given 2",
        ),
        ("$finish(1, 2);", "takes 0 to 1 argument(s)"),
        ("q = $random(i + 1);", "must name a variable"),
    ] {
        let message = analyze_error(&digital_module(&format!(
            "    reg [7:0] q; integer i;\n    initial begin {call} end"
        )));
        assert!(message.contains(expected), "`{call}`: {message}");
    }
}

/// Every design P2 section 1.3 kind is accepted by the front end, including
/// the file and waveform tasks whose host behaviour lane L5 adds.
#[test]
fn every_registered_system_call_is_accepted() {
    analyze(&digital_module(
        "    reg [7:0] q, mem [0:3]; integer i, fd; real r;\n\
         \x20   initial begin\n\
         \x20     $display(q); $write(q); $displayb(q); $writeh(q); $strobe(q); $monitoro(q);\n\
         \x20     $monitoron; $monitoroff; $timeformat(-9, 2, \" ns\", 10); $printtimescale;\n\
         \x20     $info(\"i\"); $warning(\"w\"); $error(\"e\"); $stop; $finish(0);\n\
         \x20     i = $random; i = $random(i); i = $dist_uniform(i, 0, 9); i = $dist_normal(i, 0, 1);\n\
         \x20     i = $dist_exponential(i, 3); i = $dist_poisson(i, 3); i = $dist_chi_square(i, 3);\n\
         \x20     i = $dist_t(i, 3); i = $dist_erlang(i, 2, 3); i = $urandom; i = $urandom_range(4, 1);\n\
         \x20     i = $test$plusargs(\"V\"); i = $value$plusargs(\"N=%d\", q);\n\
         \x20     fd = $fopen(\"f.txt\", \"w\"); $fdisplay(fd, q); $fwrite(fd, q); $fstrobe(fd, q);\n\
         \x20     $fmonitor(fd, q); $fflush(fd); i = $fgetc(fd); i = $ungetc(i, fd); i = $fgets(q, fd);\n\
         \x20     i = $fscanf(fd, \"%d\", q); i = $sscanf(q, \"%d\", i); i = $fread(mem, fd);\n\
         \x20     i = $ftell(fd); i = $fseek(fd, 0, 0); i = $rewind(fd); i = $feof(fd); i = $ferror(fd, q);\n\
         \x20     $fclose(fd); $sformat(q, \"%d\", i); $swrite(q, i); $readmemh(\"m.hex\", mem);\n\
         \x20     $readmemb(\"m.bin\", mem, 0, 3); $dumpfile(\"d.vcd\"); $dumpvars(0, q); $dumpon;\n\
         \x20     $dumpoff; $dumpall; $dumplimit(100); $dumpflush; $fatal(1, \"f\");\n\
         \x20     r = $sin(r) + $ln(2.0) + $pow(r, 2.0) + $atan2(r, 1.0) + $hypot(r, r) + $itor(i);\n\
         \x20     i = $rtoi(r) + $clog2(q) + $signed(q) + $unsigned(i);\n\
         \x20   end",
    ));
}

// ===========================================================================
// Tasks, functions and thread statements (IEEE 1364-2005 sections 9.7, 9.8.2,
// 10.2-10.4)
// ===========================================================================

/// Both argument styles, `automatic`, a typed function result, and the
/// `time`/`realtime`/`event` declarations parse into the module's routines
/// and variables.
#[test]
fn tasks_and_functions_parse_in_both_argument_styles() {
    let file = parse(
        "module top;\n\
         \x20 time t0; realtime r0; event go;\n\
         \x20 task t(input [7:0] a, b, output reg c);\n\
         \x20   reg [3:0] scratch;\n\
         \x20   begin c = a[0] ^ b[0]; scratch = 4'd1; end\n\
         \x20 endtask\n\
         \x20 task old;\n\
         \x20   input integer n;\n\
         \x20   output [1:0] q;\n\
         \x20   q = n;\n\
         \x20 endtask\n\
         \x20 function automatic integer fact(input integer n);\n\
         \x20   if (n <= 1) fact = 1; else fact = n * fact(n - 1);\n\
         \x20 endfunction\n\
         \x20 function [7:0] inc; input [7:0] v; inc = v + 1; endfunction\n\
         endmodule\n",
    );
    let module = parsed_module(&file);
    let names: Vec<_> = module
        .digital_routines
        .iter()
        .map(|routine| {
            (
                routine.name.as_str(),
                routine.automatic,
                routine.is_function(),
            )
        })
        .collect();
    assert_eq!(
        names,
        vec![
            ("t", false, false),
            ("old", false, false),
            ("fact", true, true),
            ("inc", false, true),
        ]
    );
    let t = &module.digital_routines[0];
    let directions: Vec<_> = t
        .arguments
        .iter()
        .map(|argument| (argument.name.as_str(), argument.direction))
        .collect();
    assert_eq!(
        directions,
        vec![
            ("a", PortDirection::Input),
            ("b", PortDirection::Input),
            ("c", PortDirection::Output),
        ]
    );
    assert!(t.arguments[1].ty.range.is_some(), "`b` continues `[7:0]`");
    assert_eq!(t.digital_variables.len(), 1);
    let old = &module.digital_routines[1];
    assert_eq!(old.arguments[0].ty.kind, DigitalRoutineValueKind::Integer);
    assert_eq!(old.arguments[1].direction, PortDirection::Output);
    let DigitalRoutineKind::Function(result) = &module.digital_routines[2].kind else {
        panic!("fact is a function");
    };
    assert_eq!(result.kind, DigitalRoutineValueKind::Integer);
    let kinds: Vec<_> = module
        .digital_variables
        .iter()
        .map(|declaration| (declaration.kind, declaration.range.is_some()))
        .collect();
    assert_eq!(
        kinds,
        vec![
            (DigitalVariableKind::Reg, true),
            (DigitalVariableKind::Event, false)
        ],
        "`time` is a 64-bit reg and `event` its own kind"
    );
    assert!(module.variables.iter().any(|declaration| {
        declaration.var_type == VarType::Real && declaration.items[0].name == "r0"
    }));
}

/// The thread statements each parse to their own node, including `@name`
/// without parentheses and a task enable with and without arguments.
#[test]
fn thread_statements_parse_to_their_own_nodes() {
    let file = parse(
        "module top;\n\
         \x20 reg q; event go;\n\
         \x20 task pulse; q = 1; endtask\n\
         \x20 initial begin : body\n\
         \x20   pulse;\n\
         \x20   pulse();\n\
         \x20   fork : both #1 q = 0; @go q = 1; join\n\
         \x20   wait (q) q = 0;\n\
         \x20   wait (q);\n\
         \x20   -> go;\n\
         \x20   disable body;\n\
         \x20 end\n\
         endmodule\n",
    );
    let module = parsed_module(&file);
    let DigitalStatement::Block(block) = &module.digital_processes[0].body else {
        panic!("the process body is a block");
    };
    let kinds: Vec<&str> = block
        .statements
        .iter()
        .map(|statement| match statement {
            DigitalStatement::TaskEnable(_) => "task",
            DigitalStatement::Fork(_) => "fork",
            DigitalStatement::Wait(_) => "wait",
            DigitalStatement::EventTrigger(_) => "->",
            DigitalStatement::Disable(_) => "disable",
            _ => "other",
        })
        .collect();
    assert_eq!(
        kinds,
        vec!["task", "task", "fork", "wait", "wait", "->", "disable"]
    );
    let DigitalStatement::Fork(fork) = &block.statements[2] else {
        unreachable!()
    };
    assert_eq!(fork.name.as_deref(), Some("both"));
    assert_eq!(fork.statements.len(), 2, "each statement is a branch");
}

/// IEEE 1800 fork forms are refused by name.
#[test]
fn ieee_1800_fork_forms_are_refused_by_name() {
    for (statement, what) in [
        ("fork q = 1; join_any", "join_any"),
        ("fork q = 1; join_none", "join_none"),
        ("disable fork;", "disable fork"),
    ] {
        let message = parse_error(&format!(
            "module top; reg q; initial begin {statement} end endmodule\n"
        ));
        assert!(
            message.contains(what),
            "expected `{what}` refused, got {message:?}"
        );
    }
}

/// The analyzer resolves routine names, arity, argument targets and the
/// section 10.4.4 function restrictions, and keeps named events out of
/// expressions and edges.
#[test]
fn routines_and_events_are_checked_where_they_are_used() {
    let analyzed = analyze(
        "module top;\n\
         \x20 reg [7:0] r; event go;\n\
         \x20 task t(input [7:0] a, output [7:0] b); #1 b = a; endtask\n\
         \x20 function [7:0] f(input [7:0] v); f = v + 1; endfunction\n\
         \x20 always t(r, r);\n\
         \x20 initial begin r = f(r); -> go; @go r = 0; end\n\
         endmodule\n",
    );
    let module = only_module(&analyzed);
    assert_eq!(module.digital.routines.len(), 2);
    assert!(module.digital.routines[0].suspends, "t has a delay");
    assert!(!module.digital.routines[1].suspends);
    for (body, expected) in [
        (
            "function f(input a); #1 f = a; endfunction initial r = f(1);",
            "section 10.4.4",
        ),
        (
            "task t(input a); endtask initial t(1, 2);",
            "takes 1 argument and is given 2",
        ),
        (
            "task t(output a); endtask initial t(1);",
            "must be a variable",
        ),
        ("initial r = go;", "is a named event"),
        ("initial @(posedge go) r = 0;", "has no edges"),
        ("initial -> r;", "not a named event"),
        ("initial disable nowhere;", "nowhere"),
        ("task t; r = 0; endtask always t;", "no timing control"),
        ("always r <= @go 1'b1;", "no timing control"),
        (
            "task automatic t; reg l; l <= #1 0; endtask",
            "a nonblocking assignment cannot write `l`",
        ),
        (
            "initial -> late; event late;",
            "used before its declaration",
        ),
    ] {
        let message = analyze_error(&format!(
            "module top; reg r; event go;\n{body}\nendmodule\n"
        ));
        assert!(
            message.contains(expected),
            "expected {expected:?} for {body:?}, got {message:?}"
        );
    }
}

/// IEEE 1364-2005 forms Icarus `-g2005` also rejects: a routine port with no
/// direction (section 10.2.1), a declaration in an unnamed `fork` (section
/// 9.8.2) and a `time` redeclaration of a ranged port (section 12.3.3). A
/// later `real x;` types the port `input x;` instead (section 12.3.3), and an
/// attribute may stand between a function's name and its arguments.
#[test]
fn routine_ports_and_fork_declarations_follow_ieee_1364() {
    for (source, expected) in [
        (
            "module m; task t(a, b); endtask endmodule",
            "section 10.2.1",
        ),
        (
            "module m; task t(input integer a, integer b); endtask endmodule",
            "section 10.2.1",
        ),
        (
            "module m; initial fork integer x; join endmodule",
            "section 9.8.2",
        ),
        (
            "module m(x); output [63:0] x; time x; endmodule",
            "section 12.3.3",
        ),
        (
            "module m; task t; event e; endtask endmodule",
            "a named event declared inside a task",
        ),
        (
            "module m; task t; input [7:0] x; reg [3:0] x; endtask endmodule",
            "Duplicate",
        ),
        (
            "module m; task t; input [63:0] x; time x; endtask endmodule",
            "Duplicate",
        ),
        (
            "module m; task t; input x; reg x; reg x; endtask endmodule",
            "Duplicate",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect_err("the form must be rejected")
            .to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
    let file = parse(
        "module m; reg r;\n\
         \x20 task t; input x; real x; input [3:0] y; reg [3:0] y; endtask\n\
         \x20 function f(input a); f = a; endfunction\n\
         \x20 initial r = f (* keep *) (1'b1);\n\
         endmodule\n",
    );
    let Item::Module(module) = &file.items[0] else {
        panic!("a module");
    };
    let routine = &module.digital_routines[0];
    assert_eq!(routine.arguments.len(), 2);
    assert!(routine.variables.is_empty() && routine.digital_variables.is_empty());
    assert_eq!(routine.arguments[0].ty.kind, DigitalRoutineValueKind::Real);
    assert!(routine.arguments[1].ty.range.is_some());
}

/// IEEE 1364-2005 section 12.3.3 declares every port of the list of ports and
/// gives a port's two declarations one range, and section 4.5 declares a name
/// implicitly only as a net and only where no declaration precedes it, so a
/// name used above its declaration is undeclared there.
#[test]
fn ports_and_names_are_declared_as_ieee_1364_declares_them() {
    for (source, expected) in [
        (
            "module m(x); output [3:0] x; reg [7:0] x; initial x = 0; endmodule",
            "declared with range [3:0] and redeclared with range [7:0]",
        ),
        (
            "module m(x); output x; reg [7:0] x; initial x = 0; endmodule",
            "declared without a range and redeclared with range [7:0]",
        ),
        (
            "module m(x); output [7:0] x; reg x; initial x = 0; endmodule",
            "declared with range [7:0] and redeclared without a range",
        ),
        (
            "module m(x); output [31:0] x; integer x; initial x = 0; endmodule",
            "redeclared as an `integer`, which has no range",
        ),
        (
            "module m(t); time t; output [63:0] t; initial t = 0; endmodule",
            "redeclared as a `time`, which has no range",
        ),
        (
            "module m(x); reg [3:0] x; input [1:0] x; wire y = x[0]; endmodule",
            "declared with range [1:0] and redeclared with range [3:0]",
        ),
        (
            "module m(a, b); input a; wire c = a; endmodule",
            "Port 'b' has no direction declaration",
        ),
        (
            "module m; initial v = 1; reg v; endmodule",
            "`v` is used before its declaration",
        ),
        (
            "module m; initial $display(w); localparam w = 8'hAA; endmodule",
            "`w` is used before its declaration",
        ),
        (
            "module m; assign w = 1; wire [7:0] w; endmodule",
            "`w` is used before its declaration",
        ),
        (
            "module m; wire o; assign o = ~i; assign i = 1'b0; endmodule",
            "`i` is used before its declaration",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect_err("the form must be rejected")
            .to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
    for source in [
        "module m(x); output [3:0] x; reg [3:0] x; initial x = 0; endmodule",
        "module m(x); parameter N = 4; output [N-1:0] x; reg [3:0] x; initial x = 0; endmodule",
        "module m(x); output x; integer x; initial x = 0; endmodule",
        "module m(t); output t; time t; initial t = 0; endmodule",
        "module m(t); output [63:0] t; reg [63:0] t; initial t = 0; endmodule",
        "module m(y, a); output y; assign y = a; input a; endmodule",
        "module m; wire o; assign i = 1'b0; assign o = ~i; endmodule",
        "module m; reg v; initial begin : b reg w; w = v; end endmodule",
    ] {
        if let Err(error) =
            VerilogACompiler::new(CompilerOptions::default()).compile_canonical_ir(source)
        {
            panic!("{source}: {error}");
        }
    }
}

/// IEEE 1364-2005 section 5.1.14 allows no unsized constant number in a
/// concatenation, and section 5.1.1 no real operand; a replication's count is
/// no operand, and a sized number or an untyped parameter is one.
#[test]
fn a_concatenation_takes_sized_operands_that_are_not_real() {
    for (source, expected) in [
        (
            "module m; parameter p = 1; initial $display(\"%d\", {p, 2}); endmodule",
            "the unsized constant `2`",
        ),
        (
            "module m; reg [7:0] r; initial r = {4{'b1}}; endmodule",
            "the unsized constant `'b1`",
        ),
        (
            "module m; real x; reg [7:0] r; initial begin x = 1.0; r = {x, 1'b0}; end endmodule",
            "the real value `x`",
        ),
        (
            "module m; parameter real a = 1.0; parameter real b = {a, 2.0}; endmodule",
            "the real value",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect_err("the operand must be rejected")
            .to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
    for source in [
        "module m; parameter p = 1; reg [40:0] r; initial r = {p, 4'd2, {2{1'b1}}}; endmodule",
        "module m; reg [7:0] r; initial r = {8{1'b1}}; endmodule",
        "module m; parameter [7:0] p = {4'h1, 4'h2}; reg [7:0] r; initial r = p; endmodule",
    ] {
        if let Err(error) =
            VerilogACompiler::new(CompilerOptions::default()).compile_canonical_ir(source)
        {
            panic!("{source}: {error}");
        }
    }
}

/// The parser's IEEE 1364-2005 rules: a unary operator's operand is a primary
/// (A.8.3), so `~ |a` is not `~|a` and no reduction follows a unary operator,
/// while a stacked negation stays readable as compact models write it; only a
/// named `begin` block declares
/// (section 9.8.1); a named block of a process and a generate block of the
/// module do not share a name (section 12.7); and an attribute's value is a
/// constant expression (section 3.8).
#[test]
fn unary_operands_block_names_and_attribute_values_follow_ieee_1364() {
    for (source, expected) in [
        (
            "module m; reg r; reg [1:0] a; initial r = ~ |a; endmodule",
            "a primary as the operand of a unary operator",
        ),
        (
            "module m; reg r; reg [1:0] a; initial r = !&a; endmodule",
            "a primary as the operand of a unary operator",
        ),
        (
            "module m; reg r; reg [1:0] a; initial r = ~ ~|a; endmodule",
            "a primary as the operand of a unary operator",
        ),
        (
            "module m; initial begin integer x; x = 1; end endmodule",
            "an unnamed `begin` block declares nothing",
        ),
        (
            "module m;\n  generate if (1) begin : b wire w; end endgenerate\n  \
             initial begin : b reg r; r = 0; end\nendmodule",
            "the name of a generate block of the module",
        ),
        (
            "module m; integer n; (* attr = n *) initial n = 0; endmodule",
            "a constant expression as the value of attribute `attr`",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect_err("the form must be rejected")
            .to_string();
        assert!(error.contains(expected), "{source}: {error}");
    }
    for source in [
        "module m; reg r; reg [1:0] a; initial begin r = ~|a; r = ~&a; r = ~(|a); r = -(-a); end endmodule",
        "module m; integer r; initial r = 2 - -1; endmodule",
        "module m; integer r; initial r = - -1; endmodule",
        "module m; initial begin : named integer x; x = 1; end endmodule",
        "module m;\n  generate if (1) begin : g wire w; end endgenerate\n  \
         initial begin : b reg r; r = 0; end\nendmodule",
        "module m; parameter p = 1; (* attr = p *) initial $display(p); endmodule",
    ] {
        if let Err(error) =
            VerilogACompiler::new(CompilerOptions::default()).compile_canonical_ir(source)
        {
            panic!("{source}: {error}");
        }
    }
}

/// IEEE 1364-2005 section 5.2.1: `+:` and `-:` are one token each, select at
/// a run-time base with a positive constant width, of a vector or of one
/// array element after all its coordinates, and a continuous target's base is
/// constant (section A.8.5).
#[test]
fn indexed_part_selects_parse_and_check_their_width_base_and_target() {
    let file = parse(
        "module m; reg [7:0] v; reg [7:0] mem [0:3]; integer i; reg [1:0] q;\n\
         \x20 initial begin q = v[i +: 2]; q = mem[i][i -: 2]; v[i+1 -: 2] = q; {mem[0][i +: 2], v[0 +: 2]} = 4'h0; end\n\
         endmodule\n",
    );
    let module = parsed_module(&file);
    let DigitalStatement::Block(block) = &module.digital_processes[0].body else {
        panic!("expected a block");
    };
    let DigitalStatement::BlockingAssign(read) = &block.statements[0] else {
        panic!("expected an assignment");
    };
    let Expression::Digital(DigitalExpr::IndexedPartSelect(select)) = &read.value else {
        panic!("expected an indexed part-select, got {:?}", read.value);
    };
    assert!(select.ascending && select.indices.is_empty());
    let DigitalStatement::BlockingAssign(element) = &block.statements[1] else {
        panic!("expected an assignment");
    };
    let Expression::Digital(DigitalExpr::IndexedPartSelect(select)) = &element.value else {
        panic!("expected an indexed part-select of an element");
    };
    assert!(!select.ascending && select.indices.len() == 1);
    let DigitalStatement::BlockingAssign(write) = &block.statements[2] else {
        panic!("expected an assignment");
    };
    assert!(matches!(
        &write.target,
        DigitalLValue::IndexedPartSelect(select) if !select.ascending
    ));
    analyze(
        "module m; reg [7:0] v; reg [7:0] mem [0:3]; integer i; reg [1:0] q; wire [7:0] w;\n\
         \x20 parameter P = 2; assign w[P +: 2] = q;\n\
         \x20 initial begin q = v[i +: P]; mem[i][i -: 2] = q; end\n\
         endmodule\n",
    );
    assert!(
        parse_error("module m; reg [7:0] v; reg q; initial q = v[0 + : 1]; endmodule")
            .contains("Invalid expression")
    );
    for (source, expected) in [
        (
            "module m; reg [7:0] v; integer w; reg q; initial q = |v[0 +: w]; endmodule",
            "positive constant",
        ),
        (
            "module m; reg [7:0] v; reg q; initial q = |v[1 +: 0]; endmodule",
            "positive constant",
        ),
        (
            "module m; reg s, q; initial q = s[0 +: 1]; endmodule",
            "a scalar has no selectable bits",
        ),
        (
            "module m; reg [7:0] mem [0:3]; reg q; initial q = mem[0 +: 1]; endmodule",
            "follows 0 unpacked indices",
        ),
        (
            "module m; wire [7:0] w; integer i; reg [1:0] q; assign w[i +: 2] = q; endmodule",
            "continuous bit driver",
        ),
        (
            "module m; reg [7:0] v; reg q; initial q = v[nope +: 1]; endmodule",
            "nope",
        ),
    ] {
        let error = analyze_error(source);
        assert!(error.contains(expected), "{source}: {error}");
    }
}

/// IEEE 1364-2005 section 13.3.1: a `config` is a file-scope declaration
/// beside the modules, with one `design` statement and its rules.
#[test]
fn a_config_declaration_parses_beside_the_modules() {
    let parsed = parse(
        "module leaf; endmodule\n\
         config cfg;\n\
           design work.top;\n\
           default liblist rtlLib gateLib;\n\
           instance top.u1.u2 use gateLib.leaf;\n\
           instance top.u3 liblist gateLib;\n\
           cell leaf use rtlLib.leaf;\n\
           cell gateLib.adder liblist gateLib work;\n\
           cell top use mid;\n\
         endconfig\n\
         module top; endmodule\n",
    );
    assert_eq!(parsed.items.len(), 2, "both modules remain items");
    let [config] = parsed.configs.as_slice() else {
        panic!("one configuration: {:?}", parsed.configs);
    };
    assert_eq!(config.name, "cfg");
    assert_eq!(
        config.library, "work",
        "a source without a marker is in `work`"
    );
    let cell = |library: Option<&str>, cell: &str| (library.map(str::to_owned), cell.to_owned());
    let written = |value: &LibraryCell| {
        (
            value.library.as_ref().map(|library| library.to_string()),
            value.cell.to_string(),
        )
    };
    assert_eq!(
        config.design.iter().map(written).collect::<Vec<_>>(),
        [cell(Some("work"), "top")]
    );
    let rules: Vec<String> = config
        .rules
        .iter()
        .map(|rule| {
            let selection = match &rule.selection {
                ConfigSelection::Default => "default".to_owned(),
                ConfigSelection::Instance(path) => format!(
                    "instance {}",
                    path.iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(".")
                ),
                ConfigSelection::Cell(value) => {
                    let (library, name) = written(value);
                    format!("cell {}{name}", library.map_or(String::new(), |l| l + "."))
                }
            };
            let expansion = match &rule.expansion {
                ConfigExpansion::Liblist(libraries) => format!(
                    "liblist {}",
                    libraries
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                ConfigExpansion::Use {
                    cell: value,
                    configuration,
                } => {
                    let (library, name) = written(value);
                    format!(
                        "use {}{name}{}",
                        library.map_or(String::new(), |l| l + "."),
                        if *configuration { ":config" } else { "" }
                    )
                }
            };
            format!("{selection} {expansion}")
        })
        .collect();
    assert_eq!(
        rules,
        [
            "default liblist rtlLib gateLib",
            "instance top.u1.u2 use gateLib.leaf",
            "instance top.u3 liblist gateLib",
            "cell leaf use rtlLib.leaf",
            "cell gateLib.adder liblist gateLib work",
            "cell top use mid",
        ]
    );
}

/// None of a configuration's words is reserved outside one: a Verilog-A
/// source may still name a net `cell`, `design`, `use`, `instance`,
/// `library` or `config`.
#[test]
fn configuration_words_stay_identifiers_outside_a_config() {
    let parsed = parse(
        "`include \"disciplines.vams\"\n\
         module m(config, cell);\n\
           inout config, cell; electrical config, cell, design, use, instance, library;\n\
           analog V(config, cell) <+ 0;\n\
         endmodule\n",
    );
    assert!(parsed.configs.is_empty());
    assert_eq!(parsed.items.len(), 1);
}

#[test]
fn a_malformed_config_is_a_parse_error() {
    for (source, expected) in [
        (
            "config cfg; default liblist a; endconfig",
            "`design` statement",
        ),
        (
            "config cfg; design top; default use a.b; endconfig",
            "`default` clause",
        ),
        (
            "config cfg; design top; cell leaf; endconfig",
            "`liblist` or `use`",
        ),
        (
            "config cfg; design top; endconfig : other",
            "closes configuration `cfg`",
        ),
        (
            "config cfg; localparam P = 1; design top; endconfig",
            "localparam",
        ),
        (
            "config cfg; design top; instance top.u1 use a.b;",
            "end of input",
        ),
    ] {
        let error = parse_error(source);
        assert!(error.contains(expected), "{source}: {error}");
    }
    // IEEE 1800-2017's closing label is accepted when it names the config.
    assert_eq!(
        parse("config cfg; design top; endconfig : cfg")
            .configs
            .len(),
        1
    );
}

/// IEEE 1364-2005 section 14.6: a path's own `PATHPULSE$input$output` sets
/// its pulse limits over the module's `PATHPULSE$`, a path with several
/// terminals takes its first input's and first output's, one limit is both,
/// and an error limit under the reject limit is the reject limit (as SDF
/// reads it); a pulse style marks every path to its output, the last of
/// each pair deciding.
#[test]
fn specify_pulse_limits_and_styles_lower_onto_each_path() {
    let plan = plan(
        "`timescale 1ns/1ns\n\
         module leaf(input a, input b, input c, output y, output z);\n\
         \x20 assign y = a & b;\n\
         \x20 assign z = c;\n\
         \x20 specify\n\
         \x20   pulsestyle_ondetect y;\n\
         \x20   showcancelled y, z;\n\
         \x20   noshowcancelled z;\n\
         \x20   specparam PATHPULSE$ = (1, 4), PATHPULSE$a$y = 2, PATHPULSE$c$z = (3, 1);\n\
         \x20   (a, b *> y) = 5;\n\
         \x20   (c => z) = 5;\n\
         \x20 endspecify\n\
         endmodule\n\
         module other(input a, output y);\n\
         \x20 assign y = a;\n\
         \x20 specify (a => y) = 5; endspecify\n\
         endmodule\n\
         module top;\n\
         \x20 reg a, b, c;\n\
         \x20 wire y, z, w;\n\
         \x20 leaf u(a, b, c, y, z);\n\
         \x20 other v(a, w);\n\
         endmodule\n",
        "top",
    );
    let pulses: Vec<_> = plan
        .delays
        .paths
        .iter()
        .map(|path| (path.pulse_limits, path.on_detect, path.show_cancelled))
        .collect();
    assert_eq!(
        pulses,
        [
            (Some([2, 2]), true, true),
            (Some([2, 2]), true, true),
            (Some([3, 3]), false, false),
            (None, false, false),
        ]
    );
}

/// A SystemVerilog real-valued net (`wire real`, IEEE 1800-2017 section
/// 6.7.1) is refused by name, pointing at the Verilog-AMS `wreal`.
#[test]
fn a_wire_real_net_is_refused_by_name() {
    let error = parse_error("module m;\n  wire real r;\nendmodule\n");
    assert!(
        error.contains("Unsupported net data type: `wire real`") && error.contains("`wreal`"),
        "{error}"
    );
}

// ===========================================================================
// Section 14 `specify` blocks
// ===========================================================================

/// IEEE 1364-2005 sections 14.2 and 14.3: each source and destination pair of
/// a path declaration is one module path in the plan, on the instance's
/// signals; a specparam names a delay; two delays become twelve by table
/// 14-3 and section 14.3.2; a condition is the net the module drives from
/// it; and a variable output port's path delays the net its connection
/// drives.
#[test]
fn specify_paths_lower_one_path_per_source_and_destination() {
    let plan = plan(
        "`timescale 1ns/1ns\n\
         module leaf(input a, input b, input s, input c, output y, output reg q);\n\
         \x20 assign y = a & b;\n\
         \x20 always @(posedge c) q <= a;\n\
         \x20 specify\n\
         \x20   specparam tr = 1, tf = 2;\n\
         \x20   (a, b *> y) = (tr, tf);\n\
         \x20   if (s) (a => y) = 3;\n\
         \x20   ifnone (a => y) = 4;\n\
         \x20   (posedge c => (q -: a)) = 5;\n\
         \x20 endspecify\n\
         endmodule\n\
         module top;\n\
         \x20 reg a, b, s, c;\n\
         \x20 wire y, q;\n\
         \x20 leaf u(a, b, s, c, y, q);\n\
         endmodule\n",
        "top",
    );
    let name = |id| plan.signal(id).unwrap().name.to_string();
    let paths: Vec<_> = plan
        .delays
        .paths
        .iter()
        .map(|path| {
            (
                name(path.source),
                name(path.destination),
                path.edge.map(|edge| edge.keyword()),
                path.condition.map(&name),
                path.ifnone,
                path.delay.transitions[..2].to_vec(),
            )
        })
        .collect();
    assert_eq!(
        paths,
        [
            ("a".into(), "y".into(), None, None, false, vec![1, 2]),
            ("b".into(), "y".into(), None, None, false, vec![1, 2]),
            (
                "a".into(),
                "y".into(),
                None,
                Some("u.specify$if0".into()),
                false,
                vec![3, 3]
            ),
            ("a".into(), "y".into(), None, None, true, vec![4, 4]),
            (
                "c".into(),
                "q".into(),
                Some("posedge"),
                None,
                false,
                vec![5, 5]
            ),
        ]
    );
    assert_eq!(
        plan.delays.paths[0].delay.transitions,
        [1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 2, 1]
    );
}

/// What `specify` does not run yet is refused by name: timing checks
/// (section 15). A path runs from an input or inout port to an output or
/// inout port (section 14.2.1) and takes one, two, three, six or twelve
/// delays (section 14.3.1); a `PATHPULSE$` name splits into one input and
/// one output (section 14.6.1); an output's pulse style comes before its
/// paths (section 14.6.4); and a generate block holds no `specify` block
/// (A.4.2).
#[test]
fn specify_items_outside_paths_are_refused() {
    for (item, expected) in [
        (
            "$timecheck(posedge c, a, 2);",
            "Unsupported `specify` timing check: `$timecheck`",
        ),
        (
            "$width(c, 2);",
            "`$width` takes an edge-controlled reference event",
        ),
        ("(a => y) = (1, 2, 3, 4);", "4 delays"),
        (
            "specparam PATHPULSE$a = 1;",
            "`PATHPULSE$a`, which names no `PATHPULSE$input$output` path",
        ),
        (
            "(a => y) = 1; showcancelled y;",
            "`showcancelled y` after a module path to it",
        ),
    ] {
        let source = format!(
            "module m(input a, input c, output y);\n  assign y = a;\n  specify {item} \
             endspecify\nendmodule\n"
        );
        let error = parse_error(&source);
        assert!(error.contains(expected), "{item}: {error}");
    }
    let error = parse_error(
        "module m(input a, output y);\n  generate if (1) begin : g\n    specify (a => y) = 1; \
         endspecify\n  end endgenerate\nendmodule\n",
    );
    assert!(error.contains("generate block item"), "{error}");
    for (path, port) in [
        ("(y => a) = 1;", "source `y`"),
        ("(a => a) = 1;", "destination `a`"),
    ] {
        let error = analyze_error(&format!(
            "module m(input a, output y);\n  assign y = a;\n  specify {path} endspecify\n\
             endmodule\n"
        ));
        assert!(
            error.contains(&format!("module path {port}")) && error.contains("14.2.1"),
            "{path}: {error}"
        );
    }
    // Section 14.2.5: a parallel connection joins equal widths; a full one
    // joins any.
    let source = |connection: &str| {
        format!(
            "module m(input a, output [7:0] y);\n  assign y = {{8{{a}}}};\n  specify \
             (a {connection} y) = 2; endspecify\nendmodule\n"
        )
    };
    let error = analyze_error(&source("=>"));
    assert!(
        error.contains("14.2.5") && error.contains("`*>`"),
        "{error}"
    );
    analyze(&source("*>"));
}

/// Section 15: a timing check's condition reads a signal (15.6), its
/// notifier is a `reg` (15.5), and only `$setuphold`, `$recrem` and
/// `$nochange` take a negative limit (15.8). A negative `$nochange` end
/// offset is refused too: its window would close before the trailing edge
/// it ends at is known.
#[test]
fn timing_check_operands_follow_section_15() {
    let module = |items: &str| {
        format!(
            "module m(input a, input c, input [1:0] v);\n  reg n;\n  specify {items} \
             endspecify\nendmodule\n"
        )
    };
    for (items, expected) in [
        (
            "$hold(posedge c &&& w, a, 1);",
            "condition signal `w` is no discrete signal",
        ),
        (
            "$setuphold(posedge c, a, 1, 1, , , w);",
            "condition signal `w` is no discrete signal",
        ),
        (
            "$setup(a, posedge c, 1, a);",
            "notifier `a` is not a variable of module `m`; IEEE 1364-2005 A.7.5.2",
        ),
    ] {
        let error = analyze_error(&module(items));
        assert!(error.contains(expected), "{items}: {error}");
    }
    for (items, expected) in [
        (
            "$setup(a, posedge c, -1);",
            "a negative limit of a timing check that takes none",
        ),
        (
            "$nochange(posedge c, a, 1, -1);",
            "a `$nochange` with a negative end edge offset",
        ),
    ] {
        let error = lowering_error(&module(items));
        assert!(error.contains(expected), "{items}: {error}");
    }
    analyze(&module(
        "$setuphold(posedge c &&& n === 1'b1, a, -1, 2, n); $nochange(posedge c, a, -1, 1);",
    ));
}

/// A.7.5.2: a delayed signal is a net or one bit of it, and copies its
/// signal or the bit of it the check names, so the widths it compares are
/// the bits named. A bit's copy that a negative limit delays is refused by
/// name: the module path that delays it would delay the whole net.
#[test]
fn a_delayed_signal_may_copy_one_bit() {
    let module = |items: &str| {
        format!(
            "module m(input c, input [1:0] v);\n  wire [1:0] w;\n  wire x;\n  specify {items} \
             endspecify\nendmodule\n"
        )
    };
    analyze(&module(
        "$setuphold(posedge c, v[1], 1, 1, , , , x, w[0]); \
         $setuphold(posedge c, v[0], 1, 1, , , , x, w[1]);",
    ));
    let error = analyze_error(&module("$setuphold(posedge c, v[1], 1, 1, , , , , w);"));
    assert!(
        error.contains("delayed signal `w` is not a net of module `m` as wide as `v`"),
        "{error}"
    );
    let error = lowering_error(&module("$setuphold(posedge c, v[1], -1, 2, , , , w[0], );"));
    assert!(
        error.contains(
            "a delayed signal that is one bit of a net, delayed by a negative timing check limit"
        ),
        "{error}"
    );
    let error = parse_error(&module("$setuphold(posedge c, v[1:0], 1, 1, , , , , w);"));
    assert!(
        error.contains("delayed signal `w` of a part-select of `v`"),
        "{error}"
    );
}

/// Section 15.6: a condition is any expression, `~` it, or it compared with
/// a scalar constant, read at its least significant bit. A vector signal is
/// read directly; any other expression, a parameter among them, drives a
/// one-bit net of its own. Each keeps its form's handling of `x` and `z`,
/// and a violation names it as written.
#[test]
fn timing_check_conditions_take_any_expression() {
    let plan = plan(
        "module m(input a, input c, input e, input [1:0] v);\n  parameter P = 1;\n  \
         specify\n    $hold(posedge c &&& v, a, 1);\n    $hold(posedge c &&& (a & e), a, 1);\n    \
         $hold(posedge c &&& ~(a | e), a, 1);\n    $hold(posedge c &&& v[1] != 1'b0, a, 1);\n    \
         $setuphold(posedge c, a, 1, 1, , P, e === 1'b1);\n  endspecify\nendmodule\n",
        "m",
    );
    let name = |id| plan.signal(id).unwrap().name.to_string();
    let conditions: Vec<_> = plan
        .delays
        .checks
        .iter()
        .flat_map(|check| {
            [
                check.reference.condition,
                check.timestamp_condition,
                check.timecheck_condition,
            ]
        })
        .flatten()
        .map(|condition| {
            (
                name(condition.signal),
                condition.enabling,
                condition.unknown_enables,
            )
        })
        .collect();
    assert_eq!(
        conditions,
        [
            ("v".into(), true, false),
            ("specify$tc0".into(), true, false),
            ("specify$tc1".into(), false, false),
            ("specify$tc2".into(), true, true),
            ("specify$tc3".into(), true, false),
            ("e".into(), true, false),
        ]
    );
    let labels: Vec<_> = plan
        .delays
        .checks
        .iter()
        .map(|check| check.label.as_str())
        .collect();
    assert_eq!(
        labels,
        [
            "$hold(posedge c &&& v, a, 1)",
            "$hold(posedge c &&& (a & e), a, 1)",
            "$hold(posedge c &&& ~(a | e), a, 1)",
            "$hold(posedge c &&& v[1] != 1'b0, a, 1)",
            "$setuphold(posedge c, a, 1, 1)",
        ]
    );
}
