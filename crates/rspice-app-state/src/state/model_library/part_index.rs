//! The parts the loaded model libraries hold: one row per definition a
//! component can name.
//!
//! The Design shelf lists its library parts from this. It is read from the
//! libraries alone, so a compiled-in foundation card and a definition the
//! project holds are the same kind of row, and each names the library that
//! holds it.

use std::collections::BTreeMap;

use super::{ModelLibrary, ModelSourceAuthority, SUBCIRCUIT_CLASS, card_device};

/// Whether a part is a `.model` card or a `.subckt` definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibraryPartKind {
    Model,
    Subcircuit,
}

/// One definition a loaded model library holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryPart {
    /// The definition's name, as a component names it.
    pub part_id: String,
    pub kind: LibraryPartKind,
    /// What class of device the part is: `diode`, `mosfet-n`, `subckt`.
    pub device: String,
    /// The library that holds the definition.
    pub library: String,
    /// Whether that library is compiled into this build.
    pub built_in: bool,
}

/// Every definition `libraries` hold, ordered by name.
///
/// A name defined as both a model card and a subcircuit is one part with the
/// subcircuit's interface, which is also how the resolver reads it. A
/// section-scoped subcircuit key carries a unit separator: a netlist cannot
/// name it by itself, so it is not a part.
#[must_use]
pub fn library_part_index(libraries: &[&ModelLibrary]) -> Vec<LibraryPart> {
    let mut parts = Vec::new();
    for library in libraries {
        let built_in = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
        let mut named = BTreeMap::new();
        for name in library.models.keys().chain(library.top_level_models.keys()) {
            named.entry(name.clone()).or_insert(LibraryPartKind::Model);
        }
        for name in library.subcircuits.keys() {
            named.insert(name.clone(), LibraryPartKind::Subcircuit);
        }
        for (part_id, kind) in named {
            if part_id.contains('\u{1f}') {
                continue;
            }
            let device = match kind {
                LibraryPartKind::Subcircuit => SUBCIRCUIT_CLASS.to_owned(),
                LibraryPartKind::Model => library
                    .models
                    .get(&part_id)
                    .or_else(|| library.top_level_models.get(&part_id))
                    .map_or_else(|| "unknown".to_owned(), card_device),
            };
            parts.push(LibraryPart {
                part_id,
                kind,
                device,
                library: library.name.clone(),
                built_in,
            });
        }
    }
    parts.sort_by(|left, right| {
        left.part_id
            .cmp(&right.part_id)
            .then_with(|| right.built_in.cmp(&left.built_in))
            .then_with(|| left.library.cmp(&right.library))
    });
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::model_library::{DeviceModel, ModelType};

    #[test]
    fn each_definition_is_one_part_named_by_the_library_that_holds_it() {
        let mut foundation = ModelLibrary::new("foundation");
        let mut npn = DeviceModel::new("Q_NPN", ModelType::Npn);
        npn.spice_type = Some("NPN".to_owned());
        foundation.add_model(npn);
        let mut project = ModelLibrary::new("vendor");
        project.source_authority = ModelSourceAuthority::External;
        let mut diode = DeviceModel::new("D_FAST", ModelType::Diode);
        diode.spice_type = Some("D".to_owned());
        project.add_model(diode);

        let parts = library_part_index(&[&project, &foundation]);
        assert_eq!(
            parts
                .iter()
                .map(|part| (
                    part.part_id.as_str(),
                    part.library.as_str(),
                    part.built_in,
                    part.device.as_str()
                ))
                .collect::<Vec<_>>(),
            [
                ("D_FAST", "vendor", false, "diode"),
                ("Q_NPN", "foundation", true, "bjt-npn"),
            ]
        );
    }
}
