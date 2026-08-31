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

    /// A continuous assignment still has no lowered form, and says so by name
    /// rather than being dropped from the artifact.
    #[test]
    fn canonical_ir_construction_refuses_a_continuous_assignment() {
        let source = digital_module(
            "    wire a, b;\n\
             \x20   wire y;\n\
             \x20   assign y = a & b;",
        );
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&source)
            .expect_err("a continuous assignment must be refused");
        let rendered = error.to_string();
        assert!(
            rendered.contains("continuous assignment to `y`"),
            "{rendered}"
        );
        assert!(rendered.contains("has no lowered form yet"), "{rendered}");
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
