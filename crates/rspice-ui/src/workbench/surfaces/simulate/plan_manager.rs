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

use crate::ui::widgets::workflow_preview_status;
use crate::workbench::app::purpose_line;
use crate::workbench::app_state::ReferencePvtPoint;
/// Re-exported through this module's `use super::*` for the child routes, which
/// state their transactions as property lists.
use crate::workbench::design_system::property_row_status;
use crate::workbench::state::SimulationPlanScope;

/// The dialog's own copy, in one place because the tests reconstruct the dialog
/// from these same constants.
///
/// The description is not layout. [`Dialog::description`] reaches exactly one
/// place — `set_description` on the dialog's AccessKit node — so it is announced
/// to assistive technology and never painted. Adding or dropping it moves the
/// measured body by 0 points on both axes, at all three gated viewports, under
/// every [`DialogSize`], and under every `initial_height` and `fixed_height`
/// hint.
///
/// What makes a body measurement lie is the body. At 1024x640 the body's scroll
/// area shrinks to its content and bottoms out on its `min_scrolled_height`, so
/// a probe that paints nothing measures 64 points where the budget is 511 —
/// which is why `measured_dialog_body_size` claims 4,000 points before it reads
/// the clip rect. The narrow viewports fill their surface and cannot collapse
/// that way; what inflates them is a reconstruction without the header, which
/// measures 57 points that are not there and so reports a fit while the last row
/// of the aside sits under the footer.
const PLAN_DIALOG_EYEBROW: &str = "SIMULATION · VERSIONED PLAN LIFECYCLE · SINGLE ACTIVE OWNER";
const PLAN_DIALOG_TITLE: &str = "Simulation plans";
const PLAN_DIALOG_PRIMARY: &str = "Open selected plan";

/// What this dialog is for, in one sentence with one owner.
///
/// It is painted as the surface's purpose line and published as the dialog's
/// accessible description, from here. Two copies of a purpose sentence is how a
/// surface comes to tell a reader one thing and a screen reader another.
///
/// The authored sentence also says "govern". RSpice has no governance state on
/// a plan — no approval, no waiver, no release — so that verb is the one word of
/// it not ported.
const PLAN_DIALOG_DESCRIPTION: &str = "Create, select, compare, import, export, and retire simulation plans without copying immutable results or obscuring their exact source plan revision.";

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
/// It does not repeat the word "Filter": the magnifier mark at the field's
/// leading edge says that, and the four field names are what the reader cannot
/// guess.
const PLAN_FILTER_HINT: &str = "Name, identity, revision, or run set…";

/// What that field is called, as against what may be typed into it.
///
/// The hint above is a placeholder and egui publishes it as one, so it is not a
/// name and a reader arriving on this control by keyboard heard nothing.
const PLAN_FILTER_LABEL: &str = "Filter plans";

/// The narrowest the filter field is ever laid out at.
///
/// The field grows into whatever the fixed controls leave, as the authored
/// toolbar's `.grow` does, so this is a floor rather than a width: it is only
/// reached below the widths this dialog is used at, where the row wraps anyway.
const PLAN_FILTER_MINIMUM: f32 = 200.0;

/// Inset from the field's leading edge to the start of its text, which is what
/// leaves room for the magnifier mark painted over it.
const PLAN_FILTER_MARK_INSET: i8 = 29;

/// The authored width of the scope control.
const PLAN_SCOPE_WIDTH: f32 = 190.0;

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
    (SimulationPlanScope::All, "Active project · all plans"),
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
const PLAN_BOUNDARY_NOTES: [(&str, &str); 2] = [
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
];

/// The headline over [`PLAN_IDENTITY_NOTE`], and what it claims.
const PLAN_IDENTITY_HEADLINE: &str = "Stable identity retained";

/// What the four lifecycle operations do to a plan's identity.
///
/// Every clause is checked against its commit path: `rename_plan` never touches
/// the identity or the revision, `clone_active_plan` and `import_plan` both mint
/// one through `clone_as_new`, `restore_plan` reverses an archive, and
/// `archive_plan` refuses the active plan itself rather than relying on this
/// dialog to disable the button.
///
/// It closes the aside, under the operations it qualifies, because it is what
/// the reader needs before pressing one of them — and it is a claim about those
/// four operations rather than about whichever plan is selected, so it is a
/// status line and not a property row.
///
/// The refusal on the active plan is [`ARCHIVE_REFUSED_ON_ACTIVE`]'s, published
/// on the disabled control it is about. Stating it here as well would spend two
/// more lines of the aside's height on a sentence the reader meets by hovering
/// the one button it applies to.
const PLAN_IDENTITY_NOTE: &str = "Rename preserves identity and revision; clone \
     and import mint new identities; archive is reversible.";

/// What the aside says in place of a modelled cost when the run set carries no
/// forecast to model one from.
///
/// Not a second "does not validate". The declaration is what fails to validate,
/// and the cost is simply not forecast as a consequence — two error-toned rows
/// would read as two independent problems.
const NO_MODELLED_COST: &str = "no forecast";

/// Why the active plan's Archive action is refused, in the words the catalog
/// itself refuses it with.
const ARCHIVE_REFUSED_ON_ACTIVE: &str =
    "The active plan cannot be archived. Open another plan first.";

/// Why an archived plan cannot be cloned.
///
/// `commit_clone_plan` activates the source before it clones it, and
/// `activate_plan` refuses an archived plan outright. So the button is not
/// merely unhelpful here — the transaction behind it cannot run.
const CLONE_REFUSED_ON_ARCHIVED: &str =
    "Cloning opens the source plan first, and an archived plan cannot be opened. Restore it first.";

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
        .size(DialogSize::CapabilityReview)
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

/// The manager's body: a purpose line, a toolbar spanning the whole dialog, and
/// a records column beside a selected-plan aside.
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
    purpose_line(ui, PLAN_DIALOG_DESCRIPTION);
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
            plan_manager_records_column(ui, draft, &visible);
        }
        SplitColumn::Aside => {
            if let Some(selected) = selected.as_ref() {
                selected_plan_aside(ui, selected, stacked, action);
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
/// field sat against the dialog's own border. The band owns that inset, its
/// panel fill and the rule that separates it from the records below.
///
/// It spans the whole dialog, over both columns of the split, because it acts on
/// the catalog rather than on the records column: the filter narrows what the
/// table shows *and* what the aside can be pointed at, and the three actions
/// mint a plan the aside has no say in. It stopped at the records column's edge,
/// which left a hole beside Import and read as a control strip belonging to the
/// table alone.
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
            // A `Frame` shrinks to its content, and a row of controls is
            // narrower than the dialog. Without this the band, its fill and its
            // closing rule all stopped wherever the last button did.
            ui.set_min_width(ui.available_width());
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
    let create = Button::new("New plan…").accent().icon(Icon::Add);
    let campaign = Button::new("Queue campaign…")
        .enabled(records.iter().filter(|record| !record.archived).count() >= 2);
    let import = Button::new("Import…");
    // The filter grows into whatever the fixed controls leave, as the authored
    // `.grow` field does. Every width here is the control's own — measured, not
    // authored beside it — so relabelling a button moves the field rather than
    // silently overflowing the row.
    let gap = ui.spacing().item_spacing.x;
    let fixed = PLAN_SCOPE_WIDTH
        + create.measured_width(ui)
        + campaign.measured_width(ui)
        + import.measured_width(ui)
        + 4.0 * gap;

    // Wrapping is the safety net, not the layout: the field takes the slack at
    // every width this dialog is used at, so wrapping only ever catches a window
    // narrower than the fixed controls themselves.
    ui.horizontal_wrapped(|ui| {
        let field = (ui.available_width() - fixed).max(PLAN_FILTER_MINIMUM);
        plan_filter_field(ui, &mut draft.filter, field);
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
            PLAN_SCOPE_WIDTH,
        ) && let Some((scope, _)) = PLAN_SCOPES.get(picked)
        {
            draft.scope = *scope;
        }
        if create.show(ui).clicked() {
            *action = Some(PlanManagerAction::Create);
        }
        if campaign.show(ui).clicked() {
            *action = Some(PlanManagerAction::Campaign);
        }
        if import.show(ui).clicked() {
            *action = Some(PlanManagerAction::Import);
        }
    });
}

/// The filter input, marked by a magnifier at its leading edge.
///
/// The mark replaces the word "Filter" that stood beside the field: a label
/// spending a control's worth of width to name the one control on the row whose
/// purpose its own placeholder already states.
///
/// It still announces a name. A painted mark is not a label and egui publishes
/// `hint_text` as a placeholder, so removing the word left the dialog's only
/// text field unreachable by name.
fn plan_filter_field(ui: &mut Ui, filter: &mut String, width: f32) {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        egui::TextEdit::singleline(filter)
            .font(egui::TextStyle::Monospace)
            .margin(egui::Margin {
                left: PLAN_FILTER_MARK_INSET,
                right: 8,
                top: 4,
                bottom: 4,
            })
            .hint_text(PLAN_FILTER_HINT),
    );
    crate::ui::widgets::name_control(ui, &response, PLAN_FILTER_LABEL);
    WorkbenchIcon::Search.paint(
        ui.painter(),
        Rect::from_center_size(
            egui::pos2(response.rect.left() + 15.0, response.rect.center().y),
            vec2(14.0, 14.0),
        ),
        t.color.text_faint,
    );
}

/// The records table and the two boundaries that qualify every operation on it.
fn plan_manager_records_column(
    ui: &mut Ui,
    draft: &mut SimulationPlanManagerDraft,
    visible: &[PlanCatalogRecord],
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

/// Gap between the action grid's cells, and between the grid and what it
/// follows. The authored `.simulation-plan-actions` grid's own metric.
const PLAN_ACTION_GAP: f32 = 6.0;

/// The selected plan, everything the catalog knows about it, the operations on
/// it, and what those operations preserve.
///
/// This is the authored aside, in the authored order: a head carrying the
/// lifecycle, one flat property list, a two-column action grid closed by the one
/// destructive action across both columns, and a status line at the bottom edge.
/// The operations sit here, on the plan they act on, rather than under the
/// table.
fn selected_plan_aside(
    ui: &mut Ui,
    selected: &PlanCatalogRecord,
    stacked: bool,
    action: &mut Option<PlanManagerAction>,
) {
    // Stacked, the aside has the whole dialog width. Both the list and the grid
    // spend it on columns rather than on height, because stacking has already
    // spent the height on the records column above.
    selected_plan_properties(ui, selected, if stacked { 2 } else { 1 });
    ui.add_space(PLAN_ACTION_GAP);
    plan_selection_actions(ui, selected, if stacked { 4 } else { 2 }, action);
    workflow_preview_status(ui, true, PLAN_IDENTITY_HEADLINE, PLAN_IDENTITY_NOTE);
}

/// The five operations on the selected plan: four equal ones in a grid, then the
/// destructive one across its whole width.
///
/// A refusal is stated where the reader meets it. Archive is disabled on the
/// active plan and Clone on an archived one, and each carries the reason its own
/// commit path would refuse with — so hovering the control answers the question
/// the disabled control raises, rather than leaving the reader to try it.
///
/// `columns` is the authored two beside the table. Stacked, the grid has the
/// whole dialog width and four 260-point buttons in two rows would be both ugly
/// and 50 points of height the stacked arrangement does not have, so the four go
/// across in one row.
fn plan_selection_actions(
    ui: &mut Ui,
    selected: &PlanCatalogRecord,
    columns: usize,
    action: &mut Option<PlanManagerAction>,
) {
    let full = ui.available_width();
    let cell =
        ((full - PLAN_ACTION_GAP * (columns.saturating_sub(1)) as f32) / columns as f32).floor();
    let mut responses = Vec::with_capacity(4);
    // The clone refusal is published on the disabled control rather than beside
    // it, so it costs no height on the surface that cannot spare any.
    let mut actions = vec![
        Button::new("Rename…"),
        Button::new("Clone…").enabled(!selected.archived),
        Button::new("Compare…"),
        Button::new("Export…"),
    ];
    while !actions.is_empty() {
        let row = actions
            .drain(..columns.min(actions.len()))
            .collect::<Vec<_>>();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = PLAN_ACTION_GAP;
            for button in row {
                responses.push(button.min_width(cell).show(ui));
            }
        });
        if !actions.is_empty() {
            ui.add_space(PLAN_ACTION_GAP);
        }
    }
    for (response, chosen) in responses.into_iter().zip([
        PlanManagerAction::Rename,
        PlanManagerAction::Clone,
        PlanManagerAction::Compare,
        PlanManagerAction::Export,
    ]) {
        let response = if matches!(chosen, PlanManagerAction::Clone) {
            response.on_disabled_hover_text(CLONE_REFUSED_ON_ARCHIVED)
        } else {
            response
        };
        if response.clicked() {
            *action = Some(chosen);
        }
    }
    ui.add_space(PLAN_ACTION_GAP);

    if selected.archived {
        if Button::new("Restore").min_width(full).show(ui).clicked() {
            *action = Some(PlanManagerAction::Restore);
        }
    } else if Button::new("Archive…")
        .destructive(true)
        .enabled(!selected.active)
        .min_width(full)
        .show(ui)
        .on_disabled_hover_text(ARCHIVE_REFUSED_ON_ACTIVE)
        .clicked()
    {
        *action = Some(PlanManagerAction::Archive);
    }
}

/// How one detail row states its value.
enum PlanDetailTone {
    /// An ordinary fact.
    Stated,
    /// A quantity that is absent because something it derives from is. Faint
    /// rather than error-toned: it is a consequence, and a second red row would
    /// read as a second problem.
    Absent,
    /// A declaration that does not validate — the failure itself.
    Failed,
}

/// One row of the selected-plan list.
struct PlanDetailRow {
    label: &'static str,
    value: String,
    tone: PlanDetailTone,
}

/// Everything the catalog knows about the selected plan, in one flat list.
///
/// The authored aside is six rows under one heading, and two of those six — a
/// per-plan design and testbench binding, and a named execution profile — have
/// no owner anywhere in RSpice. So the list states the strongest facts the
/// catalog does own in their place: the reference corner, the run set's own
/// forecast, the cost that forecast models, and the model closure.
///
/// It repeats nothing the table already paints. The stable identity is the
/// second line of every identity cell and the result count is a column, and the
/// authored aside repeats nothing but the plan's name either.
///
/// Every quantity comes off [`PlanCatalogRecord`], never off a second
/// derivation — including the two that go absent with an unvalidated run set.
/// The declared scale and its modelled cost are two rows rather than one because
/// the combined value did not fit the aside's value column and was elided at
/// every gated width; each half paints whole.
fn plan_detail_rows(selected: &PlanCatalogRecord) -> Vec<PlanDetailRow> {
    let stated = |label, value: String| PlanDetailRow {
        label,
        value,
        tone: PlanDetailTone::Stated,
    };
    vec![
        stated("Name", selected.name.clone()),
        stated("Revision", selected.revision.to_string()),
        stated(
            "Reference PVT corner",
            reference_pvt_label(selected.reference_pvt),
        ),
        match (selected.point_count(), selected.task_count()) {
            (Some(points), Some(tasks)) => stated(
                "Declared run set",
                format!(
                    "{points} PVT point{} · {tasks} task{}",
                    plan_plural_suffix(points),
                    plan_plural_suffix(tasks)
                ),
            ),
            _ => PlanDetailRow {
                label: "Declared run set",
                value: RUN_SET_DOES_NOT_VALIDATE.to_owned(),
                tone: PlanDetailTone::Failed,
            },
        },
        match (selected.estimated_duration(), selected.estimated_storage()) {
            (Some(duration), Some(storage)) => {
                stated("Modelled cost", format!("{duration} · {storage}"))
            }
            _ => PlanDetailRow {
                label: "Modelled cost",
                value: NO_MODELLED_COST.to_owned(),
                tone: PlanDetailTone::Absent,
            },
        },
        stated(
            "Model closure",
            format!(
                "{} binding{}",
                selected.model_bindings,
                plan_plural_suffix(selected.model_bindings)
            ),
        ),
        stated(
            "Regression baseline",
            selected
                .regression_baseline
                .map_or_else(|| "none pinned".to_owned(), |run| format!("run {run}")),
        ),
        stated("Source lineage", lineage_label(selected)),
    ]
}

/// The aside's head and its property list.
///
/// The head carries the selected plan's lifecycle. The authored aside states it
/// as active or available, which is a binary reading of three states: an
/// archived plan is not available, and the dialog's own primary action is
/// disabled for it. So the head states the same word the Lifecycle column does,
/// from the same owner.
///
/// `columns` is how the list spends a width it did not ask for. Beside the table
/// the aside is one narrow track and the list is flat, as the authored one is.
/// Stacked, the aside has the whole dialog width and eight rows in a single
/// track would cost 232 points of a budget the records column has already spent
/// most of — so there it pays the height of half the rows instead of all of
/// them. One row list feeds both arrangements, so the two cannot come to state
/// different things.
fn selected_plan_properties(ui: &mut Ui, selected: &PlanCatalogRecord, columns: usize) {
    kit::section_head(
        ui,
        "Selected plan",
        Some(HeadStatus {
            label: selected.lifecycle_label(),
            tone: lifecycle_tone(selected),
        }),
    );
    let rows = plan_detail_rows(selected);
    if columns <= 1 {
        for row in &rows {
            plan_detail_row(ui, row);
        }
        return;
    }
    let per_column = rows.len().div_ceil(columns);
    kit::equal_columns(ui, columns, |ui, index| {
        for row in rows.iter().skip(index * per_column).take(per_column) {
            plan_detail_row(ui, row);
        }
    });
}

fn plan_detail_row(ui: &mut Ui, row: &PlanDetailRow) {
    let t = Tokens::get(ui.ctx());
    let (tone, mark) = match row.tone {
        PlanDetailTone::Stated => (t.color.text, None),
        PlanDetailTone::Absent => (t.color.text_faint, None),
        PlanDetailTone::Failed => (t.color.err, Some(StatusMark::Failure)),
    };
    kit::detail_row(ui, row.label, &row.value, tone, mark);
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
            draft.campaign.member_ids.clear();
            for record in plan_catalog_records(app)
                .into_iter()
                .filter(|record| !record.archived)
                .take(2)
            {
                draft.campaign.member_ids.push(record.id);
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
            match campaign::commit_simulation_campaign(
                app,
                &draft.campaign.name,
                &draft.campaign.member_ids,
            ) {
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
        Some(PlanManagerAction::ApplyCreate) => Some(
            create::commit_create_plan(app, &draft.name, &draft.new_plan).map(|(id, message)| {
                draft.selected_plan_id = id;
                draft.mode = SimulationPlanManagerMode::Browse;
                message
            }),
        ),
        Some(PlanManagerAction::ApplyRename) => Some(lifecycle::commit_rename_plan(
            app,
            draft.selected_plan_id,
            &draft.name,
        )),
        Some(PlanManagerAction::ConfirmArchive) => Some(lifecycle::commit_archive_plan(
            &mut app.state.sim_setup,
            draft.selected_plan_id,
        )),
        Some(PlanManagerAction::Restore) => Some(lifecycle::commit_restore_plan(
            &mut app.state.sim_setup,
            draft.selected_plan_id,
        )),
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
