//! Project-scoped SPICE symbols and cross-file references.
//!
//! The index is derived only from the authenticated root/dependency closure.
//! It never reads an ambient filesystem path, so native and browser builds
//! answer completion and navigation from the same exact source set.

use std::sync::Arc;

use rspice_core::netlist::{
    NetlistSourceMap, NetlistValueKind, NetlistValueSpan, UnreadSourceReason, UnreadSourceRegion,
};

use super::{
    ActiveNetlistDocument, AppState, Model, NetlistDocumentState, canonical_root_document_in,
};
use crate::workbench::{MessageCatalog, MessageId};
use rspice_app_runs::workbench::slots::runs::RunsState;

#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistRenameDialogState {
    pub(crate) open: bool,
    pub(crate) original: String,
    pub(crate) replacement: String,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistSignatureHelpState {
    pub(crate) open: bool,
    pub(crate) revision: u64,
    pub(crate) cursor_char_index: usize,
    pub(crate) title: String,
    pub(crate) detail: String,
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
            ActiveNetlistDocument::GeneratedDiff | ActiveNetlistDocument::RunSnapshot => {
                return Self::default();
            }
        };
        let mut index = Self::default();
        index.index_source(root, None, root_name, document.source());
        for dependency in document.dependencies() {
            if document.is_veriloga_dependency(dependency) {
                continue;
            }
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
    project_index_in(&state.model, &mut state.code.netlist)
}

/// [`project_index`], over the model and the netlist editor's state.
pub(crate) fn project_index_in(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistProjectIndex>> {
    let root = netlist
        .active_dependency_root
        .unwrap_or(netlist.active_document);
    let document = canonical_root_document_in(model, netlist, root)?.clone();
    if let Some(cached) = netlist.project_indexes.get(&document.id())
        && cached.revision == document.revision()
    {
        return Some(Arc::clone(&cached.index));
    }
    let index = Arc::new(NetlistProjectIndex::build(root, &document));
    netlist.project_indexes.insert(
        document.id(),
        CachedProjectIndex {
            revision: document.revision(),
            index: Arc::clone(&index),
        },
    );
    let generated_id = model
        .netlist_artifacts
        .generated_document
        .as_ref()
        .map(crate::state::NetlistDocument::id);
    let owned_id = netlist
        .owned_document
        .as_ref()
        .map(crate::state::NetlistDocument::id);
    netlist.project_indexes.retain(|document_id, _| {
        Some(*document_id) == generated_id || Some(*document_id) == owned_id
    });
    Some(index)
}

pub(crate) fn symbol_name_at(source: &str, char_index: usize) -> Option<String> {
    symbol_at_cursor(source, char_index)
}

/// How far a re-parse of the engineering rendering may sit from the evaluated
/// value before the raw number is shown instead.
///
/// Engineering notation carries three decimals, so a value it has to round —
/// `1/3` becoming `333.333m` — misses by about 1e-6 relative and is rejected.
/// A value it renders exactly misses only by the last bit of the scaling
/// multiply, which is what this tolerance admits.
const ENGINEERING_ROUND_TRIP_TOLERANCE: f64 = 1e-12;

/// One editor hover over a value: the token as written, what it evaluates to
/// at that point in the deck, and where that came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NetlistValueHover {
    /// The token as the reader sees it in the buffer.
    pub(crate) token: String,
    /// The evaluated value, `None` when the deck does not answer here.
    pub(crate) value: Option<String>,
    /// Where the value came from, or why there is none.
    pub(crate) detail: Option<String>,
    /// Whether the answer, with no value, is the parser's: the token is on a
    /// line it does not read, or names a parameter only such lines define.
    pub(crate) not_read: bool,
}

/// The editor's value index for one buffer revision, plus the last answer it
/// gave.
///
/// A hover is a paint-rate query and must never cost a parse. The span index
/// is derived once per buffer revision, and again when the debounced parse of
/// that revision arrives with the lines it did not read; a pointer that has
/// not moved reads the retained answer, and a pointer that moved within one
/// scope reuses the retained parameter environment.
#[derive(Debug, Clone)]
pub(super) struct CachedValueIndex {
    revision: u64,
    /// Whether `map` applies what a parse of `revision` left unread.
    read_by_parse: bool,
    map: Arc<NetlistSourceMap>,
    /// Parameter environment last built, with the scope it is for.
    context: Option<(Option<String>, rspice_core::netlist::ParamContext)>,
    /// Character index the last hover was resolved at, with its answer.
    hovered: Option<(usize, Option<NetlistValueHover>)>,
}

impl CachedValueIndex {
    /// The index of `buffer` at `revision`, read as its parse read it when
    /// `unread` carries what that parse left out.
    fn build(revision: u64, buffer: &str, unread: Option<&[UnreadSourceRegion]>) -> Self {
        let map = match unread {
            Some(unread) => rspice_core::netlist::source_map_for_editor_as_read(buffer, unread),
            None => rspice_core::netlist::source_map_for_editor(buffer),
        };
        Self {
            revision,
            read_by_parse: unread.is_some(),
            map: Arc::new(map),
            context: None,
            hovered: None,
        }
    }

    /// The answer the last hover query got, so a surface test can state which
    /// hover a render painted rather than reading it back off pixels.
    #[cfg(test)]
    pub(super) fn last_hover(&self) -> Option<&NetlistValueHover> {
        self.hovered.as_ref().and_then(|(_, hover)| hover.as_ref())
    }

    fn context_for(&mut self, scope: Option<&str>) -> &rspice_core::netlist::ParamContext {
        let retained = self
            .context
            .as_ref()
            .is_some_and(|(retained_scope, _)| retained_scope.as_deref() == scope);
        if !retained {
            self.context = None;
        }
        let map = Arc::clone(&self.map);
        &self
            .context
            .get_or_insert_with(|| (scope.map(str::to_owned), map.param_context_in(scope)))
            .1
    }

    fn resolve(
        &mut self,
        buffer: &str,
        char_index: usize,
        messages: MessageCatalog,
    ) -> Option<NetlistValueHover> {
        // Which lines of a `.if` block or an inline `.lib` section are read is
        // the parser's decision. Until a parse of this revision has made it,
        // no value in the deck can be vouched for.
        if !self.read_by_parse && self.map.decided_by_parse {
            return None;
        }
        let byte = buffer
            .char_indices()
            .nth(char_index)
            .map(|(byte, _)| byte)?;
        let map = Arc::clone(&self.map);
        let span = narrowest_value_span(&map, byte)?;
        let token = match span.kind {
            NetlistValueKind::Expression => buffer.get(span.span.clone())?.to_owned(),
            NetlistValueKind::ParamDefinition | NetlistValueKind::ParamReference => {
                span.name.clone()
            }
        };
        if let Some(reason) = &span.unread {
            return Some(NetlistValueHover {
                token,
                value: None,
                detail: Some(messages.format(
                    MessageId::NetlistHoverNotRead,
                    &[("reason", &unread_reason(reason, messages))],
                )),
                not_read: true,
            });
        }
        let binding = match span.kind {
            NetlistValueKind::Expression => None,
            NetlistValueKind::ParamDefinition | NetlistValueKind::ParamReference => {
                map.param_definition_for(&span.name, span.scope.as_deref())
            }
        };
        // A definition a later one in its scope supersedes binds nothing: the
        // name reads as the value the binding definition gives every reader.
        let superseded = span.kind == NetlistValueKind::ParamDefinition
            && binding.is_some_and(|definition| definition.span != span.span);
        let expression = if superseded {
            &span.name
        } else {
            &span.expression
        };
        let context = self.context_for(span.scope.as_deref());
        let evaluated = rspice_core::netlist::expr::eval_expression(expression, context);
        let mut not_read = false;
        let (value, detail) = match evaluated {
            Ok(value) => (
                Some(format_hover_value(value)),
                (span.kind == NetlistValueKind::ParamReference || superseded)
                    .then_some(binding)
                    .flatten()
                    .map(|definition| {
                        messages.format(
                            MessageId::NetlistHoverDefinedLine,
                            &[("line", &definition.line.to_string())],
                        )
                    }),
            ),
            Err(rspice_core::netlist::expr::ExprError::UndefinedParam(name)) => {
                let unread_definition = map
                    .unread_definition_for(&name, span.scope.as_deref())
                    .and_then(|definition| Some((definition, definition.unread.as_ref()?)));
                not_read = unread_definition.is_some();
                let detail = match unread_definition {
                    Some((definition, reason)) => messages.format(
                        MessageId::NetlistHoverDefinedOnlyUnread,
                        &[
                            ("name", &definition.name),
                            ("line", &definition.line.to_string()),
                            ("reason", &unread_reason(reason, messages)),
                        ],
                    ),
                    None => messages.format(
                        MessageId::NetlistHoverUndefinedParameter,
                        &[("name", &name)],
                    ),
                };
                (None, Some(detail))
            }
            Err(error) => (
                None,
                Some(messages.format(
                    MessageId::NetlistHoverNotEvaluable,
                    &[("reason", &error.to_string())],
                )),
            ),
        };
        Some(NetlistValueHover {
            token,
            value,
            detail,
            not_read,
        })
    }
}

/// Why the parser did not read a line, as the hover says it.
fn unread_reason(reason: &UnreadSourceReason, messages: MessageCatalog) -> String {
    match reason {
        UnreadSourceReason::ConditionalBranch { condition, holds } => messages.format(
            MessageId::NetlistHoverUntakenBranch,
            &[
                ("condition", condition),
                ("holds", if *holds { "true" } else { "false" }),
            ],
        ),
        UnreadSourceReason::LibrarySection { name } => {
            messages.format(MessageId::NetlistHoverUnselectedSection, &[("name", name)])
        }
    }
}

/// The value a reader is pointing at: the tightest span covering `byte`, so a
/// name inside an expression wins over the expression around it.
fn narrowest_value_span(map: &NetlistSourceMap, byte: usize) -> Option<&NetlistValueSpan> {
    map.value_spans
        .iter()
        .filter(|span| span.span.contains(&byte))
        .min_by_key(|span| span.span.len())
}

/// Engineering notation only when re-parsing it lands back on the evaluated
/// value; a value the notation would round renders raw.
fn format_hover_value(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let engineering = crate::quantity::format_engineering_value(value);
    match crate::quantity::parse_engineering_value(&engineering) {
        Ok(parsed) if (parsed - value).abs() <= value.abs() * ENGINEERING_ROUND_TRIP_TOLERANCE => {
            engineering
        }
        _ => value.to_string(),
    }
}

/// What the parameter or expression under the pointer evaluates to where it is
/// written, with the bindings the engine gives its scope, without running
/// anything.
///
/// `buffer` is the visible source, which is also what a generated document or
/// a read-only run snapshot projects; a unified comparison is not a deck and
/// answers nothing.
pub(crate) fn value_hover_at_in(
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
    buffer: &str,
    char_index: usize,
) -> Option<NetlistValueHover> {
    if netlist.active_document == ActiveNetlistDocument::GeneratedDiff {
        return None;
    }
    let revision = netlist.revision;
    let unread = netlist.unread_source_at(revision);
    let index = match &mut netlist.value_index {
        Some(index) if index.revision == revision && index.read_by_parse == unread.is_some() => {
            index
        }
        slot => slot.insert(CachedValueIndex::build(revision, buffer, unread.as_deref())),
    };
    if let Some((resolved_at, hover)) = &index.hovered
        && *resolved_at == char_index
    {
        return hover.clone();
    }
    let hover = index.resolve(buffer, char_index, messages);
    index.hovered = Some((char_index, hover.clone()));
    hover
}

pub(crate) fn symbol_hover(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    name: &str,
) -> Option<ProjectSymbolHover> {
    let index = project_index_in(model, netlist)?;
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
                "{} semantic reference{} in the project's sources; no definition is available",
                references.len(),
                if references.len() == 1 { "" } else { "s" }
            ),
        }),
        [] => None,
        _ => Some(ProjectSymbolHover {
            title: format!("ambiguous {name}"),
            detail: format!(
                "{} definitions and {} semantic references in the project's sources",
                definitions.len(),
                references.len()
            ),
        }),
    }
}

/// Open `location` in the editor. It reads the model, the netlist editor's
/// state, the include tabs the reader opened and the runs.
pub(crate) fn open_project_location_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    location: &ProjectSourceLocation,
) -> Result<(), String> {
    if let Some(identity) = location.dependency_identity.as_deref() {
        super::open_netlist_dependency_from_root_in(
            model,
            netlist,
            open_documents,
            runs,
            location.root,
            identity,
        )?;
    } else {
        match location.root {
            ActiveNetlistDocument::Generated => {
                if netlist.active_document != ActiveNetlistDocument::Generated
                    || netlist.active_dependency_identity.is_some()
                {
                    super::open_generated_primary_in(model, netlist);
                }
            }
            ActiveNetlistDocument::OwnedSource => {
                if !super::open_owned_primary_in(model, netlist) {
                    return Err("The project-owned root is no longer available.".to_owned());
                }
            }
            ActiveNetlistDocument::GeneratedDiff => {
                return Err("A comparison has no project symbol definitions.".to_owned());
            }
            ActiveNetlistDocument::RunSnapshot => {
                return Err(
                    "The deck a completed run used has no project symbol definitions.".to_owned(),
                );
            }
        }
    }
    netlist.cursor_line = location.line.saturating_sub(1);
    netlist.requested_line = Some(location.line);
    Ok(())
}

pub(crate) fn go_to_definition_at_cursor(state: &mut AppState) -> Result<String, String> {
    go_to_definition_at_cursor_in(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        &mut state.runs,
        &mut state.workbench,
    )
}

/// [`go_to_definition_at_cursor`], over the model, the netlist editor's
/// state, the include tabs the reader opened, the runs and the session.
pub(crate) fn go_to_definition_at_cursor_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    let symbol = symbol_at_cursor(
        &model.simulation.source.netlist_content,
        netlist.cursor_char_index,
    )
    .ok_or_else(|| {
        "Place the caret on a model, subcircuit, parameter, or function name.".to_owned()
    })?;
    let index = project_index_in(model, netlist)
        .ok_or_else(|| "The active source has no project index.".to_owned())?;
    let definitions = index
        .definitions_named(&symbol)
        .into_iter()
        .map(|definition| definition.definition.clone())
        .collect::<Vec<_>>();
    match definitions.as_slice() {
        [] => Err(format!(
            "No definition for '{symbol}' exists in the project's sources."
        )),
        [definition] => {
            open_project_location_in(model, netlist, open_documents, runs, definition)?;
            Ok(format!("Opened definition of {symbol}"))
        }
        _ => {
            workbench.navigator_visible = true;
            workbench.set_navigator_filter(symbol.clone());
            Ok(format!(
                "Found {} definitions of {symbol}; the navigator lists where each one is.",
                definitions.len()
            ))
        }
    }
}

pub(crate) fn go_to_declaration_at_cursor(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut std::collections::HashSet<crate::workbench::state::WorkspaceDocumentId>,
    runs: &mut RunsState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    go_to_definition_at_cursor_in(model, netlist, open_documents, runs, workbench)
        .map(|message| message.replace("definition", "declaration"))
}

pub(crate) fn show_signature_help_at_cursor(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Result<String, String> {
    let source = model.simulation.source.netlist_content.clone();
    let cursor = netlist.cursor_char_index;
    let (title, detail) =
        if let Some((title, detail)) = super::completion::directive_signature_at(&source, cursor) {
            (title, detail)
        } else {
            let symbol = symbol_at_cursor(&source, cursor).ok_or_else(|| {
                "Place the caret on an analysis, subcircuit, model, parameter, or function name."
                    .to_owned()
            })?;
            let index = project_index_in(model, netlist)
                .ok_or_else(|| "The active source has no project index.".to_owned())?;
            let definitions = index.definitions_named(&symbol);
            match definitions.as_slice() {
                [definition] => (
                    format!("{} {}", definition.kind.label(), definition.name),
                    format!(
                        "{} · declared at {}:{}",
                        definition.detail,
                        definition.definition.display_name,
                        definition.definition.line
                    ),
                ),
                [] => {
                    return Err(format!(
                        "No signature for '{symbol}' exists in the project's sources."
                    ));
                }
                _ => {
                    return Err(format!(
                        "Signature help is ambiguous because '{symbol}' has {} declarations.",
                        definitions.len()
                    ));
                }
            }
        };
    netlist.signature_help = NetlistSignatureHelpState {
        open: true,
        revision: netlist.revision,
        cursor_char_index: cursor,
        title: title.clone(),
        detail,
    };
    Ok(format!("Showing signature help for {title}"))
}

pub(crate) fn open_workspace_symbols(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    let index = project_index_in(model, netlist)
        .ok_or_else(|| "The active source has no project index.".to_owned())?;
    let count = index.symbols().len();
    workbench.navigator_visible = true;
    workbench.clear_navigator_filter();
    Ok(format!("Showing {count} project symbol(s)."))
}

pub(crate) fn preferred_quick_fix_available(netlist: &NetlistDocumentState) -> bool {
    preferred_quick_fix(netlist).is_some()
}

pub(crate) fn apply_preferred_quick_fix(state: &mut AppState) -> Result<String, String> {
    let fix = preferred_quick_fix(&state.code.netlist)
        .ok_or_else(|| "No preferred quick fix is available at the caret.".to_owned())?;
    apply_quick_fix(state, &fix)
}

/// Apply the fix one named finding offered.
///
/// The Problems grid holds findings rather than a caret, so it names the
/// finding it is standing on and the editor resolves the fix. Taking the
/// caret's fix instead would apply whatever fix happened to be on the line the
/// netlist editor was last left on, which is not the finding the reader
/// selected.
pub(crate) fn apply_quick_fix_for_finding(
    state: &mut AppState,
    finding: uuid::Uuid,
) -> Result<String, String> {
    let fix = state
        .code
        .netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.canonical.diagnostic_id == finding && diagnostic.is_current())
        .and_then(BoundQuickFix::of)
        .ok_or_else(|| {
            "That finding no longer offers a fix; validate the document again.".to_owned()
        })?;
    apply_quick_fix(state, &fix)
}

/// A fix, and the exact source it was computed against.
///
/// A fix is a byte range and the text that replaces it, so it means something
/// only for the bytes it was read off. A finding's own currency cannot vouch
/// for that: the editor marks its findings stale only while it is drawn, so
/// a fix pressed from the Problems page with the editor off screen — twice, or
/// after any other edit — would rewrite whatever bytes had moved into its
/// range since.
struct BoundQuickFix {
    fix: crate::state::netlist_diagnostics::DiagnosticFix,
    /// The editor revision the finding was validated at.
    revision: u64,
    /// The digest of the exact buffer it was validated against.
    validation_id: Arc<str>,
}

impl BoundQuickFix {
    fn of(diagnostic: &crate::state::netlist_diagnostics::Diagnostic) -> Option<Self> {
        diagnostic.fix.clone().map(|fix| Self {
            fix,
            revision: diagnostic.canonical.revision,
            validation_id: Arc::clone(&diagnostic.canonical.validation_id),
        })
    }
}

fn apply_quick_fix(state: &mut AppState, bound: &BoundQuickFix) -> Result<String, String> {
    if !super::active_netlist_source_is_editable_in(&state.model, &state.code.netlist) {
        return Err("Quick fixes cannot edit this read-only source.".to_owned());
    }
    let source = state.model.simulation.source.netlist_content.clone();
    // The revision is the binding every editor path moves; the digest is what
    // still holds against a writer that replaces the bytes without moving it.
    if state.code.netlist.revision != bound.revision
        || crate::state::content_digest(&source).to_string() != *bound.validation_id
    {
        return Err(
            "The source has changed since this fix was offered; validate the document again."
                .to_owned(),
        );
    }
    let fix = &bound.fix;
    if fix.span.start > fix.span.end
        || fix.span.end > source.len()
        || !source.is_char_boundary(fix.span.start)
        || !source.is_char_boundary(fix.span.end)
    {
        return Err("The quick fix became stale; validate the document again.".to_owned());
    }
    let mut replacement = source.clone();
    replacement.replace_range(fix.span.clone(), &fix.replacement);
    let edit = if let Some(identity) = state.code.netlist.active_dependency_identity.as_deref() {
        super::OwnedNetlistReplacement::dependency(identity, &source, replacement, 1)
    } else {
        super::OwnedNetlistReplacement::root(&source, replacement, 1)
    };
    super::replace_owned_sources_atomically(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        vec![edit],
    )?;
    Ok(format!("Applied quick fix: {}", fix.label))
}

fn preferred_quick_fix(netlist: &NetlistDocumentState) -> Option<BoundQuickFix> {
    netlist
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.is_current() && diagnostic.canonical.revision == netlist.revision
        })
        .filter(|diagnostic| {
            diagnostic.line.or(diagnostic.source_line) == Some(netlist.cursor_line)
        })
        .find_map(BoundQuickFix::of)
}

pub(crate) fn find_references_at_cursor(state: &mut AppState) -> Result<String, String> {
    find_references_at_cursor_in(&state.model, &mut state.code.netlist, &mut state.workbench)
}

/// [`find_references_at_cursor`], over the model, the netlist editor's state
/// and the session.
pub(crate) fn find_references_at_cursor_in(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    let symbol = symbol_at_cursor(
        &model.simulation.source.netlist_content,
        netlist.cursor_char_index,
    )
    .ok_or_else(|| {
        "Place the caret on a model, subcircuit, parameter, or function name.".to_owned()
    })?;
    let index = project_index_in(model, netlist)
        .ok_or_else(|| "The active source has no project index.".to_owned())?;
    let count = index.references_named(&symbol).len();
    workbench.navigator_visible = true;
    workbench.set_navigator_filter(symbol.clone());
    Ok(if count == 0 {
        format!("No device references to {symbol} were found in the project's sources.")
    } else {
        format!("Found {count} device reference(s) to {symbol}.")
    })
}

pub(crate) fn begin_rename_at_cursor(state: &mut AppState) -> Result<(), String> {
    begin_rename_at_cursor_in(&state.model, &mut state.code.netlist)
}

/// [`begin_rename_at_cursor`], over the model and the netlist editor's state.
pub(crate) fn begin_rename_at_cursor_in(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Result<(), String> {
    let symbol = symbol_at_cursor(
        &model.simulation.source.netlist_content,
        netlist.cursor_char_index,
    )
    .ok_or_else(|| "Place the caret on a model or subcircuit name.".to_owned())?;
    let index = project_index_in(model, netlist)
        .ok_or_else(|| "The active source has no project index.".to_owned())?;
    let definitions = index.definitions_named(&symbol);
    let definition = match definitions.as_slice() {
        [definition] => *definition,
        [] => {
            return Err(format!(
                "No definition for '{symbol}' exists in the project's sources."
            ));
        }
        _ => {
            return Err(format!(
                "Rename is blocked because '{symbol}' has {} definitions.",
                definitions.len()
            ));
        }
    };
    ensure_location_is_project_owned(model, &definition.definition)?;
    for reference in index
        .references_named(&symbol)
        .into_iter()
        .filter(|reference| reference.kind == definition.kind)
    {
        ensure_location_is_project_owned(model, &reference.location)?;
    }
    netlist.rename_dialog = NetlistRenameDialogState {
        open: true,
        original: definition.name.clone(),
        replacement: definition.name.clone(),
        error: None,
    };
    Ok(())
}

pub(crate) fn commit_rename(state: &mut AppState) -> Result<usize, String> {
    let original = state.code.netlist.rename_dialog.original.trim().to_owned();
    let replacement = state
        .code
        .netlist
        .rename_dialog
        .replacement
        .trim()
        .to_owned();
    if !valid_symbol_name(&replacement) {
        return Err(
            "Enter a non-empty SPICE identifier containing only letters, digits, _, or $."
                .to_owned(),
        );
    }
    if original.eq_ignore_ascii_case(&replacement) {
        return Err("Enter a different model or subcircuit name.".to_owned());
    }
    let index =
        project_index(state).ok_or_else(|| "The active source has no project index.".to_owned())?;
    if !index.definitions_named(&replacement).is_empty() {
        return Err(format!(
            "A definition named '{replacement}' already exists in the project's sources."
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
    ensure_location_is_project_owned(&state.model, &definition.definition)?;
    let kind = definition.kind;
    let mut locations = vec![definition.definition.clone()];
    for reference in index
        .references_named(&original)
        .into_iter()
        .filter(|reference| reference.kind == kind)
    {
        ensure_location_is_project_owned(&state.model, &reference.location)?;
        locations.push(reference.location.clone());
    }

    let document = state
        .code
        .netlist
        .owned_document
        .as_ref()
        .or(state.model.workspace.content.netlist_document.as_ref())
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
            .ok_or_else(|| format!("Owned include '{identity}' is no longer resolved."))?;
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
    let count = super::replace_owned_sources_atomically(
        &mut state.model,
        &mut state.code.netlist,
        &mut state.code.netlist_open_documents,
        replacements,
    )?;
    state.code.netlist.rename_dialog = NetlistRenameDialogState::default();
    Ok(count)
}

fn ensure_location_is_project_owned(
    model: &Model,
    location: &ProjectSourceLocation,
) -> Result<(), String> {
    if location.root != ActiveNetlistDocument::OwnedSource {
        return Err(
            "Rename is read-only because this definition or one of its references belongs to generated source."
                .to_owned(),
        );
    }
    if let Some(identity) = location.dependency_identity.as_deref()
        && model
            .workspace
            .content
            .netlist_descriptor
            .as_ref()
            .and_then(|descriptor| descriptor.owned_include(identity))
            .is_none()
    {
        return Err(format!(
            "Rename is blocked because reference source '{identity}' is externally owned. Copy it into the project first."
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
            ActiveNetlistDocument::RunSnapshot => 3,
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
mod tests;
