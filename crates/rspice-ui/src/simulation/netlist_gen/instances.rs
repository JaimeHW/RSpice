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
        let terminals = component.terminal_positions();
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
                let expression = if expression.is_empty() { "V=0" } else { expression };
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
                        instance_name, node_names[2], node_names[0], node_names[1],
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
            // Transformers are synthesized into winding inductors plus a coupling line.
            ComponentType::Transformer => None,
            // Coupling statements are synthesized in a dedicated validation pass.
            ComponentType::CoupledInductor => None,

            // Generic library/cell/view instance.
            // Emits a standard X-instance referring to the bound master name
            // (Verilog-A module or subcircuit/cell fallback).
            ComponentType::CellInstance => {
                let Some(binding) = component.library_cell.as_ref() else {
                    self.errors.push(format!(
                        "Cell instance '{}' is missing library binding metadata",
                        component.name
                    ));
                    return None;
                };

                if binding.terminal_order.is_empty() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) is missing terminal order metadata (netlist.ports/netlist.terminals)",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                    return None;
                }
                if node_names.len() != binding.terminal_order.len() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) terminal mismatch: schematic has {} nodes but binding defines {} terminals",
                        component.name,
                        binding.library,
                        binding.cell,
                        binding.view,
                        node_names.len(),
                        binding.terminal_order.len()
                    ));
                    return None;
                }
                if binding.source_path.is_none() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) is missing source path metadata",
                        component.name, binding.library, binding.cell, binding.view
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
                let params = self.format_params(&component.params);
                Some(format!(
                    "{} {} {}{}",
                    instance_name, nodes, subckt_name, params
                ))
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
