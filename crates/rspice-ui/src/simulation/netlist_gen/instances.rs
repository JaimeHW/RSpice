//! Instance cards.
//!
//! Writes one card per placed component, with its terminals in the order
//! its device type requires and its parameter overrides appended.

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
        let terminal_points = terminals.iter().map(|(_, pos)| *pos).collect::<Vec<_>>();
        let instance_name = self.instance_name(component);

        match component.kind {
            // Two-terminal passive components: X name node+ node- value [params]
            // Spectre format: R1 net1 net2 1k m=2 tc1=0.01
            ComponentType::Resistor | ComponentType::Capacitor => {
                let nodes = self.format_nodes(&node_names, 2);
                let value_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", instance_name, nodes, value_with_params))
            }

            // Diode: D name A K model [params], with a default junction model
            // when the user has not bound one.
            ComponentType::Diode => {
                let nodes = self.format_nodes(&node_names, 2);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_diode_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            ComponentType::Inductor => {
                let nodes = self.format_nodes(&node_names, 2);
                let filtered_params = self
                    .filter_component_params(&component.params, &["coupled_to", "coupling_factor"]);
                let value_with_params =
                    self.format_value_with_params(&component.value, &filtered_params);
                Some(format!("{} {} {}", instance_name, nodes, value_with_params))
            }

            // Saturable inductor: L name node+ node- value model, where the
            // model card carries the Jiles-Atherton magnetic-core parameters.
            // The inductance value is mandatory on the core-model path.
            ComponentType::SaturableInductor => {
                let nodes = self.format_nodes(&node_names, 2);
                let value = if component.value.trim().is_empty() {
                    "1m"
                } else {
                    component.value.trim()
                };
                let model = self.get_saturable_core_model(component);
                Some(format!("{} {} {} {}", instance_name, nodes, value, model))
            }

            // Independent source properties are editor semantics, not arbitrary
            // instance assignments.  Lower AC excitation and parasitics to
            // native cards and reject unsupported PAC/XF data before a deck can
            // reach the simulator.
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => {
                self.generate_independent_source(component, &node_names, &instance_name)
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
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_mosfet_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Vertical power DMOS: M name D G S B model [params], with a
            // VDMOS model card (the body diode and drift resistance live on
            // the card, not the instance line).
            ComponentType::NVdmos | ComponentType::PVdmos => {
                let nodes = self.format_nodes(&node_names, 4);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_vdmos_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Three-terminal JFET: J name D G S model [params]
            // Spectre format: J1 drain gate source njf_J1 area=1 m=1
            ComponentType::Njfet | ComponentType::Pjfet => {
                let nodes = self.format_nodes(&node_names, 3);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_jfet_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Three-terminal MESFET/HFET: Z name D G S model [params].
            // HFET routing comes from the bound model card (NHFET/PHFET
            // type or LEVEL=5/6), not the schematic symbol.
            ComponentType::Nmesfet | ComponentType::Pmesfet => {
                let nodes = self.format_nodes(&node_names, 3);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_mesfet_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Four/five-terminal BJT: Q name C B E S [dT] model [params].
            // The thermal variant defaults to a native VBIC card, the only
            // built-in family that solves the dT node.
            ComponentType::NpnBjt4 | ComponentType::PnpBjt4 => {
                let nodes = self.format_nodes(&node_names, 4);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_bjt_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }
            ComponentType::NpnBjt5 | ComponentType::PnpBjt5 => {
                let nodes = self.format_nodes(&node_names, 5);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_vbic_bjt_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", instance_name, nodes, model, params))
            }

            // Five-terminal SOI MOSFET: M name D G S E P model [params],
            // bound to a partially-depleted BSIMSOI card with a resistive
            // body tie by default. W/L always reach the deck — the BSIMSOI
            // internal geometry defaults do not converge on a bare card.
            ComponentType::NmosSoi | ComponentType::PmosSoi => {
                let nodes = self.format_nodes(&node_names, 5);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_soi_mosfet_model(component, explicit_model.as_deref());
                let mut params_map = crate::state::parse_params_string(&params_without_model);
                params_map
                    .entry("w".to_owned())
                    .or_insert_with(|| "1u".to_owned());
                params_map
                    .entry("l".to_owned())
                    .or_insert_with(|| "180n".to_owned());
                let params = self.format_params(&crate::state::format_params_string(&params_map));
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
            // Linear E/G cards do not accept arbitrary instance assignments.
            // Lower the editor's multiplier, polynomial, and output limits to
            // an exact linear expression or a behavioral source instead of
            // appending invalid `m=`, `poly=`, `vmax=`, ... tokens.
            ComponentType::Vcvs | ComponentType::Vccs => {
                self.generate_voltage_controlled_source(component, &node_names, &instance_name)
            }

            // Current-controlled sources: SPICE H/F take a controlling
            // V-source NAME, not a node pair. The schematic's control pins
            // stay wired as a current path through a synthesized 0 V sense
            // source, exactly like the current-sensing terminals on
            // commercial CCVS/CCCS symbols.
            ComponentType::Ccvs | ComponentType::Cccs => {
                if node_names.len() < 4 {
                    self.errors.push(format!(
                        "{} '{}' is missing control terminals",
                        component.kind.display_name(),
                        component.name
                    ));
                    return None;
                }
                let params = crate::state::parse_params_string(&component.params);
                let authored_reference = params
                    .get("vref")
                    .map(|reference| reference.trim())
                    .filter(|reference| !reference.is_empty());
                let sense_name = authored_reference
                    .map(str::to_owned)
                    .unwrap_or_else(|| format!("VSENSE_{}", instance_name));
                if authored_reference.is_none() {
                    self.lines.push(format!(
                        "{} {} {} 0",
                        sense_name, node_names[2], node_names[3]
                    ));
                }
                let gain = self.format_value(&component.value);
                let multiplier = params
                    .get("m")
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty() && *value != "1");
                let polynomial = params
                    .get("poly")
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty());
                let (upper_key, lower_key) = if component.kind == ComponentType::Ccvs {
                    ("vmax", "vmin")
                } else {
                    ("imax", "imin")
                };
                let upper = params.get(upper_key).map(String::as_str);
                let lower = params.get(lower_key).map(String::as_str);

                if polynomial.is_some() || upper.is_some() || lower.is_some() {
                    let current = format!("i({sense_name})");
                    let mut expression = if let Some(coefficients) = polynomial {
                        polynomial_expression(coefficients, &current)
                    } else {
                        format!("({gain})*({current})")
                    };
                    if let Some(multiplier) = multiplier {
                        expression = format!("({multiplier})*({expression})");
                    }
                    if upper.is_some() || lower.is_some() {
                        expression = format!(
                            "limit({expression},{},{})",
                            lower.unwrap_or("-1e308"),
                            upper.unwrap_or("1e308")
                        );
                    }
                    let behavioral_name = format!(
                        "B{}",
                        instance_name
                            .strip_prefix(component.kind.spice_prefix())
                            .unwrap_or(&instance_name)
                    );
                    let quantity = if component.kind == ComponentType::Ccvs {
                        "V"
                    } else {
                        "I"
                    };
                    Some(format!(
                        "{} {} {} {}={{{}}}",
                        behavioral_name, node_names[0], node_names[1], quantity, expression
                    ))
                } else {
                    let gain = multiplier
                        .map(|multiplier| format!("{{({gain})*({multiplier})}}"))
                        .unwrap_or(gain);
                    Some(format!(
                        "{} {} {} {} {}",
                        instance_name, node_names[0], node_names[1], sense_name, gain
                    ))
                }
            }

            // Voltage-controlled switch (4 terminals: 1 2 c+ c-):
            // S name n1 n2 nc+ nc- model, plus a .MODEL <model> SW(...) card
            ComponentType::VSwitch => {
                let nodes = self.format_nodes(&node_names, 4);
                let (explicit_model, _) = Self::extract_model_override(component);
                let model = self.get_switch_model(component, explicit_model.as_deref());
                Some(format!("{} {} {}", instance_name, nodes, model))
            }

            // Xyce expression-controlled generic switch:
            // S name n+ n- model [ON|OFF] CONTROL={expr}.
            ComponentType::GenericSwitch => {
                let nodes = self.format_nodes(&node_names, 2);
                let expression = component.value.trim();
                if expression.is_empty() || expression.chars().any(char::is_control) {
                    self.errors.push(format!(
                        "Expression switch '{}' requires a single-line control expression",
                        component.name
                    ));
                    return None;
                }
                let expression = expression
                    .strip_prefix('{')
                    .and_then(|inner| inner.strip_suffix('}'))
                    .unwrap_or(expression)
                    .trim();
                if expression.is_empty() {
                    self.errors.push(format!(
                        "Expression switch '{}' has an empty control expression",
                        component.name
                    ));
                    return None;
                }
                let params = crate::state::parse_params_string(&component.params);
                let explicit_model = params
                    .get("model")
                    .map(String::as_str)
                    .map(str::trim)
                    .filter(|model| !model.is_empty());
                let model = self.get_switch_model(component, explicit_model);
                let state = match params.get("state").map(String::as_str) {
                    Some("on") => " ON",
                    Some("off") => " OFF",
                    _ => "",
                };
                Some(format!(
                    "{} {} {}{} CONTROL={{{}}}",
                    instance_name, nodes, model, state, expression
                ))
            }

            // Current-controlled switch: W name n+ n- <vsrc> model [ON|OFF].
            // The schematic's sense-coil pins become a synthesized 0 V
            // sense source wired in series with the monitored branch —
            // the same pattern as the CCVS/CCCS control terminals.
            ComponentType::ISwitch => {
                if node_names.len() < 4 {
                    self.errors.push(format!(
                        "I-Switch '{}' is missing its sense-coil terminals",
                        component.name
                    ));
                    return None;
                }
                let sense_name = format!("VSENSE_{}", instance_name);
                self.lines.push(format!(
                    "{} {} {} 0",
                    sense_name, node_names[2], node_names[3]
                ));
                let (explicit_model, _) = Self::extract_model_override(component);
                let model = self.get_iswitch_model(component, explicit_model.as_deref());
                let params = crate::state::parse_params_string(&component.params);
                let state = match params.get("state").map(String::as_str) {
                    Some("on") => " ON",
                    Some("off") => " OFF",
                    _ => "",
                };
                Some(format!(
                    "{} {} {} {} {}{}",
                    instance_name, node_names[0], node_names[1], sense_name, model, state
                ))
            }

            // Lossless transmission line (4 terminals: a+ a- b+ b-):
            // T name a+ a- b+ b- Z0=<z0> TD=<td>
            ComponentType::TransmissionLine => {
                let nodes = self.format_nodes(&node_names, 4);
                let params = crate::state::parse_params_string(&component.params);
                let z0 = Self::get_param_owned(&params, "z0", "", "50");
                let td = Self::get_param_owned(&params, "td", "", "1n");
                Some(format!("{} {} Z0={} TD={}", instance_name, nodes, z0, td))
            }

            // Lossy transmission line: O name a+ a- b+ b- <model>, with a
            // generated LTRA or TXL card carrying the per-length RLGC data.
            ComponentType::LossyTransmissionLine => {
                let nodes = self.format_nodes(&node_names, 4);
                let (explicit_model, _) = Self::extract_model_override(component);
                let model = self.get_lossy_tline_model(component, explicit_model.as_deref());
                Some(format!("{} {} {}", instance_name, nodes, model))
            }

            // Coupled transmission line: P name a1 a2 ar b1 b2 br <model>.
            // The CPL card carries upper-triangle RLGC matrices plus the
            // physical length.
            ComponentType::CoupledTransmissionLine => {
                let nodes = self.format_nodes(&node_names, 6);
                let (explicit_model, _) = Self::extract_model_override(component);
                let model = self.get_cpl_model(component, explicit_model.as_deref());
                Some(format!("{} {} {}", instance_name, nodes, model))
            }

            // Memristor: YMEMRISTOR <name> n+ n- <model> (Xyce element;
            // the TEAM LEVEL=2 card is the executable default).
            ComponentType::Memristor => {
                let nodes = self.format_nodes(&node_names, 2);
                let (explicit_model, _) = Self::extract_model_override(component);
                let model = self.get_memristor_model(component, explicit_model.as_deref());
                Some(format!("YMEMRISTOR {} {} {}", instance_name, nodes, model))
            }

            // RF port: P name n+ n- PORT=k Z0=r [DC v] [AC mag]. With no
            // source spec the core lowers it to a Z0 terminator; with one
            // it becomes a Thevenin source behind Z0.
            ComponentType::RfPort => {
                let nodes = self.format_nodes(&node_names, 2);
                let params = crate::state::parse_params_string(&component.params);
                let port = Self::get_param_owned(&params, "port", "", "1");
                let z0 = Self::get_param_owned(&params, "z0", "", "50");
                let mut line = format!("{} {} PORT={} Z0={}", instance_name, nodes, port, z0);
                let dc = Self::get_param_owned(&params, "dc", component.value.trim(), "");
                if !dc.is_empty() {
                    line.push_str(&format!(" DC {}", dc));
                }
                let ac = Self::get_param_owned(&params, "ac_mag", "", "");
                if !ac.is_empty() {
                    line.push_str(&format!(" AC {}", ac));
                }
                Some(line)
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

                if binding.is_builtin_xspice() {
                    return self.generate_builtin_xspice_instance(
                        component,
                        &binding,
                        &node_names,
                        &terminal_points,
                        &instance_name,
                    );
                }
                if binding.is_generated_veriloga() {
                    let descriptor =
                        match crate::state::validate_generated_veriloga_binding(&binding) {
                            Ok(descriptor) => descriptor,
                            Err(error) => {
                                self.errors.push(format!(
                                    "Generated Verilog-A instance '{}' is not executable: {error}",
                                    component.name
                                ));
                                return None;
                            }
                        };
                    if node_names.len() != descriptor.terminals.len() {
                        self.errors.push(format!(
                            "Generated Verilog-A instance '{}' expects {} terminals for model '{}', found {}",
                            component.name,
                            descriptor.terminals.len(),
                            descriptor.model_name,
                            node_names.len()
                        ));
                        return None;
                    }
                    let params = match model_bound_instance_params(&binding, &component.params) {
                        Ok(params) => params,
                        Err(error) => {
                            self.errors.push(format!(
                                "Generated Verilog-A instance '{}' has invalid parameters: {error}",
                                component.name
                            ));
                            return None;
                        }
                    };
                    return Some(format!(
                        "{} {} {}{}",
                        instance_name,
                        node_names.join(" "),
                        descriptor.model_name,
                        params
                    ));
                }

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

            // Legacy enum-backed XSPICE components: A name <shaped ports>
            // model, plus the code-model .MODEL card.  This list is
            // deliberately exhaustive so a new enum variant cannot fall into
            // guessed generic-card emission.
            ComponentType::XspiceGain
            | ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator
            | ComponentType::XspiceInverter
            | ComponentType::XspiceBuffer
            | ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate
            | ComponentType::XspiceTristate
            | ComponentType::XspiceDFlipFlop
            | ComponentType::XspiceJkFlipFlop
            | ComponentType::XspiceSrLatch
            | ComponentType::XspiceAdcBridge
            | ComponentType::XspiceDacBridge => {
                self.generate_xspice_instance(component, &node_names, &instance_name)
            }
        }
    }

    fn generate_voltage_controlled_source(
        &mut self,
        component: &Component,
        node_names: &[String],
        instance_name: &str,
    ) -> Option<String> {
        if node_names.len() < 4 {
            self.errors.push(format!(
                "{} '{}' is missing control terminals",
                component.kind.display_name(),
                component.name
            ));
            return None;
        }

        let params = crate::state::parse_params_string(&component.params);
        let ac_magnitude_key = if component.kind == ComponentType::Vcvs {
            "ac_gain"
        } else {
            "ac_gm"
        };
        let authored_ac_magnitude = params
            .get(ac_magnitude_key)
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && !matches!(*value, "0" | "0.0"));
        let authored_ac_phase = params
            .get("ac_phase")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && !matches!(*value, "0" | "0.0"));
        if authored_ac_magnitude.is_some() || authored_ac_phase.is_some() {
            self.errors.push(format!(
                "{} '{}' requests a separate AC gain/phase, but the RSpice controlled-source engine does not yet support AC overrides; clear those fields before simulation",
                component.kind.display_name(),
                component.name
            ));
            return None;
        }

        let gain = self.format_value(&component.value);
        let multiplier = params
            .get("m")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "1");
        let polynomial = params
            .get("poly")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        let (upper_key, lower_key) = if component.kind == ComponentType::Vcvs {
            ("vmax", "vmin")
        } else {
            ("imax", "imin")
        };
        let upper = params.get(upper_key).map(String::as_str);
        let lower = params.get(lower_key).map(String::as_str);

        if polynomial.is_some() || upper.is_some() || lower.is_some() {
            let control = format!("v({},{})", node_names[2], node_names[3]);
            let mut expression = if let Some(coefficients) = polynomial {
                polynomial_expression(coefficients, &control)
            } else {
                format!("({gain})*({control})")
            };
            if let Some(multiplier) = multiplier {
                expression = format!("({multiplier})*({expression})");
            }
            if upper.is_some() || lower.is_some() {
                expression = format!(
                    "limit({expression},{},{})",
                    lower.unwrap_or("-1e308"),
                    upper.unwrap_or("1e308")
                );
            }
            let behavioral_name = behavioral_controlled_source_name(component, instance_name);
            let quantity = if component.kind == ComponentType::Vcvs {
                "V"
            } else {
                "I"
            };
            return Some(format!(
                "{} {} {} {}={{{}}}",
                behavioral_name, node_names[0], node_names[1], quantity, expression
            ));
        }

        let effective_gain = multiplier
            .map(|multiplier| format!("{{({gain})*({multiplier})}}"))
            .unwrap_or(gain);
        Some(format!(
            "{} {} {} {} {} {}",
            instance_name,
            node_names[0],
            node_names[1],
            node_names[2],
            node_names[3],
            effective_gain
        ))
    }

    fn generate_independent_source(
        &mut self,
        component: &Component,
        node_names: &[String],
        instance_name: &str,
    ) -> Option<String> {
        if node_names.len() < 2 {
            self.errors.push(format!(
                "{} '{}' is missing a terminal",
                component.kind.display_name(),
                component.name
            ));
            return None;
        }

        let params = crate::state::parse_params_string(&component.params);
        let allowed = independent_source_parameter_names(component.kind);
        let mut unknown = params
            .keys()
            .filter(|name| !allowed.contains(&name.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        unknown.sort();
        if !unknown.is_empty() {
            self.errors.push(format!(
                "{} '{}' has unsupported source parameter(s): {}",
                component.kind.display_name(),
                component.name,
                unknown.join(", ")
            ));
            return None;
        }

        let unsupported_analysis_parameters = ["xfmag", "pacmag", "pacdbm", "pacphase"]
            .into_iter()
            .filter(|name| {
                params
                    .get(*name)
                    .is_some_and(|value| source_analysis_parameter_is_active(name, value))
            })
            .collect::<Vec<_>>();
        if !unsupported_analysis_parameters.is_empty() {
            self.errors.push(format!(
                "{} '{}' configures {}, but source-level XF/PAC excitation is not yet connected to the RSpice analysis engine",
                component.kind.display_name(),
                component.name,
                unsupported_analysis_parameters.join(", ")
            ));
            return None;
        }

        let is_voltage = matches!(
            component.kind,
            ComponentType::VoltageSource
                | ComponentType::VoltageSourceAc
                | ComponentType::VoltageSourcePulse
                | ComponentType::VoltageSourceSin
                | ComponentType::VoltageSourcePwl
                | ComponentType::VoltageSourceExp
                | ComponentType::VoltageSourceSffm
        );
        let noise_suffix = if params
            .get("isnoisy")
            .is_some_and(|value| source_boolean_is_false(value))
        {
            " NOISY=0"
        } else {
            ""
        };

        let mut positive_node = node_names[0].as_str();
        let internal_node;
        if is_voltage
            && let Some(series_resistance) = params
                .get("rs")
                .map(String::as_str)
                .filter(|value| !source_value_is_zero_or_blank(value))
        {
            internal_node = format!("__rspice_source_{}_internal", component.id);
            let resistance_name = format!("R__RSPICE_SOURCE_{}_RS", component.id);
            self.lines.push(format!(
                "{} {} {} {}{}",
                resistance_name,
                node_names[0],
                internal_node,
                series_resistance.trim(),
                noise_suffix
            ));
            positive_node = &internal_node;
        }

        if let Some(parallel_resistance) = params
            .get("rp")
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
        {
            if source_value_is_zero_or_blank(parallel_resistance) {
                self.errors.push(format!(
                    "{} '{}' parallel resistance must be positive or blank",
                    component.kind.display_name(),
                    component.name
                ));
                return None;
            }
            self.lines.push(format!(
                "R__RSPICE_SOURCE_{}_RP {} {} {}{}",
                component.id,
                node_names[0],
                node_names[1],
                parallel_resistance.trim(),
                noise_suffix
            ));
        }
        if let Some(parallel_capacitance) = params
            .get("cpar")
            .map(String::as_str)
            .filter(|value| !source_value_is_zero_or_blank(value))
        {
            self.lines.push(format!(
                "C__RSPICE_SOURCE_{}_CPAR {} {} {}",
                component.id,
                node_names[0],
                node_names[1],
                parallel_capacitance.trim()
            ));
        }

        let mut specification = self.format_source_value(component);
        if !matches!(
            component.kind,
            ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc
        ) && (params.contains_key("ac") || params.contains_key("acphase"))
        {
            let magnitude = params
                .get("ac")
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("0");
            let phase = params
                .get("acphase")
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("0");
            specification.push_str(&format!(" AC {} {}", magnitude.trim(), phase.trim()));
        }

        Some(format!(
            "{} {} {} {}",
            instance_name, positive_node, node_names[1], specification
        ))
    }
}

fn independent_source_parameter_names(kind: ComponentType) -> &'static [&'static str] {
    const COMMON_VOLTAGE: &[&str] = &[
        "ac",
        "acphase",
        "xfmag",
        "pacmag",
        "pacdbm",
        "pacphase",
        "rs",
        "rp",
        "cpar",
        "isnoisy",
        "model_library",
        "model_corner",
    ];
    const COMMON_CURRENT: &[&str] = &[
        "ac",
        "acphase",
        "xfmag",
        "pacmag",
        "pacdbm",
        "pacphase",
        "rp",
        "cpar",
        "isnoisy",
        "model_library",
        "model_corner",
    ];
    match kind {
        ComponentType::VoltageSource => COMMON_VOLTAGE,
        ComponentType::VoltageSourceAc => &[
            "dc",
            "ac",
            "acphase",
            "phase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rs",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::VoltageSourcePulse => &[
            "v1",
            "v2",
            "td",
            "tr",
            "tf",
            "pw",
            "per",
            "period",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rs",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::VoltageSourceSin => &[
            "vo",
            "va",
            "freq",
            "td",
            "theta",
            "phase",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rs",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::VoltageSourcePwl => &[
            "pwl_data",
            "td",
            "repeat",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rs",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::VoltageSourceExp => &[
            "v1",
            "v2",
            "td1",
            "tau1",
            "td2",
            "tau2",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rs",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::VoltageSourceSffm => &[
            "vo",
            "va",
            "fc",
            "mdi",
            "fs",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rs",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::CurrentSource => COMMON_CURRENT,
        ComponentType::CurrentSourceAc => &[
            "dc",
            "ac",
            "acphase",
            "phase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::CurrentSourcePulse => &[
            "i1",
            "i2",
            "td",
            "tr",
            "tf",
            "pw",
            "per",
            "period",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::CurrentSourceSin => &[
            "io",
            "ia",
            "freq",
            "td",
            "theta",
            "phase",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::CurrentSourcePwl => &[
            "pwl_data",
            "td",
            "repeat",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::CurrentSourceExp => &[
            "i1",
            "i2",
            "td1",
            "tau1",
            "td2",
            "tau2",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        ComponentType::CurrentSourceNoise => &[
            "dc",
            "na",
            "nt",
            "nalpha",
            "namp",
            "ac",
            "acphase",
            "xfmag",
            "pacmag",
            "pacdbm",
            "pacphase",
            "rp",
            "cpar",
            "isnoisy",
            "model_library",
            "model_corner",
        ],
        _ => &[],
    }
}

fn source_analysis_parameter_is_active(name: &str, value: &str) -> bool {
    if name == "pacdbm" {
        !value.trim().is_empty()
    } else {
        !source_value_is_zero_or_blank(value)
    }
}

fn source_value_is_zero_or_blank(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "0" | "+0" | "-0" | "0.0" | "0v" | "0a" | "0f" | "0ohm" | "0Ω"
    )
}

fn source_boolean_is_false(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "0" | "false" | "no" | "off"
    )
}

fn behavioral_controlled_source_name(component: &Component, instance_name: &str) -> String {
    format!(
        "B{}",
        instance_name
            .strip_prefix(component.kind.spice_prefix())
            .unwrap_or(instance_name)
    )
}

fn polynomial_expression(coefficients: &str, variable: &str) -> String {
    let coefficients = coefficients
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|coefficient| !coefficient.is_empty())
        .collect::<Vec<_>>();
    if coefficients.is_empty() {
        return "0".to_owned();
    }
    coefficients
        .iter()
        .enumerate()
        .map(|(power, coefficient)| match power {
            0 => format!("({coefficient})"),
            1 => format!("({coefficient})*({variable})"),
            _ => format!("({coefficient})*pow(({variable}),{power})"),
        })
        .collect::<Vec<_>>()
        .join("+")
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

pub(super) fn model_bound_instance_params(
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

fn same_terminal_sequence(master_ports: &[String], placed_ports: &[String]) -> bool {
    master_ports.len() == placed_ports.len()
        && master_ports
            .iter()
            .zip(placed_ports)
            .all(|(master, placed)| master.eq_ignore_ascii_case(placed))
}

#[cfg(all(test, feature = "generated-veriloga-catalog"))]
mod generated_veriloga_netlist_tests {
    use crate::simulation::netlist_gen::generate_netlist;
    use crate::state::{
        Component, ComponentType, Point, SchematicState, generated_veriloga_devices,
        generated_veriloga_library_binding,
    };
    use rspice_core::engine::{Engine, SimulationConfig};
    use rspice_core::netlist::Netlist;

    #[test]
    fn exact_generated_binding_netlists_and_routes_to_the_compiled_engine() {
        let descriptor = generated_veriloga_devices()
            .iter()
            .find(|descriptor| descriptor.model_name.eq_ignore_ascii_case("r2_cmc"))
            .expect("the shipped R2 CMC model is compiled");
        let binding = generated_veriloga_library_binding(descriptor).expect("valid binding");
        let component = Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding)
            .with_name_value("X1", descriptor.model_name);
        let mut schematic = SchematicState::default();
        schematic.components.push(component);

        let generated = generate_netlist(&schematic);
        assert!(generated.errors.is_empty(), "{:?}", generated.errors);
        let instance_line = generated
            .netlist
            .lines()
            .find(|line| line.starts_with("X1 "))
            .expect("generated instance line");
        assert_eq!(
            instance_line.split_whitespace().count(),
            descriptor.terminals.len() + 2,
            "{instance_line}"
        );
        assert!(instance_line.ends_with(descriptor.model_name));

        let parsed = Netlist::parse(&generated.netlist).expect("generated deck parses");
        let circuit = Engine::new(SimulationConfig::default())
            .build_circuit(&parsed)
            .expect("generated deck builds");
        assert!(circuit.has_generated_veriloga_devices());
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

#[cfg(test)]
mod controlled_source_lowering_tests {
    use crate::simulation::netlist_gen::NetlistGenerator;
    use crate::state::{Component, ComponentType, Point, SchematicState};

    fn nodes() -> Vec<String> {
        ["outp", "outn", "inp", "inn"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn vcvs_multiplier_is_lowered_into_the_linear_gain_expression() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component =
            Component::new(1, ComponentType::Vcvs, Point::origin()).with_name_value("E1", "10");
        component.params = "m=2".to_owned();

        let line = generator
            .generate_voltage_controlled_source(&component, &nodes(), "E1")
            .expect("VCVS line");

        assert_eq!(line, "E1 outp outn inp inn {(10)*(2)}");
        assert!(generator.errors().is_empty());
        rspice_core::netlist::parse_netlist(&format!("controlled\n{line}\n.end\n"))
            .expect("lowered VCVS must parse");
    }

    #[test]
    fn vcvs_polynomial_and_limits_lower_to_a_behavioral_voltage_source() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component =
            Component::new(1, ComponentType::Vcvs, Point::origin()).with_name_value("E7", "1");
        component.params = "poly=\"1,2,3\" m=4 vmin=-5 vmax=5".to_owned();

        let line = generator
            .generate_voltage_controlled_source(&component, &nodes(), "E7")
            .expect("behavioral VCVS line");

        assert!(line.starts_with("B7 outp outn V={limit("), "{line}");
        assert!(line.contains("v(inp,inn)"), "{line}");
        assert!(line.contains("pow((v(inp,inn)),2)"), "{line}");
        rspice_core::netlist::parse_netlist(&format!("controlled\n{line}\n.end\n"))
            .expect("behavioral VCVS must parse");
    }

    #[test]
    fn vccs_polynomial_lowers_to_a_behavioral_current_source() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component =
            Component::new(1, ComponentType::Vccs, Point::origin()).with_name_value("G3", "1m");
        component.params = "poly=\"0,1m\"".to_owned();

        let line = generator
            .generate_voltage_controlled_source(&component, &nodes(), "G3")
            .expect("behavioral VCCS line");

        assert!(line.starts_with("B3 outp outn I={"), "{line}");
        assert!(line.contains("v(inp,inn)"), "{line}");
        rspice_core::netlist::parse_netlist(&format!("controlled\n{line}\n.end\n"))
            .expect("behavioral VCCS must parse");
    }

    #[test]
    fn unsupported_controlled_source_ac_override_fails_before_netlisting() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component =
            Component::new(1, ComponentType::Vcvs, Point::origin()).with_name_value("E1", "10");
        component.params = "ac_gain=2 ac_phase=45".to_owned();

        assert!(
            generator
                .generate_voltage_controlled_source(&component, &nodes(), "E1")
                .is_none()
        );
        assert!(generator.errors().iter().any(|error| error.contains("AC")));
    }
}

#[cfg(test)]
mod independent_source_lowering_tests {
    use crate::simulation::netlist_gen::NetlistGenerator;
    use crate::state::{Component, ComponentType, Point, SchematicState};

    fn nodes() -> Vec<String> {
        vec!["source_p".to_owned(), "source_n".to_owned()]
    }

    fn assert_generated_cards_parse(generator: &NetlistGenerator<'_>, main: &str) {
        let mut cards = generator.lines.clone();
        cards.push(main.to_owned());
        rspice_core::netlist::parse_netlist(&format!(
            "independent source lowering\n{}\n.end\n",
            cards.join("\n")
        ))
        .expect("lowered independent-source cards must parse");
    }

    #[test]
    fn transient_voltage_source_preserves_ac_excitation_and_lowers_parasitics() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component = Component::new(17, ComponentType::VoltageSourcePulse, Point::origin())
            .with_name_value("V1", "0");
        component.params =
            "v2=5 pw=1u per=2u ac=2 acphase=90 rs=5 rp=1k cpar=1p isnoisy=false".to_owned();

        let main = generator
            .generate_independent_source(&component, &nodes(), "V1")
            .expect("source line");

        assert!(
            main.starts_with("V1 __rspice_source_17_internal source_n PULSE("),
            "{main}"
        );
        assert!(main.ends_with(" AC 2 90"), "{main}");
        assert_eq!(generator.lines.len(), 3);
        assert!(generator.lines[0].contains(" 5 NOISY=0"));
        assert!(generator.lines[1].contains(" 1k NOISY=0"));
        assert!(generator.lines[2].contains(" 1p"));
        assert_generated_cards_parse(&generator, &main);
    }

    #[test]
    fn explicit_ac_source_keeps_its_dc_operating_point() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component = Component::new(2, ComponentType::CurrentSourceAc, Point::origin())
            .with_name_value("I2", "3m");
        component.params = "dc=1m acphase=-45".to_owned();

        let main = generator
            .generate_independent_source(&component, &nodes(), "I2")
            .expect("source line");

        assert_eq!(main, "I2 source_p source_n DC 1m AC 3m -45");
        assert_generated_cards_parse(&generator, &main);
    }

    #[test]
    fn source_level_pac_and_xf_fields_fail_before_netlisting() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::origin())
            .with_name_value("V1", "1");
        component.params = "pacmag=1 pacphase=30".to_owned();

        assert!(
            generator
                .generate_independent_source(&component, &nodes(), "V1")
                .is_none()
        );
        assert!(generator.errors().iter().any(|error| error.contains("PAC")));
    }

    #[test]
    fn unknown_source_assignments_fail_closed() {
        let schematic = SchematicState::default();
        let mut generator = NetlistGenerator::new(&schematic);
        let mut component = Component::new(1, ComponentType::CurrentSource, Point::origin())
            .with_name_value("I1", "1m");
        component.params = "invented_parameter=7".to_owned();

        assert!(
            generator
                .generate_independent_source(&component, &nodes(), "I1")
                .is_none()
        );
        assert!(
            generator
                .errors()
                .iter()
                .any(|error| error.contains("invented_parameter"))
        );
    }
}
