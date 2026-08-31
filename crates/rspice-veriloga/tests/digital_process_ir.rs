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

// ===========================================================================
// Table 5-22, operator by operator
// ===========================================================================
//
// Section 5.4.1's table splits every operator into two columns: what its
// result is as wide as, and which of its operands the surrounding context
// reaches. The tests below take one row at a time, because the two halves fail
// independently — an operator can take the context and not pass it on, or pass
// it on and report the wrong result width — and a single fixture exercising
// the whole table would not say which.
//
// The fixtures put a *wider target* than any operand on the left of every
// assignment, because that is the only arrangement in which the two readings
// of the rule differ. With the target no wider than the operands, computing at
// the operand width and computing at the context width are the same thing.

/// Every node width in a process that matches `wanted`, in definition order.
fn widths_where(
    process: &CfgDigitalProcess,
    wanted: impl Fn(&CfgValueKind) -> bool,
) -> Vec<Option<u32>> {
    process
        .function
        .values
        .iter()
        .filter(|value| wanted(&value.kind))
        .map(|value| value.value_type.width())
        .collect()
}

fn width_of(
    process: &CfgDigitalProcess,
    value: rspice_veriloga::canonical_ir::ids::ValueId,
) -> u32 {
    process
        .function
        .value(value)
        .value_type
        .width()
        .expect("a four-state value has a width")
}

/// Both operands of `+ - * / %` and of `& | ^` are context-determined, and the
/// result takes the context size. Nine operators, one fixture each, all with
/// four-bit operands under an eight-bit target.
///
/// This is the row that was wrong: the result width was the operands' maximum,
/// so the operation ran narrow and the bits the target had room for were never
/// computed rather than merely discarded.
#[test]
fn arithmetic_and_bitwise_operators_take_the_context_width() {
    for spelling in ["+", "-", "*", "/", "%", "&", "|", "^", "~^"] {
        let process = only_process(&format!(
            "    reg [3:0] a, b;\n\
             \x20   reg [7:0] p;\n\
             \x20   initial p = a {spelling} b;",
        ));
        let widths = widths_where(&process, |kind| {
            matches!(
                kind,
                CfgValueKind::DigitalArithmetic { .. } | CfgValueKind::DigitalBitwise { .. }
            )
        });
        assert_eq!(widths, vec![Some(8)], "`{spelling}` at the context width");

        // And both operands reached that width before the operator ran, which
        // is what section 5.4.1 actually requires — a node labelled eight bits
        // whose operands are four is still a four-bit operation.
        let operands = process
            .function
            .values
            .iter()
            .find_map(|value| match value.kind {
                CfgValueKind::DigitalArithmetic { left, right, .. }
                | CfgValueKind::DigitalBitwise { left, right, .. } => Some((left, right)),
                _ => None,
            })
            .expect("the operator is in the graph");
        assert_eq!(width_of(&process, operands.0), 8, "`{spelling}` left");
        assert_eq!(width_of(&process, operands.1), 8, "`{spelling}` right");
    }
}

/// Unary `~`, `+` and `-` are context-determined with a result of L(i); `!` is
/// self-determined and one bit. The four are spelled the same way and split
/// down the middle, which is why they are checked together.
#[test]
fn unary_operators_split_between_the_two_columns() {
    for spelling in ["~", "-"] {
        let process = only_process(&format!(
            "    reg [3:0] a;\n\
             \x20   reg [7:0] p;\n\
             \x20   initial p = {spelling}a;",
        ));
        let widths = widths_where(&process, |kind| {
            matches!(
                kind,
                CfgValueKind::DigitalBitwiseNot { .. } | CfgValueKind::DigitalArithmetic { .. }
            )
        });
        assert_eq!(widths, vec![Some(8)], "unary `{spelling}`");
    }

    // Unary `+` is the identity, so it leaves no node of its own; what it must
    // not do is stop the context, and the addition under it proves it did not.
    let process = only_process(
        "    reg [3:0] a, b;\n\
         \x20   reg [7:0] p;\n\
         \x20   initial p = +(a + b);",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalArithmetic { .. }
        )),
        vec![Some(8)],
        "unary `+` passes the context through"
    );

    let process = only_process(
        "    reg [3:0] a;\n\
         \x20   reg [7:0] p;\n\
         \x20   initial p = !a;",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalLogicalNot { .. }
        )),
        vec![Some(1)],
        "section 4.1.8: `!` is one bit whatever it is assigned to"
    );
}

/// Section 4.1.6, 4.1.7 and 4.1.8: a comparison and a logical operator are one
/// bit, and the context does not reach their operands. A sixteen-bit target
/// changes neither the result width nor the operand widths.
#[test]
fn comparisons_and_logical_operators_are_one_bit_and_stop_the_context() {
    for spelling in ["==", "!=", "<", "<=", ">", ">=", "&&", "||", "===", "!=="] {
        let process = only_process(&format!(
            "    reg [3:0] a, b;\n\
             \x20   reg [15:0] p;\n\
             \x20   initial p = a {spelling} b;",
        ));
        let widths = widths_where(&process, |kind| {
            matches!(
                kind,
                CfgValueKind::DigitalEquality { .. }
                    | CfgValueKind::DigitalRelational { .. }
                    | CfgValueKind::DigitalLogical { .. }
                    | CfgValueKind::DigitalCaseMatch { .. }
            )
        });
        assert_eq!(widths, vec![Some(1)], "`{spelling}` is one bit");

        // No operand was widened to sixteen: the only four-state nodes in the
        // graph besides the comparison are the two operand reads.
        let reads = widths_where(&process, |kind| {
            matches!(kind, CfgValueKind::DigitalSignalRead { .. })
        });
        assert_eq!(reads, vec![Some(4), Some(4)], "`{spelling}` operands");
        assert!(
            widths_where(&process, |kind| matches!(
                kind,
                CfgValueKind::DigitalConcat { .. }
            ))
            .is_empty(),
            "`{spelling}` widened an operand it should not have"
        );
    }
}

/// Table 5-22 gives a shift the size of its *left* operand and makes only that
/// operand context-determined. So the shifted value reaches the context width
/// and the count does not.
#[test]
fn a_shift_widens_its_value_and_leaves_its_count_alone() {
    let process = only_process(
        "    reg [3:0] a, n;\n\
         \x20   reg [7:0] p;\n\
         \x20   initial p = a << n;",
    );
    let shift = process
        .function
        .values
        .iter()
        .find(|value| matches!(value.kind, CfgValueKind::DigitalShift { .. }))
        .expect("a shift node");
    let CfgValueKind::DigitalShift { value, count, .. } = shift.kind else {
        unreachable!("just matched")
    };
    assert_eq!(shift.value_type.width(), Some(8), "the result");
    assert_eq!(width_of(&process, value), 8, "the value being shifted");
    assert_eq!(
        width_of(&process, count),
        4,
        "the count is self-determined and stays as declared"
    );
}

/// Section 5.4.1 makes every operand of a concatenation self-determined, so an
/// operator inside one is sized by its own operands and by nothing outside.
///
/// The concatenation's own size is the sum, and it is that sum whatever the
/// target is — the target's width reaches the concatenation only through
/// section 5.2.1's resize at the write, which is a different step.
#[test]
fn concatenation_operands_are_self_determined() {
    let process = only_process(
        "    reg [3:0] a, b, c;\n\
         \x20   reg [15:0] p;\n\
         \x20   initial p = {a, b + c};",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalArithmetic { .. }
        )),
        vec![Some(4)],
        "`b + c` is as wide as `b` and `c`"
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalConcat { .. }
        )),
        vec![Some(8)],
        "the concatenation is the sum of its parts"
    );
}

/// A replication's count is self-determined and constant (section 4.1.14), and
/// what it repeats is a concatenation operand like any other.
#[test]
fn a_replication_repeats_a_self_determined_operand() {
    let process = only_process(
        "    reg [3:0] a, b;\n\
         \x20   reg [15:0] p;\n\
         \x20   initial p = {2{a + b}};",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalArithmetic { .. }
        )),
        vec![Some(4), Some(4)],
        "each copy is sized by its own operands"
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalConcat { .. }
        )),
        vec![Some(8)],
        "two four-bit copies"
    );
}

/// Section 4.1.10: a reduction is one bit and its operand is self-determined,
/// so the fold runs across the operand's own bits and the context reaches
/// neither the operand nor the result.
#[test]
fn a_reduction_folds_its_own_operand_width() {
    let process = only_process(
        "    reg [3:0] a;\n\
         \x20   reg [15:0] p;\n\
         \x20   initial p = ^a;",
    );
    // Four bit selects and three one-bit XOR steps: the fold, at the operand's
    // width rather than the target's.
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalPartSelect { .. }
        )),
        vec![Some(1); 4],
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalBitwise { .. }
        )),
        vec![Some(1); 3],
    );
}

/// Both arms of `?:` are context-determined and the condition is not, so the
/// select is as wide as the target and the condition stays one bit.
#[test]
fn a_conditional_sizes_its_arms_and_not_its_condition() {
    let process = only_process(
        "    reg [3:0] a, b;\n\
         \x20   reg [7:0] p;\n\
         \x20   reg s;\n\
         \x20   initial p = s ? a + b : a & b;",
    );
    let select = process
        .function
        .values
        .iter()
        .find(|value| matches!(value.kind, CfgValueKind::DigitalSelect { .. }))
        .expect("a select node");
    let CfgValueKind::DigitalSelect {
        condition,
        then_value,
        else_value,
    } = select.kind
    else {
        unreachable!("just matched")
    };
    assert_eq!(select.value_type.width(), Some(8));
    assert_eq!(width_of(&process, then_value), 8);
    assert_eq!(width_of(&process, else_value), 8);
    assert_eq!(width_of(&process, condition), 1, "the condition is a truth");
}

/// Section 5.4.1's unsized-literal rule, both halves.
///
/// The floor is thirty-two, and the context takes over above it. Neither half
/// alone is the rule: freezing at thirty-two loses the second, and taking the
/// context unconditionally loses the first — and with it the width of every
/// concatenation holding an unsized literal.
#[test]
fn an_unsized_literal_is_thirty_two_bits_or_the_context() {
    let wide = only_process(
        "    reg [39:0] q, p;\n\
         \x20   initial p = q | 1;",
    );
    assert_eq!(
        widths_where(&wide, |kind| matches!(
            kind,
            CfgValueKind::FourStateConstant(_)
        )),
        vec![Some(40)],
        "a forty-bit context makes the literal forty bits"
    );

    let inside = only_process(
        "    reg [3:0] a;\n\
         \x20   reg [35:0] p;\n\
         \x20   initial p = {a, 1};",
    );
    assert_eq!(
        widths_where(&inside, |kind| matches!(
            kind,
            CfgValueKind::FourStateConstant(_)
        )),
        vec![Some(32)],
        "a concatenation operand is self-determined, so the floor applies"
    );

    // A *sized* literal is not an unsized one and never grows: section 3.5.1
    // makes it exactly as wide as its author wrote it, and section 5.4.1
    // extends it as an ordinary operand — which is a separate node, so the
    // constant itself is still four bits.
    let sized = only_process(
        "    reg [7:0] p;\n\
         \x20   reg [3:0] a;\n\
         \x20   initial p = a | 4'hF;",
    );
    let constants = widths_where(&sized, |kind| {
        matches!(kind, CfgValueKind::FourStateConstant(_))
    });
    assert!(
        constants.contains(&Some(4)),
        "the sized literal keeps its four bits: {constants:?}"
    );
}

/// A narrower target does not shrink an expression. Section 5.4.1 sizes it to
/// the *largest* of the operands and the target, and section 5.2.1 then
/// truncates at the write — two steps, and collapsing them would compute a
/// four-bit addition at two bits.
#[test]
fn a_narrow_target_does_not_narrow_the_operation() {
    let process = only_process(
        "    reg [3:0] a, b;\n\
         \x20   reg [1:0] p;\n\
         \x20   initial p = a + b;",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalArithmetic { .. }
        )),
        vec![Some(4)],
    );
}

/// The context of an assignment to a concatenation target is the whole target,
/// which is what makes `{cout, sum} = a + b` the carry-out idiom rather than a
/// two-bit addition with a `cout` that is always zero.
#[test]
fn a_concatenation_target_seeds_the_context_with_its_total_width() {
    let process = only_process(
        "    reg [1:0] a, b, sum;\n\
         \x20   reg cout;\n\
         \x20   initial {cout, sum} = a + b;",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalArithmetic { .. }
        )),
        vec![Some(3)],
        "two bits of `sum` and one of `cout`"
    );
}

/// A bit-select target is one bit of context, and one bit is still a context:
/// the expression is sized to the larger of it and the operands, so a four-bit
/// addition stays four bits and its low bit is what lands.
#[test]
fn a_bit_select_target_is_a_one_bit_context() {
    let process = only_process(
        "    reg [3:0] a, b, q;\n\
         \x20   initial q[0] = a + b;",
    );
    assert_eq!(
        widths_where(&process, |kind| matches!(
            kind,
            CfgValueKind::DigitalArithmetic { .. }
        )),
        vec![Some(4)],
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

/// The constructs that reached this wave's lowering and now have one.
///
/// `for`, `repeat`, and the wildcard `case` forms were refused because each
/// needed something the lowering could not build — a loop counter, a
/// process-local, a match operator that is not `==`. They are here as the
/// other half of the refusal test below: what retreated has to be pinned as
/// pointedly as what remains, or the boundary moves without anyone noticing.
#[test]
fn the_deferred_constructs_now_lower() {
    for section in [
        "    reg [3:0] q;\n\
         \x20   initial begin : work integer i;\n\
         \x20       for (i = 0; i < 4; i = i + 1) q = q + 4'b0001; end",
        "    reg q;\n\
         \x20   initial repeat (4) q = ~q;",
        "    reg q;\n\
         \x20   initial begin : work integer i; i = 0;\n\
         \x20       while (i < 4) begin q = ~q; i = i + 1; end end",
        "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   always @* casez (sel) 2'b1?: q = 1'b1; default: q = 1'b0; endcase",
        "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   always @* casex (sel) 2'b1?: q = 1'b1; default: q = 1'b0; endcase",
    ] {
        let process = only_process(section);
        process
            .function
            .validate()
            .unwrap_or_else(|error| panic!("{section}\nproduced an invalid graph: {error}"));
    }
}

/// The boundaries that remain refuse by name, so a model that crosses one is
/// told what is missing rather than compiled into something short of what it
/// says.
#[test]
fn unlowered_constructs_refuse_by_name() {
    let cases = [
        (
            "    reg [3:0] q;\n\
             \x20   integer i;\n\
             \x20   initial begin : work i = 0; q = 4'b0000; end",
            "module-level",
        ),
        (
            "    reg q;\n\
             \x20   initial begin : work real r; r = 1.0; q = 1'b0; end",
            "process-local `real`",
        ),
        (
            "    reg [3:0] q;\n\
             \x20   initial begin : work reg [3:0] t; t = 4'b0000; t[1] = 1'b1; q = t; end",
            "select on the process-local `t`",
        ),
        (
            "    reg q;\n\
             \x20   initial begin : work integer i; i <= 0; q = 1'b0; end",
            "nonblocking assignment to the process-local `i`",
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
            rendered.contains("has no lowered form yet") || rendered.contains("cannot be assigned"),
            "the refusal must say what is missing: {rendered}"
        );
    }
}

/// A `Wait` that has live state to carry passes it as resume arguments bound
/// to the resume block's parameters.
///
/// The arity is what a kernel relies on and what the interpreter validates, so
/// the two lists are asserted against each other rather than counted alone.
#[test]
fn a_suspension_carries_live_state_as_resume_arguments() {
    let process = only_process(
        "    reg [3:0] q;\n\
         \x20   initial begin : work\n\
         \x20       integer i;\n\
         \x20       for (i = 0; i < 4; i = i + 1) begin #1 q <= i; end\n\
         \x20   end",
    );
    let waits: Vec<(usize, usize)> = process
        .function
        .blocks
        .iter()
        .filter_map(|block| match &block.terminator {
            CfgTerminator::Wait {
                resume,
                resume_args,
                ..
            } => Some((
                resume_args.len(),
                process.function.block(*resume).params.len(),
            )),
            _ => None,
        })
        .collect();
    assert_eq!(waits.len(), 1, "the loop body suspends once");
    assert_eq!(waits[0].0, waits[0].1, "arguments match parameters");
    assert!(
        waits[0].0 >= 1,
        "the counter must cross the suspension, got {} arguments",
        waits[0].0
    );

    // And every parameter in the process is a four-state value of a declared
    // width — a `real` here would be the analog default leaking in.
    for block in &process.function.blocks {
        for param in &block.params {
            assert!(
                matches!(
                    process.function.value(*param).value_type,
                    CfgValueType::FourState { .. }
                ),
                "parameter {param} is {:?}",
                process.function.value(*param).value_type
            );
        }
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

// ===========================================================================
// Typed SSA merges
// ===========================================================================

/// A merge carries the type of the variable it merges.
///
/// The builder used to create every block parameter as a `real`, which is the
/// analog body's only variable type and was therefore invisible until a
/// process needed one. A four-state merge that arrived as a `real` would reach
/// the interpreter as an analog value in a process, which it refuses — so this
/// is the property every process-local variable rests on.
#[test]
fn a_block_parameter_takes_its_variables_declared_type() {
    use rspice_veriloga::canonical_ir::cfg::{CfgVariable, SsaBuilder};
    use rspice_veriloga::canonical_ir::{DigitalLocalId, VariableId};

    let mut builder = SsaBuilder::new();
    let entry = builder.create_block();
    builder.seal_block(entry);
    let then_block = builder.create_block();
    let else_block = builder.create_block();
    let join = builder.create_block();

    let counter = CfgVariable::DigitalLocal(DigitalLocalId::new(0));
    builder.declare_variable(counter, CfgValueType::FourState { width: 4 });
    // Declared nowhere, so it keeps the default every analog variable has.
    let analog = CfgVariable::Local(VariableId::new(0));

    let condition = builder.push_leaf(CfgValueType::Boolean, CfgValueKind::BooleanConstant(true));
    builder.set_terminator(
        entry,
        CfgTerminator::Branch {
            condition,
            then_target: then_block,
            then_args: Vec::new(),
            else_target: else_block,
            else_args: Vec::new(),
        },
    );
    builder.seal_block(then_block);
    builder.seal_block(else_block);

    for (block, bits, real) in [(then_block, 0b0000u64, 1.0), (else_block, 0b1111, 2.0)] {
        let value = builder.push_leaf(
            CfgValueType::FourState { width: 4 },
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(4, bits)),
        );
        builder.write_variable(counter, block, value);
        let value = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(real));
        builder.write_variable(analog, block, value);
        builder.set_terminator(
            block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
    }
    builder.seal_block(join);

    let merged_counter = builder.read_variable(counter, join).expect("a merge");
    let merged_analog = builder.read_variable(analog, join).expect("a merge");
    assert_eq!(
        builder.value_type_of(merged_counter),
        Some(CfgValueType::FourState { width: 4 })
    );
    assert_eq!(
        builder.value_type_of(merged_analog),
        Some(CfgValueType::Real)
    );

    builder.set_terminator(join, CfgTerminator::Return);
    let function = builder.finish(entry).expect("a valid graph");
    let types: Vec<CfgValueType> = function
        .block(join)
        .params
        .iter()
        .map(|param| function.value(*param).value_type)
        .collect();
    assert_eq!(
        types,
        vec![CfgValueType::FourState { width: 4 }, CfgValueType::Real],
        "the merges keep their types through construction"
    );
}

/// A merge with no reaching definition resolves to the initial value of its own
/// type: `x` at the declared width for a four-state variable, IEEE 1364-2005
/// section 4.2.2, rather than the real zero Verilog-AMS gives an analog one.
///
/// Reached the way it is reached in practice — a loop header read before the
/// back edge exists, so the body already holds the parameter by the time
/// sealing discovers nothing defines it.
#[test]
fn an_undefined_four_state_merge_resolves_to_unknown() {
    use rspice_veriloga::canonical_ir::DigitalLocalId;
    use rspice_veriloga::canonical_ir::cfg::{CfgVariable, SsaBuilder};

    let mut builder = SsaBuilder::new();
    let entry = builder.create_block();
    builder.seal_block(entry);
    let header = builder.create_block();
    let exit = builder.create_block();
    builder.set_terminator(
        entry,
        CfgTerminator::Jump {
            target: header,
            args: Vec::new(),
        },
    );

    let local = CfgVariable::DigitalLocal(DigitalLocalId::new(0));
    builder.declare_variable(local, CfgValueType::FourState { width: 2 });
    // The header is not sealed, so this reserves a parameter whose arguments
    // arrive later — and nothing ever defines the variable on either edge.
    let read = builder.read_variable(local, header).expect("a parameter");
    let negated = builder.push(
        header,
        CfgValueType::FourState { width: 2 },
        CfgValueKind::DigitalBitwiseNot { input: read },
    );
    builder.set_terminator(
        header,
        CfgTerminator::Jump {
            target: exit,
            args: Vec::new(),
        },
    );
    builder.seal_block(header);
    builder.seal_block(exit);
    builder.set_terminator(exit, CfgTerminator::Return);

    let (function, outputs) = builder
        .finish_with_outputs(entry, &[negated])
        .expect("a valid graph");
    let CfgValueKind::DigitalBitwiseNot { input } = function.value(outputs[0]).kind else {
        panic!("the negation survives");
    };
    let CfgValueKind::FourStateConstant(value) = &function.value(input).kind else {
        panic!(
            "an undefined four-state merge must resolve to a four-state constant, got {:?}",
            function.value(input).kind
        );
    };
    assert_eq!(value.spelling(), "xx");
}

// ===========================================================================
// Elaborated hierarchy (IEEE 1364-2005 sections 12.1.2 and 12.3)
// ===========================================================================
//
// The shape pinned here is that there is no hierarchy left. A design of
// several modules lowers to one plan with one signal table, and what survives
// the flattening is identity: which instance a process belongs to, and which
// driver of a net a contribution is.

/// A two-input NAND with an internal net, so a test can see what hoisting does
/// to a name that has nowhere else to go.
const NAND2: &str = "module nand2(y, a, b);\n\
                     \x20   output y;\n\
                     \x20   input a, b;\n\
                     \x20   wire y, a, b, t;\n\
                     \x20   assign t = a & b;\n\
                     \x20   assign y = ~t;\n\
                     endmodule\n";

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

fn elaborated(child: &str, top_section: &str) -> CanonicalDigitalPlan {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(&hierarchy(child, top_section), Some("top"))
        .expect("the hierarchy must elaborate and lower")
        .digital
}

fn signal_names(plan: &CanonicalDigitalPlan) -> Vec<String> {
    plan.signals
        .iter()
        .map(|signal| signal.name.to_string())
        .collect()
}

/// A signal an instance declares and does not connect out keeps the instance
/// path, which is the IEEE 1364-2005 section 12.4 hierarchical name minus the
/// top module. A `.` cannot occur in an identifier, so a hoisted name can
/// never collide with one the author wrote.
#[test]
fn an_instance_signal_is_hoisted_under_its_instance_path() {
    let plan = elaborated(
        NAND2,
        "    wire a, b, n1, y;\n     nand2 g1(n1, a, b);\n     nand2 g2(y, n1, n1);",
    );
    assert_eq!(
        signal_names(&plan),
        vec!["a", "b", "n1", "y", "g1.t", "g2.t"]
    );
}

/// Nesting composes the path rather than restarting it.
#[test]
fn a_nested_instance_carries_the_whole_path() {
    let source = "module inv(y, a);\n\
                  \x20   output y;\n\
                  \x20   input a;\n\
                  \x20   wire y, a, t;\n\
                  \x20   assign t = ~a;\n\
                  \x20   assign y = t;\n\
                  endmodule\n\
                  module pair(y, a);\n\
                  \x20   output y;\n\
                  \x20   input a;\n\
                  \x20   wire y, a, mid;\n\
                  \x20   inv i1(mid, a);\n\
                  \x20   inv i2(y, mid);\n\
                  endmodule\n";
    let plan = elaborated(source, "    wire x, z;\n     pair u1(z, x);");
    assert_eq!(
        signal_names(&plan),
        vec!["x", "z", "u1.mid", "u1.i1.t", "u1.i2.t"]
    );
}

/// A net port and the net it is connected to are one elaborated signal.
///
/// This is the port-connection decision, stated as a count: two instances of a
/// four-signal module connected to four parent nets produce six signals, not
/// twelve. IEEE 1364-2005 section 12.3.10 asks what net type results from
/// connecting two dissimilar nets, a question that only exists because the two
/// become one; section 12.3.9.3 makes an inout connection exactly that join.
#[test]
fn a_connected_net_port_collapses_onto_the_net_it_names() {
    let plan = elaborated(
        NAND2,
        "    wire a, b, n1, y;\n     nand2 g1(n1, a, b);\n     nand2 g2(y, n1, n1);",
    );
    assert_eq!(plan.signals.len(), 6);
    // Two ports of one instance may name the same net, and that is one signal.
    let n1 = plan
        .signals
        .iter()
        .find(|signal| signal.name == "n1")
        .expect("the collapsed net");
    let reads: usize = plan
        .processes
        .iter()
        .flat_map(|process| &process.function.values)
        .filter(|value| {
            matches!(value.kind, CfgValueKind::DigitalSignalRead { signal } if signal == n1.id)
        })
        .count();
    assert!(
        reads >= 2,
        "both of g2's inputs must read the one collapsed net, found {reads} reads"
    );
}

/// An unconnected port is a net of its own: IEEE 1364-2005 section 12.3.9
/// leaves it undriven, which is what a hoisted net nobody names already is.
#[test]
fn an_unconnected_port_keeps_its_own_signal() {
    let plan = elaborated(NAND2, "    wire a, b;\n     nand2 g1(, a, b);");
    assert_eq!(signal_names(&plan), vec!["a", "b", "g1.y", "g1.t"]);
}

/// Two instances of one module are two sets of processes.
///
/// The property a scheduler needs: it resumes *a* process, so two instances of
/// one `always` block cannot be one identity. The source module's own process
/// id is shared between them and is deliberately not used.
#[test]
fn two_instances_of_one_module_own_distinct_processes() {
    let child = "module dff(q, clk, d);\n\
                 \x20   output q;\n\
                 \x20   input clk, d;\n\
                 \x20   reg q;\n\
                 \x20   wire clk, d;\n\
                 \x20   always @(posedge clk) q <= d;\n\
                 endmodule\n";
    let plan = elaborated(
        child,
        "    wire clk, d, q1, q2;\n\
     \x20   dff u1(.q(q1), .clk(clk), .d(d));\n\
     \x20   dff u2(.q(q2), .clk(clk), .d(q1));",
    );
    let always: Vec<_> = plan
        .processes
        .iter()
        .filter(|process| process.kind == DigitalProcessKind::Always)
        .collect();
    assert_eq!(always.len(), 2, "one `always` per instance");
    assert_ne!(
        always[0].id, always[1].id,
        "two instances of one process must be two identities"
    );
    // Each writes its own instance's variable, not one shared signal.
    let written: Vec<_> = always
        .iter()
        .flat_map(|process| &process.function.values)
        .filter_map(|value| match &value.kind {
            CfgValueKind::DigitalNonblockingWrite { target, .. } => Some(target.signal),
            _ => None,
        })
        .collect();
    assert_eq!(written.len(), 2);
    assert_ne!(written[0], written[1]);
}

/// The property this whole design has to preserve: a net driven by two
/// instances has two drivers, told apart by index.
///
/// Collapsing an output port does not fold its driver into the net. A driver
/// is named by net and by index among that net's drivers, so two children
/// driving one parent net are two contributions a resolver sees separately —
/// which is the case where a collapse that *did* merge the drivers would have
/// silently destroyed one of them.
#[test]
fn two_child_outputs_on_one_net_are_two_drivers() {
    let child = "module drv(y, a);\n\
                 \x20   output y;\n\
                 \x20   input a;\n\
                 \x20   wire y, a;\n\
                 \x20   assign y = a;\n\
                 endmodule\n";
    let plan = elaborated(
        child,
        "    wire a, b, bus;\n     drv d1(bus, a);\n     drv d2(bus, b);",
    );
    let bus = plan
        .signals
        .iter()
        .find(|signal| signal.name == "bus")
        .expect("the shared net");
    let drivers: Vec<_> = plan.drivers_of(bus.id).collect();
    assert_eq!(drivers.len(), 2, "one driver per instance");
    assert_eq!(drivers[0].id.index, 0);
    assert_eq!(drivers[1].id.index, 1);
    assert_ne!(
        drivers[0].process, drivers[1].process,
        "each driver is computed by its own instance's process"
    );
}

/// Driver indices are declaration order among the net's drivers, and that
/// order is elaboration order, so it does not move between compilations of the
/// same source.
#[test]
fn elaborated_driver_identities_are_stable_across_recompilation() {
    let child = "module drv(y, a);\n\
                 \x20   output y;\n\
                 \x20   input a;\n\
                 \x20   wire y, a;\n\
                 \x20   assign y = a;\n\
                 endmodule\n";
    let section = "    wire a, b, bus;\n     drv d1(bus, a);\n     drv d2(bus, b);";
    let first = elaborated(child, section);
    let second = elaborated(child, section);
    assert_eq!(first.drivers, second.drivers);
    assert_eq!(first.signals, second.signals);
    assert_eq!(
        first
            .processes
            .iter()
            .map(|process| process.id)
            .collect::<Vec<_>>(),
        second
            .processes
            .iter()
            .map(|process| process.id)
            .collect::<Vec<_>>()
    );
}

/// A variable output port is the one port class that does not collapse.
///
/// IEEE 1364-2005 section 12.3.9.2 lets an output port be a variable, and a
/// variable cannot be joined with a net: one holds what a process wrote, the
/// other is the resolution of its drivers. So the port keeps its own signal
/// and the connection becomes a driver on the connected net — a real driver
/// with a real identity, indistinguishable from an `assign` the parent could
/// have written.
#[test]
fn a_variable_output_port_drives_the_connected_net_through_an_implicit_assignment() {
    let child = "module dff(q, clk, d);\n\
                 \x20   output q;\n\
                 \x20   input clk, d;\n\
                 \x20   reg q;\n\
                 \x20   wire clk, d;\n\
                 \x20   always @(posedge clk) q <= d;\n\
                 endmodule\n";
    let plan = elaborated(
        child,
        "    wire clk, d, q;\n     dff u1(.q(q), .clk(clk), .d(d));",
    );
    // The instance's variable stayed its own signal rather than joining `q`.
    assert_eq!(signal_names(&plan), vec!["clk", "d", "q", "u1.q"]);
    let net = plan
        .signals
        .iter()
        .find(|signal| signal.name == "q")
        .expect("the connected net");
    let variable = plan
        .signals
        .iter()
        .find(|signal| signal.name == "u1.q")
        .expect("the instance's variable");
    assert!(!net.procedurally_assignable, "`q` is a net");
    assert!(variable.procedurally_assignable, "`u1.q` is a variable");

    let drivers: Vec<_> = plan.drivers_of(net.id).collect();
    assert_eq!(drivers.len(), 1, "the connection is the net's one driver");
    assert_eq!(drivers[0].id.index, 0);

    // The synthesized driver is an ordinary continuous-assignment process: it
    // reads the instance's variable and publishes it as this driver's value.
    let process = plan
        .process(drivers[0].process)
        .expect("the driver's process");
    assert_eq!(process.kind, DigitalProcessKind::ContinuousAssign);
    let sensitivity = process
        .static_sensitivity
        .as_ref()
        .expect("a driver waits on its operands");
    assert_eq!(sensitivity.origin, DigitalSensitivityOrigin::Implicit);
    assert_eq!(sensitivity.terms.len(), 1);
    assert_eq!(sensitivity.terms[0].signal, variable.id);
    assert!(
        process
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::DigitalDriverWrite { .. })),
        "the connection publishes a driver contribution"
    );
}

/// A design with no continuous-domain content anywhere — the shape an actual
/// Verilog netlist has — elaborates and lowers like any other.
///
/// Every other fixture here hangs its digital section off a module with an
/// analog body, because that is what a Verilog-AMS device is. This one has
/// none, and is worth pinning separately: nothing in the elaboration or the
/// lowering may depend on there being an analog half to hang from.
#[test]
fn a_design_with_no_analog_content_elaborates() {
    let plan = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(
            &format!(
                "{NAND2}\n\
                 module top(y, a, b);\n\
                 \x20   output y;\n\
                 \x20   input a, b;\n\
                 \x20   wire y, a, b, n1;\n\
                 \x20   nand2 g1(n1, a, b);\n\
                 \x20   nand2 g2(y, n1, n1);\n\
                 endmodule\n"
            ),
            Some("top"),
        )
        .expect("a purely digital design must elaborate")
        .digital;
    assert_eq!(
        signal_names(&plan),
        vec!["y", "a", "b", "n1", "g1.t", "g2.t"]
    );
    assert_eq!(plan.processes.len(), 4);
    assert_eq!(plan.drivers.len(), 4);
}

/// A design with no hierarchy produces exactly the plan it always did: the
/// compiled module's own signals keep their names and their positions, and
/// nothing about the elaboration is visible.
#[test]
fn a_module_with_no_instances_lowers_unchanged() {
    let plan = plan(
        "    wire a, b;\n\
     \x20   wire y;\n\
     \x20   assign y = a & b;",
    );
    assert_eq!(signal_names(&plan), vec!["a", "b", "y"]);
    assert_eq!(plan.processes.len(), 1);
    assert_eq!(plan.drivers.len(), 1);
}
