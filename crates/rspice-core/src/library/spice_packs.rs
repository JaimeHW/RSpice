//! Discovery and lookup across the shipped SPICE model packs.
//!
//! The RSpice-authored foundation library is compiled into the binary. Native
//! release archives also carry its source and small browse indexes. Browser
//! builds use the embedded library and carry no separate pack catalog.
//!
//! An external model tree (for example a development corpus or an installed
//! pack root) is never discovered by the product unless a caller opens it
//! explicitly.
//!
//! Two generated indexes back it, both produced by
//! `tools/models/build_manifest.py` and both tab-separated so that no TOML
//! parser is needed here:
//!
//! Product discovery reads `SHIPPED-PACKS.tsv` and
//! `SHIPPED-CATALOG.tsv`. [`SpiceLibraryIndex::open`] remains available for
//! an explicitly selected development corpus containing `PACKS.tsv` and
//! `CATALOG.tsv`.
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
#[cfg(not(target_arch = "wasm32"))]
const SHIPPED_PACKS_INDEX: &str = "SHIPPED-PACKS.tsv";
#[cfg(not(target_arch = "wasm32"))]
const SHIPPED_CATALOG_INDEX: &str = "SHIPPED-CATALOG.tsv";
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
    /// Stable identifier, such as `rspice-foundation`.
    pub id: String,
    /// Human-readable title.
    pub name: String,
    /// Source grouping, such as `builtin` for the embedded foundation pack.
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
    /// over six thousand times across the developer corpus. Such a name cannot be
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
    /// This is per *file*, not per pack: a pack routinely mixes clean and
    /// restricted files, so the pack flag is too blunt to decide whether one
    /// card may be shipped.
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
    catalog_index: &'static str,
    addressable_catalog: Arc<OnceLock<Result<Arc<AddressableCatalog>, String>>>,
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
    /// 1. `RSPICE_MODELS_DIR`, which wins outright and must contain the two
    ///    shipped indexes. An invalid explicit root is an error.
    /// 2. `models/spice` beside the executable, which is the installed layout.
    /// 3. `models/spice` in an ancestor of the executable, which covers running
    ///    straight out of `target/debug` during development.
    ///
    /// A missing automatically discovered tree is not an error: the foundation
    /// library is compiled in, so RSpice runs without the native source copy.
    pub fn discover() -> io::Result<Option<Self>> {
        #[cfg(target_arch = "wasm32")]
        {
            Ok(None)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::discover_native(
                std::env::var_os(MODELS_DIR_ENV).map(PathBuf::from),
                Self::candidate_roots(),
            )
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn discover_native(
        explicit: Option<PathBuf>,
        candidates: impl IntoIterator<Item = PathBuf>,
    ) -> io::Result<Option<Self>> {
        if let Some(root) = explicit {
            if !root.join(SHIPPED_PACKS_INDEX).is_file()
                || !root.join(SHIPPED_CATALOG_INDEX).is_file()
            {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "{MODELS_DIR_ENV} points to '{}', which does not contain {} and {}",
                        root.display(),
                        SHIPPED_PACKS_INDEX,
                        SHIPPED_CATALOG_INDEX
                    ),
                ));
            }
            return Self::open_shipped(root).map(Some);
        }
        for candidate in candidates {
            if candidate.join(SHIPPED_PACKS_INDEX).is_file()
                && candidate.join(SHIPPED_CATALOG_INDEX).is_file()
            {
                return Ok(Some(Self::open_shipped(candidate)?));
            }
        }
        Ok(None)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn candidate_roots() -> Vec<PathBuf> {
        let mut roots = Vec::new();

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
        Self::open_with_indexes(root.into(), PACKS_INDEX, CATALOG_INDEX)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn open_shipped(root: impl Into<PathBuf>) -> io::Result<Self> {
        Self::open_with_indexes(root.into(), SHIPPED_PACKS_INDEX, SHIPPED_CATALOG_INDEX)
    }

    fn open_with_indexes(
        root: PathBuf,
        packs_index: &'static str,
        catalog_index: &'static str,
    ) -> io::Result<Self> {
        let packs = parse_packs(&fs::read_to_string(root.join(packs_index))?);
        Ok(Self {
            root,
            packs,
            catalog_index,
            addressable_catalog: Arc::new(OnceLock::new()),
        })
    }

    /// Root directory of the model tree.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Every indexed pack, ordered as the index lists them.
    pub fn packs(&self) -> &[SpicePack] {
        &self.packs
    }

    /// Whether indexed definition source files are locally available.
    ///
    /// Browser builds have no filesystem pack catalog. Their foundation models
    /// come from the embedded library and user sources are project imports.
    pub fn source_files_available(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            false
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

    /// Total `.model` and `.subckt` definitions across indexed packs, including
    /// those private to a subcircuit body.
    pub fn definition_count(&self) -> usize {
        self.packs
            .iter()
            .map(|pack| pack.models + pack.subcircuits)
            .sum()
    }

    /// Definitions a netlist can actually reference by name.
    ///
    /// Large development catalogs contain many helper cards inside macromodel
    /// bodies, so this is the figure to show a user as a part count.
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

    /// The whole selected catalog, nested definitions included. Prefer the
    /// search methods for a full developer corpus.
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
    /// Streaming keeps a part lookup from materialising a large developer
    /// catalog.
    fn for_each_catalog_entry(
        &self,
        mut visit: impl FnMut(CatalogEntry) -> bool,
    ) -> io::Result<()> {
        let path = self.root.join(self.catalog_index);
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

    //=========================================================================
    // Synthetic corpus
    //
    // The repository ships the foundation pack alone, so multi-pack lookup,
    // paging, per-file restriction and macromodel scope are exercised against
    // a corpus built here. The `.lib` files and the index rows that address
    // them are generated together because `definition_preview` and
    // `subcircuit_interface` re-read the indexed line and reject a row that no
    // longer declares its own name: a fixture line number is exact or the test
    // fails.
    //=========================================================================

    /// Top-scope `mosfet-n` cards, enough for two disjoint browse pages.
    const FIXTURE_MOSFET_CARDS: usize = 40;
    /// Macromodel subcircuits carrying private helper cards.
    const FIXTURE_MACROMODELS: usize = 12;
    /// `DX` diodes private to each macromodel body.
    const FIXTURE_NESTED_PER_MACROMODEL: usize = 8;
    /// Top-scope `.model` cards in `fixture-open` outside the `mosfet-n` block.
    const FIXTURE_DISCRETE_CARDS: usize = 8;
    /// `.model` and `.subckt` declarations at any depth, both packs.
    const FIXTURE_DEFINITIONS: usize = 160;
    /// Declarations a netlist can reference by name, both packs.
    const FIXTURE_PARTS: usize = 64;
    /// Addressable `.model` cards, both packs.
    const FIXTURE_MODEL_CARDS: usize = 51;
    /// Addressable `DX` cards; every other `DX` is macromodel-private.
    const FIXTURE_TOP_LEVEL_DX: usize = 2;

    /// One indexed definition, recorded as its declaring line is written.
    struct FixtureRow {
        name: String,
        kind: &'static str,
        device: &'static str,
        path: String,
        line: usize,
        restricted: bool,
        scope: &'static str,
    }

    /// A `.lib` file under construction. `next_line` is the 1-based line the
    /// next append will occupy, so each row carries its own exact line.
    struct FixtureFile {
        path: String,
        text: String,
        next_line: usize,
        rows: Vec<FixtureRow>,
    }

    impl FixtureFile {
        fn new(path: &str) -> Self {
            Self {
                path: path.to_owned(),
                text: String::new(),
                next_line: 1,
                rows: Vec::new(),
            }
        }

        fn push(&mut self, line: &str) -> usize {
            let at = self.next_line;
            self.text.push_str(line);
            self.text.push('\n');
            self.next_line += 1;
            at
        }

        fn model(
            &mut self,
            name: &str,
            device: &'static str,
            card: &str,
            scope: &'static str,
            restricted: bool,
        ) {
            let line = self.push(&format!(".model {name} {card}"));
            self.rows.push(FixtureRow {
                name: name.to_owned(),
                kind: "model",
                device,
                path: self.path.clone(),
                line,
                restricted,
                scope,
            });
        }

        fn subckt(&mut self, name: &str, ports: &str, device: &'static str) {
            let line = self.push(&format!(".subckt {name} {ports}"));
            self.rows.push(FixtureRow {
                name: name.to_owned(),
                kind: "subckt",
                device,
                path: self.path.clone(),
                line,
                restricted: false,
                scope: "top",
            });
        }
    }

    struct FixturePack {
        id: &'static str,
        category: &'static str,
        tier: &'static str,
        spdx: &'static str,
        redistributable: bool,
        entry: &'static str,
        name: &'static str,
        files: Vec<FixtureFile>,
    }

    /// A materialized corpus; [`FixtureCorpus::discard`] removes it.
    struct FixtureCorpus {
        root: PathBuf,
    }

    impl FixtureCorpus {
        fn index(&self) -> SpiceLibraryIndex {
            SpiceLibraryIndex::open(&self.root).expect("fixture index opens")
        }

        fn discard(self) {
            fs::remove_dir_all(&self.root).expect("remove fixture corpus");
        }
    }

    /// The redistributable pack: the browse, paging and macromodel corpus.
    fn fixture_open_pack() -> FixturePack {
        let mut entry = FixtureFile::new("lib/open.lib");
        entry.push("* fixture-open entry deck");
        for index in 0..FIXTURE_MOSFET_CARDS {
            entry.model(
                &format!("FIXNMOS_{index:02}"),
                "mosfet-n",
                &format!("NMOS (LEVEL=1 VTO=0.7 KP={}e-6)", index + 1),
                "top",
                false,
            );
        }

        let mut parts = FixtureFile::new("lib/parts.lib");
        parts.push("* fixture-open discrete parts");
        // Three names sharing one query substring, so a bounded query has more
        // matches than a two-row page.
        for suffix in ["A", "B", "C"] {
            parts.model(
                &format!("FX1N-{suffix}"),
                "diode",
                "D (IS=1e-14 N=1)",
                "top",
                false,
            );
        }
        parts.model("FIXDIODE", "diode", "D (IS=2e-14 N=1.02", "top", false);
        parts.push("+ RS=0.5 CJO=2p)");
        parts.model(
            "FIXJFET",
            "jfet-n",
            "NJF (VTO=-2.0 BETA=1.3m)",
            "top",
            false,
        );
        parts.model("DX", "diode", "D (IS=1e-15)", "top", false);
        parts.model("FIXSHARED", "diode", "D (IS=3e-14 N=1.05)", "top", false);
        parts.subckt("FIXNAND", "A B Y VCC GND", "logic");
        parts.push("M1 Y A VCC VCC FIXPMOS W=4u L=1u");
        parts.push("M2 Y B VCC VCC FIXPMOS W=4u L=1u");
        parts.push("M3 Y A ND GND FIXNMOS_00 W=2u L=1u");
        parts.push("M4 ND B GND GND FIXNMOS_00 W=2u L=1u");
        parts.push(".ends FIXNAND");

        let mut macromodels = FixtureFile::new("lib/macromodels.lib");
        macromodels.push("* fixture-open op-amp macromodels");
        // One `DX` hoisted to file scope alongside the private ones: the
        // addressable population must be small, not empty.
        macromodels.model("DX", "diode", "D (IS=1e-15)", "top", false);
        for index in 0..FIXTURE_MACROMODELS {
            macromodels.subckt(
                &format!("FIXOPA_{index:02}"),
                "INP INN VCC VEE OUT",
                "opamp",
            );
            for helper in 0..FIXTURE_NESTED_PER_MACROMODEL {
                macromodels.model(
                    "DX",
                    "diode",
                    &format!("D (IS=1e-16 RS={helper})"),
                    "nested",
                    false,
                );
            }
            macromodels.push(".ends");
        }

        FixturePack {
            id: "fixture-open",
            category: "foundry",
            tier: "permissive",
            spdx: "Apache-2.0",
            redistributable: true,
            entry: "lib/open.lib",
            name: "Fixture Open Pack",
            files: vec![entry, parts, macromodels],
        }
    }

    /// The pack with no established redistribution grant, holding one
    /// per-file restriction and one part name that collides with the open pack.
    fn fixture_restricted_pack() -> FixturePack {
        let mut locked = FixtureFile::new("lib/locked.lib");
        locked.push("* this file alone is excluded by the licence audit");
        locked.model("FIXLOCKED", "diode", "D (IS=4e-14 N=1.2)", "top", true);

        let mut shared = FixtureFile::new("lib/shared.lib");
        shared.push("* clean file inside an unestablished pack");
        shared.model("FIXSHARED", "diode", "D (IS=5e-14 N=1.1)", "top", false);
        shared.model("FIXNPN", "bjt-npn", "NPN (BF=180 IS=1e-16)", "top", false);

        FixturePack {
            id: "fixture-restricted",
            category: "vendor",
            tier: "ambiguous",
            spdx: "NOASSERTION",
            redistributable: false,
            entry: "lib/shared.lib",
            name: "Fixture Unestablished Pack",
            files: vec![locked, shared],
        }
    }

    /// Write the corpus under a directory of its own, so a failing test never
    /// leaves a shared tree behind for the next one.
    fn fixture_corpus(label: &str) -> FixtureCorpus {
        let root = std::env::temp_dir().join(format!(
            "rspice-spice-packs-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ));
        let packs = [fixture_open_pack(), fixture_restricted_pack()];
        let mut packs_index = String::from(
            "# id\tcategory\tpath\ttier\tspdx\tredistributable\tentry\tmodels\tsubcircuits\tmodels_top\tsubcircuits_top\tfiles\tbytes\tdevices\tname\n",
        );
        let mut catalog_index =
            String::from("# name\tkind\tdevice\tpack\tpath\tline\trestricted\tscope\n");
        for pack in &packs {
            let mut models = 0usize;
            let mut subcircuits = 0usize;
            let mut models_top = 0usize;
            let mut subcircuits_top = 0usize;
            let mut bytes = 0u64;
            let mut devices = BTreeMap::new();
            for file in &pack.files {
                let path = root.join(pack.id).join(index_path(&file.path));
                fs::create_dir_all(path.parent().expect("fixture source has a parent"))
                    .expect("create fixture pack directory");
                fs::write(&path, &file.text).expect("write fixture source");
                bytes += file.text.len() as u64;
                for row in &file.rows {
                    let addressable = row.scope == "top";
                    if row.kind == "subckt" {
                        subcircuits += 1;
                        subcircuits_top += usize::from(addressable);
                    } else {
                        models += 1;
                        models_top += usize::from(addressable);
                    }
                    if addressable {
                        *devices.entry(row.device).or_insert(0usize) += 1;
                    }
                    catalog_index.push_str(&format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                        row.name,
                        row.kind,
                        row.device,
                        pack.id,
                        row.path,
                        row.line,
                        u8::from(row.restricted),
                        row.scope
                    ));
                }
            }
            packs_index.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{models}\t{subcircuits}\t{models_top}\t{subcircuits_top}\t{}\t{bytes}\t{}\t{}\n",
                pack.id,
                pack.category,
                pack.id,
                pack.tier,
                pack.spdx,
                u8::from(pack.redistributable),
                pack.entry,
                pack.files.len(),
                devices.keys().copied().collect::<Vec<_>>().join(","),
                pack.name
            ));
        }
        fs::write(root.join(PACKS_INDEX), packs_index).expect("write fixture pack index");
        fs::write(root.join(CATALOG_INDEX), catalog_index).expect("write fixture catalog");
        FixtureCorpus { root }
    }

    #[test]
    fn packs_index_parses() {
        let corpus = fixture_corpus("packs-index");
        let index = corpus.index();
        assert_eq!(index.packs().len(), 2);

        let open = index.pack("fixture-open").expect("open pack discovered");
        assert_eq!(open.category, "foundry");
        assert_eq!(open.tier, LicenseTier::Permissive);
        assert_eq!(open.spdx, "Apache-2.0");
        assert!(open.redistributable);
        assert_eq!(open.entry, Some(index_path("lib/open.lib")));
        assert_eq!(open.files, 3);
        assert_eq!(
            open.models,
            FIXTURE_MOSFET_CARDS
                + FIXTURE_DISCRETE_CARDS
                + FIXTURE_MACROMODELS * FIXTURE_NESTED_PER_MACROMODEL
        );
        assert_eq!(
            open.models_top,
            FIXTURE_MOSFET_CARDS + FIXTURE_DISCRETE_CARDS
        );
        assert_eq!(open.subcircuits, FIXTURE_MACROMODELS + 1);
        assert!(open.devices.contains(&"mosfet-n".to_string()));

        let restricted = index
            .pack("fixture-restricted")
            .expect("unestablished pack discovered");
        assert_eq!(restricted.tier, LicenseTier::Ambiguous);
        assert_eq!(restricted.spdx, "NOASSERTION");
        assert!(!restricted.redistributable);
        assert_eq!(restricted.models_top, 3);

        corpus.discard();
    }

    #[test]
    fn shipped_index_contains_only_the_foundation_pack() {
        let index =
            SpiceLibraryIndex::open_shipped(repo_models_root()).expect("shipped index opens");
        assert_eq!(index.packs().len(), 1);
        let pack = &index.packs()[0];
        assert_eq!(pack.id, "rspice-foundation");
        assert_eq!(pack.tier, LicenseTier::Own);
        assert!(pack.redistributable);
        assert_eq!(index.model_count(), 16);
        assert_eq!(index.part_count(), 17);
        assert_eq!(index.load_catalog().expect("catalog").len(), 17);
        assert!(index.find_part("1N4148").expect("lookup").is_empty());
        assert_eq!(index.find_part("RSPICE_DIODE").expect("lookup").len(), 1);
    }

    #[test]
    fn explicit_model_root_never_falls_back_to_an_automatic_candidate() {
        let valid = repo_models_root();
        let missing = valid.join("missing-explicit-root");
        let error = SpiceLibraryIndex::discover_native(Some(missing), vec![valid.clone()])
            .expect_err("an invalid explicit root must fail");
        assert_eq!(error.kind(), io::ErrorKind::NotFound);

        let discovered = SpiceLibraryIndex::discover_native(None, vec![valid])
            .expect("automatic discovery succeeds")
            .expect("foundation index found");
        assert_eq!(discovered.packs()[0].id, "rspice-foundation");
    }

    #[test]
    fn definition_count_spans_the_whole_tree() {
        let corpus = fixture_corpus("definition-count");
        let index = corpus.index();
        // Every declaration in both packs, macromodel-private cards included.
        assert_eq!(index.definition_count(), FIXTURE_DEFINITIONS);
        corpus.discard();
    }

    #[test]
    fn macromodel_internals_are_not_offered_as_parts() {
        let corpus = fixture_corpus("macromodel-scope");
        let index = corpus.index();

        // `DX` is the canonical helper-diode name inside op-amp macromodels,
        // declared once per body and referenceable from almost none of them.
        // This is the regression that made the browser unusable: every diode
        // search surfaced the same private card over and over. A file-scope
        // `DX` does occur, so the addressable result is small rather than
        // empty — the point is that it collapses.
        let as_part = index.find_part("DX").expect("catalog readable");
        let anywhere = index
            .find_definition_any_scope("DX")
            .expect("catalog readable");

        let nested = FIXTURE_MACROMODELS * FIXTURE_NESTED_PER_MACROMODEL;
        assert_eq!(anywhere.len(), nested + FIXTURE_TOP_LEVEL_DX);
        assert_eq!(as_part.len(), FIXTURE_TOP_LEVEL_DX);
        assert!(
            as_part
                .iter()
                .all(|entry| entry.scope == DefinitionScope::TopLevel),
            "a part hit must be addressable"
        );
        assert_eq!(
            anywhere
                .iter()
                .filter(|entry| entry.scope == DefinitionScope::Nested)
                .count(),
            nested
        );

        corpus.discard();
    }

    #[test]
    fn addressable_parts_are_a_fraction_of_all_definitions() {
        let corpus = fixture_corpus("addressable-fraction");
        let index = corpus.index();
        let parts = index.part_count();
        let all = index.definition_count();
        assert_eq!(parts, FIXTURE_PARTS);
        assert!(
            parts * 2 < all,
            "most definitions are macromodel internals: {parts} of {all}"
        );
        corpus.discard();
    }

    #[test]
    fn model_card_catalog_excludes_subcircuits_and_nested_helpers() {
        let corpus = fixture_corpus("model-cards");
        let index = corpus.index();
        assert_eq!(index.model_count(), FIXTURE_MODEL_CARDS);
        let matches = index
            .search_model_cards("FX1N", 50)
            .expect("model-card catalog readable");
        assert_eq!(matches.len(), 3);
        assert!(matches.iter().all(|entry| {
            entry.scope == DefinitionScope::TopLevel && entry.kind.eq_ignore_ascii_case("model")
        }));
        // A subcircuit is addressable but is not a model card.
        assert!(
            index
                .search_model_cards("FIXNAND", 50)
                .expect("model-card catalog readable")
                .is_empty()
        );
        corpus.discard();
    }

    #[test]
    fn part_lookup_returns_every_pack_that_defines_it() {
        let corpus = fixture_corpus("multi-pack-lookup");
        let index = corpus.index();
        let matches = index.find_part("FIXSHARED").expect("catalog readable");
        assert_eq!(matches.len(), 2);
        // The point of returning all of them: the same part number carries
        // different fits in different packs, and the caller must choose.
        let packs: std::collections::BTreeSet<_> =
            matches.iter().map(|m| m.pack.as_str()).collect();
        assert_eq!(
            packs,
            std::collections::BTreeSet::from(["fixture-open", "fixture-restricted"])
        );
        corpus.discard();
    }

    #[test]
    fn entry_and_source_paths_resolve_on_disk() {
        let corpus = fixture_corpus("paths-resolve");
        let index = corpus.index();

        let open = index.pack("fixture-open").expect("open pack");
        let entry = open.entry_path(index.root()).expect("open pack entry path");
        assert!(entry.is_file(), "pack entry deck missing at {entry:?}");

        let jfet = index
            .find_part("FIXJFET")
            .expect("catalog readable")
            .into_iter()
            .next()
            .expect("FIXJFET present");
        let source = jfet.source_path(&index).expect("owning pack known");
        assert!(
            source.is_file(),
            "catalog path does not resolve: {source:?}"
        );

        corpus.discard();
    }

    #[test]
    fn restricted_files_stay_traceable_but_never_shippable() {
        // Restriction is per file, not per pack: a card in a restricted file
        // must still be locatable, or a deck that references it cannot be
        // diagnosed, while every redistribution path has to drop it.
        let corpus = fixture_corpus("restricted-files");
        let index = corpus.index();

        let addressable = index.find_part("FIXLOCKED").expect("catalog readable");
        assert_eq!(addressable.len(), 1);
        assert!(addressable[0].restricted);
        assert_eq!(
            index
                .find_definition_any_scope("FIXLOCKED")
                .expect("catalog readable")
                .len(),
            1
        );
        assert!(
            index
                .find_shippable_part("FIXLOCKED")
                .expect("catalog readable")
                .is_empty(),
            "a restricted file must never reach the shippable view"
        );
        assert!(
            addressable[0]
                .source_path(&index)
                .expect("owning pack known")
                .is_file()
        );

        // A clean file in the same pack is unaffected by its neighbour.
        assert_eq!(
            index
                .find_part("FIXSHARED")
                .expect("catalog readable")
                .len(),
            index
                .find_shippable_part("FIXSHARED")
                .expect("catalog readable")
                .len()
        );

        corpus.discard();
    }

    #[test]
    fn device_class_query_finds_foundry_devices() {
        let corpus = fixture_corpus("device-class");
        let index = corpus.index();
        let jfets = index
            .parts_by_device("jfet-n", 50)
            .expect("catalog readable");
        assert_eq!(jfets.len(), 1);
        assert!(jfets.iter().all(|entry| entry.device == "jfet-n"));
        corpus.discard();
    }

    #[test]
    fn bounded_catalog_query_reports_complete_match_count() {
        let corpus = fixture_corpus("bounded-query");
        let index = corpus.index();
        let (total, page) = index
            .query_parts("fx1n", None, &[], 0, 2)
            .expect("catalog query succeeds");
        assert_eq!(total, 3);
        assert_eq!(page.len(), 2);
        assert!(page.iter().all(|entry| {
            entry.scope.is_addressable()
                && (entry.name.to_ascii_lowercase().contains("fx1n")
                    || entry.device.to_ascii_lowercase().contains("fx1n")
                    || entry.pack.to_ascii_lowercase().contains("fx1n")
                    || entry
                        .path
                        .to_string_lossy()
                        .to_ascii_lowercase()
                        .contains("fx1n"))
        }));
        corpus.discard();
    }

    #[test]
    fn catalog_pages_are_stable_disjoint_windows_of_one_index() {
        let corpus = fixture_corpus("stable-paging");
        let index = corpus.index();
        let (first_total, first) = index.query_parts("", None, &[], 0, 25).expect("first page");
        let (second_total, second) = index
            .query_parts("", None, &[], 25, 25)
            .expect("second page");
        assert_eq!(first_total, FIXTURE_PARTS);
        assert_eq!(second_total, FIXTURE_PARTS);
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
        corpus.discard();
    }

    #[test]
    fn cancelled_catalog_query_never_reports_a_partial_total() {
        let corpus = fixture_corpus("cancelled-query");
        let index = corpus.index();
        let error = index
            .query_parts_cancellable("", None, &[], 0, 25, || true)
            .expect_err("superseded query must stop");
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
        corpus.discard();
    }

    #[test]
    fn device_facets_sum_to_the_addressable_part_total() {
        let corpus = fixture_corpus("device-facets");
        let index = corpus.index();
        let counts = index.part_device_counts().expect("catalog counts");
        assert_eq!(counts.values().sum::<usize>(), index.part_count());
        assert_eq!(counts.get("mosfet-n").copied(), Some(FIXTURE_MOSFET_CARDS));
        assert_eq!(counts.get("opamp").copied(), Some(FIXTURE_MACROMODELS));
        assert_eq!(counts.get("diode").copied(), Some(9));
        assert_eq!(counts.get("jfet-n").copied(), Some(1));
        corpus.discard();
    }

    #[test]
    fn definition_preview_revalidates_the_indexed_line_and_bounds_source() {
        let corpus = fixture_corpus("definition-preview");
        let index = corpus.index();
        let entry = index
            .find_part("FIXDIODE")
            .expect("catalog readable")
            .into_iter()
            .next()
            .expect("fixture definition exists");
        let preview = index
            .definition_preview(&entry)
            .expect("indexed source preview");
        assert_eq!(preview.start_line, entry.line);
        assert!(preview.source_path.is_file());
        assert!(preview.source.to_ascii_lowercase().contains("fixdiode"));
        // The card's `+` continuation belongs to the preview.
        assert!(preview.source.contains("RS=0.5"));
        assert!(preview.source.lines().count() <= MAX_DEFINITION_PREVIEW_LINES);

        let mut unsafe_entry = entry;
        unsafe_entry.path = PathBuf::from("../outside.lib");
        assert!(unsafe_entry.source_path(&index).is_none());
        assert!(index.definition_preview(&unsafe_entry).is_err());

        corpus.discard();
    }

    #[test]
    fn subcircuit_interface_preserves_exact_terminal_order() {
        let corpus = fixture_corpus("subcircuit-interface");
        let index = corpus.index();
        let entry = index
            .find_part("FIXNAND")
            .expect("catalog readable")
            .into_iter()
            .find(|entry| entry.kind == "subckt")
            .expect("addressable FIXNAND subcircuit exists");
        let interface = index
            .subcircuit_interface(&entry)
            .expect("interface revalidates");
        assert_eq!(interface.name, "FIXNAND");
        assert_eq!(interface.ports, ["A", "B", "Y", "VCC", "GND"]);
        assert_eq!(interface.start_line, entry.line);
        assert!(interface.source_path.is_file());

        let model = index
            .find_part("FIXDIODE")
            .expect("catalog readable")
            .into_iter()
            .find(|entry| entry.kind == "model")
            .expect("model entry exists");
        assert!(index.subcircuit_interface(&model).is_err());

        corpus.discard();
    }

    #[test]
    fn browse_parts_is_bounded_and_only_returns_addressable_cards() {
        let corpus = fixture_corpus("browse-parts");
        let index = corpus.index();
        let parts = index.browse_parts(25).expect("catalog readable");
        assert_eq!(parts.len(), 25);
        assert!(
            parts
                .iter()
                .all(|entry| entry.scope == DefinitionScope::TopLevel)
        );
        assert!(index.browse_parts(0).expect("catalog readable").is_empty());
        corpus.discard();
    }

    #[test]
    fn redistributable_packs_exclude_unestablished_ones() {
        let corpus = fixture_corpus("redistributable-packs");
        let index = corpus.index();
        let shippable: Vec<_> = index
            .redistributable_packs()
            .map(|p| p.id.clone())
            .collect();
        assert!(shippable.contains(&"fixture-open".to_string()));
        assert!(
            !shippable.contains(&"fixture-restricted".to_string()),
            "an ambiguous pack has no established redistribution grant"
        );
        corpus.discard();
    }
}
