//! The parts this machine holds that the shipped corpus index cannot see.
//!
//! The shelf used to have exactly one source of rows: `browse_pack_models`,
//! which streams `SHIPPED-CATALOG.tsv` from the model tree beside the binary.
//! That file is written by `tools/models/build_manifest.py` at release time and
//! knows nothing about what happened afterwards — so a part the Model Hub
//! installed was not on the shelf, a part the project retained was not on the
//! shelf, and on a browser build, where the tree is never discovered at all,
//! the shelf reported "No addressable parts are installed" over a project that
//! held thousands of cards.
//!
//! This module supplies the other half: every addressable definition on this
//! machine that the index does not already list, with the provenance that says
//! how it is placed. The two halves are concatenated rather than interleaved
//! because the index is *streamed* — a development corpus can hold two hundred
//! thousand definitions, and materializing it to sort one page against it would
//! give back exactly what the streaming query was written to avoid.
//!
//! # Why the class vocabulary is mirrored here
//!
//! The index classifies a `.model` card by its type token, and the generator
//! owns that table. A card this project parsed never went through the
//! generator, so the same token has to be classified again — otherwise the
//! CLASS chips narrow the corpus half and silently drop the held half, which is
//! the same hiding this module exists to remove. [`DEVICE_CLASS`] is that
//! mirror, and a test reads the generator's own source and refuses to let the
//! two drift.

use std::collections::BTreeMap;

use rspice_core::library::SpiceLibraryIndex;
use rspice_pack::PartKind;

use crate::state::model_hub::InstalledPack;
use crate::state::model_library::{
    DeviceModel, ModelLibrary, ModelLibraryManager, ModelSourceAuthority, ModelType, PackModelHit,
};

/// The `.MODEL` type token to canonical device class map, mirrored from
/// `tools/models/build_manifest.py`.
///
/// Kept in the generator's own order and spelling so the drift test can compare
/// them literally.
const DEVICE_CLASS: &[(&str, &str)] = &[
    ("d", "diode"),
    ("npn", "bjt-npn"),
    ("pnp", "bjt-pnp"),
    ("lpnp", "bjt-pnp"),
    ("njf", "jfet-n"),
    ("pjf", "jfet-p"),
    ("nmos", "mosfet-n"),
    ("pmos", "mosfet-p"),
    ("nmf", "mesfet-n"),
    ("pmf", "mesfet-p"),
    ("nsw", "switch"),
    ("sw", "switch"),
    ("vswitch", "switch"),
    ("csw", "switch-current"),
    ("iswitch", "switch-current"),
    ("r", "resistor"),
    ("res", "resistor"),
    ("c", "capacitor"),
    ("cap", "capacitor"),
    ("l", "inductor"),
    ("ind", "inductor"),
    ("k", "coupling"),
    ("core", "magnetic-core"),
    ("vdmos", "mosfet-vdmos"),
    ("txl", "transmission-line"),
    ("cpl", "transmission-line"),
    ("ltra", "transmission-line"),
    ("urc", "distributed-rc"),
    ("pzt", "piezo"),
];

/// Where a shelf row's definition lives, and therefore how it is placed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PartOrigin {
    /// Compiled into this build. Always present, never installable.
    Foundation { library: String },
    /// Bytes the open project retained when the part was added.
    ProjectRetained { library: String },
    /// A verified pack release on this machine that the project has not
    /// adopted. Placing one retains its bytes first.
    InstalledPack { pack_id: String, version: String },
    /// A definition in the shipped corpus tree that no project library holds.
    /// Its bytes are on disk but not in the project, which is what
    /// "Add to project…" is for and why it is not placeable as it stands.
    Corpus,
}

/// One shelf line: a part, and where its definition lives.
///
/// The description and the specifications sit here rather than on the
/// [`PackModelHit`] because only a *signed pack manifest* has them. The hit is
/// the shipped corpus index's row type, and that index is generated from
/// source files at release time by a tool that has no such field to read — so
/// putting them on the hit would create a place every corpus row had to leave
/// empty, and a reader could not tell "this publisher said nothing" from "this
/// half of the shelf cannot say anything".
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ShelfRow {
    pub(super) hit: PackModelHit,
    pub(super) origin: PartOrigin,
    /// The publisher's one line about this part, when the row came from a
    /// signed manifest that carries one.
    pub(super) description: Option<String>,
    /// The headline specifications that manifest publishes, verbatim and with
    /// their units. Empty for every row whose definition did not come from one.
    pub(super) specs: BTreeMap<String, String>,
    /// A project library other than a built-in one holds this name.
    ///
    /// True of every retained row by construction, and true of a corpus row
    /// whose definition has already been adopted — which is the one a reader
    /// most needs to be told about, because it is the row that looks like it
    /// still needs adding and does not.
    pub(super) in_project: bool,
}

/// The library that would execute one definition name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Holder {
    pub(super) library: String,
    pub(super) built_in: bool,
}

impl Holder {
    /// The origin a row for this name carries.
    pub(super) fn origin(&self) -> PartOrigin {
        if self.built_in {
            PartOrigin::Foundation {
                library: self.library.clone(),
            }
        } else {
            PartOrigin::ProjectRetained {
                library: self.library.clone(),
            }
        }
    }
}

/// One page of what this machine holds, and the count behind it.
pub(super) struct HeldParts {
    /// Complete number of held rows matching the current query and class, not
    /// merely the length of `page`.
    pub(super) total: usize,
    pub(super) page: Vec<ShelfRow>,
    /// Which loaded library holds each definition name.
    ///
    /// The streamed half reads it to classify its own rows: a row the index
    /// reports for a pack this session has loaded is not merely a file on
    /// disk — it is a definition the project's catalog can execute, and that is
    /// the difference between a part a reader can place and one they must add
    /// first. A retained library wins over a built-in one holding the same
    /// name, because the project's own bytes are what would run.
    pub(super) holders: BTreeMap<String, Holder>,
}

/// What a row is filtered by, gathered so the two halves ask the same question.
pub(super) struct HeldQuery<'a> {
    pub(super) needle: &'a str,
    pub(super) pack: Option<&'a str>,
    pub(super) devices: &'a [&'a str],
    pub(super) offset: usize,
    pub(super) limit: usize,
}

/// Project and pack definitions the shipped index does not already list.
///
/// Counting and windowing happen in one pass: only the page is materialized,
/// so a project holding five thousand cards allocates a hundred and sixty rows
/// rather than five thousand.
pub(super) fn held_parts(
    manager: &ModelLibraryManager,
    installed: &[InstalledPack],
    query: &HeldQuery<'_>,
) -> HeldParts {
    let index = manager.spice_packs();
    let libraries = manager.libraries_sorted();
    let holders = holders(&libraries);

    let mut collector = PageCollector {
        query,
        total: 0,
        page: Vec::new(),
    };
    for library in &libraries {
        // A pack the index already carries is the index's to report: its rows
        // name the source file and the line, which a parsed library cannot.
        // Reporting both would put one part on the shelf twice and make every
        // count on the page wrong. The index's rows are classified against
        // `holders` instead, so nothing is lost by leaving them to it.
        if indexed_pack(index, library) {
            continue;
        }
        library_rows(library, &mut collector);
    }
    for pack in installed {
        for part in &pack.manifest.parts {
            // Adopted already: the project's own retained row is the one that
            // places without touching the store.
            if retained_anywhere(&libraries, &part.id) {
                continue;
            }
            collector.offer(installed_row(pack, part));
        }
    }
    HeldParts {
        total: collector.total,
        page: collector.page,
        holders,
    }
}

/// How many addressable parts this machine holds beyond the shipped index.
///
/// Counted rather than projected. The scope chip and the search hint state a
/// size, and building every row to learn a number would make the label cost a
/// page. It must agree with [`held_parts`]'s unfiltered total, which is what
/// the test beside them asserts: a chip that disagrees with the table under it
/// is worse than no chip.
pub(super) fn held_count(manager: &ModelLibraryManager, installed: &[InstalledPack]) -> usize {
    let index = manager.spice_packs();
    let libraries = manager.libraries_sorted();
    let mut total = 0_usize;
    for library in &libraries {
        if indexed_pack(index, library) {
            continue;
        }
        total = total.saturating_add(addressable_names(library).len());
    }
    for pack in installed {
        total = total.saturating_add(
            pack.manifest
                .parts
                .iter()
                .filter(|part| !retained_anywhere(&libraries, &part.id))
                .count(),
        );
    }
    total
}

/// Whether a library the project owns or retained holds this definition.
fn retained_anywhere(libraries: &[&ModelLibrary], name: &str) -> bool {
    libraries.iter().any(|library| {
        !matches!(library.source_authority, ModelSourceAuthority::BuiltIn)
            && (library.models.contains_key(name)
                || library.top_level_models.contains_key(name)
                || library.subcircuits.contains_key(name))
    })
}

/// The library that would execute each definition name this session has loaded.
fn holders(libraries: &[&ModelLibrary]) -> BTreeMap<String, Holder> {
    let mut holders: BTreeMap<String, Holder> = BTreeMap::new();
    for library in libraries {
        let built_in = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
        for name in addressable_names(library).into_keys() {
            let claims = match holders.get(&name) {
                None => true,
                // A definition the project retained outranks the compiled-in
                // card of the same name: retained bytes are what executes.
                Some(held) => held.built_in && !built_in,
            };
            if claims {
                holders.insert(
                    name,
                    Holder {
                        library: library.name.clone(),
                        built_in,
                    },
                );
            }
        }
    }
    holders
}

/// Whether the shipped index already reports this library's pack.
fn indexed_pack(index: Option<&SpiceLibraryIndex>, library: &ModelLibrary) -> bool {
    let (Some(index), Some(pack_id)) = (index, library.pack_id.as_deref()) else {
        return false;
    };
    index.pack(pack_id).is_some()
}

/// One row per addressable definition name in one library.
///
/// A name defined as both a model card and a subcircuit is one part with the
/// subcircuit's interface, which is how the resolver reads it too. Section-
/// scoped subcircuit keys carry a unit separator and are skipped: a netlist
/// cannot reference one by its bare name, so it is not a part a reader picks.
fn addressable_names(library: &ModelLibrary) -> BTreeMap<String, PartKind> {
    let mut named = BTreeMap::new();
    for name in library.models.keys().chain(library.top_level_models.keys()) {
        named.entry(name.clone()).or_insert(PartKind::Model);
    }
    for name in library.subcircuits.keys() {
        if name.contains('\u{1f}') {
            continue;
        }
        named.insert(name.clone(), PartKind::Subckt);
    }
    named
}

fn library_rows(library: &ModelLibrary, collector: &mut PageCollector<'_, '_>) {
    let built_in = matches!(library.source_authority, ModelSourceAuthority::BuiltIn);
    for (name, kind) in addressable_names(library) {
        let card = library
            .models
            .get(&name)
            .or_else(|| library.top_level_models.get(&name));
        let device = match kind {
            PartKind::Subckt => "subckt".to_owned(),
            PartKind::Model => card.map_or_else(|| "unknown".to_owned(), card_device),
        };
        let hit = PackModelHit {
            kind: kind_word(kind).to_owned(),
            device,
            // The library is the row's own address: two libraries may hold the
            // same definition name, and the pack column is what tells them
            // apart. A pack identity, where there is one, is carried by the
            // origin instead — that is what "Show pack" needs.
            pack: library.name.clone(),
            pack_name: if library.pdk_name.trim().is_empty() {
                library.name.clone()
            } else {
                library.pdk_name.clone()
            },
            source: library.root_path.clone(),
            line: card.and_then(|card| card.source_line).unwrap_or(0),
            // Bytes already on this machine and already in a project's
            // closure: there is no redistribution question left to answer.
            redistributable: true,
            restricted: false,
            name,
        };
        collector.offer(ShelfRow {
            origin: if built_in {
                PartOrigin::Foundation {
                    library: library.name.clone(),
                }
            } else {
                PartOrigin::ProjectRetained {
                    library: library.name.clone(),
                }
            },
            in_project: !built_in,
            // A parsed card carries no authored catalog presentation. Deriving
            // one from its parameters would be this workspace publishing
            // specifications nobody signed, which is the whole reason the
            // fields are optional in the format.
            description: None,
            specs: BTreeMap::new(),
            hit,
        });
    }
}

fn installed_row(pack: &InstalledPack, part: &rspice_pack::Part) -> ShelfRow {
    ShelfRow {
        hit: PackModelHit {
            name: part.id.clone(),
            kind: kind_word(part.kind).to_owned(),
            device: part.device.clone(),
            pack: pack.pack_id().to_owned(),
            pack_name: pack.manifest.pack.name.clone(),
            // The expanded copy is the store's, not a path this workspace
            // reads: the retention that places one re-proves the archive.
            source: None,
            line: part.source.line as usize,
            redistributable: true,
            restricted: false,
        },
        origin: PartOrigin::InstalledPack {
            pack_id: pack.pack_id().to_owned(),
            version: pack.version().to_owned(),
        },
        in_project: false,
        // The one half of the shelf whose rows come from a signed manifest,
        // and therefore the one that can answer a spec column at all.
        description: part.description.clone(),
        specs: part.specs.clone(),
    }
}

const fn kind_word(kind: PartKind) -> &'static str {
    match kind {
        PartKind::Model => "model",
        PartKind::Subckt => "subckt",
    }
}

/// Counts every match and keeps only the window this page shows.
struct PageCollector<'a, 'q> {
    query: &'a HeldQuery<'q>,
    total: usize,
    page: Vec<ShelfRow>,
}

impl PageCollector<'_, '_> {
    fn offer(&mut self, row: ShelfRow) {
        if !self.query.admits(&row.hit) {
            return;
        }
        if self.total >= self.query.offset && self.page.len() < self.query.limit {
            self.page.push(row);
        }
        self.total = self.total.saturating_add(1);
    }
}

impl HeldQuery<'_> {
    fn admits(&self, hit: &PackModelHit) -> bool {
        if self
            .pack
            .is_some_and(|pack| !hit.pack.eq_ignore_ascii_case(pack))
        {
            return false;
        }
        if !self.devices.is_empty()
            && !self
                .devices
                .iter()
                .any(|device| hit.device.eq_ignore_ascii_case(device))
        {
            return false;
        }
        if self.needle.is_empty() {
            return true;
        }
        // The same fields the streamed index searches: what the part is called,
        // what class it is, what kind of definition it is, and where it came
        // from. Matching fewer would make one search box mean two things.
        [
            hit.name.as_str(),
            hit.device.as_str(),
            hit.kind.as_str(),
            hit.pack.as_str(),
            hit.pack_name.as_str(),
        ]
        .iter()
        .any(|field| field.to_ascii_lowercase().contains(self.needle))
    }
}

/// The catalog's device class for one parsed card.
///
/// The declared type token is the generator's input, so it is this function's
/// input too. A card that carries no token at all — a synthesized or partially
/// migrated one — is classified from the type parsing settled on instead, which
/// names the same families in a different spelling.
fn card_device(card: &DeviceModel) -> String {
    let token = card
        .spice_type
        .as_deref()
        .map(str::trim)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if token.is_empty() {
        return parsed_type_device(card.model_type).to_owned();
    }
    device_class(&token)
}

/// The generator's classification of one `.MODEL` type token.
fn device_class(token: &str) -> String {
    if let Some((_, device)) = DEVICE_CLASS
        .iter()
        .find(|(candidate, _)| *candidate == token)
    {
        return (*device).to_owned();
    }
    // PSpice digital primitives and the XSPICE behavioural family are folded
    // into two classes rather than enumerated, exactly as the generator does.
    if token.starts_with("d_") {
        return "digital".to_owned();
    }
    if token.starts_with('u') {
        return "digital-behavioral".to_owned();
    }
    if token.is_empty() {
        return "unknown".to_owned();
    }
    token.to_owned()
}

/// The class a card whose token was lost still belongs to.
///
/// [`ModelType`] is what parsing concluded from the same token, so this names
/// the same families the generator's table does. The variants that carry no
/// device family — `Rf`, `Other` — answer `unknown` rather than inventing one,
/// which keeps them off every class chip but on the shelf.
const fn parsed_type_device(model_type: ModelType) -> &'static str {
    match model_type {
        ModelType::Nmos => "mosfet-n",
        ModelType::Pmos => "mosfet-p",
        ModelType::Npn => "bjt-npn",
        ModelType::Pnp => "bjt-pnp",
        ModelType::Njfet => "jfet-n",
        ModelType::Pjfet => "jfet-p",
        ModelType::Nmesfet => "mesfet-n",
        ModelType::Pmesfet => "mesfet-p",
        // One class for both polarities, because the generator's table has
        // one. Splitting it here would invent a class no shipped index row
        // carries, and the chip built from it would find nothing.
        ModelType::NVdmos | ModelType::PVdmos => "mosfet-vdmos",
        ModelType::Resistor => "resistor",
        ModelType::Capacitor => "capacitor",
        ModelType::Inductor => "inductor",
        ModelType::Diode | ModelType::Varactor | ModelType::Esd => "diode",
        ModelType::Rf | ModelType::Other => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The mirror and the generator classify the same tokens the same way.
    ///
    /// A shelf whose class chips disagree with the shipped index would put a
    /// project's diodes and the library's diodes under different chips, which
    /// is worse than having no chips at all. The generator is Python and cannot
    /// be called from here, so its table is read instead — a mirror nobody
    /// checks is a mirror that has already drifted.
    #[test]
    fn the_device_class_mirror_matches_the_generator() {
        const GENERATOR: &str = include_str!("../../../../../../../tools/models/build_manifest.py");

        let table = GENERATOR
            .split_once("DEVICE_CLASS = {")
            .expect("the generator declares DEVICE_CLASS")
            .1
            .split_once('}')
            .expect("the DEVICE_CLASS literal is closed")
            .0;
        let declared = table
            .lines()
            .filter_map(|line| line.trim().strip_suffix(','))
            .filter_map(|entry| entry.split_once(':'))
            .map(|(token, device)| (unquote(token), unquote(device)))
            .collect::<Vec<_>>();

        assert!(
            declared.len() >= 25,
            "only {} entries were read out of the generator; the literal's shape has changed",
            declared.len()
        );
        assert_eq!(
            declared,
            DEVICE_CLASS
                .iter()
                .map(|(token, device)| ((*token).to_owned(), (*device).to_owned()))
                .collect::<Vec<_>>(),
            "DEVICE_CLASS here no longer matches tools/models/build_manifest.py"
        );
    }

    fn unquote(value: &str) -> String {
        value.trim().trim_matches('"').trim_matches('\'').to_owned()
    }

    #[test]
    fn a_parsed_card_lands_on_the_class_its_token_names() {
        let card = |spice_type: Option<&str>, model_type| {
            let mut card = DeviceModel::new("X", model_type);
            card.spice_type = spice_type.map(str::to_owned);
            card
        };
        assert_eq!(card_device(&card(Some("NJF"), ModelType::Other)), "jfet-n");
        assert_eq!(card_device(&card(Some("D"), ModelType::Diode)), "diode");
        assert_eq!(
            card_device(&card(Some("d_and"), ModelType::Other)),
            "digital"
        );
        // A token nothing classifies stands for itself rather than becoming a
        // class the card never claimed.
        assert_eq!(
            card_device(&card(Some("psp103"), ModelType::Other)),
            "psp103"
        );
        // And a card that lost its token is still filed under its family.
        assert_eq!(card_device(&card(None, ModelType::Nmos)), "mosfet-n");
        assert_eq!(card_device(&card(None, ModelType::Other)), "unknown");
    }
}
