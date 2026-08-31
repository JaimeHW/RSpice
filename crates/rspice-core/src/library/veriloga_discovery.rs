//! Filesystem discovery for shipped Verilog-A model packs.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

use crate::abort_signal::{AbortSignal, NoAbort};
use crate::resource::{
    ResourceKind, ResourceLimitError, ResourceLimits, ResourceReadError, read_bytes_limited,
};

/// Default cap on filesystem entries inspected by one Verilog-A discovery.
pub const DEFAULT_MAX_VERILOGA_DISCOVERY_FILES: usize = 100_000;

/// Resource policy for one Verilog-A model-library discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerilogADiscoveryLimits {
    /// Maximum directory nesting below the selected root.
    pub max_depth: usize,
    /// Maximum filesystem entries inspected, including directories and
    /// non-Verilog-A files.
    pub max_files: usize,
    /// Maximum bytes read from one candidate source.
    pub max_source_bytes: usize,
    /// Maximum bytes read from all candidate sources combined.
    pub max_total_source_bytes: usize,
}

impl Default for VerilogADiscoveryLimits {
    fn default() -> Self {
        Self::from(ResourceLimits::default())
    }
}

impl From<ResourceLimits> for VerilogADiscoveryLimits {
    fn from(limits: ResourceLimits) -> Self {
        Self {
            max_depth: limits.max_include_depth,
            max_files: DEFAULT_MAX_VERILOGA_DISCOVERY_FILES,
            max_source_bytes: limits.max_netlist_bytes,
            max_total_source_bytes: limits.max_dependency_source_bytes,
        }
    }
}

/// A module-bearing Verilog-A source discovered under a model-library root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAModelEntry {
    /// Stable package identity relative to the library root.
    pub package: String,
    /// Verilog-A source path relative to the library root.
    pub source_path: PathBuf,
    /// Include directories relative to the library root, scoped to the package.
    pub include_dirs: Vec<PathBuf>,
    /// Module names declared by the source file.
    pub modules: Vec<String>,
}

/// Discover module-bearing `.va` files under a Verilog-A model-library root.
pub fn discover_veriloga_models(root: impl AsRef<Path>) -> io::Result<Vec<VerilogAModelEntry>> {
    discover_veriloga_models_with_limits_and_abort(
        root,
        VerilogADiscoveryLimits::default(),
        &NoAbort,
    )
}

/// Discover Verilog-A models with explicit traversal and source-byte limits.
pub fn discover_veriloga_models_with_limits(
    root: impl AsRef<Path>,
    limits: VerilogADiscoveryLimits,
) -> io::Result<Vec<VerilogAModelEntry>> {
    discover_veriloga_models_with_limits_and_abort(root, limits, &NoAbort)
}

/// Discover Verilog-A models with explicit limits and cooperative cancellation.
///
/// Directory symlinks are followed only when their canonical target remains
/// inside `root`. Canonical identities are visited once, so symlink cycles and
/// aliases cannot amplify traversal or source reads.
pub fn discover_veriloga_models_with_limits_and_abort(
    root: impl AsRef<Path>,
    limits: VerilogADiscoveryLimits,
    abort: &dyn AbortSignal,
) -> io::Result<Vec<VerilogAModelEntry>> {
    ensure_not_aborted(abort)?;
    let root = fs::canonicalize(root.as_ref())?;
    if !fs::metadata(&root)?.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!(
                "Verilog-A discovery root is not a directory: {}",
                root.display()
            ),
        ));
    }

    let mut traversal = DiscoveryTraversal::new(&root, limits, abort);
    traversal.collect_directory(&root, 0)?;
    traversal
        .files
        .sort_by(|left, right| left.relative.cmp(&right.relative));

    let include_files = traversal
        .files
        .iter()
        .filter(|file| is_veriloga_include_file(&file.relative))
        .map(|file| file.relative.clone())
        .collect::<Vec<_>>();
    let source_files = traversal
        .files
        .iter()
        .filter(|file| has_extension(&file.relative, "va"))
        .cloned()
        .collect::<Vec<_>>();

    let mut total_source_bytes = 0usize;
    let mut entries = Vec::new();
    for (index, source) in source_files.into_iter().enumerate() {
        if index.is_multiple_of(64) {
            ensure_not_aborted(abort)?;
        }
        let bytes = read_source_limited(&source.canonical, limits.max_source_bytes, abort)?;
        let requested = total_source_bytes.saturating_add(bytes.len());
        ResourceLimitError::ensure(
            ResourceKind::DependencySourceBytes,
            requested,
            limits.max_total_source_bytes,
        )
        .map_err(resource_error_to_io)?;
        total_source_bytes = requested;
        let text = String::from_utf8(bytes).map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Verilog-A source '{}' is not valid UTF-8: {}",
                    source.canonical.display(),
                    error.utf8_error()
                ),
            )
        })?;
        let modules = extract_veriloga_modules(&text, abort)?;
        if modules.is_empty() {
            continue;
        }

        let package = package_name_from_relative_path(&source.relative);
        let package_root = package
            .as_ref()
            .map(|package| PathBuf::from(package.replace('/', std::path::MAIN_SEPARATOR_STR)))
            .unwrap_or_else(|| {
                source
                    .relative
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_default()
            });
        let include_dirs = include_directories_for_package(&package_root, &include_files);

        entries.push(VerilogAModelEntry {
            package: package.unwrap_or_else(|| ".".to_string()),
            source_path: source.relative,
            include_dirs,
            modules,
        });
    }

    entries.sort_by(|left, right| {
        left.package
            .cmp(&right.package)
            .then_with(|| left.source_path.cmp(&right.source_path))
    });
    Ok(entries)
}

#[derive(Debug, Clone)]
struct DiscoveredFile {
    canonical: PathBuf,
    relative: PathBuf,
}

struct DiscoveryTraversal<'a> {
    root: &'a Path,
    limits: VerilogADiscoveryLimits,
    abort: &'a dyn AbortSignal,
    visited_dirs: HashSet<PathBuf>,
    visited_files: HashSet<PathBuf>,
    inspected_entries: usize,
    files: Vec<DiscoveredFile>,
}

impl<'a> DiscoveryTraversal<'a> {
    fn new(root: &'a Path, limits: VerilogADiscoveryLimits, abort: &'a dyn AbortSignal) -> Self {
        Self {
            root,
            limits,
            abort,
            visited_dirs: HashSet::new(),
            visited_files: HashSet::new(),
            inspected_entries: 0,
            files: Vec::new(),
        }
    }

    fn collect_directory(&mut self, directory: &Path, depth: usize) -> io::Result<()> {
        ensure_not_aborted(self.abort)?;
        let canonical = fs::canonicalize(directory)?;
        self.ensure_inside_root(&canonical)?;
        if self.visited_dirs.contains(&canonical) {
            return Ok(());
        }
        if depth > self.limits.max_depth {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Verilog-A discovery depth {depth} exceeds configured limit {} at '{}'",
                    self.limits.max_depth,
                    directory.display()
                ),
            ));
        }
        self.visited_dirs.insert(canonical.clone());

        let mut entries = Vec::new();
        for entry in fs::read_dir(&canonical)? {
            if entries.len().is_multiple_of(64) {
                ensure_not_aborted(self.abort)?;
            }
            self.inspected_entries = self.inspected_entries.saturating_add(1);
            if self.inspected_entries > self.limits.max_files {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Verilog-A discovery file count {} exceeds configured limit {}",
                        self.inspected_entries, self.limits.max_files
                    ),
                ));
            }
            entries.push(entry?);
        }
        entries.sort_by_key(fs::DirEntry::file_name);
        for (index, entry) in entries.into_iter().enumerate() {
            if index.is_multiple_of(64) {
                ensure_not_aborted(self.abort)?;
            }
            let logical_path = entry.path();
            if should_skip_path(&logical_path) {
                continue;
            }
            let metadata = fs::metadata(&logical_path)?;
            let target = fs::canonicalize(&logical_path)?;
            self.ensure_inside_root(&target)?;
            if metadata.is_dir() {
                self.collect_directory(&target, depth.saturating_add(1))?;
            } else if metadata.is_file() && self.visited_files.insert(target.clone()) {
                self.files.push(DiscoveredFile {
                    relative: target
                        .strip_prefix(self.root)
                        .map(Path::to_path_buf)
                        .map_err(|_| escaped_root_error(self.root, &target))?,
                    canonical: target,
                });
            }
        }
        Ok(())
    }

    fn ensure_inside_root(&self, path: &Path) -> io::Result<()> {
        if path == self.root || path.starts_with(self.root) {
            Ok(())
        } else {
            Err(escaped_root_error(self.root, path))
        }
    }
}

fn escaped_root_error(root: &Path, target: &Path) -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "Verilog-A discovery target '{}' escapes canonical root '{}'",
            target.display(),
            root.display()
        ),
    )
}

fn include_directories_for_package(package_root: &Path, include_files: &[PathBuf]) -> Vec<PathBuf> {
    let mut directories = BTreeSet::new();
    for file in include_files
        .iter()
        .filter(|file| file.starts_with(package_root))
    {
        let mut directory = file.parent();
        while let Some(current) = directory {
            if !current.starts_with(package_root) {
                break;
            }
            directories.insert(current.to_path_buf());
            if current == package_root {
                break;
            }
            directory = current.parent();
        }
    }
    directories.into_iter().collect()
}

struct AbortReader<'a, R> {
    inner: R,
    abort: &'a dyn AbortSignal,
}

impl<R: Read> Read for AbortReader<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        ensure_not_aborted(self.abort)?;
        self.inner.read(buffer)
    }
}

fn read_source_limited(path: &Path, limit: usize, abort: &dyn AbortSignal) -> io::Result<Vec<u8>> {
    let file = fs::File::open(path)?;
    let metadata_bytes = usize::try_from(file.metadata()?.len()).unwrap_or(usize::MAX);
    ResourceLimitError::ensure(ResourceKind::DependencySourceBytes, metadata_bytes, limit)
        .map_err(resource_error_to_io)?;
    read_bytes_limited(
        AbortReader { inner: file, abort },
        ResourceKind::DependencySourceBytes,
        limit,
    )
    .map_err(resource_read_error_to_io)
}

fn resource_error_to_io(error: ResourceLimitError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

fn resource_read_error_to_io(error: ResourceReadError) -> io::Error {
    match error {
        ResourceReadError::Io(error) => error,
        ResourceReadError::ResourceLimit(error) => resource_error_to_io(error),
    }
}

fn ensure_not_aborted(abort: &dyn AbortSignal) -> io::Result<()> {
    if abort.is_aborted() {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Verilog-A model discovery aborted",
        ))
    } else {
        Ok(())
    }
}

fn is_veriloga_include_file(path: &Path) -> bool {
    ["va", "vams", "inc", "include", "h"]
        .iter()
        .any(|extension| has_extension(path, extension))
}

fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

fn should_skip_path(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.') || name == "__MACOSX")
}

fn package_name_from_relative_path(path: &Path) -> Option<String> {
    let parts: Vec<String> = path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .components()
        .filter_map(component_to_string)
        .take(2)
        .collect();
    match parts.as_slice() {
        [only] => Some(only.clone()),
        [first, second, ..] => Some(format!("{first}/{second}")),
        [] => None,
    }
}

fn component_to_string(component: Component<'_>) -> Option<String> {
    match component {
        Component::Normal(part) => part.to_str().map(str::to_string),
        _ => None,
    }
}

fn extract_veriloga_modules(source: &str, abort: &dyn AbortSignal) -> io::Result<Vec<String>> {
    let tokens = tokenize_without_comments(source, abort)?;
    let mut modules = Vec::new();
    let mut iter = tokens.iter();
    while let Some(token) = iter.next() {
        if token.eq_ignore_ascii_case("module")
            && let Some(name) = iter.next()
            && is_identifier(name)
        {
            modules.push(name.clone());
        }
    }
    modules.sort();
    modules.dedup();
    Ok(modules)
}

fn tokenize_without_comments(source: &str, abort: &dyn AbortSignal) -> io::Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = source.chars().peekable();
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_string = false;
    let mut string_escape = false;
    let mut index = 0usize;

    while let Some(ch) = chars.next() {
        if index.is_multiple_of(4096) {
            ensure_not_aborted(abort)?;
        }
        index = index.saturating_add(1);
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                let _ = chars.next();
                in_block_comment = false;
            }
            continue;
        }
        if in_string {
            if string_escape {
                string_escape = false;
            } else if ch == '\\' {
                string_escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '/' && chars.peek() == Some(&'/') {
            flush_token(&mut current, &mut tokens);
            let _ = chars.next();
            in_line_comment = true;
            continue;
        }
        if ch == '/' && chars.peek() == Some(&'*') {
            flush_token(&mut current, &mut tokens);
            let _ = chars.next();
            in_block_comment = true;
            continue;
        }
        if ch == '"' {
            flush_token(&mut current, &mut tokens);
            in_string = true;
            continue;
        }

        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' {
            current.push(ch);
        } else {
            flush_token(&mut current, &mut tokens);
        }
    }
    flush_token(&mut current, &mut tokens);
    Ok(tokens)
}

fn flush_token(current: &mut String, tokens: &mut Vec<String>) {
    if !current.is_empty() {
        tokens.push(std::mem::take(current));
    }
}

fn is_identifier(token: &str) -> bool {
    token
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;

    struct TempTree(PathBuf);

    impl TempTree {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "rspice-veriloga-discovery-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("system clock after epoch")
                    .as_nanos()
            ));
            fs::create_dir_all(&path).expect("temporary discovery tree is created");
            Self(path)
        }
    }

    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn discovery_enforces_file_depth_and_source_byte_limits() {
        let tree = TempTree::new("limits");
        let nested = tree.0.join("vendor").join("model").join("code");
        fs::create_dir_all(&nested).expect("nested fixture directory is created");
        fs::write(nested.join("device.va"), "module device(a); endmodule\n")
            .expect("Verilog-A fixture is written");
        fs::write(nested.join("params.inc"), "parameter real p = 1;\n")
            .expect("include fixture is written");

        let base = VerilogADiscoveryLimits {
            max_depth: 8,
            max_files: 8,
            max_source_bytes: 1024,
            max_total_source_bytes: 1024,
        };
        let entries = discover_veriloga_models_with_limits(&tree.0, base)
            .expect("bounded normal tree is discovered");
        assert_eq!(entries.len(), 1);
        assert!(
            entries[0]
                .include_dirs
                .iter()
                .any(|dir| dir.ends_with("vendor/model/code"))
        );

        let error = discover_veriloga_models_with_limits(
            &tree.0,
            VerilogADiscoveryLimits {
                max_files: 1,
                ..base
            },
        )
        .expect_err("second regular file must exceed the file limit");
        assert!(error.to_string().contains("file count"), "{error}");

        let error = discover_veriloga_models_with_limits(
            &tree.0,
            VerilogADiscoveryLimits {
                max_depth: 2,
                ..base
            },
        )
        .expect_err("deep directory must exceed the depth limit");
        assert!(error.to_string().contains("depth"), "{error}");

        let error = discover_veriloga_models_with_limits(
            &tree.0,
            VerilogADiscoveryLimits {
                max_source_bytes: 8,
                ..base
            },
        )
        .expect_err("large source must exceed the per-source limit");
        assert!(
            error.to_string().contains("dependency_source_bytes"),
            "{error}"
        );

        let error = discover_veriloga_models_with_limits(
            &tree.0,
            VerilogADiscoveryLimits {
                max_total_source_bytes: 8,
                ..base
            },
        )
        .expect_err("aggregate source bytes must be bounded independently");
        assert!(
            error.to_string().contains("dependency_source_bytes"),
            "{error}"
        );
    }

    #[test]
    fn discovery_honors_abort_before_touching_the_tree() {
        let missing = std::env::temp_dir().join("rspice-discovery-aborted-missing-root");
        let error = discover_veriloga_models_with_limits_and_abort(
            &missing,
            VerilogADiscoveryLimits::default(),
            &ImmediateAbort,
        )
        .expect_err("immediate abort must win over filesystem access");
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
    }

    #[cfg(unix)]
    #[test]
    fn canonical_visited_set_breaks_directory_symlink_cycles() {
        use std::os::unix::fs::symlink;

        let tree = TempTree::new("cycle");
        let package = tree.0.join("vendor").join("model");
        fs::create_dir_all(&package).expect("package fixture is created");
        fs::write(package.join("device.va"), "module device(a); endmodule\n")
            .expect("Verilog-A fixture is written");
        symlink(&tree.0, package.join("cycle")).expect("directory cycle symlink is created");

        let entries = discover_veriloga_models_with_limits(
            &tree.0,
            VerilogADiscoveryLimits {
                max_depth: 8,
                max_files: 8,
                max_source_bytes: 1024,
                max_total_source_bytes: 1024,
            },
        )
        .expect("canonical directory cycle is visited only once");
        assert_eq!(entries.len(), 1);
    }
}
