//! Committed symbol history, metadata snapshots, and placed-instance edits.

use std::collections::{BTreeMap, HashMap};

use super::{SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SymbolDocument};
use crate::library::View;
use crate::schematic::component::Component;
use crate::schematic::document::SchematicDocument;
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

/// Committed symbol edits and publication depths, keyed by cellview.
#[derive(Debug, Clone, Default)]
pub struct SymbolHistory {
    undo_stacks: HashMap<String, Vec<SymbolDocumentSnapshot>>,
    redo_stacks: HashMap<String, Vec<SymbolDocumentSnapshot>>,
    save_points: HashMap<String, usize>,
}

impl SymbolHistory {
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

pub fn remap_symbol_instance_wires(
    schematic: &mut SchematicDocument,
    reference: &CellViewRef,
    pin_remaps: &HashMap<String, (Point, Point)>,
) -> bool {
    if pin_remaps.is_empty() {
        return false;
    }

    let mut world_remaps = Vec::new();
    for component in &schematic.components {
        append_component_symbol_remaps(component, reference, pin_remaps, &mut world_remaps);
    }
    if world_remaps.is_empty() {
        return false;
    }

    let mut updates: Vec<(usize, usize, Point)> = Vec::new();
    for (wire_index, wire) in schematic.wires.iter().enumerate() {
        for (point_index, point) in wire.points.iter().enumerate() {
            if let Some((_, new_position)) = world_remaps
                .iter()
                .find(|(old_position, _)| point == old_position)
            {
                updates.push((wire_index, point_index, *new_position));
            }
        }
    }
    if updates.is_empty() {
        return false;
    }

    for (wire_index, point_index, new_position) in updates {
        if let Some(wire) = schematic.wires.get_mut(wire_index)
            && point_index < wire.points.len()
        {
            wire.points[point_index] = new_position;
        }
    }
    true
}

fn append_component_symbol_remaps(
    component: &Component,
    reference: &CellViewRef,
    pin_remaps: &HashMap<String, (Point, Point)>,
    world_remaps: &mut Vec<(Point, Point)>,
) {
    let Some(binding) = component.library_cell.as_ref() else {
        return;
    };
    if !binding.library.eq_ignore_ascii_case(&reference.library)
        || !binding.cell.eq_ignore_ascii_case(&reference.cell)
    {
        return;
    }

    if binding.terminal_order.is_empty() {
        for &(old_offset, new_offset) in pin_remaps.values() {
            push_world_pin_remap(component, old_offset, new_offset, world_remaps);
        }
        return;
    }

    for terminal_name in &binding.terminal_order {
        let Some(&(old_offset, new_offset)) = pin_remaps.get(&terminal_name.to_ascii_lowercase())
        else {
            continue;
        };
        push_world_pin_remap(component, old_offset, new_offset, world_remaps);
    }
}

fn push_world_pin_remap(
    component: &Component,
    old_offset: Point,
    new_offset: Point,
    world_remaps: &mut Vec<(Point, Point)>,
) {
    let old_offset = component.transform_point(old_offset);
    let new_offset = component.transform_point(new_offset);
    let old_position = Point::new(
        component.pos.x.saturating_add(old_offset.x),
        component.pos.y.saturating_add(old_offset.y),
    );
    let new_position = Point::new(
        component.pos.x.saturating_add(new_offset.x),
        component.pos.y.saturating_add(new_offset.y),
    );
    if old_position != new_position {
        world_remaps.push((old_position, new_position));
    }
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
