//! Structural instance references when a selection is replicated.
//!
//! Names are allocated for the whole replica before its references change.
//! A target outside the copied selection keeps its authored reference.

use std::collections::HashMap;

use super::{
    Component, ComponentType, format_replacement_parameters, parse_replacement_parameters_strict,
};

/// Validated source references, reusable for every member of an array.
pub(crate) struct PreparedCopyReferences {
    card_names: HashMap<String, Option<usize>>,
    winding_aliases: HashMap<String, Option<usize>>,
    parameters: Vec<Option<HashMap<String, String>>>,
}

impl PreparedCopyReferences {
    pub(crate) fn new(sources: &[Component]) -> Result<Self, String> {
        Self::for_operation(sources, "copy")
    }

    fn for_operation(sources: &[Component], operation: &str) -> Result<Self, String> {
        let mut card_names = HashMap::new();
        let mut winding_aliases = HashMap::new();
        for (index, source) in sources.iter().enumerate() {
            let emitted = source.emitted_instance_name();
            insert_alias(&mut card_names, &emitted, index);
            if matches!(
                source.kind,
                ComponentType::Inductor | ComponentType::SaturableInductor
            ) {
                for name in [source.name.trim(), &emitted] {
                    insert_alias(&mut winding_aliases, name, index);
                }
            }
        }
        let mut prepared = Self {
            card_names,
            winding_aliases,
            parameters: Vec::new(),
        };
        let mut parameters = Vec::with_capacity(sources.len());
        for source in sources {
            if reference_keys(source.kind).is_empty() {
                parameters.push(None);
                continue;
            }
            let parsed = parse_replacement_parameters_strict(&source.params)
                .map_err(|error| format!("Cannot {operation} {}: {error}", source.name))?;
            for key in reference_keys(source.kind) {
                if let Some(value) = parsed.get(*key) {
                    for token in value
                        .split(reference_separator)
                        .filter(|token| !token.is_empty())
                    {
                        if prepared
                            .aliases(source.kind)
                            .get(&token.to_ascii_lowercase())
                            == Some(&None)
                        {
                            return Err(format!(
                                "Cannot {operation} {}: {key} reference '{token}' names more than one selected instance",
                                source.name
                            ));
                        }
                    }
                }
            }
            parameters.push(Some(parsed));
        }
        prepared.parameters = parameters;
        Ok(prepared)
    }

    fn aliases(&self, owner: ComponentType) -> &HashMap<String, Option<usize>> {
        if matches!(owner, ComponentType::Cccs | ComponentType::Ccvs) {
            &self.card_names
        } else {
            &self.winding_aliases
        }
    }

    /// `copies` has the source order and final identities for one replica.
    pub(crate) fn apply(&self, copies: &mut [Component]) {
        self.apply_selected(copies, |_| true);
    }

    fn apply_selected(&self, copies: &mut [Component], selected: impl Fn(usize) -> bool) {
        assert_eq!(copies.len(), self.parameters.len());
        let names: Vec<_> = copies
            .iter()
            .map(Component::emitted_instance_name)
            .collect();
        for (copy, parameters) in copies.iter_mut().zip(&self.parameters) {
            let Some(parameters) = parameters else {
                continue;
            };
            let mut parameters = parameters.clone();
            let mut changed = false;
            for key in reference_keys(copy.kind) {
                let Some(value) = parameters.get_mut(*key) else {
                    continue;
                };
                let remapped = map_reference_tokens(value, |token| {
                    self.aliases(copy.kind)
                        .get(&token.to_ascii_lowercase())
                        .and_then(|index| index.filter(|index| selected(*index)))
                        .map(|index| names[index].as_str())
                });
                if *value != remapped {
                    *value = remapped;
                    changed = true;
                }
            }
            if changed {
                copy.params = format_replacement_parameters(&parameters);
            }
        }
    }
}

impl super::SchematicState {
    /// Prepare the complete local component/reference edit without publishing
    /// geometry, history, or any project-level references.
    pub(crate) fn prepare_component_edit(
        &self,
        expected: &Component,
        candidate: Component,
    ) -> Result<Vec<Component>, String> {
        self.prepare_component_edits(std::iter::once((expected, candidate)))
    }

    /// Resolve all targets from their original identities, then publish their
    /// final names together. A swap must never pass through a duplicate name.
    pub(crate) fn prepare_component_renames(
        &self,
        names: &std::collections::BTreeMap<u64, String>,
    ) -> Result<Vec<Component>, String> {
        let edits = self
            .components
            .iter()
            .filter_map(|expected| {
                let name = names.get(&expected.id)?;
                let mut candidate = expected.clone();
                candidate.name.clone_from(name);
                Some((expected, candidate))
            })
            .collect::<Vec<_>>();
        if edits.len() != names.len() {
            return Err("An annotated component no longer exists.".to_owned());
        }
        self.prepare_component_edits(edits)
    }

    fn prepare_component_edits<'a>(
        &self,
        edits: impl IntoIterator<Item = (&'a Component, Component)>,
    ) -> Result<Vec<Component>, String> {
        let mut components = self.components.clone();
        let indices: HashMap<_, _> = self
            .components
            .iter()
            .enumerate()
            .map(|(index, component)| (component.id, index))
            .collect();
        let mut renamed = std::collections::HashSet::new();
        for (expected, candidate) in edits {
            let index = *indices
                .get(&expected.id)
                .ok_or("The selected component no longer exists.")?;
            if &self.components[index] != expected {
                return Err("The selected component changed before commit.".to_owned());
            }
            if candidate.id != expected.id || candidate.kind != expected.kind {
                return Err(
                    "A property edit cannot replace the component's identity or type.".to_owned(),
                );
            }
            if candidate.name != expected.name && !expected.kind.spice_prefix().is_empty() {
                candidate.validate_reference_designator(&candidate.name)?;
                renamed.insert(index);
            }
            components[index] = candidate;
        }
        if renamed.is_empty() {
            return Ok(components);
        }
        let mut names = HashMap::new();
        let mut emitted_names = HashMap::new();
        for (index, component) in components.iter().enumerate() {
            insert_alias(&mut names, &component.name, index);
            insert_alias(
                &mut emitted_names,
                &component.emitted_instance_name(),
                index,
            );
        }
        for &index in &renamed {
            if names.get(&components[index].name.to_ascii_lowercase()) != Some(&Some(index))
                || emitted_names.get(
                    &components[index]
                        .emitted_instance_name()
                        .to_ascii_lowercase(),
                ) != Some(&Some(index))
            {
                return Err(
                    "The renamed component would duplicate an existing SPICE designator."
                        .to_owned(),
                );
            }
        }
        // Resolve references using the old identities, but keep the complete
        // edited parameter draft. Preparing from the old component would
        // overwrite simultaneous parameter edits when a reference is remapped.
        let names: Vec<_> = renamed
            .iter()
            .map(|&index| {
                (
                    index,
                    std::mem::replace(
                        &mut components[index].name,
                        self.components[index].name.clone(),
                    ),
                )
            })
            .collect();
        let references = PreparedCopyReferences::for_operation(&components, "rename")?;
        for (index, name) in names {
            components[index].name = name;
        }
        references.apply_selected(&mut components, |target| renamed.contains(&target));
        Ok(components)
    }
}

fn insert_alias(aliases: &mut HashMap<String, Option<usize>>, name: &str, index: usize) {
    if !name.is_empty() {
        aliases
            .entry(name.to_ascii_lowercase())
            .and_modify(|owner| {
                if *owner != Some(index) {
                    *owner = None;
                }
            })
            .or_insert(Some(index));
    }
}

fn reference_keys(kind: ComponentType) -> &'static [&'static str] {
    match kind {
        ComponentType::CoupledInductor => {
            &["inductors", "l1", "l2", "l3", "l4", "l5", "l6", "l7", "l8"]
        }
        ComponentType::Inductor | ComponentType::SaturableInductor => &["coupled_to"],
        ComponentType::Cccs | ComponentType::Ccvs => &["vref"],
        _ => &[],
    }
}

fn reference_separator(character: char) -> bool {
    character.is_ascii_whitespace() || matches!(character, ',' | '\'' | '"')
}

fn map_reference_tokens<'a>(raw: &str, replacement: impl Fn(&str) -> Option<&'a str>) -> String {
    let mut result = String::with_capacity(raw.len());
    let mut start = 0;
    for (index, character) in raw.char_indices() {
        if reference_separator(character) {
            let token = &raw[start..index];
            result.push_str(replacement(token).unwrap_or(token));
            result.push(character);
            start = index + character.len_utf8();
        }
    }
    let token = &raw[start..];
    result.push_str(replacement(token).unwrap_or(token));
    result
}

#[cfg(test)]
mod tests;
