//! Restoring a browser origin's installed packs, once per session.
//!
//! Reading IndexedDB is asynchronous and proving an archive is not free, so
//! neither happens on the path that opens the application. The session starts
//! with an empty pack store — a hub with no packs is a hub, and every route
//! that reaches one asks what it holds rather than assuming — and what this
//! origin kept arrives on a later frame.
//!
//! # Once, and on an event
//!
//! The read is started at initialization and the result is taken by the frame
//! that observes it. Nothing here runs per frame beyond one `pop_front` on an
//! empty queue: [`poll_browser_pack_restore`] is called from `update`, and on
//! every frame but the one that lands the restore it does nothing at all.
//!
//! The latch that stops a second restore is not here either. It is on the
//! store, which is `Arc`-shared and outlives this module's queue, because the
//! service holding the hub is replaced wholesale by ordinary events — a
//! project opening, a history entry being applied — and a flag on one of those
//! comes back cleared.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::services::model_hub::ModelHubService;
use crate::state::model_hub::durable::{PersistedHubState, start_browser_pack_restore};
use crate::workbench::app_state::Model;
use crate::workbench::models::workspace_state::{ModelsOperationalState, ModelsWorkbenchViewState};

thread_local! {
    /// The restore, once storage has answered. At most one ever arrives.
    static COMPLETION: std::cell::RefCell<Option<Result<(PersistedHubState, Vec<String>), String>>> =
        const { std::cell::RefCell::new(None) };
}

/// Starts reading what this origin kept.
pub(in crate::workbench) fn initialize_browser_pack_restore(ctx: &Context) {
    let repaint = ctx.clone();
    start_browser_pack_restore(move |result| {
        COMPLETION.with(|slot| *slot.borrow_mut() = Some(result));
        repaint.request_repaint();
    });
}

/// What taking the restore writes: the hub's pack store, the Models & PDKs
/// workspace's operation ladder, and the log in the model.
pub(in crate::workbench) struct PackRestoreParts<'a> {
    pub(in crate::workbench) models_view: &'a mut ModelsWorkbenchViewState,
    pub(in crate::workbench) model: &'a mut Model,
    pub(in crate::workbench) model_hub: &'a mut ModelHubService,
}

/// Takes the restore, if this is the frame it landed on.
pub(in crate::workbench) fn poll_browser_pack_restore(
    parts: &mut PackRestoreParts<'_>,
    _ctx: &Context,
) {
    let Some(completion) = COMPLETION.with(|slot| slot.borrow_mut().take()) else {
        return;
    };
    match completion {
        Ok((persisted, unrestored)) => {
            for reason in unrestored {
                parts
                    .model
                    .record(LogSeverity::Warning, LogSource::Studio, reason);
            }
            apply_restored_packs(parts, persisted);
        }
        Err(error) => {
            // Storage that will not open is not a failed operation — this
            // session simply keeps its packs in memory, which is what it
            // did before there was anywhere else to keep them. The note on
            // the packs page already states that, derived from the same
            // standing this failure recorded, so the console carries the
            // reason and the workspace is not put into a failure state
            // over something no reader asked for.
            parts.model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("Installed model packs cannot be kept between sessions: {error}"),
            );
        }
    }
}

fn apply_restored_packs(parts: &mut PackRestoreParts<'_>, persisted: PersistedHubState) {
    let Some((report, reload_error)) = parts.model_hub.restore_persisted_packs(persisted) else {
        return;
    };
    if let Some(error) = reload_error {
        parts
            .model
            .record(LogSeverity::Error, LogSource::Studio, error);
    }
    if report.is_empty() {
        return;
    }
    if report.restored > 0 {
        parts.model.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!(
                "Restored {} kept by this browser; each signature was verified.",
                crate::ui::accessibility::counted(report.restored, "model pack", "model packs")
            ),
        );
    }
    // A discarded archive is a real refusal and is depicted as one: the
    // bytes claimed to be a pack, did not prove, and were dropped. It goes
    // on the workspace's own ladder rather than into a sentence of its
    // own, so the packs page reports it the way it reports every other
    // refusal.
    if let Some(refusal) = report.refusal() {
        parts.models_view.attempted_operation = Some(
            crate::workbench::models::workspace_state::ModelsAttemptedOperation {
                label: "restoring model packs kept by this browser".to_owned(),
                // Nothing to re-issue: the bytes are gone, and the remedy
                // is to install the release again from the hub, which is a
                // row the reader picks rather than a button that guesses.
                reissuable: false,
                landing_pack: None,
            },
        );
        parts.models_view.operational_state = ModelsOperationalState::from_failure(&refusal);
        parts
            .model
            .record(LogSeverity::Warning, LogSource::Studio, refusal);
    }
}
