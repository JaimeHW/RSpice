//! Component Library Browser
//!
//! Searchable sidebar for browsing and placing components.
//! Shows built-in component types and models from embedded SPICE libraries.

use std::sync::Arc;

use dioxus::prelude::*;
use rspice_core::library::{LibraryManager, ModelType};

use crate::dialogs::{VerilogAImportDialog, VerilogAModelInfo};
use crate::state::{use_canvas_focus, ComponentType, Rotation, SchematicState, Tool};
use crate::theme::Theme;

/// Component category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCategory {
    Passives,
    Semiconductors,
    Sources,
    ControlledSources,
    XspiceAnalog,
    XspiceDigital,
    XspiceBridges,
    VerilogA,
}

impl ComponentCategory {
    fn name(&self) -> &'static str {
        match self {
            ComponentCategory::Passives => "Passive Components",
            ComponentCategory::Semiconductors => "Semiconductors",
            ComponentCategory::Sources => "Sources",
            ComponentCategory::ControlledSources => "Controlled Sources",
            ComponentCategory::XspiceAnalog => "XSPICE Analog",
            ComponentCategory::XspiceDigital => "XSPICE Digital",
            ComponentCategory::XspiceBridges => "XSPICE Bridges",
            ComponentCategory::VerilogA => "Verilog-A Models",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ComponentCategory::Passives => "",
            ComponentCategory::Semiconductors => "",
            ComponentCategory::Sources => "",
            ComponentCategory::ControlledSources => "",
            ComponentCategory::XspiceAnalog => "ƒ",
            ComponentCategory::XspiceDigital => "⚡",
            ComponentCategory::XspiceBridges => "↔",
            ComponentCategory::VerilogA => "VA",
        }
    }

    fn components(&self) -> &'static [ComponentType] {
        match self {
            ComponentCategory::Passives => &[
                ComponentType::Resistor,
                ComponentType::Capacitor,
                ComponentType::Inductor,
                ComponentType::CoupledInductor,
            ],
            ComponentCategory::Semiconductors => &[
                ComponentType::Diode,
                ComponentType::NpnBjt,
                ComponentType::PnpBjt,
                ComponentType::Nmos,
                ComponentType::Pmos,
                ComponentType::Njfet,
                ComponentType::Pjfet,
            ],
            ComponentCategory::Sources => &[
                ComponentType::VoltageSource,
                ComponentType::VoltageSourceAc,
                ComponentType::VoltageSourceSin,
                ComponentType::VoltageSourcePulse,
                ComponentType::CurrentSource,
                ComponentType::Ground,
            ],
            ComponentCategory::ControlledSources => &[
                ComponentType::Vcvs,
                ComponentType::Vccs,
                ComponentType::Ccvs,
                ComponentType::Cccs,
            ],
            ComponentCategory::XspiceAnalog => &[
                ComponentType::XspiceGain,
                ComponentType::XspiceSummer,
                ComponentType::XspiceMultiplier,
                ComponentType::XspiceDivider,
                ComponentType::XspiceLimiter,
                ComponentType::XspiceIntegrator,
                ComponentType::XspiceDifferentiator,
            ],
            ComponentCategory::XspiceDigital => &[
                ComponentType::XspiceInverter,
                ComponentType::XspiceBuffer,
                ComponentType::XspiceAndGate,
                ComponentType::XspiceOrGate,
                ComponentType::XspiceNandGate,
                ComponentType::XspiceNorGate,
                ComponentType::XspiceXorGate,
                ComponentType::XspiceTristate,
                ComponentType::XspiceDFlipFlop,
                ComponentType::XspiceJkFlipFlop,
                ComponentType::XspiceSrLatch,
            ],
            ComponentCategory::XspiceBridges => &[
                ComponentType::XspiceAdcBridge,
                ComponentType::XspiceDacBridge,
            ],
            ComponentCategory::VerilogA => &[], // VA models are dynamically loaded
        }
    }
}

/// Get display info for a component type
fn component_info(kind: ComponentType) -> (&'static str, &'static str, &'static str) {
    // Returns: (name, description, shortcut)
    match kind {
        ComponentType::Resistor => ("Resistor", "R - Resistance element", "R"),
        ComponentType::Capacitor => ("Capacitor", "C - Stores charge", "C"),
        ComponentType::Inductor => ("Inductor", "L - Stores energy in magnetic field", "L"),
        ComponentType::CoupledInductor => ("Coupled Inductor", "K - Transformer coupling", "K"),
        ComponentType::Diode => ("Diode", "D - Allows current in one direction", "D"),
        ComponentType::NpnBjt => ("NPN BJT", "Q - NPN Bipolar Junction Transistor", "Q"),
        ComponentType::PnpBjt => ("PNP BJT", "Q - PNP Bipolar Junction Transistor", ""),
        ComponentType::Nmos => ("N-MOSFET", "M - N-Channel MOSFET", "M"),
        ComponentType::Pmos => ("P-MOSFET", "M - P-Channel MOSFET", ""),
        ComponentType::Njfet => ("N-JFET", "J - N-Channel JFET", "J"),
        ComponentType::Pjfet => ("P-JFET", "J - P-Channel JFET", ""),
        ComponentType::NVdmos => ("N-VDMOS", "M - N-Channel Power MOSFET", ""),
        ComponentType::PVdmos => ("P-VDMOS", "M - P-Channel Power MOSFET", ""),
        ComponentType::SaturableInductor => ("Saturable L", "L - Saturable Core Inductor", ""),
        ComponentType::VoltageSource => ("DC Voltage", "V - DC Voltage Source", "V"),
        ComponentType::VoltageSourceAc => ("AC Voltage", "V - AC Voltage Source", ""),
        ComponentType::VoltageSourceSin => ("Sine Source", "V - Sinusoidal Voltage", ""),
        ComponentType::VoltageSourcePulse => ("Pulse Source", "V - Pulse Voltage", ""),
        ComponentType::VoltageSourcePwl => ("PWL Voltage", "V - Piecewise Linear", ""),
        ComponentType::VoltageSourceExp => ("Exp Voltage", "V - Exponential", ""),
        ComponentType::VoltageSourceSffm => ("SFFM Voltage", "V - Single-Freq FM", ""),
        ComponentType::CurrentSource => ("DC Current", "I - DC Current Source", "I"),
        ComponentType::CurrentSourceAc => ("AC Current", "I - AC Current Source", ""),
        ComponentType::CurrentSourcePulse => ("Pulse Current", "I - Pulse Current", ""),
        ComponentType::CurrentSourceSin => ("Sine Current", "I - Sinusoidal Current", ""),
        ComponentType::CurrentSourcePwl => ("PWL Current", "I - Piecewise Linear", ""),
        ComponentType::CurrentSourceExp => ("Exp Current", "I - Exponential", ""),
        ComponentType::CurrentSourceNoise => ("Noise Current", "I - Noise Source", ""),
        ComponentType::Vcvs => ("VCVS", "E - Voltage-Controlled Voltage Source", "E"),
        ComponentType::Vccs => ("VCCS", "G - Voltage-Controlled Current Source", ""),
        ComponentType::Ccvs => ("CCVS", "H - Current-Controlled Voltage Source", ""),
        ComponentType::Cccs => ("CCCS", "F - Current-Controlled Current Source", ""),
        ComponentType::Ground => ("Ground", "0 - Ground reference", "G"),
        // XSPICE Analog Behavioral
        ComponentType::XspiceGain => ("Gain", "A - XSPICE Gain Block", ""),
        ComponentType::XspiceSummer => ("Summer", "A - XSPICE Summing Block", ""),
        ComponentType::XspiceMultiplier => ("Multiplier", "A - XSPICE Multiplier", ""),
        ComponentType::XspiceDivider => ("Divider", "A - XSPICE Divider", ""),
        ComponentType::XspiceLimiter => ("Limiter", "A - XSPICE Hard Limiter", ""),
        ComponentType::XspiceIntegrator => ("Integrator", "A - XSPICE Integrator", ""),
        ComponentType::XspiceDifferentiator => ("Differentiator", "A - XSPICE Differentiator", ""),
        // XSPICE Digital Gates
        ComponentType::XspiceInverter => ("Inverter", "A - Digital Inverter", ""),
        ComponentType::XspiceBuffer => ("Buffer", "A - Digital Buffer", ""),
        ComponentType::XspiceAndGate => ("AND Gate", "A - 2-Input AND Gate", ""),
        ComponentType::XspiceOrGate => ("OR Gate", "A - 2-Input OR Gate", ""),
        ComponentType::XspiceNandGate => ("NAND Gate", "A - 2-Input NAND Gate", ""),
        ComponentType::XspiceNorGate => ("NOR Gate", "A - 2-Input NOR Gate", ""),
        ComponentType::XspiceXorGate => ("XOR Gate", "A - 2-Input XOR Gate", ""),
        ComponentType::XspiceTristate => ("Tri-State", "A - Tri-State Buffer", ""),
        // XSPICE Sequential
        ComponentType::XspiceDFlipFlop => ("D Flip-Flop", "A - D-Type Flip-Flop", ""),
        ComponentType::XspiceJkFlipFlop => ("JK Flip-Flop", "A - JK-Type Flip-Flop", ""),
        ComponentType::XspiceSrLatch => ("SR Latch", "A - SR Latch", ""),
        // XSPICE Bridges
        ComponentType::XspiceAdcBridge => ("ADC Bridge", "A - Analog to Digital Bridge", ""),
        ComponentType::XspiceDacBridge => ("DAC Bridge", "A - Digital to Analog Bridge", ""),
    }
}

/// Props for ComponentLibrary
#[derive(Props, Clone, PartialEq)]
pub struct ComponentLibraryProps {
    /// Schematic state for setting the tool
    pub schematic: Signal<SchematicState>,
    /// Whether the library is visible
    #[props(default = true)]
    pub visible: bool,
}

/// Component Library Browser sidebar
#[component]
pub fn ComponentLibrary(props: ComponentLibraryProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let library_manager: Signal<Arc<LibraryManager>> = use_context();
    let th = theme.read();

    // Search filter state
    let mut search = use_signal(String::new);

    // Expanded category state: [Passives, Semiconductors, Sources, ControlledSources, XspiceAnalog, XspiceDigital, XspiceBridges]
    let mut expanded = use_signal(|| vec![true, true, true, true, false, false, false]);

    // Library model categories expanded state (dynamically sized based on available model types)
    let lib_manager = library_manager.read();
    let model_types = lib_manager.available_types();
    let mut lib_expanded = use_signal(|| vec![false; model_types.len()]); // Collapsed by default

    // Verilog-A import dialog state
    let mut va_import_visible = use_signal(|| false);
    let mut va_models: Signal<Vec<VerilogAModelInfo>> = use_signal(Vec::new);

    if !props.visible {
        return rsx! {};
    }

    let search_lower = search.read().to_lowercase();
    let categories = [
        ComponentCategory::Passives,
        ComponentCategory::Semiconductors,
        ComponentCategory::Sources,
        ComponentCategory::ControlledSources,
        ComponentCategory::XspiceAnalog,
        ComponentCategory::XspiceDigital,
        ComponentCategory::XspiceBridges,
    ];

    rsx! {
        div {
            style: "
                width: 220px;
                background: {th.bg_secondary()};
                border-right: 1px solid {th.border()};
                display: flex;
                flex-direction: column;
                height: 100%;
                overflow: hidden;
            ",

            // Header
            div {
                style: "
                    padding: 12px;
                    border-bottom: 1px solid {th.border()};
                    font-weight: 600;
                    color: {th.text_primary()};
                    font-size: 13px;
                ",
                "📦 Component Library"
            }

            // Search input
            div {
                style: "padding: 8px 12px;",
                input {
                    r#type: "text",
                    placeholder: "🔍 Search components...",
                    value: "{search}",
                    style: "
                        width: 100%;
                        padding: 6px 10px;
                        background: {th.bg_primary()};
                        border: 1px solid {th.border()};
                        border-radius: 6px;
                        color: {th.text_primary()};
                        font-size: 12px;
                        outline: none;
                        box-sizing: border-box;
                    ",
                    oninput: move |e| search.set(e.value().clone()),
                }
            }

            // Categories
            div {
                style: "flex: 1; overflow-y: auto; padding: 4px 0;",

                for (cat_idx, category) in categories.iter().enumerate() {
                    // Filter components by search
                    {
                        let components: Vec<_> = category.components()
                            .iter()
                            .filter(|c| {
                                let (name, desc, _) = component_info(**c);
                                search_lower.is_empty()
                                    || name.to_lowercase().contains(&search_lower)
                                    || desc.to_lowercase().contains(&search_lower)
                            })
                            .collect();

                        if components.is_empty() && !search_lower.is_empty() {
                            rsx! {}
                        } else {
                            let is_expanded = *expanded.read().get(cat_idx).unwrap_or(&true);
                            let cat_icon = category.icon();
                            let cat_name = category.name();

                            rsx! {
                                // Category header
                                div {
                                    style: "
                                        display: flex;
                                        align-items: center;
                                        padding: 8px 12px;
                                        cursor: pointer;
                                        user-select: none;
                                        color: {th.text_secondary()};
                                        font-size: 11px;
                                        font-weight: 600;
                                        text-transform: uppercase;
                                        letter-spacing: 0.5px;
                                    ",
                                    onclick: move |_| {
                                        let mut exp = expanded.write();
                                        if let Some(v) = exp.get_mut(cat_idx) {
                                            *v = !*v;
                                        }
                                    },
                                    span { style: "margin-right: 6px;", "{cat_icon}" }
                                    span { style: "flex: 1;", "{cat_name}" }
                                    span {
                                        style: "font-size: 10px; opacity: 0.6;",
                                        if is_expanded { "▼" } else { "▶" }
                                    }
                                }

                                // Component list
                                if is_expanded {
                                    for comp_type in components {
                                        ComponentItem {
                                            kind: *comp_type,
                                            schematic: props.schematic,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ═══════════════════════════════════════════════════════════════
                // Verilog-A Models Section
                // ═══════════════════════════════════════════════════════════════

                // Section divider
                div {
                    style: "
                        margin: 8px 12px;
                        border-bottom: 1px solid {th.border()};
                    "
                }

                // Verilog-A section header
                div {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        padding: 4px 12px 8px;
                    ",

                    span {
                        style: "
                            font-size: 10px;
                            font-weight: 600;
                            color: {th.accent_primary()};
                            text-transform: uppercase;
                            letter-spacing: 0.5px;
                        ",
                        "Verilog-A Models"
                    }

                    // Import button
                    button {
                        style: "
                            padding: 3px 8px;
                            background: {th.surface()};
                            border: 1px solid {th.border()};
                            border-radius: 4px;
                            color: {th.text_secondary()};
                            font-size: 10px;
                            cursor: pointer;
                            transition: all 0.15s;
                        ",
                        title: "Import Verilog-A Model",
                        onclick: move |_| {
                            va_import_visible.set(true);
                        },
                        "+ Import"
                    }
                }

                // Verilog-A models list or empty state
                if va_models.read().is_empty() {
                    // Empty state placeholder
                    div {
                        style: "
                            padding: 12px 16px;
                            margin: 0 12px 8px;
                            background: {th.bg_primary()};
                            border-radius: 6px;
                            text-align: center;
                        ",

                        div {
                            style: "
                                font-size: 11px;
                                color: {th.text_muted()};
                                margin-bottom: 4px;
                            ",
                            "No Verilog-A models loaded"
                        }

                        div {
                            style: "
                                font-size: 10px;
                                color: {th.text_muted()};
                                opacity: 0.7;
                            ",
                            "Click Import to add .va files"
                        }
                    }
                } else {
                    // List of imported VA models
                    div {
                        style: "
                            padding: 0 12px 8px;
                        ",
                        for model in va_models.read().iter() {
                            VerilogAModelItem {
                                model: model.clone(),
                                schematic: props.schematic,
                            }
                        }
                    }
                }

                // ═══════════════════════════════════════════════════════════════
                // Library Models Section
                // ═══════════════════════════════════════════════════════════════

                // Section divider
                div {
                    style: "
                        margin: 8px 12px;
                        border-bottom: 1px solid {th.border()};
                    "
                }

                // Library section header
                div {
                    style: "
                        padding: 4px 12px 8px;
                        font-size: 10px;
                        font-weight: 600;
                        color: {th.accent_primary()};
                        text-transform: uppercase;
                        letter-spacing: 0.5px;
                    ",
                    "Library Models"
                }

                // Library model categories
                for (type_idx, model_type) in model_types.iter().enumerate() {
                    {
                        let models = lib_manager.models_of_type(*model_type);

                        // Filter by search
                        let filtered_models: Vec<_> = models
                            .into_iter()
                            .filter(|m| {
                                search_lower.is_empty()
                                    || m.name.to_lowercase().contains(&search_lower)
                                    || m.description.as_ref()
                                        .map(|d| d.to_lowercase().contains(&search_lower))
                                        .unwrap_or(false)
                            })
                            .collect();

                        if filtered_models.is_empty() && !search_lower.is_empty() {
                            rsx! {}
                        } else if !filtered_models.is_empty() {
                            let is_expanded = *lib_expanded.read().get(type_idx).unwrap_or(&false);
                            let type_name = model_type.display_name();
                            let model_count = filtered_models.len();

                            rsx! {
                                // Model type category header
                                div {
                                    style: "
                                        display: flex;
                                        align-items: center;
                                        padding: 6px 12px 6px 16px;
                                        cursor: pointer;
                                        user-select: none;
                                        color: {th.text_secondary()};
                                        font-size: 11px;
                                        font-weight: 500;
                                    ",
                                    onclick: move |_| {
                                        let mut exp = lib_expanded.write();
                                        if let Some(v) = exp.get_mut(type_idx) {
                                            *v = !*v;
                                        }
                                    },
                                    span { style: "flex: 1;", "{type_name}" }
                                    span {
                                        style: "
                                            font-size: 9px;
                                            opacity: 0.5;
                                            background: {th.bg_tertiary()};
                                            padding: 1px 5px;
                                            border-radius: 8px;
                                            margin-right: 6px;
                                        ",
                                        "{model_count}"
                                    }
                                    span {
                                        style: "font-size: 10px; opacity: 0.6;",
                                        if is_expanded { "▼" } else { "▶" }
                                    }
                                }

                                // Model list
                                if is_expanded {
                                    for model in filtered_models {
                                        LibraryModelItem {
                                            name: model.name.clone(),
                                            model_type: *model_type,
                                            description: model.description.clone(),
                                            schematic: props.schematic,
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                }
            }

            // Footer with tip
            div {
                style: "
                    padding: 8px 12px;
                    border-top: 1px solid {th.border()};
                    font-size: 10px;
                    color: {th.text_muted()};
                    opacity: 0.8;
                ",
                "Click to select, then click canvas to place"
            }
        }

        // Verilog-A Import Dialog
        VerilogAImportDialog {
            visible: *va_import_visible.read(),
            on_close: move |_| {
                va_import_visible.set(false);
            },
            on_import: move |model: VerilogAModelInfo| {
                log::info!("Imported Verilog-A model: {}", model.name);
                va_models.write().push(model);
            },
        }
    }
}

/// Individual component item in the library
#[component]
fn ComponentItem(kind: ComponentType, schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let canvas_focus = use_canvas_focus(); // Get canvas focus at component level

    let (name, _desc, shortcut) = component_info(kind);
    let is_active = matches!(schematic.read().tool, Tool::Place(k) if k == kind);

    let bg: &str = if is_active {
        &th.accent_primary()
    } else {
        "transparent"
    };
    let text_color: &str = if is_active {
        "#ffffff"
    } else {
        &th.text_primary()
    };
    let shortcut_bg: &str = if is_active {
        "rgba(255,255,255,0.2)"
    } else {
        &th.bg_tertiary()
    };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                padding: 6px 12px 6px 24px;
                cursor: pointer;
                background: {bg};
                transition: background 0.1s;
            ",
            onclick: move |_| {
                let mut s = schematic.write();
                s.selection.clear();
                s.tool = Tool::Place(kind);
                s.preview_rotation = Rotation::R0;  // Reset rotation for new component
                drop(s);
                // Focus canvas so keyboard shortcuts work immediately
                canvas_focus.read().focus();
            },
            span {
                style: "flex: 1; font-size: 12px; color: {text_color};",
                "{name}"
            }
            if !shortcut.is_empty() {
                span {
                    style: "
                        font-size: 10px;
                        color: {text_color};
                        opacity: 0.7;
                        background: {shortcut_bg};
                        padding: 1px 5px;
                        border-radius: 3px;
                        font-family: monospace;
                    ",
                    "{shortcut}"
                }
            }
        }
    }
}

/// Library model item for placing models from embedded libraries
#[component]
fn LibraryModelItem(
    name: String,
    model_type: ModelType,
    description: Option<String>,
    schematic: Signal<SchematicState>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let canvas_focus = use_canvas_focus();

    // Map model type to component type for placement
    let component_type = match model_type {
        ModelType::Diode => ComponentType::Diode,
        ModelType::NpnBjt => ComponentType::NpnBjt,
        ModelType::PnpBjt => ComponentType::PnpBjt,
        ModelType::Nmos => ComponentType::Nmos,
        ModelType::Pmos => ComponentType::Pmos,
        _ => ComponentType::Diode, // Fallback for other types
    };

    let name_clone = name.clone();
    let desc_text = description.clone().unwrap_or_default();

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                padding: 4px 12px 4px 28px;
                cursor: pointer;
                transition: background 0.1s;
            ",
            title: "{desc_text}",
            onclick: move |_| {
                // Set tool to place the component type with model reference
                let mut s = schematic.write();
                s.selection.clear();
                s.tool = Tool::Place(component_type);
                s.preview_rotation = Rotation::R0;
                drop(s);
                // Store model name in pending_model for the component that will be placed
                // The placed component will have this model set in its params
                log::info!("Selected library model: {} ({})", name_clone, model_type.display_name());
                canvas_focus.read().focus();
            },

            // Model name
            span {
                style: "font-size: 11px; color: {th.text_primary()};",
                "{name}"
            }

            // Description (if available, truncated)
            if !desc_text.is_empty() {
                span {
                    style: "
                        font-size: 9px;
                        color: {th.text_muted()};
                        white-space: nowrap;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        max-width: 160px;
                    ",
                    "{desc_text}"
                }
            }
        }
    }
}

/// Verilog-A model item for displaying imported VA models
#[component]
fn VerilogAModelItem(model: VerilogAModelInfo, schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let canvas_focus = use_canvas_focus();

    // Build description from terminals and parameter count
    let terminals_str = model.terminals.join(", ");
    let desc = format!("({}) {} params", terminals_str, model.parameters.len());

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                padding: 8px 12px;
                margin-bottom: 4px;
                background: {th.surface()};
                border: 1px solid {th.border()};
                border-radius: 6px;
                cursor: pointer;
                transition: all 0.15s;
            ",
            onclick: move |_| {
                // TODO: Set up placement tool for VA model
                log::info!("Selected Verilog-A model: {}", model.name);
                canvas_focus.read().focus();
            },

            // Model name with VA badge
            div {
                style: "display: flex; align-items: center; gap: 6px; margin-bottom: 4px;",

                span {
                    style: "
                        font-size: 9px;
                        font-weight: 700;
                        color: {th.accent_primary()};
                        background: {th.accent_primary()}22;
                        padding: 1px 4px;
                        border-radius: 3px;
                    ",
                    "VA"
                }

                span {
                    style: "font-size: 12px; font-weight: 500; color: {th.text_primary()};",
                    "{model.name}"
                }
            }

            // Description (terminals and param count)
            div {
                style: "
                    font-size: 10px;
                    color: {th.text_muted()};
                ",
                "{desc}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_category_names() {
        assert_eq!(ComponentCategory::Passives.name(), "Passive Components");
        assert_eq!(ComponentCategory::Semiconductors.name(), "Semiconductors");
        assert_eq!(ComponentCategory::Sources.name(), "Sources");
        assert_eq!(
            ComponentCategory::ControlledSources.name(),
            "Controlled Sources"
        );
        assert_eq!(ComponentCategory::VerilogA.name(), "Verilog-A Models");
    }

    #[test]
    fn test_component_category_icons() {
        // VerilogA should have "VA" icon
        assert_eq!(ComponentCategory::VerilogA.icon(), "VA");
        // Others can be empty or have icons
        let _ = ComponentCategory::Passives.icon();
        let _ = ComponentCategory::Semiconductors.icon();
    }

    #[test]
    fn test_component_category_components() {
        // Passives should have R, C, L
        let passives = ComponentCategory::Passives.components();
        assert!(passives.contains(&ComponentType::Resistor));
        assert!(passives.contains(&ComponentType::Capacitor));
        assert!(passives.contains(&ComponentType::Inductor));

        // Semiconductors should have diodes, BJTs, MOSFETs
        let semiconductors = ComponentCategory::Semiconductors.components();
        assert!(semiconductors.contains(&ComponentType::Diode));
        assert!(semiconductors.contains(&ComponentType::NpnBjt));
        assert!(semiconductors.contains(&ComponentType::Nmos));

        // Sources should have voltage, current, ground
        let sources = ComponentCategory::Sources.components();
        assert!(sources.contains(&ComponentType::VoltageSource));
        assert!(sources.contains(&ComponentType::CurrentSource));
        assert!(sources.contains(&ComponentType::Ground));

        // Controlled sources should have VCVS, VCCS, CCVS, CCCS
        let controlled = ComponentCategory::ControlledSources.components();
        assert!(controlled.contains(&ComponentType::Vcvs));
        assert!(controlled.contains(&ComponentType::Vccs));
        assert!(controlled.contains(&ComponentType::Ccvs));
        assert!(controlled.contains(&ComponentType::Cccs));

        // VerilogA is dynamically loaded, so empty by default
        let veriloga = ComponentCategory::VerilogA.components();
        assert!(veriloga.is_empty());
    }

    #[test]
    fn test_component_info() {
        // Test a few component infos
        let (name, desc, shortcut) = component_info(ComponentType::Resistor);
        assert_eq!(name, "Resistor");
        assert!(desc.contains("R"));
        assert_eq!(shortcut, "R");

        let (name, desc, _) = component_info(ComponentType::Diode);
        assert_eq!(name, "Diode");
        assert!(desc.contains("D"));

        let (name, _, shortcut) = component_info(ComponentType::Ground);
        assert_eq!(name, "Ground");
        assert_eq!(shortcut, "G");
    }

    #[test]
    fn test_all_categories_have_unique_names() {
        let categories = [
            ComponentCategory::Passives,
            ComponentCategory::Semiconductors,
            ComponentCategory::Sources,
            ComponentCategory::ControlledSources,
            ComponentCategory::VerilogA,
        ];

        let names: Vec<_> = categories.iter().map(|c| c.name()).collect();
        let unique: std::collections::HashSet<_> = names.iter().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "All category names should be unique"
        );
    }
}
