//! Temporary sheet-topology isolation used by editor operations.
use super::super::{component::Component, net_label::Junction, wire::Wire};
use super::Schematic;
pub struct HiddenWireTopology {
    wires: Vec<(usize, Wire)>,
    junctions: Vec<(usize, Junction)>,
    /// Parts taken out with their sheet's wires: their pins decide dots.
    components: Vec<(usize, Component)>,
}
impl HiddenWireTopology {
    pub fn restore(self, design: &mut Schematic) {
        restore_hidden(&mut design.document.wires, self.wires);
        restore_hidden(&mut design.document.junctions, self.junctions);
        restore_hidden(&mut design.document.components, self.components);
    }
}
impl Schematic {
    pub fn take_hidden_wire_topology(
        &mut self,
        hidden: impl Fn(u64) -> bool,
    ) -> HiddenWireTopology {
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
        let hidden_components = take_hidden(&mut self.document.components, |component| {
            hidden(component.id)
        });
        HiddenWireTopology {
            wires: hidden_wires,
            junctions: hidden_junctions,
            components: hidden_components,
        }
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
