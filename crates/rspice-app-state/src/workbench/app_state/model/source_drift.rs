//! Whether the bytes behind a model still hash to what the project accepted.
//!
//! A model library's `source_closure` records one digest per source file, and
//! that record is the whole of a project's claim to reproducibility: every run
//! re-authenticates its executable sources against it and refuses when they
//! disagree. The workspace could show the pins and could not show whether they
//! still held, so the first news of an edited PDK file arrived as a preflight
//! refusal minutes before a run — with the file named and nothing said about
//! what had changed in it.
//!
//! # Hashing happens on events, never on a frame
//!
//! Deciding this reads and hashes every retained byte, which for a foundry
//! import is megabytes. So it happens when something could have changed it —
//! the workspace opening, an import completing, a reader asking — and the
//! result is retained beside the project revision *and* the catalogue content
//! it describes, so it cannot outlive either. [`SOURCE_DRIFT_HASHES`] counts
//! every hash this module performs, and the models workspace's scale test
//! asserts a painted frame performs none: the cost is invisible in the result, which is exactly how a
//! per-frame rehash would survive review.
//!
//! # What can be compared, and what cannot
//!
//! An external library executes from its live path, and the project also keeps
//! the bytes it accepted — so both sides are on hand and the finding can say
//! what changed. A retained or project-owned library keeps exactly one copy of
//! its bytes; if those no longer hash to the pin, the accepted bytes are gone
//! and nothing can honestly be said about the difference.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::state::model_library::{ModelLibrary, ModelSourceAuthority, short_digest};

use super::Model;

/// One retained source whose bytes no longer hash to what the project accepted.
///
/// The pinned digest is the whole of a project's claim to reproducibility: a
/// run authenticates every executable source against it and refuses when they
/// disagree. Discovering that disagreement in a preflight refusal, minutes
/// before a run, is the expensive place to discover it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSourceDriftFinding {
    pub path: PathBuf,
    /// The digest the accepted closure recorded, shortened for display.
    pub pinned: String,
    /// What the bytes hash to now, shortened the same way. `None` means the
    /// source could not be read at all, which is its own kind of drift.
    pub on_disk: Option<String>,
    /// What changed, when both the accepted and the present bytes are on hand
    /// to compare. A retained source keeps only one copy, so its accepted
    /// bytes are gone and nothing can be said about the difference.
    pub change: Option<String>,
}

/// What a drift report describes: which project, and which catalogue.
///
/// Both halves, because neither alone identifies the thing scanned. The
/// revision moves when a project publishes one, which is what re-arms the scan
/// after an import or a re-pin. The catalogue key is *content*, which is what
/// makes a report expire when the catalogue is replaced wholesale — opening a
/// project, accepting a recovery comparison, restoring a design-history
/// candidate. None of those routes touches this report, and a project file
/// arrives carrying whatever revision it was saved with, so two projects at
/// equal revisions is entirely ordinary. A catalogue cannot present a key it
/// did not earn, however it arrived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelSourceDriftScope {
    pub project_revision: u64,
    pub catalog_key: u64,
}

/// Pinned-versus-present source drift, as of the last scan.
///
/// Deciding this rehashes every retained byte, so it is decided on events —
/// opening the workspace, finishing an import, asking for it — and never on a
/// frame. That is also why the report records what it describes: a clean result
/// from before an import, or from another project, is not a clean result.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelSourceDrift {
    pub scanned: Option<ModelSourceDriftScope>,
    /// When the scan ran, in UTC, when the platform offered a usable clock.
    pub scanned_at: Option<String>,
    /// Findings by library name. A library with none is absent from the map,
    /// so an empty map is the healthy state and says nothing.
    pub libraries: BTreeMap<String, Vec<ModelSourceDriftFinding>>,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many source blobs this thread has hashed for drift.
    ///
    /// Counted rather than asserted about the result because the result is
    /// identical either way: a workspace that rehashes its whole corpus every
    /// frame and one that rehashes it on an import show the same findings, and
    /// only a counter can tell them apart before a user with a real PDK does.
    pub static SOURCE_DRIFT_HASHES: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

impl Model {
    /// Whether the retained drift report still describes what is in front of
    /// us.
    pub fn source_drift_needs_scan(&self) -> bool {
        self.source_drift.scanned != Some(source_drift_scope(self))
    }

    /// Rehashes every pinned source and records what no longer matches.
    pub fn scan_source_drift(&mut self) {
        let libraries = self
            .model_library_manager
            .libraries_sorted()
            .into_iter()
            .filter_map(|library| {
                let findings = library_findings(library);
                (!findings.is_empty()).then(|| (library.name.clone(), findings))
            })
            .collect();
        self.source_drift = ModelSourceDrift {
            scanned: Some(source_drift_scope(self)),
            scanned_at: scanned_at(),
            libraries,
        };
    }

    /// The drift findings recorded for one library, or nothing.
    pub fn source_drift_findings(&self, library: &str) -> &[ModelSourceDriftFinding] {
        self.source_drift
            .libraries
            .get(library)
            .map_or(&[], Vec::as_slice)
    }
}

/// Hashes one source blob, and counts that it did.
///
/// Every byte hashed for drift goes through here. That is the
/// whole point: one counted door means the paint-path assertion is a fact
/// about the module rather than a fact about the call sites someone remembered.
fn digest_of(bytes: &[u8]) -> String {
    #[cfg(any(test, feature = "test-support"))]
    SOURCE_DRIFT_HASHES.with(|count| count.set(count.get() + 1));
    use sha2::{Digest as _, Sha256};
    short_digest(
        &crate::product::ContentDigest::from_bytes(Sha256::digest(bytes).into()).to_string(),
    )
}

/// What a scan run now would describe: this project, and this catalogue.
///
/// The revision alone was the latch, and it was not enough. An import, a
/// re-pin and a project-model edit each publish one, so it re-arms every event
/// that *creates* drift — but a project opened in-session replaces the
/// workspace and the model catalogue wholesale and touches no view state at
/// all, and the revision it brings is whatever its file was saved with. Two
/// projects at equal revisions is ordinary, and at equal revisions the latch
/// held: project A's findings, its library names and its digests kept painting
/// over project B until something else happened to move the number.
///
/// The catalogue key closes that without enumerating the routes. It is
/// content, so a replacement cannot present a key it did not earn, however it
/// arrived — the same property the bin inspection and the symbol registry
/// depend on, and the reason neither of them carries a counter.
fn source_drift_scope(model: &Model) -> ModelSourceDriftScope {
    ModelSourceDriftScope {
        project_revision: model.workspace.content.project.revision().get(),
        // Cheap by construction: it hashes pins, paths and byte *lengths*, and
        // reads no source byte — so asking it every frame costs nothing the
        // paint-path hash gate would notice.
        catalog_key: model.model_library_manager.design_inspection_catalog_key(),
    }
}

/// The instant of the scan, in UTC, when this platform offers a clock.
fn scanned_at() -> Option<String> {
    let seconds = i64::try_from(crate::time_compat::checked_unix_time_ms().ok()? / 1_000).ok()?;
    let at = time::OffsetDateTime::from_unix_timestamp(seconds).ok()?;
    Some(format!(
        "{:04}-{:02}-{:02} {:02}:{:02} UTC",
        at.year(),
        u8::from(at.month()),
        at.day(),
        at.hour(),
        at.minute()
    ))
}

/// Every pinned source of one library that no longer hashes to its pin.
fn library_findings(library: &ModelLibrary) -> Vec<ModelSourceDriftFinding> {
    if matches!(library.source_authority, ModelSourceAuthority::BuiltIn) {
        // A built-in catalog has no source deck, so it has no pin to fail.
        return Vec::new();
    }
    let external = matches!(library.source_authority, ModelSourceAuthority::External);
    library
        .source_closure
        .iter()
        .filter_map(|pin| {
            let accepted = library
                .source_contents
                .iter()
                .find(|content| content.path == pin.path)
                .map(|content| content.bytes.as_slice());
            let present = if external {
                read_source(&pin.path)
            } else {
                // Nothing else has a live path: a project-owned root is a
                // regenerated identity and a retained import travels with the
                // project, so the retained bytes are what there is to check.
                accepted.map(<[u8]>::to_vec)
            };
            let pinned = short_digest(&pin.digest.to_string());
            let Some(present) = present else {
                return Some(ModelSourceDriftFinding {
                    path: pin.path.clone(),
                    pinned,
                    on_disk: None,
                    change: None,
                });
            };
            let on_disk = digest_of(&present);
            if on_disk == pinned {
                return None;
            }
            Some(ModelSourceDriftFinding {
                path: pin.path.clone(),
                pinned,
                on_disk: Some(on_disk),
                // Only an external library keeps both copies: the retained
                // bytes it accepted and the live file it executes from.
                change: external
                    .then(|| accepted.and_then(|accepted| line_change(accepted, &present)))
                    .flatten(),
            })
        })
        .collect()
}

/// The bytes at one pinned path, when this platform can read a path at all.
fn read_source(path: &Path) -> Option<Vec<u8>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read(path).ok()
    }
    #[cfg(target_arch = "wasm32")]
    {
        // A browser session has no live filesystem to re-read, so an external
        // library there is reported as unreadable rather than as unchanged.
        let _ = path;
        None
    }
}

/// What changed between two versions of one source, in one short phrase.
///
/// Lines rather than a real diff, because the answer a reader needs here is
/// "how much of this moved" — the exact edit is in the file, and the file is
/// one click away. Binary sources report nothing rather than a lie.
fn line_change(accepted: &[u8], present: &[u8]) -> Option<String> {
    let accepted = std::str::from_utf8(accepted).ok()?;
    let present = std::str::from_utf8(present).ok()?;
    let mut delta = BTreeMap::<&str, isize>::new();
    for line in accepted.lines() {
        *delta.entry(line).or_default() -= 1;
    }
    for line in present.lines() {
        *delta.entry(line).or_default() += 1;
    }
    let added: usize = delta
        .values()
        .filter(|count| **count > 0)
        .map(|count| usize::try_from(*count).unwrap_or_default())
        .sum();
    let removed: usize = delta
        .values()
        .filter(|count| **count < 0)
        .map(|count| count.unsigned_abs())
        .sum();
    Some(if added == 0 && removed == 0 {
        "the same lines, reordered or re-spaced".to_owned()
    } else {
        format!("+{added} −{removed} lines")
    })
}
