//! Vector nets become scalar nodes exactly once, here.
//!
//! A deck has no vectors. Every conductor a bus carries has to reach the
//! engine as its own node, and the engine has to be told the same name for
//! that conductor wherever it appears — on a `.SUBCKT` header, on the instance
//! that drives it, on a probe. This module is the single place a vector net
//! becomes that set of nodes.
//!
//! The rule it holds is that a vector net is identified by its declaration, not
//! by its geometry. Buses that touch and declare the same range are one vector
//! net; a vector port or a vector instance terminal joins the bus beneath it
//! only when the two declarations are identical. Projection then names the
//! net's conductors with [`deck_bit_name`], so two drawings of `DATA[7:0]`
//! anywhere in one cell resolve to the same eight nodes and can never resolve
//! to a ninth.
//!
//! The projection deliberately adds nets and never renames one. A scalar tap
//! already aliased its wire to the bit's deck name before vector nets existed;
//! after projection that alias merges into the projected bit instead of minting
//! a second node with the same name. Decks that only ever used scalar taps are
//! therefore byte-identical.

use super::*;
use crate::state::{declared_vector, vector_connectivity};

/// The formals one interface name contributes, in declaration order.
///
/// A scalar name is its own formal. A vector name expands from its declared
/// MSB end toward its LSB end — the order [`crate::state::BusDeclaration`]
/// expands members in — so a header and every instance of it agree bit for bit.
pub(super) fn formals_of(name: &str) -> Vec<String> {
    match declared_vector(name) {
        Some(declaration) => declaration
            .members()
            .into_iter()
            .map(|member| deck_bit_name(&declaration.name, member.index))
            .collect(),
        None => vec![name.to_owned()],
    }
}

/// The deck-spelled formals of one cell's interface.
///
/// `interface_ports` stays the cell's contract — one entry per drawn pin,
/// vectors unexpanded — because that is what a placement binds to and what a
/// stale-interface check compares. This is the deck's view of the same
/// contract, and only the `.SUBCKT` header and instance node lists use it.
pub(super) fn interface_formals(schematic: &SchematicState) -> Vec<String> {
    schematic
        .interface_ports()
        .iter()
        .flat_map(|port| formals_of(&port.name))
        .collect()
}

impl<'a> NetlistGenerator<'a> {
    /// Project every vector net of this schematic into named scalar nodes.
    ///
    /// Runs between net extraction and interface-port naming: the bits exist
    /// before any name is resolved, so a scalar tap and a vector port both
    /// resolve onto the same projected node instead of racing to create it.
    pub(super) fn expand_vector_nets(&mut self) {
        let schematic = self.schematic;
        let connectivity = vector_connectivity(schematic, |component| {
            self.component_terminal_positions(component)
        });

        for mismatch in &connectivity.mismatches {
            self.errors.push(mismatch.message());
        }

        let mut taken: HashSet<String> = self
            .nets
            .iter()
            .filter_map(|net| net.label.as_ref())
            .map(|label| label.to_ascii_lowercase())
            .collect();
        let mut next_id = self.nets.iter().map(|net| net.id).max().unwrap_or(0) + 1;
        for vector in &connectivity.nets {
            for member in vector.declaration.members() {
                let name = deck_bit_name(&vector.declaration.name, member.index);
                if !taken.insert(name.to_ascii_lowercase()) {
                    continue;
                }
                let mut projected = Net::new(next_id);
                projected.label = Some(name);
                self.nets.push(projected);
                next_id += 1;
            }
        }
    }

    /// Deck nodes for one component's terminals: one node per conductor.
    ///
    /// A scalar terminal contributes the node under it. A vector terminal
    /// contributes its declared conductors — the projected bits of the bus it
    /// joins when it joins one, and bits of its own floating node when it does
    /// not, so an unconnected vector pin stays as private as an unconnected
    /// scalar pin instead of shorting to every other instance of its cell.
    pub(super) fn vector_terminal_nodes(&self, names: &[String], points: &[Point]) -> Vec<String> {
        let mut nodes = Vec::with_capacity(names.len());
        for (name, point) in names.iter().zip(points) {
            let Some(declaration) = declared_vector(name) else {
                nodes.push(self.get_node_name(*point));
                continue;
            };
            let joined = self.schematic.buses.iter().any(|bus| {
                bus.declaration.as_ref() == Some(&declaration) && bus.contains_point(*point)
            });
            let base = if joined {
                declaration.name.clone()
            } else {
                self.get_node_name(*point)
            };
            nodes.extend(
                declaration
                    .members()
                    .into_iter()
                    .map(|member| deck_bit_name(&base, member.index)),
            );
        }
        nodes
    }
}

/// One interface name and the conductors it contributes, for a message that
/// has to explain a width sum rather than assert it.
pub(super) fn width_contributions(names: &[String]) -> String {
    names
        .iter()
        .map(|name| format!("{name} contributes {}", crate::state::declared_width(name)))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests;
