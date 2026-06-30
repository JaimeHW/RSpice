//! Filesystem discovery for shipped Verilog-A model packs.

use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};

/// A module-bearing Verilog-A source discovered under a model-library root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogAModelEntry {
    /// Stable package identity relative to the library root, such as
    /// `cmc/BSIM-CMG_112.1.0_04282026`.
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
    let root = root.as_ref();
    let mut source_files = Vec::new();
    collect_veriloga_sources(root, &mut source_files)?;

    let mut entries = Vec::new();
    for source in source_files {
        let text = fs::read_to_string(&source)?;
        let modules = extract_veriloga_modules(&text);
        if modules.is_empty() {
            continue;
        }

        let source_path = path_relative_to(root, &source);
        let package = package_name_from_relative_path(&source_path);
        let package_root = package
            .as_ref()
            .map(|package| root.join(package.replace('/', std::path::MAIN_SEPARATOR_STR)))
            .unwrap_or_else(|| {
                source
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| root.to_path_buf())
            });
        let include_dirs = discover_include_dirs(root, &package_root)?;

        entries.push(VerilogAModelEntry {
            package: package.unwrap_or_else(|| ".".to_string()),
            source_path,
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

fn collect_veriloga_sources(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if should_skip_path(dir) {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if should_skip_path(&path) {
            continue;
        }
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            collect_veriloga_sources(&path, files)?;
        } else if metadata.is_file() && has_extension(&path, "va") {
            files.push(path);
        }
    }
    files.sort();
    Ok(())
}

fn discover_include_dirs(root: &Path, package_root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = BTreeSet::new();
    collect_include_dirs(root, package_root, &mut dirs)?;
    Ok(dirs.into_iter().collect())
}

fn collect_include_dirs(root: &Path, dir: &Path, dirs: &mut BTreeSet<PathBuf>) -> io::Result<bool> {
    if should_skip_path(dir) {
        return Ok(false);
    }

    let mut contains_include_file = false;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if should_skip_path(&path) {
            continue;
        }
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            if collect_include_dirs(root, &path, dirs)? {
                contains_include_file = true;
            }
        } else if metadata.is_file() && is_veriloga_include_file(&path) {
            contains_include_file = true;
        }
    }

    if contains_include_file {
        dirs.insert(path_relative_to(root, dir));
    }
    Ok(contains_include_file)
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

fn path_relative_to(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf())
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

fn extract_veriloga_modules(source: &str) -> Vec<String> {
    let tokens = tokenize_without_comments(source);
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
    modules
}

fn tokenize_without_comments(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = source.chars().peekable();
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut in_string = false;
    let mut string_escape = false;

    while let Some(ch) = chars.next() {
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
    tokens
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
