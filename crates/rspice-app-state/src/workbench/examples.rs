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
//!
//! [`EXAMPLES`] lists them with the line Help's Examples dialog shows for
//! each, and the analysis whose plot shows the circuit at work;
//! [`load_example`] draws one by its name.
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

use rspice_design::schematic::document::SchematicDocument;

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{Component, ComponentType, Point, Rotation, SchematicState, Wire};
use crate::workbench::app_state::sim_setup::{DcSetup, SimSetupState, TranSetup};

// =============================================================================
// Example Circuit Definitions
// =============================================================================

/// One demonstration circuit: the name [`load_example`] draws it by, what
/// it is, in a line, and the analysis a run of it solves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Example {
    pub name: &'static str,
    pub summary: &'static str,
    pub analysis: ExampleAnalysis,
}

/// What a run of an example solves, chosen so the plot it opens on shows the
/// circuit doing its job. The values are the plan's own text, as a reader
/// would type them on the Simulation page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleAnalysis {
    /// The operating point, and a linear DC sweep of the source `source`
    /// from 0 to `stop` in steps of `step`.
    DcSweep {
        source: &'static str,
        stop: &'static str,
        step: &'static str,
    },
    /// A transient from 0 to `stop`, printed every `step`.
    Transient {
        stop: &'static str,
        step: &'static str,
    },
}

impl ExampleAnalysis {
    /// The drafts the plan holds, in order; the last is the one the plot
    /// shows.
    fn drafts(self) -> Vec<AnalysisDraft> {
        match self {
            Self::DcSweep { source, stop, step } => vec![
                AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
                AnalysisDraft::DcSweep(DcSetup {
                    source: source.to_owned(),
                    start: "0".to_owned(),
                    stop: stop.to_owned(),
                    step: step.to_owned(),
                    ..DcSetup::default()
                }),
            ],
            Self::Transient { stop, step } => vec![AnalysisDraft::Transient(TranSetup {
                stop: stop.to_owned(),
                step: step.to_owned(),
                ..TranSetup::default()
            })],
        }
    }

    /// Make the active plan of `setup` run this analysis in place of the
    /// analyses it had, and return the analysis the plot shows.
    pub fn plan_into(self, setup: &mut SimSetupState) -> Result<AnalysisInstanceId, String> {
        let plan = setup.stable_analysis_plan_mut()?;
        let replaced = plan
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect::<Vec<_>>();
        let mut shown = None;
        for (position, draft) in self.drafts().into_iter().enumerate() {
            let (id, _) = plan
                .insert_draft_with_id(AnalysisInstanceId::new(), draft, true, position)
                .map_err(|error| error.to_string())?;
            shown = Some(id);
        }
        for id in replaced {
            plan.remove(id, Vec::new())
                .map_err(|error| error.to_string())?;
        }
        setup.refresh_legacy_analysis_projections();
        shown.ok_or_else(|| "an example plans at least one analysis".to_owned())
    }
}

/// Every demonstration circuit, in the order the Examples dialog lists them.
pub const EXAMPLES: [Example; 6] = [
    Example {
        name: "RC Lowpass Filter",
        summary: "R1 1k into C1 159n: a first-order low-pass with its corner at 1 kHz.",
        analysis: ExampleAnalysis::Transient {
            stop: "3m",
            step: "2u",
        },
    },
    Example {
        name: "Voltage Divider",
        summary: "Two 10k resistors halving a 5 V supply.",
        analysis: ExampleAnalysis::DcSweep {
            source: "VCC",
            stop: "5",
            step: "0.1",
        },
    },
    Example {
        name: "Common Emitter Amplifier",
        summary: "An NPN stage on 12 V: RB 100k bias, RC 1k load, RE 100 degeneration.",
        analysis: ExampleAnalysis::Transient {
            stop: "3m",
            step: "2u",
        },
    },
    Example {
        name: "CMOS Inverter",
        summary: "A PMOS and NMOS pair on 3.3 V, switched by a pulse input.",
        analysis: ExampleAnalysis::Transient {
            stop: "40n",
            step: "50p",
        },
    },
    Example {
        name: "Differential Pair",
        summary: "Two NPN transistors sharing a 1 mA tail: an op-amp input stage.",
        analysis: ExampleAnalysis::Transient {
            stop: "3m",
            step: "2u",
        },
    },
    Example {
        name: "Opamp Inverting Amplifier",
        summary: "A gain of -10 from RF 10k over RIN 1k around a high-gain op-amp.",
        analysis: ExampleAnalysis::Transient {
            stop: "3m",
            step: "2u",
        },
    },
];

// =============================================================================
// Circuit Builders
// =============================================================================

/// Replace the schematic's circuit with the example `name`; an unknown name
/// changes nothing.
///
/// The circuit is drawn into a copy of the document, which becomes the
/// schematic's design in place of the old one; the editor session stays.
pub fn load_example(name: &str, state: &mut SchematicState) {
    let build: fn(&mut SchematicDocument) = match name {
        "RC Lowpass Filter" => build_rc_lowpass,
        "Voltage Divider" => build_voltage_divider,
        "Common Emitter Amplifier" => build_common_emitter,
        "CMOS Inverter" => build_cmos_inverter,
        "Differential Pair" => build_differential_pair,
        "Opamp Inverting Amplifier" => build_opamp_inverter,
        _ => return,
    };
    let mut doc = state.document().clone();
    doc.components.clear();
    doc.wires.clear();
    doc.junctions.clear();
    doc.net_labels.clear();
    build(&mut doc);
    let session = std::mem::take(&mut state.session);
    *state = SchematicState::from_document(doc);
    state.session = session;
    state.session.editor.selection.clear();

    // Junction dots where wires tee, frame the circuit, fresh history. The
    // example replaces the whole document, so there is no other sheet to
    // keep out of reach.
    state.with_active_wire_topology(|_| false, SchematicState::update_wire_junctions);
    state.session.editor.needs_fit = true;
    state.session.needs_history_reset = true;
    state.session.is_dirty = true;
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
fn build_rc_lowpass(doc: &mut SchematicDocument) {
    let mut id = 1u64;

    // Input source — vertical at R0: + (100,180), − (100,220). A 1 kHz
    // square wave, so the output shows the RC charging and discharging.
    let mut v1 = Component::new(id, ComponentType::VoltageSourcePulse, Point::new(100, 200))
        .with_name_value("VIN", "0");
    v1.params = "v2=1 td=0 tr=1u tf=1u pw=500u per=1m".to_owned();
    id += 1;
    doc.components.push(v1);

    // Series resistor — horizontal: (180,100), (220,100).
    let r1 = Component::new(id, ComponentType::Resistor, Point::new(200, 100))
        .with_name_value("R1", "1k");
    id += 1;
    doc.components.push(r1);

    // Shunt capacitor — vertical: top (280,180), bottom (280,220).
    let c1 = Component::new(id, ComponentType::Capacitor, Point::new(280, 200))
        .with_rotation(Rotation::R90)
        .with_name_value("C1", "159n");
    id += 1;
    doc.components.push(c1);

    // Ground — stem at (190,260), teeing into the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(190, 270));
    doc.components.push(gnd);

    // VIN+ up and over to R1 left.
    add_wire(
        doc,
        vec![
            Point::new(100, 180),
            Point::new(100, 100),
            Point::new(180, 100),
        ],
    );
    // R1 right to C1 top (the output node).
    add_wire(
        doc,
        vec![
            Point::new(220, 100),
            Point::new(280, 100),
            Point::new(280, 180),
        ],
    );
    // C1 bottom to the ground rail.
    add_wire(doc, vec![Point::new(280, 220), Point::new(280, 260)]);
    // VIN− to the ground rail (the ground stem tees in at x=190).
    add_wire(
        doc,
        vec![
            Point::new(100, 220),
            Point::new(100, 260),
            Point::new(280, 260),
        ],
    );
    add_label(doc, Point::new(250, 100), "out");
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
fn build_voltage_divider(doc: &mut SchematicDocument) {
    let mut id = 1u64;

    // Supply — vertical at R0: + (100,140), − (100,180).
    let v1 = Component::new(id, ComponentType::VoltageSource, Point::new(100, 160))
        .with_name_value("VCC", "5");
    id += 1;
    doc.components.push(v1);

    // Divider string — vertical resistors: R1 (200,80)/(200,120),
    // R2 (200,160)/(200,200).
    let r1 = Component::new(id, ComponentType::Resistor, Point::new(200, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("R1", "10k");
    id += 1;
    doc.components.push(r1);

    let r2 = Component::new(id, ComponentType::Resistor, Point::new(200, 180))
        .with_rotation(Rotation::R90)
        .with_name_value("R2", "10k");
    id += 1;
    doc.components.push(r2);

    // Ground — stem at (150,240) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(150, 250));
    doc.components.push(gnd);

    // VCC+ up and over to R1 top.
    add_wire(
        doc,
        vec![
            Point::new(100, 140),
            Point::new(100, 60),
            Point::new(200, 60),
            Point::new(200, 80),
        ],
    );
    // R1 bottom to R2 top (Vout).
    add_wire(doc, vec![Point::new(200, 120), Point::new(200, 160)]);
    // R2 bottom to the rail.
    add_wire(doc, vec![Point::new(200, 200), Point::new(200, 240)]);
    // VCC− along the rail.
    add_wire(
        doc,
        vec![
            Point::new(100, 180),
            Point::new(100, 240),
            Point::new(200, 240),
        ],
    );
    add_label(doc, Point::new(200, 140), "out");
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
fn build_common_emitter(doc: &mut SchematicDocument) {
    let mut id = 1u64;

    // Q1 — C (280,140), B (240,180), E (280,220). The foundation NPN card,
    // which every project resolves; no project defines a 2N2222.
    let q1 = Component::new(id, ComponentType::NpnBjt, Point::new(260, 180))
        .with_name_value("Q1", "RSPICE_NPN");
    id += 1;
    doc.components.push(q1);

    // Base bias from the supply rail — vertical: (180,100)/(180,140).
    let rb = Component::new(id, ComponentType::Resistor, Point::new(180, 120))
        .with_rotation(Rotation::R90)
        .with_name_value("RB", "100k");
    id += 1;
    doc.components.push(rb);

    // Collector load — vertical: (280,80)/(280,120).
    let rc = Component::new(id, ComponentType::Resistor, Point::new(280, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("RC", "1k");
    id += 1;
    doc.components.push(rc);

    // Emitter degeneration — vertical: (280,240)/(280,280).
    let re = Component::new(id, ComponentType::Resistor, Point::new(280, 260))
        .with_rotation(Rotation::R90)
        .with_name_value("RE", "100");
    id += 1;
    doc.components.push(re);

    // Input coupling — horizontal: (100,180)/(140,180).
    let cin = Component::new(id, ComponentType::Capacitor, Point::new(120, 180))
        .with_name_value("CIN", "1u");
    id += 1;
    doc.components.push(cin);

    // AC input — vertical at R0: + (60,220), − (60,260).
    // A 10 mV, 1 kHz sine, well inside the stage's band.
    let mut vin = Component::new(id, ComponentType::VoltageSourceSin, Point::new(60, 240))
        .with_name_value("VIN", "0");
    vin.params = "va=10m freq=1k".to_owned();
    id += 1;
    doc.components.push(vin);

    // Supply — vertical at R0: + (380,100), − (380,140).
    let vcc = Component::new(id, ComponentType::VoltageSource, Point::new(380, 120))
        .with_name_value("VCC", "12");
    id += 1;
    doc.components.push(vcc);

    // Ground — stem at (220,300) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(220, 310));
    doc.components.push(gnd);

    // Supply rail and drops.
    add_wire(doc, vec![Point::new(180, 60), Point::new(380, 60)]);
    add_wire(doc, vec![Point::new(180, 100), Point::new(180, 60)]);
    add_wire(doc, vec![Point::new(280, 80), Point::new(280, 60)]);
    add_wire(doc, vec![Point::new(380, 100), Point::new(380, 60)]);
    // RB bottom into the base node (tees the CIN→base wire at x=180).
    add_wire(doc, vec![Point::new(180, 140), Point::new(180, 180)]);
    // CIN right to the base.
    add_wire(doc, vec![Point::new(140, 180), Point::new(240, 180)]);
    // VIN+ to CIN left.
    add_wire(
        doc,
        vec![
            Point::new(60, 220),
            Point::new(60, 180),
            Point::new(100, 180),
        ],
    );
    // RC bottom to the collector.
    add_wire(doc, vec![Point::new(280, 120), Point::new(280, 140)]);
    // Emitter to RE top, RE bottom to the rail.
    add_wire(doc, vec![Point::new(280, 220), Point::new(280, 240)]);
    add_wire(doc, vec![Point::new(280, 280), Point::new(280, 300)]);
    // Ground rail with the source returns.
    add_wire(doc, vec![Point::new(60, 300), Point::new(380, 300)]);
    add_wire(doc, vec![Point::new(60, 260), Point::new(60, 300)]);
    add_wire(doc, vec![Point::new(380, 140), Point::new(380, 300)]);
    add_label(doc, Point::new(280, 130), "out");
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
fn build_cmos_inverter(doc: &mut SchematicDocument) {
    let mut id = 1u64;

    // A MOSFET's value is its model, the Model property; W and L are
    // instance parameters with properties of their own. Both transistors run
    // the foundation cards the built-in library ships.
    //
    // Pull-up PMOS — mirror_v puts S on top: S (260,80), G (220,120),
    // D (260,160), bulk (260,120).
    let mut mp = Component::new(id, ComponentType::Pmos, Point::new(240, 120))
        .with_mirror_v(true)
        .with_name_value("MP", "RSPICE_PMOS");
    mp.params = "w=2u l=0.18u".to_owned();
    id += 1;
    doc.components.push(mp);

    // Pull-down NMOS — D (260,200), G (220,240), S (260,280), bulk (260,240).
    let mut mn = Component::new(id, ComponentType::Nmos, Point::new(240, 240))
        .with_name_value("MN", "RSPICE_NMOS");
    mn.params = "w=1u l=0.18u".to_owned();
    id += 1;
    doc.components.push(mn);

    // Supply — vertical at R0: + (360,60), − (360,100). The foundation cards
    // threshold at ±1 V, so the rail sits well above |Vtn| + |Vtp|: at 1.8 V
    // both transistors would be off around mid-rail and the output would float
    // through every transition.
    let vdd = Component::new(id, ComponentType::VoltageSource, Point::new(360, 80))
        .with_name_value("VDD", "3.3");
    id += 1;
    doc.components.push(vdd);

    // Input pulse — vertical at R0: + (120,220), − (120,260).
    let mut vin = Component::new(id, ComponentType::VoltageSourcePulse, Point::new(120, 240))
        .with_name_value("VIN", "0");
    vin.params = "v2=3.3 td=0 tr=1n tf=1n pw=5n per=10n".to_owned();
    id += 1;
    doc.components.push(vin);

    // Ground — stem at (240,320) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(240, 330));
    doc.components.push(gnd);

    // Input to both gates (tee at (160,180)).
    add_wire(
        doc,
        vec![
            Point::new(120, 220),
            Point::new(120, 180),
            Point::new(160, 180),
        ],
    );
    add_wire(
        doc,
        vec![
            Point::new(160, 180),
            Point::new(160, 120),
            Point::new(220, 120),
        ],
    );
    add_wire(
        doc,
        vec![
            Point::new(160, 180),
            Point::new(160, 240),
            Point::new(220, 240),
        ],
    );
    // Output node: MP drain to MN drain, with a stub out.
    add_wire(doc, vec![Point::new(260, 160), Point::new(260, 200)]);
    add_wire(doc, vec![Point::new(260, 180), Point::new(330, 180)]);
    // VDD rail.
    add_wire(doc, vec![Point::new(260, 80), Point::new(260, 40)]);
    add_wire(doc, vec![Point::new(260, 40), Point::new(360, 40)]);
    add_wire(doc, vec![Point::new(360, 60), Point::new(360, 40)]);
    // PMOS bulk tied to its source.
    add_wire(
        doc,
        vec![
            Point::new(260, 120),
            Point::new(300, 120),
            Point::new(300, 80),
            Point::new(260, 80),
        ],
    );
    // NMOS bulk tied to its source.
    add_wire(
        doc,
        vec![
            Point::new(260, 240),
            Point::new(300, 240),
            Point::new(300, 280),
            Point::new(260, 280),
        ],
    );
    // Ground rail.
    add_wire(doc, vec![Point::new(260, 280), Point::new(260, 320)]);
    add_wire(doc, vec![Point::new(120, 320), Point::new(360, 320)]);
    add_wire(doc, vec![Point::new(120, 260), Point::new(120, 320)]);
    add_wire(doc, vec![Point::new(360, 100), Point::new(360, 320)]);
    add_label(doc, Point::new(160, 180), "in");
    add_label(doc, Point::new(330, 180), "out");
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
fn build_differential_pair(doc: &mut SchematicDocument) {
    let mut id = 1u64;

    // Q1 — C (180,160), B (140,200), E (180,240). Both transistors take the
    // foundation NPN card, which every project resolves.
    let q1 = Component::new(id, ComponentType::NpnBjt, Point::new(160, 200))
        .with_name_value("Q1", "RSPICE_NPN");
    id += 1;
    doc.components.push(q1);

    // Q2 mirrored — C (300,160), B (340,200), E (300,240).
    let q2 = Component::new(id, ComponentType::NpnBjt, Point::new(320, 200))
        .with_mirror_h(true)
        .with_name_value("Q2", "RSPICE_NPN");
    id += 1;
    doc.components.push(q2);

    // Collector loads — vertical: RC1 (180,80)/(180,120), RC2 (300,80)/(300,120).
    let rc1 = Component::new(id, ComponentType::Resistor, Point::new(180, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("RC1", "1k");
    id += 1;
    doc.components.push(rc1);

    let rc2 = Component::new(id, ComponentType::Resistor, Point::new(300, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("RC2", "1k");
    id += 1;
    doc.components.push(rc2);

    // Supply — vertical at R0: + (460,80), − (460,120).
    let vcc = Component::new(id, ComponentType::VoltageSource, Point::new(460, 100))
        .with_name_value("VCC", "12");
    id += 1;
    doc.components.push(vcc);

    // Differential drive — vertical at R0. The two 5 mV, 1 kHz sines swing
    // in antiphase, so the collectors swing against each other; driven in
    // phase, they would cancel.
    let mut vin1 = Component::new(id, ComponentType::VoltageSourceSin, Point::new(60, 260))
        .with_name_value("VIN1", "0");
    vin1.params = "va=5m freq=1k".to_owned();
    id += 1;
    doc.components.push(vin1);

    let mut vin2 = Component::new(id, ComponentType::VoltageSourceSin, Point::new(420, 260))
        .with_name_value("VIN2", "0");
    vin2.params = "va=5m freq=1k phase=180".to_owned();
    id += 1;
    doc.components.push(vin2);

    // Tail sink — vertical at R0: + (240,290), − (240,330).
    let iee = Component::new(id, ComponentType::CurrentSource, Point::new(240, 310))
        .with_name_value("IEE", "1m");
    id += 1;
    doc.components.push(iee);

    // Ground — stem at (240,360) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(240, 370));
    doc.components.push(gnd);

    // Collector loads down to the collectors.
    add_wire(doc, vec![Point::new(180, 120), Point::new(180, 160)]);
    add_wire(doc, vec![Point::new(300, 120), Point::new(300, 160)]);
    // Supply rail.
    add_wire(doc, vec![Point::new(180, 80), Point::new(180, 60)]);
    add_wire(doc, vec![Point::new(300, 80), Point::new(300, 60)]);
    add_wire(doc, vec![Point::new(180, 60), Point::new(460, 60)]);
    add_wire(doc, vec![Point::new(460, 80), Point::new(460, 60)]);
    // Emitters into the tail node.
    add_wire(
        doc,
        vec![
            Point::new(180, 240),
            Point::new(180, 280),
            Point::new(240, 280),
        ],
    );
    add_wire(
        doc,
        vec![
            Point::new(300, 240),
            Point::new(300, 280),
            Point::new(240, 280),
        ],
    );
    add_wire(doc, vec![Point::new(240, 280), Point::new(240, 290)]);
    add_wire(doc, vec![Point::new(240, 330), Point::new(240, 360)]);
    // Base drives.
    add_wire(
        doc,
        vec![
            Point::new(60, 240),
            Point::new(60, 200),
            Point::new(140, 200),
        ],
    );
    add_wire(
        doc,
        vec![
            Point::new(420, 240),
            Point::new(420, 200),
            Point::new(340, 200),
        ],
    );
    // Ground rail with all returns.
    add_wire(doc, vec![Point::new(60, 360), Point::new(460, 360)]);
    add_wire(doc, vec![Point::new(60, 280), Point::new(60, 360)]);
    add_wire(doc, vec![Point::new(420, 280), Point::new(420, 360)]);
    add_wire(doc, vec![Point::new(460, 120), Point::new(460, 360)]);
    add_label(doc, Point::new(180, 140), "outn");
    add_label(doc, Point::new(300, 140), "outp");
}

/// Opamp Inverting Amplifier (ideal op-amp triangle)
/// ```text
///            ┌──/\RF\──┐
///            │  10k    │
///  VIN──/\RIN\──┬──[in−]\
///         1k    │        >──┬──o Vout
///              [in+]─⏚ ────/
/// ```
fn build_opamp_inverter(doc: &mut SchematicDocument) {
    let mut id = 1u64;

    // Ideal op-amp triangle:
    // in+ (260,150), in− (260,170), out (300,160).
    let opamp =
        Component::new(id, ComponentType::OpAmp, Point::new(280, 160)).with_name_value("E1", "1e6");
    id += 1;
    doc.components.push(opamp);

    // Input resistor — horizontal: (140,170)/(180,170).
    let rin = Component::new(id, ComponentType::Resistor, Point::new(160, 170))
        .with_name_value("RIN", "1k");
    id += 1;
    doc.components.push(rin);

    // Feedback resistor — horizontal above: (260,100)/(300,100).
    let rf = Component::new(id, ComponentType::Resistor, Point::new(280, 100))
        .with_name_value("RF", "10k");
    id += 1;
    doc.components.push(rf);

    // Input source — vertical at R0: + (80,190), − (80,230). A 100 mV,
    // 1 kHz sine, which comes out ten times larger and inverted.
    let mut vin = Component::new(id, ComponentType::VoltageSourceSin, Point::new(80, 210))
        .with_name_value("VIN", "0");
    vin.params = "va=100m freq=1k".to_owned();
    id += 1;
    doc.components.push(vin);

    // Main ground — stem at (200,300) on the bottom rail.
    let gnd = Component::new(id, ComponentType::Ground, Point::new(200, 310));
    id += 1;
    doc.components.push(gnd);

    // Local reference for the non-inverting input — inverted ground whose
    // stem sits at (220,130).
    let gnd_ref = Component::new(id, ComponentType::Ground, Point::new(220, 120))
        .with_rotation(Rotation::R180);
    doc.components.push(gnd_ref);

    // VIN+ to RIN.
    add_wire(
        doc,
        vec![
            Point::new(80, 190),
            Point::new(80, 170),
            Point::new(140, 170),
        ],
    );
    // RIN to the inverting input (virtual ground node).
    add_wire(doc, vec![Point::new(180, 170), Point::new(260, 170)]);
    // RF left, down into the virtual ground node (tee at x=200).
    add_wire(
        doc,
        vec![
            Point::new(260, 100),
            Point::new(200, 100),
            Point::new(200, 170),
        ],
    );
    // Non-inverting input to its local reference.
    add_wire(
        doc,
        vec![
            Point::new(260, 150),
            Point::new(220, 150),
            Point::new(220, 130),
        ],
    );
    // Output node with stub.
    add_wire(
        doc,
        vec![
            Point::new(300, 160),
            Point::new(340, 160),
            Point::new(380, 160),
        ],
    );
    // RF right, down into the output (tee at x=340).
    add_wire(
        doc,
        vec![
            Point::new(300, 100),
            Point::new(340, 100),
            Point::new(340, 160),
        ],
    );
    // Source return to the ground rail (the ideal op-amp's output is
    // ground-referenced internally).
    add_wire(doc, vec![Point::new(80, 230), Point::new(80, 300)]);
    add_wire(doc, vec![Point::new(80, 300), Point::new(200, 300)]);
    add_label(doc, Point::new(110, 170), "in");
    add_label(doc, Point::new(380, 160), "vout");
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Add a wire with the given points to the schematic
fn add_wire(doc: &mut SchematicDocument, points: Vec<Point>) {
    if points.len() < 2 {
        return;
    }
    let wire_id = doc.wires.len() as u64 + 1;
    let wire = Wire::new(wire_id, points);
    doc.wires.push(wire);
}

/// Name a net at a point on a wire (also names the node in the netlist and
/// the resulting waveforms).
fn add_label(doc: &mut SchematicDocument, pos: Point, name: &str) {
    let label_id = doc.net_labels.len() as u64 + 1;
    doc.net_labels
        .push(crate::state::NetLabel::new(label_id, pos, name));
}
