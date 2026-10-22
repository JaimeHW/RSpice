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

use crate::source::display_path;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};

mod admission;
pub use admission::{BoundedFileSystemSourceProvider, SourceResource, SourceResourceLimit};

/// `__rspice_library (name);`, written ahead of each file of a unit a library
/// map arranged: the file's modules are cells of library `name`.
pub(crate) const LIBRARY_MARKER: &str = "__rspice_library";
/// `__rspice_libraries (a, b, ...);`, written once at the head of such a
/// unit: the map's libraries in declaration order.
pub(crate) const LIBRARY_ORDER_MARKER: &str = "__rspice_libraries";

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

    /// Poll before preprocessing a document or logical line.
    fn checkpoint(&self) -> Result<(), PreprocessorError> {
        Ok(())
    }

    fn limits(&self) -> SourceProviderLimits {
        SourceProviderLimits::UNBOUNDED
    }
}

/// Source provider preserving filesystem include resolution for regular files.
#[derive(Debug, Clone, Copy, Default)]
pub struct FileSystemSourceProvider;

impl FileSystemSourceProvider {
    fn load_path(path: &Path) -> Result<SourceDocument, PreprocessorError> {
        use std::io::Read as _;

        let io_error = |error| PreprocessorError::from_io(error, path);
        let canonical = path.canonicalize().map_err(io_error)?;
        let (mut file, metadata) = open_regular_source(&canonical).map_err(io_error)?;
        let mut source = String::new();
        let capacity = usize::try_from(metadata.len())
            .map_err(|error| io_error(std::io::Error::other(error)))?;
        source
            .try_reserve(capacity)
            .map_err(|error| io_error(std::io::Error::other(error)))?;
        file.read_to_string(&mut source).map_err(io_error)?;
        Ok(SourceDocument::provided(canonical, source))
    }
}

/// Open a filesystem source without waiting for a FIFO writer. Check the
/// opened descriptor as well as the path so replacement during open cannot
/// bypass the regular-file requirement. Symlinks to regular files still work.
fn open_regular_source(path: &Path) -> std::io::Result<(std::fs::File, std::fs::Metadata)> {
    fn ensure_regular(metadata: &std::fs::Metadata) -> std::io::Result<()> {
        if metadata.is_file() {
            Ok(())
        } else {
            Err(std::io::Error::other("source must be a regular file"))
        }
    }
    ensure_regular(&std::fs::metadata(path)?)?;
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.custom_flags(libc::O_NONBLOCK | libc::O_NOCTTY);
    }
    let file = options.open(path)?;
    let metadata = file.metadata()?;
    ensure_regular(&metadata)?;
    Ok((file, metadata))
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
            if include_candidate_exists(&candidate)? {
                return Self::load_path(&candidate).map(Some);
            }
        }

        for include_path in include_paths {
            let candidate = include_path.join(requested);
            if include_candidate_exists(&candidate)? {
                return Self::load_path(&candidate).map(Some);
            }
        }

        Ok(None)
    }
}

// A broken link or an inaccessible entry occupies its place in the include
// search order. Only absence permits falling through to a later directory or
// a built-in header; replacing an unreadable model with another changes it.
fn include_candidate_exists(path: &Path) -> Result<bool, PreprocessorError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(PreprocessorError::from_io(error, path)),
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
    /// One-based column on `line`, when the error is about one construct on it.
    pub column: Option<usize>,
    /// Structured admission failure, independent of human diagnostic text.
    pub resource_limit: Option<SourceResourceLimit>,
    pub cancelled: bool,
    /// Original loader failure, retained across cloned diagnostics so hosts
    /// can distinguish filesystem errors from invalid Verilog-A source.
    pub io_error: Option<std::sync::Arc<std::io::Error>>,
}

impl std::fmt::Display for PreprocessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.file, self.column) {
            (Some(path), Some(column)) => write!(
                f,
                "{}:{}:{column}: {}",
                display_path(path),
                self.line,
                self.message
            ),
            (Some(path), None) => {
                write!(f, "{}:{}: {}", display_path(path), self.line, self.message)
            }
            (None, Some(column)) => write!(f, "line {}:{column}: {}", self.line, self.message),
            (None, None) => write!(f, "line {}: {}", self.line, self.message),
        }
    }
}

impl std::error::Error for PreprocessorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.io_error
            .as_deref()
            .map(|error| error as &(dyn std::error::Error + 'static))
    }
}

impl PreprocessorError {
    /// Stable diagnostic code for this refusal.
    pub fn code(&self) -> &'static str {
        if self.cancelled {
            "VA-INPUT-CANCELLED"
        } else if self.resource_limit.is_some() {
            "VA-INPUT-RESOURCE-LIMIT"
        } else if self.io_error.is_some() {
            "VA-INPUT-IO"
        } else {
            "VA-INPUT-PREPROCESS"
        }
    }

    pub fn new(message: impl Into<String>, file: Option<PathBuf>, line: usize) -> Self {
        Self {
            message: message.into(),
            file,
            line,
            column: None,
            resource_limit: None,
            cancelled: false,
            io_error: None,
        }
    }

    pub fn from_io(source: std::io::Error, file: impl Into<PathBuf>) -> Self {
        let mut error = Self::new(source.to_string(), Some(file.into()), 0);
        error.io_error = Some(std::sync::Arc::new(source));
        error
    }

    pub fn resource_limit(
        resource: SourceResource,
        requested: usize,
        limit: usize,
        file: Option<PathBuf>,
        line: usize,
    ) -> Self {
        let failure = SourceResourceLimit {
            resource,
            requested,
            limit,
        };
        let mut error = Self::new(failure.to_string(), file, line);
        error.resource_limit = Some(failure);
        error
    }

    /// Whether this is the refusal of a backtick name that is neither a
    /// defined macro nor a directive.
    pub fn names_an_undefined_macro(&self) -> bool {
        self.message.starts_with("undefined macro `")
            || self
                .message
                .starts_with("unknown compiler directive or undefined macro `")
            || self.message.starts_with("unknown compiler directive `")
    }

    pub fn cancelled() -> Self {
        let mut error = Self::new("source preprocessing was cancelled", None, 0);
        error.cancelled = true;
        error
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MacroArgumentDelimiter {
    Invocation,
    Parenthesis,
    Brace,
    Bracket,
}

impl MacroArgumentDelimiter {
    const fn opening(self) -> char {
        match self {
            Self::Invocation | Self::Parenthesis => '(',
            Self::Brace => '{',
            Self::Bracket => '[',
        }
    }

    const fn closing(self) -> char {
        match self {
            Self::Invocation | Self::Parenthesis => ')',
            Self::Brace => '}',
            Self::Bracket => ']',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MacroArgumentParseError {
    MissingInvocationClose,
    UnterminatedString,
    UnmatchedClosing(char),
    MismatchedClosing {
        found: char,
        open: MacroArgumentDelimiter,
    },
    Unclosed(MacroArgumentDelimiter),
}

impl std::fmt::Display for MacroArgumentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingInvocationClose => {
                write!(f, "macro invocation is missing its closing ')'")
            }
            Self::UnterminatedString => {
                write!(
                    f,
                    "macro invocation contains an unterminated string literal"
                )
            }
            Self::UnmatchedClosing(found) => write!(
                f,
                "macro invocation contains unmatched closing delimiter '{found}'"
            ),
            Self::MismatchedClosing { found, open } => write!(
                f,
                "macro invocation closes '{}' with '{found}'; expected '{}'",
                open.opening(),
                open.closing()
            ),
            Self::Unclosed(open) => write!(
                f,
                "macro invocation has unclosed delimiter '{}'; expected '{}'",
                open.opening(),
                open.closing()
            ),
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
        Ok(apply_text_operators(result))
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

/// Apply the macro text operators of IEEE 1800-2017 section 22.5.1, which
/// Icarus also accepts for 1364-2005 sources, to an expanded macro body:
/// `` `" `` is a quote that leaves the text between it and the next one open
/// to argument substitution, `` `\`" `` an escaped quote inside such text,
/// and ```` `` ```` joins the text on either side of it.
///
/// Arguments are substituted first, so `` `"val`" `` with `val` bound to `x`
/// becomes the string literal `"x"`. The result is never longer than `text`.
fn apply_text_operators(text: String) -> String {
    if !text.contains('`') {
        return text;
    }
    let mut out = String::with_capacity(text.len());
    let mut rest = text.as_str();
    while let Some(at) = rest.find('`') {
        out.push_str(&rest[..at]);
        let tail = &rest[at..];
        if let Some(after) = tail.strip_prefix("`\"") {
            out.push('"');
            rest = after;
        } else if let Some(after) = tail.strip_prefix("`\\`\"") {
            out.push_str("\\\"");
            rest = after;
        } else if let Some(after) = tail.strip_prefix("``") {
            rest = after;
        } else {
            out.push('`');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
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
    /// Byte and dependency limits apply before loading documents into memory.
    pub fn preprocess_file_with_limits(
        &mut self,
        path: &Path,
        limits: SourceProviderLimits,
    ) -> Result<String, PreprocessorError> {
        self.preprocess_file_with_limits_mapped(path, limits)
            .map(|source| source.source)
    }

    pub(crate) fn preprocess_file_with_limits_mapped(
        &mut self,
        path: &Path,
        limits: SourceProviderLimits,
    ) -> Result<PreprocessedSource, PreprocessorError> {
        let provider = BoundedFileSystemSourceProvider::new(
            limits,
            usize::MAX,
            usize::MAX,
            &crate::NoPipelineControl,
        );
        self.preprocess_provider_root_mapped(&provider, path)
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
        provider.checkpoint()?;
        let document = provider.load_root(root)?;
        self.preprocess_document(provider, document, 1)
    }

    /// Preprocess several roots as one compilation unit, in order. Macros,
    /// `timescale` and the other compiler directives carry from one root into
    /// the next (IEEE 1364-2005 sections 19.3 and 19.8), and the dependency
    /// closure covers every root. Returns the expanded unit and the expanded
    /// byte range each root produced; each root starts on a line of its own.
    ///
    /// With `libraries`, the unit is the one a library map arranged (IEEE
    /// 1364-2005 clause 13): a [`LIBRARY_ORDER_MARKER`] heads it, each root
    /// begins with the [`LIBRARY_MARKER`] of its library, and the map's own
    /// `config` blocks follow the last root, in the library `work`.
    pub(crate) fn preprocess_provider_roots_mapped(
        &mut self,
        provider: &dyn SourceProvider,
        roots: &[&Path],
        libraries: Option<&crate::UnitLibraries>,
    ) -> Result<(PreprocessedSource, Vec<std::ops::Range<usize>>), PreprocessorError> {
        self.reset_dependency_capture();
        let mut unit = PreprocessedSource::default();
        let mut ranges = Vec::with_capacity(roots.len());
        if let (Some(libraries), Some(first)) = (libraries, roots.first()) {
            let mut declared = libraries.declared.clone();
            if !declared.iter().any(|library| library == "work") {
                declared.push("work".into());
            }
            unit.push_source(
                &format!("{LIBRARY_ORDER_MARKER} ({});\n", declared.join(", ")),
                first,
                1,
            );
        }
        for (index, root) in roots.iter().enumerate() {
            provider.checkpoint()?;
            let document = provider.load_root(root)?;
            let logical_path = document.logical_path.clone();
            let mut expanded = self.preprocess_document(provider, document, 1)?;
            if !expanded.source.is_empty() && !expanded.source.ends_with('\n') {
                let line = expanded
                    .segments
                    .last()
                    .map_or(1, |segment| segment.source_line);
                expanded.push_source("\n", &logical_path, line);
            }
            let start = unit.source.len();
            if let Some(library) = libraries.and_then(|libraries| libraries.files.get(index)) {
                unit.push_source(
                    &format!("{LIBRARY_MARKER} ({library});\n"),
                    &logical_path,
                    1,
                );
            }
            unit.append(expanded);
            ranges.push(start..unit.source.len());
        }
        for config in libraries.iter().flat_map(|libraries| &libraries.configs) {
            unit.push_source(
                &format!("{LIBRARY_MARKER} (work);\n"),
                &config.path,
                config.line,
            );
            unit.push_source(&config.text, &config.path, config.line);
            if !config.text.ends_with('\n') {
                unit.push_source("\n", &config.path, config.line);
            }
        }
        Ok((unit, ranges))
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
        provider.checkpoint()?;
        let limits = provider.limits();
        if include_depth > limits.max_include_depth {
            return Err(PreprocessorError::resource_limit(
                SourceResource::IncludeDepth,
                include_depth,
                limits.max_include_depth,
                Some(document.logical_path),
                0,
            ));
        }

        let logical_path = document.logical_path.clone();
        if self.include_stack.contains(&logical_path) {
            let mut chain = self
                .include_stack
                .iter()
                .map(|path| display_path(path))
                .collect::<Vec<_>>();
            chain.sort();
            chain.push(display_path(&logical_path));
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
                        display_path(&document.logical_path)
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
            return Err(PreprocessorError::resource_limit(
                SourceResource::Dependencies,
                next_count,
                limits.max_dependencies,
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
            return Err(PreprocessorError::resource_limit(
                SourceResource::TotalSourceBytes,
                next_bytes,
                limits.max_total_source_bytes,
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
        self.preprocess_source_mapped(source)
            .map(|preprocessed| preprocessed.source)
    }

    /// [`Self::preprocess_source`], keeping each expanded line's origin.
    pub(crate) fn preprocess_source_mapped(
        &mut self,
        source: &str,
    ) -> Result<PreprocessedSource, PreprocessorError> {
        self.preprocess_source_with_provider(source, &FileSystemSourceProvider, 0)
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
            provider.checkpoint()?;
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
                    "timescale" | "resetall" => {
                        if include_line {
                            let timing = if directive == "resetall" {
                                if !rest.trim().is_empty() {
                                    return Err(PreprocessorError::new(
                                        "`resetall takes no operands",
                                        self.current_file.clone(),
                                        line_num + 1,
                                    ));
                                }
                                crate::time_scale::ModuleTimeScale::default()
                            } else {
                                let expanded = self.expand_macros_at(
                                    rest.trim(),
                                    line_num + 1,
                                    provider.limits().max_expanded_bytes,
                                )?;
                                crate::time_scale::ModuleTimeScale::parse(&expanded).map_err(
                                    |message| {
                                        PreprocessorError::new(
                                            message,
                                            self.current_file.clone(),
                                            line_num + 1,
                                        )
                                    },
                                )?
                            };
                            // Retain stream position through nested includes. Parser
                            // state applies it to subsequent module declarations.
                            output.push_source(
                                &format!(
                                    "__rspice_timescale ({}, {});",
                                    timing.unit_exponent(),
                                    timing.precision_exponent()
                                ),
                                &current_path,
                                line_num + 1,
                            );
                            if directive == "resetall" {
                                output.push_source(&format!("__rspice_default_nettype (wire); __rspice_default_discipline (); __rspice_default_transition ({}); __rspice_nounconnected_drive;", crate::semantic::SemanticAnalyzer::SIMULATOR_DEFAULT_TRANSITION), &current_path, line_num + 1);
                            }
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "default_nettype" => {
                        if include_line {
                            let operand = self.expand_macros_at(
                                rest.trim(),
                                line_num + 1,
                                provider.limits().max_expanded_bytes,
                            )?;
                            let operand = operand.trim();
                            if !matches!(operand, "wire" | "tri" | "none") {
                                return Err(PreprocessorError::new(
                                    format!(
                                        "unsupported `default_nettype {operand}`; supported net defaults are wire, tri and none"
                                    ),
                                    self.current_file.clone(),
                                    line_num + 1,
                                ));
                            }
                            output.push_source(
                                &format!("__rspice_default_nettype ({operand});"),
                                &current_path,
                                line_num + 1,
                            );
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "default_transition" => {
                        if include_line {
                            if rest.trim().is_empty() {
                                return Err(PreprocessorError::new(
                                    "`default_transition` requires a constant expression",
                                    self.current_file.clone(),
                                    line_num + 1,
                                ));
                            }
                            let expanded = self.expand_macros_at(
                                rest.trim(),
                                line_num + 1,
                                provider.limits().max_expanded_bytes,
                            )?;
                            let retained = format!("__rspice_default_transition ({expanded});");
                            output.push_source(&retained, &current_path, line_num + 1);
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    "default_discipline" => {
                        // Verilog-AMS LRM 2.4 section 10.2. Retained as a
                        // synthetic statement for the same reason
                        // `default_transition` is: section 10.2 scopes the
                        // directive to "the text stream following" it, and a
                        // preprocessor that consumed it would lose the
                        // position that scope is measured from.
                        //
                        // Syntax 10-1's `qualifier` — `` `default_discipline d
                        // reg `` — is refused rather than dropped. It selects a
                        // default per net *type*, and reading it as the
                        // unqualified form would give every net a default the
                        // author asked only `reg` to have.
                        if include_line {
                            let operand = self.expand_macros_at(
                                rest.trim(),
                                line_num + 1,
                                provider.limits().max_expanded_bytes,
                            )?;
                            let operand = operand.trim();
                            if operand.split_whitespace().count() > 1 {
                                return Err(PreprocessorError::new(
                                    "`default_discipline` with a net-type qualifier is not \
                                     supported; Verilog-AMS LRM 2.4 Syntax 10-1 makes the \
                                     qualifier select a default per net type and this compiler \
                                     applies one default per text stream",
                                    self.current_file.clone(),
                                    line_num + 1,
                                ));
                            }
                            let retained = format!("__rspice_default_discipline ({operand});");
                            output.push_source(&retained, &current_path, line_num + 1);
                        }
                        output.push_source("\n", &current_path, line_num + 1);
                    }
                    _ => unreachable!("is_known_directive() filtered the directive set"),
                }
            } else if include_line && self.leaves_line_to_parser(trimmed) {
                // A directive the parser reads past or refuses by name; its
                // operand is not code, so nothing on the line is expanded.
                output.push_source(line, &current_path, line_num + 1);
                output.push_source("\n", &current_path, line_num + 1);
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

                // Every backtick name left on the line must be a macro: an
                // undefined one used to stay in the text, where the parser
                // dropped it at file scope and misreported it elsewhere.
                if let Some((offset, name)) = self.first_undefined_macro(&logical) {
                    return Err(self.undefined_macro_error(&logical, offset, name, line_num + 1));
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
            return Err(PreprocessorError::resource_limit(
                SourceResource::ExpandedBytes,
                output.len(),
                limit,
                self.current_file.clone(),
                0,
            ));
        }
        Ok(())
    }

    /// Directive names processed by the preprocessor itself. Anything else
    /// starting with a backtick is a macro invocation, one of the directives
    /// the parser reads ([`IGNORED_DIRECTIVES`] and [`DRIVE_DIRECTIVES`]), or
    /// an error.
    fn is_known_directive(name: &str) -> bool {
        KNOWN_DIRECTIVES.contains(&name)
    }

    /// Whether `name` is a directive the preprocessor leaves in the text for
    /// the parser: one it ignores with a warning, or one of the pair that
    /// pulls unconnected input ports, which the parser applies to the modules
    /// that follow.
    fn is_parser_directive(name: &str) -> bool {
        IGNORED_DIRECTIVES.contains(&name) || DRIVE_DIRECTIVES.contains(&name)
    }

    /// Whether `trimmed` opens with a parser directive that no macro shadows.
    fn leaves_line_to_parser(&self, trimmed: &str) -> bool {
        trimmed.strip_prefix('`').is_some_and(|rest| {
            let name = macro_name(rest);
            Self::is_parser_directive(name) && !self.macros.contains_key(name)
        })
    }

    /// The first `` `name `` in `text`, outside string literals, that is not a
    /// defined macro, with the byte offset of its backtick.
    fn first_undefined_macro<'t>(&self, text: &'t str) -> Option<(usize, &'t str)> {
        let mut in_string = false;
        let mut chars = text.char_indices();
        while let Some((offset, ch)) = chars.next() {
            match ch {
                '"' => in_string = !in_string,
                '\\' if in_string => {
                    chars.next();
                }
                '`' if !in_string => {
                    let name = macro_name(&text[offset + 1..]);
                    if !name.is_empty()
                        && !self.macros.contains_key(name)
                        && !PREDEFINED_MACROS.contains(&name)
                    {
                        return Some((offset, name));
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// The refusal of the undefined `` `name `` at byte `offset` of `text`,
    /// whose first line is source line `first_line`.
    fn undefined_macro_error(
        &self,
        text: &str,
        offset: usize,
        name: &str,
        first_line: usize,
    ) -> PreprocessorError {
        let line_start = text[..offset].rfind('\n').map_or(0, |newline| newline + 1);
        let opens_line = text[..offset].trim().is_empty();
        let invoked = text[offset + 1 + name.len()..].starts_with('(');
        // Text that opens its line and takes no arguments may be a directive.
        let directive_position = opens_line && !invoked;
        let directive = closest_name(
            name,
            KNOWN_DIRECTIVES
                .iter()
                .chain(&IGNORED_DIRECTIVES)
                .chain(&DRIVE_DIRECTIVES)
                .copied(),
        )
        .filter(|_| directive_position);
        let message = if Self::is_known_directive(name) || Self::is_parser_directive(name) {
            format!(
                "compiler directive `{name} must begin its line; this compiler reads \
                 directives only at the start of a line"
            )
        } else if let Some(directive) = directive {
            format!("unknown compiler directive `{name} (did you mean `{directive}?)")
        } else {
            let what = if directive_position {
                "unknown compiler directive or undefined macro"
            } else {
                "undefined macro"
            };
            match closest_name(name, self.macros.keys().map(String::as_str)) {
                Some(defined) => format!("{what} `{name} (did you mean `{defined}?)"),
                None => format!("{what} `{name}"),
            }
        };
        let mut error = PreprocessorError::new(
            message,
            self.current_file.clone(),
            first_line + text[..offset].matches('\n').count(),
        );
        error.column = Some(text[line_start..offset].chars().count() + 1);
        error
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

    /// Expand macros in text, reporting errors at the given line number
    fn expand_macros_at(
        &self,
        line: &str,
        line_num: usize,
        max_expanded_bytes: usize,
    ) -> Result<String, PreprocessorError> {
        // Each pass expands one level of nesting, so this bounds how deeply
        // invocations may nest (`F(`F(...)) or a chain of bodies).
        const MAX_EXPANSION_DEPTH: usize = 1024;

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

        // Still changing after the depth limit. Fail loudly instead of
        // emitting partially-expanded text, saying whether a definition
        // reaches itself or the nesting is only deep.
        let message = match self.macro_cycle(line) {
            Some(cycle) => format!(
                "recursive macro `{} ({}): its expansion never ends",
                cycle[0],
                cycle
                    .iter()
                    .map(|name| format!("`{name}"))
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
            None => format!(
                "macro invocations nest more than {MAX_EXPANSION_DEPTH} levels deep; this \
                 preprocessor expands at most that many"
            ),
        };
        Err(PreprocessorError::new(
            message,
            self.current_file.clone(),
            line_num,
        ))
    }

    /// A cycle of macro definitions reachable from the invocations in
    /// `text`, as the names along it with the first repeated at the end.
    fn macro_cycle(&self, text: &str) -> Option<Vec<String>> {
        // Depth-first over "the body of A invokes B", one path at a time.
        fn visit(
            preprocessor: &Preprocessor,
            name: &str,
            path: &mut Vec<String>,
            done: &mut HashSet<String>,
        ) -> Option<Vec<String>> {
            if let Some(start) = path.iter().position(|open| open == name) {
                let mut cycle = path[start..].to_vec();
                cycle.push(name.to_string());
                return Some(cycle);
            }
            if done.contains(name) {
                return None;
            }
            let body = &preprocessor.macros.get(name)?.body;
            path.push(name.to_string());
            for next in invoked_macro_names(body) {
                if let Some(cycle) = visit(preprocessor, next, path, done) {
                    return Some(cycle);
                }
            }
            path.pop();
            done.insert(name.to_string());
            None
        }
        let mut done = HashSet::new();
        invoked_macro_names(text).find_map(|name| visit(self, name, &mut Vec::new(), &mut done))
    }

    /// The value of a predefined macro the source has not defined itself:
    /// IEEE 1800-2017 section 22.13 `` `__FILE__ `` (the current source file
    /// as a string literal) and `` `__LINE__ `` (the line of the use).
    fn predefined_macro(&self, name: &str, line_num: usize) -> Option<String> {
        match name {
            "__FILE__" => {
                let path = self
                    .current_file
                    .as_deref()
                    .map(display_path)
                    .unwrap_or_default();
                Some(format!(
                    "\"{}\"",
                    path.replace('\\', "\\\\").replace('"', "\\\"")
                ))
            }
            "__LINE__" => Some(line_num.to_string()),
            _ => None,
        }
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
                                line_num,
                            ));
                        }
                        chars.next(); // consume '('
                        let args = Self::parse_macro_args(&mut chars).map_err(|error| {
                            PreprocessorError::new(
                                format!("Malformed arguments for macro `{}: {error}", name),
                                self.current_file.clone(),
                                line_num,
                            )
                        })?;
                        if args.len() != macro_def.params.len() {
                            return Err(PreprocessorError::new(
                                format!(
                                    "Macro `{} expects {} argument(s), got {}",
                                    name,
                                    macro_def.params.len(),
                                    args.len()
                                ),
                                self.current_file.clone(),
                                line_num,
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
                } else if let Some(value) = self.predefined_macro(&name, line_num) {
                    append_text(&mut result, &value)?;
                } else if name.is_empty() {
                    // A backtick that names nothing, such as one left by a
                    // stringification operator, is not an invocation.
                    append_char(&mut result, '`')?;
                } else {
                    // A name on the line itself is refused at its column
                    // before expansion; this one came out of a macro body or
                    // a directive's operand.
                    return Err(PreprocessorError::new(
                        format!("undefined macro `{name}"),
                        self.current_file.clone(),
                        line_num,
                    ));
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
        PreprocessorError::resource_limit(
            SourceResource::ExpandedBytes,
            max_expanded_bytes.saturating_add(1),
            max_expanded_bytes,
            self.current_file.clone(),
            line_num,
        )
    }

    /// Parse macro arguments from function-like invocation
    fn parse_macro_args(
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<Vec<String>, MacroArgumentParseError> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut delimiters = vec![MacroArgumentDelimiter::Invocation];
        let mut in_string = false;

        while let Some(ch) = chars.next() {
            if in_string {
                current_arg.push(ch);
                if ch == '\\' {
                    let Some(next) = chars.next() else {
                        return Err(MacroArgumentParseError::UnterminatedString);
                    };
                    current_arg.push(next);
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
                    delimiters.push(MacroArgumentDelimiter::Parenthesis);
                    current_arg.push(ch);
                }
                '{' => {
                    delimiters.push(MacroArgumentDelimiter::Brace);
                    current_arg.push(ch);
                }
                '[' => {
                    delimiters.push(MacroArgumentDelimiter::Bracket);
                    current_arg.push(ch);
                }
                ')' | '}' | ']' => {
                    let Some(open) = delimiters.last().copied() else {
                        return Err(MacroArgumentParseError::UnmatchedClosing(ch));
                    };
                    if open.closing() != ch {
                        if open == MacroArgumentDelimiter::Invocation {
                            return Err(MacroArgumentParseError::UnmatchedClosing(ch));
                        }
                        return Err(MacroArgumentParseError::MismatchedClosing { found: ch, open });
                    }
                    delimiters.pop();
                    if open == MacroArgumentDelimiter::Invocation {
                        args.push(current_arg.trim().to_string());
                        return Ok(args);
                    }
                    current_arg.push(ch);
                }
                ',' if delimiters.as_slice() == [MacroArgumentDelimiter::Invocation] => {
                    args.push(current_arg.trim().to_string());
                    current_arg.clear();
                }
                _ => current_arg.push(ch),
            }
        }

        if in_string {
            return Err(MacroArgumentParseError::UnterminatedString);
        }
        match delimiters.last().copied() {
            Some(MacroArgumentDelimiter::Invocation) => {
                Err(MacroArgumentParseError::MissingInvocationClose)
            }
            Some(open) => Err(MacroArgumentParseError::Unclosed(open)),
            None => Err(MacroArgumentParseError::MissingInvocationClose),
        }
    }
}

fn strip_utf8_bom(source: &str) -> &str {
    source.strip_prefix('\u{feff}').unwrap_or(source)
}

/// Macros every source has without defining them (see
/// `Preprocessor::predefined_macro`).
const PREDEFINED_MACROS: [&str; 2] = ["__FILE__", "__LINE__"];

/// The directives the preprocessor processes itself.
const KNOWN_DIRECTIVES: [&str; 13] = [
    "include",
    "define",
    "undef",
    "ifdef",
    "ifndef",
    "else",
    "elsif",
    "endif",
    "timescale",
    "resetall",
    "default_nettype",
    "default_transition",
    "default_discipline",
];

/// IEEE 1364-2005 clause 19 directives that change nothing an analog model
/// computes: cell marking, keyword-set selection, the line-number remapping of
/// `` `line `` (which only moves where diagnostics point), the implementation
/// pragmas section 19.10 says an unrecognized tool shall ignore. The
/// preprocessor leaves each in the text and the parser reads past it with a
/// warning.
pub(crate) const IGNORED_DIRECTIVES: [&str; 6] = [
    "begin_keywords",
    "celldefine",
    "end_keywords",
    "endcelldefine",
    "line",
    "pragma",
];

/// Section 19.9's pair, which pulls the unconnected input ports of the
/// modules that follow and restores the floating default. The preprocessor
/// leaves both in the text for the parser to apply.
const DRIVE_DIRECTIVES: [&str; 2] = ["unconnected_drive", "nounconnected_drive"];

/// The macro or directive name at the start of `text`, which follows its
/// backtick; empty when no name does.
fn macro_name(text: &str) -> &str {
    let end = text
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .unwrap_or(text.len());
    &text[..end]
}

/// The names `text` invokes with a backtick, outside string literals.
fn invoked_macro_names(text: &str) -> impl Iterator<Item = &str> {
    let mut in_string = false;
    let mut escaped = false;
    text.char_indices().filter_map(move |(offset, ch)| {
        if escaped {
            escaped = false;
            return None;
        }
        match ch {
            '"' => in_string = !in_string,
            '\\' if in_string => escaped = true,
            '`' if !in_string => {
                let name = macro_name(&text[offset + 1..]);
                return (!name.is_empty()).then_some(name);
            }
            _ => {}
        }
        None
    })
}

/// The candidate closest to `name` in edit distance (a transposition counts as
/// one edit), if it is close enough to be a likely misspelling. The engine
/// uses it too, to suggest a loaded module for a misspelled master.
pub fn closest_name<'a>(
    name: &str,
    candidates: impl IntoIterator<Item = &'a str>,
) -> Option<&'a str> {
    let allowed = if name.chars().count() <= 3 { 1 } else { 2 };
    candidates
        .into_iter()
        .filter(|candidate| *candidate != name)
        .map(|candidate| (edit_distance(name, candidate), candidate))
        .filter(|(distance, _)| *distance <= allowed)
        .min()
        .map(|(_, candidate)| candidate)
}

/// Optimal-string-alignment distance between `a` and `b`.
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut rows = vec![vec![0_usize; b.len() + 1]; a.len() + 1];
    for (i, row) in rows.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in rows[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let substitution = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (rows[i - 1][j] + 1)
                .min(rows[i][j - 1] + 1)
                .min(rows[i - 1][j - 1] + substitution);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(rows[i - 2][j - 2] + 1);
            }
            rows[i][j] = best;
        }
    }
    rows[a.len()][b.len()]
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
    fn function_like_macro_array_literal_is_one_argument() {
        let out = pp("`define KEEP(x) x\nv = `KEEP({1.0, 2.0, 3.0});\n");
        assert!(out.contains("v = {1.0, 2.0, 3.0};"), "got: {out}");
    }

    #[test]
    fn function_like_macro_preserves_nested_delimiter_commas() {
        let out = pp("`define PAIR(a,b) first=a; second=b\n\
             `PAIR({f(1, 2), [3, {4, 5}]}, table[{6, 7}, g(8, 9)]);\n");
        assert!(
            out.contains("first={f(1, 2), [3, {4, 5}]}; second=table[{6, 7}, g(8, 9)];"),
            "got: {out}"
        );
    }

    #[test]
    fn function_like_macro_quoted_delimiters_commas_and_escapes_are_literal() {
        let out = pp(r#"`define KEEP(x) x
value = `KEEP("a, \"quoted\", {braces}, [brackets], (parentheses)");
"#);
        assert!(
            out.contains(r#"value = "a, \"quoted\", {braces}, [brackets], (parentheses)";"#),
            "got: {out}"
        );
    }

    #[test]
    fn multiline_balanced_array_arguments_expand_atomically() {
        let out = pp("`define PAIR(a,b) first=a; second=b\n\
             `PAIR({1.0,\n 2.0}, table[3,\n 4]);\n");
        let normalized = out.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(
            normalized.contains("first={1.0, 2.0}; second=table[3, 4];"),
            "got: {out}"
        );
    }

    #[test]
    fn malformed_macro_argument_delimiters_fail_closed() {
        let cases = [
            ("value = `KEEP(1});\n", "unmatched closing delimiter '}'"),
            ("value = `KEEP(1]);\n", "unmatched closing delimiter ']'"),
            ("value = `KEEP({1]);\n", "closes '{' with ']'; expected '}'"),
            ("value = `KEEP([1});\n", "closes '[' with '}'; expected ']'"),
            (
                "value = `KEEP({1, 2\n",
                "unclosed delimiter '{'; expected '}'",
            ),
            (
                "value = `KEEP([1, 2\n",
                "unclosed delimiter '['; expected ']'",
            ),
        ];

        for (invocation, expected) in cases {
            let source = format!("`define KEEP(x) x\n{invocation}");
            let error = Preprocessor::new()
                .preprocess_source(&source)
                .expect_err("malformed delimiters must not be expanded");
            assert_eq!(error.line, 2, "wrong source line for {invocation:?}");
            assert!(
                error.message.contains(expected),
                "unexpected diagnostic for {invocation:?}: {error}"
            );
        }
    }

    #[test]
    fn malformed_unused_macro_argument_cannot_disappear() {
        let error = Preprocessor::new()
            .preprocess_source("`define FIRST(used,unused) used\nvalue = `FIRST(1, {2]);\n")
            .expect_err("an invalid unused argument must still be parsed");
        assert_eq!(error.line, 2);
        assert!(
            error.message.contains("closes '{' with ']'; expected '}'"),
            "unexpected diagnostic: {error}"
        );
    }

    #[test]
    fn unterminated_macro_invocations_report_the_invocation_line() {
        let cases = [
            ("value = `KEEP(1\n", "missing its closing ')'"),
            (
                "value = `KEEP(\"unterminated\n",
                "unterminated string literal",
            ),
        ];
        for (invocation, expected) in cases {
            let source = format!("`define KEEP(x) x\n{invocation}");
            let error = Preprocessor::new()
                .preprocess_source(&source)
                .expect_err("unterminated invocation must fail");
            assert_eq!(error.line, 2, "wrong source line for {invocation:?}");
            assert!(
                error.message.contains(expected),
                "unexpected diagnostic for {invocation:?}: {error}"
            );
        }
    }

    #[test]
    fn missing_and_wrong_macro_arity_report_the_invocation_line() {
        let missing = Preprocessor::new()
            .preprocess_source("`define KEEP(x) x\nvalue = `KEEP;\n")
            .expect_err("missing argument list must fail");
        assert_eq!(missing.line, 2);
        assert!(missing.message.contains("none were supplied"));

        let wrong = Preprocessor::new()
            .preprocess_source("`define PAIR(a,b) a+b\nvalue = `PAIR(1);\n")
            .expect_err("wrong argument count must fail");
        assert_eq!(wrong.line, 2);
        assert!(wrong.message.contains("expects 2 argument(s), got 1"));
    }

    /// An undefined macro or an unknown directive is refused where it is
    /// written, in every position.
    ///
    /// Before, the name stayed in the text: at file scope the parser dropped
    /// it with the rest of its line, so `` `inculde `` and `` `defnie `` compiled
    /// clean into a different model, and elsewhere it surfaced as "Invalid
    /// expression" or "Unsupported module item".
    #[test]
    fn undefined_macros_and_unknown_directives_are_refused_at_their_column() {
        for (source, line, column, message) in [
            (
                "`inculde \"opts.vams\"\n",
                1,
                1,
                "unknown compiler directive `inculde (did you mean `include?)",
            ),
            (
                "`define R 1\n`defnie R_SCALE 2\n",
                2,
                1,
                "unknown compiler directive `defnie (did you mean `define?)",
            ),
            (
                "`MAKE_MODULE(foo)\nmodule m; endmodule\n",
                1,
                1,
                "undefined macro `MAKE_MODULE",
            ),
            (
                "module m;\n  `MPRoz(R, 1k)\nendmodule\n",
                2,
                3,
                "undefined macro `MPRoz",
            ),
            (
                "analog begin\n    `INIT_STUFF\nend\n",
                2,
                5,
                "unknown compiler directive or undefined macro `INIT_STUFF",
            ),
            (
                "`define SCALE 2\nx = V(p) * `SCAEL;\n",
                2,
                12,
                "undefined macro `SCAEL (did you mean `SCALE?)",
            ),
            (
                "`define F(a) (a)\nx = `F(\n  `G);\n",
                3,
                3,
                "undefined macro `G",
            ),
            (
                "x = `ifdef A 1 `else 2 `endif;\n",
                1,
                5,
                "compiler directive `ifdef must begin its line",
            ),
        ] {
            let error = Preprocessor::new()
                .preprocess_source(source)
                .expect_err(source);
            assert_eq!(
                (error.line, error.column),
                (line, Some(column)),
                "{source:?}: {error}"
            );
            assert!(error.message.starts_with(message), "{source:?}: {error}");
        }

        // A name a macro body brings in is refused on the invoking line.
        let error = Preprocessor::new()
            .preprocess_source("`define A `B\nx = `A;\n")
            .expect_err("`B is undefined");
        assert_eq!((error.line, error.column), (2, None));
        assert_eq!(error.message, "undefined macro `B");

        // Text in a branch that is not taken is not examined.
        pp("`ifdef NOPE\n`inculde \"x\"\nx = `UNDEFINED;\n`endif\n");
    }

    /// The IEEE 1364 directives that change nothing an analog model computes
    /// reach the parser as written, operand and all, for it to read past with
    /// a warning, and the `` `unconnected_drive `` pair reaches it to apply.
    #[test]
    fn ignorable_directives_pass_to_the_parser_unexpanded() {
        let source = "`celldefine\n`pragma tool option `UNDEFINED\n`line 3 \"a.v\" 0\n\
                      `begin_keywords \"1364-2005\"\n`end_keywords\n`nounconnected_drive\n\
                      `unconnected_drive pull1\n`endcelldefine\n";
        assert_eq!(pp(source), source);
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
        let error = Preprocessor::new().preprocess_source(src).unwrap_err();
        assert!(
            error
                .message
                .starts_with("recursive macro `A (`A -> `B -> `A)"),
            "{error}"
        );
    }

    /// Deep nesting that terminates is not recursion: 300 nested `F(...)
    /// used to fail as "Macro expansion did not terminate (recursive
    /// `define?)". Nesting past the bound says it is a bound.
    #[test]
    fn deep_macro_nesting_expands_and_its_bound_is_not_called_recursion() {
        let nested = |depth: usize| {
            format!(
                "`define F(a) (a)\ny = {}x{};\n",
                "`F(".repeat(depth),
                ")".repeat(depth)
            )
        };
        let out = pp(&nested(300));
        assert!(out.contains(&format!("y = {}x{};", "(".repeat(300), ")".repeat(300))));
        let error = Preprocessor::new()
            .preprocess_source(&nested(1100))
            .unwrap_err();
        assert!(
            error
                .message
                .starts_with("macro invocations nest more than 1024 levels deep"),
            "{error}"
        );
    }

    /// IEEE 1800-2017 section 22.13: `__FILE__ and `__LINE__, including from
    /// a macro body (the line of the use) and inside an included file. They
    /// used to fail as undefined macros.
    #[test]
    fn file_and_line_macros_name_the_use_site() {
        let out = pp("`define WHERE `__LINE__\n\ny = `__LINE__ + `WHERE;\n");
        assert!(out.contains("y = 3 + 3;"), "got: {out}");

        let bundle = crate::VirtualSourceBundle::from_sources(
            "root.va",
            [
                ("root.va", "a = `__LINE__;\n`include \"dir/child.vams\"\n"),
                ("dir/child.vams", "\nb = `__FILE__; c = `__LINE__;\n"),
            ],
        )
        .unwrap();
        let provider = crate::virtual_source::VirtualBundleProvider::new(
            &bundle,
            crate::VirtualCompileLimits::default(),
        );
        let out = Preprocessor::new()
            .preprocess_provider_root_mapped(&provider, Path::new("root.va"))
            .unwrap()
            .source;
        assert!(out.contains("a = 1;"), "got: {out}");
        assert!(out.contains("b = \"dir/child.vams\"; c = 2;"), "got: {out}");
    }

    #[test]
    fn macro_not_expanded_inside_string() {
        let out = pp("`define NAME bob\ns = \"`NAME\";\n");
        assert!(out.contains("\"`NAME\""), "got: {out}");
    }

    #[test]
    fn macro_text_operators_quote_escape_and_join() {
        // Icarus 13 (`-g2005`) prints `FAILED. x: expected 0111` for the
        // first use; the others follow IEEE 1800-2017 section 22.5.1.
        let out = pp(concat!(
            "`define check(val, exp) $display(\"%s: %b\", `\"val`\", exp);\n",
            "`check(x, 4'b0111)\n",
            "`define Q(a) `\"a says `\\`\"hi`\\`\"`\"\n",
            "s = `Q(bob);\n",
            "`define J(p, q) p``_``q\n",
            "y = `J(left, right);\n",
            "`define PLAIN `\"text`\"\n",
            "t = `PLAIN;\n",
        ));
        assert!(
            out.contains("$display(\"%s: %b\", \"x\", 4'b0111);"),
            "got: {out}"
        );
        assert!(out.contains("s = \"bob says \\\"hi\\\"\";"), "got: {out}");
        assert!(out.contains("y = left_right;"), "got: {out}");
        assert!(out.contains("t = \"text\";"), "got: {out}");
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
