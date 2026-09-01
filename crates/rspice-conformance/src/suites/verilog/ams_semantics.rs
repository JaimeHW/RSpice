//! Verilog-AMS semantics, one case per clause, each expectation derived from
//! the clause rather than from a run.
//!
//! # What makes this a conformance suite without an oracle
//!
//! There is no open Verilog-AMS simulator to compare against — [`ams`](super::ams)
//! says why, and [`rnm`](super::rnm) answers it a second way by authoring each
//! reference block twice. This suite answers it a third way, and the three do
//! not share a failure mode.
//!
//! Here the reference is the *standard's own text*. Every case carries the
//! clause it checks, a statement of what that clause requires, and the
//! derivation of the expected answer from that requirement. Nothing is read off
//! a run: a case's numbers are computed from the clause's rule and the design's
//! declared parameters, so a change that makes RSpice answer differently fails
//! against the paragraph rather than against a recorded output.
//!
//! # The three verdicts, and why refusals are cases
//!
//! A conformance suite that admitted only passes would have to leave out the
//! most informative half of an implementation's boundary. Three outcomes are
//! recorded, and they are not the same claim:
//!
//! * [`Verdict::Conforms`] — the implementation does what the clause requires.
//! * [`Verdict::RefusesAsTheClauseAllows`] — the clause forbids the construct,
//!   or explicitly leaves it undefined, and RSpice refuses it. That *is*
//!   conformance: a simulator that accepted `wreal` with two continuous drivers
//!   would be inventing a resolution the standard declines to define.
//! * [`Verdict::BoundedByImplementation`] — the clause permits the construct
//!   and RSpice does not implement it. The case pins that the refusal names the
//!   clause and the gap, so the boundary is documented by the suite rather than
//!   discovered by a user. These are the rows to delete when the gap closes.
//!
//! # What this suite does not duplicate
//!
//! Clause 7's discipline resolution — the LRM's Figure 7-3 hierarchy, section
//! 7.4.4.1's basic algorithm and Annex F.2.1's table — is machine-checked
//! in-crate, in `rspice-veriloga`'s `connect::tests`, against the figure's own
//! net list. Restating it here would make two owners of one property and would
//! test the resolver twice while testing nothing about a deck. What this suite
//! adds instead is the part that only exists at deck level: that a
//! `connectrules` block's parameters reach a *mixed module's* boundary, which is
//! a path neither the resolver's tests nor the XSPICE bridge planner's cover.
//!
//! [`ams`]: super::ams
//! [`rnm`]: super::rnm

use std::fmt;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rspice_core::engine::TransientResult;
use rspice_core::netlist::Netlist;
use rspice_core::xspice::verilog::{
    DigitalPort, DigitalRunReport, DigitalStimulus, run_digital_verilog,
};
use rspice_core::{Engine, SimulationConfig};

// ===========================================================================
// The case table
// ===========================================================================

/// What the implementation does about one clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The clause's requirement is met.
    Conforms,
    /// The clause forbids the construct or leaves it undefined, and it is
    /// refused by name.
    RefusesAsTheClauseAllows,
    /// The clause permits the construct, this implementation does not carry it,
    /// and the refusal says so.
    BoundedByImplementation,
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conforms => write!(f, "conforms"),
            Self::RefusesAsTheClauseAllows => write!(f, "refuses, as the clause allows"),
            Self::BoundedByImplementation => write!(f, "bounded by the implementation"),
        }
    }
}

/// One clause, one case.
#[derive(Debug, Clone, Copy)]
pub struct ClauseCase {
    /// Verilog-AMS LRM 2.4 clause number, or the IEEE 1364-2005 one where the
    /// AMS standard inherits it.
    pub clause: &'static str,
    /// Test function name, which is also the row's key.
    pub name: &'static str,
    /// What the clause requires, in this suite's own words. Paraphrased rather
    /// than quoted: the standard is not ours to reproduce.
    pub requirement: &'static str,
    /// How the case's expected answer follows from the requirement. This is the
    /// load-bearing field — it is what makes the expectation a derivation
    /// instead of a golden.
    pub derivation: &'static str,
    pub verdict: Verdict,
}

/// Every case, in clause order.
pub const CASES: &[ClauseCase] = &[
    ClauseCase {
        clause: "IEEE 1364-2005 5.4.1",
        name: "an_assignment_context_sets_the_expression_width",
        requirement: "an expression is evaluated at the width of the largest operand and of \
                      the assignment's left-hand side together, so the left-hand side is part \
                      of the expression's context rather than a truncation applied afterwards",
        derivation: "`4'b1111 + 4'b0001` overflows four bits. Assigned to `reg [7:0]` the \
                     context is eight bits wide, so the addition is done in eight and the \
                     answer is 8'b0001_0000 — sixteen, carry kept. Assigned to `reg [1:0]` the \
                     context is still eight (the operands are four), and the eight-bit answer \
                     is then truncated to its low two bits, which are 2'b00. Both answers come \
                     from one rule; a simulator that sized the addition by its operands alone \
                     would give 8'b0000_0000 for the first.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "IEEE 1364-2005 5.4.2",
        name: "signedness_is_a_property_of_the_operand_not_of_the_target",
        requirement: "an expression is signed only if every operand is signed; a signed value \
                      widened into a larger context is sign-extended, an unsigned one is \
                      zero-extended",
        derivation: "`4'b1000` read through an unsigned `reg [3:0]` and widened to eight bits \
                     is 8'b0000_1000 — eight. The same bits read through a `reg signed [3:0]` \
                     are minus eight, and widening sign-extends them to 8'b1111_1000. The two \
                     differ in the top four bits and nowhere else, which is exactly what \
                     sign-extension means; a positive value such as 4'b0011 must widen the \
                     same way through both.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "LRM 2.4 3.7",
        name: "an_undriven_real_net_is_zero",
        requirement: "a real net that no driver is connected to holds zero, which is also its \
                      initial value",
        derivation: "A module declaring two `wreal` outputs and driving only one must report \
                     0.0 on the other at every observation, including the first — before any \
                     process has run — and must keep reporting it while the driven net moves. \
                     A net left at the four-state `x` a `reg` would start at, or one that \
                     tracked its neighbour, would fail both halves.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "LRM 2.4 6.5.3",
        name: "a_real_net_with_two_drivers_is_refused",
        requirement: "a `wreal` net admits at most one driver, and the standard defines no \
                      function to combine two; the resolved forms `wrealsum`, `wrealavg`, \
                      `wrealmin` and `wrealmax` exist precisely so a design that wants two can \
                      say which combination it means",
        derivation: "Two continuous assignments to one `wreal` is therefore not a value the \
                     standard has an answer for. Refusing it is conformance; picking one \
                     driver, or summing them, would be answering a question the standard \
                     declines to. The refusal must name the net, the count, and the four \
                     resolved spellings, because those are what the author has to choose \
                     between.",
        verdict: Verdict::RefusesAsTheClauseAllows,
    },
    ClauseCase {
        clause: "LRM 2.4 6.5.3",
        name: "a_real_valued_module_port_is_not_a_discipline_boundary",
        requirement: "a real-valued port carries a real number, not a discipline's potential \
                      and flow",
        derivation: "An X-card connects a module port to a circuit node, and a circuit node \
                     carries a potential. A `wreal` port therefore has no A/D or D/A bridge to \
                     be given: bridging it would mean choosing an arbitrary volts-per-unit \
                     scale the design never stated. Refused by name — and the refusal is a \
                     bound rather than conformance, because the clause permits the port and \
                     this route does not carry it.",
        verdict: Verdict::BoundedByImplementation,
    },
    ClauseCase {
        clause: "LRM 2.4 7.3.1",
        name: "a_process_may_read_a_continuous_value_but_not_contribute_to_one",
        requirement: "access functions may be read from a discrete context; contribution \
                      statements belong to the analog block and may not appear in a process",
        derivation: "The two halves are one case because the clause is one rule about \
                     direction. A process that reads `V(p, n)` and drives a discrete output \
                     from it runs, and its output has to follow the analog value it read. A \
                     process containing `I(p, n) <+ …` is not a design this rule admits, so it \
                     is refused rather than being run with the contribution dropped — which \
                     would be simulating a different circuit.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "LRM 2.4 7.3.3",
        name: "a_process_probes_the_continuous_net_at_the_edge_that_woke_it",
        requirement: "a process may probe a continuous net of its module, and the value it \
                      reads is the analog solution at the instant the process runs",
        derivation: "A clock edge arriving across an A/D bridge and a ramp on the module's own \
                     terminal separate the two candidate answers: the solution at the edge's \
                     own timepoint, and the one at the timepoint before it. The ramp is \
                     written so the threshold falls between two clock edges, so a probe \
                     reading the edge's own solution crosses exactly once and a probe reading \
                     a stale one crosses a whole clock period later.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "LRM 2.4 7.3.3",
        name: "a_probe_of_a_net_that_is_not_a_terminal_is_refused_by_name",
        requirement: "a process may probe *any* continuous net of its module, internal nets \
                      included",
        derivation: "An internal analog net has no circuit node until the builder assigns one, \
                     which happens after the module is constructed, so this route reaches only \
                     the nets the deck attached. The refusal has to name the net, the module, \
                     and the clause it is falling short of — a bound stated is a bound a user \
                     can work around, and one that is not is a mystery.",
        verdict: Verdict::BoundedByImplementation,
    },
    ClauseCase {
        clause: "LRM 2.4 7.3.6.1",
        name: "an_analog_timepoint_is_floored_onto_the_tick_grid",
        requirement: "the digital time base has a declared precision, and an analog timepoint \
                      delivered to the discrete domain is mapped onto it",
        derivation: "Flooring rather than rounding is what keeps the mapping from running the \
                     discrete domain past an instant the integrator has accepted. So a \
                     boundary change caused at accepted time `t` is published into the tick \
                     `floor(t / 1 ns)`, which is at or before `t` and within one tick of it. \
                     Measured through a process that waits a declared delay after the change: \
                     its activation is that delay past the publication tick, so subtracting \
                     the delay recovers the tick the mapping chose.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "LRM 2.4 7.3.6.2",
        name: "a_digital_activation_is_an_exact_analog_timepoint",
        requirement: "an event scheduled in the discrete domain is synchronised with the \
                      analog solver, which must not step past it",
        derivation: "The activation's time is a tick, and a tick's seconds are exact for every \
                     tick this range admits. Synchronisation therefore means the analog \
                     stepper accepts a timepoint whose `f64` is that value bit for bit — not \
                     one within tolerance of it, because 'within tolerance' of an event time \
                     is a different instant and the design's next state is decided there. A \
                     module toggling every five ticks over a microsecond gives two hundred \
                     consecutive chances to be off by one ulp.",
        verdict: Verdict::Conforms,
    },
    ClauseCase {
        clause: "LRM 2.4 7.7.3",
        name: "connect_rules_parameterize_a_mixed_module_boundary",
        requirement: "a `connectrules` block selects connect modules for the boundaries of a \
                      design and may override their parameters",
        derivation: "The `vsup` a connect statement supplies is the supply the boundary \
                     converts against, and an analog-to-discrete boundary switches at half of \
                     it. Driving the boundary net with a ramp from 0 V to 5 V over 100 ns puts \
                     the switching instant at `100 ns * (vsup / 2) / 5 V`: 33 ns for the \
                     deck's default 3.3 V supply and 10 ns for a rule supplying 1 V. The \
                     accepted timepoint that records it is the first at or after that instant, \
                     so each is bounded above by one maximum step.",
        verdict: Verdict::Conforms,
    },
];

/// Look one case up, refusing rather than answering for a name that is not in
/// the table.
///
/// Every test calls this with its own name, which is what keeps the table and
/// the suite from drifting: a test whose row is deleted fails, and a row whose
/// test is deleted fails [`census`].
pub fn case(name: &'static str) -> &'static ClauseCase {
    CASES
        .iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("`{name}` has no row in the clause table"))
}

/// The clauses this suite is required to cover.
///
/// Named rather than counted, so a clause that stops being covered fails on its
/// own number.
pub const REQUIRED_CLAUSES: &[&str] = &[
    "IEEE 1364-2005 5.4.1",
    "IEEE 1364-2005 5.4.2",
    "LRM 2.4 3.7",
    "LRM 2.4 6.5.3",
    "LRM 2.4 7.3.1",
    "LRM 2.4 7.3.3",
    "LRM 2.4 7.3.6.1",
    "LRM 2.4 7.3.6.2",
    "LRM 2.4 7.7.3",
];

// ===========================================================================
// Running a case
// ===========================================================================

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A `.va` written to a unique path, deleted when the guard drops.
///
/// Unique because the engine's Verilog-A cache is keyed by canonical path, so
/// two cases sharing a filename would share a compiled model.
pub struct ModelFile(PathBuf);

impl ModelFile {
    pub fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_ams_semantics_{name}_{}_{sequence}.va",
            std::process::id()
        ));
        let mut file = std::fs::File::create(&path).expect("create the case's model file");
        file.write_all(source.as_bytes())
            .expect("write the case's model file");
        Self(path)
    }

    /// The path as a deck writes it: forward slashes, so the same string works
    /// on both hosts.
    pub fn deck_path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// Run a deck, or report why it would not run.
pub fn run_deck(deck: &str, tstop: f64, max_step: f64) -> Result<TransientResult, String> {
    let netlist =
        Netlist::parse(deck).map_err(|error| format!("the deck does not parse: {error}"))?;
    Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .map_err(|error| error.to_string())
}

/// Run a discrete-domain design against a vector stimulus.
pub fn run_digital(source: &str, stimulus: &DigitalStimulus) -> Result<DigitalRunReport, String> {
    run_digital_verilog(source, stimulus).map_err(|error| error.to_string())
}

/// A scalar or vector port declaration; width zero is a real port.
pub fn port(name: &str, width: u32) -> DigitalPort {
    DigitalPort {
        name: name.to_string(),
        width,
    }
}

/// A stimulus with no clock, sampled halfway through each vector.
pub fn vector_stimulus(
    module: &str,
    inputs: Vec<DigitalPort>,
    outputs: Vec<DigitalPort>,
    vectors: Vec<Vec<&str>>,
) -> DigitalStimulus {
    DigitalStimulus {
        module: Some(module.to_string()),
        inputs,
        outputs,
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors
            .into_iter()
            .map(|row| row.into_iter().map(str::to_string).collect())
            .collect(),
    }
}

/// One output port's value at one observation.
pub fn observed(report: &DigitalRunReport, step: usize, port: &str) -> String {
    report
        .observations
        .get(step)
        .unwrap_or_else(|| panic!("observation {step} is missing"))
        .values
        .iter()
        .find(|(name, _)| name == port)
        .map(|(_, value)| value.clone())
        .unwrap_or_else(|| panic!("observation {step} has no port `{port}`"))
}

/// The times a boundary net's recorded value changed, opening value included.
pub fn transition_times(result: &TransientResult, net: &str) -> Vec<f64> {
    result
        .digital_trace_named(net)
        .unwrap_or_else(|| panic!("net `{net}` has no digital trace"))
        .iter()
        .map(|point| point.time)
        .collect()
}

// ===========================================================================
// Designs
// ===========================================================================

/// IEEE 1364-2005 section 5.4.1: one addition, two assignment contexts.
pub const WIDTH_CONTEXT: &str = r#"
module width_context(a, b, wide, narrow);
  input [3:0] a;
  input [3:0] b;
  output [7:0] wide;
  output [1:0] narrow;
  reg [7:0] wide;
  reg [1:0] narrow;
  always @(a or b) begin
    wide = a + b;
    narrow = a + b;
  end
endmodule
"#;

/// IEEE 1364-2005 section 5.4.2: the same bits read signed and unsigned.
pub const SIGN_CONTEXT: &str = r#"
module sign_context(a, unsigned_wide, signed_wide);
  input [3:0] a;
  output [7:0] unsigned_wide;
  output [7:0] signed_wide;
  reg [7:0] unsigned_wide;
  reg [7:0] signed_wide;
  reg signed [3:0] as_signed;
  always @(a) begin
    unsigned_wide = a;
    as_signed = a;
    signed_wide = as_signed;
  end
endmodule
"#;

/// LRM 2.4 section 3.7: one driven real net beside one that nothing drives.
pub const UNDRIVEN_REAL: &str = r#"
module undriven_real(sel, driven, undriven);
  input sel;
  output wreal driven;
  output wreal undriven;
  assign driven = sel ? 1.5 : 2.5;
endmodule
"#;

/// LRM 2.4 section 6.5.3: two continuous drivers on one `wreal`.
pub const TWO_REAL_DRIVERS: &str = r#"
module two_real_drivers(sel, out);
  input sel;
  output wreal out;
  assign out = sel ? 1.0 : 2.0;
  assign out = 3.0;
endmodule
"#;

/// LRM 2.4 section 6.5.3 at a deck boundary: a real-valued module port.
pub const REAL_PORT_MODULE: &str = r#"
`include "disciplines.vams"
module real_port_module(p, n, clk, level);
    inout p, n;
    electrical p, n;
    input clk;
    output wreal level;
    wire clk;
    assign level = 1.5;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// LRM 2.4 section 7.3.1, read half: a process reading its module's continuous
/// terminal and driving a discrete output from it.
pub const READS_CONTINUOUS: &str = r#"
`include "disciplines.vams"
module reads_continuous(p, n, clk, q);
    inout p, n;
    electrical p, n;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= (V(p, n) > 1.0);
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// LRM 2.4 section 7.3.1, contribution half: a process containing a
/// contribution statement.
pub const CONTRIBUTES_FROM_A_PROCESS: &str = r#"
`include "disciplines.vams"
module contributes_from_a_process(p, n, clk, q);
    inout p, n;
    electrical p, n;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) begin
        q <= ~q;
        I(p, n) <+ 1.0e-3;
    end
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

/// LRM 2.4 section 7.3.3: a process probing an internal analog net.
pub const PROBES_AN_INTERNAL_NET: &str = r#"
`include "disciplines.vams"
module probes_an_internal_net(p, n, clk, q);
    inout p, n;
    electrical p, n;
    electrical mid;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= (V(mid) > 0.5);
    analog begin
        I(p, mid) <+ V(p, mid) / 1000.0;
        I(mid, n) <+ V(mid, n) / 1000.0;
    end
endmodule
"#;

/// LRM 2.4 section 7.3.6.1: a process that reacts a declared delay after a
/// boundary change, so the tick the change was published into is readable.
pub const DELAYED_REACTION: &str = r#"
`include "disciplines.vams"
module delayed_reaction(p, n, c, y);
    inout p, n;
    electrical p, n;
    input c;
    output y;
    wire c;
    reg y;
    initial y = 1'b0;
    always @(c) #3 y = ~y;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// LRM 2.4 section 7.3.6.2: a module scheduling its own activations.
pub const SELF_CLOCKED_DIVIDER: &str = r#"
`include "disciplines.vams"
module self_clocked_divider(p, n, qdiv);
    inout p, n;
    electrical p, n;
    output qdiv;
    reg clk, qdiv;
    initial clk = 1'b0;
    initial qdiv = 1'b0;
    always #5 clk = ~clk;
    always @(posedge clk) qdiv <= ~qdiv;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// LRM 2.4 section 7.7.3: the boundary a connect rule parameterizes.
pub const RAMP_TOGGLE: &str = r#"
`include "disciplines.vams"
module ramp_toggle(p, n, clk, q);
    inout p, n;
    electrical p, n;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= ~q;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// The shipped connect modules plus one `connectrules` block, as a `.va` file.
///
/// The module sources are the library's own, verbatim: a suite that wrote its
/// own would pin its copy rather than what a deck actually gets.
pub fn connect_library(rules: &str) -> ModelFile {
    let mut source = String::new();
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str(rules);
    ModelFile::new("connect_library", &source)
}
