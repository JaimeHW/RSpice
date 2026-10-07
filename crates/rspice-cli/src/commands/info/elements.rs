//! Stable inspection data shared by detailed text and JSON output.
//!
//! These are parsed specifications, not evaluated device operating points.
//! Exhaustive matches make additions to the core vocabulary visible here.

use super::parameters::{instance_parameters, scalar_parameters};
use rspice_core::netlist::*;
use serde::Serialize;
use serde_json::{Value, json};
use std::io::Write;

mod sources;

macro_rules! record {
    ($kind:literal $(, $field:ident)* $(,)?) => {
        json!({"kind": $kind, $(stringify!($field): $field),*})
    };
}
pub(super) use record;

#[derive(Serialize)]
pub(super) struct ElementDetails<'a> {
    name: &'a str,
    nodes: &'a [String],
    specification: Value,
}

impl<'a> ElementDetails<'a> {
    pub(super) fn new(element: &'a Element) -> Self {
        use ElementKind::*;
        let specification = match &element.kind {
            Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            }
            | Capacitor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
                ..
            }
            | Inductor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
                ..
            } => {
                // A NaN or expression placeholder is not an evaluated value.
                let value = parsed_value(*value, value_expr);
                let parameters = scalar_parameters(instance_params, deferred_params, &[]);
                let mut report = record!("resistor", value, value_expr, model, parameters);
                match &element.kind {
                    Capacitor {
                        initial_voltage, ..
                    } => {
                        report["kind"] = json!("capacitor");
                        report["initial_voltage"] = json!(initial_voltage);
                    }
                    Inductor {
                        initial_current, ..
                    } => {
                        report["kind"] = json!("inductor");
                        report["initial_current"] = json!(initial_current);
                    }
                    _ => {}
                }
                report
            }
            JilesAthertonInductor {
                value,
                model,
                initial_current,
            } => record!("jiles_atherton_inductor", value, model, initial_current),
            VoltageSource(source) => {
                json!({"kind": "voltage_source", "source": sources::describe(source)})
            }
            CurrentSource(source) => {
                json!({"kind": "current_source", "source": sources::describe(source)})
            }
            VoltageSourceDeferred(source_expression) => {
                record!("voltage_source", source_expression)
            }
            CurrentSourceDeferred(source_expression) => {
                record!("current_source", source_expression)
            }
            RfPortDeferred {
                source: source_expression,
                multiplicity,
                line: _,
            } => record!("rf_port", source_expression, multiplicity),
            Diode {
                model,
                instance_params,
                deferred_params,
            }
            | Bjt {
                model,
                instance_params,
                deferred_params,
                ..
            }
            | Mosfet {
                model,
                instance_params,
                deferred_params,
                ..
            }
            | Jfet {
                model,
                instance_params,
                deferred_params,
                ..
            }
            | Mesfet {
                model,
                instance_params,
                deferred_params,
                ..
            }
            | XyceMemristor {
                model,
                instance_params,
                deferred_params,
            } => {
                let parameters = scalar_parameters(instance_params, deferred_params, &[]);
                let mut report = record!("diode", model, parameters);
                let (kind, polarity) = match &element.kind {
                    Bjt { bjt_type, .. } => (
                        "bjt",
                        Some(match bjt_type {
                            BjtType::Npn => "npn",
                            BjtType::Pnp => "pnp",
                        }),
                    ),
                    Mosfet {
                        mos_type,
                        compact_syntax,
                        ..
                    } => {
                        report["compact_syntax"] = json!(compact_syntax);
                        (
                            "mosfet",
                            Some(match mos_type {
                                MosType::Nmos => "nmos",
                                MosType::Pmos => "pmos",
                            }),
                        )
                    }
                    Jfet { jfet_type, .. } => (
                        "jfet",
                        Some(match jfet_type {
                            JfetType::Njf => "njf",
                            JfetType::Pjf => "pjf",
                        }),
                    ),
                    Mesfet { mesfet_type, .. } => (
                        "mesfet",
                        Some(match mesfet_type {
                            MesfetType::Nmf => "nmf",
                            MesfetType::Pmf => "pmf",
                        }),
                    ),
                    XyceMemristor { .. } => ("memristor", None),
                    _ => ("diode", None),
                };
                report["kind"] = json!(kind);
                if let Some(polarity) = polarity {
                    report["polarity"] = json!(polarity);
                }
                report
            }
            Vcvs {
                gain,
                gain_expr,
                control_nodes,
            } => {
                let gain = parsed_value(*gain, gain_expr);
                record!("vcvs", gain, gain_expr, control_nodes)
            }
            Cccs {
                gain,
                gain_expr,
                control_element,
            } => {
                let gain = parsed_value(*gain, gain_expr);
                record!("cccs", gain, gain_expr, control_element)
            }
            Vccs {
                transconductance,
                transconductance_expr,
                multiplicity,
                control_nodes,
            } => {
                let transconductance = parsed_value(*transconductance, transconductance_expr);
                let multiplicity = describe_multiplicity(multiplicity);
                record!(
                    "vccs",
                    transconductance,
                    transconductance_expr,
                    multiplicity,
                    control_nodes
                )
            }
            Ccvs {
                transresistance,
                transresistance_expr,
                control_element,
            } => {
                let transresistance = parsed_value(*transresistance, transresistance_expr);
                record!(
                    "ccvs",
                    transresistance,
                    transresistance_expr,
                    control_element
                )
            }
            PspiceChebyshev {
                source_line: _,
                input_expression,
                filter_kind,
                frequencies_hz,
                ripple_db,
                stop_db,
                voltage_output,
                multiplicity,
            } => {
                let filter_kind = match filter_kind {
                    PspiceChebyshevKind::LowPass => "low_pass",
                    PspiceChebyshevKind::HighPass => "high_pass",
                    PspiceChebyshevKind::BandPass => "band_pass",
                    PspiceChebyshevKind::BandReject => "band_reject",
                };
                let frequencies_hz: Vec<_> = frequencies_hz.iter().map(parametric_value).collect();
                let ripple_db = parametric_value(ripple_db);
                let stop_db = parametric_value(stop_db);
                let multiplicity = describe_multiplicity(multiplicity);
                record!(
                    "chebyshev_source",
                    input_expression,
                    filter_kind,
                    frequencies_hz,
                    ripple_db,
                    stop_db,
                    voltage_output,
                    multiplicity
                )
            }
            BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            }
            | BehavioralCurrent {
                expression,
                tc1,
                tc2,
                multiplicity,
            } => {
                let kind = if matches!(&element.kind, BehavioralVoltage { .. }) {
                    "behavioral_voltage"
                } else {
                    "behavioral_current"
                };
                let multiplicity = describe_multiplicity(multiplicity);
                json!({"kind":kind, "expression":expression, "tc1":tc1, "tc2":tc2, "multiplicity":multiplicity})
            }
            VSwitch {
                control_pos,
                control_neg,
                model,
                initial_state,
            } => {
                let initial_state = switch_state(initial_state);
                record!(
                    "voltage_switch",
                    control_pos,
                    control_neg,
                    model,
                    initial_state
                )
            }
            ISwitch {
                control_element,
                model,
                initial_state,
            } => {
                let initial_state = switch_state(initial_state);
                record!("current_switch", control_element, model, initial_state)
            }
            GenericSwitch {
                model,
                control_expression,
                initial_state,
            } => {
                let initial_state = switch_state(initial_state);
                record!("generic_switch", model, control_expression, initial_state)
            }
            TransmissionLine {
                z0,
                td,
                freq,
                nl,
                model,
            } => record!("transmission_line", z0, td, freq, nl, model),
            Coupling {
                inductors,
                coefficient,
                model,
            } => record!("coupling", inductors, coefficient, model),
            Subcircuit {
                subckt_name,
                params,
            } => {
                let parameters = instance_parameters(params);
                record!("subcircuit", subckt_name, parameters)
            }
            Xspice {
                model,
                pspice_u_timing,
                ports,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            } => {
                let parameters = scalar_parameters(params, expr_params, string_params);
                let ports: Vec<_> = ports.iter().map(describe_port).collect();
                let pspice_timing = pspice_u_timing.as_ref().map(|timing| json!({
                    "model": timing.timing_model,
                    "mode": match timing.delay_mode { PspiceUTimingMode::Min => "min", PspiceUTimingMode::Typ => "typ", PspiceUTimingMode::Max => "max" },
                    "power_pins": timing.power_pins,
                }));
                record!(
                    "xspice",
                    model,
                    ports,
                    parameters,
                    string_expr_params,
                    string_vector_params,
                    string_vector_expr_params,
                    real_vector_params,
                    real_vector_expr_params,
                    pspice_timing
                )
            }
        };
        Self {
            name: &element.name,
            nodes: &element.nodes,
            specification,
        }
    }

    pub(super) fn write(&self, out: &mut impl Write, indent: usize) -> std::io::Result<()> {
        let specification = self
            .specification
            .as_object()
            .expect("inspection specification is an object");
        writeln!(
            out,
            "{:indent$}{} ({}): {}",
            "",
            self.name,
            specification["kind"].as_str().unwrap_or("unknown"),
            self.nodes.join(" ")
        )?;
        for (name, value) in specification {
            if name == "kind" || value.is_null() || value.as_array().is_some_and(Vec::is_empty) {
                continue;
            }
            match value {
                Value::String(value) => writeln!(out, "{:indent$}  {name}: {value}", "")?,
                _ => writeln!(out, "{:indent$}  {name}: {value}", "")?,
            }
        }
        Ok(())
    }
}

fn parsed_value(value: f64, expression: &Option<String>) -> Option<f64> {
    (expression.is_none() && value.is_finite()).then_some(value)
}

fn describe_multiplicity(multiplicity: &SourceMultiplicity) -> Value {
    json!({"value": parsed_value(multiplicity.value, &multiplicity.value_expr), "expression": multiplicity.value_expr, "given": multiplicity.given})
}

fn parametric_value(value: &ParametricValue) -> Value {
    match value {
        ParametricValue::Resolved(value) => record!("real", value),
        ParametricValue::Expression(value) => record!("expression", value),
        ParametricValue::String(value) => record!("string", value),
        ParametricValue::StringExpression(value) => record!("string_expression", value),
    }
}

fn switch_state(state: &Option<SwitchState>) -> Option<&'static str> {
    state.map(|state| match state {
        SwitchState::On => "on",
        SwitchState::Off => "off",
    })
}

fn describe_port(port: &XspicePort) -> Value {
    use XspicePort::*;
    match port {
        Analog(node) => record!("analog", node),
        ExplicitVoltage(node) => record!("explicit_voltage", node),
        Digital(node) => record!("digital", node),
        ExplicitDigital(node) => record!("explicit_digital", node),
        DigitalInverted(node) => record!("digital_inverted", node),
        AnalogVector(nodes) => record!("analog_vector", nodes),
        DigitalVector(nodes) => {
            let nodes: Vec<_> = nodes
                .iter()
                .map(|name| json!({"name":name, "inverted":false}))
                .collect();
            record!("digital_vector", nodes)
        }
        DigitalVectorMixed(nodes) => {
            let nodes: Vec<_> = nodes
                .iter()
                .map(|node| json!({"name":node.name, "inverted":node.inverted}))
                .collect();
            record!("digital_vector", nodes)
        }
        Conductance(node) => record!("conductance", node),
        Current(node) => record!("current", node),
        VoltageName(element) => record!("voltage_name", element),
        DifferentialVoltage { pos, neg } => record!("differential_voltage", pos, neg),
        DifferentialCurrent { pos, neg } => record!("differential_current", pos, neg),
        DifferentialConductance { pos, neg } => record!("differential_conductance", pos, neg),
        Hybrid(node) => record!("hybrid", node),
        DifferentialHybrid { pos, neg } => record!("differential_hybrid", pos, neg),
        Null => record!("null"),
    }
}
