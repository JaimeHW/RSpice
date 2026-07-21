use super::*;

impl<'a> NetlistGenerator<'a> {
    pub(super) fn generate_instances(&mut self) {
        self.lines.push("* Circuit netlist".to_string());

        for component in &self.schematic.components {
            if component.kind == ComponentType::Ground
                || component.kind == ComponentType::CoupledInductor
                || component.kind == ComponentType::Transformer
            {
                // Ground symbol is implicit (node 0)
                continue;
            }

            if let Some(line) = self.generate_instance_line(component) {
                self.lines.push(line);
            }
        }

        for line in self.collect_transformer_lines() {
            self.lines.push(line);
        }

        for line in self.collect_coupling_lines() {
            self.lines.push(line);
        }
    }

    /// Generate a single SPICE instance line
    fn generate_instance_line(&mut self, component: &Component) -> Option<String> {
        let terminals = self.component_terminal_positions(component);
        let node_names: Vec<String> = terminals
            .iter()
            .map(|(_, pos)| self.get_node_name(*pos))
            .collect();
        let instance_name = self.instance_name(component);

        match component.kind {
            // Two-terminal passive components: X name node+ node- value [params]
            // Spectre format: R1 net1 net2 1k m=2 tc1=0.01
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Diode => {
                let nodes = self.format_nodes(&node_names, 2);
                let value_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, value_with_params))
            }

            ComponentType::Inductor | ComponentType::SaturableInductor => {
                let nodes = self.format_nodes(&node_names, 2);
                let filtered_params = self
                    .filter_component_params(&component.params, &["coupled_to", "coupling_factor"]);
                let value_with_params =
                    self.format_value_with_params(&component.value, &filtered_params);
                Some(format!("{} {} {}", instance_name, nodes, value_with_params))
            }

            // Two-terminal voltage sources: V name node+ node- value [params]
            // Spectre format: V1 net1 0 DC 5 acmag=1 acphase=0
            ComponentType::VoltageSource | ComponentType::VoltageSourceAc => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                let params = self.format_params(&component.params);
                Some(format!(
                    "{} {} {}{}",
                    instance_name, nodes, source_value, params
                ))
            }
            // Voltage sources with positional params (SIN, PULSE, etc.) - no extra params needed
            ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                Some(format!("{} {} {}", instance_name, nodes, source_value))
            }

            // Two-terminal current sources: I name node+ node- value [params]
            // Spectre format: I1 net1 0 DC 1m acmag=1
            ComponentType::CurrentSource | ComponentType::CurrentSourceAc => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                let params = self.format_params(&component.params);
                Some(format!(
                    "{} {} {}{}",
                    instance_name, nodes, source_value, params
                ))
            }
            // Current sources with positional params (SIN, PULSE, etc.) - no extra params needed
            ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                Some(format!("{} {} {}", instance_name, nodes, source_value))
            }

            // Behavioral source: B name node+ node- V=<expr> | I=<expr>
            // The value carries the user's expression verbatim
            // (e.g. V=V(a)*sqrt(V(b)) or I=1m+V(in)/100).
            ComponentType::BehavioralSource => {
                let nodes = self.format_nodes(&node_names, 2);
                let expression = component.value.trim();
                let expression = if expression.is_empty() {
                    "V=0"
                } else {
                    expression
                };
                Some(format!("{} {} {}", instance_name, nodes, expression))
            }

            // Three-terminal BJT: Q name C B E model [params]
            // Spectre format: Q1 coll base emit npn_Q1 area=1 m=1
            ComponentType::NpnBjt | ComponentType::PnpBjt => {
                let nodes = self.format_nodes(&node_names, 3);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_bjt_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Four-terminal MOSFET: M name D G S B model [params]
            // Spectre format: M1 drain gate source bulk nmos_M1 w=1u l=180n as=1p ad=1p
            ComponentType::Nmos | ComponentType::Pmos => {
                let nodes = self.format_nodes(&node_names, 4);
                let model = self.get_mosfet_model(component);
                let params = self.format_params(&component.params);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Three-terminal JFET: J name D G S model [params]
            // Spectre format: J1 drain gate source njf_J1 area=1 m=1
            ComponentType::Njfet | ComponentType::Pjfet => {
                let nodes = self.format_nodes(&node_names, 3);
                let model = self.get_jfet_model(component);
                let params = self.format_params(&component.params);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Ideal op-amp (3 terminals: in+ in- out) — emitted as a
            // ground-referenced VCVS: E<name> <out> 0 <in+> <in-> <gain>
            ComponentType::OpAmp => {
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                (node_names.len() >= 3).then(|| {
                    format!(
                        "{} {} 0 {} {} {}",
                        instance_name,
                        node_names[2],
                        node_names[0],
                        node_names[1],
                        gain_with_params
                    )
                })
            }

            // Controlled sources (4 terminals: + - control+ control-)
            // Spectre format: E1 out+ out- in+ in- gain [params]
            ComponentType::Vcvs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, gain_with_params))
            }

            ComponentType::Vccs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, gain_with_params))
            }

            ComponentType::Ccvs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, gain_with_params))
            }

            ComponentType::Cccs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, gain_with_params))
            }

            // Voltage-controlled switch (4 terminals: 1 2 c+ c-):
            // S name n1 n2 nc+ nc- model, plus a .MODEL <model> SW(...) card
            ComponentType::VSwitch => {
                let nodes = self.format_nodes(&node_names, 4);
                let (explicit_model, _) = Self::extract_model_override(component);
                let model = self.get_switch_model(component, explicit_model.as_deref());
                Some(format!("{} {} {}", instance_name, nodes, model))
            }

            // Lossless transmission line (4 terminals: a+ a- b+ b-):
            // T name a+ a- b+ b- Z0=<z0> TD=<td>
            ComponentType::TransmissionLine => {
                let nodes = self.format_nodes(&node_names, 4);
                let params = crate::properties::parse_params_string(&component.params);
                let z0 = Self::get_param_owned(&params, "z0", "", "50");
                let td = Self::get_param_owned(&params, "td", "", "1n");
                Some(format!("{} {} Z0={} TD={}", instance_name, nodes, z0, td))
            }

            // Ground - handled separately
            ComponentType::Ground => None,
            // Interface ports shape the .SUBCKT header, never instance lines
            ComponentType::Port => None,
            // Transformers are synthesized into winding inductors plus a coupling line.
            ComponentType::Transformer => None,
            // Coupling statements are synthesized in a dedicated validation pass.
            ComponentType::CoupledInductor => None,

            // Generic library/cell/view instance.
            // Emits a standard X-instance referring to the bound master name
            // (Verilog-A module or subcircuit/cell fallback).
            ComponentType::CellInstance => {
                let binding = match self.effective_library_binding(component) {
                    Ok(Some(binding)) => binding.clone(),
                    Ok(None) => {
                        self.errors.push(format!(
                            "Cell instance '{}' is missing library binding metadata",
                            component.name
                        ));
                        return None;
                    }
                    Err(error) => {
                        self.errors.push(error);
                        return None;
                    }
                };

                // Project cells (no source file) resolve their interface
                // through the workspace hierarchy: the master's port order
                // IS the node order, and its .SUBCKT is emitted alongside.
                let master_ports: Option<Vec<String>> = if binding.source_path.is_none() {
                    self.hierarchy
                        .and_then(|h| h.schematic_master_for_binding(&binding))
                        .map(|master| {
                            master
                                .interface_ports()
                                .into_iter()
                                .map(|port| port.name)
                                .collect()
                        })
                } else {
                    None
                };

                // A project cell must resolve to a master — otherwise the
                // X line would reference a definition nobody emits.
                if binding.source_path.is_none() && master_ports.is_none() {
                    self.errors.push(format!(
                        "Cell instance '{}' master not found in project: {}/{} \u{2039}schematic\u{203A} — open the cell once, or bind a source file",
                        component.name, binding.library, binding.cell
                    ));
                    return None;
                }

                let terminal_order: &[String] = if !binding.terminal_order.is_empty() {
                    &binding.terminal_order
                } else if let Some(ports) = master_ports.as_deref() {
                    ports
                } else {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) is missing terminal order metadata (netlist.ports/netlist.terminals)",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                    return None;
                };

                // Stale binding: the master's interface changed after this
                // instance was placed. Pin positions no longer correspond,
                // so this is a hard stop, not a guess.
                if let Some(ports) = master_ports.as_deref()
                    && !binding.terminal_order.is_empty()
                    && !same_terminal_sequence(ports, &binding.terminal_order)
                {
                    self.errors.push(format!(
                        "Cell instance '{}' is stale: master {}/{} interface no longer matches the instance terminal order — re-place the instance",
                        component.name,
                        binding.library,
                        binding.cell
                    ));
                    return None;
                }

                if node_names.len() != terminal_order.len() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) terminal mismatch: schematic has {} nodes but the interface defines {} terminals",
                        component.name,
                        binding.library,
                        binding.cell,
                        binding.view,
                        node_names.len(),
                        terminal_order.len()
                    ));
                    return None;
                }

                let subckt_name = binding
                    .module_name
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .unwrap_or(binding.cell.as_str());
                if subckt_name.is_empty() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) has no netlist master/module name",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                    return None;
                }

                let nodes = node_names.join(" ");
                if let Some(template) = binding.netlist_template.as_deref() {
                    let params = match model_bound_instance_params(&binding, &component.params) {
                        Ok(params) => params,
                        Err(error) => {
                            self.errors.push(format!(
                                "Cell instance '{}' ({}/{}/{}) has invalid typed parameters: {error}",
                                component.name, binding.library, binding.cell, binding.view
                            ));
                            return None;
                        }
                    };
                    match render_model_bound_instance_template(
                        template,
                        &instance_name,
                        &nodes,
                        subckt_name,
                        &params,
                    ) {
                        Ok(line) => Some(line),
                        Err(error) => {
                            self.errors.push(format!(
                                "Cell instance '{}' ({}/{}/{}) has an invalid netlist template: {error}",
                                component.name, binding.library, binding.cell, binding.view
                            ));
                            None
                        }
                    }
                } else {
                    let params = self.format_params(&component.params);
                    Some(format!(
                        "{} {} {}{}",
                        instance_name, nodes, subckt_name, params
                    ))
                }
            }

            // XSPICE components: A name nodes model [params]
            _ if component.kind.is_xspice() => {
                let nodes = node_names.join(" ");
                let model = format!("{}_model", instance_name.to_lowercase());
                let params = self.format_params(&component.params);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Catch-all for unhandled types
            // Include params for forward compatibility
            _ => {
                let nodes = node_names.join(" ");
                let value_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, value_with_params))
            }
        }
    }
}

fn render_model_bound_instance_template(
    template: &str,
    reference: &str,
    nodes: &str,
    model: &str,
    params: &str,
) -> Result<String, String> {
    let template = template.trim();
    crate::state::validate_library_netlist_template(template)?;

    let reference_token = template
        .split_ascii_whitespace()
        .next()
        .ok_or_else(|| "template is empty".to_owned())?;
    let name = if reference_token == "{ref}" {
        reference
    } else {
        let prefix = reference_token
            .strip_suffix("{name}")
            .ok_or_else(|| "template reference token is invalid".to_owned())?;
        let Some(head) = reference.get(..prefix.len()) else {
            return Err(format!(
                "reference `{reference}` does not use prefix `{prefix}`"
            ));
        };
        if !head.eq_ignore_ascii_case(prefix) {
            return Err(format!(
                "reference `{reference}` does not use prefix `{prefix}`"
            ));
        }
        let suffix = &reference[prefix.len()..];
        if suffix.is_empty() {
            return Err(format!(
                "reference `{reference}` has no identifier after prefix `{prefix}`"
            ));
        }
        suffix
    };
    let mut rendered = template
        .replace("{name}", name)
        .replace("{ref}", reference)
        .replace("{nodes}", nodes)
        .replace("{model}", model)
        .replace("{params}", params.trim());
    rendered = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
    if rendered.is_empty() {
        return Err("template produced an empty instance line".to_owned());
    }
    if !rendered
        .split_ascii_whitespace()
        .next()
        .is_some_and(|emitted| emitted.eq_ignore_ascii_case(reference))
    {
        return Err("template changed the validated instance reference".to_owned());
    }
    Ok(rendered)
}

fn model_bound_instance_params(
    binding: &crate::state::LibraryCellInstance,
    raw: &str,
) -> Result<String, String> {
    if raw.trim().is_empty() {
        return Ok(String::new());
    }
    if raw.chars().any(char::is_control) {
        return Err("parameter text contains a line break or control character".to_owned());
    }
    let parsed = crate::state::parse_replacement_parameters_strict(raw)
        .map_err(|error| error.to_string())?;
    for (key, value) in &parsed {
        if !binding
            .parameter_order
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(key))
        {
            return Err(format!(
                "parameter `{key}` is not declared by the symbol form"
            ));
        }
        if value.is_empty()
            || value.len() > 512
            || value.chars().any(char::is_control)
            || value
                .chars()
                .any(|character| matches!(character, ';' | '$' | '#' | '\\'))
        {
            return Err(format!("parameter `{key}` has an unsafe or invalid value"));
        }
    }
    let formatted = binding
        .parameter_order
        .iter()
        .filter_map(|key| {
            parsed
                .iter()
                .find(|(parsed_key, _)| parsed_key.eq_ignore_ascii_case(key))
                .map(|(_, value)| format!("{key}={value}"))
        })
        .collect::<Vec<_>>()
        .join(" ");
    if formatted.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!(" {formatted}"))
    }
}

#[cfg(test)]
mod model_bound_template_tests {
    use super::{model_bound_instance_params, render_model_bound_instance_template};
    use crate::simulation::netlist_gen::NetlistGenerator;
    use crate::state::{Component, ComponentType, LibraryCellInstance, Point, SchematicState};

    #[test]
    fn renders_primitive_model_instance_without_an_x_prefix() {
        let line = render_model_bound_instance_template(
            "M{name} {nodes} {model} {params}",
            "M7",
            "drain gate source bulk",
            "nmos_18",
            " w=1u l=180n",
        )
        .expect("valid template");

        assert_eq!(line, "M7 drain gate source bulk nmos_18 w=1u l=180n");
        assert_eq!(
            render_model_bound_instance_template(
                "M{name} {nodes} {model}",
                "MFOO",
                "drain gate source bulk",
                "nmos_18",
                "",
            )
            .unwrap(),
            "MFOO drain gate source bulk nmos_18"
        );
    }

    #[test]
    fn rejects_multiline_and_unknown_template_content() {
        assert!(
            render_model_bound_instance_template(
                "M{name} {nodes} {model}\n.include injected.lib",
                "M1",
                "d g s b",
                "nmos",
                "",
            )
            .is_err()
        );
        assert!(
            render_model_bound_instance_template(
                "Q{name} {nodes} {model}",
                "M1",
                "d g s b",
                "nmos",
                "",
            )
            .is_err()
        );
        assert!(
            render_model_bound_instance_template(
                "M{name} {nodes} {model} {unknown}",
                "M1",
                "d g s b",
                "nmos",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn typed_instance_parameters_are_allowlisted_and_canonicalized() {
        let mut binding = LibraryCellInstance::new("models", "nmos_18", "spice");
        binding.parameter_order = vec!["w".to_owned(), "l".to_owned()];

        assert_eq!(
            model_bound_instance_params(&binding, "w=1u l=180n").unwrap(),
            " w=1u l=180n"
        );
        assert!(model_bound_instance_params(&binding, "w=1u owner=hidden").is_err());
        assert!(model_bound_instance_params(&binding, "w=1u\n.include attack.lib").is_err());
        assert!(model_bound_instance_params(&binding, "w=1u stray").is_err());
        assert!(model_bound_instance_params(&binding, "w=1u W=2u").is_err());
        assert_eq!(
            model_bound_instance_params(&binding, "w={base * 2} l=180n").unwrap(),
            " w={base * 2} l=180n"
        );
    }

    #[test]
    fn model_bound_reference_is_not_reprefixed_as_a_subcircuit() {
        let schematic = SchematicState::default();
        let generator = NetlistGenerator::new(&schematic);
        let mut binding = LibraryCellInstance::new("models", "nmos_18", "spice");
        binding.netlist_template = Some("M{name} {nodes} {model}".to_owned());
        binding.reference_prefix = Some("M".to_owned());
        let component = Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding)
            .with_name_value("M17", "nmos_18");

        assert_eq!(generator.instance_name(&component), "M17");
    }
}

fn same_terminal_sequence(master_ports: &[String], placed_ports: &[String]) -> bool {
    master_ports.len() == placed_ports.len()
        && master_ports
            .iter()
            .zip(placed_ports)
            .all(|(master, placed)| master.eq_ignore_ascii_case(placed))
}
