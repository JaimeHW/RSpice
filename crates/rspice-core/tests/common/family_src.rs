//! Where a source file of the `rspice-core` family lives.
//!
//! The engine's sources sit under the `src/` of `crates/rspice-core` and of
//! every `crates/rspice-core-<name>` beside it, and a module keeps its path
//! from its crate's root whichever of them holds it. A guard that reads one
//! source file asks [`family_src`] for it by that path; a guard that scans the
//! sources walks every root [`family_src_roots`] returns. Neither may pass on
//! an empty read: a file that moved to another crate of the family is found
//! there, and a file found nowhere, or in two crates, is a failure.
//!
//! Included with `#[path]` rather than through `common/mod.rs`, because the
//! suites that want it are not the suites that want the model-path helpers.
//! A suite uses only some of these functions, hence the allowance.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

/// The directory holding every crate of the workspace.
fn crates_dir() -> &'static Path {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent directory", manifest.display()))
}

/// The `src/` of `rspice-core`, then of every `rspice-core-<name>` crate
/// beside it in name order. Each holds a `lib.rs`.
pub fn family_src_roots() -> Vec<PathBuf> {
    let crates = crates_dir();
    let mut siblings = fs::read_dir(crates)
        .unwrap_or_else(|error| panic!("read {}: {error}", crates.display()))
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("rspice-core-"))
                && path.join("Cargo.toml").is_file()
        })
        .map(|path| path.join("src"))
        .collect::<Vec<_>>();
    siblings.sort();

    let mut roots = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
    roots.extend(siblings);
    for root in &roots {
        assert!(
            root.join("lib.rs").is_file(),
            "{} is a source root of the rspice-core family with no lib.rs",
            root.display()
        );
    }
    roots
}

/// The one family source file at `relative`, a path from a crate's `src/`
/// such as `circuit/introspection.rs`.
///
/// Panics when no crate of the family holds it, or when more than one does:
/// a module has one home, and a guard that read either of two copies would
/// be checking the wrong one half the time.
pub fn family_src(relative: &str) -> PathBuf {
    let roots = family_src_roots();
    let found = roots
        .iter()
        .map(|root| root.join(relative))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    match found.as_slice() {
        [path] => path.clone(),
        [] => panic!(
            "src/{relative} is in no crate of the rspice-core family; searched:\n{}",
            listed(&roots)
        ),
        _ => panic!(
            "src/{relative} is in more than one crate of the rspice-core family:\n{}",
            listed(&found)
        ),
    }
}

/// The text of [`family_src`]`(relative)`.
pub fn read_family_src(relative: &str) -> String {
    let path = family_src(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// `path` as written from `crates/`, such as `rspice-core/src/lib.rs`, for a
/// message that has to say which crate a file is in.
pub fn family_relative(path: &Path) -> String {
    path.strip_prefix(crates_dir())
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

fn listed(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| format!("  {}", path.display()))
        .collect::<Vec<_>>()
        .join("\n")
}
