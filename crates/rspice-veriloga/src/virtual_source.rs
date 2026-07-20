//! Deterministic, file-system-free Verilog-A source bundles.
//!
//! This module is the transport boundary used by browser workers, retained run
//! snapshots, and any other caller that must compile exact source bytes without
//! ambient file-system access. Bundle paths are portable logical identities;
//! include resolution is sealed to the bundle plus RSpice's versioned standard
//! headers.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::canonical_ir::CanonicalValueType;
use crate::preprocessor::{
    PreprocessedDependency, PreprocessedInclude, SourceDocument, SourceDocumentOrigin,
    SourceProvider, SourceProviderLimits,
};
use crate::{CompileDiagnosticPhase, PreprocessorError};
use crate::{CompileError, CompileResult, CompilerOptions, IntegrationOrder, RuntimeCompileReport};

const CONTRACT_SCHEMA: &str = "rspice.veriloga.virtual-runtime-contract.v1";

/// One exact UTF-8 source document in a virtual bundle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualSourceFile {
    pub logical_path: String,
    pub source: String,
}

impl VirtualSourceFile {
    pub fn new(logical_path: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            logical_path: logical_path.into(),
            source: source.into(),
        }
    }
}

/// Ordered, sealed collection of logical source paths and their exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualSourceBundle {
    root_path: String,
    files: Vec<VirtualSourceFile>,
    case_folded_index: BTreeMap<String, usize>,
}

impl VirtualSourceBundle {
    /// Validate and construct a bundle while preserving caller-provided order.
    pub fn new(
        root_path: impl Into<String>,
        files: impl IntoIterator<Item = VirtualSourceFile>,
    ) -> Result<Self, VirtualSourceError> {
        let root_path = normalize_logical_path(&root_path.into())?;
        let mut normalized_files: Vec<VirtualSourceFile> = Vec::new();
        let mut case_folded_index = BTreeMap::new();

        for mut file in files {
            file.logical_path = normalize_logical_path(&file.logical_path)?;
            let folded = case_fold_path(&file.logical_path);
            if let Some(previous) = case_folded_index.insert(folded, normalized_files.len()) {
                return Err(VirtualSourceError::CaseInsensitiveDuplicate {
                    first: normalized_files[previous].logical_path.clone(),
                    second: file.logical_path,
                });
            }
            normalized_files.push(file);
        }

        if normalized_files.is_empty() {
            return Err(VirtualSourceError::EmptyBundle);
        }
        if !case_folded_index.contains_key(&case_fold_path(&root_path)) {
            return Err(VirtualSourceError::RootNotFound(root_path));
        }

        Ok(Self {
            root_path,
            files: normalized_files,
            case_folded_index,
        })
    }

    /// Convenience constructor for `(logical path, exact source)` pairs.
    pub fn from_sources<P, S>(
        root_path: impl Into<String>,
        sources: impl IntoIterator<Item = (P, S)>,
    ) -> Result<Self, VirtualSourceError>
    where
        P: Into<String>,
        S: Into<String>,
    {
        Self::new(
            root_path,
            sources
                .into_iter()
                .map(|(path, source)| VirtualSourceFile::new(path, source)),
        )
    }

    pub fn root_path(&self) -> &str {
        &self.root_path
    }

    pub fn files(&self) -> &[VirtualSourceFile] {
        &self.files
    }

    fn find(&self, path: &str) -> Option<&VirtualSourceFile> {
        self.case_folded_index
            .get(&case_fold_path(path))
            .and_then(|index| self.files.get(*index))
    }
}

/// Explicit resource contract for virtual preprocessing and compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualCompileLimits {
    pub max_files: usize,
    pub max_path_bytes: usize,
    pub max_file_bytes: usize,
    pub max_total_source_bytes: usize,
    pub max_include_depth: usize,
    pub max_expanded_bytes: usize,
    pub max_module_name_bytes: usize,
}

impl Default for VirtualCompileLimits {
    fn default() -> Self {
        Self {
            max_files: 256,
            max_path_bytes: 1_024,
            max_file_bytes: 4 * 1024 * 1024,
            max_total_source_bytes: 16 * 1024 * 1024,
            max_include_depth: 64,
            max_expanded_bytes: 64 * 1024 * 1024,
            max_module_name_bytes: 1_024,
        }
    }
}

impl VirtualCompileLimits {
    fn validate(self) -> Result<Self, VirtualSourceError> {
        let values = [
            ("max_files", self.max_files),
            ("max_path_bytes", self.max_path_bytes),
            ("max_file_bytes", self.max_file_bytes),
            ("max_total_source_bytes", self.max_total_source_bytes),
            ("max_include_depth", self.max_include_depth),
            ("max_expanded_bytes", self.max_expanded_bytes),
            ("max_module_name_bytes", self.max_module_name_bytes),
        ];
        if let Some((name, _)) = values.into_iter().find(|(_, value)| *value == 0) {
            return Err(VirtualSourceError::InvalidLimit(name));
        }
        Ok(self)
    }
}

/// Exact source dependency captured from the active preprocessor graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualSourceDependency {
    pub logical_path: String,
    pub source: String,
    pub content_digest: String,
    pub origin: SourceDocumentOrigin,
}

/// One include edge actually selected and resolved by the compiler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualSourceInclude {
    pub including_path: String,
    pub requested_path: String,
    /// Zero-based directive index within the including source document.
    pub include_index: usize,
    pub included_path: String,
}

/// One compiler diagnostic mapped back to an exact virtual source document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualSourceDiagnostic {
    pub phase: CompileDiagnosticPhase,
    pub message: String,
    pub logical_path: Option<String>,
    /// Exact source bytes for `logical_path`, retained so callers never map a
    /// diagnostic against a different live editor buffer.
    pub source: Option<String>,
    pub byte_start: Option<usize>,
    pub byte_end: Option<usize>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

/// Failed virtual compilation with source-authenticated editor diagnostics.
#[derive(Debug)]
pub struct VirtualRuntimeCompileFailure {
    pub error: CompileError,
    pub diagnostics: Vec<VirtualSourceDiagnostic>,
}

impl std::fmt::Display for VirtualRuntimeCompileFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(formatter)
    }
}

impl std::error::Error for VirtualRuntimeCompileFailure {}

impl VirtualRuntimeCompileFailure {
    pub(crate) fn unmapped(error: CompileError) -> Self {
        Self {
            diagnostics: vec![VirtualSourceDiagnostic {
                phase: CompileDiagnosticPhase::Input,
                message: error.to_string(),
                logical_path: None,
                source: None,
                byte_start: None,
                byte_end: None,
                line: None,
                column: None,
            }],
            error,
        }
    }

    pub(crate) fn from_preprocessor(
        error: PreprocessorError,
        dependencies: &[PreprocessedDependency],
    ) -> Self {
        let logical_path = error.file.as_ref().map(|path| path_to_logical(path));
        let source = logical_path.as_deref().and_then(|path| {
            dependencies
                .iter()
                .find(|dependency| path_to_logical(&dependency.logical_path) == path)
                .map(|dependency| dependency.source.clone())
        });
        let (byte_start, byte_end, line, column) = source
            .as_deref()
            .and_then(|source| source_line_range(source, error.line))
            .map_or((None, None, None, None), |range| {
                (
                    Some(range.start),
                    Some(range.end),
                    Some(error.line),
                    Some(1),
                )
            });
        let message = error.to_string();
        let compile_error = map_preprocessor_error(error);
        Self {
            error: compile_error,
            diagnostics: vec![VirtualSourceDiagnostic {
                phase: CompileDiagnosticPhase::Input,
                message,
                logical_path,
                source,
                byte_start,
                byte_end,
                line,
                column,
            }],
        }
    }

    pub(crate) fn from_compiler(
        error: CompileError,
        preprocessed: &crate::preprocessor::PreprocessedSource,
        dependencies: &[VirtualSourceDependency],
    ) -> Self {
        let diagnostics = crate::compile_diagnostics(&preprocessed.source, &error)
            .into_iter()
            .map(|diagnostic| {
                let mapped = diagnostic.span.as_ref().and_then(|span| {
                    let offset = usize::try_from(span.byte_start).ok()?;
                    let segment = preprocessed.segment_at(offset)?;
                    let logical_path = path_to_logical(&segment.logical_path);
                    let dependency = dependencies
                        .iter()
                        .find(|dependency| dependency.logical_path == logical_path)?;
                    let line_range = source_line_range(&dependency.source, segment.source_line)?;
                    let expanded_line =
                        &preprocessed.source[segment.expanded_start..segment.expanded_end];
                    let original_line = &dependency.source[line_range.clone()];
                    let exact_line = expanded_line.trim_end_matches(['\r', '\n'])
                        == original_line.trim_end_matches(['\r', '\n']);
                    let local_start = offset.saturating_sub(segment.expanded_start);
                    let requested_end = usize::try_from(span.byte_end).ok()?;
                    let local_end = requested_end.saturating_sub(segment.expanded_start);
                    let (byte_start, byte_end, column) = if exact_line {
                        let start = line_range
                            .start
                            .saturating_add(local_start)
                            .min(line_range.end);
                        let end = line_range
                            .start
                            .saturating_add(local_end)
                            .min(line_range.end);
                        (
                            start,
                            end.max(start),
                            dependency.source[line_range.start..start].chars().count() + 1,
                        )
                    } else {
                        (line_range.start, line_range.end, 1)
                    };
                    Some((
                        logical_path,
                        dependency.source.clone(),
                        byte_start,
                        byte_end,
                        segment.source_line,
                        column,
                    ))
                });
                let (logical_path, source, byte_start, byte_end, line, column) = mapped.map_or(
                    (None, None, None, None, None, None),
                    |(path, source, start, end, line, column)| {
                        (
                            Some(path),
                            Some(source),
                            Some(start),
                            Some(end),
                            Some(line),
                            Some(column),
                        )
                    },
                );
                VirtualSourceDiagnostic {
                    phase: diagnostic.phase,
                    message: diagnostic.message,
                    logical_path,
                    source,
                    byte_start,
                    byte_end,
                    line,
                    column,
                }
            })
            .collect();
        Self { error, diagnostics }
    }
}

fn source_line_range(source: &str, one_based_line: usize) -> Option<std::ops::Range<usize>> {
    if one_based_line == 0 {
        return None;
    }
    let mut start = 0usize;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        let end = start.saturating_add(line.len());
        if index + 1 == one_based_line {
            return Some(start..end);
        }
        start = end;
    }
    None
}

/// Complete output of a sealed virtual runtime compilation.
#[derive(Debug, Clone)]
pub struct VirtualRuntimeCompilation {
    pub runtime: RuntimeCompileReport,
    pub root_path: String,
    pub selected_module: String,
    pub dependency_closure: Vec<VirtualSourceDependency>,
    /// Exact resolved include graph in compiler traversal order.
    pub include_graph: Vec<VirtualSourceInclude>,
    /// Identity of every ordered bundle entry, including unused documents.
    pub source_bundle_identity: String,
    /// Identity of the exact active dependency graph.
    pub dependency_closure_identity: String,
    /// Identity of compiler version/options, selected module, and source graph.
    pub compiler_contract_identity: String,
    /// Identity of the compiler contract plus compiler-derived runtime ABI.
    pub runtime_contract_identity: String,
    pub(crate) source_bundle: VirtualSourceBundle,
    pub(crate) compiler_options: CompilerOptions,
}

impl VirtualRuntimeCompilation {
    /// Revalidate the runtime artifacts and the identities derived from them.
    pub fn validate_integrity(&self) -> Result<(), VirtualSourceError> {
        self.runtime
            .validate_integrity()
            .map_err(|error| VirtualSourceError::ArtifactIntegrity(error.to_string()))?;
        if self.runtime.abi.module_name.as_str() != self.selected_module {
            return Err(VirtualSourceError::ArtifactIntegrity(format!(
                "selected module '{}' does not match compiler-derived ABI module '{}'",
                self.selected_module, self.runtime.abi.module_name
            )));
        }
        let bundle_identity = source_bundle_identity(&self.source_bundle);
        if bundle_identity != self.source_bundle_identity {
            return Err(VirtualSourceError::ArtifactIntegrity(
                "source bundle identity mismatch".into(),
            ));
        }
        if self.source_bundle.root_path() != self.root_path {
            return Err(VirtualSourceError::ArtifactIntegrity(
                "source bundle root does not match compilation root".into(),
            ));
        }
        let root = self.dependency_closure.first().ok_or_else(|| {
            VirtualSourceError::ArtifactIntegrity("dependency closure is empty".into())
        })?;
        if root.logical_path != self.root_path {
            return Err(VirtualSourceError::ArtifactIntegrity(format!(
                "dependency root '{}' does not match compilation root '{}'",
                root.logical_path, self.root_path
            )));
        }
        let mut paths = BTreeMap::new();
        for dependency in &self.dependency_closure {
            let expected_digest = blake3_hex(dependency.source.as_bytes());
            if dependency.content_digest != expected_digest {
                return Err(VirtualSourceError::ArtifactIntegrity(format!(
                    "dependency content digest mismatch for '{}'",
                    dependency.logical_path
                )));
            }
            let folded = case_fold_path(&dependency.logical_path);
            if let Some(first) = paths.insert(folded, dependency.logical_path.as_str()) {
                return Err(VirtualSourceError::ArtifactIntegrity(format!(
                    "dependency paths '{first}' and '{}' collide case-insensitively",
                    dependency.logical_path
                )));
            }
        }
        for edge in &self.include_graph {
            if !paths.contains_key(&case_fold_path(&edge.including_path))
                || !paths.contains_key(&case_fold_path(&edge.included_path))
            {
                return Err(VirtualSourceError::ArtifactIntegrity(format!(
                    "include edge '{}' -> '{}' references a source outside the dependency closure",
                    edge.including_path, edge.included_path
                )));
            }
            if edge.requested_path.trim().is_empty() {
                return Err(VirtualSourceError::ArtifactIntegrity(
                    "include edge has an empty requested path".into(),
                ));
            }
        }
        let closure_identity =
            dependency_closure_identity(&self.dependency_closure, &self.include_graph);
        if closure_identity != self.dependency_closure_identity {
            return Err(VirtualSourceError::ArtifactIntegrity(
                "dependency closure identity mismatch".into(),
            ));
        }
        let compiler_identity = compiler_contract_identity(
            &self.compiler_options,
            &self.root_path,
            &self.selected_module,
            &self.dependency_closure_identity,
        );
        if compiler_identity != self.compiler_contract_identity {
            return Err(VirtualSourceError::ArtifactIntegrity(
                "compiler contract identity mismatch".into(),
            ));
        }
        let runtime_identity =
            runtime_contract_identity(&self.compiler_contract_identity, &self.runtime);
        if runtime_identity != self.runtime_contract_identity {
            return Err(VirtualSourceError::ArtifactIntegrity(
                "runtime contract identity mismatch".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VirtualSourceError {
    #[error("virtual source bundle is empty")]
    EmptyBundle,
    #[error("virtual source root '{0}' is not present in the bundle")]
    RootNotFound(String),
    #[error("logical path must not be empty")]
    EmptyPath,
    #[error("logical path '{0}' contains a NUL byte")]
    NulPath(String),
    #[error("logical path '{0}' is absolute")]
    AbsolutePath(String),
    #[error("logical path '{0}' contains a traversal or non-canonical segment")]
    TraversalPath(String),
    #[error("logical paths '{first}' and '{second}' collide case-insensitively")]
    CaseInsensitiveDuplicate { first: String, second: String },
    #[error("virtual include '{requested}' from '{including}' is invalid: {reason}")]
    InvalidInclude {
        including: String,
        requested: String,
        reason: String,
    },
    #[error("explicit module selection must not be empty")]
    EmptyModuleSelection,
    #[error("module selection contains a NUL byte")]
    NulModuleSelection,
    #[error("resource limit '{0}' must be greater than zero")]
    InvalidLimit(&'static str),
    #[error("virtual bundle contains {actual} files; limit is {limit}")]
    TooManyFiles { actual: usize, limit: usize },
    #[error("logical path '{path}' contains {actual} bytes; limit is {limit}")]
    PathTooLong {
        path: String,
        actual: usize,
        limit: usize,
    },
    #[error("virtual source '{path}' contains {actual} bytes; per-file limit is {limit}")]
    FileTooLarge {
        path: String,
        actual: usize,
        limit: usize,
    },
    #[error("virtual bundle contains {actual} source bytes; limit is {limit}")]
    BundleTooLarge { actual: usize, limit: usize },
    #[error("module selection contains {actual} bytes; limit is {limit}")]
    ModuleNameTooLong { actual: usize, limit: usize },
    #[error("virtual runtime artifact integrity failed: {0}")]
    ArtifactIntegrity(String),
}

pub(crate) struct VirtualBundleProvider<'a> {
    bundle: &'a VirtualSourceBundle,
    limits: VirtualCompileLimits,
}

impl<'a> VirtualBundleProvider<'a> {
    pub(crate) fn new(bundle: &'a VirtualSourceBundle, limits: VirtualCompileLimits) -> Self {
        Self { bundle, limits }
    }

    fn document(&self, path: &str) -> Option<SourceDocument> {
        self.bundle
            .find(path)
            .map(|file| SourceDocument::provided(file.logical_path.clone(), file.source.clone()))
    }
}

impl SourceProvider for VirtualBundleProvider<'_> {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, crate::PreprocessorError> {
        let requested = path_to_logical(requested);
        self.document(&requested).ok_or_else(|| {
            crate::PreprocessorError::new(
                format!("Virtual source root not found: {requested}"),
                Some(PathBuf::from(requested)),
                0,
            )
        })
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        _include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, crate::PreprocessorError> {
        let including = including_file
            .map(path_to_logical)
            .unwrap_or_else(|| self.bundle.root_path.clone());
        let requested = normalize_logical_path(requested).map_err(|error| {
            let wrapped = VirtualSourceError::InvalidInclude {
                including: including.clone(),
                requested: requested.to_owned(),
                reason: error.to_string(),
            };
            crate::PreprocessorError::new(
                wrapped.to_string(),
                including_file.map(Path::to_path_buf),
                0,
            )
        })?;
        if requested.len() > self.limits.max_path_bytes {
            return Err(crate::PreprocessorError::new(
                VirtualSourceError::PathTooLong {
                    path: requested.clone(),
                    actual: requested.len(),
                    limit: self.limits.max_path_bytes,
                }
                .to_string(),
                including_file.map(Path::to_path_buf),
                0,
            ));
        }

        if let Some(parent) = logical_parent(&including) {
            let relative = format!("{parent}/{requested}");
            if relative.len() > self.limits.max_path_bytes {
                return Err(crate::PreprocessorError::new(
                    VirtualSourceError::PathTooLong {
                        path: relative,
                        actual: parent.len() + 1 + requested.len(),
                        limit: self.limits.max_path_bytes,
                    }
                    .to_string(),
                    including_file.map(Path::to_path_buf),
                    0,
                ));
            }
            if let Some(document) = self.document(&relative) {
                return Ok(Some(document));
            }
        }

        Ok(self.document(&requested))
    }

    fn limits(&self) -> SourceProviderLimits {
        // `VirtualCompileLimits` governs project-owned bundle files and bytes.
        // The preprocessor records compiler-owned standard headers in the same
        // exact dependency closure, so reserve their finite maximum separately.
        // Up-front bundle validation still prevents project documents from
        // spending this reservation.
        SourceProviderLimits {
            max_dependencies: self
                .limits
                .max_files
                .saturating_add(crate::stdlib::BUILTIN_INCLUDE_DOCUMENT_COUNT),
            max_total_source_bytes: self
                .limits
                .max_total_source_bytes
                .saturating_add(crate::stdlib::BUILTIN_INCLUDE_SOURCE_BYTES),
            max_include_depth: self.limits.max_include_depth,
            max_expanded_bytes: self.limits.max_expanded_bytes,
        }
    }
}

pub(crate) fn validate_compile_request(
    bundle: &VirtualSourceBundle,
    module_name: &str,
    limits: VirtualCompileLimits,
) -> Result<VirtualCompileLimits, VirtualSourceError> {
    let limits = limits.validate()?;
    if module_name.is_empty() {
        return Err(VirtualSourceError::EmptyModuleSelection);
    }
    if module_name.contains('\0') {
        return Err(VirtualSourceError::NulModuleSelection);
    }
    if module_name.len() > limits.max_module_name_bytes {
        return Err(VirtualSourceError::ModuleNameTooLong {
            actual: module_name.len(),
            limit: limits.max_module_name_bytes,
        });
    }
    if bundle.files.len() > limits.max_files {
        return Err(VirtualSourceError::TooManyFiles {
            actual: bundle.files.len(),
            limit: limits.max_files,
        });
    }

    let mut total_bytes = 0usize;
    for file in &bundle.files {
        if file.logical_path.len() > limits.max_path_bytes {
            return Err(VirtualSourceError::PathTooLong {
                path: file.logical_path.clone(),
                actual: file.logical_path.len(),
                limit: limits.max_path_bytes,
            });
        }
        if file.source.len() > limits.max_file_bytes {
            return Err(VirtualSourceError::FileTooLarge {
                path: file.logical_path.clone(),
                actual: file.source.len(),
                limit: limits.max_file_bytes,
            });
        }
        total_bytes = total_bytes.checked_add(file.source.len()).ok_or(
            VirtualSourceError::BundleTooLarge {
                actual: usize::MAX,
                limit: limits.max_total_source_bytes,
            },
        )?;
    }
    if total_bytes > limits.max_total_source_bytes {
        return Err(VirtualSourceError::BundleTooLarge {
            actual: total_bytes,
            limit: limits.max_total_source_bytes,
        });
    }
    Ok(limits)
}

pub(crate) fn dependencies_from_preprocessor(
    dependencies: Vec<PreprocessedDependency>,
) -> Vec<VirtualSourceDependency> {
    dependencies
        .into_iter()
        .map(|dependency| VirtualSourceDependency {
            logical_path: path_to_logical(&dependency.logical_path),
            content_digest: blake3_hex(dependency.source.as_bytes()),
            source: dependency.source,
            origin: dependency.origin,
        })
        .collect()
}

pub(crate) fn includes_from_preprocessor(
    includes: Vec<PreprocessedInclude>,
) -> Vec<VirtualSourceInclude> {
    includes
        .into_iter()
        .map(|include| VirtualSourceInclude {
            including_path: path_to_logical(&include.including_path),
            requested_path: include.requested_path,
            include_index: include.include_index,
            included_path: path_to_logical(&include.included_path),
        })
        .collect()
}

pub(crate) fn source_bundle_identity(bundle: &VirtualSourceBundle) -> String {
    let mut hasher = ContractHasher::new("source-bundle");
    hasher.field(bundle.root_path.as_bytes());
    hasher.usize(bundle.files.len());
    for file in &bundle.files {
        hasher.field(file.logical_path.as_bytes());
        hasher.field(file.source.as_bytes());
    }
    hasher.finish()
}

pub(crate) fn dependency_closure_identity(
    dependencies: &[VirtualSourceDependency],
    include_graph: &[VirtualSourceInclude],
) -> String {
    let mut hasher = ContractHasher::new("dependency-closure");
    hasher.usize(dependencies.len());
    for dependency in dependencies {
        hasher.field(dependency.logical_path.as_bytes());
        hasher.byte(match dependency.origin {
            SourceDocumentOrigin::Provider => 0,
            SourceDocumentOrigin::BuiltIn => 1,
        });
        hasher.field(dependency.source.as_bytes());
    }
    hasher.usize(include_graph.len());
    for edge in include_graph {
        hasher.field(edge.including_path.as_bytes());
        hasher.field(edge.requested_path.as_bytes());
        hasher.usize(edge.include_index);
        hasher.field(edge.included_path.as_bytes());
    }
    hasher.finish()
}

pub(crate) fn compiler_contract_identity(
    options: &CompilerOptions,
    root_path: &str,
    module_name: &str,
    closure_identity: &str,
) -> String {
    let mut hasher = ContractHasher::new("compiler-contract");
    hasher.field(env!("CARGO_PKG_VERSION").as_bytes());
    hasher.field(root_path.as_bytes());
    hasher.field(module_name.as_bytes());
    hasher.field(closure_identity.as_bytes());
    hasher.byte(u8::from(options.enable_ams));
    hasher.byte(u8::from(options.strict_mode));
    hasher.byte(match options.integration_order {
        IntegrationOrder::First => 1,
        IntegrationOrder::Second => 2,
    });
    hasher.usize(options.defines.len());
    for (name, value) in &options.defines {
        hasher.field(name.as_bytes());
        match value {
            Some(value) => {
                hasher.byte(1);
                hasher.field(value.as_bytes());
            }
            None => hasher.byte(0),
        }
    }
    hasher.usize(options.undefines.len());
    for name in &options.undefines {
        hasher.field(name.as_bytes());
    }
    // include_paths are intentionally excluded. Virtual compilation never
    // consults them, so ambient paths cannot perturb its identity or result.
    hasher.finish()
}

pub(crate) fn runtime_contract_identity(
    compiler_identity: &str,
    runtime: &RuntimeCompileReport,
) -> String {
    let mut hasher = ContractHasher::new("runtime-contract");
    hasher.field(compiler_identity.as_bytes());
    hasher.field(runtime.model.source_digest.as_bytes());
    hasher.field(runtime.canonical_ir.metadata.source_digest.as_bytes());
    hasher.field(runtime.abi.module_name.as_bytes());
    hasher.usize(runtime.abi.analog_ports.len());
    for port in &runtime.abi.analog_ports {
        hasher.field(port.name.as_bytes());
        hasher.field(port.direction.as_bytes());
        hasher.field(port.discipline.as_bytes());
        hasher.optional_text(port.potential_nature.as_deref());
        hasher.optional_text(port.flow_nature.as_deref());
    }
    hasher.usize(runtime.abi.parameters.len());
    for parameter in &runtime.abi.parameters {
        hasher.field(parameter.name.as_bytes());
        hasher.byte(canonical_value_type_tag(parameter.value_type));
        match parameter.default {
            Some(value) => {
                hasher.byte(1);
                hasher.field(&value.to_bits().to_le_bytes());
            }
            None => hasher.byte(0),
        }
        hasher.usize(parameter.aliases.len());
        for alias in &parameter.aliases {
            hasher.field(alias.as_bytes());
        }
    }
    hasher.usize(runtime.abi.noise_source_count);
    hasher.usize(runtime.abi.state_variable_count);
    hasher.usize(runtime.abi.internal_node_count);
    hasher.usize(runtime.abi.branch_unknown_count);
    hasher.usize(runtime.abi.equation_count);
    hasher.finish()
}

fn normalize_logical_path(path: &str) -> Result<String, VirtualSourceError> {
    if path.is_empty() {
        return Err(VirtualSourceError::EmptyPath);
    }
    if path.contains('\0') {
        return Err(VirtualSourceError::NulPath(path.to_owned()));
    }
    if is_absolute_logical_path(path) {
        return Err(VirtualSourceError::AbsolutePath(path.to_owned()));
    }

    let normalized = path.replace('\\', "/");
    if normalized
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err(VirtualSourceError::TraversalPath(path.to_owned()));
    }
    Ok(normalized)
}

fn is_absolute_logical_path(path: &str) -> bool {
    path.starts_with('/')
        || path.starts_with('\\')
        || path
            .as_bytes()
            .get(1)
            .is_some_and(|separator| *separator == b':')
}

fn case_fold_path(path: &str) -> String {
    path.chars().flat_map(char::to_lowercase).collect()
}

fn logical_parent(path: &str) -> Option<&str> {
    path.rsplit_once('/').map(|(parent, _)| parent)
}

fn path_to_logical(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn canonical_value_type_tag(value: CanonicalValueType) -> u8 {
    match value {
        CanonicalValueType::Real => 0,
        CanonicalValueType::Integer => 1,
        CanonicalValueType::String => 2,
        CanonicalValueType::Boolean => 3,
        CanonicalValueType::NatureAccess => 4,
        CanonicalValueType::Void => 5,
        CanonicalValueType::Unknown => 6,
        CanonicalValueType::Error => 7,
    }
}

struct ContractHasher(blake3::Hasher);

impl ContractHasher {
    fn new(domain: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(CONTRACT_SCHEMA.as_bytes());
        hasher.update(&(domain.len() as u64).to_le_bytes());
        hasher.update(domain.as_bytes());
        Self(hasher)
    }

    fn field(&mut self, value: &[u8]) {
        self.0.update(&(value.len() as u64).to_le_bytes());
        self.0.update(value);
    }

    fn byte(&mut self, value: u8) {
        self.0.update(&[value]);
    }

    fn usize(&mut self, value: usize) {
        self.0.update(&(value as u64).to_le_bytes());
    }

    fn optional_text(&mut self, value: Option<&str>) {
        match value {
            Some(value) => {
                self.byte(1);
                self.field(value.as_bytes());
            }
            None => self.byte(0),
        }
    }

    fn finish(self) -> String {
        self.0.finalize().to_hex().to_string()
    }
}

pub(crate) fn map_preprocessor_error(error: crate::PreprocessorError) -> CompileError {
    CompileError::io_error(format!("Preprocessor error: {error}"))
}

pub(crate) fn validate_compilation(compilation: &VirtualRuntimeCompilation) -> CompileResult<()> {
    compilation.validate_integrity().map_err(CompileError::from)
}
