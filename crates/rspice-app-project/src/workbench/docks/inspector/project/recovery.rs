//! The inspector beside the History page: the selected row's record, its
//! note edited in place; and under the project while nothing is selected,
//! where the checkpoints are kept, the space they take and the automatic
//! space, chosen in place.

use egui::Ui;
use rspice_app_state::workbench::preferences::CHECKPOINT_BUDGETS_MIB;
use rspice_simulation_contract::run_set::format_bytes;
use rspice_ui_kit::panels::{Problem, section_header};

use crate::workbench::design_system::{
    property_row, property_row_choice, property_row_path, property_row_problem,
    property_row_wrapped,
};
use crate::workbench::lifecycle::project_checkpoint::{
    ProjectCheckpointQuarantine, ProjectCheckpointSummary, budget_bytes, budget_label,
    normalized_note,
};
use crate::workbench::project::revision_history::{ProjectAuditRow, set_aside_key};
use crate::workbench::surfaces::project::history::sentence;
use crate::workbench::surfaces::project::recovery::{STORE_UNREADABLE, checkpoint_created};

use super::properties::edited_text;
use super::{ProjectInspector, ProjectInspectorAction};

/// The rule the automatic space keeps checkpoints by, in one sentence.
const RETENTION_RULE: &str = "Checkpoints RSpice takes by itself are kept inside this space, \
     oldest dropped first; manual and pinned ones stay until you delete them.";

/// What a checkpoint's note cannot be changed while checkpoint work runs.
const BUSY: &str = "Checkpoint work is running; wait for it to finish.";

/// The selected row's record: a checkpoint, a damaged one, or a record of
/// the journal.
pub(super) fn history_record(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let key = inspector.history_selection.as_deref()?;
    let recovery = inspector.checkpoint_recovery;
    if let Some(checkpoint) = recovery
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.checkpoint_id().to_string() == key)
    {
        return checkpoint_record(ui, inspector, checkpoint);
    }
    if let Some(artifact) = recovery
        .quarantined
        .iter()
        .find(|artifact| set_aside_key(artifact.label()) == key)
    {
        set_aside_record(ui, inspector, artifact);
    } else if let Some(record) = inspector.history.rows.iter().find(|row| row.key == key) {
        journal_record(ui, record);
    }
    None
}

/// A checkpoint: when it was taken and why, at which revision, how large
/// and what it holds, how long it is kept and where, and its note, written
/// here.
fn checkpoint_record(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
    checkpoint: &ProjectCheckpointSummary,
) -> Option<ProjectInspectorAction> {
    section_header(ui, "Checkpoint", None);
    let now = crate::time_compat::checked_unix_time_ms().ok();
    property_row(
        ui,
        "When",
        &checkpoint_created(checkpoint.created_unix_ms(), now),
    );
    property_row(ui, "Reason", checkpoint.reason().label());
    property_row(ui, "Revision", &checkpoint.project_revision().to_string());
    property_row(ui, "Size", &format_bytes(checkpoint.snapshot_byte_len()));
    let digest = checkpoint.snapshot_digest().to_string();
    let short = digest.chars().take(12).collect::<String>();
    property_row(ui, "Digest", &short).on_hover_text(&digest);
    property_row(ui, "Pinned", if checkpoint.pinned() { "Yes" } else { "No" });
    property_row(ui, "Kept", checkpoint.retention().label());
    let busy = checkpoint_work_running(inspector).then_some(BUSY);
    let note = edited_text(
        ui,
        "Note",
        checkpoint.note().unwrap_or_default(),
        busy,
        |text| normalized_note(text).err().map(|error| sentence(&error)),
    );
    property_row_path(ui, "Where kept", location(inspector));
    note.map(|note| ProjectInspectorAction::CheckpointNote(checkpoint.checkpoint_id(), note))
}

/// A damaged checkpoint: what it was, what is wrong and what to do.
fn set_aside_record(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
    artifact: &ProjectCheckpointQuarantine,
) {
    section_header(ui, "Damaged checkpoint", None);
    property_row(ui, "Name", artifact.title().unwrap_or(artifact.label()));
    if let Some(made) = artifact.created_unix_ms().filter(|made| *made != 0) {
        let now = crate::time_compat::checked_unix_time_ms().ok();
        property_row(ui, "When", &checkpoint_created(made, now));
    }
    property_row_wrapped(ui, "Problem", artifact.problem());
    property_row_path(ui, "Where kept", location(inspector));
}

/// A record of the journal, with everything it keeps.
fn journal_record(ui: &mut Ui, record: &ProjectAuditRow) {
    section_header(ui, record.kind.label(), None);
    if let Some(made) = record.made_unix_ms {
        let now = crate::time_compat::checked_unix_time_ms().ok();
        property_row(ui, "When", &checkpoint_created(made, now));
    }
    if !record.change.is_empty() {
        property_row(ui, "Change", &record.change);
    }
    if let Some(view) = &record.view {
        property_row(ui, "View", &view.display_path());
    }
    property_row(ui, "Revision", &record.revision);
    if !record.actor.is_empty() {
        property_row(ui, "Author", &record.actor);
    }
    property_row(ui, "Status", &record.status);
    if !record.affected_evidence.is_empty() {
        property_row_wrapped(ui, "Evidence", &record.affected_evidence);
    }
}

/// Where the project's checkpoints are kept, the space they take, and the
/// space automatic ones are kept inside, chosen here as in Preferences; the
/// store's integrity only when it is not in order.
pub(super) fn history_checkpoints(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let recovery = inspector.checkpoint_recovery;
    section_header(ui, "Checkpoints", None);
    property_row_path(ui, "Location", location(inspector));
    // Unknown until the listing is in; "0 B" would say there is none.
    let used = if !recovery.initialized && recovery.error.is_none() {
        "Listing\u{2026}".to_owned()
    } else {
        format_bytes(
            recovery
                .checkpoints
                .iter()
                .map(ProjectCheckpointSummary::snapshot_byte_len)
                .sum(),
        )
    };
    property_row(ui, "Space used", &used);
    let shown = budget_label(budget_bytes(inspector.model));
    // The rule it keeps them by is the row's hover.
    let scoped = ui.scope(|ui| {
        property_row_choice(
            ui,
            "Automatic space",
            "project-inspector.checkpoint-budget",
            &shown,
            true,
            || {
                CHECKPOINT_BUDGETS_MIB
                    .iter()
                    .map(|&mib| (mib, budget_label(u64::from(mib) << 20)))
                    .collect()
            },
        )
    });
    let _ = scoped.response.on_hover_text(RETENTION_RULE);
    let budget = scoped.inner;
    if recovery.error.is_some() {
        property_row_problem(ui, "Integrity", STORE_UNREADABLE, Some(Problem::Failure));
    } else if !recovery.quarantined.is_empty() {
        property_row_problem(
            ui,
            "Integrity",
            &format!("{} damaged", recovery.quarantined.len()),
            Some(Problem::Warning),
        );
    }
    budget
        .filter(|mib| budget_label(u64::from(*mib) << 20) != shown)
        .map(ProjectInspectorAction::CheckpointBudget)
}

fn location<'a>(inspector: &ProjectInspector<'a>) -> &'a str {
    inspector
        .checkpoint_recovery
        .location
        .as_deref()
        .unwrap_or_default()
}

/// Checkpoint work is running, so a change waits for it.
fn checkpoint_work_running(inspector: &ProjectInspector<'_>) -> bool {
    let recovery = inspector.checkpoint_recovery;
    #[cfg(not(target_arch = "wasm32"))]
    {
        recovery.job.is_some()
    }
    #[cfg(target_arch = "wasm32")]
    {
        recovery.changing
    }
}
