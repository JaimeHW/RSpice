//! Coupled inductor cards.
//!
//! Writes the mutual-inductance cards implied by coupling between placed
//! inductors, which have no schematic instance of their own.

use super::*;

impl<'a> NetlistGenerator<'a> {
    pub(super) fn collect_transformer_lines(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        for component in self.schematic.components.clone() {
            if component.kind != ComponentType::Transformer {
                continue;
            }

            if let Some(transformer_lines) = self.transformer_instance_lines(&component) {
                lines.extend(transformer_lines);
            }
        }
        lines
    }

    fn transformer_instance_lines(&mut self, component: &Component) -> Option<Vec<String>> {
        let terminals = component.terminal_positions();
        let node_names: Vec<String> = terminals
            .iter()
            .map(|(_, pos)| self.get_node_name(*pos))
            .collect();
        if node_names.len() != 4 {
            self.errors.push(format!(
                "Transformer '{}' must expose exactly four winding terminals",
                component.spice_instance_name()
            ));
            return None;
        }

        let primary_inductance = component.value.trim();
        if primary_inductance.is_empty() {
            self.errors.push(format!(
                "Transformer '{}' is missing a primary inductance",
                component.spice_instance_name()
            ));
            return None;
        }
        if let Ok(value) = primary_inductance.parse::<f64>()
            && (!value.is_finite() || value <= 0.0)
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid primary inductance {}",
                component.spice_instance_name(),
                primary_inductance
            ));
            return None;
        }

        let params = crate::state::parse_params_string(&component.params);
        let ratio = params
            .get("turns_ratio")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .unwrap_or("1");
        if let Ok(value) = ratio.parse::<f64>()
            && (!value.is_finite() || value <= 0.0)
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid turns ratio {}",
                component.spice_instance_name(),
                ratio
            ));
            return None;
        }

        let explicit_secondary = params
            .get("ls")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        if let Some(value) = explicit_secondary {
            if let Ok(parsed) = value.parse::<f64>()
                && (!parsed.is_finite() || parsed <= 0.0)
            {
                self.errors.push(format!(
                    "Transformer '{}' has invalid secondary inductance {}",
                    component.spice_instance_name(),
                    value
                ));
                return None;
            }
            if ratio != "1" {
                self.warnings.push(format!(
                    "Transformer '{}' specifies both turns_ratio and secondary inductance; using explicit secondary inductance",
                    component.spice_instance_name()
                ));
            }
        }

        let coupling = params
            .get("k")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .unwrap_or("0.999");
        if let Ok(value) = coupling.parse::<f64>()
            && (!value.is_finite() || !(-1.0..=1.0).contains(&value))
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid coupling factor {} (expected -1 <= k <= 1)",
                component.spice_instance_name(),
                coupling
            ));
            return None;
        }

        let secondary_inductance = explicit_secondary
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| Self::derive_secondary_inductance(primary_inductance, ratio));

        let mut primary_params = HashMap::new();
        if let Some(value) = params
            .get("rp")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            primary_params.insert("r".to_string(), value.to_string());
        }
        if let Some(value) = params
            .get("icp")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            primary_params.insert("ic".to_string(), value.to_string());
        }

        let mut secondary_params = HashMap::new();
        if let Some(value) = params
            .get("rs")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            secondary_params.insert("r".to_string(), value.to_string());
        }
        if let Some(value) = params
            .get("ics")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && *value != "0")
        {
            secondary_params.insert("ic".to_string(), value.to_string());
        }

        let base = Self::sanitize_transformer_base(&component.spice_instance_name(), component.id);
        let primary_name = format!("L{}_PRI", base);
        let secondary_name = format!("L{}_SEC", base);
        let coupling_name = format!("K{}", base);

        let primary_nodes = self.format_nodes(&node_names[0..2], 2);
        let secondary_nodes = self.format_nodes(&node_names[2..4], 2);
        let primary_suffix = crate::state::format_params_string(&primary_params);
        let secondary_suffix = crate::state::format_params_string(&secondary_params);
        let primary_line = if primary_suffix.is_empty() {
            format!("{} {} {}", primary_name, primary_nodes, primary_inductance)
        } else {
            format!(
                "{} {} {} {}",
                primary_name, primary_nodes, primary_inductance, primary_suffix
            )
        };
        let secondary_line = if secondary_suffix.is_empty() {
            format!(
                "{} {} {}",
                secondary_name, secondary_nodes, secondary_inductance
            )
        } else {
            format!(
                "{} {} {} {}",
                secondary_name, secondary_nodes, secondary_inductance, secondary_suffix
            )
        };
        let coupling_line = format!(
            "{} {} {} {}",
            coupling_name, primary_name, secondary_name, coupling
        );

        Some(vec![primary_line, secondary_line, coupling_line])
    }

    fn derive_secondary_inductance(primary_inductance: &str, turns_ratio: &str) -> String {
        format!(
            "(({})*(({})*({})))",
            primary_inductance, turns_ratio, turns_ratio
        )
    }

    fn sanitize_transformer_base(raw: &str, fallback_id: u64) -> String {
        let sanitized = raw
            .chars()
            .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
            .collect::<String>()
            .trim_matches('_')
            .to_string();

        if sanitized.is_empty() {
            fallback_id.to_string()
        } else {
            sanitized
        }
    }

    pub(super) fn collect_coupling_lines(&mut self) -> Vec<String> {
        let inductor_lookup = self.build_inductor_lookup();
        let mut emitted: BTreeMap<String, (String, String, String)> = BTreeMap::new();

        for component in &self.schematic.components {
            if component.kind == ComponentType::CoupledInductor
                && let Some((key, coefficient, line, source)) =
                    self.explicit_coupling_line(component, &inductor_lookup)
            {
                self.insert_coupling_line(&mut emitted, key, coefficient, line, source);
            }
        }

        for component in &self.schematic.components {
            if Self::is_couplable_inductor(component.kind)
                && let Some((key, coefficient, line, source)) =
                    self.metadata_coupling_line(component, &inductor_lookup)
            {
                self.insert_coupling_line(&mut emitted, key, coefficient, line, source);
            }
        }

        emitted
            .into_values()
            .map(|(line, _, _)| line)
            .collect::<Vec<_>>()
    }

    fn build_inductor_lookup(&mut self) -> HashMap<String, String> {
        let mut lookup = HashMap::new();

        for component in &self.schematic.components {
            if !Self::is_couplable_inductor(component.kind) {
                continue;
            }

            let emitted = self.instance_name(component);
            self.register_inductor_alias(&mut lookup, emitted.as_str(), emitted.as_str());
            if !component.name.trim().is_empty() {
                self.register_inductor_alias(&mut lookup, component.name.trim(), emitted.as_str());
            }
        }

        lookup
    }

    fn register_inductor_alias(
        &mut self,
        lookup: &mut HashMap<String, String>,
        alias: &str,
        emitted: &str,
    ) {
        let key = alias.trim().to_ascii_uppercase();
        if key.is_empty() {
            return;
        }

        if let Some(existing) = lookup.get(&key) {
            if existing != emitted {
                self.errors.push(format!(
                    "Coupled inductor reference '{}' is ambiguous; rename the inductors so each winding has a unique instance name",
                    alias
                ));
            }
            return;
        }

        lookup.insert(key, emitted.to_string());
    }

    fn explicit_coupling_line(
        &mut self,
        component: &Component,
        inductor_lookup: &HashMap<String, String>,
    ) -> Option<(String, String, String, String)> {
        let params = crate::state::parse_params_string(&component.params);
        let raw_windings = params.get("inductors").cloned().unwrap_or_else(|| {
            ["l1", "l2", "l3", "l4"]
                .iter()
                .filter_map(|key| params.get(*key))
                .cloned()
                .collect::<Vec<_>>()
                .join(" ")
        });
        let winding_refs = Self::parse_inductor_list(&raw_windings);
        if winding_refs.len() < 2 {
            self.errors.push(format!(
                "Coupled inductor '{}' must reference at least two inductor instances",
                component.spice_instance_name()
            ));
            return None;
        }

        let coefficient = component.value.trim();
        if coefficient.is_empty() {
            self.errors.push(format!(
                "Coupled inductor '{}' is missing a coupling coefficient",
                component.spice_instance_name()
            ));
            return None;
        }
        if let Ok(value) = coefficient.parse::<f64>()
            && (!value.is_finite() || !(-1.0..=1.0).contains(&value))
        {
            self.errors.push(format!(
                "Coupled inductor '{}' has invalid coupling coefficient {} (expected -1 <= k <= 1)",
                component.spice_instance_name(),
                coefficient
            ));
            return None;
        }

        let emitted_windings = self.resolve_inductor_refs(
            &winding_refs,
            inductor_lookup,
            component.spice_instance_name().as_str(),
        )?;
        let key = Self::coupling_key(&emitted_windings);
        let line = format!(
            "{} {} {}",
            self.instance_name(component),
            emitted_windings.join(" "),
            coefficient
        );
        Some((
            key,
            coefficient.to_string(),
            line,
            format!("coupling '{}'", component.spice_instance_name()),
        ))
    }

    fn metadata_coupling_line(
        &mut self,
        component: &Component,
        inductor_lookup: &HashMap<String, String>,
    ) -> Option<(String, String, String, String)> {
        let params = crate::state::parse_params_string(&component.params);
        let coupled_to = params
            .get("coupled_to")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());
        let factor = params
            .get("coupling_factor")
            .map(|value| value.trim())
            .filter(|value| !value.is_empty());

        let factor_is_default_zero = factor
            .and_then(|value| value.parse::<f64>().ok())
            .is_some_and(|value| value == 0.0);
        if coupled_to.is_none() && (factor.is_none() || factor_is_default_zero) {
            return None;
        }

        let Some(coupled_to) = coupled_to else {
            self.errors.push(format!(
                "Inductor '{}' defines a coupling factor but no target winding",
                component.spice_instance_name()
            ));
            return None;
        };
        let Some(factor) = factor else {
            self.errors.push(format!(
                "Inductor '{}' references '{}' but is missing a coupling factor",
                component.spice_instance_name(),
                coupled_to
            ));
            return None;
        };
        let Ok(factor_value) = factor.parse::<f64>() else {
            self.errors.push(format!(
                "Inductor '{}' has non-numeric coupling factor '{}'",
                component.spice_instance_name(),
                factor
            ));
            return None;
        };
        if !factor_value.is_finite() || !(-1.0..=1.0).contains(&factor_value) {
            self.errors.push(format!(
                "Inductor '{}' has invalid coupling factor {} (expected -1 <= k <= 1)",
                component.spice_instance_name(),
                factor
            ));
            return None;
        }

        let this_name = self.instance_name(component);
        let emitted_windings = self.resolve_inductor_refs(
            &[this_name.clone(), coupled_to.to_string()],
            inductor_lookup,
            component.spice_instance_name().as_str(),
        )?;
        let key = Self::coupling_key(&emitted_windings);
        let line = format!(
            "{} {} {}",
            Self::derived_coupling_name(&emitted_windings),
            emitted_windings.join(" "),
            factor
        );
        Some((
            key,
            factor.to_string(),
            line,
            format!("inductor '{}'", component.spice_instance_name()),
        ))
    }

    fn resolve_inductor_refs(
        &mut self,
        refs: &[String],
        inductor_lookup: &HashMap<String, String>,
        owner: &str,
    ) -> Option<Vec<String>> {
        let mut resolved = Vec::with_capacity(refs.len());
        let mut seen = HashSet::new();

        for reference in refs {
            let key = reference.trim().to_ascii_uppercase();
            let Some(emitted) = inductor_lookup.get(&key) else {
                self.errors.push(format!(
                    "Coupling '{}' references unknown inductor '{}'",
                    owner, reference
                ));
                return None;
            };

            if !seen.insert(emitted.clone()) {
                self.errors.push(format!(
                    "Coupling '{}' references '{}' more than once",
                    owner, reference
                ));
                return None;
            }
            resolved.push(emitted.clone());
        }

        Some(resolved)
    }

    fn insert_coupling_line(
        &mut self,
        emitted: &mut BTreeMap<String, (String, String, String)>,
        key: String,
        coefficient: String,
        line: String,
        source: String,
    ) {
        if let Some((_, existing_coefficient, existing_source)) = emitted.get(&key) {
            if existing_coefficient != &coefficient {
                self.errors.push(format!(
                    "Conflicting coupling definitions for [{}]: {} from {} vs {} from {}",
                    key, existing_coefficient, existing_source, coefficient, source
                ));
            }
            return;
        }

        emitted.insert(key, (line, coefficient, source));
    }

    fn parse_inductor_list(raw: &str) -> Vec<String> {
        raw.split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    }

    fn coupling_key(windings: &[String]) -> String {
        let mut normalized = windings
            .iter()
            .map(|name| name.to_ascii_uppercase())
            .collect::<Vec<_>>();
        normalized.sort();
        normalized.join("|")
    }

    fn derived_coupling_name(windings: &[String]) -> String {
        let mut normalized = windings
            .iter()
            .map(|name| {
                name.chars()
                    .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        normalized.sort();
        format!("K{}", normalized.join("_"))
    }

    fn is_couplable_inductor(kind: ComponentType) -> bool {
        matches!(
            kind,
            ComponentType::Inductor | ComponentType::SaturableInductor
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::NetLabel;
    use num_complex::Complex64;
    use rspice_core::{Engine, Netlist};

    fn circuit(kind: ComponentType, coefficient: &str, reversed: bool) -> SchematicState {
        let mut schematic = SchematicState::default();
        let mut source = Component::new(1, ComponentType::VoltageSource, Point::origin())
            .with_name_value("V1", "0.5");
        source.params = "ac=1".to_owned();
        let resistor = Component::new(2, ComponentType::Resistor, Point::new(200, 0))
            .with_name_value("R1", "50");
        let mut add = |component: Component, nodes: &[&str]| {
            for ((_, point), node) in component.terminal_positions().iter().zip(nodes) {
                schematic.net_labels.push(NetLabel::new(
                    schematic.net_labels.len() as u64 + 1,
                    *point,
                    *node,
                ));
            }
            schematic.components.push(component);
        };
        add(source, &["in", "0"]);
        add(resistor, &["in", "p"]);
        let secondary = if reversed { ["0", "s"] } else { ["s", "0"] };
        if kind == ComponentType::Transformer {
            let mut transformer =
                Component::new(3, kind, Point::new(400, 0)).with_name_value("T1", "10m");
            transformer.params = format!("ls=40m k={coefficient}");
            add(transformer, &["p", "0", secondary[0], secondary[1]]);
        } else {
            let mut primary = Component::new(3, ComponentType::Inductor, Point::new(400, 0))
                .with_name_value("L1", "10m");
            if kind == ComponentType::Inductor {
                primary.params = format!("coupled_to=L2 coupling_factor={coefficient}");
            }
            add(primary, &["p", "0"]);
            add(
                Component::new(4, ComponentType::Inductor, Point::new(600, 0))
                    .with_name_value("L2", "40m"),
                &secondary,
            );
            if kind == ComponentType::CoupledInductor {
                let mut coupling =
                    Component::new(5, kind, Point::new(800, 0)).with_name_value("K1", coefficient);
                coupling.params = "inductors=\"L1 L2\"".to_owned();
                add(coupling, &[]);
            }
        }
        schematic
    }

    fn generated(schematic: &SchematicState) -> super::super::NetlistResult {
        let buffers = HashMap::new();
        let hierarchy = HierarchySource::from_buffers(&buffers);
        generate_netlist_hierarchical(schematic, &[], &hierarchy)
    }

    #[test]
    fn signed_schematic_coupling_preserves_observable_secondary_polarity() {
        for kind in [
            ComponentType::CoupledInductor,
            ComponentType::Inductor,
            ComponentType::Transformer,
        ] {
            for coefficient in [-1.0, -0.75, 0.0, 0.75, 1.0] {
                for reversed in [false, true] {
                    let schematic = circuit(kind, &coefficient.to_string(), reversed);
                    let generated = generated(&schematic);
                    assert!(
                        generated.errors.is_empty(),
                        "{kind:?}, {coefficient}: {:?}",
                        generated.errors
                    );
                    let deck = Netlist::parse(&generated.netlist).unwrap();
                    let engine = Engine::default();
                    for point in engine.run_ac(&deck, &[100.0, 1000.0, 10000.0]).unwrap() {
                        let omega = std::f64::consts::TAU * point.frequency;
                        let orientation = if reversed { -1.0 } else { 1.0 };
                        let mutual = coefficient * orientation * 0.02;
                        let expected = Complex64::new(0.0, omega * mutual)
                            / Complex64::new(50.0, omega * 0.01);
                        let secondary = point
                            .node_names
                            .iter()
                            .position(|node| node.eq_ignore_ascii_case("s"))
                            .unwrap_or_else(|| {
                                panic!(
                                    "Secondary node missing from {:?}:\n{}",
                                    point.node_names, generated.netlist
                                )
                            });
                        assert!(
                            (point.voltages[secondary] - expected).norm() < 2e-12,
                            "{kind:?}, k={coefficient}, reversed={reversed}: {} vs {expected}\n{}",
                            point.voltages[secondary],
                            generated.netlist
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn invalid_schematic_coupling_is_refused_without_clamping() {
        for kind in [
            ComponentType::CoupledInductor,
            ComponentType::Inductor,
            ComponentType::Transformer,
        ] {
            for coefficient in ["-1.01", "1.01", "NaN", "inf", "-inf"] {
                let schematic = circuit(kind, coefficient, false);
                let before = schematic.components.clone();
                let result = generated(&schematic);
                assert!(
                    result
                        .errors
                        .iter()
                        .any(|error| error.contains("invalid coupling")),
                    "{kind:?}, k={coefficient}: {:?}",
                    result.errors
                );
                assert_eq!(schematic.components, before);
            }
        }
    }

    #[test]
    fn inductor_coupling_property_accepts_signed_values_and_keeps_absence_distinct() {
        use crate::state::property_types::{PropertyRegistry, PropertyValue};

        let registry = PropertyRegistry::new();
        let definition = registry
            .get(ComponentType::Inductor)
            .unwrap()
            .get("coupling_factor")
            .unwrap();
        for value in [-1.0, -0.75, -0.0, 0.0, 0.75, 1.0] {
            definition.validate(&PropertyValue::number(value)).unwrap();
        }
        for value in [-1.01, 1.01, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(definition.validate(&PropertyValue::number(value)).is_err());
        }

        for params in ["", "coupling_factor=0", "coupling_factor=-0"] {
            let mut schematic = circuit(ComponentType::Inductor, "0", false);
            schematic.components[2].params = params.to_owned();
            let result = generated(&schematic);
            assert!(result.errors.is_empty(), "{:?}", result.errors);
            let deck = Netlist::parse(&result.netlist).unwrap();
            assert!(!deck.elements.iter().any(|element| matches!(
                element.kind,
                rspice_core::netlist::ElementKind::Coupling { .. }
            )));
        }
        let mut schematic = circuit(ComponentType::Inductor, "-0.75", false);
        schematic.components[2].params = "coupling_factor=-0.75".to_owned();
        assert!(
            generated(&schematic)
                .errors
                .iter()
                .any(|error| error.contains("no target winding"))
        );
        schematic.components[2].params = "coupled_to=L2".to_owned();
        assert!(
            generated(&schematic)
                .errors
                .iter()
                .any(|error| error.contains("missing a coupling factor"))
        );
    }

    #[test]
    fn reciprocal_coupling_definitions_must_agree_in_polarity() {
        let mut schematic = circuit(ComponentType::Inductor, "-0.75", false);
        schematic.components[3].params = "coupled_to=L1 coupling_factor=-0.75".to_owned();
        let result = generated(&schematic);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let deck = Netlist::parse(&result.netlist).unwrap();
        assert_eq!(
            deck.elements
                .iter()
                .filter(|element| matches!(
                    element.kind,
                    rspice_core::netlist::ElementKind::Coupling { .. }
                ))
                .count(),
            1
        );

        schematic.components[3].params = "coupled_to=L1 coupling_factor=0.75".to_owned();
        assert!(
            generated(&schematic)
                .errors
                .iter()
                .any(|error| error.contains("Conflicting coupling definitions"))
        );
    }
}
