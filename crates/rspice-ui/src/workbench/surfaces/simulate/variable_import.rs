//! Importing design variables from a spec-sheet CSV.
//!
//! Design variables arrive from a spec sheet, and typing forty of them one
//! dialog at a time is not an authoring workflow. The import is nonetheless
//! held to the same contract as a hand-typed variable: every row becomes a
//! [`DesignVariableDraft`] and clears `design_variable_from_draft_categorized`,
//! so there is one definition of a valid variable rather than a second, laxer
//! one reached through a file.
//!
//! **The import is guided rather than blind.** It used to be a single act —
//! pick a file, and either the whole sheet landed or one prose sentence came
//! back about whichever row failed first. A sheet with three different problems
//! took three rounds to learn about, and a column the import did not recognize
//! was fatal because there was nowhere to say it had not been read.
//!
//! So the file is read once and then *shown*: which of the sheet's columns
//! feeds which field, what each row would become, which rows cannot be adopted
//! and why, and what scope the whole import is owned at. Every refusal carries
//! a [`VariableImportRefusal`] identity — `VARIMP-SCHEMA`, `VARIMP-IDENTIFIER`,
//! `VARIMP-DIMENSION`, `VARIMP-BOUNDS`, `VARIMP-COLLISION` — so a refusal can
//! be counted, searched for and asserted, which a sentence cannot.
//!
//! **The selection is the transaction.** Adoption is still all-or-none: the
//! ticked rows land together or none of them does. What changed is which rows
//! are in it, not whether it is atomic. A ticked row that cannot be adopted
//! stops the import rather than being dropped from it, because the one thing
//! this must never do is land something other than what was on screen.

use csv::{ReaderBuilder, StringRecord, Trim};

use egui::Ui;

use crate::io::file_exchange::{self, FileKind};
use crate::product::SimulationPlanId;
use crate::state::{
    DesignVariableDefect, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableSweepEligibility,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogSize, select};
use crate::workbench::state::{
    DesignVariableDraft, DesignVariableImportDraft, DesignVariableImportRow,
    SimulationWorkflowDialog, VariableImportRefusal,
};
use crate::workbench::{AppState, RSpiceApp};

use super::workflows::{
    commit_plan_change, design_variable_from_draft_categorized, finish_workflow_choice,
    workflow_section_heading, workflow_setting_row, workflow_split_over,
    workflow_validation_message,
};

/// A spec sheet is a table of typed parameters, not a data file. The ceiling
/// bounds a mis-picked waveform dump before any of it is decoded.
const MAX_IMPORT_BYTES: usize = 1 << 20;

/// Row ceiling. A registry larger than this is generated, not authored, and
/// belongs in a project the plan includes rather than in one paste.
const MAX_IMPORT_ROWS: usize = 512;

/// What the picker offers, and how a refused import names the file. The
/// subject is lower case because every refusal here is reported inside the
/// `Design-variable import refused · …` sentence.
const SPEC_SHEET: FileKind = FileKind {
    label: "Design variable spec sheet",
    extensions: &["csv"],
    subject: "the spec sheet",
    fallback_name: "the spec sheet",
};

/// The frame-context slot this surface's picker posts to.
fn exchange_id() -> egui::Id {
    egui::Id::new("simulation.variables.import")
}

/// The columns a spec sheet may name.
///
/// A header row binds each column to a field, so the order they appear in the
/// file is not load-bearing. Only the first three are required: the rest fall
/// back to the values the create dialog opens with, which is what makes a
/// three-column sheet a complete import rather than a partial one.
const COLUMNS: [&str; 9] = [
    "name",
    "quantity",
    "expression",
    "minimum",
    "maximum",
    "scope",
    "sweep role",
    "override policy",
    "description",
];

const NAME: usize = 0;
const QUANTITY: usize = 1;
const EXPRESSION: usize = 2;
const MINIMUM: usize = 3;
const MAXIMUM: usize = 4;
const SCOPE: usize = 5;
const SWEEP_ROLE: usize = 6;
const OVERRIDE_POLICY: usize = 7;
const DESCRIPTION: usize = 8;

const REQUIRED: [usize; 3] = [NAME, QUANTITY, EXPRESSION];

/// The ownership scopes, spelled as `design_variable_from_draft` indexes them.
/// `cell` and `analysis` resolve against the active view and the selected
/// analysis, so an analysis-scoped row is refused when nothing is selected —
/// the same refusal the create dialog gives.
///
/// Unlike the other enumerated columns these cannot be derived from an `ALL`
/// array: two of the four variants carry payloads, so there is no value to call
/// `label()` on without inventing a cell reference. The spellings are therefore
/// copied, and `scope_spellings_match_the_scopes_they_name` pins the copy to the
/// original — a renamed scope must fail that test rather than silently stop
/// matching the column an engineer wrote.
const SCOPE_CHOICES: [(&str, usize); 6] = [
    // Matching is case-insensitive, so the abbreviation a sheet may write for
    // the testbench scope is now the label itself.
    ("Testbench", 0),
    ("Project", 1),
    ("Selected cell", 2),
    ("cell", 2),
    ("Selected analysis only", 3),
    ("analysis", 3),
];

/// Start an import by asking for the sheet.
///
/// The click only starts the pick — a browser cannot hand back a file
/// synchronously — so the sheet itself arrives at [`poll_pending_import`],
/// which opens the guided dialog on it rather than adopting it outright.
pub(super) fn import_from_file(ctx: &egui::Context, state: &mut AppState) {
    if let Err(error) = file_exchange::open_file(ctx, exchange_id(), SPEC_SHEET, MAX_IMPORT_BYTES) {
        refuse(
            state,
            &VariableImportRefusal::Schema {
                line: 0,
                detail: error,
            },
        );
    }
}

/// Open the guided import on a spec sheet the picker has finished reading.
pub(super) fn poll_pending_import(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
) {
    match file_exchange::take_opened(ctx, exchange_id()) {
        Some(Ok(Some(sheet))) => {
            let draft = import_draft_for_sheet(app, plan_id, &sheet.name, &sheet.text);
            app.state.workbench.simulation_workflow =
                Some(SimulationWorkflowDialog::DesignVariableImport(draft));
        }
        // The picker failed before any of the sheet was read, so there is no
        // line to name and nothing about the table to describe.
        Some(Err(error)) => refuse(
            &mut app.state,
            &VariableImportRefusal::Schema {
                line: 0,
                detail: error,
            },
        ),
        // A cancelled pick is a choice, and there is nothing to report.
        Some(Ok(None)) | None => {}
    }
}

/// Report a refused import where a refused edit is already reported: the plan's
/// lifecycle status, which is the channel `commit_plan_change` itself writes on
/// failure.
///
/// This writes one field, so it takes that field's owner rather than the whole
/// application — the same reason `lifecycle::record_failure` does. Announcing an
/// outcome is not a licence to mutate every subsystem.
fn refuse(state: &mut AppState, refusal: &VariableImportRefusal) {
    state
        .workbench
        .analysis_lifecycle_status
        .record_refusal(format!(
            "Design-variable import refused \u{00b7} {}",
            refusal.message()
        ));
}

/// Which import refusal a rejected row amounts to.
///
/// The row is put through the create dialog's contract, and that contract says
/// which of its rules the row broke. This only restates that answer in the
/// import's own vocabulary, so there is still exactly one definition of a valid
/// design variable and no second place where a name or a dimension is judged.
///
/// [`DesignVariableDefect::Record`] becomes a schema refusal because from a
/// sheet those fields — the description, the enumerated selections — arrive
/// from columns. The row is not wrong about a variable; the sheet is wrong
/// about the row.
fn refusal_for_defect(
    defect: DesignVariableDefect,
    line: u64,
    subject: &str,
    detail: String,
) -> VariableImportRefusal {
    let subject = subject.to_owned();
    match defect {
        DesignVariableDefect::Identifier => VariableImportRefusal::Identifier {
            line,
            subject,
            detail,
        },
        DesignVariableDefect::Dimension => VariableImportRefusal::Dimension {
            line,
            subject,
            detail,
        },
        DesignVariableDefect::Bounds => VariableImportRefusal::Bounds {
            line,
            subject,
            detail,
        },
        DesignVariableDefect::Record => VariableImportRefusal::Schema {
            line,
            detail: format!("{subject}: {detail}"),
        },
    }
}

/// A refusal about the table itself rather than about any variable in it.
fn schema(line: u64, detail: String) -> VariableImportRefusal {
    VariableImportRefusal::Schema { line, detail }
}

// The guided import
//=============================================================================

/// The scopes the adopt-at control offers, in `DesignVariableDraft::scope`
/// order. These are the same four [`SCOPE_CHOICES`] indexes, spelled as the
/// studio shows them rather than as a sheet may abbreviate them.
const SCOPE_LABELS: [&str; 4] = [
    "Testbench",
    "Project",
    "Selected cell",
    "Selected analysis only",
];

/// Read a sheet into its header cells and its data rows, without judging any of
/// them.
///
/// The split matters: the dialog binds columns *after* the file is read, so
/// reading cannot depend on the binding. Only the table's own shape is refused
/// here — bad CSV, or more than one import may carry.
/// One numbered data row: the sheet line it came from, and its cells.
///
/// Named because the line number travels with the row all the way to a
/// refusal — a defect reported against "row 4" of a file whose header is on
/// line 2 sends the reader to the wrong place.
type SheetRow = (u64, Vec<String>);

fn read_sheet(source: &str) -> Result<(Vec<String>, Vec<SheetRow>), VariableImportRefusal> {
    if source.len() > MAX_IMPORT_BYTES {
        return Err(schema(
            0,
            format!("the spec sheet exceeds the {MAX_IMPORT_BYTES} byte import limit"),
        ));
    }
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(false)
        .from_reader(source.as_bytes());
    let headers = reader
        .headers()
        .map_err(malformed_csv)?
        .iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let mut cells = Vec::new();
    for record in reader.records() {
        if cells.len() >= MAX_IMPORT_ROWS {
            return Err(schema(
                0,
                format!("the spec sheet exceeds the {MAX_IMPORT_ROWS} row import limit"),
            ));
        }
        let record = record.map_err(malformed_csv)?;
        let line = record.position().map_or(0, csv::Position::line);
        cells.push((line, record.iter().map(str::to_owned).collect()));
    }
    if cells.is_empty() {
        return Err(schema(
            1,
            "the spec sheet names its columns but holds no variables".to_owned(),
        ));
    }
    Ok((headers, cells))
}

/// Bind each declared column to the sheet field whose header names it.
///
/// A header the import does not recognize binds nothing rather than refusing
/// the sheet. That used to be a refusal because there was nowhere to say it:
/// silently dropping a column an engineer wrote is how a sheet spelling its
/// bound column `min` imports unbounded variables. The dialog now shows every
/// binding, and states which of the sheet's own columns nothing is reading, so
/// an unrecognized header is visible and correctable instead of fatal.
///
/// A column the sheet names twice is still refused. Both spellings are
/// legitimate, nothing can choose between them, and a first-wins rule would
/// pick one silently.
fn auto_binding(headers: &[String]) -> Result<Vec<Option<usize>>, VariableImportRefusal> {
    let mut binding = vec![None; COLUMNS.len()];
    for (field, header) in headers.iter().enumerate() {
        let Some(column) = COLUMNS
            .iter()
            .position(|column| same_column(column, header))
        else {
            continue;
        };
        if binding[column].replace(field).is_some() {
            return Err(schema(
                1,
                format!("column '{}' is named twice", COLUMNS[column]),
            ));
        }
    }
    Ok(binding)
}

/// The draft the guided import opens on, for a sheet the picker has read.
///
/// Reading a sheet changes nothing, so this takes the application immutably and
/// hands back what the dialog should show. The caller decides that the dialog
/// opens; this only decides what is in it.
pub(super) fn import_draft_for_sheet(
    app: &RSpiceApp,
    plan_id: SimulationPlanId,
    file_name: &str,
    source: &str,
) -> DesignVariableImportDraft {
    match read_sheet(source).and_then(|(headers, cells)| {
        let binding = auto_binding(&headers)?;
        Ok(DesignVariableImportDraft::new(
            file_name, headers, cells, binding,
        ))
    }) {
        Ok(mut draft) => {
            resolve_rows(app, plan_id, &mut draft);
            draft
        }
        // A sheet that cannot be read at all still opens the dialog, because
        // the dialog is where the reason is shown. It opens with nothing to
        // adopt and the refusal on screen.
        Err(refusal) => DesignVariableImportDraft::refused(file_name, refusal),
    }
}

/// Resolve every row against the mapping and scope the dialog is showing.
///
/// Called when a choice changes, never per frame: five hundred rows is several
/// hundred quantity parses, and a dialog resolving them at frame rate would do
/// so for as long as it stayed open.
///
/// Ticks are preserved across a re-resolve where the row is still adoptable, so
/// correcting one column's binding does not discard a selection made over the
/// rest of the sheet.
fn resolve_rows(app: &RSpiceApp, plan_id: SimulationPlanId, draft: &mut DesignVariableImportDraft) {
    let previously_declined = draft
        .rows
        .iter()
        .filter(|row| !row.accepted)
        .map(|row| row.line)
        .collect::<Vec<_>>();

    let missing = REQUIRED
        .iter()
        .filter(|column| draft.binding.get(**column).copied().flatten().is_none())
        .map(|column| COLUMNS[*column])
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        draft.rows.clear();
        draft.sheet_refusal = Some(schema(
            1,
            format!(
                "no column is bound to {}; every row needs {}",
                missing.join(", "),
                if missing.len() == 1 { "it" } else { "them" }
            ),
        ));
        return;
    }
    draft.sheet_refusal = None;

    let mut binding = [None; COLUMNS.len()];
    for (column, field) in draft.binding.iter().enumerate().take(COLUMNS.len()) {
        binding[column] = *field;
    }

    let mut rows = Vec::with_capacity(draft.cells.len());
    for (line, cells) in &draft.cells {
        let record = StringRecord::from(cells.clone());
        let resolved = draft_from_row(&record, &binding).map(|mut row_draft| {
            if draft.override_scope {
                row_draft.scope = draft.scope;
            }
            row_draft
        });
        let (row_draft, refusal) = match resolved {
            Ok(row_draft) => {
                let refusal = design_variable_from_draft_categorized(app, &row_draft)
                    .err()
                    .map(|(defect, detail)| {
                        refusal_for_defect(defect, *line, &row_draft.name, detail)
                    });
                (row_draft, refusal)
            }
            Err(refusal) => (DesignVariableDraft::default(), Some(refusal.at_line(*line))),
        };
        let name = refusal
            .as_ref()
            .and_then(VariableImportRefusal::subject)
            .unwrap_or(row_draft.name.as_str())
            .to_owned();
        let adoptable = refusal.is_none();
        rows.push(DesignVariableImportRow {
            line: *line,
            name,
            draft: row_draft,
            refusal,
            accepted: adoptable && !previously_declined.contains(line),
        });
    }
    draft.rows = rows;
    mark_collisions(app, plan_id, draft);
}

/// Mark the ticked rows that cannot be added because a name is already taken.
///
/// Collisions are a property of the *selection*, not of the sheet: unticking
/// one of two rows sharing a name resolves the other. So this is recomputed
/// whenever a tick changes, and only ticked rows are considered.
fn mark_collisions(
    app: &RSpiceApp,
    plan_id: SimulationPlanId,
    draft: &mut DesignVariableImportDraft,
) {
    let owned = app
        .state
        .workspace
        .plan_data(plan_id)
        .map(|payload| {
            payload
                .design_variables
                .iter()
                .map(|variable| variable.name.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    for row in &mut draft.rows {
        if matches!(row.refusal, Some(VariableImportRefusal::Collision { .. })) {
            row.refusal = None;
        }
    }

    let mut taken: Vec<String> = Vec::new();
    for row in &mut draft.rows {
        if row.refusal.is_some() || !row.accepted {
            continue;
        }
        let name = row.name.clone();
        if let Some(existing) = owned.iter().find(|owned| owned.eq_ignore_ascii_case(&name)) {
            row.refusal = Some(VariableImportRefusal::Collision {
                line: row.line,
                subject: name,
                detail: format!("this plan already owns a design variable named '{existing}'"),
            });
            continue;
        }
        if let Some(earlier) = taken
            .iter()
            .find(|earlier| earlier.eq_ignore_ascii_case(&name))
        {
            row.refusal = Some(VariableImportRefusal::Collision {
                line: row.line,
                subject: name,
                detail: format!("an earlier ticked row already names '{earlier}'"),
            });
            continue;
        }
        taken.push(name);
    }

    // A refused row cannot be adopted, and the table disables its tick, so
    // leaving it ticked would show a selection the control cannot express and
    // the commit will not honour.
    for row in &mut draft.rows {
        if row.refusal.is_some() {
            row.accepted = false;
        }
    }
}

/// Why the import cannot be applied as it currently stands.
///
/// A ticked row that carries a refusal disables the commit rather than being
/// quietly left out. Dropping it would break the one promise this import makes:
/// what lands is exactly what was on screen, or nothing does.
fn import_refusal(draft: &DesignVariableImportDraft) -> Option<String> {
    if let Some(refusal) = &draft.sheet_refusal {
        return Some(refusal.message());
    }
    if let Some(row) = draft
        .rows
        .iter()
        .find(|row| row.accepted && !row.is_adoptable())
    {
        return row.refusal.as_ref().map(VariableImportRefusal::message);
    }
    if draft.accepted_count() == 0 {
        return Some(
            "No rows are selected. Tick at least one row, or cancel the import.".to_owned(),
        );
    }
    None
}

/// The guided design-variable import.
pub(super) fn design_variable_import_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: DesignVariableImportDraft,
) {
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        app.state.workbench.simulation_workflow = None;
        return;
    };
    let plan_id = plan.id();

    let mut changed = false;
    draft.validation_error = import_refusal(&draft);
    let enabled = draft.validation_error.is_none();
    let adoptable = draft.rows.iter().filter(|row| row.is_adoptable()).count();
    let total = draft.rows.len();
    let accepted = draft.accepted_count();

    let choice = Dialog::new(
        "SIMULATION \u{00b7} DESIGN VARIABLES",
        "Import spec sheet",
        "Adopt selected rows",
    )
    .description(
        "Bind the sheet's columns to the fields they supply, review what each row would become, \
         and adopt the rows you keep. Every selected row is added as one transaction: if any of \
         them is refused, none of them lands.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .primary_on_enter(false)
    .show(ctx, |ui| {
        // The refusal first. It is the one thing that decides whether the
        // primary control does anything, and under a tall left column it sat
        // below the fold of the body it governs.
        workflow_validation_message(ui, draft.validation_error.as_deref());
        workflow_split_over(
            ui,
            &mut draft,
            |ui, draft| {
                workflow_setting_row(ui, "Source", "The sheet this import is reading.", |ui| {
                    ui.label(
                        egui::RichText::new(&draft.source_name)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                    );
                });
                workflow_section_heading(ui, "Column mapping");
                if draft.headers.is_empty() {
                    import_note(ui, "This sheet has no header row to bind.");
                } else {
                    changed |= mapping_rows(ui, draft);
                    import_note(
                        ui,
                        &match unread_columns(draft) {
                            Some(unread) => format!(
                                "A column marked required must be supplied by every row; an \
                                 unbound optional column takes the create dialog's default. \
                                 Nothing is reading {unread} — those columns are in the sheet \
                                 and are not being imported.",
                            ),
                            None => "A column marked required must be supplied by every row; an \
                                     unbound optional column takes the create dialog's default."
                                .to_owned(),
                        },
                    );
                }
                workflow_section_heading(ui, "Adopt at");
                changed |= scope_rows(ui, draft);
            },
            |ui, draft| {
                workflow_section_heading(
                    ui,
                    &format!("Rows \u{00b7} {adoptable} of {total} adoptable"),
                );
                if draft.rows.is_empty() {
                    import_note(
                        ui,
                        "Nothing resolves from this sheet yet. The refusal above says why.",
                    );
                } else if row_table(ui, draft) {
                    mark_collisions(app, plan_id, draft);
                }
                import_note(
                    ui,
                    &format!(
                        "{accepted} of {total} row{} selected. Adopting advances the plan \
                         revision once and leaves retained datasets unchanged.",
                        if total == 1 { "" } else { "s" }
                    ),
                );
            },
        );
    });

    if changed {
        resolve_rows(app, plan_id, &mut draft);
    }
    finish_workflow_choice(ctx, app, choice, draft, commit_selected_rows);
}

/// One line per declared column, naming which of the sheet's own columns
/// feeds it.
///
/// One line rather than the three a setting row takes. The note this drops was
/// the same pair of sentences nine times over -- "Required. Every row must
/// supply this." against "Optional. Unbound rows take the create dialog's
/// default." -- which is one rule about the block, and belongs under it once.
/// Nine two-line rows are also what pushed the row-validation table, which is
/// the point of the import, below the fold of the body.
fn mapping_rows(ui: &mut Ui, draft: &mut DesignVariableImportDraft) -> bool {
    const UNBOUND: &str = "\u{2014} not in this sheet";
    let mut options = vec![UNBOUND.to_owned()];
    options.extend(draft.headers.iter().cloned());
    let mut changed = false;

    for (column, name) in COLUMNS.iter().enumerate() {
        let bound = draft.binding.get(column).copied().flatten();
        let current = bound
            .and_then(|field| draft.headers.get(field))
            .map_or(UNBOUND, String::as_str)
            .to_owned();
        let required = REQUIRED.contains(&column);
        let label = if required {
            format!("{name} \u{00b7} required")
        } else {
            (*name).to_owned()
        };
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let row_width = ui.available_width().max(1.0);
            super::paint_control_row_label(ui, &label, row_width);
            let select_width = (ui.available_width()
                - crate::workbench::design_system::PROPERTY_ROW_TRAILING_PAD)
                .max(1.0);
            if let Some(picked) = select(
                ui,
                &format!("variable-import-column-{column}"),
                name,
                &current,
                &options,
                select_width,
            ) {
                let field = picked.checked_sub(1);
                if draft.binding.get(column).copied().flatten() != field {
                    // A field may feed only one declared column: two columns
                    // reading one cell is never what was meant, and the second
                    // choice is the one just made.
                    if let Some(field) = field {
                        for held in &mut draft.binding {
                            if *held == Some(field) {
                                *held = None;
                            }
                        }
                    }
                    if let Some(slot) = draft.binding.get_mut(column) {
                        *slot = field;
                    }
                    changed = true;
                }
            }
        });
    }
    changed
}

/// The sheet's own columns that no declared field is reading.
fn unread_columns(draft: &DesignVariableImportDraft) -> Option<String> {
    let unread = draft
        .headers
        .iter()
        .enumerate()
        .filter(|(field, _)| !draft.binding.contains(&Some(*field)))
        .map(|(_, header)| format!("'{header}'"))
        .collect::<Vec<_>>();
    (!unread.is_empty()).then(|| unread.join(", "))
}

/// The ownership scope every imported variable is adopted at.
fn scope_rows(ui: &mut Ui, draft: &mut DesignVariableImportDraft) -> bool {
    let mut changed = false;
    let sheet_declares = draft.binding.get(SCOPE).copied().flatten().is_some();
    workflow_setting_row(
        ui,
        "Ownership",
        if sheet_declares {
            "This sheet declares a scope per row. Override it to adopt the whole sheet at one scope."
        } else {
            "This sheet declares no scope, so every row is adopted at the scope chosen here."
        },
        |ui| {
            let mut override_scope = draft.override_scope || !sheet_declares;
            if ui
                .add_enabled(
                    sheet_declares,
                    egui::Checkbox::new(&mut override_scope, "Override the sheet"),
                )
                .changed()
            {
                draft.override_scope = override_scope;
                changed = true;
            }
        },
    );
    let applies = draft.override_scope || !sheet_declares;
    workflow_setting_row(
        ui,
        "Scope",
        "Where the imported variables are owned.",
        |ui| {
            let options = SCOPE_LABELS.map(str::to_owned).to_vec();
            let current = SCOPE_LABELS
                .get(draft.scope)
                .copied()
                .unwrap_or(SCOPE_LABELS[0])
                .to_owned();
            ui.add_enabled_ui(applies, |ui| {
                if let Some(picked) = select(
                    ui,
                    "variable-import-scope",
                    "Scope",
                    &current,
                    &options,
                    ui.available_width().min(260.0),
                ) && picked != draft.scope
                {
                    draft.scope = picked;
                    draft.override_scope = true;
                    changed = true;
                }
            });
        },
    );
    if !draft.override_scope && !sheet_declares && draft.scope != 0 {
        // Nothing in the sheet to override, so the choice simply applies.
        draft.override_scope = true;
        changed = true;
    }
    changed
}

/// One line per sheet row: what it would become, or why it cannot be adopted.
fn row_table(ui: &mut Ui, draft: &mut DesignVariableImportDraft) -> bool {
    let mut ticks_changed = false;
    let tokens = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical()
        .max_height(220.0)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            for row in &mut draft.rows {
                ui.horizontal(|ui| {
                    let adoptable = row.is_adoptable();
                    let mut accepted = row.accepted;
                    if ui
                        .add_enabled(adoptable, egui::Checkbox::without_text(&mut accepted))
                        .changed()
                    {
                        row.accepted = accepted;
                        ticks_changed = true;
                    }
                    ui.label(
                        egui::RichText::new(format!("{:>4}", row.line))
                            .font(theme::mono(tokens::FS_MICRO, FontWeight::Regular))
                            .color(tokens.color.text_dim),
                    );
                    ui.label(
                        egui::RichText::new(&row.name)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium)),
                    );
                    // The last cell takes the room the row has left and no
                    // more. A refusal detail is a sentence, and an egui row
                    // extends its items rather than wrapping them: laid out at
                    // its natural width one sentence carried the row past the
                    // pane, and the pane's own width out with it, so the note
                    // beneath the table wrapped at a width the pane never had
                    // and the surface's border was drawn two hundred points
                    // outside the header and footer it belongs to.
                    match &row.refusal {
                        Some(refusal) => {
                            ui.label(
                                egui::RichText::new(refusal.id())
                                    .font(theme::mono(tokens::FS_MICRO, FontWeight::Medium))
                                    .color(tokens.color.err),
                            );
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(refusal.detail())
                                        .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                                        .color(tokens.color.text_dim),
                                )
                                .truncate(),
                            )
                            .on_hover_text(refusal.detail());
                        }
                        None => {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(&row.draft.expression)
                                        .font(theme::mono(tokens::FS_MICRO, FontWeight::Regular))
                                        .color(tokens.color.text_dim),
                                )
                                .truncate(),
                            )
                            .on_hover_text(&row.draft.expression);
                        }
                    }
                });
            }
        });
    ticks_changed
}

fn import_note(ui: &mut Ui, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
            .color(tokens.color.text_dim),
    );
}

/// Adopt the ticked rows as one plan transaction.
fn commit_selected_rows(
    app: &mut RSpiceApp,
    draft: &DesignVariableImportDraft,
) -> Result<String, String> {
    // The same question the primary control is disabled on, asked again here.
    // The control is a courtesy; this is the guarantee, and it is what stops a
    // ticked row that cannot land from being quietly left out of the batch.
    if let Some(refusal) = import_refusal(draft) {
        return Err(refusal);
    }
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map_err(|error| error.to_string())?
        .id();

    let mut variables = Vec::new();
    for row in draft.accepted_rows() {
        let variable = design_variable_from_draft_categorized(app, &row.draft).map_err(
            |(defect, detail)| refusal_for_defect(defect, row.line, &row.name, detail).message(),
        )?;
        variables.push(variable);
    }
    let count = variables.len();
    let source_name = draft.source_name.clone();
    let detail = format!(
        "Imported {count} design variable{} from {source_name}.",
        if count == 1 { "" } else { "s" }
    );
    commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
        workspace
            .add_design_variables(plan_id, variables)
            .map_err(|error| error.to_string())
    });

    let status = &app.state.workbench.analysis_lifecycle_status;
    if status.is_refusal() {
        return Err(status.message().to_owned());
    }
    Ok(detail)
}

fn malformed_csv(error: csv::Error) -> VariableImportRefusal {
    schema(
        error.position().map_or(0, csv::Position::line),
        format!("the spec sheet is not valid CSV \u{00b7} {error}"),
    )
}

/// Header names are matched on their letters and digits alone, so `Sweep role`,
/// `sweep_role` and `SweepRole` all bind the same column.
fn same_column(column: &str, header: &str) -> bool {
    fn key(text: &str) -> String {
        text.chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|character| character.to_ascii_lowercase())
            .collect()
    }
    key(column) == key(header)
}

fn draft_from_row(
    row: &StringRecord,
    columns: &[Option<usize>; COLUMNS.len()],
) -> Result<DesignVariableDraft, VariableImportRefusal> {
    let cell = |column: usize| {
        columns[column]
            .and_then(|field| row.get(field))
            .unwrap_or_default()
            .trim()
    };
    let name = cell(NAME);
    let expression = cell(EXPRESSION);
    if name.is_empty() {
        // An unnamed row names nothing, so the refusal cannot either.
        return Err(VariableImportRefusal::Identifier {
            line: 0,
            subject: "(unnamed)".to_owned(),
            detail: "name is required".to_owned(),
        });
    }
    if expression.is_empty() {
        return Err(VariableImportRefusal::Dimension {
            line: 0,
            subject: name.to_owned(),
            detail: "expression is required".to_owned(),
        });
    }
    // A cell that names an option the column does not offer is a fact about
    // the sheet, not about the variable: nothing has been asserted about the
    // name, the dimension or the bounds yet.
    let unknown_option = |detail: String| schema(0, format!("{name}: {detail}"));
    Ok(DesignVariableDraft {
        name: name.to_owned(),
        expression: expression.to_owned(),
        quantity: enumerated("quantity", cell(QUANTITY), &quantity_choices(), None)
            .map_err(unknown_option)?,
        scope: enumerated("scope", cell(SCOPE), &SCOPE_CHOICES, Some(0)).map_err(unknown_option)?,
        description: cell(DESCRIPTION).to_owned(),
        allowed_range: allowed_range(cell(MINIMUM), cell(MAXIMUM)).map_err(|detail| {
            VariableImportRefusal::Bounds {
                line: 0,
                subject: name.to_owned(),
                detail,
            }
        })?,
        sweep_eligibility: enumerated(
            "sweep role",
            cell(SWEEP_ROLE),
            &sweep_role_choices(),
            Some(0),
        )
        .map_err(unknown_option)?,
        override_policy: enumerated(
            "override policy",
            cell(OVERRIDE_POLICY),
            &override_policy_choices(),
            Some(0),
        )
        .map_err(unknown_option)?,
        validation_error: None,
    })
}

/// Compose the draft's `minimum … maximum` bound from the two columns.
///
/// One bound without the other is refused rather than half-applied: the record
/// editor enforces the same rule, because a variable with only a floor is not
/// something preflight can check.
fn allowed_range(minimum: &str, maximum: &str) -> Result<String, String> {
    match (minimum.is_empty(), maximum.is_empty()) {
        (true, true) => Ok(String::new()),
        (false, false) => {
            for (column, bound) in [("minimum", minimum), ("maximum", maximum)] {
                if bound.contains('…') || bound.contains("..") {
                    return Err(format!(
                        "{column} '{bound}' contains a range separator; give each bound its own column"
                    ));
                }
            }
            Ok(format!("{minimum} … {maximum}"))
        }
        _ => Err("a bounded variable needs both a minimum and a maximum".to_owned()),
    }
}

/// Resolve one enumerated cell to the index its draft field holds.
///
/// A cell may carry the label the studio shows or a short token, matched
/// ignoring case. An empty cell takes the default; a column with no default is
/// required.
fn enumerated(
    column: &str,
    cell: &str,
    choices: &[(&str, usize)],
    default: Option<usize>,
) -> Result<usize, String> {
    if cell.is_empty() {
        return default.ok_or_else(|| format!("{column} is required"));
    }
    choices
        .iter()
        .find(|(spelling, _)| spelling.eq_ignore_ascii_case(cell))
        .map(|(_, index)| *index)
        .ok_or_else(|| {
            format!(
                "{column} '{cell}' is not one of: {}",
                choices
                    .iter()
                    .map(|(spelling, _)| *spelling)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

/// Every label paired with its draft index, then the short spellings a spec
/// sheet is likely to carry instead of a phrase with punctuation in it.
///
/// Deriving the indices from the label array is what keeps the accepted
/// spellings from drifting out of step with the options the studio offers.
fn spellings(
    labels: &[&'static str],
    aliases: &[(&'static str, usize)],
) -> Vec<(&'static str, usize)> {
    labels
        .iter()
        .enumerate()
        .map(|(index, label)| (*label, index))
        .chain(aliases.iter().copied())
        .collect()
}

/// Quantity labels are single words already, so they need no short form.
fn quantity_choices() -> Vec<(&'static str, usize)> {
    spellings(
        &DesignVariableQuantity::ALL.map(DesignVariableQuantity::label),
        &[],
    )
}

fn sweep_role_choices() -> Vec<(&'static str, usize)> {
    spellings(
        &DesignVariableSweepEligibility::ALL.map(DesignVariableSweepEligibility::label),
        &[("sweep", 0), ("optimization", 1), ("fixed", 2)],
    )
}

fn override_policy_choices() -> Vec<(&'static str, usize)> {
    spellings(
        &DesignVariableOverridePolicy::ALL.map(DesignVariableOverridePolicy::label),
        &[("override", 0), ("inherit", 1)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHEET: &str = "name,quantity,expression,minimum,maximum,scope,sweep role,override policy,description\n\
         RLOAD,Resistance,10 kohm,1 kohm,1 Mohm,project,sweep,override,Output load\n\
         CFILT,Capacitance,4.7 nF,1 nF,10 nF,testbench,fixed,inherit,Filter cap\n\
         VBIAS,Voltage,1.8 V,,,,,,\n";

    fn plan(app: &RSpiceApp) -> SimulationPlanId {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("the test instance owns a stable plan")
            .id()
    }

    /// Drive the real entry point and hand back the draft it opened on.
    fn open(
        app: &mut RSpiceApp,
        plan_id: SimulationPlanId,
        sheet: &str,
    ) -> DesignVariableImportDraft {
        let draft = import_draft_for_sheet(app, plan_id, "spec.csv", sheet);
        app.state.workbench.simulation_workflow =
            Some(SimulationWorkflowDialog::DesignVariableImport(draft));
        match app.state.workbench.simulation_workflow.clone() {
            Some(SimulationWorkflowDialog::DesignVariableImport(draft)) => draft,
            other => panic!("the import dialog must open, got {other:?}"),
        }
    }

    fn registry(app: &RSpiceApp, plan_id: SimulationPlanId) -> Vec<String> {
        app.state
            .workspace
            .plan_data(plan_id)
            .map(|payload| {
                payload
                    .design_variables
                    .iter()
                    .map(|variable| variable.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn revision(app: &RSpiceApp) -> u64 {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("the test instance owns a stable plan")
            .revision()
            .get()
    }

    /// The identity of each row's refusal, in file order.
    fn row_ids(draft: &DesignVariableImportDraft) -> Vec<Option<&'static str>> {
        draft
            .rows
            .iter()
            .map(|row| row.refusal.as_ref().map(VariableImportRefusal::id))
            .collect()
    }

    #[test]
    fn a_well_formed_sheet_previews_every_row_and_lands_as_one_transaction() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let before = revision(&app);

        let draft = open(&mut app, plan_id, SHEET);

        // Everything the sheet holds is previewed, and every row is offered.
        assert_eq!(row_ids(&draft), [None, None, None]);
        assert_eq!(
            draft
                .rows
                .iter()
                .map(|row| row.name.as_str())
                .collect::<Vec<_>>(),
            ["RLOAD", "CFILT", "VBIAS"]
        );
        assert_eq!(draft.accepted_count(), 3);
        assert_eq!(
            draft.rows[0].line, 2,
            "the file's own line, not the row index"
        );

        commit_selected_rows(&mut app, &draft).expect("a clean sheet adopts");

        assert_eq!(registry(&app, plan_id), ["RLOAD", "CFILT", "VBIAS"]);
        // One transaction, so one revision.
        assert_eq!(revision(&app), before + 1);
        let imported = &app
            .state
            .workspace
            .plan_data(plan_id)
            .expect("plan payload")
            .design_variables[0];
        assert_eq!(imported.quantity, DesignVariableQuantity::Resistance);
        assert_eq!(imported.scope, crate::state::DesignVariableScope::Project);
        assert_eq!(
            imported.sweep_eligibility,
            DesignVariableSweepEligibility::NestedSweepAndOptimization
        );
        assert_eq!(imported.description, "Output load");
        assert_eq!(
            imported
                .allowed_range
                .as_ref()
                .map(|range| (range.minimum.as_str(), range.maximum.as_str())),
            Some(("1 kohm", "1 Mohm"))
        );
        // The unbounded row is a real choice, not a missing value.
        assert!(
            app.state
                .workspace
                .plan_data(plan_id)
                .expect("plan payload")
                .design_variables[2]
                .allowed_range
                .is_none()
        );
    }

    /// Every refusal identity this import can report, each produced by a sheet
    /// that earns it rather than by constructing the variant directly.
    ///
    /// The ids are contract: they are what an engineer searches for and what a
    /// test can assert, so a reword of the sentence beside one must not move
    /// it. Prose-only refusals could not be asserted this way at all, which is
    /// why a sheet failing four different ways used to produce one sentence
    /// about whichever row came first.
    #[test]
    fn every_import_refusal_identity_is_reachable_from_a_sheet() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let seed = open(
            &mut app,
            plan_id,
            "name,quantity,expression\nRLOAD,Resistance,10 kohm\n",
        );
        commit_selected_rows(&mut app, &seed).expect("the plan owns RLOAD");

        // A sheet that fails four ways at once reports all four, in place.
        let draft = open(
            &mut app,
            plan_id,
            "name,quantity,expression,minimum,maximum\n\
             1BAD NAME,Resistance,50 ohm,,\n\
             RTERM,Resistance,not a resistance,,\n\
             RBIAS,Resistance,50 ohm,1 kohm,1 Mohm\n\
             rload,Resistance,22 kohm,,\n",
        );

        assert_eq!(
            row_ids(&draft),
            [
                Some("VARIMP-IDENTIFIER"),
                Some("VARIMP-DIMENSION"),
                Some("VARIMP-BOUNDS"),
                Some("VARIMP-COLLISION"),
            ]
        );
        // A refused row is never offered for adoption.
        assert_eq!(draft.accepted_count(), 0);
        // Each refusal names the line it came from and what it is about.
        assert_eq!(
            draft.rows[1]
                .refusal
                .as_ref()
                .map(VariableImportRefusal::line),
            Some(3)
        );
        assert_eq!(
            draft.rows[1]
                .refusal
                .as_ref()
                .and_then(VariableImportRefusal::subject),
            Some("RTERM")
        );

        // The fifth identity is about the table rather than about any row.
        let schema_refused = open(&mut app, plan_id, "name,quantity\nRTERM,Resistance\n");
        assert_eq!(
            schema_refused
                .sheet_refusal
                .as_ref()
                .map(VariableImportRefusal::id),
            Some("VARIMP-SCHEMA")
        );
    }

    /// The import promises that what lands is exactly what was on screen, or
    /// nothing does. A ticked row that cannot be adopted therefore stops the
    /// whole import rather than being quietly dropped from it.
    #[test]
    fn a_ticked_row_that_cannot_be_adopted_blocks_the_whole_import() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let mut draft = open(
            &mut app,
            plan_id,
            "name,quantity,expression\n\
             RGOOD,Resistance,50 ohm\n\
             RBAD,Resistance,not a resistance\n",
        );
        assert_eq!(row_ids(&draft), [None, Some("VARIMP-DIMENSION")]);

        // Ticking the refused row is something an operator can only do on
        // purpose; the import must then refuse rather than adopt only RGOOD.
        draft.rows[1].accepted = true;
        let refusal = import_refusal(&draft).expect("a ticked refused row blocks the import");
        assert!(refusal.contains("VARIMP-DIMENSION"), "{refusal}");

        let before = serde_json::to_value(&app.state.workspace).expect("workspace serializes");
        assert!(commit_selected_rows(&mut app, &draft).is_err());
        assert_eq!(
            serde_json::to_value(&app.state.workspace).expect("workspace still serializes"),
            before,
            "a refused import must not move the workspace"
        );
    }

    /// A collision is a property of the selection, not of the sheet: two rows
    /// may share a name as long as only one of them is being adopted.
    #[test]
    fn unticking_one_of_two_rows_sharing_a_name_clears_the_collision() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let mut draft = open(
            &mut app,
            plan_id,
            "name,quantity,expression\n\
             RLOAD,Resistance,10 kohm\n\
             RLOAD,Resistance,22 kohm\n",
        );
        assert_eq!(row_ids(&draft), [None, Some("VARIMP-COLLISION")]);

        // Give up the first row and take the second instead. The collision was
        // never a fact about the sheet; it was a fact about taking both.
        draft.rows[0].accepted = false;
        draft.rows[1].accepted = true;
        mark_collisions(&app, plan_id, &mut draft);

        assert_eq!(row_ids(&draft), [None, None]);
        assert_eq!(draft.accepted_count(), 1);
        commit_selected_rows(&mut app, &draft).expect("one of the two adopts");
        assert_eq!(registry(&app, plan_id), ["RLOAD"]);
    }

    /// A column the import does not recognize no longer refuses the sheet,
    /// because the dialog can now show that nothing is reading it. What it must
    /// never do is drop it silently — a sheet spelling its bound column `min`
    /// would otherwise import unbounded variables with nothing saying so.
    #[test]
    fn a_column_nothing_reads_is_named_rather_than_dropped() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let draft = open(
            &mut app,
            plan_id,
            "name,quantity,expression,min\nRLOAD,Resistance,10 kohm,1 kohm\n",
        );

        assert_eq!(unread_columns(&draft).as_deref(), Some("'min'"));
        // The row still resolves; it simply has no bound.
        assert_eq!(row_ids(&draft), [None]);
        assert!(draft.rows[0].draft.allowed_range.is_empty());
    }

    /// Binding a column the sheet spelled differently re-resolves the rows,
    /// which is the whole point of the mapping being editable.
    #[test]
    fn rebinding_a_column_re_resolves_every_row() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let mut draft = open(
            &mut app,
            plan_id,
            "name,quantity,expression,min,maximum\nRLOAD,Resistance,10 kohm,1 kohm,1 Mohm\n",
        );
        // `min` binds nothing, so a maximum without a minimum is a bound
        // refusal rather than an unbounded variable.
        assert_eq!(row_ids(&draft), [Some("VARIMP-BOUNDS")]);

        draft.binding[MINIMUM] = Some(3);
        resolve_rows(&app, plan_id, &mut draft);

        assert_eq!(row_ids(&draft), [None]);
        assert_eq!(draft.rows[0].draft.allowed_range, "1 kohm \u{2026} 1 Mohm");
    }

    /// A required field with nothing bound to it is a refusal about the sheet,
    /// and it is recoverable: binding a column clears it without re-picking the
    /// file.
    #[test]
    fn a_required_field_with_no_column_refuses_the_sheet_until_it_is_bound() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let mut draft = open(
            &mut app,
            plan_id,
            "name,quantity,value\nRLOAD,Resistance,10 kohm\n",
        );
        let refusal = draft.sheet_refusal.as_ref().expect("expression is unbound");
        assert_eq!(refusal.id(), "VARIMP-SCHEMA");
        assert!(
            refusal.detail().contains("expression"),
            "{}",
            refusal.message()
        );
        assert!(
            draft.rows.is_empty(),
            "nothing resolves while a required field is unbound"
        );

        draft.binding[EXPRESSION] = Some(2);
        resolve_rows(&app, plan_id, &mut draft);

        assert!(draft.sheet_refusal.is_none());
        assert_eq!(row_ids(&draft), [None]);
    }

    /// Adopting the whole sheet at one scope is the common case for a sheet
    /// written without scopes in mind, and it must override what the sheet says
    /// rather than being ignored where the sheet declares one.
    #[test]
    fn adopting_at_one_scope_overrides_the_scope_column() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let mut draft = open(
            &mut app,
            plan_id,
            "name,quantity,expression,scope\n\
             RLOAD,Resistance,10 kohm,testbench\n\
             CFILT,Capacitance,4.7 nF,testbench\n",
        );
        assert_eq!(draft.rows[0].draft.scope, 0, "the sheet says testbench");

        draft.scope = 1;
        draft.override_scope = true;
        resolve_rows(&app, plan_id, &mut draft);

        assert!(draft.rows.iter().all(|row| row.draft.scope == 1));
        commit_selected_rows(&mut app, &draft).expect("the sheet adopts at the chosen scope");
        assert!(
            app.state
                .workspace
                .plan_data(plan_id)
                .expect("plan payload")
                .design_variables
                .iter()
                .all(|variable| variable.scope == crate::state::DesignVariableScope::Project)
        );
    }

    #[test]
    fn column_order_is_not_load_bearing() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let ordered = open(
            &mut app,
            plan_id,
            "name,quantity,expression,minimum,maximum,description\n\
             RLOAD,Resistance,10 kohm,1 kohm,1 Mohm,Output load\n",
        );
        let shuffled = open(
            &mut app,
            plan_id,
            "Description,MAXIMUM,expression,Quantity,minimum,Name\n\
             Output load,1 Mohm,10 kohm,Resistance,1 kohm,RLOAD\n",
        );

        let fields = |draft: &DesignVariableImportDraft| {
            draft
                .rows
                .iter()
                .map(|row| {
                    (
                        row.draft.name.clone(),
                        row.draft.expression.clone(),
                        row.draft.quantity,
                        row.draft.allowed_range.clone(),
                        row.draft.description.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(fields(&ordered), fields(&shuffled));
    }

    #[test]
    fn a_sheet_with_only_a_header_row_imports_nothing() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let draft = open(&mut app, plan_id, "name,quantity,expression\n");
        let refusal = draft
            .sheet_refusal
            .as_ref()
            .expect("a sheet with no rows is refused");
        assert_eq!(refusal.id(), "VARIMP-SCHEMA");
        assert!(
            refusal.detail().contains("holds no variables"),
            "{}",
            refusal.message()
        );
    }

    /// A column the sheet names twice is ambiguous, and nothing can choose
    /// between the two. Unlike an unrecognized column there is no honest
    /// default, so this stays a refusal.
    #[test]
    fn a_column_named_twice_is_refused() {
        let refusal = auto_binding(&[
            "name".to_owned(),
            "quantity".to_owned(),
            "expression".to_owned(),
            "Name".to_owned(),
        ])
        .expect_err("an ambiguous binding is refused");
        assert_eq!(refusal.id(), "VARIMP-SCHEMA");
        assert!(
            refusal.detail().contains("named twice"),
            "{}",
            refusal.message()
        );
    }

    /// `SCOPE_CHOICES` copies the scope labels because two variants carry
    /// payloads and cannot be enumerated. This is the pin that keeps the copy
    /// honest: rename a scope and the spelling a spec sheet is allowed to use
    /// must be renamed with it, rather than quietly ceasing to match.
    #[test]
    fn scope_spellings_match_the_scopes_they_name() {
        use crate::state::DesignVariableScope;

        let app = RSpiceApp::test_instance();
        let scopes = [
            DesignVariableScope::Testbench,
            DesignVariableScope::Project,
            DesignVariableScope::SelectedCell {
                cell: app.state.workspace.active_view.clone(),
            },
            DesignVariableScope::SelectedAnalysis {
                analysis_id: crate::product::AnalysisInstanceId::new(),
            },
        ];
        for (index, scope) in scopes.iter().enumerate() {
            assert!(
                SCOPE_CHOICES
                    .iter()
                    .any(|(spelling, choice)| *choice == index && *spelling == scope.label()),
                "scope {index} is labelled {:?}, which no spec-sheet spelling accepts",
                scope.label()
            );
            // The adopt-at control offers the same four, in the same order.
            assert_eq!(SCOPE_LABELS[index], scope.label());
        }
    }
    /// A refusal sentence is bounded by the pane it is printed in.
    ///
    /// An egui row extends its items rather than wrapping them, so a refusal
    /// detail laid out at its natural width carried its row past the split
    /// pane -- and a row that overruns a `Ui` widens that `Ui`'s own bounds, so
    /// the note beneath the table then wrapped at a width the pane never had,
    /// and the dialog's border was painted two hundred points outside the
    /// header and footer it belongs to.
    ///
    /// Checked against the painted text rather than through AccessKit: a label
    /// is not a control, and it was the labels that escaped.
    #[test]
    fn the_import_table_prints_nothing_outside_the_dialog_surface() {
        // One refusing row, whose detail is a full sentence: the longest thing
        // the table ever has to place in its last cell.
        const REFUSED: &str = "name,quantity,expression,minimum,maximum,scope,description\n\
             RFB,Resistance,47k,,,testbench,feedback resistor\n";

        fn text_spans(shape: &egui::epaint::Shape, out: &mut Vec<(String, f32, f32)>) {
            match shape {
                egui::epaint::Shape::Text(text) => out.push((
                    text.galley.job.text.clone(),
                    text.pos.x,
                    text.pos.x + text.galley.size().x,
                )),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        text_spans(shape, out);
                    }
                }
                _ => {}
            }
        }

        for width in [1000.0f32, 1600.0] {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut app = RSpiceApp::test_instance();
            let plan_id = plan(&app);
            let _ = open(&mut app, plan_id, REFUSED);

            let mut run = || {
                ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 1_000.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| super::super::show_workflow_dialogs(ctx, &mut app),
                )
            };
            // A content-height surface lays out against the height and width
            // its previous pass measured.
            let _ = run();
            let _ = run();
            let output = run();

            let nodes = output
                .platform_output
                .accesskit_update
                .as_ref()
                .expect("AccessKit tree update")
                .nodes
                .clone();
            let surface = nodes
                .iter()
                .find(|(_, node)| node.role() == egui::accesskit::Role::Dialog)
                .and_then(|(_, node)| node.bounds())
                .expect("the dialog publishes its bounds");

            let mut spans = Vec::new();
            for shape in &output.shapes {
                text_spans(&shape.shape, &mut spans);
            }
            let escaped: Vec<String> = spans
                .iter()
                .filter(|(_, left, right)| {
                    f64::from(*right) > surface.x1 + 0.5 || f64::from(*left) < surface.x0 - 0.5
                })
                .map(|(text, left, right)| format!("{left:.0}..{right:.0} {text:?}"))
                .collect();
            assert!(
                escaped.is_empty(),
                "on a {width:.0}-point viewport the surface spans {:.0}..{:.0} and these are \
                 printed outside it:\n{}",
                surface.x0,
                surface.x1,
                escaped.join("\n")
            );
        }
    }
}
