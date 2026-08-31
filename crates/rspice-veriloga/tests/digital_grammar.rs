//! The IEEE 1364-2005 digital subset that Verilog-AMS embeds.
//!
//! Every construct here has three pins: it **parses** into a faithful tree,
//! it is **refused at the backend** because nothing can execute it yet, and its
//! **diagnostics name the construct and the fix**. The refusal is the point of
//! the wave: a digital source must be read and diagnosed properly, and must
//! never compile into a device that quietly lacks the author's digital
//! behavior.

use rspice_veriloga::ast::*;
use rspice_veriloga::four_state::{FourStateBit, LiteralBase};
use rspice_veriloga::semantic::AnalyzedFile;
use rspice_veriloga::{
    CompileError, CompilerOptions, Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler,
};

fn parse(source: &str) -> SourceFile {
    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<digital>", source);
    let tokens = Lexer::new(source, source_id)
        .collect_tokens()
        .unwrap_or_else(|error| panic!("lexing failed: {error}"));
    Parser::new(&tokens)
        .parse()
        .unwrap_or_else(|error| panic!("parsing failed: {error}"))
}

fn parse_error(source: &str) -> String {
    let source_map = SourceMap::new();
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
         \x20   electrical clk, q;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= 1'b1;\n\
         endmodule\n",
    );
    let module = only_module(&analyzed);
    let signal = &module.digital.signals[0];
    assert_eq!(signal.name, "q");
    assert!(signal.redeclares_port);
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

    let message = analyze_error(&digital_module("    reg [100000:0] enormous;"));
    assert!(
        message.contains("this compiler supports at most 65536 bits per signal"),
        "expected a width-limit diagnostic, got {message:?}"
    );
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
fn a_sensitivity_term_naming_no_signal_is_refused() {
    let message = analyze_error(&digital_module(
        "    reg q;\n\
         \x20   always @(1 + 2) q = 1'b1;",
    ));
    assert!(
        message.contains("names no signal"),
        "expected a sensitivity-term diagnostic, got {message:?}"
    );
}

// ===========================================================================
// Statements
// ===========================================================================

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

/// Every procedural statement this wave does not implement stops on its own
/// keyword, so the next construct to implement is named rather than guessed at.
#[test]
fn out_of_scope_procedural_statements_are_refused_by_name() {
    for (keyword, statement) in [
        ("fork", "fork q = 1'b1; join"),
        ("wait", "wait (q) q = 1'b1;"),
        ("disable", "disable body;"),
        ("force", "force q = 1'b1;"),
        ("release", "release q;"),
        ("deassign", "deassign q;"),
        ("$display", "$display(\"q\");"),
    ] {
        let message = parse_error(&digital_module(&format!(
            "    wire clk;\n\
             \x20   reg q;\n\
             \x20   always @(posedge clk) {statement}"
        )));
        assert!(
            message.contains(&format!(
                "Verilog-AMS digital construct not yet supported: `{keyword}`"
            )),
            "expected `{keyword}` to be refused by name, got {message:?}"
        );
    }
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

/// A constant select outside a signal's declared bounds is a defect the
/// declaration already proves, so it is refused here rather than at run time.
#[test]
fn constant_selects_are_checked_against_declared_bounds() {
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg [3:0] nibble;\n\
         \x20   always @(posedge clk) nibble[7] <= 1'b1;",
    ));
    assert!(
        message.contains("bit 7 of `nibble`, which is declared [3:0]"),
        "expected an out-of-bounds diagnostic, got {message:?}"
    );

    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg scalar;\n\
         \x20   always @(posedge clk) scalar[1] <= 1'b1;",
    ));
    assert!(
        message.contains("bit 1 of `scalar`, which is declared a scalar (1 bit)"),
        "expected a scalar-select diagnostic, got {message:?}"
    );
}

#[test]
fn part_selects_resolve_and_are_bounds_checked() {
    let analyzed = analyze(&digital_module(
        "    wire clk;\n\
         \x20   reg [7:0] bus;\n\
         \x20   reg [3:0] high;\n\
         \x20   always @(posedge clk) high <= bus[7:4];",
    ));
    assert_eq!(only_module(&analyzed).digital.signals.len(), 3);

    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg [7:0] bus;\n\
         \x20   reg [3:0] high;\n\
         \x20   always @(posedge clk) high <= bus[9:4];",
    ));
    assert!(
        message.contains("bit 9 of `bus`, which is declared [7:0]"),
        "expected a part-select bounds diagnostic, got {message:?}"
    );
}

/// The two halves of the language share one expression grammar, so a
/// continuous-domain form reaches a process's parser. It has no meaning there
/// and stops by name.
#[test]
fn continuous_domain_expressions_are_refused_inside_a_process() {
    let message = analyze_error(&digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= V(p, n);",
    ));
    assert!(
        message.contains("branch access") && message.contains("discrete-domain expression"),
        "expected a branch-access diagnostic, got {message:?}"
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
            && message.contains("`always`/`initial` process"),
        "expected a domain diagnostic naming the construct and where it is legal, got {message:?}"
    );
}

#[test]
fn part_selects_are_refused_in_the_continuous_domain() {
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
        message.contains("part-select") && message.contains("continuous (analog) domain"),
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
        ("    always @(p) gain = 1.0;\n     reg q;", "always"),
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
        message.contains("module `child` contains a"),
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

    /// A delay on a driver is a transport delay, which needs the kernel's
    /// timing wheel rather than a suspension in the process. It refuses by
    /// name.
    #[test]
    fn canonical_ir_construction_refuses_a_delayed_continuous_assignment() {
        let source = digital_module(
            "    wire a;\n\
             \x20   wire y;\n\
             \x20   assign #5 y = a;",
        );
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect_err("a delayed driver must be refused");
        assert!(
            error
                .to_string()
                .contains("delay on a continuous assignment"),
            "{error}"
        );
    }

    /// A `reg` declaration initializer is not a driver: IEEE 1364-2005 section
    /// 6.2.1 makes it an `initial` assignment. It refuses rather than being
    /// dropped the way the net form used to be.
    #[test]
    fn a_variable_declaration_initializer_is_refused() {
        let source = digital_module("    reg q = 1'b0;");
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect_err("a variable declaration initializer must be refused");
        assert!(
            error.to_string().contains("declaration initializer"),
            "{error}"
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
/// an array of instances. Nothing downstream can be told how many instances
/// exist, so it is refused on the bracket rather than misreported as a missing
/// `(`.
#[test]
fn an_instance_array_range_is_refused_by_name() {
    let message = parse_error(&hierarchy(
        NAND2,
        "    wire a, b, y;\n     nand2 g[1:0](y, a, b);",
    ));
    assert!(
        message.contains("instance array range") && message.contains("12.1.2"),
        "{message}"
    );
}

/// `defparam` is refused at its own keyword, as every other unimplemented
/// discrete-domain construct is.
#[test]
fn defparam_is_refused_by_name() {
    let message = parse_error(&hierarchy(
        NAND2,
        "    wire a, b, y;\n     nand2 g1(y, a, b);\n     defparam g1.w = 1;",
    ));
    assert!(message.contains("defparam"), "{message}");
    // The clause, and what to write instead. `defparam` is the one construct in
    // this set an author reaches for on purpose — it is the other spelling of a
    // parameter override — so the refusal has to say which spelling is read.
    assert!(message.contains("12.2.1"), "{message}");
    assert!(message.contains("`#(...)` override"), "{message}");
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

/// A module may not drive its own `input` port, whether or not anybody
/// instantiates it (IEEE 1364-2005 section 12.3.9.1).
///
/// The "whether or not" is the point. An input port is driven from outside the
/// instance, and that holds for a module compiled as the top of the design as
/// much as for one somebody instantiates — so the refusal is on the module's own
/// text rather than on a connection, and the same source cannot be accepted or
/// refused depending on where it sits in a hierarchy.
///
/// `inout` is not included: section 12.3.9.3 makes a bidirectional port
/// drivable from both sides, which is the whole reason the direction exists.
#[test]
fn a_module_may_not_drive_its_own_input_port() {
    let message = analyze_error(
        "module driver(y, a);\n\
         \x20   output y;\n     input a;\n     wire y, a;\n\
         \x20   assign y = 1'b0;\n\
         \x20   assign a = y;\n\
         endmodule\n",
    );
    assert!(message.contains("declares an `input` port"), "{message}");
    assert!(message.contains("12.3.9.1"), "{message}");

    // A gate primitive is a continuous assignment by the time this runs, so the
    // same rule reaches it — a design cannot get past the check by spelling the
    // driver as `buf`.
    let message = analyze_error(
        "module gated(y, a);\n\
         \x20   output y;\n     input a;\n     wire y, a;\n\
         \x20   buf u (a, y);\n\
         endmodule\n",
    );
    assert!(message.contains("declares an `input` port"), "{message}");

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
    let source = format!(
        "module inv(y, a);\n\
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
         endmodule\n"
    );
    let plan = plan(&source, "top");

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
            "a parameter override",
            hierarchy(
                "module gate(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y, a;\n\
                 \x20   assign y = a;\n\
                 endmodule\n",
                "    wire x, z;\n     gate #(2) g1(z, x);",
            ),
            vec!["overrides a parameter", "12.2"],
        ),
        (
            "an expression in a port connection",
            hierarchy(NAND2, "    wire a, b, y;\n     nand2 g1(y, a & b, b);"),
            vec!["connected to an expression", "12.3.9"],
        ),
        (
            "a bit-select connected to an inout port",
            hierarchy(
                "module pass(io, a);\n\
                 \x20   inout io;\n     input a;\n     wire io, a;\n\
                 \x20   assign io = a;\n\
                 endmodule\n",
                "    wire [1:0] bus;\n     wire x;\n     pass g1(bus[0], x);",
            ),
            vec!["bidirectional join", "12.3.9.3"],
        ),
        (
            "an undeclared connection name",
            hierarchy(NAND2, "    wire a, b;\n     nand2 g1(y, a, b);"),
            vec!["not a declared discrete-domain signal", "section 4.5"],
        ),
        (
            "a width mismatch",
            hierarchy(
                "module bus1(y, a);\n\
                 \x20   output y;\n     input [3:0] a;\n\
                 \x20   wire y;\n     wire [3:0] a;\n\
                 \x20   assign y = a[0];\n\
                 endmodule\n",
                "    wire x, z;\n     bus1 g1(z, x);",
            ),
            vec!["4-bit connection", "12.3.9"],
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
            "a module driving its own input port",
            hierarchy(
                "module gate(y, a);\n\
                 \x20   output y;\n     input a;\n     wire y, a;\n\
                 \x20   assign y = 1'b0;\n     assign a = y;\n\
                 endmodule\n",
                "    wire x, z;\n     gate g1(z, x);",
            ),
            vec!["declares an `input` port", "12.3.9.1"],
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
            vec!["Circular dependency", "loop -> loop"],
        ),
        (
            "a continuous-domain module inside a digital one",
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
            vec!["inside a discrete-domain module", "mixed-signal"],
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
        ("buf  u (y, a);", "assign y = a;"),
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

/// A gate delay is a transport delay on the driver, which is what an
/// `assign #2` is, and it is refused with the same words rather than dropped.
#[cfg(feature = "native")]
#[test]
fn a_gate_delay_is_refused_by_name() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            "module g(a, b, y);\n\
             \x20   input a, b;\n     output y;\n\
             \x20   nand #2 u (y, a, b);\n\
             endmodule\n",
            Some("g"),
        )
        .expect_err("a gate delay has no lowered form")
        .to_string();
    assert!(error.contains("delay"), "{error}");
}

/// A drive-strength specification selects between strengths this compiler's
/// one-strength resolution cannot represent. Ignoring it would silently resolve
/// a contention the design meant to decide.
#[test]
fn a_gate_drive_strength_is_refused_by_name() {
    let error = parse_error(
        "module g(a, b, y);\n\
         \x20   input a, b;\n     output y;\n\
         \x20   nand (strong1, weak0) u (y, a, b);\n\
         endmodule\n",
    );
    assert!(error.contains("strength"), "{error}");
}
