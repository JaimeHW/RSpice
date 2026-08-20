//! `CircuitData::device_count()` answers "how many devices are in this
//! circuit", so it has to stay a whole inventory. A counter under that name
//! that quietly omits a device family is worse than no counter at all: every
//! caller reads the number as complete.
//!
//! The ratchet below reads the `CircuitData` declaration and the body of
//! `device_count()` back out of their own source and refuses any device
//! collection that neither appears in the sum nor carries a written reason
//! for staying out of it. The tests that follow it pin the resulting count
//! for the device families that were missing from the sum.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const CIRCUIT_SOURCE: &str = include_str!("../src/circuit.rs");
const INTROSPECTION_SOURCE: &str = include_str!("../src/circuit/introspection.rs");

/// The `CircuitData` collections `device_count()` deliberately leaves out,
/// each with the reason it holds no device of its own.
///
/// Two rules put a collection here. Either it re-describes elements another
/// collection already counts -- the realization terms of one authored card, a
/// pending-resolution work list, a frozen load order, per-step scratch -- or
/// it carries node, topology, or solver facts with no circuit element behind
/// them. Nothing else belongs: a genuinely new device family belongs in the
/// sum, not in this list.
const EXCLUDED_FROM_DEVICE_COUNT: &[(&str, &str)] = &[
    // Second views of elements another collection already counts.
    (
        "coupled_inductor_pairs",
        "the pairwise mutual terms of the K cards `couplings` counts",
    ),
    (
        "multi_winding_transformers",
        "an N-winding mutual realization of the same one authored K card",
    ),
    (
        "pending_cccs",
        "control-branch work list over already-stored CCCS instances",
    ),
    (
        "pending_ccvs",
        "control-branch work list over already-stored CCVS instances",
    ),
    (
        "pending_iswitch",
        "control-branch work list over already-stored ISWITCH instances",
    ),
    (
        "xyce_topology_order",
        "Xyce's load schedule, indexing devices their own collections count",
    ),
    (
        "xyce_load_plan",
        "the frozen master/instance load order over those same devices",
    ),
    (
        "xyce_linear_f_operator",
        "a cached `F*x` operator over the resistors already counted",
    ),
    (
        "xyce_core_transient_residuals",
        "branch residuals retained from the current Core stamp",
    ),
    (
        "xspice_digital_drivers",
        "per-output digital drive state of the counted XSPICE instances",
    ),
    (
        "xspice_real_drivers",
        "per-output real-valued drive state of those same instances",
    ),
    (
        "xspice_event_queue",
        "pending XSPICE events, not the devices that raise them",
    ),
    // Node, topology, and solver facts with no element behind them.
    (
        "xspice_event_nodes",
        "the circuit nodes XSPICE event connections reach",
    ),
    (
        "xspice_touched_digital_nodes",
        "scratch nodes touched while applying one digital event batch",
    ),
    (
        "xspice_touched_real_nodes",
        "scratch nodes touched while applying one real-valued event batch",
    ),
    (
        "branch_name_by_ordinal",
        "canonical branch names, one per MNA branch unknown",
    ),
    (
        "no_dc_path_nodes",
        "nodes construction found without a DC path to ground",
    ),
    (
        "fatal_no_dc_path_nodes",
        "the subset of those nodes that must be rejected outright",
    ),
    (
        "dc_floating_component_by_node",
        "the floating DC component index of each node",
    ),
    (
        "dc_floating_component_nodes",
        "the original node spellings of each floating component",
    ),
    (
        "dc_floating_component_is_certain",
        "whether each floating component can support a topology proof",
    ),
    (
        "b3soi_gmin_scale",
        "the BSIMSOI3 terminal-GMIN multiplier, a solver scalar",
    ),
    (
        "global_shunt_conductance",
        "the `.OPTIONS RSHUNT` conductance applied at every node",
    ),
    (
        "generated_simulation_parameters",
        "the solver-controlled `$simparam` environment",
    ),
];

/// Every `CircuitData` field, as `(name, type)`.
///
/// The scan is line-based on purpose. `core.autocrlf` decides this checkout's
/// newlines, so a boundary spelled as a `"\n"` literal matches under one
/// checkout and hands back nothing under the other -- and a ratchet that
/// scans nothing passes forever. Each step below panics when it finds no
/// structure to scan, so losing the boundary is a red test rather than a
/// silent one.
fn circuit_data_fields() -> Vec<(String, String)> {
    let mut lines = CIRCUIT_SOURCE.lines();
    lines
        .by_ref()
        .find(|line| line.trim() == "pub struct CircuitData {")
        .expect("circuit.rs opens `pub struct CircuitData {` on a line of its own");

    let mut fields: Vec<(String, String)> = Vec::new();
    let mut pending = String::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed == "}" {
            assert!(
                pending.is_empty(),
                "the CircuitData declaration ends mid-field on: {pending}"
            );
            assert!(
                !fields.is_empty(),
                "the CircuitData declaration was scanned and yielded no fields at all"
            );
            return fields;
        }
        if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("#[") {
            continue;
        }
        if !pending.is_empty() {
            pending.push(' ');
        }
        pending.push_str(trimmed);
        if !pending.ends_with(',') {
            // A field whose type wrapped onto the following line.
            continue;
        }
        let declaration = pending.trim_end_matches(',');
        let (binding, field_type) = declaration
            .split_once(':')
            .unwrap_or_else(|| panic!("unparsable CircuitData field declaration: {declaration}"));
        // Drop any `pub` / `pub(crate)` / `pub(in ...)` visibility prefix.
        let name = binding
            .rsplit(' ')
            .next()
            .expect("a split always yields a first piece");
        assert!(
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "field scan produced `{name}`, which is not an identifier; the CircuitData \
             declaration uses a shape this parser does not understand"
        );
        fields.push((name.to_string(), field_type.trim().to_string()));
        pending.clear();
    }
    panic!("the CircuitData declaration is never closed by a line holding only `}}`")
}

/// Whether a field's type is device storage: a `Vec` of instances, or one of
/// the struct-of-arrays collections holding a whole device family.
///
/// A rule rather than a roster, so a family added tomorrow is caught too.
/// Device storage is a `Vec<...>` or a plain named type; anything reached
/// through some other generic -- `Option`, `HashMap`, `HashSet`, `Arc` -- is
/// wrapping something that is not a device family, and a lowercase type is a
/// primitive.
fn is_device_collection(field_type: &str) -> bool {
    if field_type.starts_with("Vec<") {
        return true;
    }
    if field_type.contains('<') || field_type.contains('(') {
        return false;
    }
    field_type
        .rsplit("::")
        .next()
        .and_then(|segment| segment.chars().next())
        .is_some_and(|first| first.is_ascii_uppercase())
}

/// The code `device_count()` runs, with its comments and all whitespace
/// stripped out.
///
/// The end of the body is found by brace depth rather than by matching a
/// closing line, so regrouping or reindenting the sum keeps it scanned.
/// Dropping whitespace is what lets the field search survive a `self` and its
/// field landing on different lines, which is how a long term wraps.
fn device_count_code() -> String {
    let mut lines = INTROSPECTION_SOURCE
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//"));
    let signature = lines
        .by_ref()
        .find(|line| line.starts_with("pub fn device_count(&self)"))
        .expect("introspection.rs declares `pub fn device_count(&self)`");
    let mut depth = brace_delta(signature);
    assert!(
        depth > 0,
        "the device_count signature does not open its own body"
    );

    let mut code = String::new();
    for line in lines {
        depth += brace_delta(line);
        if depth <= 0 {
            assert!(!code.is_empty(), "device_count was scanned as an empty body");
            return code;
        }
        code.extend(line.chars().filter(|c| !c.is_whitespace()));
    }
    panic!("the device_count body is never closed")
}

fn brace_delta(line: &str) -> i32 {
    line.chars()
        .map(|c| match c {
            '{' => 1,
            '}' => -1,
            _ => 0,
        })
        .sum()
}

/// Whether the counter reads the named field, matched at a word boundary so
/// that a field whose name merely prefixes another one is not credited to it.
fn references_field(code: &str, name: &str) -> bool {
    let needle = format!("self.{name}");
    code.match_indices(&needle).any(|(index, _)| {
        code[index + needle.len()..]
            .chars()
            .next()
            .is_none_or(|next| !next.is_alphanumeric() && next != '_')
    })
}

fn build(deck: &str) -> rspice_core::circuit::CircuitData {
    let netlist = Netlist::parse(deck).expect("fixture deck parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("fixture deck builds")
}

#[test]
fn every_circuit_data_device_collection_is_counted_or_excused() {
    let code = device_count_code();
    let fields = circuit_data_fields();
    let collections = fields
        .iter()
        .filter(|(_, field_type)| is_device_collection(field_type))
        .collect::<Vec<_>>();
    assert!(
        collections.len() > 20,
        "only {} CircuitData device collections were recognized; the field or type scan has \
         stopped seeing the declaration it is supposed to guard",
        collections.len()
    );

    for (name, field_type) in &collections {
        let counted = references_field(&code, name);
        let excused = EXCLUDED_FROM_DEVICE_COUNT
            .iter()
            .find(|(field, _)| field == name);
        match (counted, excused) {
            (true, None) | (false, Some(_)) => {}
            (true, Some((_, reason))) => panic!(
                "`CircuitData::{name}` is summed by `device_count()` and also excused from it \
                 as \"{reason}\". Remove one: a collection is counted or it is not."
            ),
            (false, None) => panic!(
                "`CircuitData::{name}: {field_type}` is a device collection that \
                 `device_count()` neither sums nor excuses.\n\
                 `device_count()` is the whole inventory of devices in a circuit, so do one \
                 of two things:\n  \
                 - add `self.{name}` to the sum in \
                 crates/rspice-core/src/circuit/introspection.rs, if it holds authored device \
                 instances; or\n  \
                 - add (\"{name}\", \"<why it is not a device of its own>\") to \
                 EXCLUDED_FROM_DEVICE_COUNT in this file, if it only re-describes elements \
                 another collection already counts, or holds no device at all."
            ),
        }
    }

    for (name, reason) in EXCLUDED_FROM_DEVICE_COUNT {
        assert!(
            collections.iter().any(|(field, _)| field == name),
            "EXCLUDED_FROM_DEVICE_COUNT excuses `{name}` (\"{reason}\"), which is no longer a \
             CircuitData device collection. Delete the entry."
        );
    }
}

/// The defect this inventory was written for: BSIM-SOI routed through
/// `b3soi_fd`, reported by `device_op_report()` and invisible to the counter.
#[test]
fn bsimsoi_level55_mosfet_is_counted() {
    let circuit = build(
        r#"* BSIM-SOI LEVEL=55 device count
V1 d 0 1
M1 d 0 0 0 nbsimsoi W=1u L=1u
.model nbsimsoi nmos level=55
.op
.end
"#,
    );
    assert_eq!(
        circuit.device_count(),
        2,
        "one source and one LEVEL=55 MOSFET"
    );
    assert!(
        circuit
            .device_op_report()
            .entries
            .iter()
            .any(|entry| entry.name.eq_ignore_ascii_case("M1") && entry.device_kind == "B3SOIFD"),
        "the LEVEL=55 fixture must still route to the native B3SOIFD family"
    );
}

#[test]
fn compact_mosfet_families_are_counted() {
    for (level, kind) in [(8, "BSIM3"), (14, "BSIM4"), (10, "B3SOIPD"), (57, "B3SOIPD")] {
        let circuit = build(&format!(
            r#"* compact MOSFET device count
V1 d 0 1
M1 d 0 0 0 nmod W=1u L=1u
.model nmod nmos level={level}
.op
.end
"#
        ));
        assert_eq!(
            circuit.device_count(),
            2,
            "one source and one LEVEL={level} MOSFET"
        );
        assert!(
            circuit
                .device_op_report()
                .entries
                .iter()
                .any(|entry| entry.name.eq_ignore_ascii_case("M1") && entry.device_kind == kind),
            "LEVEL={level} must still route to {kind}"
        );
    }
}

#[test]
fn switches_lines_and_behavioral_sources_are_counted() {
    let switched = build(
        r#"* voltage-controlled switch device count
V1 a 0 1
Vc c 0 1
S1 a b c 0 SMOD
R1 b 0 1k
.model SMOD SW vt=0.5
.op
.end
"#,
    );
    assert_eq!(switched.device_count(), 4, "two sources, one switch, one R");

    let line = build(
        r#"* transmission line device count
V1 a 0 1
T1 a 0 b 0 Z0=50 TD=1n
R1 b 0 50
.op
.end
"#,
    );
    assert_eq!(line.device_count(), 3, "one source, one T line, one R");

    let behavioral = build(
        r#"* behavioral source device count
V1 a 0 1
B1 b 0 V=V(a)*2
R1 b 0 1k
.op
.end
"#,
    );
    assert_eq!(
        behavioral.device_count(),
        3,
        "one source, one B element, one R"
    );
}

/// One K card is one device however many mutual pairs realize it, and a
/// magnetic-core model on an L card does not make that inductor two devices.
#[test]
fn a_magnetic_card_is_counted_once_however_it_is_realized() {
    let coupled = build(
        r#"* three-way coupling device count
V1 a 0 1
L1 a 0 1m
L2 b 0 1m
L3 c 0 1m
R2 b 0 1k
R3 c 0 1k
K1 L1 L2 L3 0.5
.op
.end
"#,
    );
    assert_eq!(
        coupled.device_count(),
        7,
        "one source, three inductors, two resistors and one K card -- not its three pairs"
    );

    let hysteretic = build(
        r#"* Jiles-Atherton inductor device count
V1 a 0 1
L1 a 0 1m JAMOD
.model JAMOD CORE ms=1e6 a=100 k=100 c=0.1 alpha=1e-6 area=1e-4 len=0.1 n=10
.op
.end
"#,
    );
    assert_eq!(
        hysteretic.device_count(),
        2,
        "one source and one inductor, whose core model is an overlay and not a second device"
    );

    let single_winding_core = build(
        r#"single winding nonlinear CORE
VS 1 0 1
R1 1 2 1K
L1 2 0 200
K1 L1 1 nlcore
.model nlcore core gap=.1 path=1 area=1e-4
.op
.end
"#,
    );
    assert_eq!(
        single_winding_core.device_count(),
        4,
        "one source, one resistor, one winding, and the K card that gives it a core"
    );

    let multi_winding_core = build(
        r#"generic nonlinear CORE family
VS 1 0 1
R1 1 2 1K
R2 3 0 1K
L1 2 0 200
L2 3 0 100
K1 L1 L2 1 nlcore
.model nlcore core gap=.1 path=1 area=1e-4
.op
.end
"#,
    );
    assert_eq!(
        multi_winding_core.device_count(),
        6,
        "one source, two resistors, two windings, and one shared core -- not one core per winding"
    );
}
