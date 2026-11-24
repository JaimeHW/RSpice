//! The History page: the project's checkpoints and its revision journal in
//! one table, newest first.
//!
//! A checkpoint is a complete, verified copy of the whole project; the
//! journal keeps each schematic revision, simulation plan, model or PDK
//! binding, annotation and hierarchy check. One bar over the table holds
//! what the page does: the filter and the kind it lists, Create checkpoint,
//! and Compare…, Restore…, Pin, Note… and Delete… on the selected row. A
//! row's context menu holds the same with the same enablement; Enter
//! compares the row and Delete deletes it. What those ask is asked in a
//! small dialog: restoring a checkpoint puts the whole project back, after
//! keeping it as it is now as a checkpoint, and restoring a schematic
//! revision puts back that view's drawing only.
//!
//! The rows are built when what they are built from moves, never on an idle
//! frame, and the checkpoints are listed on a background thread.

use std::hash::{Hash, Hasher};
use std::sync::Arc;

use super::recovery::{self, CATALOG_UNREADABLE, CheckpointWork, RecoveryIntent, restore_refusal};
use super::*;
use crate::state::CellViewRef;
use crate::ui::icons::Icon;
use crate::ui::widgets::table::{
    CELL_PAD, Cell, Column, DataTable, RowKey, SORT_MARK_ROOM, SortKey, TABLE_EDGE_INSET,
};
use crate::ui::widgets::{CommandForm, FormRows, TableColumn};
use crate::workbench::app::dialogs::project::{CheckpointDeletion, ProjectCheckpointRecoveryState};
use crate::workbench::lifecycle::project_checkpoint::{
    CheckpointDamage, ProjectCheckpointQuarantine, ProjectCheckpointReason,
    ProjectCheckpointSummary, TechnologyRestore, normalized_note, technology_restore,
};
use crate::workbench::project::revision_history::{
    HistoryKind, HistoryList, HistoryListSource, HistoryRow, ProjectAuditRow, ask_revision_restore,
    finish_pending_restore, kept_audit_rows, restore_revision, restore_target, revision_comparison,
    set_aside_key,
};
use crate::workbench::slots::ProjectState;
use rspice_simulation_contract::run_set::format_bytes;
use rspice_ui_kit::widgets::{PageBar, PageBarFilter, page_bar_kind};

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread built the History table's rows.
    pub static HISTORY_ROW_BUILDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

const COMPARE: &str = "Compare\u{2026}";
const RESTORE: &str = "Restore\u{2026}";
const NOTE: &str = "Note\u{2026}";
const DELETE: &str = "Delete\u{2026}";
const SAVE_AS_NEW_PROJECT: &str = "Save as new project\u{2026}";
const NOTHING_SELECTED: &str = "Select a row first.";
const BUSY: &str = "Checkpoint work is running; wait for it to finish.";
const ONLY_CHECKPOINTS_PIN: &str = "Only a checkpoint can be pinned.";
const ONLY_CHECKPOINTS_NOTE: &str = "Only a checkpoint takes a note.";
const REVISIONS_STAY: &str =
    "A saved revision stays with the project; only a checkpoint can be deleted.";
const DAMAGED_UNREADABLE: &str = "A damaged checkpoint cannot be read; delete it.";
const NOT_COMPARABLE: &str =
    "Only a checkpoint or a schematic revision is compared with the project as it is.";
const NOT_RESTORABLE: &str = "Only a checkpoint or a schematic revision can be restored.";
const AUTOSAVE_UNPINNED: &str = "The next save removes an autosave, so it cannot be pinned.";
const UNWRITABLE: &str =
    "The checkpoint folder cannot be read, so no checkpoint can be written to it.";

/// What the reader asked of the page this frame, from its bar, a row's menu
/// or a key, by the row's key.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Asked {
    Create,
    Reload,
    Compare(String),
    Restore(String),
    Pin(String, bool),
    Note(String),
    Delete(String),
    SaveAsNewProject(String),
}

impl Asked {
    /// The row asked about, if the ask is about one.
    fn key(&self) -> Option<&str> {
        match self {
            Self::Create | Self::Reload => None,
            Self::Compare(key)
            | Self::Restore(key)
            | Self::Pin(key, _)
            | Self::Note(key)
            | Self::Delete(key)
            | Self::SaveAsNewProject(key) => Some(key),
        }
    }
}

pub fn history(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let ctx = ui.ctx().clone();
    recovery::refresh(&ctx, cx);
    kept_audit_rows(&mut cx.state.history, cx.model, ctx.cumulative_pass_nr());
    let list = listed(cx);
    settle_selection(cx, &list);
    let work = CheckpointWork::read(cx);
    let width = visible_workspace_width(ui);
    let height = visible_workspace_height(ui);
    let asked = ui
        .allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            ui.spacing_mut().item_spacing.y = 0.0;
            let asked = history_bar(ui, cx, &list, work);
            let table = history_table(ui, cx, &list, work, width);
            asked.or(table)
        })
        .inner;
    if let Some(asked) = asked {
        run(&ctx, cx, asked);
    }
}

/// The table's rows, built again only when the journal's rows, the catalog,
/// the filter or the kind moved since they were.
fn listed(cx: &mut Cx<'_, ProjectFeature>) -> HistoryList {
    let recovery = &*cx.state.project_checkpoint_recovery;
    let history = &mut *cx.state.history;
    let mut source = HistoryListSource {
        journal: history.rows_source,
        catalog: catalog_fingerprint(recovery),
        filter: history.filter.trim().to_lowercase(),
        kind: history.kind,
    };
    if let Some((listed_for, list)) = &history.listed
        && *listed_for == source
    {
        return list.clone();
    }
    let list = build_list(recovery, &history.rows, &source);
    // A kind the page no longer has a row of lists everything again, as the
    // kind select then says.
    if source.kind.is_some_and(|kind| !list.kinds.contains(&kind)) {
        history.kind = None;
        source.kind = None;
    }
    history.listed = Some((source, list.clone()));
    list
}

/// What of the catalog the rows say: which checkpoints there are, which
/// are pinned and what their notes say, and what was set aside.
fn catalog_fingerprint(recovery: &ProjectCheckpointRecoveryState) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    recovery.initialized.hash(&mut hasher);
    recovery.error.is_some().hash(&mut hasher);
    for checkpoint in &recovery.checkpoints {
        checkpoint.checkpoint_id().hash(&mut hasher);
        checkpoint.pinned().hash(&mut hasher);
        checkpoint.note().hash(&mut hasher);
    }
    for artifact in &recovery.quarantined {
        artifact.label().hash(&mut hasher);
    }
    hasher.finish()
}

/// Every row, newest first, and those the filter and the kind let through.
fn build_list(
    recovery: &ProjectCheckpointRecoveryState,
    journal: &[ProjectAuditRow],
    source: &HistoryListSource,
) -> HistoryList {
    #[cfg(any(test, feature = "test-support"))]
    HISTORY_ROW_BUILDS.with(|builds| builds.set(builds.get() + 1));
    let mut rows = recovery
        .checkpoints
        .iter()
        .map(checkpoint_row)
        .chain(recovery.quarantined.iter().map(set_aside_row))
        // A journal record that keeps no time is no change: it is what the
        // plan or a library binding was when the project began, which their
        // own pages say, and a history lists only what happened, and when.
        .chain(
            journal
                .iter()
                .filter(|row| row.made_unix_ms.is_some())
                .map(journal_row),
        )
        .collect::<Vec<_>>();
    // A checkpoint whose record keeps no time follows every row that does,
    // in the order it was listed in.
    rows.sort_by(|left, right| right.made_unix_ms.cmp(&left.made_unix_ms));
    let kinds = HistoryKind::ALL
        .into_iter()
        .filter(|kind| rows.iter().any(|row| row.kind == *kind))
        .collect::<Vec<_>>();
    let kind = source.kind.filter(|kind| kinds.contains(kind));
    let visible = rows
        .iter()
        .enumerate()
        .filter(|(_, row)| kind.is_none_or(|kind| row.kind == kind))
        .filter(|(_, row)| source.filter.is_empty() || matches_filter(row, &source.filter))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let keys = visible
        .iter()
        .map(|&index| RowKey::Text(rows[index].key.clone()))
        .collect::<Arc<[RowKey]>>();
    HistoryList {
        rows: rows.into(),
        visible: visible.into(),
        keys,
        kinds: kinds.into(),
        revision: LIST_REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
    }
}

/// The revision the next list of rows is built at: every build is one.
static LIST_REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// Whether any of a row's words hold `filter`, already lowercase.
fn matches_filter(row: &HistoryRow, filter: &str) -> bool {
    [
        row.when.as_str(),
        row.change.as_str(),
        row.subject.as_str(),
        row.revision.as_str(),
        row.kind.label(),
        row.note.as_str(),
        row.status.as_str(),
    ]
    .iter()
    .any(|words| words.to_lowercase().contains(filter))
}

fn checkpoint_row(checkpoint: &ProjectCheckpointSummary) -> HistoryRow {
    let made = checkpoint.created_unix_ms();
    HistoryRow {
        key: checkpoint.checkpoint_id().to_string(),
        kind: HistoryKind::Checkpoint,
        made_unix_ms: (made != 0).then_some(made),
        when: stamp(made),
        change: checkpoint.reason().label().to_owned(),
        subject: PROJECT.to_owned(),
        revision: checkpoint.project_revision().to_string(),
        // The autosave a recovery holds is kept by the recovery, not pinned.
        pinned: checkpoint.pinned() && checkpoint.reason() != ProjectCheckpointReason::Autosave,
        note: checkpoint.note().unwrap_or_default().to_owned(),
        status: String::new(),
        detail: String::new(),
    }
}

/// A damaged checkpoint: named as its row named it while its record still
/// reads, and otherwise by its file's or record's whole name.
fn set_aside_row(artifact: &ProjectCheckpointQuarantine) -> HistoryRow {
    let made = artifact.created_unix_ms().filter(|made| *made != 0);
    HistoryRow {
        key: set_aside_key(artifact.label()),
        kind: HistoryKind::SetAside,
        made_unix_ms: made,
        when: made.map(stamp).unwrap_or_default(),
        change: artifact.title().unwrap_or(artifact.label()).to_owned(),
        subject: PROJECT.to_owned(),
        revision: String::new(),
        pinned: false,
        note: String::new(),
        status: damage_words(artifact.damage()).to_owned(),
        detail: artifact.problem().to_owned(),
    }
}

fn journal_row(row: &ProjectAuditRow) -> HistoryRow {
    HistoryRow {
        key: row.key.clone(),
        kind: row.kind.history_kind(),
        made_unix_ms: row.made_unix_ms,
        when: row.time.clone(),
        change: row.change.clone(),
        subject: row
            .view
            .as_ref()
            .map_or_else(|| PROJECT.to_owned(), CellViewRef::display_path),
        revision: row.revision.clone(),
        pinned: false,
        note: String::new(),
        status: String::new(),
        detail: String::new(),
    }
}

/// What a record that is about the whole project names as its subject.
const PROJECT: &str = "Project";

/// A moment as the When column writes it; nothing for the unknown time zero
/// stands for.
fn stamp(unix_ms: u64) -> String {
    if unix_ms == 0 {
        String::new()
    } else {
        crate::time_compat::local_stamp(unix_ms)
    }
}

/// Why a checkpoint was set aside, in the few words its Status cell says.
const fn damage_words(damage: CheckpointDamage) -> &'static str {
    match damage {
        CheckpointDamage::Unreadable => "Record unreadable",
        CheckpointDamage::Record => "Record damaged",
        CheckpointDamage::MissingRecord => "Record missing",
        CheckpointDamage::MissingCopy => "Copy missing",
        CheckpointDamage::ChangedCopy => "Copy changed",
        CheckpointDamage::Misfiled => "Filed elsewhere",
    }
}

/// A selection whose row is gone is let go: a checkpoint's once the
/// checkpoints are listed, so one restored in a session is not dropped
/// while they are listed again.
fn settle_selection(cx: &mut Cx<'_, ProjectFeature>, list: &HistoryList) {
    let Some(key) = cx.state.project_history_selection.as_deref() else {
        return;
    };
    if list.row(key).is_none() && cx.state.project_checkpoint_recovery.initialized {
        cx.state.project_history_selection = None;
    }
}

/// The page's one bar: the filter and the kind on the left, then Create
/// checkpoint and what the selected row offers; how many checkpoints there
/// are and how much of the automatic space they take on the right.
fn history_bar(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    list: &HistoryList,
    work: CheckpointWork,
) -> Option<Asked> {
    let selected = cx
        .state
        .project_history_selection
        .as_deref()
        .and_then(|key| list.row(key))
        .cloned();
    let actions = RowActions::of(cx, work, selected.as_ref());
    let (report, problem) = bar_report(cx, work);
    let create_refusal = if cx.state.project_checkpoint_recovery.error.is_some() {
        Some(UNWRITABLE)
    } else if work.busy {
        Some(BUSY)
    } else {
        None
    };
    let unreadable = cx.state.project_checkpoint_recovery.error.is_some();
    PageBar::new().report(report, problem).show(ui, |ui| {
        let mut asked = None;
        PageBarFilter::new("workbench.project.history", "Filter history")
            .show(ui, &mut cx.state.history.filter);
        let choices = std::iter::once(ALL_KINDS.to_owned())
            .chain(list.kinds.iter().map(|kind| kind.plural().to_owned()))
            .collect::<Vec<_>>();
        let current = cx
            .state
            .history
            .kind
            .and_then(|kind| list.kinds.iter().position(|listed| *listed == kind))
            .map_or(0, |index| index + 1);
        if let Some(index) = page_bar_kind(
            ui,
            "workbench.project.history.kind",
            "Kind",
            &choices,
            current,
        ) {
            cx.state.history.kind = index
                .checked_sub(1)
                .and_then(|index| list.kinds.get(index).copied());
        }
        let create = if work.creating {
            "Creating checkpoint\u{2026}"
        } else {
            "Create checkpoint"
        };
        if bar_button(ui, create, create_refusal) {
            asked = Some(Asked::Create);
        }
        let key = selected.as_ref().map(|row| row.key.clone());
        let comparing = selected
            .as_ref()
            .is_some_and(|row| work.comparing.is_some_and(|id| id.to_string() == row.key));
        let restoring = selected
            .as_ref()
            .is_some_and(|row| work.restoring.is_some_and(|id| id.to_string() == row.key));
        for (label, refusal, ask) in [
            (
                if comparing {
                    "Comparing\u{2026}"
                } else {
                    COMPARE
                },
                actions.compare.as_deref(),
                Asked::Compare as fn(String) -> Asked,
            ),
            (
                if restoring {
                    "Restoring\u{2026}"
                } else {
                    RESTORE
                },
                actions.restore.as_deref(),
                Asked::Restore,
            ),
        ] {
            if bar_button(ui, label, refusal)
                && let Some(key) = &key
            {
                asked = Some(ask(key.clone()));
            }
        }
        if bar_button(ui, actions.pin_label(), actions.pin.as_deref())
            && let Some(key) = &key
        {
            asked = Some(Asked::Pin(key.clone(), !actions.pinned));
        }
        if bar_button(ui, NOTE, actions.note.as_deref())
            && let Some(key) = &key
        {
            asked = Some(Asked::Note(key.clone()));
        }
        if bar_button(ui, DELETE, actions.delete.as_deref())
            && let Some(key) = &key
        {
            asked = Some(Asked::Delete(key.clone()));
        }
        // Listing again is offered only while the checkpoints cannot be read.
        if unreadable
            && Button::new("Try again")
                .show(ui)
                .on_hover_text(CATALOG_UNREADABLE)
                .clicked()
        {
            asked = Some(Asked::Reload);
        }
        asked
    })
}

const ALL_KINDS: &str = "All";

/// A button of the bar, disabled with its reason under the pointer.
fn bar_button(ui: &mut Ui, label: &str, refusal: Option<&str>) -> bool {
    let response = Button::new(label).enabled(refusal.is_none()).show(ui);
    let response = match refusal {
        Some(refusal) => response.on_disabled_hover_text(refusal),
        None => response,
    };
    response.clicked()
}

/// What the bar says on its right: how many checkpoints there are, how many
/// were set aside damaged, and how much of the automatic space they take.
fn bar_report(cx: &Cx<'_, ProjectFeature>, work: CheckpointWork) -> (String, Option<Problem>) {
    let recovery = &cx.state.project_checkpoint_recovery;
    if recovery.error.is_some() {
        return ("Checkpoints unreadable".to_owned(), Some(Problem::Failure));
    }
    if work.loading {
        return ("Listing checkpoints\u{2026}".to_owned(), None);
    }
    let count = match recovery.checkpoints.len() {
        1 => "1 checkpoint".to_owned(),
        count => format!("{count} checkpoints"),
    };
    // The space every checkpoint takes, as the inspector's Space used says;
    // the automatic space is a choice the inspector holds.
    let space = format_bytes(
        recovery
            .checkpoints
            .iter()
            .map(ProjectCheckpointSummary::snapshot_byte_len)
            .sum(),
    );
    match recovery.quarantined.len() {
        0 => (format!("{count} \u{b7} {space}"), None),
        damaged => (
            format!("{count} \u{b7} {damaged} damaged \u{b7} {space}"),
            Some(Problem::Warning),
        ),
    }
}

/// Why each of the bar's row actions cannot be taken on a row now, if it
/// cannot; the row's menu offers the same with the same reasons.
struct RowActions {
    compare: Option<String>,
    restore: Option<String>,
    pin: Option<String>,
    /// The row is a pinned checkpoint, so Pin unpins it.
    pinned: bool,
    note: Option<String>,
    delete: Option<String>,
    /// Only a checkpoint is saved as a new project.
    save_as: Option<Option<String>>,
}

impl RowActions {
    fn of(cx: &Cx<'_, ProjectFeature>, work: CheckpointWork, row: Option<&HistoryRow>) -> Self {
        let refused = |reason: &str| Some(reason.to_owned());
        let Some(row) = row else {
            return Self {
                compare: refused(NOTHING_SELECTED),
                restore: refused(NOTHING_SELECTED),
                pin: refused(NOTHING_SELECTED),
                pinned: false,
                note: refused(NOTHING_SELECTED),
                delete: refused(NOTHING_SELECTED),
                save_as: None,
            };
        };
        let busy = work.busy.then(|| BUSY.to_owned());
        match row.kind {
            HistoryKind::Checkpoint => {
                let autosave = cx
                    .state
                    .project_checkpoint_recovery
                    .checkpoints
                    .iter()
                    .find(|checkpoint| checkpoint.checkpoint_id().to_string() == row.key)
                    .is_some_and(|checkpoint| {
                        checkpoint.reason() == ProjectCheckpointReason::Autosave
                    });
                Self {
                    compare: busy.clone(),
                    restore: busy.clone().or_else(|| restore_refusal(cx).map(sentence)),
                    pin: if autosave {
                        refused(AUTOSAVE_UNPINNED)
                    } else {
                        busy.clone()
                    },
                    pinned: row.pinned,
                    note: busy.clone(),
                    delete: busy.clone(),
                    save_as: Some(busy),
                }
            }
            HistoryKind::SetAside => Self {
                compare: refused(DAMAGED_UNREADABLE),
                restore: refused(DAMAGED_UNREADABLE),
                pin: refused(ONLY_CHECKPOINTS_PIN),
                pinned: false,
                note: refused(ONLY_CHECKPOINTS_NOTE),
                delete: busy,
                save_as: None,
            },
            kind => {
                let schematic = kind == HistoryKind::Schematic;
                let restore = if schematic {
                    cx.state
                        .history
                        .rows
                        .iter()
                        .find(|record| record.key == row.key)
                        .map_or_else(
                            || refused(NOT_RESTORABLE),
                            |record| restore_target(record, cx.model).err(),
                        )
                } else {
                    refused(NOT_RESTORABLE)
                };
                Self {
                    compare: (!schematic).then(|| NOT_COMPARABLE.to_owned()),
                    restore,
                    pin: refused(ONLY_CHECKPOINTS_PIN),
                    pinned: false,
                    note: refused(ONLY_CHECKPOINTS_NOTE),
                    delete: refused(REVISIONS_STAY),
                    save_as: None,
                }
            }
        }
    }

    const fn pin_label(&self) -> &'static str {
        if self.pinned { "Unpin" } else { "Pin" }
    }

    /// Why `asked` cannot be taken on its row now, if it cannot.
    fn refusal(&self, asked: &Asked) -> Option<&str> {
        match asked {
            Asked::Create | Asked::Reload => None,
            Asked::Compare(_) => self.compare.as_deref(),
            Asked::Restore(_) => self.restore.as_deref(),
            Asked::Pin(..) => self.pin.as_deref(),
            Asked::Note(_) => self.note.as_deref(),
            Asked::Delete(_) => self.delete.as_deref(),
            Asked::SaveAsNewProject(_) => match &self.save_as {
                Some(refusal) => refusal.as_deref(),
                None => Some(ONLY_CHECKPOINTS_SAVE),
            },
        }
    }
}

const ONLY_CHECKPOINTS_SAVE: &str = "Only a checkpoint is saved as a new project.";

/// A refusal written as a clause, as a sentence.
pub(crate) fn sentence(clause: &str) -> String {
    let mut chars = clause.chars();
    chars.next().map_or_else(String::new, |first| {
        format!("{}{}.", first.to_uppercase(), chars.as_str())
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HistoryColumn {
    When,
    Change,
    Subject,
    Revision,
    Kind,
    Pinned,
    Note,
    Status,
}

/// The table's columns. Change, Subject and Note share what the fixed
/// columns leave, each of which is as wide as its widest words; Pinned, Note
/// and Status are left out while no row has one.
fn history_columns(ui: &Ui, list: &HistoryList) -> Vec<(HistoryColumn, Column)> {
    let has = |value: fn(&HistoryRow) -> bool| list.rows.iter().any(value);
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let sans = theme::sans(tokens::FS_0, FontWeight::Regular);
    let width = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), Color32::PLACEHOLDER)
            .size()
            .x
    };
    let fitted = |head: &str, widest: f32, lead: f32| {
        let head = width(head, &theme::sans(tokens::FS_0, FontWeight::Medium)) + SORT_MARK_ROOM;
        (widest.max(head) + lead + CELL_PAD + 2.0).ceil()
    };
    let revision = list
        .rows
        .iter()
        .map(|row| row.revision.chars().count())
        .max()
        .unwrap_or_default();
    let kind = list
        .kinds
        .iter()
        .map(|kind| width(kind.label(), &sans))
        .fold(0.0, f32::max);
    let status = list
        .rows
        .iter()
        .filter(|row| !row.status.is_empty())
        .map(|row| width(&row.status, &sans))
        .fold(0.0, f32::max)
        + crate::ui::widgets::PROBLEM_MARK_ADVANCE;
    [
        (
            HistoryColumn::When,
            Column::words(
                "When",
                fitted("When", width("0000-00-00 00:00", &mono), TABLE_EDGE_INSET),
            ),
        ),
        (HistoryColumn::Change, Column::name("Change", 120.0)),
        (HistoryColumn::Subject, Column::name("Subject", 120.0)),
        (
            HistoryColumn::Revision,
            Column::words(
                "Revision",
                fitted("Revision", width(&"0".repeat(revision), &mono), CELL_PAD),
            ),
        ),
        (
            HistoryColumn::Kind,
            Column::words("Kind", fitted("Kind", kind, CELL_PAD)),
        ),
        (
            HistoryColumn::Pinned,
            Column::words("Pinned", fitted("Pinned", 0.0, CELL_PAD)),
        ),
        (HistoryColumn::Note, Column::name("Note", 120.0)),
        (
            HistoryColumn::Status,
            Column::words("Status", fitted("Status", status, CELL_PAD)),
        ),
    ]
    .into_iter()
    .filter(|(column, _)| match column {
        HistoryColumn::Pinned => has(|row| row.pinned),
        HistoryColumn::Note => has(|row| !row.note.is_empty()),
        HistoryColumn::Status => has(|row| !row.status.is_empty()),
        _ => true,
    })
    .collect()
}

/// One row's cell under `column`.
fn history_cell(row: &HistoryRow, column: HistoryColumn) -> Cell {
    let absent = || Cell::absent().sort_by(SortKey::Missing);
    match column {
        HistoryColumn::When => match row.made_unix_ms {
            Some(made) if !row.when.is_empty() => {
                Cell::number(row.when.as_str()).sort_by(SortKey::Number(made as f64))
            }
            _ => absent(),
        },
        HistoryColumn::Change if row.change.is_empty() => absent(),
        HistoryColumn::Change => Cell::text(row.change.as_str()),
        HistoryColumn::Subject => Cell::text(row.subject.as_str()),
        HistoryColumn::Revision if row.revision.is_empty() => absent(),
        HistoryColumn::Revision => Cell::number(row.revision.as_str()),
        HistoryColumn::Kind => Cell::text(row.kind.label()),
        HistoryColumn::Pinned if row.pinned => Cell::empty()
            .icon(Icon::Check)
            .sort_by(SortKey::Number(1.0)),
        HistoryColumn::Note if !row.note.is_empty() => Cell::text(row.note.as_str()),
        HistoryColumn::Status if !row.status.is_empty() => {
            Cell::problem(row.status.as_str(), Problem::Warning).hint(row.detail.as_str())
        }
        HistoryColumn::Pinned | HistoryColumn::Note | HistoryColumn::Status => {
            Cell::empty().sort_by(SortKey::Missing)
        }
    }
}

/// The table, under the bar to the floor of the page, its selection kept
/// with the inspector's, and what the reader asked of a row: Enter or a
/// double-click compares it, Delete deletes it, the context menu offers
/// what the bar does.
fn history_table(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    list: &HistoryList,
    work: CheckpointWork,
    width: f32,
) -> Option<Asked> {
    if list.rows.is_empty() {
        // Nothing is said of the checkpoints until they are listed: the
        // bar says they are being. A store that cannot be read is not empty.
        let recovery = &cx.state.project_checkpoint_recovery;
        if recovery.error.is_some() {
            workspace_empty_table_row(ui, width, CATALOG_UNREADABLE);
        } else if recovery.initialized {
            workspace_empty_table_row(
                ui,
                width,
                "No checkpoints or revisions yet. Create checkpoint keeps the project as it is now.",
            );
        }
        return None;
    }
    if list.visible.is_empty() {
        workspace_empty_table_row(ui, width, "No row matches the filter.");
        return None;
    }
    let keys = &list.keys;
    let selected = cx
        .state
        .project_history_selection
        .as_deref()
        .and_then(|key| {
            list.visible
                .iter()
                .position(|&index| list.rows[index].key == key)
        });
    let table = &mut cx.state.history.table;
    match selected {
        Some(row) if table.selected_key() != Some(&keys[row]) => {
            table.select(row, Some(keys[row].clone()));
        }
        None if table.selected_row().is_some() => {
            table.selection = None;
            table.others.clear();
            table.anchor = None;
        }
        _ => {}
    }
    let columns = history_columns(ui, list);
    let heads = columns
        .iter()
        .map(|(_, column)| column.clone())
        .collect::<Vec<_>>();
    let change = columns
        .iter()
        .position(|(column, _)| *column == HistoryColumn::Change)
        .unwrap_or_default();
    let response = DataTable::new(
        "workbench.project.history.table",
        &heads,
        list.visible.len(),
    )
    .state(table)
    .keys(keys)
    .freeze(1)
    .pin_when_narrow(change)
    .revision(list.revision)
    .max_height(ui.available_height().max(1.0))
    .show(ui, |row| {
        let row = &list.rows[list.visible[row]];
        columns
            .iter()
            .map(|(column, _)| history_cell(row, *column))
            .collect()
    });
    cx.state.history.focused = response.focused;
    let key_at = |row: usize| {
        list.visible
            .get(row)
            .map(|&index| list.rows[index].key.clone())
    };
    if response.selection_changed {
        cx.state.project_history_selection = cx.state.history.table.selected_row().and_then(key_at);
    }
    if let Some(key) = response.activate.and_then(key_at) {
        return Some(Asked::Compare(key));
    }
    if response.delete
        && let Some(key) = cx.state.history.table.selected_row().and_then(key_at)
    {
        return Some(Asked::Delete(key));
    }
    history_menu(
        ui,
        cx,
        list,
        work,
        response
            .context_menu
            .and_then(|request| key_at(request.cell.row).map(|key| (key, request.position))),
    )
}

/// A row's context menu: opened where the reader asked for it, on the row
/// they asked it of, and kept open until an item is taken or they click
/// elsewhere. It offers what the bar does for the row, with the same
/// reasons for what cannot be taken now, and a checkpoint's Save as new
/// project….
fn history_menu(
    ui: &Ui,
    cx: &Cx<'_, ProjectFeature>,
    list: &HistoryList,
    work: CheckpointWork,
    request: Option<(String, egui::Pos2)>,
) -> Option<Asked> {
    let id = ui.id().with("workbench.project.history.row menu");
    let at = id.with("at");
    let opened = request.is_some();
    if let Some(request) = request {
        ui.ctx().data_mut(|data| data.insert_temp(at, request));
    }
    let (key, position) = ui
        .ctx()
        .data(|data| data.get_temp::<(String, egui::Pos2)>(at))?;
    if !opened && !egui::Popup::is_id_open(ui.ctx(), id) {
        return None;
    }
    let row = list.row(&key)?.clone();
    let actions = RowActions::of(cx, work, Some(&row));
    let mut taken = None;
    egui::Popup::new(
        id,
        ui.ctx().clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .layout(egui::Layout::top_down_justified(egui::Align::Min))
    .style(egui::containers::menu::menu_style)
    .open_memory(opened.then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        ui.set_min_width(180.0);
        let items = [
            (
                COMPARE,
                Some("Enter"),
                &actions.compare,
                Asked::Compare(key.clone()),
            ),
            (RESTORE, None, &actions.restore, Asked::Restore(key.clone())),
            (
                actions.pin_label(),
                None,
                &actions.pin,
                Asked::Pin(key.clone(), !actions.pinned),
            ),
            (NOTE, None, &actions.note, Asked::Note(key.clone())),
            (
                DELETE,
                Some("Delete"),
                &actions.delete,
                Asked::Delete(key.clone()),
            ),
        ];
        for (label, shortcut, refusal, asked) in items {
            if menu_item(ui, label, shortcut, refusal.as_deref()) {
                taken = Some(asked);
            }
        }
        if let Some(refusal) = &actions.save_as {
            ui.separator();
            if menu_item(ui, SAVE_AS_NEW_PROJECT, None, refusal.as_deref()) {
                taken = Some(Asked::SaveAsNewProject(key.clone()));
            }
        }
    });
    taken
}

/// One item of a menu, disabled with its reason under the pointer, its key
/// at the row's right end; taking it closes the menu.
fn menu_item(ui: &mut Ui, label: &str, key: Option<&str>, refusal: Option<&str>) -> bool {
    let mut button = egui::Button::new(label);
    if let Some(key) = key {
        button = button.shortcut_text(key);
    }
    let response = ui.add_enabled(refusal.is_none(), button);
    if let Some(key) = key {
        // The key is told as the item's shortcut, not as part of its name.
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label(label);
            node.set_keyboard_shortcut(key);
        });
    }
    let taken = response
        .on_disabled_hover_text(refusal.unwrap_or_default())
        .clicked();
    if taken {
        ui.close();
    }
    taken
}

/// What a row is: a checkpoint, a damaged one, or a record of the journal.
enum Subject {
    Checkpoint(Box<ProjectCheckpointSummary>),
    SetAside(ProjectCheckpointQuarantine),
    Journal(ProjectAuditRow),
}

fn subject(cx: &Cx<'_, ProjectFeature>, key: &str) -> Option<Subject> {
    let recovery = &cx.state.project_checkpoint_recovery;
    if let Some(checkpoint) = recovery
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.checkpoint_id().to_string() == key)
    {
        return Some(Subject::Checkpoint(Box::new(checkpoint.clone())));
    }
    if let Some(artifact) = recovery
        .quarantined
        .iter()
        .find(|artifact| set_aside_key(artifact.label()) == key)
    {
        return Some(Subject::SetAside(artifact.clone()));
    }
    cx.state
        .history
        .rows
        .iter()
        .find(|row| row.key == key)
        .cloned()
        .map(Subject::Journal)
}

/// Carry out what the reader asked: the row asked about becomes the
/// selected one, and an ask it refuses says why on the status line.
fn run(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, asked: Asked) {
    match asked {
        Asked::Create => return RecoveryIntent::Create.execute(ctx, cx),
        Asked::Reload => return RecoveryIntent::Reload.execute(ctx, cx),
        _ => {}
    }
    let Some(key) = asked.key().map(str::to_owned) else {
        return;
    };
    cx.state.project_history_selection = Some(key.clone());
    let row = cx
        .state
        .history
        .listed
        .as_ref()
        .and_then(|(_, list)| list.row(&key))
        .cloned();
    let actions = RowActions::of(cx, CheckpointWork::read(cx), row.as_ref());
    if let Some(refusal) = actions.refusal(&asked) {
        cx.wb.prompt(refusal.to_owned());
        return;
    }
    let Some(subject) = subject(cx, &key) else {
        return;
    };
    let intent = match (asked, subject) {
        (Asked::Compare(_), Subject::Checkpoint(checkpoint)) => {
            RecoveryIntent::Compare(*checkpoint)
        }
        (Asked::Compare(_), Subject::Journal(record)) => return compare_revision(cx, &record),
        (Asked::Restore(_), Subject::Checkpoint(checkpoint)) => {
            RecoveryIntent::AskRestore(*checkpoint)
        }
        (Asked::Restore(_), Subject::Journal(record)) => {
            return ask_revision_restore(&mut cx.state.history, cx.model, record.key);
        }
        (Asked::Pin(_, pinned), Subject::Checkpoint(checkpoint)) => {
            RecoveryIntent::Pin(*checkpoint, pinned)
        }
        (Asked::Note(_), Subject::Checkpoint(checkpoint)) => RecoveryIntent::WriteNote(*checkpoint),
        (Asked::Delete(_), Subject::Checkpoint(checkpoint)) => {
            RecoveryIntent::AskDelete(CheckpointDeletion::Checkpoint(checkpoint.checkpoint_id()))
        }
        (Asked::Delete(_), Subject::SetAside(artifact)) => {
            RecoveryIntent::AskDelete(CheckpointDeletion::SetAside(artifact.label().to_owned()))
        }
        (Asked::SaveAsNewProject(_), Subject::Checkpoint(checkpoint)) => {
            RecoveryIntent::SaveAsNewProject(*checkpoint)
        }
        _ => return,
    };
    intent.execute(ctx, cx);
}

/// Compare a schematic revision with its view's drawing as it stands: what
/// changed is listed in a dialog, and a drawing that did not change is said
/// to match on the status line.
fn compare_revision(cx: &mut Cx<'_, ProjectFeature>, record: &ProjectAuditRow) {
    match revision_comparison(record, cx.model) {
        Ok(found) if found.delta.is_empty() => {
            let path = record
                .view
                .as_ref()
                .map(CellViewRef::display_path)
                .unwrap_or_default();
            cx.wb.prompt(format!("{path} matches this revision."));
        }
        Ok(found) => cx.state.history.revision_comparison = Some(found),
        Err(error) => cx.model.report_failure("Could not compare", error),
    }
}

/// Keep `note` on the checkpoint `checkpoint`, as the inspector's Note field
/// was left with it.
pub(crate) fn keep_note(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
    checkpoint: uuid::Uuid,
    note: &str,
) {
    let Some(Subject::Checkpoint(checkpoint)) = subject(cx, &checkpoint.to_string()) else {
        return;
    };
    cx.state.project_checkpoint_recovery.note_draft =
        Some((checkpoint.checkpoint_id(), note.to_owned()));
    RecoveryIntent::KeepNote(*checkpoint).execute(ctx, cx);
}

/// The inspector's title beside the History page while a row is selected:
/// what the row is.
pub fn record_title(project: &ProjectState) -> &'static str {
    let Some(key) = project.project_history_selection.as_deref() else {
        return "Checkpoint";
    };
    if let Some(record) = project.history.rows.iter().find(|row| row.key == key) {
        return record.kind.label();
    }
    if project
        .project_checkpoint_recovery
        .quarantined
        .iter()
        .any(|artifact| set_aside_key(artifact.label()) == key)
    {
        "Damaged checkpoint"
    } else {
        "Checkpoint"
    }
}

/// Whether the page asks the reader something: whether to restore, delete
/// or note a checkpoint, what a comparison found, the new project a
/// checkpoint was saved as, or whether to restore a schematic revision.
pub fn asking(project: &ProjectState) -> bool {
    let recovery = &project.project_checkpoint_recovery;
    recovery.restore_review.is_some()
        || recovery.delete_review.is_some()
        || recovery.note_draft.is_some()
        || recovery.comparison.is_some()
        || recovery.saved_copy.is_some()
        || project.history.revision_restore.is_some()
        || project.history.revision_comparison.is_some()
}

/// The page's questions, one at a time, each a small dialog: and a
/// schematic revision's restore that opened its view, carried out once the
/// view is in front.
pub fn dialogs(cx: &mut Cx<'_, ProjectFeature>, ctx: &Context) {
    finish_pending_restore(cx);
    let recovery = &cx.state.project_checkpoint_recovery;
    if let Some(id) = recovery.restore_review {
        restore_checkpoint_dialog(ctx, cx, id);
    } else if let Some(deletion) = recovery.delete_review.clone() {
        delete_dialog(ctx, cx, &deletion);
    } else if let Some(id) = recovery.note_draft.as_ref().map(|(id, _)| *id) {
        note_dialog(ctx, cx, id);
    } else if recovery.comparison.is_some() {
        comparison_dialog(ctx, cx);
    } else if let Some((_, path)) = recovery.saved_copy.clone() {
        saved_copy_dialog(ctx, cx, &path);
    } else if cx.state.history.revision_restore.is_some() {
        revision_restore_dialog(ctx, cx);
    } else if cx.state.history.revision_comparison.is_some() {
        revision_comparison_dialog(ctx, cx);
    }
}

fn find_checkpoint(
    cx: &Cx<'_, ProjectFeature>,
    id: uuid::Uuid,
) -> Option<ProjectCheckpointSummary> {
    cx.state
        .project_checkpoint_recovery
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.checkpoint_id() == id)
        .cloned()
}

/// When a checkpoint was taken, as a dialog's row states it.
fn taken(checkpoint: &ProjectCheckpointSummary) -> String {
    crate::time_compat::local_stamp(checkpoint.created_unix_ms())
}

/// Whether to restore a checkpoint: the whole project goes back to it, after
/// the project as it is now is kept as a checkpoint. A checkpoint whose
/// technology this installation cannot give back offers to be saved as a
/// new project instead, or the Dependencies page where it is installed.
fn restore_checkpoint_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, id: uuid::Uuid) {
    let Some(checkpoint) = find_checkpoint(cx, id) else {
        cx.state.project_checkpoint_recovery.restore_review = None;
        return;
    };
    let work = CheckpointWork::read(cx);
    let technology = technology_restore(cx.model, checkpoint.technology());
    let intent = if let Some(refusal) = technology.refusal(checkpoint.title()) {
        let choice = CommandForm::new(RESTORE_CHECKPOINT, SAVE_AS_NEW_PROJECT)
            .describe(&refusal)
            .wide()
            .secondary("Open Dependencies")
            .primary_enabled(!work.busy)
            .status(work.busy.then_some(BUSY).map_or(Ok(None), Err))
            .show(ctx, |rows| {
                checkpoint_facts(rows, &checkpoint);
                rows.derived(&refusal);
                None
            });
        match choice {
            DialogChoice::Primary => {
                cx.state.project_checkpoint_recovery.restore_review = None;
                RecoveryIntent::SaveAsNewProject(checkpoint)
            }
            DialogChoice::Secondary => RecoveryIntent::OpenDependencies,
            DialogChoice::Ghost | DialogChoice::Cancelled => RecoveryIntent::CancelRestore,
            DialogChoice::None => return,
        }
    } else {
        let technology = match technology {
            TechnologyRestore::Restores(name) => {
                format!(" Its technology, {name}, comes back with it.")
            }
            TechnologyRestore::Detaches(name) => {
                format!(" The project had no technology then, so {name} is detached.")
            }
            _ => String::new(),
        };
        let what = format!(
            "The whole project goes back to this checkpoint, as unsaved changes. The project \
             as it is now is kept first, as the checkpoint \u{201c}{}\u{201d}.{technology}",
            ProjectCheckpointReason::Restore.label()
        );
        let refusal = work
            .busy
            .then(|| BUSY.to_owned())
            .or_else(|| restore_refusal(cx).map(sentence));
        let choice = CommandForm::new(RESTORE_CHECKPOINT, "Restore")
            .describe(&what)
            .wide()
            .primary_enabled(refusal.is_none())
            .status(refusal.as_deref().map_or(Ok(None), Err))
            .show(ctx, |rows| {
                checkpoint_facts(rows, &checkpoint);
                rows.derived(&what);
                None
            });
        match choice {
            DialogChoice::Primary => RecoveryIntent::Restore(checkpoint),
            DialogChoice::Ghost | DialogChoice::Cancelled => RecoveryIntent::CancelRestore,
            DialogChoice::Secondary | DialogChoice::None => return,
        }
    };
    intent.execute(ctx, cx);
}

const RESTORE_CHECKPOINT: &str = "Restore checkpoint";

/// What a question about a checkpoint names it by.
fn checkpoint_facts(rows: &mut FormRows<'_>, checkpoint: &ProjectCheckpointSummary) {
    rows.value("Checkpoint", checkpoint.title());
    rows.value("Taken", &taken(checkpoint));
    rows.value("Revision", &checkpoint.project_revision().to_string());
}

/// Whether to delete a checkpoint, or a damaged one set aside: it cannot be
/// brought back.
fn delete_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, deletion: &CheckpointDeletion) {
    let work = CheckpointWork::read(cx);
    let refusal = work.busy.then_some(BUSY);
    let status = refusal.map_or(Ok(Some(CANNOT_BE_BROUGHT_BACK)), Err);
    let intent = match deletion {
        CheckpointDeletion::Checkpoint(id) => {
            let Some(checkpoint) = find_checkpoint(cx, *id) else {
                cx.state.project_checkpoint_recovery.delete_review = None;
                return;
            };
            let describe = format!(
                "Delete the checkpoint \u{201c}{}\u{201d} from {}? It cannot be brought back.",
                checkpoint.title(),
                taken(&checkpoint)
            );
            let choice = CommandForm::new("Delete checkpoint", "Delete")
                .describe(&describe)
                .destructive()
                .primary_enabled(refusal.is_none())
                .status(status)
                .show(ctx, |rows| {
                    rows.value("Checkpoint", checkpoint.title());
                    rows.value("Taken", &taken(&checkpoint));
                    if checkpoint.pinned() {
                        rows.value("Kept", checkpoint.retention().label());
                    }
                    None
                });
            match choice {
                DialogChoice::Primary => RecoveryIntent::Delete(checkpoint),
                DialogChoice::Ghost | DialogChoice::Cancelled => RecoveryIntent::CancelDelete,
                DialogChoice::Secondary | DialogChoice::None => return,
            }
        }
        CheckpointDeletion::SetAside(label) => {
            let Some(artifact) = cx
                .state
                .project_checkpoint_recovery
                .quarantined
                .iter()
                .find(|artifact| artifact.label() == label)
                .cloned()
            else {
                cx.state.project_checkpoint_recovery.delete_review = None;
                return;
            };
            let name = artifact.title().unwrap_or(artifact.label()).to_owned();
            let describe =
                format!("Delete the damaged checkpoint {name}? It cannot be brought back.");
            let choice = CommandForm::new("Delete damaged checkpoint", "Delete")
                .describe(&describe)
                .destructive()
                .primary_enabled(refusal.is_none())
                .status(status)
                .show(ctx, |rows| {
                    rows.value("Checkpoint", &name);
                    if let Some(made) = artifact.created_unix_ms().filter(|made| *made != 0) {
                        rows.value("Taken", &crate::time_compat::local_stamp(made));
                    }
                    rows.derived(artifact.problem());
                    None
                });
            match choice {
                DialogChoice::Primary => RecoveryIntent::DeleteSetAside(artifact),
                DialogChoice::Ghost | DialogChoice::Cancelled => RecoveryIntent::CancelDelete,
                DialogChoice::Secondary | DialogChoice::None => return,
            }
        }
    };
    intent.execute(ctx, cx);
}

const CANNOT_BE_BROUGHT_BACK: &str = "It cannot be brought back.";

/// The note on a checkpoint, written in one field and kept, or cleared by
/// keeping it empty.
fn note_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, id: uuid::Uuid) {
    let Some(checkpoint) = find_checkpoint(cx, id) else {
        cx.state.project_checkpoint_recovery.note_draft = None;
        return;
    };
    let work = CheckpointWork::read(cx);
    let Some((_, draft)) = &mut cx.state.project_checkpoint_recovery.note_draft else {
        return;
    };
    let mut typed = draft.clone();
    let refusal = normalized_note(&typed)
        .err()
        .map(|error| sentence(&error))
        .or_else(|| work.busy.then(|| BUSY.to_owned()));
    let field = egui::Id::new("workbench.project.history.note");
    let describe = format!(
        "The note on the checkpoint {} from {}.",
        checkpoint.reason().label(),
        taken(&checkpoint)
    );
    let choice = CommandForm::new("Note", "Keep note")
        .describe(&describe)
        .first_field(field)
        .primary_enabled(refusal.is_none())
        .status(refusal.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.value("Checkpoint", checkpoint.reason().label());
            rows.value("Taken", &taken(&checkpoint));
            let _ = rows.text_sans("Note", field, &mut typed, "What this checkpoint is for");
            Some(field)
        });
    if let Some((_, draft)) = &mut cx.state.project_checkpoint_recovery.note_draft {
        *draft = typed;
    }
    let intent = match choice {
        DialogChoice::Primary => RecoveryIntent::KeepNote(checkpoint),
        DialogChoice::Ghost | DialogChoice::Cancelled => RecoveryIntent::CancelNote,
        DialogChoice::Secondary | DialogChoice::None => return,
    };
    intent.execute(ctx, cx);
}

/// What a comparison with a checkpoint found: each document in which the
/// project as it is differs from it, and how; and the offer to restore it.
fn comparison_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let Some((id, found)) = cx.state.project_checkpoint_recovery.comparison.clone() else {
        return;
    };
    // A comparison handed over before the checkpoints are listed names the
    // checkpoint once they are.
    let checkpoint = find_checkpoint(cx, id);
    let documents = if found.len() == 1 {
        "1 document".to_owned()
    } else {
        format!("{} documents", found.len())
    };
    let describe = match &checkpoint {
        Some(checkpoint) => format!(
            "The project as it is differs from the checkpoint \u{201c}{}\u{201d} in {documents}.",
            checkpoint.title()
        ),
        None => format!("The project as it is differs from the checkpoint in {documents}."),
    };
    let rows = found
        .iter()
        .map(|difference| {
            vec![
                difference.kind.label().to_owned(),
                difference.document.label(),
            ]
        })
        .collect::<Vec<_>>();
    let choice = CommandForm::new("Compare with checkpoint", RESTORE)
        .describe(&describe)
        .wide()
        .cancel_label("Close")
        .primary_on_enter(false)
        .primary_enabled(checkpoint.is_some())
        .status(Ok(Some(&describe)))
        .show(ctx, |form| {
            if let Some(checkpoint) = &checkpoint {
                form.value("Checkpoint", checkpoint.title());
                form.value("Taken", &taken(checkpoint));
            }
            form.table(
                &[
                    TableColumn {
                        header: "Change",
                        mono: false,
                    },
                    TableColumn {
                        header: "Document",
                        mono: false,
                    },
                ],
                &rows,
            );
            None
        });
    match (choice, checkpoint) {
        (DialogChoice::Primary, Some(checkpoint)) => {
            cx.state.project_checkpoint_recovery.comparison = None;
            RecoveryIntent::AskRestore(checkpoint).execute(ctx, cx);
        }
        (DialogChoice::Ghost | DialogChoice::Cancelled, _) => {
            RecoveryIntent::CloseComparison.execute(ctx, cx);
        }
        _ => {}
    }
}

/// A checkpoint just saved as a new project: where it went, and the offer to
/// open it.
fn saved_copy_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>, path: &std::path::Path) {
    let shown = path.display().to_string();
    let describe = format!("The checkpoint was saved as {shown}.");
    let choice = CommandForm::new("Saved as new project", "Open it")
        .describe(&describe)
        .wide()
        .cancel_label("Not now")
        .show(ctx, |rows| {
            rows.value_mono("File", &shown);
            None
        });
    let intent = match choice {
        DialogChoice::Primary => RecoveryIntent::OpenSavedCopy(path.to_path_buf()),
        DialogChoice::Ghost | DialogChoice::Cancelled => RecoveryIntent::DismissSavedCopy,
        DialogChoice::Secondary | DialogChoice::None => return,
    };
    intent.execute(ctx, cx);
}

/// The journal row a schematic revision's question is about.
fn asked_record(cx: &Cx<'_, ProjectFeature>, key: &str) -> Option<ProjectAuditRow> {
    cx.state
        .history
        .rows
        .iter()
        .find(|row| row.key == key)
        .cloned()
}

/// Whether to restore a schematic revision: that view's drawing goes back to
/// it, one step of Undo, and nothing else in the project changes.
fn revision_restore_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let Some(asked) = cx.state.history.revision_restore.clone() else {
        return;
    };
    let Some(record) = asked_record(cx, &asked.key) else {
        cx.state.history.revision_restore = None;
        return;
    };
    let path = record
        .view
        .as_ref()
        .map(CellViewRef::display_path)
        .unwrap_or_default();
    let what = format!(
        "Only the drawing of {path} goes back to this revision; the rest of the project stays \
         as it is. Undo takes it back."
    );
    let choice = CommandForm::new("Restore schematic revision", "Restore")
        .describe(&what)
        .wide()
        .status(asked.error.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            rows.value("View", &path);
            rows.value_mono("Revision", &record.revision);
            if !record.time.is_empty() {
                rows.value("Made", &record.time);
            }
            rows.derived(&what);
            None
        });
    match choice {
        DialogChoice::Primary => {
            if let Err(error) = restore_revision(cx, &record)
                && let Some(asked) = &mut cx.state.history.revision_restore
            {
                asked.error = Some(error);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            cx.state.history.revision_restore = None;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// What changed in a schematic view's drawing since one of its revisions,
/// object by object; and the offer to restore the revision.
fn revision_comparison_dialog(ctx: &Context, cx: &mut Cx<'_, ProjectFeature>) {
    let Some(found) = cx.state.history.revision_comparison.clone() else {
        return;
    };
    let Some(record) = asked_record(cx, &found.key) else {
        cx.state.history.revision_comparison = None;
        return;
    };
    let refusal = restore_target(&record, cx.model).err();
    let mut changes = found.delta.changes.iter().collect::<Vec<_>>();
    changes.sort_by_key(|change| (change.domain, change.kind));
    let rows = changes
        .iter()
        .map(|change| {
            let object = if change.detail.is_empty() {
                change.name.clone()
            } else {
                format!("{}: {}", change.name, change.detail)
            };
            vec![
                change.kind.label().to_owned(),
                change.domain.label().to_owned(),
                object,
            ]
        })
        .collect::<Vec<_>>();
    let choice = CommandForm::new("Compare schematic revision", RESTORE)
        .describe(&found.sides)
        .wide()
        .cancel_label("Close")
        .primary_on_enter(false)
        .primary_enabled(refusal.is_none())
        .status(Ok(refusal.as_deref()))
        .show(ctx, |form| {
            form.derived(&found.sides);
            form.table(
                &[
                    TableColumn {
                        header: "Change",
                        mono: false,
                    },
                    TableColumn {
                        header: "Part",
                        mono: false,
                    },
                    TableColumn {
                        header: "Object",
                        mono: false,
                    },
                ],
                &rows,
            );
            None
        });
    match choice {
        DialogChoice::Primary => {
            cx.state.history.revision_comparison = None;
            ask_revision_restore(&mut cx.state.history, cx.model, record.key);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            cx.state.history.revision_comparison = None;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::workbench::lifecycle::project_checkpoint::{fixture_quarantine, fixture_summary};
    use crate::workbench::project::revision_history::ProjectAuditArtifactKind;

    /// A simulation plan's record of the journal, made at `made`.
    fn plan(change: &str, made: u64) -> ProjectAuditRow {
        ProjectAuditRow {
            key: "simulation-plan:bias".to_owned(),
            kind: ProjectAuditArtifactKind::SimulationPlan,
            revision: "4".to_owned(),
            time: stamp(made),
            made_unix_ms: Some(made),
            actor: String::new(),
            change: change.to_owned(),
            affected_evidence: String::new(),
            status: "Retained".to_owned(),
            schematic_revision: None,
            view: None,
            current: false,
        }
    }

    fn shown(
        recovery: &ProjectCheckpointRecoveryState,
        journal: &[ProjectAuditRow],
        filter: &str,
        kind: Option<HistoryKind>,
    ) -> Vec<String> {
        let source = HistoryListSource {
            journal: None,
            catalog: 0,
            filter: filter.to_owned(),
            kind,
        };
        let list = build_list(recovery, journal, &source);
        list.visible
            .iter()
            .map(|&index| list.rows[index].change.clone())
            .collect()
    }

    /// Checkpoints, damaged ones and the journal's records are one list,
    /// newest first, a record that keeps no time last; the filter reads
    /// every word a row says, and a kind keeps only its own rows.
    #[test]
    fn one_list_newest_first_filtered_by_its_words_and_kind() {
        let now = 1_800_000_000_000;
        let recovery = ProjectCheckpointRecoveryState {
            initialized: true,
            checkpoints: vec![
                fixture_summary(
                    "amp",
                    3,
                    ProjectCheckpointReason::Manual,
                    now - 60_000,
                    1_000,
                ),
                fixture_summary(
                    "amp",
                    2,
                    ProjectCheckpointReason::TechnologyChange,
                    now - 7_200_000,
                    1_000,
                ),
            ],
            quarantined: vec![fixture_quarantine(
                "broken.checkpoint.json",
                CheckpointDamage::Record,
                "checkpoint manifest is invalid",
            )],
            ..ProjectCheckpointRecoveryState::default()
        };
        let journal = [plan("Bias sweep", now - 600_000)];
        assert_eq!(
            shown(&recovery, &journal, "", None),
            [
                "Manual checkpoint",
                "Bias sweep",
                "Before technology change",
                "broken.checkpoint.json"
            ]
        );
        assert_eq!(shown(&recovery, &journal, "bias", None), ["Bias sweep"]);
        assert_eq!(
            shown(&recovery, &journal, "damaged", None),
            ["broken.checkpoint.json"]
        );
        assert_eq!(
            shown(&recovery, &journal, "", Some(HistoryKind::Checkpoint)),
            ["Manual checkpoint", "Before technology change"]
        );
        let list = build_list(
            &recovery,
            &journal,
            &HistoryListSource {
                journal: None,
                catalog: 0,
                filter: String::new(),
                kind: None,
            },
        );
        assert_eq!(
            list.kinds.as_ref(),
            [
                HistoryKind::Checkpoint,
                HistoryKind::SimulationPlan,
                HistoryKind::SetAside
            ]
        );
    }
}
