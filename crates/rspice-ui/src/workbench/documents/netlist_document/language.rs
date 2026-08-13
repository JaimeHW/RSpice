//! Project-scoped SPICE symbols and cross-file references.
//!
//! The index is derived only from the authenticated root/dependency closure.
//! It never reads an ambient filesystem path, so native and browser builds
//! answer completion and navigation from the same exact source set.

use std::sync::Arc;

use super::{ActiveNetlistDocument, AppState, canonical_root_document};

#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistRenameDialogState {
    pub(crate) open: bool,
    pub(crate) original: String,
    pub(crate) replacement: String,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ProjectSymbolKind {
    Model,
    Subcircuit,
    Parameter,
    Function,
}

impl ProjectSymbolKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Subcircuit => "subckt",
            Self::Parameter => "param",
            Self::Function => "function",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectSourceLocation {
    pub(crate) root: ActiveNetlistDocument,
    pub(crate) dependency_identity: Option<String>,
    pub(crate) display_name: String,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectSymbol {
    pub(crate) name: String,
    pub(crate) kind: ProjectSymbolKind,
    pub(crate) detail: String,
    pub(crate) definition: ProjectSourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectReference {
    pub(crate) name: String,
    pub(crate) kind: ProjectSymbolKind,
    pub(crate) location: ProjectSourceLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectHierarchyInstance {
    pub(crate) path: String,
    pub(crate) target: String,
    pub(crate) location: ProjectSourceLocation,
    pub(crate) recursive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectSymbolHover {
    pub(crate) title: String,
    pub(crate) detail: String,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistProjectIndex {
    symbols: Vec<ProjectSymbol>,
    references: Vec<ProjectReference>,
    pending_model_references: Vec<PendingModelReference>,
    pending_expression_references: Vec<ProjectReference>,
    raw_hierarchy: Vec<RawHierarchyInstance>,
    hierarchy: Vec<ProjectHierarchyInstance>,
    hierarchy_truncated: bool,
    source_count: usize,
}

#[derive(Debug, Clone)]
struct PendingModelReference {
    candidates: Vec<ProjectReference>,
    retain_first_if_unresolved: bool,
}

#[derive(Debug, Clone)]
struct RawHierarchyInstance {
    parent: Option<String>,
    instance: String,
    target: String,
    location: ProjectSourceLocation,
}

const MAX_EXPANDED_HIERARCHY_INSTANCES: usize = 100_000;

impl NetlistProjectIndex {
    fn build(root: ActiveNetlistDocument, document: &crate::state::NetlistDocument) -> Self {
        let root_name = match root {
            ActiveNetlistDocument::Generated => "generated.sp".to_owned(),
            ActiveNetlistDocument::OwnedSource => "project root".to_owned(),
            ActiveNetlistDocument::GeneratedDiff => return Self::default(),
        };
        let mut index = Self::default();
        index.index_source(root, None, root_name, document.source());
        for dependency in document.dependencies() {
            let Some(source) = dependency.source() else {
                continue;
            };
            index.index_source(
                root,
                Some(dependency.locator().logical_identity().to_owned()),
                dependency.locator().display_name().to_owned(),
                source,
            );
        }
        index.resolve_pending_model_references();
        index.resolve_pending_expression_references();
        index.expand_hierarchy();
        index.symbols.sort_by(|left, right| {
            left.name
                .to_ascii_lowercase()
                .cmp(&right.name.to_ascii_lowercase())
                .then_with(|| {
                    left.definition
                        .display_name
                        .cmp(&right.definition.display_name)
                })
                .then_with(|| left.definition.line.cmp(&right.definition.line))
        });
        index
    }

    fn index_source(
        &mut self,
        root: ActiveNetlistDocument,
        dependency_identity: Option<String>,
        display_name: String,
        source: &str,
    ) {
        self.source_count = self.source_count.saturating_add(1);
        let source_index = crate::state::NetlistSourceIndex::parse(source);
        let mut current_subcircuit = None::<String>;
        for entry in source_index.outline().entries() {
            let positioned_tokens = crate::state::netlist_document::card_tokens_with_columns(
                source_index.card(entry.line()),
            );
            let tokens = positioned_tokens
                .iter()
                .map(|(token, _)| token.clone())
                .collect::<Vec<_>>();
            if tokens.is_empty() {
                continue;
            }
            let location = |token_index: usize| ProjectSourceLocation {
                root,
                dependency_identity: dependency_identity.clone(),
                display_name: display_name.clone(),
                line: entry.line(),
                column: positioned_tokens
                    .get(token_index)
                    .map_or(entry.column(), |(_, column)| *column),
            };
            match entry.kind() {
                crate::state::OutlineEntryKind::Model => {
                    if let Some(name) = tokens.get(1) {
                        self.symbols.push(ProjectSymbol {
                            name: name.clone(),
                            kind: ProjectSymbolKind::Model,
                            detail: tokens.get(2).cloned().unwrap_or_default(),
                            definition: location(1),
                        });
                    }
                }
                crate::state::OutlineEntryKind::Subcircuit => {
                    if let Some(name) = tokens.get(1) {
                        let port_count = tokens
                            .iter()
                            .skip(2)
                            .take_while(|token| {
                                !token.eq_ignore_ascii_case("params:") && !token.contains('=')
                            })
                            .count();
                        self.symbols.push(ProjectSymbol {
                            name: name.clone(),
                            kind: ProjectSymbolKind::Subcircuit,
                            detail: format!("{port_count} ports"),
                            definition: location(1),
                        });
                        current_subcircuit = Some(name.clone());
                    }
                }
                crate::state::OutlineEntryKind::Parameter => {
                    for (name, column) in parameter_definitions(source_index.card(entry.line())) {
                        self.symbols.push(ProjectSymbol {
                            name,
                            kind: ProjectSymbolKind::Parameter,
                            detail: ".param".to_owned(),
                            definition: ProjectSourceLocation {
                                root,
                                dependency_identity: dependency_identity.clone(),
                                display_name: display_name.clone(),
                                line: entry.line(),
                                column,
                            },
                        });
                    }
                }
                crate::state::OutlineEntryKind::Function => {
                    if let Some(name) = tokens
                        .get(1)
                        .and_then(|token| token.split(['(', '=']).next())
                        .filter(|name| valid_symbol_name(name))
                    {
                        self.symbols.push(ProjectSymbol {
                            name: name.to_owned(),
                            kind: ProjectSymbolKind::Function,
                            detail: ".func".to_owned(),
                            definition: location(1),
                        });
                    }
                }
                crate::state::OutlineEntryKind::EndSubcircuit => {
                    if let Some(name) = tokens.get(1) {
                        self.references.push(ProjectReference {
                            name: name.clone(),
                            kind: ProjectSymbolKind::Subcircuit,
                            location: location(1),
                        });
                    }
                    current_subcircuit = None;
                }
                crate::state::OutlineEntryKind::Device => {
                    if let Some((name, token_index)) = referenced_subcircuit(&tokens) {
                        let reference_location = location(token_index);
                        self.references.push(ProjectReference {
                            name: name.to_owned(),
                            kind: ProjectSymbolKind::Subcircuit,
                            location: reference_location.clone(),
                        });
                        self.raw_hierarchy.push(RawHierarchyInstance {
                            parent: current_subcircuit.clone(),
                            instance: tokens[0].clone(),
                            target: name.to_owned(),
                            location: reference_location,
                        });
                    } else if let Some(reference) = referenced_definition_candidates(&tokens) {
                        self.pending_model_references.push(PendingModelReference {
                            candidates: reference
                                .token_indices
                                .into_iter()
                                .filter_map(|token_index| {
                                    tokens.get(token_index).map(|name| ProjectReference {
                                        name: name.clone(),
                                        kind: ProjectSymbolKind::Model,
                                        location: location(token_index),
                                    })
                                })
                                .collect(),
                            retain_first_if_unresolved: reference.retain_first_if_unresolved,
                        });
                    }
                }
                _ => {}
            }
        }
        self.pending_expression_references
            .extend(expression_references(
                root,
                dependency_identity,
                display_name,
                source,
            ));
    }

    fn resolve_pending_model_references(&mut self) {
        let model_names = self
            .symbols
            .iter()
            .filter(|symbol| symbol.kind == ProjectSymbolKind::Model)
            .map(|symbol| symbol.name.to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        for pending in self.pending_model_references.drain(..) {
            let resolved = pending
                .candidates
                .iter()
                .find(|candidate| model_names.contains(&candidate.name.to_ascii_lowercase()))
                .cloned()
                .or_else(|| {
                    pending
                        .retain_first_if_unresolved
                        .then(|| pending.candidates.into_iter().next())
                        .flatten()
                });
            if let Some(reference) = resolved {
                self.references.push(reference);
            }
        }
    }

    fn resolve_pending_expression_references(&mut self) {
        let definitions = self
            .symbols
            .iter()
            .map(|symbol| {
                (
                    (symbol.kind, symbol.name.to_ascii_lowercase()),
                    symbol.definition.clone(),
                )
            })
            .collect::<Vec<_>>();
        let mut seen = self
            .references
            .iter()
            .map(reference_identity)
            .collect::<std::collections::HashSet<_>>();
        for reference in self.pending_expression_references.drain(..) {
            let key = (reference.kind, reference.name.to_ascii_lowercase());
            let matching_definitions = definitions
                .iter()
                .filter(|(definition_key, _)| definition_key == &key)
                .map(|(_, location)| location)
                .collect::<Vec<_>>();
            if matching_definitions.is_empty()
                || matching_definitions.contains(&&reference.location)
            {
                continue;
            }
            if seen.insert(reference_identity(&reference)) {
                self.references.push(reference);
            }
        }
    }

    fn expand_hierarchy(&mut self) {
        let mut children = std::collections::HashMap::<String, Vec<RawHierarchyInstance>>::new();
        let mut top = Vec::new();
        for instance in &self.raw_hierarchy {
            if let Some(parent) = instance.parent.as_deref() {
                children
                    .entry(parent.to_ascii_lowercase())
                    .or_default()
                    .push(instance.clone());
            } else {
                top.push(instance.clone());
            }
        }
        let mut pending = top
            .into_iter()
            .rev()
            .map(|instance| {
                let path = instance.instance.clone();
                (instance, path, Vec::<String>::new())
            })
            .collect::<Vec<_>>();
        while let Some((instance, path, mut ancestors)) = pending.pop() {
            if self.hierarchy.len() >= MAX_EXPANDED_HIERARCHY_INSTANCES {
                self.hierarchy_truncated = true;
                break;
            }
            let target_key = instance.target.to_ascii_lowercase();
            let recursive = ancestors.iter().any(|ancestor| ancestor == &target_key);
            self.hierarchy.push(ProjectHierarchyInstance {
                path: path.clone(),
                target: instance.target.clone(),
                location: instance.location.clone(),
                recursive,
            });
            if recursive {
                continue;
            }
            ancestors.push(target_key.clone());
            if let Some(nested) = children.get(&target_key) {
                pending.extend(nested.iter().rev().cloned().map(|child| {
                    let child_path = format!("{path}/{}", child.instance);
                    (child, child_path, ancestors.clone())
                }));
            }
        }
    }

    pub(crate) fn symbols(&self) -> &[ProjectSymbol] {
        &self.symbols
    }

    pub(crate) fn references(&self) -> &[ProjectReference] {
        &self.references
    }

    pub(crate) fn hierarchy(&self) -> &[ProjectHierarchyInstance] {
        &self.hierarchy
    }

    pub(crate) const fn hierarchy_truncated(&self) -> bool {
        self.hierarchy_truncated
    }

    pub(crate) const fn source_count(&self) -> usize {
        self.source_count
    }

    pub(crate) fn definitions_named(&self, name: &str) -> Vec<&ProjectSymbol> {
        self.symbols
            .iter()
            .filter(|symbol| symbol.name.eq_ignore_ascii_case(name))
            .collect()
    }

    pub(crate) fn references_named(&self, name: &str) -> Vec<&ProjectReference> {
        self.references
            .iter()
            .filter(|reference| reference.name.eq_ignore_ascii_case(name))
            .collect()
    }

    pub(crate) fn completion_symbols(&self) -> Vec<super::completion::SymbolEntry> {
        let mut seen = std::collections::HashSet::new();
        self.symbols
            .iter()
            .filter(|symbol| seen.insert((symbol.kind, symbol.name.to_ascii_lowercase())))
            .map(|symbol| super::completion::SymbolEntry {
                name: symbol.name.clone(),
                kind: symbol.kind.label(),
                detail: symbol.detail.clone(),
                doc: format!(
                    "{} defined in {}:{}",
                    symbol.kind.label(),
                    symbol.definition.display_name,
                    symbol.definition.line
                ),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub(super) struct CachedProjectIndex {
    pub(super) revision: crate::product::ObjectRevision,
    pub(super) index: Arc<NetlistProjectIndex>,
}

pub(crate) fn project_index(state: &mut AppState) -> Option<Arc<NetlistProjectIndex>> {
    let root = state
        .ui
        .netlist
        .active_dependency_root
        .unwrap_or(state.ui.netlist.active_document);
    let document = canonical_root_document(state, root)?.clone();
    if let Some(cached) = state.ui.netlist.project_indexes.get(&document.id())
        && cached.revision == document.revision()
    {
        return Some(Arc::clone(&cached.index));
    }
    let index = Arc::new(NetlistProjectIndex::build(root, &document));
    state.ui.netlist.project_indexes.insert(
        document.id(),
        CachedProjectIndex {
            revision: document.revision(),
            index: Arc::clone(&index),
        },
    );
    let generated_id = state
        .ui
        .netlist
        .generated_document
        .as_ref()
        .map(crate::state::NetlistDocument::id);
    let owned_id = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .map(crate::state::NetlistDocument::id);
    state.ui.netlist.project_indexes.retain(|document_id, _| {
        Some(*document_id) == generated_id || Some(*document_id) == owned_id
    });
    Some(index)
}

pub(crate) fn symbol_name_at(source: &str, char_index: usize) -> Option<String> {
    symbol_at_cursor(source, char_index)
}

pub(crate) fn symbol_hover(state: &mut AppState, name: &str) -> Option<ProjectSymbolHover> {
    let index = project_index(state)?;
    let definitions = index.definitions_named(name);
    let references = index.references_named(name);
    match definitions.as_slice() {
        [symbol] => Some(ProjectSymbolHover {
            title: format!("{} {}", symbol.kind.label(), symbol.name),
            detail: format!(
                "{} · {}:{} · {} semantic reference{}",
                symbol.detail,
                symbol.definition.display_name,
                symbol.definition.line,
                references.len(),
                if references.len() == 1 { "" } else { "s" }
            ),
        }),
        [] if !references.is_empty() => Some(ProjectSymbolHover {
            title: format!("unresolved {name}"),
            detail: format!(
                "{} semantic reference{} in the authenticated source closure; no definition is available",
                references.len(),
                if references.len() == 1 { "" } else { "s" }
            ),
        }),
        [] => None,
        _ => Some(ProjectSymbolHover {
            title: format!("ambiguous {name}"),
            detail: format!(
                "{} definitions and {} semantic references in the authenticated source closure",
                definitions.len(),
                references.len()
            ),
        }),
    }
}

pub(crate) fn open_project_location(
    state: &mut AppState,
    location: &ProjectSourceLocation,
) -> Result<(), String> {
    if let Some(identity) = location.dependency_identity.as_deref() {
        super::open_netlist_dependency_from_root(state, location.root, identity)?;
    } else {
        match location.root {
            ActiveNetlistDocument::Generated => {
                if state.ui.netlist.active_document != ActiveNetlistDocument::Generated
                    || state.ui.netlist.active_dependency_identity.is_some()
                {
                    super::open_generated_primary(state);
                }
            }
            ActiveNetlistDocument::OwnedSource => {
                if !super::open_owned_primary(state) {
                    return Err("The project-owned root is no longer available.".to_owned());
                }
            }
            ActiveNetlistDocument::GeneratedDiff => {
                return Err("A comparison has no project symbol definitions.".to_owned());
            }
        }
    }
    state.ui.netlist.cursor_line = location.line.saturating_sub(1);
    state.ui.netlist.requested_line = Some(location.line);
    Ok(())
}

pub(crate) fn go_to_definition_at_cursor(state: &mut AppState) -> Result<String, String> {
    let symbol = symbol_at_cursor(
        &state.simulation.netlist_content,
        state.ui.netlist.cursor_char_index,
    )
    .ok_or_else(|| {
        "Place the caret on a model, subcircuit, parameter, or function name.".to_owned()
    })?;
    let index = project_index(state)
        .ok_or_else(|| "The active source has no authenticated project index.".to_owned())?;
    let definitions = index
        .definitions_named(&symbol)
        .into_iter()
        .map(|definition| definition.definition.clone())
        .collect::<Vec<_>>();
    match definitions.as_slice() {
        [] => Err(format!(
            "No definition for {symbol:?} exists in the authenticated source closure."
        )),
        [definition] => {
            open_project_location(state, definition)?;
            Ok(format!("Opened definition of {symbol}"))
        }
        _ => {
            state.workbench.navigator_visible = true;
            state.workbench.navigator_query = symbol.clone();
            Ok(format!(
                "Found {} definitions of {symbol}; the navigator lists each exact source location.",
                definitions.len()
            ))
        }
    }
}

pub(crate) fn find_references_at_cursor(state: &mut AppState) -> Result<String, String> {
    let symbol = symbol_at_cursor(
        &state.simulation.netlist_content,
        state.ui.netlist.cursor_char_index,
    )
    .ok_or_else(|| {
        "Place the caret on a model, subcircuit, parameter, or function name.".to_owned()
    })?;
    let index = project_index(state)
        .ok_or_else(|| "The active source has no authenticated project index.".to_owned())?;
    let count = index.references_named(&symbol).len();
    state.workbench.navigator_visible = true;
    state.workbench.navigator_query = symbol.clone();
    Ok(if count == 0 {
        format!("No device references to {symbol} were found in the authenticated source closure.")
    } else {
        format!("Found {count} device reference(s) to {symbol}.")
    })
}

pub(crate) fn begin_rename_at_cursor(state: &mut AppState) -> Result<(), String> {
    let symbol = symbol_at_cursor(
        &state.simulation.netlist_content,
        state.ui.netlist.cursor_char_index,
    )
    .ok_or_else(|| "Place the caret on a model or subcircuit name.".to_owned())?;
    let index = project_index(state)
        .ok_or_else(|| "The active source has no authenticated project index.".to_owned())?;
    let definitions = index.definitions_named(&symbol);
    let definition = match definitions.as_slice() {
        [definition] => *definition,
        [] => {
            return Err(format!(
                "No definition for {symbol:?} exists in the authenticated source closure."
            ));
        }
        _ => {
            return Err(format!(
                "Rename is blocked because {symbol:?} has {} definitions.",
                definitions.len()
            ));
        }
    };
    ensure_location_is_project_owned(state, &definition.definition)?;
    for reference in index
        .references_named(&symbol)
        .into_iter()
        .filter(|reference| reference.kind == definition.kind)
    {
        ensure_location_is_project_owned(state, &reference.location)?;
    }
    state.ui.netlist.rename_dialog = NetlistRenameDialogState {
        open: true,
        original: definition.name.clone(),
        replacement: definition.name.clone(),
        error: None,
    };
    Ok(())
}

pub(crate) fn commit_rename(state: &mut AppState) -> Result<usize, String> {
    let original = state.ui.netlist.rename_dialog.original.trim().to_owned();
    let replacement = state.ui.netlist.rename_dialog.replacement.trim().to_owned();
    if !valid_symbol_name(&replacement) {
        return Err(
            "Enter a non-empty SPICE identifier containing only letters, digits, _, or $."
                .to_owned(),
        );
    }
    if original.eq_ignore_ascii_case(&replacement) {
        return Err("Enter a different model or subcircuit name.".to_owned());
    }
    let index = project_index(state)
        .ok_or_else(|| "The active source has no authenticated project index.".to_owned())?;
    if !index.definitions_named(&replacement).is_empty() {
        return Err(format!(
            "A definition named {replacement:?} already exists in the authenticated source closure."
        ));
    }
    let definitions = index.definitions_named(&original);
    let definition = match definitions.as_slice() {
        [definition] => *definition,
        _ => {
            return Err(
                "The selected definition changed while the rename dialog was open.".to_owned(),
            );
        }
    };
    ensure_location_is_project_owned(state, &definition.definition)?;
    let kind = definition.kind;
    let mut locations = vec![definition.definition.clone()];
    for reference in index
        .references_named(&original)
        .into_iter()
        .filter(|reference| reference.kind == kind)
    {
        ensure_location_is_project_owned(state, &reference.location)?;
        locations.push(reference.location.clone());
    }

    let document = state
        .ui
        .netlist
        .owned_document
        .as_ref()
        .or(state.workspace.netlist_document.as_ref())
        .ok_or_else(|| "The project-owned source closure is unavailable.".to_owned())?;
    let mut replacements = Vec::new();
    let root_locations = locations
        .iter()
        .filter(|location| location.dependency_identity.is_none())
        .cloned()
        .collect::<Vec<_>>();
    if !root_locations.is_empty() {
        let renamed =
            rename_at_locations(document.source(), &root_locations, &original, &replacement)?;
        replacements.push(super::OwnedNetlistReplacement::root(
            document.source(),
            renamed,
            root_locations.len(),
        ));
    }
    let dependency_identities = locations
        .iter()
        .filter_map(|location| location.dependency_identity.clone())
        .collect::<std::collections::BTreeSet<_>>();
    for identity in dependency_identities {
        let source = document
            .dependencies()
            .iter()
            .find(|dependency| dependency.locator().logical_identity() == identity)
            .and_then(crate::state::DependencyMetadata::source)
            .ok_or_else(|| format!("Owned include {identity:?} is no longer resolved."))?;
        let include_locations = locations
            .iter()
            .filter(|location| location.dependency_identity.as_deref() == Some(identity.as_str()))
            .cloned()
            .collect::<Vec<_>>();
        let renamed = rename_at_locations(source, &include_locations, &original, &replacement)?;
        replacements.push(super::OwnedNetlistReplacement::dependency(
            identity,
            source,
            renamed,
            include_locations.len(),
        ));
    }
    let count = super::replace_owned_sources_atomically(state, replacements)?;
    state.ui.netlist.rename_dialog = NetlistRenameDialogState::default();
    Ok(count)
}

fn ensure_location_is_project_owned(
    state: &AppState,
    location: &ProjectSourceLocation,
) -> Result<(), String> {
    if location.root != ActiveNetlistDocument::OwnedSource {
        return Err(
            "Rename is read-only because this definition or one of its references belongs to generated source."
                .to_owned(),
        );
    }
    if let Some(identity) = location.dependency_identity.as_deref()
        && state
            .workspace
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.owned_include(identity))
            .is_none()
    {
        return Err(format!(
            "Rename is blocked because reference source {identity:?} is externally owned. Copy it into the project first."
        ));
    }
    Ok(())
}

fn rename_at_locations(
    source: &str,
    locations: &[ProjectSourceLocation],
    original: &str,
    replacement: &str,
) -> Result<String, String> {
    let line_starts = source
        .char_indices()
        .filter_map(|(byte, character)| (character == '\n').then_some(byte + 1))
        .fold(vec![0usize], |mut starts, byte| {
            starts.push(byte);
            starts
        });
    let mut byte_ranges = Vec::with_capacity(locations.len());
    for location in locations {
        let line_start = *line_starts
            .get(location.line.saturating_sub(1))
            .ok_or_else(|| {
                "A rename location is outside the current source revision.".to_owned()
            })?;
        let line_end = source[line_start..]
            .find('\n')
            .map_or(source.len(), |offset| line_start + offset);
        let line = &source[line_start..line_end];
        let token_start_in_line = line
            .char_indices()
            .nth(location.column.saturating_sub(1))
            .map_or(line.len(), |(byte, _)| byte);
        let start = line_start + token_start_in_line;
        let end = start
            + source[start..]
                .char_indices()
                .nth(original.chars().count())
                .map_or_else(|| source[start..].len(), |(byte, _)| byte);
        let found = source
            .get(start..end)
            .ok_or_else(|| "A rename token is outside the current source revision.".to_owned())?;
        if !found.eq_ignore_ascii_case(original) {
            return Err(format!(
                "The source changed at {}:{} while rename results were open.",
                location.display_name, location.line
            ));
        }
        byte_ranges.push(start..end);
    }
    byte_ranges.sort_by(|left, right| right.start.cmp(&left.start));
    if byte_ranges
        .windows(2)
        .any(|pair| pair[0].start < pair[1].end)
    {
        return Err("Rename locations overlap in the current source revision.".to_owned());
    }
    let mut renamed = source.to_owned();
    for range in byte_ranges {
        renamed.replace_range(range, replacement);
    }
    Ok(renamed)
}

fn symbol_at_cursor(source: &str, cursor_char_index: usize) -> Option<String> {
    let characters = source.chars().collect::<Vec<_>>();
    if cursor_char_index > characters.len() {
        return None;
    }
    let symbol_character = |character: char| {
        character.is_ascii_alphanumeric() || matches!(character, '_' | '$' | '!' | '#')
    };
    let mut start = cursor_char_index;
    while start > 0 && symbol_character(characters[start - 1]) {
        start -= 1;
    }
    let mut end = cursor_char_index;
    while end < characters.len() && symbol_character(characters[end]) {
        end += 1;
    }
    (start < end).then(|| characters[start..end].iter().collect())
}

fn valid_symbol_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '$'))
}

fn reference_identity(
    reference: &ProjectReference,
) -> (ProjectSymbolKind, String, u8, Option<String>, usize, usize) {
    (
        reference.kind,
        reference.name.to_ascii_lowercase(),
        match reference.location.root {
            ActiveNetlistDocument::Generated => 0,
            ActiveNetlistDocument::OwnedSource => 1,
            ActiveNetlistDocument::GeneratedDiff => 2,
        },
        reference.location.dependency_identity.clone(),
        reference.location.line,
        reference.location.column,
    )
}

fn parameter_definitions(card: &str) -> Vec<(String, usize)> {
    let characters = card.chars().collect::<Vec<_>>();
    let directive_end = characters
        .iter()
        .position(|character| character.is_whitespace())
        .unwrap_or(characters.len());
    let mut definitions = Vec::new();
    let mut index = directive_end;
    while index < characters.len() {
        while index < characters.len()
            && (characters[index].is_whitespace() || characters[index] == ',')
        {
            index += 1;
        }
        let start = index;
        while index < characters.len()
            && (characters[index].is_ascii_alphanumeric() || matches!(characters[index], '_' | '$'))
        {
            index += 1;
        }
        if start == index {
            index += 1;
            continue;
        }
        let mut equals = index;
        while equals < characters.len() && characters[equals].is_whitespace() {
            equals += 1;
        }
        if characters.get(equals) == Some(&'=') && characters.get(equals + 1) != Some(&'=') {
            definitions.push((characters[start..index].iter().collect(), start + 1));
            index = equals.saturating_add(1);
        }
    }
    definitions
}

fn expression_references(
    root: ActiveNetlistDocument,
    dependency_identity: Option<String>,
    display_name: String,
    source: &str,
) -> Vec<ProjectReference> {
    let mut references = Vec::new();
    for (zero_line, line) in source.lines().enumerate() {
        for (name, column, kind) in expression_identifier_candidates(line) {
            references.push(ProjectReference {
                name,
                kind,
                location: ProjectSourceLocation {
                    root,
                    dependency_identity: dependency_identity.clone(),
                    display_name: display_name.clone(),
                    line: zero_line + 1,
                    column,
                },
            });
        }
    }
    references
}

fn expression_identifier_candidates(line: &str) -> Vec<(String, usize, ProjectSymbolKind)> {
    let trimmed = line.trim_start();
    if trimmed.is_empty()
        || trimmed.starts_with('*')
        || trimmed.starts_with(';')
        || trimmed.starts_with("//")
        || trimmed.starts_with('+')
    {
        return Vec::new();
    }
    let characters = line.chars().collect::<Vec<_>>();
    let mut expression = vec![false; characters.len()];
    let mut quoted = vec![false; characters.len()];
    let mut quote = None;
    let mut cutoff = characters.len();
    for (index, character) in characters.iter().copied().enumerate() {
        if let Some(active_quote) = quote {
            quoted[index] = true;
            if character == active_quote {
                quote = None;
            }
            continue;
        }
        if matches!(character, '\'' | '"') {
            quote = Some(character);
            quoted[index] = true;
        } else if matches!(character, ';' | '$') {
            cutoff = index;
            break;
        }
    }

    let mut brace_depth = 0usize;
    for index in 0..cutoff {
        if quoted[index] {
            continue;
        }
        match characters[index] {
            '{' => brace_depth = brace_depth.saturating_add(1),
            '}' => brace_depth = brace_depth.saturating_sub(1),
            _ if brace_depth > 0 => expression[index] = true,
            _ => {}
        }
    }

    for index in 0..cutoff {
        if characters[index] != '='
            || characters.get(index.wrapping_sub(1)) == Some(&'=')
            || characters.get(index + 1) == Some(&'=')
            || matches!(characters.get(index.wrapping_sub(1)), Some('<' | '>' | '!'))
        {
            continue;
        }
        let mut start = index + 1;
        while start < cutoff && characters[start].is_whitespace() {
            start += 1;
        }
        if let Some(delimiter @ ('\'' | '"')) = characters.get(start).copied() {
            let mut end = start + 1;
            while end < cutoff && characters[end] != delimiter {
                expression[end] = true;
                end += 1;
            }
        } else if characters.get(start) != Some(&'{') {
            let mut end = start;
            while end < cutoff && !characters[end].is_whitespace() && characters[end] != ',' {
                expression[end] = true;
                end += 1;
            }
        }
    }

    let lower = trimmed.to_ascii_lowercase();
    let leading_columns = line
        .chars()
        .take_while(|character| character.is_whitespace())
        .count();
    let full_expression_start = if lower.starts_with(".if ")
        || lower.starts_with(".elseif ")
        || lower.starts_with(".else if ")
    {
        trimmed
            .find(char::is_whitespace)
            .map(|index| leading_columns + trimmed[..index].chars().count() + 1)
    } else if lower.starts_with(".func ")
        || trimmed
            .chars()
            .next()
            .is_some_and(|head| head.eq_ignore_ascii_case(&'b'))
    {
        characters
            .iter()
            .position(|character| *character == '=')
            .map(|index| index + 1)
    } else {
        None
    };
    if let Some(start) = full_expression_start {
        for marked in expression.iter_mut().take(cutoff).skip(start) {
            *marked = true;
        }
    }

    let function_formals = function_formal_names(trimmed);
    let mut candidates = Vec::new();
    let mut index = 0usize;
    while index < cutoff {
        let character = characters[index];
        if !(character.is_ascii_alphabetic() || matches!(character, '_' | '$')) {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < cutoff
            && (characters[index].is_ascii_alphanumeric() || matches!(characters[index], '_' | '$'))
        {
            index += 1;
        }
        let name = characters[start..index].iter().collect::<String>();
        let mut following = index;
        while following < cutoff && characters[following].is_whitespace() {
            following += 1;
        }
        let function_call = characters.get(following) == Some(&'(');
        if function_formals
            .iter()
            .any(|formal| formal.eq_ignore_ascii_case(&name))
            || inside_spice_probe_argument(&characters, start)
        {
            continue;
        }
        if function_call && (!quoted[start] || expression[start]) {
            candidates.push((name, start + 1, ProjectSymbolKind::Function));
        } else if expression[start] {
            candidates.push((name, start + 1, ProjectSymbolKind::Parameter));
        }
    }

    let tokens = crate::state::netlist_document::card_tokens_with_columns(line);
    if tokens
        .first()
        .and_then(|(token, _)| token.as_bytes().first())
        .is_some_and(|head| matches!(head.to_ascii_uppercase(), b'R' | b'C' | b'L'))
        && let Some((name, column)) = tokens.get(3)
        && valid_symbol_name(name)
    {
        candidates.push((name.clone(), *column, ProjectSymbolKind::Parameter));
    }
    candidates
}

fn inside_spice_probe_argument(characters: &[char], index: usize) -> bool {
    let mut nested = 0usize;
    for cursor in (0..index).rev() {
        match characters[cursor] {
            ')' => nested = nested.saturating_add(1),
            '(' if nested > 0 => nested -= 1,
            '(' => {
                let mut name_end = cursor;
                while name_end > 0 && characters[name_end - 1].is_whitespace() {
                    name_end -= 1;
                }
                let mut name_start = name_end;
                while name_start > 0 && characters[name_start - 1].is_ascii_alphabetic() {
                    name_start -= 1;
                }
                let name = characters[name_start..name_end].iter().collect::<String>();
                return name.eq_ignore_ascii_case("v") || name.eq_ignore_ascii_case("i");
            }
            _ => {}
        }
    }
    false
}

fn function_formal_names(trimmed: &str) -> Vec<String> {
    if !trimmed.to_ascii_lowercase().starts_with(".func ") {
        return Vec::new();
    }
    let Some(open) = trimmed.find('(') else {
        return Vec::new();
    };
    let Some(close) = trimmed[open + 1..]
        .find(')')
        .map(|offset| open + 1 + offset)
    else {
        return Vec::new();
    };
    trimmed[open + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|name| valid_symbol_name(name))
        .map(str::to_owned)
        .collect()
}

struct ModelReferenceCandidates {
    token_indices: Vec<usize>,
    retain_first_if_unresolved: bool,
}

fn referenced_subcircuit(tokens: &[String]) -> Option<(&str, usize)> {
    let head = tokens.first()?;
    if !head.as_bytes().first()?.eq_ignore_ascii_case(&b'X') {
        return None;
    }
    let boundary = tokens
        .iter()
        .position(|token| token.eq_ignore_ascii_case("params:") || token.contains('='))
        .unwrap_or(tokens.len());
    let index = boundary.checked_sub(1)?.max(1);
    tokens.get(index).map(|name| (name.as_str(), index))
}

fn referenced_definition_candidates(tokens: &[String]) -> Option<ModelReferenceCandidates> {
    let head = tokens.first()?;
    let device = head.as_bytes().first()?.to_ascii_uppercase();
    let one = |token_index, retain_first_if_unresolved| ModelReferenceCandidates {
        token_indices: vec![token_index],
        retain_first_if_unresolved,
    };
    match device {
        b'D' => tokens.get(3).map(|_| one(3, true)),
        b'J' | b'Z' => tokens.get(4).map(|_| one(4, true)),
        // Ordinary MOS uses D G S B MODEL; compact VDMOS uses D G S MODEL.
        b'M' => {
            let token_indices = [5, 4]
                .into_iter()
                .filter(|index| tokens.get(*index).is_some_and(|token| !token.contains('=')))
                .collect::<Vec<_>>();
            (!token_indices.is_empty()).then_some(ModelReferenceCandidates {
                token_indices,
                retain_first_if_unresolved: true,
            })
        }
        // BJT accepts optional substrate and thermal nodes. The model is the
        // last positional token before assignments/OFF/area; definitions make
        // this exact even when node names are alphabetic.
        b'Q' => {
            let boundary = tokens
                .iter()
                .enumerate()
                .skip(4)
                .find_map(|(index, token)| {
                    (token.contains('=') || token.eq_ignore_ascii_case("off")).then_some(index)
                })
                .unwrap_or(tokens.len());
            let token_indices = (4..boundary).rev().collect::<Vec<_>>();
            (!token_indices.is_empty()).then_some(ModelReferenceCandidates {
                token_indices,
                retain_first_if_unresolved: true,
            })
        }
        // Passive model forms are lexically indistinguishable from a value or
        // parameter expression, so they become references only when the token
        // resolves to a model definition in the authenticated closure.
        b'R' | b'C' | b'L' => tokens.get(3).map(|_| one(3, false)),
        b'S' => tokens.get(5).map(|_| one(5, true)),
        b'W' | b'U' => tokens.get(4).map(|_| one(4, true)),
        b'O' => tokens.get(5).map(|_| one(5, true)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_index_resolves_cross_file_definitions_and_references_case_insensitively() {
        let root = "root\n.include \"cell.inc\"\n.param gain=2\n.func shape(x)={x*gain}\nRgain in 0 gain\nX1 in out cell gain={shape(gain)}\n.end\n";
        let include = ".subckt CELL in out params: gain=1\n.param local=2\nXBUF in out leaf\n.ends CELL\n.subckt LEAF in out\nR1 in out 1k\n.ends LEAF\n";
        let locator = crate::state::SourceLocator::try_new("cell.inc", "cell.inc").unwrap();
        let dependency =
            crate::state::DependencyMetadata::unresolved_direct_to(0, "cell.inc", locator)
                .unwrap()
                .resolve_utf8(include.as_bytes().to_vec())
                .unwrap();
        let provenance = crate::state::GeneratedProvenance::try_new(
            "language-index-test",
            crate::state::GenerationInput::new(
                crate::product::ObjectRevision::INITIAL,
                crate::state::content_digest("language-index-input"),
            ),
        )
        .unwrap();
        let artifact = crate::state::GeneratedArtifact::try_from_utf8(
            provenance,
            root.as_bytes().to_vec(),
            vec![dependency],
            Vec::new(),
        )
        .unwrap();
        let document = crate::state::NetlistDocument::from_generated(
            crate::state::NetlistDocumentId::new(),
            artifact,
        )
        .unwrap();

        let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

        let definitions = index.definitions_named("cell");
        assert_eq!(definitions.len(), 1);
        assert_eq!(
            definitions[0].definition.dependency_identity.as_deref(),
            Some("cell.inc")
        );
        assert_eq!(
            index.references_named("CELL").len(),
            2,
            "the X instance and matching .ends card are both semantic references"
        );
        assert_eq!(index.hierarchy().len(), 2);
        assert_eq!(index.hierarchy()[0].path, "X1");
        assert_eq!(index.hierarchy()[0].target, "cell");
        assert_eq!(index.hierarchy()[1].path, "X1/XBUF");
        assert_eq!(index.hierarchy()[1].target, "leaf");
        assert_eq!(index.source_count(), 2);
    }

    #[test]
    fn project_index_resolves_model_references_across_spice_device_families() {
        let source = "models\n\
.model DMOD D\n\
.model QMOD NPN\n\
.model MMOD NMOS\n\
.model RMOD R\n\
.model SMOD SW\n\
D1 a 0 dmod\n\
Q1 c b e substrate qmod area=2\n\
M1 d g s b mmod l=1u\n\
R1 a b rmod l=1u\n\
R2 b 0 resistance_value\n\
S1 a b c d smod\n\
.end\n";
        let provenance = crate::state::GeneratedProvenance::try_new(
            "language-model-reference-test",
            crate::state::GenerationInput::new(
                crate::product::ObjectRevision::INITIAL,
                crate::state::content_digest("language-model-reference-input"),
            ),
        )
        .unwrap();
        let artifact = crate::state::GeneratedArtifact::try_from_utf8(
            provenance,
            source.as_bytes().to_vec(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let document = crate::state::NetlistDocument::from_generated(
            crate::state::NetlistDocumentId::new(),
            artifact,
        )
        .unwrap();

        let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

        for name in ["dmod", "qmod", "mmod", "rmod", "smod"] {
            assert_eq!(
                index.references_named(name).len(),
                1,
                "expected exactly one semantic reference to {name}"
            );
        }
        assert!(index.references_named("resistance_value").is_empty());
    }

    #[test]
    fn expanded_hierarchy_stops_at_recursive_subcircuit_edges() {
        let location = |line| ProjectSourceLocation {
            root: ActiveNetlistDocument::Generated,
            dependency_identity: None,
            display_name: "recursive.sp".to_owned(),
            line,
            column: 1,
        };
        let mut index = NetlistProjectIndex {
            raw_hierarchy: vec![
                RawHierarchyInstance {
                    parent: None,
                    instance: "XTOP".to_owned(),
                    target: "A".to_owned(),
                    location: location(1),
                },
                RawHierarchyInstance {
                    parent: Some("A".to_owned()),
                    instance: "XB".to_owned(),
                    target: "B".to_owned(),
                    location: location(3),
                },
                RawHierarchyInstance {
                    parent: Some("B".to_owned()),
                    instance: "XA".to_owned(),
                    target: "A".to_owned(),
                    location: location(6),
                },
            ],
            ..NetlistProjectIndex::default()
        };

        index.expand_hierarchy();

        assert_eq!(index.hierarchy().len(), 3);
        assert_eq!(index.hierarchy()[2].path, "XTOP/XB/XA");
        assert!(index.hierarchy()[2].recursive);
        assert!(!index.hierarchy_truncated());
    }

    #[test]
    fn project_index_tracks_parameter_and_function_expression_references() {
        let candidates = expression_identifier_candidates(".param base=1k gain = {base*2}");
        assert!(
            candidates.iter().any(|(name, _, kind)| {
                name.eq_ignore_ascii_case("base") && *kind == ProjectSymbolKind::Parameter
            }),
            "expression candidates were {candidates:?}"
        );
        let source = "expressions\n\
.param base=1k gain = {base*2}\n\
.func shape(x)={x*gain}\n\
R1 out 0 gain\n\
B1 y 0 V={shape(V(out))*gain}\n\
.end\n";
        let provenance = crate::state::GeneratedProvenance::try_new(
            "language-expression-reference-test",
            crate::state::GenerationInput::new(
                crate::product::ObjectRevision::INITIAL,
                crate::state::content_digest("language-expression-reference-input"),
            ),
        )
        .unwrap();
        let artifact = crate::state::GeneratedArtifact::try_from_utf8(
            provenance,
            source.as_bytes().to_vec(),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        let document = crate::state::NetlistDocument::from_generated(
            crate::state::NetlistDocumentId::new(),
            artifact,
        )
        .unwrap();

        let index = NetlistProjectIndex::build(ActiveNetlistDocument::Generated, &document);

        assert_eq!(index.definitions_named("base").len(), 1);
        assert_eq!(index.definitions_named("gain").len(), 1);
        assert_eq!(index.definitions_named("shape").len(), 1);
        assert_eq!(
            index.references_named("base").len(),
            1,
            "indexed references were {:?}",
            index.references()
        );
        assert_eq!(index.references_named("gain").len(), 3);
        assert_eq!(index.references_named("shape").len(), 1);
        assert!(index.references_named("x").is_empty());
        assert!(index.references_named("out").is_empty());
    }

    #[test]
    fn cursor_symbol_is_unicode_indexed_and_accepts_spice_identifier_punctuation() {
        let source = "R1 n1 0 model_name$1\n";
        let cursor = source.find("name").unwrap() + 2;
        assert_eq!(
            symbol_at_cursor(source, cursor),
            Some("model_name$1".to_owned())
        );
    }

    #[test]
    fn semantic_rename_updates_owned_definition_and_cross_file_reference_atomically() {
        let root = "root\n.include \"cell.inc\"\n.param gain=2\n.func shape(x)={x*gain}\nRgain in 0 gain\nX1 in out cell gain={shape(gain)}\n.end\n";
        let include = ".subckt CELL in out params: gain=1\n.ends CELL\n";
        let locator = crate::state::SourceLocator::try_new("cell.inc", "cell.inc").unwrap();
        let dependency =
            crate::state::DependencyMetadata::unresolved_direct_to(0, "cell.inc", locator)
                .unwrap()
                .resolve_utf8(include.as_bytes().to_vec())
                .unwrap();
        let provenance = crate::state::GeneratedProvenance::try_new(
            "language-rename-test",
            crate::state::GenerationInput::new(
                crate::product::ObjectRevision::INITIAL,
                crate::state::content_digest("language-rename-input"),
            ),
        )
        .unwrap();
        let artifact = crate::state::GeneratedArtifact::try_from_utf8(
            provenance,
            root.as_bytes().to_vec(),
            vec![dependency],
            Vec::new(),
        )
        .unwrap();
        let generated = crate::state::NetlistDocument::from_generated(
            crate::state::NetlistDocumentId::new(),
            artifact,
        )
        .unwrap();
        let owned = generated
            .create_editable_copy(
                crate::state::NetlistDocumentId::new(),
                generated.content_digest(),
            )
            .unwrap();
        let owned_include =
            crate::state::OwnedNetlistIncludeDescriptor::try_new(&owned.dependencies()[0]).unwrap();
        let mut state = AppState::default();
        state.workspace.netlist_source = Some(root.to_owned());
        state.workspace.netlist_document = Some(owned.clone());
        state.workspace.netlist_descriptor = Some(crate::state::OwnedNetlistDescriptor {
            artifact_name: "root.sp".to_owned(),
            strategy: crate::state::OwnedNetlistEditStrategy::OwnedSource,
            source_encoding: crate::state::NetlistTextEncoding::Utf8,
            source_line_ending: crate::state::NetlistLineEnding::Lf,
            imported_dialect: Some(crate::state::NetlistSourceDialect::RSpice),
            compatibility_reviewed: true,
            execution_profile: Some(crate::state::NetlistExecutionProfile::RSpiceCanonicalV1),
            external_file_sha256: None,
            save_history: Vec::new(),
            revision_history: Vec::new(),
            owned_includes: vec![owned_include],
        });
        state.ui.netlist.generated_document = Some(generated);
        state.ui.netlist.owned_document = Some(owned);
        state.ui.netlist.active_document = ActiveNetlistDocument::OwnedSource;
        state.simulation.netlist_content = root.to_owned();
        state.ui.netlist.cursor_char_index = root.find("cell gain").unwrap() + 2;

        begin_rename_at_cursor(&mut state).unwrap();
        state.ui.netlist.rename_dialog.replacement = "amplifier".to_owned();
        assert_eq!(commit_rename(&mut state).unwrap(), 3);

        let document = state.workspace.netlist_document.as_ref().unwrap();
        assert!(
            document
                .source()
                .contains("X1 in out amplifier gain={shape(gain)}")
        );
        assert!(
            document.dependencies()[0]
                .source()
                .unwrap()
                .contains(".subckt amplifier in out")
        );
        assert!(
            document.dependencies()[0]
                .source()
                .unwrap()
                .contains(".ends amplifier")
        );
        assert!(super::super::can_undo_netlist_edit(&state));
        super::super::undo_netlist_edit(&mut state).unwrap();
        assert_eq!(state.workspace.netlist_source.as_deref(), Some(root));
        assert_eq!(
            state
                .workspace
                .netlist_document
                .as_ref()
                .unwrap()
                .dependencies()[0]
                .source(),
            Some(include)
        );

        state.ui.netlist.cursor_char_index = root.find(".param gain").unwrap() + 9;
        begin_rename_at_cursor(&mut state).unwrap();
        state.ui.netlist.rename_dialog.replacement = "drive".to_owned();
        assert_eq!(commit_rename(&mut state).unwrap(), 4);
        let renamed = state.workspace.netlist_source.as_deref().unwrap();
        assert!(renamed.contains(".param drive=2"));
        assert!(renamed.contains("{x*drive}"));
        assert!(renamed.contains("Rgain in 0 drive"));
        assert!(renamed.contains("{shape(drive)}"));
        super::super::undo_netlist_edit(&mut state).unwrap();

        state.ui.netlist.cursor_char_index = root.find("shape(x)").unwrap() + 2;
        begin_rename_at_cursor(&mut state).unwrap();
        state.ui.netlist.rename_dialog.replacement = "curve".to_owned();
        assert_eq!(commit_rename(&mut state).unwrap(), 2);
        let renamed = state.workspace.netlist_source.as_deref().unwrap();
        assert!(renamed.contains(".func curve(x)"));
        assert!(renamed.contains("{curve(gain)}"));
    }
}
