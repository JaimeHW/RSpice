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
    text_field,
};
use crate::workbench::design_system::section_header;

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::dialogs::project_revision_history::{
    ProjectAuditArtifactKind, ProjectAuditComparison, ProjectRevisionArtifactFilter,
    ProjectRevisionComparison, ProjectRevisionHistoryDialogState,
};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;

const TITLE: &str = "Project revision and audit history";
/// A fact the record's owner does not keep: left blank, as an empty cell,
/// rather than stated in every row of a column that has nothing to say.
const NOT_RETAINED: &str = "";

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

/// The retained metadata two records of one family differ in.
impl ProjectAuditComparison {
    fn between(first: &ProjectAuditRow, second: &ProjectAuditRow) -> Self {
        let mut fields = Vec::new();
        comparison_field(&mut fields, "Revision", &first.revision, &second.revision);
        comparison_field(&mut fields, "Actor", &first.actor, &second.actor);
        comparison_field(&mut fields, "Change", &first.change, &second.change);
        comparison_field(
            &mut fields,
            "Affected records",
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

#[derive(Debug, Clone, Default)]
enum HistoryAction {
    #[default]
    None,
    ToggleSelection(String, bool),
    Compare,
    Restore,
}

pub(crate) fn open_project_revision_history(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    platform: &PlatformState,
) {
    // The command is greyed with no project open.
    if !platform.project_lifecycle.is_open() {
        return;
    }
    if let Err(error) = model.schematic.document().validated_revisions.validate() {
        *dialog = ProjectRevisionHistoryDialogState {
            open: true,
            expected_journal: model.schematic.document().validated_revisions.clone(),
            authority: Some(SchematicEditAuthority::capture(model)),
            error: Some(format!(
                "The validated schematic revision journal failed integrity validation: {error}"
            )),
            ..ProjectRevisionHistoryDialogState::default()
        };
        return;
    }
    let selected = model
        .schematic
        .document()
        .validated_revisions
        .records()
        .last()
        .map(|record| vec![schematic_row_key(record.id())])
        .unwrap_or_default();
    *dialog = ProjectRevisionHistoryDialogState {
        open: true,
        selected,
        authority: Some(SchematicEditAuthority::capture(model)),
        expected_journal: model.schematic.document().validated_revisions.clone(),
        ..ProjectRevisionHistoryDialogState::default()
    };
}

impl ProjectFeature {
    /// Draw Project revision history while it is open, and carry out what
    /// its body asked for: a selection, a comparison or a restore.
    pub(in crate::workbench) fn render_project_revision_history_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        if !cx.state.project_revision_history.open {
            return;
        }
        let rows = project_audit_rows(cx.model);
        retain_existing_revision_selection(&mut cx.state.project_revision_history, &rows);
        let stale = history_authority_error(&cx.state.project_revision_history, cx.model);
        let write_allowed = !cx.model.schematic.session.read_only
            && !cx.model.active_view_read_only()
            && !cx.model.safe_mode.project_read_only();
        let mut body_scroll_offset = cx.state.project_revision_history.body_scroll_offset;
        let mut dialog = Dialog::new(TITLE, "Close")
            .description(
                "Inspect saved project revisions, what changed, and the audit history of schematics, simulations, models, and other records.",
            )
            .size(DialogSize::SimulationWorkflow)
            .initial_height(610.0)
            .overlay_scrollbar()
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl)
            .body_scroll_offset(&mut body_scroll_offset);
        let transaction_error = cx
            .state
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
                &mut cx.state.project_revision_history,
            );
            None
        });
        cx.state.project_revision_history.body_scroll_offset = body_scroll_offset;
        handle_revision_history_action(cx, action, &rows);
        if matches!(
            choice,
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled
        ) {
            cx.state.project_revision_history.close();
        }
    }
}

fn handle_revision_history_action(
    cx: &mut Cx<'_, ProjectFeature>,
    action: HistoryAction,
    rows: &[ProjectAuditRow],
) {
    match action {
        HistoryAction::None => {}
        HistoryAction::ToggleSelection(key, additive) => {
            let dialog = &mut cx.state.project_revision_history;
            dialog.restore_confirmation = false;
            dialog.comparison = None;
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
        HistoryAction::Compare => {
            compare_selected_revisions(&mut cx.state.project_revision_history, cx.model, rows);
        }
        HistoryAction::Restore => restore_selected_revision(cx, rows),
    }
}

fn compare_selected_revisions(
    dialog: &mut ProjectRevisionHistoryDialogState,
    model: &Model,
    rows: &[ProjectAuditRow],
) {
    let selected = dialog.selected.clone();
    if selected.len() != 2 {
        dialog.error = Some("Select exactly two retained records to compare.".to_owned());
        return;
    }
    let Some(first) = rows.iter().find(|row| row.key == selected[0]) else {
        dialog.error = Some("The first selected record no longer exists.".to_owned());
        return;
    };
    let Some(second) = rows.iter().find(|row| row.key == selected[1]) else {
        dialog.error = Some("The second selected record no longer exists.".to_owned());
        return;
    };
    if first.kind != second.kind {
        dialog.error = Some(
            "Semantic comparison requires two records from the same project-artifact family."
                .to_owned(),
        );
        return;
    }

    let comparison = match (first.schematic_revision, second.schematic_revision) {
        (Some(first_id), Some(second_id)) => {
            let Some(first_record) = find_project_schematic_revision(model, first_id) else {
                dialog.error =
                    Some("The first selected schematic revision no longer exists.".to_owned());
                return;
            };
            let Some(second_record) = find_project_schematic_revision(model, second_id) else {
                dialog.error =
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
            dialog.error = Some(
                "The selected records do not expose compatible semantic identities.".to_owned(),
            );
            return;
        }
    };
    dialog.comparison = Some(comparison);
    dialog.error = None;
}

fn restore_selected_revision(cx: &mut Cx<'_, ProjectFeature>, rows: &[ProjectAuditRow]) {
    let result = (|| {
        history_authority_error(&cx.state.project_revision_history, cx.model)
            .map_or(Ok(()), Err)?;
        if cx.model.safe_mode.project_read_only()
            || cx.model.schematic.session.read_only
            || cx.model.active_view_read_only()
        {
            return Err("The active schematic or project is read-only.".to_owned());
        }
        let dialog = &cx.state.project_revision_history;
        if dialog.selected.len() != 1 {
            return Err("Select exactly one validated schematic revision to restore.".to_owned());
        }
        if dialog.expected_journal != cx.model.schematic.document().validated_revisions {
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
        cx.model
            .schematic
            .restore_validated_revision(id)
            .map_err(|error| error.to_string())?;
        if let Some(prompt) = cx.model.sync_active_schematic_to_workspace() {
            cx.wb.prompt(prompt);
        }
        // What the run's preflight decided, the run prepared and the
        // netlist generated was for the design the restore replaced.
        // The next two calls are what RSpiceApp::invalidate_simulation_preflight does.
        cx.runs.preflight.invalidate();
        cx.handles.run_controller.clear_prepared_run();
        cx.model.netlist_artifacts.current_generation_input_digest = None;
        Ok::<(), String>(())
    })();

    match result {
        Ok(()) => {
            let authority = SchematicEditAuthority::capture(&*cx.model);
            let dialog = &mut cx.state.project_revision_history;
            dialog.authority = Some(authority);
            dialog.expected_journal = cx.model.schematic.document().validated_revisions.clone();
            dialog.restore_confirmation = false;
            // The restored design is on the canvas behind the dialog.
            dialog.error = None;
        }
        Err(error) => {
            let dialog = &mut cx.state.project_revision_history;
            dialog.restore_confirmation = false;
            dialog.error = Some(error);
        }
    }
}

fn history_authority_error(
    dialog: &ProjectRevisionHistoryDialogState,
    model: &Model,
) -> Option<String> {
    dialog
        .authority
        .as_ref()
        .ok_or_else(|| {
            "Revision-history authority is unavailable. Close and reopen the workflow.".to_owned()
        })
        .and_then(|authority| authority.validate(model, "Project revision history"))
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
    // The filter, the search and the comparison on one row, on the body's
    // inset with everything under it. It was a band of the panel tone that
    // stopped short of the dialog's right edge over a table that touched
    // its left one.
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
        // The search takes what the comparison leaves, so the row ends on
        // the table's right edge.
        let compare = Button::new("Compare selected").enabled(dialog.selected.len() == 2);
        let compare_width = compare.measured_width(ui);
        let width =
            (ui.available_width() - compare_width - 2.0 * ui.spacing().item_spacing.x).max(180.0);
        text_field(
            ui,
            TextEdit::singleline(&mut dialog.query)
                .hint_text("Revision, actor, artifact, or message\u{2026}"),
            width,
        );
        if compare.show(ui).clicked() {
            action = HistoryAction::Compare;
        }
    });

    let visible = visible_rows(rows, dialog);
    Frame::new()
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .inner_margin(egui::Margin::symmetric(8, 4))
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
            ScrollArea::vertical()
                .id_salt("project-revision-history-table")
                .max_height(286.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    let shown = shown_columns(&visible);
                    let widths = column_widths(ui, &visible, &shown, ui.available_width());
                    Grid::new("project-revision-history-grid")
                        .min_col_width(0.0)
                        .spacing(egui::vec2(COLUMN_GAP, 4.0))
                        .show(ui, |ui| {
                            for (&column, &width) in shown.iter().zip(&widths) {
                                table_heading(ui, COLUMNS[column], width);
                            }
                            ui.end_row();
                            for row in &visible {
                                let row = *row;
                                let selected = dialog.selected.contains(&row.key);
                                // The row is pressed to select it: one for
                                // a restore, two for a comparison. Its fill
                                // goes under the cells, which are laid out
                                // first so the row's extent is known.
                                let fill = ui.painter().add(egui::Shape::Noop);
                                let texts = row_cells(row);
                                let cells = shown
                                    .iter()
                                    .zip(&widths)
                                    .map(|(&column, &width)| {
                                        table_cell(ui, texts[column], width, MONO_COLUMNS[column])
                                    })
                                    .collect::<Vec<_>>();
                                let extent = cells
                                    .iter()
                                    .fold(egui::Rect::NOTHING, |extent, cell| extent.union(*cell))
                                    .expand2(egui::vec2(COLUMN_GAP * 0.5, 0.0));
                                let response = ui
                                    .interact(
                                        extent,
                                        ui.id().with(("revision-history-row", &row.key)),
                                        egui::Sense::click(),
                                    )
                                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                                response.widget_info(|| {
                                    egui::WidgetInfo::selected(
                                        egui::WidgetType::SelectableLabel,
                                        true,
                                        selected,
                                        format!("Revision {}", row.revision),
                                    )
                                });
                                crate::ui::theme::paint_focus_ring(ui, &response, extent);
                                if response.clicked() {
                                    action = HistoryAction::ToggleSelection(
                                        row.key.clone(),
                                        ui.input(|input| {
                                            input.modifiers.ctrl || input.modifiers.command
                                        }),
                                    );
                                }
                                let tone = if selected {
                                    t.color.bg_active
                                } else if response.hovered() {
                                    t.color.bg_hover
                                } else {
                                    egui::Color32::TRANSPARENT
                                };
                                ui.painter()
                                    .set(fill, egui::Shape::rect_filled(extent, 0.0, tone));
                                ui.end_row();
                            }
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

fn project_audit_rows(model: &Model) -> Vec<ProjectAuditRow> {
    let mut rows = Vec::new();
    append_schematic_rows(
        &mut rows,
        &model.schematic.document().validated_revisions,
        true,
    );
    let active_key = model.workspace.content.active_view.key();
    let mut retained_schematics = model
        .workspace
        .content
        .schematic_buffers
        .iter()
        .filter(|entry| entry.0.as_str() != active_key.as_str())
        .collect::<Vec<_>>();
    retained_schematics.sort_by(|(left, _), (right, _)| left.cmp(right));
    for (_, schematic) in retained_schematics {
        append_schematic_rows(&mut rows, &schematic.document().validated_revisions, false);
    }
    append_simulation_plan_rows(&mut rows, model);
    append_model_and_pdk_rows(&mut rows, model);
    append_governed_rows(&mut rows, model);
    rows
}

fn find_project_schematic_revision(
    model: &Model,
    id: ValidatedSchematicRevisionId,
) -> Option<&ValidatedSchematicRevision> {
    model
        .schematic
        .document()
        .validated_revisions
        .records()
        .iter()
        .find(|record| record.id() == id)
        .or_else(|| {
            model
                .workspace
                .content
                .schematic_buffers
                .values()
                .flat_map(|schematic| schematic.document().validated_revisions.records())
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

fn append_simulation_plan_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    if let Some(plan) = model.sim_setup.analysis_plan.as_ref() {
        rows.push(simulation_plan_row(
            model.sim_setup.active_plan_name().as_str(),
            plan.id().to_string(),
            plan.revision().get().to_string(),
            plan.instances().len(),
            model.sim_setup.active_plan_lineage(),
            true,
        ));
    }
    for plan in model.sim_setup.inactive_plans().iter().rev() {
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
    lineage: crate::workbench::app_state::SimulationPlanLineage,
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
        affected_evidence: format!(
            "{analysis_count} {} \u{00b7} {lineage_evidence}",
            if analysis_count == 1 {
                "analysis"
            } else {
                "analyses"
            }
        ),
        status: if active { "working" } else { "retained" }.to_owned(),
        schematic_revision: None,
        restorable_schematic_revision: None,
    }
}

fn append_model_and_pdk_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    if let Some(binding) = model.workspace.content.project.technology_binding() {
        let digest = binding
            .source_closure()
            .iter()
            .find(|source| source.path == binding.root_source())
            .map(|source| short_text(&source.digest.to_string()))
            .unwrap_or_else(|| model.workspace.content.project.revision().get().to_string());
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

    for library in model.model_library_manager.libraries_sorted() {
        let (revision, status, authority): (String, &str, String) = match library.source_authority {
            ModelSourceAuthority::ProjectOwned {
                source_id,
                revision,
                digest,
            } => (
                revision.get().to_string(),
                "pinned",
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
                        "pinned",
                        format!("external source \u{00b7} root digest {digest}"),
                    ),
                    None => (
                        "unversioned".to_owned(),
                        "unpinned",
                        "external source has no accepted closure".to_owned(),
                    ),
                }
            }
            ModelSourceAuthority::RetainedImport { source_id, digest } => (
                short_text(&digest.to_string()),
                "pinned",
                format!(
                    "retained import {source_id} \u{00b7} root digest {}",
                    short_text(&digest.to_string())
                ),
            ),
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

fn append_governed_rows(rows: &mut Vec<ProjectAuditRow>, model: &Model) {
    let management = &model.workspace.content.design_management;
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
            status: "committed".to_owned(),
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
                    comparison_heading(ui, "Field");
                    comparison_heading(ui, "First");
                    comparison_heading(ui, "Second");
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
            comparison_heading(ui, "Domain");
            comparison_heading(ui, "Added");
            comparison_heading(ui, "Removed");
            comparison_heading(ui, "Modified");
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

/// The history table's columns, in order.
const COLUMNS: [&str; 6] = [
    "Revision",
    "Time",
    "Actor",
    "Change",
    "Affected records",
    "Status",
];
/// The columns whose identifiers and times are set in mono.
const MONO_COLUMNS: [bool; 6] = [true, true, false, false, false, false];
/// Air between two columns.
const COLUMN_GAP: f32 = 8.0;
/// The height of one row of the table.
const CELL_HEIGHT: f32 = 22.0;

/// One row's text in each column, in [`COLUMNS`] order.
fn row_cells(row: &ProjectAuditRow) -> [&str; 6] {
    [
        &row.revision,
        &row.time,
        &row.actor,
        &row.change,
        &row.affected_evidence,
        &row.status,
    ]
}

/// The columns, by index, that any of `rows` says something in. A column no
/// row fills (times the project never retained) would be a heading over blank
/// cells.
fn shown_columns(rows: &[&ProjectAuditRow]) -> Vec<usize> {
    (0..COLUMNS.len())
        .filter(|&column| rows.iter().any(|row| !row_cells(row)[column].is_empty()))
        .collect()
}

/// The width of each `shown` column across `width`: each column as wide as
/// its widest text, then all of them widened in proportion to fill the table,
/// so nothing is elided while the table fits. A table that cannot fit takes
/// the room from its widest columns only (see [`fit_widths`]), rather than
/// running its last columns off the dialog's edge behind a scroll bar.
fn column_widths(ui: &Ui, rows: &[&ProjectAuditRow], shown: &[usize], width: f32) -> Vec<f32> {
    let measure = |text: RichText| {
        egui::WidgetText::from(text)
            .into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::TextStyle::Body,
            )
            .size()
            .x
    };
    let natural = shown
        .iter()
        .map(|&column| {
            rows.iter()
                .map(|row| measure(cell_text(row_cells(row)[column], MONO_COLUMNS[column])))
                .fold(measure(heading_text(ui, COLUMNS[column])), f32::max)
                .ceil()
        })
        .collect::<Vec<_>>();
    let room = (width - COLUMN_GAP * shown.len().saturating_sub(1) as f32).max(1.0);
    fit_widths(&natural, room)
}

/// Natural column widths fitted to `room`. Room to spare widens every column
/// in proportion. Too little is taken from the widest columns alone: they
/// are cut to one common width, as wide as still fits, and every column
/// narrower than that keeps its whole text. Shrinking every column in
/// proportion elided an eight-letter revision ("built-i...") beside a column
/// of prose that could have given the room up.
fn fit_widths(natural: &[f32], room: f32) -> Vec<f32> {
    let total = natural.iter().sum::<f32>().max(1.0);
    if total <= room {
        let scale = room / total;
        return natural
            .iter()
            .map(|width| (width * scale).floor())
            .collect();
    }
    let mut sorted = natural.to_vec();
    sorted.sort_by(f32::total_cmp);
    let mut below = 0.0;
    let mut cap = 0.0;
    for (index, width) in sorted.iter().enumerate() {
        let wider = (sorted.len() - index) as f32;
        // Every column from this one up cut to the same width.
        let candidate = (room - below) / wider;
        if candidate <= *width {
            cap = candidate;
            break;
        }
        below += width;
    }
    natural.iter().map(|width| width.min(cap).floor()).collect()
}

/// A column's heading: sentence case, in the label face, as written.
fn table_heading(ui: &mut Ui, label: &str, width: f32) {
    let text = heading_text(ui, label);
    let _ = cell(ui, width, text);
}

fn heading_text(ui: &Ui, label: &str) -> RichText {
    let t = Tokens::get(ui.ctx());
    RichText::new(label)
        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
        .color(t.color.text_dim)
}

/// A cell's text: identifiers and times in mono, the rest in the body face.
fn cell_text(text: &str, mono: bool) -> RichText {
    if mono {
        RichText::new(text).font(theme::mono(tokens::FS_0, FontWeight::Regular))
    } else {
        RichText::new(text)
    }
}

/// A cell of `width` holding `text` from its left edge, elided to fit: the
/// cell's rect, and the text's response.
fn cell(ui: &mut Ui, width: f32, text: RichText) -> (egui::Rect, egui::Response) {
    let placed = ui.allocate_ui_with_layout(
        egui::vec2(width, CELL_HEIGHT),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_width(width);
            ui.add(egui::Label::new(text).truncate())
        },
    );
    (placed.response.rect, placed.inner)
}

/// One cell: its whole text when it fits, elided with the whole text on
/// hover when it does not. Identifiers and times are set in mono. Returns
/// the cell's rect.
fn table_cell(ui: &mut Ui, text: &str, width: f32, mono: bool) -> egui::Rect {
    let (rect, response) = cell(ui, width, cell_text(text, mono));
    if !text.is_empty() {
        response.on_hover_text(text);
    }
    rect
}

/// A comparison table's column heading, in the history table's face.
fn comparison_heading(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
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
    if timestamp_ms == 0 {
        return "time unavailable".to_owned();
    }
    let Ok(now) = crate::time_compat::checked_unix_time_ms() else {
        return "time unavailable".to_owned();
    };
    if timestamp_ms > now {
        return "clock skew".to_owned();
    }
    crate::time_compat::relative_age(std::time::Duration::from_millis(now - timestamp_ms))
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
    use crate::product::{ContentDigest, ProjectId};
    use crate::state::{
        ComponentType, Point, PreflightReport, ValidatedRevisionRequest, ValidationFindingCounts,
    };
    use crate::workbench::RSpiceApp;
    use crate::workbench::project::tests::with_cx;

    fn append(state: &mut crate::state::SchematicState, note: &str) {
        let digest = state
            .editor_ref()
            .design
            .validated_design_content_digest()
            .unwrap();
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
            .document()
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
        let first = state.document().validated_revisions.records()[0].clone();
        state.add_component(ComponentType::Resistor, Point::new(10, 20));
        append(&mut state, "Add resistor");
        let second = state.document().validated_revisions.records()[1].clone();
        let delta = first.semantic_delta_to(&second);
        assert_eq!(delta.components.added, 1);

        state
            .restore_validated_revision(first.id())
            .expect("validated restore");
        assert!(state.document().components.is_empty());
        assert!(state.undo());
        assert_eq!(state.document().components.len(), 1);
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

    /// A table too wide for its room cuts its widest columns to one width and
    /// leaves every narrower column whole; one with room to spare widens
    /// every column in proportion.
    #[test]
    fn a_table_that_cannot_fit_cuts_only_its_widest_columns() {
        let fitted = fit_widths(&[60.0, 120.0, 190.0, 260.0, 60.0], 600.0);
        assert_eq!(fitted, vec![60.0, 120.0, 180.0, 180.0, 60.0]);
        let roomy = fit_widths(&[50.0, 150.0], 400.0);
        assert_eq!(roomy, vec![100.0, 300.0]);
    }

    /// The table fits the width it is given, every column of it, with its
    /// headings as written and nothing below it but the restore: it used to
    /// run its last columns off the dialog behind a horizontal scroll bar,
    /// head them in upper-case mono, fill unkept facts with `not retained`
    /// and close on two cards explaining the engine.
    #[test]
    fn the_history_table_fits_its_width_and_says_nothing_it_does_not_know() {
        const WIDTH: f32 = 736.0;
        let long = "16 models \u{b7} 0 corners \u{b7} 0 pinned sources \u{b7} more evidence";
        let rows = vec![ProjectAuditRow {
            key: "model-library:one".to_owned(),
            kind: ProjectAuditArtifactKind::ModelOrPdk,
            revision: "built-in".to_owned(),
            time: NOT_RETAINED.to_owned(),
            actor: "signed application catalog".to_owned(),
            change: "Model library \u{b7} RSpice Foundation".to_owned(),
            affected_evidence: long.to_owned(),
            status: "built-in".to_owned(),
            schematic_revision: None,
            restorable_schematic_revision: None,
        }];
        let mut dialog = ProjectRevisionHistoryDialogState {
            open: true,
            ..ProjectRevisionHistoryDialogState::default()
        };
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(WIDTH, 600.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::NONE)
                            .show(ctx, |ui| {
                                let _ = revision_history_body(ui, &rows, true, &mut dialog);
                            });
                    },
                )
                .shapes;
        }
        let texts = shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    egui::Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => None,
            })
            .collect::<Vec<_>>();
        for heading in COLUMNS {
            // No row retained a time, so no column stands over blank cells.
            let drawn = texts.iter().any(|(text, _)| text == heading);
            assert_eq!(drawn, heading != "Time", "{heading:?}: {texts:?}");
        }
        // Every cell is stated whole: the columns are as wide as their text.
        for whole in row_cells(&rows[0])
            .into_iter()
            .filter(|text| !text.is_empty())
        {
            assert!(
                texts.iter().any(|(text, _)| text == whole),
                "{whole:?} is elided: {texts:?}"
            );
        }
        for (text, rect) in &texts {
            assert!(
                rect.right() <= WIDTH + 0.5,
                "{text:?} runs past the table's edge to {}",
                rect.right()
            );
            for gone in [
                "REVISION",
                "not retained",
                "Semantic comparison",
                "Results never change",
            ] {
                assert!(!text.contains(gone), "{text:?} is still stated");
            }
        }
    }

    /// A restore in the project feature's context does what the application
    /// did in place: the design is the revision's, the dialog is re-armed on
    /// the journal it now shows, and what was decided, prepared or generated
    /// for the replaced design is dropped. A restore refused, here because
    /// the journal moved since the dialog read it, leaves all of it alone
    /// and is stated in the dialog.
    #[test]
    fn a_revision_restored_in_the_project_context_ends_where_the_restore_in_place_did() {
        for journal_moved in [false, true] {
            let mut app = RSpiceApp::test_instance();
            let schematic = &mut app.state.model.schematic;
            let baseline_components = schematic.document().components.len();
            append(schematic, "Baseline");
            let baseline = schematic.document().validated_revisions.records()[0].id();
            let read_at_open = schematic.document().validated_revisions.clone();
            schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
            append(schematic, "Add resistor");
            app.state.project.project_revision_history = ProjectRevisionHistoryDialogState {
                open: true,
                selected: vec![schematic_row_key(baseline)],
                authority: Some(SchematicEditAuthority::capture(&app.state.model)),
                expected_journal: if journal_moved {
                    read_at_open
                } else {
                    app.state
                        .model
                        .schematic
                        .document()
                        .validated_revisions
                        .clone()
                },
                restore_confirmation: true,
                ..ProjectRevisionHistoryDialogState::default()
            };
            let digest = ContentDigest::from_bytes([7; 32]);
            app.state
                .model
                .netlist_artifacts
                .current_generation_input_digest = Some(digest);
            app.state.runs.preflight.report = Some(PreflightReport {
                project_revision: 1,
                topology_root: "user/top/schematic".to_owned(),
                topology_revision: 1,
                topology_closure: Vec::new(),
                simulation_plan_id: None,
                simulation_plan_revision: None,
                blockers: Vec::new(),
                advisories: Vec::new(),
                prepared: None,
            });

            let rows = project_audit_rows(&app.state.model);
            with_cx(&mut app, |cx| restore_selected_revision(cx, &rows));

            let dialog = &app.state.project.project_revision_history;
            let model = &app.state.model;
            assert!(
                !dialog.restore_confirmation,
                "journal moved {journal_moved}"
            );
            if journal_moved {
                assert_eq!(
                    dialog.error.as_deref(),
                    Some(
                        "The validated revision journal changed. Close and reopen revision history."
                    )
                );
                assert_eq!(
                    model.schematic.document().components.len(),
                    baseline_components + 1
                );
                assert_eq!(
                    model.netlist_artifacts.current_generation_input_digest,
                    Some(digest)
                );
                assert!(app.state.runs.preflight.report.is_some());
            } else {
                assert_eq!(dialog.error, None);
                assert_eq!(
                    model.schematic.document().components.len(),
                    baseline_components
                );
                assert_eq!(
                    dialog.expected_journal,
                    model.schematic.document().validated_revisions
                );
                assert_eq!(history_authority_error(dialog, model), None);
                assert_eq!(
                    model.netlist_artifacts.current_generation_input_digest,
                    None
                );
                assert!(app.state.runs.preflight.report.is_none());
            }
        }
    }
}
