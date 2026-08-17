//! The versioned simulation-plan catalog: create, activate, rename, clone,
//! compare, archive, restore, exchange, and queue a campaign over plans.
//!
//! A plan is the unit of ownership here. Activating one is atomic — the setup
//! and the workspace payload move together or not at all — and archiving one
//! never rewrites the immutable results that reference it, which is why every
//! operation below reports what it did to the catalog rather than to the run.
//!
//! Every surface in this module reads one projection of the catalog, built by
//! [`records`]. A count shown in the table, in the comparison, and in the
//! campaign is therefore the same count from the same owner rather than three
//! re-derivations free to disagree.

mod campaign;
mod compare;
mod create;
mod exchange;
mod kit;
mod lifecycle;
mod records;

use super::*;
use kit::{ColumnTrack, HeadStatus, LifecycleTone, SplitColumn, TableColumn, TableRow};
use records::{PlanCatalogRecord, plan_catalog_records};

use crate::workbench::app_state::ReferencePvtPoint;
use crate::workbench::design_system::property_row_status;
use crate::workbench::state::SimulationPlanScope;

/// The dialog's own copy, named because its height is load-bearing.
///
/// The header grows with the description and the body budget shrinks by exactly
/// that much. A test that reconstructed this dialog without its description
/// measured a body 60 points taller than the real one and reported a surface
/// that fits while the last row of the aside was in fact scrolled under the
/// footer. One owner for the copy is what stops that measurement from lying.
const PLAN_DIALOG_EYEBROW: &str = "SIMULATION · VERSIONED PLAN LIFECYCLE · SINGLE ACTIVE OWNER";
const PLAN_DIALOG_TITLE: &str = "Simulation plans";
const PLAN_DIALOG_PRIMARY: &str = "Open selected plan";
const PLAN_DIALOG_DESCRIPTION: &str = "Create, select, compare, rename, clone, restore, and archive complete simulation plans without rewriting immutable results.";

/// The records table, left to right.
///
/// The authored table carries two more columns — a design and testbench
/// binding, and a modified timestamp — and overflows its own container by
/// roughly 210 points because of them. They are also the two columns whose
/// facts have no owner in RSpice: no plan binds a design or a testbench, and
/// nothing stamps a plan as modified. Dropping both closes the fact audit and
/// the fit in one move, which is why these seven are the set.
///
/// Only the identity column is elastic. Every other column's longest value is
/// known — a lifecycle word, a revision, two counts, a forecast, two more
/// counts — so they hold their width and stay readable by position down the
/// rows while the dialog resizes.
///
/// Each fixed width is its heading's or its longest value's, whichever is wider,
/// and no more. That is not tidiness: this set's total is what decides whether
/// the split can stay two-column, and the one-column arrangement it falls back
/// to cannot fit a 640-point viewport at all. Widening a column here can push
/// the surface into an arrangement that does not fit.
const PLAN_COLUMNS: [TableColumn; 7] = [
    TableColumn {
        heading: "Plan / identity",
        track: ColumnTrack::Elastic(120.0),
    },
    TableColumn {
        heading: "Lifecycle",
        track: ColumnTrack::Fixed(70.0),
    },
    TableColumn {
        heading: "Revision",
        track: ColumnTrack::Fixed(50.0),
    },
    TableColumn {
        heading: "Analyses",
        track: ColumnTrack::Fixed(56.0),
    },
    TableColumn {
        heading: "Run set",
        track: ColumnTrack::Fixed(66.0),
    },
    TableColumn {
        heading: "Models",
        track: ColumnTrack::Fixed(46.0),
    },
    TableColumn {
        heading: "Results",
        track: ColumnTrack::Fixed(46.0),
    },
];

/// Exactly the four fields [`matches_plan_filter`] searches, and nothing else.
///
/// The authored placeholder offers "plan, binding, run set, or revision", and a
/// binding is not a thing a RSpice plan has. A placeholder that names a field
/// the filter cannot match sends the reader looking for a plan by a word that
/// will never hit.
///
/// It does not repeat the word "Filter", which the adjacent label already says,
/// because the hint has to read inside [`PLAN_FILTER_WIDTH`].
const PLAN_FILTER_HINT: &str = "Name, identity, revision, or run set…";

/// Width of the filter input.
///
/// Fixed, not greedy. `available_width().min(320)` took every point it was
/// offered and left the five controls after it to wrap, which at the
/// edge-to-edge width orphaned Import onto a line of its own. This is the widest
/// the input can be while the whole row still fits one line there.
const PLAN_FILTER_WIDTH: f32 = 258.0;

/// The scope control's options, in order.
///
/// The authored control offers four, one of them "Governed baselines". RSpice
/// has no governance state on a plan, so that option would select an empty set
/// forever and is not ported.
///
/// "Active plan" was considered as the fourth and dropped. It is the only scope
/// whose result set is provably one row, and that row is already at a fixed
/// known position — [`records::plan_catalog_records`] always projects the active
/// plan first, and its lifecycle carries the accent tone. Narrowing to it
/// reveals nothing not already on screen, and it would leave the surface in a
/// state where the dialog's own primary action is a guaranteed no-op, since
/// [`commit_activate_plan`] returns early for the plan that is already active.
///
/// The two narrowing options that remain partition the catalog — `Working` is
/// every plan that is not archived, `Archived` is the rest — so every plan is in
/// exactly one of them and the reader's ordinary view is the one that hides
/// retired plans without also hiding the plan being worked on.
const PLAN_SCOPES: [(SimulationPlanScope, &str); 3] = [
    (SimulationPlanScope::All, "All plans"),
    (SimulationPlanScope::Working, "Working"),
    (SimulationPlanScope::Archived, "Archived"),
];

/// What the catalog guarantees about the two transactions a reader is about to
/// perform, stated from what the commit paths below actually do.
///
/// The authored notes also cite dirty editors, permissions, entitlement
/// failures and schema migrations. None of those has an owner here — RSpice
/// refuses a switch on a validation failure, not on an entitlement — so those
/// clauses are dropped rather than restated as things that might happen.
const PLAN_BOUNDARY_NOTES: [(&str, &str); 3] = [
    (
        "Switching is atomic",
        "Opening a plan moves the analysis setup and the plan-owned workspace \
         payload together. Both are migrated on a copy that is installed only \
         once the whole switch validates, so a refused switch leaves the \
         current plan active and every payload where it was.",
    ),
    (
        "Results are references",
        "A run's authenticated receipt names the plan it was dispatched from, \
         and that is what the result count counts. Renaming, cloning, \
         importing and archiving change the catalog only: no receipt is \
         rewritten and no result is copied, so a result outlives the plan it \
         points at.",
    ),
    ("Stable identity retained", PLAN_IDENTITY_NOTE),
];

/// What the four lifecycle operations do to a plan's identity.
///
/// Every clause is checked against its commit path: `rename_plan` never touches
/// the identity or the revision, `clone_active_plan` and `import_plan` both mint
/// one through `clone_as_new`, `restore_plan` reverses an archive, and
/// `archive_plan` refuses the active plan itself rather than relying on this
/// dialog to disable the button.
///
/// It is the third of [`PLAN_BOUNDARY_NOTES`] and not a row of the selected-plan
/// detail, because it is a statement about what the catalog's operations
/// guarantee — the same kind of claim as the two beside it — and not a fact about
/// whichever plan happens to be selected. It sat under the detail, which both
/// mixed those two kinds and put 60 points of invariant in the one column that
/// had no room for them.
const PLAN_IDENTITY_NOTE: &str = "Renaming keeps a plan's identity and its \
     revision. Cloning and importing each mint a new identity. Archiving is \
     reversible, and the catalog refuses it on the active plan.";

/// What the selected-plan aside says in place of a workload when the plan's
/// run-space declaration does not validate.
///
/// The forecast is absent in that case, and every quantity derived from it with
/// it. Printing zero points, zero tasks and a zero cost would read as "this
/// plan declares no work", which is a different claim and a false one — the
/// plan declares work that cannot be expanded.
const RUN_SET_DOES_NOT_VALIDATE: &str = "does not validate";

enum PlanManagerAction {
    Create,
    Campaign,
    Rename,
    Clone,
    Compare,
    Export,
    Import,
    CopyExport,
    ApplyImport,
    ApplyCampaign,
    Archive,
    Restore,
    ApplyCreate,
    ApplyRename,
    ConfirmArchive,
    CancelInline,
    /// The browse surface's primary: open the selected plan.
    OpenSelected,
    /// Dismiss the manager entirely, rather than return to browsing.
    Close,
}

/// The two exchange entries keep the path their one consumer already uses.
///
/// They moved to [`exchange`] with the rest of that route, and the studio's own
/// round-trip test reaches them under `plan_manager`. That test is not this
/// wave's file, so the path is preserved here rather than repointed there — and a
/// bare-name search for either would have reported no consumers at all. The
/// re-export is test-only because the shell itself reaches them through
/// [`exchange`].
#[cfg(test)]
pub(super) use exchange::{commit_import_simulation_plan, export_simulation_plan_package};

/// Dispatch. Every mode is a surface, and this decides which one runs.
///
/// There is no route-specific rendering below this point: each arm hands the
/// draft and the catalog projection to one module and takes back an action, and
/// [`handle_plan_manager_action`] is the single place that touches the
/// application. That is what lets five lanes redesign five routes in parallel
/// without any of them opening this file.
pub(super) fn plan_manager_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: SimulationPlanManagerDraft,
) {
    let records = plan_catalog_records(app);
    if !records
        .iter()
        .any(|record| record.id == draft.selected_plan_id)
        && let Some(active) = records.iter().find(|record| record.active)
    {
        draft.selected_plan_id = active.id;
        draft.name = active.name.clone();
    }

    let action = match draft.mode {
        SimulationPlanManagerMode::Browse => browse_dialog(ctx, &mut draft, &records),
        SimulationPlanManagerMode::Create => create::dialog(ctx, &mut draft, &records),
        SimulationPlanManagerMode::Rename | SimulationPlanManagerMode::ConfirmArchive => {
            lifecycle::dialog(ctx, &mut draft, &records)
        }
        SimulationPlanManagerMode::Compare => compare::dialog(ctx, &mut draft, &records),
        SimulationPlanManagerMode::Export | SimulationPlanManagerMode::Import => {
            exchange::dialog(ctx, &mut draft, &records)
        }
        SimulationPlanManagerMode::Campaign => campaign::dialog(ctx, &mut draft, &records),
    };
    handle_plan_manager_action(ctx, app, draft, action);
}

/// The browse surface: the records table, its actions, and the selected plan.
///
/// It has the same shape as the five child routes on purpose. The shell's own
/// surface being one more case of the contract, rather than an exception to it,
/// is what keeps the dispatch above a single expression.
fn browse_dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    let can_open = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id)
        .is_some_and(|record| !record.archived);
    let mut action = None;
    let choice = Dialog::new(PLAN_DIALOG_EYEBROW, PLAN_DIALOG_TITLE, PLAN_DIALOG_PRIMARY)
        .description(PLAN_DIALOG_DESCRIPTION)
        .size(DialogSize::WideWorkflow)
        .flush_body()
        .ghost("Close")
        .primary_enabled(can_open)
        .show(ctx, |ui| {
            plan_manager_body(ui, draft, records, &mut action);
        });
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::OpenSelected),
        DialogChoice::Ghost | DialogChoice::Cancelled => action = Some(PlanManagerAction::Close),
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// Whether `scope` admits `record`.
///
/// `Working` and `Archived` are complementary, so every plan is admitted by
/// exactly one of them and no plan can be reached only through `All plans`.
const fn plan_scope_admits(scope: SimulationPlanScope, record: &PlanCatalogRecord) -> bool {
    match scope {
        SimulationPlanScope::All => true,
        SimulationPlanScope::Working => !record.archived,
        SimulationPlanScope::Archived => record.archived,
    }
}

/// Whether `record` matches the filter, over the four fields
/// [`PLAN_FILTER_HINT`] names.
///
/// Name and identity alone were not enough to find a plan by anything a reader
/// can see in the table: a revision and a declared run-set size are both
/// painted in every row and neither was searchable.
fn matches_plan_filter(record: &PlanCatalogRecord, query: &str) -> bool {
    query.is_empty()
        || [
            record.name.clone(),
            record.id.to_string(),
            record.revision.to_string(),
            record.run_set_label(),
        ]
        .iter()
        .any(|field| field.to_ascii_lowercase().contains(query))
}

/// The tone that states this plan's lifecycle.
///
/// The word itself comes from `record.lifecycle_label()`, which is its one
/// owner; this maps the same two booleans to a tone. Tone is presentation and
/// the word is projection, so they are one derivation each rather than two
/// spellings of the same string.
const fn lifecycle_tone(record: &PlanCatalogRecord) -> LifecycleTone {
    if record.active {
        LifecycleTone::Active
    } else if record.archived {
        LifecycleTone::Archived
    } else {
        LifecycleTone::Available
    }
}

/// The manager's body: a toolbar over a records column and a selected-plan
/// aside.
///
/// The split's breakpoint is the table's own minimum width rather than a number
/// authored here, so the records column is never handed a track the seven
/// columns would overflow. That is the one defect the authored reference has and
/// this surface must not: its table is 899 points wide inside a 685-point cell.
fn plan_manager_body(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
    action: &mut Option<PlanManagerAction>,
) {
    plan_manager_toolbar(ui, draft, records, action);
    let query = draft.filter.trim().to_ascii_lowercase();
    let visible = records
        .iter()
        .filter(|record| plan_scope_admits(draft.scope, record))
        .filter(|record| matches_plan_filter(record, &query))
        .cloned()
        .collect::<Vec<_>>();
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id)
        .cloned();

    // The detail's arrangement follows the split's. `split_tracks` is pure and
    // sees the same available width `manager_split` will, so the two cannot
    // disagree about which arrangement this frame is in.
    let minimum = kit::table_minimum_width(&PLAN_COLUMNS);
    let stacked = kit::split_tracks(ui.available_width(), minimum).stacked;
    kit::manager_split(ui, minimum, |ui, column| match column {
        SplitColumn::Records => {
            plan_manager_records_column(ui, draft, &visible, selected.as_ref(), action);
        }
        SplitColumn::Aside => {
            if let Some(selected) = selected.as_ref() {
                // Stacked, the detail has the whole dialog width and lays its
                // three groups across it; beside the table it has a third of
                // the width and stacks them.
                selected_plan_properties(ui, selected, if stacked { 3 } else { 1 });
            }
        }
    });

    workflow_validation_message(ui, draft.validation_error.as_deref());
}

/// Filter, scope, and the three actions that create a plan rather than act on
/// the selected one.
///
/// Import belongs here and not in the aside's action row: importing mints a new
/// plan, so it is a sibling of New plan and Queue campaign, not an operation on
/// whatever row happens to be selected. It was in the action row, where it read
/// as "import over this plan".
///
/// The row is a band, not a plain line of controls. The dialog body is flush, so
/// nothing above the split would otherwise inset its content and the filter
/// label sat against the dialog's own border. The band owns that inset, its
/// panel fill and the rule that separates it from the records below.
fn plan_manager_toolbar(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
    action: &mut Option<PlanManagerAction>,
) {
    let t = Tokens::get(ui.ctx());
    let band = egui::Frame::new()
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            plan_manager_toolbar_controls(ui, draft, records, action);
        });
    ui.painter().hline(
        band.response.rect.x_range(),
        band.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn plan_manager_toolbar_controls(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
    action: &mut Option<PlanManagerAction>,
) {
    let t = Tokens::get(ui.ctx());
    // Wrapping is the safety net, not the layout: `PLAN_FILTER_WIDTH` is chosen
    // so every control fits one line at the edge-to-edge width, and wrapping
    // only ever catches a narrower window than this dialog is used at.
    ui.horizontal_wrapped(|ui| {
        ui.label("Filter");
        ui.add_sized(
            vec2(PLAN_FILTER_WIDTH, t.metrics.ctl_h),
            egui::TextEdit::singleline(&mut draft.filter)
                .font(egui::TextStyle::Monospace)
                .margin(egui::Margin::symmetric(8, 4))
                .hint_text(PLAN_FILTER_HINT),
        );
        let choices = PLAN_SCOPES
            .iter()
            .map(|(_, label)| (*label).to_owned())
            .collect::<Vec<_>>();
        let current = PLAN_SCOPES
            .iter()
            .find(|(scope, _)| *scope == draft.scope)
            .map_or(PLAN_SCOPES[0].1, |(_, label)| label);
        if let Some(picked) = select(
            ui,
            "simulation.plan-manager.scope",
            "Plan scope",
            current,
            &choices,
            150.0,
        ) && let Some((scope, _)) = PLAN_SCOPES.get(picked)
        {
            draft.scope = *scope;
        }
        if Button::new("New plan…").accent().show(ui).clicked() {
            *action = Some(PlanManagerAction::Create);
        }
        if Button::new("Queue campaign…")
            .enabled(records.iter().filter(|record| !record.archived).count() >= 2)
            .show(ui)
            .clicked()
        {
            *action = Some(PlanManagerAction::Campaign);
        }
        if Button::new("Import…").show(ui).clicked() {
            *action = Some(PlanManagerAction::Import);
        }
    });
}

/// The records table, the operations on the selected row, and the two
/// boundaries that qualify both.
fn plan_manager_records_column(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    visible: &[PlanCatalogRecord],
    selected: Option<&PlanCatalogRecord>,
    action: &mut Option<PlanManagerAction>,
) {
    let rows = visible
        .iter()
        .map(|record| TableRow {
            selected: record.id == draft.selected_plan_id,
            announced: announced_plan_row(record),
        })
        .collect::<Vec<_>>();
    let clicked = kit::records_table(
        ui,
        "simulation.plan-manager.rows",
        &PLAN_COLUMNS,
        &rows,
        |ui, row, column| {
            let record = &visible[row];
            let t = Tokens::get(ui.ctx());
            match column {
                0 => kit::cell_identity(ui, &record.name, &record.id.to_string()),
                1 => kit::lifecycle_chip(ui, record.lifecycle_label(), lifecycle_tone(record)),
                2 => kit::cell_value(ui, &record.revision.to_string(), t.color.text),
                3 => kit::cell_value(
                    ui,
                    &format!("{} / {}", record.enabled, record.analyses),
                    t.color.text,
                ),
                // A run set that does not validate is toned as the error it is;
                // `point_count` is absent exactly when the declaration is.
                4 => kit::cell_value(
                    ui,
                    &record.run_set_label(),
                    if record.point_count().is_some() {
                        t.color.text
                    } else {
                        t.color.err
                    },
                ),
                5 => kit::cell_value(ui, &record.model_bindings.to_string(), t.color.text),
                _ => kit::cell_value(ui, &record.results.to_string(), t.color.text),
            }
        },
    );
    if let Some(row) = clicked {
        let record = &visible[row];
        draft.selected_plan_id = record.id;
        draft.name = record.name.clone();
        draft.mode = SimulationPlanManagerMode::Browse;
        draft.validation_error = None;
    }
    if let Some(selected) = selected {
        ui.add_space(8.0);
        plan_selection_actions(ui, selected, action);
    }
    ui.add_space(8.0);
    kit::note_grid(ui, &PLAN_BOUNDARY_NOTES);
}

/// Every fact the row paints, in one accessibility node.
///
/// The row is the interactive unit, so its node has to carry the whole row.
/// Seven separately announced numbers with no plan attached to them are not
/// readable in sequence, and the identity a cell paints is elided to its column
/// while this states it in full.
fn announced_plan_row(record: &PlanCatalogRecord) -> String {
    format!(
        "{} · {} · {} · revision {} · {} of {} analyses enabled · run set {} · {} model binding{} · {} result reference{}",
        record.name,
        record.id,
        record.lifecycle_label(),
        record.revision,
        record.enabled,
        record.analyses,
        record.run_set_label(),
        record.model_bindings,
        plan_plural_suffix(record.model_bindings),
        record.results,
        plan_plural_suffix(record.results)
    )
}

/// The five operations on the selected plan.
///
/// They sit under the records table rather than in the selected-plan detail,
/// where the authored reference puts them. That aside is six rows tall in the
/// mockup and eleven here, and eleven rows plus five buttons in a narrow track
/// pushed the buttons off the bottom of a surface that is not allowed to scroll.
/// Under the table they are still where the reader's attention is — they act on
/// the selected row, and the row is right above them — and they get a wide track
/// where all five fit on one line.
fn plan_selection_actions(
    ui: &mut Ui,
    selected: &PlanCatalogRecord,
    action: &mut Option<PlanManagerAction>,
) {
    ui.horizontal_wrapped(|ui| {
        if Button::new("Rename…").show(ui).clicked() {
            *action = Some(PlanManagerAction::Rename);
        }
        if Button::new("Clone…")
            .enabled(!selected.archived)
            .show(ui)
            .clicked()
        {
            *action = Some(PlanManagerAction::Clone);
        }
        if Button::new("Compare…").show(ui).clicked() {
            *action = Some(PlanManagerAction::Compare);
        }
        if Button::new("Export…").show(ui).clicked() {
            *action = Some(PlanManagerAction::Export);
        }
        if selected.archived {
            if Button::new("Restore").show(ui).clicked() {
                *action = Some(PlanManagerAction::Restore);
            }
        } else if Button::new("Archive…")
            .enabled(!selected.active)
            .show(ui)
            .clicked()
        {
            *action = Some(PlanManagerAction::Archive);
        }
    });
}

/// Everything the catalog knows about the selected plan, in the order a reader
/// asks for it: which plan this is, what it declares as work, and what it owns
/// that a run would consume or a comparison would diff.
///
/// The mockup's aside is six rows under one heading. Two of those six —
/// a per-plan design and testbench binding, and a named execution profile —
/// have no owner anywhere in RSpice, so this states the strongest facts the
/// catalog does own instead of stubbing them: the reference corner, the run
/// set's own forecast, the model closure, and the plan-owned record counts.
/// That is more facts than fit one list, so they are grouped; eleven flat rows
/// in this column read as a wall rather than as three answers.
///
/// Every quantity here comes off [`PlanCatalogRecord`], never off a second
/// derivation — including the two that go absent with an unvalidated run set.
///
/// The first heading carries the selected plan's lifecycle. The authored aside
/// states it as active or available, which is a binary reading of three states:
/// an archived plan is not available, and the dialog's own primary action is
/// disabled for it. So the head states the same word the Lifecycle column does,
/// from the same owner.
///
/// `columns` is how the three groups are arranged, and it is the whole reason
/// this surface fits a 640-point viewport. Eleven property rows stacked in one
/// track are 308 points tall — sixty per cent of the entire body budget — so
/// when the layout hands the detail the full dialog width it spends that width
/// on three side-by-side groups and costs the height of the longest one instead
/// of the sum of all three. Each group has exactly one painter, called from
/// both arrangements, so the two cannot come to state different things.
fn selected_plan_properties(ui: &mut Ui, selected: &PlanCatalogRecord, columns: usize) {
    if columns <= 1 {
        selected_plan_identity_group(ui, selected);
        selected_plan_work_group(ui, selected);
        selected_plan_records_group(ui, selected);
    } else {
        kit::equal_columns(ui, 3, |ui, index| match index {
            0 => selected_plan_identity_group(ui, selected),
            1 => selected_plan_work_group(ui, selected),
            _ => selected_plan_records_group(ui, selected),
        });
    }
}

/// Which plan this is.
fn selected_plan_identity_group(ui: &mut Ui, selected: &PlanCatalogRecord) {
    kit::section_head(
        ui,
        "Selected plan",
        Some(HeadStatus {
            label: selected.lifecycle_label(),
            tone: lifecycle_tone(selected),
        }),
    );
    property_row(ui, "Name", &selected.name);
    property_row(ui, "Stable identity", &selected.id.to_string());
    property_row(ui, "Revision", &selected.revision.to_string());
}

/// What the plan declares as work.
fn selected_plan_work_group(ui: &mut Ui, selected: &PlanCatalogRecord) {
    let t = Tokens::get(ui.ctx());
    kit::section_head(ui, "Declared work", None);
    property_row(
        ui,
        "Reference PVT corner",
        &reference_pvt_label(selected.reference_pvt),
    );
    // The declared scale and its modelled cost are one row, because they are one
    // fact: all four numbers come off the same run-set projection. Split across
    // two rows they also stated the same absence twice — an unvalidated run set
    // printed "does not validate" under both — which read as two problems.
    match (
        selected.point_count(),
        selected.task_count(),
        selected.estimated_duration(),
        selected.estimated_storage(),
    ) {
        (Some(points), Some(tasks), Some(duration), Some(storage)) => property_row(
            ui,
            "Declared run set",
            &format!(
                "{points} PVT point{} · {tasks} task{} · {duration} · {storage}",
                plan_plural_suffix(points),
                plan_plural_suffix(tasks)
            ),
        ),
        _ => property_row_status(
            ui,
            "Declared run set",
            RUN_SET_DOES_NOT_VALIDATE,
            t.color.err,
            StatusMark::Failure,
        ),
    };
    property_row(
        ui,
        "Model closure",
        &format!(
            "{} binding{}",
            selected.model_bindings,
            plan_plural_suffix(selected.model_bindings)
        ),
    );
}

/// What the plan owns that a run would consume or a comparison would diff.
fn selected_plan_records_group(ui: &mut Ui, selected: &PlanCatalogRecord) {
    kit::section_head(ui, "Plan-owned records", None);
    property_row(
        ui,
        "Variables, outputs, specifications",
        &format!(
            "{} · {} · {}",
            selected.design_variables, selected.saved_outputs, selected.specifications
        ),
    );
    property_row(ui, "Source lineage", &lineage_label(selected));
    // The pinned baseline and the result count are one row for the same reason:
    // both are about the runs that reference this plan, and both are read
    // together when deciding whether a plan has evidence behind it.
    property_row(
        ui,
        "Runs referencing this plan",
        &format!(
            "{} immutable reference{} · {}",
            selected.results,
            plan_plural_suffix(selected.results),
            selected.regression_baseline.map_or_else(
                || "no baseline pinned".to_owned(),
                |run| format!("baseline run {run}"),
            )
        ),
    );
}

/// The plan's nominal point, corner first, the way the workbench chrome's own
/// PVT control reads it.
fn reference_pvt_label(point: ReferencePvtPoint) -> String {
    format!(
        "{} · {:.1} °C",
        point.process.short_name(),
        point.temperature_celsius
    )
}

/// Where the plan came from. A clone and an import both record a source plan
/// and the exact revision they were taken at, and a plan with neither is a
/// root — which is a fact worth stating rather than an empty row.
fn lineage_label(selected: &PlanCatalogRecord) -> String {
    match (
        selected.lineage.source_plan_id(),
        selected.lineage.source_revision(),
    ) {
        (Some(source), Some(revision)) => {
            format!("from {source} · revision {}", revision.get())
        }
        _ => "root plan · no source".to_owned(),
    }
}

/// # Child-dialog signature contract — waves W3 to W7
///
/// Each non-`Browse` mode is one module, one lane, redesigned in parallel. All
/// five expose this and nothing else:
///
/// ```ignore
/// pub(super) fn dialog(
///     ctx: &egui::Context,
///     draft: &mut SimulationPlanManagerDraft,
///     records: &[PlanCatalogRecord],
/// ) -> Option<PlanManagerAction>;
/// ```
///
/// Ownership, which is the part five lanes must not each decide for themselves:
///
/// * **Rendering cannot reach the application.** A route's `dialog` is handed the
///   draft and the catalog projection, and reports an action. It has no `&mut
///   RSpiceApp`, so it *cannot* mutate the run controller, the workspace, or any
///   other subsystem mid-render even by mistake. Whole-application access is
///   concentrated in the named commit functions below and in this handler.
/// * **Commit is the child's.** Each route owns its `commit_*` function in its
///   own file — `create::commit_create_plan`, `lifecycle::commit_rename_plan`,
///   and so on. This handler is the only caller, and it is thin wiring: it
///   matches an action, calls that route's function, and reports the outcome. A
///   lane changing what its commit does changes only its own file.
/// * **Re-arming is the shell's**, because a child holding `&mut draft` cannot
///   write `simulation_workflow`. This function re-arms after every action, so a
///   route that returns `None` stays open by default and cannot close the manager
///   by omission. Closing is still explicit: `CancelInline` returns to browsing
///   and `Close` dismisses the manager, and nothing else ends it.
/// * **`validation_error` is the child's, for its own route only.** A child sets
///   it while rendering; this handler clears it exactly when the route changes,
///   which is the event that makes a previous route's refusal stale.
/// * **The signature does not grow.** A route needing a fact the projection does
///   not carry adds it to [`PlanCatalogRecord`], where every surface gets it, and
///   paints through [`kit`]. No route-specific parameter is added here, because a
///   fourth parameter for one lane is five lanes' signature change.
fn handle_plan_manager_action(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: SimulationPlanManagerDraft,
    action: Option<PlanManagerAction>,
) {
    let result = match action {
        Some(PlanManagerAction::Close) => {
            app.state.workbench.simulation_workflow = None;
            return;
        }
        Some(PlanManagerAction::OpenSelected) => {
            match lifecycle::commit_activate_plan(app, draft.selected_plan_id) {
                Ok(message) => {
                    app.state.workbench.simulation_workflow = None;
                    app.state
                        .ui
                        .toasts
                        .success(ctx, "Simulation plan opened", message);
                    return;
                }
                Err(error) => Some(Err(error)),
            }
        }
        Some(PlanManagerAction::Create) => {
            draft.mode = SimulationPlanManagerMode::Create;
            draft.name = "New simulation plan".to_owned();
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Campaign) => {
            draft.mode = SimulationPlanManagerMode::Campaign;
            draft.validation_error = None;
            draft.campaign_member_ids.clear();
            for record in plan_catalog_records(app)
                .into_iter()
                .filter(|record| !record.archived)
                .take(2)
            {
                draft.campaign_member_ids.push(record.id);
            }
            None
        }
        Some(PlanManagerAction::Rename) => {
            draft.mode = SimulationPlanManagerMode::Rename;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Compare) => {
            draft.mode = SimulationPlanManagerMode::Compare;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Export) => {
            match exchange::export_simulation_plan_package(app, draft.selected_plan_id) {
                Ok(json) => {
                    draft.exchange_text = json;
                    draft.mode = SimulationPlanManagerMode::Export;
                    draft.validation_error = None;
                    None
                }
                Err(error) => Some(Err(error)),
            }
        }
        Some(PlanManagerAction::Import) => {
            draft.mode = SimulationPlanManagerMode::Import;
            draft.name = "Imported simulation plan".to_owned();
            draft.exchange_text.clear();
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::CopyExport) => {
            ctx.copy_text(draft.exchange_text.clone());
            app.state.ui.toasts.success(
                ctx,
                "Simulation plan copied",
                "Portable plan JSON was copied to the clipboard.",
            );
            None
        }
        Some(PlanManagerAction::ApplyImport) => Some(
            exchange::commit_import_simulation_plan(app, &draft.exchange_text, &draft.name).map(
                |(id, message)| {
                    draft.selected_plan_id = id;
                    draft.mode = SimulationPlanManagerMode::Browse;
                    message
                },
            ),
        ),
        Some(PlanManagerAction::ApplyCampaign) => {
            match campaign::commit_simulation_campaign(app, &draft.campaign_name, &draft.campaign_member_ids)
            {
                Ok(message) => {
                    app.state.workbench.simulation_workflow = None;
                    app.state
                        .ui
                        .toasts
                        .success(ctx, "Simulation campaign queued", message);
                    return;
                }
                Err(error) => Some(Err(error)),
            }
        }
        Some(PlanManagerAction::Archive) => {
            draft.mode = SimulationPlanManagerMode::ConfirmArchive;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::CancelInline) => {
            draft.mode = SimulationPlanManagerMode::Browse;
            draft.validation_error = None;
            None
        }
        Some(PlanManagerAction::Clone) => {
            let name = simulation_plan_catalog_entry(&app.state.sim_setup, draft.selected_plan_id)
                .map(|entry| entry.0)
                .unwrap_or_else(|| "Simulation plan".to_owned());
            app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::ClonePlan(
                ClonePlanDraft::for_source(draft.selected_plan_id, &name),
            ));
            return;
        }
        Some(PlanManagerAction::ApplyCreate) => {
            Some(create::commit_create_plan(app, &draft.name).map(|(id, message)| {
                draft.selected_plan_id = id;
                draft.mode = SimulationPlanManagerMode::Browse;
                message
            }))
        }
        Some(PlanManagerAction::ApplyRename) => {
            Some(lifecycle::commit_rename_plan(app, draft.selected_plan_id, &draft.name))
        }
        Some(PlanManagerAction::ConfirmArchive) => {
            Some(lifecycle::commit_archive_plan(&mut app.state.sim_setup, draft.selected_plan_id))
        }
        Some(PlanManagerAction::Restore) => Some(lifecycle::commit_restore_plan(&mut app.state.sim_setup, draft.selected_plan_id)),
        None => None,
    };

    match result {
        Some(Ok(message)) => {
            draft.mode = SimulationPlanManagerMode::Browse;
            draft.validation_error = None;
            app.state
                .ui
                .toasts
                .success(ctx, "Simulation plan updated", message);
        }
        None => {}
        Some(Err(error)) => draft.validation_error = Some(error),
    }
    app.state.workbench.simulation_workflow = Some(SimulationWorkflowDialog::PlanManager(draft));
}

const fn plan_plural_suffix(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod raster;

#[cfg(test)]
mod tests;
