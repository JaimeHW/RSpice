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
//! - **Ring Oscillator**: 3-stage CMOS oscillator

use crate::state::{Component, ComponentType, Point, Rotation, SchematicState, Wire};

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
}

/// All available example circuits
pub const EXAMPLES: &[ExampleCircuit] = &[
    ExampleCircuit {
        name: "RC Lowpass Filter",
        description: "Simple 1st-order RC lowpass with 1kHz cutoff",
        category: "Analog",
    },
    ExampleCircuit {
        name: "Voltage Divider",
        description: "Two-resistor voltage divider (2:1 ratio)",
        category: "Basics",
    },
    ExampleCircuit {
        name: "Common Emitter Amplifier",
        description: "Single-stage BJT amplifier with biasing",
        category: "Analog",
    },
    ExampleCircuit {
        name: "CMOS Inverter",
        description: "Basic CMOS NOT gate",
        category: "Digital",
    },
    ExampleCircuit {
        name: "Differential Pair",
        description: "BJT differential amplifier input stage",
        category: "Analog",
    },
    ExampleCircuit {
        name: "Opamp Inverting Amplifier",
        description: "Inverting amplifier with gain = -10",
        category: "Analog",
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
        _ => {}
    }
}

/// RC Lowpass Filter
/// ```text
///        R1=1k           
///  Vin o──/\/\/──┬── Vout
///                │
///               C1=159n
///                │
///               GND
/// ```
fn build_rc_lowpass(state: &mut SchematicState) {
    let mut id = 1u64;

    // Input voltage source (left side, vertical orientation for + on top, - on bottom)
    let v1 = Component::new(id, ComponentType::VoltageSourceAc, Point::new(100, 200))
        .with_rotation(Rotation::R90)
        .with_name_value("VIN", "1");
    id += 1;
    state.components.push(v1);

    // Resistor (horizontal, from input to output - default rotation)
    let r1 = Component::new(id, ComponentType::Resistor, Point::new(200, 100))
        .with_name_value("R1", "1k");
    id += 1;
    state.components.push(r1);

    // Capacitor (vertical, from output to ground - needs R90)
    let c1 = Component::new(id, ComponentType::Capacitor, Point::new(280, 180))
        .with_rotation(Rotation::R90)
        .with_name_value("C1", "159n");
    id += 1;
    state.components.push(c1);

    // Ground symbol (at bottom of capacitor)
    let gnd = Component::new(id, ComponentType::Ground, Point::new(280, 260));
    state.components.push(gnd);

    // Add wires to connect components
    // Vin+ to R1 input
    add_wire(
        state,
        vec![
            Point::new(100, 180),
            Point::new(100, 100),
            Point::new(180, 100),
        ],
    );
    // R1 output to C1 top (output node)
    add_wire(
        state,
        vec![
            Point::new(220, 100),
            Point::new(280, 100),
            Point::new(280, 160),
        ],
    );
    // C1 bottom to ground
    add_wire(state, vec![Point::new(280, 200), Point::new(280, 260)]);
    // Vin- to ground (common ground reference)
    add_wire(
        state,
        vec![
            Point::new(100, 220),
            Point::new(100, 260),
            Point::new(280, 260),
        ],
    );
}

/// Voltage Divider
/// ```text
///  Vcc o──┬──
///         │
///        R1=10k
///         │
///         ├── Vout
///         │
///        R2=10k
///         │
///        GND
/// ```
fn build_voltage_divider(state: &mut SchematicState) {
    let mut id = 1u64;

    // DC Voltage source (vertical orientation)
    let v1 = Component::new(id, ComponentType::VoltageSource, Point::new(100, 150))
        .with_rotation(Rotation::R90)
        .with_name_value("VCC", "5");
    id += 1;
    state.components.push(v1);

    // Top resistor (vertical)
    let r1 = Component::new(id, ComponentType::Resistor, Point::new(200, 100))
        .with_rotation(Rotation::R90)
        .with_name_value("R1", "10k");
    id += 1;
    state.components.push(r1);

    // Bottom resistor (vertical)
    let r2 = Component::new(id, ComponentType::Resistor, Point::new(200, 180))
        .with_rotation(Rotation::R90)
        .with_name_value("R2", "10k");
    id += 1;
    state.components.push(r2);

    // Ground
    let gnd = Component::new(id, ComponentType::Ground, Point::new(200, 260));
    state.components.push(gnd);

    // Wires
    // Vcc+ to R1 top
    add_wire(
        state,
        vec![
            Point::new(100, 130),
            Point::new(100, 60),
            Point::new(200, 60),
            Point::new(200, 80),
        ],
    );
    // R1 bottom to R2 top (output node)
    add_wire(state, vec![Point::new(200, 120), Point::new(200, 160)]);
    // R2 bottom to ground
    add_wire(state, vec![Point::new(200, 200), Point::new(200, 260)]);
    // Vcc- to ground
    add_wire(
        state,
        vec![
            Point::new(100, 170),
            Point::new(100, 260),
            Point::new(200, 260),
        ],
    );
}

/// Common Emitter Amplifier
fn build_common_emitter(state: &mut SchematicState) {
    let mut id = 1u64;

    // NPN BJT
    let q1 = Component::new(id, ComponentType::NpnBjt, Point::new(200, 160))
        .with_name_value("Q1", "2N2222");
    id += 1;
    state.components.push(q1);

    // Base resistor
    let rb = Component::new(id, ComponentType::Resistor, Point::new(100, 160))
        .with_name_value("RB", "100k");
    id += 1;
    state.components.push(rb);

    // Collector resistor
    let rc = Component::new(id, ComponentType::Resistor, Point::new(220, 80))
        .with_rotation(Rotation::R90)
        .with_name_value("RC", "1k");
    id += 1;
    state.components.push(rc);

    // Emitter resistor
    let re = Component::new(id, ComponentType::Resistor, Point::new(220, 240))
        .with_rotation(Rotation::R90)
        .with_name_value("RE", "100");
    id += 1;
    state.components.push(re);

    // Input coupling capacitor
    let cin = Component::new(id, ComponentType::Capacitor, Point::new(40, 160))
        .with_name_value("CIN", "1u");
    id += 1;
    state.components.push(cin);

    // Voltage source (VCC) - vertical orientation
    let vcc = Component::new(id, ComponentType::VoltageSource, Point::new(300, 80))
        .with_rotation(Rotation::R90)
        .with_name_value("VCC", "12");
    id += 1;
    state.components.push(vcc);

    // Ground
    let gnd = Component::new(id, ComponentType::Ground, Point::new(220, 320));
    state.components.push(gnd);

    // Basic wires
    add_wire(state, vec![Point::new(60, 160), Point::new(80, 160)]);
    add_wire(state, vec![Point::new(120, 160), Point::new(180, 160)]);
    add_wire(state, vec![Point::new(220, 140), Point::new(220, 100)]);
    add_wire(state, vec![Point::new(220, 180), Point::new(220, 220)]);
    add_wire(state, vec![Point::new(220, 260), Point::new(220, 300)]);
}

/// CMOS Inverter
fn build_cmos_inverter(state: &mut SchematicState) {
    let mut id = 1u64;

    // PMOS (pull-up)
    let mp = Component::new(id, ComponentType::Pmos, Point::new(200, 100))
        .with_name_value("MP", "PMOS W=2u L=0.18u");
    id += 1;
    state.components.push(mp);

    // NMOS (pull-down)
    let mn = Component::new(id, ComponentType::Nmos, Point::new(200, 200))
        .with_name_value("MN", "NMOS W=1u L=0.18u");
    id += 1;
    state.components.push(mn);

    // VDD source (vertical orientation)
    let vdd = Component::new(id, ComponentType::VoltageSource, Point::new(280, 60))
        .with_rotation(Rotation::R90)
        .with_name_value("VDD", "1.8");
    id += 1;
    state.components.push(vdd);

    // Input pulse source (vertical orientation)
    let vin = Component::new(id, ComponentType::VoltageSourcePulse, Point::new(80, 150))
        .with_rotation(Rotation::R90)
        .with_name_value("VIN", "PULSE(0 1.8 0 1n 1n 5n 10n)");
    id += 1;
    state.components.push(vin);

    // Ground
    let gnd = Component::new(id, ComponentType::Ground, Point::new(220, 280));
    state.components.push(gnd);

    // Wires for gates (input)
    add_wire(
        state,
        vec![
            Point::new(80, 130),
            Point::new(80, 100),
            Point::new(180, 100),
        ],
    );
    add_wire(
        state,
        vec![
            Point::new(80, 170),
            Point::new(80, 200),
            Point::new(180, 200),
        ],
    );
    // Output node
    add_wire(
        state,
        vec![
            Point::new(220, 120),
            Point::new(220, 150),
            Point::new(220, 180),
        ],
    );
}

/// Differential Pair
fn build_differential_pair(state: &mut SchematicState) {
    let mut id = 1u64;

    // Left BJT
    let q1 = Component::new(id, ComponentType::NpnBjt, Point::new(150, 180))
        .with_name_value("Q1", "2N2222");
    id += 1;
    state.components.push(q1);

    // Right BJT
    let q2 = Component::new(id, ComponentType::NpnBjt, Point::new(250, 180))
        .with_mirror_h(true)
        .with_name_value("Q2", "2N2222");
    id += 1;
    state.components.push(q2);

    // Left collector resistor
    let rc1 = Component::new(id, ComponentType::Resistor, Point::new(170, 80))
        .with_rotation(Rotation::R90)
        .with_name_value("RC1", "1k");
    id += 1;
    state.components.push(rc1);

    // Right collector resistor
    let rc2 = Component::new(id, ComponentType::Resistor, Point::new(230, 80))
        .with_rotation(Rotation::R90)
        .with_name_value("RC2", "1k");
    id += 1;
    state.components.push(rc2);

    // Tail current source (vertical orientation)
    let iee = Component::new(id, ComponentType::CurrentSource, Point::new(200, 280))
        .with_rotation(Rotation::R90)
        .with_name_value("IEE", "1m");
    id += 1;
    state.components.push(iee);

    // Ground
    let gnd = Component::new(id, ComponentType::Ground, Point::new(200, 360));
    state.components.push(gnd);

    // Emitter tie
    add_wire(
        state,
        vec![
            Point::new(170, 200),
            Point::new(170, 240),
            Point::new(200, 240),
        ],
    );
    add_wire(
        state,
        vec![
            Point::new(230, 200),
            Point::new(230, 240),
            Point::new(200, 240),
        ],
    );
    add_wire(state, vec![Point::new(200, 240), Point::new(200, 260)]);
    add_wire(state, vec![Point::new(200, 300), Point::new(200, 340)]);
}

/// Opamp Inverting Amplifier
fn build_opamp_inverter(state: &mut SchematicState) {
    let mut id = 1u64;

    // We'll use a VCVS as simple opamp model
    let opamp =
        Component::new(id, ComponentType::Vcvs, Point::new(200, 160)).with_name_value("E1", "1e6"); // High gain
    id += 1;
    state.components.push(opamp);

    // Input resistor
    let rin = Component::new(id, ComponentType::Resistor, Point::new(100, 140))
        .with_name_value("RIN", "1k");
    id += 1;
    state.components.push(rin);

    // Feedback resistor
    let rf = Component::new(id, ComponentType::Resistor, Point::new(200, 80))
        .with_name_value("RF", "10k");
    id += 1;
    state.components.push(rf);

    // Input source (vertical orientation)
    let vin = Component::new(id, ComponentType::VoltageSourceAc, Point::new(40, 180))
        .with_rotation(Rotation::R90)
        .with_name_value("VIN", "100m");
    id += 1;
    state.components.push(vin);

    // Ground
    let gnd = Component::new(id, ComponentType::Ground, Point::new(40, 260));
    state.components.push(gnd);

    // Basic wiring
    add_wire(
        state,
        vec![
            Point::new(40, 160),
            Point::new(40, 140),
            Point::new(80, 140),
        ],
    );
    add_wire(state, vec![Point::new(120, 140), Point::new(160, 140)]);
    add_wire(state, vec![Point::new(40, 200), Point::new(40, 240)]);
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_defined() {
        assert!(!EXAMPLES.is_empty());
        assert!(EXAMPLES.len() >= 5);
    }

    #[test]
    fn test_load_rc_lowpass() {
        let mut state = SchematicState::default();
        load_example("RC Lowpass Filter", &mut state);
        assert!(!state.components.is_empty());
        assert!(
            state
                .components
                .iter()
                .any(|c| c.kind == ComponentType::Resistor)
        );
        assert!(
            state
                .components
                .iter()
                .any(|c| c.kind == ComponentType::Capacitor)
        );
    }

    #[test]
    fn test_load_voltage_divider() {
        let mut state = SchematicState::default();
        load_example("Voltage Divider", &mut state);
        let resistor_count = state
            .components
            .iter()
            .filter(|c| c.kind == ComponentType::Resistor)
            .count();
        assert_eq!(resistor_count, 2);
    }

    #[test]
    fn test_load_common_emitter() {
        let mut state = SchematicState::default();
        load_example("Common Emitter Amplifier", &mut state);
        assert!(
            state
                .components
                .iter()
                .any(|c| c.kind == ComponentType::NpnBjt)
        );
    }

    #[test]
    fn test_load_cmos_inverter() {
        let mut state = SchematicState::default();
        load_example("CMOS Inverter", &mut state);
        assert!(
            state
                .components
                .iter()
                .any(|c| c.kind == ComponentType::Nmos)
        );
        assert!(
            state
                .components
                .iter()
                .any(|c| c.kind == ComponentType::Pmos)
        );
    }

    #[test]
    fn test_load_differential_pair() {
        let mut state = SchematicState::default();
        load_example("Differential Pair", &mut state);
        let bjt_count = state
            .components
            .iter()
            .filter(|c| c.kind == ComponentType::NpnBjt)
            .count();
        assert_eq!(bjt_count, 2);
    }

    #[test]
    fn test_load_invalid_example() {
        let mut state = SchematicState::default();
        state
            .components
            .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        load_example("NonExistent", &mut state);
        // Should clear and leave empty for unknown examples
        assert!(state.components.is_empty());
    }

    #[test]
    fn test_example_categories() {
        let analog_count = EXAMPLES.iter().filter(|e| e.category == "Analog").count();
        let digital_count = EXAMPLES.iter().filter(|e| e.category == "Digital").count();
        assert!(analog_count >= 2);
        assert!(digital_count >= 1);
    }
}
