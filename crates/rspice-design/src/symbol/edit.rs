//! Committed symbol history, metadata snapshots, and placed-instance edits.

use std::collections::{BTreeMap, HashMap};

use super::{SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SymbolDocument};
use crate::library::View;
use crate::schematic::component::Component;
use crate::schematic::document::SchematicDocument;
use crate::schematic::history::SchematicSnapshot;
use rspice_design_model::{Point, cell_view::CellViewRef};

/// What an author meant by a symbol edit, beyond the geometry it produced.
///
/// Comparing two documents can only report what changed, and a pin rename is
/// indistinguishable from a delete plus an add by that measure — the old name
/// is simply gone. Placed instances wire to pins *by name*, so the rename has
/// to be declared by whoever performed it and carried with the commit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolCommitIntent {
    /// Old pin name → new pin name.
    pub renames: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolDocumentSnapshot {
    pub document: SymbolDocument,
    pub symbol_document_metadata: Option<String>,
    pub symbol_editor_metadata: Option<String>,
    pub generated_metadata: Option<String>,
    pub ports_metadata: Option<String>,
    /// Pin renames restoring this snapshot must carry to placed instances.
    ///
    /// Restoring the document alone would leave every instance wired to the
    /// name the edit introduced while the symbol declares the old one again,
    /// which detaches the connection instead of undoing it. The entry is
    /// written when a rename is committed, and inverted each time the edit
    /// crosses between the undo and redo stacks.
    pub renames: std::collections::BTreeMap<String, String>,
}

impl SymbolDocumentSnapshot {
    pub fn from_document(document: &SymbolDocument) -> Self {
        Self {
            document: document.clone(),
            symbol_document_metadata: None,
            symbol_editor_metadata: None,
            generated_metadata: None,
            ports_metadata: None,
            renames: std::collections::BTreeMap::new(),
        }
    }

    /// The same edit read in the other direction.
    pub fn inverted_renames(&self) -> std::collections::BTreeMap<String, String> {
        self.renames
            .iter()
            .map(|(from, to)| (to.clone(), from.clone()))
            .collect()
    }
}

/// One open schematic that places the edited cell, held for the length of a
/// symbol edit.
#[derive(Debug, Clone)]
pub struct SymbolParentWiring {
    /// The open schematic buffer's key, or `None` for the active schematic.
    pub buffer: Option<String>,
    /// The schematic before the edit's first change. The edit reaches the
    /// schematic as one undo record taken from here when it ends.
    pub before: SchematicSnapshot,
    pub wiring: SymbolInstanceWiring,
}

/// A symbol edit in progress, and the parent schematics it carries along.
///
/// A pin drag publishes the symbol on every frame, but it is one edit: its
/// wire ownership is resolved at the first frame and each parent records one
/// undo step when it ends.
#[derive(Debug, Clone)]
pub struct SymbolParentEdit {
    /// The symbol before the edit's first change.
    pub base: SymbolDocument,
    pub parents: Vec<SymbolParentWiring>,
}

/// Committed symbol edits and publication depths, keyed by cellview.
#[derive(Debug, Clone, Default)]
pub struct SymbolHistory {
    undo_stacks: HashMap<String, Vec<SymbolDocumentSnapshot>>,
    redo_stacks: HashMap<String, Vec<SymbolDocumentSnapshot>>,
    save_points: HashMap<String, usize>,
    /// The edit whose parent wiring is held open, by the cellview it edits.
    parent_edit: Option<(String, SymbolParentEdit)>,
}

impl SymbolHistory {
    pub fn begin_parent_edit(&mut self, key: String, edit: SymbolParentEdit) {
        self.parent_edit = Some((key, edit));
    }

    pub fn parent_edit(&self, key: &str) -> Option<&SymbolParentEdit> {
        self.parent_edit
            .as_ref()
            .filter(|(owner, _)| owner == key)
            .map(|(_, edit)| edit)
    }

    pub fn take_parent_edit(&mut self) -> Option<SymbolParentEdit> {
        self.parent_edit.take().map(|(_, edit)| edit)
    }

    /// Whether the cellview carries edits its last published revision does
    /// not contain. This is the single authority: no surface re-derives
    /// dirtiness by comparing documents or reading a stored modified flag.
    pub fn is_dirty(&self, key: &str) -> bool {
        let depth = self.undo_depth(key);
        depth != self.save_points.get(key).copied().unwrap_or(0)
    }

    /// Record that everything on the stack up to here is published.
    pub fn mark_save_point(&mut self, key: impl Into<String>) {
        let key = key.into();
        let depth = self.undo_stacks.get(&key).map_or(0, Vec::len);
        self.save_points.insert(key, depth);
    }

    pub fn undo_depth(&self, key: &str) -> usize {
        self.undo_stacks.get(key).map_or(0, Vec::len)
    }

    pub fn record_edit(&mut self, key: String, snapshot: SymbolDocumentSnapshot, max_len: usize) {
        let undo_stack = self.undo_stacks.entry(key.clone()).or_default();
        undo_stack.push(snapshot);
        if undo_stack.len() > max_len {
            undo_stack.remove(0);
        }
        self.redo_stacks.remove(&key);
    }

    pub fn record_inverse_renames(&mut self, key: &str, renames: &BTreeMap<String, String>) {
        if let Some(entry) = self
            .undo_stacks
            .get_mut(key)
            .and_then(|stack| stack.last_mut())
        {
            entry.renames = renames
                .iter()
                .map(|(from, to)| (to.clone(), from.clone()))
                .collect();
        }
    }

    pub fn can_undo(&self, key: &str) -> bool {
        self.undo_stacks
            .get(key)
            .is_some_and(|stack| !stack.is_empty())
    }

    pub fn can_redo(&self, key: &str) -> bool {
        self.redo_stacks
            .get(key)
            .is_some_and(|stack| !stack.is_empty())
    }

    pub fn pop_undo(&mut self, key: &str) -> Option<SymbolDocumentSnapshot> {
        self.undo_stacks.get_mut(key).and_then(Vec::pop)
    }

    pub fn pop_redo(&mut self, key: &str) -> Option<SymbolDocumentSnapshot> {
        self.redo_stacks.get_mut(key).and_then(Vec::pop)
    }

    pub fn finish_undo(&mut self, key: String, current: SymbolDocumentSnapshot) {
        self.redo_stacks.entry(key).or_default().push(current);
    }

    pub fn finish_redo(&mut self, key: String, current: SymbolDocumentSnapshot) {
        self.undo_stacks.entry(key).or_default().push(current);
    }

    /// Forget the edits waiting to be redone, for edits the author threw
    /// away rather than undid.
    pub fn discard_redo(&mut self, key: &str) {
        self.redo_stacks.remove(key);
    }
}

/// Where each pin's terminal moved to, keyed by the name it had before.
///
/// This answers one question only — did a terminal move — so that the wire
/// endpoints attached to it move with it. It cannot answer whether a pin was
/// renamed: the old name is simply absent from `after`, which is
/// indistinguishable from a deletion. `renames` supplies that half, declared
/// by whoever performed the rename, so a pin that was renamed *and* moved is
/// still followed to its new position.
pub fn symbol_pin_position_remaps(
    before: &SymbolDocument,
    after: &SymbolDocument,
    renames: &std::collections::BTreeMap<String, String>,
) -> HashMap<String, (Point, Point)> {
    let mut remaps = HashMap::new();
    for before_pin in &before.pins {
        let Some(old_position) = before_pin.position.map(|position| position - before.origin)
        else {
            continue;
        };
        let after_name = renames
            .get(&before_pin.name)
            .map_or(before_pin.name.as_str(), String::as_str);
        let Some(after_pin) = after.pin(after_name) else {
            continue;
        };
        let Some(new_position) = after_pin.position.map(|position| position - after.origin) else {
            continue;
        };
        if old_position != new_position {
            remaps.insert(
                before_pin.name.to_ascii_lowercase(),
                (old_position, new_position),
            );
        }
    }
    remaps
}

/// One wire vertex that a placed instance's terminal holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalWireEnd {
    pub component_id: u64,
    /// The pin's name in the symbol the hold was resolved against, lowercased.
    pub terminal: String,
    pub wire_id: u64,
    pub point_index: usize,
    /// Where the vertex stood when the hold was resolved.
    pub point: Point,
}

/// Which wire vertices the terminals of every placed instance of one cell hold
/// in one schematic.
///
/// A symbol edit moves a pin, and every wire end that pin held has to move
/// with it. Which ends those are is a question about ownership, and it can
/// only be answered by position *before* anything moves: once a dragged pin
/// passes over another pin's point, a vertex at that point no longer says
/// whose it is. So the hold is resolved once, when the edit starts, and the
/// edit then moves vertices by the terminal that holds them, never by where
/// they happen to sit.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolInstanceWiring {
    ends: Vec<TerminalWireEnd>,
}

impl SymbolInstanceWiring {
    pub fn ends(&self) -> &[TerminalWireEnd] {
        &self.ends
    }

    pub fn is_empty(&self) -> bool {
        self.ends.is_empty()
    }
}

/// Whether the schematic places at least one instance of `reference`.
pub fn places_symbol(schematic: &SchematicDocument, reference: &CellViewRef) -> bool {
    schematic
        .components
        .iter()
        .any(|component| is_instance_of(component, reference))
}

/// Resolve which wire vertices each terminal of each instance of `reference`
/// holds, with the terminals where `symbol` puts them.
///
/// A vertex is held when it sits exactly on the terminal and its wire is drawn
/// on the instance's own sheet: `same_sheet(component_id, wire_id)` answers
/// that, since a cell view's sheets share one coordinate space and a wire on
/// another sheet can sit on the same point without touching the instance. A
/// vertex that two terminals both sit on belongs to neither, so a pin edit
/// leaves it where it is rather than guessing.
pub fn resolve_symbol_instance_wiring(
    schematic: &SchematicDocument,
    reference: &CellViewRef,
    symbol: &SymbolDocument,
    same_sheet: impl Fn(u64, u64) -> bool,
) -> SymbolInstanceWiring {
    let mut claims: Vec<TerminalWireEnd> = Vec::new();
    for component in &schematic.components {
        if !is_instance_of(component, reference) {
            continue;
        }
        let terminals = instance_terminal_points(component, symbol);
        if terminals.is_empty() {
            continue;
        }
        for wire in &schematic.wires {
            if !same_sheet(component.id, wire.id) {
                continue;
            }
            for (point_index, point) in wire.points.iter().enumerate() {
                claims.extend(
                    terminals
                        .iter()
                        .filter(|(_, terminal)| terminal == point)
                        .map(|(name, _)| TerminalWireEnd {
                            component_id: component.id,
                            terminal: name.clone(),
                            wire_id: wire.id,
                            point_index,
                            point: *point,
                        }),
                );
            }
        }
    }
    let mut holders: HashMap<(u64, usize), usize> = HashMap::new();
    for claim in &claims {
        *holders
            .entry((claim.wire_id, claim.point_index))
            .or_default() += 1;
    }
    claims.retain(|claim| holders.get(&(claim.wire_id, claim.point_index)) == Some(&1));
    SymbolInstanceWiring { ends: claims }
}

/// Put every held vertex where its terminal now stands.
///
/// `pin_remaps` is [`symbol_pin_position_remaps`] from the symbol the wiring
/// was resolved against to the symbol as it is now. A terminal it does not
/// name has not moved since then, so its vertices go back to where they were
/// resolved; that is what makes the result depend only on the two symbols and
/// not on the path a drag took between them.
pub fn apply_symbol_instance_wiring(
    schematic: &mut SchematicDocument,
    wiring: &SymbolInstanceWiring,
    pin_remaps: &HashMap<String, (Point, Point)>,
) -> bool {
    let mut changed = false;
    for end in &wiring.ends {
        let target = match pin_remaps.get(&end.terminal) {
            Some(&(_, new_offset)) => {
                let Some(component) = schematic
                    .components
                    .iter()
                    .find(|component| component.id == end.component_id)
                else {
                    continue;
                };
                world_terminal_point(component, new_offset)
            }
            None => end.point,
        };
        let Some(vertex) = schematic
            .wires
            .iter_mut()
            .find(|wire| wire.id == end.wire_id)
            .and_then(|wire| wire.points.get_mut(end.point_index))
        else {
            continue;
        };
        if *vertex != target {
            *vertex = target;
            changed = true;
        }
    }
    changed
}

fn is_instance_of(component: &Component, reference: &CellViewRef) -> bool {
    component.library_cell.as_ref().is_some_and(|binding| {
        binding.library.eq_ignore_ascii_case(&reference.library)
            && binding.cell.eq_ignore_ascii_case(&reference.cell)
    })
}

/// The instance's terminals, by lowercased pin name, in schematic space.
fn instance_terminal_points(
    component: &Component,
    symbol: &SymbolDocument,
) -> Vec<(String, Point)> {
    let terminal_order = component
        .library_cell
        .as_ref()
        .map(|binding| binding.terminal_order.as_slice())
        .unwrap_or_default();
    symbol
        .pins
        .iter()
        .filter(|pin| {
            terminal_order.is_empty()
                || terminal_order
                    .iter()
                    .any(|terminal| terminal.eq_ignore_ascii_case(&pin.name))
        })
        .filter_map(|pin| {
            let offset = pin.position? - symbol.origin;
            Some((
                pin.name.to_ascii_lowercase(),
                world_terminal_point(component, offset),
            ))
        })
        .collect()
}

fn world_terminal_point(component: &Component, offset: Point) -> Point {
    let offset = component.transform_point(offset);
    Point::new(
        component.pos.x.saturating_add(offset.x),
        component.pos.y.saturating_add(offset.y),
    )
}

pub fn symbol_snapshot_from_view(
    view: &View,
    fallback: &SymbolDocument,
    preserve_missing_metadata: bool,
) -> SymbolDocumentSnapshot {
    let symbol_document_metadata = match view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY) {
        Some(encoded) => Some(encoded.clone()),
        None if preserve_missing_metadata => None,
        None => serde_json::to_string(fallback).ok(),
    };
    SymbolDocumentSnapshot {
        document: fallback.clone(),
        symbol_document_metadata,
        symbol_editor_metadata: view.metadata.get(SYMBOL_EDITOR_METADATA_KEY).cloned(),
        generated_metadata: view.metadata.get("generated").cloned(),
        ports_metadata: view.metadata.get("ports").cloned(),
        renames: std::collections::BTreeMap::new(),
    }
}

pub fn symbol_metadata_snapshot_from_view(
    view: &View,
    fallback: &SymbolDocument,
) -> SymbolDocumentSnapshot {
    SymbolDocumentSnapshot {
        document: fallback.clone(),
        symbol_document_metadata: view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY).cloned(),
        symbol_editor_metadata: view.metadata.get(SYMBOL_EDITOR_METADATA_KEY).cloned(),
        generated_metadata: view.metadata.get("generated").cloned(),
        ports_metadata: view.metadata.get("ports").cloned(),
        renames: std::collections::BTreeMap::new(),
    }
}

pub fn restore_symbol_snapshot_in_view(view: &mut View, snapshot: &SymbolDocumentSnapshot) {
    match &snapshot.symbol_document_metadata {
        Some(encoded) => {
            view.metadata
                .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove(SYMBOL_DOCUMENT_METADATA_KEY);
        }
    }
    match &snapshot.symbol_editor_metadata {
        Some(encoded) => {
            view.metadata
                .insert(SYMBOL_EDITOR_METADATA_KEY.to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove(SYMBOL_EDITOR_METADATA_KEY);
        }
    }
    match &snapshot.generated_metadata {
        Some(encoded) => {
            view.metadata
                .insert("generated".to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove("generated");
        }
    }
    match &snapshot.ports_metadata {
        Some(encoded) => {
            view.metadata.insert("ports".to_owned(), encoded.clone());
        }
        None => {
            view.metadata.remove("ports");
        }
    }
}

/// Rewrite the terminal names of every instance of `reference` in one sheet.
///
/// `renames` is keyed by lowercased old name; instance bindings and wire
/// connections both match terminals case-insensitively, so both are rewritten
/// from the same table and cannot drift apart.
pub fn rename_instance_terminals(
    schematic: &mut SchematicDocument,
    reference: &CellViewRef,
    renames: &BTreeMap<String, String>,
) -> usize {
    let instances: Vec<u64> = schematic
        .components
        .iter()
        .filter(|component| {
            component.library_cell.as_ref().is_some_and(|binding| {
                binding.library.eq_ignore_ascii_case(&reference.library)
                    && binding.cell.eq_ignore_ascii_case(&reference.cell)
            })
        })
        .map(|component| component.id)
        .collect();
    if instances.is_empty() {
        return 0;
    }

    let mut renamed = 0;
    for component in &mut schematic.components {
        let Some(binding) = component.library_cell.as_mut() else {
            continue;
        };
        if !instances.contains(&component.id) {
            continue;
        }
        for terminal in &mut binding.terminal_order {
            if let Some(new_name) = renames.get(&terminal.to_ascii_lowercase()) {
                *terminal = new_name.clone();
            }
        }
    }
    for connection in &mut schematic.connections {
        if !instances.contains(&connection.component_id) {
            continue;
        }
        if let Some(new_name) = renames.get(&connection.terminal_name.to_ascii_lowercase()) {
            connection.terminal_name = new_name.clone();
            renamed += 1;
        }
    }
    renamed
}
