//! Example Circuits
//!
//! Cadence-style example circuits for demonstration and learning.
//! Each example provides a complete, ready-to-simulate circuit.
//!
//! # Available Examples
//!
//! - **RC Lowpass Filter**: Simple 1st-order filter
//! - **Voltage Divider**: Basic resistive divider
//! - **Common Emitter Amplifier**: BJT amplifier stage
//! - **CMOS Inverter**: Basic digital gate
//! - **Differential Pair**: Op-amp input stage
//! - **Opamp Inverting Amplifier**: VCVS-based inverting stage
//! - **Hierarchical RC Filter**: Testbench instantiating an authored cell symbol
//!
//! # Geometry rules
//!
//! Every wire endpoint in these builders lands **exactly** on a component
//! terminal (see `ComponentType::terminal_offsets` + `Component::
//! transform_point`), or tees into another wire's interior. The relevant
//! terminal facts, with `pos = (x, y)`:
//!
//! - R / L / C at `R0` are horizontal: `(x−20, y)`, `(x+20, y)`;
//!   at `R90` vertical: top `(x, y−20)`, bottom `(x, y+20)`.
//! - V/I sources are **vertical at `R0`**: `+ (x, y−20)`, `− (x, y+20)`.
//!   (Do not rotate them "to make them vertical" — R90 turns them sideways.)
//! - Ground connects at its stem, `(x, y−10)` — 10 units *above* `pos`.
//! - NPN: C `(x+20, y−40)`, B `(x−20, y)`, E `(x+20, y+40)`;
//!   `mirror_h` flips C/E/B to the left/right respectively.
//! - NMOS/PMOS: D `(x+20, y−40)`, G `(x−20, y)`, S `(x+20, y+40)`,
//!   bulk `(x+20, y)`; `mirror_v` swaps D and S (used for PMOS pull-ups).
//! - VCVS: O± on the left at `(x−20, y∓10)`, C± on the right at
//!   `(x+20, y∓10)`; netlists as `E out+ out− in+ in−`. `mirror_h` puts the
//!   output on the right.

use crate::state::workspace::{DEFAULT_PROJECT_LIBRARY, DEFAULT_SCHEMATIC_VIEW};
use crate::state::{
    Cell, CellViewRef, Component, ComponentType, Library, LibraryCellInstance, OpenCellView, Point,
    PortDirection, PortSpec, Rotation, SchematicState, SymbolDocument, SymbolPin, SymbolShape,
    View, ViewType, Wire,
};

const HIERARCHICAL_RC_FILTER: &str = "Hierarchical RC Filter";
const HIERARCHICAL_RC_TOP_CELL: &str = "hierarchical_rc_filter_tb";
const HIERARCHICAL_RC_CORE_CELL: &str = "rc_filter_core";

// =============================================================================
// Example Circuit Definitions
// =============================================================================

/// Example circuit metadata
#[derive(Debug, Clone)]
pub struct ExampleCircuit {
    /// Display name
    pub name: &'static str,
    /// Description
    pub description: &'static str,
    /// Category (Analog, Digital, RF, etc.)
    pub category: &'static str,
    /// Whether this example needs the full project workspace to load.
    pub requires_workspace: bool,
}

/// All available example circuits
pub const EXAMPLES: &[ExampleCircuit] = &[
    ExampleCircuit {
        name: "RC Lowpass Filter",
        description: "Simple 1st-order RC lowpass with 1kHz cutoff",
        category: "Analog",
        requires_workspace: false,
    },
    ExampleCircuit {
        name: "Voltage Divider",
        description: "Two-resistor voltage divider (2:1 ratio)",
        category: "Basics",
        requires_workspace: false,
    },
    ExampleCircuit {
        name: "Common Emitter Amplifier",
        description: "Single-stage BJT amplifier with biasing",
        category: "Analog",
        requires_workspace: false,
    },
    ExampleCircuit {
        name: "CMOS Inverter",
        description: "Basic CMOS NOT gate",
        category: "Digital",
        requires_workspace: false,
    },
    ExampleCircuit {
        name: "Differential Pair",
        description: "BJT differential amplifier input stage",
        category: "Analog",
        requires_workspace: false,
    },
    ExampleCircuit {
        name: "Opamp Inverting Amplifier",
        description: "Inverting amplifier with gain = -10",
        category: "Analog",
        requires_workspace: false,
    },
    ExampleCircuit {
        name: HIERARCHICAL_RC_FILTER,
        description: "Top-level testbench using an authored RC filter cell symbol",
        category: "Hierarchy",
        requires_workspace: true,
    },
];

// =============================================================================
// Circuit Builders
// =============================================================================

/// Load an example circuit by name
pub fn load_example(name: &str, state: &mut SchematicState) {
    // Clear existing circuit
    state.components.clear();
    state.wires.clear();
    state.junctions.clear();
    state.net_labels.clear();
    state.selection.clear();

    match name {
        "RC Lowpass Filter" => build_rc_lowpass(state),
        "Voltage Divider" => build_voltage_divider(state),
        "Common Emitter Amplifier" => build_common_emitter(state),
        "CMOS Inverter" => build_cmos_inverter(state),
        "Differential Pair" => build_differential_pair(state),
        "Opamp Inverting Amplifier" => build_opamp_inverter(state),
        _ => return,
    }

    // Junction dots where wires tee, frame the circuit, fresh history.
    state.update_wire_junctions();
    state.needs_fit = true;
    state.needs_history_reset = true;
    state.is_dirty = true;
}

/// Load an example into the full application workspace. Flat examples retain
/// the legacy schematic-only behavior; hierarchy examples also populate the
/// project library, open views, and schematic buffers that their cell
/// instances resolve through.
pub(crate) fn load_example_into_app(name: &str, app: &mut crate::common::app::AppState) -> bool {
    if name == HIERARCHICAL_RC_FILTER {
        build_hierarchical_rc_project(app);
        return true;
    }

    if !EXAMPLES.iter().any(|example| example.name == name) {
        return false;
    }

    load_example(name, &mut app.schematic);
    app.sync_active_schematic_to_workspace();
    true
}

/// RC Lowpass Filter
/// ```text
///         ┌──/\R1\──┬─o Vout
///         │  1k     │
///       (VIN)      C1 159n
///         │         │
///         └────┬────┘
///             GND
/// ```
fn build_rc_lowpass(state: &mut SchematicState) {
    let mut id = 1u64;

    // Input source — vertical at R0: + (100,180), − (100,220).
    let v1 = Component::new(id, ComponentType::VoltageSourceAc, Point::new(100, 200))
        .with_name_value("VIN", "1");
    id += 1;
    state.components.push(v1);

    // Series resistor — horizontal: (180,100), (220,100).
    let r1 = Component::new(id, ComponentType::Resistor, Point::new(200, 100))
        .with_name_value("R1", "1k");
    id += 1;
    state.components.push(r1);

    // Shunt capacitor — vertical: top (280,180), bottom (280,220).
    let c1 = Component::new(id, ComponentType::Capacitor, Point::new(280, 200))
        .with_rotation(Rotation::R90)
        .with_name_value("C1", "159n");
    id += 1;
    state.components.push(c1);

    // Ground — stem at (190,260), teeing into the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(190, 270));
    state.components.push(gnd);

    // VIN+ up and over to R1 left.
    add_wire(
        state,
        vec![
            Point::new(100, 180),
            Point::new(100, 100),
            Point::new(180, 100),
        ],
    );
    // R1 right to C1 top (the output node).
    add_wire(
        state,
        vec![
            Point::new(220, 100),
            Point::new(280, 100),
            Point::new(280, 180),
        ],
    );
    // C1 bottom to the ground rail.
    add_wire(state, vec![Point::new(280, 220), Point::new(280, 260)]);
    // VIN− to the ground rail (the ground stem tees in at x=190).
    add_wire(
        state,
        vec![
            Point::new(100, 220),
            Point::new(100, 260),
            Point::new(280, 260),
        ],
    );
    add_label(state, Point::new(250, 100), "out");
}

/// Voltage Divider
/// ```text
///   ┌──────────┐
/// (VCC)       R1 10k
///   │          ├─o Vout
///   │         R2 10k
///   └──────────┘
///        GND
/// ```
fn build_voltage_divider(state: &mut SchematicState) {
    let mut id = 1u64;

    // Supply — vertical at R0: + (100,140), − (100,180).
    let v1 = Component::new(id, ComponentType::VoltageSource, Point::new(100, 160))
        .with_name_value("VCC", "5");
    id += 1;
    state.components.push(v1);

    // Divider string — vertical resistors: R1 (200,80)/(200,120),
    // R2 (200,160)/(200,200).
    let r1 = Component::new(id, ComponentType::Resistor, Point::new(200, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("R1", "10k");
    id += 1;
    state.components.push(r1);

    let r2 = Component::new(id, ComponentType::Resistor, Point::new(200, 180))
        .with_rotation(Rotation::R90)
        .with_name_value("R2", "10k");
    id += 1;
    state.components.push(r2);

    // Ground — stem at (150,240) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(150, 250));
    state.components.push(gnd);

    // VCC+ up and over to R1 top.
    add_wire(
        state,
        vec![
            Point::new(100, 140),
            Point::new(100, 60),
            Point::new(200, 60),
            Point::new(200, 80),
        ],
    );
    // R1 bottom to R2 top (Vout).
    add_wire(state, vec![Point::new(200, 120), Point::new(200, 160)]);
    // R2 bottom to the rail.
    add_wire(state, vec![Point::new(200, 200), Point::new(200, 240)]);
    // VCC− along the rail.
    add_wire(
        state,
        vec![
            Point::new(100, 180),
            Point::new(100, 240),
            Point::new(200, 240),
        ],
    );
    add_label(state, Point::new(200, 140), "out");
}

/// Common Emitter Amplifier
/// ```text
///        ┌────────┬───────────(VCC)
///       RB       RC
///        │        ├──o Vout
///  VIN─CIN──┬───B Q1
///           │     E
///        (VIN)   RE
///           └─────┴──────GND
/// ```
fn build_common_emitter(state: &mut SchematicState) {
    let mut id = 1u64;

    // Q1 — C (280,140), B (240,180), E (280,220).
    let q1 = Component::new(id, ComponentType::NpnBjt, Point::new(260, 180))
        .with_name_value("Q1", "2N2222");
    id += 1;
    state.components.push(q1);

    // Base bias from the supply rail — vertical: (180,100)/(180,140).
    let rb = Component::new(id, ComponentType::Resistor, Point::new(180, 120))
        .with_rotation(Rotation::R90)
        .with_name_value("RB", "100k");
    id += 1;
    state.components.push(rb);

    // Collector load — vertical: (280,80)/(280,120).
    let rc = Component::new(id, ComponentType::Resistor, Point::new(280, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("RC", "1k");
    id += 1;
    state.components.push(rc);

    // Emitter degeneration — vertical: (280,240)/(280,280).
    let re = Component::new(id, ComponentType::Resistor, Point::new(280, 260))
        .with_rotation(Rotation::R90)
        .with_name_value("RE", "100");
    id += 1;
    state.components.push(re);

    // Input coupling — horizontal: (100,180)/(140,180).
    let cin = Component::new(id, ComponentType::Capacitor, Point::new(120, 180))
        .with_name_value("CIN", "1u");
    id += 1;
    state.components.push(cin);

    // AC input — vertical at R0: + (60,220), − (60,260).
    let vin = Component::new(id, ComponentType::VoltageSourceAc, Point::new(60, 240))
        .with_name_value("VIN", "10m");
    id += 1;
    state.components.push(vin);

    // Supply — vertical at R0: + (380,100), − (380,140).
    let vcc = Component::new(id, ComponentType::VoltageSource, Point::new(380, 120))
        .with_name_value("VCC", "12");
    id += 1;
    state.components.push(vcc);

    // Ground — stem at (220,300) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(220, 310));
    state.components.push(gnd);

    // Supply rail and drops.
    add_wire(state, vec![Point::new(180, 60), Point::new(380, 60)]);
    add_wire(state, vec![Point::new(180, 100), Point::new(180, 60)]);
    add_wire(state, vec![Point::new(280, 80), Point::new(280, 60)]);
    add_wire(state, vec![Point::new(380, 100), Point::new(380, 60)]);
    // RB bottom into the base node (tees the CIN→base wire at x=180).
    add_wire(state, vec![Point::new(180, 140), Point::new(180, 180)]);
    // CIN right to the base.
    add_wire(state, vec![Point::new(140, 180), Point::new(240, 180)]);
    // VIN+ to CIN left.
    add_wire(
        state,
        vec![
            Point::new(60, 220),
            Point::new(60, 180),
            Point::new(100, 180),
        ],
    );
    // RC bottom to the collector.
    add_wire(state, vec![Point::new(280, 120), Point::new(280, 140)]);
    // Emitter to RE top, RE bottom to the rail.
    add_wire(state, vec![Point::new(280, 220), Point::new(280, 240)]);
    add_wire(state, vec![Point::new(280, 280), Point::new(280, 300)]);
    // Ground rail with the source returns.
    add_wire(state, vec![Point::new(60, 300), Point::new(380, 300)]);
    add_wire(state, vec![Point::new(60, 260), Point::new(60, 300)]);
    add_wire(state, vec![Point::new(380, 140), Point::new(380, 300)]);
    add_label(state, Point::new(280, 130), "out");
}

/// CMOS Inverter
/// ```text
///        VDD rail ──────(VDD)
///         S│
///      ┌─G[MP]   (PMOS, mirror_v: source on top)
///  in ─┤  D│
///      │   ├──o out
///      │  D│
///      └─G[MN]
///         S│
///        GND rail
/// ```
fn build_cmos_inverter(state: &mut SchematicState) {
    let mut id = 1u64;

    // Pull-up PMOS — mirror_v puts S on top: S (260,80), G (220,120),
    // D (260,160), bulk (260,120).
    let mp = Component::new(id, ComponentType::Pmos, Point::new(240, 120))
        .with_mirror_v(true)
        .with_name_value("MP", "PMOS W=2u L=0.18u");
    id += 1;
    state.components.push(mp);

    // Pull-down NMOS — D (260,200), G (220,240), S (260,280), bulk (260,240).
    let mn = Component::new(id, ComponentType::Nmos, Point::new(240, 240))
        .with_name_value("MN", "NMOS W=1u L=0.18u");
    id += 1;
    state.components.push(mn);

    // Supply — vertical at R0: + (360,60), − (360,100).
    let vdd = Component::new(id, ComponentType::VoltageSource, Point::new(360, 80))
        .with_name_value("VDD", "1.8");
    id += 1;
    state.components.push(vdd);

    // Input pulse — vertical at R0: + (120,220), − (120,260).
    let vin = Component::new(id, ComponentType::VoltageSourcePulse, Point::new(120, 240))
        .with_name_value("VIN", "PULSE(0 1.8 0 1n 1n 5n 10n)");
    id += 1;
    state.components.push(vin);

    // Ground — stem at (240,320) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(240, 330));
    state.components.push(gnd);

    // Input to both gates (tee at (160,180)).
    add_wire(
        state,
        vec![
            Point::new(120, 220),
            Point::new(120, 180),
            Point::new(160, 180),
        ],
    );
    add_wire(
        state,
        vec![
            Point::new(160, 180),
            Point::new(160, 120),
            Point::new(220, 120),
        ],
    );
    add_wire(
        state,
        vec![
            Point::new(160, 180),
            Point::new(160, 240),
            Point::new(220, 240),
        ],
    );
    // Output node: MP drain to MN drain, with a stub out.
    add_wire(state, vec![Point::new(260, 160), Point::new(260, 200)]);
    add_wire(state, vec![Point::new(260, 180), Point::new(330, 180)]);
    // VDD rail.
    add_wire(state, vec![Point::new(260, 80), Point::new(260, 40)]);
    add_wire(state, vec![Point::new(260, 40), Point::new(360, 40)]);
    add_wire(state, vec![Point::new(360, 60), Point::new(360, 40)]);
    // PMOS bulk tied to its source.
    add_wire(
        state,
        vec![
            Point::new(260, 120),
            Point::new(300, 120),
            Point::new(300, 80),
            Point::new(260, 80),
        ],
    );
    // NMOS bulk tied to its source.
    add_wire(
        state,
        vec![
            Point::new(260, 240),
            Point::new(300, 240),
            Point::new(300, 280),
            Point::new(260, 280),
        ],
    );
    // Ground rail.
    add_wire(state, vec![Point::new(260, 280), Point::new(260, 320)]);
    add_wire(state, vec![Point::new(120, 320), Point::new(360, 320)]);
    add_wire(state, vec![Point::new(120, 260), Point::new(120, 320)]);
    add_wire(state, vec![Point::new(360, 100), Point::new(360, 320)]);
    add_label(state, Point::new(160, 180), "in");
    add_label(state, Point::new(330, 180), "out");
}

/// Differential Pair
/// ```text
///   ┌───┬──────────┬─────(VCC)
///  RC1 RC2         │
///   │   │          │
///   C   C
///  Q1    Q2     (bases driven by VIN1 / VIN2)
///   E     E
///   └──┬──┘
///    (IEE)
///      │
///     GND
/// ```
fn build_differential_pair(state: &mut SchematicState) {
    let mut id = 1u64;

    // Q1 — C (180,160), B (140,200), E (180,240).
    let q1 = Component::new(id, ComponentType::NpnBjt, Point::new(160, 200))
        .with_name_value("Q1", "2N2222");
    id += 1;
    state.components.push(q1);

    // Q2 mirrored — C (300,160), B (340,200), E (300,240).
    let q2 = Component::new(id, ComponentType::NpnBjt, Point::new(320, 200))
        .with_mirror_h(true)
        .with_name_value("Q2", "2N2222");
    id += 1;
    state.components.push(q2);

    // Collector loads — vertical: RC1 (180,80)/(180,120), RC2 (300,80)/(300,120).
    let rc1 = Component::new(id, ComponentType::Resistor, Point::new(180, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("RC1", "1k");
    id += 1;
    state.components.push(rc1);

    let rc2 = Component::new(id, ComponentType::Resistor, Point::new(300, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("RC2", "1k");
    id += 1;
    state.components.push(rc2);

    // Supply — vertical at R0: + (460,80), − (460,120).
    let vcc = Component::new(id, ComponentType::VoltageSource, Point::new(460, 100))
        .with_name_value("VCC", "12");
    id += 1;
    state.components.push(vcc);

    // Differential drive — vertical at R0.
    let vin1 = Component::new(id, ComponentType::VoltageSourceAc, Point::new(60, 260))
        .with_name_value("VIN1", "1m");
    id += 1;
    state.components.push(vin1);

    let vin2 = Component::new(id, ComponentType::VoltageSourceAc, Point::new(420, 260))
        .with_name_value("VIN2", "1m");
    id += 1;
    state.components.push(vin2);

    // Tail sink — vertical at R0: + (240,290), − (240,330).
    let iee = Component::new(id, ComponentType::CurrentSource, Point::new(240, 310))
        .with_name_value("IEE", "1m");
    id += 1;
    state.components.push(iee);

    // Ground — stem at (240,360) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(240, 370));
    state.components.push(gnd);

    // Collector loads down to the collectors.
    add_wire(state, vec![Point::new(180, 120), Point::new(180, 160)]);
    add_wire(state, vec![Point::new(300, 120), Point::new(300, 160)]);
    // Supply rail.
    add_wire(state, vec![Point::new(180, 80), Point::new(180, 60)]);
    add_wire(state, vec![Point::new(300, 80), Point::new(300, 60)]);
    add_wire(state, vec![Point::new(180, 60), Point::new(460, 60)]);
    add_wire(state, vec![Point::new(460, 80), Point::new(460, 60)]);
    // Emitters into the tail node.
    add_wire(
        state,
        vec![
            Point::new(180, 240),
            Point::new(180, 280),
            Point::new(240, 280),
        ],
    );
    add_wire(
        state,
        vec![
            Point::new(300, 240),
            Point::new(300, 280),
            Point::new(240, 280),
        ],
    );
    add_wire(state, vec![Point::new(240, 280), Point::new(240, 290)]);
    add_wire(state, vec![Point::new(240, 330), Point::new(240, 360)]);
    // Base drives.
    add_wire(
        state,
        vec![
            Point::new(60, 240),
            Point::new(60, 200),
            Point::new(140, 200),
        ],
    );
    add_wire(
        state,
        vec![
            Point::new(420, 240),
            Point::new(420, 200),
            Point::new(340, 200),
        ],
    );
    // Ground rail with all returns.
    add_wire(state, vec![Point::new(60, 360), Point::new(460, 360)]);
    add_wire(state, vec![Point::new(60, 280), Point::new(60, 360)]);
    add_wire(state, vec![Point::new(420, 280), Point::new(420, 360)]);
    add_wire(state, vec![Point::new(460, 120), Point::new(460, 360)]);
    add_label(state, Point::new(180, 140), "outn");
    add_label(state, Point::new(300, 140), "outp");
}

/// Opamp Inverting Amplifier (ideal op-amp triangle)
/// ```text
///            ┌──/\RF\──┐
///            │  10k    │
///  VIN──/\RIN\──┬──[in−]\
///         1k    │        >──┬──o Vout
///              [in+]─⏚ ────/
/// ```
fn build_opamp_inverter(state: &mut SchematicState) {
    let mut id = 1u64;

    // Ideal op-amp triangle:
    // in+ (260,150), in− (260,170), out (300,160).
    let opamp =
        Component::new(id, ComponentType::OpAmp, Point::new(280, 160)).with_name_value("E1", "1e6");
    id += 1;
    state.components.push(opamp);

    // Input resistor — horizontal: (140,170)/(180,170).
    let rin = Component::new(id, ComponentType::Resistor, Point::new(160, 170))
        .with_name_value("RIN", "1k");
    id += 1;
    state.components.push(rin);

    // Feedback resistor — horizontal above: (260,100)/(300,100).
    let rf = Component::new(id, ComponentType::Resistor, Point::new(280, 100))
        .with_name_value("RF", "10k");
    id += 1;
    state.components.push(rf);

    // Input source — vertical at R0: + (80,190), − (80,230).
    let vin = Component::new(id, ComponentType::VoltageSourceAc, Point::new(80, 210))
        .with_name_value("VIN", "100m");
    id += 1;
    state.components.push(vin);

    // Main ground — stem at (200,300) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(200, 310));
    id += 1;
    state.components.push(gnd);

    // Local reference for the non-inverting input — inverted ground whose
    // stem sits at (220,130).
    let gnd_ref = Component::new(id, ComponentType::Ground, Point::new(220, 120))
        .with_rotation(Rotation::R180);
    state.components.push(gnd_ref);

    // VIN+ to RIN.
    add_wire(
        state,
        vec![
            Point::new(80, 190),
            Point::new(80, 170),
            Point::new(140, 170),
        ],
    );
    // RIN to the inverting input (virtual ground node).
    add_wire(state, vec![Point::new(180, 170), Point::new(260, 170)]);
    // RF left, down into the virtual ground node (tee at x=200).
    add_wire(
        state,
        vec![
            Point::new(260, 100),
            Point::new(200, 100),
            Point::new(200, 170),
        ],
    );
    // Non-inverting input to its local reference.
    add_wire(
        state,
        vec![
            Point::new(260, 150),
            Point::new(220, 150),
            Point::new(220, 130),
        ],
    );
    // Output node with stub.
    add_wire(
        state,
        vec![
            Point::new(300, 160),
            Point::new(340, 160),
            Point::new(380, 160),
        ],
    );
    // RF right, down into the output (tee at x=340).
    add_wire(
        state,
        vec![
            Point::new(300, 100),
            Point::new(340, 100),
            Point::new(340, 160),
        ],
    );
    // Source return to the ground rail (the ideal op-amp's output is
    // ground-referenced internally).
    add_wire(state, vec![Point::new(80, 230), Point::new(80, 300)]);
    add_wire(state, vec![Point::new(80, 300), Point::new(200, 300)]);
    add_label(state, Point::new(380, 160), "vout");
}

fn build_hierarchical_rc_project(app: &mut crate::common::app::AppState) {
    let mut core_schematic = build_hierarchical_rc_core();
    let core_symbol = hierarchical_rc_symbol();
    let mut top = SchematicState::default();
    build_hierarchical_rc_top(&mut top);

    finish_loaded_example(&mut core_schematic);
    finish_loaded_example(&mut top);

    if app
        .library_manager
        .get_library(DEFAULT_PROJECT_LIBRARY)
        .is_none()
    {
        app.library_manager
            .add_library(Library::new(DEFAULT_PROJECT_LIBRARY));
    }

    let mut core_symbol_view = View::new("symbol", ViewType::Symbol);
    core_symbol
        .store_in_view(&mut core_symbol_view)
        .expect("hierarchical example symbol metadata serializes");

    let library = app
        .library_manager
        .get_library_mut(DEFAULT_PROJECT_LIBRARY)
        .expect("project library exists");
    replace_example_cell(
        library,
        HIERARCHICAL_RC_TOP_CELL,
        "Top-level testbench for the hierarchical RC filter example.",
        vec![View::new(DEFAULT_SCHEMATIC_VIEW, ViewType::Schematic)],
    );
    replace_example_cell(
        library,
        HIERARCHICAL_RC_CORE_CELL,
        "Reusable RC lowpass core with an authored symbol view.",
        vec![
            View::new(DEFAULT_SCHEMATIC_VIEW, ViewType::Schematic),
            core_symbol_view,
        ],
    );

    let top_ref = CellViewRef::new(
        DEFAULT_PROJECT_LIBRARY,
        HIERARCHICAL_RC_TOP_CELL,
        DEFAULT_SCHEMATIC_VIEW,
    );
    let core_ref = CellViewRef::new(
        DEFAULT_PROJECT_LIBRARY,
        HIERARCHICAL_RC_CORE_CELL,
        DEFAULT_SCHEMATIC_VIEW,
    );

    app.workspace.project.name = HIERARCHICAL_RC_FILTER.to_owned();
    app.workspace.project.root_library = DEFAULT_PROJECT_LIBRARY.to_owned();
    app.workspace.project.top_cell = HIERARCHICAL_RC_TOP_CELL.to_owned();
    app.workspace.active_view = top_ref.clone();
    app.workspace.open_views = vec![OpenCellView::new(top_ref.clone(), ViewType::Schematic)];
    app.workspace.hierarchy_stack = vec![top_ref.clone()];
    app.workspace.hierarchy_instances.clear();
    app.workspace.netlist_source = None;
    app.workspace.netlist_source_path = None;
    app.workspace
        .schematic_buffers
        .insert(top_ref.key(), top.clone());
    app.workspace
        .schematic_buffers
        .insert(core_ref.key(), core_schematic);
    app.schematic = top;
    app.library_manager.select_view(
        DEFAULT_PROJECT_LIBRARY,
        HIERARCHICAL_RC_TOP_CELL,
        DEFAULT_SCHEMATIC_VIEW,
    );
    app.shell.view = crate::shell::WorkspaceView::Schematic;
}

fn replace_example_cell(library: &mut Library, name: &str, description: &str, views: Vec<View>) {
    let mut cell = Cell::new(name);
    cell.description = description.to_owned();
    cell.category = "Examples".to_owned();
    for view in views {
        cell.add_view(view);
    }
    library.add_cell(cell);
}

fn finish_loaded_example(state: &mut SchematicState) {
    state.update_wire_junctions();
    state.needs_fit = true;
    state.needs_history_reset = true;
    state.is_dirty = true;
}

fn hierarchical_rc_ports() -> Vec<PortSpec> {
    vec![
        PortSpec {
            name: "in".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "out".to_owned(),
            direction: PortDirection::Out,
        },
        PortSpec {
            name: "vss".to_owned(),
            direction: PortDirection::Supply,
        },
    ]
}

fn build_hierarchical_rc_core() -> SchematicState {
    let mut state = SchematicState::default();
    place_port(&mut state, 1, "in", PortDirection::In, Point::new(10, 100));
    place_port(
        &mut state,
        2,
        "out",
        PortDirection::Out,
        Point::new(110, 100),
    );
    place_port(
        &mut state,
        3,
        "vss",
        PortDirection::Supply,
        Point::new(110, 180),
    );

    state.components.push(
        Component::new(4, ComponentType::Resistor, Point::new(50, 100)).with_name_value("R1", "1k"),
    );
    state.components.push(
        Component::new(5, ComponentType::Capacitor, Point::new(100, 140))
            .with_rotation(Rotation::R90)
            .with_name_value("C1", "159n"),
    );

    add_wire(&mut state, vec![Point::new(0, 100), Point::new(30, 100)]);
    add_wire(&mut state, vec![Point::new(70, 100), Point::new(100, 100)]);
    add_wire(&mut state, vec![Point::new(100, 100), Point::new(100, 120)]);
    add_wire(&mut state, vec![Point::new(100, 160), Point::new(100, 180)]);
    state
}

fn place_port(
    state: &mut SchematicState,
    id: u64,
    name: &str,
    direction: PortDirection,
    pos: Point,
) {
    let mut port = Component::new(id, ComponentType::Port, pos).with_name_value("", name);
    port.params = format!("dir={}", direction.keyword());
    state.components.push(port);
}

fn hierarchical_rc_symbol() -> SymbolDocument {
    SymbolDocument {
        pins: vec![
            SymbolPin::new("in", PortDirection::In, Some(Point::new(-40, 0))),
            SymbolPin::new("out", PortDirection::Out, Some(Point::new(40, 0))),
            SymbolPin::new("vss", PortDirection::Supply, Some(Point::new(0, 40))),
        ],
        body: vec![
            SymbolShape::Polyline {
                points: vec![
                    Point::new(-20, -20),
                    Point::new(20, -20),
                    Point::new(20, 20),
                    Point::new(-20, 20),
                ],
                closed: true,
            },
            SymbolShape::Polyline {
                points: vec![Point::new(-8, 0), Point::new(8, 0)],
                closed: false,
            },
            SymbolShape::Polyline {
                points: vec![Point::new(0, 20), Point::new(0, 40)],
                closed: false,
            },
        ],
        origin: Point::origin(),
        name_anchor: Point::new(-24, -36),
        value_anchor: Point::new(-24, 36),
    }
}

fn build_hierarchical_rc_top(state: &mut SchematicState) {
    let mut binding = LibraryCellInstance::new(
        DEFAULT_PROJECT_LIBRARY,
        HIERARCHICAL_RC_CORE_CELL,
        DEFAULT_SCHEMATIC_VIEW,
    );
    binding.bind_interface(&hierarchical_rc_ports());

    let mut instance = Component::new(1, ComponentType::CellInstance, Point::new(240, 140))
        .with_library_cell(binding);
    instance.name = "XU1".to_owned();
    instance.value = HIERARCHICAL_RC_CORE_CELL.to_owned();
    state.components.push(instance);

    state.components.push(
        Component::new(2, ComponentType::VoltageSourceAc, Point::new(100, 160))
            .with_name_value("VIN", "1"),
    );
    state.components.push(
        Component::new(3, ComponentType::Resistor, Point::new(360, 170))
            .with_rotation(Rotation::R90)
            .with_name_value("RLOAD", "10k"),
    );
    state.components.push(Component::new(
        4,
        ComponentType::Ground,
        Point::new(170, 220),
    ));

    add_wire(state, vec![Point::new(100, 140), Point::new(200, 140)]);
    add_wire(
        state,
        vec![
            Point::new(280, 140),
            Point::new(360, 140),
            Point::new(360, 150),
        ],
    );
    add_wire(state, vec![Point::new(360, 190), Point::new(360, 210)]);
    add_wire(state, vec![Point::new(100, 180), Point::new(100, 210)]);
    add_wire(state, vec![Point::new(240, 180), Point::new(240, 210)]);
    add_wire(state, vec![Point::new(100, 210), Point::new(360, 210)]);
    add_label(state, Point::new(150, 140), "vin");
    add_label(state, Point::new(320, 140), "out");
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Add a wire with the given points to the schematic
fn add_wire(state: &mut SchematicState, points: Vec<Point>) {
    if points.len() < 2 {
        return;
    }
    let wire_id = state.wires.len() as u64 + 1;
    let wire = Wire::new(wire_id, points);
    state.wires.push(wire);
}

/// Name a net at a point on a wire (also names the node in the netlist and
/// the resulting waveforms).
fn add_label(state: &mut SchematicState, pos: Point, name: &str) {
    let label_id = state.net_labels.len() as u64 + 1;
    state
        .net_labels
        .push(crate::state::NetLabel::new(label_id, pos, name));
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::app::AppState;
    use crate::simulation::netlist_gen::{HierarchySource, generate_netlist_hierarchical};
    use crate::state::{SYMBOL_DOCUMENT_METADATA_KEY, SymbolDocument};
    use std::collections::HashSet;

    /// Every wire endpoint must coincide with a component terminal or lie on
    /// (the interior of) another wire's segment — no dangling ends.
    #[test]
    fn example_wires_land_on_terminals() {
        for example in EXAMPLES {
            if example.requires_workspace {
                continue;
            }
            let mut state = SchematicState::default();
            load_example(example.name, &mut state);
            assert!(
                !state.components.is_empty(),
                "{}: example did not build",
                example.name
            );

            let terminals: HashSet<Point> = state
                .components
                .iter()
                .flat_map(|component| {
                    component
                        .terminal_positions()
                        .into_iter()
                        .map(|(_, point)| point)
                })
                .collect();

            let on_some_wire = |point: Point, skip_wire: u64| {
                state.wires.iter().any(|wire| {
                    wire.id != skip_wire
                        && wire.points.windows(2).any(|seg| {
                            let (a, b) = (seg[0], seg[1]);
                            if a.x == b.x {
                                point.x == a.x && point.y >= a.y.min(b.y) && point.y <= a.y.max(b.y)
                            } else if a.y == b.y {
                                point.y == a.y && point.x >= a.x.min(b.x) && point.x <= a.x.max(b.x)
                            } else {
                                false
                            }
                        })
                })
            };

            let labels: HashSet<Point> = state.net_labels.iter().map(|label| label.pos).collect();

            for wire in &state.wires {
                for endpoint in [wire.points[0], *wire.points.last().unwrap()] {
                    assert!(
                        terminals.contains(&endpoint)
                            || on_some_wire(endpoint, wire.id)
                            || labels.contains(&endpoint),
                        "{}: wire {} endpoint ({}, {}) dangles",
                        example.name,
                        wire.id,
                        endpoint.x,
                        endpoint.y
                    );
                }
            }
        }
    }

    /// Wire segments must be axis-aligned and on the 10-unit grid.
    #[test]
    fn example_wires_are_manhattan_and_on_grid() {
        for example in EXAMPLES {
            if example.requires_workspace {
                continue;
            }
            let mut state = SchematicState::default();
            load_example(example.name, &mut state);
            for wire in &state.wires {
                for seg in wire.points.windows(2) {
                    assert!(
                        seg[0].x == seg[1].x || seg[0].y == seg[1].y,
                        "{}: wire {} has a diagonal segment",
                        example.name,
                        wire.id
                    );
                }
                for p in &wire.points {
                    assert!(
                        p.x % 10 == 0 && p.y % 10 == 0,
                        "{}: wire {} point off-grid ({}, {})",
                        example.name,
                        wire.id,
                        p.x,
                        p.y
                    );
                }
            }
        }
    }

    #[test]
    fn hierarchical_example_loads_project_cells_and_symbol() {
        let example = EXAMPLES
            .iter()
            .find(|example| example.name == "Hierarchical RC Filter")
            .expect("hierarchical example is registered");
        assert!(example.requires_workspace);

        let mut app = AppState::default();
        assert!(load_example_into_app(example.name, &mut app));
        assert_eq!(app.workspace.active_view.cell, "hierarchical_rc_filter_tb");

        let library = app
            .library_manager
            .get_library("user")
            .expect("user library exists");
        let core = library
            .get_cell("rc_filter_core")
            .expect("child cell is created");
        assert!(core.get_view("schematic").is_some());
        let symbol_view = core.get_view("symbol").expect("child symbol view exists");
        assert!(
            symbol_view
                .metadata
                .contains_key(SYMBOL_DOCUMENT_METADATA_KEY),
            "symbol view stores authored symbol metadata"
        );

        let symbol = SymbolDocument::load_from_view(symbol_view).expect("symbol metadata decodes");
        let pin_names: Vec<&str> = symbol.pins.iter().map(|pin| pin.name.as_str()).collect();
        assert_eq!(pin_names, ["in", "out", "vss"]);
        assert!(
            symbol.pins.iter().all(|pin| pin.position.is_some()),
            "every child symbol pin is placed"
        );

        let top = app
            .workspace
            .schematic_buffers
            .get("user/hierarchical_rc_filter_tb/schematic")
            .expect("top testbench buffer exists");
        let instance = top
            .components
            .iter()
            .find(|component| component.kind == ComponentType::CellInstance)
            .expect("top schematic places the child cell");
        let binding = instance
            .library_cell
            .as_ref()
            .expect("cell instance has library binding");
        assert_eq!(instance.name, "XU1");
        assert_eq!(binding.library, "user");
        assert_eq!(binding.cell, "rc_filter_core");
        assert_eq!(binding.view, "schematic");
        assert_eq!(
            binding.terminal_order,
            vec!["in".to_owned(), "out".to_owned(), "vss".to_owned()]
        );
    }

    #[test]
    fn hierarchical_example_generates_project_subcircuit_netlist() {
        let mut app = AppState::default();
        assert!(load_example_into_app("Hierarchical RC Filter", &mut app));

        let hierarchy =
            HierarchySource::from_workspace(&app.library_manager, &app.workspace.schematic_buffers);
        let result = generate_netlist_hierarchical(&app.schematic, &[], &hierarchy);

        assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
        let lower = result.netlist.to_ascii_lowercase();
        assert!(
            lower.contains(".subckt rc_filter_core in out vss"),
            "netlist:\n{}",
            result.netlist
        );
        assert!(lower.contains(".ends rc_filter_core"));
        let x_line = result
            .netlist
            .lines()
            .find(|line| line.to_ascii_lowercase().starts_with("xu1 "))
            .expect("XU1 instance line is present");
        assert!(
            x_line
                .split_whitespace()
                .eq(["XU1", "vin", "out", "0", "rc_filter_core"]),
            "unexpected instance line: {x_line}\n{}",
            result.netlist
        );
    }
}
