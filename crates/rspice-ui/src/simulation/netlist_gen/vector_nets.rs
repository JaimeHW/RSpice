//! Naming a vector's conductors on the cards that carry them.
//!
//! A deck has no vectors. Every conductor a bus carries has to reach the engine
//! as its own node, and the engine has to be told the same name for that
//! conductor wherever it appears — on a `.SUBCKT` header, on the instance that
//! drives it, on a probe. Minting those nodes belongs to
//! [`super::extraction`], which is the design's one connectivity pass; what
//! lives here is spelling the same conductors onto the emitted cards, so a
//! header and every instance of it agree bit for bit.
//!
//! The rule both halves hold is that a vector net is identified by its
//! declaration, not by its geometry. Buses that touch and declare the same
//! range are one vector net; a vector port or a vector instance terminal joins
//! the bus beneath it only when the two declarations are identical. Names come
//! from [`deck_bit_name`], so two drawings of `DATA[7:0]` anywhere in one cell
//! resolve to the same eight nodes and can never resolve to a ninth.
//!
//! A vector join whose two ends declare different conductors is refused under
//! either [`crate::state::BundleWidthMismatchPolicy`], and that is what the
//! permissive variant's name asks for rather than a weakening of it: the only
//! way to author "explicit slice or extend" is a tap carrying the selector, and
//! a tap that carries one produces a destination declaration that matches, so
//! no mismatch reaches the projection. What is left when one does reach it is
//! an implicit mismatch, and emitting it would mean inventing or dropping
//! conductors the drawing never named. The policy decides the severity the ERC
//! reports it at — a mismatch a wider bus could be sliced into is a warning
//! there — but it never decides whether a deck may carry one.

use super::*;
use crate::state::declared_vector;

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
