//! Component Edit Modal
//!
//! Modal popup for editing component properties with type-specific fields.

use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;

use crate::state::{ComponentType, SchematicState};
use crate::theme::Theme;

/// Props for ComponentEditModal
#[derive(Props, Clone, PartialEq)]
pub struct ComponentEditProps {
    /// ID of component being edited
    pub component_id: u64,
    /// Screen position for the modal
    pub position: (f64, f64),
    /// Callback when editing is complete
    pub on_save: EventHandler<(String, String, String)>,
    /// Callback when editing is cancelled
    pub on_cancel: EventHandler<()>,
    /// Schematic state for reading component data
    pub schematic: Signal<SchematicState>,
}

/// Get field labels and placeholders for each component type
fn get_type_fields(
    comp_type: ComponentType,
) -> (&'static str, &'static str, &'static str, &'static str, bool) {
    // Returns: (value_label, value_placeholder, params_label, params_placeholder, show_params)
    match comp_type {
        // Passive components - show Value field
        ComponentType::Resistor => (
            "Resistance",
            "1k, 10k, 4.7M",
            "Tolerance / TC",
            "tol=5% tc=100ppm",
            true,
        ),
        ComponentType::Capacitor => (
            "Capacitance",
            "1u, 100n, 47p",
            "Initial Condition",
            "ic=0",
            true,
        ),
        ComponentType::Inductor => (
            "Inductance",
            "1m, 100u, 10n",
            "Series Resistance",
            "rser=0.1",
            true,
        ),

        // Semiconductors - show Model field
        ComponentType::Diode => (
            "Model",
            "1N4148, 1N4007, BAT54",
            "Area Factor",
            "area=1",
            true,
        ),
        ComponentType::NpnBjt | ComponentType::PnpBjt => {
            ("Model", "2N2222, 2N3904, BC547", "Multiplier", "m=1", true)
        }
        ComponentType::Nmos | ComponentType::Pmos => (
            "Model",
            "2N7000, IRF530, BSS138",
            "W / L",
            "W=10u L=1u",
            true,
        ),

        // Voltage sources - show complex parameters
        ComponentType::VoltageSource => {
            ("DC Value", "5, 3.3, 12", "Source Type", "DC, AC 1", false)
        }
        ComponentType::VoltageSourceAc => (
            "AC Amplitude",
            "1, 5, 10V",
            "Frequency",
            "1k, 100k, 1MEG",
            true,
        ),
        ComponentType::VoltageSourceSin => (
            "Amplitude",
            "5",
            "SIN Parameters",
            "0 5 1k (offset amp freq)",
            true,
        ),
        ComponentType::VoltageSourcePulse => (
            "High Level",
            "5",
            "PULSE Parameters",
            "0 5 0 1n 1n 1u 2u",
            true,
        ),

        // Current source
        ComponentType::CurrentSource => (
            "DC Current",
            "1m, 10u, 100n",
            "Source Type",
            "DC, AC 1m",
            false,
        ),

        // Semiconductors - JFETs
        ComponentType::Njfet | ComponentType::Pjfet => {
            ("Model", "J310, 2N5457, MPF102", "Multiplier", "m=1", true)
        }

        // Coupled inductor (transformer)
        ComponentType::CoupledInductor => (
            "Coupling",
            "L1 L2 0.99",
            "Coupling Coefficient",
            "k=0.99",
            true,
        ),

        // Controlled sources
        ComponentType::Vcvs => (
            "Gain",
            "10, 100",
            "Parameters",
            "E=V(ctrl+, ctrl-)*gain",
            false,
        ),
        ComponentType::Vccs => (
            "Transconductance",
            "1m, 10m",
            "Parameters",
            "gm in Siemens",
            false,
        ),
        ComponentType::Ccvs => (
            "Transresistance",
            "1k, 10k",
            "Control Source",
            "Vsense",
            true,
        ),
        ComponentType::Cccs => ("Current Gain", "10, 100", "Control Source", "Vsense", true),

        // Ground - minimal editing
        ComponentType::Ground => ("(no value)", "", "", "", false),

        // XSPICE Analog Behavioral
        ComponentType::XspiceGain => ("Gain", "1, 10, 100", "Parameters", "gain=1", true),
        ComponentType::XspiceSummer => ("(no value)", "", "Parameters", "", false),
        ComponentType::XspiceMultiplier => ("(no value)", "", "Parameters", "", false),
        ComponentType::XspiceDivider => ("(no value)", "", "Parameters", "", false),
        ComponentType::XspiceLimiter => (
            "Limits",
            "-1, 1",
            "Parameters",
            "out_lower_limit=-1 out_upper_limit=1",
            true,
        ),
        ComponentType::XspiceIntegrator => ("Initial Condition", "0", "Parameters", "ic=0", true),
        ComponentType::XspiceDifferentiator => ("(no value)", "", "Parameters", "", false),
        // XSPICE Digital Gates
        ComponentType::XspiceInverter | ComponentType::XspiceBuffer => {
            ("(no value)", "", "Parameters", "", false)
        }
        ComponentType::XspiceAndGate
        | ComponentType::XspiceOrGate
        | ComponentType::XspiceNandGate
        | ComponentType::XspiceNorGate
        | ComponentType::XspiceXorGate => ("(no value)", "", "Parameters", "", false),
        ComponentType::XspiceTristate => ("(no value)", "", "Parameters", "", false),
        // XSPICE Sequential
        ComponentType::XspiceDFlipFlop
        | ComponentType::XspiceJkFlipFlop
        | ComponentType::XspiceSrLatch => ("(no value)", "", "Parameters", "", false),
        // XSPICE Bridges
        ComponentType::XspiceAdcBridge => (
            "Thresholds",
            "0.5, 2.5",
            "Parameters",
            "in_low=0.5 in_high=2.5",
            true,
        ),
        ComponentType::XspiceDacBridge => (
            "Output Levels",
            "0, 5",
            "Parameters",
            "out_low=0 out_high=5",
            true,
        ),
    }
}

/// Get a friendly type name
fn get_type_name(comp_type: ComponentType) -> &'static str {
    match comp_type {
        ComponentType::Resistor => "Resistor",
        ComponentType::Capacitor => "Capacitor",
        ComponentType::Inductor => "Inductor",
        ComponentType::Diode => "Diode",
        ComponentType::NpnBjt => "NPN Transistor",
        ComponentType::PnpBjt => "PNP Transistor",
        ComponentType::Nmos => "N-Channel MOSFET",
        ComponentType::Pmos => "P-Channel MOSFET",
        ComponentType::VoltageSource => "DC Voltage Source",
        ComponentType::VoltageSourceAc => "AC Voltage Source",
        ComponentType::VoltageSourceSin => "Sine Voltage Source",
        ComponentType::VoltageSourcePulse => "Pulse Voltage Source",
        ComponentType::CurrentSource => "Current Source",
        ComponentType::Njfet => "N-JFET",
        ComponentType::Pjfet => "P-JFET",
        ComponentType::CoupledInductor => "Coupled Inductor",
        ComponentType::Vcvs => "VCVS (E)",
        ComponentType::Vccs => "VCCS (G)",
        ComponentType::Ccvs => "CCVS (H)",
        ComponentType::Cccs => "CCCS (F)",
        ComponentType::Ground => "Ground",
        // XSPICE types use display_name() method
        ComponentType::XspiceGain => "Gain Block",
        ComponentType::XspiceSummer => "Summer",
        ComponentType::XspiceMultiplier => "Multiplier",
        ComponentType::XspiceDivider => "Divider",
        ComponentType::XspiceLimiter => "Limiter",
        ComponentType::XspiceIntegrator => "Integrator",
        ComponentType::XspiceDifferentiator => "Differentiator",
        ComponentType::XspiceInverter => "Inverter",
        ComponentType::XspiceBuffer => "Buffer",
        ComponentType::XspiceAndGate => "AND Gate",
        ComponentType::XspiceOrGate => "OR Gate",
        ComponentType::XspiceNandGate => "NAND Gate",
        ComponentType::XspiceNorGate => "NOR Gate",
        ComponentType::XspiceXorGate => "XOR Gate",
        ComponentType::XspiceTristate => "Tri-State Buffer",
        ComponentType::XspiceDFlipFlop => "D Flip-Flop",
        ComponentType::XspiceJkFlipFlop => "JK Flip-Flop",
        ComponentType::XspiceSrLatch => "SR Latch",
        ComponentType::XspiceAdcBridge => "ADC Bridge",
        ComponentType::XspiceDacBridge => "DAC Bridge",
    }
}

/// Modal popup for editing component properties
#[component]
pub fn ComponentEditModal(props: ComponentEditProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Get initial values from component
    let schematic = props.schematic.read();
    let comp = schematic
        .components
        .iter()
        .find(|c| c.id == props.component_id);

    let (initial_name, initial_value, initial_params, comp_type) = match comp {
        Some(c) => (
            c.name.clone(),
            c.value.clone(),
            c.params.clone(),
            Some(c.kind),
        ),
        None => (String::new(), String::new(), String::new(), None),
    };
    drop(schematic);

    // Get type-specific field configuration
    let (value_label, value_placeholder, params_label, params_placeholder, show_params) = comp_type
        .map(get_type_fields)
        .unwrap_or(("Value", "", "Parameters", "", true));

    let type_name = comp_type.map(get_type_name).unwrap_or("Component");
    let is_ground = matches!(comp_type, Some(ComponentType::Ground));

    // Local state for editing
    let mut name = use_signal(|| initial_name.clone());
    let mut value = use_signal(|| initial_value.clone());
    let mut params = use_signal(|| initial_params.clone());

    // Position the modal near the component
    let (x, y) = props.position;
    let left = (x - 150.0).max(10.0);
    let top = (y + 20.0).max(10.0);

    rsx! {
        // Backdrop
        div {
            style: "
                position: fixed;
                inset: 0;
                background: rgba(0, 0, 0, 0.5);
                z-index: 1000;
            ",
            onclick: move |_| props.on_cancel.call(()),
        }

        // Modal
        div {
            style: "
                position: fixed;
                left: {left}px;
                top: {top}px;
                background: {th.bg_secondary()};
                border: 1px solid {th.border()};
                border-radius: 8px;
                padding: 16px;
                min-width: 300px;
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
                z-index: 1001;
            ",
            // Prevent click from closing modal
            onclick: move |e| e.stop_propagation(),

            onkeydown: move |e| {
                match e.key() {
                    Key::Escape => props.on_cancel.call(()),
                    Key::Enter if !e.modifiers().shift() => {
                        props.on_save.call((
                            name.read().clone(),
                            value.read().clone(),
                            params.read().clone(),
                        ));
                    }
                    _ => {}
                }
            },

            // Header with component type
            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 16px;
                    padding-bottom: 8px;
                    border-bottom: 1px solid {th.border()};
                ",
                span {
                    style: "font-weight: 600; color: {th.text_primary()}; font-size: 14px;",
                    "Edit {type_name}"
                }
            }

            // Form fields
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                // Name field (always shown)
                div {
                    label {
                        style: "display: block; font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Reference Designator"
                    }
                    input {
                        r#type: "text",
                        value: "{name}",
                        placeholder: "R1, C2, etc.",
                        style: "
                            width: 100%;
                            padding: 8px 10px;
                            background: {th.bg_primary()};
                            border: 1px solid {th.border()};
                            border-radius: 6px;
                            color: {th.text_primary()};
                            font-size: 13px;
                            font-family: {Theme::FONT_MONO};
                            outline: none;
                            box-sizing: border-box;
                        ",
                        autofocus: true,
                        oninput: move |e| name.set(e.value().clone()),
                    }
                }

                // Value/Model field (not shown for Ground)
                if !is_ground {
                    div {
                        label {
                            style: "display: block; font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px;",
                            "{value_label}"
                        }
                        input {
                            r#type: "text",
                            value: "{value}",
                            placeholder: "{value_placeholder}",
                            style: "
                                width: 100%;
                                padding: 8px 10px;
                                background: {th.bg_primary()};
                                border: 1px solid {th.border()};
                                border-radius: 6px;
                                color: {th.text_primary()};
                                font-size: 13px;
                                font-family: {Theme::FONT_MONO};
                                outline: none;
                                box-sizing: border-box;
                            ",
                            oninput: move |e| value.set(e.value().clone()),
                        }
                    }
                }

                // Parameters field (conditionally shown)
                if show_params && !is_ground {
                    div {
                        label {
                            style: "display: block; font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.5px;",
                            "{params_label}"
                        }
                        input {
                            r#type: "text",
                            value: "{params}",
                            placeholder: "{params_placeholder}",
                            style: "
                                width: 100%;
                                padding: 8px 10px;
                                background: {th.bg_primary()};
                                border: 1px solid {th.border()};
                                border-radius: 6px;
                                color: {th.text_primary()};
                                font-size: 13px;
                                font-family: {Theme::FONT_MONO};
                                outline: none;
                                box-sizing: border-box;
                            ",
                            oninput: move |e| params.set(e.value().clone()),
                        }
                    }
                }
            }

            // Buttons
            div {
                style: "
                    display: flex;
                    justify-content: flex-end;
                    gap: 8px;
                    margin-top: 16px;
                    padding-top: 12px;
                    border-top: 1px solid {th.border()};
                ",

                button {
                    style: "
                        padding: 8px 16px;
                        background: transparent;
                        border: 1px solid {th.border()};
                        border-radius: 6px;
                        color: {th.text_secondary()};
                        font-size: 12px;
                        cursor: pointer;
                    ",
                    onclick: move |_| props.on_cancel.call(()),
                    "Cancel"
                }

                button {
                    style: "
                        padding: 8px 16px;
                        background: {th.accent_primary()};
                        border: none;
                        border-radius: 6px;
                        color: white;
                        font-size: 12px;
                        font-weight: 500;
                        cursor: pointer;
                    ",
                    onclick: move |_| {
                        props.on_save.call((
                            name.read().clone(),
                            value.read().clone(),
                            params.read().clone(),
                        ));
                    },
                    "Save"
                }
            }

            // Help text
            div {
                style: "margin-top: 12px; font-size: 10px; color: {th.text_muted()}; opacity: 0.7;",
                "Press Enter to save, Escape to cancel"
            }
        }
    }
}
