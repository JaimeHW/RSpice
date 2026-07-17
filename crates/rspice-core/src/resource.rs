//! Shared resource-governance types for untrusted and batch workloads.

use std::fmt;
use std::io::Read;
use std::path::Path;

use thiserror::Error;

/// A resource whose configured production limit can be enforced.
///
/// This enum is non-exhaustive so new analysis and frontend resource classes
/// can be added without forcing downstream callers to update exhaustive
/// matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceKind {
    /// Bytes in one root netlist before preprocessing.
    NetlistBytes,
    /// Logical lines in one root netlist before preprocessing.
    NetlistLines,
    /// Bytes materialized after include and library expansion.
    ExpandedSourceBytes,
    /// Decoded source bytes retained from external dependency files.
    DependencySourceBytes,
    /// Bytes accepted from one runtime data or waveform file.
    ExternalDataBytes,
    /// Parsed values or structural items materialized from one runtime data file.
    ExternalDataValues,
    /// Bytes retained by a process-wide registry or shared cache.
    SharedCacheBytes,
    /// Active include/library nesting depth.
    IncludeDepth,
    /// Active subcircuit hierarchy depth.
    HierarchyDepth,
    /// Device records produced by hierarchy flattening.
    FlattenedElements,
    /// Electrical nodes allocated while constructing a circuit.
    CircuitNodes,
    /// Modified nodal-analysis unknowns allocated for a solve.
    MatrixUnknowns,
    /// Points requested from one analysis or sweep.
    AnalysisPoints,
    /// Scalar values retained by one result object.
    ResultValues,
    /// Independent runs requested from a batch analysis.
    BatchRuns,
}

impl ResourceKind {
    /// Stable machine-readable resource name for logs and API responses.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetlistBytes => "netlist_bytes",
            Self::NetlistLines => "netlist_lines",
            Self::ExpandedSourceBytes => "expanded_source_bytes",
            Self::DependencySourceBytes => "dependency_source_bytes",
            Self::ExternalDataBytes => "external_data_bytes",
            Self::ExternalDataValues => "external_data_values",
            Self::SharedCacheBytes => "shared_cache_bytes",
            Self::IncludeDepth => "include_depth",
            Self::HierarchyDepth => "hierarchy_depth",
            Self::FlattenedElements => "flattened_elements",
            Self::CircuitNodes => "circuit_nodes",
            Self::MatrixUnknowns => "matrix_unknowns",
            Self::AnalysisPoints => "analysis_points",
            Self::ResultValues => "result_values",
            Self::BatchRuns => "batch_runs",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A workload exceeded one configured resource limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("{resource} limit exceeded: requested {requested}, limit {limit}")]
pub struct ResourceLimitError {
    /// Stable category of the rejected resource.
    pub resource: ResourceKind,
    /// Requested or observed amount.
    pub requested: usize,
    /// Configured maximum amount.
    pub limit: usize,
}

impl ResourceLimitError {
    /// Check one observed amount against a limit.
    pub(crate) fn ensure(
        resource: ResourceKind,
        requested: usize,
        limit: usize,
    ) -> Result<(), Self> {
        if requested <= limit {
            Ok(())
        } else {
            Err(Self {
                resource,
                requested,
                limit,
            })
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum ResourceReadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),
}

pub(crate) fn read_utf8_file_limited_with_metadata(
    path: &Path,
    resource: ResourceKind,
    limit: usize,
) -> Result<(String, std::fs::Metadata), ResourceReadError> {
    const READ_CHUNK_BYTES: usize = 64 * 1024;

    let mut file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    let metadata_bytes = usize::try_from(metadata.len()).unwrap_or(usize::MAX);
    ResourceLimitError::ensure(resource, metadata_bytes, limit)?;

    let mut bytes = Vec::new();
    bytes.try_reserve_exact(metadata_bytes).map_err(|error| {
        std::io::Error::other(format!(
            "unable to reserve {metadata_bytes} bytes for '{}': {error}",
            path.display()
        ))
    })?;
    let mut chunk = [0_u8; READ_CHUNK_BYTES];
    loop {
        let count = file.read(&mut chunk)?;
        if count == 0 {
            break;
        }
        ResourceLimitError::ensure(resource, bytes.len().saturating_add(count), limit)?;
        if bytes.capacity().saturating_sub(bytes.len()) < count {
            bytes.try_reserve(count).map_err(|error| {
                std::io::Error::other(format!(
                    "unable to grow data buffer for '{}': {error}",
                    path.display()
                ))
            })?;
        }
        bytes.extend_from_slice(&chunk[..count]);
    }

    let contents = String::from_utf8(bytes).map_err(|error| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, error.utf8_error())
    })?;
    Ok((contents, metadata))
}

pub(crate) fn read_utf8_file_limited(
    path: &Path,
    resource: ResourceKind,
    limit: usize,
) -> Result<String, ResourceReadError> {
    read_utf8_file_limited_with_metadata(path, resource, limit).map(|(contents, _)| contents)
}

/// Configurable limits applied at parsing, construction, and analysis
/// boundaries.
///
/// Defaults are deliberately generous for desktop simulation while bounding
/// accidental or hostile amplification. Services with a known memory budget
/// should start from [`Default`] and lower individual fields. Setting a field
/// to `usize::MAX` effectively disables that specific limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct ResourceLimits {
    /// Maximum bytes accepted in one root netlist.
    pub max_netlist_bytes: usize,
    /// Maximum logical lines accepted in one root netlist.
    pub max_netlist_lines: usize,
    /// Maximum bytes materialized after include and library expansion.
    pub max_expanded_source_bytes: usize,
    /// Maximum decoded bytes retained from external dependency files.
    pub max_dependency_source_bytes: usize,
    /// Maximum bytes accepted from one runtime data or waveform file.
    pub max_external_data_bytes: usize,
    /// Maximum parsed values or structural items materialized from one runtime data file.
    pub max_external_data_values: usize,
    /// Maximum bytes retained by one process-wide registry or shared cache.
    pub max_shared_cache_bytes: usize,
    /// Maximum active include/library nesting depth.
    pub max_include_depth: usize,
    /// Maximum active subcircuit hierarchy depth.
    pub max_hierarchy_depth: usize,
    /// Maximum device records after subcircuit flattening.
    pub max_flattened_elements: usize,
    /// Maximum electrical nodes in a constructed circuit.
    pub max_circuit_nodes: usize,
    /// Maximum modified nodal-analysis unknowns in a constructed circuit.
    pub max_matrix_unknowns: usize,
    /// Maximum points generated by one analysis or sweep.
    pub max_analysis_points: usize,
    /// Maximum scalar values retained by one result object.
    pub max_result_values: usize,
    /// Maximum independent runs in one batch analysis.
    pub max_batch_runs: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_netlist_bytes: 64 * 1024 * 1024,
            max_netlist_lines: 2_000_000,
            max_expanded_source_bytes: 256 * 1024 * 1024,
            max_dependency_source_bytes: 256 * 1024 * 1024,
            max_external_data_bytes: 256 * 1024 * 1024,
            max_external_data_values: 25_000_000,
            max_shared_cache_bytes: 512 * 1024 * 1024,
            max_include_depth: crate::netlist::DEFAULT_MAX_INCLUDE_DEPTH,
            max_hierarchy_depth: 100,
            max_flattened_elements: 250_000,
            max_circuit_nodes: 250_000,
            max_matrix_unknowns: 250_000,
            max_analysis_points: 2_000_000,
            max_result_values: 25_000_000,
            max_batch_runs: 10_000,
        }
    }
}

impl ResourceLimits {
    /// Create a policy with no practical per-resource limits.
    pub const fn unlimited() -> Self {
        Self {
            max_netlist_bytes: usize::MAX,
            max_netlist_lines: usize::MAX,
            max_expanded_source_bytes: usize::MAX,
            max_dependency_source_bytes: usize::MAX,
            max_external_data_bytes: usize::MAX,
            max_external_data_values: usize::MAX,
            max_shared_cache_bytes: usize::MAX,
            max_include_depth: usize::MAX,
            max_hierarchy_depth: usize::MAX,
            max_flattened_elements: usize::MAX,
            max_circuit_nodes: usize::MAX,
            max_matrix_unknowns: usize::MAX,
            max_analysis_points: usize::MAX,
            max_result_values: usize::MAX,
            max_batch_runs: usize::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_names_are_stable() {
        assert_eq!(
            ResourceKind::FlattenedElements.as_str(),
            "flattened_elements"
        );
        assert_eq!(
            ResourceKind::DependencySourceBytes.as_str(),
            "dependency_source_bytes"
        );
        assert_eq!(
            ResourceKind::ExternalDataBytes.as_str(),
            "external_data_bytes"
        );
        assert_eq!(
            ResourceKind::ExternalDataValues.as_str(),
            "external_data_values"
        );
        assert_eq!(
            ResourceKind::SharedCacheBytes.as_str(),
            "shared_cache_bytes"
        );
        assert_eq!(ResourceKind::MatrixUnknowns.to_string(), "matrix_unknowns");
    }

    #[test]
    fn unlimited_policy_sets_every_field_to_usize_max() {
        let limits = ResourceLimits::unlimited();
        assert_eq!(limits.max_netlist_bytes, usize::MAX);
        assert_eq!(limits.max_netlist_lines, usize::MAX);
        assert_eq!(limits.max_expanded_source_bytes, usize::MAX);
        assert_eq!(limits.max_dependency_source_bytes, usize::MAX);
        assert_eq!(limits.max_external_data_bytes, usize::MAX);
        assert_eq!(limits.max_external_data_values, usize::MAX);
        assert_eq!(limits.max_shared_cache_bytes, usize::MAX);
        assert_eq!(limits.max_include_depth, usize::MAX);
        assert_eq!(limits.max_hierarchy_depth, usize::MAX);
        assert_eq!(limits.max_flattened_elements, usize::MAX);
        assert_eq!(limits.max_circuit_nodes, usize::MAX);
        assert_eq!(limits.max_matrix_unknowns, usize::MAX);
        assert_eq!(limits.max_analysis_points, usize::MAX);
        assert_eq!(limits.max_result_values, usize::MAX);
        assert_eq!(limits.max_batch_runs, usize::MAX);
    }
}
