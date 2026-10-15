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

/// The small-signal response at each frequency, as an AC plot reads it: every
/// named node voltage and branch current in decibels and degrees.
fn ac_trace(deck: &str, frequencies: &[f64], nodes: &[&str], branches: &[&str]) -> Vec<Entry> {
    let netlist = match Netlist::parse_validated(deck) {
        Ok(netlist) => netlist,
        Err(err) => return vec![text("ac", format!("parse: {err}"))],
    };
    let points = match Engine::default().run_ac(&netlist, frequencies) {
        Ok(points) => points,
        Err(err) => return vec![text("ac", err.to_string())],
    };
    let mut entries = vec![text("ac", "ok")];
    for (index, point) in points.iter().enumerate() {
        entries.push(num(format!("ac[{index}].f"), point.frequency));
        let named = |names: &[String], values: &[rspice_core::Complex64], name: &str| {
            names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .and_then(|position| values.get(position).copied())
        };
        let observations = nodes
            .iter()
            .map(|node| {
                (
                    format!("v({node})"),
                    named(&point.node_names, &point.voltages, node),
                )
            })
            .chain(branches.iter().map(|branch| {
                (
                    format!("i({branch})"),
                    named(&point.branch_names, &point.currents, branch),
                )
            }));
        for (label, value) in observations {
            match value {
                Some(value) => {
                    entries.push(num(
                        format!("ac[{index}].db({label})"),
                        20.0 * value.norm().log10(),
                    ));
                    entries.push(num(
                        format!("ac[{index}].ph({label})"),
                        value.arg().to_degrees(),
                    ));
                }
                None => entries.push(text(format!("ac[{index}].{label}"), "absent")),
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

    fn compare_ac(&self, frequencies: &[f64]) {
        self.assert_routes_differ();
        let generated = ac_trace(
            &self.generated_deck(),
            frequencies,
            self.nodes,
            self.branches,
        );
        let runtime = ac_trace(&self.runtime_deck(), frequencies, self.nodes, self.branches);
        assert_eq!(
            generated.first().and_then(|entry| entry.text.as_deref()),
            Some("ok"),
            "{}: the generated route must run the AC analysis: {generated:?}",
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

/// The small-signal response, in decibels and degrees, from the conduction
/// knee to where the junction capacitance carries the current.
#[test]
fn diode_cmc_ac_agrees_across_routes() {
    diode_cmc_row("V1 in 0 DC 0.7 AC 1\nR1 in a 1k\nX1 a 0 DIODE_CMC")
        .compare_ac(&[1.0e3, 1.0e6, 1.0e8, 1.0e9, 1.0e10]);
}

// ---------------------------------------------------------------------------
// A deck without node 0
//
// The build takes the first voltage source's negative terminal as the
// reference. It used to choose it after every device was built and shift every
// higher node number down by one, device by device; the generated devices were
// never shifted, so they read and loaded their neighbours' nets. A diode in
// such a deck carried 2.53e-2 A in a transient where the same deck naming the
// reference 0 carried -1.9e-23 A, and the operating point did not converge.
// ---------------------------------------------------------------------------

/// One deck run twice, naming its reference `0` and naming it `ref`, which the
/// build must then choose: the operating point and the transient must report
/// the same bits. `generated` says which route the deck must take.
fn assert_reference_choice_is_invisible(label: &str, deck: &str, generated: bool) {
    let grounded = deck.replace("{ref}", "0");
    let chosen = deck.replace("{ref}", "ref");
    assert_eq!(
        uses_generated_devices(&chosen),
        generated,
        "{label}: the deck must take the route the row names"
    );
    let (nodes, branches) = (&["in", "a"][..], &["V1"][..]);
    for (analysis, with_zero, without_zero) in [
        (
            "op",
            operating_point_trace(&grounded, nodes, branches),
            operating_point_trace(&chosen, nodes, branches),
        ),
        (
            "tran",
            transient_trace(&grounded, 20.0e-9, 1.0e-9, nodes, branches),
            transient_trace(&chosen, 20.0e-9, 1.0e-9, nodes, branches),
        ),
    ] {
        assert_eq!(
            with_zero.first().and_then(|entry| entry.text.as_deref()),
            Some("ok"),
            "{label} {analysis} with node 0: {with_zero:?}"
        );
        let same = |left: &Entry, right: &Entry| {
            left.key == right.key
                && left.text == right.text
                && left.value.to_bits() == right.value.to_bits()
        };
        let differences: Vec<String> = with_zero
            .iter()
            .zip(&without_zero)
            .filter(|(left, right)| !same(left, right))
            .map(|(left, right)| format!("{left:?} with node 0, {right:?} without"))
            .collect();
        assert!(
            with_zero.len() == without_zero.len() && differences.is_empty(),
            "{label} {analysis}: {} of {} observations differ without node 0 ({} without):\n{}",
            differences.len(),
            with_zero.len(),
            without_zero.len(),
            differences
                .iter()
                .take(12)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

#[test]
fn a_deck_without_node_zero_matches_it_for_a_generated_diode_x_card() {
    assert_reference_choice_is_invisible(
        "DIODE_CMC X card",
        "* diode_cmc X card without node 0\nV1 in {ref} SIN(0.5 0.5 5e7)\nR1 in a 1k\n\
         X1 a {ref} DIODE_CMC\n.end\n",
        true,
    );
}

#[test]
fn a_deck_without_node_zero_matches_it_for_a_generated_diode_d_card() {
    assert_reference_choice_is_invisible(
        "DIODE_CMC D card",
        "* diode_cmc D card without node 0\nV1 in {ref} SIN(0.5 0.5 5e7)\nR1 in a 1k\n\
         D1 a {ref} dmod\n.model dmod diode_cmc\n.end\n",
        true,
    );
}

#[test]
fn a_deck_without_node_zero_matches_it_for_the_diode_cmc_va_runtime() {
    let deck = format!(
        "* diode_cmc .va runtime without node 0\n.va \"{}\" DIODE_CMC\n\
         V1 in {{ref}} SIN(0.5 0.5 5e7)\nR1 in a 1k\nX1 a {{ref}} DIODE_CMC\n.end\n",
        model_source(DIODE_CMC_SOURCE)
    );
    assert_reference_choice_is_invisible("DIODE_CMC .va runtime", &deck, false);
}

#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn a_deck_without_node_zero_matches_it_for_a_generated_resistor_r_card() {
    assert_reference_choice_is_invisible(
        "R2_CMC R card",
        "* r2_cmc R card without node 0\nV1 in {ref} SIN(0.5 0.5 5e7)\nR1 in a rmod r=1k\n\
         R2 a {ref} 2k\n.model rmod r2_cmc\n.end\n",
        true,
    );
}

/// The R card leaves R2_ET_CMC's thermal node out, and it is a net of the
/// instance's own: it does not touch node 0, so the deck still takes `ref` as
/// its reference.
#[cfg(feature = "veriloga-model-r2-et-cmc")]
#[test]
fn a_deck_without_node_zero_matches_it_with_an_omitted_thermal_node() {
    let deck = "* r2_et_cmc R card without node 0\nV1 in {ref} SIN(0.5 0.5 5e7)\n\
                R1 in a rmod r=1k\nR2 a {ref} 2k\n.model rmod r2_et_cmc\n.end\n";
    let netlist = Netlist::parse_validated(&deck.replace("{ref}", "ref")).expect("deck parses");
    let circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("deck builds");
    assert_eq!(circuit.get_node_by_name("ref"), Some(0));
    assert_reference_choice_is_invisible("R2_ET_CMC R card", deck, true);
}

/// ANGELOV_GAN declares its thermal node `t` electrical and contributes to it
/// against ground (`I(t) <+ V(t) / Rth_T` with self-heating, `I(t) <+ V(t) *
/// SPICE_GMIN` without), so an instance with `t` connected ties it to node 0,
/// and a deck that names no node 0 keeps node 0.
/// The `.va` route reads that from the module's canonical IR as it builds;
/// the generated route reads what the generator recorded from the same IR.
/// They must decide alike.
#[cfg(feature = "veriloga-model-angelov-gan")]
#[test]
fn both_routes_keep_node_zero_for_a_module_that_reaches_a_port_single_ended() {
    let reference = |deck: &str| {
        let netlist = Netlist::parse_validated(deck).unwrap_or_else(|err| panic!("{err}\n{deck}"));
        Engine::default()
            .build_circuit(&netlist)
            .unwrap_or_else(|err| panic!("{err}\n{deck}"))
            .get_node_by_name("ref")
    };
    let body = "V1 d ref 5\nV2 g ref -1\nX1 d g ref t rf angelov_gan\nRt t ref 1k\n\
                Rrf rf ref 50\n.end\n";
    let generated = format!("* angelov_gan generated without node 0\n{body}");
    let runtime = format!(
        "* angelov_gan .va without node 0\n.va \"{}\" angelov_gan\n{body}",
        model_source("models/veriloga/angelov_2.0/vacode/angelov_gan.va")
    );
    assert!(uses_generated_devices(&generated));
    assert!(!uses_generated_devices(&runtime));
    let (generated, runtime) = (reference(&generated), reference(&runtime));
    assert_eq!(
        generated, runtime,
        "both routes must decide the reference alike"
    );
    assert_ne!(
        generated,
        Some(0),
        "`I(t) <+` ties the connected thermal node to node 0, so the deck keeps it"
    );
}

// ---------------------------------------------------------------------------
// `$port_connected` — a card that leaves trailing terminals out
//
// The `.va` route reads a terminal the card leaves out unconnected. The
// generated route folded `$port_connected` to 1, so BSIM-SOI 4.7 with
// `tnodeout=1` and `t` left out took the 7-terminal arm there, `Temp(t) <+ 0`
// with `t` on node 0, a singular system, where the `.va` route took the `b` arm
// and tied `b`; and an X card that omitted a terminal was refused outright.
//
// BSIM-SOI's generated crate is 914 KB of Rust and its `.va` compile is not a
// debug build's work, so the rows carry their own `cfg` and run in a release
// build that selects the model.
// ---------------------------------------------------------------------------

#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
const BSIMSOI47_SOURCE: &str = "models/veriloga/cmc/BSIM-SOI_4.7.0_05192025/code/bsimsoi.va";

/// BSIM-SOI 4.7's source under the generated catalog's name for it.
///
/// The catalog calls the model `bsimsoi__18c250bc` and the source declares
/// `bsimsoi`. Under one name, one X card takes the generated route when the
/// deck includes nothing and the `.va` route when it includes this copy, since
/// an authored source takes precedence over the catalog.
#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
struct Bsimsoi47Source(std::path::PathBuf);

#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
impl Bsimsoi47Source {
    fn write(label: &str) -> Self {
        let source = std::fs::read_to_string(model_source(BSIMSOI47_SOURCE))
            .expect("BSIM-SOI 4.7's source is in the tree");
        let renamed = source.replace("module bsimsoi(", "module bsimsoi__18c250bc(");
        assert_ne!(renamed, source, "the source declares module bsimsoi");
        let path = std::env::temp_dir().join(format!(
            "rspice_port_connected_{label}_{}.va",
            std::process::id()
        ));
        std::fs::write(&path, renamed).expect("write the renamed source");
        Self(path)
    }

    fn runtime_deck(&self, label: &str, body: &str) -> String {
        format!(
            "* {label} runtime route\n.va \"{}\" bsimsoi__18c250bc\n{body}\n.end\n",
            self.0.display().to_string().replace('\\', "/")
        )
    }
}

#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
impl Drop for Bsimsoi47Source {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// The value an operating-point trace recorded for `key`.
fn traced(trace: &[Entry], key: &str) -> f64 {
    trace
        .iter()
        .find(|entry| entry.key == key && entry.text.is_none())
        .unwrap_or_else(|| panic!("{key} is not in the trace: {trace:?}"))
        .value
}

/// With `tnodeout=1` and self-heating off, BSIM-SOI 4.7 puts its temperature
/// node on the last terminal the card connects: a 6-terminal card leaves `t`
/// out and the model ties `b` (`V(b) <+ 0`); a 7-terminal card gives `t` and
/// the model ties it (`Temp(t) <+ 0`). The X card and the M card both take the
/// generated route; the `.va` route takes the X card. Every route must tie the
/// same node, exactly, and agree on the rest bit for bit. On six terminals no
/// equation reaches the instance's own `t`, which takes the floor and reads
/// 0 V on every route.
#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
#[test]
fn bsimsoi_reads_an_omitted_thermal_terminal_unconnected_on_both_routes() {
    let source = Bsimsoi47Source::write("omitted");
    let supplies = "Vd d 0 0.1\nVg g 0 1.0\nVp vp 0 0.5\nRp vp p 1k\nVb vb 0 0.3\nRb vb b 1k\n\
                    Vt vt 0 0.2\nRt vt t 1k";
    let nodes = &["d", "g", "p", "b", "t"][..];
    let branches = &["Vd", "Vp", "Vb", "Vt"][..];
    for (terminals, tied, free) in [("d g 0 0 p b", "b", "t"), ("d g 0 0 p b t", "t", "b")] {
        let label = format!("BSIM-SOI 4.7 on {terminals}");
        let x_card = format!("{supplies}\nX1 {terminals} bsimsoi__18c250bc tnodeout=1");
        let m_card = format!("{supplies}\nM1 {terminals} nch tnodeout=1\n.model nch bsimsoi47");
        // On six terminals the instance's own `t` is reached by no equation
        // of the model; it takes the floor again and reads exactly 0 V.
        let omitted = |route_name: &str| (tied == "b").then(|| route_name.to_string());
        fn with_omitted<'a>(nodes: &[&'a str], name: &'a Option<String>) -> Vec<&'a str> {
            nodes.iter().copied().chain(name.as_deref()).collect()
        }
        let split_omitted = |label: &str, mut trace: Vec<Entry>, name: &Option<String>| {
            if let Some(name) = name {
                let key = format!("op.v({name})");
                let index = trace
                    .iter()
                    .position(|entry| entry.key == key)
                    .unwrap_or_else(|| panic!("{label}: {key} is not in {trace:?}"));
                let entry = trace.remove(index);
                assert!(
                    entry.text.is_none() && entry.value.to_bits() == 0.0f64.to_bits(),
                    "{label}: the instance's own t reads 0 V: {entry:?}"
                );
            }
            trace
        };
        let runtime_omitted = omitted("X1.t");
        let runtime = split_omitted(
            &label,
            operating_point_trace(
                &source.runtime_deck(&label, &x_card),
                &with_omitted(nodes, &runtime_omitted),
                branches,
            ),
            &runtime_omitted,
        );
        assert!(!uses_generated_devices(
            &source.runtime_deck(&label, &x_card)
        ));
        assert_eq!(
            runtime[0].text.as_deref(),
            Some("ok"),
            "{label}: {runtime:?}"
        );
        assert_eq!(
            traced(&runtime, &format!("op.v({tied})")).to_bits(),
            0.0f64.to_bits(),
            "{label}: the .va route ties {tied} to node 0"
        );
        assert_ne!(
            traced(&runtime, &format!("op.v({free})")),
            0.0,
            "{label}: the .va route leaves {free} untied"
        );
        for (card, body) in [("M", &m_card), ("X", &x_card)] {
            let deck = format!("* {label} generated {card} card\n{body}\n.end\n");
            assert!(uses_generated_devices(&deck), "{label}: the {card} card");
            let generated_omitted = omitted(&format!("{card}1.__t.internal"));
            let generated =
                operating_point_trace(&deck, &with_omitted(nodes, &generated_omitted), branches);
            assert_eq!(
                generated[0].text.as_deref(),
                Some("ok"),
                "{label} {card} card: {generated:?}"
            );
            let generated = split_omitted(&label, generated, &generated_omitted);
            let bits = |trace: &[Entry]| {
                trace
                    .iter()
                    .map(|entry| (entry.key.clone(), entry.text.clone(), entry.value.to_bits()))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                bits(&generated),
                bits(&runtime),
                "{label}: the generated {card} card against the .va route\n\
                 generated {generated:?}\nruntime {runtime:?}"
            );
        }
    }
}

/// With `p`, `b` and `t` left out, `soimod=2` ties neither `p` nor `b`, and
/// nothing else in the module reaches a connected port single-ended: a deck
/// that names no node 0 takes its reference. The generated route read all
/// three connected, tied `b` and `t` (each node 0 itself) and kept node 0 for
/// those ties alone.
#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
#[test]
fn bsimsoi_takes_the_same_reference_on_both_routes_when_its_body_terminals_are_omitted() {
    let source = Bsimsoi47Source::write("census");
    let reference = |deck: &str| {
        let netlist = Netlist::parse_validated(deck).unwrap_or_else(|err| panic!("{err}\n{deck}"));
        Engine::default()
            .build_circuit(&netlist)
            .unwrap_or_else(|err| panic!("{err}\n{deck}"))
            .get_node_by_name("ref")
    };
    let supplies = "Vd d ref 0.1\nVg g ref 1.0";
    let x_card = format!("{supplies}\nX1 d g ref ref bsimsoi__18c250bc soimod=2");
    let runtime = reference(&source.runtime_deck("census", &x_card));
    assert_eq!(
        runtime,
        Some(0),
        "the .va route connects nothing to node 0, so the deck takes `ref`"
    );
    for (card, body) in [
        (
            "M",
            format!("{supplies}\nM1 d g ref ref nch soimod=2\n.model nch bsimsoi47"),
        ),
        ("X", x_card.clone()),
    ] {
        let deck = format!("* census generated {card} card\n{body}\n.end\n");
        assert!(uses_generated_devices(&deck), "the {card} card");
        assert_eq!(
            reference(&deck),
            runtime,
            "the generated {card} card must take the reference the .va route takes"
        );
    }
}

// ---------------------------------------------------------------------------
// A compact model's own nodes take no simulator GMIN floor
//
// BSIM-SOI's floating body balances junction currents of about 1e-18 A. The
// final DC solve put its 1e-15 S nodal floor on that node too, which held the
// body at 1.2 mV instead of 35 mV: both routes ran the threshold high and the
// drain current 31% low. The model owns its internal topology and its own
// GMIN, and ngspice ends its continuation with no diagonal GMIN at all
// (`CKTdiagGmin = CKTgshunt`, `cktop.c`).
// ---------------------------------------------------------------------------

#[cfg(feature = "veriloga-model-bsimsoi-va")]
const BSIMSOI461_SOURCE: &str = "models/veriloga/bsimsoi_4.6.1/vacode/bsimsoi.va";

/// BSIM-SOI 4.6.1 on its default card at Vd = Vg = 1 V, body floating,
/// against Xyce 7.10's ADMS build of the same source: `M1 d g 0 0 nch` with
/// `.model nch nmos level=70 shmod=0` gives I(VD) = -7.98129602e-08 A, and
/// with `shmod=1 rth0=50` I(VD) = -8.39461504e-08 A and N(M1_t) =
/// 8.39461503e-01 K (Xyce prints nine digits). With the floor on the body the
/// routes gave -5.500e-8 A, and -5.705e-8 A with a 0.571 K rise.
///
/// The deck's drain node keeps the 1e-15 S final floor, so `Vd` also feeds
/// 1e-15 A into it at 1 V: RSpice's I(Vd) is Xyce's plus that floor current.
///
/// At `rth0=120` the cold root sits near the runaway, and Xyce's own answer
/// depends on its path by 2.6e-8: its direct operating point gives
/// -9.09270331e-08 A and 2.18224879 K, and its `.dc Vd 0 1 0.05` sweep
/// -9.09270307e-08 A and 2.18224874 K. The sweep's value is pinned, to 1e-8.
/// Each card here has one root; at `rth0=140` the cold root and a hot one
/// (176.6 K) coexist, and the routes need not land on the same one.
#[cfg(feature = "veriloga-model-bsimsoi-va")]
#[test]
fn bsimsoi_floating_body_matches_xyce_with_no_floor_on_the_models_own_nodes() {
    const DRAIN_FLOOR_CURRENT: f64 = -1e-15;
    let source = model_source(BSIMSOI461_SOURCE);
    for (card, xyce_current, rise, tolerance) in [
        ("shmod=0", -7.981_296_02e-8, None, 2e-9),
        (
            "shmod=1 rth0=50",
            -8.394_615_04e-8,
            Some(8.394_615_03e-1),
            2e-9,
        ),
        (
            "shmod=1 rth0=120",
            -9.092_703_07e-8,
            Some(2.182_248_74),
            1e-8,
        ),
    ] {
        let current = xyce_current + DRAIN_FLOOR_CURRENT;
        let body = format!("Vd d 0 1.0\nVg g 0 1.0\nX1 d g 0 0 bsimsoi_va {card}");
        for (route, deck, thermal) in [
            (
                ".va",
                format!("* {card} .va route\n.va \"{source}\" bsimsoi_va\n{body}\n.end\n"),
                "X1.t",
            ),
            (
                "generated",
                format!("* {card} generated route\n{body}\n.end\n"),
                "X1.__t.internal",
            ),
        ] {
            assert_eq!(
                uses_generated_devices(&deck),
                route == "generated",
                "{route}"
            );
            let trace = operating_point_trace(&deck, &[thermal], &["Vd"]);
            assert_eq!(
                trace[0].text.as_deref(),
                Some("ok"),
                "{card} {route}: {trace:?}"
            );
            let id = traced(&trace, "op.i(Vd)");
            assert!(
                ((id - current) / current).abs() < tolerance,
                "{card} {route}: I(Vd) {id:e} against Xyce's {xyce_current:e} plus the floor"
            );
            if let Some(rise) = rise {
                let temperature = traced(&trace, &format!("op.v({thermal})"));
                assert!(
                    ((temperature - rise) / rise).abs() < tolerance,
                    "{card} {route}: rise {temperature:e} K against Xyce's {rise:e} K"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// An omitted terminal is a net of the instance's own
//
// The Verilog-AMS LRM makes an unconnected port's net an implicit net local to
// the instance, and an OSDI simulator builds it so: ngspice's `OSDIsetup`
// makes a node for every terminal past the connected ones. Both routes put it
// on node 0 instead, so a module that ties the port itself, `Temp(t) <+ 0`,
// stamped a potential branch from ground to ground and the operating point was
// singular: every 4-terminal BSIM-BULK, ASM-HEMT and BSIM-SOI 4.7 card. The
// `.va` route names the net as it names an internal one, `X1.t`; the generated
// route as it names its own, `X1.__t.internal`.
// ---------------------------------------------------------------------------

/// The operating point of `deck`, the instance's own net `private` recorded
/// under `key` whichever name the route gives it.
#[cfg(any(
    feature = "veriloga-model-bsimbulk",
    feature = "veriloga-model-asmhemt",
    feature = "veriloga-model-bsimsoi-18c250bc"
))]
fn operating_point_with_private_net(
    deck: &str,
    nodes: &[&str],
    branches: &[&str],
    private: &str,
    key: &str,
) -> Vec<Entry> {
    let mut all = nodes.to_vec();
    all.push(private);
    let mut trace = operating_point_trace(deck, &all, branches);
    let spelled = format!("op.v({private})");
    if let Some(entry) = trace.iter_mut().find(|entry| entry.key == spelled) {
        entry.key = format!("op.v({key})");
    }
    trace
}

/// Every trace reached an operating point and records the same bits as the
/// first.
#[cfg(any(
    feature = "veriloga-model-bsimbulk",
    feature = "veriloga-model-asmhemt",
    feature = "veriloga-model-bsimsoi-18c250bc"
))]
fn assert_operating_points_identical(label: &str, traces: &[(&str, Vec<Entry>)]) {
    let bits = |trace: &[Entry]| {
        trace
            .iter()
            .map(|entry| (entry.key.clone(), entry.text.clone(), entry.value.to_bits()))
            .collect::<Vec<_>>()
    };
    let (first, reference) = &traces[0];
    for (card, trace) in traces {
        assert_eq!(
            trace.first().and_then(|entry| entry.text.as_deref()),
            Some("ok"),
            "{label} {card}: {trace:?}"
        );
        assert_eq!(
            bits(trace),
            bits(reference),
            "{label}: {card} against {first}\n{card} {trace:?}\n{first} {reference:?}"
        );
    }
}

/// The `.va` route alone, on a module a debug build compiles at once: the card
/// leaves `t` out and the module ties it. On node 0 that tie is `V(0) <+ 0`,
/// and the operating point is singular; on a net of the instance's own it
/// holds that net at 0 V and the rest of the circuit solves. The net does not
/// touch node 0, so a deck that names no node 0 still takes its reference.
#[test]
fn an_omitted_port_the_module_ties_is_a_net_of_the_instances_own() {
    let path = std::env::temp_dir().join(format!("rspice_omitted_tie_{}.va", std::process::id()));
    std::fs::write(
        &path,
        "`include \"disciplines.vams\"\nmodule tie(a, b, t);\ninout a, b, t;\n\
         electrical a, b, t;\nanalog begin\n  I(a, b) <+ V(a, b) / 1k;\n  V(t) <+ 0;\nend\n\
         endmodule\n",
    )
    .expect("write the module");
    let source = path.display().to_string().replace('\\', "/");
    let deck = |reference: &str| {
        format!(
            "* omitted tied port\n.va \"{source}\" tie\nV1 in {reference} 1\n\
             X1 in {reference} tie\n.end\n"
        )
    };
    let trace = operating_point_trace(&deck("0"), &["in", "x1.t"], &["V1"]);
    let netlist = Netlist::parse_validated(&deck("ref")).expect("deck parses");
    let reference = Engine::default()
        .build_circuit(&netlist)
        .expect("deck builds")
        .get_node_by_name("ref");
    let _ = std::fs::remove_file(&path);
    assert_eq!(trace[0].text.as_deref(), Some("ok"), "{trace:?}");
    assert_eq!(traced(&trace, "op.v(x1.t)"), 0.0);
    assert!(
        (traced(&trace, "op.i(V1)") + 1.0e-3).abs() < 1.0e-9,
        "1 V across 1k, and gmin: {trace:?}"
    );
    assert_eq!(
        reference,
        Some(0),
        "the tie reaches only the instance's own net, so the deck takes `ref`"
    );
}

/// A compact model's own nodes take no GMIN floor, so the model alone must
/// give each one a DC path. An omitted port no equation reaches (`u`: empty
/// row, column and right-hand side) is solved by any value and nothing sees
/// it; it takes the floor again and reads 0 V, Spectre's "Gmin installed".
/// One the model reads but never drives (`r`: an empty row under a stamped
/// column) has no canonical value, so the operating point refuses it by name.
/// A settled node must read 0 V once the operating point converges: with
/// `m` set, the model drives `m * V(a, b)` into `u` and nothing drains it,
/// the floor alone would read -1 mV, and the operating point refuses `u` by
/// name.
#[test]
fn a_compact_models_own_node_with_no_dc_path_settles_or_is_refused_by_name() {
    let path = std::env::temp_dir().join(format!("rspice_no_dc_path_{}.va", std::process::id()));
    std::fs::write(
        &path,
        "`include \"disciplines.vams\"\nmodule open(a, b, u, r);\ninout a, b, u, r;\n\
         electrical a, b, u, r;\nparameter real k = 0;\nparameter real m = 0;\n\
         analog begin\n  I(a, b) <+ V(a, b) / 1k + k * V(r);\n  I(u) <+ m * V(a, b);\n\
         end\nendmodule\n",
    )
    .expect("write the module");
    let source = path.display().to_string().replace('\\', "/");
    let deck = |card: &str| {
        format!(
            "* port with no DC path\n.va \"{source}\" open\nV1 in 0 1\nX1 in 0 open {card}\n.end\n"
        )
    };
    let settled = operating_point_trace(&deck(""), &["in", "x1.u", "x1.r"], &["V1"]);
    let refused = operating_point_trace(&deck("k=1m"), &["in"], &["V1"]);
    let undrained = operating_point_trace(&deck("m=1e-18"), &["in"], &["V1"]);
    let _ = std::fs::remove_file(&path);
    assert_eq!(settled[0].text.as_deref(), Some("ok"), "{settled:?}");
    assert_eq!(traced(&settled, "op.v(x1.u)").to_bits(), 0.0f64.to_bits());
    assert_eq!(traced(&settled, "op.v(x1.r)").to_bits(), 0.0f64.to_bits());
    let message = |trace: &[Entry]| {
        trace[0]
            .text
            .as_deref()
            .unwrap_or_default()
            .to_ascii_lowercase()
    };
    assert!(
        message(&refused).contains("node x1.r has no dc path")
            && message(&refused).contains("constrains it at dc"),
        "the refusal names the node: {refused:?}"
    );
    assert!(
        message(&undrained).contains("node x1.u has no dc path")
            && message(&undrained).contains("gives that current no path"),
        "the settled node fails its post-condition by name: {undrained:?}"
    );
}

/// BSIM-BULK 107.2.1 on `d g s b`, `t` left out: `Temp(t) <+ 0` with
/// self-heating off. The `.va` route's X card, the generated X card and the
/// generated M card reach the same operating point, bit for bit, the thermal
/// net held at 0.
#[cfg(feature = "veriloga-model-bsimbulk")]
#[test]
fn bsimbulk_reaches_an_operating_point_with_its_thermal_terminal_omitted() {
    let supplies = "Vd d 0 0.5\nVg g 0 1.0";
    let x_card = format!("{supplies}\nX1 d g 0 0 bsimbulk L=1u W=10u");
    let m_card = format!("{supplies}\nM1 d g 0 0 nch L=1u W=10u\n.model nch bsimbulk");
    let runtime = format!(
        "* bsimbulk runtime route\n.va \"{}\" bsimbulk\n{x_card}\n.end\n",
        model_source("models/veriloga/cmc/BSIM-BULK107.2.1_02112025/code/bsimbulk.va")
    );
    let generated_x = format!("* bsimbulk generated X card\n{x_card}\n.end\n");
    let generated_m = format!("* bsimbulk generated M card\n{m_card}\n.end\n");
    assert!(!uses_generated_devices(&runtime));
    assert!(uses_generated_devices(&generated_x) && uses_generated_devices(&generated_m));
    let (nodes, branches) = (&["d", "g"][..], &["Vd", "Vg"][..]);
    let traces = [
        (
            ".va X",
            operating_point_with_private_net(&runtime, nodes, branches, "x1.t", "t"),
        ),
        (
            "generated X",
            operating_point_with_private_net(&generated_x, nodes, branches, "x1.__t.internal", "t"),
        ),
        (
            "generated M",
            operating_point_with_private_net(&generated_m, nodes, branches, "m1.__t.internal", "t"),
        ),
    ];
    assert_operating_points_identical("BSIM-BULK on d g s b", &traces);
    assert_eq!(traced(&traces[0].1, "op.v(t)"), 0.0);
    assert!(
        traced(&traces[0].1, "op.i(Vd)") < -1.0e-6,
        "{:?}",
        traces[0].1
    );
}

/// ASM-HEMT 101.6.0 on `d g s b`, `dt` left out. With self-heating off the
/// module ties `dt` (`Temp(dt) <+ 0`); `shmod` is a model-card parameter, so
/// the generated route takes it on the M card's `.model`. With the default
/// self-heating (`shmod=1`, `rth0=5`) the thermal network heats `dt`, which
/// on node 0 it never could.
///
/// The `.va` route matches ngspice-46 running `asmhemt.osdi` on the same N
/// card to 1e-12: `i(Vd)` -0.273940199358580 A with self-heating off, and
/// -0.273673402940843 A at a 1.368367014699217 K rise with it on. The
/// generated cards agree with each other bit for bit, but the generated
/// lowering of ASM-HEMT sits 3e-8 relative from both, with `dt` connected
/// as much as without it, so they are held to that against the `.va` route.
#[cfg(feature = "veriloga-model-asmhemt")]
#[test]
fn asmhemt_reaches_an_operating_point_with_its_thermal_terminal_omitted() {
    let supplies = "Vd d 0 1.0\nVg g 0 0";
    let source = model_source("models/veriloga/cmc/ASM-HEMT101.6.0_05132026/vacode/asmhemt.va");
    let (nodes, branches) = (&["d", "g"][..], &["Vd", "Vg"][..]);
    let close = |left: f64, right: f64, relative: f64| {
        (left - right).abs() <= relative * left.abs().max(right.abs())
    };
    for (heating, instance, model, drain, rise) in [
        ("shmod=0", " shmod=0", " shmod=0", -0.273940199358580, 0.0),
        ("default", "", "", -0.273673402940843, 1.368367014699217),
    ] {
        let label = format!("ASM-HEMT on d g s b, {heating}");
        let x_card = format!("{supplies}\nX1 d g 0 0 asmhemt{instance}");
        let runtime =
            format!("* {label} runtime route\n.va \"{source}\" asmhemt\n{x_card}\n.end\n");
        let generated_m = format!(
            "* {label} generated M card\n{supplies}\nM1 d g 0 0 hemt\n.model hemt asmhemt{model}\n.end\n"
        );
        assert!(!uses_generated_devices(&runtime));
        assert!(uses_generated_devices(&generated_m));
        let va = operating_point_with_private_net(&runtime, nodes, branches, "x1.dt", "dt");
        assert_eq!(va[0].text.as_deref(), Some("ok"), "{label}: {va:?}");
        assert!(
            close(traced(&va, "op.i(Vd)"), drain, 1.0e-12),
            "{label}: ngspice-OSDI drain current {drain}: {va:?}"
        );
        assert!(
            close(traced(&va, "op.v(dt)"), rise, 1.0e-12),
            "{label}: ngspice-OSDI temperature rise {rise}: {va:?}"
        );
        let mut generated = vec![(
            "generated M",
            operating_point_with_private_net(
                &generated_m,
                nodes,
                branches,
                "m1.__dt.internal",
                "dt",
            ),
        )];
        if instance.is_empty() {
            let generated_x = format!("* {label} generated X card\n{x_card}\n.end\n");
            assert!(uses_generated_devices(&generated_x));
            generated.push((
                "generated X",
                operating_point_with_private_net(
                    &generated_x,
                    nodes,
                    branches,
                    "x1.__dt.internal",
                    "dt",
                ),
            ));
        }
        assert_operating_points_identical(&label, &generated);
        for (left, right) in generated[0].1.iter().zip(&va) {
            assert!(
                left.key == right.key
                    && left.text == right.text
                    && close(left.value, right.value, 1.0e-7),
                "{label}: generated {left:?} against .va {right:?}"
            );
        }
    }
}

/// BSIM-SOI 4.7 on `d g s e`, `p`, `b` and `t` left out: the floating-body
/// configuration, with the body on a net of its own. With `shmod=0` the
/// module ties `t`; with `shmod=1` and `rth0` the temperature rise is the
/// thermal network's on `t`, which is above ambient. The generated X and M
/// cards reach the same operating point bit for bit, and the `.va` route
/// reaches it to 1e-13.
///
/// The floating body balances junction currents of about 5e-18 A on a
/// conductance of its own of about 1e-16 S. With the 1e-15 S floor gone from
/// the model's own nodes that root is ill-conditioned, and it magnifies any
/// rounding difference in those currents. The generated junction code is not
/// op for op with the `.va` lowering: at `rth0=50` the body sits 2 ULP apart
/// and I(Vd) 1 ULP.
#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
#[test]
fn bsimsoi_reaches_an_operating_point_with_body_and_thermal_terminals_omitted() {
    let source = Bsimsoi47Source::write("floating");
    let supplies = "Vd d 0 1.0\nVg g 0 1.0";
    let (nodes, branches) = (&["d", "g"][..], &["Vd", "Vg"][..]);
    for (heating, parameters) in [("shmod=0", "shmod=0"), ("shmod=1", "shmod=1 rth0=50")] {
        let label = format!("BSIM-SOI 4.7 on d g s e, {heating}");
        let x_card = format!("{supplies}\nX1 d g 0 0 bsimsoi__18c250bc {parameters}");
        let m_card = format!("{supplies}\nM1 d g 0 0 nch {parameters}\n.model nch bsimsoi47");
        let runtime = source.runtime_deck(&label, &x_card);
        let generated_x = format!("* {label} generated X card\n{x_card}\n.end\n");
        let generated_m = format!("* {label} generated M card\n{m_card}\n.end\n");
        assert!(!uses_generated_devices(&runtime));
        assert!(uses_generated_devices(&generated_x) && uses_generated_devices(&generated_m));
        let traces = [
            (
                ".va X",
                operating_point_with_private_net(&runtime, nodes, branches, "x1.t", "t"),
            ),
            (
                "generated X",
                operating_point_with_private_net(
                    &generated_x,
                    nodes,
                    branches,
                    "x1.__t.internal",
                    "t",
                ),
            ),
            (
                "generated M",
                operating_point_with_private_net(
                    &generated_m,
                    nodes,
                    branches,
                    "m1.__t.internal",
                    "t",
                ),
            ),
        ];
        let (va, generated) = traces.split_first().expect("three routes");
        assert_operating_points_identical(&label, generated);
        assert_eq!(va.1.len(), generated[0].1.len(), "{label}: {va:?}");
        for (left, right) in generated[0].1.iter().zip(&va.1) {
            assert!(
                left.key == right.key
                    && left.text == right.text
                    && (left.value - right.value).abs()
                        <= 1e-13 * left.value.abs().max(right.value.abs()),
                "{label}: generated {left:?} against .va {right:?}"
            );
        }
        let rise = traced(&traces[0].1, "op.v(t)");
        if heating == "shmod=0" {
            assert_eq!(rise, 0.0, "{label}");
        } else {
            assert!(
                rise > 0.0,
                "{label}: the device heats its own net, {rise} K"
            );
        }
    }
}

/// BSIM-SOI 4.7 at `shmod=1 rth0=1e3`, Vd = Vg = 1 V, has no operating point:
/// thermal runaway. The operating point must refuse, and in bounded time.
///
/// The runaway, measured against Xyce 7.10's ADMS BSIM-SOI 4.6 (`level=70`,
/// 4.6.1 and 4.7 differ there by under 0.12%):
/// - Xyce's `.dc Vd` continuation finds the cold root at Vd = 1 V for
///   rth0 = 140, 200, 250 and 400, rising 2.61, 4.06, 5.52 and 14.03 K. Its
///   direct operating point jumps to the hot middle-branch root at 140.
/// - At rth0 = 1e3 the cold branch folds near Vd = 0.43 V, so no cold root
///   exists at Vd = 1 V.
/// - At rth0 = 140 RSpice's `.va` route lands on the hot root and the
///   generated route on the cold one. The routes are compared only where the
///   root is single (shmod=0, rth0=50 and rth0=120).
/// - ngspice's native B4SOI converges here with a 17.3 K rise. It is BSIM-SOI
///   4.4, from before 4.6 carried the self-heating rise into the
///   temperature-dependent values (`bsimsoi.va:3661`, "v4.6").
///
/// GMIN stepping creeps toward the fold. It gives up as ngspice's dynamic GMIN
/// does (`cktop.c`, `dynamic_gmin`): when a failed stage finds the step
/// factor below 1.00005. Before, it crept for 13,035 Newton iterations, about
/// 0.7 s here and an estimated half hour on the interpreter's `.va` route.
/// Now it takes
/// 6,817 iterations and about 0.2 s.
#[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
#[test]
fn bsimsoi_thermal_runaway_is_refused_in_bounded_time() {
    let supplies = "Vd d 0 1.0\nVg g 0 1.0";
    let parameters = "shmod=1 rth0=1e3";
    let mut refusals = Vec::new();
    for (card, body) in [
        (
            "X",
            format!("{supplies}\nX1 d g 0 0 bsimsoi__18c250bc {parameters}"),
        ),
        (
            "M",
            format!("{supplies}\nM1 d g 0 0 nch {parameters}\n.model nch bsimsoi47"),
        ),
    ] {
        let deck = format!("* BSIM-SOI 4.7 runaway, generated {card} card\n{body}\n.end\n");
        assert!(uses_generated_devices(&deck), "the {card} card");
        let start = std::time::Instant::now();
        let trace = operating_point_trace(&deck, &[], &[]);
        let elapsed = start.elapsed();
        let message = trace[0].text.clone().unwrap_or_default();
        assert!(
            message.contains("GMIN stepping stopped"),
            "the {card} card refuses: {trace:?}"
        );
        if !cfg!(debug_assertions) {
            assert!(
                elapsed < std::time::Duration::from_secs(30),
                "the {card} card refuses in {elapsed:?}"
            );
        }
        refusals.push(message);
    }
    assert_eq!(refusals[0], refusals[1], "both cards refuse alike");

    // The `.va` route on the native JIT refuses in bounded time too, and as a
    // convergence failure: a non-finite `ddx()` operand at a wild iterate is
    // a rejected trial, not a fault of the JIT. The interpreter's `.va` route
    // refuses with the same message after the same 2,013 iterations, but takes
    // 230 s in release, about 114 ms per iteration: it re-evaluates the
    // model's parameter preprocessing on every evaluation. That cost is
    // inherent to interpretation, so that route is not timed here.
    #[cfg(feature = "veriloga-native")]
    {
        let source = Bsimsoi47Source::write("runaway");
        let deck = source.runtime_deck(
            "BSIM-SOI 4.7 runaway",
            &format!("{supplies}\nX1 d g 0 0 bsimsoi__18c250bc {parameters}"),
        );
        assert!(!uses_generated_devices(&deck));
        let start = std::time::Instant::now();
        let trace = operating_point_trace(&deck, &[], &[]);
        let elapsed = start.elapsed();
        let message = trace[0].text.clone().unwrap_or_default();
        assert!(
            message != "ok" && !message.contains("native JIT error"),
            "the .va route refuses as a convergence failure: {trace:?}"
        );
        if !cfg!(debug_assertions) {
            assert!(
                elapsed < std::time::Duration::from_secs(30),
                "the .va route refuses in {elapsed:?}"
            );
        }
    }
}

/// ANGELOV_GAN ties its thermal node `t` to ground (`I(t) <+ V(t) *
/// SPICE_GMIN` without self-heating), and nothing else in the module reaches
/// a port single-ended. Left out, `t` is a net of the instance's own and the
/// tie reaches nothing of the deck's, so a deck that names no node 0 takes
/// its reference on both routes; connected, the tie keeps node 0
/// (`both_routes_keep_node_zero_for_a_module_that_reaches_a_port_single_ended`).
/// On node 0 the omitted `t` kept node 0 for a tie from ground to ground.
#[cfg(feature = "veriloga-model-angelov-gan")]
#[test]
fn both_routes_take_the_reference_when_only_an_omitted_port_reaches_ground() {
    let reference = |deck: &str| {
        let netlist = Netlist::parse_validated(deck).unwrap_or_else(|err| panic!("{err}\n{deck}"));
        Engine::default()
            .build_circuit(&netlist)
            .unwrap_or_else(|err| panic!("{err}\n{deck}"))
            .get_node_by_name("ref")
    };
    let body = "V1 d ref 5\nV2 g ref -1\nX1 d g ref angelov_gan\n.end\n";
    let generated = format!("* angelov_gan generated, t and rf left out\n{body}");
    let runtime = format!(
        "* angelov_gan .va, t and rf left out\n.va \"{}\" angelov_gan\n{body}",
        model_source("models/veriloga/angelov_2.0/vacode/angelov_gan.va")
    );
    assert!(uses_generated_devices(&generated));
    assert!(!uses_generated_devices(&runtime));
    assert_eq!(reference(&runtime), Some(0), "the .va route takes `ref`");
    assert_eq!(
        reference(&generated),
        Some(0),
        "the generated route takes `ref`"
    );
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
// R2_CMC on an R card — the card's value is the module's resistance `r`
//
// `R1 in 0 rmod 1k` with `.model rmod r2_cmc` used to run at R2_CMC's own
// default of 100 ohm without a word: the card's value — positional, `R=` or
// `VALUE=` — was parsed and then never handed to the module. The X card
// `X1 in 0 r2_cmc r=1k` names the parameter and always honoured it.
// ---------------------------------------------------------------------------

#[cfg(feature = "veriloga-model-r2-cmc")]
const R2_CMC_SOURCE: &str = "models/veriloga/cmc/r2_cmc_v1.0.2/r2_cmc.va";

/// One volt across the device and the device's own terminal current, so the
/// ratio is its resistance with no nodal GMIN in it.
#[cfg(feature = "veriloga-model-r2-cmc")]
fn measured_resistance(deck: &str, device: &str) -> f64 {
    let netlist = Netlist::parse_validated(deck).unwrap_or_else(|err| panic!("{err}\n{deck}"));
    let result = Engine::default()
        .run_dc_op(&netlist)
        .unwrap_or_else(|err| panic!("{err}\n{deck}"));
    let volts = node_voltage(&result, "in").expect("V(in)");
    let amperes = result
        .try_dc_observable_named(&format!("i({device})"))
        .unwrap_or_else(|| panic!("I({device}) is published\n{deck}"));
    volts / amperes
}

#[cfg(feature = "veriloga-model-r2-cmc")]
fn r_card_deck(card: &str) -> String {
    format!(
        "* R2_CMC on an R card\n.param rv=1k\nV1 in 0 1\n{card}\n.model rmod r2_cmc\n\
         .save v(in) i(r1)\n.end\n"
    )
}

#[cfg(feature = "veriloga-model-r2-cmc")]
fn assert_ohms(label: &str, measured: f64, expected: f64) {
    assert!(
        agrees(measured, expected),
        "{label}: {measured:.17e} ohm, expected {expected:.17e} (relative {:.3e})",
        (measured - expected).abs() / expected
    );
}

#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn an_r_cards_value_is_the_generated_resistors_resistance() {
    for card in [
        "R1 in 0 rmod 1k",
        "R1 in 0 rmod r=1k",
        "R1 in 0 rmod R=1k",
        "R1 in 0 rmod value=1k",
        "R1 in 0 1k rmod",
        "R1 in 0 rmod {rv}",
        "R1 in 0 rmod r={2*rv/2}",
    ] {
        let deck = r_card_deck(card);
        assert!(
            uses_generated_devices(&deck),
            "{card}: the card must route to the generated R2_CMC"
        );
        assert_ohms(card, measured_resistance(&deck, "r1"), 1.0e3);
    }

    // A card without a value leaves `r` unset, so the module keeps its own
    // default: no value is not a value of zero.
    assert_ohms(
        "R1 in 0 rmod",
        measured_resistance(&r_card_deck("R1 in 0 rmod"), "r1"),
        100.0,
    );
}

/// The R card, the generated X card and the X card on the `.va` runtime are
/// one resistor.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn an_r_card_and_both_x_card_routes_agree() {
    let r_card = measured_resistance(&r_card_deck("R1 in 0 rmod 1k"), "r1");
    let x_body = "V1 in 0 1\nX1 in 0 r2_cmc r=1k\n.save v(in) i(x1)";
    let generated_deck = format!("* R2_CMC X card generated\n{x_body}\n.end\n");
    let runtime_deck = format!(
        "* R2_CMC X card runtime\n.va \"{}\" r2_cmc\n{x_body}\n.end\n",
        model_source(R2_CMC_SOURCE)
    );
    assert!(uses_generated_devices(&generated_deck));
    assert!(!uses_generated_devices(&runtime_deck));
    let generated = measured_resistance(&generated_deck, "x1");
    let runtime = measured_resistance(&runtime_deck, "x1");
    assert_ohms("generated X card", generated, 1.0e3);
    assert_ohms("runtime X card", runtime, 1.0e3);
    assert_ohms("R card against the generated X card", r_card, generated);
    assert_ohms("R card against the runtime X card", r_card, runtime);
}

/// R2_CMC computes its resistance from `w` and `l` when both are given and
/// then ignores `r`, so a card that also states a value has said two things
/// about one quantity. Neither is chosen in silence.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn an_r_card_value_beside_w_and_l_is_refused() {
    // The geometry alone is rsh·l/w = 100·20u/1u = 2 kohm.
    let deck = r_card_deck("R1 in 0 rmod 1k w=1u l=20u");
    let error = Engine::default()
        .run_dc_op(&Netlist::parse_validated(&deck).expect("deck parses"))
        .expect_err("a value and a full geometry for one resistance must be refused");
    let rspice_core::SimulationError::Elaboration(refusal) = &error else {
        panic!("the conflict must be a typed elaboration refusal: {error}");
    };
    assert_eq!(
        refusal.kind,
        rspice_core::ElaborationErrorKind::ParameterValue,
        "{error}"
    );
    assert!(
        refusal
            .instance
            .as_deref()
            .is_some_and(|instance| instance.eq_ignore_ascii_case("r1")),
        "{error}"
    );
    assert!(
        refusal
            .module
            .as_deref()
            .is_some_and(|module| module.eq_ignore_ascii_case("r2_cmc")),
        "{error}"
    );
    let message = error.to_string();
    assert!(
        message.contains("'w'") && message.contains("'l'"),
        "the refusal must name the geometry it conflicts with: {message}"
    );

    // Either one alone leaves `r` in force, as R2_CMC documents.
    for card in ["R1 in 0 rmod 1k w=1u", "R1 in 0 rmod 1k l=10u"] {
        assert_ohms(card, measured_resistance(&r_card_deck(card), "r1"), 1.0e3);
    }
}

/// A resistance that moves with the solution is a behavioural resistor, and a
/// compact model has no input for one.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn an_r_card_value_that_reads_the_circuit_is_refused_on_the_generated_route() {
    let deck = r_card_deck("R1 in 0 rmod r={1k + v(in)}");
    let error = Engine::default()
        .run_dc_op(&Netlist::parse_validated(&deck).expect("deck parses"))
        .expect_err("a state-dependent value cannot be a module parameter");
    let rspice_core::SimulationError::Elaboration(refusal) = &error else {
        panic!("the refusal must be typed: {error}");
    };
    assert_eq!(
        refusal.kind,
        rspice_core::ElaborationErrorKind::ParameterValue,
        "{error}"
    );
    assert!(
        error.to_string().contains("circuit state"),
        "the refusal must say why: {error}"
    );
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
    use rspice_core::device::veriloga_builtins::{GeneratedStamper, GeneratedStaticStampCache};
    use rspice_core::solver::{ComplexMatrix, StaticMatrix};

    // The two sinks the Rust backend actually emits for a DC/transient stamp
    // are `stamp_current_sparse_local` and `stamp_potential_sparse_local`, and
    // both resolve their rows through the device's static stamp cache: with no
    // linked cache they return before writing anything, which is exactly how a
    // fixture can look green while stamping nothing. So the cache is linked
    // here the way the engine links it — two nodes and one branch unknown.
    let entries = (0..3)
        .flat_map(|row| (0..3).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    let voltages = [1.0, 2.0, 0.5];
    let structure = StaticMatrix::from_triplets(3, 3, &entries).expect("dense 3x3");
    let mut cache = GeneratedStaticStampCache::default();
    cache.link(&structure, &[1, 2], &[1], 2);

    // A DC/transient stamper writes both sinks, and the equivalent source it
    // builds is `value - dI/dV * V`: a non-finite derivative therefore poisons
    // the right-hand side as well, which is why the two flags are reported
    // together rather than as one bit.
    let cases = [
        ("finite", 1.0, 2.0, None),
        ("value", f64::NAN, 2.0, Some("right-hand-side contribution")),
        (
            "derivative",
            1.0,
            f64::INFINITY,
            Some("Jacobian entry and right-hand-side contribution"),
        ),
    ];

    for (label, value, derivative, expected) in cases {
        let mut matrix = StaticMatrix::from_triplets(3, 3, &entries).expect("dense 3x3");
        let mut rhs = [0.0; 3];
        let mut stamper =
            GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 2, &cache);
        // Two contributions, the second always finite: an early exit would
        // lose the first observation or skip the second stamp.
        stamper.stamp_current_sparse_local::<1, 0>(
            Some(0),
            Some(1),
            value,
            [0],
            [derivative],
            [],
            [],
            1.0,
        );
        stamper.stamp_current_sparse_local::<1, 0>(Some(0), Some(1), 3.0, [1], [4.0], [], [], 1.0);
        let reported = stamper.non_finite_contribution();
        assert_eq!(
            reported, expected,
            "{label}: the current sink must report which sink saw a non-finite value"
        );
        assert!(
            rhs.iter().any(|value| *value != 0.0),
            "{label}: the current sink must have written the right-hand side"
        );
    }

    for (label, value, derivative, expected) in cases {
        let mut matrix = StaticMatrix::from_triplets(3, 3, &entries).expect("dense 3x3");
        let mut rhs = [0.0; 3];
        let mut stamper =
            GeneratedStamper::new_with_static_cache(&mut matrix, &mut rhs, &voltages, 2, &cache);
        stamper.stamp_potential_sparse_local::<1, 0>(0, value, [0], [derivative], [], []);
        stamper.stamp_potential_sparse_local::<1, 0>(0, 3.0, [1], [4.0], [], []);
        let reported = stamper.non_finite_contribution();
        assert_eq!(
            reported, expected,
            "{label}: the potential sink must report which sink saw a non-finite value"
        );
        assert!(
            rhs.iter().any(|value| *value != 0.0),
            "{label}: the potential sink must have written the right-hand side"
        );
    }

    // The AC-real stamper has no right-hand side, so it isolates the Jacobian.
    let mut matrix = ComplexMatrix::from_real_structure(&structure);
    let mut stamper =
        GeneratedStamper::new_ac_real_with_static_cache(&mut matrix, &voltages, 2, &cache);
    stamper.stamp_current_sparse_local::<1, 0>(
        Some(0),
        Some(1),
        1.0,
        [0],
        [f64::INFINITY],
        [],
        [],
        1.0,
    );
    stamper.stamp_current_sparse_local::<1, 0>(Some(0), Some(1), 3.0, [1], [4.0], [], [], 1.0);
    assert_eq!(stamper.non_finite_contribution(), Some("Jacobian entry"));
}

/// A generated device whose linearization at the offered iterate is not finite
/// is a rejectable trial naming the instance, not a silent infinity in the
/// matrix.
///
/// The iterate here is what a singular or badly conditioned linear solve hands
/// back — `±inf` on a node — and it is the case the model's own defences do
/// *not* catch: DIODE_CMC's limited exponential and the Newton limiter between
/// them keep the evaluation itself finite, but the equivalent source every
/// contribution carries is `value - dI/dV * V` and the stamper computes that
/// against the raw iterate. So the model is happy, the linearization is
/// garbage, and before this audit nothing noticed: the assembly returned
/// `Ok(())`, having written infinities into the matrix and the right-hand
/// side, and the transient read the failed solve that followed as a reason to
/// cut dt while DC walked its whole ladder with no diagnostic at all.
///
/// The iterate is injected on each of the device's OWN nodes in turn, by name.
/// A hand-picked solution index proves nothing: R1.15's first attempt at this
/// fixture wrote `+inf` at `get_or_create_node("a") - 1`, which — after the
/// build has already canonicalized that node to `A` — *appends a fresh node*
/// and writes past every node the device owns, so the device stamped an
/// all-zero point and returned `Ok(())` for a reason that had nothing to do
/// with the audit.
#[test]
fn a_generated_device_refuses_a_non_finite_contribution_as_a_rejectable_trial() {
    use rspice_core::device::StampError;
    use rspice_core::solver::StaticMatrix;

    let deck = "* generated finiteness\nV1 in 0 0.8\nR1 in a 1k\nX1 a 0 DIODE_CMC\n.end\n";
    assert!(
        uses_generated_devices(deck),
        "the plain X card must resolve to the generated built-in"
    );
    let netlist = Netlist::parse_validated(deck).expect("deck parses");
    let mut circuit = Engine::default()
        .build_circuit(&netlist)
        .expect("deck builds");

    let size = circuit.matrix_size();
    let entries = (0..size)
        .flat_map(|row| (0..size).map(move |column| (row, column, 0.0)))
        .collect::<Vec<_>>();
    // Index i of this list is node i + 1, i.e. solution index i.
    let names = circuit.node_names_sorted();
    let device_nodes = names
        .iter()
        .enumerate()
        .filter(|(_, name)| name.eq_ignore_ascii_case("a") || name.starts_with("X1."))
        .map(|(index, name)| (index, name.clone()))
        .collect::<Vec<_>>();
    assert!(
        device_nodes.len() > 1,
        "the generated device must own its terminal and its internal nodes: {names:?}"
    );

    let mut audited = Vec::new();
    for (index, name) in &device_nodes {
        let mut matrix =
            StaticMatrix::from_triplets(size, size, &entries).expect("dense structure");
        let mut rhs = vec![0.0; size];
        let mut voltages = vec![0.0; size];
        voltages[*index] = f64::INFINITY;

        // Generated devices are assembled by `stamp_behavioral`, not by
        // `try_stamp_nonlinear`: `stamp_behavioral_with_generated_mode` is
        // where `stamp_all_with_mode` and `generated_veriloga_stamp_error`
        // live.
        let refusal = circuit.stamp_behavioral(
            &mut matrix,
            &mut rhs,
            &voltages,
            0.0,
            rspice_core::xspice::AnalysisType::DcOp,
        );
        let Err(error) = refusal else {
            panic!("a non-finite iterate on {name} must refuse the point");
        };
        let StampError::NonFiniteTrial(trial) = &error else {
            panic!("a non-finite iterate on {name} must be a rejectable trial: {error}");
        };
        assert!(
            trial.instance.eq_ignore_ascii_case("x1"),
            "the refusal must name the instance, got {:?}",
            trial.instance
        );
        assert!(
            trial.detail.contains("generated Verilog-A") && trial.detail.contains("finite"),
            "the refusal must come from the generated route and say what went wrong: {}",
            trial.detail
        );
        // The assembly is completed rather than abandoned half-written — the
        // caller is about to clear this matrix and offer another point, and an
        // early exit would make the diagnostic depend on emission order — so
        // the infinities are still here. What has changed is that the solver
        // is told.
        assert!(
            rhs.iter().any(|value| !value.is_finite()),
            "the audit must observe rather than suppress the contribution on {name}: {rhs:?}"
        );
        if trial.detail.contains("stamped a non-finite") {
            audited.push((name.clone(), trial.detail.clone()));
        }
    }

    // A typed refusal the module raised for itself wins where it exists (the
    // anode feeds a `ddt`, which names its own operator). The stamper's audit
    // is what answers everywhere else, and it must actually be reachable
    // through a shipped model's own assembly — that is the whole claim, and
    // it is not provable by driving a sink the backend never emits.
    assert!(
        !audited.is_empty(),
        "the stamper's own audit must refuse at least one of {device_nodes:?}"
    );
}
