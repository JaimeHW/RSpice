//! Project-scoped SPICE symbols and cross-file references.
//!
//! The index is the engine's index of the authenticated root/dependency
//! closure, built away from the editor's thread. It never reads an ambient
//! filesystem path, so native and browser builds answer completion and
//! navigation from the same exact source set.

use std::collections::HashSet;
use std::sync::Arc;

use rspice_core::netlist::{
    DeckIndex, DeckIndexSource, DeckLocation, DeckNameAt, DeckSymbol, DeckSymbolKind,
    NetlistSourceMap, NetlistValueKind, NetlistValueSpan, SourceKind, UnreadSourceReason,
    UnreadSourceRegion,
};

use super::index_worker::BackgroundIndex;
use super::{Model, NetlistDocumentState, NetlistRoot, canonical_root_document_in};
use crate::state::RevisedText;
use crate::workbench::state::WorkspaceDocumentId;
use crate::workbench::{MessageCatalog, MessageId};
use rspice_app_runs::workbench::slots::runs::RunsState;

#[derive(Debug, Clone, Default)]
pub struct NetlistSignatureHelpState {
    pub open: bool,
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
    pub(crate) root: NetlistRoot,
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

/// The names the root and its SPICE includes define and read, as the
/// engine's index of them binds each read under SPICE scoping.
#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistProjectIndex {
    symbols: Vec<ProjectSymbol>,
    references: Vec<ProjectReference>,
    raw_hierarchy: Vec<RawHierarchyInstance>,
    hierarchy: Vec<ProjectHierarchyInstance>,
    hierarchy_truncated: bool,
    source_count: usize,
    deck: DeckIndex,
    /// The files indexed, in the order the index numbers them: the root,
    /// then its SPICE includes.
    files: Vec<IndexedFile>,
}

/// The index of a large deck is millions of small allocations: whoever lets
/// go of it last, it is freed away from that thread.
impl Drop for NetlistProjectIndex {
    fn drop(&mut self) {
        rspice_design::freed_away::free_away(self);
    }
}

/// One file of the index: where it opens, and the text it was indexed from.
#[derive(Debug, Clone)]
struct IndexedFile {
    root: NetlistRoot,
    dependency_identity: Option<String>,
    display_name: String,
    digest: crate::product::ContentDigest,
    /// The revision of the text indexed: a text at it is that text, told
    /// without reading it.
    revision: u64,
}

#[derive(Debug, Clone)]
struct RawHierarchyInstance {
    parent: Option<String>,
    instance: String,
    target: String,
    location: ProjectSourceLocation,
}

const MAX_EXPANDED_HIERARCHY_INSTANCES: usize = 100_000;

impl ProjectSymbolKind {
    /// The project's kind of a name the index records; an instance is none.
    const fn of(kind: DeckSymbolKind) -> Option<Self> {
        match kind {
            DeckSymbolKind::Parameter => Some(Self::Parameter),
            DeckSymbolKind::Function => Some(Self::Function),
            DeckSymbolKind::Subcircuit => Some(Self::Subcircuit),
            DeckSymbolKind::Model => Some(Self::Model),
            DeckSymbolKind::Instance => None,
        }
    }
}

/// One file the project index reads: the root first, then the SPICE files
/// it includes, each with the identity its tab opens by.
#[derive(Debug, Clone)]
struct IndexText {
    dependency_identity: Option<String>,
    display_name: String,
    path: std::path::PathBuf,
    /// The file's text, shared with the editor rather than copied.
    text: crate::state::RevisedText,
}

/// The files the index of `root` reads: the generated netlist and the files
/// it includes, or the open deck and the includes open beside it.
fn index_texts_in(
    model: &Model,
    netlist: &NetlistDocumentState,
    root: NetlistRoot,
) -> Option<Vec<IndexText>> {
    match root {
        NetlistRoot::Generated => Some(document_texts(canonical_root_document_in(
            model, netlist, root,
        )?)),
        NetlistRoot::Deck => {
            let deck = netlist.open_deck.as_ref()?;
            let file = |file: &crate::workbench::documents::deck_files::OpenFile,
                        identity: Option<String>| IndexText {
                dependency_identity: identity,
                display_name: file.name(),
                path: file.path.clone(),
                text: file.text.clone(),
            };
            Some(
                std::iter::once(file(&deck.root, None))
                    .chain(
                        deck.includes
                            .iter()
                            .map(|include| file(include, Some(include.key()))),
                    )
                    .collect(),
            )
        }
        NetlistRoot::AddedLines | NetlistRoot::Comparison | NetlistRoot::RunInput => None,
    }
}

/// The generated netlist `document` and the SPICE files it includes.
fn document_texts(document: &crate::state::NetlistDocument) -> Vec<IndexText> {
    let mut texts = vec![IndexText {
        dependency_identity: None,
        display_name: "generated.sp".to_owned(),
        path: std::path::PathBuf::new(),
        text: document.source().into(),
    }];
    for dependency in document.dependencies() {
        if document.is_veriloga_dependency(dependency) {
            continue;
        }
        let Some(source) = dependency.source() else {
            continue;
        };
        let locator = dependency.locator();
        texts.push(IndexText {
            dependency_identity: Some(locator.logical_identity().to_owned()),
            display_name: locator.display_name().to_owned(),
            path: std::path::PathBuf::from(
                locator
                    .native_origin()
                    .unwrap_or_else(|| locator.logical_identity()),
            ),
            text: source.into(),
        });
    }
    texts
}

impl NetlistProjectIndex {
    /// The index of `texts`, the root `root` first, then the SPICE files it
    /// includes.
    fn build(root: NetlistRoot, texts: &[IndexText]) -> Self {
        let files = texts
            .iter()
            .map(|text| IndexedFile {
                root,
                dependency_identity: text.dependency_identity.clone(),
                display_name: text.display_name.clone(),
                digest: crate::state::content_digest(&text.text),
                revision: text.text.revision(),
            })
            .collect::<Vec<_>>();
        let sources = texts
            .iter()
            .enumerate()
            .map(|(position, text)| DeckIndexSource {
                identity: text.dependency_identity.as_deref().unwrap_or(""),
                text: &text.text,
                kind: SourceKind::of(&text.path, &text.text, position == 0),
            })
            .collect::<Vec<_>>();
        let deck = DeckIndex::build(&sources);

        let symbols = deck
            .symbols()
            .iter()
            .filter_map(|symbol| {
                let kind = ProjectSymbolKind::of(symbol.kind)?;
                let detail = match kind {
                    ProjectSymbolKind::Parameter if symbol.detail.is_empty() => ".param".to_owned(),
                    ProjectSymbolKind::Parameter => format!("= {}", symbol.detail),
                    _ => symbol.detail.clone(),
                };
                Some(ProjectSymbol {
                    name: symbol.name.clone(),
                    kind,
                    detail,
                    definition: location_in(&files, &symbol.location),
                })
            })
            .collect();
        // A name nothing defines is still read where it can only be one
        // thing: the subcircuit an instance is of, the model a device names.
        // Elsewhere it may be a node or a keyword, and is no read.
        let references = deck
            .references()
            .iter()
            .filter_map(|reference| {
                let kind = ProjectSymbolKind::of(reference.kind)?;
                (reference.target.is_some()
                    || matches!(
                        kind,
                        ProjectSymbolKind::Subcircuit | ProjectSymbolKind::Model
                    ))
                .then(|| ProjectReference {
                    name: reference.name.clone(),
                    kind,
                    location: location_in(&files, &reference.location),
                })
            })
            .collect();
        let raw_hierarchy = deck
            .symbols()
            .iter()
            .filter(|symbol| {
                symbol.kind == DeckSymbolKind::Instance
                    && symbol.name.starts_with(['X', 'x'])
                    && !symbol.detail.is_empty()
            })
            .map(|instance| RawHierarchyInstance {
                parent: deck
                    .scopes()
                    .get(instance.scope)
                    .and_then(|scope| scope.name.clone()),
                instance: instance.name.clone(),
                target: instance.detail.clone(),
                location: location_in(&files, &instance.location),
            })
            .collect();

        let mut index = Self::default();
        index.symbols = symbols;
        index.references = references;
        index.raw_hierarchy = raw_hierarchy;
        index.source_count = files.len();
        index.deck = deck;
        index.files = files;
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

    #[cfg(test)]
    pub(crate) fn references_named(&self, name: &str) -> Vec<&ProjectReference> {
        self.references
            .iter()
            .filter(|reference| reference.name.eq_ignore_ascii_case(name))
            .collect()
    }

    /// The engine's index of the deck and its SPICE includes.
    pub(crate) const fn deck(&self) -> &DeckIndex {
        &self.deck
    }

    /// The include file `file` of the index is, by its identity in the deck;
    /// `None` for the deck itself.
    pub(crate) fn file_dependency(&self, file: usize) -> Option<&str> {
        self.files.get(file)?.dependency_identity.as_deref()
    }

    /// The name the reader knows file `file` of the index by.
    pub(crate) fn file_name(&self, file: usize) -> &str {
        self.files
            .get(file)
            .map_or("", |file| file.display_name.as_str())
    }

    /// The root the index was built for.
    pub(crate) fn root(&self) -> Option<NetlistRoot> {
        self.files.first().map(|file| file.root)
    }

    /// Whether `texts`, one to a file of the index in its order, are still
    /// the texts it was built from.
    pub(crate) fn indexed(&self, texts: &[&str]) -> bool {
        texts.len() == self.files.len()
            && self
                .files
                .iter()
                .zip(texts)
                .all(|(file, text)| file.digest == crate::state::content_digest(text))
    }

    /// The file the index numbers the source `dependency_identity` names
    /// (the root for `None`), while `text` is still the text it indexed: at
    /// once when it is at the revision indexed, else by its digest.
    pub(crate) fn file_holding(
        &self,
        dependency_identity: Option<&str>,
        text: &RevisedText,
    ) -> Option<usize> {
        let file = self
            .files
            .iter()
            .position(|file| file.dependency_identity.as_deref() == dependency_identity)?;
        let indexed = &self.files[file];
        (indexed.revision == text.revision()
            || indexed.digest == crate::state::content_digest(text))
        .then_some(file)
    }

    /// What the name at character `char_index` of `text`, the source
    /// `dependency_identity` names, is bound to: `None` when the index has no
    /// name there or was built from other text, `Some(None)` when it has the
    /// name and nothing in scope defines it.
    pub(crate) fn definition_at(
        &self,
        dependency_identity: Option<&str>,
        text: &RevisedText,
        char_index: usize,
    ) -> Option<Option<ProjectSourceLocation>> {
        let file = self.file_holding(dependency_identity, text)?;
        let byte = text.byte_of_char(char_index).unwrap_or(text.len());
        let target = match self.deck.name_at(file, byte)? {
            DeckNameAt::Definition(symbol) => Some(symbol),
            DeckNameAt::Reference(reference) => self.deck.references()[reference].target,
        };
        Some(
            target
                .and_then(|symbol| self.deck.symbols().get(symbol))
                .filter(|symbol| symbol.kind != DeckSymbolKind::Instance)
                .map(|symbol| location_in(&self.files, &symbol.location)),
        )
    }

    /// What a hover on the name at character `char_index` of `text`, the
    /// source `dependency_identity` names, says: the definition the engine
    /// binds the name to, the file and line it is written on and how many
    /// references the index binds to it; or that nothing in scope defines
    /// it. `None` where the
    /// index has no such name, or was built from other text.
    pub(crate) fn hover_at(
        &self,
        dependency_identity: Option<&str>,
        text: &RevisedText,
        char_index: usize,
        messages: MessageCatalog,
    ) -> Option<ProjectSymbolHover> {
        let file = self.file_holding(dependency_identity, text)?;
        let byte = text.byte_of_char(char_index).unwrap_or(text.len());
        let symbol = match self.deck.name_at(file, byte)? {
            DeckNameAt::Definition(symbol) => symbol,
            DeckNameAt::Reference(reference) => {
                let read = &self.deck.references()[reference];
                let Some(symbol) = read.target else {
                    return Some(ProjectSymbolHover {
                        title: read.name.clone(),
                        detail: messages.format(
                            MessageId::NetlistNothingInScopeDefines,
                            &[("name", &read.name)],
                        ),
                    });
                };
                symbol
            }
        };
        let definition = self.deck.symbols().get(symbol)?;
        let (kind, what) = match definition.kind {
            DeckSymbolKind::Instance => return None,
            DeckSymbolKind::Parameter if definition.detail.is_empty() => {
                (MessageId::NetlistSymbolParameter, String::new())
            }
            DeckSymbolKind::Parameter => (
                MessageId::NetlistSymbolParameter,
                format!("= {}", definition.detail),
            ),
            DeckSymbolKind::Subcircuit => (
                MessageId::NetlistSymbolSubcircuit,
                definition
                    .opens
                    .and_then(|scope| self.deck.scopes().get(scope))
                    .map(|scope| {
                        messages.format(
                            MessageId::NetlistSymbolPorts,
                            &[("ports", &scope.ports.join(" "))],
                        )
                    })
                    .unwrap_or_default(),
            ),
            DeckSymbolKind::Model => (MessageId::NetlistSymbolModel, definition.detail.clone()),
            DeckSymbolKind::Function => {
                (MessageId::NetlistSymbolFunction, definition.detail.clone())
            }
        };
        let read = match self.deck.references_to(symbol).count() {
            0 => messages.text(MessageId::NetlistNoReferences),
            reads => messages.counted(
                reads,
                MessageId::CountOneReference,
                MessageId::CountReferences,
            ),
        };
        let place = format!(
            "{} · {read}",
            messages.format(
                MessageId::NetlistSymbolPlace,
                &[
                    ("file", self.file_name(definition.location.file)),
                    ("line", &definition.location.line.to_string()),
                ],
            )
        );
        Some(ProjectSymbolHover {
            title: messages.format(kind, &[("name", &definition.name)]),
            detail: if what.is_empty() {
                place
            } else {
                format!("{what} · {place}")
            },
        })
    }

    /// The signature of the call at character `char_index` of `text`, the
    /// source `dependency_identity` names, as the engine's scoping binds it:
    /// the definition a name there is bound to, or, anywhere else on an
    /// instance's card, the subcircuit or model the instance is of. So a
    /// subcircuit defined in an include, or one a subcircuit's own definition
    /// shadows, shows its own ports and parameters. `None` where the index
    /// has no name or call there, or was built from other text;
    /// `Some(Err(name))` when nothing in scope defines the name.
    pub(crate) fn signature_at(
        &self,
        dependency_identity: Option<&str>,
        text: &RevisedText,
        char_index: usize,
        messages: MessageCatalog,
    ) -> Option<Result<ProjectSymbolHover, String>> {
        let file = self.file_holding(dependency_identity, text)?;
        let byte = text.byte_of_char(char_index).unwrap_or(text.len());
        let bound = match self.deck.name_at(file, byte) {
            Some(DeckNameAt::Definition(symbol))
                if self.deck.symbols()[symbol].kind != DeckSymbolKind::Instance =>
            {
                Ok(symbol)
            }
            Some(DeckNameAt::Reference(reference)) => {
                let read = &self.deck.references()[reference];
                read.target.ok_or_else(|| read.name.clone())
            }
            _ => self.call_on_line(file, text[..byte].matches('\n').count() + 1)?,
        };
        let symbol = match bound {
            Ok(symbol) => self.deck.symbols().get(symbol)?,
            Err(name) => return Some(Err(name)),
        };
        let name = [("name", symbol.name.as_str())];
        let mut parts = Vec::new();
        let title = match symbol.kind {
            DeckSymbolKind::Instance => return None,
            DeckSymbolKind::Parameter => {
                if !symbol.detail.is_empty() {
                    parts.push(format!("= {}", symbol.detail));
                }
                messages.format(MessageId::NetlistSymbolParameter, &name)
            }
            DeckSymbolKind::Subcircuit => {
                let ports = self.ports_of(symbol);
                if !ports.is_empty() {
                    parts.push(messages.format(
                        MessageId::NetlistSymbolPorts,
                        &[("ports", &ports.join(" "))],
                    ));
                }
                // Its formals: the parameters its own card defines.
                let formals = self
                    .deck
                    .symbols()
                    .iter()
                    .filter(|formal| {
                        formal.kind == DeckSymbolKind::Parameter
                            && formal.card == symbol.card
                            && Some(formal.scope) == symbol.opens
                    })
                    .map(|formal| {
                        if formal.detail.is_empty() {
                            formal.name.clone()
                        } else {
                            format!("{}={}", formal.name, formal.detail)
                        }
                    })
                    .collect::<Vec<_>>();
                if !formals.is_empty() {
                    parts.push(messages.format(
                        MessageId::NetlistSignatureParameters,
                        &[("parameters", &formals.join(" "))],
                    ));
                }
                messages.format(MessageId::NetlistSymbolSubcircuit, &name)
            }
            DeckSymbolKind::Model => {
                if !symbol.detail.is_empty() {
                    parts.push(symbol.detail.clone());
                }
                messages.format(MessageId::NetlistSymbolModel, &name)
            }
            DeckSymbolKind::Function => {
                if !symbol.detail.is_empty() {
                    parts.push(symbol.detail.clone());
                }
                messages.format(MessageId::NetlistSymbolFunction, &name)
            }
        };
        parts.push(messages.format(
            MessageId::NetlistSymbolPlace,
            &[
                ("file", self.file_name(symbol.location.file)),
                ("line", &symbol.location.line.to_string()),
            ],
        ));
        Some(Ok(ProjectSymbolHover {
            title,
            detail: parts.join(" · "),
        }))
    }

    /// The subcircuit or model the instance written on one-based `line` of
    /// `file` is of: the definition the engine binds, or `Err` with the name
    /// when nothing in scope defines it. `None` when no instance that names
    /// one is written there.
    fn call_on_line(&self, file: usize, line: usize) -> Option<Result<usize, String>> {
        let card = self.deck.card_on_line(file, line)?;
        if let Some(target) = self
            .deck
            .symbols()
            .iter()
            .find(|symbol| symbol.card == card && symbol.kind == DeckSymbolKind::Instance)
            .and_then(|instance| instance.target)
        {
            return Some(Ok(target));
        }
        let read = self.deck.references().iter().find(|read| {
            read.card == card
                && matches!(
                    read.kind,
                    DeckSymbolKind::Subcircuit | DeckSymbolKind::Model
                )
        })?;
        Some(read.target.ok_or_else(|| read.name.clone()))
    }

    /// The parameters a reader on one-based `line` of the source
    /// `dependency_identity` names sees, each the definition the engine binds
    /// there: a subcircuit's own and its formals inside it, the deck's
    /// outside.
    pub(crate) fn parameters_visible_at(
        &self,
        dependency_identity: Option<&str>,
        line: usize,
    ) -> Vec<&DeckSymbol> {
        let scope = self.scope_at(dependency_identity, line);
        self.deck
            .parameters_visible_in(scope)
            .into_iter()
            .filter_map(|symbol| self.deck.symbols().get(symbol))
            .collect()
    }

    /// The scope a reader on one-based `line` of the source
    /// `dependency_identity` names is in: the deck's top level, or the
    /// subcircuit whose body the line is in.
    pub(crate) fn scope_at(&self, dependency_identity: Option<&str>, line: usize) -> usize {
        self.files
            .iter()
            .position(|file| file.dependency_identity.as_deref() == dependency_identity)
            .map_or(0, |file| self.deck.scope_on_line(file, line))
    }

    /// The definitions of `kinds` a reader in `scope` sees: those written in
    /// it and in each scope enclosing it.
    pub(crate) fn defined_in_reach(
        &self,
        scope: usize,
        kinds: &[DeckSymbolKind],
    ) -> Vec<&DeckSymbol> {
        let reach = self.deck.scope_chain(scope).collect::<Vec<_>>();
        let wanted =
            |symbol: &&DeckSymbol| kinds.contains(&symbol.kind) && reach.contains(&symbol.scope);
        // A deck's devices are read only when they are asked for.
        if kinds.contains(&DeckSymbolKind::Instance) {
            self.deck.symbols().iter().filter(wanted).collect()
        } else {
            self.deck.definitions().filter(wanted).collect()
        }
    }

    /// The node names written in `scope`, and the deck's global ones, each
    /// once as first written.
    pub(crate) fn nodes_in(&self, scope: usize) -> Vec<&str> {
        let mut seen = HashSet::new();
        self.deck
            .nodes()
            .iter()
            .filter(|node| {
                node.scope == scope
                    || self.deck.cards().get(node.card).map(|card| card.kind)
                        == Some(rspice_core::netlist::DeckCardKind::Global)
                    || node.name == "0"
            })
            .filter(|node| seen.insert(node.name.to_ascii_lowercase()))
            .map(|node| node.name.as_str())
            .collect()
    }

    /// The ports of the subcircuit `symbol`, in order.
    pub(crate) fn ports_of(&self, symbol: &DeckSymbol) -> &[String] {
        symbol
            .opens
            .and_then(|scope| self.deck.scopes().get(scope))
            .map_or(&[], |scope| scope.ports.as_slice())
    }

    /// The subcircuits, models and functions a reader in `scope` sees, for
    /// completion, each name once.
    pub(crate) fn completion_symbols_in(
        &self,
        messages: MessageCatalog,
        scope: usize,
    ) -> Vec<super::completion::SymbolEntry> {
        let mut seen = HashSet::new();
        self.defined_in_reach(
            scope,
            &[
                DeckSymbolKind::Subcircuit,
                DeckSymbolKind::Model,
                DeckSymbolKind::Function,
            ],
        )
        .into_iter()
        .filter(|symbol| seen.insert((symbol.kind, symbol.name.to_ascii_lowercase())))
        .filter_map(|symbol| {
            let kind = ProjectSymbolKind::of(symbol.kind)?;
            let definition = location_in(&self.files, &symbol.location);
            Some(super::completion::SymbolEntry {
                name: symbol.name.clone(),
                kind: kind.label(),
                detail: symbol.detail.clone(),
                doc: messages.format(
                    MessageId::NetlistDefinedIn,
                    &[
                        ("kind", kind.label()),
                        ("file", &definition.display_name),
                        ("line", &definition.line.to_string()),
                    ],
                ),
            })
        })
        .collect()
    }
}

/// Where `location` is, as the editor opens it.
fn location_in(files: &[IndexedFile], location: &DeckLocation) -> ProjectSourceLocation {
    let file = files.get(location.file);
    ProjectSourceLocation {
        root: file.map_or(NetlistRoot::Generated, |file| file.root),
        dependency_identity: file.and_then(|file| file.dependency_identity.clone()),
        display_name: file
            .map(|file| file.display_name.clone())
            .unwrap_or_default(),
        line: location.line,
        column: location.column,
    }
}

/// What the project index of a root was built from: the generated
/// document's identity and revision, or the open deck's identity, the
/// editor's revision of its text and how many of its includes are open.
type ProjectIndexKey = (crate::state::NetlistDocumentId, u64, usize);

/// The project index of the root the editor shows, kept in the netlist
/// editor's state against what it was built from and built away from the
/// editor's thread.
pub(crate) type ProjectIndexSlot = BackgroundIndex<ProjectIndexKey, NetlistProjectIndex>;

/// The root the editor's language tools read, and the key of its text.
fn shown_root_in(
    model: &Model,
    netlist: &NetlistDocumentState,
) -> Option<(NetlistRoot, ProjectIndexKey)> {
    let root = netlist.front.root_kind();
    let key = match root {
        NetlistRoot::Deck => {
            let deck = netlist.open_deck.as_ref()?;
            (deck.tab_id(), netlist.revision, deck.includes.len())
        }
        _ => {
            let document = canonical_root_document_in(model, netlist, root)?;
            (document.id(), document.revision().get(), 0)
        }
    };
    Some((root, key))
}

/// The project index of the root the editor shows, for a frame, built away
/// from it. While the writer types, the last index built stands in until
/// the one of the root as it stands is built; a root changed any other way
/// has none until its own is built. It reads the model.
pub(crate) fn project_index_in(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistProjectIndex>> {
    let typed = netlist.is_being_typed();
    if !typed && netlist.waits_for_indexes() {
        return project_index_now(model, netlist);
    }
    let (root, key) = shown_root_in(model, netlist)?;
    if netlist.project_index.wants(|built| *built == key)
        && let Some(texts) = index_texts_in(model, netlist, root)
    {
        let wake = ctx.clone();
        netlist.project_index.start(
            key,
            move || NetlistProjectIndex::build(root, &texts),
            move || wake.request_repaint(),
        );
    }
    let (built, index) = netlist.project_index.built()?;
    (*built == key || typed).then_some(index)
}

/// The project index of the root the editor shows as it stands, waiting for
/// its build to finish: what an action answers from.
pub(crate) fn project_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistProjectIndex>> {
    let (root, key) = shown_root_in(model, netlist)?;
    if let Some(index) = netlist.project_index.wait_for(|built| *built == key) {
        return Some(index);
    }
    let index = NetlistProjectIndex::build(root, &index_texts_in(model, netlist, root)?);
    Some(netlist.project_index.store(key, index))
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
/// that revision arrives with the lines it did not read, away from the frame;
/// a pointer that has not moved reads the retained answer, and a pointer that
/// moved within one scope reuses the retained parameter environment.
#[derive(Debug, Clone)]
pub(super) struct CachedValueIndex {
    revision: u64,
    /// Whether `map` applies what a parse of `revision` left unread.
    read_by_parse: bool,
    map: Arc<ValueMap>,
    /// Parameter environment last built, with the scope it is for.
    context: Option<(Option<String>, rspice_core::netlist::ParamContext)>,
    /// Character index the last hover was resolved at, with its answer.
    hovered: Option<(usize, Option<NetlistValueHover>)>,
}

/// What a value hover reads of one revision of the buffer, built away from
/// the frame: the engine's map of its values, and where each value span
/// starts, so the span under the pointer is found by a search.
#[derive(Debug, Default)]
pub(crate) struct ValueMap {
    map: NetlistSourceMap,
    /// Positions in the map's value spans, by the byte each starts at.
    by_start: Vec<usize>,
    /// The longest value span, in bytes.
    widest: usize,
}

/// What a value map was built for: the buffer's revision, and whether it
/// applies what a parse of it left unread.
type ValueMapKey = (u64, bool);

/// The value maps of the buffer, built away from the editor's thread.
pub(crate) type ValueMapSlot = BackgroundIndex<ValueMapKey, ValueMap>;

impl ValueMap {
    /// The map of `buffer`, read as its parse read it when `unread` carries
    /// what that parse left out.
    fn build(buffer: &str, unread: Option<&[UnreadSourceRegion]>) -> Self {
        let map = match unread {
            Some(unread) => rspice_core::netlist::source_map_for_editor_as_read(buffer, unread),
            None => rspice_core::netlist::source_map_for_editor(buffer),
        };
        let spans = &map.value_spans;
        let mut by_start = (0..spans.len()).collect::<Vec<_>>();
        by_start.sort_by_key(|at| spans[*at].span.start);
        let widest = spans.iter().map(|span| span.span.len()).max().unwrap_or(0);
        Self {
            map,
            by_start,
            widest,
        }
    }
}

impl std::ops::Deref for ValueMap {
    type Target = NetlistSourceMap;

    fn deref(&self) -> &NetlistSourceMap {
        &self.map
    }
}

/// A large deck's map is as many allocations as it writes values: whoever
/// lets go of it last, it is freed away from that thread.
impl Drop for ValueMap {
    fn drop(&mut self) {
        rspice_design::freed_away::free_away(&mut self.map);
    }
}

impl CachedValueIndex {
    /// The index of `map`, built for `revision`, read as its parse read it
    /// when `read_by_parse`.
    const fn of(revision: u64, read_by_parse: bool, map: Arc<ValueMap>) -> Self {
        Self {
            revision,
            read_by_parse,
            map,
            context: None,
            hovered: None,
        }
    }

    /// A blank map in place of the one kept, for a test of what a hover
    /// reads it for.
    #[cfg(test)]
    pub(super) fn blank_map(&mut self) {
        self.map = Arc::new(ValueMap::default());
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
        buffer: &RevisedText,
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
            .byte_of_char(char_index)
            .filter(|byte| *byte < buffer.len())?;
        let map = Arc::clone(&self.map);
        let span = narrowest_value_span(&map, byte)?;
        let token = match span.kind {
            NetlistValueKind::Expression => (span.span.end <= buffer.len())
                .then(|| buffer.slice(span.span.clone()).into_owned())?,
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
/// name inside an expression wins over the expression around it, and of two
/// as tight the first. Only the spans starting at most the longest span's
/// length before it are read.
fn narrowest_value_span(map: &ValueMap, byte: usize) -> Option<&NetlistValueSpan> {
    let spans = &map.value_spans;
    let first = map
        .by_start
        .partition_point(|at| spans[*at].span.start < byte.saturating_sub(map.widest));
    let end = map
        .by_start
        .partition_point(|at| spans[*at].span.start <= byte);
    map.by_start[first..end]
        .iter()
        .copied()
        .filter(|at| spans[*at].span.contains(&byte))
        .min_by_key(|at| (spans[*at].span.len(), *at))
        .map(|at| &spans[at])
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
///
/// The map of the buffer's values is built away from the frame, once per
/// revision: a pointer hovering before it is built answers nothing, and the
/// frame its build wakes answers. `wait` waits for it, as a command does.
pub(crate) fn value_hover_at_in(
    ctx: &egui::Context,
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
    buffer: &RevisedText,
    char_index: usize,
    wait: bool,
) -> Option<NetlistValueHover> {
    if netlist.front.is_comparison() {
        return None;
    }
    let revision = netlist.revision;
    let unread = netlist.unread_source_at(revision);
    let key = (revision, unread.is_some());
    let current = netlist
        .value_index
        .as_ref()
        .is_some_and(|index| (index.revision, index.read_by_parse) == key);
    if !current {
        let wait = wait || netlist.waits_for_indexes();
        let slot = &mut netlist.value_maps;
        let map = if wait {
            slot.wait_for(|built| *built == key)
                .unwrap_or_else(|| slot.store(key, ValueMap::build(buffer, unread.as_deref())))
        } else {
            if slot.wants(|built| *built == key) {
                let (text, wake) = (buffer.clone(), ctx.clone());
                slot.start(
                    key,
                    move || ValueMap::build(&text, unread.as_deref()),
                    move || wake.request_repaint(),
                );
            }
            let (built, map) = slot.built()?;
            if *built != key {
                return None;
            }
            map
        };
        netlist.value_index = Some(CachedValueIndex::of(key.0, key.1, map));
    }
    let index = netlist.value_index.as_mut()?;
    if let Some((resolved_at, hover)) = &index.hovered
        && *resolved_at == char_index
    {
        return hover.clone();
    }
    let hover = index.resolve(buffer, char_index, messages);
    index.hovered = Some((char_index, hover.clone()));
    hover
}

/// What a hover on the name at character `char_index` of the source the
/// editor shows says, as the deck's index binds it. It is read once for each
/// index, revision and position, so a held pointer does not re-read the deck
/// every frame.
pub(crate) fn name_hover(
    ctx: &egui::Context,
    model: &Model,
    netlist: &mut NetlistDocumentState,
    char_index: usize,
    messages: MessageCatalog,
) -> Option<ProjectSymbolHover> {
    type Held = (
        std::sync::Weak<NetlistProjectIndex>,
        u64,
        Option<String>,
        usize,
        Option<ProjectSymbolHover>,
    );
    let index = project_index_in(ctx, model, netlist)?;
    let id = egui::Id::new("netlist_name_hover");
    let held = ctx.data(|data| data.get_temp::<Held>(id));
    if let Some((built, revision, dependency, at, hover)) = held
        && std::sync::Weak::ptr_eq(&built, &Arc::downgrade(&index))
        && revision == netlist.revision
        && dependency.as_deref() == netlist.front.include()
        && at == char_index
    {
        return hover;
    }
    let hover = index.hover_at(
        netlist.front.include(),
        &model.simulation.source.netlist_content,
        char_index,
        messages,
    );
    let held: Held = (
        Arc::downgrade(&index),
        netlist.revision,
        netlist.front.include().map(str::to_owned),
        char_index,
        hover.clone(),
    );
    ctx.data_mut(|data| data.insert_temp(id, held));
    hover
}

/// Open `location` in the editor. It reads the model, the netlist editor's
/// state, the include tabs the reader opened and the runs.
pub(crate) fn open_project_location_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    location: &ProjectSourceLocation,
    messages: MessageCatalog,
) -> Result<(), String> {
    if let Some(identity) = location.dependency_identity.as_deref() {
        super::open_netlist_dependency_from_root_in(
            model,
            netlist,
            open_documents,
            runs,
            location.root,
            identity,
            messages,
        )?;
    } else {
        match location.root {
            NetlistRoot::Generated => {
                if !netlist.front.shows_generated() || netlist.front.include().is_some() {
                    super::open_generated_primary_in(model, netlist);
                }
            }
            NetlistRoot::Deck => {
                if !crate::workbench::documents::deck_files::show_deck(model, netlist) {
                    return Err(messages.text(MessageId::NetlistDeckClosed));
                }
            }
            NetlistRoot::AddedLines => {
                super::show_added_lines(model, netlist);
            }
            NetlistRoot::Comparison => {
                return Err(messages.text(MessageId::NetlistComparisonHasNoDefinitions));
            }
            NetlistRoot::RunInput => {
                return Err(messages.text(MessageId::NetlistRunInputHasNoDefinitions));
            }
        }
    }
    netlist.cursor_line = location.line.saturating_sub(1);
    netlist.requested_line = Some(location.line);
    Ok(())
}

/// Go to the definition of the symbol at the caret, opening the source it
/// is in, over the model, the netlist editor's state, the include tabs the
/// reader opened, the runs and the session.
pub fn go_to_definition_at_cursor_in(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    let messages = workbench.messages();
    let symbol = symbol_at_cursor(
        &model.simulation.source.netlist_content,
        netlist.cursor_char_index,
    )
    .ok_or_else(|| messages.text(MessageId::NetlistPlaceCaretOnName))?;
    let index = project_index_now(model, netlist)
        .ok_or_else(|| messages.text(MessageId::NetlistSignatureNoIndex))?;
    // The read under the caret binds the definition its scope sees: a
    // subcircuit's own parameter inside it, the deck's outside.
    let bound = index.definition_at(
        netlist.front.include(),
        &model.simulation.source.netlist_content,
        netlist.cursor_char_index,
    );
    let definitions = match bound {
        Some(Some(definition)) => vec![definition],
        Some(None) => Vec::new(),
        None => index
            .definitions_named(&symbol)
            .into_iter()
            .map(|definition| definition.definition.clone())
            .collect::<Vec<_>>(),
    };
    match definitions.as_slice() {
        [] => Err(messages.format(MessageId::NetlistNoDefinition, &[("name", &symbol)])),
        [definition] => {
            open_project_location_in(model, netlist, open_documents, runs, definition, messages)?;
            Ok(messages.format(MessageId::NetlistOpenedDefinition, &[("name", &symbol)]))
        }
        _ => {
            workbench.navigator_visible = true;
            workbench.set_navigator_filter(symbol.clone());
            Ok(messages.format(
                MessageId::NetlistFoundDefinitions,
                &[("count", &definitions.len().to_string()), ("name", &symbol)],
            ))
        }
    }
}

pub(crate) fn go_to_declaration_at_cursor(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    open_documents: &mut HashSet<WorkspaceDocumentId>,
    runs: &mut RunsState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    go_to_definition_at_cursor_in(model, netlist, open_documents, runs, workbench)
        .map(|message| message.replace("definition", "declaration"))
}

/// Show the signature of the directive or call at the caret: a directive's
/// own, or the definition the deck's index binds the call to.
pub fn show_signature_help_at_cursor(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
) -> Result<String, String> {
    let source = model.simulation.source.netlist_content.clone();
    let cursor = netlist.cursor_char_index;
    let (title, detail) = match super::completion::directive_signature_at(&source, cursor, messages)
    {
        Some(directive) => directive,
        None => {
            let index = project_index_now(model, netlist)
                .ok_or_else(|| messages.text(MessageId::NetlistSignatureNoIndex))?;
            match index.signature_at(netlist.front.include(), &source, cursor, messages) {
                Some(Ok(signature)) => (signature.title, signature.detail),
                Some(Err(name)) => {
                    return Err(messages
                        .format(MessageId::NetlistNothingInScopeDefines, &[("name", &name)]));
                }
                None => return Err(messages.text(MessageId::NetlistSignaturePlaceCaret)),
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
    Ok(messages.format(MessageId::NetlistSignatureShowing, &[("name", &title)]))
}

pub(crate) fn open_workspace_symbols(
    model: &Model,
    netlist: &mut NetlistDocumentState,
    workbench: &mut rspice_app_workbench::workbench::state::WorkbenchSession,
) -> Result<String, String> {
    let messages = workbench.messages();
    let index = project_index_now(model, netlist)
        .ok_or_else(|| messages.text(MessageId::NetlistSignatureNoIndex))?;
    let count = index.symbols().len();
    workbench.navigator_visible = true;
    workbench.clear_navigator_filter();
    Ok(messages.counted(
        count,
        MessageId::NetlistShowingSymbolSingular,
        MessageId::NetlistShowingSymbols,
    ))
}

pub(crate) fn preferred_quick_fix_available(netlist: &NetlistDocumentState) -> bool {
    preferred_quick_fix(netlist).is_some()
}

pub fn apply_preferred_quick_fix(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    messages: MessageCatalog,
) -> Result<String, String> {
    let fix =
        preferred_quick_fix(netlist).ok_or_else(|| messages.text(MessageId::NetlistNoQuickFix))?;
    apply_quick_fix(model, netlist, &fix, messages)
}

/// Apply the fix one named finding offered.
///
/// The Problems grid holds findings rather than a caret, so it names the
/// finding it is standing on and the editor resolves the fix. Taking the
/// caret's fix instead would apply whatever fix happened to be on the line the
/// netlist editor was last left on, which is not the finding the reader
/// selected.
pub fn apply_quick_fix_for_finding(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    finding: uuid::Uuid,
    messages: MessageCatalog,
) -> Result<String, String> {
    let fix = netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.canonical.diagnostic_id == finding && diagnostic.is_current())
        .and_then(BoundQuickFix::of)
        .ok_or_else(|| messages.text(MessageId::NetlistFindingHasNoFix))?;
    apply_quick_fix(model, netlist, &fix, messages)
}

/// The file the finding `finding` offers to add to its deck, when its fix
/// is Add file…: the fix asks the reader for a file before it edits.
#[must_use]
pub fn add_file_request_for_finding(
    netlist: &NetlistDocumentState,
    finding: uuid::Uuid,
) -> Option<crate::state::netlist_diagnostics::DeckFileRequest> {
    netlist
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.canonical.diagnostic_id == finding && diagnostic.is_current())
        .and_then(|diagnostic| diagnostic.fix.as_ref()?.add_file.clone())
}

/// The file the caret's line offers to add to its deck, when the fix at
/// the caret is Add file….
#[must_use]
pub fn preferred_add_file_request(
    netlist: &NetlistDocumentState,
) -> Option<crate::state::netlist_diagnostics::DeckFileRequest> {
    preferred_quick_fix(netlist).and_then(|bound| bound.fix.add_file)
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

fn apply_quick_fix(
    model: &mut Model,
    netlist: &mut NetlistDocumentState,
    bound: &BoundQuickFix,
    messages: MessageCatalog,
) -> Result<String, String> {
    if !super::active_netlist_source_is_editable_in(model, netlist) {
        return Err(messages.text(MessageId::NetlistQuickFixReadOnly));
    }
    let source = model.simulation.source.netlist_content.clone();
    // The revision is the binding every editor path moves; the digest is what
    // still holds against a writer that replaces the bytes without moving it.
    if netlist.revision != bound.revision
        || crate::state::content_digest(&source).to_string() != *bound.validation_id
    {
        return Err(messages.text(MessageId::NetlistQuickFixTextChanged));
    }
    let fix = &bound.fix;
    if fix.add_file.is_some() {
        return Err(messages.format(
            MessageId::NetlistQuickFixAsksForFile,
            &[("fix", &fix.label)],
        ));
    }
    if fix.span.start > fix.span.end
        || fix.span.end > source.len()
        || !source.is_char_boundary(fix.span.start)
        || !source.is_char_boundary(fix.span.end)
    {
        return Err(messages.text(MessageId::NetlistQuickFixNoLongerFits));
    }
    let mut replacement = source.as_str().to_owned();
    replacement.replace_range(fix.span.clone(), &fix.replacement);
    let edit = if let Some(identity) = netlist.front.include() {
        super::OwnedNetlistReplacement::dependency(identity, &source, replacement, 1)
    } else {
        super::OwnedNetlistReplacement::root(&source, replacement, 1)
    };
    super::replace_deck_files(model, netlist, vec![edit], messages)?;
    Ok(messages.format(MessageId::NetlistQuickFixApplied, &[("fix", &fix.label)]))
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

/// The name the caret at character `cursor_char_index` of `source` stands
/// in or at either end of. A name is written on one line, so only the
/// caret's line is read.
fn symbol_at_cursor(source: &RevisedText, cursor_char_index: usize) -> Option<String> {
    let at = source.byte_of_char(cursor_char_index)?;
    let start = source.line_start_at(at);
    let line = source.slice(start..source.line_end_at(at));
    let characters = line.chars().collect::<Vec<_>>();
    let cursor_char_index = line[..at - start].chars().count();
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

#[cfg(test)]
mod tests;
