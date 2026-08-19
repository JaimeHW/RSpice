//! The notation a solved vector bit is shown in, taken from the bus that
//! declared it.
//!
//! The deck spells one bit `DATA#3` and the engine answers under that name, so
//! every result vector, every lookup key and every raw export carries it. A
//! reader must never see it: `#` is a character no drawing can contain, and
//! quoting it back names a conductor the design does not have.
//!
//! [`super::vector_names::display_bit_name`] owns the spelling; this module
//! owns the one question it cannot answer on its own — which bus declared the
//! bit, and therefore which delimiters the design wrote it between. The answer
//! is a lookup and never a default, because `BASE#DIGITS` is not proof of a
//! bus: the engine mints `T1#1` for a multiconductor line's conductors and
//! `R#0` for a periodic-noise reference, and an imported deck may author `#`
//! anywhere. A name no declaration claims is left exactly as the engine
//! returned it.
//!
//! The substitution is display-only. Nothing here is written back into a
//! waveform name, a saved output, a persisted label or an export: the engine
//! spelling stays the key, and the design's spelling is produced afresh each
//! time a surface paints one.

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use super::vector_names::display_bit_name;
use crate::state::{BusNotation, PortSpec, ProjectWorkspace, SchematicState};

/// Every vector one design declares, and the notation it was declared in.
///
/// Buses and vector interface pins both declare, and both mint deck bits, so
/// both are read. A name two sheets declare in two different notations has no
/// answer here: either would quote a spelling the other sheet does not use, so
/// the bit keeps its deck name rather than picking a side.
#[derive(Debug, Default)]
pub(crate) struct BusNotations {
    by_bus: HashMap<String, Option<BusNotation>>,
}

/// The notations the project's own drawings declare, live editor buffer
/// included.
///
/// The retained buffers answer rather than a resolved design projection,
/// because this runs on the paint path of four surfaces and a projection costs
/// a content digest over every cell view per call. A declaration is authored on
/// a drawing, so nothing about the answer is weaker for the design's own cells;
/// a bus declared only inside a master the project does not own keeps its deck
/// spelling, which is what an unclaimed bit does anyway.
pub(crate) fn bus_notations(workspace: &ProjectWorkspace, active: &SchematicState) -> BusNotations {
    BusNotations::of_sheets(
        workspace
            .schematic_buffers
            .values()
            .chain(std::iter::once(active)),
    )
}

impl BusNotations {
    /// The notations `sheets` declare between them.
    fn of_sheets<'a>(sheets: impl IntoIterator<Item = &'a SchematicState>) -> Self {
        let mut by_bus = HashMap::new();
        for sheet in sheets {
            let buses = sheet.buses.iter().filter_map(|bus| bus.declaration.clone());
            let ports = sheet.interface_ports();
            for declaration in buses.chain(ports.iter().filter_map(PortSpec::vector)) {
                match by_bus.entry(declaration.name.to_ascii_lowercase()) {
                    Entry::Vacant(slot) => {
                        slot.insert(Some(declaration.notation));
                    }
                    Entry::Occupied(mut slot) => {
                        if *slot.get() != Some(declaration.notation) {
                            slot.insert(None);
                        }
                    }
                }
            }
        }
        Self { by_bus }
    }

    /// One result vector name, spelled as the design that produced it declared
    /// it.
    ///
    /// A bare node name, an accessor-wrapped one such as `V(x1.data#3)`, and a
    /// nested projection such as `phase(v(data#3))` are all handled, because
    /// the surfaces that show a result name show whichever of those the engine
    /// returned. Anything else — an expression over two signals, a name no
    /// declaration claims, a scalar — is returned untouched.
    pub(crate) fn display<'a>(&self, name: &'a str) -> Cow<'a, str> {
        let name = name.trim();
        if let Some((accessor, operand)) = name.split_once('(')
            && let Some(operand) = operand.strip_suffix(')')
            && !accessor.is_empty()
            && accessor.bytes().all(|byte| byte.is_ascii_alphanumeric())
        {
            return match self.display(operand) {
                Cow::Borrowed(_) => Cow::Borrowed(name),
                Cow::Owned(shown) => Cow::Owned(format!("{accessor}({shown})")),
            };
        }
        self.bit(name).map_or(Cow::Borrowed(name), Cow::Owned)
    }

    /// `name` rendered as the bit it is, or `None` when no bus declares it.
    ///
    /// The base is recovered from a rendering rather than by splitting the
    /// deck name here — `display_bit_name` is the only reader of `#`, and the
    /// base it produces is the same whichever notation it is asked for.
    fn bit(&self, name: &str) -> Option<String> {
        let square = display_bit_name(name, BusNotation::Square)?;
        let (base, _) = square.rsplit_once('[')?;
        match self.declared(base)? {
            BusNotation::Square => Some(square),
            notation => display_bit_name(name, notation),
        }
    }

    /// The notation declared for `base`, whose leading segments may be the
    /// instance path the engine flattened the bit through.
    ///
    /// Segments are dropped from the front, longest candidate first, so a bus
    /// whose own name contains a dot is found before the suffix it ends with.
    fn declared(&self, base: &str) -> Option<BusNotation> {
        let mut candidate = base;
        loop {
            if let Some(declared) = self.by_bus.get(&candidate.to_ascii_lowercase()) {
                return *declared;
            }
            candidate = candidate.split_once('.')?.1;
        }
    }
}

#[cfg(test)]
mod tests;
