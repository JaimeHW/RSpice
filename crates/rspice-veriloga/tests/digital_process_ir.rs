//! Digital processes as canonical-IR functions.
//!
//! The shapes pinned here are the ones a process interpreter will read, so
//! they are asserted as structure — which terminator ends the entry block,
//! which node kind a `<=` produced — rather than as a rendering. A test that
//! matched a debug string would pass while the graph underneath it changed.

#![cfg(feature = "native")]

use rspice_veriloga::canonical_ir::cfg::{CfgTerminator, CfgValueKind, CfgValueType, DigitalWait};
use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalEdge, DigitalProcessKind,
    DigitalSchedulingRegion, DigitalSensitivityOrigin,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::{BlockId, CanonicalIrArtifact};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

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

fn artifact(section: &str) -> CanonicalIrArtifact {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module(section))
        .expect("fixture must lower to canonical IR")
}

fn plan(section: &str) -> CanonicalDigitalPlan {
    artifact(section).digital
}

fn only_process(section: &str) -> CfgDigitalProcess {
    let mut plan = plan(section);
    assert_eq!(plan.processes.len(), 1, "fixture must have one process");
    plan.processes.remove(0)
}

/// The terminator of a process's entry block.
fn entry_terminator(process: &CfgDigitalProcess) -> &CfgTerminator {
    &process.function.block(process.function.entry).terminator
}

/// Every value kind in a process, in definition order.
fn kinds(process: &CfgDigitalProcess) -> Vec<&CfgValueKind> {
    process
        .function
        .values
        .iter()
        .map(|value| &value.kind)
        .collect()
}

fn signal_named(
    plan: &CanonicalDigitalPlan,
    name: &str,
) -> rspice_veriloga::canonical_ir::DigitalSignalId {
    plan.signals
        .iter()
        .find(|signal| signal.name == name)
        .unwrap_or_else(|| panic!("no signal named {name}"))
        .id
}

// ===========================================================================
// Suspension
// ===========================================================================

/// The shape the whole workstream is about: an opening `@(...)` is the entry
/// block's terminator, edge-qualified, naming the signal it waits on.
#[test]
fn an_opening_event_control_becomes_the_entry_block_wait() {
    let section = "    wire clk;\n\
                   \x20   reg q, d;\n\
                   \x20   always @(posedge clk) q <= d;";
    let plan = plan(section);
    let process = &plan.processes[0];

    let CfgTerminator::Wait { wait, resume, .. } = entry_terminator(process) else {
        panic!("entry must suspend, found {:?}", entry_terminator(process));
    };
    let DigitalWait::Event(terms) = wait else {
        panic!("an `@` control is an event wait, found {wait:?}");
    };
    assert_eq!(terms.len(), 1);
    assert_eq!(terms[0].signal, signal_named(&plan, "clk"));
    assert_eq!(terms[0].edge, Some(DigitalEdge::Posedge));
    // The resume block is a real successor, not a dangling id.
    assert_ne!(*resume, process.function.entry);
    assert!(usize::from(*resume) < process.function.blocks.len());
}

/// A level-sensitive term carries no edge, and `negedge` is distinguished from
/// `posedge` rather than collapsed into "an edge".
#[test]
fn sensitivity_terms_keep_their_edge_qualifier() {
    let section = "    wire clk, rst, en;\n\
                   \x20   reg q;\n\
                   \x20   always @(posedge clk or negedge rst or en) q <= 1'b1;";
    let plan = plan(section);
    let CfgTerminator::Wait {
        wait: DigitalWait::Event(terms),
        ..
    } = entry_terminator(&plan.processes[0])
    else {
        panic!("entry must suspend on an event");
    };
    let edges: Vec<Option<DigitalEdge>> = terms.iter().map(|term| term.edge).collect();
    assert_eq!(
        edges,
        vec![Some(DigitalEdge::Posedge), Some(DigitalEdge::Negedge), None]
    );
}

/// `#delay` is a time wait, and its operand carries the CFG's integer type
/// rather than being tunnelled through a real.
#[test]
fn a_delay_control_becomes_a_time_wait_on_an_integer() {
    let section = "    reg q;\n\
                   \x20   initial #5 q = 1'b1;";
    let process = only_process(section);
    let CfgTerminator::Wait {
        wait: DigitalWait::Delay(delay),
        ..
    } = entry_terminator(&process)
    else {
        panic!("a `#` control is a delay wait");
    };
    assert_eq!(
        process.function.value(*delay).value_type,
        CfgValueType::Integer
    );
    assert_eq!(
        process.function.value(*delay).kind,
        CfgValueKind::IntegerConstant(5)
    );
}

/// An intra-assignment delay evaluates the right-hand side *before* the
/// suspension (IEEE 1364-2005 section 9.2.2), so the read is on the near side
/// of the `Wait` and the write on the far side.
#[test]
fn an_intra_assignment_delay_reads_before_it_suspends() {
    let section = "    reg q, d;\n\
                   \x20   initial q <= #5 d;";
    let process = only_process(section);
    let entry = process.function.entry;
    let entry_block = process.function.block(entry);

    let reads_before: Vec<_> = entry_block
        .instructions
        .iter()
        .filter(|instruction| {
            matches!(
                process.function.value(instruction.result).kind,
                CfgValueKind::DigitalSignalRead { .. }
            )
        })
        .collect();
    assert_eq!(reads_before.len(), 1, "the RHS is read before suspending");

    let CfgTerminator::Wait { resume, .. } = entry_block.terminator else {
        panic!("the entry must suspend");
    };
    let writes_after = process
        .function
        .block(resume)
        .instructions
        .iter()
        .filter(|instruction| {
            matches!(
                process.function.value(instruction.result).kind,
                CfgValueKind::DigitalNonblockingWrite { .. }
            )
        })
        .count();
    assert_eq!(writes_after, 1, "the write lands after the suspension");
}

// ===========================================================================
// Process kind
// ===========================================================================

/// IEEE 1364-2005 section 9.9.2: an `always` process restarts when it
/// finishes, and that is an edge in the graph rather than a flag on it.
#[test]
fn an_always_process_loops_back_to_its_entry() {
    let section = "    wire clk;\n\
                   \x20   reg q, d;\n\
                   \x20   always @(posedge clk) q <= d;";
    let process = only_process(section);
    assert_eq!(process.kind, DigitalProcessKind::Always);
    assert!(process.kind.restarts());

    let back_edges = process
        .function
        .blocks
        .iter()
        .filter(|block| {
            matches!(
                block.terminator,
                CfgTerminator::Jump { target, .. } if target == process.function.entry
            )
        })
        .count();
    assert_eq!(back_edges, 1, "the restart is one edge back to the entry");
    assert!(
        !process
            .function
            .blocks
            .iter()
            .any(|block| matches!(block.terminator, CfgTerminator::Return)),
        "an always process never returns"
    );
}

/// Section 9.9.1: an `initial` process runs once, so it returns and has no
/// edge back to its entry.
#[test]
fn an_initial_process_runs_once() {
    let section = "    reg q;\n\
                   \x20   initial q = 1'b0;";
    let process = only_process(section);
    assert_eq!(process.kind, DigitalProcessKind::Initial);
    assert!(!process.kind.restarts());

    assert!(
        process
            .function
            .blocks
            .iter()
            .any(|block| matches!(block.terminator, CfgTerminator::Return)),
        "an initial process returns"
    );
    assert!(
        !process.function.blocks.iter().any(|block| matches!(
            block.terminator,
            CfgTerminator::Jump { target, .. } if target == process.function.entry
        )),
        "an initial process does not restart"
    );
}

// ===========================================================================
// Assignments
// ===========================================================================

/// A nonblocking assignment is a different node from a blocking one, and
/// carries the region its update is deferred to.
#[test]
fn a_nonblocking_write_is_a_distinct_node_from_a_blocking_one() {
    let blocking = only_process(
        "    reg q, d;\n\
         \x20   initial q = d;",
    );
    let nonblocking = only_process(
        "    reg q, d;\n\
         \x20   initial q <= d;",
    );

    assert!(
        kinds(&blocking)
            .iter()
            .any(|kind| matches!(kind, CfgValueKind::DigitalBlockingWrite { .. })),
        "`=` is a blocking write"
    );
    assert!(
        !kinds(&blocking)
            .iter()
            .any(|kind| matches!(kind, CfgValueKind::DigitalNonblockingWrite { .. })),
        "`=` is not a buffered write"
    );

    let deferred: Vec<_> = kinds(&nonblocking)
        .into_iter()
        .filter_map(|kind| match kind {
            CfgValueKind::DigitalNonblockingWrite { region, .. } => Some(*region),
            _ => None,
        })
        .collect();
    assert_eq!(deferred, vec![DigitalSchedulingRegion::NonBlockingAssign]);
    assert!(
        !kinds(&nonblocking)
            .iter()
            .any(|kind| matches!(kind, CfgValueKind::DigitalBlockingWrite { .. })),
        "`<=` is not an immediate write"
    );
}

/// A write node carries ordering, not data: nothing may read what it produced,
/// and the type says so.
#[test]
fn a_write_produces_an_effect_rather_than_a_value() {
    let process = only_process(
        "    reg q, d;\n\
         \x20   initial q <= d;",
    );
    let write = process
        .function
        .values
        .iter()
        .find(|value| matches!(value.kind, CfgValueKind::DigitalNonblockingWrite { .. }))
        .expect("a nonblocking write");
    assert_eq!(write.value_type, CfgValueType::Effect);
    assert!(write.value_type.is_digital());

    // And nothing reads it.
    assert!(
        !process
            .function
            .values
            .iter()
            .any(|value| value.kind.operands().contains(&write.id)),
        "an effect is never an operand"
    );
}

/// A concatenation target becomes one write per element over slices of the
/// right-hand side, most significant element first.
#[test]
fn a_concatenation_target_becomes_one_write_per_element() {
    let plan = plan(
        "    reg carry, sum;\n\
         \x20   reg [1:0] result;\n\
         \x20   initial {carry, sum} = result;",
    );
    let process = &plan.processes[0];
    let writes: Vec<_> = process
        .function
        .values
        .iter()
        .filter_map(|value| match &value.kind {
            CfgValueKind::DigitalBlockingWrite { target, .. } => Some(target.signal),
            _ => None,
        })
        .collect();
    assert_eq!(
        writes,
        vec![signal_named(&plan, "carry"), signal_named(&plan, "sum")]
    );

    // Each write reads a one-bit slice, and `carry` takes the high bit.
    let slices: Vec<(i64, i64)> = process
        .function
        .values
        .iter()
        .filter_map(|value| match value.kind {
            CfgValueKind::DigitalPartSelect { msb, lsb, .. } => Some((msb, lsb)),
            _ => None,
        })
        .collect();
    assert_eq!(slices, vec![(1, 1), (0, 0)]);
}

// ===========================================================================
// Implicit sensitivity
// ===========================================================================

/// IEEE 1364-2005 section 9.7.5: `@*` is the read set of the statement it
/// guards, computed at lowering because the front end deliberately does not
/// store one.
#[test]
fn an_implicit_sensitivity_list_is_derived_from_the_read_set() {
    let plan = plan(
        "    wire a, b, c;\n\
         \x20   reg y;\n\
         \x20   always @* y = (a & b) | c;",
    );
    let process = &plan.processes[0];
    let CfgTerminator::Wait {
        wait: DigitalWait::Event(terms),
        ..
    } = entry_terminator(process)
    else {
        panic!("`@*` still suspends");
    };

    let mut signals: Vec<_> = terms.iter().map(|term| term.signal).collect();
    signals.sort();
    let mut expected = vec![
        signal_named(&plan, "a"),
        signal_named(&plan, "b"),
        signal_named(&plan, "c"),
    ];
    expected.sort();
    assert_eq!(signals, expected, "every signal on the RHS is in the list");
    assert!(
        terms.iter().all(|term| term.edge.is_none()),
        "an implicit list is level-sensitive"
    );

    let sensitivity = process
        .static_sensitivity
        .as_ref()
        .expect("an opening `@*` is a static list");
    assert_eq!(sensitivity.origin, DigitalSensitivityOrigin::Implicit);
}

/// The asymmetry that makes the rule work: a name that is only written does
/// not appear in the list it would otherwise retrigger.
#[test]
fn an_implicit_list_excludes_the_name_it_writes() {
    let plan = plan(
        "    wire a;\n\
         \x20   reg y;\n\
         \x20   always @* y = ~a;",
    );
    let CfgTerminator::Wait {
        wait: DigitalWait::Event(terms),
        ..
    } = entry_terminator(&plan.processes[0])
    else {
        panic!("`@*` suspends");
    };
    let signals: Vec<_> = terms.iter().map(|term| term.signal).collect();
    assert_eq!(signals, vec![signal_named(&plan, "a")]);
    assert!(
        !signals.contains(&signal_named(&plan, "y")),
        "the assignment target must not retrigger its own process"
    );
}

// ===========================================================================
// Identity and metadata
// ===========================================================================

/// A process keeps the declaration-ordered identity the parser gave it, and
/// the static list on the process is the one the entry `Wait` holds — read off
/// it rather than derived a second time.
#[test]
fn process_identity_and_sensitivity_survive_lowering() {
    let plan = plan(
        "    wire clk, rst;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= 1'b1;\n\
         \x20   always @(negedge rst) q <= 1'b0;\n\
         \x20   initial q = 1'b0;",
    );
    assert_eq!(plan.processes.len(), 3);

    let ids: Vec<u32> = plan
        .processes
        .iter()
        .map(|process| process.id.index())
        .collect();
    assert_eq!(ids, vec![0, 1, 2], "ids are declaration-ordered");
    for (position, process) in plan.processes.iter().enumerate() {
        assert_eq!(
            plan.process(process.id).map(|found| found.id),
            Some(process.id),
            "process {position} is findable by its own id"
        );
    }

    for process in &plan.processes[..2] {
        let sensitivity = process
            .static_sensitivity
            .as_ref()
            .expect("both `always` processes open with an event control");
        assert_eq!(sensitivity.origin, DigitalSensitivityOrigin::Explicit);
        let CfgTerminator::Wait {
            wait: DigitalWait::Event(terms),
            ..
        } = entry_terminator(process)
        else {
            panic!("the entry suspends");
        };
        assert_eq!(
            &sensitivity.terms, terms,
            "the metadata is the terminator's list, not a second copy"
        );
    }

    assert!(
        plan.processes[2].static_sensitivity.is_none(),
        "a process that does not open with an event control has no static list"
    );
}

/// A declared signal's shape survives, including a reversed packed range,
/// which IEEE 1364-2005 section 4.2.1 makes a different declaration.
#[test]
fn declared_signal_shapes_survive_lowering() {
    let plan = plan(
        "    reg [7:0] bus;\n\
         \x20   reg [0:3] reversed;\n\
         \x20   wire scalar;\n\
         \x20   initial bus = 8'h00;",
    );
    let bus = &plan.signals[usize::from(signal_named(&plan, "bus"))];
    assert_eq!(bus.width, 8);
    assert_eq!(bus.bounds, Some((7, 0)));
    assert!(bus.procedurally_assignable, "a reg may be written");

    let reversed = &plan.signals[usize::from(signal_named(&plan, "reversed"))];
    assert_eq!(reversed.width, 4);
    assert_eq!(reversed.bounds, Some((0, 3)), "the direction is as written");

    let scalar = &plan.signals[usize::from(signal_named(&plan, "scalar"))];
    assert_eq!(scalar.width, 1);
    assert_eq!(scalar.bounds, None);
    assert!(
        !scalar.procedurally_assignable,
        "a wire is not procedurally assignable"
    );
}

// ===========================================================================
// Values
// ===========================================================================

/// A four-state literal reaches the CFG already decoded to its planes, with
/// the width it was written at.
#[test]
fn a_four_state_literal_lowers_to_its_exact_planes() {
    let process = only_process(
        "    reg [3:0] q;\n\
         \x20   initial q = 4'b10xz;",
    );
    let constant = process
        .function
        .values
        .iter()
        .find_map(|value| match &value.kind {
            CfgValueKind::FourStateConstant(constant) => Some((value.value_type, constant)),
            _ => None,
        })
        .expect("the literal is a constant");

    assert_eq!(constant.0, CfgValueType::FourState { width: 4 });
    assert_eq!(constant.1.spelling(), "10xz");
    assert_eq!(constant.1.aval(), [0b1010]);
    assert_eq!(constant.1.bval(), [0b0011]);
    assert_eq!(
        constant.1,
        &FourStateValue::from_bits_msb_first(&[
            rspice_veriloga::four_state::FourStateBit::One,
            rspice_veriloga::four_state::FourStateBit::Zero,
            rspice_veriloga::four_state::FourStateBit::Unknown,
            rspice_veriloga::four_state::FourStateBit::HighImpedance,
        ])
    );
}

/// An expression's result width follows IEEE 1364-2005 section 5.4.1 rather
/// than defaulting to the operands' — a comparison is one bit however wide its
/// operands are, and a bitwise operator is as wide as the wider side.
#[test]
fn expression_result_widths_follow_the_standard() {
    let process = only_process(
        "    reg [7:0] wide;\n\
         \x20   reg [3:0] narrow;\n\
         \x20   reg y;\n\
         \x20   initial y = (wide & {narrow, narrow}) == 8'h00;",
    );
    let widths: Vec<(bool, Option<u32>)> = process
        .function
        .values
        .iter()
        .filter_map(|value| match value.kind {
            CfgValueKind::DigitalBitwise { .. } => Some((true, value.value_type.width())),
            CfgValueKind::DigitalEquality { .. } => Some((false, value.value_type.width())),
            _ => None,
        })
        .collect();
    assert_eq!(
        widths,
        vec![(true, Some(8)), (false, Some(1))],
        "`&` is eight bits wide; `==` is one"
    );
}

/// A part select keeps the bounds as written, so it agrees with the direction
/// its signal was declared in.
#[test]
fn a_part_select_keeps_the_bounds_as_written() {
    let process = only_process(
        "    reg [7:0] bus;\n\
         \x20   reg [3:0] nibble;\n\
         \x20   initial nibble = bus[7:4];",
    );
    let select = process
        .function
        .values
        .iter()
        .find_map(|value| match value.kind {
            CfgValueKind::DigitalPartSelect { msb, lsb, .. } => Some((msb, lsb, value.value_type)),
            _ => None,
        })
        .expect("a part select");
    assert_eq!(select.0, 7);
    assert_eq!(select.1, 4);
    assert_eq!(select.2, CfgValueType::FourState { width: 4 });
}

/// A replication is expanded at lowering, because IEEE 1364-2005 section
/// 4.1.14 requires a constant count and an IR node for it would carry no
/// information the expansion does not.
#[test]
fn a_replication_expands_into_the_concatenation() {
    let process = only_process(
        "    reg [3:0] q;\n\
         \x20   reg bit_value;\n\
         \x20   initial q = {4{bit_value}};",
    );
    let concat = process
        .function
        .values
        .iter()
        .find_map(|value| match &value.kind {
            CfgValueKind::DigitalConcat { parts } => Some((parts.len(), value.value_type)),
            _ => None,
        })
        .expect("a concatenation");
    assert_eq!(concat.0, 4, "the count is expanded, not stored");
    assert_eq!(concat.1, CfgValueType::FourState { width: 4 });
}

// ===========================================================================
// Refusals
// ===========================================================================

/// The generated-Rust device path refuses a module with processes, naming what
/// is missing. The refusal moved here from artifact construction; what it says
/// changed with it, from "this construct is unsupported" to "this construct is
/// lowered but cannot yet run".
#[test]
fn the_device_path_refuses_a_lowered_process() {
    let source = digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= 1'b1;",
    );
    let artifact = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&source)
        .expect("the artifact itself is built; the refusal is downstream of it");
    assert_eq!(artifact.digital.processes.len(), 1);

    let error = rspice_veriloga::rust_backend::canonical::generate_device(
        &artifact,
        &rspice_veriloga::rust_backend::RustTranspileOptions::default(),
    )
    .expect_err("a module with a process must not generate a device");
    let rendered = error.to_string();
    assert!(
        rendered.contains("digital process execution"),
        "the refusal must name process execution: {rendered}"
    );
    assert!(
        rendered.contains("always") && rendered.contains("process 0"),
        "the refusal must name the process it refused: {rendered}"
    );
}

/// The bytecode path refuses earlier and for a different reason: it has no
/// representation for a process at all, so it never sees a lowered one.
///
/// Worth pinning separately, because the two refusals are easy to confuse and
/// a test that accepted either would not notice the emitter's going missing.
#[test]
fn the_bytecode_path_still_refuses_before_lowering() {
    let source = digital_module(
        "    wire clk;\n\
         \x20   reg q;\n\
         \x20   always @(posedge clk) q <= 1'b1;",
    );
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(&source)
        .expect_err("the bytecode builder has no representation for a process");
    assert_eq!(
        error.diagnostic_code(),
        "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
    );
}

/// The derivative pass refuses a discrete-domain value by name rather than
/// reporting it non-differentiable and leaving a silent zero.
#[test]
fn the_derivative_pass_refuses_a_digital_value() {
    use rspice_veriloga::canonical_ir::cfg::{CfgValidationError, SsaBuilder};
    use rspice_veriloga::canonical_ir::differentiate;

    let mut builder = SsaBuilder::new();
    let entry = builder.create_block();
    builder.seal_block(entry);
    let value = builder.push_leaf(
        CfgValueType::FourState { width: 4 },
        CfgValueKind::FourStateConstant(FourStateValue::zero(4)),
    );
    builder.push(
        entry,
        CfgValueType::FourState { width: 4 },
        CfgValueKind::DigitalBitwiseNot { input: value },
    );
    builder.set_terminator(entry, CfgTerminator::Return);
    let function = builder.finish(entry).expect("a valid graph");

    let error = differentiate(&function, &[]).expect_err("a digital value is not differentiable");
    assert!(
        matches!(error, CfgValidationError::DigitalValueInDerivative(_)),
        "{error:?}"
    );
    assert!(
        error.to_string().contains("cannot be differentiated"),
        "{error}"
    );
}

/// Wave-1 boundaries refuse by name, so a model that crosses one is told what
/// is missing rather than compiled into something short of what it says.
#[test]
fn unlowered_constructs_refuse_by_name() {
    let cases = [
        (
            "    reg [3:0] q;\n\
             \x20   integer i;\n\
             \x20   initial for (i = 0; i < 4; i = i + 1) q[i] = 1'b0;",
            "`for` statement",
        ),
        (
            "    reg q;\n\
             \x20   initial repeat (4) q = ~q;",
            "`repeat` statement",
        ),
        (
            "    reg [1:0] sel;\n\
             \x20   reg q;\n\
             \x20   always @* casez (sel) 2'b1?: q = 1'b1; default: q = 1'b0; endcase",
            "`casez`",
        ),
    ];
    for (section, expected) in cases {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&digital_module(section))
            .expect_err("an unlowered construct must be refused");
        let rendered = error.to_string();
        assert!(
            rendered.contains(expected),
            "expected {expected} to be named in: {rendered}"
        );
        assert!(
            rendered.contains("has no lowered form yet"),
            "the refusal must say what is missing: {rendered}"
        );
    }
}

/// A continuous-domain module is untouched: the plan is empty and the artifact
/// carries no digital content at all.
#[test]
fn a_continuous_domain_module_has_an_empty_plan() {
    let artifact = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module("    // no digital section"))
        .expect("a continuous-domain module still compiles");
    assert!(artifact.digital.is_empty());
    assert!(artifact.digital.processes.is_empty());
    assert!(artifact.digital.signals.is_empty());
}

/// A process function is a well-formed graph by the CFG's own rules: every
/// block terminated, every value defined once, every edge's arguments matching
/// its target's parameters.
#[test]
fn a_lowered_process_is_a_valid_graph() {
    for section in [
        "    wire clk;\n\
         \x20   reg q, d;\n\
         \x20   always @(posedge clk) q <= d;",
        "    wire a, b;\n\
         \x20   reg y;\n\
         \x20   always @* if (a) y = b; else y = 1'b0;",
        "    reg clk;\n\
         \x20   initial begin clk = 1'b0; forever #5 clk = ~clk; end",
        "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   always @* case (sel) 2'b00: q = 1'b1; 2'b01: q = 1'b0; default: q = 1'bx; endcase",
    ] {
        let process = only_process(section);
        process
            .function
            .validate()
            .unwrap_or_else(|error| panic!("{section}\nproduced an invalid graph: {error}"));
        assert!(
            !matches!(entry_terminator(&process), CfgTerminator::Unset),
            "no block may survive unterminated"
        );
        // Every value a block references is defined by some block or is a leaf.
        for block in &process.function.blocks {
            assert!(
                !matches!(block.terminator, CfgTerminator::Unset),
                "block {:?} is unterminated in: {section}",
                block.id
            );
        }
    }
}

/// A `Wait`'s resume edge is visible to the CFG's own successor walk, so no
/// pass can mistake the resumed half of a process for dead code.
#[test]
fn a_wait_reports_its_resume_edge_as_a_successor() {
    let process = only_process(
        "    wire clk;\n\
         \x20   reg q, d;\n\
         \x20   always @(posedge clk) q <= d;",
    );
    let entry = process.function.block(process.function.entry);
    let CfgTerminator::Wait { resume, .. } = entry.terminator else {
        panic!("the entry suspends");
    };
    assert_eq!(entry.successors(), vec![resume]);

    // And every block is reachable from the entry, so nothing was stranded.
    let mut seen = vec![false; process.function.blocks.len()];
    let mut stack = vec![process.function.entry];
    while let Some(block) = stack.pop() {
        if std::mem::replace(&mut seen[usize::from(block)], true) {
            continue;
        }
        stack.extend(process.function.block(block).successors());
    }
    let unreachable: Vec<BlockId> = process
        .function
        .blocks
        .iter()
        .filter(|block| !seen[usize::from(block.id)])
        .map(|block| block.id)
        .collect();
    assert!(unreachable.is_empty(), "stranded blocks: {unreachable:?}");
}
