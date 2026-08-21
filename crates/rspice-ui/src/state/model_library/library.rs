//! Model source authority.
//!
//! Where a model came from and what that entitles it to. A PDK model is
//! read-only; a project model can be edited; an unauthenticated source
//! cannot seal a run. The authority is carried with the model rather than
//! inferred from its path.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::{
    DeviceModel, ModelCorrelationState, ModelDefinitionMetadata, ModelQualificationState,
    ModelType, ProcessCorner,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};

/// The comment every materialized model block is sealed under in a deck.
///
/// One spelling, in one place, because three things depend on it agreeing: the
/// corner materializer that writes it into the deck an engine reads, the model
/// execution plan that writes it into a reference deck, and the executed-deck
/// archive that reads it back to say which model sources a completed run was
/// actually given. Two spellings would mean a deck a reader can see and a
/// workspace cannot describe.
///
/// It lives beside the authority it labels rather than beside either writer,
/// so both writers and the reader can name it without any of them reaching
/// across a layer for it.
pub const SEALED_MODEL_SOURCE_MARKER: &str = "* RSpice sealed model source: ";

/// The pack a provenance label names, as `(pack id, release)`.
///
/// The inverse of [`ModelLibrary::provenance_label`], and tested against it. A
/// deck carries the label and nothing else, so a surface routing from a sealed
/// model block back to the release it came from has to read the phrase that
/// was written — which is only safe while one function writes it and this one
/// reads it.
///
/// It reads the first two words after the prefix, so a sectioned label —
/// `pack rspice-opamps 2.1.0 [tt] (nmos)` — resolves to the same release as
/// the whole-root form.
pub fn labelled_pack(label: &str) -> Option<(&str, &str)> {
    let mut words = label.strip_prefix("pack ")?.split_whitespace();
    Some((words.next()?, words.next()?))
}

/// Ownership and execution policy for a model library's source material.
///
/// External libraries are re-authenticated from the live filesystem for every
/// native run. Project-owned sources and retained imports execute from their
/// digest-checked bytes on every platform. A retained import remains
/// read-only and does not acquire project-authoring or qualification metadata
/// merely because its bytes travel with the project. Built-in catalogs have no
/// source deck and therefore contribute no executable model cards through the
/// source resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ModelSourceAuthority {
    #[default]
    BuiltIn,
    External,
    RetainedImport {
        source_id: ModelSourceId,
        digest: ContentDigest,
    },
    ProjectOwned {
        source_id: ModelSourceId,
        revision: ObjectRevision,
        digest: ContentDigest,
    },
}

impl ModelSourceAuthority {
    #[must_use]
    pub const fn is_project_owned(self) -> bool {
        matches!(self, Self::ProjectOwned { .. })
    }

    #[must_use]
    pub const fn has_execution_source(self) -> bool {
        !matches!(self, Self::BuiltIn)
    }

    #[must_use]
    pub const fn uses_retained_bytes(self) -> bool {
        matches!(
            self,
            Self::RetainedImport { .. } | Self::ProjectOwned { .. }
        )
    }

    #[must_use]
    pub const fn retained_root_digest(self) -> Option<ContentDigest> {
        match self {
            Self::RetainedImport { digest, .. } | Self::ProjectOwned { digest, .. } => Some(digest),
            Self::BuiltIn | Self::External => None,
        }
    }
}

/// Host-local absolute identity used only inside the authenticated in-memory
/// source resolver. Project ownership is carried by `ModelSourceId`; this path
/// is regenerated on restore so cross-platform projects never depend on the
/// path syntax of the machine that saved them.
pub(crate) fn project_owned_source_path(source_id: ModelSourceId) -> PathBuf {
    #[cfg(windows)]
    {
        PathBuf::from(format!(
            r"C:\__rspice_project__\model-sources\{source_id}\definition.model"
        ))
    }
    #[cfg(not(windows))]
    {
        PathBuf::from(format!(
            "/__rspice_project__/model-sources/{source_id}/definition.model"
        ))
    }
}

/// One canonical member of an explicitly accepted external model-source
/// closure. Paths are absolute, symlink-resolved identities captured by the
/// library parser; digests identify the exact bytes parsed at refresh time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSourcePin {
    pub path: PathBuf,
    pub digest: crate::product::ContentDigest,
}

/// The published release a project part was taken from.
///
/// The retained source closure beside this pin is what makes a project
/// reproducible; the pin is what makes it attributable. It names the exact
/// signed archive the bytes came from, so a saved design can still say which
/// release supplied a model after that release has been uninstalled,
/// superseded, or withdrawn from the catalog — and a later reader can prove
/// the claim, because `archive_sha256` identifies exactly one published
/// archive forever.
///
/// A project written before packs were distributed carries no pin. That is
/// not a defect and is never repaired by guessing: the field is absent, the
/// retained closure is unchanged, and the project loads and runs identically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackPartPin {
    /// Canonical pack identity, `rspice-<family>`.
    pub pack_id: String,
    /// Semantic version of the release the part was added from.
    pub pack_version: String,
    /// Lowercase hexadecimal SHA-256 of that release's signed archive.
    pub archive_sha256: String,
    /// Part identifier as the signed manifest publishes it.
    pub part_id: String,
}

/// Authenticated bytes retained for one pinned source. Native execution
/// revalidates external libraries against the live filesystem, while
/// project-owned execution and browser execution use these retained bytes.
/// Keeping the bytes in the project also makes recovery checkpoints
/// self-contained.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSourceContent {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
}

/// One authenticated dependency-resolution edge captured at explicit import
/// or refresh. Retaining the owning source and written path literal preserves
/// symlink, filesystem case, and search-precedence behavior without consulting
/// the filesystem during a run.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSourceEdge {
    pub owner: PathBuf,
    pub requested_path: String,
    pub target: PathBuf,
}

/// Whether a persisted source identity is absolute on either the current host
/// or one of the other desktop platforms supported by RSpice.
///
/// Projects are portable metadata. A Windows path restored on Unix (or a Unix
/// path restored on Windows) must remain a valid, repairable binding even
/// though it cannot be opened on the current host.
pub(crate) fn is_portable_absolute_path(path: &Path) -> bool {
    if path.is_absolute() {
        return true;
    }

    let literal = path.to_string_lossy();
    if literal.starts_with('/') {
        return true;
    }

    let normalized = literal.replace('\\', "/");
    let candidate = normalized
        .strip_prefix("//?/")
        .or_else(|| normalized.strip_prefix("//./"))
        .unwrap_or(&normalized);
    let candidate = candidate
        .strip_prefix("UNC/")
        .or_else(|| candidate.strip_prefix("unc/"))
        .unwrap_or(candidate);

    let bytes = candidate.as_bytes();
    (bytes.len() >= 3 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' && bytes[2] == b'/')
        || (normalized.starts_with("//")
            && candidate
                .split('/')
                .filter(|component| !component.is_empty())
                .take(2)
                .count()
                == 2)
}

/// Whether an otherwise-valid absolute identity belongs to another host path
/// syntax and therefore must never be probed by this process.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn is_foreign_platform_absolute_path(path: &Path) -> bool {
    is_portable_absolute_path(path) && !path.is_absolute()
}

/// Find the first pinned source that cannot be reached by following captured
/// dependency edges from this library's root.
pub(crate) fn first_unreachable_source<'a>(
    root: &Path,
    sources: &'a [ModelSourcePin],
    edges: &[ModelSourceEdge],
) -> Option<&'a Path> {
    let mut reachable = HashSet::<PathBuf>::with_capacity(sources.len());
    reachable.insert(root.to_path_buf());

    loop {
        let before = reachable.len();
        for edge in edges {
            if reachable.contains(&edge.owner) {
                reachable.insert(edge.target.clone());
            }
        }
        if reachable.len() == before {
            break;
        }
    }

    sources
        .iter()
        .map(|source| source.path.as_path())
        .find(|source| !reachable.contains(*source))
}

pub(crate) fn subcircuit_interface_key(section: Option<&str>, name: &str) -> String {
    section.map_or_else(
        || name.to_owned(),
        |section| format!("{section}\u{1f}{name}"),
    )
}

/// Exact, source-authenticated public interface of one executable `.SUBCKT`.
///
/// This is intentionally separate from a schematic symbol. Import establishes
/// the ordered terminal and parameter contract; an engineer can then review
/// that contract before creating a governed `X`-instance symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSubcircuitInterface {
    pub name: String,
    pub ports: Vec<String>,
    #[serde(default)]
    pub parameter_defaults: BTreeMap<String, String>,
    pub description: Option<String>,
    pub file_path: Option<PathBuf>,
    pub source_line: Option<usize>,
    pub section: Option<String>,
}

/// A PDK model library
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelLibrary {
    /// Library name (e.g., "tsmc180_1p8v")
    pub name: String,
    /// PDK name
    pub pdk_name: String,
    /// Technology node (e.g., "180nm", "65nm")
    pub technology_node: String,
    /// Stable shipped-corpus identity when this library was attached from a
    /// redistributable model pack. Detection never depends on the original
    /// installation path after the retained snapshot enters a project.
    #[serde(default)]
    pub pack_id: Option<String>,
    /// The signed release this library's parts were taken from, when they came
    /// from a Model Hub pack. Absent on every library that predates pack
    /// distribution and on every library imported from a local file.
    #[serde(default)]
    pub pack_pin: Option<PackPartPin>,
    /// Root execution identity. This is a live filesystem path only for an
    /// external library; project-owned sources use a regenerated virtual path.
    pub root_path: Option<PathBuf>,
    /// Authority that decides whether execution reads the live path or the
    /// retained project bytes. Older direct serializations default to built-in;
    /// project-file migrations classify legacy external roots explicitly.
    #[serde(default)]
    pub source_authority: ModelSourceAuthority,
    /// Canonical, deterministic root-plus-transitive-include closure accepted
    /// by the last successful load or refresh. An empty closure is valid only
    /// for an in-memory library or a legacy external binding that has not yet
    /// been explicitly refreshed; unpinned external bindings are never runnable.
    #[serde(default)]
    pub source_closure: Vec<ModelSourcePin>,
    /// Exact bytes corresponding one-for-one with `source_closure`.
    #[serde(default)]
    pub source_contents: Vec<ModelSourceContent>,
    /// Authenticated resolution graph for the accepted source closure.
    #[serde(default)]
    pub source_edges: Vec<ModelSourceEdge>,
    /// Effective device-model projection for the execution-active section.
    /// Top-level definitions are overlaid by definitions from
    /// `selected_corner`; UI inspection is held separately by the Models view,
    /// while execution independently materializes the same section from the
    /// authenticated source closure.
    pub models: HashMap<String, DeviceModel>,
    /// Complete top-level model namespace retained independently of the active
    /// section so changing corners can rebuild `models` without reparsing or
    /// leaving parameters from the previously selected section behind.
    #[serde(default)]
    pub top_level_models: HashMap<String, DeviceModel>,
    /// Complete model definitions by exact source section name.
    #[serde(default)]
    pub section_models: HashMap<String, HashMap<String, DeviceModel>>,
    /// Exact public interfaces of top-level and section-addressable
    /// subcircuits. Map keys are the exact name at top level and the stable
    /// `section + unit-separator + name` identity inside a `.lib` section.
    #[serde(default)]
    pub subcircuits: HashMap<String, ModelSubcircuitInterface>,
    /// Project-owned typed authoring metadata keyed by exact model name.
    /// External and built-in libraries may omit it; its absence is displayed
    /// honestly rather than synthesized into qualification claims.
    #[serde(default)]
    pub model_definition_metadata: HashMap<String, ModelDefinitionMetadata>,
    /// Versioned qualification and release records keyed by exact model name.
    #[serde(default)]
    pub model_qualification: HashMap<String, ModelQualificationState>,
    /// Versioned measurement-correlation suites and immutable reviewer
    /// evidence keyed by exact model name.
    #[serde(default)]
    pub model_correlation: HashMap<String, ModelCorrelationState>,
    /// Process corners
    pub corners: HashMap<String, ProcessCorner>,
    /// Execution-active corner. The serialized field keeps its historical
    /// name for project compatibility; it is never the Models table's
    /// inspected-row selection.
    pub selected_corner: Option<String>,
    /// Version string
    pub version: String,
    /// Is expanded in browser
    pub expanded: bool,
}

impl ModelLibrary {
    /// What this library is, in the words its own provenance justifies.
    ///
    /// This is the phrase every executed deck seals its model blocks under and
    /// every run receipt attributes a result to, so it has to be the strongest
    /// true statement about where the bytes came from.
    ///
    /// A pinned pack part names its pack and release, because that is the fact
    /// a reader needs to attribute a result and the one a path cannot carry: a
    /// retained bundle is addressed by the digest of its own contents, so its
    /// path names bytes rather than an origin, and a deck sealed under
    /// `/rspice-browser/model-sources/9f2c…` tells nobody which release they
    /// simulated. Everything else says what it is — compiled into RSpice,
    /// authored here, or retained — except a file on this machine, which is
    /// named by its path because the path *is* its identity.
    pub fn provenance_label(&self) -> String {
        if let Some(pin) = self.pack_pin.as_ref() {
            return format!("pack {} {}", pin.pack_id, pin.pack_version);
        }
        let origin = match self.source_authority {
            ModelSourceAuthority::BuiltIn => "built into RSpice",
            ModelSourceAuthority::ProjectOwned { .. } => "authored in this project",
            ModelSourceAuthority::RetainedImport { .. } => "retained into this project",
            ModelSourceAuthority::External => {
                return self
                    .root_path
                    .as_ref()
                    .map_or_else(|| self.name.clone(), |path| path.display().to_string());
            }
        };
        match self.pack_id.as_deref() {
            Some(pack) => format!("{origin} from pack {pack}"),
            None => format!("{origin} · {}", self.name),
        }
    }

    /// Create a new library
    pub fn new(name: impl Into<String>) -> Self {
        let mut lib = Self {
            name: name.into(),
            ..Default::default()
        };
        // Add standard corners by default
        for corner in ProcessCorner::standard_corners() {
            if corner.is_default {
                lib.selected_corner = Some(corner.name.clone());
            }
            lib.corners.insert(corner.name.clone(), corner);
        }
        lib
    }

    /// Set technology
    pub fn with_technology(mut self, pdk: impl Into<String>, node: impl Into<String>) -> Self {
        self.pdk_name = pdk.into();
        self.technology_node = node.into();
        self
    }

    /// Add a model
    pub fn add_model(&mut self, model: DeviceModel) {
        self.top_level_models
            .insert(model.name.clone(), model.clone());
        self.models.insert(model.name.clone(), model);
    }

    /// Get a model by name
    pub fn get_model(&self, name: &str) -> Option<&DeviceModel> {
        self.models.get(name)
    }

    /// Get models by type
    pub fn models_by_type(&self, model_type: ModelType) -> Vec<&DeviceModel> {
        self.models
            .values()
            .filter(|m| m.model_type == model_type)
            .collect()
    }

    /// Activate a corner for the library's executable model projection.
    pub fn activate_corner(&mut self, name: &str) -> bool {
        if self.corners.contains_key(name) {
            self.selected_corner = Some(name.to_string());
            self.refresh_effective_model_projection();
            true
        } else {
            false
        }
    }

    /// Backward-compatible spelling retained for internal callers that have
    /// not yet been migrated to the explicit execution terminology.
    pub fn select_corner(&mut self, name: &str) -> bool {
        self.activate_corner(name)
    }

    /// Rebuild the browsable definition set from the complete section-aware
    /// catalog. Legacy and synthetic libraries that predate the complete
    /// catalog keep their existing projection until they are explicitly
    /// refreshed from source.
    pub fn refresh_effective_model_projection(&mut self) {
        if self.top_level_models.is_empty() && self.section_models.is_empty() {
            return;
        }
        let mut effective = self.top_level_models.clone();
        for active_section in self.active_section_names() {
            let Some(section) = self
                .section_models
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(&active_section))
                .map(|(_, models)| models)
            else {
                continue;
            };
            for model in section.values() {
                if let Some(existing) = effective
                    .keys()
                    .find(|name| name.eq_ignore_ascii_case(&model.name))
                    .cloned()
                {
                    effective.remove(&existing);
                }
                effective.insert(model.name.clone(), model.clone());
            }
        }
        self.models = effective;
    }

    /// Exact source sections active for the execution corner, in
    /// overlay order. The same projection governs primitive models,
    /// subcircuit providers, symbol creation, and sealed execution.
    pub(crate) fn active_section_names(&self) -> Vec<String> {
        let selected = self.selected_corner.as_deref();
        let mut active_sections = selected
            .and_then(|name| self.corners.get(name))
            .map(|corner| {
                corner
                    .effective_section_bindings()
                    .into_iter()
                    .map(|binding| binding.section)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if active_sections.is_empty()
            && let Some(selected) = selected
        {
            active_sections.push(selected.to_owned());
        }
        active_sections
    }

    /// Get the execution-active corner.
    pub fn current_corner(&self) -> Option<&ProcessCorner> {
        self.selected_corner
            .as_ref()
            .and_then(|name| self.corners.get(name))
    }

    /// Get model count
    pub fn model_count(&self) -> usize {
        self.models.len()
    }

    /// Stable source revision for an editable, project-owned definition.
    #[must_use]
    pub const fn project_source_revision(&self) -> Option<ObjectRevision> {
        match self.source_authority {
            ModelSourceAuthority::ProjectOwned { revision, .. } => Some(revision),
            ModelSourceAuthority::BuiltIn
            | ModelSourceAuthority::External
            | ModelSourceAuthority::RetainedImport { .. } => None,
        }
    }

    /// Get corner count
    pub fn corner_count(&self) -> usize {
        self.corners.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both desktop path syntaxes are absolute identities on every host.
    ///
    /// The two cases cover each other: on Windows the Unix spelling is the one
    /// `Path::is_absolute` rejects, and on macOS and Linux it is the Windows
    /// spelling. Whichever host runs this, one of them is exercising the
    /// syntax-only branch — so a regression that quietly reverts to asking the
    /// running host fails here rather than on one runner in CI.
    #[test]
    fn either_desktop_path_syntax_is_absolute_on_any_host() {
        for absolute in [
            r"C:\models\cmos.lib",
            "C:/models/cmos.lib",
            r"\\?\C:\models\cmos.lib",
            r"\\server\share\models\cmos.lib",
            "/models/cmos.lib",
        ] {
            assert!(
                is_portable_absolute_path(Path::new(absolute)),
                "{absolute} is an absolute identity"
            );
        }

        for relative in ["models/cmos.lib", r"models\cmos.lib", "cmos.lib", ""] {
            assert!(
                !is_portable_absolute_path(Path::new(relative)),
                "{relative} is not an absolute identity"
            );
        }
    }

    /// Every origin says what it is, and a pinned pack part says which release.
    ///
    /// The retained case is the one that matters: its root path is the digest
    /// of the bundle's own contents, so sealing a deck under that path names
    /// bytes and tells a reader nothing about which release they simulated.
    #[test]
    fn a_library_is_labelled_as_what_its_provenance_says_it_is() {
        let mut library = ModelLibrary::new("proving");
        assert_eq!(
            library.provenance_label(),
            "built into RSpice · proving",
            "a definition with no other origin is the one this build carries"
        );

        library.source_authority = ModelSourceAuthority::External;
        library.root_path = Some(PathBuf::from("/models/cmos.lib"));
        assert_eq!(
            library.provenance_label(),
            "/models/cmos.lib",
            "a file on this machine is named by the path that is its identity"
        );

        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id: crate::product::ModelSourceId::new(),
            digest: crate::product::ContentDigest::from_bytes([7; 32]),
        };
        library.root_path = Some(PathBuf::from(
            "/rspice-browser/model-sources/9f2c/models/proving.lib",
        ));
        assert_eq!(
            library.provenance_label(),
            "retained into this project · proving"
        );

        library.pack_pin = Some(PackPartPin {
            pack_id: "rspice-opamps".to_owned(),
            pack_version: "2.1.0".to_owned(),
            archive_sha256: "9f2c".repeat(16),
            part_id: "OPA2340".to_owned(),
        });
        assert_eq!(
            library.provenance_label(),
            "pack rspice-opamps 2.1.0",
            "and a pinned part names the release a result can be attributed to"
        );
    }

    /// The label a deck carries is the only route back to the release, so the
    /// reader and the writer are held to each other here.
    #[test]
    fn a_sealed_label_reads_back_as_the_release_that_wrote_it() {
        let mut library = ModelLibrary::new("proving");
        library.pack_pin = Some(PackPartPin {
            pack_id: "rspice-opamps".to_owned(),
            pack_version: "2.1.0".to_owned(),
            archive_sha256: "9f2c".repeat(16),
            part_id: "OPA2340".to_owned(),
        });
        let label = library.provenance_label();
        assert_eq!(labelled_pack(&label), Some(("rspice-opamps", "2.1.0")));
        assert_eq!(
            labelled_pack(&format!("{label} [tt] (nmos + pmos)")),
            Some(("rspice-opamps", "2.1.0")),
            "a sectioned label names the same release"
        );

        library.pack_pin = None;
        library.source_authority = ModelSourceAuthority::External;
        library.root_path = Some(PathBuf::from("/models/cmos.lib"));
        assert_eq!(
            labelled_pack(&library.provenance_label()),
            None,
            "and a label that names no pack is not made to name one"
        );
        assert_eq!(labelled_pack("pack rspice-opamps"), None);
    }
}
