//! Everything doctrinal about the Model Hub, behind one dialog.
//!
//! The pages state none of this while the hub is healthy. Who signed what this
//! client holds, the contract that snapshot is accepted under, and what the
//! last hub operation did are all reference material: a reader needs them once,
//! when something looks wrong or when somebody asks how the trust chain works,
//! and printing them under every selection costs every other reader every time.
//!
//! # Nothing here is computed for the reader
//!
//! Every value is a fact the client already settled elsewhere. The snapshot
//! digest was taken where the bytes arrived and were proved; the signing key is
//! the compiled-in anchor; the licences are the ones the signed manifests
//! publish; the last attempt is the operational state the failing operation
//! already recorded. A card that re-derived any of it would be a second
//! opinion about facts that have exactly one owner — and re-hashing a snapshot
//! to fill a label is work a repainting dialog would repeat sixty times a
//! second.

use super::*;

use crate::workbench::state::ModelsAttemptedOperation;

/// The held catalog's identity, its acceptance contract, and the last attempt.
pub(super) fn dialog(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &hub::HubCatalog) {
    let mut open = true;
    egui::Window::new("The held catalog")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_size(egui::vec2(660.0, 560.0))
        .show(ui.ctx(), |ui| {
            let t = Tokens::get(ui.ctx());
            ui.label(
                RichText::new("MODEL HUB · IDENTITY, ACCEPTANCE AND HISTORY")
                    .small()
                    .color(t.color.text_faint),
            );
            identity(ui, hub);
            contract(ui, hub);
            last_attempt(ui, app);
            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    app.state.workbench.models_view.dialog = None;
                }
            });
        });
    if !open {
        app.state.workbench.models_view.dialog = None;
    }
}

/// What this client holds, and what makes it believable.
fn identity(ui: &mut Ui, hub: &hub::HubCatalog) {
    card(ui, |ui| {
        card_title(ui, "IDENTITY", Some("signed snapshot"));
        let Some(identity) = hub.identity.as_ref() else {
            empty_state(
                ui,
                "No catalog is held.",
                if hub.cache_discarded {
                    "The cached copy failed verification and was discarded, so there is nothing \
                     to describe. Refreshing fetches and proves a new one."
                } else {
                    "Nothing has been fetched yet. Refreshing fetches a snapshot and proves it \
                     under the key compiled into this build."
                },
            );
            return;
        };
        match identity.generation {
            Some(generation) => property(
                ui,
                "Generation",
                &generation.to_string(),
                "reported by the refresh that fetched it",
            ),
            // Not a gap to paper over: the generation is a service handoff
            // field that never travelled inside the signed bytes, so a session
            // that read the snapshot off disk genuinely does not know it.
            None => property(
                ui,
                "Generation",
                "not recorded for the cached copy",
                "refresh to learn it",
            ),
        }
        property(
            ui,
            "Signed",
            &identity.generated_at,
            "covered by the signature",
        );
        property(
            ui,
            "Snapshot digest",
            &identity.digest,
            "proved on arrival, then kept",
        );
        property(
            ui,
            "Snapshot schema",
            &identity.schema.to_string(),
            "refused if this build cannot read it",
        );
        property(
            ui,
            "Signing key",
            &hub.signing_key,
            "ed25519, compiled into this build",
        );
        property(
            ui,
            "Root anchor",
            "the compiled-in key, and nothing else",
            "no file, variable or switch can substitute one",
        );
        property(
            ui,
            "Licence",
            &if hub.licences.is_empty() {
                "no release listed".to_owned()
            } else {
                hub.licences.join(" · ")
            },
            "from each release's signed manifest",
        );
        let releases: usize = hub.packs.iter().map(|pack| pack.releases.len()).sum();
        property(
            ui,
            "Contents",
            &format!("{} packs · {releases} releases", hub.packs.len()),
            "as the held snapshot lists them",
        );
    });
}

/// The two statements the whole client rests on.
fn contract(ui: &mut Ui, hub: &hub::HubCatalog) {
    let t = Tokens::get(ui.ctx());
    card(ui, |ui| {
        card_title(
            ui,
            "ACCEPTANCE",
            Some(if hub.cache_discarded {
                "cache discarded"
            } else if hub.identity.is_some() {
                "verified"
            } else {
                "nothing held"
            }),
        );
        for (headline, detail) in [
            (
                "Nothing is believed because it arrived.",
                "A catalog handoff is a claim about which bytes to fetch, and nothing more. A \
                 downloaded snapshot means nothing until its signature verifies under the key \
                 above, and every release's archive digest is taken from that signed snapshot \
                 rather than from the download — so a compromised service can misdirect a fetch \
                 and still cannot change which bytes this client will accept.",
            ),
            (
                "Nothing local changes on a refusal.",
                "A refused or failed refresh leaves the held snapshot, the installed packs and \
                 every project pin exactly as they are. An install is a staged expansion \
                 followed by a rename, so a failure — including a killed process — leaves a \
                 staging directory and nothing else, which the next start sweeps.",
            ),
        ] {
            hub::announced(
                ui,
                RichText::new(headline).small().strong().color(t.color.text),
                headline,
            );
            ui.label(RichText::new(detail).small().color(t.color.text_dim));
            ui.add_space(4.0);
        }
    });
}

/// What the last hub or model-source operation did, and what that left.
///
/// One attempt rather than a journal, because one attempt is what this client
/// records. A table of plausible-looking history assembled from nothing would
/// be the most convincing lie on the page.
fn last_attempt(ui: &mut Ui, app: &mut ManagerRenderContext<'_>) {
    let t = Tokens::get(ui.ctx());
    let receipt = app.state.workbench.models_view.action_receipt.clone();
    let attempted = app.state.workbench.models_view.attempted_operation.clone();
    let state = app.state.workbench.models_view.operational_state;
    card(ui, |ui| {
        card_title(ui, "LAST ATTEMPT", Some("this session only"));
        let Some(ModelsAttemptedOperation { label, .. }) = attempted else {
            empty_state(
                ui,
                "Nothing has been attempted this session.",
                "Attempts are not journalled across sessions: a saved verdict about a network \
                 and a machine describes a moment that has passed.",
            );
            return;
        };
        property(ui, "Operation", &label, "as the operation named itself");
        match receipt {
            Some(Ok(receipt)) => {
                property(ui, "Outcome", "accepted", "published");
                ui.label(RichText::new(receipt).small().color(t.color.text_dim));
            }
            Some(Err(reason)) => {
                property(ui, "Outcome", state.label(), "refused");
                ui.label(RichText::new(reason).small().color(t.color.err));
                ui.label(
                    RichText::new(state.consequence())
                        .small()
                        .color(t.color.text_dim),
                );
            }
            None => property(
                ui,
                "Outcome",
                "still running",
                "the receipt lands when it finishes",
            ),
        }
    });
}
