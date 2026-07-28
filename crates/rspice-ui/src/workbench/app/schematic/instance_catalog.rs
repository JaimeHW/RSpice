//! Resolved instance catalog used by replacement workflows.
//!
//! Primitive devices and ready library/cell/view masters are normalized into
//! one deterministic catalog. The dialog can therefore accept a human label,
//! a primitive type, or an exact L/C/V identity without guessing at commit
//! time.

use std::collections::HashMap;

use crate::state::{
    Component, ComponentType, Point, PropertyValue, SchematicReplacementParameter,
    SchematicReplacementTargetSpec, SchematicReplacementTerminal, SymbolResolver,
    library_cell_placement_candidates,
};

use crate::workbench::app_state::AppState;

#[derive(Debug, Clone)]
pub(crate) struct InstanceCatalogEntry {
    pub(crate) identity: String,
    pub(crate) display: String,
    pub(crate) template: Component,
    pub(crate) terminal_names: Vec<String>,
    pub(crate) parameters: Vec<SchematicReplacementParameter>,
    aliases: Vec<String>,
}

impl InstanceCatalogEntry {
    pub(crate) fn is_same_master(&self, component: &Component) -> bool {
        self.template.kind == component.kind
            && self.template.library_cell == component.library_cell
            && self.template.symbol_variant == component.symbol_variant
    }

    fn matches_exact(&self, query: &str) -> bool {
        normalize(&self.identity) == query
            || normalize(&self.display) == query
            || self.aliases.iter().any(|alias| normalize(alias) == query)
    }

    fn matches_partial(&self, query: &str) -> bool {
        normalize(&self.identity).contains(query)
            || normalize(&self.display).contains(query)
            || self
                .aliases
                .iter()
                .any(|alias| normalize(alias).contains(query))
    }

    /// Build the exact target contract from the same symbol and CDF sources
    /// used by placement and rendering. Display strings never participate in
    /// the commit identity.
    pub(crate) fn target_spec(
        &self,
        state: &AppState,
    ) -> Result<SchematicReplacementTargetSpec, String> {
        let resolved = self.template.library_cell.as_ref().and_then(|binding| {
            SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers)
                .resolve_binding(binding)
        });
        let positions = self.template.terminal_positions_resolved(resolved.as_ref());
        if positions.len() != self.terminal_names.len() {
            return Err(format!(
                "Replacement `{}` resolved {} symbol pins but declares {} netlist terminals.",
                self.identity,
                positions.len(),
                self.terminal_names.len()
            ));
        }
        let directions = self
            .template
            .library_cell
            .as_ref()
            .map(|binding| binding.terminal_dirs.as_slice())
            .unwrap_or_default();
        let terminals = positions
            .into_iter()
            .enumerate()
            .map(|(index, (resolved_name, offset))| {
                let declared_name = self
                    .terminal_names
                    .get(index)
                    .map(String::as_str)
                    .unwrap_or(&resolved_name);
                let aliases = terminal_aliases(self.template.kind, declared_name)
                    .into_iter()
                    .filter(|alias| !alias.eq_ignore_ascii_case(declared_name));
                let mut terminal =
                    SchematicReplacementTerminal::new(declared_name, offset).with_aliases(aliases);
                if let Some(direction) = directions.get(index).copied() {
                    terminal = terminal.with_direction(direction);
                }
                terminal
            })
            .collect::<Vec<_>>();
        let target = if let Some(binding) = self.template.library_cell.clone() {
            SchematicReplacementTargetSpec::library_cell(binding)
        } else {
            SchematicReplacementTargetSpec::primitive(self.template.kind)
        };
        Ok(target
            .with_value(self.template.value.clone())
            .with_default_params(self.template.params.clone())
            .with_symbol_variant(self.template.symbol_variant.clone())
            .with_terminals(terminals)
            .with_parameters(self.parameters.clone()))
    }
}

pub(crate) fn instance_catalog(state: &AppState) -> Vec<InstanceCatalogEntry> {
    let mut entries = primitive_entries(state);
    entries.extend(library_entries(state));
    entries.sort_by(|left, right| {
        left.display
            .to_ascii_lowercase()
            .cmp(&right.display.to_ascii_lowercase())
            .then_with(|| left.identity.cmp(&right.identity))
    });
    disambiguate_duplicate_displays(&mut entries);
    entries
}

pub(crate) fn resolve_instance_catalog_entry<'a>(
    entries: &'a [InstanceCatalogEntry],
    authored: &str,
) -> Result<&'a InstanceCatalogEntry, String> {
    let query = normalize(authored);
    if query.is_empty() {
        return Err("Replacement is required.".to_owned());
    }

    let exact = entries
        .iter()
        .filter(|entry| entry.matches_exact(&query))
        .collect::<Vec<_>>();
    if exact.len() == 1 {
        return Ok(exact[0]);
    }
    if exact.len() > 1 {
        return Err(ambiguous_message(authored, &exact));
    }

    let partial = entries
        .iter()
        .filter(|entry| entry.matches_partial(&query))
        .collect::<Vec<_>>();
    match partial.as_slice() {
        [entry] => Ok(*entry),
        [] => Err(format!(
            "Replacement `{}` is not a ready primitive or library/cell/view master.",
            authored.trim()
        )),
        entries => Err(ambiguous_message(authored, entries)),
    }
}

fn primitive_entries(state: &AppState) -> Vec<InstanceCatalogEntry> {
    crate::schematic::component_palette()
        .iter()
        .flat_map(|section| section.entries)
        .filter(|entry| !matches!(entry.kind, ComponentType::Ground | ComponentType::Port))
        .map(|entry| {
            let terminal_names = entry
                .kind
                .terminal_offsets()
                .iter()
                .map(|(name, _)| (*name).to_owned())
                .collect::<Vec<_>>();
            let primary = crate::properties::property_bridge::get_primary_property_name(entry.kind);
            let parameters = state
                .property_registry
                .get(entry.kind)
                .map(|sheet| {
                    sheet
                        .iter()
                        .filter(|definition| {
                            !definition.name.eq_ignore_ascii_case("name")
                                && !definition.name.eq_ignore_ascii_case("symbol")
                                && !definition.name.eq_ignore_ascii_case(primary)
                        })
                        .map(|definition| {
                            let mut parameter =
                                SchematicReplacementParameter::new(definition.name.clone());
                            if definition.required {
                                parameter = parameter.required();
                            }
                            if let Some(default) =
                                replacement_property_default(&definition.default_value)
                            {
                                parameter = parameter.with_default(default);
                            }
                            parameter
                        })
                        .collect()
                })
                .unwrap_or_default();
            let mut template = Component::new(0, entry.kind, Point::origin());
            template.value = entry.kind.default_value().to_owned();
            let identity = format!("primitive/{}", enum_key(entry.kind));
            let display = format!("{} \u{00b7} {} pin", entry.label, terminal_names.len());
            InstanceCatalogEntry {
                identity,
                display,
                template,
                terminal_names,
                parameters,
                aliases: vec![
                    entry.label.to_owned(),
                    entry.kind.display_name().to_owned(),
                    enum_key(entry.kind),
                ],
            }
        })
        .collect()
}

fn library_entries(state: &AppState) -> Vec<InstanceCatalogEntry> {
    let mut entries = Vec::new();
    for candidate in library_cell_placement_candidates(&state.library_manager, &state.workspace) {
        if !candidate.ready
            || candidate.parameter_contract_error.is_some()
            || candidate.binding.terminal_order.is_empty()
        {
            continue;
        }
        let identity = format!(
            "{}/{}/{}",
            candidate.library, candidate.cell, candidate.view
        );
        let display = format!(
            "{} \u{00b7} {} pin",
            candidate.cell,
            candidate.binding.terminal_order.len()
        );
        let terminal_names = candidate.binding.terminal_order.clone();
        let mut template = Component::new(0, ComponentType::CellInstance, Point::origin())
            .with_library_cell(candidate.binding);
        template.value.clone_from(&candidate.cell);
        entries.push(InstanceCatalogEntry {
            identity: identity.clone(),
            display,
            template,
            terminal_names,
            parameters: candidate
                .parameters
                .into_iter()
                .map(|contract| {
                    let mut parameter = SchematicReplacementParameter::new(contract.name)
                        .with_aliases(contract.aliases);
                    if contract.required {
                        parameter = parameter.required();
                    }
                    if let Some(default) = contract.default_value {
                        parameter = parameter.with_default(default);
                    }
                    parameter
                })
                .collect(),
            aliases: vec![
                candidate.cell.clone(),
                format!("{}/{}", candidate.library, candidate.cell),
                identity,
            ],
        });
    }
    entries
}

fn terminal_aliases(kind: ComponentType, name: &str) -> Vec<&'static str> {
    let normalized = name.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "+" | "p" | "pos" | "positive" => vec!["+", "p", "pos", "positive"],
        "-" | "n" | "neg" | "negative" => vec!["-", "n", "neg", "negative"],
        "a" | "anode" if kind == ComponentType::Diode => vec!["a", "anode"],
        "k" | "cathode" if kind == ComponentType::Diode => vec!["k", "cathode"],
        "d" | "drain"
            if matches!(
                kind,
                ComponentType::Nmos
                    | ComponentType::Pmos
                    | ComponentType::NVdmos
                    | ComponentType::PVdmos
                    | ComponentType::Njfet
                    | ComponentType::Pjfet
            ) =>
        {
            vec!["d", "drain"]
        }
        "g" | "gate"
            if matches!(
                kind,
                ComponentType::Nmos
                    | ComponentType::Pmos
                    | ComponentType::NVdmos
                    | ComponentType::PVdmos
                    | ComponentType::Njfet
                    | ComponentType::Pjfet
            ) =>
        {
            vec!["g", "gate"]
        }
        "s" | "source"
            if matches!(
                kind,
                ComponentType::Nmos
                    | ComponentType::Pmos
                    | ComponentType::NVdmos
                    | ComponentType::PVdmos
                    | ComponentType::Njfet
                    | ComponentType::Pjfet
            ) =>
        {
            vec!["s", "source"]
        }
        "b" | "bulk"
            if matches!(
                kind,
                ComponentType::Nmos
                    | ComponentType::Pmos
                    | ComponentType::NVdmos
                    | ComponentType::PVdmos
            ) =>
        {
            vec!["b", "bulk"]
        }
        "c" | "collector" if matches!(kind, ComponentType::NpnBjt | ComponentType::PnpBjt) => {
            vec!["c", "collector"]
        }
        "b" | "base" if matches!(kind, ComponentType::NpnBjt | ComponentType::PnpBjt) => {
            vec!["b", "base"]
        }
        "e" | "emitter" if matches!(kind, ComponentType::NpnBjt | ComponentType::PnpBjt) => {
            vec!["e", "emitter"]
        }
        "in+" | "noninverting" => vec!["in+", "noninverting"],
        "in-" | "inverting" => vec!["in-", "inverting"],
        "out" | "output" => vec!["out", "output"],
        _ => Vec::new(),
    }
}

fn replacement_property_default(value: &PropertyValue) -> Option<String> {
    match value {
        PropertyValue::String(value) if value.is_empty() => None,
        PropertyValue::String(value) => Some(format!(
            "\"{}\"",
            value.replace('\\', "\\\\").replace('"', "\\\"")
        )),
        _ => Some(value.display_string()),
    }
}

fn disambiguate_duplicate_displays(entries: &mut [InstanceCatalogEntry]) {
    let mut counts = display_counts(entries);
    for entry in entries.iter_mut() {
        if counts.get(&normalize(&entry.display)).copied().unwrap_or(0) > 1
            && entry.template.kind == ComponentType::CellInstance
            && let Some(binding) = entry.template.library_cell.as_ref()
        {
            entry.display = format!(
                "{}/{} \u{00b7} {} pin",
                binding.library,
                binding.cell,
                entry.terminal_names.len()
            );
        }
    }
    counts = display_counts(entries);
    for entry in entries.iter_mut() {
        if counts.get(&normalize(&entry.display)).copied().unwrap_or(0) > 1
            && entry.template.kind == ComponentType::CellInstance
            && let Some(binding) = entry.template.library_cell.as_ref()
        {
            entry.display = format!(
                "{}/{}/{} \u{00b7} {} pin",
                binding.library,
                binding.cell,
                binding.view,
                entry.terminal_names.len()
            );
        }
    }
}

fn display_counts(entries: &[InstanceCatalogEntry]) -> HashMap<String, usize> {
    let mut counts = HashMap::<String, usize>::new();
    for entry in entries {
        *counts.entry(normalize(&entry.display)).or_default() += 1;
    }
    counts
}

fn ambiguous_message(authored: &str, entries: &[&InstanceCatalogEntry]) -> String {
    let candidates = entries
        .iter()
        .take(5)
        .map(|entry| entry.identity.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Replacement `{}` is ambiguous; enter one exact identity: {candidates}.",
        authored.trim()
    )
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn enum_key(kind: ComponentType) -> String {
    format!("{kind:?}").to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_catalog_is_deterministic_resolvable_and_excludes_non_instances() {
        let state = AppState::default();
        let first = instance_catalog(&state);
        let second = instance_catalog(&state);
        assert_eq!(
            first
                .iter()
                .map(|entry| &entry.identity)
                .collect::<Vec<_>>(),
            second
                .iter()
                .map(|entry| &entry.identity)
                .collect::<Vec<_>>()
        );
        assert!(first.iter().all(|entry| !matches!(
            entry.template.kind,
            ComponentType::Ground | ComponentType::Port
        )));
        let resistor = resolve_instance_catalog_entry(&first, "resistor")
            .expect("primitive label resolves exactly");
        assert_eq!(resistor.template.kind, ComponentType::Resistor);
        assert_eq!(resistor.terminal_names, ["+", "-"]);
    }

    #[test]
    fn catalog_rejects_empty_unknown_and_ambiguous_queries() {
        let state = AppState::default();
        let entries = instance_catalog(&state);
        assert!(resolve_instance_catalog_entry(&entries, " ").is_err());
        assert!(resolve_instance_catalog_entry(&entries, "not-a-real-master").is_err());
        assert!(resolve_instance_catalog_entry(&entries, "voltage").is_err());
    }
}
