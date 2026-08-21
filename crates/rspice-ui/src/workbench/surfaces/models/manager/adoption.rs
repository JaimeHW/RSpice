//! What a newer release changes, and the one decision that earns.
//!
//! The ledger next door answers "what do this machine and this project hold".
//! This answers the question that follows it: *should any of it move*. Both
//! halves live here because they are one thought — a reader is told what the
//! catalog says is different and, on the same lines, offered the only action
//! that difference justifies.
//!
//! # Nothing here is inferred
//!
//! Every phrase is a comparison of two signed release records, computed by
//! [`crate::state::model_hub::release_diff`] and rendered verbatim. Where the
//! snapshot schema publishes nothing — most importantly, there is still no
//! per-part digest — the pane says so in full rather than letting a silent
//! count imply that a re-listed part carries the same model source. An implied
//! fact is worse than a missing one, because nobody thinks to check it.
//!
//! # Adoption is per part, and always explicit
//!
//! An update is notified and never applied. Adopting moves *one* part: it
//! re-runs the retention path that added it — the same download, the same
//! end-to-end proof under the release key, the same authenticated closure —
//! against the newer release, and moves that part's pin. The older release
//! stays installed, because other parts are still pinned to it, and every
//! other pin stays exactly where it was.

use super::*;

use crate::services::model_hub::ModelHubService;
use crate::state::model_hub::{PartFact, PartStanding, ReleaseDiff};
use crate::state::model_library::PackPartPin;
use crate::workbench::app::ModelHubRequest;

use super::hub::{HubCatalog, HubLedgerRow, announced, announced_widget, plain_list, selected_row};

/// Recomputes the selected pack's "what changed" projection when — and only
/// when — the question changed.
///
/// The question is four things: this catalog, this pack, the release held, and
/// the release offered. A frame that asks the same one again reads the answer
/// already in state. That is the whole point: the ledger projection is rebuilt
/// every frame, and a diff rebuilt with it would walk both releases' entire
/// part lists sixty times a second to redraw a section that cannot have
/// changed.
///
/// The catalog half of the key is the snapshot *digest*, so a catalog replaced
/// wholesale — refreshed, restored, or discarded and re-fetched — can never be
/// served an answer computed from the previous one, however it arrived.
///
/// A refusal from the runtime clears the projection rather than reporting it.
/// The only refusals reachable here are "no catalog" and "the catalog does not
/// publish that release", and neither is a state in which an update was on
/// offer to diff in the first place; the section simply has nothing to say.
pub(super) fn refresh_release_diff(
    service: &ModelHubService,
    state: &mut AppState,
    hub: &HubCatalog,
) {
    let question = selected_row(hub, state).and_then(|row| {
        let held = row.installed.as_ref()?;
        let offered = row.update.as_deref()?;
        Some((
            row.pack_id.clone(),
            held.version.clone(),
            offered.to_owned(),
        ))
    });
    let (Some((pack_id, from, to)), Some(identity)) = (question, hub.identity.as_ref()) else {
        state.workbench.models_view.release_diff = None;
        return;
    };
    let key = crate::state::model_hub::ReleaseDiffKey {
        catalog_digest: identity.digest.clone(),
        pack_id,
        from,
        to,
    };
    if state
        .workbench
        .models_view
        .release_diff
        .as_ref()
        .is_some_and(|held| held.key == key)
    {
        return;
    }
    state.workbench.models_view.release_diff = service
        .hub()
        .and_then(|hub| hub.release_diff(&key.pack_id, &key.from, &key.to).ok());
}

/// The projection the Releases pane grows when an update is on offer.
pub(super) fn pane(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &HubLedgerRow,
    diff: &ReleaseDiff,
) {
    what_changed(ui, diff);
    adopt(ui, app, row, diff);
}

/// What the catalog states the offered release changes about the held one.
fn what_changed(ui: &mut Ui, diff: &ReleaseDiff) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    let headline = format!("WHAT CHANGED · {} → {}", diff.key.from, diff.key.to);
    announced(
        ui,
        RichText::new(&headline)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
        &headline,
    );
    ScrollArea::vertical()
        .id_salt("models-hub-release-diff")
        .max_height(84.0)
        .show(ui, |ui| {
            for added in &diff.added {
                changed_line(ui, "+", added, "added by this release", t.color.text_dim);
            }
            for removed in &diff.removed {
                // ASCII `+`/`-`, not typographic signs. It is the vocabulary
                // every reader already knows a diff by, and it is the pair
                // that is certain to paint at this size on every face.
                changed_line(
                    ui,
                    "-",
                    removed,
                    "not published by this release",
                    t.color.warn,
                );
            }
            for part in &diff.changed {
                changed_line(ui, "~", &part.part_id, &facts_of(part), t.color.warn);
            }
            if diff.is_empty() {
                let phrase = "The catalog states no difference in the part lists.";
                announced(
                    ui,
                    RichText::new(phrase).small().color(t.color.text_faint),
                    phrase,
                );
            }
        });
    for (phrase, colour) in [
        (
            diff.licence
                .as_ref()
                .map(|(from, to)| format!("Licence {from} → {to}")),
            t.color.warn,
        ),
        (
            (!diff.capabilities_added.is_empty())
                .then(|| format!("Now requires {}", plain_list(&diff.capabilities_added))),
            t.color.warn,
        ),
        (
            (!diff.capabilities_removed.is_empty()).then(|| {
                format!(
                    "No longer requires {}",
                    plain_list(&diff.capabilities_removed)
                )
            }),
            t.color.text_dim,
        ),
        (Some(schema_limit(diff)), t.color.text_faint),
    ] {
        if let Some(phrase) = phrase {
            announced(ui, RichText::new(&phrase).small().color(colour), &phrase);
        }
    }
}

/// The honest floor of a diff over the snapshot schema, stated in full.
///
/// It is a sentence rather than a footnote because the thing it guards against
/// is a reader concluding, from a quiet count, that the parts not listed above
/// are the same models. Schema 2 added a description and authored
/// specifications, which the diff now compares — but not a per-part digest, so
/// the strongest true statement is still that the catalog does not say.
fn schema_limit(diff: &ReleaseDiff) -> String {
    format!(
        "{} part{} re-listed. The catalog publishes one digest per release and none per part, so \
         a re-listed part is not evidence that its model source is unchanged.{}",
        diff.relisted,
        if diff.relisted == 1 { "" } else { "s" },
        if diff.archive_differs {
            " The two archives are different documents."
        } else {
            ""
        }
    )
}

/// Every difference the catalog states about one part, in one phrase.
fn facts_of(part: &crate::state::model_hub::ChangedPart) -> String {
    part.facts
        .iter()
        .map(PartFact::describe)
        .collect::<Vec<_>>()
        .join(" · ")
}

/// One line of the diff: a sign, the part, and what the catalog says about it.
fn changed_line(ui: &mut Ui, sign: &str, part_id: &str, detail: &str, colour: Color32) {
    let line = format!("{sign} {part_id} · {detail}");
    announced(ui, RichText::new(&line).small().color(colour), &line);
}

/// One decision per part this project pinned to the release being superseded.
fn adopt(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, row: &HubLedgerRow, diff: &ReleaseDiff) {
    let t = Tokens::get(ui.ctx());
    let pinned = row
        .pins
        .iter()
        .filter(|pin| pin.pack_version == diff.key.from)
        .cloned()
        .collect::<Vec<_>>();
    if pinned.is_empty() {
        return;
    }
    ui.add_space(6.0);
    let headline = format!(
        "ADOPT · {} part{} pinned to {}",
        pinned.len(),
        if pinned.len() == 1 { "" } else { "s" },
        diff.key.from
    );
    announced(
        ui,
        RichText::new(&headline)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
        &headline,
    );
    let idle = !app.state.workbench.models_view.model_import_in_progress;
    for pin in &pinned {
        adopt_line(ui, app, row, diff, pin, idle, &t);
    }
}

/// One pinned part: where it stands in the offered release, and the control
/// that moves it — or the reason there is nothing to move it onto.
fn adopt_line(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    row: &HubLedgerRow,
    diff: &ReleaseDiff,
    pin: &PackPartPin,
    idle: bool,
    t: &Tokens,
) {
    let standing = diff.part_standing(&pin.part_id);
    let summary = match standing {
        PartStanding::Withdrawn => "not published by this release".to_owned(),
        PartStanding::Changed(part) => facts_of(part),
        PartStanding::Relisted => "re-listed".to_owned(),
    };
    // A refusal states its own reason where the action would be, rather than
    // as a control that fails after being pressed.
    let blocker = match standing {
        PartStanding::Withdrawn => Some(format!(
            "{} does not publish {}. The bytes this project retained still execute; there is \
             nothing newer to move onto.",
            diff.key.to, pin.part_id
        )),
        _ if !row.missing.is_empty() => Some(format!(
            "This build does not offer {}, so nothing from {} can be proved here.",
            plain_list(&row.missing),
            diff.key.to
        )),
        _ => None,
    };
    // The control is claimed first, from the right, and the line then takes
    // everything left of it and truncates. Laid out the other way round, a
    // part whose change needs a long phrase — a terminal list names every
    // terminal twice — takes the whole row and pushes its own Adopt control
    // off the pane. The inner left-to-right layout is what keeps the part
    // identifiers on one left edge: packed against the control instead, three
    // rows start at three different columns and nothing scans.
    ui.horizontal(|ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let clicked = ui
                .add_enabled(idle && blocker.is_none(), compact_button("Adopt"))
                .on_disabled_hover_text(blocker.clone().unwrap_or_else(|| {
                    "Another model-source operation is still running.".to_owned()
                }))
                .on_hover_text(format!(
                    "Re-retains {} from {} under the release key and moves this part's pin. The \
                     {} copy and every other pinned part stay as they are.",
                    pin.part_id, diff.key.to, diff.key.from
                ))
                .clicked();
            if clicked {
                app.queue_model_hub(ModelHubRequest::AdoptPart {
                    pack_id: row.pack_id.clone(),
                    from_version: diff.key.from.clone(),
                    to_version: diff.key.to.clone(),
                    part_id: pin.part_id.clone(),
                });
            }
            let line = format!("{} · {summary}", pin.part_id);
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                announced_widget(
                    ui,
                    egui::Label::new(RichText::new(&line).small().monospace().color(
                        if matches!(standing, PartStanding::Withdrawn) {
                            t.color.warn
                        } else {
                            t.color.text_dim
                        },
                    ))
                    .truncate(),
                    &line,
                );
            });
        });
    });
}

#[cfg(test)]
mod tests;
