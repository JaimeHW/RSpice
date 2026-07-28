//! Model source authority.
//!
//! Where a model came from and what that entitles it to. A PDK model is
//! read-only; a project model can be edited; an unauthenticated source
//! cannot seal a run. The authority is carried with the model rather than
//! inferred from its path.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::{
    DeviceModel, ModelCorrelationState, ModelDefinitionMetadata, ModelQualificationState,
    ModelType, ProcessCorner,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};

/// Ownership and execution policy for a model library's source material.
///
/// External libraries are re-authenticated from the live filesystem for every
/// native run. Project-owned sources are immutable revision records and
/// execute from their retained, digest-checked bytes on every platform. Built-in
/// catalogs have no source deck and therefore contribute no executable model
/// cards through the source resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ModelSourceAuthority {
    #[default]
    BuiltIn,
    External,
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSourcePin {
    pub path: PathBuf,
    pub digest: crate::product::ContentDigest,
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

/// A PDK model library
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ModelLibrary {
    /// Library name (e.g., "tsmc180_1p8v")
    pub name: String,
    /// PDK name
    pub pdk_name: String,
    /// Technology node (e.g., "180nm", "65nm")
    pub technology_node: String,
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
    /// Device models
    pub models: HashMap<String, DeviceModel>,
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
    /// Currently selected corner
    pub selected_corner: Option<String>,
    /// Version string
    pub version: String,
    /// Is expanded in browser
    pub expanded: bool,
}

impl ModelLibrary {
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

    /// Select a corner
    pub fn select_corner(&mut self, name: &str) -> bool {
        if self.corners.contains_key(name) {
            self.selected_corner = Some(name.to_string());
            true
        } else {
            false
        }
    }

    /// Get the selected corner
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
            ModelSourceAuthority::BuiltIn | ModelSourceAuthority::External => None,
        }
    }

    /// Get corner count
    pub fn corner_count(&self) -> usize {
        self.corners.len()
    }
}
