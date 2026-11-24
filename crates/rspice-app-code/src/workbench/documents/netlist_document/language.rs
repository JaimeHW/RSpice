//! Project-scoped SPICE symbols and cross-file references.
//!
//! The index is the engine's index of the authenticated root/dependency
//! closure, built away from the editor's thread. It never reads an ambient
//! filesystem path, so native and browser builds answer completion and
//! navigation from the same exact source set.

use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use rspice_core::netlist::{
    DeckIndex, DeckIndexSource, DeckLocation, DeckNameAt, DeckSymbol, DeckSymbolKind,
    DeckTextChange, NetlistSourceMap, NetlistValueKind, NetlistValueSpan, SharedDeckText,
    SourceKind, UnreadSourceReason, UnreadSourceRegion,
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

/// A definition as the navigator lists it, read from the index when it is
/// asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectSymbol {
    pub(crate) name: String,
    pub(crate) kind: ProjectSymbolKind,
    pub(crate) detail: String,
    pub(crate) definition: ProjectSourceLocation,
}

/// A read as the navigator lists it, read from the index when it is asked
/// for.
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
///
/// It holds the engine's index, which shares the texts, and tables of
/// positions into it; a name, a detail or a place is read from the index
/// when it is asked for. What it lists of the whole deck, the instance
/// hierarchy and the node names each scope sees, is its [`ProjectTables`],
/// worked out apart and kept up to date behind the index as the deck is
/// edited.
#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistProjectIndex {
    deck: DeckIndex,
    /// The files indexed, in the order the index numbers them: the root,
    /// then its SPICE includes.
    files: Vec<IndexedFile>,
    /// The definitions, by their position among the deck's, in the order
    /// the navigator lists them: by name ignoring case, then file and line.
    /// An edit that keeps them shares them.
    symbols: Arc<[u32]>,
    tables: Arc<TablesCell>,
}

/// The index of a large deck holds large tables: whoever lets go of it last,
/// it is freed away from that thread, a page at a time.
impl Drop for NetlistProjectIndex {
    fn drop(&mut self) {
        rspice_design::freed_away::free_away_with(self, |index, release| {
            std::mem::take(&mut index.deck).let_go(release);
        });
    }
}

/// One file of the index: where it opens, and the text it was indexed from.
#[derive(Debug, Clone)]
struct IndexedFile {
    root: NetlistRoot,
    dependency_identity: Option<String>,
    display_name: String,
    kind: SourceKind,
    /// The text indexed, and its revision: a text at it is that text, told
    /// without reading it.
    text: RevisedText,
}

/// The tables a project index lists of the whole deck, read from the index
/// of one revision of it.
#[derive(Debug, Default)]
pub(crate) struct ProjectTables {
    deck: DeckIndex,
    files: Vec<IndexedFile>,
    hierarchy: Vec<ProjectHierarchyInstance>,
    hierarchy_truncated: bool,
    /// The node names each scope sees, by the scope: the positions in the
    /// deck's nodes of the first written of each name, in the order of the
    /// names ignoring case, so the names a prefix starts are found by a
    /// search.
    nodes_by_name: Vec<Vec<u32>>,
}

/// The tables share the index of their revision with the project index:
/// whichever lets go of it last, its pages are freed away from that thread,
/// a page at a time.
impl Drop for ProjectTables {
    fn drop(&mut self) {
        rspice_design::freed_away::free_away_with(self, |tables, release| {
            std::mem::take(&mut tables.deck).let_go(release);
        });
    }
}

/// The tables of the latest revision worked out, shared by the indexes of
/// one root from one edit to the next, and the index whose tables are being
/// worked out next.
#[derive(Debug, Default)]
struct TablesCell {
    state: Mutex<TablesState>,
}

#[derive(Debug, Default)]
struct TablesState {
    latest: Option<Arc<ProjectTables>>,
    building: bool,
    /// The deck and files of the newest revision, when it came while
    /// another's tables were being worked out.
    pending: Option<(DeckIndex, Vec<IndexedFile>)>,
    /// What wakes the frame once the tables are worked out: the last wake a
    /// frame gave, kept for an edit an action follows, which has none.
    wake: Option<TablesWake>,
}

/// Wakes the frame that shows the tables.
#[derive(Clone)]
struct TablesWake(Arc<dyn Fn() + Send + Sync>);

impl TablesWake {
    fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Arc::new(wake))
    }
}

impl std::fmt::Debug for TablesWake {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("TablesWake").finish_non_exhaustive()
    }
}

impl TablesCell {
    /// A cell holding the tables of `deck`, worked out here.
    fn worked_out(deck: &DeckIndex, files: &[IndexedFile]) -> Self {
        Self {
            state: Mutex::new(TablesState {
                latest: Some(Arc::new(ProjectTables::of(deck.clone(), files.to_vec()))),
                building: false,
                pending: None,
                wake: None,
            }),
        }
    }

    fn latest(&self) -> Option<Arc<ProjectTables>> {
        self.state.lock().ok()?.latest.clone()
    }

    /// Work out the tables of `deck` away from this thread, after those
    /// being worked out; `wake` once they are, or without one, the last
    /// wake given.
    fn refresh(
        self: &Arc<Self>,
        deck: DeckIndex,
        files: Vec<IndexedFile>,
        wake: Option<TablesWake>,
    ) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        if wake.is_some() {
            state.wake = wake;
        }
        if state.building {
            state.pending = Some((deck, files));
            return;
        }
        state.building = true;
        drop(state);
        let cell = Arc::clone(self);
        let mut next = Some((deck, files));
        let spawned = std::thread::Builder::new()
            .name("netlist-tables".to_owned())
            .spawn(move || {
                let mut wake = None;
                while let Some((deck, files)) = next.take() {
                    let tables = Arc::new(ProjectTables::of(deck, files));
                    let Ok(mut state) = cell.state.lock() else {
                        return;
                    };
                    if let Some(old) = state.latest.replace(tables) {
                        drop(state);
                        drop(old);
                        state = match cell.state.lock() {
                            Ok(state) => state,
                            Err(_) => return,
                        };
                    }
                    next = state.pending.take();
                    if next.is_none() {
                        state.building = false;
                        wake = state.wake.clone();
                    }
                }
                if let Some(TablesWake(wake)) = wake {
                    wake();
                }
            });
        if spawned.is_err()
            && let Ok(mut state) = self.state.lock()
        {
            state.building = false;
        }
    }
}

impl ProjectTables {
    fn of(deck: DeckIndex, files: Vec<IndexedFile>) -> Self {
        let mut tables = Self {
            nodes_by_name: nodes_by_name(&deck),
            deck,
            files,
            hierarchy: Vec::new(),
            hierarchy_truncated: false,
        };
        tables.expand_hierarchy();
        tables
    }

    fn expand_hierarchy(&mut self) {
        let deck = &self.deck;
        let mut children = std::collections::HashMap::<String, Vec<usize>>::new();
        let mut top = Vec::new();
        for instance in deck.symbols() {
            if instance.kind() != DeckSymbolKind::Instance
                || !instance.name().starts_with(['X', 'x'])
                || instance.detail().is_empty()
            {
                continue;
            }
            let parent = deck.scope(instance.scope()).and_then(|scope| scope.name());
            match parent {
                Some(parent) => children
                    .entry(parent.to_ascii_lowercase())
                    .or_default()
                    .push(instance.id()),
                None => top.push(instance.id()),
            }
        }
        let mut pending = top
            .into_iter()
            .rev()
            .filter_map(|instance| {
                let path = deck.symbol(instance)?.name().into_owned();
                Some((instance, path, Vec::<String>::new()))
            })
            .collect::<Vec<_>>();
        let mut hierarchy = Vec::new();
        while let Some((instance, path, mut ancestors)) = pending.pop() {
            if hierarchy.len() >= MAX_EXPANDED_HIERARCHY_INSTANCES {
                self.hierarchy_truncated = true;
                break;
            }
            let Some(symbol) = deck.symbol(instance) else {
                continue;
            };
            let target = symbol.detail().into_owned();
            let target_key = target.to_ascii_lowercase();
            let recursive = ancestors.iter().any(|ancestor| ancestor == &target_key);
            hierarchy.push(ProjectHierarchyInstance {
                path: path.clone(),
                target,
                location: location_in(&self.files, &symbol.location()),
                recursive,
            });
            if recursive {
                continue;
            }
            ancestors.push(target_key.clone());
            if let Some(nested) = children.get(&target_key) {
                pending.extend(nested.iter().rev().filter_map(|child| {
                    let child_path = format!("{path}/{}", deck.symbol(*child)?.name());
                    Some((*child, child_path, ancestors.clone()))
                }));
            }
        }
        self.hierarchy = hierarchy;
    }

    pub(crate) fn hierarchy(&self) -> &[ProjectHierarchyInstance] {
        &self.hierarchy
    }

    pub(crate) const fn hierarchy_truncated(&self) -> bool {
        self.hierarchy_truncated
    }

    /// The node name at `position` of those a reader in `scope` sees: the
    /// names written in the scope, the deck's global ones and ground, each
    /// once as first written, in the order of the names ignoring case.
    pub(crate) fn node_at(&self, scope: usize, position: usize) -> Option<Cow<'_, str>> {
        let node = *self.nodes_by_name.get(scope)?.get(position)?;
        self.deck.node(node as usize).map(|node| node.name())
    }

    fn node_name(&self, node: u32) -> Cow<'_, str> {
        self.deck
            .node(node as usize)
            .map_or(Cow::Borrowed(""), |node| node.name())
    }

    /// The positions, as [`Self::node_at`] reads them, of the node names a
    /// reader in `scope` sees that start with `prefix`, ignoring case.
    pub(crate) fn nodes_starting(&self, scope: usize, prefix: &str) -> std::ops::Range<usize> {
        let Some(names) = self.nodes_by_name.get(scope) else {
            return 0..0;
        };
        let start = names.partition_point(|&node| by_name(&self.node_name(node), prefix).is_lt());
        let starting = names[start..].partition_point(|&node| {
            self.node_name(node)
                .as_bytes()
                .get(..prefix.len())
                .is_some_and(|head| head.eq_ignore_ascii_case(prefix.as_bytes()))
        });
        start..start + starting
    }

    /// The position, as [`Self::node_at`] reads it, of the node `name` a
    /// reader in `scope` sees, ignoring case.
    pub(crate) fn node_named(&self, scope: usize, name: &str) -> Option<usize> {
        self.nodes_by_name
            .get(scope)?
            .binary_search_by(|&node| by_name(&self.node_name(node), name))
            .ok()
    }
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

/// Whether an edit from `old` to `new` can change how the engine reads the
/// text as a whole, SPICE or Spectre: whether the text it removes or puts in,
/// or the bytes beside them, could open or close a block comment or write a
/// `simulator lang=` switch.
fn may_change_language(change: DeckTextChange, old: &RevisedText, new: &RevisedText) -> bool {
    let lines = |text: &RevisedText, start: usize, end: usize| {
        text.slice(text.line_start_at(start)..text.line_end_at(end))
            .to_ascii_lowercase()
    };
    [
        lines(old, change.start, change.old_end),
        lines(new, change.start, change.new_end),
    ]
    .iter()
    .any(|written| written.contains("/*") || written.contains("*/") || written.contains("lang"))
}

impl NetlistProjectIndex {
    /// The index of `texts`, the root `root` first, then the SPICE files it
    /// includes.
    fn build(root: NetlistRoot, texts: &[IndexText]) -> Self {
        let files = texts
            .iter()
            .enumerate()
            .map(|(position, text)| IndexedFile {
                root,
                dependency_identity: text.dependency_identity.clone(),
                display_name: text.display_name.clone(),
                kind: SourceKind::of(&text.path, &text.text, position == 0),
                text: text.text.clone(),
            })
            .collect::<Vec<_>>();
        let sources = files
            .iter()
            .map(|file| {
                DeckIndexSource::new(
                    file.dependency_identity.as_deref().unwrap_or(""),
                    Arc::new(file.text.clone()) as SharedDeckText,
                    file.kind,
                )
            })
            .collect::<Vec<_>>();
        let deck = DeckIndex::build(&sources);
        let tables = Arc::new(TablesCell::worked_out(&deck, &files));
        Self::of(deck, files, tables)
    }

    fn of(deck: DeckIndex, files: Vec<IndexedFile>, tables: Arc<TablesCell>) -> Self {
        let mut symbols = (0..deck.definition_count() as u32).collect::<Vec<_>>();
        let definitions = deck.definitions().collect::<Vec<_>>();
        symbols.sort_by(|left, right| {
            let (left, right) = (&definitions[*left as usize], &definitions[*right as usize]);
            by_name(&left.name(), &right.name())
                .then_with(|| {
                    file_name_of(&files, left.file()).cmp(file_name_of(&files, right.file()))
                })
                .then_with(|| left.line().cmp(&right.line()))
        });
        drop(definitions);
        Self {
            deck,
            files,
            symbols: symbols.into(),
            tables,
        }
    }

    /// The index of `texts` built from this one, the index of the same
    /// files before an edit: when one file was edited and the edit leaves
    /// what the deck defines as it was, the part of the engine's index it
    /// touched is read again and the rest shared, the definitions keep their
    /// order, and the tables follow away from this thread; else it is built
    /// anew.
    fn edited(&self, root: NetlistRoot, texts: &[IndexText], wake: Option<TablesWake>) -> Self {
        let same_files = self.files.len() == texts.len()
            && self.files.iter().zip(texts).all(|(file, text)| {
                file.root == root && file.dependency_identity == text.dependency_identity
            });
        if !same_files {
            return Self::build(root, texts);
        }
        let mut edited = None;
        for (position, (file, text)) in self.files.iter().zip(texts).enumerate() {
            if file.text.same_text(&text.text) {
                continue;
            }
            let change = text
                .text
                .edit_since(file.text.revision())
                .or_else(|| DeckTextChange::between(&file.text, &text.text));
            let Some(change) = change else {
                continue;
            };
            if edited.is_some() || may_change_language(change, &file.text, &text.text) {
                return Self::build(root, texts);
            }
            edited = Some((position, change));
        }
        let mut files = self.files.clone();
        for (file, text) in files.iter_mut().zip(texts) {
            file.text = text.text.clone();
        }
        let deck = match edited {
            None => self.deck.clone(),
            Some((position, change)) => {
                let text = Arc::new(files[position].text.clone()) as SharedDeckText;
                match self.deck.edited(position, change, text) {
                    Some(deck) => deck,
                    None => return Self::build(root, texts),
                }
            }
        };
        self.tables.refresh(deck.clone(), files.clone(), wake);
        Self {
            deck,
            files,
            // The definitions are the same, in the same order: an edit that
            // changes them is built anew.
            symbols: Arc::clone(&self.symbols),
            tables: Arc::clone(&self.tables),
        }
    }

    /// The tables of the latest revision worked out: this index's, or until
    /// they are, an earlier one's.
    pub(crate) fn tables(&self) -> Arc<ProjectTables> {
        self.tables.latest().unwrap_or_default()
    }

    pub(crate) fn symbol_count(&self) -> usize {
        self.symbols.len()
    }

    /// The definition at `position` of the order the navigator lists them
    /// in.
    pub(crate) fn symbol(&self, position: usize) -> Option<ProjectSymbol> {
        let symbol = self
            .deck
            .nth_definition(*self.symbols.get(position)? as usize)?;
        let kind = ProjectSymbolKind::of(symbol.kind())?;
        let detail = symbol.detail();
        let detail = match kind {
            ProjectSymbolKind::Parameter if detail.is_empty() => ".param".to_owned(),
            ProjectSymbolKind::Parameter => format!("= {detail}"),
            _ => detail.into_owned(),
        };
        Some(ProjectSymbol {
            name: symbol.name().into_owned(),
            kind,
            detail,
            definition: location_in(&self.files, &symbol.location()),
        })
    }

    /// The definitions, in the order the navigator lists them.
    pub(crate) fn symbols(&self) -> impl Iterator<Item = ProjectSymbol> + '_ {
        (0..self.symbols.len()).filter_map(|position| self.symbol(position))
    }

    /// The read at `position` among the deck's, as the navigator lists it:
    /// one bound to a definition, or a name that can only be a subcircuit or
    /// a model. Elsewhere a name nothing defines may be a node or a keyword,
    /// and is no read.
    pub(crate) fn reference(&self, position: usize) -> Option<ProjectReference> {
        let reference = self.deck.reference(position)?;
        let kind = ProjectSymbolKind::of(reference.kind())?;
        (reference.target().is_some()
            || matches!(
                kind,
                ProjectSymbolKind::Subcircuit | ProjectSymbolKind::Model
            ))
        .then(|| ProjectReference {
            name: reference.name().into_owned(),
            kind,
            location: location_in(&self.files, &reference.location()),
        })
    }

    /// The reads the navigator lists, each with its position among the
    /// deck's.
    pub(crate) fn references(&self) -> impl Iterator<Item = (usize, ProjectReference)> + '_ {
        (0..self.deck.reference_count())
            .filter_map(|position| Some((position, self.reference(position)?)))
    }

    pub(crate) fn source_count(&self) -> usize {
        self.files.len()
    }

    /// The definitions named `name`, ignoring case, found by a search of
    /// the order the navigator lists them in.
    pub(crate) fn definitions_named(&self, name: &str) -> Vec<ProjectSymbol> {
        let name_of = |at: &u32| {
            self.deck
                .nth_definition(*at as usize)
                .map(|symbol| symbol.name())
                .unwrap_or_default()
        };
        let start = self
            .symbols
            .partition_point(|at| by_name(&name_of(at), name).is_lt());
        let end =
            start + self.symbols[start..].partition_point(|at| by_name(&name_of(at), name).is_eq());
        (start..end)
            .filter_map(|position| self.symbol(position))
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn references_named(&self, name: &str) -> Vec<ProjectReference> {
        self.references()
            .map(|(_, reference)| reference)
            .filter(|reference| reference.name.eq_ignore_ascii_case(name))
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn hierarchy(&self) -> Vec<ProjectHierarchyInstance> {
        self.tables().hierarchy().to_vec()
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
        file_name_of(&self.files, file)
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
                .all(|(file, text)| file.text == **text)
    }

    /// The file the index numbers the source `dependency_identity` names
    /// (the root for `None`), while `text` is still the text it indexed: at
    /// once when it is at the revision indexed, else by comparing the two.
    pub(crate) fn file_holding(
        &self,
        dependency_identity: Option<&str>,
        text: &RevisedText,
    ) -> Option<usize> {
        let file = self
            .files
            .iter()
            .position(|file| file.dependency_identity.as_deref() == dependency_identity)?;
        self.files[file].text.same_text(text).then_some(file)
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
            DeckNameAt::Reference(reference) => self.deck.reference(reference)?.target(),
        };
        Some(
            target
                .and_then(|symbol| self.deck.symbol(symbol))
                .filter(|symbol| symbol.kind() != DeckSymbolKind::Instance)
                .map(|symbol| location_in(&self.files, &symbol.location())),
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
                let read = self.deck.reference(reference)?;
                let Some(symbol) = read.target() else {
                    let name = read.name();
                    return Some(ProjectSymbolHover {
                        title: name.clone().into_owned(),
                        detail: messages
                            .format(MessageId::NetlistNothingInScopeDefines, &[("name", &name)]),
                    });
                };
                symbol
            }
        };
        let definition = self.deck.symbol(symbol)?;
        let detail = definition.detail();
        let (kind, what) = match definition.kind() {
            DeckSymbolKind::Instance => return None,
            DeckSymbolKind::Parameter if detail.is_empty() => {
                (MessageId::NetlistSymbolParameter, String::new())
            }
            DeckSymbolKind::Parameter => (MessageId::NetlistSymbolParameter, format!("= {detail}")),
            DeckSymbolKind::Subcircuit => (
                MessageId::NetlistSymbolSubcircuit,
                definition
                    .opens()
                    .and_then(|scope| self.deck.scope(scope))
                    .map(|scope| {
                        messages.format(
                            MessageId::NetlistSymbolPorts,
                            &[("ports", &scope.ports().join(" "))],
                        )
                    })
                    .unwrap_or_default(),
            ),
            DeckSymbolKind::Model => (MessageId::NetlistSymbolModel, detail.into_owned()),
            DeckSymbolKind::Function => (MessageId::NetlistSymbolFunction, detail.into_owned()),
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
                    ("file", self.file_name(definition.file())),
                    ("line", &definition.line().to_string()),
                ],
            )
        );
        Some(ProjectSymbolHover {
            title: messages.format(kind, &[("name", &definition.name())]),
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
                if self
                    .deck
                    .symbol(symbol)
                    .is_some_and(|symbol| symbol.kind() != DeckSymbolKind::Instance) =>
            {
                Ok(symbol)
            }
            Some(DeckNameAt::Reference(reference)) => {
                let read = self.deck.reference(reference)?;
                read.target().ok_or_else(|| read.name().into_owned())
            }
            _ => {
                let line = self.deck.line_of(file, byte)?;
                self.call_on_line(file, line)?
            }
        };
        let symbol = match bound {
            Ok(symbol) => self.deck.symbol(symbol)?,
            Err(name) => return Some(Err(name)),
        };
        let symbol_name = symbol.name();
        let name = [("name", &*symbol_name)];
        let detail = symbol.detail();
        let mut parts = Vec::new();
        let title = match symbol.kind() {
            DeckSymbolKind::Instance => return None,
            DeckSymbolKind::Parameter => {
                if !detail.is_empty() {
                    parts.push(format!("= {detail}"));
                }
                messages.format(MessageId::NetlistSymbolParameter, &name)
            }
            DeckSymbolKind::Subcircuit => {
                let ports = self.ports_of(&symbol);
                if !ports.is_empty() {
                    parts.push(messages.format(
                        MessageId::NetlistSymbolPorts,
                        &[("ports", &ports.join(" "))],
                    ));
                }
                // Its formals: the parameters its own card defines.
                let formals = symbol
                    .card_view()
                    .symbols()
                    .filter(|formal| {
                        formal.kind() == DeckSymbolKind::Parameter
                            && Some(formal.scope()) == symbol.opens()
                    })
                    .map(|formal| {
                        let value = formal.detail();
                        if value.is_empty() {
                            formal.name().into_owned()
                        } else {
                            format!("{}={value}", formal.name())
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
                if !detail.is_empty() {
                    parts.push(detail.clone().into_owned());
                }
                messages.format(MessageId::NetlistSymbolModel, &name)
            }
            DeckSymbolKind::Function => {
                if !detail.is_empty() {
                    parts.push(detail.clone().into_owned());
                }
                messages.format(MessageId::NetlistSymbolFunction, &name)
            }
        };
        parts.push(messages.format(
            MessageId::NetlistSymbolPlace,
            &[
                ("file", self.file_name(symbol.file())),
                ("line", &symbol.line().to_string()),
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
        let card = self.deck.card(self.deck.card_on_line(file, line)?)?;
        if let Some(target) = card
            .symbols()
            .find(|symbol| symbol.kind() == DeckSymbolKind::Instance)
            .and_then(|instance| instance.target())
        {
            return Some(Ok(target));
        }
        let read = card.references().find(|read| {
            matches!(
                read.kind(),
                DeckSymbolKind::Subcircuit | DeckSymbolKind::Model
            )
        })?;
        Some(read.target().ok_or_else(|| read.name().into_owned()))
    }

    /// The parameters a reader on one-based `line` of the source
    /// `dependency_identity` names sees, each the definition the engine binds
    /// there: a subcircuit's own and its formals inside it, the deck's
    /// outside.
    pub(crate) fn parameters_visible_at(
        &self,
        dependency_identity: Option<&str>,
        line: usize,
    ) -> Vec<DeckSymbol<'_>> {
        let scope = self.scope_at(dependency_identity, line);
        self.deck
            .parameters_visible_in(scope)
            .into_iter()
            .filter_map(|symbol| self.deck.symbol(symbol))
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
    ) -> Vec<DeckSymbol<'_>> {
        let reach = self.deck.scope_chain(scope).collect::<Vec<_>>();
        let wanted = |symbol: &DeckSymbol<'_>| {
            kinds.contains(&symbol.kind()) && reach.contains(&symbol.scope())
        };
        // A deck's devices are read only when they are asked for.
        if kinds.contains(&DeckSymbolKind::Instance) {
            self.deck.symbols().filter(wanted).collect()
        } else {
            self.deck.definitions().filter(wanted).collect()
        }
    }

    /// The ports of the subcircuit `symbol`, in order.
    pub(crate) fn ports_of<'a>(&'a self, symbol: &DeckSymbol<'_>) -> Vec<Cow<'a, str>> {
        symbol
            .opens()
            .and_then(|scope| self.deck.scope(scope))
            .map_or_else(Vec::new, |scope| scope.ports())
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
        .filter(|symbol| seen.insert((symbol.kind(), symbol.name().to_ascii_lowercase())))
        .filter_map(|symbol| {
            let kind = ProjectSymbolKind::of(symbol.kind())?;
            let definition = location_in(&self.files, &symbol.location());
            Some(super::completion::SymbolEntry {
                name: symbol.name().into_owned(),
                kind: kind.label(),
                detail: symbol.detail().into_owned(),
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

fn file_name_of(files: &[IndexedFile], file: usize) -> &str {
    files
        .get(file)
        .map_or("", |file| file.display_name.as_str())
}

/// `left` against `right`, ignoring ASCII case.
fn by_name(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

/// The node names each scope of `deck` sees, as
/// [`ProjectTables::node_at`] reads them: those written in the scope, the
/// global ones and ground, the first written of each name standing for it.
fn nodes_by_name(deck: &DeckIndex) -> Vec<Vec<u32>> {
    let mut by_scope = vec![Vec::<u32>::new(); deck.scope_count().max(1)];
    let mut everywhere = Vec::new();
    // Every node's name, as `by_name` orders it, in one buffer while the
    // names are sorted, read out of the deck's text once rather than at
    // every comparison: where each ends, by the node's position.
    let mut written = Vec::<u8>::new();
    let mut ends = Vec::<usize>::with_capacity(deck.node_count());
    for node in deck.nodes() {
        let position = node.id() as u32;
        let name = node.name();
        written.extend(name.bytes().map(|byte| byte.to_ascii_lowercase()));
        ends.push(written.len());
        if name == "0" || node.card_kind() == rspice_core::netlist::DeckCardKind::Global {
            everywhere.push(position);
        } else if let Some(names) = by_scope.get_mut(node.scope()) {
            names.push(position);
        }
    }
    let name_of = |position: u32| {
        let at = position as usize;
        let start = at.checked_sub(1).map_or(0, |before| ends[before]);
        &written[start..ends[at]]
    };
    for names in &mut by_scope {
        // Sorted by each name ignoring case, and then by where it is
        // written, so the first written of a name is the one kept.
        names.extend_from_slice(&everywhere);
        names.sort_unstable_by(|one, two| name_of(*one).cmp(name_of(*two)).then(one.cmp(two)));
        names.dedup_by(|later, kept| name_of(*later) == name_of(*kept));
        names.shrink_to_fit();
    }
    by_scope
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
            if netlist.language_off().is_some() {
                return None;
            }
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
/// from it: from the last one built, reading again only what an edit
/// touched, when it is of the same root. While the writer types, the last
/// index built stands in until the one of the root as it stands is built; a
/// root changed any other way has none until its own is built. It reads the
/// model.
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
        let last = netlist
            .project_index
            .built()
            .filter(|(built, _)| built.0 == key.0)
            .map(|(_, index)| index);
        let wake = ctx.clone();
        let wake_tables = ctx.clone();
        netlist.project_index.start(
            key,
            move || match last {
                Some(last) => last.edited(
                    root,
                    &texts,
                    Some(TablesWake::new(move || wake_tables.request_repaint())),
                ),
                None => NetlistProjectIndex::build(root, &texts),
            },
            move || wake.request_repaint(),
        );
    }
    let (built, index) = netlist.project_index.built()?;
    (*built == key || typed).then_some(index)
}

/// The project index of the root the editor shows as it stands, waiting for
/// its build to finish: what an action answers from. An action pressed
/// before the frame started that build reads the text from the index of the
/// same root built last, again only where the edit since touched it, so a
/// key and the action after it cost what the key touched.
pub(crate) fn project_index_now(
    model: &Model,
    netlist: &mut NetlistDocumentState,
) -> Option<Arc<NetlistProjectIndex>> {
    let (root, key) = shown_root_in(model, netlist)?;
    if let Some(index) = netlist.project_index.wait_for(|built| *built == key) {
        return Some(index);
    }
    let texts = index_texts_in(model, netlist, root)?;
    let last = netlist
        .project_index
        .built()
        .filter(|(built, _)| built.0 == key.0)
        .map(|(_, index)| index);
    let index = match last {
        Some(last) => last.edited(root, &texts, None),
        None => NetlistProjectIndex::build(root, &texts),
    };
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
    /// The text mapped, shared: the next revision's map is this one read
    /// again where the edit since touched it.
    text: RevisedText,
    /// Whether it is the lexical map, every line taken as read: built
    /// without a parse, or after one that left nothing unread.
    lexical: bool,
}

/// What a value map was built for: the buffer's revision, and whether it
/// applies what a parse of it left unread.
type ValueMapKey = (u64, bool);

/// The value maps of the buffer, built away from the editor's thread.
pub(crate) type ValueMapSlot = BackgroundIndex<ValueMapKey, ValueMap>;

impl ValueMap {
    /// The map of `buffer`, read as its parse read it when `unread` carries
    /// what that parse left out.
    fn build(buffer: &RevisedText, unread: Option<&[UnreadSourceRegion]>) -> Self {
        let map = match unread {
            Some(unread) => rspice_core::netlist::source_map_for_editor_as_read(buffer, unread),
            None => rspice_core::netlist::source_map_for_editor(buffer),
        };
        Self::of(map, buffer.clone(), unread.is_none_or(<[_]>::is_empty))
    }

    /// The lexical map of `buffer`, from this one, a lexical map: the cards
    /// the edit since touched read again and the rest moved. An edit to what
    /// scopes the lines around it, or to a deck only a parse reads, is
    /// built anew.
    fn edited(&self, buffer: &RevisedText) -> Self {
        let change = buffer
            .edit_since(self.text.revision())
            .or_else(|| DeckTextChange::between(&self.text, buffer));
        let Some(change) = change else {
            return Self::of(self.map.clone(), buffer.clone(), true);
        };
        match rspice_core::netlist::source_map_for_editor_edited(
            &self.map, &self.text, buffer, change,
        ) {
            Some(map) => Self::of(map, buffer.clone(), true),
            None => Self::build(buffer, None),
        }
    }

    fn of(map: NetlistSourceMap, text: RevisedText, lexical: bool) -> Self {
        let spans = &map.value_spans;
        let mut by_start = (0..spans.len()).collect::<Vec<_>>();
        by_start.sort_by_key(|at| spans[*at].span.start);
        let widest = spans.iter().map(|span| span.span.len()).max().unwrap_or(0);
        Self {
            map,
            by_start,
            widest,
            text,
            lexical,
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
    if netlist.front.is_comparison() || netlist.shown_language_off() {
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
        // The lexical map of a revision before this one is read again only
        // where the edit since touched it. A parse that left nothing unread
        // read every line as the lexical map does.
        let unread = unread.filter(|unread| !unread.is_empty());
        let last = slot.built().map(|(_, map)| map).filter(|map| map.lexical);
        let build = move |text: &RevisedText| match (&last, unread.as_deref()) {
            (Some(last), None) => last.edited(text),
            (_, unread) => ValueMap::build(text, unread),
        };
        let map = if wait {
            slot.wait_for(|built| *built == key)
                .unwrap_or_else(|| slot.store(key, build(buffer)))
        } else {
            if slot.wants(|built| *built == key) {
                let (text, wake) = (buffer.clone(), ctx.clone());
                slot.start(key, move || build(&text), move || wake.request_repaint());
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

/// Why an action that reads the project index has none: the deck's language
/// features are off, or there is no index of the text shown.
pub(crate) fn no_index(netlist: &NetlistDocumentState, messages: MessageCatalog) -> String {
    messages.text(if netlist.shown_language_off() {
        MessageId::NetlistDeckLanguageOffShort
    } else {
        MessageId::NetlistSignatureNoIndex
    })
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
    let index = project_index_now(model, netlist).ok_or_else(|| no_index(netlist, messages))?;
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
            let index =
                project_index_now(model, netlist).ok_or_else(|| no_index(netlist, messages))?;
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
    let index = project_index_now(model, netlist).ok_or_else(|| no_index(netlist, messages))?;
    let count = index.symbol_count();
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
