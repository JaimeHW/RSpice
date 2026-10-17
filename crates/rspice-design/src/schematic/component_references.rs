//! Structural instance references when a selection is replicated.
//!
//! Names are allocated for the whole replica before its references change.
//! A target outside the copied selection keeps its authored reference.

use std::collections::HashMap;

use super::{
    component::{CardRole, Component, WrittenCard, written_cards},
    component_type::ComponentType,
    document::SchematicDocument,
    replacement::{format_replacement_parameters, parse_replacement_parameters_strict},
};

/// Validated source references, reusable for every member of an array.
pub struct PreparedCopyReferences {
    card_names: HashMap<String, Option<usize>>,
    winding_aliases: HashMap<String, Option<usize>>,
    parameters: Vec<Option<HashMap<String, String>>>,
}

impl PreparedCopyReferences {
    pub fn new(sources: &[Component]) -> Result<Self, String> {
        Self::for_operation(sources, "copy")
    }

    fn for_operation(sources: &[Component], operation: &str) -> Result<Self, String> {
        let mut card_names = HashMap::new();
        let mut winding_aliases = HashMap::new();
        for (index, source) in sources.iter().enumerate() {
            let written = source.written_element_name();
            insert_alias(&mut card_names, &written, index);
            if matches!(
                source.kind,
                ComponentType::Inductor | ComponentType::SaturableInductor
            ) {
                for name in [source.name.trim(), &written] {
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
    pub fn apply(&self, copies: &mut [Component]) {
        self.apply_selected(copies, |_| true);
    }

    fn apply_selected(&self, copies: &mut [Component], selected: impl Fn(usize) -> bool) {
        assert_eq!(copies.len(), self.parameters.len());
        let names: Vec<_> = copies.iter().map(Component::written_element_name).collect();
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

/// Prepare the complete local component/reference edit without publishing
/// geometry, history, or any project-level references.
pub fn prepare_component_edit(
    document: &SchematicDocument,
    expected: &Component,
    candidate: Component,
) -> Result<Vec<Component>, String> {
    prepare_component_edits(document, std::iter::once((expected, candidate)))
}

/// Resolve all targets from their original identities, then publish their
/// final names together. A swap must never pass through a duplicate name.
pub fn prepare_component_renames(
    document: &SchematicDocument,
    names: &std::collections::BTreeMap<u64, String>,
) -> Result<Vec<Component>, String> {
    let edits = document
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
    prepare_component_edits(document, edits)
}

fn prepare_component_edits<'a>(
    document: &SchematicDocument,
    edits: impl IntoIterator<Item = (&'a Component, Component)>,
) -> Result<Vec<Component>, String> {
    let mut components = document.components.clone();
    let indices: HashMap<_, _> = document
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
        if &document.components[index] != expected {
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
    let mut written_names = HashMap::new();
    for (index, component) in components.iter().enumerate() {
        insert_alias(&mut names, &component.name, index);
        insert_alias(&mut written_names, &component.written_element_name(), index);
    }
    for &index in &renamed {
        if names.get(&components[index].name.to_ascii_lowercase()) != Some(&Some(index))
            || written_names.get(
                &components[index]
                    .written_element_name()
                    .to_ascii_lowercase(),
            ) != Some(&Some(index))
        {
            return Err(
                "The renamed component would duplicate an existing SPICE designator.".to_owned(),
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
                    document.components[index].name.clone(),
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

/// What one authored instance reference names within its cellview.
#[derive(Debug, Clone, PartialEq)]
pub enum InstanceReference<'a> {
    /// The one card it names, and the instance that makes the deck write it.
    /// The deck writes the reference as `card`, which is the name the engine
    /// knows the element by.
    Names {
        instance: &'a Component,
        card: String,
    },
    /// More than one card answers to it: the instances that write them, once
    /// each, in the order they are drawn.
    Ambiguous(Vec<&'a Component>),
    /// No card the cellview writes answers to it.
    Unresolved,
}

/// Resolve a reference one instance makes to another — a current-controlled
/// source's controlling element, for one — the way the deck must write it.
///
/// The engine resolves the reference against the names the deck writes, and
/// those are the cards [`written_cards`] lists: a source drawn `sense` is
/// written `Vsense`, a limited VCVS drawn `E1` is written `B1`, and a
/// current-controlled source that names no controlling element of its own is
/// written beside a sense source `VSENSE_H1`, which another source may name.
/// So a reference that already is a written name names that card, and failing
/// that, a reference that is an instance's drawn name names the element the
/// deck writes for the instance the author drew. Both compare without case,
/// as the engine does. Writing a drawn name through unresolved would point the
/// engine at an element the deck does not have — which is also why an
/// instance the deck writes no element for, such as a transformer, is not
/// named by its drawn name.
pub fn resolve_instance_reference<'a>(
    components: &'a [Component],
    reference: &str,
) -> InstanceReference<'a> {
    let reference = reference.trim();
    if reference.is_empty() {
        return InstanceReference::Unresolved;
    }
    let cards = written_cards(components);
    let written = |matches: &dyn Fn(&Component, &WrittenCard) -> bool| {
        let mut found = Vec::new();
        for component in components {
            for card in cards.get(&component.id).into_iter().flatten() {
                if matches(component, card) {
                    found.push((component, card.name.clone()));
                }
            }
        }
        found
    };
    let mut candidates = written(&|_, card| card.name.eq_ignore_ascii_case(reference));
    if candidates.is_empty() {
        candidates = written(&|component, card| {
            card.role == CardRole::Element && component.name.trim().eq_ignore_ascii_case(reference)
        });
    }
    if candidates.len() > 1 {
        let mut instances = candidates
            .into_iter()
            .map(|(instance, _)| instance)
            .collect::<Vec<_>>();
        instances.dedup_by_key(|instance| instance.id);
        return InstanceReference::Ambiguous(instances);
    }
    match candidates.pop() {
        Some((instance, card)) => InstanceReference::Names { instance, card },
        None => InstanceReference::Unresolved,
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
