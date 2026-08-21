//! The crate's public surface may shrink, never grow.
//!
//! `rspice-core` exports 4,831 public items. Its five frontends — the CLI,
//! the GUI, the Python and WASM bindings, and the conformance suite — name
//! roughly two hundred distinct paths between them. The rest is internal
//! machinery that happens to be spelled `pub`.
//!
//! That has a cost even though the crate is `publish = false` and carries no
//! semver obligation. Every `pub fn` is a thing a refactor must consider
//! moving, a name rustdoc must list, and a hint to the next reader that
//! somebody outside might depend on this. When almost everything is public,
//! `pub` stops carrying information.
//!
//! This test does not judge which items should be private — that is Phase 9,
//! driven by what the frontends actually import. It exists so the number
//! cannot drift upward while the earlier phases are moving code around. New
//! code should be `pub(crate)` unless a frontend needs it.
//!
//! # What is counted
//!
//! Item declarations written `pub fn` / `pub struct` / `pub enum` /
//! `pub trait` / `pub type` / `pub const` / `pub static`, plus `pub use`
//! re-export statements, at the start of a line after indentation.
//!
//! The unit is the *statement*, not the name. A grouped re-export —
//! `pub use foo::{A, B, C};` — counts once while exposing three names. That
//! makes the number a proxy rather than a census, and it means relocating a
//! type behind a grouped re-export reads as +1 even though the set of public
//! names did not change.
//!
//! # Raising the ceiling
//!
//! Two cases justify it, and both require saying so in the commit:
//!
//! 1. A move that provably preserves the set of public names, by the number of
//!    re-export statements it adds.
//! 2. A deliberate new public API — an entry point a frontend is meant to
//!    call.
//!
//! Nothing else. In particular, "the number went up because I added a helper"
//! is the case this test exists to catch: make it `pub(crate)`. Prefer
//! investigating a rise to explaining it away.
//!
//! `pub(crate)`, `pub(super)` and `pub(in ...)` are deliberately *not*
//! counted: restricting visibility is the direction this test wants, so
//! narrowing `pub` to `pub(crate)` lowers the number, which is the point.
//!
//! Generated Verilog-A is excluded for the same reason as in the layering
//! ratchet: it is machine output, and its size is the generator's business.

use std::fs;
use std::path::{Path, PathBuf};

/// Ceiling on public items. Lower it whenever the real count drops. The build
/// fails if the count exceeds this, and also if it falls far enough below that
/// the ceiling has gone stale.
///
/// Raise it only for an item a frontend is meant to call, and say which one in
/// the commit. "Never raise it" would be the simpler rule and it is the wrong
/// one: it does not stop a public surface from growing, it stops a feature
/// from shipping an API, and the way that gets resolved is by marking things
/// `pub` somewhere this test does not count. The ceiling exists to make growth
/// a decision, not to forbid it.
///
/// The last raise was +1 for `CircuitData::independent_source_names`, which
/// the GUI's noise form calls to offer the elaborated excitations a run can
/// actually be referred to. The listing that existed required an authored
/// transient waveform, so it hid the DC- and AC-only sources that noise is
/// usually referred to.
const MAX_PUBLIC_ITEMS: usize = 4364;

/// How far under the ceiling the count may sit before the ceiling is
/// considered stale and must be lowered. Without this, a ratchet silently
/// stops ratcheting: the number falls, nobody updates the constant, and the
/// gap quietly becomes headroom for regrowth.
const STALE_CEILING_SLACK: usize = 100;

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
                if path
                    .file_name()
                    .is_some_and(|name| name == "veriloga_builtins")
                {
                    continue;
                }
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

const ITEM_KEYWORDS: &[&str] = &[
    "fn ",
    "struct ",
    "enum ",
    "trait ",
    "type ",
    "const ",
    "static ",
    "unsafe fn ",
    "async fn ",
];

/// Whether a line declares a public item or re-export.
///
/// Takes the line already trimmed of leading whitespace. `pub(` is rejected
/// before the keyword check so restricted visibility never counts.
fn is_public_item(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("pub ") else {
        return false;
    };
    if rest.starts_with("use ") {
        return true;
    }
    ITEM_KEYWORDS
        .iter()
        .any(|keyword| rest.starts_with(keyword))
}

fn count_public_items() -> (usize, Vec<(String, usize)>) {
    let root = src_dir();
    let mut total = 0;
    let mut per_file = Vec::new();
    for path in rust_sources(&root) {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let count = source
            .lines()
            .filter(|line| is_public_item(line.trim_start()))
            .count();
        if count > 0 {
            total += count;
            per_file.push((
                path.strip_prefix(&root)
                    .unwrap_or(&path)
                    .display()
                    .to_string()
                    .replace('\\', "/"),
                count,
            ));
        }
    }
    per_file.sort_by(|a, b| b.1.cmp(&a.1));
    (total, per_file)
}

#[test]
fn public_surface_does_not_grow() {
    let (total, per_file) = count_public_items();

    if total > MAX_PUBLIC_ITEMS {
        let worst: Vec<String> = per_file
            .iter()
            .take(10)
            .map(|(path, count)| format!("  {count:>5}  src/{path}"))
            .collect();
        panic!(
            "public surface grew: {total} items, ceiling is {MAX_PUBLIC_ITEMS}.\n\n\
             Largest contributors:\n{}\n\n\
             New items should be `pub(crate)` unless a frontend needs them. \
             The CLI, GUI, Python and WASM bindings, and the conformance \
             suite together name about two hundred distinct paths; anything \
             outside that set is internal machinery and should say so.",
            worst.join("\n")
        );
    }

    assert!(
        total + STALE_CEILING_SLACK >= MAX_PUBLIC_ITEMS,
        "public surface is {total} items but the ceiling is still \
         {MAX_PUBLIC_ITEMS}, a gap of {}.\n\n\
         Lower MAX_PUBLIC_ITEMS in tests/public_surface.rs to {total}. A \
         ceiling left far above the real count is not a ratchet — it is \
         headroom for the surface to grow back into.",
        MAX_PUBLIC_ITEMS - total
    );
}

#[test]
fn restricted_visibility_does_not_count_as_public() {
    assert!(is_public_item("pub fn solve()"));
    assert!(is_public_item("pub struct Circuit"));
    assert!(is_public_item("pub use crate::circuit::CircuitData;"));
    assert!(is_public_item("pub const GMIN: f64 = 1e-12;"));

    // The whole point: narrowing visibility must lower the count, so these
    // are not public for this test's purposes.
    assert!(!is_public_item("pub(crate) fn stamp()"));
    assert!(!is_public_item("pub(super) struct State"));
    assert!(!is_public_item("pub(in crate::engine) fn drive()"));
    assert!(!is_public_item("pub(crate) use super::Thing;"));

    // Not item declarations. The struct field is passed already trimmed,
    // as the counter sees it, so this exercises the keyword check rather
    // than the leading whitespace.
    assert!(!is_public_item("fn private()"));
    assert!(!is_public_item("pub node_pos: Vec<NodeId>,"));
    assert!(!is_public_item("// pub fn commented_out()"));
}
