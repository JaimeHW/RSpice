//! Sheet-scoped geometry operations over the document and its editor history.

use crate::state::{BusTap, Junction, SchematicState, Wire};

impl SchematicState {
    pub(crate) fn with_active_wire_topology<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState) -> R,
    ) -> R {
        let hidden_wire_ids = self
            .document
            .wires
            .iter()
            .filter(|wire| hidden(wire.id))
            .map(|wire| wire.id)
            .collect::<std::collections::HashSet<_>>();
        let hidden_junction_ids = self
            .document
            .junctions
            .iter()
            .filter(|junction| hidden(junction.id))
            .map(|junction| junction.id)
            .collect::<std::collections::HashSet<_>>();

        let hidden_wires = take_hidden(&mut self.document.wires, |wire: &Wire| {
            hidden_wire_ids.contains(&wire.id)
        });
        let hidden_junctions = take_hidden(&mut self.document.junctions, |junction: &Junction| {
            hidden_junction_ids.contains(&junction.id)
        });
        let result = operation(self);
        restore_hidden(&mut self.document.wires, hidden_wires);
        restore_hidden(&mut self.document.junctions, hidden_junctions);
        result
    }

    pub(crate) fn with_hidden_wire_topology_preserved<R>(
        &mut self,
        hidden: impl Fn(u64) -> bool,
        operation: impl FnOnce(&mut SchematicState) -> R,
    ) -> R {
        let hidden_wires = self
            .document
            .wires
            .iter()
            .enumerate()
            .filter(|(_, wire)| hidden(wire.id))
            .map(|(index, wire)| (index, wire.clone()))
            .collect::<Vec<_>>();
        let hidden_junctions = self
            .document
            .junctions
            .iter()
            .enumerate()
            .filter(|(_, junction)| hidden(junction.id))
            .map(|(index, junction)| (index, *junction))
            .collect::<Vec<_>>();
        let hidden_bus_taps = self
            .document
            .bus_taps
            .iter()
            .enumerate()
            .filter(|(_, tap)| hidden(tap.id))
            .map(|(index, tap)| (index, tap.clone()))
            .collect::<Vec<_>>();

        let result = operation(self);
        restore_authored_by_id(&mut self.document.wires, hidden_wires, |wire: &Wire| {
            wire.id
        });
        restore_authored_by_id(
            &mut self.document.junctions,
            hidden_junctions,
            |junction: &Junction| junction.id,
        );
        restore_authored_by_id(
            &mut self.document.bus_taps,
            hidden_bus_taps,
            |tap: &BusTap| tap.id,
        );
        result
    }
}

fn take_hidden<T>(items: &mut Vec<T>, hidden: impl Fn(&T) -> bool) -> Vec<(usize, T)> {
    let mut retained = Vec::with_capacity(items.len());
    let mut removed = Vec::new();
    for (index, item) in std::mem::take(items).into_iter().enumerate() {
        if hidden(&item) {
            removed.push((index, item));
        } else {
            retained.push(item);
        }
    }
    *items = retained;
    removed
}

fn restore_hidden<T>(items: &mut Vec<T>, hidden: Vec<(usize, T)>) {
    for (index, item) in hidden {
        items.insert(index.min(items.len()), item);
    }
}

fn restore_authored_by_id<T>(
    items: &mut Vec<T>,
    authored: Vec<(usize, T)>,
    id: impl Fn(&T) -> u64,
) {
    for (original_index, object) in authored {
        if let Some(current_index) = items
            .iter()
            .position(|candidate| id(candidate) == id(&object))
        {
            items[current_index] = object;
        } else {
            items.insert(original_index.min(items.len()), object);
        }
    }
}
