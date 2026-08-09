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
    defaults: &'static [(&'static str, &'static str)],
}

fn xspice_spec(kind: ComponentType) -> Option<XspiceSpec> {
    let spec = match kind {
        ComponentType::XspiceGain => XspiceSpec {
            defaults: &[("gain", "1"), ("in_offset", "0"), ("out_offset", "0")],
        },
        ComponentType::XspiceSummer => XspiceSpec {
            defaults: &[("out_gain", "1"), ("out_offset", "0")],
        },
        ComponentType::XspiceMultiplier => XspiceSpec {
            defaults: &[("out_gain", "1"), ("out_offset", "0")],
        },
        ComponentType::XspiceDivider => XspiceSpec {
            defaults: &[
                ("num_gain", "1"),
                ("den_gain", "1"),
                ("den_lower_limit", "1e-10"),
            ],
        },
        // `limit`, `int`, and `d_dt` REQUIRE both output limits on the card.
        ComponentType::XspiceLimiter => XspiceSpec {
            defaults: &[
                ("gain", "1"),
                ("out_lower_limit", "-1"),
                ("out_upper_limit", "1"),
                ("limit_range", "1e-6"),
            ],
        },
        ComponentType::XspiceIntegrator => XspiceSpec {
            defaults: &[
                ("gain", "1"),
                ("out_lower_limit", "-1e12"),
                ("out_upper_limit", "1e12"),
                ("out_ic", "0"),
            ],
        },
        ComponentType::XspiceDifferentiator => XspiceSpec {
            defaults: &[
                ("gain", "1"),
                ("out_lower_limit", "-1e12"),
                ("out_upper_limit", "1e12"),
            ],
        },
        ComponentType::XspiceBuffer => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceInverter => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceAndGate => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceOrGate => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceNandGate => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceNorGate => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceXorGate => XspiceSpec {
            defaults: &[("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceTristate => XspiceSpec {
            defaults: &[("delay", "1n")],
        },
        ComponentType::XspiceDFlipFlop => XspiceSpec {
            defaults: &[("ic", "0"), ("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceJkFlipFlop => XspiceSpec {
            defaults: &[("ic", "0"), ("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceSrLatch => XspiceSpec {
            defaults: &[("ic", "0"), ("rise_delay", "1n"), ("fall_delay", "1n")],
        },
        ComponentType::XspiceAdcBridge => XspiceSpec {
            defaults: &[("in_low", "1.0"), ("in_high", "2.0")],
        },
        ComponentType::XspiceDacBridge => XspiceSpec {
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
    /// Emit a registry-backed XSPICE catalog placement. The persisted binding
    /// owns exact terminal-to-port shaping; the live registry signature must
    /// still match before execution is allowed.
    pub(super) fn generate_builtin_xspice_instance(
        &mut self,
        component: &Component,
        binding: &crate::state::LibraryCellInstance,
        node_names: &[String],
        terminal_points: &[Point],
        instance_name: &str,
    ) -> Option<String> {
        let Some(contract) = binding.builtin_xspice.as_ref() else {
            self.errors.push(format!(
                "Cell instance '{}' reached the built-in XSPICE emitter without a contract",
                component.name
            ));
            return None;
        };
        if let Err(error) = crate::state::validate_builtin_xspice_binding(binding) {
            self.errors.push(format!(
                "Built-in XSPICE instance '{}' is not executable: {error}",
                component.name
            ));
            return None;
        }
        if node_names.len() != binding.terminal_order.len()
            || terminal_points.len() != node_names.len()
        {
            self.errors.push(format!(
                "Built-in XSPICE instance '{}' terminal mismatch: schematic has {} nodes but its frozen interface defines {} terminals",
                component.name,
                node_names.len(),
                binding.terminal_order.len()
            ));
            return None;
        }

        let terminal_connected = terminal_points
            .iter()
            .map(|point| self.catalog_terminal_is_connected(component.id, *point))
            .collect::<Vec<_>>();
        let mut emitted_ports = Vec::with_capacity(contract.ports.len());
        for port in &contract.ports {
            let mut nodes = Vec::with_capacity(port.terminals.len());
            let mut any_connected = false;
            for &terminal in &port.terminals {
                let Some(node) = node_names.get(terminal) else {
                    self.errors.push(format!(
                        "Built-in XSPICE instance '{}' port '{}' references missing terminal {}",
                        component.name, port.name, terminal
                    ));
                    return None;
                };
                nodes.push(node.as_str());
                any_connected |= terminal_connected.get(terminal).copied().unwrap_or(false);
            }
            if port.null_allowed && !any_connected {
                emitted_ports.push("null".to_owned());
                continue;
            }
            match format_builtin_xspice_port(port, &nodes) {
                Ok(value) => emitted_ports.push(value),
                Err(error) => {
                    self.errors.push(format!(
                        "Built-in XSPICE instance '{}' port '{}': {error}",
                        component.name, port.name
                    ));
                    return None;
                }
            }
        }

        let registry = rspice_core::xspice::CodeModelRegistry::with_builtins();
        let Some(model) = registry.get(&contract.model_type) else {
            self.errors.push(format!(
                "Built-in XSPICE model '{}' required by '{}' is unavailable in this build",
                contract.model_type, component.name
            ));
            return None;
        };
        let parameters = match format_builtin_xspice_parameters(
            model.parameters(),
            &binding.parameter_order,
            &component.params,
        ) {
            Ok(parameters) => parameters,
            Err(error) => {
                self.errors.push(format!(
                    "Built-in XSPICE instance '{}' has invalid model parameters: {error}",
                    component.name
                ));
                return None;
            }
        };
        let model_name = format!("{}_model", instance_name.to_ascii_lowercase());
        if !self.models.contains_key(&model_name) {
            let card = if parameters.is_empty() {
                format!(".MODEL {model_name} {}", contract.model_type)
            } else {
                format!(".MODEL {model_name} {} ({parameters})", contract.model_type)
            };
            self.models.insert(model_name.clone(), card);
        }
        Some(format!(
            "{} {} {}",
            instance_name,
            emitted_ports.join(" "),
            model_name
        ))
    }

    fn catalog_terminal_is_connected(&self, component_id: u64, point: Point) -> bool {
        let Some(net_id) = self.point_to_net.get(&point).copied() else {
            return false;
        };
        if self
            .nets
            .iter()
            .find(|net| net.id == net_id)
            .is_some_and(|net| net.points.len() > 1 || net.label.is_some())
        {
            return true;
        }
        self.schematic.components.iter().any(|candidate| {
            candidate.id != component_id
                && self
                    .component_terminal_positions(candidate)
                    .iter()
                    .any(|(_, candidate_point)| *candidate_point == point)
        })
    }

    /// Emit an XSPICE `A` instance plus its `.MODEL` card.
    pub(super) fn generate_xspice_instance(
        &mut self,
        component: &Component,
        node_names: &[String],
        instance_name: &str,
    ) -> Option<String> {
        let crate::state::DeviceImplementation::Xspice { model_type } =
            component.kind.descriptor().implementation
        else {
            self.errors.push(format!(
                "{} '{}' reached the XSPICE emitter without an XSPICE device descriptor",
                component.kind.display_name(),
                component.name
            ));
            return None;
        };
        let Some(spec) = xspice_spec(component.kind) else {
            self.errors.push(format!(
                "{} '{}' has no XSPICE parameter schema",
                component.kind.display_name(),
                component.name
            ));
            return None;
        };
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
            let user_params = crate::state::parse_params_string(&component.params);
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
                format!(".MODEL {} {} ({})", model_name, model_type, card_params),
            );
        }

        Some(format!("{} {} {}", instance_name, ports, model_name))
    }
}

fn format_builtin_xspice_parameters(
    specifications: &[rspice_core::xspice::ParamSpec],
    parameter_order: &[String],
    raw: &str,
) -> Result<String, String> {
    use rspice_core::xspice::ParamType;

    let parsed = crate::state::parse_replacement_parameters_strict(raw)
        .map_err(|error| error.to_string())?;
    for key in parsed.keys() {
        if !specifications
            .iter()
            .any(|parameter| parameter.name.eq_ignore_ascii_case(key))
        {
            return Err(format!(
                "parameter `{key}` is not declared by the code model"
            ));
        }
    }

    let mut formatted = Vec::new();
    for ordered_name in parameter_order {
        let Some(specification) = specifications
            .iter()
            .find(|parameter| parameter.name.eq_ignore_ascii_case(ordered_name))
        else {
            return Err(format!(
                "frozen parameter `{ordered_name}` is absent from the code-model registry"
            ));
        };
        let value = parsed
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(ordered_name))
            .map(|(_, value)| value.as_str());
        let Some(value) = value else {
            if specification.required {
                return Err(format!("required parameter `{ordered_name}` is missing"));
            }
            continue;
        };
        if value.len() > 4096 || value.chars().any(char::is_control) {
            return Err(format!(
                "parameter `{ordered_name}` is too long or contains controls"
            ));
        }
        let decoded = decode_persisted_parameter_value(value)?;
        let decoded = decoded.trim();
        if decoded.is_empty() {
            return Err(format!("parameter `{ordered_name}` is empty"));
        }
        let emitted = match specification.param_type {
            ParamType::String => quote_xspice_string(decoded),
            ParamType::StringVector
            | ParamType::RealVector
            | ParamType::IntegerVector
            | ParamType::ComplexVector => {
                if !(decoded.starts_with('[') && decoded.ends_with(']')) {
                    return Err(format!(
                        "parameter `{ordered_name}` must use bracketed vector syntax"
                    ));
                }
                decoded.to_owned()
            }
            ParamType::Complex => {
                if !(decoded.starts_with('<') && decoded.ends_with('>')) {
                    return Err(format!(
                        "parameter `{ordered_name}` must use <real imaginary> complex syntax"
                    ));
                }
                decoded.to_owned()
            }
            ParamType::Real | ParamType::Integer | ParamType::Boolean => decoded.to_owned(),
        };
        formatted.push(format!("{}={emitted}", specification.name));
    }
    Ok(formatted.join(" "))
}

fn decode_persisted_parameter_value(value: &str) -> Result<String, String> {
    let value = value.trim();
    let Some(quote) = value
        .chars()
        .next()
        .filter(|quote| matches!(quote, '\'' | '"'))
    else {
        return Ok(value.to_owned());
    };
    if value.len() < 2 || !value.ends_with(quote) {
        return Err("quoted parameter value is unterminated".to_owned());
    }
    let inner = &value[quote.len_utf8()..value.len() - quote.len_utf8()];
    let mut decoded = String::with_capacity(inner.len());
    let mut characters = inner.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            decoded.push(character);
            continue;
        }
        let escaped = characters
            .next()
            .ok_or_else(|| "quoted parameter value ends with an escape".to_owned())?;
        if escaped == quote || escaped == '\\' {
            decoded.push(escaped);
        } else {
            decoded.push('\\');
            decoded.push(escaped);
        }
    }
    Ok(decoded)
}

fn quote_xspice_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn format_builtin_xspice_port(
    port: &crate::state::BuiltinXspicePortBinding,
    nodes: &[&str],
) -> Result<String, String> {
    use crate::state::BuiltinXspicePortType as Type;

    if nodes.is_empty() {
        return Err("has no schematic terminals".to_owned());
    }
    let differential = matches!(
        port.port_type,
        Type::DifferentialVoltage
            | Type::DifferentialConductance
            | Type::DifferentialHybrid
            | Type::DifferentialCurrent
    );
    let element_width = if differential { 2 } else { 1 };
    let expected_terminals = port.vector_width.saturating_mul(element_width);
    if nodes.len() != expected_terminals {
        return Err(format!(
            "declares width {} but has {} terminal(s); expected {expected_terminals}",
            port.vector_width,
            nodes.len()
        ));
    }
    if nodes.len() % element_width != 0 {
        return Err(format!(
            "requires terminal groups of {element_width}, got {}",
            nodes.len()
        ));
    }
    if !port.is_vector && nodes.len() != element_width {
        return Err(format!(
            "is scalar but has {} terminal(s); expected {element_width}",
            nodes.len()
        ));
    }

    let joined = nodes.join(" ");
    if port.is_vector {
        let value = match port.port_type {
            Type::Voltage | Type::Digital | Type::Real => format!("[{joined}]"),
            Type::DifferentialVoltage => format!("%vd([{joined}])"),
            Type::Conductance => format!("%g([{joined}])"),
            Type::DifferentialConductance => format!("%gd([{joined}])"),
            Type::Hybrid => format!("%h([{joined}])"),
            Type::DifferentialHybrid => format!("%hd([{joined}])"),
            Type::Current => format!("%i([{joined}])"),
            Type::DifferentialCurrent => format!("%id([{joined}])"),
            Type::VoltageName => format!("%vnam([{joined}])"),
            Type::Integer | Type::UserDefined => {
                return Err(format!("uses unsupported {:?} event nodes", port.port_type));
            }
        };
        return Ok(value);
    }

    Ok(match port.port_type {
        Type::Voltage => format!("%v {}", nodes[0]),
        Type::DifferentialVoltage => format!("%vd({} {})", nodes[0], nodes[1]),
        Type::Conductance => format!("%g {}", nodes[0]),
        Type::DifferentialConductance => format!("%gd({} {})", nodes[0], nodes[1]),
        Type::Hybrid => format!("%h {}", nodes[0]),
        Type::DifferentialHybrid => format!("%hd({} {})", nodes[0], nodes[1]),
        Type::Current => format!("%i {}", nodes[0]),
        Type::DifferentialCurrent => format!("%id({} {})", nodes[0], nodes[1]),
        Type::VoltageName => format!("%vnam {}", nodes[0]),
        Type::Digital | Type::Real => nodes[0].to_owned(),
        Type::Integer | Type::UserDefined => {
            return Err(format!("uses unsupported {:?} event nodes", port.port_type));
        }
    })
}

#[cfg(test)]
mod descriptor_contract_tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn every_legacy_xspice_descriptor_has_an_emitter_schema() {
        for kind in ComponentType::ALL {
            let is_xspice_descriptor = matches!(
                kind.descriptor().implementation,
                crate::state::DeviceImplementation::Xspice { .. }
            );
            assert_eq!(
                xspice_spec(kind).is_some(),
                is_xspice_descriptor,
                "XSPICE descriptor/emitter drift for {}",
                kind.descriptor().stable_id
            );
        }
    }

    #[test]
    fn typed_catalog_parameters_survive_property_storage_and_core_parsing() {
        let registry = rspice_core::xspice::CodeModelRegistry::with_builtins();
        let model = registry.get("d_cosim").expect("d_cosim model");
        let order = model
            .parameters()
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect::<Vec<_>>();
        let values = HashMap::from([
            ("simulation".to_owned(), "ivlng simulator".to_owned()),
            (
                "sim_args".to_owned(),
                r#"["deck" "--payload=two words"]"#.to_owned(),
            ),
            ("queue_size".to_owned(), "32".to_owned()),
        ]);
        let stored = crate::state::format_params_string(&values);
        let emitted = format_builtin_xspice_parameters(model.parameters(), &order, &stored)
            .expect("typed parameters emit");
        assert!(emitted.contains("simulation=\"ivlng simulator\""));
        assert!(emitted.contains(r#"sim_args=["deck" "--payload=two words"]"#));
        assert!(emitted.contains("queue_size=32"));

        let deck = format!("typed catalog parameters\n.model co d_cosim ({emitted})\n.end\n");
        let parsed = rspice_core::Netlist::parse(&deck).expect("emitted model card parses");
        let parsed_model = parsed
            .models
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case("co"))
            .expect("parsed model");
        assert!(parsed_model.string_params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("simulation") && value == "ivlng simulator"
        }));
        assert!(
            parsed_model
                .string_vector_params
                .iter()
                .any(|(name, values)| {
                    name.eq_ignore_ascii_case("sim_args")
                        && values == &["deck".to_owned(), "--payload=two words".to_owned()]
                })
        );
    }

    #[test]
    fn required_catalog_parameter_is_never_filled_from_a_sentinel_default() {
        let registry = rspice_core::xspice::CodeModelRegistry::with_builtins();
        let model = registry.get("d_process").expect("d_process model");
        let order = model
            .parameters()
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect::<Vec<_>>();

        let error = format_builtin_xspice_parameters(model.parameters(), &order, "")
            .expect_err("process_file is required");
        assert!(error.contains("process_file"));
    }
}
