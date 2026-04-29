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

        let params = crate::properties::parse_params_string(&component.params);
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
            && (!value.is_finite() || value <= 0.0 || value > 1.0)
        {
            self.errors.push(format!(
                "Transformer '{}' has invalid coupling factor {} (expected 0 < k <= 1)",
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
        let primary_suffix =
            crate::properties::property_bridge::format_params_string(&primary_params);
        let secondary_suffix =
            crate::properties::property_bridge::format_params_string(&secondary_params);
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
        let params = crate::properties::parse_params_string(&component.params);
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
            && (!value.is_finite() || value <= 0.0 || value > 1.0)
        {
            self.errors.push(format!(
                "Coupled inductor '{}' has invalid coupling coefficient {} (expected 0 < k <= 1)",
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
        let params = crate::properties::parse_params_string(&component.params);
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
                "Inductor '{}' references '{}' but is missing a non-zero coupling factor",
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
        if !factor_value.is_finite() || factor_value <= 0.0 || factor_value > 1.0 {
            self.errors.push(format!(
                "Inductor '{}' has invalid coupling factor {} (expected 0 < k <= 1)",
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
