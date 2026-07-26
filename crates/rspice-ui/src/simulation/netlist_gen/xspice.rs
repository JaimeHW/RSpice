//! XSPICE (`A` element) instance and model-card emission.
//!
//! Every XSPICE block needs two things the generic instance path cannot
//! provide: a port list shaped to the code model's port specs (vector
//! ports in brackets, `null` for unconnected optional ports), and a
//! `.MODEL` card whose type is the registered code-model name. Digital
//! blocks are emitted on plain schematic nets — the core auto-inserts
//! ADC/DAC bridges on mixed-type nodes.

use super::*;

/// Per-kind emission spec: registered code-model type, and the default
/// model-card parameters (user `params` values override by key).
struct XspiceSpec {
    model_type: &'static str,
    defaults: &'static [(&'static str, &'static str)],
}

fn xspice_spec(kind: ComponentType) -> Option<XspiceSpec> {
    let spec = match kind {
        ComponentType::XspiceGain => XspiceSpec {
            model_type: "gain",
            defaults: &[("gain", "1"), ("in_offset", "0"), ("out_offset", "0")],
        },
        ComponentType::XspiceSummer => XspiceSpec {
            model_type: "summer",
            defaults: &[("out_gain", "1"), ("out_offset", "0")],
        },
        ComponentType::XspiceMultiplier => XspiceSpec {
            model_type: "mult",
            defaults: &[("out_gain", "1"), ("out_offset", "0")],
        },
        ComponentType::XspiceDivider => XspiceSpec {
            model_type: "divide",
            defaults: &[
                ("num_gain", "1"),
                ("den_gain", "1"),
                ("den_lower_limit", "1e-10"),
            ],
        },
        // `limit`, `int`, and `d_dt` REQUIRE both output limits on the card.
        ComponentType::XspiceLimiter => XspiceSpec {
            model_type: "limit",
            defaults: &[
                ("gain", "1"),
                ("out_lower_limit", "-1"),
                ("out_upper_limit", "1"),
                ("limit_range", "1e-6"),
            ],
        },
        ComponentType::XspiceIntegrator => XspiceSpec {
            model_type: "int",
            defaults: &[
                ("gain", "1"),
                ("out_lower_limit", "-1e12"),
                ("out_upper_limit", "1e12"),
                ("out_ic", "0"),
            ],
        },
        ComponentType::XspiceDifferentiator => XspiceSpec {
            model_type: "d_dt",
            defaults: &[
                ("gain", "1"),
                ("out_lower_limit", "-1e12"),
                ("out_upper_limit", "1e12"),
            ],
        },
        ComponentType::XspiceBuffer => XspiceSpec {
            model_type: "d_buffer",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceInverter => XspiceSpec {
            model_type: "d_inverter",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceAndGate => XspiceSpec {
            model_type: "d_and",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceOrGate => XspiceSpec {
            model_type: "d_or",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceNandGate => XspiceSpec {
            model_type: "d_nand",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceNorGate => XspiceSpec {
            model_type: "d_nor",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceXorGate => XspiceSpec {
            model_type: "d_xor",
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceTristate => XspiceSpec {
            model_type: "d_tristate",
            defaults: &[("delay", "1n")],
        },
        ComponentType::XspiceDFlipFlop => XspiceSpec {
            model_type: "d_dff",
            defaults: &[("ic", "0"), ("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceJkFlipFlop => XspiceSpec {
            model_type: "d_jkff",
            defaults: &[("ic", "0"), ("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceSrLatch => XspiceSpec {
            model_type: "d_srlatch",
            defaults: &[("ic", "0"), ("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceAdcBridge => XspiceSpec {
            model_type: "adc_bridge",
            defaults: &[("in_low", "1.0"), ("in_high", "2.0")],
        },
        ComponentType::XspiceDacBridge => XspiceSpec {
            model_type: "dac_bridge",
            defaults: &[("out_low", "0"), ("out_high", "3.3")],
        },
        _ => return None,
    };
    Some(spec)
}

/// Shape the port list to the code model's port specs. `nodes` follows
/// `terminal_offsets()` order.
fn xspice_ports(kind: ComponentType, nodes: &[String]) -> Option<String> {
    let ports = match kind {
        // Scalar analog in/out.
        ComponentType::XspiceGain
        | ComponentType::XspiceLimiter
        | ComponentType::XspiceIntegrator
        | ComponentType::XspiceDifferentiator => format!("{} {}", nodes[0], nodes[1]),
        // Vector analog input.
        ComponentType::XspiceSummer | ComponentType::XspiceMultiplier => {
            format!("[{} {}] {}", nodes[0], nodes[1], nodes[2])
        }
        // divide takes scalar num, den, out.
        ComponentType::XspiceDivider => format!("{} {} {}", nodes[0], nodes[1], nodes[2]),
        // Scalar digital in/out.
        ComponentType::XspiceBuffer | ComponentType::XspiceInverter => {
            format!("{} {}", nodes[0], nodes[1])
        }
        // Two-input digital vector gates.
        ComponentType::XspiceAndGate
        | ComponentType::XspiceOrGate
        | ComponentType::XspiceNandGate
        | ComponentType::XspiceNorGate
        | ComponentType::XspiceXorGate => {
            format!("[{} {}] [{}]", nodes[0], nodes[1], nodes[2])
        }
        // in, enable, out.
        ComponentType::XspiceTristate => format!("{} {} {}", nodes[0], nodes[1], nodes[2]),
        // data, clk, set*, reset*, out*, Nout* — set/reset unconnected.
        ComponentType::XspiceDFlipFlop => format!(
            "{} {} null null {} {}",
            nodes[0], nodes[1], nodes[2], nodes[3]
        ),
        // j, k, clk, set*, reset*, out*, Nout*.
        ComponentType::XspiceJkFlipFlop => format!(
            "{} {} {} null null {} {}",
            nodes[0], nodes[1], nodes[2], nodes[3], nodes[4]
        ),
        // s, r, enable, set*, reset*, out*, Nout*.
        ComponentType::XspiceSrLatch => format!(
            "{} {} {} null null {} {}",
            nodes[0], nodes[1], nodes[2], nodes[3], nodes[4]
        ),
        // Analog vector in, digital vector out (and vice versa).
        ComponentType::XspiceAdcBridge | ComponentType::XspiceDacBridge => {
            format!("[{}] [{}]", nodes[0], nodes[1])
        }
        _ => return None,
    };
    Some(ports)
}

impl<'a> NetlistGenerator<'a> {
    /// Emit an XSPICE `A` instance plus its `.MODEL` card.
    pub(super) fn generate_xspice_instance(
        &mut self,
        component: &Component,
        node_names: &[String],
        instance_name: &str,
    ) -> Option<String> {
        let spec = xspice_spec(component.kind)?;
        let expected = component.kind.terminal_count();
        if node_names.len() < expected {
            self.errors.push(format!(
                "{} '{}' is missing terminals ({} of {} connected)",
                component.kind.display_name(),
                component.name,
                node_names.len(),
                expected
            ));
            return None;
        }
        let ports = xspice_ports(component.kind, node_names)?;
        let model_name = format!("{}_model", instance_name.to_lowercase());

        if !self.models.contains_key(&model_name) {
            let user_params = crate::properties::parse_params_string(&component.params);
            let card_params = spec
                .defaults
                .iter()
                .map(|(key, default)| {
                    let value = user_params
                        .get(*key)
                        .map(String::as_str)
                        .filter(|v| !v.is_empty())
                        .unwrap_or(default);
                    format!("{}={}", key, value)
                })
                .collect::<Vec<_>>()
                .join(" ");
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} ({})", model_name, spec.model_type, card_params),
            );
        }

        Some(format!("{} {} {}", instance_name, ports, model_name))
    }
}
