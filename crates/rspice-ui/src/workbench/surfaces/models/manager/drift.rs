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
//! every hash this module performs, and `scale.rs` asserts a painted frame
//! performs none: the cost is invisible in the result, which is exactly how a
//! per-frame rehash would survive review.
//!
//! # What can be compared, and what cannot
//!
//! An external library executes from its live path, and the project also keeps
//! the bytes it accepted — so both sides are on hand and the finding can say
//! what changed. A retained or project-owned library keeps exactly one copy of
//! its bytes; if those no longer hash to the pin, the accepted bytes are gone
//! and nothing can honestly be said about the difference.

use super::*;

use crate::workbench::state::{ModelSourceDrift, ModelSourceDriftFinding, ModelSourceDriftScope};

#[cfg(test)]
thread_local! {
    /// How many source blobs this thread has hashed for drift.
    ///
    /// Counted rather than asserted about the result because the result is
    /// identical either way: a workspace that rehashes its whole corpus every
    /// frame and one that rehashes it on an import show the same findings, and
    /// only a counter can tell them apart before a user with a real PDK does.
    pub(crate) static SOURCE_DRIFT_HASHES: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// What a drifted source means for the project, in one sentence.
pub(super) const DRIFT_EFFECT: &str = "A run authenticates every executable source against its pin, so this library is refused \
     until it is re-pinned. Re-pinning accepts the bytes as they are now and publishes a project \
     revision that records the change.";

/// Hashes one source blob, and counts that it did.
///
/// Every byte this workspace hashes for drift goes through here. That is the
/// whole point: one counted door means the paint-path assertion is a fact
/// about the module rather than a fact about the call sites someone remembered.
fn digest_of(bytes: &[u8]) -> String {
    #[cfg(test)]
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
fn scope(state: &AppState) -> ModelSourceDriftScope {
    ModelSourceDriftScope {
        project_revision: state.workspace.project.revision().get(),
        // Cheap by construction: it hashes pins, paths and byte *lengths*, and
        // reads no source byte — so asking it every frame costs nothing the
        // paint-path hash gate would notice.
        catalog_key: state.model_library_manager.design_inspection_catalog_key(),
    }
}

/// Whether the retained report still describes what is in front of us.
pub(in crate::workbench::surfaces) fn needs_scan(state: &AppState) -> bool {
    state.workbench.models_view.source_drift.scanned != Some(scope(state))
}

/// Rehashes every pinned source and records what no longer matches.
pub(in crate::workbench::surfaces) fn scan(state: &mut AppState) {
    let libraries = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            let findings = library_findings(library);
            (!findings.is_empty()).then(|| (library.name.clone(), findings))
        })
        .collect();
    state.workbench.models_view.source_drift = ModelSourceDrift {
        scanned: Some(scope(state)),
        scanned_at: scanned_at(),
        libraries,
    };
}

/// The instant of the scan, in UTC, when this platform offers a clock.
fn scanned_at() -> Option<String> {
    let seconds = i64::try_from(crate::time_compat::unix_epoch().as_secs()).ok()?;
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

/// The findings recorded for one library, or nothing.
pub(in crate::workbench::surfaces) fn findings_for<'a>(
    state: &'a AppState,
    library: &str,
) -> &'a [ModelSourceDriftFinding] {
    state
        .workbench
        .models_view
        .source_drift
        .libraries
        .get(library)
        .map_or(&[], Vec::as_slice)
}

/// Paints the catalog row's drift mark inside the source column.
///
/// Exception-only and right-aligned against the column it qualifies, because
/// it is a fact *about* the source rather than another source column: a row
/// whose bytes still match says nothing at all.
pub(super) fn paint_source_chip(ui: &Ui, rect: egui::Rect, column_end: f32) {
    let t = Tokens::get(ui.ctx());
    ui.painter().text(
        egui::pos2(
            rect.left() + rect.width() * column_end - 5.0,
            rect.center().y,
        ),
        egui::Align2::RIGHT_CENTER,
        "drift",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.warn,
    );
}

/// The selected model's drift banner: both digests, when, and what it means.
///
/// One banner rather than a per-source list, because the pane it sits in is
/// about one model and the list belongs to the library. Opening it is what the
/// resolve flow is for.
pub(super) fn detail_banner(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, library: &str) {
    let findings = findings_for(app.state, library);
    let Some(first) = findings.first() else {
        return;
    };
    let headline = format!(
        "{} of this library's pinned sources no longer hash to the digest the project accepted",
        findings.len()
    );
    let digests = format!(
        "{} · pinned {} → now {}",
        path_label(&first.path),
        first.pinned,
        first.on_disk.as_deref().unwrap_or("unreadable")
    );
    let detected = app
        .state
        .workbench
        .models_view
        .source_drift
        .scanned_at
        .clone()
        .map_or_else(
            || "detected by the last source scan".to_owned(),
            |at| format!("detected {at}"),
        );
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.warn))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width().max(1.0));
            ui.spacing_mut().item_spacing.y = 3.0;
            hub::announced(
                ui,
                RichText::new(&headline)
                    .small()
                    .strong()
                    .color(t.color.warn),
                &headline,
            );
            hub::announced(
                ui,
                RichText::new(&digests)
                    .small()
                    .monospace()
                    .color(t.color.text),
                &digests,
            );
            hub::announced(
                ui,
                RichText::new(&detected).small().color(t.color.text_faint),
                &detected,
            );
            hub::announced(
                ui,
                RichText::new(DRIFT_EFFECT).small().color(t.color.text_dim),
                DRIFT_EFFECT,
            );
            if Button::new("Resolve drift…").show(ui).clicked() {
                app.state.workbench.models_view.dialog =
                    Some(ModelsWorkbenchDialog::ResolveDrift {
                        library: library.to_owned(),
                    });
            }
        });
}

/// The resolve dialog: every finding this library has, and the one repair.
///
/// It opens on the findings a scan actually produced rather than re-running
/// the check, so what a reader decided against is what they were shown. The
/// repair is a re-pin — the same refresh the source pane offers — which reads
/// the bytes as they are now and publishes a revision recording the change.
pub(super) fn resolve_dialog(ui: &mut Ui, app: &ManagerRenderContext<'_>, library: &str) {
    let findings = findings_for(app.state, library).to_vec();
    let t = Tokens::get(ui.ctx());
    ui.label(format!(
        "'{library}' pinned {} source{} that no longer hash to the digests this project accepted.",
        findings.len(),
        if findings.len() == 1 { "" } else { "s" }
    ));
    ui.label(RichText::new(DRIFT_EFFECT).small().color(t.color.text_dim));
    card(ui, |ui| {
        card_title(ui, "FINDINGS", Some("pinned → present"));
        if findings.is_empty() {
            empty_state(
                ui,
                "This library's sources all match their pins.",
                "The last scan found nothing to resolve here.",
            );
            return;
        }
        ScrollArea::vertical()
            .id_salt("models-drift-findings")
            .max_height(260.0)
            .show(ui, |ui| {
                for finding in &findings {
                    property(
                        ui,
                        &path_label(&finding.path),
                        &format!(
                            "{} → {}",
                            finding.pinned,
                            finding.on_disk.as_deref().unwrap_or("unreadable")
                        ),
                        finding.change.as_deref().unwrap_or(match finding.on_disk {
                            None => "the pinned source could not be read",
                            Some(_) => "the accepted bytes are gone, so the change cannot be shown",
                        }),
                    );
                }
            });
    });
}

/// Why the re-pin cannot run right now, if it cannot.
///
/// The dialog's footer owns the commit, so the reason a commit is blocked has
/// to travel there with it.
pub(super) fn repin_block_reason(
    app: &ManagerRenderContext<'_>,
    library: &str,
) -> Option<&'static str> {
    if findings_for(app.state, library).is_empty() {
        return Some("There is nothing here to re-pin.");
    }
    app.state
        .workbench
        .models_view
        .model_import_in_progress
        .then_some("Another model-source operation is still running.")
}

#[cfg(test)]
mod tests;
