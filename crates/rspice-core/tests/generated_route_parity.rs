//! Generated route parity: the shipped Rust against the `.va` runtime.
//!
//! A `veriloga-model-*` feature ships a compact model as Rust generated at
//! build time from a `.va` source that is also in this repository. Two routes,
//! one set of equations — and nothing states that they answer the same. This
//! file runs one deck twice, once resolving the X card to the generated device
//! and once with a `.va` directive naming the *same source file* (an authored
//! source takes precedence over the built-in alias), and compares the
//! operating point and a short transient point for point.
//!
//! Bit-identity is not the bar here, unlike `route_parity.rs` in
//! `rspice-veriloga`: the two lowerings schedule their arithmetic differently
//! and each answer is the output of a Newton solve, so the rows ask for a
//! tight *relative* agreement (1e-12, with a small absolute floor) and report
//! the measured numbers when they miss it.
//!
//! Each row asserts it actually took two different routes — a deck that
//! silently resolved both runs to the same device would pass while testing
//! nothing.
#![cfg(all(feature = "veriloga", feature = "veriloga-model-diode-cmc"))]

use rspice_core::{Engine, Netlist};

/// Relative agreement asked of two lowerings of one model through one solve.
const RELATIVE_TOLERANCE: f64 = 1.0e-12;

/// Absolute floor, so a quantity passing through zero is not compared by ratio.
/// Currents in these decks are milliamps at most; volts are order one.
const ABSOLUTE_FLOOR: f64 = 1.0e-15;

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    key: String,
    value: f64,
    text: Option<String>,
}

fn num(key: impl Into<String>, value: f64) -> Entry {
    Entry {
        key: key.into(),
        value,
        text: None,
    }
}

fn text(key: impl Into<String>, text: impl Into<String>) -> Entry {
    Entry {
        key: key.into(),
        value: 0.0,
        text: Some(text.into()),
    }
}

/// Repository-relative path to a `.va` source, in deck-safe spelling.
///
/// The deck parser treats a backslash as an escape, so the separator is
/// normalized here rather than at every call site.
fn model_source(relative: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(relative);
    let path = path.canonicalize().unwrap_or_else(|err| {
        panic!(
            "the generated model's own Verilog-A source must be in the tree: {} ({err})",
            path.display()
        )
    });
    path.display()
        .to_string()
        .replace('\\', "/")
        .replace("//?/", "")
}

fn node_voltage(result: &rspice_core::solver::SimulationResult, name: &str) -> Option<f64> {
    let index = result
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
    result.node_voltages.get(index).copied()
}

fn branch_current(result: &rspice_core::solver::SimulationResult, name: &str) -> Option<f64> {
    let index = result
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
    result.branch_currents.get(index).copied()
}

/// True when the built circuit resolved an X card to a generated device.
fn uses_generated_devices(deck: &str) -> bool {
    let netlist = Netlist::parse_validated(deck).expect("deck parses");
    Engine::default()
        .build_circuit(&netlist)
        .expect("deck builds")
        .has_generated_veriloga_devices()
}

fn operating_point_trace(deck: &str, nodes: &[&str], branches: &[&str]) -> Vec<Entry> {
    let netlist = match Netlist::parse_validated(deck) {
        Ok(netlist) => netlist,
        Err(err) => return vec![text("op", format!("parse: {err}"))],
    };
    let result = match Engine::default().run_dc_op(&netlist) {
        Ok(result) => result,
        Err(err) => return vec![text("op", err.to_string())],
    };
    let mut entries = vec![text("op", "ok")];
    for node in nodes {
        match node_voltage(&result, node) {
            Some(voltage) => entries.push(num(format!("op.v({node})"), voltage)),
            None => entries.push(text(format!("op.v({node})"), "absent")),
        }
    }
    for branch in branches {
        match branch_current(&result, branch) {
            Some(current) => entries.push(num(format!("op.i({branch})"), current)),
            None => entries.push(text(format!("op.i({branch})"), "absent")),
        }
    }
    entries
}

fn transient_trace(
    deck: &str,
    tstop: f64,
    tstep: f64,
    nodes: &[&str],
    branches: &[&str],
) -> Vec<Entry> {
    let netlist = match Netlist::parse_validated(deck) {
        Ok(netlist) => netlist,
        Err(err) => return vec![text("tran", format!("parse: {err}"))],
    };
    let result = match Engine::default().run_tran(&netlist, tstop, tstep) {
        Ok(result) => result,
        Err(err) => return vec![text("tran", err.to_string())],
    };
    let mut entries = vec![text("tran", "ok")];
    entries.push(num("tran.points", result.time.len() as f64));
    let node_indices: Vec<Option<usize>> = nodes
        .iter()
        .map(|name| {
            result
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .collect();
    let branch_indices: Vec<Option<usize>> = branches
        .iter()
        .map(|name| {
            result
                .branch_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .collect();
    for (point, time) in result.time.iter().enumerate() {
        entries.push(num(format!("tran[{point}].t"), *time));
        for (node, index) in nodes.iter().zip(&node_indices) {
            match index.and_then(|index| {
                result
                    .voltages
                    .get(index)
                    .and_then(|series| series.get(point))
            }) {
                Some(voltage) => entries.push(num(format!("tran[{point}].v({node})"), *voltage)),
                None => entries.push(text(format!("tran[{point}].v({node})"), "absent")),
            }
        }
        for (branch, index) in branches.iter().zip(&branch_indices) {
            match index.and_then(|index| {
                result
                    .branch_currents
                    .get(index)
                    .and_then(|series| series.get(point))
            }) {
                Some(current) => entries.push(num(format!("tran[{point}].i({branch})"), *current)),
                None => entries.push(text(format!("tran[{point}].i({branch})"), "absent")),
            }
        }
    }
    entries
}

fn agrees(generated: f64, runtime: f64) -> bool {
    if generated.to_bits() == runtime.to_bits() {
        return true;
    }
    if generated.is_nan() || runtime.is_nan() {
        return false;
    }
    let difference = (generated - runtime).abs();
    difference <= ABSOLUTE_FLOOR
        || difference <= RELATIVE_TOLERANCE * generated.abs().max(runtime.abs())
}

fn assert_traces_agree(label: &str, generated: &[Entry], runtime: &[Entry]) {
    let mut differences = Vec::new();
    for index in 0..generated.len().max(runtime.len()) {
        match (generated.get(index), runtime.get(index)) {
            (Some(left), Some(right)) if left.key != right.key => differences.push(format!(
                "[{index}] observation order: generated '{}', runtime '{}'",
                left.key, right.key
            )),
            (Some(left), Some(right)) if left.text != right.text => differences.push(format!(
                "{}: generated {:?}, runtime {:?}",
                left.key, left.text, right.text
            )),
            (Some(left), Some(right))
                if left.text.is_none() && !agrees(left.value, right.value) =>
            {
                let relative = (left.value - right.value).abs()
                    / left
                        .value
                        .abs()
                        .max(right.value.abs())
                        .max(f64::MIN_POSITIVE);
                differences.push(format!(
                    "{}: generated {:.17e}, runtime {:.17e} (relative {relative:.3e})",
                    left.key, left.value, right.value
                ));
            }
            (Some(_), Some(_)) => {}
            (Some(left), None) => differences.push(format!(
                "[{index}] '{}' only on the generated route",
                left.key
            )),
            (None, Some(right)) => differences.push(format!(
                "[{index}] '{}' only on the runtime route",
                right.key
            )),
            (None, None) => unreachable!("index is below one of the two lengths"),
        }
    }
    assert!(
        differences.is_empty(),
        "{label}: {} route difference(s) between the generated device and the .va runtime \
         ({} generated observations, {} runtime):\n{}",
        differences.len(),
        generated.len(),
        runtime.len(),
        differences
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// A row: one deck body, run against the built-in and against its own source.
struct Row<'a> {
    label: &'a str,
    module: &'a str,
    source: &'a str,
    body: &'a str,
    nodes: &'a [&'a str],
    branches: &'a [&'a str],
    tstop: f64,
    tstep: f64,
}

impl Row<'_> {
    fn generated_deck(&self) -> String {
        format!("* {} generated route\n{}\n.end\n", self.label, self.body)
    }

    fn runtime_deck(&self) -> String {
        format!(
            "* {} runtime route\n.va \"{}\" {}\n{}\n.end\n",
            self.label,
            model_source(self.source),
            self.module,
            self.body
        )
    }

    /// Both decks must build, and they must not build the same thing.
    fn assert_routes_differ(&self) {
        assert!(
            uses_generated_devices(&self.generated_deck()),
            "{}: the plain X card must resolve to the generated built-in",
            self.label
        );
        assert!(
            !uses_generated_devices(&self.runtime_deck()),
            "{}: the .va directive must take the X card off the generated route",
            self.label
        );
    }

    fn compare_operating_point(&self) {
        self.assert_routes_differ();
        let generated = operating_point_trace(&self.generated_deck(), self.nodes, self.branches);
        let runtime = operating_point_trace(&self.runtime_deck(), self.nodes, self.branches);
        assert_eq!(
            generated.first().and_then(|entry| entry.text.as_deref()),
            Some("ok"),
            "{}: the generated route must reach an operating point: {generated:?}",
            self.label
        );
        assert_traces_agree(self.label, &generated, &runtime);
    }

    fn compare_transient(&self) {
        self.assert_routes_differ();
        let generated = transient_trace(
            &self.generated_deck(),
            self.tstop,
            self.tstep,
            self.nodes,
            self.branches,
        );
        let runtime = transient_trace(
            &self.runtime_deck(),
            self.tstop,
            self.tstep,
            self.nodes,
            self.branches,
        );
        assert_eq!(
            generated.first().and_then(|entry| entry.text.as_deref()),
            Some("ok"),
            "{}: the generated route must run the transient: {generated:?}",
            self.label
        );
        assert_traces_agree(self.label, &generated, &runtime);
    }
}

// ---------------------------------------------------------------------------
// DIODE_CMC — charges and `ddt`
// ---------------------------------------------------------------------------

const DIODE_CMC_SOURCE: &str = "models/veriloga/cmc/diode_cmc_3.0_20250714/vacode/diode_cmc.va";

fn diode_cmc_row(body: &str) -> Row<'_> {
    Row {
        label: "DIODE_CMC",
        module: "DIODE_CMC",
        source: DIODE_CMC_SOURCE,
        body,
        nodes: &["in", "a"],
        branches: &["V1"],
        tstop: 20.0e-9,
        tstep: 1.0e-9,
    }
}

#[test]
fn diode_cmc_operating_point_agrees_across_routes() {
    diode_cmc_row("V1 in 0 0.8\nR1 in a 1k\nX1 a 0 DIODE_CMC").compare_operating_point();
}

/// The two routes walk the same time grid *and* the same trajectory along it.
///
/// They did not before `76849f71b`. The grid this row compares is the deck's
/// `tstep`, but the internal steps behind it were the stepper's, and the
/// stepper heard from only one of the two routes. A deck whose only reactive
/// state is a `ddt` operand instantiates none of the native reactive families,
/// so `ngspice_device_truncation_covers_transient_lte` found every one of them
/// vacuously controlled and handed the deck's accuracy to the charge rule —
/// which then walked native charge vectors an authored `ddt` is in none of.
/// The generated route was nevertheless bounded by
/// `generated_veriloga_ngspice_truncation_limit`, and the runtime route by
/// nothing at all, so the same equations were integrated over different step
/// sequences and reported different values at the same output times: 54 of 122
/// observations differed, v(a) 1.87345989660413583e-3 generated against
/// 1.92713544463720093e-3 runtime (2.785e-2) and i(V1)
/// -6.39811580402103769e-7 against -5.86136032369038741e-7 (8.389e-2) at point
/// 3, decaying to 2.002e-5 and 2.390e-3 by point 8. Neither lowering's
/// arithmetic was ever involved, which is why the operating point agreed to
/// 1e-12 throughout.
#[test]
fn diode_cmc_transient_agrees_across_routes() {
    diode_cmc_row("V1 in 0 SIN(0 1 5e7)\nR1 in a 1k\nX1 a 0 DIODE_CMC").compare_transient();
}

// ---------------------------------------------------------------------------
// JUNCAP200 — junction charge storage, a second two-terminal generated family.
//
// Its generated crate is 289 KB of Rust. The feature is not in this lane's
// gate set, so the rows carry their own `cfg` and a build that selects the
// model picks them up.
// ---------------------------------------------------------------------------

#[cfg(feature = "veriloga-model-juncap200")]
const JUNCAP200_SOURCE: &str = "models/veriloga/cmc/PSP104.1.0_vacode/vacode/juncap200.va";

#[cfg(feature = "veriloga-model-juncap200")]
fn juncap200_row(body: &str) -> Row<'_> {
    Row {
        label: "JUNCAP200",
        module: "JUNCAP200",
        source: JUNCAP200_SOURCE,
        body,
        nodes: &["in", "a"],
        branches: &["V1"],
        tstop: 20.0e-9,
        tstep: 1.0e-9,
    }
}

#[cfg(feature = "veriloga-model-juncap200")]
#[test]
fn juncap200_operating_point_agrees_across_routes() {
    juncap200_row("V1 in 0 -1.0\nR1 in a 1k\nX1 a 0 JUNCAP200").compare_operating_point();
}

/// A second charge-storing generated family, held to the same agreement.
///
/// It diverged with the DIODE_CMC row above and on exactly the same count —
/// 54 of 122 observations, 4.625e-2 on v(a) and 8.872e-2 on i(V1) at point 3,
/// decaying to 2.810e-5 and 2.219e-3 by point 8 — because what went missing
/// was the deck's step control and not any one model's: nothing in it knew
/// which module it was failing to bound. The row stays so that the timestep
/// authority an authored `ddt` operand carries is asserted by two unrelated
/// models rather than one.
#[cfg(feature = "veriloga-model-juncap200")]
#[test]
fn juncap200_transient_agrees_across_routes() {
    juncap200_row("V1 in 0 SIN(0 1 5e7)\nR1 in a 1k\nX1 a 0 JUNCAP200").compare_transient();
}

// ---------------------------------------------------------------------------
// r3_cmc — the shipped model the audit names for read-before-assign
//
// The generated crate is 185 KB of Rust — the smallest of the three families
// here, and far inside the budget — so the model is included. Its feature is
// not in this lane's gate set either, so the rows carry their own `cfg`.
//
// Both rows pass. That is a statement about these decks, not an acquittal of
// the read-before-assign difference `route_parity.rs` pins: r3_cmc reads a
// module-scope variable before assigning it, and on this stimulus the two
// routes still agree, so whatever the persistence difference costs is not
// reached here. R4.1 stays open on the row that isolates it.
// ---------------------------------------------------------------------------

#[cfg(feature = "veriloga-model-r3-cmc")]
const R3_CMC_SOURCE: &str = "models/veriloga/cmc/r3_cmc_release1.1.2_2023Jun16/r3_cmc.va";

#[cfg(feature = "veriloga-model-r3-cmc")]
fn r3_cmc_row(body: &str) -> Row<'_> {
    Row {
        label: "r3_cmc",
        module: "r3_cmc",
        source: R3_CMC_SOURCE,
        body,
        nodes: &["in", "a", "b"],
        branches: &["V1"],
        tstop: 1.0e-6,
        tstep: 5.0e-8,
    }
}

/// Terminals are `(n1, nc, n2, dt)`: the body node `nc` and the thermal node
/// `dt` each need a DC path of their own.
#[cfg(feature = "veriloga-model-r3-cmc")]
const R3_CMC_BODY: &str = "R1 in a 1k\nX1 a 0 b t r3_cmc\nRB b 0 1k\nRT t 0 1meg";

#[cfg(feature = "veriloga-model-r3-cmc")]
#[test]
fn r3_cmc_operating_point_agrees_across_routes() {
    r3_cmc_row(&format!("V1 in 0 1.0\n{R3_CMC_BODY}")).compare_operating_point();
}

#[cfg(feature = "veriloga-model-r3-cmc")]
#[test]
fn r3_cmc_transient_agrees_across_routes() {
    r3_cmc_row(&format!("V1 in 0 SIN(0 1 1e6)\n{R3_CMC_BODY}")).compare_transient();
}

// ---------------------------------------------------------------------------
// Finiteness: what the generated route hands the solver at a wild iterate
//
// The runtime route audits every stamped value (`finite_stamp_value` in
// `rspice-veriloga`'s device adapter) and refuses a point that produced a
// non-finite one. Generated Rust emits no such check and the adapter around it
// made none either, so a module whose arithmetic left the reals at the iterate
// the solver offered wrote NaN or infinity straight into the matrix. The
// transient then read the failed linear solve as "cut dt" and DC walked the
// whole ladder with no diagnostic at all.
// ---------------------------------------------------------------------------

/// Every sink accumulates, and the observation survives to the end of the
/// evaluation rather than stopping at the first offender.
///
/// The audit is deliberately not an early exit: generated code emits its
/// equations in whatever order the backend scheduled them, and an exit would
/// make the diagnostic depend on that order while leaving the rest of the
/// device out of a system the caller is about to discard anyway.
#[test]
fn a_generated_stamper_reports_which_sink_received_a_non_finite_value() {
    use rspice_core::device::veriloga_builtins::GeneratedStamper;
    use rspice_core::solver::{ComplexMatrix, StaticMatrix};

    let entries = (0..2)
        .flat_map(|row| (0..2).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    let voltages = [1.0, 2.0];

    // A DC/transient stamper writes both sinks, and the equivalent source it
    // builds is `value - dI/dV * V`: a non-finite derivative therefore poisons
    // the right-hand side as well, which is why the two flags are reported
    // together rather than as one bit.
    for (label, value, derivative, expected) in [
        ("finite", 1.0, 2.0, None),
        ("value", f64::NAN, 2.0, Some("right-hand-side contribution")),
        (
            "derivative",
            1.0,
            f64::INFINITY,
            Some("Jacobian entry and right-hand-side contribution"),
        ),
    ] {
        let mut matrix = StaticMatrix::from_triplets(2, 2, &entries).expect("dense 2x2");
        let mut rhs = [0.0; 2];
        let mut stamper = GeneratedStamper::new(&mut matrix, &mut rhs, &voltages, 2);
        // Two contributions, the second always finite: an early exit would
        // lose the first observation or skip the second stamp.
        stamper.stamp_current_node1(Some(1), Some(2), value, 1, derivative);
        stamper.stamp_current_node1(Some(1), Some(2), 3.0, 2, 4.0);
        assert_eq!(
            stamper.non_finite_contribution(),
            expected,
            "{label}: the stamper must report which sink saw a non-finite value"
        );
    }

    // The AC-real stamper has no right-hand side, so it isolates the Jacobian.
    let structure = StaticMatrix::from_triplets(2, 2, &entries).expect("dense 2x2");
    let mut matrix = ComplexMatrix::from_real_structure(&structure);
    let mut stamper = GeneratedStamper::new_ac_real(&mut matrix, &voltages, 2);
    stamper.stamp_current_node1(Some(1), Some(2), 1.0, 1, f64::INFINITY);
    stamper.stamp_current_node1(Some(1), Some(2), 3.0, 2, 4.0);
    assert_eq!(stamper.non_finite_contribution(), Some("Jacobian entry"));
}
