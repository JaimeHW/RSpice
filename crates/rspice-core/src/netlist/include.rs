//! .INCLUDE and .LIB directive processing
//!
//! Handles file inclusion for SPICE netlists, supporting:
//! - `.INCLUDE "filename"` - Include entire file contents
//! - `.INC "filename"` / `.INCL "filename"` - HSPICE/Xyce include aliases
//! - `.LIB "filename" [section]` - Include library section
//!
//! Features:
//! - Relative path resolution from parent file
//! - Circular inclusion detection
//! - Library section extraction
//! - Case-insensitive matching for Windows compatibility

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{
    NetlistSourceLocation, ParseError, ParseWithAbortError, ensure_parse_not_aborted,
    finish_non_aborting_parse, map_abort_parse_error, poll_parse_abort, poll_parse_text,
    read_file_with_encoding_with_abort,
};
use crate::abort_signal::{AbortSignal, NoAbort};

/// One source-aware item in an include-expanded netlist.
///
/// Source boundaries are out-of-band so they cannot collide with authored
/// SPICE directives or perturb expanded-text line numbering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExpandedSourceItem {
    EnterSource {
        path: PathBuf,
    },
    Line {
        text: String,
        origin: NetlistSourceLocation,
    },
    EndCard {
        origin: NetlistSourceLocation,
    },
    ExitSource {
        path: PathBuf,
        eof_line: usize,
    },
}

/// Include-expanded source retaining exact ownership of every physical line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ExpandedSource {
    pub(crate) items: Vec<ExpandedSourceItem>,
}

impl ExpandedSource {
    pub(crate) fn render(&self) -> String {
        let mut output = String::new();
        for item in &self.items {
            if let ExpandedSourceItem::Line { text, .. } = item {
                output.push_str(text);
                output.push('\n');
            }
        }
        output
    }

    fn append(&mut self, other: Self) {
        self.items.extend(other.items);
    }
}

/// Production include-depth limit shared by library discovery and executable
/// netlist expansion. Foundry PDKs commonly have substantially deeper include
/// trees than small hand-written decks.
pub const DEFAULT_MAX_INCLUDE_DEPTH: usize = 64;

/// An immutable, in-memory set of canonical source files.
///
/// A sealed bundle is deliberately content-only: resolving an include checks
/// this map and never consults the filesystem. Callers are responsible for
/// reading and authenticating every source before constructing the bundle.
#[derive(Clone, Default)]
pub struct SealedSourceBundle {
    sources: BTreeMap<PathBuf, Arc<str>>,
    edges: BTreeMap<(PathBuf, String), PathBuf>,
    #[cfg(windows)]
    windows_identities: BTreeMap<String, PathBuf>,
}

impl std::fmt::Debug for SealedSourceBundle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sources = self
            .sources
            .iter()
            .map(|(path, content)| (path, content.len()))
            .collect::<Vec<_>>();
        formatter
            .debug_struct("SealedSourceBundle")
            .field("sources", &sources)
            .field("edges", &self.edges)
            .finish()
    }
}

/// One authenticated include-resolution decision captured while the source
/// closure was imported. Runtime sealed resolution follows these edges instead
/// of reconstructing host filesystem semantics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SealedSourceEdge {
    pub owner: PathBuf,
    pub requested_path: String,
    pub target: PathBuf,
}

impl SealedSourceBundle {
    /// Construct a bundle from canonical absolute paths and their exact UTF-8
    /// contents. Duplicate canonical identities are rejected instead of being
    /// silently replaced.
    pub fn try_new(
        sources: impl IntoIterator<Item = (PathBuf, String)>,
    ) -> Result<Self, ParseError> {
        Self::try_new_with_edges(sources, std::iter::empty())
    }

    /// Construct a bundle with authenticated include-resolution edges.
    pub fn try_new_with_edges(
        sources: impl IntoIterator<Item = (PathBuf, String)>,
        edges: impl IntoIterator<Item = SealedSourceEdge>,
    ) -> Result<Self, ParseError> {
        let mut bundle = Self::default();
        for (path, content) in sources {
            if !path.is_absolute() {
                return Err(ParseError::Syntax {
                    line: 0,
                    message: format!(
                        "Sealed source path must be an absolute canonical path: {}",
                        path.display()
                    ),
                });
            }
            let path = lexically_normalize_path(&path);
            if bundle.sources.contains_key(&path) {
                return Err(ParseError::Syntax {
                    line: 0,
                    message: format!("Duplicate sealed source path: {}", path.display()),
                });
            }

            #[cfg(windows)]
            {
                let identity = windows_path_identity(&path);
                if let Some(existing) = bundle.windows_identities.get(&identity) {
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "Duplicate sealed source identity: {} and {}",
                            existing.display(),
                            path.display()
                        ),
                    });
                }
                bundle.windows_identities.insert(identity, path.clone());
            }

            bundle.sources.insert(path, Arc::from(content));
        }

        for edge in edges {
            let owner = bundle
                .canonical_member(&edge.owner)
                .ok_or_else(|| ParseError::Syntax {
                    line: 0,
                    message: format!(
                        "Sealed dependency edge owner is not a bundle member: {}",
                        edge.owner.display()
                    ),
                })?;
            let target =
                bundle
                    .canonical_member(&edge.target)
                    .ok_or_else(|| ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "Sealed dependency edge target is not a bundle member: {}",
                            edge.target.display()
                        ),
                    })?;
            let requested_path = normalize_source_path_literal(&edge.requested_path)?;
            let key = (owner.clone(), requested_path.clone());
            if let Some(existing) = bundle.edges.get(&key) {
                if existing != &target {
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "Conflicting sealed dependency edges for '{}' in '{}': '{}' and '{}'",
                            requested_path,
                            owner.display(),
                            existing.display(),
                            target.display()
                        ),
                    });
                }
                continue;
            }
            bundle.edges.insert(key, target);
        }
        Ok(bundle)
    }

    /// Number of authenticated source members in this bundle.
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Whether the bundle contains no source members.
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    fn canonical_member(&self, candidate: &Path) -> Option<PathBuf> {
        let candidate = lexically_normalize_path(candidate);
        if let Some((path, _)) = self.sources.get_key_value(&candidate) {
            return Some(path.clone());
        }

        #[cfg(windows)]
        {
            self.windows_identities
                .get(&windows_path_identity(&candidate))
                .cloned()
        }

        #[cfg(not(windows))]
        None
    }

    fn content(&self, path: &Path) -> Option<Arc<str>> {
        let canonical = self.canonical_member(path)?;
        self.sources.get(&canonical).cloned()
    }

    fn resolve_edge(&self, owner: &Path, requested_path: &str) -> Result<PathBuf, ParseError> {
        let owner = self
            .canonical_member(owner)
            .ok_or_else(|| ParseError::Syntax {
                line: 0,
                message: format!(
                    "Include owner is not present in the verified sealed source bundle: {}",
                    owner.display()
                ),
            })?;
        let requested_path = normalize_source_path_literal(requested_path)?;
        self.edges
            .get(&(owner.clone(), requested_path.clone()))
            .cloned()
            .ok_or_else(|| ParseError::Syntax {
                line: 0,
                message: format!(
                    "Dependency '{}' referenced by '{}' is not present in the authenticated sealed resolution graph",
                    requested_path,
                    owner.display()
                ),
            })
    }
}

//=============================================================================
// Include Processor
//=============================================================================

/// Processes .INCLUDE and .LIB directives
///
/// Maintains state to prevent infinite recursion from circular includes
/// and resolves relative paths based on the including file's location.
#[derive(Debug)]
pub struct IncludeProcessor {
    /// Base directory for resolving relative paths
    base_dir: PathBuf,
    /// Execution directory used as Xyce's final relative include fallback
    execution_dir: PathBuf,
    /// Currently active include/lib stack entries used for recursion detection
    active_includes: HashSet<IncludeKey>,
    /// Additional library search paths
    lib_paths: Vec<PathBuf>,
    /// Optional authenticated in-memory resolver. When present, every source
    /// lookup is confined to this bundle and filesystem fallback is forbidden.
    sealed_sources: Option<SealedSourceBundle>,
    /// Maximum include depth to prevent stack overflow
    max_depth: usize,
    /// Current include depth
    current_depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IncludeKey {
    path: PathBuf,
    section: Option<String>,
}

#[derive(Debug)]
struct InlineLibFrame {
    name: String,
    opened_at_line: usize,
    selected: bool,
}

impl IncludeKey {
    fn new(path: PathBuf, section: Option<&str>) -> Self {
        Self {
            path,
            section: section.map(|name| name.to_ascii_uppercase()),
        }
    }

    fn describe(&self) -> String {
        match &self.section {
            Some(section) => format!("{} [{}]", self.path.display(), section),
            None => self.path.display().to_string(),
        }
    }
}

impl IncludeProcessor {
    /// Create a new include processor
    ///
    /// # Arguments
    /// * `base_path` - Path to the main netlist file (or its directory)
    pub fn new(base_path: &Path) -> Self {
        Self::new_with_execution_dir(base_path, None)
    }

    /// Create a new include processor with an explicit execution directory.
    ///
    /// Xyce resolves nested includes relative to the including file first, then
    /// the top-level netlist directory, then the process execution directory.
    /// Most callers use the top-level directory as the execution directory, but
    /// upstream wrapper tests can intentionally run a deck from another
    /// directory.
    pub fn new_with_execution_dir(base_path: &Path, execution_dir: Option<&Path>) -> Self {
        let base_dir = if base_path.is_file() {
            base_path.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            base_path.to_path_buf()
        };
        let execution_dir = execution_dir
            .map(Path::to_path_buf)
            .unwrap_or_else(|| base_dir.clone());

        Self {
            base_dir,
            execution_dir,
            active_includes: HashSet::new(),
            lib_paths: Vec::new(),
            sealed_sources: None,
            max_depth: DEFAULT_MAX_INCLUDE_DEPTH,
            current_depth: 0,
        }
    }

    /// Create a processor that can resolve sources only from an authenticated
    /// in-memory bundle. No path lookup or source read performed by this
    /// processor accesses the filesystem.
    pub fn new_sealed(base_path: &Path, sources: SealedSourceBundle) -> Self {
        let mut processor = Self::new(base_path);
        processor.sealed_sources = Some(sources);
        processor
    }

    /// Add a library search path
    pub fn add_lib_path(&mut self, path: PathBuf) {
        if !self.lib_paths.contains(&path) {
            self.lib_paths.push(path);
        }
    }

    /// Process a .INCLUDE directive
    ///
    /// Reads and returns the entire contents of the specified file.
    ///
    /// # Arguments
    /// * `filename` - Path to include (relative to base_dir or absolute)
    ///
    /// # Returns
    /// The file contents, or an error if the file cannot be read
    pub fn process_include(&mut self, filename: &str) -> Result<String, ParseError> {
        finish_non_aborting_parse(self.process_include_with_abort(filename, &NoAbort))
    }

    /// Process an include directive with cooperative cancellation.
    pub fn process_include_with_abort(
        &mut self,
        filename: &str,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        let owner = self.base_dir.join("__rspice_include_owner__");
        self.process_include_from_with_abort(&owner, filename, abort)
    }

    /// Process a .LIB directive
    ///
    /// Reads a library file and extracts the specified section.
    /// If no section is specified, returns the entire file.
    ///
    /// # Arguments
    /// * `filename` - Path to library file
    /// * `section` - Optional section name to extract
    ///
    /// # Returns
    /// The section contents, or an error if not found
    pub fn process_lib(
        &mut self,
        filename: &str,
        section: Option<&str>,
    ) -> Result<String, ParseError> {
        finish_non_aborting_parse(self.process_lib_with_abort(filename, section, &NoAbort))
    }

    /// Process a library directive with cooperative cancellation.
    pub fn process_lib_with_abort(
        &mut self,
        filename: &str,
        section: Option<&str>,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        let owner = self.base_dir.join("__rspice_include_owner__");
        self.process_lib_from_with_abort(&owner, filename, section, abort)
    }

    /// Materialize one listed root source, optionally selecting an inline
    /// `.lib` section, using only the sealed bundle.
    pub fn process_sealed_root(
        &mut self,
        root_path: &Path,
        section: Option<&str>,
    ) -> Result<String, ParseError> {
        finish_non_aborting_parse(self.process_sealed_root_with_abort(root_path, section, &NoAbort))
    }

    /// Materialize one sealed root with cooperative cancellation.
    pub fn process_sealed_root_with_abort(
        &mut self,
        root_path: &Path,
        section: Option<&str>,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let sources = self
            .sealed_sources
            .as_ref()
            .ok_or_else(|| ParseError::Syntax {
                line: 0,
                message: "process_sealed_root requires a sealed source bundle".to_owned(),
            })
            .map_err(ParseWithAbortError::from)?;
        let canonical = sources
            .canonical_member(root_path)
            .ok_or_else(|| ParseError::Syntax {
                line: 0,
                message: format!(
                    "Model root is not present in the verified source bundle: {}",
                    root_path.display()
                ),
            })
            .map_err(ParseWithAbortError::from)?;
        let key = IncludeKey::new(canonical.clone(), section);
        self.enter_include(&key)
            .map_err(ParseWithAbortError::from)?;

        let result = (|| {
            let content =
                self.read_source_with_abort(&canonical, &canonical.display().to_string(), abort)?;
            let expanded = self.expand_content_from_mapped_with_abort(
                &content,
                &canonical,
                section,
                section.is_some(),
                true,
                abort,
            )?;
            let rendered = expanded.render();
            for (line_index, line) in rendered.lines().enumerate() {
                poll_parse_abort(abort, line_index)?;
                if parse_include_directive(line).is_some() || parse_lib_directive(line).is_some() {
                    return Err(ParseWithAbortError::from(ParseError::Syntax {
                        line: line_index + 1,
                        message: format!(
                            "{}: sealed model materialization left an unresolved include/library directive: {}",
                            canonical.display(),
                            line.trim()
                        ),
                    }));
                }
            }
            ensure_parse_not_aborted(abort)?;
            Ok(rendered)
        })();

        self.leave_include(&key);
        result
    }

    /// Recursively expand `.INCLUDE` and `.LIB` directives in raw content.
    pub fn expand_content(
        &mut self,
        content: &str,
        current_path: &Path,
    ) -> Result<String, ParseError> {
        finish_non_aborting_parse(self.expand_content_with_abort(content, current_path, &NoAbort))
    }

    /// Recursively expand `.INCLUDE` and `.LIB` directives with cooperative
    /// cancellation.
    pub fn expand_content_with_abort(
        &mut self,
        content: &str,
        current_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        self.expand_content_mapped_with_abort(content, current_path, abort)
            .map(|expanded| expanded.render())
    }

    /// Recursively expand includes while preserving exact source ownership.
    pub(crate) fn expand_content_mapped_with_abort(
        &mut self,
        content: &str,
        current_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<ExpandedSource, ParseWithAbortError> {
        self.expand_content_from_mapped_with_abort(content, current_path, None, false, false, abort)
    }

    /// Resolve a filename to an absolute path
    pub(crate) fn resolve_path_from_with_abort(
        &self,
        owner_path: &Path,
        filename: &str,
        abort: &dyn AbortSignal,
    ) -> Result<PathBuf, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        // Remove quotes if present
        let clean_name = filename.trim_matches('"').trim_matches('\'');
        let path = Path::new(clean_name);

        if let Some(sources) = &self.sealed_sources {
            return sources
                .resolve_edge(owner_path, clean_name)
                .map_err(ParseWithAbortError::from);
        }

        let base_dir = owner_path.parent().unwrap_or(Path::new("."));

        if let Some(relative_to_execution_dir) = windows_drive_relative_suffix(clean_name) {
            let relative = spice_relative_path(relative_to_execution_dir);
            let candidate = self.base_dir.join(relative);
            if candidate.exists() {
                return Ok(candidate);
            }
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Include file not found: {} (searched {})",
                    clean_name,
                    self.base_dir.display()
                ),
            }
            .into());
        }

        // If absolute, use as-is
        if path.is_absolute() {
            if path.exists() {
                return Ok(path.to_path_buf());
            }
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("File not found: {}", clean_name),
            }
            .into());
        }

        // Try relative to base directory first
        let relative_path = spice_relative_path(clean_name);
        let relative = base_dir.join(&relative_path);
        if relative.exists() {
            return Ok(relative);
        }

        // Xyce resolves nested include/lib paths relative to the including file
        // first, then falls back to the top-level netlist directory.
        let top_level_relative = self.base_dir.join(&relative_path);
        if top_level_relative.exists() {
            return Ok(top_level_relative);
        }

        let execution_relative = self.execution_dir.join(&relative_path);
        if execution_relative.exists() {
            return Ok(execution_relative);
        }

        // Try library search paths
        for (index, lib_path) in self.lib_paths.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            let candidate = lib_path.join(&relative_path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        // Try common library locations
        let common_paths = ["lib", "models", "../lib", "../models"];

        for (index, common) in common_paths.into_iter().enumerate() {
            poll_parse_abort(abort, index)?;
            let candidate = base_dir.join(common).join(&relative_path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(ParseError::Syntax {
            line: 0,
            message: format!(
                "Include file not found: {} (searched {})",
                clean_name,
                base_dir.display()
            ),
        }
        .into())
    }

    /// Resolve a dependency using Xyce's execution-directory-only rule.
    ///
    /// `.INITCOND FILE` is opened directly by Xyce and does not inherit the
    /// including source's directory or model-library search paths. In sealed
    /// mode the top-level source identity owns an authenticated edge to the
    /// resource that represents the execution-directory decision.
    pub(crate) fn resolve_execution_path_with_abort(
        &self,
        top_level_path: &Path,
        filename: &str,
        abort: &dyn AbortSignal,
    ) -> Result<PathBuf, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let clean_name = filename.trim_matches('"').trim_matches('\'');
        if let Some(sources) = &self.sealed_sources {
            return sources
                .resolve_edge(top_level_path, clean_name)
                .map_err(ParseWithAbortError::from);
        }

        let path = source_path_literal_to_host_path(clean_name);
        let candidate = if path.is_absolute() {
            path
        } else if let Some(relative) = windows_drive_relative_suffix(clean_name) {
            self.execution_dir.join(spice_relative_path(relative))
        } else {
            self.execution_dir.join(path)
        };
        if candidate.exists() {
            Ok(candidate)
        } else {
            Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Execution-directory dependency not found: {} (searched {})",
                    clean_name,
                    self.execution_dir.display()
                ),
            }
            .into())
        }
    }

    pub(crate) fn read_source_with_abort(
        &self,
        path: &Path,
        requested_name: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<str>, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        if let Some(sources) = &self.sealed_sources {
            let content = sources
                .content(path)
                .ok_or_else(|| ParseError::Syntax {
                    line: 0,
                    message: format!(
                        "Dependency is not present in the verified sealed source bundle: '{}'",
                        path.display()
                    ),
                })
                .map_err(ParseWithAbortError::from)?;
            ensure_parse_not_aborted(abort)?;
            return Ok(content);
        }

        read_file_with_encoding_with_abort(path, abort)
            .map(Arc::from)
            .map_err(|error| {
                map_abort_parse_error(error, |error| ParseError::Syntax {
                    line: 0,
                    message: format!("Failed to include '{}': {error}", requested_name),
                })
            })
    }

    fn process_include_from_with_abort(
        &mut self,
        owner_path: &Path,
        filename: &str,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        self.process_include_from_with_selection_and_abort(owner_path, filename, None, abort)
    }

    fn process_include_from_with_selection_and_abort(
        &mut self,
        owner_path: &Path,
        filename: &str,
        selected_section: Option<&str>,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        self.process_include_from_with_selection_mapped_and_abort(
            owner_path,
            filename,
            selected_section,
            abort,
        )
        .map(|expanded| expanded.render())
    }

    fn process_include_from_with_selection_mapped_and_abort(
        &mut self,
        owner_path: &Path,
        filename: &str,
        selected_section: Option<&str>,
        abort: &dyn AbortSignal,
    ) -> Result<ExpandedSource, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let path = self.resolve_path_from_with_abort(owner_path, filename, abort)?;
        let canonical = if self.sealed_sources.is_some() {
            path.clone()
        } else {
            path.canonicalize().unwrap_or_else(|_| path.clone())
        };
        let key = IncludeKey::new(canonical.clone(), None);
        self.enter_include(&key)
            .map_err(ParseWithAbortError::from)?;

        let result = (|| {
            let content = self.read_source_with_abort(&path, filename, abort)?;
            self.expand_content_from_mapped_with_abort(
                &content,
                &canonical,
                selected_section,
                false,
                true,
                abort,
            )
        })();

        self.leave_include(&key);
        result
    }

    fn process_lib_from_with_abort(
        &mut self,
        owner_path: &Path,
        filename: &str,
        section: Option<&str>,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        self.process_lib_from_mapped_with_abort(owner_path, filename, section, abort)
            .map(|expanded| expanded.render())
    }

    fn process_lib_from_mapped_with_abort(
        &mut self,
        owner_path: &Path,
        filename: &str,
        section: Option<&str>,
        abort: &dyn AbortSignal,
    ) -> Result<ExpandedSource, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let path = self.resolve_path_from_with_abort(owner_path, filename, abort)?;
        let canonical = if self.sealed_sources.is_some() {
            path.clone()
        } else {
            path.canonicalize().unwrap_or_else(|_| path.clone())
        };
        let key = IncludeKey::new(canonical.clone(), section);
        self.enter_include(&key)
            .map_err(ParseWithAbortError::from)?;

        let result = (|| {
            let content = self.read_source_with_abort(&path, filename, abort)?;
            self.expand_content_from_mapped_with_abort(
                &content,
                &canonical,
                section,
                section.is_some(),
                true,
                abort,
            )
        })();

        self.leave_include(&key);
        result
    }

    fn expand_content_from_mapped_with_abort(
        &mut self,
        content: &str,
        current_path: &Path,
        selected_section: Option<&str>,
        require_selected_section: bool,
        strip_end_cards: bool,
        abort: &dyn AbortSignal,
    ) -> Result<ExpandedSource, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let mut result = ExpandedSource::default();
        result.items.push(ExpandedSourceItem::EnterSource {
            path: current_path.to_path_buf(),
        });
        let mut inline_sections: Vec<InlineLibFrame> = Vec::new();
        let mut selected_section_found = selected_section.is_none();
        let source_line_count = content.lines().count();

        for (line_index, line) in content.lines().enumerate() {
            poll_parse_abort(abort, line_index)?;
            poll_parse_text(abort, line)?;
            let line_number = line_index + 1;
            let trimmed = line.trim();

            if split_directive(trimmed)
                .is_some_and(|(directive, _)| directive.eq_ignore_ascii_case(".lib"))
            {
                let Some((filename, section)) = parse_lib_directive(trimmed) else {
                    return Err(ParseError::Syntax {
                        line: line_number,
                        message: format!(
                            "{}:{}: malformed .lib directive",
                            current_path.display(),
                            line_number
                        ),
                    }
                    .into());
                };
                if section.is_none() {
                    let parent_selected = inline_sections
                        .last()
                        .map(|frame| frame.selected)
                        .unwrap_or(true);
                    let selected = parent_selected
                        && selected_section
                            .is_some_and(|wanted| filename.eq_ignore_ascii_case(wanted));
                    selected_section_found |= selected;
                    inline_sections.push(InlineLibFrame {
                        name: filename,
                        opened_at_line: line_number,
                        selected,
                    });
                    continue;
                }
                if inline_sections.last().is_some_and(|frame| !frame.selected) {
                    continue;
                }

                let included = self
                    .process_lib_from_mapped_with_abort(
                        current_path,
                        &filename,
                        section.as_deref(),
                        abort,
                    )
                    .map_err(|error| {
                        map_abort_parse_error(error, |error| {
                            include_error_at(error, current_path, line_number, ".lib")
                        })
                    })?;
                result.append(included);
                continue;
            }

            if let Some(end_name) = parse_endl_directive(trimmed, line_number)? {
                let Some(open_frame) = inline_sections.last() else {
                    return Err(ParseError::Syntax {
                        line: line_number,
                        message: ".ENDL encountered without an open .LIB section".to_string(),
                    }
                    .into());
                };
                if let Some(end_name) = end_name
                    && !end_name.eq_ignore_ascii_case(&open_frame.name)
                {
                    return Err(ParseError::Syntax {
                        line: line_number,
                        message: format!(
                            ".ENDL section '{end_name}' does not match open .LIB section '{}'",
                            open_frame.name
                        ),
                    }
                    .into());
                }
                inline_sections.pop();
                continue;
            }

            if inline_sections.last().is_some_and(|frame| !frame.selected) {
                continue;
            }

            if strip_end_cards && trimmed.eq_ignore_ascii_case(".end") {
                result.items.push(ExpandedSourceItem::EndCard {
                    origin: NetlistSourceLocation::in_file(current_path, line_number),
                });
                break;
            }

            if split_directive(trimmed).is_some_and(|(directive, _)| {
                matches_ignore_ascii_case(directive, &[".include", ".inc", ".incl"])
            }) {
                let Some(filename) = parse_include_directive(trimmed) else {
                    return Err(ParseError::Syntax {
                        line: line_number,
                        message: format!(
                            "{}:{}: malformed include directive",
                            current_path.display(),
                            line_number
                        ),
                    }
                    .into());
                };
                // SPEF files are parasitic data, not SPICE text: route to
                // the back-annotation pass (`.spef_include`) with the path
                // resolved here, where include search rules apply.
                if filename.to_ascii_lowercase().ends_with(".spef") {
                    if self.sealed_sources.is_some() {
                        return Err(ParseError::Syntax {
                            line: line_number,
                            message: format!(
                                "{}:{}: sealed model materialization cannot defer SPEF dependency '{}'; filesystem-backed parasitic reads are forbidden",
                                current_path.display(),
                                line_number,
                                filename
                            ),
                        }
                        .into());
                    }
                    let path = self.resolve_path_from_with_abort(current_path, &filename, abort)?;
                    let normalized = path.display().to_string().replace('\\', "/");
                    result.items.push(ExpandedSourceItem::Line {
                        text: format!(".spef_include \"{normalized}\""),
                        origin: NetlistSourceLocation::in_file(current_path, line_number),
                    });
                    continue;
                }
                let included = self
                    .process_include_from_with_selection_mapped_and_abort(
                        current_path,
                        &filename,
                        selected_section,
                        abort,
                    )
                    .map_err(|error| {
                        map_abort_parse_error(error, |error| {
                            include_error_at(error, current_path, line_number, ".include")
                        })
                    })?;
                result.append(included);
                continue;
            }

            result.items.push(ExpandedSourceItem::Line {
                text: line.to_string(),
                origin: NetlistSourceLocation::in_file(current_path, line_number),
            });
        }

        if let Some(frame) = inline_sections.last() {
            return Err(ParseError::Syntax {
                line: frame.opened_at_line,
                message: format!("Library section '{}' missing .ENDL", frame.name),
            }
            .into());
        }

        if require_selected_section && !selected_section_found {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "{}: library section '{}' was not found",
                    current_path.display(),
                    selected_section.unwrap_or_default()
                ),
            }
            .into());
        }

        ensure_parse_not_aborted(abort)?;
        result.items.push(ExpandedSourceItem::ExitSource {
            path: current_path.to_path_buf(),
            eof_line: source_line_count + 1,
        });
        Ok(result)
    }

    fn enter_include(&mut self, key: &IncludeKey) -> Result<(), ParseError> {
        if self.current_depth >= self.max_depth {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Include depth exceeded maximum of {}", self.max_depth),
            });
        }

        if !self.active_includes.insert(key.clone()) {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Circular include/lib detected: {}", key.describe()),
            });
        }

        self.current_depth += 1;
        Ok(())
    }

    fn leave_include(&mut self, key: &IncludeKey) {
        self.current_depth = self.current_depth.saturating_sub(1);
        self.active_includes.remove(key);
    }

    /// Extract a named section from library content
    ///
    /// Library sections are delimited by:
    /// ```text
    /// .LIB section_name
    /// ... content ...
    /// .ENDL [section_name]
    /// ```
    #[cfg(test)]
    fn extract_section(&self, content: &str, section: &str) -> Result<String, ParseError> {
        let mut in_section = false;
        let mut section_content = Vec::new();
        let mut found = false;
        // Section *definitions* nested inside the requested one (a 2-token
        // `.LIB name` line, as opposed to the 3-token `.LIB file section`
        // call form) open their own `.ENDL` scope; counting them keeps an
        // inner `.ENDL` from terminating the outer section early.
        let mut nested_definitions = 0usize;

        for line in content.lines() {
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();

            if upper.starts_with(".LIB") && !upper.starts_with(".LIBS") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if !in_section {
                    // Check if this is our section start
                    if parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section) {
                        in_section = true;
                        found = true;
                        continue;
                    }
                } else if parts.len() == 2 {
                    nested_definitions += 1;
                }
            }

            if in_section {
                if upper.starts_with(".ENDL") {
                    if nested_definitions > 0 {
                        nested_definitions -= 1;
                        section_content.push(line);
                        continue;
                    }
                    // Check if this ends our section
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() == 1
                        || (parts.len() >= 2 && parts[1].eq_ignore_ascii_case(section))
                    {
                        in_section = false;
                        break;
                    }
                    log::warn!(
                        ".ENDL '{}' does not match the open library section '{}'",
                        parts.get(1).copied().unwrap_or(""),
                        section
                    );
                }
                section_content.push(line);
            }
        }

        if !found {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Library section '{}' not found", section),
            });
        }

        if in_section {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Library section '{}' missing .ENDL", section),
            });
        }

        Ok(section_content.join("\n"))
    }

    /// Reset the processor for a new netlist
    pub fn reset(&mut self) {
        self.active_includes.clear();
        self.current_depth = 0;
    }

    /// Set base directory (useful when changing context)
    pub fn set_base_dir(&mut self, path: &Path) {
        self.base_dir = if path.is_file() {
            path.parent().unwrap_or(Path::new(".")).to_path_buf()
        } else {
            path.to_path_buf()
        };
    }
}

impl Default for IncludeProcessor {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

/// Stable identity for a path literal as written in an include or external
/// library directive. The exact verified source text is reused at runtime, so
/// separator normalization is sufficient while preserving case-sensitive host
/// semantics captured by the authenticated edge.
pub fn normalize_source_path_literal(literal: &str) -> Result<String, ParseError> {
    let literal = literal.trim().trim_matches('"').trim_matches('\'');
    if literal.is_empty() {
        return Err(ParseError::Syntax {
            line: 0,
            message: "Include/library path cannot be empty".to_owned(),
        });
    }
    if literal.chars().any(char::is_control) {
        return Err(ParseError::Syntax {
            line: 0,
            message: "Include/library path contains a control character".to_owned(),
        });
    }
    let mut normalized = String::with_capacity(literal.len());
    let mut previous_separator = false;
    for character in literal.chars() {
        let separator = matches!(character, '/' | '\\');
        if separator {
            if !previous_separator || normalized.len() < 2 {
                normalized.push('/');
            }
        } else {
            normalized.push(character);
        }
        previous_separator = separator;
    }
    Ok(normalized)
}

fn include_error_at(
    error: ParseError,
    source_path: &Path,
    source_line: usize,
    directive: &str,
) -> ParseError {
    let detail = match error {
        ParseError::Syntax { line, message } if line > 0 => {
            format!("{message} (nested source line {line})")
        }
        ParseError::Syntax { message, .. } => message,
        other => other.to_string(),
    };
    ParseError::Syntax {
        line: source_line,
        message: format!(
            "{}:{}: {directive} resolution failed: {detail}",
            source_path.display(),
            source_line
        ),
    }
}

fn lexically_normalize_path(path: &Path) -> PathBuf {
    use std::path::Component;

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let can_pop = normalized
                    .components()
                    .next_back()
                    .is_some_and(|last| matches!(last, Component::Normal(_)));
                if can_pop {
                    normalized.pop();
                } else if !path.is_absolute() {
                    normalized.push(component.as_os_str());
                }
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

#[cfg(windows)]
fn windows_path_identity(path: &Path) -> String {
    let mut identity = lexically_normalize_path(path)
        .to_string_lossy()
        .replace('\\', "/");
    if let Some(unprefixed) = identity.strip_prefix("//?/") {
        identity = if let Some(unc) = unprefixed.strip_prefix("UNC/") {
            format!("//{unc}")
        } else {
            unprefixed.to_owned()
        };
    }
    identity.make_ascii_lowercase();
    identity
}

//=============================================================================
// Helper Functions
//=============================================================================

/// Parse an include directive line
///
/// Extracts the filename from `.include`, `.inc`, and `.incl` directives.
/// Handles quote styles and whitespace without accepting longer lookalike
/// directive names.
pub fn parse_include_directive(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let (directive, rest) = split_directive(trimmed)?;
    if !matches_ignore_ascii_case(directive, &[".include", ".inc", ".incl"]) {
        return None;
    }
    let rest = rest.trim();

    // Handle quoted paths
    if let Some(quoted) = rest.strip_prefix('"') {
        if let Some(end) = quoted.find('"') {
            return Some(quoted[..end].to_string());
        }
        return None;
    } else if let Some(quoted) = rest.strip_prefix('\'')
        && let Some(end) = quoted.find('\'')
    {
        return Some(quoted[..end].to_string());
    } else if rest.starts_with('\'') {
        return None;
    }

    // Unquoted - take first word
    Some(rest.split_whitespace().next()?.to_string())
}

fn split_directive(line: &str) -> Option<(&str, &str)> {
    let mut end = line.len();
    for (index, ch) in line.char_indices() {
        if ch.is_whitespace() {
            end = index;
            break;
        }
    }
    let directive = &line[..end];
    if directive.is_empty() {
        return None;
    }
    Some((directive, &line[end..]))
}

fn matches_ignore_ascii_case(value: &str, accepted: &[&str]) -> bool {
    accepted
        .iter()
        .any(|accepted| value.eq_ignore_ascii_case(accepted))
}

fn windows_drive_relative_suffix(path: &str) -> Option<&str> {
    let bytes = path.as_bytes();
    if bytes.len() < 3 || bytes[1] != b':' || !bytes[0].is_ascii_alphabetic() {
        return None;
    }
    if matches!(bytes[2], b'/' | b'\\') {
        return None;
    }
    Some(&path[2..])
}

/// Convert a SPICE source-path literal to the current host's path syntax.
/// Both slash styles are directive separators regardless of the host that is
/// doing discovery; native absolute paths retain their prefix/root semantics.
pub(crate) fn source_path_literal_to_host_path(path: &str) -> PathBuf {
    let path = path.trim().trim_matches('"').trim_matches('\'');
    let native = Path::new(path);
    if native.is_absolute() {
        native.to_path_buf()
    } else {
        spice_relative_path(path)
    }
}

fn spice_relative_path(path: &str) -> PathBuf {
    path.split(['/', '\\'])
        .filter(|component| !component.is_empty() && *component != ".")
        .fold(PathBuf::new(), |mut out, component| {
            out.push(component);
            out
        })
}

/// Parse a lib directive line
///
/// Returns (filename, optional_section)
pub fn parse_lib_directive(line: &str) -> Option<(String, Option<String>)> {
    let trimmed = line.trim();
    let (directive, rest) = split_directive(trimmed)?;
    if !directive.eq_ignore_ascii_case(".lib") {
        return None;
    }
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }

    let (filename, remainder) = if let Some(quote) = rest
        .chars()
        .next()
        .filter(|character| matches!(character, '"' | '\''))
    {
        let quoted = &rest[quote.len_utf8()..];
        let end = quoted.find(quote)?;
        (
            quoted[..end].to_owned(),
            quoted[end + quote.len_utf8()..].trim(),
        )
    } else {
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        (rest[..end].to_owned(), rest[end..].trim())
    };
    if filename.is_empty() {
        return None;
    }

    let section = remainder.split_whitespace().next().map(str::to_owned);

    Some((filename, section))
}

fn parse_endl_directive(
    line: &str,
    line_number: usize,
) -> Result<Option<Option<String>>, ParseError> {
    let Some((directive, rest)) = split_directive(line.trim()) else {
        return Ok(None);
    };
    if !directive.eq_ignore_ascii_case(".endl") {
        return Ok(None);
    }

    let fields = rest.split_whitespace().collect::<Vec<_>>();
    if fields.len() > 1 {
        log::warn!(
            ".ENDL at line {line_number} has extraneous fields after the section name; ignoring them"
        );
    }
    Ok(Some(fields.first().map(|name| (*name).to_string())))
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn include_expansion_aborts_after_multiple_mid_stream_polls() {
        let mut source = String::from("include cancellation fixture\n");
        for index in 0..1_024 {
            source.push_str(&format!("R{index} n{index} 0 1k\n"));
        }
        let abort = crate::abort_signal::CountingAbort::new(8);
        let mut processor = IncludeProcessor::new(Path::new("fixture.cir"));

        let result = processor.expand_content_with_abort(&source, Path::new("fixture.cir"), &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(
            abort.count() > 8,
            "abort must be observed after repeated expansion work"
        );
    }

    #[test]
    fn include_directive_parser_accepts_xyce_aliases_exactly() {
        assert_eq!(
            parse_include_directive(".include \"model cards/mod.inc\"").as_deref(),
            Some("model cards/mod.inc")
        );
        assert_eq!(
            parse_include_directive(".INC incFile1").as_deref(),
            Some("incFile1")
        );
        assert_eq!(
            parse_include_directive(".incl 'sub1/include1'").as_deref(),
            Some("sub1/include1")
        );
        assert_eq!(parse_include_directive(".includex bad"), None);
        assert_eq!(parse_include_directive(".incbin bad"), None);
    }

    #[test]
    fn include_processor_expands_xyce_include_aliases() {
        let dir = unique_include_temp_dir("aliases");
        std::fs::create_dir_all(&dir).expect("create include alias fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(dir.join("incFile1"), "R1 1 2 1\n").expect("write incFile1");
        std::fs::write(dir.join("incFile2"), "R2 2 3 1\n").expect("write incFile2");
        std::fs::write(dir.join("incFile3"), "R3 3 0 1\n").expect("write incFile3");

        let deck = ".INC incFile1\n.INCL incFile2\n.INCLUDE incFile3\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new(&deck_path);
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("include aliases expand");

        assert!(expanded.contains("R1 1 2 1"), "{expanded}");
        assert!(expanded.contains("R2 2 3 1"), "{expanded}");
        assert!(expanded.contains("R3 3 0 1"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn mapped_include_expansion_preserves_nested_source_boundaries_and_lines() {
        let dir = unique_include_temp_dir("mapped-origins");
        let nested = dir.join("nested");
        std::fs::create_dir_all(&nested).expect("create mapped include fixture");
        let deck_path = dir.join("deck.cir");
        let child_path = nested.join("child.inc");
        let leaf_path = nested.join("leaf.inc");
        std::fs::write(
            &child_path,
            "Rchild 1 0 1\n.include leaf.inc\n.end\nRignored 9 0 9\n",
        )
        .expect("write child include");
        std::fs::write(&leaf_path, "Rleaf 2 0 2\n").expect("write leaf include");
        let deck = "mapped title\n.include nested/child.inc\nRtop 3 0 3\n.end\n";

        let mut processor = IncludeProcessor::new(&deck_path);
        let mapped = processor
            .expand_content_mapped_with_abort(deck, &deck_path, &NoAbort)
            .expect("mapped expansion succeeds");
        assert_eq!(
            mapped.render(),
            "mapped title\nRchild 1 0 1\nRleaf 2 0 2\nRtop 3 0 3\n.end\n"
        );

        let child_canonical = child_path.canonicalize().expect("canonical child path");
        let leaf_canonical = leaf_path.canonicalize().expect("canonical leaf path");
        assert_eq!(
            mapped.items,
            vec![
                ExpandedSourceItem::EnterSource {
                    path: deck_path.clone(),
                },
                ExpandedSourceItem::Line {
                    text: "mapped title".to_string(),
                    origin: NetlistSourceLocation::in_file(&deck_path, 1),
                },
                ExpandedSourceItem::EnterSource {
                    path: child_canonical.clone(),
                },
                ExpandedSourceItem::Line {
                    text: "Rchild 1 0 1".to_string(),
                    origin: NetlistSourceLocation::in_file(&child_canonical, 1),
                },
                ExpandedSourceItem::EnterSource {
                    path: leaf_canonical.clone(),
                },
                ExpandedSourceItem::Line {
                    text: "Rleaf 2 0 2".to_string(),
                    origin: NetlistSourceLocation::in_file(&leaf_canonical, 1),
                },
                ExpandedSourceItem::ExitSource {
                    path: leaf_canonical,
                    eof_line: 2,
                },
                ExpandedSourceItem::EndCard {
                    origin: NetlistSourceLocation::in_file(&child_canonical, 3),
                },
                ExpandedSourceItem::ExitSource {
                    path: child_canonical,
                    eof_line: 5,
                },
                ExpandedSourceItem::Line {
                    text: "Rtop 3 0 3".to_string(),
                    origin: NetlistSourceLocation::in_file(&deck_path, 3),
                },
                ExpandedSourceItem::Line {
                    text: ".end".to_string(),
                    origin: NetlistSourceLocation::in_file(&deck_path, 4),
                },
                ExpandedSourceItem::ExitSource {
                    path: deck_path.clone(),
                    eof_line: 5,
                },
            ]
        );

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn include_processor_uses_xyce_top_level_fallback_after_local_path() {
        let dir = unique_include_temp_dir("fallback");
        let sub1 = dir.join("sub1");
        let sub2 = sub1.join("sub2");
        std::fs::create_dir_all(&sub2).expect("create nested include fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(
            sub1.join("include1"),
            ".INC sub2/local\n.INC sub1/sub2/top\n.INC precedence/wins_local\n",
        )
        .expect("write include1");
        std::fs::write(sub2.join("local"), "RLOCAL 1 0 3\n").expect("write local include");
        std::fs::write(sub2.join("top"), "RTOP 1 0 4\n").expect("write fallback include");
        std::fs::create_dir_all(sub1.join("precedence")).expect("create local precedence dir");
        std::fs::create_dir_all(dir.join("precedence")).expect("create top precedence dir");
        std::fs::write(sub1.join("precedence").join("wins_local"), "RWIN 1 0 5\n")
            .expect("write local precedence include");
        std::fs::write(dir.join("precedence").join("wins_local"), "RLOSE 1 0 6\n")
            .expect("write top precedence include");

        let deck = ".INC sub1/include1\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new(&deck_path);
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("nested include fallback expands");

        assert!(expanded.contains("RLOCAL 1 0 3"), "{expanded}");
        assert!(expanded.contains("RTOP 1 0 4"), "{expanded}");
        assert!(expanded.contains("RWIN 1 0 5"), "{expanded}");
        assert!(!expanded.contains("RLOSE"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn include_processor_uses_xyce_execution_dir_fallback_after_top_level_path() {
        let exec_dir = unique_include_temp_dir("execution-fallback");
        let top_dir = exec_dir.join("top");
        let include_dir = top_dir.join("sub1");
        std::fs::create_dir_all(&include_dir).expect("create nested include fixture");
        let deck_path = top_dir.join("deck.cir");
        std::fs::write(
            include_dir.join("include1"),
            ".INC local\n.INC top\n.INC execution\n.INC precedence/wins_top\n",
        )
        .expect("write include1");
        std::fs::write(include_dir.join("local"), "RLOCAL 1 0 3\n").expect("write local include");
        std::fs::write(top_dir.join("top"), "RTOP 1 0 4\n").expect("write top include");
        std::fs::write(exec_dir.join("execution"), "REXEC 1 0 5\n")
            .expect("write execution include");
        std::fs::create_dir_all(top_dir.join("precedence")).expect("create top precedence dir");
        std::fs::create_dir_all(exec_dir.join("precedence"))
            .expect("create execution precedence dir");
        std::fs::write(top_dir.join("precedence").join("wins_top"), "RWIN 1 0 6\n")
            .expect("write top precedence include");
        std::fs::write(
            exec_dir.join("precedence").join("wins_top"),
            "RLOSE 1 0 7\n",
        )
        .expect("write execution precedence include");

        let deck = ".INC sub1/include1\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new_with_execution_dir(&deck_path, Some(&exec_dir));
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("execution fallback expands");

        assert!(expanded.contains("RLOCAL 1 0 3"), "{expanded}");
        assert!(expanded.contains("RTOP 1 0 4"), "{expanded}");
        assert!(expanded.contains("REXEC 1 0 5"), "{expanded}");
        assert!(expanded.contains("RWIN 1 0 6"), "{expanded}");
        assert!(!expanded.contains("RLOSE"), "{expanded}");
        let _ = std::fs::remove_dir_all(exec_dir);
    }

    #[test]
    fn include_processor_resolves_xyce_drive_relative_paths_from_top_level() {
        let dir = unique_include_temp_dir("drive-relative");
        let sub1 = dir.join("sub1");
        std::fs::create_dir_all(&sub1).expect("create drive-relative fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(sub1.join("include1"), ".INC C:drive_file\n")
            .expect("write drive-relative include");
        std::fs::write(dir.join("drive_file"), "RDRIVE 1 0 7\n").expect("write drive file");

        let deck = ".INC sub1\\include1\n";
        std::fs::write(&deck_path, deck).expect("write deck");
        let mut processor = IncludeProcessor::new(&deck_path);
        let expanded = processor
            .expand_content(deck, &deck_path)
            .expect("drive-relative include expands");

        assert!(expanded.contains("RDRIVE 1 0 7"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn inline_library_definitions_are_omitted_with_nested_scope_tracking() {
        let deck = "\
.lib Unused
.invalid line hidden from the parser
.include missing-file.inc
.lib missing-library.lib nominal
.lib nested
Rhidden 1 0 1
.endl NESTED
.endl unused
Rkept 1 0 2
";
        let deck_path = Path::new("deck.cir");
        let expanded = IncludeProcessor::new(deck_path)
            .expand_content(deck, deck_path)
            .expect("inline library definitions preprocess");

        assert_eq!(expanded, "Rkept 1 0 2\n");
    }

    #[test]
    fn external_library_section_still_expands_after_inline_definition() {
        let dir = unique_include_temp_dir("inline-and-external-lib");
        std::fs::create_dir_all(&dir).expect("create library fixture");
        let deck_path = dir.join("deck.cir");
        std::fs::write(
            dir.join("models.lib"),
            ".lib nominal\n.param selected=7\n.include child.lib\n.endl NOMINAL\n",
        )
        .expect("write library fixture");
        std::fs::write(
            dir.join("child.lib"),
            ".lib low\n.param inherited=1\n.endl low\n.lib nominal\n.param inherited=9\n.endl nominal\n",
        )
        .expect("write nested library fixture");
        let deck = "\
.lib ignored
.invalid hidden
.endl ignored
.lib models.lib nominal
R1 1 0 {selected}
";
        let expanded = IncludeProcessor::new(&deck_path)
            .expand_content(deck, &deck_path)
            .expect("external library section expands");

        assert!(expanded.contains(".param selected=7"), "{expanded}");
        assert!(expanded.contains(".param inherited=9"), "{expanded}");
        assert!(!expanded.contains(".param inherited=1"), "{expanded}");
        assert!(expanded.contains("R1 1 0 {selected}"), "{expanded}");
        assert!(!expanded.contains("hidden"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn inactive_library_end_card_does_not_preempt_selected_section() {
        let dir = unique_include_temp_dir("inactive-lib-end-card");
        std::fs::create_dir_all(&dir).expect("create inactive library END fixture");
        let library_path = dir.join("corners.lib");
        std::fs::write(
            &library_path,
            ".lib SS\n.param corner=1\n.end\n.endl SS\n.lib TT\n.param corner=2\n.endl TT\n",
        )
        .expect("write inactive library END fixture");

        let expanded = IncludeProcessor::new(&dir)
            .process_lib("corners.lib", Some("TT"))
            .expect("inactive SS .END must not prevent TT selection");

        assert!(expanded.contains(".param corner=2"), "{expanded}");
        assert!(!expanded.contains(".param corner=1"), "{expanded}");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn inline_library_scope_errors_are_line_aware() {
        for (deck, expected_line, expected_message) in [
            (
                ".endl orphan\n",
                1,
                ".ENDL encountered without an open .LIB section",
            ),
            (
                ".lib first\n.endl second\n",
                2,
                ".ENDL section 'second' does not match open .LIB section 'first'",
            ),
            (
                ".lib unfinished\nR1 1 0 1\n",
                1,
                "Library section 'unfinished' missing .ENDL",
            ),
        ] {
            let err = IncludeProcessor::new(Path::new("deck.cir"))
                .expand_content(deck, Path::new("deck.cir"))
                .expect_err("malformed inline library scope must reject");
            match err {
                ParseError::Syntax { line, message } => {
                    assert_eq!(line, expected_line, "{message}");
                    assert!(message.contains(expected_message), "{message}");
                }
                other => panic!("expected syntax error, got {other:?}"),
            }
        }
    }

    #[test]
    fn lib_parser_rejects_longer_directive_lookalikes() {
        assert_eq!(parse_lib_directive(".libs foo"), None);
        assert_eq!(parse_lib_directive(".library foo"), None);
        assert_eq!(
            parse_lib_directive(".LIB \"model cards.lib\" nominal"),
            Some(("model cards.lib".to_string(), Some("nominal".to_string())))
        );
    }

    fn sealed_test_path(name: &str) -> PathBuf {
        #[cfg(windows)]
        {
            PathBuf::from(r"C:\rspice-sealed-tests").join(name)
        }
        #[cfg(not(windows))]
        {
            PathBuf::from("/rspice-sealed-tests").join(name)
        }
    }

    fn sealed_bundle(
        sources: Vec<(PathBuf, String)>,
        edges: Vec<SealedSourceEdge>,
    ) -> SealedSourceBundle {
        SealedSourceBundle::try_new_with_edges(sources, edges)
            .expect("sealed fixture is internally consistent")
    }

    #[test]
    fn sealed_materialization_uses_authenticated_edges_for_aliases_and_nested_sections() {
        let root = sealed_test_path("root.lib");
        let aliased = sealed_test_path("vendor/device.inc");
        let absolute = sealed_test_path("absolute.inc");
        let child = sealed_test_path("actual/child.lib");
        let absolute_literal = absolute.to_string_lossy().into_owned();
        let root_content = format!(
            ".lib TT\n.include \"alias path/device.inc\"\n.inc '{absolute_literal}'\n.lib 'search-name/child.lib' FAST\n.endl TT\n.lib SS\n.include missing-but-inactive.inc\n.endl SS\n"
        );
        let bundle = sealed_bundle(
            vec![
                (root.clone(), root_content),
                (
                    aliased.clone(),
                    ".model alias_n NMOS (LEVEL=1 KP=1e-3)\n".to_owned(),
                ),
                (
                    absolute.clone(),
                    ".model absolute_n NMOS (LEVEL=1 KP=2e-3)\n".to_owned(),
                ),
                (
                    child.clone(),
                    ".lib SLOW\n.model slow_n NMOS (LEVEL=1 KP=3e-3)\n.endl SLOW\n.lib FAST\n.model fast_n NMOS (LEVEL=1 KP=4e-3)\n.endl FAST\n"
                        .to_owned(),
                ),
            ],
            vec![
                SealedSourceEdge {
                    owner: root.clone(),
                    requested_path: "alias path/device.inc".to_owned(),
                    target: aliased,
                },
                SealedSourceEdge {
                    owner: root.clone(),
                    requested_path: absolute_literal,
                    target: absolute,
                },
                SealedSourceEdge {
                    owner: root.clone(),
                    requested_path: "search-name/child.lib".to_owned(),
                    target: child,
                },
            ],
        );

        let expanded = IncludeProcessor::new_sealed(&root, bundle)
            .process_sealed_root(&root, Some("tt"))
            .expect("selected section materializes only from authenticated edges");

        assert!(expanded.contains("alias_n"), "{expanded}");
        assert!(expanded.contains("absolute_n"), "{expanded}");
        assert!(expanded.contains("fast_n"), "{expanded}");
        assert!(!expanded.contains("slow_n"), "{expanded}");
        assert!(!expanded.contains("missing-but-inactive"), "{expanded}");
        assert!(expanded.lines().all(|line| {
            parse_include_directive(line).is_none() && parse_lib_directive(line).is_none()
        }));
    }

    #[test]
    fn sealed_materialization_rejects_unlisted_edges_even_when_source_is_listed() {
        let root = sealed_test_path("unlisted-root.lib");
        let tempting = sealed_test_path("tempting.inc");
        let bundle = sealed_bundle(
            vec![
                (root.clone(), ".include tempting.inc\n".to_owned()),
                (tempting, "Rtempt 1 0 1k\n".to_owned()),
            ],
            Vec::new(),
        );

        let error = IncludeProcessor::new_sealed(&root, bundle)
            .process_sealed_root(&root, None)
            .expect_err("a listed source without an authenticated edge must not resolve");
        let message = error.to_string();
        assert!(
            message.contains("authenticated sealed resolution graph"),
            "{message}"
        );
        assert!(message.contains("unlisted-root.lib:1"), "{message}");
    }

    #[test]
    fn sealed_materialization_rejects_missing_sections_cycles_and_spef_escape() {
        let section_root = sealed_test_path("section-root.lib");
        let section_bundle = sealed_bundle(
            vec![(
                section_root.clone(),
                ".lib TT\n.model only_tt D\n.endl TT\n".to_owned(),
            )],
            Vec::new(),
        );
        let missing = IncludeProcessor::new_sealed(&section_root, section_bundle)
            .process_sealed_root(&section_root, Some("FF"))
            .expect_err("missing external section must fail");
        assert!(missing.to_string().contains("section 'FF' was not found"));

        let cycle_root = sealed_test_path("cycle-root.lib");
        let cycle_child = sealed_test_path("cycle-child.inc");
        let cycle_bundle = sealed_bundle(
            vec![
                (cycle_root.clone(), ".include cycle-child.inc\n".to_owned()),
                (cycle_child.clone(), ".include cycle-root.lib\n".to_owned()),
            ],
            vec![
                SealedSourceEdge {
                    owner: cycle_root.clone(),
                    requested_path: "cycle-child.inc".to_owned(),
                    target: cycle_child.clone(),
                },
                SealedSourceEdge {
                    owner: cycle_child,
                    requested_path: "cycle-root.lib".to_owned(),
                    target: cycle_root.clone(),
                },
            ],
        );
        let cycle = IncludeProcessor::new_sealed(&cycle_root, cycle_bundle)
            .process_sealed_root(&cycle_root, None)
            .expect_err("sealed include cycle must fail");
        assert!(cycle.to_string().contains("Circular include/lib"));

        let spef_root = sealed_test_path("spef-root.lib");
        let spef = sealed_test_path("parasitics.spef");
        let spef_bundle = sealed_bundle(
            vec![
                (spef_root.clone(), ".include parasitics.spef\n".to_owned()),
                (spef.clone(), "*SPEF \"IEEE 1481-1998\"\n".to_owned()),
            ],
            vec![SealedSourceEdge {
                owner: spef_root.clone(),
                requested_path: "parasitics.spef".to_owned(),
                target: spef,
            }],
        );
        let spef_error = IncludeProcessor::new_sealed(&spef_root, spef_bundle)
            .process_sealed_root(&spef_root, None)
            .expect_err("SPEF must not escape sealed source handling");
        assert!(
            spef_error
                .to_string()
                .contains("filesystem-backed parasitic reads are forbidden")
        );
    }

    fn sealed_depth_bundle(source_count: usize) -> (PathBuf, SealedSourceBundle) {
        let paths = (0..source_count)
            .map(|index| sealed_test_path(&format!("depth/{index}.inc")))
            .collect::<Vec<_>>();
        let sources = paths
            .iter()
            .enumerate()
            .map(|(index, path)| {
                let content = if index + 1 < paths.len() {
                    format!(".include {}.inc\n", index + 1)
                } else {
                    "Rlast 1 0 1k\n".to_owned()
                };
                (path.clone(), content)
            })
            .collect::<Vec<_>>();
        let edges = paths
            .windows(2)
            .enumerate()
            .map(|(index, pair)| SealedSourceEdge {
                owner: pair[0].clone(),
                requested_path: format!("{}.inc", index + 1),
                target: pair[1].clone(),
            })
            .collect::<Vec<_>>();
        let root = paths[0].clone();
        (root, sealed_bundle(sources, edges))
    }

    #[test]
    fn sealed_include_depth_accepts_64_frames_and_rejects_65() {
        let (accepted_root, accepted_bundle) = sealed_depth_bundle(DEFAULT_MAX_INCLUDE_DEPTH);
        let accepted = IncludeProcessor::new_sealed(&accepted_root, accepted_bundle)
            .process_sealed_root(&accepted_root, None)
            .expect("the production include-depth boundary is accepted");
        assert!(accepted.contains("Rlast"));

        let (rejected_root, rejected_bundle) = sealed_depth_bundle(DEFAULT_MAX_INCLUDE_DEPTH + 1);
        let rejected = IncludeProcessor::new_sealed(&rejected_root, rejected_bundle)
            .process_sealed_root(&rejected_root, None)
            .expect_err("one frame beyond the production boundary must fail");
        assert!(
            rejected
                .to_string()
                .contains("Include depth exceeded maximum of 64")
        );
    }

    fn unique_include_temp_dir(label: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rspice-include-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ))
    }

    fn extract(content: &str, section: &str) -> String {
        IncludeProcessor::new(std::path::Path::new("."))
            .extract_section(content, section)
            .expect("section extracts")
    }

    #[test]
    fn nested_section_definition_does_not_terminate_outer() {
        let lib = "\
.lib outer
r1 a b 1k
.lib inner
r2 b c 2k
.endl
r3 c d 3k
.endl outer
";
        let body = extract(lib, "outer");
        assert!(body.contains("r1"), "outer head kept: {body}");
        assert!(
            body.contains(".lib inner") && body.contains("r2"),
            "nested definition preserved intact: {body}"
        );
        assert!(
            body.contains("r3"),
            "content after the nested definition still belongs to outer: {body}"
        );
    }

    #[test]
    fn bare_endl_still_terminates_unnested_section() {
        let lib = "\
.lib tt
r1 a b 1k
.endl
.lib ss
r2 a b 9k
.endl
";
        let tt = extract(lib, "tt");
        assert!(tt.contains("r1") && !tt.contains("r2"), "{tt}");
        let ss = extract(lib, "ss");
        assert!(ss.contains("r2") && !ss.contains("r1"), "{ss}");
    }

    #[test]
    fn mismatched_endl_name_does_not_end_the_section() {
        let lib = "\
.lib tt
r1 a b 1k
.endl ff
r2 a b 2k
.endl tt
";
        let tt = extract(lib, "tt");
        assert!(
            tt.contains("r1") && tt.contains("r2"),
            "mismatched .endl is content, not a terminator: {tt}"
        );
    }

    #[test]
    fn unterminated_selected_library_section_is_rejected() {
        let lib = "\
.lib tt
r1 a b 1k
.lib ss
r2 a b 2k
.endl ss
";
        let err = IncludeProcessor::default()
            .extract_section(lib, "tt")
            .expect_err("unterminated selected library section must reject");

        match err {
            ParseError::Syntax { message, .. } => {
                assert!(
                    message.contains("Library section 'tt' missing .ENDL"),
                    "unexpected error: {message}"
                );
            }
            other => panic!("expected syntax error, got {other:?}"),
        }
    }
}
