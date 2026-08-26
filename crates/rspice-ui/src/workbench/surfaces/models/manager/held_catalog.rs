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
    // Reference material, not a transaction: the footer states what is being
    // read rather than offering an action whose only effect is to close.
    let choice = Dialog::new("Model packs", "The held catalog", "Close")
        .description(
            "Who signed the catalog snapshot this client holds, the contract it is accepted \
             under, and the last hub operation this session attempted.",
        )
        // `Manager` is a fixed 760 x 530 surface — `fill_height`, so it is 530
        // tall whatever it holds — and this card holds an identity table, two
        // paragraphs of acceptance contract and one attempt. The contract is
        // the half a reader came for, and at 530 it began below the fold every
        // time.
        //
        // `WideWorkflow` is the widest content-height surface the design system
        // offers, and this dialog family already reads its two release-diff
        // cards under it. Both dimensions earn their place here: the height is
        // what puts the acceptance contract and the last attempt on the page
        // together, and the width is what stops the identity table eliding its
        // own origin column — "covered by the signature; a lower serial is
        // refus…" was the sentence a reader came to this card to read.
        .size(DialogSize::WideWorkflow)
        .primary_on_enter(false)
        .note_only_footer()
        .hint("Model hub · identity, acceptance and history")
        .show(ui.ctx(), |ui| {
            identity(ui, hub);
            contract(ui, hub);
            last_attempt(ui, app);
        });
    if choice == DialogChoice::Cancelled {
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
        // The serial is the signed ordinal, so unlike the generation above it
        // survives a restart — and it is the value the anti-rollback floor is
        // compared against, which is exactly what a reader asking "why was a
        // refresh refused" needs to see.
        property(
            ui,
            "Catalog serial",
            &identity.serial.to_string(),
            "covered by the signature; a lower serial is refused",
        );
        property(
            ui,
            "Signed",
            &identity.generated_at,
            "covered by the signature",
        );
        // Stated as the instant rather than as a verdict, and the verdict is
        // appended only when it is one. A card that said "valid" on every
        // healthy day would be a word nobody reads by the time it matters.
        property(
            ui,
            "Expires",
            &identity.expires_at,
            if hub.expired.is_some() {
                "passed — the hub offers nothing until this is refreshed"
            } else {
                "after this the hub offers nothing until it is refreshed"
            },
        );
        // Elided the way every other digest in this workspace is elided, and
        // by the same function: a 64-character value in a 34%-wide cell is cut
        // off by the painter with no ellipsis and no way to tell how much is
        // missing, which reads as a shorter digest rather than a truncated one.
        property(
            ui,
            "Snapshot digest",
            &short_digest(&identity.digest),
            "sha-256, proved on arrival",
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
            &short_digest(&hub.signing_key),
            "ed25519, compiled into this build",
        );
        property(
            ui,
            "Root anchor",
            "the key above",
            "no file, variable or switch can substitute another",
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
            &format!(
                "{} · {}",
                crate::ui::accessibility::counted(hub.packs.len(), "pack", "packs"),
                crate::ui::accessibility::counted(releases, "release", "releases"),
            ),
            "as the held snapshot lists them",
        );
    });
}

/// The two statements the whole client rests on.
///
/// Both are the same on every host except for one clause: what a failed
/// install leaves behind, which is a directory on a machine and an unreachable
/// staged copy in a browser session. That clause comes from the host rather
/// than from this file, because a card that promised a staging directory to a
/// browser was describing a machine the reader is not using.
fn contract(ui: &mut Ui, hub: &hub::HubCatalog) {
    let t = Tokens::get(ui.ctx());
    let refusal = refusal_contract(hub.host);
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
            ("Nothing local changes on a refusal.", refusal.as_str()),
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

/// What a refused operation leaves behind, on the host this build runs on.
///
/// The shared half is the promise: the held snapshot, the installed packs and
/// every project pin survive a refusal. The half that follows is the one thing
/// the two hosts do differently, and it comes from the host rather than from a
/// literal here so no reader is told about a directory their platform has
/// never had.
fn refusal_contract(host: browser::Host) -> String {
    format!(
        "A refused or failed refresh leaves the held snapshot, the installed packs and every \
         project pin exactly as they are. {}",
        host.install_failure_detail()
    )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_host_states_its_own_failed_install_after_the_shared_promise() {
        for host in [browser::Host::Desktop, browser::Host::Browser] {
            let sentence = refusal_contract(host);
            assert!(
                sentence.starts_with("A refused or failed refresh leaves the held snapshot"),
                "{host:?} keeps the promise that is true on both"
            );
            assert!(sentence.ends_with(host.install_failure_detail()));
        }
        assert_ne!(
            refusal_contract(browser::Host::Desktop),
            refusal_contract(browser::Host::Browser)
        );
    }
}
