//! Verilog-A Preprocessor
//!
//! Implements a full C-like preprocessor for Verilog-A source files.
//! Handles:
//! - `include "file"` / `include <file>` - File inclusion
//! - `define NAME value` - Macro definitions
//! - `define NAME(args) value` - Parameterized macros
//! - `ifdef NAME` / `ifndef NAME` / `else` / `elsif NAME` / `endif` - Conditionals
//! - `undef NAME` - Undefine macros
//!
//! This preprocessor runs before lexing, producing a single expanded source string.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Origin of an exact source document consumed by preprocessing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceDocumentOrigin {
    /// The document was returned by the caller-selected source provider.
    Provider,
    /// The document is one of RSpice's versioned Verilog-AMS standard headers.
    BuiltIn,
}

/// Exact source document returned by a [`SourceProvider`].
///
/// `logical_path` is the provider's stable identity for the document. A
/// provider must return identical UTF-8 source whenever it returns the same
/// logical path during one preprocessing operation. The preprocessor enforces
/// that contract before publishing dependency metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    pub logical_path: PathBuf,
    pub source: String,
    pub origin: SourceDocumentOrigin,
}

impl SourceDocument {
    pub fn provided(logical_path: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        Self {
            logical_path: logical_path.into(),
            source: source.into(),
            origin: SourceDocumentOrigin::Provider,
        }
    }
}

/// Resource bounds advertised by a source provider.
///
/// File-system preprocessing keeps the historical unbounded behavior. Virtual
/// and remotely supplied providers should use finite values so hostile source
/// graphs cannot exhaust memory or recurse without bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceProviderLimits {
    pub max_dependencies: usize,
    pub max_total_source_bytes: usize,
    pub max_include_depth: usize,
    pub max_expanded_bytes: usize,
}

impl SourceProviderLimits {
    pub const UNBOUNDED: Self = Self {
        max_dependencies: usize::MAX,
        max_total_source_bytes: usize::MAX,
        max_include_depth: usize::MAX,
        max_expanded_bytes: usize::MAX,
    };
}

/// Deterministic source-loading boundary used by the Verilog-A preprocessor.
///
/// Implementations decide where source comes from; preprocessing itself never
/// falls through to another provider. Returning `None` from `resolve_include`
/// allows the preprocessor to try its versioned built-in standard headers and
/// then report a missing include. Implementations must not return different
/// contents for the same logical path during a preprocessing operation.
pub trait SourceProvider {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, PreprocessorError>;

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError>;

    fn limits(&self) -> SourceProviderLimits {
        SourceProviderLimits::UNBOUNDED
    }
}

/// Source provider preserving the pre-existing file-system include behavior.
#[derive(Debug, Clone, Copy, Default)]
pub struct FileSystemSourceProvider;

impl FileSystemSourceProvider {
    fn load_path(path: &Path) -> Result<SourceDocument, PreprocessorError> {
        let canonical = path.canonicalize().map_err(|error| {
            PreprocessorError::new(
                format!("Cannot open file: {error}"),
                Some(path.to_path_buf()),
                0,
            )
        })?;
        let source = std::fs::read_to_string(&canonical).map_err(|error| {
            PreprocessorError::new(
                format!("Cannot read file: {error}"),
                Some(path.to_path_buf()),
                0,
            )
        })?;
        Ok(SourceDocument::provided(canonical, source))
    }
}

impl SourceProvider for FileSystemSourceProvider {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, PreprocessorError> {
        Self::load_path(requested)
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError> {
        if let Some(parent) = including_file.and_then(Path::parent) {
            let candidate = parent.join(requested);
            if candidate.exists() {
                return Self::load_path(&candidate).map(Some);
            }
        }

        for include_path in include_paths {
            let candidate = include_path.join(requested);
            if candidate.exists() {
                return Self::load_path(&candidate).map(Some);
            }
        }

        Ok(None)
    }
}

struct LimitedSourceProvider<'a> {
    inner: &'a dyn SourceProvider,
    limits: SourceProviderLimits,
}

impl SourceProvider for LimitedSourceProvider<'_> {
    fn load_root(&self, requested: &Path) -> Result<SourceDocument, PreprocessorError> {
        self.inner.load_root(requested)
    }

    fn resolve_include(
        &self,
        including_file: Option<&Path>,
        include_paths: &[PathBuf],
        requested: &str,
    ) -> Result<Option<SourceDocument>, PreprocessorError> {
        self.inner
            .resolve_include(including_file, include_paths, requested)
    }

    fn limits(&self) -> SourceProviderLimits {
        self.limits
    }
}

/// Exact dependency record captured during provider-backed preprocessing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocessedDependency {
    pub logical_path: PathBuf,
    pub source: String,
    pub origin: SourceDocumentOrigin,
}

/// One include edge actually traversed by preprocessing.
///
/// Unlike a textual scan, this records only directives selected by active
/// conditional compilation and resolved by the configured source provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocessedInclude {
    pub including_path: PathBuf,
    pub requested_path: String,
    pub include_index: usize,
    pub included_path: PathBuf,
}

/// Source origin for one contiguous range of the expanded compiler input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreprocessedSourceSegment {
    pub expanded_start: usize,
    pub expanded_end: usize,
    pub logical_path: PathBuf,
    pub source_line: usize,
}

/// Expanded compiler input paired with its exact source-origin map.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct PreprocessedSource {
    pub source: String,
    pub segments: Vec<PreprocessedSourceSegment>,
}

impl PreprocessedSource {
    fn push_source(&mut self, source: &str, logical_path: &Path, first_source_line: usize) {
        for (line_offset, line) in source.split_inclusive('\n').enumerate() {
            let expanded_start = self.source.len();
            self.source.push_str(line);
            self.segments.push(PreprocessedSourceSegment {
                expanded_start,
                expanded_end: self.source.len(),
                logical_path: logical_path.to_path_buf(),
                source_line: first_source_line.saturating_add(line_offset),
            });
        }
        if !source.is_empty() && !source.ends_with('\n') {
            // `split_inclusive` already emitted the final unterminated line.
            debug_assert_eq!(
                self.segments.last().map(|segment| segment.expanded_end),
                Some(self.source.len())
            );
        }
    }

    fn append(&mut self, mut nested: Self) {
        let offset = self.source.len();
        self.source.push_str(&nested.source);
        for segment in &mut nested.segments {
            segment.expanded_start = segment.expanded_start.saturating_add(offset);
            segment.expanded_end = segment.expanded_end.saturating_add(offset);
        }
        self.segments.extend(nested.segments);
    }

    pub(crate) fn segment_at(&self, offset: usize) -> Option<&PreprocessedSourceSegment> {
        let index = self
            .segments
            .partition_point(|segment| segment.expanded_end <= offset);
        self.segments
            .get(index)
            .filter(|segment| segment.expanded_start <= offset && offset < segment.expanded_end)
            .or_else(|| {
                (offset == self.source.len())
                    .then(|| self.segments.last())
                    .flatten()
            })
    }
}

/// Error type for preprocessor errors
#[derive(Debug, Clone)]
pub struct PreprocessorError {
    pub message: String,
    pub file: Option<PathBuf>,
    pub line: usize,
}

impl std::fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            Some(path) => write!(f, "{}:{}: {}", path.display(), self.line, self.message),
            None => write!(f, "line {}: {}", self.line, self.message),
        }
    }
}

impl std::error::Error for PreprocessorError {}

impl PreprocessorError {
    pub fn new(message: impl Into<String>, file: Option<PathBuf>, line: usize) -> Self {
        Self {
            message: message.into(),
            file,
            line,
        }
    }
}

/// Macro definition with optional parameters
#[derive(Debug, Clone)]
pub struct MacroDef {
    /// Parameter names (empty for simple macros)
    pub params: Vec<String>,
    /// Replacement text
    pub body: String,
}

impl MacroDef {
    pub fn simple(body: impl Into<String>) -> Self {
        Self {
            params: Vec::new(),
            body: body.into(),
        }
    }

    pub fn with_params(params: Vec<String>, body: impl Into<String>) -> Self {
        Self {
            params,
            body: body.into(),
        }
    }

    /// Expand macro with given arguments
    /// Uses word-boundary matching to prevent replacing substrings within identifiers
    pub fn expand(&self, args: &[String]) -> String {
        self.expand_bounded(args, usize::MAX)
            .expect("an unbounded macro expansion cannot exceed its limit")
    }

    fn expand_bounded(&self, args: &[String], max_bytes: usize) -> Result<String, ()> {
        if self.params.is_empty() {
            return (self.body.len() <= max_bytes)
                .then(|| self.body.clone())
                .ok_or(());
        }

        if self.body.len() > max_bytes {
            return Err(());
        }
        let mut result = self.body.clone();
        for (i, param) in self.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                // Replace parameter with argument using word-boundary matching
                // Only replace when param is a complete identifier, not part of another word
                result = Self::replace_identifier_bounded(&result, param, arg, max_bytes)?;
            }
        }
        Ok(result)
    }

    /// Replace identifier occurrences respecting word boundaries.
    fn replace_identifier_bounded(
        text: &str,
        pattern: &str,
        replacement: &str,
        max_bytes: usize,
    ) -> Result<String, ()> {
        if pattern.is_empty() {
            return Err(());
        }
        let mut result = String::with_capacity(text.len().min(max_bytes));
        let mut chars = text.chars().peekable();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        while let Some(ch) = chars.next() {
            // Check if this could be the start of an identifier match
            if ch == pattern_chars[0] {
                // Check if we're not in the middle of an identifier
                let prev_was_alnum = result
                    .chars()
                    .last()
                    .map(|c| c.is_ascii_alphanumeric() || c == '_')
                    .unwrap_or(false);

                if prev_was_alnum {
                    // We're in the middle of an identifier, can't match
                    push_char_bounded(&mut result, ch, max_bytes)?;
                    continue;
                }

                // Try to match the pattern
                let mut matched = true;
                let mut consumed: Vec<char> = vec![ch];

                for &pat_ch in &pattern_chars[1..] {
                    if let Some(&next) = chars.peek() {
                        if next == pat_ch {
                            consumed.push(chars.next().unwrap());
                        } else {
                            matched = false;
                            break;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    // Check if the next character is NOT an identifier char (word boundary)
                    let next_is_alnum = chars
                        .peek()
                        .map(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .unwrap_or(false);

                    if !next_is_alnum {
                        // Full match with word boundary - replace!
                        push_str_bounded(&mut result, replacement, max_bytes)?;
                    } else {
                        // Next char is part of identifier, not a match
                        for c in consumed {
                            push_char_bounded(&mut result, c, max_bytes)?;
                        }
                    }
                } else {
                    // Pattern didn't match completely
                    for c in consumed {
                        push_char_bounded(&mut result, c, max_bytes)?;
                    }
                }
            } else {
                push_char_bounded(&mut result, ch, max_bytes)?;
            }
        }

        Ok(result)
    }
}

fn push_str_bounded(result: &mut String, value: &str, max_bytes: usize) -> Result<(), ()> {
    let next_len = result.len().checked_add(value.len()).ok_or(())?;
    if next_len > max_bytes {
        return Err(());
    }
    result.push_str(value);
    Ok(())
}

fn push_char_bounded(result: &mut String, value: char, max_bytes: usize) -> Result<(), ()> {
    let next_len = result.len().checked_add(value.len_utf8()).ok_or(())?;
    if next_len > max_bytes {
        return Err(());
    }
    result.push(value);
    Ok(())
}

/// Verilog-A Preprocessor
pub struct Preprocessor {
    /// Defined macros
    macros: HashMap<String, MacroDef>,
    /// Include search paths
    include_paths: Vec<PathBuf>,
    /// Files currently being included (for circular detection)
    include_stack: HashSet<PathBuf>,
    /// Current file being processed
    current_file: Option<PathBuf>,
    /// Canonical dependency set encountered during preprocessing.
    dependencies: BTreeSet<PathBuf>,
    /// Exact dependencies in first-load traversal order.
    dependency_documents: Vec<PreprocessedDependency>,
    /// Exact include edges traversed in compiler order.
    include_graph: Vec<PreprocessedInclude>,
    /// Exact content indexed by logical path, used to enforce provider
    /// determinism even when a guarded header is included repeatedly.
    dependency_content: BTreeMap<PathBuf, String>,
    /// Total bytes across unique dependency documents.
    dependency_source_bytes: usize,
}

impl Default for Preprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor {
    /// Create a new preprocessor with standard definitions
    pub fn new() -> Self {
        let mut pp = Self {
            macros: HashMap::new(),
            include_paths: Vec::new(),
            include_stack: HashSet::new(),
            current_file: None,
            dependencies: BTreeSet::new(),
            dependency_documents: Vec::new(),
            include_graph: Vec::new(),
            dependency_content: BTreeMap::new(),
            dependency_source_bytes: 0,
        };

        // Add standard Verilog-AMS predefined macros
        pp.define("__VAMS_ENABLE__", MacroDef::simple("1"));
        pp.define("__VAMS_COMPACT_MODELING__", MacroDef::simple("1"));

        pp
    }

    /// Add an include search path
    pub fn add_include_path(&mut self, path: impl Into<PathBuf>) {
        self.include_paths.push(path.into());
    }

    /// Define a macro
    pub fn define(&mut self, name: impl Into<String>, def: MacroDef) {
        self.macros.insert(name.into(), def);
    }

    /// Check if a macro is defined
    pub fn is_defined(&self, name: &str) -> bool {
        self.macros.contains_key(name)
    }

    /// Undefine a macro
    pub fn undefine(&mut self, name: &str) {
        self.macros.remove(name);
    }

    /// Preprocess a file, returning the expanded source
    pub fn preprocess_file(&mut self, path: &Path) -> Result<String, PreprocessorError> {
        self.preprocess_provider_root(&FileSystemSourceProvider, path)
    }

    /// Preprocess a host file while enforcing the caller's complete closure
    /// limits. This is used when importing third-party model trees: ordinary
    /// compiler entry points remain unbounded unless their own contract says
    /// otherwise, while acquisition cannot traverse an arbitrary host tree.
    pub fn preprocess_file_with_limits(
        &mut self,
        path: &Path,
        limits: SourceProviderLimits,
    ) -> Result<String, PreprocessorError> {
        let provider = LimitedSourceProvider {
            inner: &FileSystemSourceProvider,
            limits,
        };
        self.preprocess_provider_root(&provider, path)
    }

    /// Preprocess a provider-backed root and capture its exact dependency
    /// closure. This is the common loading path for file and virtual sources.
    pub fn preprocess_provider_root(
        &mut self,
        provider: &dyn SourceProvider,
        root: &Path,
    ) -> Result<String, PreprocessorError> {
        self.preprocess_provider_root_mapped(provider, root)
            .map(|preprocessed| preprocessed.source)
    }

    pub(crate) fn preprocess_provider_root_mapped(
        &mut self,
        provider: &dyn SourceProvider,
        root: &Path,
    ) -> Result<PreprocessedSource, PreprocessorError> {
        self.reset_dependency_capture();
        let document = provider.load_root(root)?;
        self.preprocess_document(provider, document, 1)
    }

    /// Return canonical source/include dependencies captured during the most
    /// recent top-level `preprocess_file` call.
    pub fn dependencies(&self) -> Vec<PathBuf> {
        self.dependencies.iter().cloned().collect()
    }

    /// Consume and return the captured dependency list.
    pub fn take_dependencies(&mut self) -> Vec<PathBuf> {
        std::mem::take(&mut self.dependencies).into_iter().collect()
    }

    /// Return exact dependency documents in deterministic first-load order.
    pub fn dependency_documents(&self) -> &[PreprocessedDependency] {
        &self.dependency_documents
    }

    /// Consume exact dependency documents in deterministic first-load order.
    pub fn take_dependency_documents(&mut self) -> Vec<PreprocessedDependency> {
        self.dependency_content.clear();
        self.dependency_source_bytes = 0;
        std::mem::take(&mut self.dependency_documents)
    }

    /// Consume include edges actually resolved during the latest preprocessing.
    pub fn take_include_graph(&mut self) -> Vec<PreprocessedInclude> {
        std::mem::take(&mut self.include_graph)
    }

    fn reset_dependency_capture(&mut self) {
        self.dependencies.clear();
        self.dependency_documents.clear();
        self.dependency_content.clear();
        self.include_graph.clear();
        self.dependency_source_bytes = 0;
        self.include_stack.clear();
        self.current_file = None;
    }

    fn preprocess_document(
        &mut self,
        provider: &dyn SourceProvider,
        document: SourceDocument,
        include_depth: usize,
    ) -> Result<PreprocessedSource, PreprocessorError> {
        let limits = provider.limits();
        if include_depth > limits.max_include_depth {
            return Err(PreprocessorError::new(
                format!(
                    "Include depth exceeds the provider limit of {}",
                    limits.max_include_depth
                ),
                Some(document.logical_path),
                0,
            ));
        }

        let logical_path = document.logical_path.clone();
        if self.include_stack.contains(&logical_path) {
            let mut chain = self
                .include_stack
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>();
            chain.sort();
            chain.push(logical_path.display().to_string());
            return Err(PreprocessorError::new(
                format!("Circular include detected: {}", chain.join(" -> ")),
                Some(logical_path),
                0,
            ));
        }

        self.record_dependency(provider, &document)?;

        // Preserve the historical search behavior: every consumed provider
        // document contributes its containing directory as an include root.
        if document.origin == SourceDocumentOrigin::Provider
            && let Some(parent) = logical_path.parent()
            && !parent.as_os_str().is_empty()
            && !self.include_paths.contains(&parent.to_path_buf())
        {
            self.include_paths.insert(0, parent.to_path_buf());
        }

        self.include_stack.insert(logical_path.clone());
        let previous_file = self.current_file.replace(logical_path.clone());
        let result =
            self.preprocess_source_with_provider(&document.source, provider, include_depth);
        self.include_stack.remove(&logical_path);
        self.current_file = previous_file;
        result
    }

    fn record_dependency(
        &mut self,
        provider: &dyn SourceProvider,
        document: &SourceDocument,
    ) -> Result<(), PreprocessorError> {
        if let Some(existing) = self.dependency_content.get(&document.logical_path) {
            if existing != &document.source {
                return Err(PreprocessorError::new(
                    format!(
                        "Source provider returned different contents for logical path {}",
                        document.logical_path.display()
                    ),
                    Some(document.logical_path.clone()),
                    0,
                ));
            }
            return Ok(());
        }

        let limits = provider.limits();
        let next_count = self.dependency_documents.len().saturating_add(1);
        if next_count > limits.max_dependencies {
            return Err(PreprocessorError::new(
                format!(
                    "Dependency count exceeds the provider limit of {}",
                    limits.max_dependencies
                ),
                Some(document.logical_path.clone()),
                0,
            ));
        }
        let next_bytes = self
            .dependency_source_bytes
            .checked_add(document.source.len())
            .ok_or_else(|| {
                PreprocessorError::new(
                    "Dependency source byte count overflowed",
                    Some(document.logical_path.clone()),
                    0,
                )
            })?;
        if next_bytes > limits.max_total_source_bytes {
            return Err(PreprocessorError::new(
                format!(
                    "Dependency source exceeds the provider limit of {} total bytes",
                    limits.max_total_source_bytes
                ),
                Some(document.logical_path.clone()),
                0,
            ));
        }

        // Preserve the legacy file API contract: `dependencies()` contains
        // canonical provider paths that callers may watch or stat. Built-in
        // headers have no file-system path and live only in the exact document
        // closure exposed by `dependency_documents()`.
        if document.origin == SourceDocumentOrigin::Provider {
            self.dependencies.insert(document.logical_path.clone());
        }
        self.dependency_content
            .insert(document.logical_path.clone(), document.source.clone());
        self.dependency_documents.push(PreprocessedDependency {
            logical_path: document.logical_path.clone(),
            source: document.source.clone(),
            origin: document.origin,
        });
        self.dependency_source_bytes = next_bytes;
        Ok(())
    }

    /// Preprocess source string
    pub fn preprocess_source(&mut self, source: &str) -> Result<String, PreprocessorError> {
        self.preprocess_source_with_provider(source, &FileSystemSourceProvider, 0)
            .map(|preprocessed| preprocessed.source)
    }

    fn preprocess_source_with_provider(
        &mut self,
        source: &str,
        provider: &dyn SourceProvider,
        include_depth: usize,
    ) -> Result<PreprocessedSource, PreprocessorError> {
        let source = strip_utf8_bom(source);
        // Strip comments first so directives inside comments are inert and
        // trailing comments never leak into directive arguments or macro
        // bodies. Newlines are preserved to keep line numbers stable.
        let source = self.strip_comments(source)?;

        let mut output = PreprocessedSource {
            source: String::with_capacity(source.len()),
            segments: Vec::new(),
        };
        let current_path = self
            .current_file
            .clone()
            .unwrap_or_else(|| PathBuf::from("<input>"));
        let mut lines = source.lines().enumerate().peekable();
        let mut include_index = 0usize;

        // Conditional stack: true = include this section, false = skip
        let mut cond_stack: Vec<bool> = Vec::new();
        // Track if we've seen a true branch at each level (for else handling)
        let mut seen_true: Vec<bool> = Vec::new();

        while let Some((line_num, line)) = lines.next() {
            let trimmed = line.trim();

            // Check if we're in a skipped conditional block
            let include_line = cond_stack.iter().all(|&b| b);

            // Handle directives
            let directive_parts = if trimmed.starts_with('`') {
                let (directive, rest) = Self::split_directive(trimmed);
                if Self::is_known_directive(directive) {
                    Some((directive, rest))
                } else {
                    None
                }
            } else {
                None
            };

            if let Some((directive, rest)) = directive_parts {
                match directive {
                    "include" => {
                        let current_include_index = include_index;
                        include_index = include_index.saturating_add(1);
                        if include_line {
                            let included = self.handle_include(
                                provider,
                                rest,
                                line_num + 1,
                                include_depth,
                                current_include_index,
                            )?;
                            output.append(included);
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "define" => {
                        if include_line {
                            // Handle multi-line defines with backslash continuation
                            let mut full_define = rest.to_string();
                            output.push_source("\n", &current_path, line_num + 1);

                            // Check if this define continues on next lines (ends with backslash)
                            while full_define.trim_end().ends_with('\\') {
                                // Remove trailing backslash
                                let trimmed = full_define.trim_end();
                                full_define = trimmed[..trimmed.len() - 1].to_string();
                                full_define.push(' ');

                                // Consume next line
                                if let Some((_, next_line)) = lines.next() {
                                    full_define.push_str(next_line);
                                    output.push_source("\n", &current_path, line_num + 2);
                                } else {
                                    break;
                                }
                            }

                            self.handle_define(&full_define, line_num + 1)?;
                        } else {
                            output.push_source("\n", &current_path, line_num + 1);
                            // Still need to skip continuation lines even when not including
                            let mut current_line = rest;
                            while current_line.trim_end().ends_with('\\') {
                                if let Some((_, next_line)) = lines.next() {
                                    current_line = next_line;
                                    output.push_source("\n", &current_path, line_num + 2);
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    "undef" => {
                        if include_line {
                            let name = rest.trim();
                            self.undefine(name);
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "ifdef" => {
                        let name = rest.trim();
                        let is_defined = self.is_defined(name);
                        let include = include_line && is_defined;
                        cond_stack.push(include);
                        seen_true.push(is_defined);
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "ifndef" => {
                        let name = rest.trim();
                        let is_defined = self.is_defined(name);
                        let include = include_line && !is_defined;
                        cond_stack.push(include);
                        seen_true.push(!is_defined);
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "else" => {
                        if cond_stack.is_empty() {
                            return Err(PreprocessorError::new(
                                "`else without matching `ifdef/`ifndef",
                                self.current_file.clone(),
                                line_num + 1,
                            ));
                        }
                        let parent_active = cond_stack.len() <= 1
                            || cond_stack[..cond_stack.len() - 1].iter().all(|&b| b);
                        let already_true = seen_true.last().copied().unwrap_or(false);
                        let new_val = parent_active && !already_true;
                        *cond_stack.last_mut().unwrap() = new_val;
                        if new_val {
                            *seen_true.last_mut().unwrap() = true;
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "elsif" => {
                        if cond_stack.is_empty() {
                            return Err(PreprocessorError::new(
                                "`elsif without matching `ifdef/`ifndef",
                                self.current_file.clone(),
                                line_num + 1,
                            ));
                        }
                        let name = rest.trim();
                        let parent_active = cond_stack.len() <= 1
                            || cond_stack[..cond_stack.len() - 1].iter().all(|&b| b);
                        let already_true = seen_true.last().copied().unwrap_or(false);
                        let is_defined = self.is_defined(name);
                        let new_val = parent_active && !already_true && is_defined;
                        *cond_stack.last_mut().unwrap() = new_val;
                        if new_val {
                            *seen_true.last_mut().unwrap() = true;
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "endif" => {
                        if cond_stack.is_empty() {
                            return Err(PreprocessorError::new(
                                "`endif without matching `ifdef/`ifndef",
                                self.current_file.clone(),
                                line_num + 1,
                            ));
                        }
                        cond_stack.pop();
                        seen_true.pop();
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    _ => unreachable!("is_known_directive() filtered the directive set"),
                }
            } else if include_line {
                // Regular line (possibly starting with a macro invocation).
                // Expand macros inline so that `define/`undef ordering is
                // honored. A function-like invocation may span multiple
                // lines, so pull lines until its parentheses balance.
                let mut logical = line.to_string();
                while self.invocation_needs_more_input(&logical) {
                    match lines.next() {
                        Some((_, next_line)) => {
                            logical.push('\n');
                            logical.push_str(next_line);
                        }
                        None => break,
                    }
                }

                let expanded = self.expand_macros_at(
                    &logical,
                    line_num + 1,
                    provider.limits().max_expanded_bytes,
                )?;

                // Preserve the overall line count even if the expansion
                // swallowed embedded newlines from a multi-line invocation.
                let consumed_newlines = logical.matches('\n').count();
                let emitted_newlines = expanded.matches('\n').count();
                output.push_source(&expanded, &current_path, line_num + 1);
                output.push_source("\n", &current_path, line_num + 1 + emitted_newlines);
                for _ in emitted_newlines..consumed_newlines {
                    output.push_source("\n", &current_path, line_num + 1);
                }
            } else {
                // Skipped line - preserve line count
                output.push_source("\n", &current_path, line_num + 1);
            }

            self.ensure_expanded_size(&output.source, provider)?;
        }

        // Check for unclosed conditionals
        if !cond_stack.is_empty() {
            return Err(PreprocessorError::new(
                format!("Unclosed conditional ({} `endif missing)", cond_stack.len()),
                self.current_file.clone(),
                0,
            ));
        }

        self.ensure_expanded_size(&output.source, provider)?;

        Ok(output)
    }

    fn ensure_expanded_size(
        &self,
        output: &str,
        provider: &dyn SourceProvider,
    ) -> Result<(), PreprocessorError> {
        let limit = provider.limits().max_expanded_bytes;
        if output.len() > limit {
            return Err(PreprocessorError::new(
                format!("Expanded source exceeds the provider limit of {limit} bytes"),
                self.current_file.clone(),
                0,
            ));
        }
        Ok(())
    }

    /// Directive names processed by the preprocessor itself. Anything else
    /// starting with a backtick is a macro invocation (or an unknown
    /// directive left for downstream stages).
    fn is_known_directive(name: &str) -> bool {
        matches!(
            name,
            "include" | "define" | "undef" | "ifdef" | "ifndef" | "else" | "elsif" | "endif"
        )
    }

    /// Remove comments while preserving line structure and string literals.
    fn strip_comments(&self, source: &str) -> Result<String, PreprocessorError> {
        let mut out = String::with_capacity(source.len());
        let mut chars = source.chars().peekable();
        let mut line = 1usize;
        let mut in_string = false;

        while let Some(ch) = chars.next() {
            if ch == '\n' {
                line += 1;
                out.push(ch);
                in_string = false; // strings do not span raw newlines
                continue;
            }

            if in_string {
                out.push(ch);
                if ch == '\\' {
                    if let Some(&next) = chars.peek() {
                        out.push(next);
                        chars.next();
                        if next == '\n' {
                            line += 1;
                        }
                    }
                } else if ch == '"' {
                    in_string = false;
                }
                continue;
            }

            match ch {
                '"' => {
                    in_string = true;
                    out.push(ch);
                }
                '/' => match chars.peek() {
                    Some('/') => {
                        // Line comment: drop to end of line (newline kept by outer loop)
                        while let Some(&next) = chars.peek() {
                            if next == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    }
                    Some('*') => {
                        let start_line = line;
                        chars.next(); // consume '*'
                        let mut prev = '\0';
                        let mut closed = false;
                        for next in chars.by_ref() {
                            if next == '\n' {
                                line += 1;
                                out.push('\n');
                            }
                            if prev == '*' && next == '/' {
                                closed = true;
                                break;
                            }
                            prev = next;
                        }
                        if !closed {
                            return Err(PreprocessorError::new(
                                "Unterminated block comment",
                                self.current_file.clone(),
                                start_line,
                            ));
                        }
                        out.push(' ');
                    }
                    _ => out.push(ch),
                },
                _ => out.push(ch),
            }
        }

        Ok(out)
    }

    /// Check whether `text` ends inside the argument list of a defined
    /// function-like macro invocation (so the invocation continues on the
    /// next physical line).
    fn invocation_needs_more_input(&self, text: &str) -> bool {
        let mut chars = text.chars().peekable();
        let mut in_string = false;

        while let Some(ch) = chars.next() {
            if in_string {
                if ch == '\\' {
                    chars.next();
                } else if ch == '"' {
                    in_string = false;
                }
                continue;
            }
            match ch {
                '"' => in_string = true,
                '`' => {
                    let mut name = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let is_function_like = self
                        .macros
                        .get(&name)
                        .is_some_and(|def| !def.params.is_empty());
                    if is_function_like && chars.peek() == Some(&'(') {
                        chars.next(); // consume '('
                        let mut depth = 1usize;
                        let mut arg_string = false;
                        loop {
                            let Some(c) = chars.next() else {
                                return true; // ran out before the args closed
                            };
                            if arg_string {
                                if c == '\\' {
                                    chars.next();
                                } else if c == '"' {
                                    arg_string = false;
                                }
                                continue;
                            }
                            match c {
                                '"' => arg_string = true,
                                '(' => depth += 1,
                                ')' => {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        false
    }

    /// Split a directive line into directive name and rest
    fn split_directive(line: &str) -> (&str, &str) {
        let line = &line[1..]; // Skip the backtick
        if let Some(space_pos) = line.find(|c: char| c.is_whitespace()) {
            (&line[..space_pos], line[space_pos..].trim())
        } else {
            (line, "")
        }
    }

    /// Handle `include directive
    fn handle_include(
        &mut self,
        provider: &dyn SourceProvider,
        rest: &str,
        line_num: usize,
        include_depth: usize,
        include_index: usize,
    ) -> Result<PreprocessedSource, PreprocessorError> {
        let rest = rest.trim();

        // Parse filename from "file" or <file>, tolerating trailing text
        // (comments are already stripped, but be robust to stray tokens).
        let filename = if let Some(stripped) = rest.strip_prefix('"') {
            match stripped.find('"') {
                Some(end) => &stripped[..end],
                None => {
                    return Err(PreprocessorError::new(
                        format!("Unterminated include filename: {}", rest),
                        self.current_file.clone(),
                        line_num,
                    ));
                }
            }
        } else if let Some(stripped) = rest.strip_prefix('<') {
            match stripped.find('>') {
                Some(end) => &stripped[..end],
                None => {
                    return Err(PreprocessorError::new(
                        format!("Unterminated include filename: {}", rest),
                        self.current_file.clone(),
                        line_num,
                    ));
                }
            }
        } else {
            return Err(PreprocessorError::new(
                format!("Invalid include syntax: {}", rest),
                self.current_file.clone(),
                line_num,
            ));
        };

        // Resolve through the caller-selected provider. There is deliberately
        // no file-system fallback here: virtual compilation must remain sealed
        // to its exact source bundle.
        if let Some(document) =
            provider.resolve_include(self.current_file.as_deref(), &self.include_paths, filename)?
        {
            if let Some(including_path) = self.current_file.clone() {
                self.include_graph.push(PreprocessedInclude {
                    including_path,
                    requested_path: filename.to_owned(),
                    include_index,
                    included_path: document.logical_path.clone(),
                });
            }
            return self.preprocess_document(provider, document, include_depth.saturating_add(1));
        }

        if let Some((logical_path, source)) = crate::stdlib::builtin_include_document(filename) {
            if let Some(including_path) = self.current_file.clone() {
                self.include_graph.push(PreprocessedInclude {
                    including_path,
                    requested_path: filename.to_owned(),
                    include_index,
                    included_path: PathBuf::from(logical_path),
                });
            }
            return self.preprocess_document(
                provider,
                SourceDocument {
                    logical_path: PathBuf::from(logical_path),
                    source: source.to_owned(),
                    origin: SourceDocumentOrigin::BuiltIn,
                },
                include_depth.saturating_add(1),
            );
        }

        Err(PreprocessorError::new(
            format!("Include file not found: {filename}"),
            self.current_file.clone(),
            line_num,
        ))
    }

    /// Handle `define directive
    fn handle_define(&mut self, rest: &str, line_num: usize) -> Result<(), PreprocessorError> {
        let rest = rest.trim();

        // The macro name is the leading identifier.
        let name_end = rest
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(rest.len());
        let name = &rest[..name_end];
        if name.is_empty() {
            return Err(PreprocessorError::new(
                format!("Invalid macro definition: {}", rest),
                self.current_file.clone(),
                line_num,
            ));
        }
        let after_name = &rest[name_end..];

        // A macro is function-like only when '(' immediately follows the
        // name (LRM 10.2). `define X (expr) defines a simple macro whose
        // body happens to start with a parenthesis.
        if let Some(after_paren) = after_name.strip_prefix('(') {
            let Some(close_paren) = after_paren.find(')') else {
                return Err(PreprocessorError::new(
                    format!("Unterminated macro parameter list for `{}", name),
                    self.current_file.clone(),
                    line_num,
                ));
            };

            let params_str = &after_paren[..close_paren];
            let params: Vec<String> = params_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            for param in &params {
                let valid = !param.is_empty()
                    && param
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                    && param.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
                if !valid {
                    return Err(PreprocessorError::new(
                        format!("Invalid macro parameter '{}' for `{}", param, name),
                        self.current_file.clone(),
                        line_num,
                    ));
                }
            }

            let body = after_paren[close_paren + 1..].trim().to_string();
            self.define(name, MacroDef::with_params(params, body));
        } else {
            let body = after_name.trim();
            self.define(name, MacroDef::simple(body));
        }

        Ok(())
    }

    /// Expand macros in text (with recursive expansion for nested macros)
    pub fn expand_macros(&self, line: &str) -> Result<String, PreprocessorError> {
        self.expand_macros_at(line, 0, usize::MAX)
    }

    /// Expand macros in text, reporting errors at the given line number
    fn expand_macros_at(
        &self,
        line: &str,
        line_num: usize,
        max_expanded_bytes: usize,
    ) -> Result<String, PreprocessorError> {
        const MAX_EXPANSION_DEPTH: usize = 100;

        let mut result = line.to_string();
        if result.len() > max_expanded_bytes {
            return Err(self.macro_expansion_limit_error(max_expanded_bytes, line_num));
        }
        for _ in 0..MAX_EXPANSION_DEPTH {
            let expanded = self.expand_macros_single_pass(&result, max_expanded_bytes, line_num)?;
            if expanded == result {
                return Ok(result); // Fixed point reached
            }
            result = expanded;
        }

        // Still changing after the depth limit: macro expansion does not
        // terminate (mutually recursive definitions). Fail loudly instead
        // of emitting partially-expanded text.
        Err(PreprocessorError::new(
            "Macro expansion did not terminate (recursive `define?)",
            self.current_file.clone(),
            line_num,
        ))
    }

    /// Single pass of macro expansion
    fn expand_macros_single_pass(
        &self,
        line: &str,
        max_expanded_bytes: usize,
        line_num: usize,
    ) -> Result<String, PreprocessorError> {
        let mut result = String::with_capacity(line.len().min(max_expanded_bytes));
        let mut chars = line.chars().peekable();
        let mut in_string = false;

        let append_text = |result: &mut String, value: &str| {
            push_str_bounded(result, value, max_expanded_bytes)
                .map_err(|()| self.macro_expansion_limit_error(max_expanded_bytes, line_num))
        };
        let append_char = |result: &mut String, value: char| {
            push_char_bounded(result, value, max_expanded_bytes)
                .map_err(|()| self.macro_expansion_limit_error(max_expanded_bytes, line_num))
        };

        while let Some(ch) = chars.next() {
            if ch == '"' && !in_string {
                in_string = true;
                append_char(&mut result, ch)?;
            } else if ch == '"' && in_string {
                in_string = false;
                append_char(&mut result, ch)?;
            } else if ch == '\\' && in_string {
                // Escape in string
                append_char(&mut result, ch)?;
                if let Some(next) = chars.next() {
                    append_char(&mut result, next)?;
                }
            } else if ch == '`' && !in_string {
                // Potential macro invocation
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Some(macro_def) = self.macros.get(&name) {
                    // Check for arguments
                    if !macro_def.params.is_empty() {
                        if chars.peek() != Some(&'(') {
                            return Err(PreprocessorError::new(
                                format!(
                                    "Macro `{} expects {} argument(s) but none were supplied",
                                    name,
                                    macro_def.params.len()
                                ),
                                self.current_file.clone(),
                                0,
                            ));
                        }
                        chars.next(); // consume '('
                        let args = Self::parse_macro_args(&mut chars);
                        if args.len() != macro_def.params.len() {
                            return Err(PreprocessorError::new(
                                format!(
                                    "Macro `{} expects {} argument(s), got {}",
                                    name,
                                    macro_def.params.len(),
                                    args.len()
                                ),
                                self.current_file.clone(),
                                0,
                            ));
                        }
                        let remaining = max_expanded_bytes.saturating_sub(result.len());
                        let expanded =
                            macro_def.expand_bounded(&args, remaining).map_err(|()| {
                                self.macro_expansion_limit_error(max_expanded_bytes, line_num)
                            })?;
                        append_text(&mut result, &expanded)?;
                    } else {
                        let remaining = max_expanded_bytes.saturating_sub(result.len());
                        let expanded = macro_def.expand_bounded(&[], remaining).map_err(|()| {
                            self.macro_expansion_limit_error(max_expanded_bytes, line_num)
                        })?;
                        append_text(&mut result, &expanded)?;
                    }
                } else {
                    // Not a defined macro, keep as-is (might be a directive)
                    append_char(&mut result, '`')?;
                    append_text(&mut result, &name)?;
                }
            } else {
                append_char(&mut result, ch)?;
            }
        }

        Ok(result)
    }

    fn macro_expansion_limit_error(
        &self,
        max_expanded_bytes: usize,
        line_num: usize,
    ) -> PreprocessorError {
        PreprocessorError::new(
            format!("Macro expansion exceeds the provider limit of {max_expanded_bytes} bytes"),
            self.current_file.clone(),
            line_num,
        )
    }

    /// Parse macro arguments from function-like invocation
    fn parse_macro_args(chars: &mut std::iter::Peekable<std::str::Chars>) -> Vec<String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut paren_depth = 1;
        let mut in_string = false;

        while let Some(ch) = chars.next() {
            if in_string {
                current_arg.push(ch);
                if ch == '\\' {
                    if let Some(next) = chars.next() {
                        current_arg.push(next);
                    }
                } else if ch == '"' {
                    in_string = false;
                }
                continue;
            }
            match ch {
                '"' => {
                    in_string = true;
                    current_arg.push(ch);
                }
                '(' => {
                    paren_depth += 1;
                    current_arg.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        args.push(current_arg.trim().to_string());
                        break;
                    }
                    current_arg.push(ch);
                }
                ',' if paren_depth == 1 => {
                    args.push(current_arg.trim().to_string());
                    current_arg.clear();
                }
                _ => current_arg.push(ch),
            }
        }

        args
    }
}

fn strip_utf8_bom(source: &str) -> &str {
    source.strip_prefix('\u{feff}').unwrap_or(source)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn pp(source: &str) -> String {
        Preprocessor::new()
            .preprocess_source(source)
            .expect("preprocessing failed")
    }

    #[test]
    fn simple_macro_expansion() {
        let out = pp("`define TWO 2\nx = `TWO;\n");
        assert!(out.contains("x = 2;"), "got: {out}");
    }

    #[test]
    fn define_with_space_before_paren_is_simple_macro() {
        // `define GMIN (1e-12) must define a simple macro whose body is
        // "(1e-12)", not a function-like macro with a bogus parameter.
        let out = pp("`define GMIN (1e-12)\ng = `GMIN;\n");
        assert!(out.contains("g = (1e-12);"), "got: {out}");
    }

    #[test]
    fn function_like_macro() {
        let out = pp("`define MAX(a,b) ((a)>(b)?(a):(b))\nm = `MAX(x, y+1);\n");
        assert!(out.contains("m = ((x)>(y+1)?(x):(y+1));"), "got: {out}");
    }

    #[test]
    fn function_like_macro_nested_parens() {
        let out = pp("`define SQ(x) ((x)*(x))\nv = `SQ(f(a,b));\n");
        assert!(out.contains("v = ((f(a,b))*(f(a,b)));"), "got: {out}");
    }

    #[test]
    fn directive_inside_block_comment_is_ignored() {
        let out = pp("/*\n`define HIDDEN 1\n*/\n`ifdef HIDDEN\nbad\n`endif\nok\n");
        assert!(!out.contains("bad"), "got: {out}");
        assert!(out.contains("ok"), "got: {out}");
    }

    #[test]
    fn trailing_comment_on_define_excluded_from_body() {
        let out = pp("`define TMAX 326.85 // celsius limit\nt = `TMAX + 1;\n");
        assert!(out.contains("t = 326.85 + 1;"), "got: {out}");
    }

    #[test]
    fn define_undef_ordering_is_honored() {
        let src = "`define X 1\na = `X;\n`undef X\n`define X 2\nb = `X;\n";
        let out = pp(src);
        assert!(out.contains("a = 1;"), "got: {out}");
        assert!(out.contains("b = 2;"), "got: {out}");
    }

    #[test]
    fn use_before_redefinition_keeps_old_value() {
        let src = "`define V 10\nfirst = `V;\n`define V 20\nsecond = `V;\n";
        let out = pp(src);
        assert!(out.contains("first = 10;"), "got: {out}");
        assert!(out.contains("second = 20;"), "got: {out}");
    }

    #[test]
    fn multiline_define_continuation() {
        let src = "`define BIG(a) ((a) + \\\n  1.0)\nr = `BIG(z);\n";
        let out = pp(src);
        let normalized: String = out.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(normalized.contains("r = ((z) + 1.0);"), "got: {out}");
    }

    #[test]
    fn multiline_invocation_pulls_lines() {
        let src = "`define ADD(a,b) ((a)+(b))\ns = `ADD(x,\n        y);\n";
        let out = pp(src);
        assert!(out.contains("s = ((x)+(y));"), "got: {out}");
    }

    #[test]
    fn ifdef_else_endif() {
        let src = "`define FEATURE\n`ifdef FEATURE\nyes\n`else\nno\n`endif\n";
        let out = pp(src);
        assert!(out.contains("yes"));
        assert!(!out.contains("no"));

        let src2 = "`ifdef MISSING\nyes\n`else\nno\n`endif\n";
        let out2 = pp(src2);
        assert!(!out2.contains("yes"));
        assert!(out2.contains("no"));
    }

    #[test]
    fn nested_ifdef_in_skipped_region() {
        let src = "`ifdef OUTER\n`ifdef INNER\na\n`endif\nb\n`endif\nc\n";
        let out = pp(src);
        assert!(!out.contains('a'));
        assert!(!out.contains('b'));
        assert!(out.contains('c'));
    }

    #[test]
    fn elsif_chain() {
        let src = "`define B\n`ifdef A\n1\n`elsif B\n2\n`else\n3\n`endif\n";
        let out = pp(src);
        assert!(out.contains('2'), "got: {out}");
        assert!(!out.contains('1'));
        assert!(!out.contains('3'));
    }

    #[test]
    fn mutually_recursive_macros_error() {
        let src = "`define A `B\n`define B `A\nx = `A;\n";
        assert!(Preprocessor::new().preprocess_source(src).is_err());
    }

    #[test]
    fn macro_not_expanded_inside_string() {
        let out = pp("`define NAME bob\ns = \"`NAME\";\n");
        assert!(out.contains("\"`NAME\""), "got: {out}");
    }

    #[test]
    fn macro_args_with_string_commas() {
        let out = pp("`define MSG(s) $strobe(s)\n`MSG(\"a, b\");\n");
        assert!(out.contains("$strobe(\"a, b\");"), "got: {out}");
    }

    #[test]
    fn line_count_preserved() {
        let src = "`define X 1\n/* multi\nline\ncomment */\nx = `X;\n";
        let out = pp(src);
        assert_eq!(out.matches('\n').count(), 5, "got: {out:?}");
        // "x = 1;" must sit on the 5th line for span fidelity.
        assert_eq!(out.lines().nth(4), Some("x = 1;"), "got: {out:?}");
    }

    #[test]
    fn unterminated_block_comment_errors() {
        assert!(
            Preprocessor::new()
                .preprocess_source("/* never closed\nmodule x;\n")
                .is_err()
        );
    }

    #[test]
    fn wrong_argument_count_errors() {
        let src = "`define ADD(a,b) ((a)+(b))\nx = `ADD(1);\n";
        assert!(Preprocessor::new().preprocess_source(src).is_err());
    }

    #[test]
    fn word_boundary_in_macro_body_substitution() {
        // Parameter "a" must not be replaced inside the identifier "axis".
        let out = pp("`define F(a) (axis + a)\ny = `F(3);\n");
        assert!(out.contains("y = (axis + 3);"), "got: {out}");
    }

    #[test]
    fn predefined_vams_macros() {
        let out = pp("`ifdef __VAMS_ENABLE__\nvams\n`endif\n");
        assert!(out.contains("vams"));
    }
}
