//! Discovery and lookup across the shipped SPICE model packs.
//!
//! The built-in library ([`super::LibraryManager`]) is compiled into the binary
//! and is deliberately small. The bulk of the model tree — 142,333 model cards
//! and 56,601 subcircuits across the foundry PDKs, academic sets and vendor
//! libraries — ships as data beside the binary and is found at runtime through
//! this module.
//!
//! Only 68,676 of those are addressable from a netlist. The remainder are
//! helper cards declared inside macromodel bodies, which the catalog records
//! with `scope=nested` and the lookups here exclude by default. Quoting the raw
//! definition total as a part count overstates it roughly threefold.
//!
//! Two generated indexes back it, both produced by
//! `tools/models/build_manifest.py` and both tab-separated so that no TOML
//! parser is needed here:
//!
//! * `PACKS.tsv` — one row per pack: identity, licence tier, redistribution
//!   flag and counts. Small, always loaded.
//! * `CATALOG.tsv` — one row per definition. Around 16 MB, so it is streamed
//!   for lookups rather than held in memory; [`SpiceLibraryIndex::load_catalog`]
//!   is available when a caller genuinely wants the whole list.
//!
//! Parts are addressed by pack, not by a flat global namespace. That is
//! deliberate: part names routinely occur in more than one pack, often with
//! materially different parameter fits, so a lookup returns every match and
//! leaves the choice to the caller rather than silently picking one.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Environment variable that pins the model root explicitly.
pub const MODELS_DIR_ENV: &str = "RSPICE_MODELS_DIR";

const PACKS_INDEX: &str = "PACKS.tsv";
const CATALOG_INDEX: &str = "CATALOG.tsv";

/// How a pack's licence bears on redistribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LicenseTier {
    /// Apache-2.0, BSD, MIT, CC-BY and equivalents.
    Permissive,
    /// GPL/LGPL family; not embeddable in a proprietary binary.
    Copyleft,
    /// No explicit grant either way.
    Ambiguous,
    /// Authored by the RSpice project.
    Own,
    /// Tier string that this build does not recognise.
    Unknown,
}

impl LicenseTier {
    fn parse(value: &str) -> Self {
        match value {
            "permissive" => LicenseTier::Permissive,
            "copyleft" => LicenseTier::Copyleft,
            "ambiguous" => LicenseTier::Ambiguous,
            "own" => LicenseTier::Own,
            _ => LicenseTier::Unknown,
        }
    }

    /// Display name for a parts browser.
    pub fn display_name(&self) -> &'static str {
        match self {
            LicenseTier::Permissive => "Permissive",
            LicenseTier::Copyleft => "Copyleft",
            LicenseTier::Ambiguous => "Unestablished",
            LicenseTier::Own => "RSpice",
            LicenseTier::Unknown => "Unknown",
        }
    }
}

/// One model pack discovered under the model root.
#[derive(Debug, Clone)]
pub struct SpicePack {
    /// Stable identifier, such as `sky130` or `interfet-jfet`.
    pub id: String,
    /// Human-readable title.
    pub name: String,
    /// Source grouping: `foundry`, `academic`, `community`, `vendor`, `builtin`.
    pub category: String,
    /// Pack directory relative to the model root.
    pub path: PathBuf,
    /// Licence tier governing whether this pack may be shipped.
    pub tier: LicenseTier,
    /// SPDX identifier, or `NOASSERTION`.
    pub spdx: String,
    /// Whether RSpice has established the right to redistribute this pack.
    pub redistributable: bool,
    /// Suggested entry file relative to the pack directory, if the pack has one.
    pub entry: Option<PathBuf>,
    /// Device classes the pack covers.
    pub devices: Vec<String>,
    /// Count of `.model` cards, at any nesting depth.
    pub models: usize,
    /// Count of `.subckt` definitions, at any nesting depth.
    pub subcircuits: usize,
    /// Count of `.model` cards at file scope.
    pub models_top: usize,
    /// Count of `.subckt` definitions at file scope.
    pub subcircuits_top: usize,
    /// Count of files.
    pub files: usize,
    /// Total bytes on disk.
    pub bytes: u64,
}

impl SpicePack {
    /// Absolute path to this pack's entry file, when it declares one.
    pub fn entry_path(&self, root: &Path) -> Option<PathBuf> {
        self.entry
            .as_ref()
            .map(|entry| root.join(&self.path).join(entry))
    }
}

/// Where a definition sits in its source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionScope {
    /// Declared at file scope, so a netlist can reference it by name.
    TopLevel,
    /// Declared inside a `.SUBCKT` body and private to it.
    ///
    /// Macromodels routinely declare helper cards this way — `.model DX D(...)`
    /// inside an op-amp subcircuit is the standard idiom, and `DX` alone occurs
    /// over six thousand times across the shipped packs. Such a name cannot be
    /// referenced from a netlist, so a parts browser must not offer it.
    Nested,
}

impl DefinitionScope {
    fn parse(value: &str) -> Self {
        match value {
            "nested" => DefinitionScope::Nested,
            _ => DefinitionScope::TopLevel,
        }
    }

    /// Whether a netlist can reference this definition by name.
    pub fn is_addressable(&self) -> bool {
        matches!(self, DefinitionScope::TopLevel)
    }
}

/// One definition located in the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogEntry {
    /// Definition name as written in the source.
    pub name: String,
    /// `model` or `subckt`.
    pub kind: String,
    /// Canonical device class, such as `diode` or `mosfet-n`.
    pub device: String,
    /// Owning pack identifier.
    pub pack: String,
    /// Source file relative to the pack directory.
    pub path: PathBuf,
    /// 1-based line number of the definition.
    pub line: usize,
    /// The defining file is marked non-shippable by the licence audit.
    ///
    /// This is per *file*, not per pack: upstream `microcap-library` holds 153
    /// clean files alongside 28 restricted ones, so the pack flag is too blunt
    /// to decide whether one card may be shipped.
    pub restricted: bool,
    /// Whether the definition is addressable from a netlist or private to an
    /// enclosing subcircuit.
    pub scope: DefinitionScope,
}

impl CatalogEntry {
    /// Absolute path to the file holding this definition.
    pub fn source_path(&self, index: &SpiceLibraryIndex) -> Option<PathBuf> {
        index
            .pack(&self.pack)
            .map(|pack| index.root().join(&pack.path).join(&self.path))
    }
}

/// The shipped model tree: its packs and its part catalog.
#[derive(Debug, Clone)]
pub struct SpiceLibraryIndex {
    root: PathBuf,
    packs: Vec<SpicePack>,
}

impl SpiceLibraryIndex {
    /// Locate and open the model tree, or return `None` when it is absent.
    ///
    /// Resolution order:
    ///
    /// 1. `RSPICE_MODELS_DIR`, which wins outright so an installation or a test
    ///    can point at a specific tree.
    /// 2. `models/spice` beside the executable, which is the installed layout.
    /// 3. `models/spice` in an ancestor of the executable, which covers running
    ///    straight out of `target/debug` during development.
    ///
    /// A missing tree is not an error: the built-in library is compiled in, so
    /// RSpice runs without the packs. Only an unreadable index is an error.
    pub fn discover() -> io::Result<Option<Self>> {
        // The browser build has no filesystem to discover a tree on, and
        // `current_exe` is unsupported there. Answer "no tree" up front rather
        // than probing paths that cannot exist.
        if cfg!(target_arch = "wasm32") {
            return Ok(None);
        }
        for candidate in Self::candidate_roots() {
            if candidate.join(PACKS_INDEX).is_file() {
                return Ok(Some(Self::open(candidate)?));
            }
        }
        Ok(None)
    }

    fn candidate_roots() -> Vec<PathBuf> {
        let mut roots = Vec::new();

        if let Some(explicit) = std::env::var_os(MODELS_DIR_ENV) {
            roots.push(PathBuf::from(explicit));
        }

        if let Ok(exe) = std::env::current_exe() {
            let mut dir = exe.parent().map(Path::to_path_buf);
            // Walk up a bounded number of levels: `models/spice` sits beside an
            // installed binary, and a few levels above one in `target/<profile>`.
            for _ in 0..5 {
                let Some(current) = dir else { break };
                roots.push(current.join("models").join("spice"));
                dir = current.parent().map(Path::to_path_buf);
            }
        }

        roots
    }

    /// Open a model tree at a known root.
    pub fn open(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        let packs = parse_packs(&fs::read_to_string(root.join(PACKS_INDEX))?);
        Ok(Self { root, packs })
    }

    /// Root directory of the model tree.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Every discovered pack, ordered as the index lists them.
    pub fn packs(&self) -> &[SpicePack] {
        &self.packs
    }

    /// Look up a pack by identifier.
    pub fn pack(&self, id: &str) -> Option<&SpicePack> {
        self.packs.iter().find(|pack| pack.id == id)
    }

    /// Packs RSpice has established the right to redistribute.
    pub fn redistributable_packs(&self) -> impl Iterator<Item = &SpicePack> {
        self.packs.iter().filter(|pack| pack.redistributable)
    }

    /// Total `.model` and `.subckt` definitions across all packs, including
    /// those private to a subcircuit body.
    pub fn definition_count(&self) -> usize {
        self.packs
            .iter()
            .map(|pack| pack.models + pack.subcircuits)
            .sum()
    }

    /// Definitions a netlist can actually reference by name.
    ///
    /// Two thirds of the shipped definitions are helper cards inside macromodel
    /// bodies, so this is the figure to show a user as a part count;
    /// [`Self::definition_count`] overstates it by roughly a factor of three.
    pub fn part_count(&self) -> usize {
        self.packs
            .iter()
            .map(|pack| pack.models_top + pack.subcircuits_top)
            .sum()
    }

    /// Every addressable definition matching `name`, compared case-insensitively.
    ///
    /// A part number routinely appears in several packs with different
    /// parameter fits, so this returns all of them rather than choosing.
    /// Subcircuit-private cards are excluded; use
    /// [`Self::find_definition_any_scope`] when tracing one to its source.
    pub fn find_part(&self, name: &str) -> io::Result<Vec<CatalogEntry>> {
        self.filter_catalog(|entry| {
            entry.scope.is_addressable() && entry.name.eq_ignore_ascii_case(name)
        })
    }

    /// Every definition matching `name` at any nesting depth.
    ///
    /// For diagnostics: when a deck fails on a card declared inside a
    /// macromodel, the name is not addressable but still needs locating.
    pub fn find_definition_any_scope(&self, name: &str) -> io::Result<Vec<CatalogEntry>> {
        self.filter_catalog(|entry| entry.name.eq_ignore_ascii_case(name))
    }

    /// Addressable definitions of `name` whose defining file may be shipped.
    ///
    /// Narrower than [`Self::find_part`]: a pack can be usable while individual
    /// files inside it are not.
    pub fn find_shippable_part(&self, name: &str) -> io::Result<Vec<CatalogEntry>> {
        self.filter_catalog(|entry| {
            entry.scope.is_addressable()
                && !entry.restricted
                && entry.name.eq_ignore_ascii_case(name)
        })
    }

    /// Every addressable definition whose name contains `needle`,
    /// compared case-insensitively.
    pub fn search_parts(&self, needle: &str, limit: usize) -> io::Result<Vec<CatalogEntry>> {
        let needle = needle.to_ascii_uppercase();
        let mut found = Vec::new();
        self.for_each_catalog_entry(|entry| {
            if found.len() >= limit {
                return false;
            }
            if entry.scope.is_addressable() && entry.name.to_ascii_uppercase().contains(&needle) {
                found.push(entry);
            }
            true
        })?;
        Ok(found)
    }

    /// A bounded, deterministic first page of addressable definitions.
    ///
    /// Catalog UIs use this when no query or class facet is active. Streaming
    /// stops at `limit`, so opening a parts browser never materializes the
    /// complete multi-megabyte index.
    pub fn browse_parts(&self, limit: usize) -> io::Result<Vec<CatalogEntry>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        self.for_each_catalog_entry(|entry| {
            if entry.scope.is_addressable() {
                found.push(entry);
            }
            found.len() < limit
        })?;
        Ok(found)
    }

    /// Every addressable definition of a given canonical device class.
    pub fn parts_by_device(&self, device: &str, limit: usize) -> io::Result<Vec<CatalogEntry>> {
        let mut found = Vec::new();
        self.for_each_catalog_entry(|entry| {
            if found.len() >= limit {
                return false;
            }
            if entry.scope.is_addressable() && entry.device == device {
                found.push(entry);
            }
            true
        })?;
        Ok(found)
    }

    /// The whole catalog, nested definitions included. Around 199,000 entries;
    /// prefer the search methods.
    pub fn load_catalog(&self) -> io::Result<Vec<CatalogEntry>> {
        self.filter_catalog(|_| true)
    }

    fn filter_catalog(
        &self,
        mut keep: impl FnMut(&CatalogEntry) -> bool,
    ) -> io::Result<Vec<CatalogEntry>> {
        let mut found = Vec::new();
        self.for_each_catalog_entry(|entry| {
            if keep(&entry) {
                found.push(entry);
            }
            true
        })?;
        Ok(found)
    }

    /// Stream the catalog, stopping early when `visit` returns `false`.
    ///
    /// The catalog is around 16 MB. Streaming keeps a part lookup from paying
    /// the allocation cost of materialising nearly 200,000 entries.
    fn for_each_catalog_entry(
        &self,
        mut visit: impl FnMut(CatalogEntry) -> bool,
    ) -> io::Result<()> {
        let path = self.root.join(CATALOG_INDEX);
        let text = fs::read_to_string(path)?;
        for line in text.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let Some(entry) = parse_catalog_row(line) else {
                continue;
            };
            if !visit(entry) {
                break;
            }
        }
        Ok(())
    }
}

/// Build a path from an index field.
///
/// The generated indexes always use forward slashes. Rebuilding component by
/// component means a later `join` produces native separators throughout,
/// instead of a path that mixes them and reads as broken in diagnostics.
fn index_path(value: &str) -> PathBuf {
    value.split('/').filter(|part| !part.is_empty()).collect()
}

fn parse_catalog_row(line: &str) -> Option<CatalogEntry> {
    let mut fields = line.split('\t');
    let name = fields.next()?;
    let kind = fields.next()?;
    let device = fields.next()?;
    let pack = fields.next()?;
    let path = fields.next()?;
    let line_number = fields.next()?.parse().ok()?;
    // Older catalogs predate the restriction column; absent means unflagged.
    let restricted = fields.next().is_some_and(|value| value == "1");
    // Likewise for scope: a catalog without the column predates nesting being
    // tracked, and treating those rows as addressable preserves the old
    // behaviour rather than silently emptying a browser.
    let scope = fields
        .next()
        .map_or(DefinitionScope::TopLevel, DefinitionScope::parse);
    Some(CatalogEntry {
        name: name.to_string(),
        kind: kind.to_string(),
        device: device.to_string(),
        pack: pack.to_string(),
        path: index_path(path),
        line: line_number,
        restricted,
        scope,
    })
}

fn parse_packs(text: &str) -> Vec<SpicePack> {
    let mut packs = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 15 {
            continue;
        }
        let entry = fields[6].trim();
        packs.push(SpicePack {
            id: fields[0].to_string(),
            category: fields[1].to_string(),
            path: index_path(fields[2]),
            tier: LicenseTier::parse(fields[3]),
            spdx: fields[4].to_string(),
            redistributable: fields[5] == "1",
            entry: (!entry.is_empty()).then(|| index_path(entry)),
            models: fields[7].parse().unwrap_or(0),
            subcircuits: fields[8].parse().unwrap_or(0),
            models_top: fields[9].parse().unwrap_or(0),
            subcircuits_top: fields[10].parse().unwrap_or(0),
            files: fields[11].parse().unwrap_or(0),
            bytes: fields[12].parse().unwrap_or(0),
            devices: fields[13]
                .split(',')
                .filter(|d| !d.is_empty())
                .map(str::to_string)
                .collect(),
            name: fields[14].to_string(),
        });
    }
    packs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_models_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../models/spice")
            .canonicalize()
            .expect("repository model tree present")
    }

    #[test]
    fn packs_index_parses() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        assert!(
            index.packs().len() >= 15,
            "expected the shipped packs, found {}",
            index.packs().len()
        );

        let sky130 = index.pack("sky130").expect("sky130 pack discovered");
        assert_eq!(sky130.category, "foundry");
        assert_eq!(sky130.tier, LicenseTier::Permissive);
        assert!(sky130.redistributable);
        assert!(
            sky130.models > 1000,
            "sky130 carries {} models",
            sky130.models
        );
        assert!(sky130.entry.is_some(), "sky130 declares an entry deck");
    }

    #[test]
    fn definition_count_spans_the_whole_tree() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        assert!(
            index.definition_count() > 190_000,
            "expected the full tree, counted {}",
            index.definition_count()
        );
    }

    #[test]
    fn macromodel_internals_are_not_offered_as_parts() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");

        // `DX` is the canonical helper-diode name inside op-amp macromodels,
        // declared thousands of times across these corpora and referenceable
        // from almost none of them. This is the regression that made the
        // browser unusable: every diode search surfaced the same private card
        // over and over. Two packs do declare a `dx` at file scope, so the
        // addressable result is small rather than empty — the point is that it
        // collapses by three orders of magnitude.
        let as_part = index.find_part("DX").expect("catalog readable");
        let anywhere = index
            .find_definition_any_scope("DX")
            .expect("catalog readable");

        assert!(
            anywhere.len() > 1000,
            "expected DX throughout the macromodels, found {}",
            anywhere.len()
        );
        assert!(
            as_part.len() < 10,
            "DX is macromodel-private nearly everywhere, got {} part hits",
            as_part.len()
        );
        assert!(
            as_part
                .iter()
                .all(|entry| entry.scope == DefinitionScope::TopLevel),
            "a part hit must be addressable"
        );
        assert!(
            anywhere
                .iter()
                .filter(|entry| entry.scope == DefinitionScope::Nested)
                .count()
                > 1000,
            "the bulk of DX cards should be nested"
        );
    }

    #[test]
    fn addressable_parts_are_a_fraction_of_all_definitions() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let parts = index.part_count();
        let all = index.definition_count();
        assert!(parts > 60_000, "expected a real part count, got {parts}");
        assert!(
            parts * 2 < all,
            "most definitions are macromodel internals: {parts} of {all}"
        );
    }

    #[test]
    fn part_lookup_returns_every_pack_that_defines_it() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let matches = index.find_part("1N4148").expect("catalog readable");
        assert!(
            matches.len() > 1,
            "1N4148 should resolve in several packs, got {matches:?}"
        );
        // The point of returning all of them: the same part number carries
        // different fits in different packs, and the caller must choose.
        let packs: std::collections::BTreeSet<_> =
            matches.iter().map(|m| m.pack.as_str()).collect();
        assert!(packs.len() > 1, "expected several packs, got {packs:?}");
    }

    #[test]
    fn entry_and_source_paths_resolve_on_disk() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");

        let sky130 = index.pack("sky130").expect("sky130 pack");
        let entry = sky130.entry_path(index.root()).expect("sky130 entry path");
        assert!(entry.is_file(), "sky130 entry deck missing at {entry:?}");

        let jfet = index
            .find_part("2N3819")
            .expect("catalog readable")
            .into_iter()
            .next()
            .expect("2N3819 present");
        let source = jfet.source_path(&index).expect("owning pack known");
        assert!(
            source.is_file(),
            "catalog path does not resolve: {source:?}"
        );
    }

    #[test]
    fn nothing_vendored_is_restricted() {
        // Restricted files are dropped at the vendoring boundary by
        // tools/models/sync_packs.py, because this repository is public and
        // committing such a file would itself be the redistribution its terms
        // forbid. The catalog still carries the column so a consumer can rely
        // on it, but in a correctly synced tree nothing is flagged.
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let flagged: Vec<_> = index
            .load_catalog()
            .expect("catalog readable")
            .into_iter()
            .filter(|entry| entry.restricted)
            .take(5)
            .collect();
        assert!(
            flagged.is_empty(),
            "restricted material reached the repository: {flagged:?}"
        );

        // The shippable view must therefore agree with the unfiltered one.
        let all = index.find_part("1N4148").expect("catalog readable");
        let shippable = index
            .find_shippable_part("1N4148")
            .expect("catalog readable");
        assert!(!all.is_empty());
        assert_eq!(all.len(), shippable.len());
    }

    #[test]
    fn device_class_query_finds_foundry_devices() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let jfets = index
            .parts_by_device("jfet-n", 50)
            .expect("catalog readable");
        assert!(!jfets.is_empty(), "expected N-JFETs in the catalog");
        assert!(jfets.iter().all(|entry| entry.device == "jfet-n"));
    }

    #[test]
    fn browse_parts_is_bounded_and_only_returns_addressable_cards() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let parts = index.browse_parts(25).expect("catalog readable");
        assert_eq!(parts.len(), 25);
        assert!(
            parts
                .iter()
                .all(|entry| entry.scope == DefinitionScope::TopLevel)
        );
        assert!(index.browse_parts(0).expect("catalog readable").is_empty());
    }

    #[test]
    fn redistributable_packs_exclude_unestablished_ones() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let shippable: Vec<_> = index
            .redistributable_packs()
            .map(|p| p.id.clone())
            .collect();
        assert!(shippable.contains(&"sky130".to_string()));
        assert!(
            !shippable.contains(&"microcap-library".to_string()),
            "microcap-library has no established redistribution grant"
        );
    }
}
