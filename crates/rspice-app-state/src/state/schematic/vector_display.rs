//! The notation a solved vector bit is shown in, taken from the bus that
//! declared it, and the spelling a solved net is shown in, taken from the
//! drawing that named it.
//!
//! The deck spells one bit `DATA#3` and the engine answers under that name, so
//! every result vector, every lookup key and every raw export carries it. A
//! reader must never see it: `#` is a character no drawing can contain, and
//! quoting it back names a conductor the design does not have.
//!
//! [`rspice_design::connectivity::display_bit_name`] owns the spelling; this module
//! owns the one question it cannot answer on its own — which bus declared the
//! bit, and therefore which delimiters the design wrote it between. The answer
//! is a lookup and never a default, because `BASE#DIGITS` is not proof of a
//! bus: the engine mints `T1#1` for a multiconductor line's conductors and
//! `R#0` for a periodic-noise reference, and an imported deck may author `#`
//! anywhere. A name no declaration claims is left exactly as the engine
//! returned it.
//!
//! The engine folds every node name to upper case, so a net drawn `b0` comes
//! back `B0`. A reader looks for the name on the drawing, so each net name
//! and bus name a drawing writes is shown as written; the case is the only
//! thing that changes.
//!
//! The substitution is display-only. Nothing here is written back into a
//! waveform name, a saved output, a persisted label or an export: the engine
//! spelling stays the key, and the design's spelling is produced afresh each
//! time a surface paints one.

use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::state::{BusNotation, PortSpec, ProjectWorkspace, SchematicState};
use rspice_design::connectivity::display_bit_name;

/// Every vector one design declares, and the notation it was declared in;
/// every net and bus name it draws, and how it spells each.
///
/// Buses and vector interface pins both declare, and both mint deck bits, so
/// both are read. A name two sheets declare in two different notations has no
/// answer here: either would quote a spelling the other sheet does not use, so
/// the bit keeps its deck name rather than picking a side. A name two drawings
/// spell in two different cases keeps the engine's spelling for the same
/// reason.
#[derive(Debug, Default)]
pub struct BusNotations {
    by_bus: HashMap<String, Option<BusNotation>>,
    /// Keyed by the engine's spelling: the name folded to upper case.
    drawn: HashMap<String, Option<String>>,
}

/// The notations the project's own drawings declare, live editor buffer
/// included.
///
/// The retained buffers answer rather than a resolved design projection,
/// because this runs on the paint path of six surfaces and a projection costs
/// a content digest over every cell view per call. A declaration is authored on
/// a drawing, so nothing about the answer is weaker for the design's own cells;
/// a bus declared only inside a master the project does not own keeps its deck
/// spelling, which is what an unclaimed bit does anyway.
pub fn bus_notations(workspace: &ProjectWorkspace, active: &SchematicState) -> BusNotations {
    BusNotations::of_sheets(
        workspace
            .content
            .schematic_buffers
            .values()
            .map(|schematic| schematic.document())
            .chain(std::iter::once(active.document())),
    )
}

impl BusNotations {
    /// The notations `sheets` declare between them.
    fn of_sheets<'a>(
        sheets: impl IntoIterator<Item = &'a rspice_design::schematic::document::SchematicDocument>,
    ) -> Self {
        let mut by_bus = HashMap::new();
        let mut drawn = HashMap::new();
        for sheet in sheets {
            let buses = sheet.buses.iter().filter_map(|bus| bus.declaration.clone());
            let ports = sheet.interface_ports();
            for port in ports.iter().filter(|port| port.vector().is_none()) {
                spell(&mut drawn, port.name.trim());
            }
            for label in &sheet.net_labels {
                match label.global_declaration() {
                    Some(global) => spell(&mut drawn, &global),
                    None => spell(&mut drawn, label.name.trim()),
                }
            }
            for declaration in buses.chain(ports.iter().filter_map(PortSpec::vector)) {
                spell(&mut drawn, &declaration.name);
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
        Self { by_bus, drawn }
    }

    /// One result vector name, spelled as the design that produced it declared
    /// it.
    ///
    /// A bare node name, an accessor-wrapped one such as `V(x1.data#3)`, and a
    /// nested projection such as `phase(v(data#3))` are all handled, because
    /// the surfaces that show a result name show whichever of those the engine
    /// returned. Anything else — an expression over two signals, a name no
    /// declaration claims, a scalar — is returned untouched. A net the design
    /// draws reads as drawn; inside a current or a power, whose operand names
    /// a device rather than a net, only a bit is re-spelled.
    pub fn display<'a>(&self, name: &'a str) -> Cow<'a, str> {
        self.display_operand(name, true)
    }

    fn display_operand<'a>(&self, name: &'a str, net: bool) -> Cow<'a, str> {
        let name = name.trim();
        if let Some((accessor, operand)) = name.split_once('(')
            && let Some(operand) = operand.strip_suffix(')')
            && !accessor.is_empty()
            && accessor.bytes().all(|byte| byte.is_ascii_alphanumeric())
        {
            let net = net
                && !DEVICE_ACCESSORS
                    .iter()
                    .any(|device| accessor.eq_ignore_ascii_case(device));
            return match self.display_operand(operand, net) {
                Cow::Borrowed(_) => Cow::Borrowed(name),
                Cow::Owned(shown) => Cow::Owned(format!("{accessor}({shown})")),
            };
        }
        if let Some(bit) = self.bit(name) {
            return Cow::Owned(bit);
        }
        if net && let Some(shown) = self.drawn_net(name) {
            return Cow::Owned(shown);
        }
        Cow::Borrowed(name)
    }

    /// `name` with the net its last segments name spelled as drawn, its
    /// instance scope kept; `None` when no drawing names it, or it already
    /// reads as drawn.
    fn drawn_net(&self, name: &str) -> Option<String> {
        if self.drawn.is_empty() {
            return None;
        }
        let mut candidate = name;
        loop {
            if let Some(spelling) = self.spelling(candidate) {
                let drawn = spelling?;
                let scope = &name[..name.len() - candidate.len()];
                return (drawn != candidate).then(|| format!("{scope}{drawn}"));
            }
            candidate = candidate.split_once('.')?.1;
        }
    }

    /// How the drawings spell `name`: `Some(None)` when two of them disagree.
    fn spelling(&self, name: &str) -> Option<Option<&str>> {
        let mut buffer = [0_u8; 64];
        let key = match buffer.get_mut(..name.len()) {
            Some(key) => {
                key.copy_from_slice(name.as_bytes());
                key.make_ascii_uppercase();
                std::str::from_utf8(key).ok()
            }
            None => None,
        };
        let entry = match key {
            Some(key) => self.drawn.get(key),
            None => self.drawn.get(&name.to_ascii_uppercase()),
        }?;
        Some(entry.as_deref())
    }

    /// `name` rendered as the bit it is, or `None` when no bus declares it.
    ///
    /// The base is recovered from a rendering rather than by splitting the
    /// deck name here — `display_bit_name` is the only reader of `#`, and the
    /// base it produces is the same whichever notation it is asked for.
    fn bit(&self, name: &str) -> Option<String> {
        let square = display_bit_name(name, BusNotation::Square)?;
        let (base, _) = square.rsplit_once('[')?;
        let (scope, notation) = self.declared(base)?;
        let rendered = match notation {
            BusNotation::Square => square.clone(),
            notation => display_bit_name(name, notation)?,
        };
        let bus = &base[scope..];
        match self.spelling(bus) {
            Some(Some(drawn)) if drawn != bus => Some(format!(
                "{}{drawn}{}",
                &rendered[..scope],
                &rendered[base.len()..]
            )),
            _ => Some(rendered),
        }
    }

    /// The notation declared for `base`, whose leading segments may be the
    /// instance path the engine flattened the bit through.
    ///
    /// Segments are dropped from the front, longest candidate first, so a bus
    /// whose own name contains a dot is found before the suffix it ends with.
    /// The answer carries where in `base` the bus's own name starts.
    fn declared(&self, base: &str) -> Option<(usize, BusNotation)> {
        let mut candidate = base;
        loop {
            if let Some(declared) = self.by_bus.get(&candidate.to_ascii_lowercase()) {
                return declared.map(|notation| (base.len() - candidate.len(), notation));
            }
            candidate = candidate.split_once('.')?.1;
        }
    }
}

/// The accessors whose operand names a device, not a net: a branch current
/// and its parts, and a device's power.
const DEVICE_ACCESSORS: [&str; 7] = ["i", "im", "ip", "ir", "ii", "idb", "p"];

/// Record that a drawing spells a net or bus `name`, keyed by the engine's
/// spelling of it. A vector-shaped label (`data[3]`) names a bit the bus's
/// notation answers for, so only whole names are recorded.
fn spell(drawn: &mut HashMap<String, Option<String>>, name: &str) {
    if name.is_empty() || name.contains(['[', '<', '{', '#', '(']) {
        return;
    }
    match drawn.entry(name.to_ascii_uppercase()) {
        Entry::Vacant(slot) => {
            slot.insert(Some(name.to_owned()));
        }
        Entry::Occupied(mut slot) => {
            if slot.get().as_deref() != Some(name) {
                slot.insert(None);
            }
        }
    }
}

#[cfg(test)]
mod tests;
