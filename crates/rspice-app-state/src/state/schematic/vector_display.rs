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
//! thing that changes. Instance and device names fold the same way, so a
//! capacitor drawn `Cout` is answered as `COUT`; each element a drawing
//! places reads under the name it is written under, as drawn, in a current
//! (`I(Cout)`), a power, and every instance scope a name is flattened through
//! (`xtb.ok`). A measurement's name folds too, so one a plan authors `okt`
//! is answered as `OKT`; it reads as the plan spells it.
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
    /// The elements and instances the drawings place, by the name each card
    /// is written under, keyed by the engine's spelling as `drawn` is.
    elements: HashMap<String, Option<String>>,
    /// The measurements the project's plans author, keyed by the engine's
    /// spelling as `drawn` is.
    measurements: HashMap<String, Option<String>>,
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
    let mut notations = BusNotations::of_sheets(
        workspace
            .content
            .schematic_buffers
            .values()
            .map(|schematic| schematic.document())
            .chain(std::iter::once(active.document())),
    );
    for record in &workspace.content.simulation_plan_payloads {
        let payload = &record.payload;
        let defined = payload
            .specification_definitions
            .iter()
            .map(|definition| definition.measurement.as_str());
        let listed = payload.specs.iter().map(|spec| spec.measurement.as_str());
        for name in defined.chain(listed) {
            spell(&mut notations.measurements, name.trim());
        }
    }
    notations
}

impl BusNotations {
    /// The notations `sheets` declare between them.
    fn of_sheets<'a>(
        sheets: impl IntoIterator<Item = &'a rspice_design::schematic::document::SchematicDocument>,
    ) -> Self {
        let mut by_bus = HashMap::new();
        let mut drawn = HashMap::new();
        let mut elements = HashMap::new();
        for sheet in sheets {
            // A part with no device prefix (a port, a ground) writes no card.
            for component in &sheet.components {
                if !component.kind.spice_prefix().is_empty() {
                    spell(&mut elements, &component.engine_element_name());
                }
            }
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
        Self {
            by_bus,
            drawn,
            elements,
            measurements: HashMap::new(),
        }
    }

    /// A measurement's name, spelled as the plan that authors it writes it.
    /// One no plan authors, or two plans spell two ways, is left as the
    /// engine named it.
    pub fn display_measurement<'a>(&self, name: &'a str) -> Cow<'a, str> {
        match lookup(&self.measurements, name).flatten() {
            Some(authored) if authored != name => Cow::Owned(authored.to_owned()),
            _ => Cow::Borrowed(name),
        }
    }

    /// One result vector name, spelled as the design that produced it declared
    /// it.
    ///
    /// A bare node name, an accessor-wrapped one such as `V(x1.data#3)`, and a
    /// nested projection such as `phase(v(data#3))` are all handled, because
    /// the surfaces that show a result name show whichever of those the engine
    /// returned. Anything else — an expression over two signals, a name no
    /// declaration claims, a scalar — is returned untouched. A net the design
    /// draws reads as drawn, and so does each instance it is scoped through;
    /// inside a current or a power, whose operand names a device rather than
    /// a net, the device reads as drawn.
    pub fn display<'a>(&self, name: &'a str) -> Cow<'a, str> {
        self.display_operand(name, true)
    }

    /// An element or instance name read off a result (`XTB`, `XLDO.M1`), each
    /// segment spelled as the drawing that places it writes it; a segment no
    /// drawing places is left as the engine spelled it.
    pub fn display_element<'a>(&self, name: &'a str) -> Cow<'a, str> {
        let name = name.trim();
        let (scope, leaf) = match name.rsplit_once('.') {
            Some((scope, leaf)) => (&name[..=scope.len()], leaf),
            None => ("", name),
        };
        let drawn = self.element(leaf);
        match (self.scope(scope), drawn) {
            (Cow::Borrowed(_), None) => Cow::Borrowed(name),
            (scope, drawn) => Cow::Owned(format!("{scope}{}", drawn.unwrap_or(leaf))),
        }
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
        if !net {
            return self.display_element(name);
        }
        if let Some(shown) = self.drawn_net(name) {
            return Cow::Owned(shown);
        }
        // A net no drawing names keeps its own spelling; the instances it is
        // scoped through still read as drawn.
        match name.rsplit_once('.') {
            Some((scope, leaf)) => match self.scope(&name[..=scope.len()]) {
                Cow::Borrowed(_) => Cow::Borrowed(name),
                Cow::Owned(scope) => Cow::Owned(format!("{scope}{leaf}")),
            },
            None => Cow::Borrowed(name),
        }
    }

    /// An instance scope (`XLDO.XAMP.`, its trailing dot included) with each
    /// segment a drawing places spelled as drawn.
    fn scope<'a>(&self, scope: &'a str) -> Cow<'a, str> {
        if self.elements.is_empty()
            || !scope
                .split('.')
                .any(|segment| self.element(segment).is_some())
        {
            return Cow::Borrowed(scope);
        }
        let segments: Vec<&str> = scope
            .split('.')
            .map(|segment| self.element(segment).unwrap_or(segment))
            .collect();
        Cow::Owned(segments.join("."))
    }

    /// How the drawings write the element the engine names `name`, when that
    /// differs from the engine's spelling.
    fn element(&self, name: &str) -> Option<&str> {
        lookup(&self.elements, name)?.filter(|drawn| *drawn != name)
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
                let scope = self.scope(&name[..name.len() - candidate.len()]);
                return (drawn != candidate || matches!(scope, Cow::Owned(_)))
                    .then(|| format!("{scope}{drawn}"));
            }
            candidate = candidate.split_once('.')?.1;
        }
    }

    /// How the drawings spell `name`: `Some(None)` when two of them disagree.
    fn spelling(&self, name: &str) -> Option<Option<&str>> {
        lookup(&self.drawn, name)
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
        let drawn_scope = self.scope(&rendered[..scope]);
        match self.spelling(bus) {
            Some(Some(drawn)) if drawn != bus => {
                Some(format!("{drawn_scope}{drawn}{}", &rendered[base.len()..]))
            }
            _ if matches!(drawn_scope, Cow::Owned(_)) => {
                Some(format!("{drawn_scope}{}", &rendered[scope..]))
            }
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

/// How the drawings spell `name` in `spellings`: `Some(None)` when two of them
/// disagree, `None` when none spells it.
fn lookup<'m>(
    spellings: &'m HashMap<String, Option<String>>,
    name: &str,
) -> Option<Option<&'m str>> {
    if spellings.is_empty() {
        return None;
    }
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
        Some(key) => spellings.get(key),
        None => spellings.get(&name.to_ascii_uppercase()),
    }?;
    Some(entry.as_deref())
}

/// Record that a drawing spells a net, bus or element `name`, keyed by the
/// engine's spelling of it. A vector-shaped label (`data[3]`) names a bit the
/// bus's notation answers for, so only whole names are recorded.
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
