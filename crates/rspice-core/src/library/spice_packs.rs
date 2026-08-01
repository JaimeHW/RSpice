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

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

/// Environment variable that pins the model root explicitly.
pub const MODELS_DIR_ENV: &str = "RSPICE_MODELS_DIR";

const PACKS_INDEX: &str = "PACKS.tsv";
const CATALOG_INDEX: &str = "CATALOG.tsv";
const MAX_DEFINITION_PREVIEW_FILE_BYTES: u64 = 32 * 1024 * 1024;
const MAX_DEFINITION_PREVIEW_LINES: usize = 240;

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
        index.catalog_source_path(self)
    }
}

/// Bounded, line-attributed source preview for one indexed definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogDefinitionPreview {
    pub source_path: PathBuf,
    pub start_line: usize,
    pub source: String,
    pub truncated: bool,
}

/// Exact ordered terminal contract for one addressable `.subckt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSubcircuitInterface {
    pub name: String,
    pub ports: Vec<String>,
    pub source_path: PathBuf,
    pub start_line: usize,
}

/// The shipped model tree: its packs and its part catalog.
#[derive(Debug, Clone)]
pub struct SpiceLibraryIndex {
    root: PathBuf,
    packs: Vec<SpicePack>,
    addressable_catalog: Arc<OnceLock<Result<Arc<AddressableCatalog>, String>>>,
    #[cfg(target_arch = "wasm32")]
    embedded_catalog: Option<&'static str>,
}

#[derive(Debug)]
struct AddressableCatalog {
    entries: Vec<IndexedCatalogEntry>,
    device_counts: BTreeMap<String, usize>,
}

#[derive(Debug)]
struct IndexedCatalogEntry {
    entry: CatalogEntry,
    search_text: String,
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
        #[cfg(target_arch = "wasm32")]
        {
            const PACKS: &str = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../models/spice/PACKS.tsv"
            ));
            const CATALOG: &str = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../models/spice/CATALOG.tsv"
            ));
            Ok(Some(Self::from_embedded_indexes(PACKS, CATALOG)))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            for candidate in Self::candidate_roots() {
                if candidate.join(PACKS_INDEX).is_file() {
                    return Ok(Some(Self::open(candidate)?));
                }
            }
            Ok(None)
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
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
        Ok(Self {
            root,
            packs,
            addressable_catalog: Arc::new(OnceLock::new()),
            #[cfg(target_arch = "wasm32")]
            embedded_catalog: None,
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn from_embedded_indexes(packs: &'static str, catalog: &'static str) -> Self {
        Self {
            root: PathBuf::from("/rspice-browser/model-catalog"),
            packs: parse_packs(packs),
            addressable_catalog: Arc::new(OnceLock::new()),
            embedded_catalog: Some(catalog),
        }
    }

    /// Root directory of the model tree.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Every discovered pack, ordered as the index lists them.
    pub fn packs(&self) -> &[SpicePack] {
        &self.packs
    }

    /// Whether indexed definition source files are locally available.
    ///
    /// The browser embeds the redistributable metadata index for discovery,
    /// but never pretends that the roughly 200 MB source corpus is present.
    /// A user must import an authenticated source bundle before execution.
    pub fn source_files_available(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            self.embedded_catalog.is_none()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            true
        }
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

    /// Top-level `.model` cards addressable from a device instance.
    pub fn model_count(&self) -> usize {
        self.packs.iter().map(|pack| pack.models_top).sum()
    }

    /// Every addressable definition matching `name`, compared case-insensitively.
    ///
    /// A part number routinely appears in several packs with different
    /// parameter fits, so this returns all of them rather than choosing.
    /// Subcircuit-private cards are excluded; use
    /// [`Self::find_definition_any_scope`] when tracing one to its source.
    pub fn find_part(&self, name: &str) -> io::Result<Vec<CatalogEntry>> {
        Ok(self
            .addressable_catalog()?
            .entries
            .iter()
            .filter(|entry| entry.entry.name.eq_ignore_ascii_case(name))
            .map(|entry| entry.entry.clone())
            .collect())
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
        Ok(self
            .addressable_catalog()?
            .entries
            .iter()
            .filter(|entry| !entry.entry.restricted && entry.entry.name.eq_ignore_ascii_case(name))
            .map(|entry| entry.entry.clone())
            .collect())
    }

    /// Every addressable definition whose name contains `needle`,
    /// compared case-insensitively.
    pub fn search_parts(&self, needle: &str, limit: usize) -> io::Result<Vec<CatalogEntry>> {
        let needle = needle.to_ascii_lowercase();
        Ok(self
            .addressable_catalog()?
            .entries
            .iter()
            .filter(|entry| entry.entry.name.to_ascii_lowercase().contains(&needle))
            .take(limit)
            .map(|entry| entry.entry.clone())
            .collect())
    }

    /// Every addressable top-level `.model` whose name contains `needle`.
    ///
    /// This is the contract for a model-card browser. Subcircuits are omitted
    /// because activating them requires a symbol/interface workflow rather
    /// than a model-card selection.
    pub fn search_model_cards(&self, needle: &str, limit: usize) -> io::Result<Vec<CatalogEntry>> {
        let needle = needle.to_ascii_lowercase();
        Ok(self
            .addressable_catalog()?
            .entries
            .iter()
            .filter(|entry| {
                entry.entry.kind.eq_ignore_ascii_case("model")
                    && entry.entry.name.to_ascii_lowercase().contains(&needle)
            })
            .take(limit)
            .map(|entry| entry.entry.clone())
            .collect())
    }

    /// Query the complete addressable part catalog with bounded paging.
    ///
    /// The catalog is streamed once. `needle` matches name, device class,
    /// definition kind, pack identity, and relative source path. `devices`
    /// contains exact canonical device-class tokens; an empty slice selects
    /// every class. The returned count is the complete number of matches, not
    /// merely the bounded page length.
    pub fn query_parts(
        &self,
        needle: &str,
        pack: Option<&str>,
        devices: &[&str],
        offset: usize,
        limit: usize,
    ) -> io::Result<(usize, Vec<CatalogEntry>)> {
        self.query_parts_cancellable(needle, pack, devices, offset, limit, || false)
    }

    /// Query the complete addressable catalog while allowing a superseded UI
    /// request to stop the scan.
    ///
    /// Cancellation is checked between catalog rows. It returns
    /// [`io::ErrorKind::Interrupted`] and never reports a partial match count as
    /// complete.
    pub fn query_parts_cancellable(
        &self,
        needle: &str,
        pack: Option<&str>,
        devices: &[&str],
        offset: usize,
        limit: usize,
        mut is_cancelled: impl FnMut() -> bool,
    ) -> io::Result<(usize, Vec<CatalogEntry>)> {
        let needle = needle.trim().to_ascii_lowercase();
        let pack = pack.map(str::trim).filter(|pack| !pack.is_empty());
        let catalog = self.addressable_catalog()?;
        let mut total = 0usize;
        let mut page = Vec::with_capacity(limit.min(1_024));
        for indexed in &catalog.entries {
            if is_cancelled() {
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "catalog query was superseded",
                ));
            }
            let entry = &indexed.entry;
            if pack.is_some_and(|pack| !entry.pack.eq_ignore_ascii_case(pack))
                || (!devices.is_empty()
                    && !devices
                        .iter()
                        .any(|device| entry.device.eq_ignore_ascii_case(device)))
            {
                continue;
            }
            if !needle.is_empty() && !indexed.search_text.contains(&needle) {
                continue;
            }
            if total >= offset && page.len() < limit {
                page.push(entry.clone());
            }
            total = total.saturating_add(1);
        }
        Ok((total, page))
    }

    /// Exact addressable part counts by canonical device class.
    pub fn part_device_counts(&self) -> io::Result<BTreeMap<String, usize>> {
        Ok(self.addressable_catalog()?.device_counts.clone())
    }

    /// Read a bounded, line-attributed preview from the indexed source file.
    ///
    /// Paths are accepted only when both the pack path and catalog member are
    /// safe relative paths. The indexed line must still declare the requested
    /// name and kind; a stale or tampered index fails rather than showing a
    /// nearby plausible card.
    pub fn definition_preview(&self, entry: &CatalogEntry) -> io::Result<CatalogDefinitionPreview> {
        let source_path = self.catalog_source_path(entry).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "catalog entry has an unsafe or unknown source path",
            )
        })?;
        let metadata = fs::metadata(&source_path)?;
        if metadata.len() > MAX_DEFINITION_PREVIEW_FILE_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "definition source exceeds the {}-byte preview limit",
                    MAX_DEFINITION_PREVIEW_FILE_BYTES
                ),
            ));
        }
        let source = fs::read_to_string(&source_path)?;
        let lines = source.lines().collect::<Vec<_>>();
        let start = entry.line.checked_sub(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "catalog definition line must be one-based",
            )
        })?;
        let indexed = lines.get(start).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "catalog definition line lies beyond the source file",
            )
        })?;
        if !definition_line_matches(indexed, entry) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "catalog entry {}:{} no longer declares {} '{}'",
                    source_path.display(),
                    entry.line,
                    entry.kind,
                    entry.name
                ),
            ));
        }

        let mut selected = Vec::new();
        let mut truncated = false;
        if entry.kind.eq_ignore_ascii_case("subckt") {
            for line in lines.iter().skip(start) {
                if selected.len() >= MAX_DEFINITION_PREVIEW_LINES {
                    truncated = true;
                    break;
                }
                selected.push(*line);
                if selected.len() > 1
                    && line
                        .split_ascii_whitespace()
                        .next()
                        .is_some_and(|token| token.eq_ignore_ascii_case(".ends"))
                {
                    break;
                }
            }
        } else {
            selected.push(indexed);
            for line in lines.iter().skip(start + 1) {
                let trimmed = line.trim_start();
                if !trimmed.starts_with('+') && !trimmed.starts_with('*') {
                    break;
                }
                if selected.len() >= MAX_DEFINITION_PREVIEW_LINES {
                    truncated = true;
                    break;
                }
                selected.push(*line);
            }
        }
        let source = selected
            .iter()
            .enumerate()
            .map(|(offset, line)| format!("{:>7}  {line}", entry.line + offset))
            .collect::<Vec<_>>()
            .join("\n");
        Ok(CatalogDefinitionPreview {
            source_path,
            start_line: entry.line,
            source,
            truncated,
        })
    }

    /// Revalidate and extract the exact ordered terminal list from an indexed
    /// top-level `.subckt` declaration.
    pub fn subcircuit_interface(
        &self,
        entry: &CatalogEntry,
    ) -> io::Result<CatalogSubcircuitInterface> {
        if !entry.kind.eq_ignore_ascii_case("subckt") || !entry.scope.is_addressable() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "catalog entry is not an addressable subcircuit",
            ));
        }
        let source_path = self.catalog_source_path(entry).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "catalog entry has an unsafe or unknown source path",
            )
        })?;
        let metadata = fs::metadata(&source_path)?;
        if metadata.len() > MAX_DEFINITION_PREVIEW_FILE_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "subcircuit source exceeds the bounded interface-inspection limit",
            ));
        }
        let source = fs::read_to_string(&source_path)?;
        let lines = source.lines().collect::<Vec<_>>();
        let start = entry.line.checked_sub(1).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "catalog definition line must be one-based",
            )
        })?;
        let indexed = lines.get(start).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "catalog definition line lies beyond the source file",
            )
        })?;
        if !definition_line_matches(indexed, entry) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "indexed subcircuit declaration changed",
            ));
        }

        let mut declaration = indexed.trim().to_owned();
        for line in lines.iter().skip(start + 1) {
            let Some(continuation) = line.trim_start().strip_prefix('+') else {
                break;
            };
            declaration.push(' ');
            declaration.push_str(continuation.trim());
        }
        let mut tokens = declaration.split_ascii_whitespace();
        let directive = tokens.next().unwrap_or_default();
        let name = tokens.next().unwrap_or_default();
        if !directive.eq_ignore_ascii_case(".subckt") || !name.eq_ignore_ascii_case(&entry.name) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "indexed subcircuit declaration does not match its catalog identity",
            ));
        }
        let mut ports = Vec::new();
        let mut folded = HashSet::new();
        for token in tokens {
            if token.eq_ignore_ascii_case("params:")
                || token.eq_ignore_ascii_case("param:")
                || token.contains('=')
            {
                break;
            }
            let canonical = token.to_ascii_lowercase();
            if token.is_empty() || !folded.insert(canonical) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "subcircuit '{}' has an invalid repeated terminal",
                        entry.name
                    ),
                ));
            }
            ports.push(token.to_owned());
        }
        if ports.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("subcircuit '{}' declares no terminals", entry.name),
            ));
        }
        Ok(CatalogSubcircuitInterface {
            name: entry.name.clone(),
            ports,
            source_path,
            start_line: entry.line,
        })
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
        Ok(self
            .addressable_catalog()?
            .entries
            .iter()
            .filter(|entry| entry.entry.device == device)
            .take(limit)
            .map(|entry| entry.entry.clone())
            .collect())
    }

    /// The whole catalog, nested definitions included. Around 199,000 entries;
    /// prefer the search methods.
    pub fn load_catalog(&self) -> io::Result<Vec<CatalogEntry>> {
        self.filter_catalog(|_| true)
    }

    fn catalog_source_path(&self, entry: &CatalogEntry) -> Option<PathBuf> {
        if !self.source_files_available() {
            return None;
        }
        let pack = self.pack(&entry.pack)?;
        if !safe_relative_path(&pack.path) || !safe_relative_path(&entry.path) {
            return None;
        }
        Some(self.root.join(&pack.path).join(&entry.path))
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

    fn addressable_catalog(&self) -> io::Result<&AddressableCatalog> {
        let result = self.addressable_catalog.get_or_init(|| {
            self.build_addressable_catalog()
                .map(Arc::new)
                .map_err(|error| error.to_string())
        });
        match result {
            Ok(catalog) => Ok(catalog),
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error.clone())),
        }
    }

    fn build_addressable_catalog(&self) -> io::Result<AddressableCatalog> {
        let mut entries = Vec::with_capacity(self.part_count());
        let mut device_counts = BTreeMap::new();
        self.for_each_catalog_entry(|entry| {
            if !entry.scope.is_addressable() {
                return true;
            }
            *device_counts.entry(entry.device.clone()).or_default() += 1;
            let search_text = format!(
                "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
                entry.name,
                entry.device,
                entry.kind,
                entry.pack,
                entry.path.to_string_lossy()
            )
            .to_ascii_lowercase();
            entries.push(IndexedCatalogEntry { entry, search_text });
            true
        })?;
        Ok(AddressableCatalog {
            entries,
            device_counts,
        })
    }

    /// Stream the catalog, stopping early when `visit` returns `false`.
    ///
    /// The catalog is around 16 MB. Streaming keeps a part lookup from paying
    /// the allocation cost of materialising nearly 200,000 entries.
    fn for_each_catalog_entry(
        &self,
        mut visit: impl FnMut(CatalogEntry) -> bool,
    ) -> io::Result<()> {
        #[cfg(target_arch = "wasm32")]
        if let Some(catalog) = self.embedded_catalog {
            for line in catalog.lines() {
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
            return Ok(());
        }
        let path = self.root.join(CATALOG_INDEX);
        let reader = BufReader::new(fs::File::open(path)?);
        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let Some(entry) = parse_catalog_row(&line) else {
                continue;
            };
            if !visit(entry) {
                break;
            }
        }
        Ok(())
    }
}

fn safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path.components().all(|component| {
            matches!(
                component,
                std::path::Component::Normal(_) | std::path::Component::CurDir
            )
        })
}

fn definition_line_matches(line: &str, entry: &CatalogEntry) -> bool {
    let mut tokens = line.split_ascii_whitespace();
    let Some(directive) = tokens.next() else {
        return false;
    };
    let Some(name) = tokens.next() else {
        return false;
    };
    directive
        .strip_prefix('.')
        .is_some_and(|directive| directive.eq_ignore_ascii_case(&entry.kind))
        && name.eq_ignore_ascii_case(&entry.name)
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
    fn model_card_catalog_excludes_subcircuits_and_nested_helpers() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        assert!(
            index.model_count() > 15_000,
            "expected the shipped top-level model set"
        );
        let matches = index
            .search_model_cards("1N4148", 50)
            .expect("model-card catalog readable");
        assert!(!matches.is_empty());
        assert!(matches.iter().all(|entry| {
            entry.scope == DefinitionScope::TopLevel && entry.kind.eq_ignore_ascii_case("model")
        }));
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
    fn bounded_catalog_query_reports_complete_match_count() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let (total, page) = index
            .query_parts("1n4148", None, &[], 0, 2)
            .expect("catalog query succeeds");
        assert!(total > page.len());
        assert_eq!(page.len(), 2);
        assert!(page.iter().all(|entry| {
            entry.scope.is_addressable()
                && (entry.name.to_ascii_lowercase().contains("1n4148")
                    || entry.device.to_ascii_lowercase().contains("1n4148")
                    || entry.pack.to_ascii_lowercase().contains("1n4148")
                    || entry
                        .path
                        .to_string_lossy()
                        .to_ascii_lowercase()
                        .contains("1n4148"))
        }));
    }

    #[test]
    fn catalog_pages_are_stable_disjoint_windows_of_one_index() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let (first_total, first) = index.query_parts("", None, &[], 0, 25).expect("first page");
        let (second_total, second) = index
            .query_parts("", None, &[], 25, 25)
            .expect("second page");
        assert_eq!(first_total, second_total);
        assert_eq!(first.len(), 25);
        assert_eq!(second.len(), 25);
        let identity = |entry: &CatalogEntry| {
            (
                entry.pack.clone(),
                entry.path.clone(),
                entry.line,
                entry.name.clone(),
            )
        };
        let first = first
            .iter()
            .map(identity)
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            second
                .iter()
                .map(identity)
                .all(|entry| !first.contains(&entry)),
            "adjacent pages must never repeat an exact catalog identity"
        );
    }

    #[test]
    fn cancelled_catalog_query_never_reports_a_partial_total() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let error = index
            .query_parts_cancellable("", None, &[], 0, 25, || true)
            .expect_err("superseded query must stop");
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
    }

    #[test]
    fn device_facets_sum_to_the_addressable_part_total() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let counts = index.part_device_counts().expect("catalog counts");
        assert_eq!(counts.values().sum::<usize>(), index.part_count());
        assert!(counts.get("diode").copied().unwrap_or_default() > 1_000);
    }

    #[test]
    fn definition_preview_revalidates_the_indexed_line_and_bounds_source() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let entry = index
            .find_part("1N4148")
            .expect("catalog readable")
            .into_iter()
            .next()
            .expect("fixture definition exists");
        let preview = index
            .definition_preview(&entry)
            .expect("indexed source preview");
        assert_eq!(preview.start_line, entry.line);
        assert!(preview.source_path.is_file());
        assert!(preview.source.to_ascii_lowercase().contains("1n4148"));
        assert!(preview.source.lines().count() <= MAX_DEFINITION_PREVIEW_LINES);

        let mut unsafe_entry = entry;
        unsafe_entry.path = PathBuf::from("../outside.lib");
        assert!(unsafe_entry.source_path(&index).is_none());
        assert!(index.definition_preview(&unsafe_entry).is_err());
    }

    #[test]
    fn subcircuit_interface_preserves_exact_terminal_order() {
        let index = SpiceLibraryIndex::open(repo_models_root()).expect("index opens");
        let entry = index
            .find_part("7400")
            .expect("catalog readable")
            .into_iter()
            .find(|entry| entry.pack == "ngspice-74xx-logic" && entry.kind == "subckt")
            .expect("addressable 7400 subcircuit exists");
        let interface = index
            .subcircuit_interface(&entry)
            .expect("interface revalidates");
        assert_eq!(interface.name, "7400");
        assert!(!interface.ports.is_empty());
        assert_eq!(interface.start_line, entry.line);
        assert!(interface.source_path.is_file());

        let model = index
            .find_part("1N4148")
            .expect("catalog readable")
            .into_iter()
            .find(|entry| entry.kind == "model")
            .expect("model entry exists");
        assert!(index.subcircuit_interface(&model).is_err());
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
