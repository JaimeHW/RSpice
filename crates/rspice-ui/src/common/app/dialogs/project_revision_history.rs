//! Project revision and audit history backed exclusively by persisted owners.
//!
//! Validated schematic revisions own exact restorable snapshots. Simulation
//! plans, model/PDK bindings, and governed design-management receipts expose
//! their retained identities and metadata without inventing missing actors,
//! timestamps, or restoration authority.

use egui::{Context, Frame, Grid, RichText, ScrollArea, Stroke, TextEdit, Ui};

use crate::state::model_library::ModelSourceAuthority;
use crate::state::{
    ValidatedRevisionJournal, ValidatedRevisionObjectDelta, ValidatedRevisionSemanticDelta,
    ValidatedSchematicRevision, ValidatedSchematicRevisionId,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone, select,
};
use crate::workbench::design_system::section_header;

use crate::common::app::{AppState, ConsoleMessage, RSpiceApp, SchematicEditAuthority};

const EYEBROW: &str = "PROJECT \u{00b7} SEMANTIC HISTORY \u{00b7} TRACEABILITY";
const TITLE: &str = "Project revision and audit history";
const NOT_RETAINED: &str = "not retained";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ProjectRevisionArtifactFilter {
    #[default]
    All,
    Schematic,
    SimulationPlans,
    ModelsAndPdk,
    GovernedRecords,
}

impl ProjectRevisionArtifactFilter {
    const ALL: [Self; 5] = [
        Self::All,
        Self::Schematic,
        Self::SimulationPlans,
        Self::ModelsAndPdk,
        Self::GovernedRecords,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "All project artifacts",
            Self::Schematic => "Schematic",
            Self::SimulationPlans => "Simulation plans",
            Self::ModelsAndPdk => "Models and PDK",
            Self::GovernedRecords => "Governed records",
        }
    }

    const fn accepts(self, kind: ProjectAuditArtifactKind) -> bool {
        match self {
            Self::All => true,
            Self::Schematic => matches!(kind, ProjectAuditArtifactKind::Schematic),
            Self::SimulationPlans => {
                matches!(kind, ProjectAuditArtifactKind::SimulationPlan)
            }
            Self::ModelsAndPdk => matches!(kind, ProjectAuditArtifactKind::ModelOrPdk),
            Self::GovernedRecords => matches!(kind, ProjectAuditArtifactKind::GovernedRecord),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectAuditArtifactKind {
    Schematic,
    SimulationPlan,
    ModelOrPdk,
    GovernedRecord,
}

impl ProjectAuditArtifactKind {
    const fn label(self) -> &'static str {
        match self {
            Self::Schematic => "Schematic",
            Self::SimulationPlan => "Simulation plan",
            Self::ModelOrPdk => "Model / PDK",
            Self::GovernedRecord => "Governed record",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectAuditRow {
    key: String,
    kind: ProjectAuditArtifactKind,
    revision: String,
    time: String,
    actor: String,
    change: String,
    affected_evidence: String,
    status: String,
    schematic_revision: Option<ValidatedSchematicRevisionId>,
    restorable_schematic_revision: Option<ValidatedSchematicRevisionId>,
}

impl ProjectAuditRow {
    fn matches_query(&self, query: &str) -> bool {
        query.is_empty()
            || [
                self.kind.label(),
                &self.revision,
                &self.time,
                &self.actor,
                &self.change,
                &self.affected_evidence,
                &self.status,
            ]
            .iter()
            .any(|value| value.to_ascii_lowercase().contains(query))
    }
}

#[derive(Debug, Clone)]
struct ProjectAuditComparison {
    label: String,
    fields: Vec<(&'static str, String, String)>,
}

impl ProjectAuditComparison {
    fn between(first: &ProjectAuditRow, second: &ProjectAuditRow) -> Self {
        let mut fields = Vec::new();
        comparison_field(&mut fields, "Revision", &first.revision, &second.revision);
        comparison_field(&mut fields, "Actor", &first.actor, &second.actor);
        comparison_field(&mut fields, "Change", &first.change, &second.change);
        comparison_field(
            &mut fields,
            "Affected evidence",
            &first.affected_evidence,
            &second.affected_evidence,
        );
        comparison_field(&mut fields, "Status", &first.status, &second.status);
        Self {
            label: format!("{} \u{2192} {}", first.revision, second.revision),
            fields,
        }
    }
}

#[derive(Debug, Clone)]
enum ProjectRevisionComparison {
    Schematic {
        label: String,
        delta: ValidatedRevisionSemanticDelta,
    },
    RetainedMetadata(ProjectAuditComparison),
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectRevisionHistoryDialogState {
    pub(crate) open: bool,
    pub(crate) artifact_filter: ProjectRevisionArtifactFilter,
    pub(crate) query: String,
    selected: Vec<String>,
    comparison: Option<ProjectRevisionComparison>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) expected_journal: ValidatedRevisionJournal,
    pub(crate) restore_confirmation: bool,
    pub(crate) error: Option<String>,
    pub(crate) receipt: Option<String>,
    pub(crate) body_scroll_offset: f32,
}

impl Default for ProjectRevisionHistoryDialogState {
    fn default() -> Self {
        Self {
            open: false,
            artifact_filter: ProjectRevisionArtifactFilter::All,
            query: String::new(),
            selected: Vec::new(),
            comparison: None,
            authority: None,
            expected_journal: ValidatedRevisionJournal::default(),
            restore_confirmation: false,
            error: None,
            receipt: None,
            body_scroll_offset: 0.0,
        }
    }
}

impl ProjectRevisionHistoryDialogState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Default)]
enum HistoryAction {
    #[default]
    None,
    ToggleSelection(String, bool),
    Compare,
    Restore,
}

pub(crate) fn open_project_revision_history(state: &mut AppState) {
    if !state.project_lifecycle.project_open {
        state.push_user_message(ConsoleMessage::warning(
            "Project revision history requires an open project.",
        ));
        return;
    }
    if let Err(error) = state.schematic.validated_revisions.validate() {
        state.dialogs.project_revision_history = ProjectRevisionHistoryDialogState {
            open: true,
            expected_journal: state.schematic.validated_revisions.clone(),
            authority: Some(SchematicEditAuthority::capture(state)),
            error: Some(format!(
                "The validated schematic revision journal failed integrity validation: {error}"
            )),
            ..ProjectRevisionHistoryDialogState::default()
        };
        return;
    }
    let selected = state
        .schematic
        .validated_revisions
        .records()
        .last()
        .map(|record| vec![schematic_row_key(record.id())])
        .unwrap_or_default();
    state.dialogs.project_revision_history = ProjectRevisionHistoryDialogState {
        open: true,
        selected,
        authority: Some(SchematicEditAuthority::capture(state)),
        expected_journal: state.schematic.validated_revisions.clone(),
        ..ProjectRevisionHistoryDialogState::default()
    };
}

impl RSpiceApp {
    pub(in crate::common::app) fn render_project_revision_history_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.project_revision_history.open {
            return;
        }
        let rows = project_audit_rows(&self.state);
        retain_existing_revision_selection(&mut self.state.dialogs.project_revision_history, &rows);
        let stale = history_authority_error(&self.state);
        let write_allowed = !self.state.schematic.read_only
            && !self.state.active_view_read_only()
            && !self.state.workbench.safe_mode.project_read_only();
        let mut body_scroll_offset = self
            .state
            .dialogs
            .project_revision_history
            .body_scroll_offset;
        let mut dialog = Dialog::new(EYEBROW, TITLE, "Close")
            .description(
                "Inspect retained project revisions, semantic changes, and immutable audit evidence across schematic, simulation, model, and governed records.",
            )
            .size(DialogSize::SimulationWorkflow)
            .initial_height(610.0)
            .flush_body()
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        let transaction_error = self
            .state
            .dialogs
            .project_revision_history
            .error
            .clone()
            .or_else(|| stale.clone());
        if let Some(error) = transaction_error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Revision history cannot continue",
                error,
            );
        } else if !write_allowed {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Progress,
                "Inspection only",
                "The active schematic or project is read-only. Filtering and comparison remain available; schematic restore is disabled.",
            );
        }

        let mut action = HistoryAction::None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            action = revision_history_body(
                ui,
                &rows,
                write_allowed && stale.is_none(),
                &mut self.state.dialogs.project_revision_history,
            );
            None
        });
        self.state
            .dialogs
            .project_revision_history
            .body_scroll_offset = body_scroll_offset;
        self.handle_revision_history_action(action, &rows);
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            self.state.dialogs.project_revision_history.close();
        }
    }

    fn handle_revision_history_action(&mut self, action: HistoryAction, rows: &[ProjectAuditRow]) {
        match action {
            HistoryAction::None => {}
            HistoryAction::ToggleSelection(key, additive) => {
                let dialog = &mut self.state.dialogs.project_revision_history;
                dialog.restore_confirmation = false;
                dialog.comparison = None;
                dialog.receipt = None;
                if let Some(index) = dialog.selected.iter().position(|selected| *selected == key) {
                    if additive {
                        dialog.selected.remove(index);
                    } else {
                        dialog.selected = vec![key];
                    }
                } else if additive && dialog.selected.len() < 2 {
                    dialog.selected.push(key);
                } else {
                    dialog.selected = vec![key];
                }
            }
            HistoryAction::Compare => self.compare_selected_revisions(rows),
            HistoryAction::Restore => self.restore_selected_revision(rows),
        }
    }

    fn compare_selected_revisions(&mut self, rows: &[ProjectAuditRow]) {
        let selected = self.state.dialogs.project_revision_history.selected.clone();
        if selected.len() != 2 {
            self.state.dialogs.project_revision_history.error =
                Some("Select exactly two retained records to compare.".to_owned());
            return;
        }
        let Some(first) = rows.iter().find(|row| row.key == selected[0]) else {
            self.state.dialogs.project_revision_history.error =
                Some("The first selected record no longer exists.".to_owned());
            return;
        };
        let Some(second) = rows.iter().find(|row| row.key == selected[1]) else {
            self.state.dialogs.project_revision_history.error =
                Some("The second selected record no longer exists.".to_owned());
            return;
        };
        if first.kind != second.kind {
            self.state.dialogs.project_revision_history.error = Some(
                "Semantic comparison requires two records from the same project-artifact family."
                    .to_owned(),
            );
            return;
        }

        let comparison = match (first.schematic_revision, second.schematic_revision) {
            (Some(first_id), Some(second_id)) => {
                let Some(first_record) = find_project_schematic_revision(&self.state, first_id)
                else {
                    self.state.dialogs.project_revision_history.error =
                        Some("The first selected schematic revision no longer exists.".to_owned());
                    return;
                };
                let Some(second_record) = find_project_schematic_revision(&self.state, second_id)
                else {
                    self.state.dialogs.project_revision_history.error =
                        Some("The second selected schematic revision no longer exists.".to_owned());
                    return;
                };
                let (older, newer) = if first_record.sequence() <= second_record.sequence() {
                    (first_record, second_record)
                } else {
                    (second_record, first_record)
                };
                ProjectRevisionComparison::Schematic {
                    label: format!("{} \u{2192} {}", short_digest(older), short_digest(newer)),
                    delta: older.semantic_delta_to(newer),
                }
            }
            (None, None) => ProjectRevisionComparison::RetainedMetadata(
                ProjectAuditComparison::between(first, second),
            ),
            _ => {
                self.state.dialogs.project_revision_history.error = Some(
                    "The selected records do not expose compatible semantic identities.".to_owned(),
                );
                return;
            }
        };
        let dialog = &mut self.state.dialogs.project_revision_history;
        dialog.comparison = Some(comparison);
        dialog.error = None;
        dialog.receipt = None;
    }

    fn restore_selected_revision(&mut self, rows: &[ProjectAuditRow]) {
        let result = (|| {
            history_authority_error(&self.state).map_or(Ok(()), Err)?;
            if self.state.workbench.safe_mode.project_read_only()
                || self.state.schematic.read_only
                || self.state.active_view_read_only()
            {
                return Err("The active schematic or project is read-only.".to_owned());
            }
            let dialog = &self.state.dialogs.project_revision_history;
            if dialog.selected.len() != 1 {
                return Err(
                    "Select exactly one validated schematic revision to restore.".to_owned(),
                );
            }
            if dialog.expected_journal != self.state.schematic.validated_revisions {
                return Err(
                    "The validated revision journal changed. Close and reopen revision history."
                        .to_owned(),
                );
            }
            let selected = rows
                .iter()
                .find(|row| row.key == dialog.selected[0])
                .ok_or_else(|| "The selected record no longer exists.".to_owned())?;
            let id = selected.restorable_schematic_revision.ok_or_else(|| {
                "Restore is available only for a validated revision of the active schematic view."
                    .to_owned()
            })?;
            self.state
                .schematic
                .restore_validated_revision(id)
                .map_err(|error| error.to_string())?;
            self.state.sync_active_schematic_to_workspace();
            self.invalidate_simulation_preflight();
            self.state.ui.netlist.current_generation_input_digest = None;
            Ok::<(), String>(())
        })();

        match result {
            Ok(()) => {
                let authority = SchematicEditAuthority::capture(&self.state);
                let dialog = &mut self.state.dialogs.project_revision_history;
                dialog.authority = Some(authority);
                dialog.expected_journal = self.state.schematic.validated_revisions.clone();
                dialog.restore_confirmation = false;
                dialog.error = None;
                dialog.receipt = Some(
                    "Restored the exact validated schematic snapshot. One Undo returns to the prior working design; retained result manifests remain immutable."
                        .to_owned(),
                );
            }
            Err(error) => {
                let dialog = &mut self.state.dialogs.project_revision_history;
                dialog.restore_confirmation = false;
                dialog.error = Some(error);
            }
        }
    }
}

fn history_authority_error(state: &AppState) -> Option<String> {
    state
        .dialogs
        .project_revision_history
        .authority
        .as_ref()
        .ok_or_else(|| {
            "Revision-history authority is unavailable. Close and reopen the workflow.".to_owned()
        })
        .and_then(|authority| authority.validate(state, "Project revision history"))
        .err()
}

fn retain_existing_revision_selection(
    dialog: &mut ProjectRevisionHistoryDialogState,
    rows: &[ProjectAuditRow],
) {
    dialog
        .selected
        .retain(|selected| rows.iter().any(|row| row.key == *selected));
    dialog.selected.truncate(2);
}

fn revision_history_body(
    ui: &mut Ui,
    rows: &[ProjectAuditRow],
    write_allowed: bool,
    dialog: &mut ProjectRevisionHistoryDialogState,
) -> HistoryAction {
    let t = Tokens::get(ui.ctx());
    let mut action = HistoryAction::None;
    Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let filter_options = ProjectRevisionArtifactFilter::ALL
                    .iter()
                    .map(|filter| filter.label().to_owned())
                    .collect::<Vec<_>>();
                if let Some(index) = select(
                    ui,
                    "project-revision-artifact-filter",
                    "Project artifact filter",
                    dialog.artifact_filter.label(),
                    &filter_options,
                    180.0,
                ) {
                    dialog.artifact_filter = ProjectRevisionArtifactFilter::ALL[index];
                    dialog.selected.clear();
                    dialog.comparison = None;
                    dialog.restore_confirmation = false;
                }
                ui.add(
                    TextEdit::singleline(&mut dialog.query)
                        .desired_width((ui.available_width() - 150.0).max(180.0))
                        .hint_text("Revision, actor, artifact, or message\u{2026}"),
                );
                if Button::new("Compare selected")
                    .enabled(dialog.selected.len() == 2)
                    .show(ui)
                    .clicked()
                {
                    action = HistoryAction::Compare;
                }
            });
        });

    let visible = visible_rows(rows, dialog);
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .show(ui, |ui| {
            if visible.is_empty() {
                ui.add_space(24.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new(if dialog.query.trim().is_empty() {
                            format!(
                                "No retained {} records in this project",
                                dialog.artifact_filter.label().to_ascii_lowercase()
                            )
                        } else {
                            "No retained records match this search".to_owned()
                        })
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                    );
                });
                ui.add_space(24.0);
                return;
            }
            ScrollArea::both()
                .id_salt("project-revision-history-table")
                .max_height(286.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        revision_graph(ui, &visible, &dialog.selected);
                        Grid::new("project-revision-history-grid")
                            .min_col_width(86.0)
                            .striped(true)
                            .show(ui, |ui| {
                                table_heading(ui, "Revision");
                                table_heading(ui, "Time");
                                table_heading(ui, "Actor");
                                table_heading(ui, "Change");
                                table_heading(ui, "Affected evidence");
                                table_heading(ui, "Status");
                                ui.end_row();
                                for row in &visible {
                                    let row = *row;
                                    let selected = dialog.selected.contains(&row.key);
                                    let selection_glyph =
                                        if selected { "\u{25c9} " } else { "\u{25cb} " };
                                    let response = ui.selectable_label(
                                        selected,
                                        RichText::new(format!("{selection_glyph}{}", row.revision))
                                            .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                                    );
                                    if response.clicked() {
                                        action = HistoryAction::ToggleSelection(
                                            row.key.clone(),
                                            ui.input(|input| {
                                                input.modifiers.ctrl || input.modifiers.command
                                            }),
                                        );
                                    }
                                    ui.monospace(&row.time);
                                    ui.label(&row.actor);
                                    ui.label(&row.change);
                                    ui.label(&row.affected_evidence);
                                    let status_color = match row.status.as_str() {
                                        "accepted" | "passed" | "attached" | "authenticated" => {
                                            t.color.ok
                                        }
                                        "working" | "unpinned" | "findings" => t.color.warn,
                                        _ => t.color.text,
                                    };
                                    ui.label(
                                        RichText::new(&row.status)
                                            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                            .color(status_color),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
                });
        });

    if let Some(comparison) = dialog.comparison.as_ref() {
        revision_comparison(ui, comparison);
    }

    ui.horizontal(|ui| {
        let restore_ready = write_allowed
            && dialog.selected.len() == 1
            && rows.iter().any(|row| {
                row.key == dialog.selected[0] && row.restorable_schematic_revision.is_some()
            });
        let restore_label = if dialog.restore_confirmation {
            "Confirm restore"
        } else {
            "Restore selected\u{2026}"
        };
        if Button::new(restore_label)
            .enabled(restore_ready)
            .destructive(dialog.restore_confirmation)
            .show(ui)
            .clicked()
        {
            if dialog.restore_confirmation {
                action = HistoryAction::Restore;
            } else {
                dialog.restore_confirmation = true;
            }
        }
        if dialog.restore_confirmation {
            ui.label(
                RichText::new(
                    "Restores only the exact validated schematic snapshot; result manifests are not rewritten.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.warn),
            );
        }
    });

    if let Some(receipt) = dialog.receipt.as_deref() {
        ui.label(
            RichText::new(receipt)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.ok),
        );
    }

    ui.columns(2, |columns| {
        revision_note_card(
            &mut columns[0],
            "Semantic comparison",
            "Schematic records compare topology and project-portable policy. Other artifact families compare only metadata their durable owners retain.",
        );
        revision_note_card(
            &mut columns[1],
            "Immutable results",
            "Result manifests retain their original input revision and are never rewritten when the project moves forward.",
        );
    });
    action
}

fn visible_rows<'a>(
    rows: &'a [ProjectAuditRow],
    dialog: &ProjectRevisionHistoryDialogState,
) -> Vec<&'a ProjectAuditRow> {
    let query = dialog.query.trim().to_ascii_lowercase();
    rows.iter()
        .filter(|row| dialog.artifact_filter.accepts(row.kind) && row.matches_query(&query))
        .collect()
}

fn project_audit_rows(state: &AppState) -> Vec<ProjectAuditRow> {
    let mut rows = Vec::new();
    append_schematic_rows(&mut rows, &state.schematic.validated_revisions, true);
    let active_key = state.workspace.active_view.key();
    let mut retained_schematics = state
        .workspace
        .schematic_buffers
        .iter()
        .filter(|entry| entry.0.as_str() != active_key.as_str())
        .collect::<Vec<_>>();
    retained_schematics.sort_by(|(left, _), (right, _)| left.cmp(right));
    for (_, schematic) in retained_schematics {
        append_schematic_rows(&mut rows, &schematic.validated_revisions, false);
    }
    append_simulation_plan_rows(&mut rows, state);
    append_model_and_pdk_rows(&mut rows, state);
    append_governed_rows(&mut rows, state);
    rows
}

fn find_project_schematic_revision(
    state: &AppState,
    id: ValidatedSchematicRevisionId,
) -> Option<&ValidatedSchematicRevision> {
    state
        .schematic
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == id)
        .or_else(|| {
            state
                .workspace
                .schematic_buffers
                .values()
                .flat_map(|schematic| schematic.validated_revisions.records())
                .find(|record| record.id() == id)
        })
}

fn append_schematic_rows(
    rows: &mut Vec<ProjectAuditRow>,
    journal: &ValidatedRevisionJournal,
    active_view: bool,
) {
    let latest = journal.records().last().map(ValidatedSchematicRevision::id);
    for record in journal.records().iter().rev() {
        let status = if record.is_accepted_baseline() {
            "accepted"
        } else if active_view && Some(record.id()) == latest {
            "working"
        } else {
            "validated"
        };
        rows.push(ProjectAuditRow {
            key: schematic_row_key(record.id()),
            kind: ProjectAuditArtifactKind::Schematic,
            revision: short_digest(record),
            time: format_revision_time(record.created_unix_ms()),
            actor: record.author().to_owned(),
            change: if record.is_accepted_baseline() {
                "Accepted schematic baseline".to_owned()
            } else {
                record.revision_note().to_owned()
            },
            affected_evidence: affected_schematic_evidence(record),
            status: status.to_owned(),
            schematic_revision: Some(record.id()),
            restorable_schematic_revision: active_view.then_some(record.id()),
        });
    }
}

fn append_simulation_plan_rows(rows: &mut Vec<ProjectAuditRow>, state: &AppState) {
    if let Some(plan) = state.sim_setup.analysis_plan.as_ref() {
        rows.push(simulation_plan_row(
            state.sim_setup.active_plan_name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.instances().len(),
            state.sim_setup.active_plan_lineage(),
            true,
        ));
    }
    for plan in state.sim_setup.inactive_plans().iter().rev() {
        rows.push(simulation_plan_row(
            plan.name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.analysis_plan().instances().len(),
            plan.lineage(),
            false,
        ));
    }
}

fn simulation_plan_row(
    name: &str,
    id: String,
    revision: String,
    analysis_count: usize,
    lineage: crate::common::app::SimulationPlanLineage,
    active: bool,
) -> ProjectAuditRow {
    let lineage_evidence = match (lineage.source_plan_id(), lineage.source_revision()) {
        (Some(source), Some(source_revision)) => {
            format!("cloned from {source} @ {}", source_revision.get())
        }
        _ => "root plan".to_owned(),
    };
    ProjectAuditRow {
        key: format!("simulation-plan:{id}:{revision}"),
        kind: ProjectAuditArtifactKind::SimulationPlan,
        revision,
        time: NOT_RETAINED.to_owned(),
        actor: NOT_RETAINED.to_owned(),
        change: format!(
            "{} \u{00b7} {name}",
            if active { "Active" } else { "Retained" }
        ),
        affected_evidence: format!("{analysis_count} analyses \u{00b7} {lineage_evidence}"),
        status: if active { "working" } else { "retained" }.to_owned(),
        schematic_revision: None,
        restorable_schematic_revision: None,
    }
}

fn append_model_and_pdk_rows(rows: &mut Vec<ProjectAuditRow>, state: &AppState) {
    if let Some(binding) = state.workspace.project.technology_binding() {
        let digest = binding
            .source_closure()
            .iter()
            .find(|source| source.path == binding.root_source())
            .map(|source| short_text(&source.digest.to_string()))
            .unwrap_or_else(|| state.workspace.project.revision().get().to_string());
        rows.push(ProjectAuditRow {
            key: format!("technology-binding:{}:{}", binding.model_library(), digest),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision: digest,
            time: NOT_RETAINED.to_owned(),
            actor: "Project metadata transaction".to_owned(),
            change: format!("Attached technology \u{00b7} {}", binding.display_label()),
            affected_evidence: format!(
                "{} models \u{00b7} {} pinned sources \u{00b7} {} process sections",
                binding.model_count(),
                binding.source_closure().len(),
                binding.process_sections().len()
            ),
            status: "attached".to_owned(),
            schematic_revision: None,
            restorable_schematic_revision: None,
        });
    }

    for library in state.model_library_manager.libraries_sorted() {
        let (revision, status, authority): (String, &str, String) = match library.source_authority {
            ModelSourceAuthority::ProjectOwned {
                source_id,
                revision,
                digest,
            } => (
                revision.get().to_string(),
                "authenticated",
                format!(
                    "project source {source_id} \u{00b7} {}",
                    short_text(&digest.to_string())
                ),
            ),
            ModelSourceAuthority::External => {
                let root_digest = library
                    .root_path
                    .as_ref()
                    .and_then(|root| {
                        library
                            .source_closure
                            .iter()
                            .find(|source| &source.path == root)
                    })
                    .map(|source| short_text(&source.digest.to_string()));
                match root_digest {
                    Some(digest) => (
                        digest.clone(),
                        "authenticated",
                        format!("external source \u{00b7} root digest {digest}"),
                    ),
                    None => (
                        "unversioned".to_owned(),
                        "unpinned",
                        "external source has no accepted closure".to_owned(),
                    ),
                }
            }
            ModelSourceAuthority::BuiltIn => (
                nonempty_or(&library.version, "built-in"),
                "built-in",
                "signed application catalog".to_owned(),
            ),
        };
        rows.push(ProjectAuditRow {
            key: format!("model-library:{}:{revision}", library.name),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision,
            time: NOT_RETAINED.to_owned(),
            actor: authority,
            change: format!(
                "Model library \u{00b7} {}{}",
                library.name,
                if library.pdk_name.trim().is_empty() {
                    String::new()
                } else {
                    format!(" \u{00b7} {}", library.pdk_name)
                }
            ),
            affected_evidence: format!(
                "{} models \u{00b7} {} corners \u{00b7} {} pinned sources",
                library.model_count(),
                library.corner_count(),
                library.source_closure.len()
            ),
            status: status.to_owned(),
            schematic_revision: None,
            restorable_schematic_revision: None,
        });
    }
}

fn append_governed_rows(rows: &mut Vec<ProjectAuditRow>, state: &AppState) {
    let management = &state.workspace.design_management;
    for entry in management.annotation().journal().iter().rev() {
        rows.push(ProjectAuditRow {
            key: format!("annotation-journal:{}", entry.id()),
            kind: ProjectAuditArtifactKind::GovernedRecord,
            revision: entry.sequence().to_string(),
            time: NOT_RETAINED.to_owned(),
            actor: "Annotation policy transaction".to_owned(),
            change: format!("Committed annotation policy r{}", entry.policy_revision()),
            affected_evidence: format!(
                "{} mappings \u{00b7} policy {} \u{00b7} receipt {}",
                entry.mappings().len(),
                short_text(&entry.policy_digest().to_string()),
                short_text(&entry.semantic_digest().to_string())
            ),
            status: "immutable".to_owned(),
            schematic_revision: None,
            restorable_schematic_revision: None,
        });
    }
    for receipt in management.hierarchy_audits().iter().rev() {
        rows.push(ProjectAuditRow {
            key: format!("hierarchy-audit:{}", receipt.id()),
            kind: ProjectAuditArtifactKind::GovernedRecord,
            revision: receipt.sequence().to_string(),
            time: NOT_RETAINED.to_owned(),
            actor: "Hierarchy audit transaction".to_owned(),
            change: "Hierarchy audit receipt".to_owned(),
            affected_evidence: format!(
                "{} resolved subjects \u{00b7} {} findings \u{00b7} receipt {}",
                receipt.resolved_subjects(),
                receipt.findings().len(),
                short_text(&receipt.semantic_digest().to_string())
            ),
            status: if receipt.passed() {
                "passed"
            } else {
                "findings"
            }
            .to_owned(),
            schematic_revision: None,
            restorable_schematic_revision: None,
        });
    }
}

fn revision_comparison(ui: &mut Ui, comparison: &ProjectRevisionComparison) {
    match comparison {
        ProjectRevisionComparison::Schematic { label, delta } => {
            semantic_comparison(ui, delta, Some(label));
        }
        ProjectRevisionComparison::RetainedMetadata(comparison) => {
            section_header(ui, "Retained metadata comparison", Some(&comparison.label));
            if comparison.fields.is_empty() {
                ui.label("The selected retained records expose identical metadata.");
                return;
            }
            Grid::new("retained-project-record-comparison")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    table_heading(ui, "Field");
                    table_heading(ui, "First");
                    table_heading(ui, "Second");
                    ui.end_row();
                    for (field, first, second) in &comparison.fields {
                        ui.label(*field);
                        ui.monospace(first);
                        ui.monospace(second);
                        ui.end_row();
                    }
                });
        }
    }
}

fn revision_graph(ui: &mut Ui, rows: &[&ProjectAuditRow], selected: &[String]) {
    let t = Tokens::get(ui.ctx());
    let row_height = 20.0;
    let header_height = 22.0;
    let height = (header_height + row_height * rows.len() as f32).max(48.0);
    let (rect, response) = ui.allocate_exact_size(egui::vec2(30.0, height), egui::Sense::hover());
    response.on_hover_text("Revision graph");
    if rows.is_empty() {
        return;
    }
    let center_x = rect.center().x;
    let first_y = rect.top() + header_height + row_height * 0.5;
    for (index, row) in rows.iter().enumerate() {
        let y = first_y + index as f32 * row_height;
        if let Some(next) = rows.get(index + 1)
            && graph_records_are_linked(row, next)
        {
            ui.painter().line_segment(
                [
                    egui::pos2(center_x, y),
                    egui::pos2(center_x, y + row_height),
                ],
                Stroke::new(1.0, t.color.border_strong),
            );
        }
        let is_selected = selected.contains(&row.key);
        let approved = matches!(
            row.status.as_str(),
            "accepted" | "passed" | "attached" | "authenticated"
        );
        let color = if approved {
            t.color.ok
        } else if is_selected {
            t.color.accent
        } else {
            t.color.text_dim
        };
        if row.affected_evidence.contains("cloned from") {
            ui.painter().line_segment(
                [
                    egui::pos2(center_x, y),
                    egui::pos2(center_x + 8.0, y + row_height * 0.5),
                ],
                Stroke::new(1.0, t.color.border_strong),
            );
        }
        ui.painter().circle_filled(
            egui::pos2(center_x, y),
            if is_selected { 4.5 } else { 3.5 },
            color,
        );
    }
}

fn graph_records_are_linked(first: &ProjectAuditRow, second: &ProjectAuditRow) -> bool {
    if first.kind != second.kind {
        return false;
    }
    match first.kind {
        ProjectAuditArtifactKind::Schematic => {
            evidence_owner(&first.affected_evidence) == evidence_owner(&second.affected_evidence)
        }
        ProjectAuditArtifactKind::GovernedRecord => {
            let first_annotation = first.change.starts_with("Committed annotation policy");
            let second_annotation = second.change.starts_with("Committed annotation policy");
            let first_hierarchy = first.change == "Hierarchy audit receipt";
            let second_hierarchy = second.change == "Hierarchy audit receipt";
            (first_annotation && second_annotation) || (first_hierarchy && second_hierarchy)
        }
        ProjectAuditArtifactKind::SimulationPlan | ProjectAuditArtifactKind::ModelOrPdk => false,
    }
}

fn evidence_owner(value: &str) -> &str {
    value.split(" \u{00b7} ").next().unwrap_or(value)
}

fn semantic_comparison(ui: &mut Ui, delta: &ValidatedRevisionSemanticDelta, label: Option<&str>) {
    section_header(ui, "Semantic comparison", label);
    if delta.is_empty() {
        ui.label("The selected validated revisions contain identical schematic semantics.");
        return;
    }
    Grid::new("validated-revision-semantic-delta")
        .num_columns(4)
        .striped(true)
        .show(ui, |ui| {
            table_heading(ui, "Domain");
            table_heading(ui, "Added");
            table_heading(ui, "Removed");
            table_heading(ui, "Modified");
            ui.end_row();
            delta_row(ui, "Components", delta.components);
            delta_row(ui, "Wires", delta.wires);
            delta_row(ui, "Buses", delta.buses);
            delta_row(ui, "Bus taps", delta.bus_taps);
            delta_row(ui, "Junctions", delta.junctions);
            delta_row(ui, "Net labels", delta.net_labels);
            delta_row(ui, "Design notes", delta.design_notes);
            delta_row(ui, "Documentation shapes", delta.documentation_shapes);
            ui.label("Connections");
            ui.monospace(delta.connections_added.to_string());
            ui.monospace(delta.connections_removed.to_string());
            ui.monospace("\u{2014}");
            ui.end_row();
            ui.label("Document policy");
            ui.monospace("\u{2014}");
            ui.monospace("\u{2014}");
            ui.monospace(if delta.grid_changed || delta.document_policy_changed {
                "changed"
            } else {
                "\u{2014}"
            });
            ui.end_row();
        });
}

fn delta_row(ui: &mut Ui, label: &str, delta: ValidatedRevisionObjectDelta) {
    ui.label(label);
    ui.monospace(delta.added.to_string());
    ui.monospace(delta.removed.to_string());
    ui.monospace(delta.modified.to_string());
    ui.end_row();
}

fn revision_note_card(ui: &mut Ui, title: &str, body: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(title)
                    .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                RichText::new(body)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
}

fn table_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label.to_ascii_uppercase())
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
}

fn schematic_row_key(id: ValidatedSchematicRevisionId) -> String {
    format!("schematic:{}", id.as_uuid())
}

fn short_digest(record: &ValidatedSchematicRevision) -> String {
    short_text(&record.revision_digest().to_string())
}

fn short_text(value: &str) -> String {
    value.chars().take(10).collect()
}

fn nonempty_or(value: &str, fallback: &str) -> String {
    let value = value.trim();
    if value.is_empty() {
        fallback.to_owned()
    } else {
        value.to_owned()
    }
}

fn format_revision_time(timestamp_ms: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().try_into().unwrap_or(u64::MAX))
        .unwrap_or_default();
    if timestamp_ms > now {
        return "clock skew".to_owned();
    }
    match now - timestamp_ms {
        0..=59_999 => "just now".to_owned(),
        60_000..=3_599_999 => format!("{} min ago", (now - timestamp_ms) / 60_000),
        3_600_000..=86_399_999 => format!("{} h ago", (now - timestamp_ms) / 3_600_000),
        elapsed => format!("{} d ago", elapsed / 86_400_000),
    }
}

fn affected_schematic_evidence(record: &ValidatedSchematicRevision) -> String {
    let findings = record.finding_counts();
    let mut parts = vec![record.view_identity().to_owned()];
    if record.dependency_count() > 0 {
        parts.push(format!("{} dependencies", record.dependency_count()));
    }
    if findings.advisories > 0 {
        parts.push(format!("{} advisories", findings.advisories));
    }
    if findings.blockers > 0 {
        parts.push(format!("{} blockers", findings.blockers));
    }
    if parts.len() == 1 {
        parts.push(format!("project revision {}", record.project_revision()));
    }
    parts.join(" \u{00b7} ")
}

fn comparison_field(
    fields: &mut Vec<(&'static str, String, String)>,
    label: &'static str,
    first: &str,
    second: &str,
) {
    if first != second {
        fields.push((label, first.to_owned(), second.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProjectId;
    use crate::state::{ComponentType, Point, ValidatedRevisionRequest, ValidationFindingCounts};

    fn append(state: &mut crate::state::SchematicState, note: &str) {
        let digest = state.validated_design_content_digest().unwrap();
        state
            .append_validated_revision(ValidatedRevisionRequest {
                project_id: ProjectId::new().to_string(),
                project_revision: 1,
                view_identity: "user/top/schematic".to_owned(),
                revision_note: note.to_owned(),
                author: CURRENT_TEST_ACTOR.to_owned(),
                validation_receipt_digest: digest,
                finding_counts: ValidationFindingCounts::default(),
                dependencies: Vec::new(),
                advisory_dispositions: Vec::new(),
            })
            .unwrap();
    }

    const CURRENT_TEST_ACTOR: &str = "Test engineer";

    #[test]
    fn search_is_bound_to_retained_record_fields() {
        let mut state = crate::state::SchematicState::default();
        append(&mut state, "Update compensation");
        let rows = state
            .validated_revisions
            .records()
            .iter()
            .map(|record| ProjectAuditRow {
                key: schematic_row_key(record.id()),
                kind: ProjectAuditArtifactKind::Schematic,
                revision: short_digest(record),
                time: format_revision_time(record.created_unix_ms()),
                actor: record.author().to_owned(),
                change: record.revision_note().to_owned(),
                affected_evidence: affected_schematic_evidence(record),
                status: "validated".to_owned(),
                schematic_revision: Some(record.id()),
                restorable_schematic_revision: Some(record.id()),
            })
            .collect::<Vec<_>>();
        let mut dialog = ProjectRevisionHistoryDialogState {
            query: "compensation".to_owned(),
            ..ProjectRevisionHistoryDialogState::default()
        };
        assert_eq!(visible_rows(&rows, &dialog).len(), 1);
        dialog.query = "fixture-only-row".to_owned();
        assert!(visible_rows(&rows, &dialog).is_empty());
    }

    #[test]
    fn semantic_compare_and_restore_are_exact_and_undoable() {
        let mut state = crate::state::SchematicState::default();
        append(&mut state, "Empty baseline");
        let first = state.validated_revisions.records()[0].clone();
        state.add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut state, "Add resistor");
        let second = state.validated_revisions.records()[1].clone();
        let delta = first.semantic_delta_to(&second);
        assert_eq!(delta.components.added, 1);

        state
            .restore_validated_revision(first.id())
            .expect("validated restore");
        assert!(state.components.is_empty());
        assert!(state.undo());
        assert_eq!(state.components.len(), 1);
    }

    #[test]
    fn artifact_filters_never_relabel_records() {
        let rows = vec![
            ProjectAuditRow {
                key: "schematic:one".to_owned(),
                kind: ProjectAuditArtifactKind::Schematic,
                revision: "one".to_owned(),
                time: NOT_RETAINED.to_owned(),
                actor: NOT_RETAINED.to_owned(),
                change: "schematic".to_owned(),
                affected_evidence: String::new(),
                status: "validated".to_owned(),
                schematic_revision: None,
                restorable_schematic_revision: None,
            },
            ProjectAuditRow {
                key: "simulation-plan:two".to_owned(),
                kind: ProjectAuditArtifactKind::SimulationPlan,
                revision: "two".to_owned(),
                time: NOT_RETAINED.to_owned(),
                actor: NOT_RETAINED.to_owned(),
                change: "plan".to_owned(),
                affected_evidence: String::new(),
                status: "working".to_owned(),
                schematic_revision: None,
                restorable_schematic_revision: None,
            },
        ];
        let dialog = ProjectRevisionHistoryDialogState {
            artifact_filter: ProjectRevisionArtifactFilter::SimulationPlans,
            ..ProjectRevisionHistoryDialogState::default()
        };
        let visible = visible_rows(&rows, &dialog);
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].kind, ProjectAuditArtifactKind::SimulationPlan);
    }
}
