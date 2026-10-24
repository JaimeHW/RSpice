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

#[path = "common/family_src.rs"]
mod family_src;

use family_src::{family_relative, family_src_roots};

/// Every `.rs` file of the `rspice-core` family, with the `src/` it sits
/// under. Each crate's root must hold a module beside its `lib.rs`: a root
/// read as empty would let every check below pass on it unread.
fn family_sources() -> Vec<(PathBuf, PathBuf)> {
    let mut found = Vec::new();
    for root in family_src_roots() {
        let files = rust_sources(&root);
        assert!(
            files.iter().any(|file| *file != root.join("lib.rs")),
            "{} holds no source file besides lib.rs",
            root.display()
        );
        found.extend(files.into_iter().map(|file| (root.clone(), file)));
    }
    found
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
/// `include!` matters because it is a perfectly real way for a file to be part
/// of the crate and must not read as an orphan. The Verilog-A registry used to
/// be the live example, pulled in as `include!("registry.rs")`; it compiles
/// inside `rspice-veriloga-models` now, so the crate currently has no such
/// file. The detection stays because the next one should not be an orphan
/// either.
///
/// Attributes may sit between the visibility and the keyword in real code
/// (`#[cfg(feature = "veriloga")] pub mod verilog;`), but those are on their own
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
        if let Some(rest) = line.split_once("include!(").map(|(_, rest)| rest)
            && let Some(path) = rest.split('"').nth(1)
            && let Some(stem) = Path::new(path).file_stem().and_then(|stem| stem.to_str())
        {
            names.insert(stem.to_owned());
        }

        let Some(rest) = module_item(line) else {
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

/// The text after `mod ` on a trimmed module item line, its visibility
/// stripped, or `None` when the line declares no module.
fn module_item(line: &str) -> Option<&str> {
    let rest = if let Some(restricted) = line.strip_prefix("pub(") {
        // Rust also permits pub(self) and pub(in crate::parent), not
        // just the two common crate/super spellings.
        restricted
            .split_once(')')
            .map_or(line, |(_, rest)| rest.trim_start())
    } else {
        line.strip_prefix("pub ").unwrap_or(line)
    };
    rest.strip_prefix("mod ")
}

/// Whether a trimmed line opens an inline `mod <name> {` block.
fn opens_inline_module(line: &str, name: &str) -> bool {
    module_item(line)
        .and_then(|rest| rest.strip_prefix(name))
        .is_some_and(|rest| rest.trim_start().starts_with('{'))
}

/// File stems the inline `mod <name> { .. }` block of `source` pulls into
/// the crate: the `mod foo;` declarations directly in its body.
///
/// A module whose files moved to another crate of the family but which
/// keeps one child here is declared inline, around a re-export of the half
/// that moved and the `mod` of the child it kept; that child's file sits in
/// the directory named for the module, with no `<name>.rs` beside it.
fn inline_module_submodules(source: &str, name: &str) -> BTreeSet<String> {
    let mut body = String::new();
    let mut depth = 0usize;
    for line in source.lines() {
        let code = line.find("//").map_or(line, |index| &line[..index]).trim();
        let opens = code.matches('{').count();
        let closes = code.matches('}').count();
        if depth == 0 {
            if opens_inline_module(code, name) {
                depth = opens.saturating_sub(closes);
                if depth == 0 {
                    break;
                }
            }
            continue;
        }
        if depth == 1 {
            body.push_str(code);
            body.push('\n');
        }
        depth = (depth + opens).saturating_sub(closes);
        if depth == 0 {
            break;
        }
    }
    declared_submodules(&body)
}

/// Whether the file that would declare `path`'s directory as a module
/// declares it inline, with `stem` among that body's file-backed children.
fn declared_by_inline_parent(root: &Path, path: &Path, stem: &str) -> bool {
    let dir = path.parent().expect("source file has a parent");
    let Some(module) = dir.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    if dir == root {
        return false;
    }
    possible_parents(root, &dir.with_extension("rs"))
        .into_iter()
        .any(|holder| {
            fs::read_to_string(&holder)
                .map(|source| inline_module_submodules(&source, module).contains(stem))
                .unwrap_or(false)
        })
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
    if let Some(name) = dir.file_name()
        && let Some(grandparent) = dir.parent()
    {
        parents.push(grandparent.join(format!("{}.rs", name.to_string_lossy())));
    }
    parents
}

#[test]
fn every_source_file_is_reachable_from_a_mod_declaration() {
    let mut orphans = Vec::new();

    for (root, path) in family_sources() {
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
        }) || declared_by_inline_parent(&root, &path, stem);

        if !declared {
            orphans.push(family_relative(&path));
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
            .map(|path| format!("  {path}"))
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
pub(self) mod local_file;
pub(in crate::engine) mod nested_file;
pub(in crate::engine) mod restricted_inline { }
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
    assert!(declared.contains("local_file"));
    assert!(declared.contains("nested_file"));
    // `mod tests { .. }` and `mod inline_with_brace { }` are inline bodies,
    // not file-backed declarations. Counting them would let an orphan named
    // `tests.rs` slip through beside any file with a test module.
    assert!(!declared.contains("tests"));
    assert!(!declared.contains("inline_with_brace"));
    assert!(!declared.contains("restricted_inline"));
}

#[test]
fn an_inline_module_declares_the_files_its_own_body_names() {
    let source = "\
pub mod devices;
pub mod device {
    pub use rspice_core_devices::device::*;
    #[cfg(feature = \"veriloga\")]
    pub mod veriloga;
    #[cfg(test)]
    mod tests {
        mod nested;
    }
}
mod after;
";
    let declared = inline_module_submodules(source, "device");
    assert_eq!(declared, BTreeSet::from(["veriloga".to_owned()]));
    assert!(inline_module_submodules(source, "devices").is_empty());
    assert!(inline_module_submodules(source, "absent").is_empty());
}

/// A module with submodules is `foo.rs` beside `foo/`, never `foo/mod.rs`.
///
/// Both spellings compile and Rust has no opinion between them. The crate
/// does, because it had twenty-nine of one and nineteen of the other, and a
/// convention held half the time is not a convention — it is a coin flip you
/// perform every time you open a directory.
///
/// Sibling style wins on the one thing that is not taste: every file gets a
/// distinct name. `mod.rs` in an editor tab, a fuzzy-finder result, a `grep
/// -l` list or a stack frame identifies nothing, and this crate has enough
/// depth (`device/mosfet/b3soi/fd/`) that the parent directory is not on
/// screen to disambiguate it.
///
/// `device/veriloga_builtins/` is exempt, and the reason has expired. It was
/// emitted by `rspice-veriloga-gen`, so its layout was the generator's call. The
/// directory holds one hand-written file now, and `veriloga_builtins/mod.rs` is
/// exactly what this test exists to forbid. Collapsing it to a sibling
/// `veriloga_builtins.rs` is a one-file move; it is deliberately not done here
/// because the same directory name is what excludes the adapter from
/// `module_layering`, and that exclusion has to survive until its upward edge
/// to `engine` is closed.
#[test]
fn module_files_use_sibling_style_not_mod_rs() {
    let mut offenders = Vec::new();

    for (root, path) in family_sources() {
        if path.file_name().is_some_and(|name| name != "mod.rs") {
            continue;
        }
        let relative = path.strip_prefix(&root).unwrap_or(&path);
        let display = relative.display().to_string().replace('\\', "/");
        if display.starts_with("device/veriloga_builtins/") {
            continue;
        }
        let directory = relative
            .parent()
            .and_then(|dir| dir.to_str())
            .unwrap_or_default()
            .replace('\\', "/");
        offenders.push(format!("  {}  ->  {directory}.rs", family_relative(&path)));
    }

    assert!(
        offenders.is_empty(),
        "these modules still use `mod.rs`; rename each to the sibling form:\n{}",
        offenders.join("\n")
    );
}

/// Whether a glob of `target` re-exports a module of another crate of the
/// family into the inline module of the same name that `source` declares.
fn continues_a_moved_module(target: &str, source: &str) -> bool {
    let Some((krate, module)) = target.split_once("::") else {
        return false;
    };
    krate
        .strip_prefix("rspice_core_")
        .is_some_and(|name| !name.is_empty())
        && !module.is_empty()
        && !module.contains("::")
        && source
            .lines()
            .any(|line| opens_inline_module(line.trim(), module))
}

#[test]
fn a_glob_may_continue_a_moved_module_only_in_its_inline_namesake() {
    let source = "pub mod device {\n    pub use rspice_core_devices::device::*;\n}\n";
    assert!(continues_a_moved_module(
        "rspice_core_devices::device",
        source
    ));
    assert!(!continues_a_moved_module(
        "rspice_core_devices::netlist",
        source
    ));
    assert!(!continues_a_moved_module(
        "rspice_core_devices::device::mosfet",
        source
    ));
    assert!(!continues_a_moved_module("rspice_veriloga::device", source));
    assert!(!continues_a_moved_module(
        "rspice_core_devices::device",
        "pub mod device;\n"
    ));
}

/// A glob import may only join a module file to its own submodules.
///
/// `use super::*` appears 142 times here, and that is deliberate rather than
/// lazy. These files are not modules that happen to glob — they are one
/// module split across files for size, and the glob says exactly that: this
/// file continues the module above it. The reciprocal (`engine/builder.rs`
/// globbing `model_resolution::*`) is the same statement from the other end.
/// Replacing them with explicit lists would tax every move of a helper
/// between sibling files and tell the reader nothing new.
///
/// What does hurt is a glob that crosses into a *foreign* module, because
/// then a bare name in the file has no visible origin and adding an item over
/// there can shadow one over here. There are none today. This keeps it that
/// way.
#[test]
fn glob_imports_stay_within_their_own_module() {
    let mut offenders = Vec::new();

    for (root, path) in family_sources() {
        let relative = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .display()
            .to_string()
            .replace('\\', "/");
        if relative.starts_with("device/veriloga_builtins/") {
            continue;
        }
        let display = family_relative(&path);
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let children = declared_submodules(&source);

        for (number, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            let rest = trimmed
                .strip_prefix("pub use ")
                .or_else(|| trimmed.strip_prefix("use "));
            let Some(rest) = rest else { continue };
            let Some(path_text) = rest.strip_suffix("::*;") else {
                continue;
            };
            let target = path_text.trim().trim_start_matches("self::");
            // `super` is a file continuing its parent; a bare identifier that
            // this file declares as a module is the parent reaching the other
            // way. Anything else names somewhere it does not own.
            //
            // A foreign crate's `prelude` is exempt: `rayon::prelude::*` is
            // the only way to get `par_iter` into scope, and a module named
            // `prelude` is a published invitation to glob it. The rule is
            // about this crate's own module boundaries.
            //
            // So is the last exemption: an inline module that keeps one
            // child here and globs the rest of itself from the family crate
            // its other files moved to is one module continued across the
            // crates, as `super::*` continues one across files.
            if target == "super"
                || children.contains(target)
                || target.ends_with("prelude")
                || continues_a_moved_module(target, &source)
            {
                continue;
            }
            offenders.push(format!("  {display}:{}  {trimmed}", number + 1));
        }
    }

    assert!(
        offenders.is_empty(),
        "glob imports must name `super` or a module this file declares:\n{}",
        offenders.join("\n")
    );
}
