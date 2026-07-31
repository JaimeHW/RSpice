//! Every `.rs` file under `src/` must be part of the crate.
//!
//! Rust does not complain about a source file nobody declared. It is simply
//! not compiled — invisible to `cargo build`, `cargo clippy`, `cargo test`,
//! and every lint and coverage tool. The file still looks live: it sits in the
//! module tree it belongs to, it reads like the code around it, and `grep`
//! finds it.
//!
//! That is not hypothetical here. Three files had drifted out of the crate:
//!
//! - `src/diagnostics.rs`
//! - `src/netlist/commands.rs`
//! - `src/netlist/helpers.rs`
//!
//! The last one is why this test exists. Commit `ec566f064`, "fix(netlist):
//! accept vendor model-name and mfg annotation grammar", rewrote
//! `expect_model_name` in `helpers.rs` — 44 lines, a fresh doc comment
//! explaining `2N3819-VSH` and `BC547A/PLP`, a switch to `consume_node_label`.
//! None of it reached the binary. The change that shipped was the parallel
//! edit to `parser/values.rs`. The two files had been function-for-function
//! duplicates for months, and nothing said so.
//!
//! A stale duplicate of live code is worse than no code: it answers the
//! question "where does this behavior live?" with a confident lie.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// File stems this file pulls into the crate, via `mod foo;` / `pub mod foo;`
/// / `pub(crate) mod foo;`, or `include!("foo.rs")`. Inline `mod foo { .. }`
/// bodies are ignored — they need no file, so counting them would let an
/// orphan slip through beside any module with a `mod tests { .. }`.
///
/// `include!` matters because generated code uses it: the Verilog-A registry
/// is `include!("registry.rs")` inside `veriloga_generated/mod.rs`, which is a
/// perfectly real way to be part of the crate and must not read as an orphan.
///
/// Attributes may sit between the visibility and the keyword in real code
/// (`#[cfg(feature = "simd")] pub mod simd;`), but those are on their own
/// line, so a per-line scan is sufficient and cheap.
fn declared_submodules(source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for line in source.lines() {
        let line = match line.find("//") {
            Some(index) => &line[..index],
            None => line,
        };
        let line = line.trim();

        // `include!("foo.rs")` — take the file stem.
        if let Some(rest) = line.split_once("include!(").map(|(_, rest)| rest) {
            if let Some(path) = rest.split('"').nth(1) {
                if let Some(stem) = Path::new(path).file_stem().and_then(|stem| stem.to_str()) {
                    names.insert(stem.to_owned());
                }
            }
        }

        let rest = line
            .strip_prefix("pub(crate) ")
            .or_else(|| line.strip_prefix("pub(super) "))
            .or_else(|| line.strip_prefix("pub(in crate) "))
            .or_else(|| line.strip_prefix("pub "))
            .unwrap_or(line);
        let Some(rest) = rest.strip_prefix("mod ") else {
            continue;
        };
        // `mod foo;` is a file-backed declaration. `mod foo {` is inline and
        // needs no file, so it must not count as a declaration of one.
        let Some(name) = rest.strip_suffix(';') else {
            continue;
        };
        let name = name.trim();
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            names.insert(name.to_owned());
        }
    }
    names
}

/// The files that could declare `path` as a module: the `mod.rs` beside it, or
/// the `foo.rs` sibling of its `foo/` directory. `src/lib.rs` covers the crate
/// root.
fn possible_parents(root: &Path, path: &Path) -> Vec<PathBuf> {
    let dir = path
        .parent()
        .expect("source file has a parent")
        .to_path_buf();
    if dir == root {
        return vec![root.join("lib.rs")];
    }
    let mut parents = vec![dir.join("mod.rs")];
    if let Some(name) = dir.file_name() {
        if let Some(grandparent) = dir.parent() {
            parents.push(grandparent.join(format!("{}.rs", name.to_string_lossy())));
        }
    }
    parents
}

#[test]
fn every_source_file_is_reachable_from_a_mod_declaration() {
    let root = src_dir();
    let mut orphans = Vec::new();

    for path in rust_sources(&root) {
        let stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("source file stem");
        // Crate root and directory roots are declared by their own existence.
        if stem == "lib" || stem == "mod" {
            continue;
        }

        let declared = possible_parents(&root, &path).into_iter().any(|parent| {
            fs::read_to_string(&parent)
                .map(|source| declared_submodules(&source).contains(stem))
                .unwrap_or(false)
        });

        if !declared {
            orphans.push(
                path.strip_prefix(&root)
                    .unwrap_or(&path)
                    .display()
                    .to_string()
                    .replace('\\', "/"),
            );
        }
    }

    assert!(
        orphans.is_empty(),
        "source files that are not part of the crate:\n{}\n\n\
         Nothing declares these as modules, so they are never compiled, \
         linted, or tested — but they still read as live code. Either add \
         the `mod` declaration that makes them real, or delete them.",
        orphans
            .iter()
            .map(|path| format!("  src/{path}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn declared_submodules_ignores_inline_module_bodies() {
    let source = "\
pub mod real_file;
mod private_file;
pub(crate) mod scoped_file;
#[cfg(test)]
mod tests {
    fn inner() {}
}
mod inline_with_brace { }
";
    let declared = declared_submodules(source);
    assert!(declared.contains("real_file"));
    assert!(declared.contains("private_file"));
    assert!(declared.contains("scoped_file"));
    // `mod tests { .. }` and `mod inline_with_brace { }` are inline bodies,
    // not file-backed declarations. Counting them would let an orphan named
    // `tests.rs` slip through beside any file with a test module.
    assert!(!declared.contains("tests"));
    assert!(!declared.contains("inline_with_brace"));
}
